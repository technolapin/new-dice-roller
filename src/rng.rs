pub struct RNG
{
}

impl RNG
{
    pub fn range(&self, start: usize, end: usize) -> usize
    {
        rand::random::<usize>() % (end - start) + start
    }
}

