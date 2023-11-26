fn main() {
    // EXEC 01 Basic Loop
    let mut counter = 0;

    loop {
        counter += 1;
        println!("{}", counter);
        if counter == 5 {
            break;
        }
    }
    println!("\n");

    // EXEC 02  : 이중 LOOP 문
    let mut counter = 0;
    let mut counter2 = 0;

    'first_loop: loop {
        // GIVE THE FIRST LOOP A NAME
        counter += 1;
        println!("FIRST LOOP {}", counter);
        if counter > 9 {
            // STARTS A SECOND LOPP INSIDE THIS LOOP
            println!("Now Entering the Second_loop");
            'Second_loop: loop {
                // NOW WE ARE INSIDE 'SECOND_LOOP
                println!("SECOND LOOP {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop;
                }
            }
        }
    }

    println!("\nEXEC03");
    // EXEC03 : Counter 없이 짜는 LOOP
    let mut counter = 0;
    while counter != 5 {
        counter += 1;
        println!("{}", counter);
    }

    println!("\nEXEC04");
    // EXEC04 : Range 있는 LOOP => For 문
    for number in 0..3 {
        // Exclusive Range
        println!("The number is {}", number);
    }

    println!("\nEXEC05");
    // EXEC05 : input에 관련된거 없이 쓰게 하고싶으려면,
    // 05-1
    for _ in 0..3 {
        // Exclusive Range
        println!("The number is");
    }
    // 05-2
    for _number in 0..3 {
        // Exclusive Range
        println!("The number is");
    }

    // EXEC06 : Return값을 보내면서 Break하기
    println!("\nEXEC06");
    let mut counter = 5;
    loop {
        counter +=1;
        if counter %53 == 3{
            break println!("my number is {}",counter);
        }
    };
}
