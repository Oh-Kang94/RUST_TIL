// EXEC01
#[derive(Debug)] // 꼭 내부에 있는 것도 있어야 한다.
struct Animal {
    age: u8,
    animal_type: AnimalType,
}
#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

///
impl Animal {
    /// EXEC05
    fn new(age: u8, animal_type: AnimalType) -> Self {
        // function Signature
        // Self 는 Animal을 뜻하는 거다. -> Refacotring에 이점이 있다.
        Self { age, animal_type }
    }
    
    fn new_cat(age: u8) -> Self {
        // function Signature
        // Self 는 Animal을 뜻하는 거다. -> Refacotring에 이점이 있다.
        Self { age, animal_type: AnimalType::Cat }
    }
    fn new_dog(age: u8) -> Self {
        // function Signature
        // Self 는 Animal을 뜻하는 거다. -> Refacotring에 이점이 있다.
        Self { age, animal_type: AnimalType::Dog }
    }

    //EXEC02
    fn print(&self) {
        println!("I am a: {:?}", self)
    }

    // EXEC03 Mut self 이용한
    fn change_to(&mut self) {
        // ENUM은 비교 연산자를 사용 할 수 없다. 패턴 매칭만 가능
        // if self.animal_type == AnimalType::Dog { 
        //     println!("Changed to Cat! Now I am: {:?}", self)
        // }
        // else{
        //     println!("Changed to Dog! Now I am: {:?}", self)
        // }
        match self.animal_type {
            AnimalType::Dog => {
                self.animal_type = AnimalType::Cat;
                println!("Changed to Cat! Now I am: {:?}", self);
            }
            AnimalType::Cat => {
                self.animal_type = AnimalType::Dog;
                println!("Changed to Dog! Now I am: {:?}", self);
            }
        }
    }

    // EXEC06 ENUM impl BLOCKS
    fn check_type(&self){
        use AnimalType::*;
        match self.animal_type{
            Cat=> println!("Animal type is Cat"),
            Dog => println!("Animal type is Dog")
        }
    }
}

// impl은 여러가지 쓸 수 있다.

// EXEC 07 -> ENUM 안에 값을 넣기
#[derive(Debug)]
struct Animal2 {
    age: u8,
    animal_type: AnimalType2,
}
#[derive(Debug)]
enum AnimalType2{
    Cat(String),
    Dog(String),
}

impl Animal2{
    fn new(age :u8, animal_type:AnimalType2) -> Self{
        Self {age, animal_type}
    }
}

impl AnimalType2{
    fn print_name(&self){
        match self {
            AnimalType2::Dog(name) => println!("Animal type is Dog, and he name is {}",name),
            AnimalType2::Cat(name) => println!("Animal type is Cat, and he name is {}",name) 
        }
    }
}

fn main() {
    println!("\nEXEC 01");
    // EXEC01
    let my_animal = Animal::new_cat(15); //  .mehtod를 안쓰는 이유는 Associated Funtion이고, 인스턴스가 없기 때문에
    // 즉, Associated Funtion을 쓰면 instance를 안만들어도 가능하다.
    println!("I made a: {:?}", my_animal);

    println!("\nEXEC 02");
    // EXEC02
    let my_animal = Animal::new_dog(15);
    my_animal.print(); // Syntactic sugars
    // Animal::print(&my_animal); // 이게 귀찮으니까 위처럼 쓰는거다.

    println!("\nEXEC 03");
    // EXEC03 -> Mut self 이용한 impl
    let mut my_animal = Animal::new_dog(15);
    my_animal.change_to();
    my_animal.change_to();

    println!("\nEXEC 05");
    // EXEC05
    use AnimalType::*;
    let my_cat = Animal::new(13,Cat);
    println!("{:?}",my_cat);

    println!("\nEXEC 06");
    // EXEC 06 -> ENUM impl BLOCKS
    my_cat.check_type();

    println!("\nEXEC 07");

    // EXEC 07 -> ENUM impl BLOCKS
    let my_dog = Animal2::new(13,AnimalType2::Dog("Windy".to_string()));
    my_dog.animal_type.print_name();
}
