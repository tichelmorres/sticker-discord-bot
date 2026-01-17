use poise::CreateReply;
use crate::sticker::StickerCollection;
use crate::{Context, Error};

/// Send a sticker to the current channel
#[poise::command(slash_command, rename = "do")]
pub async fn do_sticker(
    ctx: Context<'_>,
    #[description = "Sticker name"] sticker_name: String,
) -> Result<(), Error> {
    let stickers = StickerCollection::load_from_file("stickers_store.json")?;

    if let Some(sticker) = stickers.find(&sticker_name) {
        let content = format!("[{}]({})", sticker.name, sticker.link);

        let reply = CreateReply::default()
            .content(content)
            .ephemeral(false);

        ctx.send(reply).await?;
    } else {
        let reply = CreateReply::default()
            .content(format!("Sticker `{}` not found...", sticker_name))
            .ephemeral(true);

        ctx.send(reply).await?;
    }

    Ok(())
}
