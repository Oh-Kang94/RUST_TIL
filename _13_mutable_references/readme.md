소유권(Ownership)은 Rust의 가장 유니크한 특성이며, Rust가 Garbage collector 없이 안정성 보장을 가능하게 해줍니다. Rust에서 소유권이 어떻게 동작하는지 이해하는 것은 매우 중요합니다. 소유권 뿐만 아니라 관련된 특성들(빌림, 슬라이스, Rust 메모리 데이터 저장 등)에 대해 알아보겠습니다.

## 소유권이란?

모든 프로그램은 실행하는 동안 컴퓨터의 메모리를 사용하는 방법을 관리해야 합니다. 몇몇 언어들은 프로그램이 실행될 때 더 이상 사용하지 않는 메모리를 끊임없이 찾는 Garbage collection을 갖고있거나 프로그래머가 직접 명시적으로 메모리를 할당하고 해제해야 합니다.

하지만 Rust에서 메모리는 컴파일 타임에 컴파일러가 체크할 규칙들로 구성된 소유권 시스템을 통해 관리됩니다. 소유권 기능의 어떤 것도 런타임 비용이 발생하지 않습니다.

## 스택과 힙

Rust와 같은 시스템 프로그래밍 언어에서는 값이 스택에 있는지 힙에 있는지의 여부가 언어의 동작 방식과 우리의 결단에 큰 영향을 줍니다. 스택과 힙에 관계된 소유권을 뒤에서 더 자세하게 다룰 것입니다.

스택은 데이터에 접근하는 방식 덕에 빠릅니다. 새로운 데이터를 넣어두기 위한 공간 혹은 데이터를 가져올 공간을 검색할 필요가 전혀 없습니다. 또한, 스택에 담긴 모든 데이터가 결정되어있는 고정된 크기를 갖고 있어야 합니다.

컴파일 시에 크기가 결정되어 있지 않거나 크기가 변경될 수 있는 데이터를 위해서는 힙을 사용할 수 있습니다. 데이터를 힙에 넣을 때 먼저 저장할 공간이 있는지 확인합니다. 운영체제는 힙 안의 어떤 빈 지점을 찾아 사용중으로 변경하고 해당 지점의 포인터를 돌려줍니다. 이 절차를 "힙 공간 할당하기" 혹은 그냥 "할당"이라고 부릅니다.

힙이 저장된 데이터에 접근하는 것이 비교적 느린 이유는 포인터를 따라가야 하기 때문입니다. 또한, 힙으로부터 큰 공간을 할당받는 것 또한 시간이 걸릴 수 있습니다.

코드의 어느 부분이 힙의 어떤 데이터를 사용하는 지 추적하는 것, 힙의 중복된 데이터의 양을 최소화하는 것, 힙 내에 사용하지 않는 데이터를 제거하여 공간이 모자라지 않게 하는 것은 모두 소유권과 관계된 문제들입니다. 힙 데이터를 관리하는 것이 곧 소유권의 존재 이유입니다.

## 소유권 규칙

Rust에서 각각의 값은 해당값의 owner라고 불리는 변수를 갖고 있다.
한 번에 딱 하나의 owner만 존재할 수 있다.
owner가 스코프 밖으로 벗어날 때, 값은 버려진다(dropped).
변수의 스코프

스코프란 프로그램 내에서 아이템이 유효함을 표시하기 위한 범위입니다.

let s = "hello";
변수 s는 스트링 리터럴을 나타내는데, 스트링 리터럴의 값은 프로그램의 텍스트 내에 하드코딩되어있습니다. 변수는 선언된 시점부터 현재 스코프가 끝날 때까지 유효합니다.

아래의 코드는 변수 s가 유효한 지점을 주석으로 표시한 것입니다.
```
{                      // 선언 전이므로 s는 유효하지 않습니다.
    let s = "hello";   // s는 이 지점부터 유효합니다.

    // s를 가지고 뭔가 합니다.
}  // 이 스코프는 이제 끝이므로, s는 더이상 유효하지 않습니다.
```
두 가지 중요한 지점이 있습니다.

스코프 안에서 s가 등장하면 유효합니다.
이 유효기간은 스코프 밖으로 벗어날 때까지 지속됩니다.
스코프와 변수가 유효한 시점 간의 관계는 다른 프로그래밍 언어와 비슷합니다.

String 타입

앞서 스트링 리터럴을 봤는데, 이 값은 프로그램 안에 하드코딩 되어있습니다. 문자열 값은 편리하지만, 텍스트를 필요하는 모든 경우에 대해 항상 적절하진 않습니다. 문자열은 불변(immutable)입니다. 또한 모든 문자열이 프로그래밍 시점에서 다 알 수 있는 것이 아닙니다. 이러한 경우들을 위해 String을 사용하게 됩니다. String은 힙에 할당되어 컴파일 타임에 알 수 없는 양의 텍스트를 저장할 수 있습니다.

