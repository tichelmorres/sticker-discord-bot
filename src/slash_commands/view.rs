use poise::serenity_prelude as serenity;
use poise::CreateReply;
use crate::sticker::StickerCollection;
use crate::{Context, Error};

/// View a specific sticker's details
#[poise::command(slash_command)]
pub async fn view(
    ctx: Context<'_>,
    #[description = "Sticker name"] sticker_name: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let stickers = StickerCollection::load_from_file("stickers_store.json")?;

    if let Some(sticker) = stickers.find(&sticker_name) {
        let embed = serenity::CreateEmbed::new()
            .title(format!("{}", sticker.name))
            .description(&sticker.description)
            .field("Link", format!("[Click here]({})", sticker.link), false)
            .image(&sticker.link)
            .color(0xe91e63) // Pink
            .timestamp(serenity::Timestamp::now())
            .footer(serenity::CreateEmbedFooter::new(format!("Use /do {} to send this sticker", sticker.name)));

        let reply = CreateReply::default()
            .embed(embed)
            .ephemeral(true);

        ctx.send(reply).await?;
    } else {
        let embed = serenity::CreateEmbed::new()
            .title("Sticker not found.")
            .description(format!("The sticker `{}` doesn't exist in your collection.", sticker_name))
            .color(0xff4444) // Red
            .footer(serenity::CreateEmbedFooter::new("Use /list to see all available stickers"))
            .timestamp(serenity::Timestamp::now());

        let reply = CreateReply::default()
            .embed(embed)
            .ephemeral(true);

        ctx.send(reply).await?;
    }

    Ok(())
}
