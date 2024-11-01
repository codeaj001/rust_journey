fn main() {
    // Hello World code
    println!("Hello, world!");

    // Hello World with placeholder
    println!("{}, {}!", "Hello", "World");

    // Positional Argument
    println!("{0} {1}", "Hello,", "World!");

    // Named Argument
    let zero = "Hello,";
    let one = "World!";
    println!("{zero} {one}", zero = zero, one = one);

    // Placeholder Traits
    println!("Number: {}\n Binary: {:b}\n Hexadecimal: {:x}\n Octal: {:o}", 10, 10, 10, 10);
}
