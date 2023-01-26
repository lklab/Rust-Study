/********** 15.1 Box<T> **********/
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main_15_1() {
    println!("\n********** 15.1 **********");

    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(
        Cons(2, Box::new(
            Cons(3, Box::new(
                Nil))))));

    println!("{:?}", list);
}

/********** 15.2 Deref **********/
use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);
struct  MyBoxMut<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> MyBoxMut<T> {
    fn new(x: T) -> MyBoxMut<T> {
        MyBoxMut(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Deref for MyBoxMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBoxMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main_15_2() {
    println!("\n********** 15.2 **********");

    let x = 5;
    let y = MyBox::new(x);

    println!("{}, {}, {}", x, *y, *(y.deref()));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);

    let m = String::from("hello");
    let mut mut_box = MyBoxMut::new(m);
    mut_box.push_str(", world");
    println!("{}", *mut_box);
}

/********** 15.3 Drop **********/
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main_15_3() {
    println!("\n********** 15.3 **********");

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointer created.");

    {
        let b = Box::new(c);
        println!("CustomSmartPointer Box created. {}", b.data);
    }

    println!("End of main.");
}

/********** Main **********/
fn main() {
    main_15_1();
    main_15_2();
    main_15_3();
}
