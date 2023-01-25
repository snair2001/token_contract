# token_contract
This set of files implement the NEAR smart contracts required for the basic functioning of an NFT. 
This repository has to be cloned in to a workspace and the below command has to be run
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
to generate the ./target/wasm32-unknown-unknown/release/token.wasm file. 
This wasm file has to be used in the command below to deploy the smart contract. 
near deploy --accountId xyz.near --wasmFile ./target/wasm32-unknown-unknown/release/token.wasm
Subsequently the contract has to be initialized with the command below.
near call abc.near new_default_meta '{"owner_id": "def.near"}' --accountId ghi.near
