pub(crate) fn strings_chars_and_bytes() {

    let s = String::from("Hello, World!");

    println!("Length is -> {}", s.len());
    println!("Length is -> {}", count_l(&s));

    for (i, c) in s.chars().enumerate() {
        println!("{} = {}", i, c)
    }

    for c in s.bytes() {
        println!("{}", c);
    }
    println!("Square of two is {}", square(2));
}

fn count_l(s:&str) -> i32 {
    let mut res = 0;
    for c in s.chars() {
        if c == 'l' {
            res += 1;
        }
    }
    res
}

#[test]
fn string_contain_two_l() {
    let s = String::from("This is for test purpuse: Color is purple.");

    assert_eq!(2, count_l(&s));
}

fn square(num: i32) -> i32 {
    num * num
}

#[test]
fn square_of_two_is_four() {
    assert_eq!(4, square(2));
}