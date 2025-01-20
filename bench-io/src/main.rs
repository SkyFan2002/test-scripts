use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use databend_driver::new_connection;
use env_logger::Env;
use log::info;

/// Bench IO Testing Script
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// number of tables
    #[arg(long)]
    num_tables: u32,

    /// number of rows in each batch
    #[arg(long)]
    batch_size: u32,

    /// number of batches
    #[arg(long)]
    num_batches: u32,
}

const SCHEMA: &str = "(
id bigint,
id1 bigint,
id2 bigint,
id3 bigint,
id4 bigint,
id5 bigint,
id6 bigint,
id7 bigint,

s1 varchar,
s2 varchar,
s3 varchar,
s4 varchar,
s5 varchar,
s6 varchar,
s7 varchar,
s8 varchar,
s9 varchar,
s10 varchar,
s11 varchar,
s12 varchar,
s13 varchar,

d1 DECIMAL(20, 8),
d2 DECIMAL(20, 8),
d3 DECIMAL(20, 8),
d4 DECIMAL(20, 8),
d5 DECIMAL(20, 8),
d6 DECIMAL(30, 8),
d7 DECIMAL(30, 8),
d8 DECIMAL(30, 8),
d9 DECIMAL(30, 8),
d10 DECIMAL(30, 8),

insert_time datetime,
insert_time1 datetime,
insert_time2 datetime,
insert_time3 datetime,

i int
)";

const TABLE_NAME: &str = "bench_io";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Args::parse();
    let Args {
        num_tables,
        batch_size,
        num_batches,
    } = args;
    info!("###options###: \n {:#?}", args);

    let dsn = std::env::var("DATABEND_DSN")
        .map_err(|_| {
            "DATABEND_DSN is empty, please EXPORT DATABEND_DSN=<your-databend-dsn>".to_string()
        })
        .unwrap_or("databend://root:@localhost:8000/default?sslmode=disable&enable_experimental_merge_into=1".to_owned());
    info!("using DSN: {}", dsn);

    create_tables(&dsn, num_tables).await?;

    let start = Instant::now();
    let mut tasks: Vec<_> = Vec::new();
    for i in 0..num_tables {
        let dsn = dsn.clone();
        tasks.push(tokio::spawn(async move {
            let conn = new_connection(&dsn)?;
            for _ in 0..num_batches {
                conn.exec(&format!(
                    "insert into {}_{} select * from source limit {}",
                    TABLE_NAME, i, batch_size
                ))
                .await?;
            }
            Ok::<(), anyhow::Error>(())
        }));
    }
    for task in tasks {
        task.await??;
    }
    let end = Instant::now();
    info!("insert time: {:?}", end.duration_since(start));

    Ok(())
}

async fn create_tables(dsn: &str, num_tables: u32) -> Result<()> {
    let conn = new_connection(dsn)?;
    for i in 0..num_tables {
        conn.exec(&format!(
            "CREATE OR REPLACE TABLE {}_{} {}",
            TABLE_NAME, i, SCHEMA
        ))
        .await?;
    }
    conn.exec(&format!("CREATE OR REPLACE TABLE source {}", SCHEMA))
        .await?;
    Ok(())
}
