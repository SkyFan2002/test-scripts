use anyhow::Result;
use databend_driver::Client;
mod util;
use util::ConnectionExt;

const SET_UP: &str = "./sql/setup.sql";
const INSRT_NUM: usize = 1000;

#[tokio::main]
async fn main() -> Result<()> {
    let dsn = std::env::var("BENDSQL_DSN").unwrap();
    println!("dsn: {}", dsn);
    let client = Client::new(dsn);
    let c1 = client.get_conn().await?;
    c1.exec_lines(SET_UP).await?;
    println!("setup done");
    for i in 0..INSRT_NUM {
        println!("insert: {}", i);
        c1.exec("insert into test_order select * from source;")
            .await?;
        println!("insert done: {}", i);
    }
    let block_count: Vec<(u32,)> = c1
        .exec_query("select block_count from fuse_snapshot('default','test_order') limit 1;")
        .await?;
    println!("all insert done, block count: {:?}", block_count);
    let start = std::time::Instant::now();
    c1.exec("optimize table test_order compact").await?;
    println!("compact done, cost: {:?}", start.elapsed());
    let block_count: Vec<(u32,)> = c1
        .exec_query("select block_count from fuse_snapshot('default','test_order') limit 1;")
        .await?;
    println!("block count: {:?}", block_count);

    Ok(())
}

#[tokio::test]
async fn show_blocks() {
    let dsn = std::env::var("BENDSQL_DSN").unwrap();
    println!("dsn: {}", dsn);
    let client = Client::new(dsn);
    let c1 = client.get_conn().await.unwrap();
    let block_count: Vec<(u32,)> = c1
        .exec_query("select block_count from fuse_snapshot('default','test_order');")
        .await
        .unwrap();
    println!("block count: {:?}", block_count);
}
