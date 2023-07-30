pub mod utils;
pub mod commands;
pub mod ast;
pub use crate::communication::{InputMessage, OutputMessage};


pub fn parse(s: &str) -> Result<Vec<commands::Command>, String>
{
    return crate::expression::CmdListParser::new().parse(s).map_err(|e| format!("{:?}", e));
}




use tokio::sync::mpsc::{Receiver, Sender};


pub struct World
{
    input: Receiver<InputMessage>,
    output: Sender<OutputMessage>
}

impl World
{
    pub fn new(input: Receiver<InputMessage>,
               output: Sender<OutputMessage>) -> Self
    {
        Self{
            input, output
        }
    }

 // pub fn get(&mut self, cmd: String)
 //    {
 //        self.input.push(cmd);
 //    }


    // pub async fn process_all(&mut self)
    // {
    //     while let Ok(msg) = self.input.try_recv()
    //     {
    //         println!("recus {:?}", msg);

    //         let response = OutputMessage{chanid: msg.chanid, content: msg.content.chars().rev().collect::<String>()+"_answer"};
    //         // devrait bloquer
    //         self.output.send(response).await;
    //     }
    // }
    pub fn process(&mut self, msg: InputMessage) -> OutputMessage
    {
        unimplemented!();
    }
}
