#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

pub mod rng;
pub mod dice;
pub mod atom;
pub mod bucket;
pub mod ast;
pub mod context;
use atom::Atom;
use rng::RNG;
use dice::*;
use bucket::Bucket;
use ast::Expression;





fn d(n: i32) -> Dice
{
    Dice::new(vec![DiceFace::Range(1..(n+1))])
}
fn zerod10() -> Dice
{
    Dice::new(vec![DiceFace::Range(0..10)])
}
fn degenesis(rng: &mut RNG, score: usize) -> Result<(), String>
{
    
    let mut bucket = Bucket::new();
    bucket.add_several(d(6), score.min(12));
    bucket.roll(rng);
    let success = bucket
        .filter(|num| match num
                {
                    Atom::Num(n) => n >= 4,
                    _ => false
                });
    let triggers = bucket
        .filter(|num| match num
                {
                    Atom::Num(n) => n == 6,
                    _ => false
                });
    print!("roll:");
    for dice in bucket.values()
    {
        print!(" {:?}", dice.as_num()?);
    }
    println!();
    println!("    {:?} success, {:?} triggers", success.size() + if score > 12 {score - 12} else {0}, triggers.size());

    return Ok(());

}

fn warhammer(rng: &mut RNG, score: i32) -> Result<(), String>
{
    let mut bucket = Bucket::new();
    bucket.add_several(zerod10(), 2);

    bucket.roll(rng);
    let dices = bucket.values().iter()
        .map(|a| a.as_num())
        .collect::<Result<Vec<i32>, String>>()?;
    let val = ((dices[0]*10 + dices[1]) + 99)%100 +1;
    let is_crit = dices[0] == dices[1];
    println!("{:?}", bucket.values());
    print!("VAL: {:?} vs {}| ", val, score);
    if is_crit
    {
        print!("CRITICAL ")
    }
    if val <= score.max(5).min(95)
    {
        print!("SUCCESS");
    }
    else
    {
        print!("FAILURE");
    }
    print!(" ({} degrees)", score/10 - dices[0]);
    println!();

    return Ok(());
    
}



fn main()
{
    let mut rng = RNG{};

    println!("DEGENESIS: ");
    degenesis(&mut rng, 20);
    println!("WARHAMMER: ");
    for _ in 0..1
    {
        warhammer(&mut rng, rand::random::<i32>().abs() % 150);
        println!();
    }
    println!("{:?}", crate::grammar::TermParser::new().parse("((((22))))"));
    println!("{:?}", crate::grammar::TermParser::new().parse("((((-22))))"));
}



#[test]
fn calculator1() {
    assert!(crate::grammar::TermParser::new().parse("22").is_ok());
    assert!(crate::grammar::TermParser::new().parse("(22)").is_ok());
    assert!(crate::grammar::TermParser::new().parse("((((22))))").is_ok());
    assert!(crate::grammar::TermParser::new().parse("((22)").is_err());
}

#[test]
fn ast() {
    assert!(Expression::Add(Box::new(Expression::Atom(Atom::Num(4))), Box::new(Expression::Atom(Atom::Num(6)))).eval() == Ok(Atom::Num(10)));
}



