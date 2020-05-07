pub fn match_four(x: i32) -> i32 {

    match x {
        1 => return 1,
        2 => return 2,
        3 => return 3,
        4 => return 4,
        _ => 0
    }
}

#[test]
fn return_four() {
    println!("Runned");
    assert_eq!(4, match_four(4));
}

pub fn return_five() -> i32 {
    let mut x = 0;
    for i in 1..9 {
       if i % 5 == 0 {
           x = i;
       } 
    }
    return x;
}

#[test]
fn returned_five() {
    assert_eq!(5, return_five());
}

#[test]
fn result_is_nine() {
    assert_eq!(9, return_five() + match_four(4));
}