struct Person {
    age: i32,
    name: String,
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn hello(person: Person) {
    println!(
        "my name is {name} and i am {age} years ",
        name = person.name,
        age = person.age
    )
}

fn main() {
    let age = sum(10, 18);
    let person = Person {
        name: "Thiago".to_string(),
        age: age,
    };

    hello(person)
}
