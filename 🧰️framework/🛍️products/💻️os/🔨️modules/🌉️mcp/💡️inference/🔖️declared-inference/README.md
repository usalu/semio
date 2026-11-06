# Design Notes

## Preserved Source Rationale

<a id="source-notes-79"></a>

### 🦀️.rs:79

🧩️ The installed packages that CONTRIBUTE an inference onto an artifact kind `primary` already
owns, and that `primary` does not itself carry.

🐛️ A contribution-only package declares no app, no command and no owner-authored inference
service, so `catalog::compile` — which walks apps, commands, modes and
`contributions.{inference,mutation,io,composer}_services`, never
`contributions.artifact_contributions` — emits no catalog entry for it at all. Folder-mode
`HeadlessWorkspace::discovery_descriptors` derives its descriptor set from exactly those catalog
entries (`catalog_plugin_ids`), so an extension's inference could never appear in `inference_list`
— `cad-extension-aec-building`'s `s.cad-extension-aec-building.building-structure-summary` on
`s.cad.cad` is real, committed, and was invisible (`📓️g19-ai-user-experience-audit.md` gap 3,
measured by CE1 §3b: the roster was `gis ×1 + wfc ×5`).

🎯️ The membership rule is the reachability one, not "every installed package": a contributed
inference is listed exactly when the artifact kind it contributes onto is owned by a package this
workspace already reaches. An extension whose host plugin is absent contributes to nothing here
and stays out, so the roster never advertises a service no artifact in this workspace can carry.

<a id="source-notes-123"></a>

### 🦀️.rs:123

💡️ Real, static, plugin-agnostic discovery: the UNION of every registered plugin's own declared
inference roster (`workspace.catalog_plugin_ids()` — no plugin id is hardcoded here, no
single-plugin assumption; ticket 26/08/29/AI-MCP-END-TO-END packet W8, `📓️w8-capability-routing.md`
replaced the OLD `HeadlessWorkspace::resolve_default_plugin_id`-based version, which could only
ever read ONE plugin's descriptor and outright FAILED — the same `PLUGIN_UNAVAILABLE` "2+ plugins
is ambiguous" defect this whole ticket fixes — the moment a catalog named more than one). Zero
registered plugins is still the same typed, retryable `PLUGIN_UNAVAILABLE`; any OTHER plugin's
registry/descriptor lookup failing aborts the whole roster rather than silently dropping it.

🧩️ …plus every extension-contributed service reachable from that set — see
[`contributed_inference_descriptors`] for why a contribution-only package is structurally absent
from the catalog the primary set is derived from.

<a id="source-notes-148"></a>

### 🦀️.rs:148

💡️ Resolves `artifact_id`'s real schema by delegating to the SAME `semio://artifact/{id}/schema`
answer `HeadlessWorkspace::read_artifact_resource` already implements, rather than re-deriving
"is this an open probe" a second time: a real schema for an open probe artifact (`PROBE_SCHEMA`),
the same typed, retryable `PLUGIN_UNAVAILABLE` for any other id — including one this workspace has
never seen at all — since the real wire protocol has no schema/describe query command yet to tell
the two apart (`🏠️workspace`'s own `/schema` arm never answers `NOT_FOUND` either, only `Ok` or
this one retryable gap).

<a id="source-notes-161"></a>

### 🦀️.rs:161

💡️ Discovery for one artifact id: `PROBE_SCHEMA` (this crate's own synthetic probe schema, real
for any workspace-opened probe artifact) genuinely has zero declared inferences — no plugin
depends on it — so that case answers `[]` honestly rather than propagating a gap. Any other
document is matched against the workspace-wide roster by its artifact kind (the owning app's
dialect coordinate, [`document_artifact_kind`]) — the one key [`select_inference_service`] runs a
request on, so what a list names is exactly what a submit can start.
