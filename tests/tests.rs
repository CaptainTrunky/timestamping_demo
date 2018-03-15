extern crate exonum;
#[macro_use]
extern crate exonum_testkit;

extern crate timestamping;

extern crate serde_json;

use exonum::blockchain::Schema;
use exonum::crypto;

use exonum::crypto::{CryptoHash, gen_keypair, PublicKey};
use exonum_testkit::{ApiKind, TestKitBuilder};

use timestamping::{TimestampService, TxTimestamp};

#[test]
fn test_submit_basic() {
    // Create testkit for network with four validators.
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Create few transactions.
    let keypair = gen_keypair();
    let tx1 = TxTimestamp::new(&keypair.0, &crypto::hash(b"Down To Earth"), &keypair.1);
    let tx2 = TxTimestamp::new(&keypair.0, &crypto::hash(b"Cry Over Spilt Milk"), &keypair.1);
    let tx3 = TxTimestamp::new(&keypair.0, &crypto::hash(b"Dropping Like Flies"), &keypair.1);

    // Commit them into blockchain.
    testkit.create_block_with_transactions(
        txvec![tx1.clone(), tx2.clone(), tx3.clone()]
        );

    // Check results with schema.
    let snapshot = testkit.snapshot();
    let schema = Schema::new(&snapshot);

    assert!(schema.transactions().contains(&tx1.hash()));
    assert!(schema.transactions().contains(&tx2.hash()));
    assert!(schema.transactions().contains(&tx3.hash()));
}

#[test]
fn test_submit_rest() {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Create few transactions.
    let keypair = gen_keypair();
    let tx1 = TxTimestamp::new(&keypair.0, &crypto::hash(b"Down To Earth"), &keypair.1);

    // Check results with api
    let api = testkit.api();

    api.post::<TxTimestamp, serde_json::Value>(ApiKind::Service("timestamp"), "v1/submit", &tx1);

    testkit.create_block();

    let res = api.get::<serde_json::Value>(ApiKind::Service("timestamp"), "v1/timestamps");

    assert_eq!(res.as_array().unwrap().len(), 1);
}

#[test]
fn test_get_timestamp_by_hash_rest() {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Create few transactions.
    let keypair = gen_keypair();
    let tx1 = TxTimestamp::new(&keypair.0, &crypto::hash(b"Down To Earth"), &keypair.1);

    // Check results with api
    let api = testkit.api();

    api.post::<TxTimestamp, serde_json::Value>(ApiKind::Service("timestamp"), "v1/submit", &tx1);

    testkit.create_block();

    let pk = PublicKey::to_hex(&keypair.0);

    let res = api.get::<serde_json::Value>(ApiKind::Service("timestamp"), &format!("v1/timestamp/{}", &pk));

    assert!(res.is_object());
    assert_eq!(&res.as_object().unwrap()["pub_key"], &pk);
}

#[test]
fn test_submit_get_block_by_id() {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Create few transactions.
    let keypair = gen_keypair();
    let tx1 = TxTimestamp::new(&keypair.0, &crypto::hash(b"Down To Earth"), &keypair.1);
    let tx2 = TxTimestamp::new(&keypair.0, &crypto::hash(b"Cry Over Spilt Milk"), &keypair.1);
    let tx3 = TxTimestamp::new(&keypair.0, &crypto::hash(b"Dropping Like Flies"), &keypair.1);

    // Check results with api
    let api = testkit.api();

    api.post::<TxTimestamp, serde_json::Value>(ApiKind::Service("timestamp"), "v1/submit", &tx1);
    api.post::<TxTimestamp, serde_json::Value>(ApiKind::Service("timestamp"), "v1/submit", &tx2);
    api.post::<TxTimestamp, serde_json::Value>(ApiKind::Service("timestamp"), "v1/submit", &tx3);

    testkit.create_block();

    let res = api.get::<serde_json::Value>(ApiKind::Service("timestamp"), &format!("v1/block_stats/{}", 1));

    assert!(res.is_string());
    assert!(!res.as_str().unwrap().contains("not found"));
}


