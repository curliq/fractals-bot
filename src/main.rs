extern crate kankyo;

use std::env;

#[macro_use]
extern crate serenity;

use serenity::{
    client::Client,
    client::Context,
    model::gateway::Ready,
    model::event::ResumedEvent,
    prelude::EventHandler,
    framework::StandardFramework,
};

mod commands;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _resume: ResumedEvent) {
        println!("Resumed");
    }
}

fn main() {
    kankyo::load().expect("Failed to load .env file");

    let mut client = Client::new(
        &env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN is not set"),
        Handler,
    ).expect("Error creating client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.prefix(&env::var("PREFIX").expect("PREFIX not set"))
            })
            .cmd("start", commands::fractals::start)
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
