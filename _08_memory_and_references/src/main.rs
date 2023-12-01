use std::ops::{Deref, DerefMut};


struct HoldsANumber(u8);

//  Deref
// Smart Pointer 같은 거임
// 
impl Deref for HoldsANumber {
    type Target = u8; // TARGET을 정하게 하는것.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Deref가 있어야만 가능 한 것이다.
impl DerefMut for HoldsANumber{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
    
}

// EXEC
// #[derive(Debug)]
// struct Character {
//     name: String,
//     strength: u8,
//     dexterity: u8,
//     health: u8,
//     intelligence: u8,
//     wisdom: u8,
//     charm: u8,
//     hit_points: i8,
//     alignment: Alignment,
// }

// impl Character {
//     fn new(
//         name: String,
//         strength: u8,
//         dexterity: u8,
//         health: u8,
//         intelligence: u8,
//         wisdom: u8,
//         charm: u8,
//         hit_points: i8,
//         alignment: Alignment,
//     ) -> Self {
//         Self {
//             name,
//             strength,
//             dexterity,
//             health,
//             intelligence,
//             wisdom,
//             charm,
//             hit_points,
//             alignment,
//         }
//     }
// }
// #[derive(Debug)]
// enum Alignment {
//     Good,
//     Neutral,
//     Evil,
// }

// 위코드는 그지 같이 만든거다.
// 정석은
struct Character {
    name: String,
    strength: u8,
    dexterity: u8,
    health: u8,
    intelligence: u8,
    wisdom: u8,
    charm: u8,
    hit_points: i8,
    alignment: Alignment,
}

impl Character {
    fn new(
        name: String,
        strength: u8,
        dexterity: u8,
        health: u8,
        intelligence: u8,
        wisdom: u8,
        charm: u8,
        hit_points: i8,
        alignment: Alignment,
    ) -> Self {
        Self {
            name,
            strength,
            dexterity,
            health,
            intelligence,
            wisdom,
            charm,
            hit_points,
            alignment,
        }
    }
}

enum Alignment {
    Good,
    Neutral,
    Evil,
}

impl Deref for Character { // impl Deref for Character. Now we can do any integer math we want!
    type Target = i8;

    fn deref(&self) -> &Self::Target {
        &self.hit_points
    }
}

fn main() {
    // &
    println!("\nEXEC01");
    // EXEC01 ->
    let my_number = 15;
    let single_ref = &my_number; // Ref to My_number
    let double_ref = &single_ref; // Ref의 Ref도 만들수 있지만, 가끔 나온다.
    let five_ref: &&&&&i32 = &&&&&my_number;

    println!("\nEXEC02");
    // EXEC02 -> DeRef;
    let value = 7;
    let reference = &7;
    println!("{}", value == *reference); // 여기서 *refer 하는게 DeRef이다.

    println!("\nEXEC03");
    // EXEC03 -> DeRef;
    let my_num = HoldsANumber(20);
    println!("{}", *my_num + 20);
    println!("{}", my_num.checked_add(10).unwrap()+20); // 20 +10 +20

    println!("\nEXEC03");
    // EXEC04 DeRef 예시
    let billy = Character::new("Billy".to_string(), 9, 8, 7, 10, 19, 19, 5, Alignment::Good);
    let brandy = Character::new("Brandy".to_string(), 9, 8, 7, 10, 19, 19, 5, Alignment::Evil);

    let mut hit_points_vec = vec![]; // Put our hit points data in here
    println!("{}",*billy-*brandy); // 연산 가능 한건 Deref 때문에
    hit_points_vec.push(*billy);     // Push *billy?
    hit_points_vec.push(*brandy);    // Push *brandy?

    println!("{:?}", hit_points_vec);
}
