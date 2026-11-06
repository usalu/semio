# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🧩️ The semantic `Component` enum and its per-component prop structs — the closed set of
things a [`crate::UiNodeRecord`] can render. Every prop struct carries only the data specific to
that component: identity lives on the record (`key`), actions live on the record (`bindings`),
visual state lives on the record (`activity`/`disabled`/`transition`) or its `layout`/`style`. A
prop struct that inlined any of those would be reintroducing the implicit coupling this contract
replaces.

🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
below is plain sync by owner ruling U1.
