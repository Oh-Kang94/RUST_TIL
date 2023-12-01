use std::io;
use std::collections::BTreeMap;
use rand::{ rngs::StdRng, SeedableRng, Rng };



#[derive(Debug)]
enum Question {
    Android,
    Spring,
    JSP,
    JAVA,
    PHP,
    RUST,
}

impl Question {
    fn to_string(&self) -> String {
        match self {
            Question::Android => String::from("android"),
            Question::Spring => String::from("spring"),
            Question::JAVA => String::from("java"),
            Question::JSP => String::from("jsp"),
            Question::PHP => String::from("php"),
            Question::RUST => String::from("rust"),
        }
    }
}

/// CLI로 값 받는 Method
fn insert_input(mem: &mut String) {
    io::stdin().read_line(mem).expect("Failed to read line");
}



fn main() {
    use Question::*;
    let questions: Vec<String> = vec![
        Android.to_string(),
        Spring.to_string(),
        JSP.to_string(),
        JAVA.to_string(),
        PHP.to_string(),
        RUST.to_string()
    ];

    println!("타자연습게임");
    println!("1) 게임 2) 종료");
    println!("메뉴 선택 >>");

    let mut menu = String::new(); // 메뉴
    loop {
        insert_input(&mut menu);
        match menu.trim().parse::<u8>() {
            Ok(_num) =>
                match _num {
                    1 => { _num }
                    2 => {
                        break;
                    }
                    _ => {
                        println!("다시 입력하세요");
                        continue;
                    }
                }
            Err(_) => {
                println!("다시 입력하세요");
                continue;
            }
        };
        let mut scores: Vec<bool> = Vec::new();
        for (index, question) in questions.iter().enumerate() {
            // enumerate()는 iter()와 쓸때 index 제공
            let mut rng = StdRng::from_entropy();

            // 문자열 길이
            let len = question.len();

            // 두 개의 랜덤한 인덱스 선택 (중복 허용)
            let index1 = rng.gen_range(0..len);
            let index2 = rng.gen_range(0..len);

            // 선택된 두 인덱스에 해당하는 문자를 '*'로 변경
            let asterisk_string: String = question
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == index1 || i == index2 {
                        '*' // 선택된 두 인덱스에 해당하는 문자를 '*'로 변경
                    } else {
                        c // 나머지 문자는 그대로 유지
                    }
                })
                .collect();

            println!("{}){:?}의 단어를 맞춰 보세요.", index + 1, &asterisk_string);

            let mut answer = String::new(); // 정답 표현

            insert_input(&mut answer);
            let answer = answer.trim().to_lowercase();

            if answer == *question {
                scores.push(true);
                println!("맞췄습니다.");
            } else {
                scores.push(false);
                println!("틀렸습니다.");
            }
        }
        // 결과 처리
        let mut results: BTreeMap<String, bool> = BTreeMap::new();
        for (key, value) in questions.iter().zip(scores.iter()) {
            results.insert(key.to_string(), *value);
        }

        // 결과 표시
        let mut output: String = String::new();
        // 절취선
        let br: String = format!("{}\n", (0..13).map(|_| "-").collect::<String>());
        output.push_str(&br);
        for question in results.keys() {
            output.push_str(&format!("{:<10}", question));
        }
        output.push_str("\n");
        for result in results.values() {
            output.push_str(&format!("{:<10}", result));
        }
        output.push_str("\n");
        output.push_str(&br);
        output.push_str("[Game Over]\n");
        output.push_str(&br);
        println!("{output}");

        break;
    }
}
