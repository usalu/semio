//! 🧬️ Puzzle3d artifact schema — every field of the artifact with its state class.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use crate::snapshot::Puzzle3dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
#[path = "🗿️artifact-identity/🦀️.rs"]
mod artifact_identity;
pub use self::artifact_identity::*;

//#region 🔖️Descriptor
/// 🧬️ The artifact facet's five handcrafted schema leaves (rust, typescript, graphql, json schema, proto).
pub const SCHEMA_LEAVES: ::semio_framework_schema_registry::FacetLeaves = ::semio_framework_schema_registry::FacetLeaves {
    rust: include_str!("🦀️.rs"),
    typescript: include_str!("🟦️.ts"),
    graphql: include_str!("🔗️.graphql"),
    json_schema: include_str!("🔣️.json"),
    proto: include_str!("🛰️.proto"),
};
//#endregion 🔖️Descriptor

#[path = "🧱️precompute-catalogs/🦀️.rs"]
mod precompute_catalogs;
pub use self::precompute_catalogs::*;

#[path = "🎬️precompute-commands/🦀️.rs"]
mod precompute_commands;
pub use self::precompute_commands::*;

//#region 🧪️PrecomputeModelTests
/// 🧪️ The one puzzle3d-precompute test harness — every sibling app-side precompute test file builds on
/// it instead of re-deriving a mesh-buffer/scene/fill-plan scaffold of its own. Compiled for this crate's own tests
/// and, behind the `testing` feature, for the precompute crates' tests (session/geometry/brush/fill).
#[cfg(any(test, feature = "testing"))]
#[path = "🧪️tests/🔬️precompute-model/🦀️.rs"]
pub mod precompute_model_tests;
//#endregion 🧪️PrecomputeModelTests

#[path = "📚️document-model/🦀️.rs"]
mod document_model;
pub use self::document_model::*;

#[path = "📐️pose-math/🦀️.rs"]
mod pose_math;
pub use self::pose_math::*;

//#region 🌳️Modules
#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
    #[path = "."]
    pub mod sqlite {
        #[path = "../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
    #[path = "."]
    pub mod text {
        #[path = "../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
}
#[path = "."]
pub mod snapshot {
    #[path = "📸️snapshot/🦀️.rs"]
    mod component;
    pub use component::*;
}
//#endregion 🌳️Modules
