// Attribute
// #[~~~]
// #![~~] 전역으로 Attribute주기
// Compiler한테 힌트를 주는 것이다.
// Macro를 만들어야하는데, 겁나 어렵다.

#![allow(dead_code)]
struct JustAStruct {}

struct Thing{}

#[cfg(target_os = "linux")] // -> linux에서만 실행하게 설정 하는것.
fn do_something(){
    println!("I am running in Linux")
}
#[cfg(target_os = "windows")] 
fn do_something(){
    println!("I am running in windows")
}
#[cfg(target_os = "macos")] 
fn do_something(){
    println!("I am running in macos")
}
// #[cfg_attr(predicate, attr)] // 자신만의 환경설정을 만들어주면 된다.

#[test] // Cargo test하면, 그냥 Test를 만들어주고, 일반 run이 없다. 코드만 통과되면 가능이다.
#[should_panic] // Test중에, 
fn tests_a_thing() {
    assert_eq!(8,9); // 이런식으로 비교하는 식으로 한다.
}

// #[derive(debug)] -> impl를 가져오는 것이다.
/// 안녕?
#[deprecated(since = "0.1", note = "Please use the other function now")] // -> not used anymore
pub fn do_a(){
    println!("a")
}

// #[repr]
#[repr(C)] // C언어로 바꿔줘라~
struct SomeRustStruct{
    one: u8,
    two: u16
}

// #[no_mangle] // Change the name a bit
fn some_function_1() {
    
}

fn main() {
    println!("Hello, world!");
    do_something();
    do_a(); 
}
