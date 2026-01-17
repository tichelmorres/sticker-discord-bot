use poise::serenity_prelude as serenity;
use poise::CreateReply;
use crate::sticker::{NewStickerCollection, Sticker, StickerCollection};
use crate::{Context, Error};

/// Check #stickers channel for new stickers
#[poise::command(slash_command)]
pub async fn check(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let stickers_channel_id: u64 = std::env::var("STICKERS_CHANNEL_ID")
        .expect("missing STICKERS_CHANNEL_ID")
        .parse()
        .expect("STICKERS_CHANNEL_ID must be numeric");

    let channel_id = serenity::ChannelId::new(stickers_channel_id);

    // Load existing stickers
    let existing_stickers = StickerCollection::load_from_file("stickers_store.json")?;

    // Fetch messages from the channel
    let messages = channel_id.messages(ctx.http(), serenity::GetMessages::new().limit(100)).await?;

    // Parse messages and find new stickers
    let mut new_stickers = Vec::new();

    for message in messages.iter().rev() {
        if let Some(sticker) = parse_sticker_from_message(&message.content) {
            if !existing_stickers.contains(&sticker.name) {
                new_stickers.push(sticker);
            }
        }
    }

    // Save new stickers to new_stickers.json
    let new_collection = NewStickerCollection { new_stickers: new_stickers.clone() };
    new_collection.save_to_file("new_stickers.json")?;

    let count = new_stickers.len();

    // Output
    let embed = if count > 0 {
        let sticker_list = new_stickers.iter()
            .map(|s| format!("- `{}`", s.name))
            .collect::<Vec<_>>()
            .join("\n");

        serenity::CreateEmbed::new()
            .title("New Stickers Found.")
            .description(format!("Discovered **{}** new sticker(s) in <#{}>", count, stickers_channel_id))
            .field("New Stickers", sticker_list, false)
            .footer(serenity::CreateEmbedFooter::new("Use /populate to add them to the store"))
            .color(0x00ff88) // Green
            .timestamp(serenity::Timestamp::now())
    } else {
        serenity::CreateEmbed::new()
            .title("All caught up.")
            .description("No new stickers found. Your collection is up to date!")
            .color(0x0099ff) // Blue
            .timestamp(serenity::Timestamp::now())
    };

    let reply = CreateReply::default()
        .embed(embed)
        .ephemeral(true);

    ctx.send(reply).await?;

    Ok(())
}

fn parse_sticker_from_message(content: &str) -> Option<Sticker> {
    let mut name: Option<String> = None;
    let mut link: Option<String> = None;
    let mut description: Option<String> = None;

    for line in content.lines() {
        let line = line.trim();

        if line.to_lowercase().starts_with("name:") {
            name = Some(line[5..].trim().to_string());
        } else if line.to_lowercase().starts_with("link:") {
            link = Some(line[5..].trim().to_string());
        } else if line.to_lowercase().starts_with("description:") || line.to_lowercase().starts_with("desc:") {
            let start = if line.to_lowercase().starts_with("description:") { 12 } else { 5 };
            description = Some(line[start..].trim().to_string());
        }
    }

    // Both name and link are required
    if let (Some(n), Some(l)) = (name, link) {
        Some(Sticker::new(n, l, description))
    } else {
        None
    }
}
