#![no_std]

elrond_wasm::imports!();

use crate::storage::{SaleInfo, Attributes};

pub mod admin;
pub mod storage;
pub mod attributes;
pub mod mint;
pub mod royalties;
pub mod sale;

#[elrond_wasm::contract]
pub trait Minter:
    storage::StorageModule
    + admin::AdminModule
    + mint::MintModule
    + attributes::AttributesModule
    + royalties::RoyaltiesModule
    + sale::SaleModule
{
    #[init]
    fn init(
        &self,
        royalties_claim_address: ManagedAddress,
        mint_payments_claim_address: ManagedAddress,
    ) {
        self.royalties_claim_address().set(&royalties_claim_address);
        self.mint_payments_claim_address().set(&mint_payments_claim_address);
    }

    #[endpoint(pauseSale)]
    fn pause_sale(&self) {
        self.require_caller_is_admin();
        self.paused().set(true);
    }

    #[endpoint(resumeSale)]
    fn resume_sale(&self) {
        self.require_caller_is_admin();
        self.paused().set(false);
    }

    #[endpoint(enableWhitelist)]
    fn enable_whitelist(&self) {
        self.require_caller_is_admin();
        self.whitelist_enabled().set(true);
    }

    #[endpoint(disableWhitelist)]
    fn disable_whitelist(&self) {
        self.require_caller_is_admin();
        self.whitelist_enabled().set(false);
    }

    #[endpoint(setEgldPrice)]
    fn set_egld_price(&self, price: u64) {
        self.require_caller_is_admin();

        let info = self.sale_info().get();

        self.sale_info().set(SaleInfo {
            mint_price_in_egld: BigUint::from(price),
            mint_price_in_usdc: info.mint_price_in_usdc,
            initial_quantity: info.initial_quantity,
            max_nfts_per_wallet: info.max_nfts_per_wallet,
            max_nfts_per_tx: info.max_nfts_per_tx,
        });
    }

    #[endpoint(setSaleInfo)]
    fn set_sale_info(&self, 
        mint_price_in_egld: u64, 
        mint_price_in_usdc: u64,
        max_nfts_per_wallet: usize,
        max_nfts_per_tx: usize,
        initial_quantity: usize,
    ) {
        self.require_caller_is_admin();

        self.sale_info().set(SaleInfo {
            mint_price_in_egld: BigUint::from(mint_price_in_egld),
            mint_price_in_usdc: BigUint::from(mint_price_in_usdc),
            initial_quantity,
            max_nfts_per_wallet,
            max_nfts_per_tx,
        });
    }

    #[endpoint(setWhitelistedAddresses)]
    fn set_whitelisted_addresses(
        &self,
        addresses: MultiValueEncoded<Self::Api, ManagedAddress<Self::Api>>,
    ) {
        self.require_caller_is_admin();

        self.whitelist_enabled().set(true);
        for address in addresses {
            self.whitelist().add(&address);
        }
    }

    #[endpoint(setAttributesStats)]
    fn set_attributes_stats(
        &self,
        formatted_attributes: MultiValueEncoded<
            MultiValue9<
                usize,
                ManagedBuffer,
                ManagedBuffer,
                ManagedBuffer,
                ManagedBuffer,
                ManagedBuffer,
                ManagedBuffer,
                ManagedBuffer,
                ManagedBuffer,
            >,
        >,
    ) {
        self.require_caller_is_admin();

        for attributes in formatted_attributes {
            let (nonce, luck, life, mana, strength, dexterity, mind, focusing, energy) =
                attributes.into_tuple();

            self.attributes_for_nonce(&nonce).set(Attributes {
                luck,
                life,
                mana,
                strength,
                dexterity,
                mind,
                focusing,
                energy,
            });
        }
    }
}
