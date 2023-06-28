use rust_i18n::t;
use rust_user_registration_system::_rust_i18n_translate;

pub fn print_header() {
    let header = t!("welcome");
    println!("\n{}\n{}\n", header, "=".repeat(header.len()));
}

pub fn print_logging_header() {
    let message = t!("logging_enabled");
    println!();
    println!("{}", "=".repeat(message.len()));
    println!("{}", message);
    println!("{}", "=".repeat(message.len()));
    println!();
}
