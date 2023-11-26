fn main() {
    print!("This\\nis"); // Escape 문자 작동
    println!(r#"c:\thisdrive\new_drive"#); // Escape 문자 무시
    println!("
Let me tell you
something
to do"); // 이렇게 앞에 끌어야지 공백이 없다.
    let my_var = &9;
    println!("{:p}", my_var); // Pointer를 출력도 가능
    let my_var = 9000;
    println!("{:X}", my_var); // hexadecimal(16진법)를 출력도 가능
    let my_var = 9000;
    println!("{:b}", my_var); // byte 출력도 가능

    let title = "Today's News";
    println!("{:-^30}", title); // no var name, pad with -, put in center, 30 chars long

    let bar = "|";
    println!("{: <15}{:>15}",bar,bar);

    let genius_oh = "코딩천재 오성민";
    println!("{:⭐^12}",genius_oh);
    println!("{:⭐>10}{:⭐<2}",genius_oh,"");
    let a: &str = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}",city1 =a, city2=b);
}
