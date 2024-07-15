mod commands;
mod config;
mod gh;
mod init;

#[tokio::main]
async fn main() {
    let res = tokio::task::spawn_blocking(commands::figure)
        .await
        .expect("async comp not working")
        .await;

    match res {
        Ok(message) => println!("{message}"),
        Err(error_message) => {
            eprintln!("dy-error: {error_message}");
            std::process::exit(1)
        }
    }
}
