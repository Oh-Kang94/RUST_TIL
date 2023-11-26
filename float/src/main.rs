fn main() {
    let my_number: u8= 9;
    let my_number_= 9_u8;  // 이렇게 적어도 상관 없다.
    println!("{} {}", my_number, my_number_);

    let my_fnumber: f64 = 9.; // f64
    let other_number = 9;

    // println!("{}", my_fnumber+other_number); // 이거는 불가능
    println!("{}", my_fnumber as i32+other_number); // 이거는 가능 역으로도 가능
}