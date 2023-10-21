//! Implementations of [rand::Distribution] for [messages](crate::messages) to support [FuzzProvider](crate::FuzzProvider)

use crate::messages;
use crate::randutil::FuzzDistribution;
use rand::distributions::Distribution;
use rand::Rng;

impl Distribution<messages::GetInfo> for FuzzDistribution {
    fn sample<R>(&self, rng: &mut R) -> messages::GetInfo
    where
        R: Rng + ?Sized,
    {
        messages::GetInfo {
            version: self.sample(rng),
            build: self.sample(rng),
            subversion: self.sample(rng),
            protocolversion: self.sample(rng),
            wallet_info: self.sample(rng),
            blocks: self.sample(rng),
            timeoffset: self.sample(rng),
            connections: self.sample(rng),
            proxy: self.sample(rng),
            testnet: self.sample(rng),
            relayfee: self.sample(rng),
            errors: self.sample(rng),
            errorstimestamp: self.sample(rng),
        }
    }
}

impl Distribution<messages::WalletInfo> for FuzzDistribution {
    fn sample<R>(&self, rng: &mut R) -> messages::WalletInfo
    where
        R: Rng + ?Sized,
    {
        messages::WalletInfo {
            walletversion: self.sample(rng),
            balance: self.sample(rng),
            paytxfee: self.sample(rng),
            active_info: self.sample(rng),
        }
    }
}

impl Distribution<messages::ActiveWalletInfo> for FuzzDistribution {
    fn sample<R>(&self, rng: &mut R) -> messages::ActiveWalletInfo
    where
        R: Rng + ?Sized,
    {
        messages::ActiveWalletInfo {
            keypoololdest: self.sample(rng),
            keypoolsize: self.sample(rng),
        }
    }
}
