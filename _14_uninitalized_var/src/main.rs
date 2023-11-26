// uninitalized var
// control flow

fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        // while 문이라 생각하셈
        counter += 1;
        if counter % 50 == 0 {
            break;
        }
    }
    counter
}

// possibly uninitialized = maybe doesn't have a value yet
fn main() {
    let my_num = {
        // complexing sth
        let x = 9;
        x + 9

    };
    
    let my_number;
    {
        let x = loop_then_return(12);
        my_number = x
    }

    println!("{}", my_number);
    println!("{}", my_num);
}
