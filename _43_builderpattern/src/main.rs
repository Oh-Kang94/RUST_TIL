struct SomeStruct;

#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
}

#[derive(Debug)]
struct CharacterBuilder {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
}

#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain,
}

impl CharacterBuilder {
    fn new(name: String, age: u8, height: u32, weight: u32, alive: bool) -> Self {
        Self {
            name,
            age,
            height,
            weight,
            lifestate: if alive {
                LifeState::Alive
            } else {
                LifeState::Dead
            },
        }
    }
    fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    fn with_age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }

    fn with_weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }

    fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    // 여기서, 빌드 할때, 검증해서 넣는다.
    fn build(self) -> Result<Character, String> {
        if
            self.age < 100 &&
            self.height < 200 &&
            self.weight < 300 &&
            !self.name.to_ascii_lowercase().contains("smurf")
        {
            Ok(Character {
                name: self.name,
                age: self.age,
                height: self.height,
                weight: self.weight,
                lifestate: self.lifestate,
            })
        } else {
            Err("제대로 적어".to_string())
        }
    }
}

impl Default for CharacterBuilder {
    fn default() -> Self {
        Self {
            name: "Oh-Kang".to_string(),
            age: 29,
            height: 172,
            weight: 72,
            lifestate: LifeState::Alive,
        }
    }
}

fn main() {
    println!("\nEXEC01");
    //EXEC01 -> Default trait
    let npc_1: CharacterBuilder = CharacterBuilder::new("Billy".to_string(), 15, 170, 70, true);
    println!("{npc_1:?}");

    let npc_2 = CharacterBuilder::default();
    println!("{npc_2:?}");

    println!("\nEXEC02");
    //EXEC02 -> Builder Pattern

    let npc_3 = CharacterBuilder::default()
        .with_age(12)
        .with_height(176)
        .with_weight(73)
        .with_name("name")
        .build();

    match &npc_3 {
        Ok(character) => {
            // OK인 경우 unwrap() 호출하여 값을 언래핑하고 사용
            println!("{:?}", character);
        }
        Err(error) => {
            // Err인 경우 에러 메시지 출력
            println!("{}", error);
        }
    }
}
