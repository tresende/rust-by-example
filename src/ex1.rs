//

fn sum(x: u64, y: u64) -> u64 {
    x + y
}

fn hello(name: String, age: u64) {
    println!(
        "my name is {name} and i am {age} years ",
        name = name,
        age = age
    )
}

fn main() {
    let name = String::from("Thiago");
    let age = sum(10, 18);
    hello(name, age)
}
