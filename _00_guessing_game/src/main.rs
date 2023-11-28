use std::{ io, cmp::Ordering };
use rand::Rng;

fn main() {
    println!("숫자 야구 게임");

    let secret_number = rand::thread_rng().gen_range(1..=45);

    let mut counter = 0;
    let mut try_num: Vec<u32> = Vec::new();

    fn show_try(counter: i32, try_num: &Vec<u32>) {
        println!("지금까지 {counter}번 시도했고, 시도한 숫자들은{try_num:?}입니다.");
    }
    loop {
        println!("1~45 중 숫자를 하나 맞춰봐여~");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("숫자만 적으세요~");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) if num < 45 => num,
            Ok(_) => {
                println!("45이하만 치라고여~~");
                println!("");
                continue;
            }
            Err(_) => {
                println!("숫자만 적으세요~");
                println!("");
                continue;
            }
        };
        if try_num.contains(&guess) {
            println!("이전에 쳐봤어요~");
            println!("");
        } else {
            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("틀림! 정답보다 작음");
                    counter += 1;
                    try_num.push(guess);
                    show_try(counter, &try_num);
                }
                Ordering::Greater => {
                    println!("틀림! 정답보다 큼");
                    counter += 1;
                    try_num.push(guess);
                    show_try(counter, &try_num);
                }
                Ordering::Equal => {
                    println!("정답! 드디어 맞췄네요");
                    show_try(counter, &try_num);
                    break;
                }
            }
        }
        println!("");
    }
}
