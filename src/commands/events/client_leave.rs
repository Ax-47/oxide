use crate::database::models::{self, Data, TextwithICon, RGB};
use poise::serenity_prelude::ChannelId;
use poise::serenity_prelude::{self as serenityy, Guild, GuildId};
use serenity::utils::Colour;

pub async fn see_you_next_time(
    ctx: &serenityy::Context,
    guild_id: &GuildId,
    user: &serenityy::User,
    data: &Data,
) {
    let data_guild = data.database.get_guildid_data(&guild_id.to_string()).await;

    if data_guild.is_err() {
        return;
    }

    let data_guild_un = data_guild.unwrap();
    if data_guild_un.leave.is_none() {
        return;
    }

    let guild = guild_id.to_guild_cached(ctx.cache.clone()).unwrap();
    let data_leave = data_guild_un.leave.unwrap();
    let room = guild.channels.get(&ChannelId(data_leave.id));
    if room.is_none() {
        return;
    }
    let d = data_leave.embed.unwrap_or(models::Embed::default());

    let title = string_to_text(
        &d.title.unwrap_or("See u nekst time".to_string()),
        user,
        guild.clone(),
    );
    let description = string_to_text(
        &d.description
            .unwrap_or("See u nekst time <mantion(user)>".to_string()),
        user,
        guild.clone(),
    );
    let rgb = d.color.unwrap_or(RGB {
        r: 220,
        g: 71,
        b: 0,
    });
    let color = Colour::from_rgb(rgb.r, rgb.g, rgb.b);
    let image=text_to_img(&d.image.unwrap_or("https://images-ext-2.discordapp.net/external/72u7pCkEPUfaXZu7t1OB0FX4AsCUnCryPqZQPHOiGk4/https/miro.medium.com/max/1400/0%2AcUpkVai00QRZHYDu?width=994&height=662".to_string()),user,guild.clone());
    let thumbnail = text_to_img(
        &d.thumbnail.unwrap_or("!img(guild)".to_string()),
        user,
        guild.clone(),
    );

    let auth = d.auth.unwrap_or(TextwithICon::default());
    let authtext = string_to_text(
        &auth.text.unwrap_or("<name(user)>".to_string()),
        user,
        guild.clone(),
    );
    let authicon = text_to_img(
        &auth.icon.unwrap_or("!img(user)".to_string()),
        user,
        guild.clone(),
    );
    let footer = d.footer.unwrap_or(TextwithICon::default());
    let footertext = string_to_text(
        &footer.text.unwrap_or("<none>".to_string()),
        user,
        guild.clone(),
    );
    let footericon = text_to_img(
        &footer.icon.unwrap_or("!img(none)".to_string()),
        user,
        guild.clone(),
    );

    let senddd = room.unwrap().clone().guild().unwrap();
    senddd
        .send_message(ctx.http.clone(), |m| {
            m.embed(|e| {
                e.title(title)
                    .colour(color)
                    .description(description)
                    .image(image)
                    .thumbnail(thumbnail)
                    .author(|f| f.icon_url(authicon).name(authtext))
                    .footer(|f| f.text(footertext).icon_url(footericon))
            })
        })
        .await
        .unwrap();
}

fn string_to_text(s: &String, user: &serenityy::User, guild: Guild) -> std::string::String {
    s.replace("<mantion(user)>", format!("<@{}>", user.id).as_str())
        .replace("<name(user)>", user.name.as_str())
        .replace("<id(user)>", user.id.to_string().as_str())
        .replace("<tag(user)>", user.tag().as_str())
        .replace("<none>", "")
        .replace("<name(guild)>", guild.name.as_str())
        .replace("<id(guild)>", guild.id.to_string().as_str())
}
fn text_to_img(s: &String, user: &serenityy::User, guild: Guild) -> std::string::String {
    s.replace(
        "!img(guild)",
        guild.icon_url().unwrap_or("None".to_string()).as_str(),
    )
    .replace(
        "!img(user)",
        user.avatar_url().unwrap_or("non".to_string()).as_str(),
    )
    .replace("!img(none)", "")
}

