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

/********** 15.4 Rc **********/
enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    NilRc,
}

impl Drop for ListRc {
    fn drop(&mut self) {
        println!("Dropping ListRc");
    }
}

struct Owner {
    name: String
    // ...other fields
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>
    // ...other fields
}

use ListRc::{ConsRc, NilRc};
use std::rc::{Rc, Weak};

fn main_15_4() {
    println!("\n********** 15.4 **********");

    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    println!("count after creating a = a:{}", Rc::strong_count(&a));
    let b = Rc::new(ConsRc(3, Rc::clone(&a)));
    println!("count after creating b = a:{}, b:{}",
             Rc::strong_count(&a),
             Rc::strong_count(&b));
    {
        let c = Rc::new(ConsRc(4, Rc::clone(&a)));
        println!("count after creating c = a:{}, b:{}, c:{}",
                 Rc::strong_count(&a),
                 Rc::strong_count(&b),
                 Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = a:{}, b:{}",
             Rc::strong_count(&a),
             Rc::strong_count(&b));

    let gadget_owner : Rc<Owner> = Rc::new(
        Owner { name: String::from("Gadget Man") }
    );

    let gadget1 = Gadget { id: 1, owner: gadget_owner.clone() };
    let gadget2 = Gadget { id: 2, owner: gadget_owner.clone() };

    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    let owner = &gadget1.owner;
    println!("owner '{}' borrowed", (*owner).name);
    println!("owner '{}' borrowed", owner.name);
    print_owner(&*gadget1.owner);
    print_owner(&gadget1.owner);
}

fn print_owner(owner: &Owner) {
    println!("owner '{}' borrowed", owner.name);
}

/********** 15.5 RefCell **********/
#[derive(Debug)]
enum ListRefCell {
    ConsRefCell(Rc<RefCell<i32>>, Rc<ListRefCell>),
    NilRefCell,
}

use ListRefCell::{ConsRefCell, NilRefCell};
use std::cell::RefCell;

fn main_15_5() {
    println!("\n********** 15.5 **********");

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(ConsRefCell(Rc::clone(&value), Rc::new(NilRefCell)));

    let b = ConsRefCell(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = ConsRefCell(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 100;
    {
        let mut d = value.borrow_mut();
        *d += 10;
    }

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

/********** 15.6 Reference Cycles **********/
use ListCycle::{ConsCycle, NilCycle};

#[derive(Debug)]
enum ListCycle {
    ConsCycle(i32, RefCell<Rc<ListCycle>>),
    NilCycle,
}


impl ListCycle {
    fn tail(&self) -> Option<&RefCell<Rc<ListCycle>>> {
        match self {
            ConsCycle(_, item) => Some(item),
            NilCycle => None,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    value: i32,
    name: String,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn print(&self) {
        let parent = match self.parent.borrow().upgrade() {
            Some(node) => format!("{}", node.value),
            None => String::from("none"),
        };

        let children_vec = &*self.children.borrow();

        let children = if children_vec.len() == 0 {
            String::from("none")
        } else {
            children_vec.iter()
                .map(|node| format!("{}", node.value))
                .collect::<Vec<String>>()
                .join(", ")
        };

        println!("[{}] value = {}, parent = {}, children = {}",
             self.name,
             self.value,
             parent,
             children);
    }
}

fn main_15_6() {
    println!("\n********** 15.6 **********");

    let a = Rc::new(ConsCycle(5, RefCell::new(Rc::new(NilCycle))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(ConsCycle(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        name: String::from("leaf"),
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    leaf.print();

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            name: String::from("branch"),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        leaf.print();
        branch.print();

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        let leaf2 = Rc::new(Node {
            value: 7,
            name: String::from("leaf2"),
            parent: RefCell::new(Rc::downgrade(&branch)),
            children: RefCell::new(vec![]),
        });

        branch.children.borrow_mut().push(Rc::clone(&leaf2));

        leaf2.print();
        branch.print();

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

/********** Main **********/
fn main() {
    main_15_1();
    main_15_2();
    main_15_3();
    main_15_4();
    main_15_5();
    main_15_6();
}
