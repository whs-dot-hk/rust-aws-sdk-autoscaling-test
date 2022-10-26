use aws_config::meta::region::RegionProviderChain;
use aws_sdk_autoscaling as autoscaling;

#[tokio::main]
async fn main() -> Result<(), autoscaling::Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

    let config = aws_config::from_env().region(region_provider).load().await;

    let client = autoscaling::Client::new(&config);

    // ... make some calls with the client
    let test = "test".to_string();

    let r = client
        .describe_auto_scaling_groups()
        .auto_scaling_group_names(test)
        .send()
        .await?;

    println!("{:?}", r);

    let r = client
        .describe_launch_configurations()
        .launch_configuration_names(
            r.auto_scaling_groups
                .unwrap()
                .first()
                .unwrap()
                .launch_configuration_name
                .as_ref()
                .unwrap(),
        )
        .send()
        .await?;

    println!("{:?}", r);

    Ok(())
}
