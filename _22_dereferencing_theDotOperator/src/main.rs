// references and the dot operator
// EXEC02
struct Item{
    number :u8
}

// EXEC03 -> dot operator 
impl Item{
    fn compare_number(&self, other_number:u8){
        println!("Are they equal? {}", self.number == other_number)
    }
}

fn main() {
    println!("\nEXEC01");
    // EXEC01 ->
    let my_number = 10;
    let reference = &my_number;
    println!("Are they the same? {}", my_number == *reference);

    println!("\nEXEC02");
    // EXEC02 ->
    let item = Item{
        number : 10
    };

    let reference = &item.number; // & u8
    println!("Are they the same? {}", reference == &10);

    println!("\nEXEC03");
    // EXEC03 -> dot operator 
    // Deref * 을 이용해서 다 알기 때문에 가능한 일임.
    let item = Item{
        number : 10
    };
    let reference_item = &item; // &Item
    let other_reference_item = &&item; // &&Item

    item.compare_number(5);
    reference_item.compare_number(5);
    other_reference_item.compare_number(5);
}
