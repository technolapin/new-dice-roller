use crate::atom::Atom;


pub enum Expression
{
    Atom(Atom),
  //  Seq(Vec<Expression>),

    // binary operators
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Rem(Box<Expression>, Box<Expression>),

    Minus(Box<Expression>),

    If(Box<Expression>, Box<Expression>, Box<Expression>)

    
}




impl Expression
{
    pub fn eval(self) -> Result<Atom, String>
    {
       
        Ok(match self
        {
            Expression::Atom(a) => a,
            Expression::Add(a, b) => Atom::add(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Sub(a, b) => Atom::sub(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Mul(a, b) => Atom::mul(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Div(a, b) => Atom::div(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Rem(a, b) => Atom::rem(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Minus(a) => Atom::minus(&(*a).eval()?)?,
            Expression::If(cond, a, b) => if ((*cond).eval()?.is_true()) {(*a).eval()?} else {(*b).eval()?}
        })
    }
}
