#[macro_use]
extern crate bencher;

extern crate exonum;
extern crate exonum_testkit;

extern crate rand;

extern crate serde_json;

extern crate timestamping;

use exonum::crypto;

use exonum::crypto::{gen_keypair, PublicKey};
use exonum_testkit::{ApiKind, TestKitBuilder};

use timestamping::{TimestampService, TxTimestamp};

use rand::{Rng, thread_rng};

use bencher::Bencher;

fn bench_submit_10(b: &mut Bencher) {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Check results with api
    let api = testkit.api();

    let mut rng = thread_rng();

    let mut data: Vec<TxTimestamp> = Vec::with_capacity(10);
    for _ in 0..10 {
        let letter: char = rng.gen_range(b'A', b'Z') as char;
        let number: u32 = rng.gen_range(0, 999999);
        let s = format!("{}{:06}", letter, number);

        let keypair = gen_keypair();
        let tx = TxTimestamp::new(&keypair.0, &crypto::hash(s.as_bytes()), &keypair.1);

        data.push(tx);
    }

    b.iter(
        || { for i in 1..11 {
            api.post::<TxTimestamp, serde_json::Value>(
                ApiKind::Service("timestamp"),
                "v1/submit", &data[i-1]
                );
            }
            testkit.create_block();
        }
    );
}

fn bench_submit_50(b: &mut Bencher) {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Check results with api
    let api = testkit.api();

    let mut rng = thread_rng();

    let mut data: Vec<TxTimestamp> = Vec::with_capacity(50);
    for _ in 0..50 {
        let letter: char = rng.gen_range(b'A', b'Z') as char;
        let number: u32 = rng.gen_range(0, 999999);
        let s = format!("{}{:06}", letter, number);

        let keypair = gen_keypair();
        let tx = TxTimestamp::new(&keypair.0, &crypto::hash(s.as_bytes()), &keypair.1);

        data.push(tx);
    }

    b.iter(
        || { for i in 1..51 {
            api.post::<TxTimestamp, serde_json::Value>(
                ApiKind::Service("timestamp"),
                "v1/submit", &data[i-1]
                );
            }
            testkit.create_block();
        }
    );
}

fn bench_submit_100(b: &mut Bencher) {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Check results with api
    let api = testkit.api();

    let mut rng = thread_rng();

    let mut data: Vec<TxTimestamp> = Vec::with_capacity(1000);
    for _ in 0..1000 {
        let letter: char = rng.gen_range(b'A', b'Z') as char;
        let number: u32 = rng.gen_range(0, 999999);
        let s = format!("{}{:06}", letter, number);

        let keypair = gen_keypair();
        let tx = TxTimestamp::new(&keypair.0, &crypto::hash(s.as_bytes()), &keypair.1);

        data.push(tx);
    }

    b.iter(
        || { for i in 1..101 {
            api.post::<TxTimestamp, serde_json::Value>(
                ApiKind::Service("timestamp"),
                "v1/submit", &data[i-1]
                );
            if i % 10 == 0 { testkit.create_block(); }
            }
        }
    );
}

fn bench_submit_200(b: &mut Bencher) {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Check results with api
    let api = testkit.api();

    let mut rng = thread_rng();

    let mut data: Vec<TxTimestamp> = Vec::with_capacity(200);
    for _ in 0..200 {
        let letter: char = rng.gen_range(b'A', b'Z') as char;
        let number: u32 = rng.gen_range(0, 999999);
        let s = format!("{}{:06}", letter, number);

        let keypair = gen_keypair();
        let tx = TxTimestamp::new(&keypair.0, &crypto::hash(s.as_bytes()), &keypair.1);

        data.push(tx);
    }

    b.iter(
        || { for i in 1..201 {
            api.post::<TxTimestamp, serde_json::Value>(
                ApiKind::Service("timestamp"),
                "v1/submit", &data[i-1]
                );
            if i % 50 == 0 { testkit.create_block(); }
            }
        }
    );
}
fn bench_submit_500(b: &mut Bencher) {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Check results with api
    let api = testkit.api();

    let mut rng = thread_rng();

    let mut data: Vec<TxTimestamp> = Vec::with_capacity(500);
    for _ in 0..500 {
        let letter: char = rng.gen_range(b'A', b'Z') as char;
        let number: u32 = rng.gen_range(0, 999999);
        let s = format!("{}{:06}", letter, number);

        let keypair = gen_keypair();
        let tx = TxTimestamp::new(&keypair.0, &crypto::hash(s.as_bytes()), &keypair.1);

        data.push(tx);
    }

    b.iter(
        || { for i in 1..501 {
            api.post::<TxTimestamp, serde_json::Value>(
                ApiKind::Service("timestamp"),
                "v1/submit", &data[i-1]
                );
            if i % 50 == 0 { testkit.create_block(); }
            }
        }
    );
}

