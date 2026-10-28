//! 🧱️ Native IFC2x3 representation geometry entry points.
use super::snapshot::Ifc2x3Snapshot;
#[path="../../../../../../📐️geometry/🦀️.rs"]
mod shared;
#[path="📖️reader/🦀️.rs"]
mod reader;
pub use shared::boolean;
pub use shared::units;
pub use shared::preparation;
pub use shared::graph;
pub use reader::Ifc2x3GeometryGraph;
pub use shared::{GeometryMesh,GeometryIssue,GeometryStatus,GeometryControl,GeometryScene,GeometryLimits,GeometryProgress,CURVE_RELATIVE_TOLERANCE,CURVE_SEGMENTS};
pub fn project(snapshot:&Ifc2x3Snapshot)->GeometryScene{shared::project(&reader::Ifc2x3GeometryGraph(&snapshot.document))}
pub fn project_controlled(snapshot:&Ifc2x3Snapshot,limits:GeometryLimits,progress:&mut dyn FnMut(GeometryProgress)->GeometryControl)->GeometryScene{shared::project_controlled(&reader::Ifc2x3GeometryGraph(&snapshot.document),limits,progress)}
pub fn project_with(snapshot:&Ifc2x3Snapshot,limits:GeometryLimits,progress:&mut dyn FnMut(GeometryProgress)->bool)->GeometryScene{shared::project_with(&reader::Ifc2x3GeometryGraph(&snapshot.document),limits,progress)}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;

#[cfg(test)]
#[path="🧪️tests/📏️units.rs"]mod unit_tests;
