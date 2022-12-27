use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use std::time::Instant;
pub async fn run(_options: &[CommandDataOption]) -> String {
    let post_latency = {
        let now = Instant::now();
        now.elapsed().as_millis() as f64
    };
    format!("The shard latency is {}",post_latency)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}