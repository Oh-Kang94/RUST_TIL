// traits -> Traits는 보통 동사로 되어 있다.

/*
    struct -> Thing
    enum -> choice 선택에 관련
    trait -> Verb /adjectives 움직임에 관련
*/

/*  Trait와 Method의 차이
    method는 단순히 .으로 쓰는 function이고, 
    trait는 대부분 method를 몇 가지 갖고 있어요. 
    일반적으로 trait를 만드실 때 trait이름 정해 놓고, 그 다음에 어떤 method가 있을지 씁니다.
    즉
    - method는 .으로 쓰는 function (&self, &mut self, self가 있으면 .으로 씀)
    - trait 없어도 어떤 struct나 enum을 위한 method를 쓸 수 가 있지만,
    - trait 안에 method를 쓰면 그 trait를 쓰는 모든 타입이 확실하게 그 method를 쓸 수가 있음
*/

//EXEC01
use std::fmt::{ Debug, Display, Formatter, Result };

#[derive(Clone, Copy, Debug)]
struct MyStruct {
    number: usize,
}

// fn print_as_debug<T: Debug>(input: T) {
//     println!("{input:?}")
// }

fn print_as_debug<T>(input: T) where T: Debug {
    println!("{input:?}")
}

//EXEC01
pub trait Add {
    fn add(&self, other: &Self) -> Self;
}

#[derive(Debug)]
struct ThingsToAdd {
    first: u32,
    second: f32,
}

impl Add for ThingsToAdd {
    fn add(&self, other: &Self) -> Self {
        ThingsToAdd {
            first: self.first + other.first,
            second: self.second + other.second,
        }
    }
}

// EXEC02
#[derive(Debug)]
struct Animal {
    name: String,
}

trait Canine {
    // dog-like
    fn bark(&self) {
        println!("BowWow!")
    }
    fn run(&self) {
        println!("I am running!")
    }
}

impl Canine for Animal {
    fn bark(&self) {
        println!("멍멍!{:?}", self.name);
    }
}

// EXEC03
#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

impl Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let age = self.age;
        write!(f, "My cat name is {name:?}, and, he is {age:?}")
    }
}

// EXEC04 -> Custom trait
struct Monster {
    health: i32,
}

struct Wizard {}
struct Ranger {}

trait FightClose {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!("You attack with sword!, oppponent's health is now {}", opponent.health)
    }
    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!("You attack with fist!, oppponent's health is now {}", opponent.health)
    }
}

impl FightClose for Wizard {}
impl FightClose for Ranger {}
trait FightFromDistance {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u8) {
        if distance < 10 {
            opponent.health -= 10;
            println!("You attack with bow!, oppponent's health is now {}", opponent.health)
        }
    }
    fn attack_with_rock(&self, opponent: &mut Monster, distance: u8) {
        if distance < 3 {
            opponent.health -= 4;
            println!("You attack with rock!, oppponent's health is now {}", opponent.health)
        }
    }
}

impl FightFromDistance for Ranger {}

// EXEC05
#[derive(Debug)]
struct Monster2 {
    health: i32,
}

#[derive(Debug)]
struct Wizard2 {
    health: i32,
}

#[derive(Debug)]
struct Ranger2 {
    health: i32,
}

trait Magic {}
trait Fight {}
trait Shoot {}

impl Magic for Wizard2 {}
impl Fight for Ranger2 {}
impl Shoot for Ranger2 {}

fn attack_with_bow<T>(character: &T, opponent: &mut Monster2, distance: u32) where T: Shoot + Debug {
    if distance < 10 {
        opponent.health -= 10;
        println!(
            "You attack with bow. Your oppponent now has {} health left.\n You are now at {character:?}",
            opponent.health
        )
    }
}

fn attack_with_sword<T>(character: &T, opponent: &mut Monster2) where T: Fight + Debug {
    opponent.health -= 10;
    println!(
        "You attack with sword. Your oppponent now has {} health left.\n You are now at {character:?}",
        opponent.health
    )
}

fn attack_with_fireball<T>(character: &T, opponent: &mut Monster2, distance: u32)
    where T: Magic + Debug
{
    if distance < 15 {
        opponent.health -= 20;
        println!(
            "You attack with fireball. Your oppponent now has {} health left.\n You are now at {character:?}",
            opponent.health
        )
    }
}

// EXEC06
fn print_vec<T>(input: &Vec<T>) where T: Display {
    for item in input {
        print!("{item} ");
    }
    println!();
}

// EXEC07
#[derive(Debug)]
struct City {
    name: String,
    population: u32,
}
#[derive(Debug)]
struct Country {
    cities: Vec<City>,
}

impl City {
    fn new(name: &str, population: u32) -> Self {
        Self { name: name.to_string(), population }
    }
}

impl From<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Self {
        Self { cities }
    }
}

impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!("{:?} has a population of {:?}", city.name, city.population);
        }
    }
}

