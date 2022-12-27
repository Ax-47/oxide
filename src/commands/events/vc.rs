use poise::serenity_prelude as serenity;
use crate::database::models::{Data};
pub async fn vc(ctx: &serenity::Context,old:&Option<serenity::VoiceState>,new:&serenity::VoiceState,data:&Data){
    let category=new.channel_id.unwrap().to_channel_cached(ctx.cache.clone()).unwrap().category();
    println!("{:?}",new);
    if !old.is_none(){
        return 
    }
    let db =data.database.get_guildid_data(&new.guild_id.unwrap().to_string()).await;
    if db.is_err(){
        return
    }
    if new.channel_id.unwrap().as_u64() != &db.unwrap().vc.unwrap(){
        return
    }
    let guild=new.guild_id.unwrap();
    
    if category.is_none(){
        let err=guild.create_channel(ctx.http.clone(), 
            |c| 
            c.name(new.member.clone().unwrap().user.name)
            .kind(serenity::ChannelType::Voice)
        ).await;
        if err.is_err(){
            return
        }
        return
    }
    
    let err=guild.create_channel(ctx.http.clone(), 
    |c| 
    c
    .category(category.unwrap().id)
    .name(new.member.clone().unwrap().user.name)
    .kind(serenity::ChannelType::Voice)


    ).await;
    if err.is_err(){
        return
    }
    return


}