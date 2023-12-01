// Clipy 는 엄청 좋은 툴이다. Linter를 하게 해준다.

// BOX : owned data on the heap

fn print_vec_ref(input: &Vec<i32>) {
    if input.is_empty() {
        println!("Vec is empty")
    } else {
        for num in input {
            println!("{num}");
        }
    }
}
/*
    warning: length comparison to zero
    --> src/main.rs:4:8
    |
    4 |     if input.len() ==0 {
    |        ^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `input.is_empty()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#len_zero
    = note: `#[warn(clippy::len_zero)]` on by default
*/
struct SomeStruct {
    name: String,
    number: u8,
    data: Box<[u8; 1000]>,
}
fn take_thing<T>(_thing: T) {}

//EXEC03
// struct List {
//     content: Box<List>,
// }
// recursive Types 그래서 List로 적지 않고, Box<List>로 하면 된다.

enum List {
    Content(i32, Box<List>),
    NoContent,
}

//EXEC04
trait Booky {
    fn print(&self){}
}

#[derive(Debug)]
struct Book {
    name: String,
}
#[derive(Debug)]
struct BigBook;
#[derive(Debug)]
struct City{
    year_founded : i32,
}

impl Booky for Book {
    fn print(&self){
        println!("{:?}",self);
    }
}
impl Booky for BigBook {
    fn print(&self){
        println!("{:?}",self);
    }
}
impl Booky for City{
    fn print(&self){
        println!("{:?}",self);
    }
}

fn main() {
    println!("\nEXEC01");
    // EXEC01 -> Clipy 사용 하는 법은
    // cargo clipy를 치면된다.
    let my_vec = vec![8, 9, 10];
    print_vec_ref(&my_vec);

    let mut done = false;
    let mut counter = 0;

    while !done {
        counter += 1;
        if counter > 10 {
            done = true;
        }
    }
    /*
        warning: equality checks against false can be replaced by a negation
        --> src/main.rs:23:11
        |
        23 |     while done ==false {
        |           ^^^^^^^^^^^^ help: try simplifying it as shown: `!done`
        |
        = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#bool_comparison
        = note: `#[warn(clippy::bool_comparison)]` on by default
     */
    let some_var = Some(9);

    match some_var {
        Some(number) => println!("We Got {number}"),
        None => println!("End"),
    }

    println!("\nEXEC02");
    // EXEC02 -> Box

    let my_struct = SomeStruct {
        name: "Hi there".to_string(),
        number: 0,
        data: Box::new([9; 1000]),
        // Box : 40 bytes , data : 1072 bytes
    };

    println!("The struct is {} bytes", std::mem::size_of_val(&my_struct));

    let my_box = Box::new(9);
    println!("{my_box:?}");
    println!("{:?}", *my_box);

    println!("\nEXEC03");
    // EXEC03 -> Box를 이용해서 Heap 메모리에 옮겨놓고 자기 자신의 한 요소를 쓰는 struct를 만들 수 있다.
    // Box에 넣으면 자료가 heap메모리로 가고 stack에 pointer 주소를 남김

    // let my_list = List { content: Box::new(List) };
    let _my_list = List::Content(8786, Box::new(List::NoContent));

    println!("\nEXEC04");
    // EXEC04 -> dyn : trait를 타입으로 만들어주기도 하고, 그냥 앞에 쓰면, trait를 obj로 만들어준다.
    let my_city = City{
        year_founded: 1999
    };
    let vec_of_booky_things: Vec<Box<dyn Booky>> = vec![
        Box::new(Book { name: "Hello".to_string() }),
        Box::new(BigBook),
        Box::new(my_city)
    ];
    for thing in vec_of_booky_things{
        thing.print();
    }

}
