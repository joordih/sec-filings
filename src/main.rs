use chrono::NaiveDate;
use chrono_tz::America::New_York;
use secfilings::miner::Miner;
#[tokio::main]
async fn main() {
    let start = chrono::Utc::now()
        .with_timezone(&New_York)
        .date_naive();

    let end = NaiveDate::from_ymd_opt(2020, 1, 4).unwrap();
    let mut miner = Miner::new(&start);

    loop {
        if miner.mine_date == end {
            println!("Stop date reached {:?}", end);
            break;
        }

        miner.run(8).await;
    }
}