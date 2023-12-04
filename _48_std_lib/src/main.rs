//
use rand::prelude::*; // 꼭 필요한것만 가져오기

use std::{ str::FromStr, mem::{align_of, size_of_val, size_of, take, transmute } };
//EXEC02
fn four_operations(input: f64) {
    println!(
        "For the number {}:
        floor : {}
        ceiling : {}
        rounded : {}
        truncated: {}\n",
        input,
        input.floor(),
        input.ceil(),
        input.round(),
        input.trunc() // cut off - 버림
    );
}

//EXEC04
#[derive(Debug)]
struct NeedAStatic {
    name: &'static str,
}

fn get_our_data() -> String {
    "Data".to_string()
}

// EXEC 06
struct MyStruct {
    bunch_of_stuff: [u32; 1000],
}

struct MyStructHeap {
    bunch_of_stuff: Box<[u32; 1000]>,
}

fn main() {
    println!("\nEXEC01");
    // EXEC01 -> unicode 확인하기
    let korean_word = "청춘예찬";
    for c in korean_word.chars() {
        print!("{}", c.escape_unicode());
    }
    let c_char = char::from(99);
    println!("{c_char}");
    let mut rand_gen = rand::thread_rng();
    for _ in 0..1 {
        let bigger_char = char::try_from(rand_gen.gen_range(u32::MIN..u32::MAX));
        print!("{bigger_char:?}");
    }

    println!("\nEXEC02");
    // EXEC02 -> 숫자 타입
    let first_num = 200u8; // 256이하만 가능 u8은
    println!("{:?}", first_num.saturating_add(200));
    four_operations(9.1);
    four_operations(10.1);
    four_operations(-1.1);

    println!("\nEXEC03");
    // EXEC03 -> bool 타입
    println!("{:?}", bool::from_str("false"));
    println!("{:?}", "true".parse::<bool>());
    // println!("{:?}",true.then(||{
    //     "abc"
    // })); //Option
    println!("{:?}", true.then_some("abc")); //Option

    // let bool_vec = vec![true,false,true,false,false];
    let bool_vec = [true, false, true, false, false]; // 도 가능!

    let option_vec = bool_vec
        .iter()
        .map(|item| {
            item.then(|| {
                println!("Got a {item}");
                "It's true"
            })
        })
        // .filter_map(|c| c)
        .collect::<Vec<_>>();
    println!("{option_vec:?}");

    println!("\nEXEC04");
    // EXEC04 -> leak
    let our_data = get_our_data();
    let boxed_data = Box::new(our_data); // BOX<String>
    let leaked_data = Box::leak(boxed_data); // &'static str

    let our_struct = NeedAStatic {
        name: leaked_data,
    };

    println!("{our_struct:?}");

    println!("\nEXEC05");
    // EXEC05 -> Vec
    let mut my_vec = vec![100, 20, 30, 40, 0, 0, 0];
    my_vec.sort(); // [0, 0, 0, 20, 30, 40, 100]
    println!("{my_vec:?}");
    my_vec.sort_unstable(); // 직접적인 값을 가져오지 않지만, 속도가 무지빠르다.
    println!("{my_vec:?}");

    let mut my_vec = vec!["sun", "moon", "moon", "moon", "sun", "moon"];
    my_vec.dedup(); // ["sun", "moon", "sun", "moon"] 옆에 겹친것만 지운다.
    println!("{my_vec:?}");
    my_vec.sort_unstable();
    my_vec.dedup(); // 이러면 모든 중복 지움;
    println!("{my_vec:?}");

    let mut my_vec = vec![10; 10000]; // push()
    println!("{}", my_vec.capacity()); // Vec::with_capacity(10001);
    my_vec.push(9);
    println!("{}", my_vec.capacity());
    my_vec.shrink_to_fit(); // 최대한 데이터 용량을 축소 시키게 하는것.
    println!("{}", my_vec.capacity());
    let mut vec = vec![1, 2, 3];
    vec.insert(1, 4);
    println!("{:?}", vec);

    let mut my_vec = vec![8, 9, 10, 11];
    my_vec.retain(|num| num > &9); // 조건 충족해야지만, 남긴다. 직접변형을 시킴
    println!("{my_vec:?}");

    let mut my_string = String::from("hello there");
    let second_string = my_string.split_off(5); // 직접 짜르고 값을 넣을 수 있다.
    println!("{my_string}, {second_string}");

    println!("\nEXEC06");
    //EXEC06 -> mem
    println!(
        "{},{},{}",
        size_of::<MyStruct>(),  // 4000 바이트
        size_of::<MyStructHeap>(), // 8 바이트
        size_of_val("I am a &str") // 11 바이트
    ); // 타입의 크기를 확인해볼수 있다.
    println!("{}",align_of::<MyStruct>());  // 특정 데이터 타입이 메모리에서 차지하는 영역을 정렬하는 방법
    println!("{}",align_of::<MyStructHeap>()); // 특정 데이터 타입이 메모리에서 차지하는 영역을 정렬하는 방법

    let mut my_string = "I am a String".to_string();
    let take_thing = take(&mut my_string); // 소유권 강제로 가져가게 만들기
    println!("{}, old String: {}", take_thing, my_string);

    let my_numbers = [8u8, 9, 10,11]; // [us; 4]
    let new_number =
    unsafe {
    transmute::<[u8; 4], i32>(my_numbers)  // 같은 크기의 메모리의 특정타입으로 바꾸게 함. 안하는게 맞음
    };
    println! ("{new_number}");

    println!("\nEXEC07");
    //EXEC07 -> Mecro
    println!("{}, {}, {}, {}", column!(),line!(),file!(), module_path!());

    let my_file = include_str!("./docs/my_file.txt");
    println!("{my_file}");

    // let some_struct = Nothing;
    let my_string_slice = concat!('a',true,7.7,"Hello there"); // 타입 상관없이 이어붙이는 Macro
    println!("{my_string_slice}");
}
