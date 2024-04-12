Install Solana
```
sh -c "$(curl -sSfL https://release.solana.com/v1.18.9/install)"
```

Check installed Solana verion
```
solana --version
```

Run the following commands
```
solana config set --url localhost
solana config get
```

Run Solana test validator node
```
solana-test-validator
```

Install Mocha
```
npm install -g mocha
```

Install the magic of Anchor
```
cargo install --git https://github.com/project-serum/anchor anchor-cli --locked
```

Check anchor version
```
anchor --version
```

Install anchor and web3 dependencies in the project
```
npm install @project-serum/anchor @solana/web3.js
```

Create a test project and run it.
```
brew install yarn
anchor init myepicproject --javascript
cd myepicproject
```

Create a local keypair.
```
solana-keygen new 
solana address 
```

Let's run our program
```
anchor test
```

# Setup a Token

Install SPL Token
```
cargo install spl-token
```

In case if the following error comes
```
error: failed to compile `spl-token-cli v3.4.0`, intermediate artifacts can be found at `/tmp/cargo-installmP2EaK`.
```

Set cargo target directory CARGO_TARGET_DIR to /tmp/cargo-installmP2EaK
```
export CARGO_TARGET_DIR="/tmp/cargo-installmP2EaK"
export PATH="$CARGO_TARGET_DIRECTORY:$PATH"
```

Generate a new key and use --force to override it if already created
```
solana-keygen new --force
```

To display the key use
```
solana-keygen pubkey
or
solana address
```

Check wallet balance
```
solana balance --url devnet
```

Open <a href="https://www.explorer.solana.com">Solana Explorer</a>. Switch to devnet. Paste your wallet address. You can see the details.

Airdrop some tokens.
```
solana airdrop NUMBER_OF_TOKENS PUB_KEY --url devnet
solana airdrop 2 HZbBo78cLeHAE3ANB95FzyMcyWtYE3XF7u5X7HYg1jTQ --url devnet
```

Check wallet balance
```
solana balance --url devnet
```

Check on Solana explorer.

Create a token
```
 spl-token create-token --url devnet     
```

Example: 
PUB_KEY: HZbBo78cLeHAE3ANB95FzyMcyWtYE3XF7u5X7HYg1jTQ
TOKEN_ADDRESS: 62oj2nrxoeWotz2s428fy1KKJraXqPf2NQRoMHhTwZxH

Create an account against a wallet. 
```
spl-token create-account TOKEN_ADDRESS --url devnet
spl-token create-account 62oj2nrxoeWotz2s428fy1KKJraXqPf2NQRoMHhTwZxH --url devnet
```

Example:
TOKEN_ACCOUNT_ADDRESS: 56EVi9npqTtSsjMRNtSbvnRCk93K6cgYL9LFjv9s4ZcJ

Check token account balance
```
spl-token balance TOKEN_ADDRESS --url devnet
spl-token balance 62oj2nrxoeWotz2s428fy1KKJraXqPf2NQRoMHhTwZxH --url devnet
```

Mint token
```
spl-token mint TOKEN_ADDRESS NUMBER_OF_TOKENS_TO_MINT --url devnet
spl-token mint 62oj2nrxoeWotz2s428fy1KKJraXqPf2NQRoMHhTwZxH 1000 --url devnet
```

Check that tokens have been minted
```
spl-token balance TOKEN_ADDRESS --url devnet
spl-token balance 62oj2nrxoeWotz2s428fy1KKJraXqPf2NQRoMHhTwZxH --url devnet
```

Check token supply
```
spl-token supply TOKEN_ADDRESS --url devnet 
1000
spl-token supply 62oj2nrxoeWotz2s428fy1KKJraXqPf2NQRoMHhTwZxH --url devnet 
1000
```

Disable token minting
```
spl-token authorize TOKEN_ADDRESS mint --disable --url devnet
```

Burn tokens
```
spl-token burn TOKEN_ADDRESS NUMBER_OF_TOKENS_TO_BURN --url devnet
```

---------------------------------------------------------------------------
# Test a Custom Solana Program

```
anchor init my-dapp
cd my-dapp
anchor build
```









