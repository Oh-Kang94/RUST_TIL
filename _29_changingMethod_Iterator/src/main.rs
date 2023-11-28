// Chanining methods and functional style

//EXEC03
#[derive(Debug)]
struct Library {
    lib_type: LibraryType,
    books: Vec<String>,
}
#[derive(Debug)]
enum LibraryType {
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }
    fn new() -> Self {
        Self { lib_type: LibraryType::City, books: Vec::new() }
    }
}

impl Iterator for Library{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.books.pop(){
            Some(book) => Some(book + " is found!"), // String + &str
            None => None,
        }
    }
    
}

fn main() {
    println!("\nEXEC01");
    // EXEC01 -> Chaining Mehtod
    /* 일반적인 코드 스티알링
        let mut new_vec = Vec::new();
        let mut counter = 1;

        while counter < 11{
            new_vec.push(counter);
            counter +=1;
        }

        println!("{new_vec:?}");
    */

    let new_vec = (1..=10).collect::<Vec<i32>>();
    println!("{new_vec:?}");

    let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_vec = my_vec
        .into_iter() // 값을 소유하는 iterator로 들어간다고 선언
        .skip(3)
        .take(4)
        .collect::<Vec<i32>>();
    println!("{new_vec:?}");

    println!("\nEXEC02");
    // EXEC02 -> Iterator : a Collection of things that you can call .next() on
    /*
        .iter() -> iterator of referencs &T
        .iter_mut() -> iterator of mutable refs & mut T
        .into_iter() -> consuming iterator : ref안봐도되고 상황에 따라서 가장 많이 편하고, 쓰임.
     */
    let vec1 = vec![1, 2, 3];
    let vec_a = vec1
        .iter()
        .map(|x| x + 1) // map안 에서는 X라는 변수에 대해 조종을 할 수 있다.
        .collect::<Vec<i32>>();
    let vec_b: Vec<i32> = vec1
        .into_iter()
        .map(|x| x * 10)
        .collect();
    let mut vec2 = vec![10, 20, 30];
    vec2.iter_mut().for_each(|num| {
        *num += 100;
    });

    println!("{vec_a:?}");
    println!("{vec_b:?}");
    println!("{vec2:?}");
    // println!("{vec1:?}"); -> // into_iter에 대해 소모되어서 소유권때문에 불러오는게 불가능

    println!("\nEXEC03");
    // EXEC03 -> assert_eq! : 같다는 것을 알려주는 비교연산 그리고, 안되면 Panicked된다.
    // 왜쓰냐? : 테스트 코드를 작성할 때 쓰이게 된다.
    let my_vec = vec!['a', 'b', '거', '國'];
    let mut my_vec_iter = my_vec.iter();

    assert_eq!(my_vec_iter.next(), Some(&'a')); // 'a'를 기대하고 아니면, panicked 된다.
    assert_eq!(my_vec_iter.next(), Some(&'b'));
    assert_eq!(my_vec_iter.next(), Some(&'거'));
    assert_eq!(my_vec_iter.next(), Some(&'國'));
    assert_eq!(my_vec_iter.next(), None);

    println!("\nEXEC03");
    // EXEC03 -> Impl Iterator
    let mut my_lib = Library::new();
    my_lib.add_book("반지의 제왕");
    my_lib.add_book("흥부와 놀부");
    my_lib.add_book("구운몽");
    my_lib.add_book("Demian");

    println!("{my_lib:?}");

    for item in my_lib {
        println!("{item}")
    }
    
}
