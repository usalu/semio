# Design Notes

## Preserved Source Rationale

<a id="source-notes-114"></a>

### 🦀️.rs:114

#endregion 🔖️Invocation
#region 🔖️Effect
🪪️ `rename_all` on an enum only renames variant tags ("setActiveUtility"), not the fields *inside* each
struct-variant — those need `rename_all_fields` (serde 1.0.126+) or every multi-word field here
(window_kind_id, mime_type, plugin_id, ...) silently serializes as snake_case, breaking any TS side
that destructures camelCase (confirmed live: `SetActiveUtility` was shipping `window_kind_id`/`utility_id`,
so the host-owned utility switch after `openVortexSuggestions` never applied and the brush preview never
rendered).
🐚️ A typed side effect the guest emits toward the host — `📓️design-abi.md` §2, replacing
`HostEffect` now that plugins and extensions share one `actor` world. Every variant that
existed as `HostEffect` keeps its exact name and fields (mechanical `HostEffect` → `Effect`
rename at every call site); six of them (`OpenWindow`, `RequestFileOpen`, `RequestMediaFrames`,
`SpawnPluginInstance`, `OpenDialog`, `DispatchAction`) additionally gain a `req: RequestId` now
that they complete; `InvokeExtension` loses `response_action` and gains `req` (the SDK resumes
the awaiting future instead of a redispatch). The rest are new: messaging, blobs, documents,
links, registry lookups, io composition, engine caches, jobs, storage, capability admin, and
pub/sub — see `📓️design-abi.md` §2's table.
🚧️ Both `Serialize` AND `Deserialize` stay unconditional (kept additive, not stripped):
`semio-plugin`'s `RefreshResponse` (`🔌️plugin/🦀️.rs`, `requested_effects: Vec<Effect>`)
derives `Serialize` in production and needs `Effect: Serialize`; this file's own `TurnResult`
(`effects: Vec<Effect>`, above `🔖️Invocation`) derives `Deserialize` in production too — a real
consumer a first pass at `#[cfg_attr(test, derive(Deserialize))]` missed (confirmed the hard way:
`cargo check -p semio-framework` failed E0277 on `Effect: serde::Deserialize` at `TurnResult`'s
own derive site before this was reverted). `Effect` already carries `ToValue`/`FromValue`
alongside — this file's own 4 `#[cfg(test)]` round-trip oracles in `🛂️manifest/🦀️.rs` exercise
the same, still-production, serde derives; nothing further to move to `[dev-dependencies]`.
Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.

<a id="source-notes-224"></a>

### 🦀️.rs:224

🎞️ Asks the shell to decode a video (via file picker, or `payload` bytes when the
caller already has them, e.g. a drop zone) and re-dispatch `frame_action` once per sampled
frame with `{payload: dataUrl(image/jpeg), name, frameIndex, timestampMs, index, total, width,
height, ...args}`, then `done_action` once with `{name, durationMs, frameCount, sampledCount,
width, height, codec, ...args}`. `sample_stride`/🧰️framework/🔨️modules/🎠️kernel`max_frames`/🧰️framework/🔨️modules/🎠️kernel`max_long_edge_px`/🧰️framework/🔨️modules/🎠️kernel`fps_hint` are
hints only (0 = host default); a host that can't decode the codec dispatches `fallback_action`
once instead, with `{payload: dataUrl(raw container bytes), name, ...args}` — mirrors
`RequestFileOpen`'s per-file re-dispatch shape but fans out video frames instead of files.

<a id="source-notes-314"></a>

### 🦀️.rs:314

⏪️ Asks the shell to redispatch a shell-owned command (dock/theme/locale/panel chrome)
whose real mutation and its inverse both live client-side — the plugin has no access to that
state, so `revertToCommand` on a `🐚️Shell`-kind history row bubbles the row's stored inverse out
here instead of replaying it internally the way a `View`-kind row does (see
`NOTE_SHELL_COMMAND_ACTION_ID` and `VcsArtifactApp::dispatch_action`'s `REVERT_TO_COMMAND_ACTION_ID`
arm). The shell is expected to redispatch `action_id`/`args` through its normal command funnel,
which itself calls `noteShellCommand` again — so the revert is itself a new, further-revertible row.

<a id="source-notes-327"></a>

### 🦀️.rs:327

🔌️ Asks the shell to invoke an extension capability. `req` is MINTED by the guest
request registry (`semio-plugin`'s `RequestRegistry`) and is meaningless — a
silently discarded result — when it is written by hand: `Event::Completed { req, .. }` is
routed exclusively through that registry, and an id with no registry slot resolves nothing.
The variant is therefore `#[non_exhaustive]`: no crate outside this one can write the struct
literal, and the only in-repo constructor is [`Effect::invoke_extension`], called from the
three registry/host lift sites that legitimately own a minted id. Application code reaches
this effect exclusively through `Emit::extension_invocations`.

<a id="source-notes-540"></a>

### 🦀️.rs:540

#endregion 🧵️SpawnedJobDrive
#endregion 🔖️Effect
🎯️ W6 kernel unification: re-exports `protocol::ArtifactDiff` (schema: `SchemaId`, payload:
`Vec<u8>` — the binary shape from W5's causal envelope reshape) in place of the old kernel-local
`{schema_id, payload: Value}` shape. Zero external consumers of the old shape existed outside
this crate's own (now-deleted) OS JSON-patch kernel and `store`/`store_sync` (both repointed to
`protocol::ArtifactDiff` directly in this same wave) — verified by a repo-wide grep before this
change, not assumed.

<a id="source-notes-850"></a>

### 🦀️.rs:850

👥️ M2 (ticket 26/08/17 `design-unified.md`): render-plane presence derived this turn by the
reactor's own `PresenceHub` — `(surface, node_key)`-addressed, TTL-scoped, NEVER a document
revision (a turn where only presence changed emits `presence` and zero `ui_patches`). Distinct
from the roster's own replication channel (`protocol::PresencePeer`/`adopt_presence`), which
is unchanged and carries collaboration TRUTH, not render addressing — see this field's own
wire doc at `kernel_turn_result_to_wit` for the WIT `presence-update` repoint this pairs with.
