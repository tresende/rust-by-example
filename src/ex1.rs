struct Person {
    age: i32,
    name: String,
}

impl Person {
    fn hello(&self) -> String {
        format!("my name is {} and i am {} years ", self.name, self.age)
    }
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn hello(person: Person) {
    println!("{}", person.hello())
}

fn main() {
    let age = sum(10, 18);
    let person = Person {
        name: "Thiago".to_string(),
        age: age,
    };

    hello(person)
}
