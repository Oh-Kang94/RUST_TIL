fn main() { // &
    let my_number = 15;
    let single_ref = &my_number; // Ref to My_number
    let double_ref = &single_ref; // Ref의 Ref도 만들수 있지만, 가끔 나온다.
    let five_ref: &&&&&i32 = &&&&&my_number;
}
