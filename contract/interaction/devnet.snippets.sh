WASM_PATH="../output/contract.wasm"
DEV="./wallet.pem"
ADDRESS=$(erdpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
PROXY="https://devnet-gateway.elrond.com"
CHAIN="D"
CLAIM_ADDRESS="erd1..."

ATTRIBUTES_STATS=`cat ./data/attributes.txt`

deploy() {
    erdpy --verbose contract deploy --recall-nonce --metadata-payable --metadata-payable-by-sc \
    --bytecode=${WASM_PATH} \
    --pem=${DEV} \
    --gas-limit=600000000 \
    --proxy=${PROXY} --chain=${CHAIN} \
    --arguments ${CLAIM_ADDRESS} ${CLAIM_ADDRESS} \
    --send \
    --outfile="deploy-devnet.interaction.json" || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --recall-nonce --metadata-payable --metadata-payable-by-sc \
        --bytecode=${WASM_PATH} \
        --pem=${DEV} \
        --gas-limit=600000000 \
        --proxy=${PROXY} --chain=${CHAIN} \
        --arguments ${CLAIM_ADDRESS} ${CLAIM_ADDRESS} \
        --outfile="deploy-devnet.interaction.json" \
        --send || return
}

issueToken() {
    issue_cost=50000000000000000
    metadata_cid="str:QmQ198Vu5zqBnDcLyNoHhHZ3UfHJpfG6EVTCGGBgTWprNQ"
    image_cid="str:Qmb5UZLAimajhdhZeLNaZmpWAt2qf6PCut3E4xL5dE4KBd"
    media_type="str:png"
    royalties=500
    max_nfts=2500
    token_display_name="str:Nfts"
    token_ticker="str:NFTS"
    nft_name="str:Nft"
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${DEV} \
    --proxy=${PROXY} --chain=${CHAIN} \
    --gas-limit=500000000 \
    --function="issueToken" \
    --arguments $metadata_cid $image_cid $media_type $royalties $max_nfts $token_display_name $token_ticker $nft_name \
    --value=50000000000000000 \
    --send || return
}

setAttributes() {
    erdpy --verbose tx new --receiver=${ADDRESS} --recall-nonce \
        --pem=${DEV} \
        --chain=${CHAIN} --proxy=${PROXY} \
        --gas-limit=600000000 \
        --value=0 \
        --data="${ATTRIBUTES}" \
        --send || return 
}

premintAllNfts() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${DEV} \
    --proxy=${PROXY} --chain=${CHAIN} \
    --gas-limit=600000000 \
    --function="premintAllNfts" \
    --arguments $startNonce \
    --send || return
}

addToAdminList() {
    user="erd1j80nuxqu5lpt2tfh47nzg7r99jnn9zq9sdh3hympnzf04x58s0zsyhr8re"
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${DEV} \
    --proxy=${PROXY} --chain=${CHAIN} \
    --gas-limit=600000000 \
    --function="addUserToAdminList" \
    --arguments $user \
    --send || return
}

removeFromAdminList() {
    user="erd1j80nuxqu5lpt2tfh47nzg7r99jnn9zq9sdh3hympnzf04x58s0zsyhr8re"
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${DEV} \
    --proxy=${PROXY} --chain=${CHAIN} \
    --gas-limit=600000000 \
    --function="removeUserFromAdminList" \
    --arguments $user \
    --send || return
}

buy() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${DEV} \
    --proxy=${PROXY} --chain=${CHAIN} \
    --gas-limit=50000000 \
    --function="buyRandomNft" \
    --value=10000000000000000 \
    --send || return
}

giveaway() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${DEV} \
    --proxy=${PROXY} --chain=${CHAIN} \
    --gas-limit=50000000 \
    --function="giveawayNfts" \
    --arguments "erd1..." \
    --send || return
}

setWlAddr() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${DEV} \
    --proxy=${PROXY} --chain=${CHAIN} \
    --gas-limit=50000000 \
    --function="setWhitelistedAddresses" \
    --arguments "erd1..." \
    --send || return
}

pause() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${DEV} \
    --proxy=${PROXY} --chain=${CHAIN} \
    --gas-limit=50000000 \
    --function="pauseMint" \
    --send || return
}

resume() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${DEV} \
    --proxy=${PROXY} --chain=${CHAIN} \
    --gas-limit=50000000 \
    --function="resumeMint" \
    --send || return
}

claimRoyalties() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${DEV} \
    --proxy=${PROXY} --chain=${CHAIN} \
    --gas-limit=50000000 \
    --function="claimRoyalties" \
    --send || return
}

claimMintPayments() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${DEV} \
    --proxy=${PROXY} --chain=${CHAIN} \
    --gas-limit=50000000 \
    --function="claimMintPayments" \
    --send || return
}

getAttributes() {
    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function="getAttributesForNonce" \
        --arguments $1
}

getIssuedTokenId() {
    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function="getIssuedTokenId" \
}
