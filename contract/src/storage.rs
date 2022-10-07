elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const CID_LEN: usize = 46;

pub type PaymentsVec<M> = ManagedVec<M, EsdtTokenPayment<M>>;
pub type EgldValuePaymentsVecPair<M> = MultiValue2<BigUint<M>, PaymentsVec<M>>;
pub type Cid<M> = ManagedByteArray<M, CID_LEN>;

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct BrandInfo<M: ManagedTypeApi> {
    pub metadata_cid: Cid<M>,
    pub image_cid: Cid<M>,
    pub token_display_name: ManagedBuffer<M>,
    pub nft_name: ManagedBuffer<M>,
    pub media_type: ManagedBuffer<M>,
    pub royalties: BigUint<M>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct SaleInfo<M: ManagedTypeApi> {
    pub mint_price_in_egld: BigUint<M>,
    pub mint_price_in_usdc: BigUint<M>,
    pub initial_quantity: usize,
    pub max_nfts_per_wallet: usize,
    pub max_nfts_per_tx: usize,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct SaleStats<M: ManagedTypeApi> {
    pub egld_raised: BigUint<M>,
    pub usdc_raised: BigUint<M>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct OnChainAttributes<M: ManagedTypeApi> {
    pub attributes: Attributes<M>,
    pub metadata: ManagedBuffer<M>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct Attributes<M: ManagedTypeApi> {
    pub luck: ManagedBuffer<M>,
    pub life: ManagedBuffer<M>,
    pub mana: ManagedBuffer<M>,
    pub strength: ManagedBuffer<M>,
    pub dexterity: ManagedBuffer<M>,
    pub mind: ManagedBuffer<M>,
    pub focusing: ManagedBuffer<M>,
    pub energy: ManagedBuffer<M>,
}

#[elrond_wasm::module]
pub trait StorageModule {
    #[storage_mapper("paused")]
    fn paused(&self) -> SingleValueMapper<bool>;
    
    #[view(getIssuedTokenId)]
    #[storage_mapper("issuedTokenId")]
    fn issued_token_id(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[view(getBrandInfo)]
    #[storage_mapper("brandInfo")]
    fn brand_info(&self) -> SingleValueMapper<BrandInfo<Self::Api>>;

    #[view(getAttributesForNonce)]
    #[storage_mapper("attributesForNonce")]
    fn attributes_for_nonce(&self, nonce: &usize) -> SingleValueMapper<Attributes<Self::Api>>;

    //sale
    #[view(getSaleInfo)]
    #[storage_mapper("saleInfo")]
    fn sale_info(&self) -> SingleValueMapper<SaleInfo<Self::Api>>;

    #[view(getSaleStats)]
    #[storage_mapper("saleStats")]
    fn sale_stats(&self) -> SingleValueMapper<SaleStats<Self::Api>>;

    #[storage_mapper("availableIds")]
    fn available_ids(&self) -> UniqueIdMapper<Self::Api>;

    #[storage_mapper("nftsPerAddress")]
    fn nfts_per_address(&self, address: &ManagedAddress) -> SingleValueMapper<usize>;

    //whitelist
    #[view(getQuantityPerDrop)]
    #[storage_mapper("quantityPerDrop")]
    fn quantity_for_whitelist(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("whitelist")]
    fn whitelist(&self) -> WhitelistMapper<Self::Api, ManagedAddress>;

    #[storage_mapper("whitelistEnabled")]
    fn whitelist_enabled(&self) -> SingleValueMapper<bool>;
}
