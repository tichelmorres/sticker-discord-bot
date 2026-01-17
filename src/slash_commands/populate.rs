use poise::serenity_prelude as serenity;
use poise::CreateReply;
use crate::sticker::{NewStickerCollection, StickerCollection};
use crate::{Context, Error};

/// Populate stickers_store.json with new stickers
#[poise::command(slash_command)]
pub async fn populate(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    // Load new stickers
    let new_stickers = NewStickerCollection::load_from_file("new_stickers.json")?;

    if new_stickers.new_stickers.is_empty() {
        let embed = serenity::CreateEmbed::new()
            .title("No new stickers.")
            .description("There are no new stickers to populate. Run `/check` first to scan for new stickers!")
            .color(0xffcc00) // Yellow/Orange
            .timestamp(serenity::Timestamp::now());

        let reply = CreateReply::default()
            .embed(embed)
            .ephemeral(true);

        ctx.send(reply).await?;
        return Ok(());
    }

    // Load existing stickers
    let mut existing_stickers = StickerCollection::load_from_file("stickers_store.json")?;

    // Add only non-duplicate stickers
    let mut added_stickers = Vec::new();
    for sticker in new_stickers.new_stickers {
        if !existing_stickers.contains(&sticker.name) {
            added_stickers.push(sticker.name.clone());
            existing_stickers.stickers.push(sticker);
        }
    }

    existing_stickers.save_to_file("stickers_store.json")?;

    let added_count = added_stickers.len();

    let embed = if added_count > 0 {
        let sticker_list = if added_stickers.len() <= 10 {
            added_stickers.iter()
                .map(|name| format!("- `{}`", name))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            format!("{} stickers added (too many to list)", added_count)
        };

        serenity::CreateEmbed::new()
            .title("Stickers added successfully.")
            .description(format!("**{}** new sticker(s) have been added to your collection", added_count))
            .field("Added Stickers", sticker_list, false)
            .footer(serenity::CreateEmbedFooter::new("Use /list to see all available stickers"))
            .color(0x00ff88) // Green
            .timestamp(serenity::Timestamp::now())
    } else {
        serenity::CreateEmbed::new()
            .title("No duplicates added.")
            .description("All stickers from the new stickers file already exist in your collection.")
            .color(0x0099ff) // Blue
            .timestamp(serenity::Timestamp::now())
    };

    let reply = CreateReply::default()
        .embed(embed)
        .ephemeral(true);

    ctx.send(reply).await?;

    Ok(())
}
