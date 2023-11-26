fn main() {
    // Owned type
    // String
    // &str ref str "string slice"
    let my_name = "Jimmy".to_string(); // &str
    let other_name = String::from("Jimmy2");
    let mut my_other_name: &str = "Jimmy3";
    my_other_name.to_string().push('!');
    print!("{}",my_other_name);
    println!("{}",my_name);

    // String = Sized type => String은 대부분 data가 Heap메모리에 있고,
    // str = dynamic type (&str없는것은) => Stack에 저장되어 있기 때문에


    // reallocation -> 할당
    

    // String
        // .capacity
        // .push
        // .push_str
        // .pop
        // .with_capacity
    let mut my_name = "Oh Kang".to_string();
    my_name.push('!'); // 하나만 가능
    println!("{}", my_name);
    println!("length는  {}, Capacity 확인 {}",my_name.len(), my_name.capacity());
    my_name.push_str("내 이름은!");
    println!("{}", my_name);
    println!("Capacity 확인 {}", my_name.capacity());
    println!("{}",my_name);
    assert_eq!(my_name.pop(), Some('!')); // 오른쪽에 있는 것만 빼기 가능 Stack이기 때문에
    println!("{}",my_name);

    let mut city = String::with_capacity(26);
    city.push_str("Seoul 입니다.!!!!!!!!!!!!!!!!!!");
    println!("{} \n {}", city, city.capacity());
}
