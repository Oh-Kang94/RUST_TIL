use std::{ rc::Rc, cell::RefCell };

// Rc
fn takes_a_string(input: String) {
    println!("It is: {}", input)
}

fn also_takes_a_string(input: String) {
    println!("It is: {}", input)
}
// EXEC02
fn takes_a_string_rc(input: Rc<String>) {
    println!("It is: {}", input)
}

fn also_takes_a_string_rc(input: Rc<String>) {
    println!("It is: {}", input)
}

// EXEC03
#[derive(Debug)]
struct City {
    name: String,
    pop: u32,
    hist: Rc<String>, // Rc로 교체
}
#[derive(Debug)]
struct CityData {
    names: Vec<String>,
    hist: Vec<Rc<String>>,
}

// EXEC04
#[derive(Debug)]
// struct DataContainer<'a>{
//     data: &'a mut String
// }
struct DataContainer {
    data: Rc<RefCell<String>>,
}

fn main() {
    println!("\nEXEC01");
    // EXEC01 -> Rc::clone
    let user_name = String::from("User MacUserson");
    takes_a_string(user_name.clone());
    also_takes_a_string(user_name);

    println!("\nEXEC02");
    // EXEC01 -> Rc의 정석
    let user_name = Rc::new("Oh-Kang".to_string());
    takes_a_string_rc(Rc::clone(&user_name));
    also_takes_a_string_rc(Rc::clone(&user_name));

    println!("\nEXEC03");
    // EXEC03 -> Weak, String Count
    // let seoul = City {
    //     name: "Seoul".to_string(),
    //     pop: 10_000_000,
    //     hist: "Seoul is ~~~~~~".to_string(),
    // };

    // let korea_cities = CityData {
    //     names: vec![seoul.name],
    //     hist: vec![seoul.hist],
    // };

    // println!("Seoul's hist is  : {:?} ", korea_cities.hist[0]);
    // -> 일반적인 사용
    let seoul = City {
        name: "Seoul".to_string(),
        pop: 10_000_000,
        hist: Rc::new("Seoul is ~~~~~~".to_string()),
    };

    let korea_cities = CityData {
        names: vec![seoul.name],
        hist: vec![Rc::clone(&seoul.hist)],
    };
    println!("Seoul's hist is  : {:?} ", seoul.hist);

    // 소유권자 확인 명령어 Strong count
    println!("Data has {} strong owners", Rc::strong_count(&seoul.hist));

    // Weak count ->  'Weak'는 'Rc'의 한 종류  관리되는 할당에 대한 소유권을 갖지 않고 있는 참조
    println!("Data has {} weak owners", Rc::weak_count(&seoul.hist));

    println!("\nEXEC04");
    // EXEC04 -> Rc & RcCell
    /*
        하나의 문장을 바꾸면서, 융통성있게 전체를 바꿀수 있게 만들어준다.
     */

    // let mut important_data = "Super important data".to_string();

    // let cntr_1 = DataContainer{
    //     data: &mut important_data,
    // };

    // let mut cntr_2 = DataContainer{
    //     data : &mut important_data
    /* 
        이런경우에는 한번 더 빌려오는게 불가능 하다.
        그때, Rc<RefCell<Stirng>>을 써야한다.
    */
    // };

    // for _ in 0..10{
    //     *cntr_1.data = String::from("hi");
    //     *cntr_2.data = String::from("hello");
    // }

    let important_data = Rc::new(RefCell::new("Super important data".to_string()));

    let cntr_1 = DataContainer {
        data: Rc::clone(&important_data),
    };

    let cntr_2 = DataContainer {
        data: Rc::clone(&important_data),
    };

    for _ in 0..10 {
        cntr_1.data.borrow_mut().push('a');
        cntr_2.data.borrow_mut().push('b');
    }
    println!("{cntr_1:?}\n{cntr_2:?}\n{important_data:?}");
}

