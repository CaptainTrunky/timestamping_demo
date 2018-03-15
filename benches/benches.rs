#[macro_use]
extern crate bencher;

extern crate exonum;
extern crate exonum_testkit;

extern crate rand;

extern crate serde_json;

extern crate timestamping;

use exonum::crypto;

use exonum::crypto::gen_keypair;
use exonum_testkit::{ApiKind, TestKitBuilder};

use timestamping::{TimestampService, TxTimestamp};

use rand::{Rng, thread_rng};

use bencher::Bencher;

fn bench_submit_1000(b: &mut Bencher) {
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

benchmark_group!(benches, bench_submit_1000);
benchmark_main!(benches);
