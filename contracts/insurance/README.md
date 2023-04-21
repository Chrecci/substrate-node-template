
## Run Blockchain (from root)
```
cargo run --release -- --dev
```

## Enter Contract Repo
```
cd contracts/insurance
```

## Build and Run Tests
```
cargo contract build
cargo test
```

## Access Contract
After running the node instance, your node should be detectable here https://contracts-ui.substrate.io/ 

Deploy and Instantiate the insurance contract file under /insurance/target/ink/insurance.contract which is produced upon contract build

Enjoy!

