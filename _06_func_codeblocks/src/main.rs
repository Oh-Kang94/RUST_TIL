fn give_number(one: i32, two : i32)-> i32 {
    one * two
}

fn multiple_number(one: i16, two : i16){
    let multiplied = one * two;
    // code Block => 변수를 code Block으로 선언해서 길게 써도 상관이 없다.
    let multiplied_by_ten = {
        let first_number = 10;
        // return 일때는 ;를 생략해야함
        first_number * one * two
    };
    println!("{}", multiplied);
    println!("{}", multiplied_by_ten);
}

fn main() {
    let my_number = give_number(9, 8);
    println!("{}",my_number);
    multiple_number(7, 8)
}
