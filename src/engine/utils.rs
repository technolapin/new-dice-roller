pub fn dice(n_faces: i64) -> i64
{
    (rand::random::<u64>() % (n_faces as u64) + 1) as i64
}
pub fn d100_from_d10(tenth_dice: i64, unit_dice: i64) -> i64
{
    match (tenth_dice % 10)*10 + (unit_dice % 10)
    {
        0 => 100,
        other => other
    }
}

pub fn explode<F>(n_faces: i64,
              condition: F,
              depth: usize) -> Result<Vec<i64>, String>
where
    F: Fn(i64)-> bool
{
    if depth > 1000
    {
        return Err(format!("nan mais t'abuse là avec ton dé explosif"))
    }
    let r = dice(n_faces);
    let mut v = vec![r];
    if !condition(r)
    {
        Ok(v)
    }
    else
    {
        v.append(&mut explode(n_faces, condition, depth+1)?);
        Ok(v)
    }
}

