elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{storage::{BrandInfo, Cid}};

const NFT_ISSUE_COST: u64 = 50_000_000_000_000_000; // 0.05 EGLD
const ROYALTIES_MAX: u32 = 10_000; // 100%

#[derive(TopEncode, TopDecode)]
pub struct TempCallbackStorageInfo<M: ManagedTypeApi> {
    pub brand_info: BrandInfo<M>,
    pub total_nfts: usize,
}

#[elrond_wasm::module]
pub trait MintModule:
    crate::storage::StorageModule
    + crate::admin::AdminModule
    + crate::attributes::AttributesModule
    + crate::royalties::RoyaltiesModule
{
    //admin only

    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(
        &self,
        metadata_cid: Cid<Self::Api>,
        image_cid: Cid<Self::Api>,
        media_type: ManagedBuffer,
        royalties: BigUint,
        total_nfts: usize,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        nft_name: ManagedBuffer,
    ) {
        //verifications
        self.require_caller_is_admin();

        require!(self.issued_token_id().is_empty(), "Token already issued");
        let issue_cost = self.call_value().egld_value();
        require!(
            issue_cost == NFT_ISSUE_COST,
            "Invalid payment amount. Issue costs exactly 0.05 EGLD"
        );
        require!(
            self.is_supported_media_type(&media_type),
            "Invalid media type"
        );
        require!(royalties <= ROYALTIES_MAX, "Royalties cannot be over 100%");
        require!(total_nfts > 0, "Cannot create brand with max 0 items");

        let brand_info = BrandInfo {
            metadata_cid: metadata_cid.clone(),
            image_cid: image_cid.clone(),
            token_display_name: token_display_name.clone(),
            nft_name: nft_name.clone(),
            media_type,
            royalties,
        };

        self.temporary_callback_storage()
            .set(&TempCallbackStorageInfo {
                brand_info,
                total_nfts,
            });

        self.issued_token_id().issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            issue_cost,
            token_display_name,
            token_ticker,
            0,
            Some(self.callbacks().issue_callback()),
        );
    }

    #[callback]
    fn issue_callback(&self, #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                let cb_info: TempCallbackStorageInfo<Self::Api> =
                    self.temporary_callback_storage().get();

                self.issued_token_id().set_token_id(&token_id);
                self.brand_info().set(&cb_info.brand_info);
                self.available_ids().set_initial_len(cb_info.total_nfts);
            }
            ManagedAsyncCallResult::Err(_) => {
                let caller = self.blockchain().get_owner_address();
                let (token_id, nonce, amount) = self.call_value().egld_or_single_esdt().into_tuple();
                if amount > BigUint::zero() {
                    self.send().direct(&caller, &token_id, nonce, &amount);
                }
            }
        }
    }

    #[endpoint(premintAllNfts)]
    fn premint_all_nfts(&self, nonces: usize) {
        self.require_caller_is_admin();

        let nft_token_id = self.issued_token_id().get_token_id();
        // let total_nfts_amount = self.get_total_nfts_left();
        let brand_info = self.brand_info().get();

        for nonce in nonces..nonces + 100 {
            let nft_amount = BigUint::from(1 as usize);
            let nft_name = sc_format!("{} #{}", brand_info.nft_name, nonce);
            let attributes = self.build_nft_attributes(&brand_info.metadata_cid, nonce);

            let nft_uri =
                self.build_media_uri(&brand_info.image_cid, nonce, &brand_info.media_type);
            let nft_json = self.build_json_uri(&brand_info.metadata_cid, nonce);
            let mut uris: ManagedVec<ManagedBuffer> = ManagedVec::new();
            uris.push(nft_uri);
            uris.push(nft_json);

            self.send().esdt_nft_create(
                &nft_token_id,
                &nft_amount,
                &nft_name,
                &brand_info.royalties,
                &ManagedBuffer::new(),
                &attributes,
                &uris,
            );
        }
    }

    //storage

    #[storage_mapper("temporaryCallbackStorage")]
    fn temporary_callback_storage(&self) -> SingleValueMapper<TempCallbackStorageInfo<Self::Api>>;
}
