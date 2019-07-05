transition.wasm: src/transition_wasm.rs
	rustc --target wasm32-unknown-unknown -O --crate-type=cdylib src/transition_wasm.rs -o transition.wasm
