Install Solana

```
sh -c "$(curl -sSfL https://release.solana.com/v1.18.9/install)"
solana --version

solana config set --url localhost
solana config get

solana-test-validator
```

Install Node, NPM, and Mocha
```
npm install -g mocha
```

The magic of Anchor
```
cargo install --git https://github.com/project-serum/anchor anchor-cli --locked
anchor --version
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