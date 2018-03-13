#[macro_use]
extern crate exonum;
#[macro_use]
extern crate serde_json;

use exonum::crypto::{gen_keypair, Hash, PublicKey, CryptoHash};
use exonum::blockchain::{Schema, Service, Transaction, ExecutionResult};
use exonum::messages::{Message};
use exonum::storage::{Fork, MapIndex, Snapshot};
use exonum::encoding;

const SERVICE_ID: u16 = 13;

transactions! {
    TimestampingServiceTransactions {
        const SERVICE_ID = SERVICE_ID;

        struct TxTimestamp {
            from: &PublicKey,
            msg: &str,
        }
    }
}

encoding_struct! {
    struct Timestamp {
        content_hash: &Hash,
        time: u64,
    }
}

impl Transaction for TxTimestamp {
    fn verify(&self) -> bool {
        self.verify_signature(self.from())
    }

    fn execute(&self, _fork: &mut Fork) -> ExecutionResult {
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

fn main() {
  println!("waka-waka");
}
