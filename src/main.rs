mod greeting;

use greeting::greet;

#[tokio::main]
async fn main() {
    println!("{}", greet("chennai geeks"));
}


