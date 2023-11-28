// Closures  = Anonymous functions that capture their env
// 보통 ()안에서 함수를 적어야 할때 많이 쓰임.
// EX) .map(|x| x+1)
/*
    |x: 변수| method -> Pipes
    () ->
*/

//EXEC07
#[derive(Clone, Debug)]
struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };
        Self { name: name.to_string(), ceo }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

//EXEC12
fn in_char_vec(char_vec: &Vec<char>, check: char) {
    println!("Is {} inside? {}", check, char_vec.iter().any(|&char| char == check));  // any는 &를 쓴다.
}

use std::collections::{ HashMap, BTreeSet };

fn main() {
    println!("\nEXEC01");
    // EXEC01 -> Closures Basic
    let my_closure = || println!("This is a Closures");
    my_closure();
    let my_closure_i32 = |x: i32| println!("This is a Closures. num : {x:?}");
    my_closure_i32(3);

    println!("\nEXEC02");
    // EXEC02 -> Closures W/Bracket
    let my_closures_bracket = || {
        let my_num = 7;
        let other_num = 10;
        println!("The two nums are {my_num:?} and {other_num:?}");
        my_num + other_num
    };

    let my_var = my_closures_bracket();
    println!("{my_var:?}");

    println!("\nEXEC03");
    // EXEC03 -> Zero Cost Abstractions : Compile은 오래 걸리겠지만, 프로그램의 속도는 지장받지 않는다.
    // .iter().map().filter().inspect().collect()
    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        if my_vec.get(0).is_some() { &my_vec[0] } else { &0 }
    });
    println!("{fourth:?}");

    println!("\nEXEC04");
    // EXEC04 -> Map
    let num_vec = vec![2, 4, 6];
    let double_vec: Vec<i32> = num_vec
        .iter()
        .map(|num| num * 2)
        .map(|num| num * 3)
        .map(|num| num * 4) // 여기까지는 그냥 없음 Iter의 반복밖에 안함.
        .collect(); // 출력을 도와주는 것.
    println!("{double_vec:?}");

    let num_vec = vec![2, 4, 6];
    num_vec
        .iter() //2, 4, 6
        .enumerate() // (0,2),(1,4),(2,6)
        .for_each(|(index, num)| println!("the num at index {index} is {num}"));
    // .for_each(|tuple| println!("the num at index {} is {}", tuple.0, tuple.1));
    // 이렇게도 쓸수 있지만, Desturcting을 하면 좋다. 가독성 측면에서

    println!("\nEXEC05");
    // EXEC05 -> Zip
    let some_nums = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four", "five"];

    let num_word_hashmap: HashMap<usize, &str> = some_nums
        .into_iter() // -> .zip의 선행 조건
        .zip(some_words.into_iter())
        .collect();
    println!("{num_word_hashmap:?}");

    let some_nums = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four", "five"];

    let num_word_btreeset: BTreeSet<(usize, &str)> = some_nums
        .into_iter()
        .zip(some_words.into_iter())
        .collect();
    println!("{num_word_btreeset:?}");

    let result_str = num_word_hashmap.get(&10).unwrap_or_else(|| {
        println!("Uh oh, didn't work");
        &"no number"
    });
    println!("{result_str}");

    num_word_hashmap.iter().for_each(|stuf| { println!("{stuf:?}") });

    println!("\nEXEC06");
    // EXEC06 -> .filter()
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];
    let filtered_months: Vec<&str> = months
        .into_iter()
        .filter(|month| month.len() < 5)
        .filter(|month| month.contains("u"))
        .collect();
    println!("{filtered_months:?}");

    println!("\nEXEC07");
    // EXEC07 -> filter_map

    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", "")
    ];

    let all_the_ceos: Vec<_> = company_vec
        .clone()
        .into_iter()
        .filter_map(|company| company.get_ceo())
        .collect();

    println!("{all_the_ceos:?}");

    println!("\nEXEC08");
    // EXEC08 -> filter_map result to option
    /*
        .ok() => Result to Option
        .ok_or() => Option to Result
        .ok_or_else() => Option to Result with Clousre
     */
    let user_input = vec!["8.9", "Nine point nine five", "8.0", "7.6", "eleventy-twelve"];

    let actual_numbers = user_input
        .into_iter()
        .filter_map(|input| input.parse::<f32>().ok())
        .collect::<Vec<f32>>();

    println!("{:?}", actual_numbers);

    let mut results_vec = vec![]; // Pretend we need to gather error results too
    company_vec
        .clone()
        .iter()
        .for_each(|company| results_vec.push(company.get_ceo().ok_or("No CEO found")));

    for item in results_vec {
        println!("{:?}", item);
    }

    let mut results_vec = vec![]; // Pretend we need to gather error results too
    company_vec
        .clone()
        .iter()
        .for_each(|company|
            results_vec.push(
                company.get_ceo().ok_or_else(|| {
                    let err_msg = format!("No CEO found for {}", company.name);
                    err_msg
                })
            )
        );

    for item in results_vec {
        println!("{:?}", item);
    }

    println!("\nEXEC09");
    // EXEC09 -> map, and_then
    let some_output = Some(vec![8, 9, 10]);
    let first = some_output
        .clone()
        .map(|some_vec: Vec<i32>| {
            some_vec
                .iter()
                .map(|num| num + 1)
                .collect::<Vec<i32>>()
        })
        .unwrap();
    println!("{first:?}");

    let second = some_output
        .and_then(|some_vec| { // .get() Option
            match some_vec.len() {
                0 => None,
                1 => Some(vec![some_vec[0]]),
                _ => Some(some_vec),
            }
        });  // map을 쓰면, Some에 귀속되기 때문에, Some이 나온다. 나오기 싫으면 and_then(flatmap)

    println!("{second:?}");

    println!("\nEXEC10");
    // EXEC10 -> and, or for Option
    let one = true;
    let two = false;
    let three = true;
    let four = true;

    println!("{}", one && three); // prints true
    println!("{}", one && two && three && four); // prints false
    println!("{}", one || two || three || four); // prints true or

    println!("\nEXEC11");
    // EXEC11 -> and, or for Option
    let first_try = vec![Some("success!"), None, Some("success!"), Some("success!"), None];
    let second_try = vec![None, Some("success!"), Some("success!"), Some("success!"), Some("success!")];
    let third_try = vec![Some("success!"), Some("success!"), Some("success!"), Some("success!"), None];

    for i in 0..first_try.len() {
        println!("{:?}", first_try[i].and(second_try[i]).and(third_try[i]));
    }
    println!("\nEXEC12");
    // EXEC12 -> any(true / false), all(true / false)
    let char_vec = ('a'..'働') 
        .collect::<Vec<char>>();
    in_char_vec(&char_vec, 'i'); 
    in_char_vec(&char_vec, '뷁');
    in_char_vec(&char_vec, '鑿');

    // short-circuiting
    let smaller_vec = ('A'..'z')
        .collect::<Vec<char>>();
    println!("All alphabetic? {}", smaller_vec.iter().all(|&char| char.is_alphabetic()));
    println!("All less than the character 행? {}", smaller_vec.iter().all(|&x| x < '행'));
    println!("{smaller_vec:?}");

    println!("\nEXEC13");
    //EXEC13 any, .rev()
    let mut big_vec = vec![6;1000];
    big_vec.push(5);
    let mut iterator = big_vec.iter().rev();
    // assert_eq!(Some(&5), iterator.next());
    // assert_eq!(Some(&6), iterator.next());
    // println!("{:?}",iterator.next());
    // println!("{:?}",iterator.next());

    println!("{:?}", big_vec.iter().rev().any(|&num|num==5));

    println!("\nEXEC14");
    //EXEC14 any, .rev()
    let mut big_vec = vec![6;1000];
    big_vec.push(5);

    let mut counter = 0;
    // let mut big_iter = big_vec.into_iter(); // counter = 1001
    let mut big_iter = big_vec.into_iter().rev(); // counter = 1 -> 0.004초나 빨라짐

    loop {
        counter +=1;
        if big_iter.next() == Some(5){
            break;
        }
    }
    println!("{:?}", counter);
}
