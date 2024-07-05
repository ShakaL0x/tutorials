fn main() {
    let data = ('a', "Hello World!", 123, 2.0);

    println!("{data:?}"); // Regular print
    println!("{data:#?}"); // Pretty print!

    let (a, b, c, d) = data; // Destructuring

    println!("{}, {}, {}, {}", a, b, c, d);

    let first = data.0; // Direct access
    let second = data.1;
    let third = data.2;
    let fourth = data.3;

    println!("{} {} {} {}", first, second, third, fourth);
}
