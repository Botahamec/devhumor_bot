extern crate serenity;
extern crate rawr;
extern crate rand;

use serenity::{
	client::Client,
	model::{
		channel::Message,
		gateway::Ready
	},
	prelude::{EventHandler, Context}
};
use serenity::framework::standard::{
	StandardFramework,
	CommandResult,
	macros::{
		command,
		group
	}
};

use rawr::prelude::*;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

// GENERAL COMMANDS

#[command]
fn devhumor(ctx: &mut Context, msg: &Message) -> CommandResult {
	// Creates a new client to access the reddit API. You need to set a user agent so Reddit knows
    // who is using this client.
    let client = RedditClient::new("DevHumor", AnonymousAuthenticator::new());
    // Access the subreddit /r/rust.
    let subreddit = client.subreddit("ProgrammerHumor");
    // Gets the hot listing of /r/rust. If the API request fails, we will panic with `expect`.
    let mut hot_listing = subreddit.hot(ListingOptions::default()).expect("Could not fetch post listing!");
    // Randomly selects one of the top 50 posts
	let rand_num = rand::thread_rng().gen_range(0, 50);
	let post = hot_listing.nth(rand_num).unwrap();
	match post.link_url() {
		Some(url) => {if let Err(e) = msg.channel_id.say(&ctx.http, url) {
			println!("An error occurred: {}", e);
		};},
		None => {if let Err(e) = msg.channel_id.say(&ctx.http, "An error occurred") {
			println!("An error occurred: {}", e);
		};}
	}
	Ok(())
}

group!({
	name: "general",
	options: {},
	commands: [devhumor]
});

struct Handler;

impl EventHandler for Handler {
	fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
		println!("It is now fun time!");
	}
}

fn main() {

	// Get key from an external file
	let mut discord_token = String::new();
	let mut key_file = File::open(".key").unwrap();
	key_file.read_to_string(&mut discord_token).unwrap();

	// Login with a bot token from the environment
	let mut client = Client::new(discord_token, Handler)
		.expect("Error creating client");
	client.with_framework(StandardFramework::new()
		.configure(|c| c.prefix("!")) // set the bot's prefix to ":"
		.group(&GENERAL_GROUP));

	// start listening for events by starting a single shard
	if let Err(why) = client.start() {
		println!("An error occurred while running the client: {:?}", why);
	}
}
