use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Type a number:");
    io::stdin()
        .read_line(&mut buffer)
        .expect("there was a problem reading a line");

    let number = buffer.trim().replace(",", ".")
        .parse::<f64>()
        .unwrap();

    match number % 2f64 {
        0f64 => println!("The {} is even number", number),
        _ => println!("The {} is not even number", number),
    }
}
