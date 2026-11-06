# Design Notes

## Preserved Source Rationale

<a id="source-notes-76"></a>

### 🦀️.rs:76

🚧️ **Known, documented gap, same class as `🏃️run`'s own** (this file's module doc): a full
`AppCommand`/`AppFrame` exchange loop over `GuestRuntime::execute_turn`'s effects (encode each
`AppCommand` through the retained page ingress, scan `TurnResult.effects` for `Effect::Respond{req,
result}` where `req.0 == seq`, decode `RequestOutcome::Ok(bytes)` as an `AppFrame`) is NOT built
by this packet either — `🏃️run/🦀️.rs`'s own `WasmtimeNodeHost::exchange` doc comment
names this as real, correctly-typed, still-unwritten "post-turn effect-dispatch loop" work
belonging with the kernel/scheduler (H1-H4/T1), not a single headless-workspace packet. What THIS
region proves for real, with a real compiled `.wasm` component and a real `wasmtime::Store`, is
the step immediately before that gap: compile -> instantiate -> submit `Event::InstanceOpen` and
read back a real `TurnResult` (status/fuel/effect count) — the actor genuinely comes up. See the
packet report's live transcript for what this returns against `🗒️note`'s real committed descriptor
and real `wasm32-wasip2` build.

<a id="source-notes-98"></a>

### 🦀️.rs:98

🎟️ Every capability the descriptor requests is granted with a zero token/no expiry — the REAL
broker (`P6-actions-policy`'s `AgentBroker`) belongs one layer up, gating which scopes an agent
principal is even allowed to request before an instance is ever opened; this is the narrowest
grant shape that lets `Event::InstanceOpen` proceed for a first activation proof, not a policy
decision this packet is authorized to make for real (§2 of the brief: `🛡️policy` is P6's).

🎟️ `GuestRuntime::instantiate` takes the grants: under the compiled runtime the store's
capability table is built with the instance, not handed to it by the first event. The same
vector still travels in `Event::InstanceOpen` below, which is where the guest reads it.
