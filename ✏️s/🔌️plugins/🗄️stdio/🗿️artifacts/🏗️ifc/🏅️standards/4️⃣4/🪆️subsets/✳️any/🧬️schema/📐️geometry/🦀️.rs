//! 🏗️ Native IFC4 representation geometry entry points.
use super::snapshot::{IfcSnapshot,IfcEntity,IfcValue};
#[path="../../../../../../📐️geometry/🦀️.rs"]
mod shared;
#[path="📖️reader/🦀️.rs"]
mod reader;
pub use shared::boolean;
pub use shared::units;
pub use shared::preparation;
pub use shared::graph;
pub use reader::Ifc4GeometryGraph;
pub use shared::{GeometryMesh,GeometryIssue,GeometryStatus,GeometryControl,GeometryScene,GeometryLimits,GeometryProgress,CURVE_RELATIVE_TOLERANCE,CURVE_SEGMENTS};
#[cfg(test)]use shared::{cross,subtract,dot};
pub fn project(snapshot:&IfcSnapshot)->GeometryScene{shared::project(&reader::Ifc4GeometryGraph(snapshot))}
pub fn project_controlled(snapshot:&IfcSnapshot,limits:GeometryLimits,progress:&mut dyn FnMut(GeometryProgress)->GeometryControl)->GeometryScene{shared::project_controlled(&reader::Ifc4GeometryGraph(snapshot),limits,progress)}
pub fn project_with(snapshot:&IfcSnapshot,limits:GeometryLimits,progress:&mut dyn FnMut(GeometryProgress)->bool)->GeometryScene{shared::project_with(&reader::Ifc4GeometryGraph(snapshot),limits,progress)}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="🧪️tests/📏️units.rs"]mod unit_tests;
