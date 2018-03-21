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
use exonum::blockchain::{ApiContext, Blockchain, Service, Transaction,
  TransactionSet, ExecutionResult};
use exonum::crypto::{Hash, PublicKey};
use exonum::encoding;
use exonum::encoding::serialize::FromHex;
use exonum::explorer::BlockchainExplorer;
use exonum::messages::{Message, RawTransaction};
use exonum::node::{ApiSender, TransactionSend};
use exonum::storage::{Fork, ProofMapIndex, Snapshot};

use iron::prelude::*;
use iron::Handler;
use iron::status::Status;
use iron::headers::ContentType;
use iron::modifiers::Header;

use router::Router;

const SERVICE_ID: u16 = 13;

// base data types.
// A timestamp contains a public key, a hash of a document/content, time (UNIX time)
encoding_struct! {
    struct Timestamp {
        pub_key: &PublicKey,
        content: &Hash,
        time: u64,
    }
}

// Any blockchain operation should be expressed as a transaction. In this case it is described with
// the service's ID, a public key and data
transactions! {
    TimestampServiceTransactions {
        const SERVICE_ID = SERVICE_ID;

        struct TxTimestamp {
            from: &PublicKey,
            content: &Hash,
        }
    }
}

// Each transaction implements two basic operations:
// verify - logical check BEFORE commiting to blockchain. In this case we verify senders ID
// execute - storing a new timestamp withn blockchain
impl Transaction for TxTimestamp {
    fn verify(&self) -> bool {
        self.verify_signature(self.from())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = TimestampSchema::new(view);

        if schema.timestamp(self.from()).is_none() {
            let now = time::precise_time_s() as u64;
            let timestamp = Timestamp::new(self.from(), self.content(), now);

            schema.timestamps_mut().put(self.from(), timestamp);
        }
        Ok(())
    }
}

// To interact with blockchain we should define two views: for reading and writing data
// interface
pub struct TimestampSchema<T> {
    view: T,
}

// this one allows as to modify blockchain data
impl<'a> TimestampSchema<&'a mut Fork> {
    pub fn timestamps_mut(&mut self) -> ProofMapIndex<&mut Fork, PublicKey, Timestamp> {
        ProofMapIndex::new("timestamp.timestamps", &mut self.view)
    }
}

// this one is read-only and provides access to a blockchain snapshot
impl<T: AsRef<Snapshot>> TimestampSchema<T> {
    pub fn new(view: T) -> Self {
        TimestampSchema { view }
    }

    pub fn timestamps(&self) -> ProofMapIndex<&Snapshot, PublicKey, Timestamp> {
        ProofMapIndex::new("timestamp.timestamps", self.view.as_ref())
    }

    pub fn timestamp(&self, pub_key: &PublicKey) -> Option<Timestamp> {
        self.timestamps().get(pub_key)
    }

    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.timestamps().root_hash()]
    }
}

// basic type to get REST response
#[derive(Serialize, Deserialize)]
pub struct TimestampResponse {
    pub tx_hash: Hash,
}

// Interface for system's backend
#[derive(Clone)]
struct TimestampApi {
    channel: ApiSender,
    blockchain: Blockchain,
}

// Registering handlers for REST API. We define 4 endpoints
impl Api for TimestampApi {
    fn wire(&self, router: &mut Router) {
        self.clone().set_timestamp(router);
        self.clone().set_timestamps(router);
        self.clone().set_submit(router);
        self.clone().set_block_stats(router);
    }
}

impl TimestampApi {
    // helpers to register all endpoints
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

    // Endpoint for creating a new timestamp.
    // Input: a transaction in JSON format
    // Effect: serializes the input into TxTransaction, stores it into a blockchain
    // Return value: transaction's hash
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

    // Endpoint for searching for specific transaction.
    // Input: a public key
    // Effect: Finds a transaction by its public key
    // Return value: a transaction data in JSON
    fn timestamp(&self, req: &mut Request) -> IronResult<Response> {
        let path = req.url.path();
        let public_key = PublicKey::from_hex(path.last().unwrap()).map_err(
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

    // Endpoint for listing all available transactions.
    // Input: N/A
    // Effect: gets a blockchain's snapshot and gathers all transactions
    // Return value: list of transactions
    fn timestamps(&self, _: &mut Request) -> IronResult<Response> {
        let snapshot = self.blockchain.snapshot();
        let schema = TimestampSchema::new(snapshot);
        let idx = schema.timestamps();
        let timestamps: Vec<Timestamp> = idx.values().collect();

        self.ok_response(&serde_json::to_value(&timestamps).unwrap())
    }

    // Endpoint for searching for a specific trasactions block.
    // Input: Block ID
    // Effect: Creates a blockchain explorer, which is used to find a block
    // Return value: list of transactions within a block
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
}

// Exonum model relies on introducing various public services to interact with blockchain
pub struct TimestampService;

impl Service for TimestampService {
    // mandatory identifications
    fn service_name(&self) -> &'static str {
        "timestamp"
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
        let tx = TimestampServiceTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    fn state_hash(&self, snapshot: &Snapshot) -> Vec<Hash> {
        let schema = TimestampSchema::new(snapshot);
        schema.state_hash()
    }

    // setup REST API
    fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();
        let api = TimestampApi {
            channel: ctx.node_channel().clone(),
            blockchain: ctx.blockchain().clone(),
        };
        api.wire(&mut router);
        Some(Box::new(router))
    }
}
