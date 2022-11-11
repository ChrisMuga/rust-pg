use std::io::{stdin};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use std::error::Error;
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};

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

    let response = fetch().await;

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

async fn fetch()->Result<(), Box<dyn Error>>{
    // Create new hyper client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let uri: Uri = "https://jsonplaceholder.typicode.com/users".parse().unwrap();

    // Make a GET request to entered API
    let mut res = client.get(uri).await?;

    // Print status
    println!("Response: {}", res.status());

    while let Some(chunk) = res.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
}
