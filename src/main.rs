use clap::Parser;

mod arguments;

#[tokio::main]
async fn main() {
    crate::arguments::Arguments::parse();
    println!("Hello, world!");
}
