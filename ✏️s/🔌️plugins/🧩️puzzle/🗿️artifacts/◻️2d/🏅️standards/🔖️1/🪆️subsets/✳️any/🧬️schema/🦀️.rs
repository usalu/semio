//! 🧬️ Puzzle2d artifact schema — every field of the artifact with its state class.

use semio_framework_value::{list::PagedList, paged::PagedUtf8};

use ::semio_framework_schema::ArtifactSchema;

pub use semio_s_artifact_puzzle_2d_graph_manifest::standards::v1::subsets::any::schema::graph_manifest;

//#region 🔖️Artifact
/// 🧬️ puzzle2d document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle2d")]
pub struct Puzzle2dArtifact {
    #[state(artifact)]
    pub schema: PagedUtf8<{ usize::MAX }>,
    #[state(artifact)]
    pub camera: Puzzle2dCamera,
    #[state(artifact)]
    pub nodes: PagedList<Puzzle2dNode, { usize::MAX }>,
    #[state(artifact)]
    pub edges: PagedList<Puzzle2dEdge, { usize::MAX }>,
    #[state(artifact)]
    pub target_regions: PagedList<Puzzle2dTargetRegion, { usize::MAX }>,
    #[state(artifact)]
    pub meta: Puzzle2dMeta,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Puzzle2dArtifact {
    fn default() -> Self {
        Self::from_snapshot(Puzzle2dSnapshot::default())
    }
}

impl Puzzle2dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Puzzle2dSnapshot {
        Puzzle2dSnapshot { schema: self.schema.clone(), camera: self.camera.clone(), nodes: self.nodes.clone(), edges: self.edges.clone(), target_regions: self.target_regions.clone(), meta: self.meta.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Puzzle2dSnapshot) -> Self {
        Self { schema: snapshot.schema, camera: snapshot.camera, nodes: snapshot.nodes, edges: snapshot.edges, target_regions: snapshot.target_regions, meta: snapshot.meta }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Puzzle2dSnapshot) {
        self.schema = snapshot.schema;
        self.camera = snapshot.camera;
        self.nodes = snapshot.nodes;
        self.edges = snapshot.edges;
        self.target_regions = snapshot.target_regions;
        self.meta = snapshot.meta;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.puzzle.puzzle2d` — twenty handcrafted schema leaves.
pub fn puzzle2d_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.puzzle.puzzle2d",
        artifact: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 📄️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1e): a pure default-snapshot constructor over document types, no `AppIo`/app dependency.
pub fn empty_puzzle2d_snapshot() -> Puzzle2dSnapshot {
    Puzzle2dSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::records;

pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::lookup;
pub use records::*;

use crate::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
use {semio_framework_value::paged::PagedUtf8};

/// 📍️ The board position of the handle `handle_id` on `document`: its node's rim point at the handle's angle, the
/// same rim geometry the board engine draws with and the editor's proximity search measures
/// (`puzzle2d_handle_world_position`) — a circle's east-zero angle on the node's radius, a rectangle's north-zero
/// angle on its outline. `None` when no node carries the handle.
pub fn puzzle2d_handle_position(document: &Puzzle2dSnapshot, handle_id: &PagedUtf8<{ usize::MAX }>) -> Option<(f64, f64)> {
    let (node, handle) = document.nodes.iter().find_map(|node| node.handles.iter().find(|handle| &handle.id == handle_id).map(|handle| (node, handle)))?;
    let centre = semio_framework_geometry::Point::new(node.x, node.y);
    let point = if node.shape.as_ref().is_some_and(|shape| shape.eq_str("rectangle")) {
        semio_framework_graph_drawing::routing::handle_position_on_rectangle(centre, node.width.unwrap_or(48.0), node.height.unwrap_or(48.0), handle.angle)
    } else {
        semio_framework_graph_drawing::routing::handle_position_on_circle(centre, node.radius.unwrap_or(24.0), handle.angle)
    };
    Some((point.x, point.y))
}

/// 📏️ The board distance between the handles `source` and `target` of `document` — what a recorded proximity
/// tolerance is measured against. `None` when either handle is on no node.
pub fn puzzle2d_handle_distance(document: &Puzzle2dSnapshot, source: &PagedUtf8<{ usize::MAX }>, target: &PagedUtf8<{ usize::MAX }>) -> Option<f64> {
    let ((source_x, source_y), (target_x, target_y)) = (puzzle2d_handle_position(document, source)?, puzzle2d_handle_position(document, target)?);
    Some((target_x - source_x).hypot(target_y - source_y))
}
