// destructuring

//EXEC01,2,3
#[derive(Debug)]
struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

impl Person {
    fn clone_with_name(&self, new_name: String) -> Self {
        Self {
            name: new_name,
            real_name: self.real_name.clone(),
            height: self.height,
            happiness: self.happiness,
        }
    }
}


// EXEC04
#[derive(Debug)]
struct Person2{ 
    name:String,
    height :u8,
}
impl Person2{
    fn from_person(input:Person) -> Self{
        let Person {name, height,..} = input;

        Self{
            name, 
            height,
        }
    }
}

fn main() {
    println!("\nEXEC01");
    // EXEC01 -> 처음 개고생 하는 경우
    let papa_doc = Person {
        name: "PaPa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };
    println!(
        "They call him {}, but his real name is {}. He is {}cm, and is he Happy? {}",
        papa_doc.name,
        papa_doc.real_name,
        papa_doc.height,
        papa_doc.happiness
    );

    println!("\nEXEC02");
    // EXEC02 -> Destructuring 이용 하는 경우
    let Person { name, real_name, height, happiness } = papa_doc.clone_with_name("PaPa Doc".to_string());
    println!(
        "They call him {}, but his real name is {}. He is {}cm, and is he Happy? {}",
        name,
        real_name,
        height,
        happiness
    );

    println!("\nEXEC03");
    // EXEC03 -> Destructuring 이용 하는 경우 (다른 변수 이용)
    let Person { name: a, real_name: b, height: c, happiness: d } = papa_doc.clone_with_name("PaPa Doc".to_string());
    println!(
        "They call him {}, but his real name is {}. He is {}cm, and is he Happy? {}",
        a,
        b,
        c,
        d
    );

    println!("\nEXEC04");
    // EXEC04 -> Struct를 Destucturing 해서, 다른 Struct에 쓰기
    let person2 = Person2::from_person(papa_doc);

    println!("Person2 type is: {:?}", person2)
}
