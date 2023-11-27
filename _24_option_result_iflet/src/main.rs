// OPTION -> Null 대신, 없을수도 있는 값을 표현할 때 쓰는것이다.
// RESULT -> 작동 할수 도 있고, 안될 수 도 있을때 쓰는것.
// OCaml
// if let, while let

// Panicked :

// enum Option<T> {
//     None,
//     Some(T)
// }

use std::num::ParseIntError;

// EXEC01 -> Option을 쓸때는 예외사항을 다 말해주고 써야함.
// .is_some()
// .is_none()
// 로 처리가능
fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 { None } else { Some(value[4]) }
}

// EXEC06 -> RESULT 구문 쓰기
// .is_ok()
// .is_err()
// 로 처리가능
fn check_error(input: i32) -> Result<(), ()> {
    if (input & 2) == 0 { Ok(()) } else { Err(()) }
}

// EXEC08 -> Ok, Err type 지정하기
fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("It's wrong".to_string()),
    }
}

// EXEC09
fn parse_num(num: &str) -> Result<i32, ParseIntError> {
    num.parse()
}

fn main() {
    println!("\nEXEC01");
    // EXEC01 -> OPTION -> Null 대신, 없을수도 있는 값을 표현할 때 쓰는것이다.
    let new_vec_None = vec![1, 2];
    let index_None = take_fifth(new_vec_None);
    println!("{:?}", index_None);

    let new_vec = vec![1, 2, 3, 4, 5, 6];
    let index = take_fifth(new_vec);
    println!("{:?}", index);

    println!("\nEXEC02");
    //EXEC02 -> .unwrap() - take out what is inside
    println!("{}", index.unwrap());

    println!("\nEXEC03");
    //EXEC03 -> match로 더 안전하게 unwrap하기
    let new_vec = vec![1, 2, 3, 4, 5];
    let index = take_fifth(new_vec);
    match index {
        Some(num) => println!("I GOT num: {}", num),
        None => println!("There was noting inside"),
    }

    println!("\nEXEC04");
    //EXEC04 -> .is_some으로 더 간단하게 unwrap 하기
    if index.is_some() {
        println!("I GOT num: {}", index.unwrap());
    }

    println!("\nEXEC05");
    //EXEC05 -> .expect로 디버그 과정에서 확인하고 넘기기
    // index_None.expect("Needed at lease five items");

    println!("\nEXEC06");
    //EXEC06 -> Result구문
    if check_error(6).is_ok() {
        println!("It's OK");
    } else {
        println!("It's an error");
    }

    println!("\nEXEC07");
    //EXEC07 -> Result구문 match로 같이 하기
    match check_error(5) {
        Ok(_) => println!("Ok"), // 이것도 가능
        Err(()) => println!("It's an error"),
    }

    println!("\nEXEC08");
    //EXEC08- > Ok, Err type 지정하기
    let mut result_vec = Vec::new();
    for number in 2..7 {
        result_vec.push(check_if_five(number));
    }
    println!("{:#?}", result_vec);

    println!("\nEXEC09");
    //EXEC09- > Ok, Err type 지정하기
    let mut result_vec = vec![];
    result_vec.push(parse_num("8"));
    result_vec.push(parse_num("String"));
    result_vec.push(parse_num("9"));
    for number in result_vec {
        println!("{:?}", number);
    }

    println!("\nEXEC10");
    //EXEC10 -> .get은 값이 있으면 가져오는것이고 없으면 안가져오는것
    let my_vec = vec![2, 3, 4];
    let get_one = my_vec.get(999);
    match get_one.is_some() {
        true => println!("{}", get_one.unwrap()),
        false => println!("There is nothing"),
    }
    println!("{:?}", get_one);

    println!("\nEXEC11");
    //EXEC11 -> if let을 사용한 것. if let은 가능한 것만 관심이 있고, 아닌것에 관심이 없다.
    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("{}", number);
        }
    }

    println!("\nEXEC12");
    //EXEC12 -> if let을 사용한 것. if let은 가능한 것만 관심이 있고, 아닌것에 관심이 없다.
    let weather_vec = vec![
        vec!["Berlin", "Cloudy", "5", "-7", "78"],
        vec!["Ahtens", "Sunny", "not humid", "-7", "78"]
    ];
    for mut city in weather_vec.clone() {
        println!("For the city of {}", city[0]);
        while let Some(information) = city.pop() {
            if let Ok(number) = information.parse::<i32>() {
                println!("{:?}", number);
            } 
            // else {
            //     println!("{}", information);
            // }
        }
    }

    // 정방향 출력
    for city in weather_vec {
        println!("For the city of {}", city[0]);
        
        for information in city.iter().rev() // -> 역순으로 순회하라는 뜻
        {
            if let Ok(number) = information.parse::<i32>() {
                println!("{:?}", number);
            } 
        }
    }
}
