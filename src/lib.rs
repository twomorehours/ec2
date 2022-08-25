use anyhow::anyhow;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::Client;
use clap::{Parser, Subcommand};
use std::time::Duration;
use tracing::info;

mod instance;

#[derive(Parser, Debug)]
#[clap(version, author)]
pub struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Reboot your ec2 instance and return an new Ip
    Reboot { instance_id: String },
}

pub async fn run(args: Args) -> anyhow::Result<()> {
    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    match args.action {
        Action::Reboot { instance_id } => {
            info!("your instance id is {}", instance_id);
            let ec2_instance = instance::get_instance(&client, &instance_id)
                .await?
                .ok_or_else(|| anyhow!("instance not found"))?;
            info!("your instance: {}", ec2_instance);

            //     // stop your instance if running
            //     if !ec2_instance.is_stopped() {
            //         info!("stopping your instance, please be patient.");
            //         instance::stop_instance(&client, &instance_id).await?;
            //         loop {
            //             let instance = instance::get_instance(&client, &instance_id)
            //                 .await?
            //                 .unwrap();
            //             info!("checking your instance: {}", instance);
            //             if instance.is_stopped() {
            //                 info!("your instance has been stopped");
            //                 break;
            //             }
            //             tokio::time::sleep(Duration::from_secs(5)).await;
            //         }
            //     }

            //     // start your instance
            //     info!("starting your instance, please be patient.");
            //     instance::start_instance(&client, &instance_id).await?;
            //     loop {
            //         let instance = instance::get_instance(&client, &instance_id)
            //             .await?
            //             .unwrap();
            //         info!("checking your instance: {}", instance);
            //         if instance.is_running() {
            //             info!(
            //                 "your instance has been started, the new ip is {}, enjoy!",
            //                 instance.ipv4
            //             );
            //             break;
            //         }
            //         tokio::time::sleep(Duration::from_secs(5)).await;
            //     }
        }
    }

    Ok(())
}
