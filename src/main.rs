// import library to read env variables
// use std::env;

#[tokio::main]
async fn main() {
    let file = std::env::args().nth(1).expect("no file given");

    println!("file: {:?}", file)
}
