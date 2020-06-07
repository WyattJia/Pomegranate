use rand::prelude::*;

pub trait LevelGenerator {
    fn total(&self) -> usize;

    fn random(&mut self) -> usize;
}

pub struct GeoLevelGenerator {
    total: usize,
    p: f64,
    rng: SmallRng,
}

impl GeoLevelGenerator {
    pub fn new(total: usize, p: f64) -> Self{
        if total == 0 {
            panic!("total can not be zero.");
        }
        if p <= 0.0 || p >= 1.0 {
            panic!("p value must in between in (0, 1)");
        }
        geo_level_gen {
            total,
            p,
            rng: SmallRng::from_rng(thread_rng()).unwrap(),
        }
    }
}

impl LevelGenerator for GeoLevelGenerator {
    fn random(&mut self) -> usize {
        let mut h = 0;
        let mut x = self.p;
        let f = 1.0 - self.rng.gen::<f64>();
        while x > f && h + 1 < self.total {
            h += 1;
            x *= self.p
        }
        h
    }

    fn total(&self) -> usize {
        self.total
    }
}