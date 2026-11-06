# Design Notes

## Preserved Source Rationale

<a id="source-notes-68"></a>

### 🦀️.rs:68

📬️ The plugin-facing on-demand menu request — deliberately does NOT carry view state (this crate
must never reference `semio_framework`'s `ViewModel`, same boundary as every other type
here). Mirrors `handle_action`/`render`/`tool_measures`, which all take `view_state: &ViewModel`
as a separate `ArtifactApp` method parameter rather than embedding it in the request payload; the
plugin SDK's `plugin_context_menu` free function parses the WIT-level combined JSON (which DOES
carry `viewState`, matching the TS `PluginContextMenuRequest` wire shape) and splits it into this
smaller struct plus a typed `ViewModel` before calling `ArtifactApp::context_menu`.
