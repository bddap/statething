use wasmer_runtime::{imports, instantiate, Func};

/// The choosen state machine. Implementors are wasm and naitive.
pub trait Transition {
    /// Central state transition function. Using u32 as state for now.
    fn transition(&self, _: u32) -> Result<u32, String>;

    /// Used for checking whether versions are discrepant.
    fn version(&self) -> u32;
}

/// A state transition using a wasm runtime
pub struct WasmTransition {
    // instance is reused for every call to transition, if transition is not a pure function, calls
    // will be nonderterministinc. This wouldn't be suitable for untrusted wasm, like in polkadot
    // but I'll let is slide here for now.
    module: wasmer_runtime::Instance,
    version: u32,
}

impl WasmTransition {
    /// Attempt to load wasm runtime.
    pub fn new() -> Result<Self, wasmer_runtime::error::Error> {
        const WASM_TRANSITON: &[u8] = include_bytes!("../transition.wasm");

        let import_object = imports! {};
        let instance = instantiate(WASM_TRANSITON, &import_object)?;

        // make sure transition is exported
        let _check_transition: Func<u32, u32> = instance.func("transition")?;

        let version_f: Func<(), u32> = instance.func("version")?;
        let version: u32 = version_f.call()?;

        Ok(WasmTransition {
            module: instance,
            version,
        })
    }
}

impl Transition for WasmTransition {
    fn transition(&self, a: u32) -> Result<u32, String> {
        let transition: Func<u32, u32> = self
            .module
            .func("transition")
            .expect("transition function disappeared from wasm module");
        transition.call(a).map_err(|e| match e {
            wasmer_runtime::error::RuntimeError::Error { .. } => "unknown error".into(),
            wasmer_runtime::error::RuntimeError::Trap { msg } => msg.into(),
        })
    }

    fn version(&self) -> u32 {
        self.version
    }
}

pub struct NaitiveTransition;

impl Transition for NaitiveTransition {
    fn transition(&self, a: u32) -> Result<u32, String> {
        Ok(crate::transition::transition(a))
    }

    fn version(&self) -> u32 {
        crate::transition::version()
    }
}