아래 코드처럼 스트링 리터럴로 부터 from함수를 이용해 String을 만들 수 있습니다.

```
let s = String::from("hello");
let mut s = String::from("hello");


s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스티링에 붙여줍니다.

println!("{}", s); // hello, world!
```
메모리와 할당

String 타입은 변경 가능하고 커질 수 있는 텍스트를 지원하기 위해 만들어졌고, 힙에서 컴파일 타임에는 알 수 없는 어느 정도 크기의 메모리 공간을 할당받아 내용물을 저장합니다. 즉, 이는 다음을 의미합니다.

런타임에 운영체제로부터 메모리가 요청되어야 함
String의 사용이 끝났을 때 운영체제에게 메모리를 반납할 방법 필요
1번은 우리가 직접 수행하게 됩니다. String::from 을 호출하면 구현 부분에서 필요한 만큼의 메모리를 요청합니다.

2번은 가비지 콜렉터(GC)가 없는 경우, 할당받은 메모리가 더 이상 필요없는 시점을 알아서 명시적으로 반납하는 코드를 호출하는 것이 프로그래머의 책임입니다. 반납을 잊는다면 메모리를 낭비하게 되고, 너무 빨리 반납하면 유효하지 않은 변수를 갖게 됩니다. 만약 반납을 두 번 한다면 버그가 발생합니다. 딱 한번의 allocate와 free 쌍을 사용해야 합니다.

Rust에서 메모리는 변수가 소속되어있는 스코프 밖으로 벗어나는 순간 자동으로 반납됩니다.

{
    let s = String::from("hello"); // s는 여기서부터 유효합니다

    // s를 가지고 뭔가 합니다
}                                  // 이 스코프는 끝났고, s는 더 이상 유효하지 않습니다
변수가 스코프 밖으로 벗어나면 (괄호가 닫힐 때) Rust는 자동적으로 drop 함수를 호출합니다.

이동(move) : 변수와 데이터가 상호작용하는 방법

let x = 5;
let y = x;
위의 코드는 정수값 5를 x에 묶어놓고 x의 복사본을 만들어 y에 묶는 코드입니다. 정수값이 결정되어있는 고정된 크기의 단순한 값이고 5라는 값들이 스택에 push됩니다.

위의 코드를 String 버전으로 바꿔봅시다.

let s1 = String::from("hello");
let s2 = s1;
코드가 앞선 코드와 유사해보이기 때문에 동작 방식도 비슷해보이지만, 실제로는 다릅니다.



String 은 그림의 왼쪽처럼 세 개의 부분으로 이루어져 있습니다. 문자열의 내용물을 담고 있는 메모리의 포인터, 길이, 용량입니다. 왼쪽 데이터 그룹은 스택에 저장됩니다. 내용물을 담은 오른쪽 부분은 힙 메모리에 있습니다.



s2에 s1을 대입하면 String 데이터가 복사되는데, 이는 위의 그림처럼 스택에 있는 포인터, 길이값, 용량이 복사된다는 의미입니다. 포인터가 가리키고 있는 힙 메모리 상의 데이터는 복사되지 않습니다. 만약 데이터까지 복사된다면 런타임 상에서 매우 느려질 가능성이 있습니다.

앞서 변수가 스코프 밖으로 벗어날 때 러스트는 자동적으로 drop 함수를 호출해 해당 변수가 사용하는 힙 메모리를 제거한다고 했습니다. 하지만 위 그림 처럼 두 데이터 포인터가 모두 같은 곳을 가리키고 있는 경우 문제가 발생합니다. s2와 s1이 스코프 밖으로 벗어나게 되면, 둘 다 같은 메모리를 해제하려 할 것입니다. 이는 두번 해제(double free) 오류로 메모리 안정성 버그 중 하나입니다. 두번 해제는 메모리 손상(corruption)의 원인이 되며 보안 취약성 문제를 일으킬 가능성이 있습니다.

메모리 안정성 보장을 위해 Rust에서는 할당된 메모리를 복사하는 것 대신 s1이 더 이상 유효하지 않다고 간주해 s1이 스코프 밖으로 벗어났을 때 아무것도 해제할 필요가 없어지도록 만듭니다.

let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
위의 코드처럼 s2가 만들어진 후에 s1을 사용하려고 할 때 다음과 같은 에러 메세지를 보게됩니다.

