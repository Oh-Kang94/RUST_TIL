// type generics - generic over a type
// const generics - generic over a const

// const context-> 컴파일 타임에 평가되는 상수

use std::time::Duration;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;

// EXEC01
struct SomeVecs<T> {
    vec_1: Vec<T>,
    vec_2: Vec<T>,
}

#[derive(Debug)]
struct SomeArray<T, const N: usize> { // const generics을 쓰면 편하다.
    // array_1: [T; 3], 
    // array_2: [T; 4],
    // -> 원래는 이렇게 적게 하였지만, Const Generics가 나오면서,
    array_1: [T; N], 
    array_2: [T; N],
    // 이렇게 많이 쓴다.
}

//EXEC02 
const fn get_nums(input:i32) -> i32 {
    if input >10 {
        9
    }else{
        10
    }
}

const MY_NUMS: i32 = get_nums(10);

const fn get_my_duration(seconds : u64, nanos : u32)-> Duration{
    Duration::new(seconds, nanos)
}

const MY_DURATION : Duration = get_my_duration(100, 10);


const fn check(){
    // let my_vec = vec![8,7,5]; 
    // 이건 안됨, Vec은 Heap에 관리되어야 하는데, Const는 컴파일때 모든게 완성되어,
    // HEAP을 쓰지 않음

    let my_array = [9,9,10]; // -> 그래서 Array를 써야함.
}

// EXEC03

// const ERROR_LISTRENRER : ErrorListener = ErrorListener{
//     url : "www.oh-kang.kro.kr".to_string() 
//     // 이거는 non const이기 떄문에 불가. 그래서, lazy Static을 쓴다.
//     // Allocation은 안됬지만, 그냥 있다가 사용하게 되면 바로 일어난다.
// };

// lazy_static!{
    //     static ref ERROR_LISTRENER: ErrorListener = ErrorListener {
        //         url : "www.oh-kang.kro.kr".to_string() 
        //     };
        // }
        
// EXEC05 -> OnceCell
static ERROR_LISTRENER: OnceCell<ErrorListener> = OnceCell::new();

#[derive(Debug)]
struct ErrorListener{
    url : String, 
}

impl ErrorListener { // ErrorListener::get_listener()
    fn check_for_error(&self) -> Result<(),()>{
        println!("Checking for error");
        Ok(())
    }
    // fn get_listener() -> &'static ErrorListener{
    //     ERROR_LISTRENER.get().expect("Where is ErrorListenr");
    // }
}

fn do_stuff() {
    // ERROR_LISTRENER.check_for_error();
    // let listener = ErrorListener::get();
    // listener.check_for_error();
}
fn check_sth_else(){
    // ERROR_LISTRENER.check_for_error();
}



fn main() {
    println!("\nEXEC01");
    // EXEC01 -> Const Generics
    let my_vecs = SomeVecs {
        vec_1: vec![6, 7, 8],
        vec_2: vec![1, 2, 123, 12],
    };
    let my_arr = SomeArray {
        array_1: [3; 3],
        array_2: [16; 3],
    };
    println!("{my_arr:?}");
    println!("\nEXEC02");
    // EXEC02 -> Const Fn
    let my_fn = MY_NUMS;
    println!("{my_fn}");

    println!("{MY_DURATION:#?}");

    println!("\nEXEC03");
    // EXEC03 -> Lazy Static
    do_stuff();
    check_sth_else();

    println!("\nEXEC04");
    // EXEC03 -> Lazy Static
    // ErrorListener.set(ErrorListener{
    //     url : "www.oh-kang.kro.kr".to_string()
    // }).expect("Couldnt set ErrorListener");
}