fn bench_submit_1000(b: &mut Bencher) {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Check results with api
    let api = testkit.api();

    let mut rng = thread_rng();

    let mut data: Vec<TxTimestamp> = Vec::with_capacity(1000);
    for _ in 0..1000 {
        let letter: char = rng.gen_range(b'A', b'Z') as char;
        let number: u32 = rng.gen_range(0, 999999);
        let s = format!("{}{:06}", letter, number);

        let keypair = gen_keypair();
        let tx = TxTimestamp::new(&keypair.0, &crypto::hash(s.as_bytes()), &keypair.1);

        data.push(tx);
    }

    b.iter(
        || { for i in 1..1001 {
            api.post::<TxTimestamp, serde_json::Value>(
                ApiKind::Service("timestamp"),
                "v1/submit", &data[i-1]
                );
            if i % 100 == 0 { testkit.create_block(); }
            }
        }
    );
}

fn bench_search_10(b: &mut Bencher) {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Check results with api
    let api = testkit.api();

    let mut rng = thread_rng();

    let mut data: Vec<TxTimestamp> = Vec::with_capacity(10);

    for _ in 0..10 {
        let letter: char = rng.gen_range(b'A', b'Z') as char;
        let number: u32 = rng.gen_range(0, 999999);
        let s = format!("{}{:06}", letter, number);

        let keypair = gen_keypair();
        let tx = TxTimestamp::new(&keypair.0, &crypto::hash(s.as_bytes()), &keypair.1);

        data.push(tx);
    }

    for i in 1..11 {
        api.post::<TxTimestamp, serde_json::Value>(
            ApiKind::Service("timestamp"),
            "v1/submit", &data[i-1]
        );
    }
    testkit.create_block();

    let req = format!("/v1/timestamp/{}", PublicKey::to_hex(&data[0].from()));

    b.iter(|| { api.get::<serde_json::Value>(ApiKind::Service("timestamp"), &req); });
}

fn bench_search_50(b: &mut Bencher) {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Check results with api
    let api = testkit.api();

    let mut rng = thread_rng();

    let mut data: Vec<TxTimestamp> = Vec::with_capacity(50);

    for _ in 0..50 {
        let letter: char = rng.gen_range(b'A', b'Z') as char;
        let number: u32 = rng.gen_range(0, 999999);
        let s = format!("{}{:06}", letter, number);

        let keypair = gen_keypair();
        let tx = TxTimestamp::new(&keypair.0, &crypto::hash(s.as_bytes()), &keypair.1);

        data.push(tx);
    }

    for i in 1..51 {
        api.post::<TxTimestamp, serde_json::Value>(
            ApiKind::Service("timestamp"),
            "v1/submit", &data[i-1]
        );
    }
    testkit.create_block();

    let req = format!("/v1/timestamp/{}", PublicKey::to_hex(&data[0].from()));

    b.iter(|| { api.get::<serde_json::Value>(ApiKind::Service("timestamp"), &req); });
}

fn bench_search_100(b: &mut Bencher) {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Check results with api
    let api = testkit.api();

    let mut rng = thread_rng();

    let mut data: Vec<TxTimestamp> = Vec::with_capacity(100);

    for _ in 0..100 {
        let letter: char = rng.gen_range(b'A', b'Z') as char;
        let number: u32 = rng.gen_range(0, 999999);
        let s = format!("{}{:06}", letter, number);

        let keypair = gen_keypair();
        let tx = TxTimestamp::new(&keypair.0, &crypto::hash(s.as_bytes()), &keypair.1);

        data.push(tx);
    }

    for i in 1..101 {
        api.post::<TxTimestamp, serde_json::Value>(
            ApiKind::Service("timestamp"),
            "v1/submit", &data[i-1]
        );
        if i % 10 == 0 { testkit.create_block(); }
    }

    let req = format!("/v1/timestamp/{}", PublicKey::to_hex(&data[0].from()));

    b.iter(|| { api.get::<serde_json::Value>(ApiKind::Service("timestamp"), &req); });
}

