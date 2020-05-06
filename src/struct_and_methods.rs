#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    height: i32,
    shoes_size: i32,
}

impl User {
    fn simple_string(&self) -> String {
        format!("{} - {} - {}cm - shoes:{}",
        self.name,
        self.age,
        self.height,
        self.shoes_size)
    }

    fn grow(&mut self, h: i32) {
        self.height += h;
    }

    fn die(self) {
        println!("Dead {}", self.simple_string());
    }
}

pub(crate) fn create_new_user() {
    let mut u = User {
        name: "Mark".to_string(),
        age: 33,
        height: 250,
        shoes_size: 10,
    };

    println!("User is {}", u.simple_string());
    u.grow(20);
    println!("User is {}", u.simple_string());
    u.die();
}