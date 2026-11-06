# Design Notes

## Preserved Source Rationale

<a id="source-notes-54"></a>

### 🦀️.rs:54

🗂️ `artifact_id` → the plugin-typed binding [`HeadlessWorkspace::create_plugin_artifact`]
minted for it. The probe lane (`open_probes`) covers this crate's own generic document; this
covers every artifact created for a REAL plugin artifact kind, and is what finally gives
`artifact_export`/`semio://artifact/{id}/schema` an artifact → plugin mapping instead of the
"exactly one registered plugin or bust" guess they used to make.

🗿️ Shared with this workspace's [`RoutingArtifactChannel`] (`Arc`, one map, never a copy):
the channel needs the INVERSE lookup — which artifact a plugin's live session document is —
so `AppFrame::HistorySnapshot` can stamp the artifact rather than the plugin id.

<a id="source-notes-78"></a>

### 🦀️.rs:78

🚪️ Real teardown for a `--folder`/`--hub`-bound `semio-os-mcp` process (and every test that opens
a real probe): drains every store `open_probes` still owns to `ArtifactStore::drop`'s
terminal-empty witness instead of leaving that to `HashMap`'s own field drop, which would hit the
witness assert with nothing ever drained. `artifact_host.close` runs first for each id so the
backbone's remote side (the actor this workspace itself opened via `ArtifactHost::open`) releases
its shared queue reference before `close_probe_store_to_terminal` needs to `Arc::try_unwrap` it —
the same "release the other side before draining" order `store::sync::ArtifactHost`'s own `Drop`
already follows for its actor runners.

<a id="source-notes-159"></a>

### 🦀️.rs:159

🗿️ The body and the schema are both answerable for a hub document: the body from the
authenticated binding's own verified canonical pair, the schema from the descriptor
the binding already published (`DocumentDescriptor.artifact_schema` — the hub's own
authority over what this document IS, never a guess). Validation decodes the canonical
pair with its owner codec; remote history requires a revision-bound history surface.

🪢 Two vocabularies, both published, neither collapsed into the other. `schema` is
the declared pack schema — what the bytes are. `artifactKind` is the owning
app's canonical `<domain>.<plugin>.<artifact>` identity, whose declaring owner
`DocumentIndexEntryV1::validate` enforces — what a capability's `artifactKind`
filter and `capabilities_search` match on. Publishing only the schema is what made
an agent unable to match ANY verb to a hub document (M8 §5.4(1), measured live).

🧯️ Real answer when this workspace genuinely knows the schema (an open probe
artifact — `PROBE_SCHEMA` is this crate's own, real, no fabrication needed), a typed,
retryable gap otherwise: the real wire protocol (`store::AppCommand`, enumerated
exhaustively at `PluginArtifactChannel::exchange`'s own doc) has no
schema/describe/manifest QUERY command at all — nothing to "ask the guest" through yet
for an arbitrary plugin-backed artifact, so this names precisely what's missing rather
than fabricating a schema this workspace cannot see.

🔎️ Validation decodes the live canonical pair through its installed owner codec.
