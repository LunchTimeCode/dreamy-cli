mod commands;
mod config;
mod gh;
mod init;

fn main() {
    match commands::figure() {
        Ok(message) => println!("{message}"),
        Err(error_message) => {
            eprintln!("dy-error: {error_message}");
            std::process::exit(1)
        }
    }
}
