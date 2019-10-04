extern crate serenity;
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
	Args,
	macros::{
		command,
		group
	}
};

mod rawr;
use rawr::prelude::*;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

// SETUP COMMANDS

#[command]
fn setchannel(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	Ok(())
}

#[command]
fn setinterval(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	Ok(())
}

// GENERAL COMMANDS

#[command]
fn devhumor(ctx: &mut Context, msg: &Message) -> CommandResult {
	// Creates a new client to access the reddit API. You need to set a user agent so Reddit knows
    // who is using this client.
    let client = RedditClient::new("your user agent here", AnonymousAuthenticator::new());
    // Access the subreddit /r/rust.
    let subreddit = client.subreddit("ProgrammerHumor");
    // Gets the hot listing of /r/rust. If the API request fails, we will panic with `expect`.
    let mut hot_listing = subreddit.hot(ListingOptions::default()).expect("Could not fetch post listing!");
    // Randomly selects one of the top 50 posts
	let top_posts = hot_listing.take(50);
	let rand_num = rand::thread_rng().gen_range(0, 50);
	let post = top_posts[rand_num];
	match post.link_url() {
		Some(url) => {},
		None => {}
	}
}

group!({
	name: "setup",
	options: {},
	commands: [setchannel, setinterval]
});

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
	let mut DISCORD_TOKEN = String::new();
	let mut key_file = File::open("files/.key").unwrap();
	key_file.read_to_string(&mut DISCORD_TOKEN).unwrap();

	// Login with a bot token from the environment
	let mut client = Client::new(DISCORD_TOKEN, Handler)
		.expect("Error creating client");
	client.with_framework(StandardFramework::new()
		.configure(|c| c.prefix(":")) // set the bot's prefix to ":"
		.group(&SETUP_GROUP)
		.group(&GENERAL_GROUP));

	// start listening for events by starting a single shard
	if let Err(why) = client.start() {
		println!("An error occurred while running the client: {:?}", why);
	}
}
