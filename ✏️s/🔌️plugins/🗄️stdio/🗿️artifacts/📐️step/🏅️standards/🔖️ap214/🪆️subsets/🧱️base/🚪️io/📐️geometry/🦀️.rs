//! 📐️ STEP Part-21 geometry codec consuming the general computational representation.
/// 🚨️ Hand-rolled ISO 10303-21 (STEP) reader/writer failure modes.
#[derive(Clone, Debug, PartialEq)]
pub enum StepError {
    /// 🚨️ The Part-21 lexer/parser rejected the input text.
    Syntax(String),
    /// 🚨️ An instance reference (`#123`) does not resolve to any parsed entity.
    UnresolvedReference(u64),
    /// 🚨️ An entity type is recognized but not translatable by this subset reader.
    Unsupported(String),
}

impl std::fmt::Display for StepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StepError::Syntax(msg) => write!(f, "syntax error: {msg}"),
            StepError::UnresolvedReference(id) => write!(f, "unresolved reference #{id}"),
            StepError::Unsupported(name) => write!(f, "unsupported entity: {name}"),
        }
    }
}

/// 🪢 Degree, control-point identifiers, multiplicities, and knots.
pub type BsplineCurveAttributes = (usize, Vec<u64>, Vec<u32>, Vec<f64>);

use std::collections::HashMap;
use std::fmt::Write as _;

use {semio_framework_3d_brep_construction::euler::{add_face, add_shell, add_solid, make_edge, make_vertex}, semio_framework_3d_brep_representation::topology::make_loop};
use semio_framework_3d_brep_representation::arena::{ArenaId, Curve3Id, EdgeId, FaceId, SolidId, SurfaceId, VertexId};
use semio_framework_3d_brep_curve::bspline::KnotVector;
use semio_framework_3d_brep_curve::Curve3;

use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_representation::topology::history::OpRecorder;
use semio_framework_3d_brep_representation::topology::Body;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::{Pnt3, Vec3};

// #region 🔖️Api

/// 📄 Serializes one or more manifold solids from `body` to STEP Part 21 (AP203 subset).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_step(body: &Body, solids: &[SolidId]) -> Result<String, StepError> {
    if solids.is_empty() {
        return Err(StepError::Syntax("no solids to export".to_string()));
    }
    let mut ctx = StepWriteContext::new();
    let repr_context_id = ctx.write_geometric_context();
    let product_ids = ctx.write_product_structure();
    let mut brep_ids = Vec::new();
    for &solid_id in solids {
        brep_ids.push(ctx.write_solid(body, solid_id)?);
    }
    let items: Vec<String> = brep_ids.iter().map(|id| format!("#{id}")).collect();
    let shape_repr_id = ctx.next_id();
    ctx.write_entity(shape_repr_id, "ADVANCED_BREP_SHAPE_REPRESENTATION", &format!("'semio export', ({},), #{repr_context_id})", items.join(", ")));
    let prod_def_shape_id = ctx.next_id();
    ctx.write_entity(prod_def_shape_id, "PRODUCT_DEFINITION_SHAPE", &format!("'','',#{})", product_ids.definition));
    let shape_def_repr_id = ctx.next_id();
    ctx.write_entity(shape_def_repr_id, "SHAPE_DEFINITION_REPRESENTATION", &format!("#{prod_def_shape_id}, #{shape_repr_id})"));
    Ok(ctx.finish())
}

/// 📄 Parses STEP Part 21 text into a fresh [`Body`] containing every `MANIFOLD_SOLID_BREP` found.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_step(text: &str) -> Result<Body, StepError> {
    let entities = parse_step_entities(text)?;
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let mut builder = StepBuilder::new(&mut body, &entities, &mut rec);
    builder.build_all_solids()?;
    Ok(body)
}

// #endregion 🔖️Api

#[path = "✍️writer/🦀️.rs"]
mod writer;
use writer::*;

#[path = "📖️entity-parsing/🦀️.rs"]
mod entity_parsing;
use entity_parsing::*;

#[path = "🏗️solid-builder/🦀️.rs"]
mod solid_builder;
use solid_builder::*;

// #region 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests

/// 📤️ STEP format-owned geometry writer over the general mathematical engine.
pub fn export_step(kernel: &semio_framework_3d_brep_engine::Brep, shapes: &[semio_framework_3d_brep_engine::GeometryHandle]) -> Result<String,semio_framework_3d_brep_engine::BrepError> {
    let solids = shapes.iter().map(|shape| kernel.geometry_solid(shape)).collect::<Result<Vec<_>,_>>()?;
    write_step(kernel.representation(),&solids).map_err(|error| semio_framework_3d_brep_engine::BrepError::Operation(error.to_string()))
}
/// 📥️ STEP format-owned reader preserves previously issued geometry handles.
pub fn import_step(kernel: &mut semio_framework_3d_brep_engine::Brep, text: &str) -> Result<Vec<semio_framework_3d_brep_engine::GeometryHandle>,semio_framework_3d_brep_engine::BrepError> {
    let body = read_step(text).map_err(|error| semio_framework_3d_brep_engine::BrepError::Operation(error.to_string()))?;
    Ok(kernel.merge_representation(body))
}
/// 📤️ Owner-supplied STEP solid export codec.
pub struct StepSolidExporter;
/// 📥️ Owner-supplied STEP solid import codec.
pub struct StepSolidImporter;
impl semio_framework_3d_brep_engine::SolidExporter for StepSolidExporter {
    async fn format_kind(&self) -> &'static str { "step" }
    async fn export(&self,kernel:&semio_framework_3d_brep_engine::Brep,shapes:&[semio_framework_3d_brep_engine::GeometryHandle],_:f64) -> Result<Vec<u8>,semio_framework_3d_brep_engine::BrepError> { export_step(kernel,shapes).map(String::into_bytes) }
}
impl semio_framework_3d_brep_engine::SolidImporter for StepSolidImporter {
    async fn format_kind(&self) -> &'static str { "step" }
    async fn import(&self,kernel:&mut semio_framework_3d_brep_engine::Brep,bytes:&[u8],_:f64) -> Result<Vec<semio_framework_3d_brep_engine::GeometryHandle>,semio_framework_3d_brep_engine::BrepError> {
        let text=std::str::from_utf8(bytes).map_err(|error|semio_framework_3d_brep_engine::BrepError::InvalidInput(error.to_string()))?;
        import_step(kernel,text)
    }
}

#[path = "🌊️session/🦀️.rs"]
pub mod session;

/// 📐️ Exact STEP geometry operation identities owned by this codec.
pub const STEP_GEOMETRY_OPERATIONS: &[&str] = &["export_step","import_step"];
/// 📏️ Format fidelity for the closed STEP geometry operation table.
pub fn operation_quality(method:&str) -> semio_framework_3d_brep_representation::contract::OpQuality {
    if STEP_GEOMETRY_OPERATIONS.contains(&method) { semio_framework_3d_brep_representation::contract::OpQuality::ExactAnalytic } else { semio_framework_3d_brep_representation::contract::OpQuality::Unsupported }
}
