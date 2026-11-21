//! 🧠️ Live B-Rep topology, deterministic handles, and budgeted publication of native geometry.

use semio_framework_3d_brep_boolean::BooleanOp;
use semio_framework_3d_brep_construction::primitives::SolidPrimitive;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_kernel_contract::{BrepError, GeometryHandle, GeometryKind};
use semio_framework_3d_brep_mesh_io::PendingMutations;
use semio_framework_3d_brep_representation::arena::{ArenaId, EdgeId, FaceId, ShellId, SolidId, VertexId};
use semio_framework_3d_brep_representation::contract::Vec3 as EVec3;
use semio_framework_3d_brep_representation::topology::{Body, EntityRef, Wire};
use semio_framework_3d_brep_representation::topology::history::PersistentLabel;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::matrix::{Affine3, Frame3};
use semio_framework_3d_brep_vector::{Pnt3, Vec3 as NativeVec3};
use semio_framework_value::{ValueError, ValueRefusalKind};

#[path = "〰️analytic-publication/🦀️.rs"]
mod analytic_publication;
pub use analytic_publication::{analytic_arc_recipe, AnalyticCurveParameters, AnalyticPublication, AnalyticTransform};
pub use analytic_publication::primitive_handle;
pub use semio_framework_3d_brep_solid_transaction::{BoxFinish, CuboidSew, SolidGroupOwnership, SolidPatternRecipe, UnpublishedSolidPattern};

#[path = "🗃️session-state/🦀️.rs"]
mod session_state;
pub use session_state::{BrepRegistry, Entity, PrimitiveProvenance, SolidProvenance};

#[path = "🧾️conversions-and-registry/🦀️.rs"]
mod conversions_and_registry;
pub use conversions_and_registry::{entity_roots, entity_tag, evec, label_of_entity, pnt, require_direction, vec3};
pub use semio_framework_3d_brep_kernel_contract::{require_finite_scalar, require_finite_vector, require_positive_size, require_scale_factor};

#[path = "🔀️certified-boolean/🦀️.rs"]
mod certified_boolean;
pub use certified_boolean::{BooleanOperand, BooleanPreparation, CertifiedBooleanResult, BorePreviewSource, ShellDraft};

#[path = "🧾️topology-selection/🦀️.rs"]
mod topology_selection;
pub use topology_selection::{TopologyAlias, TopologyList, TopologySelection};

#[path = "📍️vertex-publication/🦀️.rs"]
mod vertex_publication;
pub use vertex_publication::VertexPublication;

#[path = "🛤️polygon-wire-publication/🦀️.rs"]
mod polygon_wire_publication;
pub use polygon_wire_publication::{HelixWireRecipe, PolygonWirePublication, PolygonWireRecipe};

#[path = "📐️polygon-face-publication/🦀️.rs"]
mod polygon_face_publication;
pub use polygon_face_publication::{PolygonFacePublication};

#[path = "🪷️conic-face-publication/🦀️.rs"]
mod conic_face_publication;
pub use conic_face_publication::{ConicFacePublication,PolygonSectionSet,SECTION_FACE_LIMIT,SectionPlan};

#[path = "🗜️prism-boundary/🦀️.rs"]
mod prism_boundary;
pub use prism_boundary::{PrismDraft};

#[path = "🔳️analytic-bounds/🦀️.rs"]
mod analytic_bounds;

#[path = "📍️certified-solid-queries/🦀️.rs"]
mod certified_solid_queries;

#[path = "↔️certified-offset/🦀️.rs"]
mod certified_offset;

#[path = "🥞️two-profile-loft/🦀️.rs"]
mod two_profile_loft;
pub use two_profile_loft::{LoftDraft, LoftPlan};

#[path = "🛤️straight-sweep/🦀️.rs"]
mod straight_sweep;

#[path = "🧱️planar-thickening/🦀️.rs"]
mod planar_thickening;
pub use planar_thickening::PlanarExtrusion;

#[path = "🔄️circular-revolution/🦀️.rs"]
mod circular_revolution;

#[path = "📦️point-hull/🦀️.rs"]
mod point_hull;
pub use point_hull::HullDraft;

#[path = "🧊️solid-publication/🦀️.rs"]
mod solid_publication;
pub use solid_publication::{SolidExchangeBoundary,SolidPublication};

#[path="🧺️compound-publication/🦀️.rs"]
mod compound_publication;
pub use compound_publication::CompoundPublication;
