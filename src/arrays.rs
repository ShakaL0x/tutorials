fn main() {
    let data = [1, 2, 3, 4];

    println!("{data:?}"); // Regular print
    println!("{data:#?}"); // Pretty print!

    let [a, b, c, d] = data; // Destructuring

    println!("{}, {}, {}, {}", a, b, c, d);

    let first = data[0]; // Direct access
    let second = data[1];
    let third = data[2];
    let fourth = data[3];

    println!("{} {} {} {}", first, second, third, fourth);

    let data = [3; 5]; // Filling array of length 5 with "3"

    println!("{data:?}");
}
