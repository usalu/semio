//! 🧬️ Raster artifact schema — every field of the artifact with its state class.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

use protocol::{Mutation, OpBinary};
use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ raster document artifact state.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.raster.raster")]
pub struct RasterArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    pub layers: Vec<RasterLayerNode>,
    #[state(artifact)]
    pub assets: RasterOwnedMap<RasterAssetChild>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for RasterArtifact {
    fn default() -> Self {
        Self { schema: RASTER_DOCUMENT_SCHEMA.into(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() }
    }
}

impl RasterArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> RasterSnapshot {
        RasterSnapshot { schema: self.schema.clone(), id: self.id.clone(), title: self.title.clone(), layers: self.layers.clone(), assets: self.assets.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: RasterSnapshot) -> Self {
        Self { schema: snapshot.schema, id: snapshot.id, title: snapshot.title, layers: snapshot.layers, assets: snapshot.assets }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: RasterSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.title = snapshot.title;
        self.layers = snapshot.layers;
        self.assets = snapshot.assets;
    }
}
//#endregion 🔖️Conversions

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🌱️ Relocated verbatim from `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
/// rule 3: pure helpers over document types live in `🧬️schema/`). Every external call site now reads
/// `crate::standards::v1::subsets::any::schema::…` (the artifact root's own pre-existing `pub mod schema { pub
/// use super::standards::v1::subsets::any::schema::*; }` shim keeps that path resolving).
use crate::snapshot::RasterSnapshot;

#[path = "🌳️layer-tree-helpers/🦀️.rs"]
mod layer_tree_helpers;
pub use layer_tree_helpers::*;

//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;

//#endregion 🔁️Re-exports


/// 🖼️ A one-frame straight-alpha RGBA8 image of `width × height` over `rgba8`.
pub fn semio_image_from_rgba8(width: u32, height: u32, rgba8: Vec<u8>) -> SemioImageSnapshot {
    SemioImageSnapshot { schema: "s.stdio.semio.image".into(), width, height, colorspace: semio_s_artifact_stdio_semio_image::schema::snapshot::SemioColorspace::Rgba, bit_depth: 8, frames: vec![semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageFrame { delay_ms: 0, rgba8 }], icc: None, metadata: Vec::new() }
}


pub const RASTER_DOCUMENT_SCHEMA: &str = "raster.document";

#[path = "🗺️owned-map/🦀️.rs"]
mod owned_map;
pub use owned_map::*;

#[path = "📐️layer-model/🦀️.rs"]
mod layer_model;
pub use layer_model::*;

#[path = "🖼️asset-content-and-patch/🦀️.rs"]
mod asset_content_and_patch;
pub use asset_content_and_patch::*;

//#region 🪢️TaxonomyMounts
#[path = "💡️inferences/🦀️.rs"]
pub mod inferences;
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
#[path = "📸️snapshot/🦀️.rs"]
pub mod snapshot;
//#endregion 🪢️TaxonomyMounts
