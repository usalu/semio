//! 📄️ Canonical owned mesh document, topology, surface and conversion operations.

use semio_framework_test_fixtures::fixture_path;

extern crate semio_framework_value_derive as value_derive;
#[path = "../../🛞️inertia/🦀️.rs"]
pub mod inertia;
#[cfg(test)]
pub(crate) use semio_framework_3d_testing_support as test_support;

/// 📥️ Reads a committed language-agnostic fixture at run time, by its path relative to the mesh folder, so the test closure stays inside the source gate instead of embedding the fixtures with `include_str!`.
/// @see <https://doc.rust-lang.org/std/macro.include_str.html>
#[cfg(test)]
pub(crate) fn fixture_json(relative: &str) -> serde_json::Value {
    let path = fixture_path!("../../..").join(relative);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()))
}
// 🔬️ `serde`/`serde_json` survive ONLY as a `#[cfg(test)]` differential oracle now that these
// types have their own first-party `ToValue`/`FromValue` codec — never a production dependency
// of this crate. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[cfg(test)]
use serde::{Deserialize, Serialize};
use std::collections::{HashMap};
#[cfg(test)] use std::collections::{BTreeMap};
#[cfg(test)]
use std::collections::HashSet;

#[path = "../🔎️quality/🦀️.rs"]
mod quality;
pub use quality::{analyze_polygon_soup, MeshBounds, MeshMassProperties, MeshQualityReport, ScalarStats};

#[path = "../🎨️surface/🦀️.rs"]
mod surface;
pub use surface::MeshSurfaceJob;


#[path = "📝️journal/🦀️.rs"]
mod journal;
pub use journal::{ArenaJournal, MeshJournal, MeshOverlay};

#[path = "🔍️structure/🦀️.rs"]
mod structure;
/// 🌉️ Encodes original ordered UV membership as the declared array of vertex ids.
mod u32_membership_bridge {
    pub fn to_value(set: &super::HistoryFoldSet<u32>) -> semio_framework_value::DslValue {
        let mut ids: Vec<&u32> = set.iter().collect();
        ids.sort_unstable();
        semio_framework_value::DslValue::Array(ids.into_iter().map(semio_framework_value::ToValue::to_value).collect())
    }
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<super::HistoryFoldSet<u32>, semio_framework_value::ValueError> {
        <Vec<u32> as semio_framework_value::FromValue>::from_value(value).map(|items| items.into_iter().collect())
    }
}

#[cfg(test)]
mod attribute_serde_bridge {
    use semio_framework_value::{FromValue, ToValue};
    use serde::{Deserialize, Serialize};
    pub fn serialize<T:ToValue,S: serde::Serializer>(value: &T, serializer: S) -> Result<S::Ok,S::Error> {
        serde_json::Value::from(value.to_value()).serialize(serializer)
    }
    pub fn deserialize<'de,T:FromValue,D:serde::Deserializer<'de>>(deserializer:D)->Result<T,D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        T::from_value(semio_framework_value::DslValue::from(value)).map_err(serde::de::Error::custom)
    }
}

//#region Types

pub use semio_framework_3d_mesh_contract::{MeshModelingProgress,Vec3,VertexId,HalfEdgeId,FaceId,EdgeId,WeldMode,MirrorAxis,MeshKernelError,MeshResult};
pub use semio_framework_mesh_engine::{HistoryFoldIndex,HistoryFoldSet, MeshAttribute, MeshAttributeDomain, MeshAttributeSemantic, MeshAttributeInterpolation, MeshTexture};

#[path = "🧱️mesh-records/🦀️.rs"]
mod mesh_records;
pub use mesh_records::{HalfEdge, HalfedgeMesh, MeshFace, MeshTransfer, MeshVertex};

#[path = "🕸️halfedge-mesh-operations/🦀️.rs"]
mod halfedge_mesh_operations;

#[path = "🔺️polygon-triangulation/🦀️.rs"]
mod polygon_triangulation;
use polygon_triangulation::{Vec3f64, cross2, normalize3, plane_basis, point_in_triangle, triangulate_indexed_polygon_with_holes};
pub use polygon_triangulation::{cross3, dot3, length3, newell_normal, sub3, triangulate_polygon};

#[path = "⏱️tessellation-job-and-obj-export/🦀️.rs"]
mod tessellation_job_and_obj_export;
pub use tessellation_job_and_obj_export::{MeshObjExport, MeshTessellationJob, MeshTessellationStep};

//#endregion Types

//#region Primitives

//#endregion Primitives

//#region Export

#[path = "../🧩️tessellation/🎟️reservation/🦀️.rs"]
mod tessellation_buffer_reservation;

#[path = "../🧩️tessellation/🎨️metadata/🦀️.rs"]
mod tessellation_metadata_capture;

#[path = "../🧩️tessellation/📤️corner/🦀️.rs"]
mod tessellation_corner_emission;

#[path = "../🧩️tessellation/📤️edge/🦀️.rs"]
mod tessellation_edge_emission;

#[path = "../🧩️tessellation/📤️triangle/🦀️.rs"]
mod tessellation_triangle_emission;

#[path = "../🧩️tessellation/🎨️projection/🦀️.rs"]
mod tessellation_attribute_projection;

#[path = "../🧩️tessellation/🧭️face/🦀️.rs"]
mod tessellation_face_preparation;

#[path = "../🧩️tessellation/⏱️normal/🦀️.rs"]
mod tessellation_normal_work;

//#endregion Export

//#region Tests

//#endregion Tests

#[cfg(test)]
#[path = "../🧪️tests/🔬️tessellation/🎟️reservation/🦀️.rs"]
mod tessellation_reservation_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️tessellation/🎨️metadata/🦀️.rs"]
mod tessellation_metadata_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️tessellation/📤️corner/🦀️.rs"]
mod tessellation_corner_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️tessellation/📤️edge/🦀️.rs"]
mod tessellation_edge_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️tessellation/📤️triangle/🦀️.rs"]
mod tessellation_triangle_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️tessellation/🎨️projection/🦀️.rs"]
mod tessellation_projection_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️tessellation/🧭️face/🦀️.rs"]
mod tessellation_face_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️tessellation/⏱️normal/🦀️.rs"]
mod tessellation_normal_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️tessellation/⏱️polygons/🦀️.rs"]
mod tessellation_polygon_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️tessellation/🎨️channels/🦀️.rs"]
mod tessellation_channel_tests;

/// 🧭 A budgeted modeling result; completed geometry is transferred exactly once.
#[derive(Debug)]
pub enum MeshModelingStep {
    Working(MeshModelingProgress),
    Done(HalfedgeMesh),
    Cancelled(MeshModelingProgress),
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod codec_tests;
#[cfg(test)]
#[path = "../🧪️tests/🔬️tessellation/📦️ownership/🦀️.rs"]
mod ownership_tests;
