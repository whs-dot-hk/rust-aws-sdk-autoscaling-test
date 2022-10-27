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
        .auto_scaling_group_names(test.clone())
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

    let launch_configuration = r
        .launch_configurations
        .unwrap()
        .clone()
        .first()
        .unwrap()
        .clone();

    println!("{:?}", launch_configuration);

    //let r = client
    //    .create_launch_configuration()
    //    .launch_configuration_name("test3".to_string())
    //    .image_id("ami-0223db08811f6fb2d".to_string())
    //    .set_instance_type(launch_configuration.instance_type)
    //    .set_key_name(launch_configuration.key_name)
    //    .set_security_groups(launch_configuration.security_groups)
    //    .send()
    //    .await?;

    //println!("{:?}", r);

    let r = client
        .update_auto_scaling_group()
        .auto_scaling_group_name(test)
        .launch_configuration_name("test2".to_string())
        .send()
        .await?;

    println!("{:?}", r);

    Ok(())
}
