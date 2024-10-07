use crate::{commands::events::client_join, commands::events::client_leave, database::models::*};
#[poise::command(slash_command, prefix_command)]
pub async fn sudo_join(ctx: Context<'_>) -> Result<(), Error> {
    client_join::welcome(
        ctx.serenity_context(),
        &ctx.author_member().await.unwrap(),
        ctx.data(),
    )
    .await;
    ctx.say("sended").await?;
    Ok(())
}
#[poise::command(slash_command, prefix_command)]
pub async fn sudo_leave(ctx: Context<'_>) -> Result<(), Error> {
    client_leave::see_you_next_time(
        ctx.serenity_context(),
        &ctx.guild_id().unwrap(),
        &ctx.author_member().await.unwrap().user,
        ctx.data(),
    )
    .await;
    ctx.say("sended").await?;
    Ok(())
}

