// interior mutability
// changing on the inside
/*
    &immutable reference -> shared reference
    &mut reference -> unique reference 
*/
// Cell - small room
// RefCell
// Mutex
// RwLock

/*
    let mut x = 9;
    thread 1{
        x+=1;
    }
    thread 2{
        x+=1;
    }
    이런 상황일 때, 프로그램의 입장에서는 동기 작업일때는 안되기때문에 
    not thread safe 하다.
    이럴때 나오는게 cell 이다.

    *그리고, Cell안에 있으면, 보내주기만 하고, 들어갈 수가 없다.*
 */

use std::{ cell::{ Cell, RefCell }, borrow::BorrowMut };

// struct PhoneModel {
//     company_name: String,
//     model_name: String,
//     screen_size: f32,
//     memory: usize,
//     date_issued: u32,
//     on_sale: bool,
// }
#[derive(Debug)]
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

// EXEC03
#[derive(Debug)]
struct User {
    id: u32,
    year_register: u32,
    username: String,
    active: RefCell<bool>,
    // Many other fields
}

// 예를 들어, 외부의 package를 쓰려고 하면, tokio, rocket 같은
// Interior mutability를 만드는 법
// EXEC04
trait SuperCoolTrait {
    fn cool_func(&self);
}
#[derive(Debug)]
struct User2 {
    id: u32,
    time_used: Cell<u32>,
}
impl SuperCoolTrait for User2 {
    fn cool_func(&self) {
        println!("Now using Cool function");
        let time_used = self.time_used.get();
        self.time_used.set(time_used + 1);
        println!("{}번이다.", self.time_used.get());
    }
}

//EXEC05
// RC - Reference Counter : Python에서는 Reference counter가 있다고 생각하면 된다.
// 힙 메모리에 할당된 타입 T 값의 소유권을 공유할 수 있게 해주는 타입 ~~ Smartpointer라고 생각하면 된다. 
// 그럼 여러개의 소유자를 가지게 된다.
/*
    Rc<RefCell>
    Arc<Mutex> -> Atomic Reference counter
*/

fn main() {
    println!("\nEXEC01");
    // EXEC01 -> Cell 쓰는 법
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true), // -> Cell 선언
    };
    println!("{super_phone_3000:?}");
    super_phone_3000.on_sale.set(false); // -> 이렇게만 변경 가능
    println!("{super_phone_3000:?}");

    println!("\nEXEC02");
    // EXEC02 -> Cell 쓰는 법
    let my_cell = Cell::new(String::from("I am a String"));
    my_cell.set(String::from("Am I a String?"));
    // let my_string = my_cell.get(); // small copy는 안된다.

    println!("\nEXEC03");
    // EXEC03 -> RefCell -> runtime check borrowing rules
    let user_1 = User {
        id: 1,
        year_register: 2020,
        username: "User_1".to_string(),
        active: RefCell::new(true),
    };

    println!("{user_1:?}");
    // output : User { id: 1, year_register: 2020, username: "User_1", active: RefCell { value: true } }
    let mut first_ref = user_1.active.borrow_mut();
    println!("{user_1:?}");
    // output : User { id: 1, year_register: 2020, username: "User_1", active: RefCell { value: <borrowed> } }
    // 보여주지 않는다. 빌려와서
    *first_ref = false;
    println!("{user_1:?}");
    // output : User { id: 1, year_register: 2020, username: "User_1", active: RefCell { value: <borrowed> } }
    drop(first_ref);
    println!("{user_1:?}");
    // output : User { id: 1, year_register: 2020, username: "User_1", active: RefCell { value: false } }

    println!("\nEXEC03");
    // EXEC03 -> RefCell 쓰는 법
    // Unsafe하지만, Runtime에 결정된다고 해서, Heap은 아니다.
    let my_cell = RefCell::new(String::from("I am a String"));
    println!("{my_cell:?}");
    // *my_cell.borrow_mut() = String::from("I am not a String");

    // Error test
    // let blocking_ref = my_cell.borrow_mut();
    // Output : We got an error : already borrowed, RefCell { value: <borrowed> }
    match my_cell.try_borrow_mut() {
        Ok(mut cell) => {
            *cell = String::from("I am not a String");
        }
        Err(e) => println!("We got an error : {e}"),
    } // 이게 가장 안전한 코드
    println!("{my_cell:?}");

    println!("\nEXEC04");
    // EXEC04 -> RefCell 쓰는 법
    let user = User2 {
        id: 123123,
        time_used: Cell::new(1),
    };
    for _ in 1..20 {
        user.cool_func();
    }
    println!("{user:?}");
}
