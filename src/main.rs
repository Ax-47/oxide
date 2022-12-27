mod database;
mod handler;
mod commands;
mod imgchacker;
use poise::serenity_prelude as serenity;

use dotenv::dotenv;
use crate::{
    database::mongox,
    database::models::*,
    handler::handler as hl,
    commands::slash::*
};

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().expect("ooo");









    let database = mongox::MongoRepo::init().await;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                age(),
                setjoin::set_join(),
                setjoin::config_join(),
                setjoin::unset_join(),
                setleave::set_leave(),
                setleave::config_leave(),
                setleave::unset_leave(),
                craetevc::set_vc(),
                craetevc::unset_vc(),
                sudotest::sudo_join(),
                sudotest::sudo_leave()
                ],
                event_handler:|ctx,event,framework,user_data |{
                    Box::pin(hl::event_handler(ctx,event,framework,user_data))
                },
                
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {database})
            })
        });

    framework.run().await.unwrap();
}