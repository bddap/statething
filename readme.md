# Goals

In ~4 hours, design and implement some state machine accepting some non-empty set of events. Express the state machine as a native program and as wasm. Create a system to dynamically choose an implementation of the state machine.

# Dev log

## 0 - design brainstorm - 40 minutes

Loose brainstorm session, organizing implementation plan. `design0.md` was populated with a tentative roadmap.

## 1 - research and roadmap culling - 20 minutes

Researched available wasm runtimes and picked wasmer. Culled unnecessary items from previous brainstorm. Wrote new roadmap to `design1.md`

## 2 - implement interface for native and wasm state machines - 1 hour and 15 minutes

Got wasm module to compile. Got wasmer to import and run said module. Wasmer was easier to use than expected.

Initially, I expected code compiled to wasm not to support types outside the list of wasm primitives (i32,i64, f32, f64) but experimentation revealed function signatures like `fn([[u8; 9]; 40]) -> ()` compile to wasm without issue. This meant serialization and deserialization at native->wasm boundaries was not necessary.

u32 was used as the state for simplicity and one event/transition was defined. The transition was simply a wrapping increment. Because wasmer make easy the passing complex types as arguments to wasm functions, the state could be changed to something more interesting with little effort.

## 3 - write tests, debug - 45 minutes

The first test I wrote caught an bug. After some time debugging, I discovered it was an issue with my Makefile. Debugging and fixing took a while so there is currently only one test.

## 4 - assemble this summary - 1 hour

After ~3 hours, this projects stated goals were met, so I assembled this log of my progress.

# Unimplemented features

- more interesting state machine
- more tests for the runtime selection feature
- build.rs for easy .wasm compilation
- exhaustive implementation description

# Running the test

Make sure the `wasm32-unknown-unknown` compilation target is installed using rustup. Run `make transition.wasm` to build the wasm version of the state transition then run `cargo test`.
