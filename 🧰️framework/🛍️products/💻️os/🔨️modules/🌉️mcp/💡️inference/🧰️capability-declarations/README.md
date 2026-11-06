# Design Notes

## Preserved Source Rationale

<a id="source-notes-147"></a>

### 🦀️.rs:147

💡️ The general plugin-declared inference execution capability — the one that is NOT hub-bound.
Every inference service any installed plugin declares is reachable through it, routed to that
plugin's own guest by `ArtifactInferenceRouter` (`🏠️workspace`'s `PluginArtifactChannel`), so
this is the tool that retires the blanket `channel.not-wired` gap. Expensive by construction, so
it always mints a job handle: progress is readable with `job_get` and cancellation is requested
with `job_cancel`, the same plugin-agnostic registry every other long-running gateway job uses.
