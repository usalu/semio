//! 🪪️ Capabilities facet for `🗄️stdio` — declares nothing on purpose. No plugin in this repo has
//! ever called `PluginBuilder::capability`/`.local_backbone_storage()` from a `🎟️capabilities` facet
//! file (`rg -n '\.capability\(' .` finds real calls only on artifact-level `ArtifactDefinition`
//! builders, e.g. `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🦀️.rs:266`); the one real
//! `.local_backbone_storage()` call in the whole repo lives at `✏️s/🔌️plugins/🪐️space/🦀️.rs:559`,
//! on the plugin's own root builder chain, not in a facet file like this one.
