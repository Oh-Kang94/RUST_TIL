// #![feature(panic_internals)]
// Panic Hook
use chrono::{Utc, TimeZone};
use chrono_tz::Asia::Seoul;
use std::{panic::{set_hook, take_hook}, time::{Instant, Duration}, thread::sleep}; // take_hook은 Hook을 취소

// PanicHOOK의 사용 방법은
// const IMPORTANT_INFO: Vec<i32> = vec![200,300,400];

fn main() {
    println!("\nEXEC01");
    // EXEC01 ->

    // let mut important_code = 400;

    // set_hook(
    //     Box::new(|panic_info| {
    //         // panic_info.set_payload(&200);
    //         println!("It's not a 200 code");
    //         println!("don't forget about x: {:?}", panic_info.payload().downcast_ref::<&str>().unwrap());
    //     })
    // );
    // let my_num = "ok-kang94".parse::<i32>().unwrap();
    // important_code = 200;
    // let _ = take_hook();
    // let other_num = "123asd".parse::<i32>().unwrap();
    // panic!("Oh the humanity");

    println!("\nEXEC02");
    // EXEC02 -> Time
    let now = Instant::now();
    println!("{now:?}");
    let time1 = Instant::now();
    for _ in 0..1000{
        let _ = String::from("Iam a String to keep you busy");
    }
    let time2 = Instant::now();
    println!("{:?}", time2 - time1);

    let time1 = Instant::now();
    sleep(Duration::from_secs(1));
    println!("{:?}", time1.elapsed()); // elapsed = Time that passed
    println!("{:?}", time1.elapsed()); // elapsed = Time that passed
    println!("{:?}", time1.elapsed()); // elapsed = Time that passed

    let now = Utc::now().with_timezone(&Seoul);
    let now_show = &now.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    // 특정 시간
    let dt = Utc.with_ymd_and_hms(2014, 11, 28, 12, 0, 9).unwrap();
    // 오후/오전 표시 포함하여 시간 출력
    let formatted_time = &now.format("%Y-%m-%d %I:%M:%S %p %Z").to_string();
    println!("{dt:#?}");
    println!("{now:#?}");
    println!("{now_show:#?}");
    println!("오전 오후 서울 시간 : {formatted_time:#?}");
}
