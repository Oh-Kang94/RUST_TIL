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
    compare_print("Listen up!", 9, 8)
}
