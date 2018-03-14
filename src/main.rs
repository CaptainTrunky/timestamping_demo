extern crate bodyparser;

#[macro_use]
extern crate exonum;

extern crate iron;

extern crate router;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate time;

use exonum::api::{Api, ApiError};
use exonum::blockchain::{Blockchain, Service, Transaction, ExecutionResult};
use exonum::crypto::{gen_keypair, Hash, PublicKey, CryptoHash};
use exonum::encoding::serialize::FromHex;
use exonum::explorer::BlockchainExplorer;
use exonum::messages::Message;
use exonum::node::{ApiSender, TransactionSend};
use exonum::storage::{Fork, MapIndex, Snapshot};

use iron::prelude::*;
use iron::Handler;
use iron::status::Status;
use iron::headers::ContentType;
use iron::modifiers::Header;

use router::Router;

const SERVICE_ID: u16 = 13;

encoding_struct! {
    struct Timestamp {
        pub_key: &PublicKey,
        msg: &str,
        time: u64,
    }
}

transactions! {
    TimestampServiceTransactions {
        const SERVICE_ID = SERVICE_ID;

        struct TxTimestamp {
            from: &PublicKey,
            msg: &str,
        }
    }
}

impl Transaction for TxTimestamp {
    fn verify(&self) -> bool {
        self.verify_signature(self.from())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = TimestampSchema::new(view);

        if schema.timestamp(self.from()).is_none() {
            let now = time::precise_time_s() as u64;
            let timestamp = Timestamp::new(self.from(), self.msg(), now);

            println!("Create the timestamp: {:?}", timestamp);

            schema.timestamps_mut().put(self.from(), timestamp);
        }
        Ok(())
    }
}

pub struct TimestampSchema<T> {
    view: T,
}

impl<'a> TimestampSchema<&'a mut Fork> {
    pub fn timestamps_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, Timestamp> {
        MapIndex::new("timestamp.timestamps", &mut self.view)
    }
}

impl<T: AsRef<Snapshot>> TimestampSchema<T> {
    pub fn new(view: T) -> Self {
        TimestampSchema { view }
    }

    pub fn timestamps(&self) -> MapIndex<&Snapshot, PublicKey, Timestamp> {
        MapIndex::new("timestamp.timestamps", self.view.as_ref())
    }

    pub fn timestamp(&self, pub_key: &PublicKey) -> Option<Timestamp> {
        self.timestamps().get(pub_key)
    }
}

#[derive(Serialize, Deserialize)]
pub struct TimestampResponse {
    pub tx_hash: Hash,
}

#[derive(Clone)]
struct TimestampApi {
    channel: ApiSender,
    blockchain: Blockchain,
}

impl Api for TimestampApi {
    fn wire(&self, router: &mut Router) {
        self.clone().set_timestamp(router);
        self.clone().set_timestamps(router);
        self.clone().set_submit(router);
        self.clone().set_block_stats(router);
    }
}

impl TimestampApi {
    fn submit(&self, req: &mut Request) -> IronResult<Response> {
        match req.get::<bodyparser::Struct<TimestampServiceTransactions>>() {
            Ok(Some(transaction)) => {
                let transaction: Box<Transaction> = transaction.into();
                let tx_hash = transaction.hash();
                self.channel.send(transaction).map_err(ApiError::from)?;
                let json = TimestampResponse { tx_hash };
                self.ok_response(&serde_json::to_value(&json).unwrap())
            }
            Ok(None) => Err(ApiError::BadRequest("Empty request".into()))?,
            Err(e) => Err(ApiError::BadRequest(e.to_string()))?,
        }
    }

    fn timestamp(&self, req: &mut Request) -> IronResult<Response> {
        let path = req.url.path();
        let key = path.last().unwrap();
        let public_key = PublicKey::from_hex(key).map_err(
            |e| {
                IronError::new(e, (
                        Status::BadRequest,
                        Header(ContentType::json()),
                        "\"Invalid request param: `pub_key`\"",
                    )
                )
            }
        )?;

        let timestamp = {
            let snapshot = self.blockchain.snapshot();
            let schema = TimestampSchema::new(snapshot);
            schema.timestamp(&public_key)
        };

        if let Some(timestamp) = timestamp {
            self.ok_response(&serde_json::to_value(timestamp).unwrap())
        } else {
            self.not_found_response(&serde_json::to_value("Not found").unwrap())
        }
    }

    fn timestamps(&self, req: &mut Request) -> IronResult<Response> {
        let snapshot = self.blockchain.snapshot();
        let schema = TimestampSchema::new(snapshot);
        let idx = schema.timestamps();
        let timestamps: Vec<Timestamp> = idx.values().collect();

        self.ok_response(&serde_json::to_value(&timestamps).unwrap())
    }

    fn block_stats(&self, req: &mut Request) -> IronResult<Response> {
        let path = req.url.path();
        let block_id = exonum::helpers::Height(path.last().unwrap().parse::<u64>().unwrap());

        let blockchain_explorer = BlockchainExplorer::new(&self.blockchain);

        match blockchain_explorer.block_info(block_id) {
            Some(block_info) => {
                let result = format!("Block info: {:?}", block_info);
                self.ok_response(&serde_json::to_value(result).unwrap())
            }
            None => {
                let result = format!("Block {:?} not found", block_id);
                self.ok_response(&serde_json::to_value(result).unwrap())
            }
        }
    }

    fn set_timestamp(self, router: &mut Router) {
        let timestamp = move |req: &mut Request| self.timestamp(req);
        router.get("/v1/timestamp/:pub_key", timestamp, "timestamp");
    }

    fn set_timestamps(self, router: &mut Router) {
        let timestamps = move |req: &mut Request| self.timestamps(req);
        router.get("/v1/timestamps", timestamps, "timestamps");
    }

    fn set_submit(self, router: &mut Router) {
        let submit = move |req: &mut Request| self.submit(req);
        router.post("/v1/submit", submit, "submit");
    }

    fn set_block_stats(self, router: &mut Router) {
        let stats = move |req: &mut Request| self.block_stats(req);
        router.get("/v1/block_stats/:id", stats, "block_stats");
    }
}

fn main() {
  println!("waka-waka");
}
