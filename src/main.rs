pub mod handlers;

use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Use: {} <argument>", args[0]);
        return Ok(());
    }

    let main_arg: &str = &args[1];

    if main_arg == "list" {
        let _ = handlers::list().await;
    }

    Ok(())
}
