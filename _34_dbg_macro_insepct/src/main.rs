// dbg! -> debug = quick print
// eprint -> standard error

// pub이면 다른 곳에서 쓸수 있다. 아니면 못씀 mod , client 이렇게 2개가 가능
mod client {
    #[derive(Debug)] //넣어줄때도 있지만, 없으면 struct에 직접 만들어야 한다.
    pub struct InternetClient {
        pub(crate) client_id: u32,
    }
}

use client::InternetClient;

#[derive(Debug)]
struct Customer<'a>{
    money : u32,
    name: &'a str,
    client: &'a InternetClient
}

fn main() {
    println!("\nEXEC01");
    // EXEC01 ->dbg
    let my_num = 9;
    dbg!(my_num); // [main.rs:7] my_num = 9
    println!("{my_num}");
    println!("{my_num}");
    dbg!(1);
    /*  Output => 이렇게 debug랑 출력이랑 분리되어서 보여준다.
        9
        9
        [main.rs:7] my_num = 9
        [main.rs:10] 1 = 1
     */

    println!("\nEXEC02");
    // EXEC02 -> dbg
    // let mut my_num_mut = 9;
    // my_num_mut +=10;

    // let new_vec = vec![8,9,10];

    // let double_vec : Vec<i32> = new_vec
    //     .iter()
    //     .map(|x|x*2)
    //     .collect();
    let mut my_num_mut = dbg!(9);
    dbg!(my_num_mut += 10);
    let new_vec = dbg!(vec![8, 9, 10]);
    let double_vec: Vec<i32> = dbg!(
        new_vec
            .iter()
            .map(|x| x * 2)
            .collect()
    );
    /* 보면 dbg!갇힌거 보여주고  = 결과값을 보여준다.
        [main.rs:29] 9 = 9
        [main.rs:30] my_num_mut += 10 = ()
        [main.rs:32] vec![8, 9, 10] = [
            8,
            9,
            10,
        ]
        [main.rs:34] new_vec.iter().map(|x| x * 2).collect() = [
            16,
            18,
            20,
        ]
     */

    println!("\nEXEC03");
    // EXEC03 -> .inspect
    let new_vec = [8, 9, 10];
    let double_vec: Vec<_> = new_vec
        .iter()
        .inspect(|first_item| {
            dbg!(first_item);
        })
        // .iter()의 값을 하나씩 전달
        // 실제 코드에 영향을 끼치지 않고, 중간에 껴서 행동을 하고 검사를 해준다.
        .map(|x| x * 2)
        .inspect(|first_item| {
            dbg!(first_item);
        })
        .filter(|num| *num > 17)
        .inspect(|first_item| {
            dbg!(first_item);
        })
        .collect();

    dbg!(double_vec);

    println!("\nEXEC04");
    // EXEC04 -> mod, pub, struct
    let client = client::InternetClient{
        client_id: 0
    };

    let customer1 = Customer{
        money:6_789,
        name:"oh-kang",
        client : &client
    };

    println!("{customer1:?}");

}
