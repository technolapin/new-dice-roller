use crate::dice::Dice;
use crate::bucket::Bucket;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value
{
    Str(String),
    Num(i32),
    Die(Dice),
    Buc(Bucket),
    Nil
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Atom
{
    Ident(String), // variables, ....
    Value(Value)
}

use Value::*;

impl Value
{
    pub fn as_num(&self) -> Result<i32, String>
    {
        match self
        {
            Num(n) => Ok(*n),
            _ => Err(format!("Cannot interpret {:?} as a number", self))
        }
    }
    pub fn is_true(&self) -> bool
    {
        match self
        {
            Num(n) => *n != 0,
            _ => true
        }
    }
}

impl std::fmt::Display for Value
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Num(n) => write!(f, "{}", n),
            Str(s) => write!(f, "{}", s),
            Die(d) => write!(f, "{:?}", d),
            Buc(b) => write!(f, "{:?}", b),
            Nil => write!(f, "()")
        }
    }
}


    
impl Value
{
    pub fn add(a: &Self, b: &Self) -> Result<Self, String>
    {
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num(a+b)),
            (Str(a), b) => Ok(Str(format!("{}{}", a, b))),
            (a, Str(b)) => Ok(Str(format!("{}{}", a, b))),
            (Die(a), Die(b)) => Ok(Buc(Bucket::from_vec(vec![a.clone(),b.clone()]))),
            (Buc(a), Die(b)) | (Die(b), Buc(a)) => {let mut buck = a.clone(); buck.add(b.clone()); Ok(Buc(buck))},
            (Buc(a), Buc(b)) => {let mut buck = a.clone(); buck.fuse(b.clone()); Ok(Buc(buck))}
            _ => Err(format!("{:?} + {:?} is not valid", a, b))
        }
    }
    pub fn sub(a: &Self, b: &Self) -> Result<Self, String>
    {
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num(a-b)),
            _ => Err(format!("{:?} - {:?} is not valid", a, b))
        }
    }
    pub fn mul(a: &Self, b: &Self) -> Result<Self, String>
    {
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num(a*b)),
            _ => Err(format!("{:?} * {:?} is not valid", a, b))
        }
    }
    pub fn div(a: &Self, b: &Self) -> Result<Self, String>
    {
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num(a/b)),
            _ => Err(format!("{:?} / {:?} is not valid", a, b))
        }
    }
    pub fn rem(a: &Self, b: &Self) -> Result<Self, String>
    {
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num(a%b)),
            _ => Err(format!("{:?} % {:?} is not valid", a, b))
        }
    }
    pub fn or(a: &Self, b: &Self) -> Result<Self, String>
    {
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num(((*a != 0) || (*b != 0)) as i32)),
            _ => Err(format!("{:?} || {:?} is not valid", a, b))
        }
    }
    pub fn and(a: &Self, b: &Self) -> Result<Self, String>
    {
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num(((*a != 0) && (*b != 0)) as i32)),
            _ => Err(format!("{:?} && {:?} is not valid", a, b))
        }
    }
    pub fn xor(a: &Self, b: &Self) -> Result<Self, String>
    {
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num(((*a != 0) ^ (*b != 0)) as i32)),
            _ => Err(format!("{:?} ^ {:?} is not valid", a, b))
        }
    }

    pub fn equ(a: &Self, b: &Self) -> Result<Self, String>
    {
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num((a==b) as i32)),
            (Str(a), Str(b)) => Ok(Num((a==b) as i32)),
            _ => Err(format!("{:?} == {:?} is not valid", a, b))
        }
    }
    pub fn geq(a: &Self, b: &Self) -> Result<Self, String>
    {
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num((a>=b) as i32)),
            (Str(a), Str(b)) => Ok(Num((a>=b) as i32)),
            _ => Err(format!("{:?} >= {:?} is not valid", a, b))
        }
    }
    pub fn leq(a: &Self, b: &Self) -> Result<Self, String>
    {
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num((a<=b) as i32)),
            (Str(a), Str(b)) => Ok(Num((a<=b) as i32)),
            _ => Err(format!("{:?} <= {:?} is not valid", a, b))
        }
    }
    pub fn gst(a: &Self, b: &Self) -> Result<Self, String>
    {
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num((a>b) as i32)),
            (Str(a), Str(b)) => Ok(Num((a>b) as i32)),
            _ => Err(format!("{:?} > {:?} is not valid", a, b))
        }
    }
    pub fn lst(a: &Self, b: &Self) -> Result<Self, String>
    {
        match (a, b)
        {
            (Num(a), Num(b)) => Ok(Num((a<b) as i32)),
            (Str(a), Str(b)) => Ok(Num((a<b) as i32)),
            _ => Err(format!("{:?} < {:?} is not valid", a, b))
        }
    }

    pub fn minus(a: &Self) -> Result<Self, String>
    {
        match a
        {
            Num(a) => Ok(Num(-a)),
            _ => Err(format!("- {:?} is not valid", a))
        }
    }
    pub fn not(a: &Self) -> Result<Self, String>
    {
        match a
        {
            Num(0) => Ok(Num(1)),
            Num(_) => Ok(Num(0)),
            _ => Err(format!("! {:?} is not valid", a))
        }
    }
}
