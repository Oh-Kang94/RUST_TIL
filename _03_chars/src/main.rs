fn main() {
    println!("Hello, world!");
    let first_letter = 'A';
    let space = ' '; // 띄어쓰기도  ' '도  Char이다.
    let other_language_char = 'q';
    let cat_face = '🐱';
    println!("{}{}{}{}",first_letter,space,other_language_char,cat_face);

    // Casting = Simple type change
    let my_number: u16 = 8; // i32
    let second_number: u8 = 10;
    let third_number = my_number + second_number as u16; // 이게 Casting
    println!("{}",third_number); 

    let my_number = 's' as u8;
    println!("{}",my_number);

    // :: 는 의존성 부르기
    println!("{}", std::mem::size_of::<char>());
    // .len() = length는 문장의 길이을 말하는 것 같지만
    // RUST는 바이트의 크기를 말하는 것이다.
    println!("Size of string containing 'a': {}", "a".len());
    println!("Size of string containing 'β': {}", "β".len());
    println!("Size of string containing '國': {}", "國".len());
    println!("Size of string containing '👌': {}", "👌".len());

    println!("Size of string containing 'a': {} {}", "aaaaaaaaaa".chars().count(), "asdsds".chars().count());

    println!("\nEXEC-ex01");
    //EXECex01 -> str 관련 iterator
    let big_string = "Hello, I an &str";
    big_string.chars().for_each(|c| println!("{c}"));
    println!("Big String has {}", big_string.chars().count());
    big_string.char_indices().for_each(|(index, chars)|{
        println!("At index {index} is the chars {chars}");
    })
}
