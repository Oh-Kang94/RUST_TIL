// mutablilty
// shadowing

// immutable by default
// mut

// Shadowing 은 같은 이름을 다시 쓰는 것.
fn double(input:i32) -> i32{
    input * 2
}

fn triple(input:i32) -> i32{
    input * 3
}

fn main() {
    // Mutable을 해야하는 것.
    // let my_number = 10;
    let mut my_number = 10;
    println!("{}", my_number);
    my_number = 9; // immutable이기 때문에 변경이 불가능 하다.
    println!("{}", my_number);

    let my_variable = 10;
    {
        let my_variable = "My variable";
    } 
    println!("{}", my_variable);
    // 마지막인 변수만 관심을 가지는 것.
    let x = 9;
    let x = double(x);
    let x = triple(x);
    println!("{}", x);  // ln 4


    let my_variable = 9;
    {
        let my_varaible = "Some String";
        println!("{}",my_varaible); // ln 5
    }
    println!("{}",my_variable); // ln 6
}
