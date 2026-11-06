# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🧬️ The `framework.ui.contract` schema leaf: this scope's named-export registration, the Rust half
of the `PresenceUpdate` export `🔣️.json` declares — contract
§B: a `🧬️schema/🦀️.rs` carries the scope's pub types and `include_str!`s its siblings), plus the
versioned semantic UI wire-type metadata and its owned TypeScript projection — the latter
extracted from owner `📦️packages/🦀️rust/🦀️.rs` so that file stays pure wiring (no
`struct`/`impl` of its own), which keeps its taxonomy package role classified as thin
delegation rather than implementation — otherwise it competes with `🦀️.rs` for the crate's one
canonical implementation slot.

The module itself mounts unconditionally (the registration below must run in every build); only
the TypeScript projection stays behind `feature = "typegen"`, which is the only thing that ever
needed gating.

<a id="source-notes-18"></a>

### 🦀️.rs:18

🗂️ The formats every export of this scope exists in, which is every format this module carries a
file for. It is the single source of the `"x-semio-formats"` annotation each `$defs` entry of
`🔣️.json` declares (contract §A: the annotation lists every format the export exists in,
including the normative one) — `restricted_formats_match_the_annotation` below asserts the two
cannot drift. Public so a boot call site or a conformance checker reads the declared set from the
scope itself instead of restating it.
