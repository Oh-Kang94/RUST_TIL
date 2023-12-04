// Any는 'static을 제외하고 모든 타입을 지원한다.
// Channels는 Mutex와 같은 Multi Thread 관련 이다.
// MPSC : Multiple Produce Single Consumer

// Anyhow : 아무거나 에러가 잠시 필요하는데 쓰는거
// thiserror : ERROR를 메크로로 만들어, Attribute를 부여 해서 할 수 있다.

use std::{ any::{ Any, type_name }, sync::mpsc::channel, thread::{ self, sleep }, time::Duration, num::{ParseIntError, ParseFloatError}, fmt::Display, io };
use anyhow::{anyhow, Error , Context};
use thiserror::Error;
// 이런식으로 Logger를 만들때 쓰인다.
struct MyLogger<'a>(Vec<&'a dyn Any>);

struct MyType;

fn get_type_name<T: Any, U: Any>(_: T, _: U) {
    let t_type = type_name::<T>();
    let u_type: &str = type_name::<U>();
    println!("First type:{t_type}");
    println!("Second type:{u_type}");
}

fn do_stf_depending(input: &dyn Any) {
    if input.is::<String>() {
        println!("We Got a String")
    } else if input.is::<i32>() {
        println!("We Got a number")
    } else {
        println!("IDK")
    }
}

fn try_get_string(input: &dyn Any) {
    if let Some(a_string) = input.downcast_ref::<String>() {
        // REF를 아
        println!("We Got a String, {}", a_string)
    } else {
        println!("We got sth else")
    }
}


// EXEC04
fn sleepy(time: u64) {
    sleep(Duration::from_millis(time));
}


// EXEC05
#[derive(Debug)]
struct Book {
    name: String,
}
#[derive(Debug)]
struct Megazine {
    name: String,
}

fn book() -> Box<dyn Any + Send> {
    // => Box라는 Smart Pointer를 이용
    let book = Book { name: "my Book".to_string() };
    return Box::new(book);
}

fn megazine() -> Box<dyn Any + Send> {
    let meagzine = Megazine { name: "my megazine".to_string() };
    return Box::new(meagzine);
}

// EXEC06
// ERROR 만들기
#[derive(Debug)]
enum CompanyError{
    CouldntConnect,
    NotEnoughData,
    UserTimedOut
}

impl Display for CompanyError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Got a CompayError")
    }
}
// impl Error for CompanyError{}

impl CompanyError {
    fn print_extra_detail(&self){
        println!("ERROR MESSAGE SHOW");
    }
}

#[derive(Debug)]
struct BaseError;

impl Display for BaseError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Got a BaseError")
    }
}

// impl Error for BaseError{
    
// }

fn give_error(is_company_error:bool) -> Result<(), Box<dyn Any>>{
    if is_company_error {
        Err(Box::new(CompanyError::CouldntConnect))
    } else{
        Err(Box::new(BaseError))
    }
}


// EXEC07 
fn try_to_make_number(int_input: &str, float_input: &str)-> Result<(), Error>{
    let my_num = int_input.parse::<i32>().with_context(|| "Extra info is here"); // OK(8) else retrun Err
    let other_num = float_input.parse::<f32>().with_context(|| "Extra info is here"); // ParseFloatError를 기대한다. -> 그래서 ,ENUM을 만든다.

    // let x = 9;
    // if x ==9 {
    //     return Err(anyhow!("x shouldnt be 9"));
    // }
    Ok(())
}

#[derive(Debug)]
struct User{
	points: u32,
	age : u8
}

impl User{
	fn try_new(age:u8, points:u32) -> Result<Self, UserError>{
		use UserError::*;
		match (age, points){
            (age, points) if age>120 && points > 10000 => {
                Err(TooBigandOld(User{age, points}))
            },
            (_,p) if p >10000 => {
                Err(TooBig(p))
            },
            (a,_) if a>120 => {
                Err(TooOld(a))
            }
            _ => Ok(Self { points, age})
		}
	}
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("data store disconnected")]
    NotEnoughData,
    #[error("Too old: `{0}` Can't be over 120")]
    TooOld(u8),
    #[error("Too old: `{0}` Can't be over 10000")]
    TooBig(u32),
    #[error("Too old: Can't be over 120")]
    TooBigandOld(User),
}

