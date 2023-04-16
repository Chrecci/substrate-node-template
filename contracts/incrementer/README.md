
## Run Test Chain
```
substrate-contracts-node --log info,runtime::contracts=debug 2>&1
```

## Enter Contract Repo
```
cd contracts/incrementer
```

## Build and Run Tests
```
cargo contract build
cargo test
```

## Deploy Contract Onto chain
```
cargo contract instantiate --constructor default --suri //Alice --salt $(date +%s)
```

This will return a Contract address, which can then be used to make calls to

## Make Calls 
```
// (Increment Alice by 42)
cargo contract call --contract <CONTRACT_ADDRESS> --message inc --args 42 --suri //Alice

// (Get current value for Alice)
cargo contract call --contract 5HAqjLd633nss8pfSJNqhyWaTbxvYwxDmdaAiMnhDCb9YE8p --message get --suri //Alice --dry-run
```



