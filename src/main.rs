use std::io;
use std::io::Read;
use std::fs::File;
use std::env;

use study::my_module;

extern crate adder;

fn main() {
    let x = 5;
    let y = &x;
    let z = &x;
    //let z = &mut x;

    println!("{}, {}", y, z);

    println!("{}", adder::add(3, 7));

    let v1 = vec![2, 3, 4];
    for num in v1.iter().skip(2) {
        println!("{}", num);
    }

    my_module::do_something();

    let data = read_username_from_file();
    match data {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("{:?}", e),
    }

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
