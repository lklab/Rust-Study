use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut list = vec![1, 2, 3];
    println!("new list: {:?}", list);

    do_something(true, || {
        list.push(5);
        println!("hello {:?}", list);
    });

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        sort_operations.push(num_sort_operations);
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);

    println!("\n========== iterators ==========\n");

    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    v1_iter.next();
    for v in v1_iter {
        println!("{}", v);
    }

    let v2: Vec<_> = v1.iter().map(|x| x + 1).map(|x| x + 1).collect();
    println!("two maps: {:?}", v2);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(&shoes, 10);
    let in_my_size_iter = shoes_in_size_iter(shoes.iter(), 10);
    println!("shoes: {:?}", shoes);
    println!("in_my_size: {:?}", in_my_size);
    println!("in_my_size_iter: {:?}", in_my_size_iter);
}

fn do_something<F>(check: bool, mut f: F) where F: FnMut() {
    if check {
        f();
    }
    f();
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// &[Shoe] == &Vec<Shoe>
fn shoes_in_size(shoes: &[Shoe], shoe_size: u32) -> Vec<&Shoe> {
    shoes.iter().filter(|s| s.size == shoe_size).collect()
}

fn shoes_in_size_iter<'a>(shoes: impl Iterator<Item = &'a Shoe>, shoe_size: u32) -> Vec<&'a Shoe> {
    shoes.filter(|s| s.size == shoe_size).collect()
}
