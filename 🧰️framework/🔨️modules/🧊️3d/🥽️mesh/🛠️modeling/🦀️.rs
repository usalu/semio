//! 🛠️ Retained modeling work, measured in vertices, corners, and candidate transitions.

use super::*;
use semio_framework_3d_mesh_reconstruction::{MeshSourceSnapshot as Snapshot,MeshTopologyBuild as Build,MeshPolygonNormal as Normal,MeshSourceRemap as SourceAttributeRemap,MeshMergeJob as Merge,MeshImportJob as Import,MeshMergeIntent,MeshPrimitiveKind};

use semio_framework_3d_mesh_contract::{MeshPolygonSoup as Soup,MeshSourceVertex as SourceVertex,MeshRankedEdge as Ranked};
use semio_framework_3d_mesh_clipping::MeshClipJob;
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_3d_brep_vector::{Pnt3, Vec3 as GeometryVector};
use semio_framework_3d_brep_vector::matrix::{Affine3, Mat3};
use std::cmp::Reverse;
use std::collections::{BinaryHeap,HashMap};
use semio_framework_mesh_engine::HistoryFoldIndex;

type Vec3f64 = (f64, f64, f64);

#[cfg(feature="mesh-test-observation")]
#[path="🔬️inspection/🦀️.rs"]
pub(super) mod inspection;

#[path = "📥️input/🦀️.rs"]
mod input;
pub use input::{MeshTranslationSource,MeshTranslationInput,MeshPlaneProduction,MeshTranslationProduction};

#[path = "🧵️modeling-job/🦀️.rs"]
mod modeling_job;
use modeling_job::{Work, close_tessellation_handoff};
pub use modeling_job::MeshModelingJob;

#[path = "🧰️modeling-operations/🦀️.rs"]
mod modeling_operations;
use modeling_operations::MeshModelingInternals;
pub use modeling_operations::MeshModelingOps;

#[path = "🔄️transform-and-knife/🦀️.rs"]
mod transform_and_knife;
use transform_and_knife::{Knife, PositionPolicy, Transform, TransformSelection};

#[path = "🎈️inflate-orient-and-fill-holes/🦀️.rs"]
mod inflate_orient_and_fill_holes;
use inflate_orient_and_fill_holes::{FillHoles, Inflate, Orient};

#[path = "🪢️loop-cut-mirror-and-subdivide/🦀️.rs"]
mod loop_cut_mirror_and_subdivide;
use loop_cut_mirror_and_subdivide::{LoopCut, Mirror, Subdivide};

#[path = "📉️bevel-and-decimate/🦀️.rs"]
mod bevel_and_decimate;
use bevel_and_decimate::{Bevel, Candidate, Decimate};

