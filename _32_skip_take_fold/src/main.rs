// Itertator을 이용한 Methods
// skip, take, fold
fn main() {
    println!("\nEXEC01");
    // EXEC01 -> take, skip
    let ten_chars: Vec<_> = ('a'..).take(10).collect();
    println!("{ten_chars:?}");

    let skip_then_ten_chars: Vec<_> = ('a'..).skip(1000).take(10).collect();
    println!("{skip_then_ten_chars:?}");

    println!("\nEXEC02");
    // EXEC02 -> fold
    let some_numbers = vec![9, 6, 9, 10, 11];
    println!(
        "{}",
        some_numbers.iter().fold(0, |total_so_far, next_number| total_so_far + next_number)
        // 각요소에 fold에 관련된 함수를 적용한다. 시작값은 설정해주고 
    );
    println!("{}", some_numbers.iter().sum::<i32>()); 

    println!("\nEXEC03");
    // EXEC02 -> fold
    let a_string = "I don't have any dashes in me";
    let dashed = a_string
        .chars() // iterator
        .fold("-".to_string(), |mut string_so_far, next_char|{
            string_so_far.push(next_char);
            string_so_far.push('-');
            string_so_far
        });
    println!("{dashed:?}")
} 
