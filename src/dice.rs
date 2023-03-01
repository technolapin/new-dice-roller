use crate::atom::Atom;
use crate::rng::RNG;
use std::ops::Range;

#[derive(Debug, Clone)]
pub enum DiceFace
{
    Symbol(String),
    Range(Range<i32>)
}

impl DiceFace
{
    fn size(&self) -> usize
    {
        match &self
        {
            DiceFace::Range(range) => if range.is_empty() {0} else {(range.end - range.start) as usize},
            _ => 1
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dice
{
    faces: Vec<DiceFace>,
    nb_faces: usize,
    state: usize
}


impl Dice
{
    pub fn new(faces: Vec<DiceFace>) -> Self
    {
        let nb_faces = faces.iter().map(|face| face.size()).sum();
        Self
        {
            faces,
            nb_faces,
            state: 0
        }
    }

    pub fn roll(&mut self, rng: &mut RNG)
    {
        self.state = rng.range(0, self.nb_faces);
    }

    pub fn value(&self) -> Atom
    {
        let mut rmd = self.state;
        for face in &self.faces
        {
            // ignoring bad ranges
            // (we would be stuck instead)
            if face.size() == 0 { continue; }
            
            if rmd < face.size()
            {
                match face
                {
                    DiceFace::Symbol(s) =>
                    {
                        return Atom::Str(s.to_string());
                    },
                    DiceFace::Range(rg) => 
                    {
                        return Atom::Num(rg.start + (rmd as i32));
                    }
                }
            }
            rmd -= face.size();
        }
        unreachable!();
    }
    
}


