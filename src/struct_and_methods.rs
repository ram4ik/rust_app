#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    height: i32,
    shoes_size: i32,
}

pub(crate) fn create_new_user() {
    let u = User {
        name: "Mark",
        age: 33,
        height: 250,
        shoes_size: 10,
    };

    println!("User is {:?}", u);
}