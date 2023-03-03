use crate::atom::Atom;
use crate::context::Context;


#[derive(Debug, Clone)]
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

    Equ(Box<Expression>, Box<Expression>),
    Geq(Box<Expression>, Box<Expression>),
    Leq(Box<Expression>, Box<Expression>),
    Gst(Box<Expression>, Box<Expression>),
    Lst(Box<Expression>, Box<Expression>),

    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Xor(Box<Expression>, Box<Expression>),

    Not(Box<Expression>),
    
    Minus(Box<Expression>),

    If(Box<Expression>, Box<Expression>, Box<Expression>),
    While(Box<Expression>, Box<Expression>),
    Let(String, Box<Expression>),
    Assign(String, Box<Expression>)
    
}





impl Expression
{
    pub fn eval(&self) -> Result<Atom, String>
    {
       
        Ok(match self
        {
            Expression::Atom(a) => a.clone(),
            Expression::Add(a, b) => Atom::add(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Sub(a, b) => Atom::sub(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Mul(a, b) => Atom::mul(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Div(a, b) => Atom::div(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Rem(a, b) => Atom::rem(&(*a).eval()?, &(*b).eval()?)?,

            Expression::Or (a, b) => Atom::or(&(*a).eval()?, &(*b).eval()?)?,
            Expression::And(a, b) => Atom::and(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Xor(a, b) => Atom::xor(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Not(a) => Atom::not(&(*a).eval()?)?,
            Expression::Minus(a) => Atom::minus(&(*a).eval()?)?,

            Expression::Equ (a, b) => Atom::equ(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Geq (a, b) => Atom::geq(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Leq (a, b) => Atom::leq(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Gst (a, b) => Atom::gst(&(*a).eval()?, &(*b).eval()?)?,
            Expression::Lst (a, b) => Atom::lst(&(*a).eval()?, &(*b).eval()?)?,

            
            Expression::If(cond, a, b) => if (*cond).eval()?.is_true() {(*a).eval()?} else {(*b).eval()?},
            Expression::While(cond, expr) =>
            {
                while (*cond).eval()?.is_true()
                {
                    (*expr).eval()?;
                }
                Atom::Str("nil".to_owned())
            },
            Expression::Let(ident, e) => unimplemented!(),
            Expression::Assign(ident, e) => unimplemented!(),
            
        })
    }
}
