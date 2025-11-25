#![cfg(feature = "rust_decimal")]
use rust_decimal::{prelude::FromPrimitive, Decimal};

#[derive(klickhouse::Row, Debug, Default, PartialEq, Clone)]
pub struct TestType {
    d_d128: Decimal,
}

#[tokio::test]
async fn test_client() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init();
    let client = super::get_client().await;

    super::prepare_table("test_decimal", "d_d128 Decimal128(5) default 0", &client).await;

    println!("begin insert");

    let block = TestType {
        d_d128: Decimal::new(12345, 2),
    };

    let block2 = TestType {
        d_d128: Decimal::from_f64(0.05).unwrap(),
    };

    client
        .insert_native_block(
            "INSERT INTO test_decimal (d_d128) FORMAT Native",
            vec![block.clone(), block2.clone()],
        )
        .await
        .unwrap();

    let answer_block = client
        .query_collect("SELECT * from test_decimal ORDER BY d_d128 DESC")
        .await
        .unwrap();
    assert_eq!(vec![block, block2], answer_block);

    println!("done");
}
