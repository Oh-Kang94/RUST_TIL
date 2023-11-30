// todo!() macro

// type alias

// EXEC01
type Mystring = String; // -> 그냥 내가 타입 이름을 바꾸는 것

type SkipType = std::iter::Take<std::iter::Skip<std::vec::IntoIter<char>>>;
fn skip_five_take_five(input : Vec<char>) -> SkipType{ // skip type alias안하면, 위에있는걸 다 적어야한다.
    input.into_iter().skip(5).take(5)
}

//EXEC02
//todo!() - I'll do it later
struct SomeType{
    name : String,
    num : u8,
}
fn some_func(input : SomeType) -> Vec<SomeType>{
    todo!() // -> 일단 compile은 되게 만들어줘서 중간 디버깅할때 쓴다.
}

// Cow
pub enu

fn main() {
    println!("\nEXEC01");
    // EXEC01 -> type alias;
    let my_string = Mystring::from("Some &str");
    println!("\nEXEC02");
    // EXEC02 -> todo!()
    some_func(SomeType { name: "12".to_string(), num: 3 });
}
