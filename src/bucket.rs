use crate::atom::Value;
use crate::rng::RNG;
use crate::dice::Dice;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bucket(Vec<Dice>);

impl Bucket
{
    pub fn from_vec(vec: Vec<Dice>) -> Self
    {
        Self(vec)
    }
    pub fn new() -> Self
    {
        return Self(vec![])
    }
    pub fn size(&self) -> usize
    {
        self.0.len()
    }
    pub fn add(&mut self, dice: Dice) -> &mut Self
    {
        self.0.push(dice);
        self
    }
    pub fn add_several(&mut self, dice: Dice, nb: usize) -> &mut Self
    {
        for _ in 0..nb
        {
            self.0.push(dice.clone());
        }
        self
    }
    pub fn fuse(&mut self, mut rhs:Bucket)
    {
        self.0.append(&mut rhs.0);
    }
    pub fn roll(&mut self, rng: &mut RNG)
    {
        for dice in &mut self.0
        {
            dice.roll(rng);
        }
    }
    pub fn fold<F>(&self, start: Value, f: F) -> Value
    where
        F: Fn(Value, Value) -> Value
    {
        self.0.iter()
            .map(|dice| dice.value())
            .fold(start, f)
    }
    pub fn filter<F>(&self, f: F) -> Bucket
    where
        F: Fn(Value) -> bool
    {
        Self{0: self.0.iter()
             .filter(|dice| f(dice.value())).cloned().collect()}
    }
    
    pub fn values(&self) -> Vec<Value>
    {
        self.0.iter().map(|dice| dice.value()).collect()
    }
}



