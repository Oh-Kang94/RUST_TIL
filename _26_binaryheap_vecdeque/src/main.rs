use std::collections::{BinaryHeap, VecDeque};

// EXEC01 : BinaryHeap
fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
    let mut remainder_vec = vec![];
    for number in input {
        remainder_vec.push(*number);
    }
    remainder_vec
}
fn main() {
    println!("\nEXEC01");
    // EXEC01 -> BinaryHeap
    // 자주 쓰이는게 Primary queue의 경우, 서버가 먼저 여러가지 request를 주는데, Priority를 줄때 쓰인다.
    let many_nums = vec![0, 5, 10, 15, 20, 25, 30];
    let mut my_heap = BinaryHeap::new();

    for number in many_nums {
        my_heap.push(number);
    }

    while let Some(number) = my_heap.pop() {
        println!("Popped off {}. Remaining number are: {:?}", number, show_remainder(&my_heap));
    }
    /*
        Popped off 30. Remaining number are: [25, 15, 20, 0, 10, 5]
        Popped off 25. Remaining number are: [20, 15, 5, 0, 10]
        Popped off 20. Remaining number are: [15, 10, 5, 0]
        Popped off 15. Remaining number are: [10, 0, 5]
        Popped off 10. Remaining number are: [5, 0]
        Popped off 5. Remaining number are: [0]
        Popped off 0. Remaining number are: []
     */ // 순서는 정리되어 있지 않지만, 순서대로 빠지기는 한다.

    println!("\nEXEC02");
    // EXEC02 -> BinaryHeap
    let mut jobs = BinaryHeap::new();
    jobs.push((100, "I Write back to email from the CEO"));
    jobs.push((80, "Finish the report today"));
    jobs.push((5, "Watch some YouTube"));
    jobs.push((70, "Tell your team members thanks for always working hard"));
    jobs.push((30, "Plan who to hire next for the team"));

    while let Some(job) = jobs.pop() {
        println!("U need to: {}", job.1);
    }

    // .peek()는 빼는게 아니라 그냥 보는것이다.

    println!("\nEXEC03");
    
    // EXEC03 -> VecDeque
    let mut my_vec = vec![0;600_000];
    for i in 0..600_000 {
        my_vec.remove(0);
    }
    // -> 이건 힘든과정이므로 처리가 어려울것이다.

    // VecDeque는 pop_front나 Pop_back중 설정해야함
    // 이걸로 실행하면, vecDeque는 빠르다.
    let mut my_vec = VecDeque::from(vec![0;600_000]);
    for i in 0..600_000 {
        my_vec.pop_front();
    }
}
