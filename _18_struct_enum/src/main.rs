use std::mem::size_of_val;

// struct
// unit struct : 아무것도 없는 이름만 있는 struct
struct FileDirectory; // => bytes가 0이므로 가끔 쓰인다.

fn take_file_dir(input: FileDirectory) {
    println!("I got a file dir")
}

// tuple struct
// Tuple처럼 쓰게 한다.
#[derive(Debug)] // attribute인데, 나중에 배워야한다. 간단히 말해서 debug 할 수 있게 만든다고 만 생각해두셈
struct Color(u8, u8, u8);

// named struct
#[derive(Debug)] // attribute인데, 나중에 배워야한다. 간단히 말해서 debug 할 수 있게 만든다고 만 생각해두셈
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
    // all_pop: [u32; 5500],
}

struct Numbers {
    one: u8,
    two: u8,
    three: u8,
    four: u32,
}

// ENUM
// struct = and라고 한다면,
// enum = or 이라고 생각하면 된다.
// 그 안의 값중 하나만 써야 한다.
enum ThingsInTheSky {
    Sun,
    Stars,
}

fn create_skystate(time: u32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars,
    }
}
fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the Sun"),
        ThingsInTheSky::Stars => println!("I can see the Stars"),
    }
}
//

enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

// fn match_mood(mood: &Mood) -> i32 {
//     let happiness_level = match mood {
//         Mood::Happy => 10,
//         Mood::Sleepy => 8,
//         Mood::NotBad => 6,
//         Mood::Angry => 2,
//     };
//     return happiness_level;
// }

// 이렇게 쓰는 편이 더 편하고 좋다.
fn match_mood(mood: &Mood) -> i32 {
    use Mood::*;
    match mood {
        Happy => 10,
        Sleepy => 8,
        NotBad => 6,
        Angry => 2,
    }
}

// Enum은 숫자로도 간주된다.

enum Season {
    Spring, // 0으로 간주한다.
    Summer,
    Autumn,
    Winter,
}

//

enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowDwarf = 100,
    RedGiant = 1000,
    DeadStar,
}

enum Number{
    Positive(u32),
    CouldBeNegative(i32),
}

fn get_number(input: i32) -> Number {
    use Number::*;
    match input.is_positive() {
        true => Positive(input as u32),
        false => CouldBeNegative(input)
    }
}

fn main() {
    let x = FileDirectory;
    take_file_dir(x);
    let x = FileDirectory;
    println!("The size is {}", std::mem::size_of_val(&x));

    let my_color = Color(20, 50, 100);
    println!("The second color is {}", my_color.1);
    println!("The second color is {:?}", my_color);

    let korea = Country {
        population: 50_000_000,
        capital: "Seoul".to_string(),
        leader_name: "윤".to_string(),
        // all_pop: [3; 5500],
    };
    println!("The country population is {}", korea.population);
    println!("The country capital is {}", korea.capital);
    println!("The country leader_name is {}", korea.leader_name);
    println!("The country leader_name is {:?}", korea);
    // The country leader_name is Country { population: 50000000, capital: "Seoul", leader_name: "윤" }
    println!("The country leader_name is {:#?}", korea);
    /*
        The country leader_name is Country {
            population: 50000000,
            capital: "Seoul",
            leader_name: "윤",
        }
     */

    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justin Trudeau".to_string();
    let your_country = Country {
        // population : population,
        // capital : capital,
        // leader_name : leader
        population,
        capital,
        leader_name,
        // all_pop: [500; 5500], // => Country is 22056 bytes in size
        // 이렇게 도 가능하다.
    };
    println!("{:#?}", your_country);

    println!("Country is {} bytes in size", size_of_val(&your_country));

    let numbers = Numbers {
        one: 8,
        two: 19,
        three: 20,
        four: 30,
    };

    println!("Size is : {}", size_of_val(&numbers));

    let time = 20;
    let sky_state = create_skystate(time);
    check_skystate(&sky_state);
    check_skystate(&create_skystate(8));

    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&my_mood);
    println!("My happiness level is {} ", &happiness_level);

    use Season::*;
    let four_seasons: Vec<Season> = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("The number is : {}", season as u32);
    }

    use Star::*;
    let starvec = vec![BrownDwarf, RedDwarf, YellowDwarf, RedGiant, DeadStar];
    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star : {}", size),
            size if size >= 80 => println!("Prety biggest star : {}", size),
            _ => println!("Dead star")
        }
    }


    let my_vec = vec![get_number(-800), get_number(8)];
    use Number::*;
    for item in my_vec{
        match item {
            CouldBeNegative(number) => println!("It's a i32 with value is Negative, {}",number),
            Positive(number) => println!("It's a u32 with value is Positive {} ", number),
        }
    }
}