fn bench_search_200(b: &mut Bencher) {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Check results with api
    let api = testkit.api();

    let mut rng = thread_rng();

    let mut data: Vec<TxTimestamp> = Vec::with_capacity(200);

    for _ in 0..200 {
        let letter: char = rng.gen_range(b'A', b'Z') as char;
        let number: u32 = rng.gen_range(0, 999999);
        let s = format!("{}{:06}", letter, number);

        let keypair = gen_keypair();
        let tx = TxTimestamp::new(&keypair.0, &crypto::hash(s.as_bytes()), &keypair.1);

        data.push(tx);
    }

    for i in 1..201 {
        api.post::<TxTimestamp, serde_json::Value>(
            ApiKind::Service("timestamp"),
            "v1/submit", &data[i-1]
        );
        if i % 50 == 0 { testkit.create_block(); }
    }

    let req = format!("/v1/timestamp/{}", PublicKey::to_hex(&data[0].from()));

    b.iter(|| { api.get::<serde_json::Value>(ApiKind::Service("timestamp"), &req); });
}

fn bench_search_500(b: &mut Bencher) {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Check results with api
    let api = testkit.api();

    let mut rng = thread_rng();

    let mut data: Vec<TxTimestamp> = Vec::with_capacity(500);

    for _ in 0..500 {
        let letter: char = rng.gen_range(b'A', b'Z') as char;
        let number: u32 = rng.gen_range(0, 999999);
        let s = format!("{}{:06}", letter, number);

        let keypair = gen_keypair();
        let tx = TxTimestamp::new(&keypair.0, &crypto::hash(s.as_bytes()), &keypair.1);

        data.push(tx);
    }

    for i in 1..501 {
        api.post::<TxTimestamp, serde_json::Value>(
            ApiKind::Service("timestamp"),
            "v1/submit", &data[i-1]
        );
        if i % 50 == 0 { testkit.create_block(); }
    }

    let req = format!("/v1/timestamp/{}", PublicKey::to_hex(&data[0].from()));

    b.iter(|| { api.get::<serde_json::Value>(ApiKind::Service("timestamp"), &req); });
}
fn bench_search_1000(b: &mut Bencher) {
    // Create testkit for network with four validators
    let mut testkit = TestKitBuilder::validator()
        .with_validators(4)
        .with_service(TimestampService)
        .create();

    // Check results with api
    let api = testkit.api();

    let mut rng = thread_rng();

    let mut data: Vec<TxTimestamp> = Vec::with_capacity(10);

    for _ in 0..1000 {
        let letter: char = rng.gen_range(b'A', b'Z') as char;
        let number: u32 = rng.gen_range(0, 999999);
        let s = format!("{}{:06}", letter, number);

        let keypair = gen_keypair();
        let tx = TxTimestamp::new(&keypair.0, &crypto::hash(s.as_bytes()), &keypair.1);

        data.push(tx);
    }

    for i in 1..1001 {
        api.post::<TxTimestamp, serde_json::Value>(
            ApiKind::Service("timestamp"),
            "v1/submit", &data[i-1]
        );
        if i % 100 == 0 { testkit.create_block(); }
    }

    let req = format!("/v1/timestamp/{}", PublicKey::to_hex(&data[0].from()));

    b.iter(|| { api.get::<serde_json::Value>(ApiKind::Service("timestamp"), &req); });
}

benchmark_group!(benches, bench_submit_10, bench_submit_50, bench_submit_100,
                 bench_submit_200, bench_submit_500, bench_submit_1000,
                 bench_search_10, bench_search_50, bench_search_100,
                 bench_search_200, bench_search_500, bench_search_1000);
benchmark_main!(benches);