impl semio_framework_value::retirement::RetireOwned for TransformSelection {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(Vec<VertexId>,Vec<EdgeId>,Vec<FaceId>,HistoryFoldSet<usize>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.vertices,self.edges,self.faces,self.selected)}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Transform {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(HalfedgeMesh,Vec<Option<Vec3>>,Vec<Vec3>,Vec<u32>,Vec<String>,Option<TransformSelection>,Vec<semio_framework_value::DslValue>,Vec<u32>,HistoryFoldIndex<(u32,bool),u32>,Vec<semio_framework_value::DslValue>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.mesh,self.flat,self.smooth,self.hes,self.attribute_names,self.selection,self.attribute_values,self.attribute_indices,self.attribute_samples,self.attribute_retired)}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Knife {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(Snapshot,Vec<f64>,Option<MeshTessellationJob>,Option<semio_framework_value::retirement::controlled::ControlledRetirement<MeshTessellationJob>>,Option<MeshTransfer>,Vec<Vec<u32>>,Vec<Vec<u32>>,HistoryFoldIndex<(u32,u32),u32>,Soup,Vec<u32>,Option<Build>,Vec<SourceVertex>,Vec<u32>,Option<SourceAttributeRemap>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.snapshot,self.distances,self.tessellation,self.tessellation_retirement,self.transfer,self.pieces,self.split,self.intersections,self.soup,self.corners,self.build,self.vertex_sources,self.source_faces,self.attributes)}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for Inflate {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(Snapshot,Vec<FaceId>,HistoryFoldSet<u32>,HistoryFoldIndex<u32,Vec3>,HistoryFoldIndex<(u32,u32),(u32,u32,usize,u32)>,HistoryFoldSet<(u32,u32)>,HistoryFoldIndex<u32,u32>,Soup,Vec<Vec<u32>>,Vec<u32>,Vec<u32>,Option<Build>,Vec<SourceVertex>,Vec<u32>,Vec<u32>,Option<SourceAttributeRemap>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.snapshot,self.input,self.selected,self.normals,self.boundary,self.edges,self.displaced,self.soup,self.borders,self.inner,self.corners,self.build,self.vertex_sources,self.source_faces,self.border_faces,self.attributes)}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Orient {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(Option<Vec<FaceId>>,Snapshot,Option<SourceAttributeRemap>,HistoryFoldIndex<(u32,u32),Vec<(usize,bool)>>,Vec<(u32,u32)>,Vec<Vec<(usize,bool)>>,Vec<bool>,Vec<bool>,Vec<usize>,Option<Build>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.input,self.snapshot,self.attributes,self.edges,self.keys,self.adjacency,self.oriented,self.flipped,self.stack,self.build)}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for FillHoles {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(Snapshot,HistoryFoldSet<u32>,Vec<u32>,Vec<u32>,Vec<u32>,Vec<u32>,Vec<Option<u32>>,Vec<Option<u32>>,Vec<Vec<u32>>,Option<SourceAttributeRemap>,Option<Build>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.snapshot,self.visited,self.vertices,self.boundary_edges,self.cap,self.cap_faces,self.corner_sources,self.edge_sources,self.face_sources,self.attributes,self.build)}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for LoopCut {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(Snapshot,Vec<EdgeId>,HistoryFoldIndex<(u32,u32),Vec<(usize,usize)>>,HistoryFoldSet<(u32,u32)>,Vec<(u32,u32)>,HistoryFoldIndex<(u32,u32),Vec<u32>>,Vec<u32>,Soup,Vec<u32>,Vec<u32>,Option<Build>,Vec<SourceVertex>,Vec<u32>,Option<SourceAttributeRemap>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.snapshot,self.edges,self.incidence,self.marked,self.pending,self.cut_vertices,self.cut_ids,self.soup,self.boundary,self.grid,self.build,self.vertex_sources,self.source_faces,self.attributes)}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Mirror {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(Snapshot,Vec<u32>,Vec<u32>,Vec<bool>,Vec<u32>,Option<SourceAttributeRemap>,Vec<u32>,Soup,Vec<u32>,Vec<u32>,Option<Build>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.snapshot,self.source_vertices,self.source_faces,self.mirrored_faces,self.attribute_vertices,self.attributes,self.reflected,self.soup,self.original,self.mirrored,self.build)}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Subdivide {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(Snapshot,Vec<FaceId>,HistoryFoldSet<u32>,HistoryFoldSet<u32>,Option<MeshTessellationJob>,Option<semio_framework_value::retirement::controlled::ControlledRetirement<MeshTessellationJob>>,Option<MeshTransfer>,Soup,Vec<u32>,Option<Build>,Vec<SourceVertex>,Vec<u32>,Option<SourceAttributeRemap>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.snapshot,self.input,self.selected,self.concave,self.tessellation,self.tessellation_retirement,self.transfer,self.soup,self.corners,self.build,self.vertex_sources,self.source_faces,self.attributes)}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for Bevel {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(Snapshot,Vec<EdgeId>,HistoryFoldSet<u32>,Vec<(Vec3, f32)>,Option<MeshClipJob>,Option<Build>,Option<SourceAttributeRemap>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.snapshot,self.edges,self.selected,self.planes,self.clip,self.build,self.attributes)}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Decimate {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(Snapshot,HistoryFoldSet<(u32, u32)>,BinaryHeap<Reverse<Ranked>>,Option<Candidate>,Option<Build>,Vec<u32>,Vec<(u32,u32)>,Option<SourceAttributeRemap>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.snapshot,self.edges,self.ranked,self.candidate,self.build,self.source_faces,self.edge_sources,self.attributes)}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Candidate {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(Soup,Vec<u32>,Vec<(u32,u32)>,Vec<(u32,u32)>,Vec<u32>,HistoryFoldIndex<u32, usize>,HistoryFoldIndex<(u32, u32), (usize, i32)>,HistoryFoldIndex<u32, u32>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.soup,self.corners,self.corner_sources,self.edge_sources,self.face_sources,self.seen,self.incidence,self.links)}
    fn controlled_retirement_supported()->bool {true}
}
