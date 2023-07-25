 #[macro_use] extern crate lalrpop_util;

// lalrpop_mod!(pub grammar); // synthesized by LALRPOP

// pub mod rng;
// pub mod dice;
// pub mod atom;
// pub mod bucket;
// pub mod ast;
// pub mod context;
// use atom::{Atom, Value};
// use rng::RNG;
// use dice::*;
// use bucket::Bucket;
// use ast::AST;
// use context::Context;





// fn d(n: i32) -> Dice
// {
//     Dice::new(vec![DiceFace::Range(1..(n+1))])
// }
// fn zerod10() -> Dice
// {
//     Dice::new(vec![DiceFace::Range(0..10)])
// }
// fn degenesis(rng: &mut RNG, score: usize) -> Result<(), String>
// {
    
//     let mut bucket = Bucket::new();
//     bucket.add_several(d(6), score.min(12));
//     bucket.roll(rng);
//     let success = bucket
//         .filter(|num| match num
//                 {
//                     Value::Num(n) => n >= 4,
//                     _ => false
//                 });
//     let triggers = bucket
//         .filter(|num| match num
//                 {
//                     Value::Num(n) => n == 6,
//                     _ => false
//                 });
//     print!("roll:");
//     for dice in bucket.values()
//     {
//         print!(" {:?}", dice.as_num()?);
//     }
//     println!();
//     println!("    {:?} success, {:?} triggers", success.size() + if score > 12 {score - 12} else {0}, triggers.size());

//     return Ok(());

// }

// fn warhammer(rng: &mut RNG, score: i32) -> Result<(), String>
// {
//     let mut bucket = Bucket::new();
//     bucket.add_several(zerod10(), 2);

//     bucket.roll(rng);
//     let dices = bucket.values().iter()
//         .map(|a| a.as_num())
//         .collect::<Result<Vec<i32>, String>>()?;
//     let val = ((dices[0]*10 + dices[1]) + 99)%100 +1;
//     let is_crit = dices[0] == dices[1];
//     println!("{:?}", bucket.values());
//     print!("VAL: {:?} vs {}| ", val, score);
//     if is_crit
//     {
//         print!("CRITICAL ")
//     }
//     if val <= score.max(5).min(95)
//     {
//         print!("SUCCESS");
//     }
//     else
//     {
//         print!("FAILURE");
//     }
//     print!(" ({} degrees)", score/10 - dices[0]);
//     println!();

//     return Ok(());
    
// }






// #[test]
// fn calculator1() {
//     assert!(crate::grammar::TermParser::new().parse("22").is_ok());
//     assert!(crate::grammar::TermParser::new().parse("(22)").is_ok());
//     assert!(crate::grammar::TermParser::new().parse("((((22))))").is_ok());
//     assert!(crate::grammar::TermParser::new().parse("((22)").is_err());
// }

// #[test]
// fn ast() {
//     let mut ctxt = Context::empty();
//     assert!(AST::Add(Box::new(AST::Atom(Atom::Value(Value::Num(4)))), Box::new(AST::Atom(Atom::Value(Value::Num(6))))).eval(&mut ctxt) == Ok(Value::Num(10)));
//     assert!(AST::Seq(vec![
//         AST::Let("i".to_owned(), Box::new(AST::Atom(Atom::Value(Value::Num(0))))),
//         AST::Let("j".to_owned(), Box::new(AST::Atom(Atom::Value(Value::Num(0))))),
//         AST::While(Box::new(AST::Lst(Box::new(AST::Atom(Atom::Ident("i".to_owned()))), Box::new(AST::Atom(Atom::Value(Value::Num(10)))))),
//                    Box::new(AST::Scope(Box::new(AST::Seq(vec![
//                        AST::Assign("i".to_owned(), Box::new(AST::Add(Box::new(AST::Atom(Atom::Ident("i".to_owned()))), Box::new(AST::Atom(Atom::Value(Value::Num(1))))))),
//                        AST::Assign("j".to_owned(), Box::new(AST::Add(Box::new(AST::Atom(Atom::Ident("i".to_owned()))), Box::new(AST::Atom(Atom::Ident("j".to_owned())))))),
//                    ]))))),
//         AST::Atom(Atom::Ident("j".to_owned()))
//     ]).eval(&mut ctxt)

//            == Ok(Value::Num(55)));
// }




// fn main()
// {
//     let mut rng = RNG{};

//     println!("DEGENESIS: ");
//     degenesis(&mut rng, 20);
//     println!("WARHAMMER: ");
//     for _ in 0..1
//     {
//         warhammer(&mut rng, rand::random::<i32>().abs() % 150);
//         println!();
//     }
//     println!("{:?}", crate::grammar::TermParser::new().parse("((((22))))"));
//     println!("{:?}", crate::grammar::TermParser::new().parse("((((-22))))"));

    
// }

mod discord;
mod engine;
mod communication;
pub use discord::Discord;
pub use engine::World;
pub use communication::*;

use tokio::sync::RwLock;

use tokio::sync::mpsc;
use tokio::task::JoinSet;
use engine::ast::*;
/*
discord sur thread spawné, en async
world est sync
*/
#[tokio::main]
async fn main() {
    let (input_tx, mut input_rx) = tokio::sync::mpsc::channel::<InputMessage>(32);
    let (output_tx, mut output_rx) = tokio::sync::mpsc::channel::<OutputMessage>(32);

    let mut discord = Discord::new("!".to_owned(),
                                   |cmd: &str| Some(cmd.chars().rev().collect::<String>()),
                                   input_tx,
                                   output_rx).await;
//    discord.send();

    
    let mut input  = input_rx;
    let     output = output_tx;
    let mut world = World::new(input, output);
    let cmd_test = vec![
        "wh 110+20",
        "wh 14 59-23*2 (69)+2*4",
        "1d8",
        "1d10",
        "1d10+2",
        "1d10+8",
        "sr8 120",
        "124 51 152213 6+5 13d5"
    ];
    for s in cmd_test
    {
        let ex = engine::parse(s);
        print!("{:?}           ", ex);
        println!("{:?}", ex.map(|cmd| cmd.eval()));
    }
    
    // loop
    // {
    //     discord.send();
    //     world.process_all().await;
    // }

    
}

