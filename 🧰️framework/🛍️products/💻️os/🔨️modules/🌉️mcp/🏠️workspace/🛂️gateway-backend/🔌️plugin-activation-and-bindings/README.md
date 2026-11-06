# Design Notes

## Preserved Source Rationale

<a id="source-notes-24"></a>

### 🦀️.rs:24

🔌️ Constructs a real `crate::actions::ArtifactChannel` for `plugin_id` — what
`P6-actions-policy`'s `ActionAdapter::new` takes in place of `MockArtifactChannel` once wired
(bin.rs's job, once a real `--folder`/`--hub` workspace names which plugin its default
instance targets). Thin wrapper over `open_plugin_artifact_channel` — the SAME body
`RoutingArtifactChannel::exchange` calls per plugin, kept here too for direct low-level tests
and any caller that already knows exactly which plugin it wants (never duplicated).

<a id="source-notes-158"></a>

### 🦀️.rs:158

🗂️ Every artifact kind the INSTALLED plugins actually declare, read off their descriptors
(`AppIo.artifact_schema` of each editor app, plus every `ArtifactKindSpec.kind_id` that app
declares) — the real vocabulary `artifact_create{kind}` validates against and `artifact_export`
reads its ports from, so an unknown kind is a typed refusal naming the installed set instead of a
silently generic document. Never invented: a folder workspace reads the repository's committed
descriptors (a plugin with no decodable descriptor contributes nothing); a hub workspace reads the
packages the hub's live descriptor authority selected — the same descriptors its guests run.

📤️ `media_out_ports` is [`app_media_out_ports`], not `AppDefinition.media_outputs` alone —
see that function for the measurement.
