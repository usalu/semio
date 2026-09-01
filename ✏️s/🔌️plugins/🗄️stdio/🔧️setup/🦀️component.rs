//! 🪹️ Setup facet for `🗄️stdio` — declares nothing on purpose. `🗄️stdio` registers its codecs,
//! languages and importers directly on each artifact's own `ArtifactDeclaration` (`.setup(...)` on
//! the artifact's `🦀️component.rs`, e.g. `🗿️artifacts/🧊️gltf/…/🦀️component.rs`), not through a
//! standalone plugin-root fan-out hook — this facet has never carried real registration code (only
//! `🌍️gis`/`💠️lowpoly`/`📕️norm` ever did, and their `register_*_exports` fan-outs have since been
//! folded into their own artifacts' registration paths, leaving no plugin with real content here).
