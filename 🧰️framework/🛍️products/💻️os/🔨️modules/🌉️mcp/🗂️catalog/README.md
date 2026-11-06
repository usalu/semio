# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🗂️ `CapabilityDefinition` + the catalog compiler — packet `P2-catalog`, `📓️design-decisions.md`
D5 (these types live HERE, not in `🛂️manifest`) and D3 (the `<plugin_id>.<app_id>.<action_id>`
id grammar is mandatory: a bare action id is never a capability id, since 14 action ids collide
across plugins). `compile()` walks real `semio_framework::manifest` types
(`PackageDescriptor`/`PluginManifest`/`AppDefinition`) — this facet is the ONLY place in the
gateway crate that depends on `semio-framework`; every other facet stays framework-free
(`📓️terra-P1a-report.md` §5, D8) so the peer ticket's mid-flight plugin-host rewrite can never
break this crate's own build.
