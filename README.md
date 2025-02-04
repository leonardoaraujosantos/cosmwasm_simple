# Introduction

Simple contract
1. Query for Hello World <parameter>
2. Message to Increment Counter
3. Query to get Counter Value
4. Message to Execte custom Module

## Build the Smart Contract
```bash
cargo build --release --target wasm32-unknown-unknown
cosmwasm-check ./target/wasm32-unknown-unknown/release/contract.wasm    
```

## Push to Blockchain
```bash
aminichaind tx wasm store ./contract.wasm  --from alice --gas auto --gas-adjustment 1.5 --chain-id aminichain -y
aminichaind query wasm list-code
# Imagine that the last returned is 5
aminichaind tx wasm instantiate 5 '{}' --label helloParCont --from alice --gas auto --gas-adjustment 1.5 --chain-id aminichain --no-admin -y
aminichaind query wasm list-contract-by-code 5
# Imagine: cosmos1eyfccmjm6732k7wp4p6gdjwhxjwsvje44j0hfx8nkgrm8fs7vqfsxcj9n3
```

## Execute few commands
```bash
aminichaind query wasm contract-state smart cosmos1eyfccmjm6732k7wp4p6gdjwhxjwsvje44j0hfx8nkgrm8fs7vqfsxcj9n3 '{"get_message": {"name": "leo!"}}' --chain-id aminichain

# data:
# message: Hello World leo!

aminichaind tx wasm execute cosmos1eyfccmjm6732k7wp4p6gdjwhxjwsvje44j0hfx8nkgrm8fs7vqfsxcj9n3 '{"increment": {}}' --from alice -y

aminichaind query wasm contract-state smart cosmos1eyfccmjm6732k7wp4p6gdjwhxjwsvje44j0hfx8nkgrm8fs7vqfsxcj9n3 '{"get_count": {}}' --chain-id aminichain
# data:
# count: 1

aminichaind tx wasm execute cosmos1eyfccmjm6732k7wp4p6gdjwhxjwsvje44j0hfx8nkgrm8fs7vqfsxcj9n3 '{"push_oracle_result": {"job_id": 2, "results_json": "Hello"}}' --from alice -y
```

## Execute Custom Message Call before permission
It should fail with an error:
```bash
raw_log: 'failed to execute message; message index: 0: dispatch: submessages: failed
  to get grant with given granter: cosmos1xfa7689z3ew03s7fw6qer3xwdw3kgzlzrgcwdf,
  grantee: cosmos1eyfccmjm6732k7wp4p6gdjwhxjwsvje44j0hfx8nkgrm8fs7vqfsxcj9n3 & msgType:
  /aminichain.apigateway.MsgOraclePushResult : authorization not found'
```
If we grant our smart contract to execute as "alice"
```bash
aminichaind tx authz grant cosmos1eyfccmjm6732k7wp4p6gdjwhxjwsvje44j0hfx8nkgrm8fs7vqfsxcj9n3 generic \
    --msg-type="/aminichain.apigateway.MsgOraclePushResult" \
    --from alice -y

# Execute again and will work
aminichaind tx wasm execute cosmos1eyfccmjm6732k7wp4p6gdjwhxjwsvje44j0hfx8nkgrm8fs7vqfsxcj9n3 '{"push_oracle_result": {"job_id": 2, "results_json": "Hello"}}' --from alice -y
```
### Result
```
raw_log: 'failed to execute message; message index: 0: dispatch: submessages: failed
  to execute message; message creator:"cosmos1xfa7689z3ew03s7fw6qer3xwdw3kgzlzrgcwdf"
  jobId:2 resultsJson:"Hello" : JobId 2 alredy computed: key not found'
timestamp: "2025-02-04T17:07:06Z"
```
