pub fn dice(n_faces: i64) -> i64
{
    (rand::random::<u64>() % (n_faces as u64) + 1) as i64
}


