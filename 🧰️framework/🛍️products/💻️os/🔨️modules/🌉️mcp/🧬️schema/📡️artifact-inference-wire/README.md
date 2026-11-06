# Design Notes

## Preserved Source Rationale

<a id="source-notes-40"></a>

### 🦀️.rs:40

💡️ The request `semio_plugin_host::ArtifactInferenceRouter::infer` decodes — os is a
CLIENT of that router here, so this export is an explicit MIRROR of the router's own private
`InferenceRouteRequest` (which that crate does not export), never a second authority. The router
re-checks every identity field against its registered route and against the guest's echo before
any result is publishable, so a drift between this mirror and the router is a decode failure at
the first call, not a silently wrong answer.
