use serde::{ Serialize, Deserialize };
use anyhow::{ anyhow, Error as AnyhowError };
use serde_json::{Result as JSONResult, Value};
// SERIALIZE -> TURN INTO JSON, YANL, ETC
// DESERIALIZE -> TURN FROM JSON, YANL, ETC

//Rand
use rand::{self, thread_rng, Rng};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    points: u32,
    age: u8,
}
#[derive(Debug, Serialize, Deserialize)]
struct UserRequest {
    points: u32,
    age: u8,
}

impl User {
    fn from_request(request: UserRequest) -> Result<User, AnyhowError> {
        if request.age < 120 && request.points < 10000 {
            Ok(User { points: request.points, age: request.age })
        } else {
            Err(anyhow!("User is bad"))
        }
    }
}
fn main() {
    println!("\nEXEC01");
    // EXEC01 -> Serde
    let request = r#"
    {
        "points":1000,
        "age" : 110
    }
    "#;

    let user_1: User = serde_json::from_str(&request).unwrap();
    println!("{:#?}", user_1);
    
    let user_request: UserRequest = serde_json::from_str(&request).unwrap();
    let user_try = User::from_request(user_request);
    println!("{user_try:?}");


    let request_json = r#"
    {
        "points":1000,
        "age" : [110, 11]
    }
    "#;
    let v: Value = serde_json::from_str(request_json).unwrap();
    println!("{}\n{}", v["points"], v["age"][1]);

    println!("\nEXEC02");
    // EXEC02 -> Rand
    for _ in 0..5{
        let random_u16 = rand::random::<u16>();
        let random_char = rand::random::<char>();
        println!("{random_u16}\n{random_char}")
    }

    print!("\nEXEC03");
    // EXEC03 -> thread_rng
    let mut rng = thread_rng(); // 현재 스레드에 안전한 난수 생성기
    for _ in 0..5{
        println!("{}",rng.gen_range(1..11));
    }  

}
