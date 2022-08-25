use anyhow::Ok;
use aws_sdk_ec2::{
    model::{Filter, Instance, InstanceState, InstanceStateName},
    Client,
};
use std::fmt::Display;

pub struct Ec2Instance {
    pub id: String,
    pub ipv4: String,
    pub state: InstanceState,
}

impl Ec2Instance {
    pub fn is_stopped(&self) -> bool {
        self.state.name() == Some(&InstanceStateName::Stopped)
    }

    pub fn is_running(&self) -> bool {
        self.state.name() == Some(&InstanceStateName::Running)
    }
}

impl From<&Instance> for Ec2Instance {
    fn from(ins: &Instance) -> Self {
        Self {
            id: ins.instance_id().unwrap().to_owned(),
            ipv4: ins.public_ip_address().unwrap_or("none").to_owned(),
            state: ins.state().unwrap().clone(),
        }
    }
}

impl Display for Ec2Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[").unwrap();
        write!(f, "id:{}", self.id).unwrap();
        write!(f, ", ip:{}", self.ipv4).unwrap();
        write!(f, ", state:{}", self.state.name().unwrap().as_str()).unwrap();
        writeln!(f, "]")
    }
}

pub async fn get_instance(client: &Client, id: &str) -> anyhow::Result<Option<Ec2Instance>> {
    let output = client
        .describe_instances()
        .filters(Filter::builder().name("instance-id").values(id).build())
        .send()
        .await?;

    let reservations = output.reservations().unwrap();
    if reservations.is_empty() {
        return Ok(None);
    }

    let instances = reservations[0].instances().unwrap();
    let ec2_instance = instances
        .iter()
        .find(|ins| ins.instance_id().unwrap() == id)
        .map(|ins| ins.into());

    Ok(ec2_instance)
}

pub async fn stop_instance(client: &Client, id: &str) -> anyhow::Result<()> {
    client.stop_instances().instance_ids(id).send().await?;
    Ok(())
}

pub async fn start_instance(client: &Client, id: &str) -> anyhow::Result<()> {
    client.start_instances().instance_ids(id).send().await?;
    Ok(())
}
