This is an slighly more structured brainstorm.

## Implementation plan V1 in order of decreasing priority. Items are to be executed in order.

Use wasmer as it looks mature relative to other runtimes.

Define super simple state and state transition e.g. u32 and increment. that runs naitively, or in a WASM interpreter. Both naitive and wasm should be built from the same source. Derive serde.

If runtime required ser/de, change hello world to take bytes, deserialize, do state transition, serialize result, return serialized bytes.

Add version number to determine whether wasm and naitive are discrepant. If a wasm module can export a constant, export version number thusly.

Write function, `fn transition(wasm_module: mut Mod, st: &[u8]) -> Vec<u8>` unless runtime can handle typed data.
