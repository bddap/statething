This is an unstructured brainstorm.

## Implementation plan V0 in order of decreasing priority. Items are to be executed in order.

Determine a wasm runtime, reorg following items according to limitations in runtime. Ideally runtime can accept and return complex data types do ser/de is not needed.

Build a hello world that runs naitively, or in a WASM interpreter. Both naitive and wasm should be built from the same source.

Define super simple state and state transition e.g. u32 and increment. Derive serde.

If runtime required ser/de, change hello world to take bytes, deserialize, do state transition, serialize result, return serialized bytes.

Add version number to determine whether wasm and naitive are discrepant. If a wasm module can export a constant, export version number thusly.

Write function, `fn transition(wasm_module: mut Mod, st: &[u8]) -> Vec<u8>` unless runtime can handle typed data.

### Stretch

The state will be maping from public key to SocketAddr.

The only state transition will be adding to the map (insertion).

Accept insertions only when (data, id) is crypto-signed where id is an incrementing transition id (analagous to block hieght)
