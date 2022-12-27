use poise::serenity_prelude::{self as serenity};
use crate::{
    database::models::*,
};
use mongodb::{
    bson::{doc,to_bson}
};
#[poise::command(slash_command, prefix_command)]
pub async fn set_vc(
    ctx: Context<'_>,
    #[description = "Selected user"]
    #[channel_types("Voice")] 
    channel: Option<serenity::Channel>,
) -> Result<(), Error> {
    let u = channel.as_ref();

    let channel_id=if !u.is_none(){
        u.unwrap().id().clone()
    }else{
        ctx.guild().unwrap().create_channel(ctx.serenity_context().http.clone(), |c| c.name("Click").kind(serenity::ChannelType::Voice)).await.unwrap().id
       
    };
    ctx.data().database.update_guild(&ctx.guild_id().unwrap().to_string(), doc!{
        "$set":{
            "vc":to_bson(&channel_id.as_u64()).unwrap()
        }
    }).await.unwrap();
    println!("{}",channel_id);
    ctx.say("hee hom hee aroy").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn unset_vc(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.data().database.update_guild(&ctx.guild_id().unwrap().to_string(), doc!{
        "$unset":{
            "create_vc":""
        }
    }).await.unwrap();
    ctx.say("hee hom hee aroy").await?;
    Ok(())
}