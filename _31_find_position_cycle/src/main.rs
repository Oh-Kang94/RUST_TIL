// find -> Option<Self::Item> : 찾아보고, 있으면 주고, 없으면 None
// position -> Option<usize> : 위치값을 찾아봐 준다.
// cycle -> 끝나지 않는 iterator
// (1..3).iter().cycle()
fn main() {
    println!("\nEXEC01");
    // EXEC01 ->
    let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    println!(
        "{:?}",
        num_vec.iter().find(|&n| n % 3 == 0)
    );
    println!(
        "{:?}",
        num_vec.iter().position(|&n| n % 3 == 0) // Vec에서 답인 index를 알려준다.
    );
    let even_odd= vec!["even", "odd"].into_iter().cycle();
    let even_odd_vec : Vec<(i32,&str)>= (0..18)
        .zip(even_odd)
        .take(6) // 6개만 가져가겠다.
        .collect();
    println!("{:?}", even_odd_vec)
}   
