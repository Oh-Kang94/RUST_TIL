#![allow(dead_code)]

// TDD => 다음 Refactoring 하기 => 그다음 main으로 옮기기

// Refactoring
#[derive(Clone)]
struct Calc {
    results: Vec<String>,
    current_input: String,
    total: i32,
    adds: bool,
}

impl Calc {
    fn new() -> Self {
        Self { results: vec![], current_input: String::new(), total: 0, adds: true }
    }

    fn clear(&mut self){
        self.current_input.clear();
    }

    fn push_char(&mut self, character : char){
        self.current_input.push(character);
    }

    fn math(&mut self, input: &str) -> i32 {
        // input 예외 처리
        if
            !input.chars().all(|character| OKAY_CHARACTERS.contains(character)) ||
            !input
                .chars()
                .take(2)
                .any(|character| character.is_numeric())
        {
            panic!("Please only input numbers, +-, or spaces.");
        }
    
        // input 을 VEC에 넣는다.
        let input = input
            .trim_end_matches(|x| "+- ".contains(x))
            .chars()
            .filter(|x| *x != ' ')
            .collect::<String>(); // +,-,""을 제거 하고, filter해서 문자열을 만든다.
    
        for character in input.chars() {
            match character {
                '+' => {
                    if !self.current_input.is_empty() {
                        // 문자열이 비여 있을때, 공백을 넣는 것을 막는다.
                        self.results.push(self.current_input.clone()); // 하지만 없다고 하면, 그것은 숫자이기때문에 그다음에 넣는다.
                        self.clear(); // 그리고 문자 관리 변수를 비워준다.
                    }
                }
                '-' => {
                    // If we get a -,
                    if self.current_input.contains('-') || self.current_input.is_empty() {
                        self.push_char(character);
                    } else {
                        // 아니면, 숫자이다.
                        self.results.push(self.current_input.clone()); // so push the number into self.results, clear it and then push -
                        self.clear();
                        self.push_char(character);
                    }
                }
                number => {
                    // number here means "anything else that matches". We selected the name here
                    if self.current_input.contains('-') {
                        // We might have some - characters to push in first
                        self.results.push(self.current_input.clone());
                        self.clear();
                        self.push_char(number);
                    } else {
                        // But if we don't, that means we can push the number in
                        self.push_char(number);
                    }
                }
            }
        }
        self.results.push(self.current_input.clone()); // Push one last time after the loop is over. Don't need to .clone() because we don't use it anymore
    
        // 계산 파트
        let math_iter = self.results.clone().into_iter();
        for entry in math_iter {
            if entry.contains('-') {
                // If it has a - character, check if it's even or odd
                if entry.chars().count() % 2 == 1 {
                    self.adds = match self.adds {
                        true => false,
                        false => true,
                    };
                    continue; // Go to the next item
                } else {
                    continue;
                }
            }
            if self.adds {
                self.total += entry.parse::<i32>().unwrap(); // If there is no '-', it must be a number. So we are safe to unwrap
            } else {
                self.total -= entry.parse::<i32>().unwrap();
                self.adds = true; // After subtracting, reset self.adds to true.
            }
        }
        self.total // Finally, return the self.total
    }
}

//본 Method 작성
const OKAY_CHARACTERS: &str = "1234567890+- ";



#[cfg(test)]
mod tests {
    use crate::Calc;

    #[test]
    fn one_plus_one_is_two() {
        let mut calc = Calc::new();
        assert_eq!(calc.math("1 +1"), 2);
    }
    #[test]
    fn one_minus_one_is_two() {
        assert_eq!(1 - 2, -1);
    }

    #[test]
    #[should_panic] // 패닉이면 OK
    fn panics_when_characters_not_right() {
        let mut calc = Calc::new();
        calc.math("7 + seven");
    }
}
fn main() {
    let mut calc = Calc::new();
    let res = calc.math("1+1");
    println!("{res}")
}   