error[E0382]: use of moved value: `s1`
 --> src/main.rs:4:27
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`,
which does not implement the `Copy` trait
Rust가 유효하지 않은 참조자를 사용하는 것을 막기 때문입니다.

다른 언어의 "앝은 복사(shallow copy)"와 비슷해보일 수 있지만, Rust는 첫 번째 변수를 무효화시키기도 하기 때문에 얕은 복사라 부르지않고 "이동(move)" 이라고 부릅니다. 위 코드에서는 s1이 s2로 이동되었다 고 표현합니다.



위 그림은 s1이 무효화된 후의 메모리 구조입니다. 오직 s2만 유효한 상황에서 스코프 밖으로 벗어나면 혼자 메모리를 해제할 것이고 에러가 발생하지 않을 것입니다.

Rust는 결코 자동적으로 "깊은 복사"를 하지 않습니다. 그러므로 어떠한 자동적인 복사라도 런타임 실행 과정에서 효율적일 수 있을 것이라 가정할 수 있습니다.

클론(clone) : 변수와 데이터가 상호작용하는 방법

let s1 = String::from("hello");
let s2 = s1. clone();

println!("s1 = {}, s2 = {}", s1, s2);
만약 깊은 복사를 원한다면, clone 이라는 공용 메소드를 사용할 수 있습니다. clone 을 호출하는 부분을 보면 어떤 비용이 많이 들어갈지도 모르는 코드가 실행 중이라는 것을 알 수 있습니다. 이는 무언가 다른 동작이 수행되는 것을 알려주는 시각적인 지시자입니다.

복사(copy) : 스택에만 있는 데이터

let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
위의 코드는 clone 을 호출하지 않았지만, x도 유효하며 y로 이동하지도 않았습니다.

그 이유는 정수형과 같이 컴파일 타임에 결정되어있는 크기의 타입은 모두 스택에 저장되기 때문에 실제 값의 복사본이 빠르게 만들어질 수 있습니다. 그러므로 y가 생성된 후에 x가 더 이상 유효하지 않도록 해야할 이유가 없습니다. 다르게 말하면, 깊은 복사와 얕은 복사 간의 차이가 없습니다.

Rust는 정수형과 같이 스택에 저장할 수 있는 타입에 대해 달 수 있는 Copy 트레잇이라고 불리는 특별한 annotation을 갖고있습니다. 만일 어떤 타입이 Copy 트레잇을 갖고있다면, 대입 과정 후에도 예전 변수를 계속 사용할 수 있습니다.

Rust는 만일 그 타입 혹은 그 타입이 가지고 있는 부분 중에서 Drop 트레잇을 구현한 것이 있다면 Copy 트레잇을 annotation 할 수 없게끔 합니다. 만일 어떤 타입이 스코프 밖으로 벗어났을 때 어떤 특수한 동작을 필요로 하고 그 타입에 대해 Copy annotation을 추가한다면 컴파일 타임 오류가 발생합니다. 

다음은 Copy 가능한 타입들입니다. 일반적인 규칙으로 단순한 스칼라 값들의 묶음은 Copy 가능하고, 할당이 필요하거나 어떤 자원의 형태인 경우 Copy 불가능합니다.

u32와 같은 모든 정수형 타입들
true / false 값을 갖는 boolean 타입
f64와 같은 모든 부동 소수점 타입들
Copy 가능한 타입만으로 구성된 튜플들. (i32, i32)는 Copy 가능하지만, (i32, String)은 불가능
소유권과 함수
```
fn main() {
    let s = String::from("hello");  // s가 스코프 안으로 들어왔습니다.

    takes_ownership(s); // s의 값이 함수 안으로 이동했습니다...
    // ... 그리고 이제 더이상 유효하지 않습니다.
    let x = 5;                      // x가 스코프 안으로 들어왔습니다.

    makes_copy(x);                  // x가 함수 안으로 이동했습니다만,
                                    // i32는 Copy가 되므로, x를 이후에 계속
                                    // 사용해도 됩니다.

} // 여기서 x는 스코프 밖으로 나가고, s도 그 후 나갑니다. 하지만 s는 이미 이동되었으므로,
  // 별다른 일이 발생하지 않습니다.

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
  // 해제되었습니다.

fn makes_copy(some_integer: i32) { // some_integer이 스코프 안으로 들어왔습니다.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.
```
만일 s를 takes_ownership 함수를 호출 이후에 사용하려 한다면, 컴파일 타임 오류가 발생합니다. 이러한 정적 확인은 여러 실수들을 방지해줍니다.

반환 값과 스코프
```
fn main() {
    let s1 = gives_ownership();         // gives_ownership은 반환값을 s1에게 이동시킵니다.

    let s2 = String::from("hello");     // s2가 스코프 안에 들어왔습니다.

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back 안으로 이동되었고,
                                        // 이 함수가 반환값을 s3으로도 이동시켰습니다.

} // 여기서 s3는 스코프 밖으로 벗어났으며 drop이 호출됩니다. 
  // s2는 스코프 밖으로 벗어났지만 이동되었으므로 아무 일도 일어나지 않습니다. 
  // s1은 스코프 밖으로 벗어나서 drop이 호출됩니다.

fn gives_ownership() -> String {             // gives_ownership 함수가 반환 값을 호출한 쪽으로 이동시킵니다.

    let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.

    some_string                              // some_string이 반환되고, 호출한 쪽의 함수로 이동됩니다.
}

// takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환합니다.
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로 들어왔습니다.

    a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
}
```
값의 반환 또한 소유권을 이동시킵니다.