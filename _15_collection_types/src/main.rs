// Collection types
// Array
// [&str; Number]

// Slices
// Arr의 범위 지정에 쓰인다.

// Vecs -> array보다는 느리지만, 편리하다.
// String도 Vec으로 이루어져 있다.

// tuples
// tuple은 여러 타입이든지 다 받아 준다.

// Destruructing
// 튜플(tuple)이나 배열(array)의 요소를 개별 변수로 분해하는 과정
// Structure

fn main() {
    let array = ["One", "Two"];
    let array2 = ["One", "Two", "Three"];
    let array3 = ["One", "Two", "Five"];

    // array 와 array2는 다른 타입이다.
    // println!("Is array the same as array2? {}",array == array2);
    println!("Is array2 the same as array3? {}", array2 == array3);
    // array.tasjdnak();  -> 어떤타입인지 알수 있는 가장 쉬운 방법. .찍고 아무거나 적으면
    //                          그타입에 대한 것을 알려준다.

    let array = [0; 640]; // -> 0을 640개 적어 달라는 것이다.
    println!("{:?}", array); // 디버깅 형식으로 출력하는데 사용

    let mut month_arr = Vec::new();
    for i in 1..=12 {
        month_arr.push(format!("{}월", i));
    }
    println!("{:?}", month_arr[11]);
    println!("{:?}", month_arr.get(0));
    println!("{:?}", month_arr.get(15));

    //Slices
    let seasons = ["봄", "여름", "가을", "겨울"];
    // println!("{:?}", seasons[0..2]); 이렇게 쓰면 안되고, &을 써야한다.
    println!("{:?}", &seasons[0..2]);
    println!("{:?}", &seasons[0..=2]); // 2 포함
    println!("{:?}", &seasons[..]); // 2 포함
    println!("{:?}", &seasons[1..]); // 2 포함
    println!("{:?}", &seasons[..3]); // 2 포함

    // Vecs -> array보다는 느리지만, 편리하다.
    // String도 Vec으로 이루어져 있다.
    // ex) Vec<u8>, Vec<String>
    // array는
    let arr: [&str; 3];
    let arr: Vec<&str>;
    let mut my_vec = Vec::new();
    println!("cap : {}", my_vec.capacity());
    let name1 = String::from("windy");
    let name2 = String::from("Gomesy");
    my_vec.push(&name1);
    println!("cap : {}", my_vec.capacity());
    my_vec.push(&name2);
    println!("{:?}", my_vec);
    println!("cap : {}", my_vec.capacity());

    // 가장 많이 쓰는 방법
    // macro를 이용한 vec 쓰기
    let my_vec_macro = vec![name1, name2];
    println!("{:?}", my_vec_macro);

    // tuple
    // tuple은 여러 타입이든지 다 받아 준다.
    let my_tuple = (8, "Oh-Kang", my_vec_macro);
    println!("{:?}", my_tuple);
    println!("{}", my_tuple.0);
    println!("{}", my_tuple.1);
    println!("{:?}", my_tuple.2);

    // let my_vec_wTuple = vec![("hey",1),(12,"ㅎㅇ")]; // 이건 안된다. 순서를 지키면 가능
    let my_vec_wTuple = vec![("hey", 1), ("ㅎㅇ", 12)];

    // Destruructing
    // : 튜플(tuple)이나 배열(array)의 요소를 개별 변수로 분해하는 과정
    // Structure

    let str_tuple = ("one", "two","three");
    let (a,오,_) = str_tuple;
    println!("Item a is : {}, {}", a,오);
    let str_tuple = ["one", "two","three"];
    let [a,오,_] = str_tuple;
    println!("Item a is : {}, {}", a,오);
}
