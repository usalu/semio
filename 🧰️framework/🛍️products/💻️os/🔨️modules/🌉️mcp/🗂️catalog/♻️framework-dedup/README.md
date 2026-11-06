# Design Notes

## Preserved Source Rationale

<a id="source-notes-19"></a>

### 🦀️.rs:19

🎬️ One entry per distinct verb an app declares across all of its window kinds, in declaration
order, each paired with the FIRST window kind that declares it. An app's window kinds are views
onto one action registry — every plugin in the installed registry repeats its whole roster in
every window kind (`note` 54 verbs × 2 kinds, `architect` 39 × 5, `norm` 240 × 15) — so an action
id is a verb of the APP, not of a window, and collapsing them here is what makes
`<plugin>.<app>.<verb>` a unique capability id at all. The retained window kind is deliberately a
concrete declaring kind rather than a `"*"` marker: it is the `window_kind_id` a dispatch has to
address (`ActionAddress.window_kind_id`, checked against the guest's own
`registry.window_action`), which `"*"` could not satisfy. Two window kinds declaring the same id
with DIFFERENT definitions stay a real collision, reported as
[`CatalogError::DuplicateCapabilityId`].

🗒️ App-SCOPE actions (`AppDefinition.actions`) are the other half of the surface, and walking
only `window_kinds` silently dropped every one of them: `🗒️note` declares all 48 of its verbs
here, so `capabilities_search "delete the selected blocks"` could not find a single note
capability while `🖍️draw` — which declares on its canvas window kind — worked. An app-scope
action belongs to no particular window, so it addresses `"*"`, the same marker
`framework_capabilities` already uses for the framework-injected verbs. A window kind that
also declares the id wins (it carries a dispatchable concrete `window_kind_id`), which is why
this loop runs second and skips what `seen` already holds instead of reporting a collision.
