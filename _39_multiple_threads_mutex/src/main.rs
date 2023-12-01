// multi threads
// 다른 언어와 달리 진짜 쓰레드를 쓴다.

// RefCell -> Thread unsafe    safe(x).   (Mutex보다 더 빠름)
// Mutex -> Thread safe (o)
//
// Rc - Thread unsafe   unsafe.   safe(x) Rc 가 더 빠름
// Arc - > Thread safe (o)
// Arc has more overhead  뭔가 더 복잡하다는 뜻

use std::{ thread, borrow::BorrowMut, cell::RefCell, sync::{ Mutex, Arc, RwLock } };
trait CoolTrait {
    fn cool_func(&self);
}

// Mutex 이전, multi threads를 안쓰면 이렇게 써도 된다.
// struct OutStruct{
//     data : RefCell<u8>
// }
// impl CoolTrait for OutStruct{
//     fn cool_func(&self) {
//         *self.data.borrow_mut() +=1;
//     }
// }

// Mutex 사용
// Arc : Atomic Reference Counter => OS primitives을 지키게 만들어주며, ThreadSafe로 만들어준다.

#[derive(Debug)]
struct OutStruct {
    data: Arc<Mutex<u8>>,
}
impl CoolTrait for OutStruct {
    fn cool_func(&self) {
        *self.data.lock().unwrap() += 1;
    }
}

// Main
fn main() {
    println!("\nEXEC01");
    // EXEC01 -> Thread 만들기
    // for _ in 0..1_000 { // Set up ten Threads
    //     thread::spawn(|| {
    //         println!("I am printing sth");
    //     });
    //     let x = 8;
    // }

    println!("\nEXEC02");
    // EXEC02 -> JoinHandle 쓰레드가 기다리게 만들기
    let mut join_vec = vec![];
    for _ in 0..10 {
        let join_handle = thread::spawn(|| {
            // Thread 만들기
            println!("I am printing sth");
        });
        join_vec.push(join_handle);
    }
    // choice 1
    // join_vec.into_iter().for_each(|handle| handle.join().expect("join handle didn't work")); // Wait
    // choice 2
    for handle in join_vec {
        handle.join().expect("join handle didn't work");
    }

    println!("\nEXEC03");
    //EXEC03 -> Mutex
    // Mutex 이전, 안되는 예시임.
    // 0을 만들고, 다른 Thread에서 0을 동시에 놀아야해서 문제가 생긴다.
    // 그래서 Mutex가 필요한것이다.
    // let our_struct = OutStruct {
    //     data: RefCell::new(0),
    // };

    // let mut join_vec = vec![];
    // for _ in 0..10 {
    //     let join_handle = thread::spawn(|| {
    //         *our_struct.data.borrow_mut() += 1;
    //     });
    //     join_vec.push(join_handle);
    // }

    // for handle in join_vec {
    //     handle.join().unwrap();
    // }

    let our_struct = OutStruct {
        data: Arc::new(Mutex::new(0)),
    };

    let mut join_vec = vec![];
    for _ in 0..10 {
        let clone = Arc::clone(&our_struct.data); // Arc<Mutex<u8>>
        let join_handle = thread::spawn(move || {
            // take by value -> move를 통해 lifetime이슈를 없엤다.
            *clone.lock().unwrap() += 1;
            println!("There are {} owners", Arc::strong_count(&clone))
            // 소유권이 각자 바뀌게 때문에 이런식으로 자주 바뀌게 되면서, 바뀌게 된다.
        });
        join_vec.push(join_handle);
    }

    for handle in join_vec {
        handle.join().unwrap();
    }
    println!("Our struct is now : {our_struct:?}");
    // Our struct is now : OutStruct { data: Mutex { data: 10, poisoned: false, .. } }
    // poisoned=False는 지금 아무도 소유권이 없다는 것이다.

    println!("\nEXEC04");
    //EXEC04 -> Mutex 심화, try_lock
    let my_mutex = Mutex::new(5);

    let mut mutex_changer = my_mutex.lock().unwrap();
    let other_mutex_changer = my_mutex.try_lock();
    *mutex_changer = 6;

    if let Ok(value) = other_mutex_changer {
        println!("the mutexchanger has : {value}");
    } else {
        println!("Didn't get a lock");
    } // mutex를 mutex_changer가 점유하고 있어서, Didn't get a lock가 뜰것이다.


    drop(mutex_changer);
    println!("{my_mutex:?}");

    println!("\nEXEC05");
    //EXEC05 -> RwLock
    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();

    println!("{read1:?},{read2:?}");
    drop(read1);
    drop(read2);
    let mut write1 = my_rwlock.write().unwrap();
    *write1 = 7;
    drop(write1);
    let read3 = my_rwlock.read().unwrap();
    // rwlock read lock would result in deadlock
    println!("{:?}",*read3);
}
