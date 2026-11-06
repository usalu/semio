# Guest runtime notes

🎭️ `design-runtime.md` §2's `MockGuestRuntime` (`#[cfg(any(test, feature = "testkit"))]`): scripted turns + a controllable
clock, backing scheduler/failure-ladder tests without a real wasm component or `bindgen!` — the
TS twin is `createMockShard()`. `script_turn`/`script_fault` queue outcomes FIFO per actor;
`execute_turn` on an actor with an empty queue returns `TurnFault::Exhausted` rather than
silently fabricating an idle turn, so a test that forgets to script a call fails loudly instead
of passing by accident.

🧪️ Only `MockGuestRuntime` (`#[cfg(any(test, feature = "testkit"))]`) ever constructs this variant — `#[allow(dead_code)]`
rather than `#[cfg(any(test, feature = "testkit"))]` on the variant itself, so `GuestInstanceState` stays a REFUTABLE
match target (`Wasmtime` is not the only variant) in every build config; a `#[cfg(any(test, feature = "testkit"))]`
variant would make the `let GuestInstanceState::Wasmtime(state) = .. else { .. }` guards in
`WasmtimeRuntime`'s own methods irrefutable outside `cfg(test)`, turning their `else` arms into
a NEW dead-code warning instead of fixing one.
    
