use crate::atom::{Value, Atom};
use crate::context::Context;


#[derive(Debug, Clone)]
pub enum AST
{
    Atom(Atom),
  //  Seq(Vec<AST>),

    // binary operators
    Add(Box<AST>, Box<AST>),
    Sub(Box<AST>, Box<AST>),
    Mul(Box<AST>, Box<AST>),
    Div(Box<AST>, Box<AST>),
    Rem(Box<AST>, Box<AST>),

    Equ(Box<AST>, Box<AST>),
    Geq(Box<AST>, Box<AST>),
    Leq(Box<AST>, Box<AST>),
    Gst(Box<AST>, Box<AST>),
    Lst(Box<AST>, Box<AST>),

    And(Box<AST>, Box<AST>),
    Or(Box<AST>, Box<AST>),
    Xor(Box<AST>, Box<AST>),

    Not(Box<AST>),
    
    Minus(Box<AST>),

    If(Box<AST>, Box<AST>, Box<AST>),
    While(Box<AST>, Box<AST>),
    Let(String, Box<AST>),
    Assign(String, Box<AST>),
    Scope(Box<AST>),
    Seq(Vec<AST>)
        
}





impl AST
{
    pub fn eval(&self, ctxt: &mut Context) -> Result<Value, String>
    {
       
        Ok(match self
        {
            AST::Atom(a) => match a
            {
                Atom::Value(v) => v.clone(),
                Atom::Ident(name) => ctxt.get(name)?.clone()
            },
            AST::Add(a, b) => Value::add(&(*a).eval(ctxt)?, &(*b).eval(ctxt)?)?,
            AST::Sub(a, b) => Value::sub(&(*a).eval(ctxt)?, &(*b).eval(ctxt)?)?,
            AST::Mul(a, b) => Value::mul(&(*a).eval(ctxt)?, &(*b).eval(ctxt)?)?,
            AST::Div(a, b) => Value::div(&(*a).eval(ctxt)?, &(*b).eval(ctxt)?)?,
            AST::Rem(a, b) => Value::rem(&(*a).eval(ctxt)?, &(*b).eval(ctxt)?)?,

            AST::Or (a, b) => Value::or(&(*a).eval(ctxt)?, &(*b).eval(ctxt)?)?,
            AST::And(a, b) => Value::and(&(*a).eval(ctxt)?, &(*b).eval(ctxt)?)?,
            AST::Xor(a, b) => Value::xor(&(*a).eval(ctxt)?, &(*b).eval(ctxt)?)?,
            AST::Not(a) => Value::not(&(*a).eval(ctxt)?)?,
            AST::Minus(a) => Value::minus(&(*a).eval(ctxt)?)?,

            AST::Equ (a, b) => Value::equ(&(*a).eval(ctxt)?, &(*b).eval(ctxt)?)?,
            AST::Geq (a, b) => Value::geq(&(*a).eval(ctxt)?, &(*b).eval(ctxt)?)?,
            AST::Leq (a, b) => Value::leq(&(*a).eval(ctxt)?, &(*b).eval(ctxt)?)?,
            AST::Gst (a, b) => Value::gst(&(*a).eval(ctxt)?, &(*b).eval(ctxt)?)?,
            AST::Lst (a, b) => Value::lst(&(*a).eval(ctxt)?, &(*b).eval(ctxt)?)?,

            
            AST::If(cond, a, b) => if (*cond).eval(ctxt)?.is_true() {(*a).eval(ctxt)?} else {(*b).eval(ctxt)?},
            AST::While(cond, expr) =>
            {
                while (*cond).eval(ctxt)?.is_true()
                {
                    (*expr).eval(ctxt)?;
                }
                Value::Nil
            },
            AST::Let(ident, e) => {
                let val = (*e).eval(ctxt)?;
                ctxt.set_var(ident, val.clone())?;
                val
            },
            AST::Assign(ident, e) => {
                let val = (*e).eval(ctxt)?;
                let var: &mut Value = ctxt.get_mut(ident)?;
                *var = val.clone();
                val
            },
            AST::Scope(e) =>
            {
                ctxt.push();
                let tmp = (*e).eval(ctxt)?;
                ctxt.pop();
                tmp
            },
            AST::Seq(vec) =>
            {
                if vec.len() > 0
                {
                    for e in &vec[0..vec.len()-1]
                    {
                        e.eval(ctxt)?;
                    }
                    vec.last().unwrap().eval(ctxt)?
                }
                else
                {
                    Value::Nil
                }
            },
        })
    }
}
