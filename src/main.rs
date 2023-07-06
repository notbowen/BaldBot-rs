#![allow(unused)]

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod commands;
mod utils;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::{ChannelId, GuildChannel, Member, Message};
use serenity::model::webhook::WebhookType;
use serenity::prelude::*;
use utils::swear_detector;

struct BaldHandler;

#[async_trait]
impl EventHandler for BaldHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            debug!("Received command: /{}", command.data.name);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                _ => "Not implemented :|".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                error!("Couldn't respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Logged in as: {}", ready.user.name);

        if cfg!(debug_assertions) {
            let guild_id = GuildId(791588775456800798);
            let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
                commands.create_application_command(|command| commands::ping::register(command))
            })
            .await;

            info!("Initialised guild slash commands");
        } else {
            let commands = Command::create_global_application_command(&ctx.http, |command| {
                commands::ping::register(command)
            })
            .await;

            info!("Initialised global commands");
        }
    }

    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        debug!("User `{}` joined!", member.user.name);
        let channel_id = ChannelId(792718122226417704);

        if let Err(why) = channel_id
            .send_message(&ctx.http, |message| {
                message.content(format!("Welcome to the BaldSMP, {}", member.mention()))
            })
            .await
        {
            error!("Unable to send welcome message: {}", why);
        }
    }

    async fn message(&self, ctx: Context, message: Message) {
        // Skip if message was sent by a bot
        if message.author.bot {
            return;
        }

        // Check and respond if there is swear word
        if let Some(response) = utils::swear_detector::get_swear_response(message.content).await {
            if let Err(why) = message
                .channel_id
                .send_message(&ctx.http, |message| message.content(response))
                .await
            {
                error!("Unable to respond to swear word: {}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialise logging
    std::env::set_var("RUST_LOG", "bald_bot=info");
    pretty_env_logger::init();

    // Retrieve token
    let token =
        std::env::var("BALD_BOT_TOKEN").expect("Expected bot's token to be found in env vars!");

    #[cfg(debug_assertions)]
    let token = std::env::var("BALD_BOT_DEV_TOKEN").expect("Expected dev token to be in env vars");

    // Define intents and start client
    let intents = GatewayIntents::all();
    let mut client = Client::builder(token, intents)
        .event_handler(BaldHandler)
        .await
        .expect("Able to create client");

    if let Err(why) = client.start().await {
        error!("Client Error: {:?}", why);
    }
}
