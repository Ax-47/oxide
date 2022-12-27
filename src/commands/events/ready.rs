use poise::serenity_prelude as serenity;
pub fn i_ready(data_about_bot: &serenity::Ready){
    println!("{} is connected!", data_about_bot.user.name)
}