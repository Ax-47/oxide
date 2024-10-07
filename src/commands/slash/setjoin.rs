use crate::{database::models::*, imgchacker::chack::image_chacker};
use mongodb::bson::{doc, to_bson};
use poise::serenity_prelude::{self as serenity};
#[poise::command(slash_command, prefix_command)]
pub async fn set_join(
    ctx: Context<'_>,
    #[description = "Selected user"] channel: Option<serenity::Channel>,
) -> Result<(), Error> {
    let u = channel.as_ref();

    let channel_id = if !u.is_none() {
        u.unwrap().id().clone()
    } else {
        ctx.channel_id().clone()
    };
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.id":to_bson(&channel_id.as_u64()).unwrap()
                }
            },
        )
        .await
        .unwrap();
    ctx.say("hee hom hee aroy").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn unset_join(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$unset":{
                    "join":""
                }
            },
        )
        .await
        .unwrap();
    ctx.say("hee hom hee aroy").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    subcommands(
        "title",
        "description",
        "image_upload",
        "image",
        "thumbnail_upload",
        "thumbnail",
        "color",
        "del_image",
        "del_thumbnail",
        "auth_text",
        "auth_icon",
        "auth_icon_upload",
        "footer_text",
        "footer_icon",
        "footer_icon_upload"
    )
)]
pub async fn config_join(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
#[poise::command(prefix_command, slash_command)]
pub async fn title(ctx: Context<'_>, title: String) -> Result<(), Error> {
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.title":title
                }
            },
        )
        .await
        .unwrap();
    ctx.say("Changed").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn description(ctx: Context<'_>, description: String) -> Result<(), Error> {
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.description":description
                }
            },
        )
        .await
        .unwrap();
    ctx.say("Changed").await?;
    Ok(())
}
#[poise::command(prefix_command, slash_command)]
pub async fn image_upload(ctx: Context<'_>, image: serenity::Attachment) -> Result<(), Error> {
    if !image
        .clone()
        .content_type
        .unwrap_or("Unknow".to_string())
        .starts_with("image")
    {
        ctx.say("only image").await?;
        return Ok(());
    }
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.image":image.url
                }
            },
        )
        .await
        .unwrap();
    ctx.say("image").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn image(ctx: Context<'_>, image: String) -> Result<(), Error> {
    if !image_chacker(&image) {
        ctx.say("error,this is not url img || oxide command")
            .await?;
        return Ok(());
    }
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.image":image
                }
            },
        )
        .await
        .unwrap();
    ctx.say("image").await?;
    Ok(())
}
#[poise::command(prefix_command, slash_command)]
pub async fn thumbnail_upload(
    ctx: Context<'_>,

    thumbnail: serenity::Attachment,
) -> Result<(), Error> {
    if !thumbnail
        .clone()
        .content_type
        .unwrap_or("Unknow".to_string())
        .starts_with("image")
    {
        ctx.say("only image").await?;
    }
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.thumbnail":thumbnail.url
                }
            },
        )
        .await
        .unwrap();
    ctx.say("image").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn thumbnail(ctx: Context<'_>, thumbnail: String) -> Result<(), Error> {
    if !image_chacker(&thumbnail) {
        ctx.say("error,this is not url img || oxide command")
            .await?;
        return Ok(());
    }
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.thumbnail":thumbnail
                }
            },
        )
        .await
        .unwrap();
    ctx.say("image").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn color(ctx: Context<'_>, r: u8, g: u8, b: u8) -> Result<(), Error> {
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.color":to_bson( &RGB{r,g,b}).unwrap(),
                }
            },
        )
        .await
        .unwrap();
    ctx.say("image").await?;
    Ok(())
}
#[poise::command(prefix_command, slash_command)]
pub async fn del_thumbnail(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.thumbnail":""
                }
            },
        )
        .await
        .unwrap();
    ctx.say("image").await?;
    Ok(())
}
#[poise::command(prefix_command, slash_command)]
pub async fn del_image(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.image":""
                }
            },
        )
        .await
        .unwrap();
    ctx.say("image").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn auth_text(ctx: Context<'_>, text: String) -> Result<(), Error> {
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.auth.text":text
                }
            },
        )
        .await
        .unwrap();
    ctx.say("image").await?;
    Ok(())
}
#[poise::command(prefix_command, slash_command)]
pub async fn auth_icon(ctx: Context<'_>, img: String) -> Result<(), Error> {
    if !image_chacker(&img) {
        ctx.say("error,this is not url img || oxide command")
            .await?;
        return Ok(());
    }
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.auth.icon":img
                }
            },
        )
        .await
        .unwrap();
    ctx.say("image").await?;
    Ok(())
}
#[poise::command(prefix_command, slash_command)]
pub async fn auth_icon_upload(ctx: Context<'_>, icon: serenity::Attachment) -> Result<(), Error> {
    if !icon
        .clone()
        .content_type
        .unwrap_or("Unknow".to_string())
        .starts_with("image")
    {
        ctx.say("only image").await?;
    }
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.auth.icon":icon.url
                }
            },
        )
        .await
        .unwrap();
    ctx.say("image").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn footer_text(ctx: Context<'_>, text: String) -> Result<(), Error> {
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.footer.text":text
                }
            },
        )
        .await
        .unwrap();
    ctx.say("image").await?;
    Ok(())
}
#[poise::command(prefix_command, slash_command)]
pub async fn footer_icon(ctx: Context<'_>, img: String) -> Result<(), Error> {
    if !image_chacker(&img) {
        ctx.say("error,this is not url img || oxide command")
            .await?;
        return Ok(());
    }
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.footer.icon":img
                }
            },
        )
        .await
        .unwrap();
    ctx.say("image").await?;
    Ok(())
}
#[poise::command(prefix_command, slash_command)]
pub async fn footer_icon_upload(ctx: Context<'_>, icon: serenity::Attachment) -> Result<(), Error> {
    if !icon
        .clone()
        .content_type
        .unwrap_or("Unknow".to_string())
        .starts_with("image")
    {
        ctx.say("only image").await?;
    }
    ctx.data()
        .database
        .update_guild(
            &ctx.guild_id().unwrap().to_string(),
            doc! {
                "$set":{
                    "join.embed.footer.icon":icon.url
                }
            },
        )
        .await
        .unwrap();
    ctx.say("image").await?;
    Ok(())
}

