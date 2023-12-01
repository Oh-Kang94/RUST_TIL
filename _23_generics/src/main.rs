use std::fmt::{ self, Display };
//EXEC02
struct Book {
    year: i32,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.year)
    }
}

fn give_book<T: fmt::Display>(input: T) -> T {
    input
}
//EXEC01
fn give_item<T>(input: T) -> T {
    input
}

// EXEC03
fn compare_print<T: Display, U: Display+PartialOrd>(stmt: T, num1: U, num2: U) {
    println!("{}!, Is {} Greater than {},? {}",
        stmt,
        num1,
        num2,
        num1>num2
    );
}
// EXEC03-reformatted
fn compare_print_reformatted<T, U>(stmt: T, num1: U, num2: U) 
where 
    T:Display, 
    U: Display+PartialOrd{
    println!("{}!, Is {} Greater than {},? {}",
        stmt,
        num1,
        num2,
        num1>num2
    );
}

//EXEC04
// Concrete Type(Compile Time) vs Dynamic(=dyn)(Runtime)
// Runtime은 프로그램이 실행되고 있는 환경 또는 동작되는 동안
// Compile 개발자가 프로그램을 위해 작성한 소스코드를 다른 프로그램이나 기계(H/W)가 처리하기 용이한 형태로 바꾸는 과정

// concrete
fn print<T>(input:T) where T : Display{
    println!("Hi, I am a {input}")
}
// concrete
fn print_2(input: impl Display){
    println!("hi i am a {input}")
}

// dynamic
fn print_3(input: Box<dyn Display>){
    println!("hi i am a {input}")
} 



fn main() {
    println!("\nEXEC01");
    // EXEC01 ->  Generic type을 주는것.
    let x = give_item(String::from("Take this"));
    let y = give_item(9);
    println!("{}", x);
    println!("{}", y);

    println!("\nEXEC02");
    // EXEC02 ->  Generic type을 주는것.
    let z = give_book(Book { year: 2023 });
    println!("{}", z); // 이제 Book 구조체를 Display로 출력할 수 있습니다.

    println!("\nEXEC03");
    // EXEC03 ->  Generic type을 두개 이상 정하고, 변수마다 다른 타입 만들어서 지정해주기
    compare_print("Listen up!", 9, 8);

    println!("\nEXEC04");
    // EXEC04 ->  Generic type을 두개 이상 정하고, 변수마다 다른 타입 만들어서 지정해주기
}
