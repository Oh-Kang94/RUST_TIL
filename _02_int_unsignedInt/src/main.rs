fn main() {
    // 9.0 f32 Float이다.
    /*
        Integer Types
        + sign
        i8,i16, i32,i64,i128, and isize. => - 가능
        u8, U16, U32, u64, U128, and usize. => +만 가능
     */
    // 변수 선언
    let my_number: u16 = 100; // => + 255 까지만 가능한 작은 숫자
    let my_other_number : u16 = 200; // Type interference
    let third_number = my_number + my_other_number; 
    /* 
    u8 + u16은 안되고, 그냥 위가 u8까지이면, type inference에 실패해 안됨.
    */ 
    println!("{}",third_number)
}
