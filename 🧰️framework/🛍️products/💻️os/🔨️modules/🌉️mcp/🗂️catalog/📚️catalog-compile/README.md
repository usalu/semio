# Design Notes

## Preserved Source Rationale

<a id="source-notes-75"></a>

### 🦀️.rs:75

🏭️ Compiles a `CatalogSource` into a sorted, hashed `Catalog` — `📋️master.md` §3.2. Walks, per
descriptor: every app's window-kind actions (skipping the 21 framework-injected ids, folded
separately into `framework.*` — see `framework_capabilities`), app-scope commands, mode-scope
commands, plugin-scope commands, and the four contribution categories (inference/mutation/io/
composer → `Query`/`Job`); then folds every collected `DialogDefinition`/`ExampleDefinition` id
into the two gateway-owned `ui.dialog.open`/`artifact.create` capabilities; then walks
`source.os_commands` and appends `source.shell`/`source.gateway` verbatim.

<a id="source-notes-86"></a>

### 🦀️.rs:86

🏭️ [`compile`] with an explicit audience projection — see [`CapabilityAudience`]. Every
declaration is still walked and still fights for its id (a duplicate stays
[`CatalogError::DuplicateCapabilityId`] even when the colliding pair is filtered out), so
narrowing the published set can never hide a real collision; only the FINAL entry vector, and
therefore `hash`, is filtered.

🌱️ `artifact.create` has exactly ONE definition in a compiled catalog: `🗿️artifact`'s own
invocable `artifact_create` tool when the source carries it (the live gateway always does),
otherwise this module's catalog-only projection. The declared templates are an ARGUMENT of
that verb, never a second capability wearing its id — which is what made a live catalog with
both sources refuse to compile at all (`duplicate capability id: artifact.create`).
