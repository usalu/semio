//! @emoji 🧬️ The semantic UI contract — the single language-neutral boundary between the headless UI
//! runtime and every renderer (React DOM, the custom GPU family, anything later).
//!
//! Three properties define this crate:
//!
//! 1. **Flat, not recursive.** A [`UiSnapshot`] is an id-keyed table of [`UiNodeRecord`]s, never a
//!    nested tree. That is what lets one patch address one node, and what makes the whole surface
//!    schema-projectable — the owned versioned metadata keeps recursive wire types explicit.
//! 2. **Synchronous.** Validation and patch application are run-to-completion transactions with no
//!    suspension point — see ruling U1 in ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`.
//! 3. **Dependency-free.** serde, the styling tokens, and (additively, ticket
//!    `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`) `protocol::value`'s
//!    `ToValue`/`FromValue` + their leaf derive macro — nothing else. `protocol`
//!    (`semio-framework-replication`) has no edge back to os-kernel (`cargo tree -p
//!    semio-framework-replication` — hash, io-base64, value-derive, serde, serde_json only). Still
//!    no engine, no `wgpu`, no `winit`, no actor kernel, no os-kernel `dsl` — so this compiles for
//!    `wasm32-wasip2` guests and `wasm32-unknown-unknown` browsers by construction, and a CI
//!    `cargo tree` assertion keeps it so.

#[cfg(feature = "typegen")]
#[path = "../../🧬️schema/🦀️.rs"]
pub mod schema_metadata;

#[path = "🦀️accessibility.rs"]
mod accessibility;
#[path = "🦀️action.rs"]
mod action;
#[path = "🦀️builder.rs"]
mod builder;
#[path = "🦀️component.rs"]
mod component;
/// 🧪️ Loads and asserts against `📚️examples/🧪️conformance/` — entirely `#[cfg(test)]` inside, so it
/// mounts unconditionally here without affecting the wasm check targets (see the file's own header).
#[path = "🦀️conformance.rs"]
mod conformance;
#[path = "🦀️document.rs"]
mod document;
#[path = "🦀️layout.rs"]
mod layout;
#[path = "🦀️limits.rs"]
mod limits;
#[path = "🦀️presence.rs"]
mod presence;
#[path = "🦀️style.rs"]
mod style;
#[path = "🦀️surface.rs"]
mod surface;
#[path = "🦀️text_edit.rs"]
mod text_edit;

pub use accessibility::*;
pub use action::*;
pub use builder::*;
pub use component::*;
pub use document::*;
pub use layout::*;
pub use limits::*;
pub use presence::*;
pub use style::*;
pub use surface::*;
pub use text_edit::*;
