//! 🌉️ `semio-framework-os-mcp` glue — mounts every `🌉️mcp` facet (`⚠️errors`/`🧬️schema`/`🧭️protocol`/
//! `🚚️transport`/`🎫️handles`/`📒️audit`/`🧵️bridge`/`🗂️catalog`/`🔎️search`/`🧠️context`/`🧪️conformance`/
//! `🧫️fixtures`/`🔀️dispatch`/`🛡️policy`/`🏠️workspace`/`📇️registry`/`🗿️artifact`/`💡️inference`/`🖥️ui`/`💬️prompts`)
//! plus the module root, exactly as
//! `🏃️run`/`🖥️shell`'s own glue files mount theirs.

// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P7-headless-workspace: `store`
// is `semio-framework-os-kernel` under the SAME alias `🏃️run/🦀️.rs` uses for it — a single alias
// (not also `dsl`/`protocol`, `🏃️run`'s own convention) because this crate's own `🧭️protocol` facet
// (P1a, the MCP JSON-RPC protocol core) already owns the name `protocol` at this crate's root;
// `semio_framework_os_kernel`'s crate-root glob re-exports (`os_dsl::*`/`os_store::*`/`os_spr::*`,
// verified in that crate's own `🦀️.rs`) make every item `🏠️workspace` needs (`ArtifactDsl`,
// `ArtifactPack`, `Mutation`, `MutationDiff`, `OpText`, `OpBinary`, `ArtifactStore`,
// `create_document_envelope`, `TextError`, `TextSpan`, `sync::*`) reachable through this one alias.
extern crate semio_framework_os_kernel as store;

#[path = "../../⚠️errors/🦀️.rs"]
pub mod errors;

#[path = "../../🧬️schema/🦀️.rs"]
pub mod schema;

#[path = "../../🧭️protocol/🦀️.rs"]
#[macro_use]
pub mod protocol;

#[path = "../../🚚️transport/🦀️.rs"]
pub mod transport;

#[path = "../../🎫️handles/🦀️.rs"]
pub mod handles;

#[path = "../../📒️audit/🦀️.rs"]
pub mod audit;

#[path = "../../🧵️bridge/🦀️.rs"]
pub mod bridge;

#[path = "../../🗂️catalog/🦀️.rs"]
pub mod catalog;

#[path = "../../🔎️search/🦀️.rs"]
pub mod search;

#[path = "../../🧠️context/🦀️.rs"]
pub mod context;

#[path = "../../🧪️conformance/🦀️.rs"]
pub mod conformance;

#[path = "../../🧫️fixtures/🦀️.rs"]
pub mod fixtures;

#[path = "../../🔀️dispatch/🦀️.rs"]
#[macro_use]
pub mod actions;

#[path = "../../🛡️policy/🦀️.rs"]
pub mod policy;

#[path = "../../🏠️workspace/🦀️.rs"]
pub mod workspace;

#[path = "../../📇️registry/🦀️.rs"]
pub mod registry;

#[path = "../../🗿️artifact/🦀️.rs"]
pub mod artifact;

#[path = "../../💡️inference/🦀️.rs"]
pub mod inference;

#[path = "../../🖥️ui/🦀️.rs"]
pub mod ui;

#[path = "../../💬️prompts/🦀️.rs"]
pub mod prompts;

#[path = "../../🦀️.rs"]
mod root;
pub use root::*;
