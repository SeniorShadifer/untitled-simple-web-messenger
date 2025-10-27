#[derive(clap::Parser, Debug)]
#[command(name = "Untitled messenger server")]
#[command(about = "Web-server for Untitled messenger.")]
#[command(version = "0.1.0-alpha.1")]
pub struct Arguments {
    #[arg(short, long, default_value = "0.0.0.0:5141")]
    address: String,
}
