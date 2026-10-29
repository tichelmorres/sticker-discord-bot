use poise::serenity_prelude as serenity;

mod slash_commands;
mod sticker;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let token = std::env::var("BOT_TOKEN").expect("missing BOT_TOKEN");

    let guild_id: u64 = std::env::var("GUILD_ID")
        .expect("missing GUILD_ID")
        .parse()
        .expect("GUILD_ID must be a numeric guild id");

    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                slash_commands::check(),
                slash_commands::populate(),
                slash_commands::list(),
                slash_commands::view(),
                slash_commands::do_sticker(),
            ],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                let dev_guild_id = serenity::GuildId::from(guild_id);
                serenity::Command::set_global_commands(
                    ctx,
                    Vec::<serenity::CreateCommand>::new(),
                )
                    .await?;

                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    dev_guild_id,
                )
                    .await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
