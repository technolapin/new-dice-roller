use crate::atom::Value;

use std::collections::{HashMap, HashSet};


#[derive(Debug, Default)]
pub struct Context
{
    vars: HashMap<String, Vec<Value>>,
    scopes: Vec<HashSet<String>>
}

// #[derive(Debug, Default)]
// pub struct Context
// {
//     values: Vec<Value>,
//     maps: Vec<(usize, HashMap<String, usize>)>
// }

impl Context
{
    pub fn new() -> Self
    {
        Self::default()
    }


    pub fn get(&self, identifier: &str) -> Result<&Value, String>
    {
        self.vars
            .get(identifier)
            .map(|vec| vec.last())
            .flatten()
            .ok_or(format!("Variable {} out of scope", identifier))
    }
    
    pub fn get_mut(&mut self, identifier: &str) -> Result<&mut Value, String>
    {
        self.vars
            .get_mut(identifier)
            .map(|vec| vec.last_mut())
            .flatten()
            .ok_or(format!("Variable {} out of scope", identifier))
    }

    // fn is_var_in_current_scope(&self, identifier: &str) -> bool
    // {
    //     self.scopes.last().map(|scope| scope.contains(identifier)) == Some(true)
    // }
        
    
    pub fn set_var(&mut self, identifier: &str, val: Value) -> Result<(), String>
    {
        if self.scopes.last().map(|scope| scope.contains(identifier)) == Some(true)
        {
            Err(format!("Variable {} is already declared in this scope.", identifier))
        }
        else
        {
            self.scopes.last_mut().map(|scope| scope.insert(identifier.to_owned()));
            if !self.vars.contains_key(identifier)
            {
                self.vars.insert(identifier.to_owned(), vec![val]);
            }
            else
            {
                self.vars.get_mut(identifier).unwrap().push(val);
            }
            Ok(())
        }
    }
    pub fn push(&mut self)
    {
        self.scopes.push(HashSet::new());
    }
    pub fn pop(&mut self)
    {
        for id in self.scopes.last().expect("POPED AN EMPTY CONTEXT")
        {
            self.vars.get_mut(id).unwrap().pop();
        }
        self.scopes.pop();
    }
    
}


