use clap::Parser;
use ec2::{run, Args};
use time::{macros::format_description, UtcOffset};
use tracing_subscriber::fmt::time::OffsetTime;

#[tokio::main]
async fn main() {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[month]-[day] [hour]:[minute]:[second]"),
    );

    tracing_subscriber::fmt().with_timer(local_time).init();

    let args = Args::parse();
    if let Err(e) = run(args).await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