fn main() {
    println!("\nEXEC01");
    // EXEC01 ->
    get_type_name(8, true);
    get_type_name(vec![8], MyType);

    println!("\nEXEC02");
    // EXEC02 ->
    do_stf_depending(&8);
    do_stf_depending(&String::from("abc"));
    do_stf_depending(&'a');

    println!("\nEXEC03");
    // EXEC03 ->
    try_get_string(&4);
    try_get_string(&"Hello".to_string());

    println!("\nEXEC04");
    // EXEC04 -> Channels
    let (sender, receiver) = channel();
    sender.send(9).unwrap();
    let received = receiver.recv().unwrap();
    println!("received : {received}");

    let s1 = sender.clone();
    let s2 = sender.clone();

    thread::spawn(move || {
        // take by value
        sleepy(1000);
        s1.send(9).unwrap();
    });
    thread::spawn(move || {
        // take by value
        sleepy(1000);
        s2.send(9).unwrap();
    });
    // println!("s1 : {:?}", receiver.try_recv());
    // println!("s2 : {:?}", receiver.try_recv());
    // println!("s1 : {}", receiver.recv().unwrap());
    // println!("s2 : {}", receiver.recv().unwrap());
    println!("s1 : {:?}", receiver.recv_timeout(Duration::from_millis(500)));
    println!("s2 : {:?}", receiver.recv_timeout(Duration::from_millis(500)));

    println!("\nEXEC05");
    // EXEC05 -> AnyType with Channels
    let (sender, receiver) = channel();
    let s1 = sender.clone();
    let s2 = sender.clone();

    thread::spawn(move || {
        // take by value
        for _ in 0..5 {
            sleepy(100);
            s1.send(book()).unwrap();
        }
    });
    thread::spawn(move || {
        // take by value
        for _ in 0..5 {
            sleepy(100);
            s2.send(megazine()).unwrap();
        }
    });

    // while let Ok(any_type) = receiver.recv(){
    //     println!("{:?}", any_type)
    // }
    while let Ok(any_type) = receiver.recv_timeout(Duration::from_secs(3)) { 
        // TimeOut을 설정안하면 프로그램이 죽을때까지 대기만 한다.
        if let Some(book) = any_type.downcast_ref::<Book>() {
            println!("GOT a BOOK: {book:?}");
        } else if let Some(megazine) = any_type.downcast_ref::<Megazine>() {
            println!("GOT a BOOK: {megazine:?}");
        } else {
            panic!("Expected a megazine or a book");
        }
    }

    println!("\nEXEC06");
    // EXEC06 -> Error
    let error_1 = give_error(true);
    let error_2 = give_error(false);

    println!("{error_1:?}, {error_2:?}");
    // if let Some(company_error) = error_1.downcast_ref::<CompanyError>(){
    //     company_error.print_extra_detail();
    // }else{
    //     println!("{error_1}");
    // }

    println!("\nEXEC07");
    // EXEC07 -> BoilerPlate 깔기 귀찮으니 Crates의 힘을 받자.
    // ANYHOW

    let first_try = try_to_make_number("8", "asda");
    let second_try = try_to_make_number("asda", "10.9");
    println!("{first_try:?}");
    println!("{second_try:?}");


    println!("\nEXEC08");
    // EXEC08
    // thiserror
    let user_requests = vec![
        User::try_new(150, 20000),
        User::try_new(100, 20000),
        User::try_new(200, 1000),
        User::try_new(40, 5000),
    ];
    println!("{:#?}", &user_requests);

    let users = user_requests
        .into_iter()
        .filter_map(|user_request| match user_request{
            Ok(user) =>  Some(user),
            Err(message) => {
                println!("{message}");
                None
            } 
        })
        .collect::<Vec<User>>();

    print!("{users:?}")
}
