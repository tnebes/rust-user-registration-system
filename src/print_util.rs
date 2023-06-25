pub fn print_header() {
    println!();
    let header:&str = "Welcome to the User Registration App";
    println!("{}", header);
    println!("{}", "=".repeat(header.len()));
    println!();
}

pub fn print_logging_header() {
    let message = "Logging enabled";
    println!();
    println!("{}", "=".repeat(message.len()));
    println!("{}", message);
    println!("{}", "=".repeat(message.len()));
    println!();
}
