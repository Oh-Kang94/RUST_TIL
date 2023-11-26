// trait -> 초능력 같은것
// This type implements (trait name)
// 

// from, into
fn main() {
    let my_name = String::from("Oh-Kang");
    let my_city: String = "Seoul".into(); // &str

    println!("{}",my_city);

    let my_vec = Vec::from([8,9,10]);
    print!("{:?}",my_vec.binary_search(&8));
}
