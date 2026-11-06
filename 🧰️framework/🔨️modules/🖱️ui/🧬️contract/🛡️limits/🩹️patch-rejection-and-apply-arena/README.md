# Design Notes

## Preserved Source Rationale

<a id="source-notes-23"></a>

### 🦀️.rs:23

🕳️ An op named a [`crate::UiNodeId`] that has no record in the receiver's current state — only
possible for ops that mutate an existing node ([`crate::UiPatchOp::SetComponent`]/
[`crate::UiPatchOp::SetLayout`]/[`crate::UiPatchOp::SetActivity`]/
[`crate::UiPatchOp::SetChildren`]/[`crate::UiPatchOp::SetStyle`]/
[`crate::UiPatchOp::SetAccessibility`]/[`crate::UiPatchOp::SetBindings`]/
[`crate::UiPatchOp::SetMenu`]); `Upsert`/`Remove`/`SetRoot` never fail this way.

<a id="source-notes-121"></a>

### 🦀️.rs:121

🧊️ `const`, not `Default` — and the static below is a `const`-initialised `Mutex`, not a
`LazyLock`. One slot is ~11.7 KiB (its `UiPatchRetirement` carries the validation `seen` matrix),
so the arena is ~93.5 KiB; at `opt-level = 0` the old `std::array::from_fn` initialiser
materialised that array once per frame of the `from_fn → try_from_fn → try_from_fn_erased` chain
plus the `LazyLock` closure's return temporary — ~654 KiB of shadow stack to zero a static, which
is most of a guest's tape and overflowed the whole thing during `Event::InstanceOpen`
(ticket 26/09/18 slice A2). A `const` initialiser is laid down in the image instead: zero frames,
zero copies, zero first-touch cost.
