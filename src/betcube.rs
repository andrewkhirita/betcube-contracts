#![no_std]

pub mod storage;
pub mod constants;
pub mod events;
pub mod bet_manager;
pub mod errors;
pub mod nft_manager;
pub mod fund_manager;
pub mod market_manager;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();
#[multiversx_sc::contract]
pub trait BetCube:
storage::StorageModule
+ events::EventsModule
+ nft_manager::NftManagerModule
+ fund_manager::FundManagerModule
+ bet_manager::BetManagerModule
+ market_manager::MarketManagerModule{
    #[upgrade]
    fn upgrade(&self) {}

    #[init]
    fn init(&self) {
        self.market_counter().set(1);
    }
}