use std::error::Error;
use std::io::{stdin};

mod helpers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    // Fetch the url from the user input.

    println!("RPG: init");
    let mut s = String::new();
    println!("Enter your name: ");

    // Read the input from the console.
    match stdin().read_line(&mut s) {
        Ok(_n) => {}
        Err(error) => {
            println!("{error} ====>");
        }
    }

    println!("Fetching for {}", s);

    let response = helpers::fetch::fetch().await;

    match response {
        Ok(()) => {
            println!("Done!");
        }

        Err(error) => {
            println!("{error}");
        }
    }

    Ok(())
}
