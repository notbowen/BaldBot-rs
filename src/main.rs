#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod commands;
mod utils;

use poise::event::Event;
use poise::serenity_prelude::{self as serenity, Mentionable};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => error!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            error!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {}", e)
            }
        }
    }
}

async fn handle_event<'a>(ctx: &serenity::Context, event: &Event<'a>) -> Result<(), Error> {
    match event {
        Event::Message { new_message } => {
            if new_message.author.bot {
                return Ok(());
            }

            // Check and respond if there is swear word
            if let Some(response) =
                utils::swear_detector::get_swear_response(new_message.content.clone()).await
            {
                if let Err(why) = new_message
                    .channel_id
                    .send_message(&ctx.http, |message| message.content(response))
                    .await
                {
                    error!("Unable to respond to swear word: {}", why);
                }
            }
        }

        Event::GuildMemberAddition { new_member } => {
            debug!("User `{}` joined!", new_member.user.name);

            // Welcome a user in the Welcome channel of BaldSMP
            let channel_id = serenity::ChannelId(792718122226417704);

            if let Err(why) = channel_id
                .send_message(&ctx.http, |message| {
                    message.content(format!("Welcome to the BaldSMP, {}", new_member.mention()))
                })
                .await
            {
                error!("Unable to send welcome message: {}", why);
            }

            // Add role to user
            if let Err(why) = ctx
                .http
                .add_member_role(
                    791588775456800798,
                    *new_member.user.id.as_u64(),
                    791594597577916426,
                    None,
                )
                .await
            {
                error!(
                    "Unable to add role to user {}: {}",
                    new_member.user.name, why
                );
            }
            debug!("Added Bald role to {}", new_member.user.name);
        }
        _ => (),
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialise logging
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "bald_bot=debug");
    pretty_env_logger::init();

    // Retrieve token
    let token =
        std::env::var("BALD_BOT_TOKEN").expect("Expected bot's token to be found in env vars!");

    // Init poise
    let options = poise::FrameworkOptions {
        commands: vec![commands::ping()],
        on_error: |error| Box::pin(on_error(error)),
        event_handler: |ctx, event, _framework, _data| Box::pin(handle_event(ctx, event)),
        ..Default::default()
    };

    poise::Framework::builder()
        .token(token)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                info!("Logged in as: {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .intents(serenity::GatewayIntents::all())
        .run()
        .await
        .unwrap();
}
