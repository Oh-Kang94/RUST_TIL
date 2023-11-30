use std::{ io, cmp::Ordering, cell::RefCell };
use rand::Rng;

fn main() {
    println!("숫자 야구 게임");
    let winning_count:RefCell<u16> = RefCell::new(0);
    loop {
        let mut doagain: bool = true;
        let secret_number: u32 = rand::thread_rng().gen_range(1..=45);
        println!("{secret_number}");
        let mut counter: u32 = 0;
        let mut try_num: Vec<u32> = Vec::new();

        fn show_try(counter: u32, try_num: &Vec<u32>) {
            println!("지금까지 {counter}번 시도했고, 시도한 숫자들은{try_num:?}입니다.");
        }
        loop {
            println!("1~45 중 숫자를 하나 맞춰봐여~");

            let mut guess = String::new();

            io::stdin().read_line(&mut guess).expect("숫자만 적으세요~");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) if num > 0 && num <= 45 => num,
                Ok(_) => {
                    println!("1~45의 유효한 숫자를 적으세요~");
                    println!("");
                    continue;
                }
                Err(_) => {
                    println!("1~45의 유효한 숫자를 적으세요~");
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
        println!("다시 하실래요?? \n다시하고 싶으시면 1을 누르시고, \n그만두려면 2를 누르세요.");

        let mut input_doagain = String::new();
        io::stdin().read_line(&mut input_doagain).expect("숫자만 적으세요~");
        let input_doagain = input_doagain.trim().parse::<u32>();
        match input_doagain {
            Ok(num) =>
                match num {
                    1 => {
                        println!("다시 합시다~!");
                    }
                    2 => {
                        println!("안녕히 가세요~");
                        doagain = false;
                    }
                    _ => {
                        println!("유효한 숫자만 치세요 제발!");
                        continue;
                    }
                }
            Err(_) => {
                println!("유효한 숫자만 치세요 제발!");
                continue;
            }
        }
        if !doagain {
            break;
        }else{
            println!("지금까지 {:?}번 이기셨습니다.",winning_count.try_borrow_mut())
        }
    }
}
