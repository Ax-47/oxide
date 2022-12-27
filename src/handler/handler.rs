use poise::serenity_prelude as serenity;
use crate::{
    database::models::*,
    commands::events::*,
};

    
    pub async fn event_handler(
        ctx: &serenity::Context,
        event: &poise::Event<'_>,
        _framework: poise::FrameworkContext<'_, Data, Error>,
        user_data: &Data,
    ) -> Result<(), Error> {
        println!("{:?}",event.name());
        match event {
            poise::Event::Ready { data_about_bot } => ready::i_ready(data_about_bot),
            // join
            poise::Event::GuildMemberAddition {new_member}=>client_join::welcome(ctx, new_member, user_data).await,
            // leave
            poise::Event::GuildMemberRemoval { guild_id, user, member_data_if_available:_ }=>client_leave::see_you_next_time(ctx, guild_id, user,user_data).await,
            // vc
            poise::Event::VoiceStateUpdate { old, new }=>vc::vc(ctx,old,new,user_data).await,
            //bot join
            poise::Event::GuildCreate {guild,is_new}=>{
                if *is_new{
                    user_data.database.create_guild_data(guild.id.to_string()).await.unwrap();
                }
                
                
            }
            //bot laeve
            poise::Event::GuildDelete { incomplete, full:_ }=>{
                let guilid =incomplete.id;
                user_data.database.delete_guild_data(&guilid.to_string()).await.unwrap();
            }
            _ => {}
        }
    
        Ok(())
    }



