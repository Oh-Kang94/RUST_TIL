//
fn match_colors(rgb: (u32, u32, u32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("Not red"),
        (_, g, _) if g < 10 => println!("Not Green"),
        (_, _, b) if b < 10 => println!("Not Blue"),
        _ => println!("Every Color has at least 10"),
    }
}

fn match_num(input: u32) {
    match input {
        0..=10 => println!("It's between 0 and 10. It's {}", input),
        _ => println!("It's greater than 10"),
    }
}
fn match_num2(input: u32) {
    match input {
        // num => println!("It's number {}",num), // 첫번째 조건이 다 충족하면 아래 조건들은 없게 되니까 안된다.
        num @ 0..=10 => println!("It's between 0 and 10. It's {}", num),
        _ => println!("It's greater than 10"),
    }
}
fn main() {
    let my_num: i32 = 5;
    let my_second_num: i32 = 15;
    if my_num == 5 && my_second_num == 15 {
        println!("맞음!");
    } else if my_num == 5 || my_second_num == 15 {
        println!("하나만 맞음");
    } else {
        println!("틀림!");
    }

    // match
    // Switch 랑 비슷 하지만, 더 좋은 성능과 기능이 있음
    let my_number = 5;
    match my_number {
        0 => println!("0"),
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("없어"), // '_'는 아무거나 들갈 수 있다.
    }
    // Expression Language의 특성을 이용하는 법
    let second_number = match my_number {
        0 => 23,
        1 => 65,
        _ => 0,
    };
    println!("{}", second_number);

    //
    let sky = "cloudy";
    let temperature = "warm";
    match (sky, temperature) {
        ("cloudy", "warm") => println!("It's perfect"),
        ("cloudy", "as") => println!("wrong"),
        ("cloudy", _) => println!("wrong"),
        (_, _) => println!("fail"),
    }

    let children = 5;
    let married = true;
    match (children, married) {
        (children, married) if married == false =>
            println!("Not married with {} children", children),
        (c, m) if c == 0 && m => println!("Married but with no children"),
        _ => println!("Nothing"),
    }

    let first: (u32, u32, u32) = (200, 0, 10);
    let second: (u32, u32, u32) = (10, 200, 0);
    let third: (u32, u32, u32) = (0, 10, 200);

    match_colors(first);
    match_colors(second);
    match_colors(third);

    let my_num = 10;
    let some_var = match my_num {
        10 => 8,
        // _ => "Not ten", // 안된다. int를 예상했으나, &str을 받아서
        _ => 0,
    };
    // 3항 연산자
    let some_variable = if my_num == 10 { true } else { false };

    println!("{}", some_variable);

    match_num(8);
    match_num2(100);
}
