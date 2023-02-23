
// abigen!(
//     LendingPool,
//     "./abi/LendingPool.json",
//     event_derives(serde::Deserialize, serde::Serialize)
// );


//     let pool_contract = LendingPool::new(lending_pool, client.clone());
//     let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
//     let events = pool_contract.events();
//     let filtered = events.from_block(last_block - 10000);
//     let mut stream = filtered.stream().await?;
//     while let Some(log) = stream.next().await {
//         println!("{:?}", &log);
//     }

// fn listen_to_events()  