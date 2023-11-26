// Macro
fn give_age() -> i32 {
    return 29;
}

// fn give_city() -> String{
//     return String::from("seoul");
// }

// &의 의미는 참조를 만드는데 사용
// 'static은 정적 수명, 전체 프로그램의 수명동안 살아 있다는 것
// str -> 문자열 슬라이스이다.

///str과 String은 Rust에서 문자열을 다루는 두 가지 타입입니다. 주요 차이점은 다음과 같습니다:
///불변성 vs. 가변성:
///str: 불변(immutable)한 문자열 슬라이스를 나타냅니다. str은 유니코드 문자열 데이터의 불변 참조(슬라이스)를 가리킵니다. 이는 수정이 불가능하며, 한 번 생성되면 변경할 수 없습니다.
///String: 가변(mutable)한 문자열을 나타냅니다. String은 힙(heap)에 할당된 UTF-8로 인코딩된 텍스트 데이터를 보유하는데, 필요에 따라 수정 가능합니다.
///소유권과 할당:
/// str: &str 형태로 다른 데이터(예: 문자열 리터럴 또는 다른 String의 슬라이스)를 참조할 때 사용됩니다. 따라서 str은 소유권을 가지지 않고, 다른 데이터의 참조를 통해 사용됩니다.
/// String: 메모리를 할당하여 문자열을 보유하고 소유합니다. 따라서 String은 데이터의 소유권을 가지며, 필요에 따라 수정할 수 있습니다.
/// 라이프타임:
/// str은 라이프타임을 명시적으로 가지지 않습니다. 대부분의 경우, str은 참조된 데이터의 라이프타임에 종속됩니다.
/// String은 힙(heap) 할당을 통해 동적으로 생성되므로, 런타임 중에 할당 및 해제되는 것을 나타내는 라이프타임을 가지게 됩니다.
fn give_city() -> &'static str {
    return "seoul";
}

// () -> Empty tuple, unit type (void) 
fn main() {
    let my_name = "Oh Kang";
    // 방법 1
    println!("My name is {}, age is {}", my_name, give_age());
    // 방법 2
    println!("My name is {name}, age is {age}", name = my_name, age =give_age());
    // 방법 3
    println!("My name is {0}, age is {1}, Nice to meet You {0}", my_name, give_age());
    // 방법 4 추후에 나올 예정
    // println!("My name is {my_name}, age is {give_age()}, Nice to meet You {my_name}");
    // 방법 5
    println!("My name is {0}, age is {1}, I live in {2}", my_name, give_age(), give_city());
}

