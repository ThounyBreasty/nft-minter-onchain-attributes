source mainnet.snippets.sh

for startNonce in {1..2401..100}
do
    premintAllNfts
    echo startNonce $startNonce
    sleep 12
done