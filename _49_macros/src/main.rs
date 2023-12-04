// EXEC01
macro_rules! give_six {
    () => {
        6
    };
}

macro_rules! six_or_print {
    (6) => {
        6
    };
    () => {
        println!("You didn't give me six")
    };
}

// EXEC02
macro_rules! might_print {
    ($input:expr) => {  // ->Expression을 말하는것.
        println!("{}",$input);
    };
}
macro_rules! check {
    (
        $input1:ident,
        $input2:expr
    ) => { // ident는 변수명, 함수명, 구조체 명을 넣어야한다.
        println!("Is {:?} equal to {:?}? {}", $input1, $input2, $input1 == $input2)
    };
}
macro_rules! print_any {
    (
        $input:tt,
        $input2:tt
    ) => { //&(x) , *
        let output = stringify!($input);
        let output2 = stringify!($input2);
        println!("{}, {}",output, output2)
    };
}
macro_rules! print_any2 { // tt는 토큰트리를 말하고, 토큰을 처리할때 쓴다.
    (
        $($input:tt),+
    ) => { //&(x) , *
        let output = stringify!($($input:tt), +);
        println!("{}",output)
    };
}

macro_rules! make_a_fn {
    ($name:ident, $($input:expr),*) => {
        fn $name() {
            let output = format!("{:?}", ($($input),*));
            println!("{}", output);
        }
    };
}

macro_rules! my_macro { // Macro는 자기 자신을 Callable할 수 있다. 
    () => {
        println!("Let's print this;")
    };
    ($input:expr) => {
        my_macro!();
    };
    ($($input:expr),*) => {
        my_macro!();
    };
}

fn main() {
    println!("\nEXEC01");
    // EXEC01 -> Macro 기본
    let six = give_six!();
    println!("{six}");
    println!("{}", six_or_print!(6));
    six_or_print!();

    println!("\nEXEC02");
    // EXEC02 -> Macro 기본
    might_print!("assd");

    let x = 8;
    check!(x, 9);
    print_any!(2131kasdnfsad안, asdasdsads);
    print_any2!(2131kasdnfsad안, asdasdsads);
    make_a_fn!(print_it, 6, 7, 8, "asd");
    print_it();

    println!("\nEXEC03");
    // EXEC02 -> Macro 기본
    my_macro!(vec![8,7,3,21]);
    dbg!(vec![1,2,3]);
}
