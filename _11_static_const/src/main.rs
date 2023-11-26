// const 를 쓰게 되면 컴퓨터가 유추하지않는다. -> 하게 되면 절대 이 이상은 초과할 수 없으니 좋다.
// const는 대문자로 써야한다. main안에도 쓸 수 있지만, 보통 main 밖에 쓴다. 
// 이유는 main에 선언하면 main에만 쓸 수 있기 때문이다.
const NUMBER: i32 = 20;
// static
// Static은 같은 메모리에서 쓴다는 보장이 있다.
// mut도 가능하지만, unsage이므로 잘 안쓴다.
static mut LOW_SCORE : i32 = 0; // unsafe
// 'Static lifetime 이면,
static S: &'static str = "hello world";

fn print_high_score() {
    println!("the high score is {}", NUMBER);
    println!("{}",S);
}

fn main() {
    print_high_score();    
    unsafe {LOW_SCORE = 1;}  // unsafe는 안쓰는게 맞다. 필요하다고 생각하면, 니가 코딩을 ㅈ같이 짠거임
}
