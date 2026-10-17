//! 🔨️ Internal glTF 2.0 static-pose geometry kernel.
//!
/// 🕸️ Topology summary, vertices, faces, and oriented edge incidences.
pub type MeshTopology = (Topology, Vec<V3>, Vec<[usize; 3]>, BTreeMap<(usize, usize), Vec<(usize, bool)>>);

/// 🧊 Hull surface area, volume, and oriented supporting planes.
pub type ConvexHullMetrics = (f64, f64, Vec<(V3, f64)>);

// 🚫️async: E1/R9 whole-file revert — every fn here is pure in-memory vector/matrix/mesh math
// over an already-decoded `GltfSnapshot` (verified zero `std::fs`/`tokio`/`reqwest`/`File::`/
// `TcpStream`/`spawn`/`sleep`/`SystemTime` across the file, same shape as the ticket's
// `terra-number-green` precedent). The blind universal-async codemod made all 44 fns `async fn`
// with zero suspension points; consumed by ~79 sibling inference files across the gltf tree, so
// R9 propagates outward from here rather than the other way around. Reverted 2026-08-20.

use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfAccessorType;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfComponentType;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfDecodedSnapshot;
use std::collections::{BTreeMap, BTreeSet};

use crate::standards::v2_0::subsets::any::schema::modules::{measurement_contracts::*, mesh_topology::Topology};

#[path = "🧮️vector-math-and-part-decoding/🦀️.rs"]
mod vector_math_and_part_decoding;
pub use vector_math_and_part_decoding::*;

#[path = "🕸️pair-geometry-hull-and-topology/🦀️.rs"]
mod pair_geometry_hull_and_topology;
pub use pair_geometry_hull_and_topology::*;

#[path = "🧊️shell-metrics-and-geometry-context/🦀️.rs"]
mod shell_metrics_and_geometry_context;
pub use shell_metrics_and_geometry_context::*;


