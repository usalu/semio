# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🗺️ `SurfaceProps` — embedded product surfaces with an opaque pack-encoded payload.

Final shape (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` 📌️important.md, packet
`ui-w4-core`, item 4 — replaces this file's own former `⚠️ SCAFFOLD` header and the placement-
carrying shape `contract-doc`/`contract-layout` left behind wholesale, per that ruling):

```text
pub struct SurfaceProps {
    pub kind: SurfaceKind,
    pub doc_schema: UiText,      // "<kind>@<version>", e.g. "world3d@1"
    pub doc: SurfaceDoc,         // opaque pack-encoded bytes; this crate NEVER parses them
    pub bindings: Vec<ActionBinding>,
}
```

`SurfaceProps` names WHAT is embedded (`kind` plus a `doc_schema`-tagged opaque payload) and WHICH
surface-level intents it can fire (`bindings`) — never WHERE it sits or which product instance owns
it. The scaffold's `surface_id`/`controller_id`/`pane_id`/`binding_id`/`domain_id`/
`domain_granularity_id` fields are gone: placement is `semio-ui-render`'s own
`SurfacePlacement`/`AnySurface` concern (that crate's `🦀️surface.rs` already imports only
[`SurfaceId`]/[`SurfaceKind`] from this crate, never those dropped fields), and a per-instance
identity (which pane, which binding, which domain granularity) is exactly the kind of
product-specific concern `doc`'s own opaque, schema-tagged bytes now carry — decoded only by the
`🎬️scene` crate that actually knows `doc_schema`'s shape, never by this dependency-free contract.

`doc_schema` is `"<kind>@<version>"` (e.g. `"world3d@1"`) — the axis a renderer gates its own
per-kind decode logic on. This crate NEVER parses `doc.bytes`: [`parse_doc_schema`] only splits the
schema STRING itself, never touches the payload, and never panics — a malformed or unrecognised
schema is a typed [`SurfaceSchemaFault`], not a crash.

**The contract-side rule this file exists to guarantee: an unrecognised `doc_schema` must never
reject the surrounding [`crate::UiPatch`] or panic reconciliation.** Neither `🛡️limits.rs`'s
`component_text_bytes` nor its `component_is_finite` special-cases `Component::Surface` in any way
that could reject on an unrecognised schema (both fall through their catch-all arm for it), and
`validate_snapshot`/`apply_patch` impose zero constraint on `doc_schema` content — see this file's
own tests. The renderer that DOES recognise schemas (a sibling crate, out of this packet's scope) is
the one that actually renders a placeholder and logs the fault; this crate's only job is to make
sure nothing here stops it from doing so.

`doc`'s bytes diff as an opaque blob: [`SurfaceProps`] derives plain structural [`PartialEq`], so a
changed byte anywhere inside `doc.bytes` makes the whole `SurfaceProps` unequal to what it was —
exactly one `component` change as far as `semio-ui-runtime`'s `SurfaceReconciler` is
concerned (`Component::Surface` is diffed as part of the same `component` field group every other
`Component` variant is), which folds into a single `SetComponent`/`Upsert` op. No separate
`SetSurface` op exists or is needed — a changed scene is exactly one op.

**Signature the `🎬️scene` crate must build its typed `encode`/`decode` helpers against** (that crate
depends on this one, never the reverse):

```text
fn encode<T: Serialize>(kind: SurfaceKind, version: u32, value: &T) -> SurfaceProps;
fn decode<T: DeserializeOwned>(props: &SurfaceProps) -> Result<T, DecodeFault>;
```

where `encode` is expected to set `doc_schema` to `format!("{kind_slug}@{version}")` — the exact
string [`parse_doc_schema`] splits back apart — and `decode` is expected to call
[`parse_doc_schema`] first and treat a [`SurfaceSchemaFault`] (or a recognised-but-unimplemented
kind/version pair) as its own `DecodeFault::UnknownSchema`-shaped case, never a panic. The scene
crate owns `DecodeFault`, the per-kind `kind_slug` strings, and the actual pack encode/decode of
`T`; this crate defines only the opaque envelope and the schema-string convention.

🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
below is plain sync by owner ruling U1.

<a id="source-notes-143"></a>

### 🦀️.rs:143

🛍️ An embedded product surface. Replaces the old `UiComponentSceneNode`'s 15 sparse
`Option<XxxScene>` fields with exactly ONE payload, identified by `doc_schema` — the 15 product
scene structs themselves stay product payloads and move to `🖱️ui/🎬️scene/🦀️.rs` in a
later packet, never into this dependency-free contract crate. See this file's own module doc for
the exact reasoning behind each field (and each field the scaffold this replaces used to carry but
no longer does).
🌱️ No `ToValue`/`FromValue` here: `bindings: UiNodeBindings` embeds `crate::ActionBinding`, the
deliberate DslValue-free exception — see its own note above.

<a id="source-notes-187"></a>

### 🦀️.rs:187

🚧️ Why a `doc_schema` string could not be parsed into a [`SurfaceSchema`] — NEVER a panic, always
this typed fault. A renderer that cannot resolve a `doc_schema` — either it fails to parse, per this
type, or it parses cleanly but names a `kind`/`version` pair the renderer does not implement — MUST
render a placeholder and log the fault; it must never panic and must never drop the surrounding
[`crate::UiPatch`]. Dropping one surface's content is not license to reject the whole document a
`SetComponent`/`Upsert` arrived in.

<a id="source-notes-205"></a>

### 🦀️.rs:205

🔪️ Splits `doc_schema` into a [`SurfaceSchema`], or a [`SurfaceSchemaFault`] describing exactly why
it could not — never panics on any input, including empty strings, strings with no `@`, or a
non-numeric version half. This crate calls this function nowhere itself (validation never rejects on
`doc_schema` content — see this file's own module doc); it exists for renderers to call so their own
"unknown schema → placeholder + logged fault, never a panic" behaviour has a shared, tested
building block instead of each renderer hand-rolling its own `split_once('@')`.
🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
