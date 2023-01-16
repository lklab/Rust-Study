fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    change(&mut s);
    println!("{}", s);

    println!("{}", first_word(&s[..]));
    println!("{}", first_word(&s));
    println!("{}", first_word("broccoli is delicious"));

    println!("{}", nth_word(0, "broccoli is delicious"));
    println!("{}", nth_word(1, "broccoli is delicious"));
    println!("{}", nth_word(2, "broccoli is delicious"));
    println!("{}", nth_word(3, "broccoli is delicious"));
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn nth_word(n: usize, s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut base: usize = 0;
    let mut count: usize = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if count == n {
                return &s[base..i];
            }
            count += 1;
            base = i + 1;
        }
    }

    let len = bytes.len();

    if count == n {
        if base >= len {
            ""
        }
        else {
            &s[base..]
        }
    }
    else {
        ""
    }
}
