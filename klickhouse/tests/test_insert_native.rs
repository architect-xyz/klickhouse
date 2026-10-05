#[derive(klickhouse::Row, Debug, PartialEq, Clone)]
struct TestRow {
    id: u32,
}

// The server checks constraints only after it has received the data, so a violation arrives
// as an exception after the final block has been written.
#[tokio::test]
async fn insert_native_server_exception() {
    let client = super::get_client().await;

    super::prepare_table(
        "test_insert_native_exception",
        "id UInt32, CONSTRAINT id_small CHECK id < 10",
        &client,
    )
    .await;

    let result = client
        .insert_native_block(
            "INSERT INTO test_insert_native_exception FORMAT Native",
            vec![TestRow { id: 100 }],
        )
        .await;
    assert!(
        matches!(
            result,
            Err(klickhouse::KlickhouseError::ServerException { .. })
        ),
        "{result:?}"
    );

    // The connection is still usable, and the rejected row was not inserted.
    let rows: Vec<TestRow> = client
        .query_collect("SELECT * FROM test_insert_native_exception")
        .await
        .unwrap();
    assert!(rows.is_empty());
}

// Once insert_native returns, the rows are visible from other connections.
#[tokio::test]
async fn insert_native_visible() {
    let client = super::get_client().await;

    super::prepare_table("test_insert_native_visible", "id UInt32", &client).await;

    let rows = vec![TestRow { id: 1 }, TestRow { id: 2 }];
    client
        .insert_native_block(
            "INSERT INTO test_insert_native_visible FORMAT Native",
            rows.clone(),
        )
        .await
        .unwrap();

    let other = super::get_client().await;
    let rows2: Vec<TestRow> = other
        .query_collect("SELECT * FROM test_insert_native_visible ORDER BY id")
        .await
        .unwrap();
    assert_eq!(rows, rows2);
}
