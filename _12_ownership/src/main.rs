// OWNERSHIP - 소유권
// Rust에서 가장 중요한 개념

// 해당변수의 오너십이 return_str함수에 있어서 메인에서 쓸 수 없다는 의미

// fn return_str() -> &'static String {
//     let country = String::from("대한민국");
//     return &country; // return &String 은 불가능
// }

// & = reference
fn main() {
    // let my_country = return_str();
    // println!("{}", my_country)
}
