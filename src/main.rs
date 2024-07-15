mod commands;
mod config;
mod gh;
mod init;

use colored::Colorize;

#[tokio::main]
async fn main() {
    let res = tokio::task::spawn_blocking(commands::figure)
        .await
        .expect("async comp not working")
        .await;

    match res {
        Ok(message) => {
            let message = format!("{}", message.to_string().green());
            println!("{message}")
        }

        Err(error_message) => {
            let message = format!("{}", error_message.to_string().red());
            println!("{message}");
            std::process::exit(1)
        }
    }
}
