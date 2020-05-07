fn match_four(x: i32) -> String {

    match x {
        1 => return "One",
        2 => return "Two",
        3 => return "Three",
        4 => return "Four",
        _ => "None",
    }
}

#[test]
fn return_four() {
    println!("Runned");
    assert_eq!("Two", match_four(4));
}