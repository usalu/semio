# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

📄️ `UiSnapshot` / `UiNodeRecord` / `UiNodeId` / `UiPatch` / `UiPatchOp` and the revision
model — the flat, id-keyed document every renderer reads and every reconciler writes. No type in
this file nests another node inline; a node only ever refers to a child by [`UiNodeId`], which is
what keeps the whole surface schema-projectable (see the crate's `🦀️.rs` header).

🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
below is plain sync by owner ruling U1, which supersedes this program's general async-everything
default for exactly this crate.

<a id="source-notes-371"></a>

### 🦀️.rs:371

💲️ What a resident document holding `nodes` records COSTS the aggregate: the header, the terminal page
and one record each at the size the contract gives a record, plus the fixed opening metadata. This is
the one pricing arithmetic of the whole document contract — [`UiDocumentLease::try_publish`], the cold
[`UiDocumentBuilder`] and [`UiDocumentAssembly::open_into`] all price through it, and a body that grows
climbs through it rather than reserving a ceiling it will never reach.

🐛️ ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6: the cold builder reserved
[`UI_RESIDENT_SURFACE_ITEMS`]/[`UI_RESIDENT_SURFACE_BYTES`] — the per-surface CEILING — so the ITEM
ledger alone capped concurrent documents at `floor(UI_RESIDENT_AGGREGATE_ITEMS / UI_RESIDENT_SURFACE_ITEMS)`
= 31 of the [`UI_RESIDENT_SLOTS`] = 64 slots the slot ledger offers. Every hostile fixture that walks all
sixty-four slots refused at the thirty-second, leaked its reservations on the panic and starved every
later test in the same binary. [`UI_RESIDENT_SURFACE_BYTES`]'s own docstring already names the ceiling
"a maximum a surface may reach, never a price"; this is that rule applied at the reservation itself.

<a id="source-notes-787"></a>

### 🦀️.rs:787

📜️ The payload of a document published as a paged-text carrier: the depth-first concatenation
of every `Component::Text` leaf's packed payload under the root. This is the retained wire
home of every reserved `UiRefreshSection` surface (`framework.section.measures`,
`framework.section.catalogue`, …) that `semio_plugin::app::paged_text_carrier`
publishes; the walk follows the record graph, so any page order reassembles byte-identically.

See `🧬️contract/🧪️tests/🧫️fixtures/📜️paged-text.json`.

<a id="source-notes-837"></a>

### 🦀️.rs:837

🧾️ Publishes `nodes` as one generation-qualified document through the stepped assembly protocol
with a RIGHT-SIZED resident reservation. The cold [`UiDocumentBuilder`] reserves the fixed
per-surface ceiling, and the process aggregate holds only four of those beside every live surface,
so a host that owns its records (the browser bridge's published snapshot, the wgpu shell's
Measures overlay) must price the root by what it actually holds. A refused publication returns
its reservation before it answers.
