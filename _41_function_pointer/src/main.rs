use std::fmt::Display;

// function pointer
// EXEC01
fn give_five() -> u8 {
    5
}

fn give_six() -> u8 {
    6
}

fn add_to_func_out(function: fn() -> u8, some_number: u8) {
    let my_num = function();
    let next_num = my_num + some_number;
    println!("We Got {next_num}");
}

// Closure
/*
    fn() -> u8 // ref *Self
    fnMut() -> u8 // can mut *mut Self
    fnOnce() -> u8 // can be used once Self
*/
fn fn_closure<F>(f: F) where F: Fn() {
    f()
}

fn fn_mut_closure<F>(mut f: F) where F: FnMut() {
    f();
}
// 이건 안된다. mut f로 바꿔줘야함.

fn fn__once_closure<F>(f: F) where F: FnOnce() {
    f();
}

// impl trait
// EXEC 03
fn return_impl_closures() -> impl Fn(i32) {
    |x| println!("{x}")
}
fn returns_a_closures() -> Box<dyn Fn(i32)> {
    Box::new(|x| println!("{x}"))
}

// Caller 주도로 함.
fn generic_fn<T: std::fmt::Display>(input: T) {
    println!("{input}");
}

// Function 주도로 함.  -> 진짜 많이 쓰이는 패턴이다.
fn impl_fn(input: impl Display) {
    println!("{input}");
}
// type MyString = impl Display;
// unstable 하고, 불가능하다.

fn main() {
    println!("\nEXEC01");
    // EXEC01 -> fn Pointer
    add_to_func_out(give_five, 5);
    add_to_func_out(give_six, 5);
    // instance를 만들지 않는 게 중요하다. 그냥 pointer만 불러오는게 중요하다.

    println!("\nEXEC02");
    // EXEC02 ->

    let mut my_string = String::from("Hello There");

    let print_it = || {
        println!("{}", &my_string);
    }; // impl Fn()
    print_it();

    fn_closure(|| { println!("{}", &my_string) });
    fn_mut_closure(|| {
        my_string.push('a');
        println!("{my_string}");
        // drop(my_string); drop을 못한다.
    }); // let my_string이면 불가능

    fn__once_closure(|| {
        my_string.push('a');
        println!("{my_string}");
        drop(my_string);
    });

    println!("\nEXEC03");
    // EXEC03 ->
    let my_nums = 9;
    let my_clousre = return_impl_closures();
    my_clousre(my_nums);

    generic_fn::<u8>(12);
    //-> 이건 가능
    // impl_fn::<u8>(12);
    // -> 이건 불가능

}
