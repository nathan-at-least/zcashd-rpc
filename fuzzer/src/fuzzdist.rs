use rand::distributions::{Distribution, Standard};
use rand::Rng;

pub(crate) struct FuzzDistribution;

pub(crate) fn weighted_coin_flip<R>(rng: &mut R, heads_chance: f32) -> bool
where
    R: Rng + ?Sized,
{
    rng.gen_range(0f32..1f32) >= heads_chance
}

impl<T> Distribution<Option<T>> for FuzzDistribution
where
    FuzzDistribution: Distribution<T>,
{
    fn sample<R>(&self, rng: &mut R) -> Option<T>
    where
        R: Rng + ?Sized,
    {
        if weighted_coin_flip(rng, 0.9) {
            Some(self.sample(rng))
        } else {
            None
        }
    }
}

impl Distribution<bool> for FuzzDistribution {
    fn sample<R>(&self, rng: &mut R) -> bool
    where
        R: Rng + ?Sized,
    {
        // Use uneven weight just for the fuzzy hell of it
        weighted_coin_flip(rng, 0.57)
    }
}

impl Distribution<u64> for FuzzDistribution {
    fn sample<R>(&self, rng: &mut R) -> u64
    where
        R: Rng + ?Sized,
    {
        rng.gen_range(0..u64::MAX)
    }
}

impl Distribution<i64> for FuzzDistribution {
    fn sample<R>(&self, rng: &mut R) -> i64
    where
        R: Rng + ?Sized,
    {
        rng.gen_range(i64::MIN..i64::MAX)
    }
}

impl Distribution<f64> for FuzzDistribution {
    fn sample<R>(&self, rng: &mut R) -> f64
    where
        R: Rng + ?Sized,
    {
        rng.gen_range(f64::MIN..f64::MAX)
    }
}

impl Distribution<String> for FuzzDistribution {
    fn sample<R>(&self, rng: &mut R) -> String
    where
        R: Rng + ?Sized,
    {
        let mut s = String::new();
        // Expected length: 999
        while weighted_coin_flip(rng, 0.999) {
            s.push(Standard.sample(rng));
        }
        s
    }
}
