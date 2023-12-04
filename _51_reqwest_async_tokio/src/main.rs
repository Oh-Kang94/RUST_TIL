use std::time;

use serde::{ Serialize, Deserialize };
use tokio::{ join, select };

#[derive(Serialize, Deserialize, Debug)]
struct Result {
    message: String,
    result: Vec<Auctions>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Auctions {
    auctionid: u64,
    seller_id: String,
    title: String,
    content: String,
    pic: String,
    fish: String,
    view: u64,
    pricestart: u64,
    pricenow: u64,
    insertdate: String,
    deletedate: Option<String>, // 이 필드는 null을 포함할 수 있으므로 Option으로 정의
    issuccessed: bool,
}

// EXEC03
// impl Future
fn give_u8() -> u8 {
    8
}
async fn async_give_u8() -> u8 {
    8
}

// EXEC04
async fn sleep(duration: u64) {
    std::thread::sleep(time::Duration::from_millis(duration));
}

async fn give_data() -> u8 {
    sleep(1000).await;
    7
}

async fn give_data_again() -> u8 {
    sleep(1000).await;
    8
}
async fn give_str() {
    sleep(1000).await;
    println!("DATA FOUND")
}
async fn give_err() {
    sleep(1000).await;
    println!("ERROR")
}

#[tokio::main]
async fn main() {
    println!("\nEXEC01");
    // let body = reqwest::get("https://final.oh-kang.kro.kr/auctions/")
    //     .await?
    //     .text()
    //     .await?;
    // println!("{body:?}");

    println!("\nEXEC02");
    // let our_auctions: Result = serde_json::from_str(&body).unwrap();
    // println!("{our_auctions:#?}");

    println!("\nEXEC03");
    let my_num = give_u8();
    let my_num_async = async_give_u8().await;
    // my_num_async.await; 이걸 tokio runtime없이는 구동이 불가

    println!("{}", my_num_async);

    println!("\nEXEC04");
    let now = time::Instant::now();
    println!("{:?}", now.elapsed());
    for _ in 0..10000 {
        let x = 7;
    }
    println!("{:?}", now.elapsed());
    let num_one = give_data().await; // didn't poll yet
    let num_two = give_data_again().await;
    println!("{}", num_one);
    println!("{}", num_two);
    println!("{:?}", now.elapsed());

    println!("\nEXEC06");
    let now = time::Instant::now();
    let num_one = give_data(); // didn't poll yet
    let num_two = give_data_again();
    let (num_one, num_two) = join!(num_one, num_two); // join! 은 2개 다 받고 싶고, 끝날때 다음 명령 해야하는것.

    println!("{:?}", now.elapsed());

    println!("\nEXEC07"); // SELECT는 동시에 한것중 먼저끝나는것에 대해서 결과를 뱉는다.
    for _ in 1..10 {
        let now = time::Instant::now();
        select! {
        data = give_str() => data,
        error = give_err() => error
        }
        println!("{:?}", now.elapsed());
    }
}
