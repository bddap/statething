use crate::statething::*;

/// Loads a runtime either native or wasm, based on specified rule.
/// Specified rule:
///   If runtime versions match, use wasm, otherwise use native.
pub fn select_runtime() -> Result<Box<dyn Transition>, FailedToLoadWasm> {
    let wasm = WasmTransition::new().map_err(|_| FailedToLoadWasm)?;
    let native = NaitiveTransition;
    Ok(if wasm.version() == native.version() {
        Box::new(wasm)
    } else {
        Box::new(native)
    })
}

#[derive(Debug)]
pub struct FailedToLoadWasm;
