from lib2to3.pgen2.pgen import NFAState
import requests

# get metadata.json file with information on all the collection
# or read it from .txt file

response = requests.get(
    "https://bafybeif76kpiab73bmooj4eduoqmmsp6trifenuf3zq6be3mcw6rcjoesu.ipfs.dweb.link/")
metadata = response.json()


def getNonce(nft):
    nonce = hex(nft["edition"])
    nonce = nonce[2:]
    if len(nonce) % 2 == 1:
        nonce = f"0{nonce}"
    return nonce


nonce = list(map(getNonce, metadata))

def valueAttributes(attributes):
    value = attributes["value"]
    encoded = value.encode('utf-8').hex()
    return encoded


def filterAttributes(attributes):
    x = attributes["trait_type"]
    if x == "Luck" or x == "Life" or x == "Mana" or x == "Strength" or x == "Dexterity" or x == "Mind" or x == "Focusing" or x == "Energy":
        return attributes


def getAttributes(metadata):
    attr = metadata["attributes"]
    filtered = list(filter(filterAttributes, attr))
    mapped = list(map(valueAttributes, filtered))
    return mapped


attributes = list(map(getAttributes, metadata))


def concatenate(metadata):
    string = "@".join(
        map(str, ([nonce[metadata["edition"]-1]] + attributes[metadata["edition"]-1])))
    return string


onChainAttributes = "@".join(map(str, (list(map(concatenate, metadata)))))

with open('./inputData/attributes.txt', 'w') as fd:
    fd.write(f"@{onChainAttributes}")