// EXEC08

trait Prints {
    // :Debug를 하게 되면, Debug 특성이 있는 애들만 가능
    fn prints_sth(&self) where Self: Debug {
        println!("I like to print things {:?}", self)
    }

    fn display_sth(&self) where Self: Display {
        println!("I like to print things {}", self)
    }
}
#[derive(Debug)]
struct Person;

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Person struct")?;
        Ok(())
    }
}


#[derive(Debug)]
struct Building;

impl<T> Prints for T where T: Debug + Display {} // Debug를 impl 한 T만 가능

// EXEC09
trait Printsth {
    fn print_sth(&self){
        // println!("I am a {:?}", self); // 이러면 해당이 되지 않는다. 즉, Debug 속성을 부여하면 안된다.
        /*
            impl<T> Printsth for T where T : Debug{} 정확히 이거에 해당이 안됨
         */
        println!("I am sth");
    }
}

struct PersonSilly;
struct BuildingSilly;

impl<T> Printsth for T{
    
}


// EXEC10
// AsRef
fn print_in<T>(input: T) where T: Display{
    println!("{input}")
}

// 이러면 숫자를 넣을 수 없음.
fn print_str<T>(input: T) where T: Display+ AsRef<str>{
    println!("{input}")
}

fn main() {
    println!("\nEXEC01");
    // EXEC01 ->
    let first_thing = ThingsToAdd {
        first: 32,
        second: 8.8,
    };
    let second_thing = ThingsToAdd {
        first: 32,
        second: 8.8,
    };
    let sum = first_thing.add(&second_thing);
    // 그냥 하면 모른다. Trait으로 명시가 필요
    println!("{sum:?}");

    println!("\nEXEC02");
    // EXEC02 -> trait과 impl의 순서 관계 및 활용
    let my_animal = Animal {
        name: "Dog".to_string(),
    };
    my_animal.bark();
    // trait에 직접 적은것 보다 impl에 있는게 더 우선되서 발동이 된다.
    my_animal.run();

    println!("\nEXEC03");
    // EXEC03 -> trait과 impl의 순서 관계 및 활용
    let mr_mantle = Cat {
        name: "냐용이".to_string(),
        age: 4,
    };
    println!("{mr_mantle}");
    // impl로 설정한 값이 print된다. fmt::Display
    println!("{mr_mantle:?}");
    // 하지만, 이렇게 쓰면, Debug의 힘을 빌려서 쓴거여서
    // Cat { name: "냐용이", age: 4 } 이런식으로 작성이 된다.

    println!("\nEXEC04");
    // EXEC04 -> Custom Trait
    let radagast = Wizard {};
    let aragorn = Ranger {};
    let mut uruk_hai = Monster { health: 30 };

    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attack_with_bow(&mut uruk_hai, 16);

    println!("\nEXEC05");
    // EXEC05 -> trait as bounds
    // fn을 쓰고 싶으면, 관련된 특성을 확인해야만 쓸 수 있다는 것을 알면 된다.
    let radagast = Wizard2 {
        health: 60,
    };
    let aragorn = Ranger2 {
        health: 80,
    };
    let mut uruk_hai = Monster2 { health: 40 };

    attack_with_sword(&aragorn, &mut uruk_hai);
    attack_with_fireball(&radagast, &mut uruk_hai, 5);
    attack_with_bow(&aragorn, &mut uruk_hai, 5);

    println!("\nEXEC06");
    // EXEC06 -> Implementing Form
    let array_vec = Vec::from([8, 9, 10]);
    print_vec(&array_vec);

    let str_vec = Vec::from(["a", "b", "c"]);
    print_vec(&str_vec);
    let string_vec = Vec::from(["abc"]);
    print_vec(&str_vec);

    println!("\nEXEC07");
    // EXEC07 -> Custom Implementing Form
    let helsinki = City::new("Helsinki", 631_695);
    let turku = City::new("turku", 123_123_123);

    let fin_cities = vec![helsinki, turku];
    let finland = Country::from(fin_cities);
    // let finland: Country = fin_cities.into(); // -> 이렇게 써도 된다. 근데,from을 하면 into의 기능이 포함이다.
    // let x = ...iter().for_each().into();
    finland.print_cities();

    println!("\nEXEC08");
    // EXEC08 -> Blanket trait impl
    // impl a trait을 내가 원하는 타입으로 지정 가능
    let my_person = Person;
    let my_building = Building;
    my_person.prints_sth();
    my_person.display_sth();
    let x = String::from("Hello");
    x.prints_sth();

    println!("\nEXEC09");
    // EXEC09 -> Blanket trait impl
    let person = PersonSilly;
    let building = BuildingSilly;
    person.print_sth();
    building.print_sth();

    println!("\nEXEC10");
    // EXEC10 -> AsRef trait
    print_in(9);
    // print_str(9); -> str trait을 asRef했기 때문에 안된다.
    print_str("9");
}
