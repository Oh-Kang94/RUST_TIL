// & immutable reference / shared reference
// &mut mutable reference / unique reference
// &*는 ref를 말하는게 아니다.

// mutable ref랑 ref를 둘다 같이 못쓴다.
// immut ref이 참조하는 값이 mut ref으로 수정하기 전인지 후인지 판단할 수 없기 때문

// OWNERRSHIP
fn print_country(country_name: String) {
    println!("My country is {}", country_name)
}
fn print_country_ref(country_name: &String) {
    println!("My country is {}", country_name)
}

fn add_is_great(country_name: &mut String) { // take by value, declare as mutable
    country_name.push_str("is Great!");
    println!("{}", country_name)
    // country_name 이렇게 다시 선언하면, 다시 재활용을 할 수 있다.
}

// Ownership and copy types
// RUST BOOKS에서는 It's trivial to copy the bytes
fn prints_number(number: i32){
    println!("{}", number);
}

fn prints_string(input: String){
    println!("{}", input);
}



fn main() {
    let mut my_number = 9;
    let num_ref = &mut my_number;
    // num_ref = 10; -> &을 10으로 바꿀수는ㄴ 없으니까
    *num_ref = 10;
    println!("{}", my_number);

    // let mut number = 10;
    // let number_ref = &number;
    // let num_change = &mut number;
    // *num_change += 10;
    // println!("{}", number_ref);
    /*
        error[E0502]: cannot borrow `number` as mutable because it is also borrowed as immutable
        --> main.rs:16:22
        |
        15 |     let number_ref = &number;
        |                      ------- immutable borrow occurs here
        16 |     let num_change = &mut number;
        |                      ^^^^^^^^^^^ mutable borrow occurs here
        17 |     *num_change += 10;
        18 |     println!("{}", number_ref)
        |                    ---------- immutable borrow later used here
     */

    // Shadowing
    let country = "대한민국";
    let country_ref = &country;
    let country = 8;
    println!("{}, {}", country, country_ref);

    // OWNERSHIP 기본 개념
    let country = "대한민국".to_string();
    print_country(country);
    // print_country(country);  -> 값을 한번 썼기 때문에 안된다. 불러오고 싶으면
    let country = "대한민국".to_string();
    print_country_ref(&country);
    print_country_ref(&country);
    print_country_ref(&country);
    

    // mut ref and mut in func
    let mut country = "대한민국 ".to_string();
    // add_is_great(country); by value이므로 이건 안된다.
    // add_is_great(&country); 그냥 ref라 안된다.
    add_is_great(&mut country);


    // Ownership and copy types
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);


    // copy - copy types
    // clone - non-copy types
    let my_country = "대한민국".to_string();
    prints_string(my_country.clone()); // -> clone은 본인이 아니기 때문에, my_country 의 소유권이 없기 때문에, 에러가 안생긴다.
    prints_string(my_country.clone()); // -> ref로 안쓰게 만들어 주어서 좋다. 하지만, 자주 쓰면 GC가 안되니까 
    prints_string(my_country.clone()); 
    prints_string(my_country);
}
