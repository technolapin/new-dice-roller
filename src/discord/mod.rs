pub use crate::engine::World;
pub use crate::communication::{InputMessage, OutputMessage};

use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tokio::sync::mpsc::{Sender, Receiver};

use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;

struct SendToEngine;
struct FetchFromEngine;

impl TypeMapKey for SendToEngine {
    type Value = Sender<InputMessage>;
}
impl TypeMapKey for FetchFromEngine {
    type Value = Receiver<OutputMessage>;
}

struct DiscordHandler<F>
    where F: Fn(&str, &Message) -> Option<String>
{
    prefix: String,
    cmd_processor: F,
}


#[async_trait]
impl<F> EventHandler for DiscordHandler<F>
    where F: Fn(&str, &Message)->Option<String> + Send + Sync
{
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message)
    {
        let pref_len = self.prefix.len();
        let pref = msg.content.get(..pref_len);
        let post = msg.content.get(pref_len..);
        match (pref == Some(&self.prefix), post)
        {
            (_, Some("")) => (),
            (true, Some(cmd)) =>
            {
                if let Some(response) = (self.cmd_processor)(cmd, &msg)
                {
                    println!("User id: {}", msg.author.id);
                    println!("{}", msg.content);
                    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
            _ => ()
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


pub struct Discord
{
 //   handler: DiscordHandler<F>,
//    client: Client,
//    received: Sender<InputMessage>,
    //    to_send: Receiver<OutputMessage>,
    // watcher: std::thread::JoinHandle<()>,
    // to_send: Receiver<OutputMessage>
}

impl Discord
{
    /*
    pub async fn new<F>(prefix: String,
                        cmd_processor: F,
                        received: Sender<InputMessage>,
                        to_send: Receiver<OutputMessage>) -> Self
    where F: Fn(&str)->Option<String> + Send + Sync + 'static
    {
        let handler = DiscordHandler{prefix, cmd_processor};
        let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;
        let mut client = Client::builder(&token, intents)
            .event_handler(handler)
            .await.expect("Err creating client");
        
        {
            let mut data = client.data.write().await;
            data.insert::<SendToEngine>(received);
//            data.insert::<FetchFromEngine>(to_send);
        }
        let watcher = std::thread::spawn(move ||{
            let mut rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(
            client.start());
        });
        Self{
            watcher,
            to_send
        }
    }
*/
    pub async fn run<F>(prefix: String,
                        cmd_processor: F)
    where F: Fn(&str, &Message)->Option<String> + Send + Sync + 'static
    {
        let handler = DiscordHandler{prefix, cmd_processor};
        let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;
        let mut client = Client::builder(&token, intents)
            .event_handler(handler)
            .await.expect("Err creating client");

        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    }
    // pub async fn send(&mut self) -> Result<(), String>
    // {
    //     while let Ok(msg) = self.to_send.try_recv()
    //     {
    //         println!("{:?}", msg);
    //         if let Err(why) = serenity::model::prelude::ChannelId(msg.chanid)
    //             .say(&ctx.http, response).await {
    //             println!("Error sending message: {:?}", why);
    //         }
    //     }
    //     Ok(())
    // }
    
}

