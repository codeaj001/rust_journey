fn main() {
    // Hello World code
    println!("Hello, world!");

    // Hello World with placeholder
    println!("{:?}", "Hello, World!");

    // Positional Argument
    println!("{0} {1}", "Hello,", "World!");

    // Named Argument
    let zero = "Hello,";
    let one = "World!";
    println!("{zero} {one}", zero = zero, one = one);

    // Placeholder Traits
    // Displays the number in the three formats (binary, hexadecimal and octal)
    println!("Number: {}\n Binary: {:b}\n Hexadecimal: {:x}\n Octal: {:o}", 10, 10, 10, 10);

    //Basic Math
    println!("{} - {} = {}", 10, 10, 10 - 10);

    // Placeholder
    println!("{:?}", ("This is a Rust Course", 101));

    let point = (3, 5);
    println!("Point coordinates: {:?}", point); 
    let name = "Alice";
    let age = 25;
    let scores = [85, 90, 92];
    println!("Details: {:?}", (name, age, scores)); 
    // Output: Details: ("Alice", 25, [85, 90, 92])

    // Name shadowing
    let x = 5;
    println!("x is: {}", x);
    {
        let x = 10;
        println!("x is: {}", x);
    }

    //Integer
    //------- Signed Integer (Can either be + or -)
    let temperature: i8 = -10;
    println!("Current temperature is {}°C", temperature);

    // -------- Unsigned Integer (can only be positive)
    let stock: u16 = 6500;
    println!("Current stock is {}", stock);

}
