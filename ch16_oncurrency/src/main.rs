/********** 16.1 Threads **********/
use std::thread;
use std::time::Duration;

fn main_16_1() {
    println!("\n********** 16.1 **********");

    let mut handles = vec![];

    for i in 1..10 {
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(1));
            println!("number is {} from thread.", i);
        });
        handles.push(handle);
    }

    println!("all threads are created.");

    for handle in handles {
        handle.join().unwrap();
    }

    println!("all threads are terminated.");
}

/********** 16.2 Message Passing **********/
use std::sync::mpsc;

fn main_16_2() {
    println!("\n********** 16.2 **********");

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let handle1 = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    let handle2 = thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    let handle3 = thread::spawn(move || {
        for received in rx {
            println!("Got: {}", received);
        }
    });

    println!("all threads are created.");
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    println!("all threads are terminated.");
}

/********** 16.3 Shared-State **********/
use std::sync::{Arc, Mutex};

fn main_16_3() {
    println!("\n********** 16.3 **********");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

/********** 16.4 Sync and Send Traits **********/
struct Character {
    name: String,
    // value: Rc<i32>, // this is not allowed.
}

impl Character {
    fn change_name(&mut self, name: String) {
        self.name = name;
    }
}

fn main_16_4() {
    println!("\n********** 16.4 **********");

    let mutex =
        Arc::new(
        Mutex::new(
            Character {
                name: String::from("Marine"),
    }));
    let mutex2 = Arc::clone(&mutex);

    let num = Arc::new(5);
    let num2 = Arc::clone(&num);

    let handle = thread::spawn(move || {
        let mut data = mutex2.lock().unwrap();
        data.change_name(String::from("Guard man"));
        println!("haha: {}", num2);
    });

    thread::sleep(Duration::from_millis(1));

    {
        let data = mutex.lock().unwrap();
        println!("character's name is {}", data.name);
    }

    handle.join().unwrap();
}

/********** Main **********/
fn main() {
    main_16_1();
    main_16_2();
    main_16_3();
    main_16_4();
}
