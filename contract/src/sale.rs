elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::storage::SaleStats;

const USDC_ID: &[u8] = b"USDC-c76f1f";

#[elrond_wasm::module]
pub trait SaleModule: crate::storage::StorageModule + crate::admin::AdminModule {
    #[endpoint(giveawayNfts)]
    fn giveaway_nfts(&self, addresses: MultiValueEncoded<ManagedAddress<Self::Api>>) {
        self.require_caller_is_admin();

        for address in addresses.into_iter() {
            self.send_random_nft(&address, 1);
        }
    }

    // can buy in EGLD or USDC
    #[payable("*")]
    #[endpoint(buyRandomNft)]
    fn buy_random_nft(&self, opt_nb_nfts: OptionalValue<usize>) {
        require!(self.paused().get() == false, "Mint closed for the moment");

        let caller = self.blockchain().get_caller();
        let info = self.sale_info().get();

        let nb_nfts = match opt_nb_nfts {
            OptionalValue::Some(number) => {
                require!(number != 0, "Wrong data, cannot buy 0 NFT");

                let nfts_for_caller = self.nfts_per_address(&caller).get();
                require!(
                    number + nfts_for_caller <= info.max_nfts_per_wallet,
                    "Max NFTs per wallet limit reached"
                );

                number
            }
            OptionalValue::None => 1usize,
        };

        //verify if nfts left
        if self.whitelist_enabled().get() == true {
            self.whitelist().require_whitelisted(&caller);
            require!(
                nb_nfts + (info.initial_quantity - self.available_ids().len())
                    <= self.quantity_for_whitelist().get(),
                "no more nfts left for whitelist drop"
            );
        }
        require!(
            nb_nfts <= self.available_ids().len(),
            "No more NFTs available"
        );

        //verify payment
        let mut egld_amount = BigUint::zero();
        let mut usdc_amount = BigUint::zero();
        let (payment_token_id, payment_amount) = self.call_value().egld_or_single_fungible_esdt();
        match payment_token_id.into_esdt_option() {
            Option::None => {
                let required_amount = &info.mint_price_in_egld * (nb_nfts as u32);
                require!(payment_amount == required_amount, "Wrong amount");
                egld_amount = payment_amount;
            }
            Option::Some(esdt_id) => {
                require!(
                    esdt_id == TokenIdentifier::from(USDC_ID),
                    "Wrong payment token, egld or usdc required"
                );
                let required_amount = &info.mint_price_in_usdc * (nb_nfts as u32);
                require!(payment_amount == required_amount, "Wrong amount");
                usdc_amount = payment_amount;
            }
        }

        //increment data
        let stats = self.sale_stats().get();
        self.sale_stats().set(SaleStats {
            egld_raised: stats.egld_raised + egld_amount,
            usdc_raised: stats.usdc_raised + usdc_amount,
        });

        self.send_random_nft(&caller, nb_nfts);
    }


    fn send_random_nft(&self, to: &ManagedAddress<Self::Api>, nfts_to_send: usize) {
        let collection_id = self.issued_token_id().get_token_id();
        let mut nft_output_payments = ManagedVec::new();
        for _ in 0..nfts_to_send {
            let nft_amount = BigUint::from(1 as usize);
            let nft_nonce = self.get_next_random_id() as u64;

            nft_output_payments.push(EsdtTokenPayment::new(
                collection_id.clone(),
                nft_nonce,
                nft_amount,
            ));
        }

        self.send().direct_multi(to, &nft_output_payments);
    }

    fn get_next_random_id(&self) -> UniqueId {
        let mut ids = self.available_ids();
        require!(ids.len() > 0, "No more NFTs available");

        let mut rand_source = RandomnessSource::<Self::Api>::new();
        let rand_index = rand_source.next_usize_in_range(1, ids.len() + 1);
        ids.swap_remove(rand_index)
    }

    #[view(getTotalNftsLeft)]
    fn get_total_nfts_left(&self) -> usize {
        self.available_ids().len()
    }
}
