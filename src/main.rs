mod syntax;
mod basic_loops;
mod strings;
mod struct_and_methods;

fn main() {
    syntax();
    basic_loops();
    strings();
    struct_and_methods();
}

fn syntax() {
    let b = syntax::highest(14, 12, 18);
    println!("{} is highest", b);
    format!("{} is highest", b);

    let o = syntax::other(15, 19);
    println!("{} is other", o)
}

fn basic_loops() {
    basic_loops::loop_to_10();
    basic_loops::loop_to_9();
    basic_loops::array_loop();
    basic_loops::array();
}

fn strings() {
    strings::strings_chars_and_bytes();
}

fn struct_and_methods() {
    struct_and_methods::create_new_user();
}
