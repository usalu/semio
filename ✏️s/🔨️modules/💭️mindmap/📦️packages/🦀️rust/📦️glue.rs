//! 🧠️ Mindmap module — reasoning graph-extension trait/type-alias pattern for topics and relationships
//! on a property graph. See `AGENTS.md` for the domain overview.
//!
//! WIRING ONLY. `#[path]` written in full from the owner root, prefixed with `../../` since this file
//! now lives two levels below the owner root (`📦️packages/🦀️rust/`, moved here for Shape V2 tree
//! purity — see ticket `26/08/06/S-MODULES-CRATE-CONSOLIDATION-AND-NAMING-FIX`). Do not inline the
//! component file back into this one: the taxonomy validator and the `TaxonomyLibShape` policy lint
//! both fail on it (see master ticket `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`,
//! Single-File-Repo hazard ruling).

//#region 🧩️Extension
#[path = "../../🧩️extension/🦀️component.rs"]
mod component;
pub use component::*;
//#endregion 🧩️Extension
