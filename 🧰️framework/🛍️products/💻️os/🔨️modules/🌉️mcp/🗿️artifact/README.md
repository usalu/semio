# Design Notes

## Preserved Source Rationale

<a id="source-notes-26"></a>

### 🦀️.rs:26

🥈️ `artifact_export` alone needs a SPECIFIC plugin id (to read its committed `export_formats`
via `resolve_plugin_export_formats`). This workspace has no `artifact_id` → `plugin_id` mapping —
every artifact it manages today is the generic, schema-agnostic probe document `🏠️workspace`'s own
module doc describes, never a plugin-typed one — so the only honest answer is the workspace's
plugin when there is EXACTLY one; two or more is a typed, named gap (never a guess at which one
owns `artifact_id`), same real, retryable `PLUGIN_UNAVAILABLE` shape the deleted
`resolve_default_plugin_id` used to build.

<a id="source-notes-140"></a>

### 🦀️.rs:140

🌎️ Opening a HUB document is what binds it to the plugin, app and surface the hub's own
execution-target lease names, and hands that binding the canonical pair just read — so
every later `action_prepare` on this session runs the plugin's guest against THIS
document rather than against the plugin's genesis (ticket 26/09/18 slice M10, step 1 of
M8 §5.3's write path). A workspace that is not hub-bound, or a document the hub
authorizes no execution target for, binds nothing and still opens.

🟢️ The open answers once the document's actor is live (bounded), so the hub's tail past that pair —
every writer's edit since the last checkpoint — is already queued for the guest when the agent's first
command runs; `sessionDocument.sync` reports the link either way.
