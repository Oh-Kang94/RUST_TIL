// todo!() macro

// type alias
use std::borrow::Cow;
// EXEC01
type Mystring = String; // -> 그냥 내가 타입 이름을 바꾸는 것

type SkipType = std::iter::Take<std::iter::Skip<std::vec::IntoIter<char>>>;
fn skip_five_take_five(input: Vec<char>) -> SkipType {
    // skip type alias안하면, 위에있는걸 다 적어야한다.
    input.into_iter().skip(5).take(5)
}

//EXEC02
//todo!() - I'll do it later
struct SomeType {
    name: String,
    num: u8,
}
// fn some_func(input: SomeType) -> Vec<SomeType> {
//     todo!() // -> 일단 compile은 되게 만들어줘서 중간 디버깅할때 쓴다.
// }

// Cow -> Clone On Write : convinent enum


// pub enum Cow<'a, B> where B: 'a + ToOwned + ?Sized {
//     Borrowed(&'a B),
//     ToOwned(<B as ToOwned>::Owned),
// }

fn module_3(input: u8) -> Cow<'static, str> {
    match input % 3 {
        0 => "Remainder is 0".into(),
        1 => "Remainder is 1".into(),
        remainder => format!("Remainder is {remainder}").into(),
    }
}

trait MyDefault {
    fn new(input: &str)-> Self;
}

struct User {
    name: String,
}

impl MyDefault for User {
    fn new(input: &str) -> Self{
        Self { name: input.to_string() }
    }
}
// 위에가 기본적인 trait를 이용한 구조 하지만,
#[derive(Debug)]
struct MyUserCow<'a> {
    name : Cow<'a, str>
}


// EXEC05
#[derive(Debug)]
struct MyUser<'a> {
    name : Cow<'a,str>
}

impl MyUser<'_> {
    fn is_borrowed(&self){
        match &self.name{
            Cow::Borrowed(name) => println!("It's borrowed: {name}"),
            Cow::Owned(name) => println!("It's owned: {name}")
        }
    }
}


fn main() {
    println!("\nEXEC01");
    // EXEC01 -> type alias;
    let my_string = Mystring::from("Some &str");
    println!("\nEXEC02");
    // EXEC02 -> todo!()
    // some_func(SomeType { name: "12".to_string(), num: 3 });

    println!("\nEXEC03");
    //EXEC03 -> Cow
    // AWS Region 선택할때 나온다.
    for number in 1..=6 {
        match module_3(number) {
            Cow::Borrowed(message) => println!("The Cow is borrowed with {message}"),
            Cow::Owned(message) => println!("The Cow is owned with {message}"),
        }
    }

    println!("\nEXEC04");
    //EXEC04 -> Cow
    let my_string = "Hello there".to_owned();

    // 다 각기 다른 방식의 Trait으로 String을 만들 수 있다.
    let string_1 = String::from("Hello"); // From trait
    let string_2 = "Hello".to_string(); // Display trait
    let string_3: String = "Hello".into();
    let string_4 = "Hello".to_owned(); // ToOwned trait

    let user_1 = "User 1";
    let user_2 = "User 2".to_string();

    // 이렇게 적어도 된다.
    let my_user1 = User::new(user_1);
    let my_user2 = User::new(&user_2);

    // 하지만, Cow를 이용하면,
    let my_user1 = MyUserCow{
        name: user_1.into()
    };
    let my_user2 = MyUserCow{
        name: user_2.into()
    };
    println!("User 1 = {my_user1:?}, User 2= {my_user2:?}");

    //EXEC05
    let mut user_1 = MyUser{
        name : "User 1".into()
    };
    let user_2 = MyUser{
        name : "User 2".to_string().into()
    };
    user_1.is_borrowed();
    user_1.name.to_mut().push('!');

    user_1.is_borrowed();


}
