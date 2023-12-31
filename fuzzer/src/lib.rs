#![deny(missing_docs, warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

//! [FuzzProvider] is an [RpcProvider] impl that returns strucutrally valid random responses

mod fuzzdist;
mod messages;

use async_trait::async_trait;
use rand_chacha::ChaCha8Rng;
use zcashd_rpc_messages::GetInfo;
use zcashd_rpc_provider::RpcProvider;

/// An [RpcProvider] that returns structurally valid, yet random responses
pub struct FuzzProvider {
    #[allow(dead_code)]
    rng: ChaCha8Rng,
}

impl FuzzProvider {
    /// Construct a new client
    pub fn from_seed(seed: u64) -> Self {
        use rand::SeedableRng;

        FuzzProvider {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }
}

#[async_trait]
impl RpcProvider for FuzzProvider {
    type Error = ();

    async fn get_info(&mut self) -> Result<GetInfo, Self::Error> {
        use crate::fuzzdist::FuzzDistribution;
        use rand::Rng;

        Ok(self.rng.sample(FuzzDistribution))
    }
}
