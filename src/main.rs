use clap::Parser;
use ec2::{run, Args};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let args = Args::parse();
    if let Err(e) = run(args).await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
