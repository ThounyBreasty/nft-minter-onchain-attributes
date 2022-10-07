elrond_wasm::imports!();

use crate::{
    storage::{Cid, OnChainAttributes},
};

static SUPPORTED_MEDIA_TYPES: &[&[u8]] = &[
    b"png",
    b"jpeg",
    b"jpg",
    b"gif",
    b"acc",
    b"flac",
    b"m4a",
    b"mp3",
    b"wav",
    b"mov",
    b"quicktime",
    b"mp4",
    b"webm",
];
const MAX_MEDIA_TYPE_LEN: usize = 9;

#[elrond_wasm::module]
pub trait AttributesModule: crate::storage::StorageModule + crate::admin::AdminModule {

    fn build_nft_attributes(
        &self,
        metadata_cid: &Cid<Self::Api>,
        nonce: UniqueId,
    ) -> OnChainAttributes<Self::Api> {
        let attributes = self.attributes_for_nonce(&nonce).get();
        let metadata = self.build_metadata(metadata_cid, nonce);

        let on_chain_attributes = OnChainAttributes {
            attributes,
            metadata,
        };

        on_chain_attributes
    }

    fn build_metadata(
        &self,
        metadata_cid: &Cid<Self::Api>,
        nonce: UniqueId,
    ) -> ManagedBuffer<Self::Api> {
        sc_format!(
            "metadata:{}/{}.json",
            metadata_cid.as_managed_buffer(),
            nonce
        )
    }

    //URIs

    fn build_media_uri(
        &self,
        image_cid: &Cid<Self::Api>,
        nonce: UniqueId,
        media_type: &ManagedBuffer<Self::Api>,
    ) -> ManagedBuffer<Self::Api> {
        sc_format!(
            "https://ipfs.io/ipfs/{}/{}.{}",
            image_cid.as_managed_buffer(),
            nonce,
            media_type
        )
    }

    fn build_json_uri(
        &self,
        metadata_cid: &Cid<Self::Api>,
        nonce: UniqueId,
    ) -> ManagedBuffer<Self::Api> {
        sc_format!(
            "https://ipfs.io/ipfs/{}/{}.json",
            metadata_cid.as_managed_buffer(),
            nonce,
        )
    }

    fn is_supported_media_type(&self, media_type: &ManagedBuffer<Self::Api>) -> bool {
        let media_type_len = media_type.len();
        if media_type_len > MAX_MEDIA_TYPE_LEN {
            return false;
        }

        let mut media_static_buffer = [0u8; MAX_MEDIA_TYPE_LEN];
        let slice = &mut media_static_buffer[..media_type_len];
        let _ = media_type.load_slice(0, slice);

        // clippy is wrong, using `slice` directly causes an error
        #[allow(clippy::redundant_slicing)]
        SUPPORTED_MEDIA_TYPES.contains(&&slice[..])
    }
}
