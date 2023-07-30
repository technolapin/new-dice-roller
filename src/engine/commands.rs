use crate::engine::utils::*;
use new_dice_roller::misc::Error;
use crate::Storage;

#[derive(Debug)]
pub enum Value
{
    Id(String),
    Num(i64),
    Str(String)
}

#[derive(Debug)]
pub enum ValueRef<'a>
{
    Id(&'a str),
    Num(i64),
    Str(&'a str)
}

impl Value
{
    fn to_ref<'a>(&'a self) -> ValueRef<'a>
    {
        match self
        {
            Value::Id(id) => ValueRef::Id(&id),
            Value::Num(n) => ValueRef::Num(*n),
            Value::Str(s) => ValueRef::Str(&s)
        }
    }
}

impl std::fmt::Display for Value
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
    {
        match self
        {
            Value::Id(id) => write!(f, "{}", id),
            Value::Num(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, r#""{}""#, s)
        }
    }
}
impl<'a> std::fmt::Display for ValueRef<'a>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
    {
        match self
        {
            ValueRef::Id(id) => write!(f, "{}", id),
            ValueRef::Num(n) => write!(f, "{}", n),
            ValueRef::Str(s) => write!(f, r#"{}""#, s)
        }
    }
}

#[derive(Debug)]
pub struct Command
{
    pub lst: Vec<Value>
}

impl Command
{
    pub fn eval(&self, userid: u64) -> Result<String, Error>
    {
        use ValueRef::{Id, Num, Str};
        let peek = self.lst.iter().map(|v|v.to_ref()).collect::<Vec<ValueRef>>();
        let (title, args) = match peek.first()
        {
            None => Err(Error("Empty command!".to_owned()))?,
            Some(Str(a)) => (Some(a), &peek[1..]),
            _ => (None, &peek[..])
        };
        let output = match args
        {
            &[] => String::new(),
            &[Num(v), ..] => args.iter().map(|n| format!("{}",n)).reduce(|acc, s| acc + " " + s.as_str())
                .unwrap_or_default(),
            &[Str(s), ..] => args.iter().map(|n| format!("{}",n)).reduce(|acc, s| acc + " " + s.as_str())
                .unwrap_or_default(),
            &[Id("#"), Num(v)] => format!("{}", v),

            &[Id("register"), Id(key), Str(cmd)] => {
                unimplemented!();
                Storage::open()?
                    .store_command(userid as i64, key, cmd)?;
                format!(r#"Registered command "{}" as "{}""#, cmd, key)
            },
            
            &[Id("wh"), Num(score)] => warhammer(score),


            &[Id("sr"), Num(n)] => shadowrun(n, None, false),
            &[Id("sr"), Num(n), Num(goal)] => shadowrun(n, Some(goal), false),
            &[Id("srboom"), Num(n)] => shadowrun(n, None, true),
            &[Id("srboom"), Num(n), Num(goal)] => shadowrun(n, Some(goal), true),
            
            &[Id("dg"), Num(n)] => degenesis(n, None),
            &[Id("dg"), Num(n), Num(goal)] => degenesis(n, Some(goal)),

            &[Id("trud"), Num(n)] => trudvang(n, 11, 0),
            &[Id("trud"), Num(n), Num(bonus)] => trudvang(n, 11, bonus),
            &[Id("trudboom"), Num(n), Num(expl_tresh)] => trudvang(n, expl_tresh, 0),
            &[Id("trudboom"), Num(n), Num(expl_tresh), Num(bonus)] => trudvang(n, expl_tresh, bonus),

            &[Id("brigandine"), Num(n), Num(expl_tresh)] => brigandine(n, expl_tresh),

            &[Id("vamp"), Num(score), Num(difficulty)] => vampire(score, difficulty, false),
            &[Id("vampspe"), Num(score), Num(difficulty)] => vampire(score, difficulty, true),


            &[Id("shaan")] =>
            {
                format!("Corp: {}   Esprit: {}   Âme: {}", dice(10), dice(10), dice(10))
            }

            _ => Err(Error(format!("Unknown command pattern {:?}", self)))?
                
        };

        match title
        {
            Some(title) => Ok(format!("```markdown\n## {}\n{}```", title, output)),
            None => Ok(format!("```markdown\n{}```", output))
        }
            
    }
}


/////////////////////////////////////////////////////////////
// COMMANDS IMPLEMENTATION
fn warhammer(score: i64) -> String
{
    let d = dice(100);
    if d % 11 == 0
    {
        if d <= score
        {
            format!("{}: Réussite critique de {} degrés", d, score/10 - d/10)
        }
        else
        {
            format!("{}: Échec critique de {} degrés", d, d/10 - score/10)
        }
        
    }
    else
    {
        if d <= score
        {
            format!("{}: Réussite de {} degrés", d, score/10 - d/10)
        }
        else
        {
            format!("{}: Échec de {} degrés", d, d/10 - score/10)
        }

    }
}

fn shadowrun(n: i64, maybe_goal: Option<i64>, explode: bool) -> String
{
    let mut dies = (0..n).map(|_| dice(6)).collect::<Vec<_>>();
    dies.sort();

    let mut n_success = dies.iter().filter(|&&a| a >= 5).count() as i64;
    let mut n_ones =  dies.iter().filter(|&&a| a == 1).count() as i64;
    let mut all_throws = vec![dies.clone()];
    let mut n_sixs = dies.iter().filter(|&&a| a == 6).count() as i64;
    while explode && n_sixs != 0
    {
        let mut dies = (0..n_sixs).map(|_| dice(6)).collect::<Vec<_>>();
        dies.sort();
        n_sixs = dies.iter().filter(|&&a| a == 6).count() as i64;
        n_success += dies.iter().filter(|&&a| a >= 5).count() as i64;
        n_ones +=  dies.iter().filter(|&&a| a == 1).count() as i64;
        all_throws.push(dies);
    }

    
    let throw_s = all_throws.into_iter()
        .fold(String::new(), |s, dies|
              {
                  let m = dies.iter().fold(String::new(), |s, n| format!("{} {}", s, n));
                  format!("{} [{}]", s, m)
              });
    
    let complication_m = if n_ones > n_success
    {format!("Complication - ")}
    else
    {String::new()};
    
    let msg = if let Some(goal) = maybe_goal
    {
        if goal <= n_success
        {
            format!("Réussite ({}/{})", n_success, goal)
        }
        else if n_ones > n_success
        {
            format!("Échec critique! ({}/{})", n_success, goal)
        }
        else
        {
            format!("Échec ({}/{})", n_success, goal)
        }
    }
    else
    {
        if n_ones > n_success
        {
            format!("{} réussites (Risque d'échec critique)", n_success)
        }
        else
        {
            format!("{} réussites", n_success)
        }

    };

    format!("{}\n{}{}", throw_s, complication_m, msg)
        
}


fn degenesis(n: i64, maybe_goal: Option<i64>) -> String
{
    let mut dies = (0..n.min(12)).map(|_| dice(6)).collect::<Vec<_>>();
    dies.sort();
    let n_auto = n - dies.len() as i64;
    let n_success = dies.iter().filter(|&&a| a >= 4).count() as i64 + n_auto;
    let n_trigg = dies.iter().filter(|&&a| a == 6).count() as i64;
    let n_ones =  dies.iter().filter(|&&a| a == 1).count() as i64;

    let throw_s = dies.iter()
        .fold(String::new(), |s, n| format!("{} {}", s, n));

    let m_auto = if n_auto == 0 {String::new()} else {format!("({} automatiques) ", n_auto)};
    let m_triggers = if n_trigg == 0 {String::new()} else {format!("dont {} triggers", n_trigg)};
    let m_bilan = format!("{} réussites {}{}", n_success, m_auto, m_triggers);
    let m_analyse = if let Some(goal) = maybe_goal
    {
        if n_success >= goal
        {
            format!("Réussite ({} sur {})", n_success, goal)
        }
        else if n_ones > n_success
        {
            format!("Échec critique!")
        }
        else
        {
            format!("Échec")
        }

    }
    else
    {
        if n_ones > n_success
        {
            format!("Possibilité d'échec critique")
        }
        else
        {
            format!("")
        }
        
    };
    
    format!("[{}]\n{}\n{}", throw_s, m_bilan, m_analyse)
        
        
}



fn trudvang(n: i64, expl_tresh: i64, bonus: i64) -> String
{
    let mut dices = Vec::new();
    dices.resize_with(
        n as usize,
        || {explode(10, |n| n >= expl_tresh, 0)}
    );

    if let Some(Err(err)) = dices.iter()
        .find(|maybe| maybe.is_err())
    {
        format!("{}", err)
    }
    else
    {
        let dices = dices.iter().map(|maybe| maybe.clone().unwrap()).collect::<Vec<_>>();
        let sum = dices.iter().flatten()
            .fold(0i64, |sum, die| sum + die) + bonus;
        let throw_m = dices.iter()
            .map(|v|
                 {
                     v.iter()
                         .fold(String::new(),
                               |s, n| format!("{} {}", s, n))
                 }
            )
            .fold(String::new(),
                  |out, s| format!("[{}] {}", s, out)
            );
        let total_m = format!("Total: {}", sum);
        format!("{}\n{}", throw_m, total_m)

    }
}



fn brigandine(score: i64, explode_tresh: i64) -> String
{
    let mut units = vec![];
    match explode(10, |n| n >= explode_tresh, 0)
    {
        Err(err) => {return err;},
        Ok(mut explosion) =>
        {
            units.append(
                &mut explosion
            );
        }
    }

    
    let unit_dice = units[0];
    let tenth_dice = dice(10);
    let d100 = d100_from_d10(tenth_dice, unit_dice);
    let inverse = d100_from_d10(unit_dice, tenth_dice);

    let is_success = d100 <= score;
    let degrees = (d100/10 - score/10).abs();
    
    println!("inverse: {}", inverse);
    let hit_location = match inverse
    {
        1..=9 => "Tête",
        10 => "Main gauche",
        11..=24 => "Bras gauche",
        25 => "Main droite",
        26..=44 => "Bras droit",
        45..=69 => "Torse",
        70..=80 => "Abdomen",
        81..=88 => "Jambe gauche",
        89 => "Pied gauche",
        90..=99 => "Jambe droite",
        100 => "Pied droit",
        err => {
            println!("Impossible location for br: {}", err);
            unreachable!()
        }
    };
    

    let damages: i64 = units.iter().sum();
    let result_m = if is_success
    {
        format!("Réussite de {} degrés", degrees)
    }
    else
    {
        format!("Échec de {} degrés", degrees)
    };
    let throw_m = units[1..].iter()
        .fold(format!("{}", units[0]), |s, n| format!("{}+{}", s, n));
    let explode_m = if units.len() == 1
    {String::new()}
    else
    {
        if dice(100) == 1
        {
            format!("\n(explosion: {}) Macron Explosion!", throw_m)
        }
        else
        {
            format!("\n(explosion: {})", throw_m)
        }
    };
    let location_m = format!("localisation: {}", hit_location);
    format!("test: {} pour {}\n{}\nDégâts: {} {}{}", d100, score, result_m, damages, location_m, explode_m)
        
        
}



fn vampire(score: i64, difficulty: i64, speciality: bool) -> String
{
    if score < 1 {return format!("https://cdn.discordapp.com/attachments/984109033143296030/1133208340042891346/mojus.jpg")}
    let mut throw = (0..score).map(|_| dice(10)).collect::<Vec<i64>>();
    let str_throw = throw.iter()
        .fold(String::new(), |s, n| format!("{} {}", s, n));
    throw.sort();
    let nb_ones = throw.iter().filter(|x| **x == 1).count();
    if *throw.last().unwrap() < difficulty && nb_ones > 0
    {
        return format!("{}\nÉchec Critique!", str_throw);
    }
    throw.drain(0..nb_ones); // the 1 annulates the success (the 10s last)
    let nb_success = throw.iter().filter(|x| **x >= difficulty).count();
    let nb_tens = throw.iter().filter(|x| **x == 10).count();
    let mut result = nb_success;
    if speciality
    {
        result += nb_tens;
    }

    return format!("{}\n{} Succès", str_throw, result);
    
}
