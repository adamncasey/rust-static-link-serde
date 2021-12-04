use serde::{Serialize};

#[derive(Serialize)]
struct Person {
    name: String,
    age: u8,
}

extern {
    fn cpplib_function();
}

fn main() {
    let person = Person {
        name: String::from("Adam"),
        age: 28
    };

    println!("{}", serde_json::to_string(&person).unwrap());

    unsafe {
        cpplib_function();
    }
}
