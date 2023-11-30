// fold
/*
    .take_while
    .cloned
    .by_ref
    .skip_while
    .map_while
*/
use std::{cmp::min, slice::Windows};

fn main() {
    println!("\nEXEC01");
    // EXEC01 -> biggest fold 비교
    let my_vec = vec![-878, 87987, -98798, 0, 9123];

    let biggest = my_vec
        .into_iter()
        // .fold(i32::MIN, |num1, num2| {
        //     if num1 > num2{
        //         num1
        //     } else{
        //         num2
        //     }
        // });
        .fold(i32::MAX, |num1, num2| min(num1, num2));  // 시작 값부터 계산을 해서, MAX, 비교해서 결과 표기

    println!("Biggest is : {}", biggest);

    println!("\nEXEC02");
    // EXEC02 -> chunks , Windows
    /*
        .chunks // 1,2,3    4,5,6   7,8,9
        .windows // 1,2,3
    */
    let num_vec = vec![1,2,3,4,5,6,7,8,9,0];
    println!("chunks");
    for chunk in num_vec.chunks(3){
        println!("{chunk:?}");
    }
    println!("windows");
    for windows in num_vec.windows(3){
        println!("{windows:?}");
    }

    println!("\nEXEC03"); 
    // EXEC03 -> match_indices  매치되는 값 인덱스랑 함께 튜퓰로 제공, 
    /*
        .match_indices // 1,2,3    4,5,6   7,8,9
        .peekable // 1,2,3
    */
    let rules= "Rule number 1: No Fighting
    Rule number 2: Go to bed at 8 pm.
    Rule number 3: Wake up at 6 am.";

    let rule_locations: Vec<(_,_)> = rules.match_indices("Rule").collect();  // 글자 자릿수 말하는 것.
    // 보통은 함수로 정해놓고 패턴을 검사하게 한다.
    println!("Rule locations : {rule_locations:?}");

    println!("\nEXEC04"); 
    // EXEC04 -> peekable 음식 먹기전에 음식 괜찮은지 간만보는 것, next는 현재음식 먹고 다음음식부르는거.
    let just_num = vec![1,5,100];
    let mut num_iter = just_num.iter().peekable();

    for _ in 0..3{
        println!("I love the number {}", num_iter.peek().unwrap());
        println!("I love the number {}", num_iter.peek().unwrap());
        println!("I love the number {}", num_iter.peek().unwrap());
        num_iter.next();
    }
    
}
