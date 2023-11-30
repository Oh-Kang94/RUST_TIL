// lifetimes
// String
// &str

// EXEC01
fn return_ref() -> &'static str {
    // -> 이렇게 해야만, 프로그램이 끝날때까지 지속이 되니까 가능
    // let my_string = "oh-kang".to_string(); // &'static -> 프로그램이 끝날때 까지
    // let my_string_ref = &my_string; // &str -> reference sth else
    "Oh-Kang"
}

//EXEC02
struct Book<'booklifetime, 'otherlifetime> { //
    // 'booklifetime는 수명을 말한다. 하지만, 이게 수명을 바꾸는 역할을 하지는 않는다.
    // 이렇게 되면, Book은 'booklifetime 만큼은 살것이다라는 것을 알 고 있다.
    name: &'booklifetime str,
    sec_name: &'otherlifetime str,
}

// 이렇게는 쓸수는 있는데, 잘 안슨다. 그냥 하나의 lifetime으로 가능하다고 생각해라.

//EXEC03
struct BookStatic {
    // 오히려, <'a> 는 안된다. 왜냐하면, static으로 선언이 안에 들어갔는데,
    // 'a만큼 살아 있으라고 명할 수는 없기때문에.
    name: &'static str,
}

//EXEC04
struct Adventurer<'Adventurer> {
    name: &'Adventurer str,
    hit_points: u32,
}

// implicit == not said
// elided == not shown
impl Adventurer<'_> {
    // lifetime을 명시해두면 에러가 뜬다. 쓸거면 앞에 명시해야하는데 그냥 뒤에 두는게 낫다.
    // 그래서, 그냥 <'_>이걸 적는게 낫다.
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hp left!", self.name, self.hit_points)
    }
}

fn main() {
    println!("\nEXEC01");
    // EXEC01 ->
    let my_name = "oh-kang";
    {
        let my_string = "oh-kang".to_string(); // &'static -> 프로그램이 끝날때 까지
        let my_string_ref = &my_string; // &str -> reference sth else
    }
    println!("\nEXEC02");
    // EXEC02 -> 'a lifetime
    let my_book = Book {
        name: "my Book",
        sec_name: "abc",
    };

    println!("\nEXEC02");
    // EXEC02 -> 'a lifetime
    let my_book_title = "my_book_title".to_string();
    // let my_book = BookStatic{
    // name : &my_book_title
    // };
    // }
    // ---->이렇게는 book_title이 살아 있지 않으니까 쓸 수 없다.

    let my_book = Book {
        name: &my_book_title,
        sec_name: &my_book_title,
    };
}
