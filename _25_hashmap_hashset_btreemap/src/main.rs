// Hashmap => Python의 Dict랑 같음. <Key, Value>
/*
    key, Value,
    Key : String,
    Value : Vec<String>
    land : 나라, 국가
    HashMap<String, Vec<String>>
*/

// BtreeMap =>이진 트리 검색

use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;

// EXEC01
struct CityHash {
    name: String,
    population: HashMap<u32, u32>, // year +population
}
// EXEC02
struct CityBtree {
    name: String,
    population: BTreeMap<u32, u32>, // year +population
}

fn main() {
    println!("\nEXEC01");
    // EXEC01 -> HaspMap 기본 사용
    let mut tallin = CityHash {
        name: "Tallin".to_string(),
        population: HashMap::new(),
    };
    // .insert HashMap에 Key, Value 넣기
    tallin.population.insert(1234, 3_250);
    tallin.population.insert(1999, 13_250);
    tallin.population.insert(2003, 33_250);

    for (year, population) in tallin.population {
        println!("In the year {}, the population is {}", year, population);
    }
    /*  순서가 없다. 그래서 빠르다.
        In the year 1999, the population is 13250
        In the year 1234, the population is 3250
        In the year 2003, the population is 33250
     */

    println!("\nEXEC02");
    // EXEC02 -> BTreeMap 기본 사용 -> 순서대로 나오긴한데 느리다.
    let mut tallin_btree = CityBtree {
        name: "Tallin".to_string(),
        population: BTreeMap::new(),
    };

    // .insert HashMap에 Key, Value 넣기
    tallin_btree.population.insert(1234, 3_250);
    tallin_btree.population.insert(1999, 13_250);
    tallin_btree.population.insert(2003, 33_250);
    for (year, population) in tallin_btree.population {
        println!("In the year {}, the population is {}", year, population);
    }

    println!("\nEXEC03");
    // EXEC03 -> Read HashMap의 Value가져오기
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();
    for city in canadian_cities {
        city_hashmap.insert(city, "canada");
    }
    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }

    println!("{:?}", city_hashmap["Bielefeld"]);
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Bielefeld"));

    println!("\nEXEC04");
    // EXEC04 -> 소유권을 옮기지 않게 하기 위해 &을 써서 값을 부른다.
    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어, 스마일");
    book_hashmap.insert(1, "Eye of the World");
    println!("{:?}", book_hashmap.get(&1)); // 소유권을 옮기지 않게 하기 위해 &을 써서 값을 부른다.

    println!("\nEXEC05");
    // EXEC05 -> 예외 사항 처리
    if book_hashmap.get(&1).is_none() {
        book_hashmap.insert(1, "L'Allemagne Moderne");
    } else {
        println!("Already Got a book");
    }

    //EXEC05-1 : Refactored
    if let Some(book_name) = book_hashmap.get(&1) {
        println!("Already Got a book : {}", book_name);
    } else {
        book_hashmap.insert(1, "L'Allemagne Moderne");
    }

    println!("\nEXEC06");
    // EXEC06 : .entry()
    /*
        Rust의 entry 메서드는 책장에 있는 책을 확인하고, 새 책을 책장에 추가하거나 이미 있는 책을 찾아낼 수 있게 도와줘요.
        만약 책장에 이미 같은 책이 있다면 그 책을 찾아서 주거나, 새로운 책을 책장에 추가함.
     */
    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "섀도우 오브 유어 스마일",
        "Eye of the World",
        "Eye of the World"
    ]; // 일부러 2권 을 마지막에 넣음
    let mut book_hashmap = HashMap::new();

    for book in &book_collection {
        book_hashmap.entry(book).or_insert(0);
    }

    for book in &book_collection {
        let number_of_books = book_hashmap.entry(book).or_insert(0);
        *number_of_books += 1;
    }

    for (book, numbers) in book_hashmap {
        println!("Do we have {}? {}", book, numbers);
    }

    println!("\nEXEC07");
    // EXEC07: .entry() 응용
    let data = vec![
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("male", 5),
        ("female", 1_100)
    ];
    let mut survey_hash = HashMap::new();

    for item in &data {
        survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
    }

    // Refactored
    let mut survey_hash = HashMap::new();
    for (gender, number) in &data{
        survey_hash.entry(gender).or_insert(Vec::new()).push(number);
    }

    for (male_or_femal, numbers) in survey_hash {
        println!("{:?}, {:?}", male_or_femal, numbers);
    }

    println!("\nEXEC08");
    // EXEC08: HashSet
    let many_numbers = vec! [
    94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,51,35,58,80,34,
    8, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,1,  25, 22, 33, 1, 4, 6,
    96,95,37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95, 13, 60, 59,58, 64,55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 
    86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82,80, 16, 61, 57, 14, 11];
    let mut number_hashset = HashSet::new();

    for number in many_numbers{
        number_hashset.insert(number);
    }
    let hashset_length = number_hashset.len();
    println!("There are {} unique numbers, so we are missing {}", hashset_length, 100-hashset_length);

    println!("\nEXEC09");
    // EXEC09 : HASHSet 을 응용해서 자료 뽑기
    let mut missing_vec = vec![];
    for number in 0..100{
        if number_hashset.get(&number).is_none(){
            missing_vec.push(number);
        }
    }

    for number in missing_vec{
        print!("{},", number)
    }

}
 