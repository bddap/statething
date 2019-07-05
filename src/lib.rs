mod runtime_selector;
mod statething;
mod transition;

pub use runtime_selector::select_runtime;

#[cfg(test)]
mod test {
    use crate::runtime_selector::select_runtime;
    use crate::statething::*;

    #[test]
    fn transition() {
        let initial: u32 = 10;
        let num_transitions = 400;

        // do n transions in wasm
        let wasm = {
            let mut st = initial;
            let runtime = WasmTransition::new().unwrap();
            for _ in 0..num_transitions {
                st = runtime.transition(st).unwrap();
            }
            st
        };

        // do n transions in native
        let native = {
            let mut st = initial;
            let runtime = NaitiveTransition;
            for _ in 0..num_transitions {
                st = runtime.transition(st).unwrap();
            }
            st
        };

        // do n transions in dynamically selected
        let dynamic = {
            let mut st = initial;
            let runtime = select_runtime().unwrap();
            for _ in 0..num_transitions {
                st = runtime.transition(st).unwrap();
            }
            st
        };

        // assert all results equal
        assert_eq!(wasm, native);
        assert_eq!(native, dynamic);
    }
}
