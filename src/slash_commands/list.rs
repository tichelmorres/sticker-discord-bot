use poise::serenity_prelude as serenity;
use poise::CreateReply;
use crate::sticker::StickerCollection;
use crate::{Context, Error};

/// List all available stickers
#[poise::command(slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let stickers = StickerCollection::load_from_file("stickers_store.json")?;

    if stickers.stickers.is_empty() {
        let embed = serenity::CreateEmbed::new()
            .title("No stickers available.")
            .description("Your sticker collection is empty! Send stickers in <#stickers> and use `/check` followed by `/populate` to add them.")
            .color(0xffcc00) // Yellow/Orange
            .timestamp(serenity::Timestamp::now());

        let reply = CreateReply::default()
            .embed(embed)
            .ephemeral(true);

        ctx.send(reply).await?;
        return Ok(());
    }

    let total_count = stickers.stickers.len();

    // Group stickers into chunks if there are many
    let mut embed = serenity::CreateEmbed::new()
        .title("Available stickers.")
        .description(format!("Your collection has **{}** sticker(s):", total_count))
        .color(0x9b59b6) // Purple
        .timestamp(serenity::Timestamp::now())
        .footer(serenity::CreateEmbedFooter::new("Use /view {name} to see details or /do {name} to send"));

    // Discord embed fields have a 1024 character limit
    // solution: split if necessary
    let mut current_field = String::new();
    let mut field_count = 0;

    for (idx, sticker) in stickers.stickers.iter().enumerate() {
        let entry = format!("**`{}`**\n{}\n\n", sticker.name, sticker.description);

        // If adding this entry would exceed field limit, create a new field
        if current_field.len() + entry.len() > 1000 {
            field_count += 1;
            let field_title = if field_count == 1 {
                "Stickers".to_string()
            } else {
                format!("Stickers (cont. {})", field_count)
            };
            embed = embed.field(field_title, current_field.clone(), false);
            current_field.clear();
        }

        current_field.push_str(&entry);

        // If this is the last sticker, add the remaining field
        if idx == stickers.stickers.len() - 1 && !current_field.is_empty() {
            field_count += 1;
            let field_title = if field_count == 1 {
                "Stickers".to_string()
            } else {
                format!("Stickers (cont. {})", field_count)
            };
            embed = embed.field(field_title, current_field.clone(), false);
        }
    }

    let reply = CreateReply::default()
        .embed(embed)
        .ephemeral(true);

    ctx.send(reply).await?;

    Ok(())
}
