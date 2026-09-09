//! 🌐️ `stdio.semio` artifact root — the inbuilt semio semantic artifact (standard `v1`, 18
//! schema-owning domain subsets + the `✉️base` envelope union). See the master plan's
//! "Architecture > The semio artifact" section.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_graph as graph_core;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use standards::v1::subsets::base::schema::diff::SemioDiff;
pub use standards::v1::subsets::base::schema::mutations::SemioMutation;
pub use standards::v1::subsets::base::schema::snapshot::SemioSnapshot;
pub use standards::v1::subsets::base::schema::SemioArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_SEMIO_DOCUMENT_SCHEMA: &str = "stdio.semio";

/// 🧬️ Artifact schema descriptor id.
pub const SEMIO_ARTIFACT_SCHEMA_ID: &str = "s.stdio.semio";

/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::definition_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    Vec::new()
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { identity: "semio", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::definition_only_assembly("semio", definition()?)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.semio".into(),
        name: "Semio".into(),
        source_format: STDIO_SEMIO_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_SEMIO_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🗂️ Registers all 19 of `v1`'s subsets' IO composers (18 domain subsets + `✉️base`
/// itself) — dissolved out of the former standard-level `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES). `semio` is one of stdio's 10
/// deliberate imperative-`register()` artifacts (never converted to the `ArtifactDeclaration`
/// builder pattern). The Semio package contribution invokes it directly with the established call
/// order and behavior.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    subsets::brep::io::register();
    subsets::mesh::io::register();
    subsets::model::io::register();
    subsets::value::io::register();
    subsets::document::io::register();
    subsets::cad::io::register();
    subsets::drawing::io::register();
    subsets::image::io::register();
    subsets::video::io::register();
    subsets::audio::io::register();
    subsets::animation::io::register();
    subsets::presentation::io::register();
    subsets::flow::io::register();
    subsets::text::io::register();
    subsets::table::io::register();
    subsets::graph::io::register();
    subsets::object::io::register();
    subsets::kit::io::register();
    subsets::base::io::register();
}
//#endregion 🔖️Register

//#region 🔖️Members
/// 🧸️ The subset table, written once and expanded into both the `SemioMembers` enum and the
/// subset-name list, so a new subset cannot be added to one and forgotten in the other.
macro_rules! semio_subset_table {
    ($macro_name:ident) => {
        $macro_name! {
            animation => animation, SemioAnimationSnapshot, SemioAnimationMutation;
            audio => audio, SemioAudioSnapshot, SemioAudioMutation;
            brep => brep, SemioBrepSnapshot, SemioBrepMutation;
            cad => cad, SemioCadSnapshot, SemioCadMutation;
            document => document, SemioDocumentSnapshot, SemioDocumentMutation;
            drawing => drawing, SemioDrawingSnapshot, SemioDrawingMutation;
            flow => flow, SemioFlowSnapshot, SemioFlowMutation;
            graph => graph, SemioGraphSnapshot, SemioGraphMutation;
            image => image, SemioImageSnapshot, SemioImageMutation;
            kit => kit, SemioKitSnapshot, SemioKitMutation;
            mesh => mesh, SemioMeshSnapshot, SemioMeshMutation;
            model => model, SemioModelSnapshot, SemioModelMutation;
            object => object, SemioObjectSnapshot, SemioObjectMutation;
            presentation => presentation, SemioPresentationSnapshot, SemioPresentationMutation;
            table => table, SemioTableSnapshot, SemioTableMutation;
            text => text, SemioTextSnapshot, SemioTextMutation;
            value => value, SemioValueSnapshot, SemioValueMutation;
            video => video, SemioVideoSnapshot, SemioVideoMutation;
        }
    };
}

/// 🧸️ Every subset name this artifact can materialize a child as.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn composable_subsets() -> Vec<&'static str> {
    macro_rules! subset_names {
        ($($name:ident => $module:ident, $snapshot:ident, $mutation:ident);* $(;)?) => { vec![$(stringify!($name)),*] };
    }
    semio_subset_table!(subset_names)
}

use crate::standards::v1::subsets;

//#region 🧹️SnapshotRetirement
use std::{marker::PhantomData, sync::Arc};

use dsl::os_store::retirement::{owned_retirement, shared_retirement, RetireOwned, RetirementCursor};
use dsl::{artifact_retire_leaf as retire_leaf, artifact_retire_struct as retire_struct, artifact_retirement_sequence as seq};

use subsets::base::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform, SemioUv};
use subsets::{
    animation::schema::mutations as animation_mutation, audio::schema::mutations as audio_mutation, brep::schema::mutations as brep_mutation, cad::schema::mutations as cad_mutation, document::schema::mutations as document_mutation,
    drawing::schema::mutations as drawing_mutation, flow::schema::mutations as flow_mutation, graph::schema::mutations as graph_mutation, image::schema::mutations as image_mutation, kit::schema::mutations as kit_mutation,
    mesh::schema::mutations as mesh_mutation, model::schema::mutations as model_mutation, object::schema::mutations as object_mutation, presentation::schema::mutations as presentation_mutation, table::schema::mutations as table_mutation,
    text::schema::mutations as text_mutation, value::schema::mutations as value_mutation, video::schema::mutations as video_mutation,
};

macro_rules! semio_snapshot_open {
    (flow, $snapshot:ty) => {
        type SnapshotOpen = subsets::flow::schema::snapshot::binary::SemioFlowSnapshotDecode;
    };
    ($module:ident, $snapshot:ty) => {
        type SnapshotOpen = dsl::UnsupportedMemberSnapshotOpen<$snapshot>;
    };
}
use subsets::{
    animation::schema::snapshot as animation, audio::schema::snapshot as audio, brep::schema::snapshot as brep, cad::schema::snapshot as cad, document::schema::snapshot as document, drawing::schema::snapshot as drawing,
    flow::schema::snapshot as flow, graph::schema::snapshot as graph, image::schema::snapshot as image, kit::schema::snapshot as kit, mesh::schema::snapshot as mesh, model::schema::snapshot as model, object::schema::snapshot as object,
    presentation::schema::snapshot as presentation, table::schema::snapshot as table, text::schema::snapshot as text, value::schema::snapshot as value, video::schema::snapshot as video,
};

retire_leaf!(
    SemioPoint2,
    SemioPoint3,
    SemioQuaternion,
    SemioRgba,
    SemioTransform,
    SemioUv,
    animation::AnimInterpolation,
    audio::SemioAudioFormat,
    drawing::DrawCanvas,
    graph::SemioGraphPortKind,
    image::SemioColorspace,
    mesh::SemioTopology,
    model::SpatialKind,
    presentation::SlideFrame,
    table::SemioTableCellKind,
    text::SemioTextMarkKind,
    video::SemioRational,
    video::SemioVideoStreamKind,
);

retire_struct!(animation::AnimTarget { node, property });
retire_struct!(animation::AnimKeyframe { t, value });
retire_struct!(animation::AnimChannel { target, interpolation, keyframes });
retire_struct!(animation::AnimTimeline { name, channels });
retire_struct!(animation::SemioAnimationSnapshot { schema, timelines });
impl RetireOwned for animation::AnimTargetProperty {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Custom { name } => name.retirement(),
            _ => seq![],
        }
    }
}
impl RetireOwned for animation::AnimValue {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Scalar { value } => value.retirement(),
            Self::Vec3 { value } => value.retirement(),
            Self::Quat { value } => value.retirement(),
            Self::Weights { values } => values.retirement(),
        }
    }
}

retire_struct!(audio::SemioAudioChannel { samples });
retire_struct!(audio::SemioAudioTag { key, value });
retire_struct!(audio::SemioAudioSnapshot { schema, sample_rate, format, channels, tags });

retire_struct!(brep::BrepVertex { id, point, tol });
retire_struct!(brep::BrepEdge { id, start_vertex, end_vertex, curve, tol });
retire_struct!(brep::BrepLoopEdge { edge, orientation });
retire_struct!(brep::BrepLoop { id, edges });
retire_struct!(brep::BrepFace { id, outer_loop, inner_loops, surface, orientation, tol });
retire_struct!(brep::BrepShellFace { face, orientation });
retire_struct!(brep::BrepShell { id, faces });
retire_struct!(brep::BrepSolidShell { shell, is_void });
retire_struct!(brep::BrepSolid { id, shells });
retire_struct!(brep::BrepCoedge { id, edge, forward, pcurve, prange, loop_id, next, prev });
retire_struct!(brep::SemioBrepSnapshot { schema, vertices, edges, loops, faces, shells, solids, coedges, next_label });
impl RetireOwned for brep::BrepCurve2 {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Line { origin, direction } => seq![origin, direction],
            Self::Circle { center, radius } => seq![center, radius],
            Self::Ellipse { center, x_axis, radius_major, radius_minor } => seq![center, x_axis, radius_major, radius_minor],
            Self::Nurbs { control_points, weights, degree, knots } => seq![control_points, weights, degree, knots],
        }
    }
}
impl RetireOwned for brep::BrepCurve {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Line { origin, direction } => seq![origin, direction],
            Self::Circle { center, axis, radius } => seq![center, axis, radius],
            Self::Ellipse { center, axis, radius_major, radius_minor } => seq![center, axis, radius_major, radius_minor],
            Self::Nurbs { control_points, weights, degree, knots } => seq![control_points, weights, degree, knots],
        }
    }
}
impl RetireOwned for brep::BrepSurface {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Plane { origin, normal } => seq![origin, normal],
            Self::Cylinder { origin, axis, radius } => seq![origin, axis, radius],
            Self::Cone { origin, axis, radius, half_angle } => seq![origin, axis, radius, half_angle],
            Self::Sphere { center, radius } => seq![center, radius],
            Self::Torus { center, axis, major_radius, minor_radius } => seq![center, axis, major_radius, minor_radius],
            Self::Nurbs { control_points, weights, u_count, v_count, degree_u, degree_v, knots_u, knots_v } => seq![control_points, weights, u_count, v_count, degree_u, degree_v, knots_u, knots_v],
        }
    }
}

retire_struct!(cad::CadLayer { name, color_index, line_type, visible });
retire_struct!(cad::CadEntityRecord { handle, layer, entity });
retire_struct!(cad::CadBlock { name, base_point, entities });
retire_struct!(cad::SemioCadSnapshot { schema, layers, blocks, entities });
impl RetireOwned for cad::CadEntity {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Line { a, b } => seq![a, b],
            Self::Arc { center, radius, start_angle, end_angle } => seq![center, radius, start_angle, end_angle],
            Self::Circle { center, radius } => seq![center, radius],
            Self::Ellipse { center, major_axis_end, ratio, start_param, end_param } => seq![center, major_axis_end, ratio, start_param, end_param],
            Self::Polyline { vertices, closed } => seq![vertices, closed],
            Self::Text { position, height, rotation, content } => seq![position, height, rotation, content],
            Self::Insert { block_name, insertion_point, scale, rotation } => seq![block_name, insertion_point, scale, rotation],
            Self::Solid { p1, p2, p3, p4 } => seq![p1, p2, p3, p4],
            Self::Dimension { def_point, text_position, measurement, text } => seq![def_point, text_position, measurement, text],
        }
    }
}

retire_struct!(document::RunStyle { bold, italic, underline, size, font, color, link });
retire_struct!(document::DocRun { text, style });
retire_struct!(document::DocStyle { id, name, based_on });
retire_struct!(document::DocImage { id, mime, bytes });
retire_struct!(document::DocListItem { blocks });
retire_struct!(document::DocTableCell { blocks });
retire_struct!(document::DocTableRow { cells });
retire_struct!(document::SemioDocumentSnapshot { schema, styles, images, blocks });
impl RetireOwned for document::DocBlock {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Paragraph { style_id, runs } | Self::Heading { style_id, runs, .. } => seq![style_id, runs],
            Self::List { ordered, items } => seq![ordered, items],
            Self::Table { rows } => rows.retirement(),
            Self::Code { language, text } => seq![language, text],
            Self::Quote { blocks } => blocks.retirement(),
            Self::Image { image_id, alt, width, height } => seq![image_id, alt, width, height],
            Self::PageBreak => seq![],
        }
    }
}

retire_struct!(drawing::DrawStyle { name, fill, stroke, stroke_width, opacity });
retire_struct!(drawing::DrawLayer { id, name, visible, root });
retire_struct!(drawing::SemioDrawingSnapshot { schema, canvas, styles, layers });
impl RetireOwned for drawing::DrawNode {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Path { segments, style } => seq![segments, style],
            Self::Text { value, at, style } => seq![value, at, style],
            Self::Group { transform, children } => seq![transform, children],
            Self::Image { at, width, height, mime, bytes } => seq![at, width, height, mime, bytes],
        }
    }
}
impl RetireOwned for drawing::PathSegment {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::MoveTo { to } | Self::LineTo { to } => to.retirement(),
            Self::CubicTo { c1, c2, to } => seq![c1, c2, to],
            Self::QuadTo { c, to } => seq![c, to],
            Self::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => seq![rx, ry, x_rotation, large_arc, sweep, to],
            Self::Close => seq![],
        }
    }
}

retire_struct!(flow::PortRef { node, port });
retire_struct!(flow::FlowParam { key, value });
retire_struct!(flow::FlowNode { id, kind, label, params, position });
retire_struct!(flow::FlowEdge { id, from, to, kind });
retire_struct!(flow::SemioFlowSnapshot { schema, nodes, edges });

retire_struct!(value::ValueId { value });
retire_struct!(value::SemioValueEntry { key, value });
retire_struct!(value::SemioValueNode { id, value });
retire_struct!(value::SemioValueSnapshot { schema, root, nodes });
impl RetireOwned for value::SemioValue {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Null => seq![],
            Self::Bool { value } => value.retirement(),
            Self::Int { lexeme } | Self::Float { lexeme } => lexeme.retirement(),
            Self::Str { value } => value.retirement(),
            Self::Bytes { value } => value.retirement(),
            Self::List { items } => items.retirement(),
            Self::Map { entries } => entries.retirement(),
            Self::Ref { id } => id.retirement(),
        }
    }
}

retire_struct!(graph::GraphNodeId { value });
retire_struct!(graph::GraphEdgeId { value });
retire_struct!(graph::SemioGraphPort { name, kind });
retire_struct!(graph::SemioGraphNode { id, kind, label, position, ports, properties });
retire_struct!(graph::SemioGraphEdge { id, source, target, kind, label });
retire_struct!(graph::SemioGraphSnapshot { schema, nodes, edges });
retire_struct!(image::SemioImageFrame { delay_ms, rgba8 });
retire_struct!(image::SemioImageMetadataEntry { key, value });
retire_struct!(image::SemioImageSnapshot { schema, width, height, colorspace, bit_depth, frames, icc, metadata });
retire_struct!(kit::SemioKitType { id, name, category });
retire_struct!(kit::SemioKitPiece { id, type_id, transform });
retire_struct!(kit::SemioKitConnection { id, connecting_piece_id, connecting_port, connected_piece_id, connected_port });
retire_struct!(kit::SemioKitDesign { id, name, pieces, connections });
retire_struct!(kit::SemioKitSnapshot { schema, types, designs, objects, models, properties, representations });
retire_struct!(mesh::SemioPrimitive { id, topology, positions, normals, uvs, colors, indices, material_id });
retire_struct!(mesh::SemioMesh { id, primitives });
retire_struct!(mesh::SemioMaterial { id, base_color, metallic, roughness });
retire_struct!(mesh::SemioTexture { id, mime, bytes });
retire_struct!(mesh::SemioMeshSnapshot { schema, meshes, materials, textures });
retire_struct!(model::SpatialNode { id, kind, name, parent_id, placement });
retire_struct!(model::Property { key, value });
retire_struct!(model::PropertySet { name, properties });
retire_struct!(model::SemioModelElement { id, class, placement, geometry, spatial_id, psets });
retire_struct!(model::ModelRelation { id, kind, from, to });
retire_struct!(model::SemioModelSnapshot { schema, spatial, elements, relations });
impl RetireOwned for model::ElementClass {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Other { name } => name.retirement(),
            _ => seq![],
        }
    }
}
impl RetireOwned for model::GeometryRef {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Brep { brep_id } => brep_id.retirement(),
            Self::Mesh { mesh_id } => mesh_id.retirement(),
            Self::None => seq![],
        }
    }
}
impl RetireOwned for model::PsetValue {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Text { value } => value.retirement(),
            Self::Number { value } => value.retirement(),
            Self::Boolean { value } => value.retirement(),
        }
    }
}
impl RetireOwned for model::RelationKind {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Other { label } => label.retirement(),
            _ => seq![],
        }
    }
}
retire_struct!(object::SemioObjectSnapshot { schema, transform, brep, mesh, properties });
retire_struct!(presentation::SlidePictureImage { asset_id, mime, bytes });
retire_struct!(presentation::SlideTableCell { blocks });
retire_struct!(presentation::SlideTableRow { cells });
retire_struct!(presentation::SlideMaster { id, shapes });
retire_struct!(presentation::SlideLayout { id, master_id, shapes });
retire_struct!(presentation::Slide { id, layout_id, shapes, notes });
retire_struct!(presentation::SemioPresentationSnapshot { schema, masters, layouts, slides });
impl RetireOwned for presentation::PlaceholderKind {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Other { value } => value.retirement(),
            _ => seq![],
        }
    }
}
impl RetireOwned for presentation::SlideShape {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::TextBox { frame, blocks } => seq![frame, blocks],
            Self::Picture { frame, image } => seq![frame, image],
            Self::Table { frame, rows } => seq![frame, rows],
            Self::Placeholder { frame, kind } => seq![frame, kind],
        }
    }
}
retire_struct!(table::SemioTableColumn { name, kind });
retire_struct!(table::SemioTableRow { cells });
retire_struct!(table::SemioTableSnapshot { schema, columns, rows });
retire_struct!(text::SemioTextMark { kind, href });
retire_struct!(text::SemioTextRun { language, content, marks });
retire_struct!(text::SemioTextSnapshot { schema, runs });
retire_struct!(video::SemioVideoSample { pts, key, data });
retire_struct!(video::SemioVideoStream { kind, codec, width, height, rate, samples });
retire_struct!(video::SemioVideoSnapshot { schema, streams });

retire_struct!(subsets::drawing::schema::diff::NodePath { layer, path });
retire_struct!(document_mutation::DocBlockPath { segments, index });
impl RetireOwned for document_mutation::DocPathSegment {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Quote { block_index } => block_index.retirement(),
            Self::ListItem { block_index, item } => seq![block_index, item],
            Self::TableCell { block_index, row, cell } => seq![block_index, row, cell],
        }
    }
}
impl RetireOwned for value_mutation::SemioValuePathSegment {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Key { key } => key.retirement(),
            Self::Index { index } => index.retirement(),
        }
    }
}

impl RetireOwned for animation_mutation::SemioAnimationMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::SetSnapshot(payload) => payload.snapshot.retirement(),
            Self::InsertTimeline(animation_mutation::insert_timeline::InsertTimeline { index, timeline }) => seq![index, timeline],
            Self::RemoveTimeline(animation_mutation::remove_timeline::RemoveTimeline { index }) => index.retirement(),
            Self::SetTimelineName(animation_mutation::set_timeline_name::SetTimelineName { index, name }) => seq![index, name],
            Self::InsertChannel(animation_mutation::insert_channel::InsertChannel { timeline_index, index, channel }) => seq![timeline_index, index, channel],
            Self::RemoveChannel(animation_mutation::remove_channel::RemoveChannel { timeline_index, index }) => seq![timeline_index, index],
            Self::SetChannelTarget(animation_mutation::set_channel_target::SetChannelTarget { timeline_index, index, target }) => seq![timeline_index, index, target],
            Self::SetChannelInterpolation(animation_mutation::set_channel_interpolation::SetChannelInterpolation { timeline_index, index, interpolation }) => seq![timeline_index, index, interpolation],
            Self::InsertKeyframe(animation_mutation::insert_keyframe::InsertKeyframe { timeline_index, channel_index, index, keyframe }) => seq![timeline_index, channel_index, index, keyframe],
            Self::RemoveKeyframe(animation_mutation::remove_keyframe::RemoveKeyframe { timeline_index, channel_index, index }) => seq![timeline_index, channel_index, index],
            Self::SetKeyframeTime(animation_mutation::set_keyframe_time::SetKeyframeTime { timeline_index, channel_index, index, t }) => seq![timeline_index, channel_index, index, t],
            Self::SetKeyframeValue(animation_mutation::set_keyframe_value::SetKeyframeValue { timeline_index, channel_index, index, value }) => seq![timeline_index, channel_index, index, value],
        }
    }
}

impl RetireOwned for audio_mutation::SemioAudioMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::SetSnapshot(payload) => payload.snapshot.retirement(),
            Self::SetSampleRate(audio_mutation::set_sample_rate::SetSampleRate { sample_rate }) => sample_rate.retirement(),
            Self::SetFormat(audio_mutation::set_format::SetFormat { format }) => format.retirement(),
            Self::InsertChannel(audio_mutation::insert_channel::InsertChannel { index, channel }) => seq![index, channel],
            Self::RemoveChannel(audio_mutation::remove_channel::RemoveChannel { index }) => index.retirement(),
            Self::SetChannelSamples(audio_mutation::set_channel_samples::SetChannelSamples { index, samples }) => seq![index, samples],
            Self::InsertTag(audio_mutation::insert_tag::InsertTag { index, tag }) => seq![index, tag],
            Self::RemoveTag(audio_mutation::remove_tag::RemoveTag { index }) => index.retirement(),
            Self::SetTagValue(audio_mutation::set_tag_value::SetTagValue { index, value }) => seq![index, value],
        }
    }
}

impl RetireOwned for cad_mutation::SemioCadMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::SetSnapshot(payload) => payload.snapshot.retirement(),
            Self::AddLayer(cad_mutation::add_layer::AddLayer { layer }) => layer.retirement(),
            Self::RemoveLayer(cad_mutation::remove_layer::RemoveLayer { name }) => name.retirement(),
            Self::SetLayer(cad_mutation::set_layer::SetLayer { name, color_index, line_type, visible }) => seq![name, color_index, line_type, visible],
            Self::AddBlock(cad_mutation::add_block::AddBlock { block }) => block.retirement(),
            Self::RemoveBlock(cad_mutation::remove_block::RemoveBlock { name }) => name.retirement(),
            Self::SetBlockBasePoint(cad_mutation::set_block_base_point::SetBlockBasePoint { name, base_point }) => seq![name, base_point],
            Self::AddEntity(cad_mutation::add_entity::AddEntity { entity }) => entity.retirement(),
            Self::RemoveEntity(cad_mutation::remove_entity::RemoveEntity { handle }) => handle.retirement(),
            Self::SetEntityLayer(cad_mutation::set_entity_layer::SetEntityLayer { handle, layer }) => seq![handle, layer],
            Self::SetEntityGeometry(cad_mutation::set_entity_geometry::SetEntityGeometry { handle, entity }) => seq![handle, entity],
            Self::AddBlockEntity(cad_mutation::add_block_entity::AddBlockEntity { block_name, entity }) => seq![block_name, entity],
            Self::RemoveBlockEntity(cad_mutation::remove_block_entity::RemoveBlockEntity { block_name, handle }) => seq![block_name, handle],
            Self::SetBlockEntityLayer(cad_mutation::set_block_entity_layer::SetBlockEntityLayer { block_name, handle, layer }) => seq![block_name, handle, layer],
            Self::SetBlockEntityGeometry(cad_mutation::set_block_entity_geometry::SetBlockEntityGeometry { block_name, handle, entity }) => seq![block_name, handle, entity],
        }
    }
}

impl RetireOwned for document_mutation::SemioDocumentMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::SetSnapshot(payload) => payload.snapshot.retirement(),
            Self::InsertBlock(document_mutation::insert_block::InsertBlock { path, block }) => seq![path, block],
            Self::SetBlockContent(document_mutation::set_block_content::SetBlockContent { path, block }) => seq![path, block],
            Self::RemoveBlock(document_mutation::remove_block::RemoveBlock { path }) => path.retirement(),
            Self::SetParagraphStyle(document_mutation::set_paragraph_style::SetParagraphStyle { path, style_id }) => seq![path, style_id],
            Self::SetHeadingLevel(document_mutation::set_heading_level::SetHeadingLevel { path, level }) => seq![path, level],
            Self::SetListOrdered(document_mutation::set_list_ordered::SetListOrdered { path, ordered }) => seq![path, ordered],
            Self::SetRunText(document_mutation::set_run_text::SetRunText { path, run_index, text }) => seq![path, run_index, text],
            Self::SetRunStyle(document_mutation::set_run_style::SetRunStyle { path, run_index, style }) => seq![path, run_index, style],
            Self::SetImageBlock(document_mutation::set_image_block::SetImageBlock { path, image_id, alt, width, height }) => seq![path, image_id, alt, width, height],
            Self::InsertStyle(document_mutation::insert_style::InsertStyle { style }) => style.retirement(),
            Self::RemoveStyle(document_mutation::remove_style::RemoveStyle { id }) => id.retirement(),
            Self::SetStyleName(document_mutation::set_style_name::SetStyleName { id, name }) => seq![id, name],
            Self::SetStyleBasedOn(document_mutation::set_style_based_on::SetStyleBasedOn { id, based_on }) => seq![id, based_on],
            Self::InsertImage(document_mutation::insert_image::InsertImage { image }) => image.retirement(),
            Self::RemoveImage(document_mutation::remove_image::RemoveImage { id }) => id.retirement(),
            Self::SetImageBytes(document_mutation::set_image_bytes::SetImageBytes { id, mime, bytes }) => seq![id, mime, bytes],
        }
    }
}

impl RetireOwned for flow_mutation::SemioFlowMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::SetSnapshot(payload) => payload.snapshot.retirement(),
            Self::InsertNode(flow_mutation::insert_node::InsertNode { node }) => node.retirement(),
            Self::RemoveNode(flow_mutation::remove_node::RemoveNode { id }) => id.retirement(),
            Self::SetNodeKind(flow_mutation::set_node_kind::SetNodeKind { id, kind }) => seq![id, kind],
            Self::SetNodeLabel(flow_mutation::set_node_label::SetNodeLabel { id, label }) => seq![id, label],
            Self::SetNodePosition(flow_mutation::set_node_position::SetNodePosition { id, position }) => seq![id, position],
            Self::SetNodeParam(flow_mutation::set_node_param::SetNodeParam { id, key, value }) => seq![id, key, value],
            Self::RemoveNodeParam(flow_mutation::remove_node_param::RemoveNodeParam { id, key }) => seq![id, key],
            Self::InsertEdge(flow_mutation::insert_edge::InsertEdge { edge }) => edge.retirement(),
            Self::RemoveEdge(flow_mutation::remove_edge::RemoveEdge { id }) => id.retirement(),
            Self::SetEdgeEndpoints(flow_mutation::set_edge_endpoints::SetEdgeEndpoints { id, from, to }) => seq![id, from, to],
            Self::SetEdgeKind(flow_mutation::set_edge_kind::SetEdgeKind { id, kind }) => seq![id, kind],
        }
    }
}

impl RetireOwned for image_mutation::SemioImageMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::SetSnapshot(payload) => payload.snapshot.retirement(),
            Self::SetDimensions(image_mutation::set_dimensions::SetDimensions { width, height }) => seq![width, height],
            Self::SetColorspace(image_mutation::set_colorspace::SetColorspace { colorspace }) => colorspace.retirement(),
            Self::SetBitDepth(image_mutation::set_bit_depth::SetBitDepth { bit_depth }) => bit_depth.retirement(),
            Self::SetIcc(image_mutation::set_icc::SetIcc { icc }) => icc.retirement(),
            Self::InsertFrame(image_mutation::insert_frame::InsertFrame { index, frame }) => seq![index, frame],
            Self::RemoveFrame(image_mutation::remove_frame::RemoveFrame { index }) => index.retirement(),
            Self::MoveFrame(image_mutation::move_frame::MoveFrame { from, to }) => seq![from, to],
            Self::SetFrameDelay(image_mutation::set_frame_delay::SetFrameDelay { index, delay_ms }) => seq![index, delay_ms],
            Self::SetFramePixels(image_mutation::set_frame_pixels::SetFramePixels { index, rgba8 }) => seq![index, rgba8],
            Self::SetMetadataEntry(image_mutation::set_metadata_entry::SetMetadataEntry { key, value }) => seq![key, value],
            Self::RemoveMetadataEntry(image_mutation::remove_metadata_entry::RemoveMetadataEntry { key }) => key.retirement(),
        }
    }
}

impl RetireOwned for model_mutation::SemioModelMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::SetSnapshot(payload) => payload.snapshot.retirement(),
            Self::InsertSpatialNode(model_mutation::insert_spatial_node::InsertSpatialNode { node }) => node.retirement(),
            Self::RemoveSpatialNode(model_mutation::remove_spatial_node::RemoveSpatialNode { id }) => id.retirement(),
            Self::SetSpatialNode(model_mutation::set_spatial_node::SetSpatialNode { id, kind, name, parent_id, placement }) => seq![id, kind, name, parent_id, placement],
            Self::InsertElement(model_mutation::insert_element::InsertElement { element }) => element.retirement(),
            Self::RemoveElement(model_mutation::remove_element::RemoveElement { id }) => id.retirement(),
            Self::SetElement(model_mutation::set_element::SetElement { id, class, placement, geometry, spatial_id, psets }) => seq![id, class, placement, geometry, spatial_id, psets],
            Self::InsertRelation(model_mutation::insert_relation::InsertRelation { relation }) => relation.retirement(),
            Self::RemoveRelation(model_mutation::remove_relation::RemoveRelation { id }) => id.retirement(),
            Self::SetRelation(model_mutation::set_relation::SetRelation { id, kind, from, to }) => seq![id, kind, from, to],
        }
    }
}

impl RetireOwned for presentation_mutation::SemioPresentationMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::SetSnapshot(payload) => payload.snapshot.retirement(),
            Self::InsertSlide(presentation_mutation::insert_slide::InsertSlide { index, slide }) => seq![index, slide],
            Self::RemoveSlide(presentation_mutation::remove_slide::RemoveSlide { index }) => index.retirement(),
            Self::SetSlideLayout(presentation_mutation::set_slide_layout::SetSlideLayout { index, layout_id }) => seq![index, layout_id],
            Self::SetSlideNotes(presentation_mutation::set_slide_notes::SetSlideNotes { index, notes }) => seq![index, notes],
            Self::InsertShape(presentation_mutation::insert_shape::InsertShape { slide_index, shape_index, shape }) => seq![slide_index, shape_index, shape],
            Self::RemoveShape(presentation_mutation::remove_shape::RemoveShape { slide_index, shape_index }) => seq![slide_index, shape_index],
            Self::SetShapeFrame(presentation_mutation::set_shape_frame::SetShapeFrame { slide_index, shape_index, frame }) => seq![slide_index, shape_index, frame],
            Self::SetTextBoxBlocks(presentation_mutation::set_textbox_blocks::SetTextBoxBlocks { slide_index, shape_index, blocks }) => seq![slide_index, shape_index, blocks],
            Self::InsertMaster(presentation_mutation::insert_master::InsertMaster { master }) => master.retirement(),
            Self::RemoveMaster(presentation_mutation::remove_master::RemoveMaster { id }) => id.retirement(),
            Self::InsertLayout(presentation_mutation::insert_layout::InsertLayout { layout }) => layout.retirement(),
            Self::RemoveLayout(presentation_mutation::remove_layout::RemoveLayout { id }) => id.retirement(),
            Self::SetLayoutMaster(presentation_mutation::set_layout_master::SetLayoutMaster { id, master_id }) => seq![id, master_id],
        }
    }
}

impl RetireOwned for value_mutation::SemioValueMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::SetSnapshot(payload) => payload.snapshot.retirement(),
            Self::SetValue(value_mutation::set_value::SetValue { path, value }) => seq![path, value],
            Self::SetMapEntry(value_mutation::set_map_entry::SetMapEntry { path, key, value }) => seq![path, key, value],
            Self::RemoveMapEntry(value_mutation::remove_map_entry::RemoveMapEntry { path, key }) => seq![path, key],
            Self::InsertListItem(value_mutation::insert_list_item::InsertListItem { path, index, value }) => seq![path, index, value],
            Self::RemoveListItem(value_mutation::remove_list_item::RemoveListItem { path, index }) => seq![path, index],
            Self::SetNode(value_mutation::set_node::SetNode { id, value }) => seq![id, value],
            Self::RemoveNode(value_mutation::remove_node::RemoveNode { id }) => id.retirement(),
        }
    }
}

impl RetireOwned for video_mutation::SemioVideoMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::SetSnapshot(payload) => payload.snapshot.retirement(),
            Self::InsertStream(video_mutation::insert_stream::InsertStream { index, stream }) => seq![index, stream],
            Self::RemoveStream(video_mutation::remove_stream::RemoveStream { index }) => index.retirement(),
            Self::SetStreamMeta(video_mutation::set_stream_meta::SetStreamMeta { index, kind, codec, width, height, rate }) => seq![index, kind, codec, width, height, rate],
            Self::InsertSample(video_mutation::insert_sample::InsertSample { stream_index, index, sample }) => seq![stream_index, index, sample],
            Self::RemoveSample(video_mutation::remove_sample::RemoveSample { stream_index, index }) => seq![stream_index, index],
            Self::SetSampleData(video_mutation::set_sample_data::SetSampleData { stream_index, index, data }) => seq![stream_index, index, data],
            Self::SetSampleFlags(video_mutation::set_sample_flags::SetSampleFlags { stream_index, index, pts, key }) => seq![stream_index, index, pts, key],
        }
    }
}

impl RetireOwned for mesh_mutation::SemioMeshMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::CreateMesh(value) => value.mesh.retirement(),
            Self::DeleteMesh(value) => value.id.retirement(),
            Self::CreatePrimitive(value) => seq![value.mesh_id, value.primitive],
            Self::DeletePrimitive(value) => seq![value.mesh_id, value.primitive_id],
            Self::SetPrimitiveTopology(value) => seq![value.mesh_id, value.primitive_id, value.topology],
            Self::ReplacePrimitiveGeometry(value) => seq![value.mesh_id, value.primitive_id, value.positions, value.normals, value.uvs, value.colors, value.indices],
            Self::SetPrimitiveMaterial(value) => seq![value.mesh_id, value.primitive_id, value.material_id],
            Self::CreateMaterial(value) => value.material.retirement(),
            Self::DeleteMaterial(value) => value.id.retirement(),
            Self::ChangeMaterialBaseColor(value) => seq![value.id, value.new_base_color],
            Self::ChangeMaterialMetallic(value) => seq![value.id, value.new_metallic],
            Self::ChangeMaterialRoughness(value) => seq![value.id, value.new_roughness],
            Self::CreateTexture(value) => value.texture.retirement(),
            Self::DeleteTexture(value) => value.id.retirement(),
            Self::ChangeTextureMime(value) => seq![value.id, value.new_mime],
            Self::ReplaceTextureBytes(value) => seq![value.id, value.new_bytes],
            Self::MoveVertex(value) => seq![value.mesh_id, value.primitive_id, value.vertex_index, value.new_point],
        }
    }
}

impl RetireOwned for drawing_mutation::SemioDrawingMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::CreateLayer(value) => seq![value.index, value.layer],
            Self::DeleteLayer(value) => value.id.retirement(),
            Self::CreateNode(value) => seq![value.parent, value.index, value.node],
            Self::DeleteNode(value) => value.at.retirement(),
            Self::MoveNode(value) => seq![value.at, value.new_origin],
            Self::DragNodes(value) => seq![value.ats, value.offset],
            Self::RotateNode(value) => seq![value.at, value.new_rotation],
            Self::ScaleNode(value) => seq![value.at, value.new_scale],
            Self::ReorderNodes(value) => seq![value.parent, value.from, value.to],
            Self::GroupNodes(value) => seq![value.parent, value.indices, value.transform],
            Self::UngroupNode(value) => value.at.retirement(),
            Self::FlattenNode(value) => value.at.retirement(),
            Self::UnflattenNode(value) => seq![value.at, value.original],
            Self::ReplacePath(value) => seq![value.at, value.new_segments],
            Self::ReplaceFill(value) => seq![value.style_name, value.new_fill],
            Self::ChangeStrokeColor(value) => seq![value.style_name, value.new_color],
            Self::ChangeStrokeWidth(value) => seq![value.style_name, value.new_width],
        }
    }
}

impl RetireOwned for table_mutation::SemioTableMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::CreateColumn(value) => seq![value.name, value.kind, value.index],
            Self::DeleteColumn(value) => value.name.retirement(),
            Self::RenameColumn(value) => seq![value.name, value.new_name],
            Self::ReorderColumns(value) => seq![value.name, value.to_index],
            Self::InsertRow(value) => seq![value.index, value.row],
            Self::RemoveRow(value) => value.index.retirement(),
            Self::ReorderRows(value) => seq![value.from, value.to],
            Self::EditCell(value) => seq![value.row_index, value.column_name, value.new_value],
        }
    }
}

impl RetireOwned for brep_mutation::SemioBrepMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::CreateVertex(value) => seq![value.id, value.point],
            Self::DeleteVertex(value) => value.id.retirement(),
            Self::CreateEdge(value) => seq![value.id, value.start_vertex, value.end_vertex, value.curve],
            Self::DeleteEdge(value) => value.id.retirement(),
            Self::CreateFace(value) => seq![value.id, value.outer_loop, value.inner_loops, value.surface, value.orientation],
            Self::DeleteFace(value) => value.id.retirement(),
            Self::CreateShell(value) => seq![value.id, value.faces],
            Self::DeleteShell(value) => value.id.retirement(),
            Self::CreateSolid(value) => seq![value.id, value.shells],
            Self::DeleteSolid(value) => value.id.retirement(),
            Self::ReplaceCurve(value) => seq![value.edge_id, value.new_curve],
            Self::ReplaceSurface(value) => seq![value.face_id, value.new_surface],
            Self::MoveVertex(value) => seq![value.vertex_id, value.new_point],
        }
    }
}

impl RetireOwned for graph_mutation::SemioGraphMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::CreateNode(value) => seq![value.id, value.kind, value.label, value.position, value.ports, value.properties],
            Self::DeleteNode(value) => value.id.retirement(),
            Self::ChangeNodeKind(value) => seq![value.id, value.new_kind],
            Self::ChangeNodeLabel(value) => seq![value.id, value.new_label],
            Self::MoveNode(value) => seq![value.id, value.new_position],
            Self::AddNodePort(value) => seq![value.node_id, value.index, value.port],
            Self::RemoveNodePort(value) => seq![value.node_id, value.index],
            Self::AddNodeProperty(value) => seq![value.node_id, value.index, value.property],
            Self::RemoveNodeProperty(value) => seq![value.node_id, value.index],
            Self::CreateEdge(value) => seq![value.id, value.source, value.target, value.kind, value.label],
            Self::DeleteEdge(value) => value.id.retirement(),
        }
    }
}

impl RetireOwned for object_mutation::SemioObjectMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::MoveObject(value) => value.translation.retirement(),
            Self::RotateObject(value) => value.rotation.retirement(),
            Self::ScaleObject(value) => value.scale.retirement(),
            Self::CreateBrep(value) => seq![value.child_id, value.target],
            Self::DeleteBrep(_) => seq![],
            Self::CreateMesh(value) => seq![value.child_id, value.target],
            Self::DeleteMesh(_) => seq![],
            Self::CreateProperties(value) => seq![value.child_id, value.target],
            Self::DeleteProperties(_) => seq![],
        }
    }
}

impl RetireOwned for kit_mutation::SemioKitMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::CreateObject(value) => seq![value.child_id, value.target],
            Self::DeleteObject(value) => value.child_id.retirement(),
            Self::CreateModel(value) => seq![value.child_id, value.target],
            Self::DeleteModel(value) => value.child_id.retirement(),
            Self::CreateProperties(value) => seq![value.child_id, value.target],
            Self::DeleteProperties(_) => seq![],
            Self::BindRepresentation(value) => seq![value.target, value.pin, value.role],
            Self::UnbindRepresentation(value) => value.index.retirement(),
            Self::ChangeRepresentationPin(value) => seq![value.index, value.pin],
            Self::AddType(value) => seq![value.id, value.name, value.category],
            Self::RemoveType(value) => value.id.retirement(),
            Self::RenameType(value) => seq![value.id, value.new_name],
            Self::AddDesign(value) => seq![value.id, value.name],
            Self::RemoveDesign(value) => value.id.retirement(),
            Self::EditDesign(value) => seq![value.id, value.pieces, value.connections],
        }
    }
}

impl RetireOwned for text_mutation::SemioTextMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::InsertRun(value) => seq![value.index, value.run],
            Self::RemoveRun(value) => value.index.retirement(),
            Self::EditRun(value) => seq![value.index, value.new_content],
            Self::ChangeRunLanguage(value) => seq![value.index, value.new_language],
            Self::ReorderRuns(value) => seq![value.from, value.to],
            Self::AddMark(value) => seq![value.run_index, value.index, value.mark],
            Self::RemoveMark(value) => seq![value.run_index, value.index],
        }
    }
}

struct SemioSnapshotRetirementFactory<P>(PhantomData<fn() -> P>);
struct SemioOwnedValueRetirementFactory<T>(PhantomData<fn() -> T>);

impl<T: RetireOwned> dsl::ArtifactOwnedValueRetirementFactory<T> for SemioOwnedValueRetirementFactory<T> {
    fn retire_owned(&self, value: T) -> Box<dyn dsl::ErasedSnapshotRetirement> {
        owned_retirement(value)
    }
}

struct SemioMutationRetirementFactory<T>(PhantomData<fn() -> T>);

impl<T: RetireOwned> dsl::ArtifactOwnedValueRetirementFactory<T> for SemioMutationRetirementFactory<T> {
    fn retire_owned(&self, value: T) -> Box<dyn dsl::ErasedSnapshotRetirement> {
        owned_retirement(value)
    }
}

enum SemioStoreClosePhase {
    DisplacedOwners,
    ReturnedLeases,
    HistoryMutations { edit_index: Option<usize> },
    HistoryEdits,
    HistoryMetadata { lane: u8 },
    MessageLedgers { lane: u8 },
    Conflicts,
    PendingReport,
    RuntimeStrings { lane: u8 },
    EnvelopeMetadata,
    TailSnapshot,
    CurrentSnapshot,
    Backbone,
    CausalIndex,
    StructuralOwners,
    FinalEnvelope,
    Complete,
}

struct SemioStoreOwnedDisposer<P, Mutation> {
    phase: SemioStoreClosePhase,
    started: bool,
    active: std::mem::ManuallyDrop<Option<Box<dyn dsl::ErasedSnapshotRetirement>>>,
    marker: PhantomData<fn() -> (P, Mutation)>,
}

impl<P, Mutation> SemioStoreOwnedDisposer<P, Mutation> {
    fn new() -> Self {
        Self { phase: SemioStoreClosePhase::DisplacedOwners, started: false, active: std::mem::ManuallyDrop::new(None), marker: PhantomData }
    }
}
macro_rules! member_owners {
    ($($name:ident => $module:ident, $snapshot:ident, $mutation:ident);* $(;)?) => {$ (
        impl dsl::SnapshotRetirementFactory<subsets::$module::schema::snapshot::$snapshot>
            for SemioSnapshotRetirementFactory<subsets::$module::schema::snapshot::$snapshot>
        {
            fn retire(
                &self,
                snapshot: Arc<subsets::$module::schema::snapshot::$snapshot>,
            ) -> Box<dyn dsl::ErasedSnapshotRetirement> {
                shared_retirement(snapshot)
            }
        }

        impl dsl::ArtifactStoreOwnedDisposer<
            subsets::$module::schema::snapshot::$snapshot,
            subsets::$module::schema::mutations::$mutation,
        > for SemioStoreOwnedDisposer<
            subsets::$module::schema::snapshot::$snapshot,
            subsets::$module::schema::mutations::$mutation,
        > {
            fn close_step(
                &mut self,
                store: &mut dsl::ArtifactStoreCloseView<'_,
                    subsets::$module::schema::snapshot::$snapshot,
                    subsets::$module::schema::mutations::$mutation,
                >,
                maximum_items: usize,
                maximum_bytes: usize,
            ) -> Result<dsl::SnapshotRetirementStep, String> {
                self.started = true;
                if maximum_items == 0 {
                    return Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                if let Some(active) = self.active.as_mut() {
                    return match active.close_step(maximum_items, maximum_bytes)? {
                        dsl::SnapshotRetirementStep::Pending { released_items, released_bytes }
                            if released_items <= maximum_items && released_bytes <= maximum_bytes =>
                        {
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items, released_bytes })
                        }
                        dsl::SnapshotRetirementStep::Pending { .. } => Err("semio store nested retirement exceeded its exact item or byte grant".into()),
                        dsl::SnapshotRetirementStep::Blocked => Ok(dsl::SnapshotRetirementStep::Blocked),
                        dsl::SnapshotRetirementStep::Complete => {
                            if !active.terminal_is_empty() {
                                return Err("semio store nested retirement reported Complete without its terminal-empty witness".into());
                            }
                            drop(self.active.take());
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                        }
                    };
                }
                match &mut self.phase {
                    SemioStoreClosePhase::DisplacedOwners => match store.maintenance_retirements_step(maximum_items, maximum_bytes)? {
                        dsl::SnapshotRetirementStep::Complete => {
                            if !store.maintenance_retirements_terminal_is_empty() {
                                return Err("semio store displaced retirement reported Complete without its terminal-empty witness".into());
                            }
                            self.phase = SemioStoreClosePhase::ReturnedLeases;
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                        step => Ok(step),
                    },
                    SemioStoreClosePhase::ReturnedLeases => match store.take_returned_snapshot_read_retirement().map_err(|error| error.to_string())? {
                        Some(retirement) => {
                            *self.active = Some(retirement);
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                        None if !store.snapshot_read_leases_terminal_is_empty() => Ok(dsl::SnapshotRetirementStep::Blocked),
                        None => {
                            self.phase = SemioStoreClosePhase::HistoryMutations { edit_index: store.history_edit_count().checked_sub(1) };
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                    },
                    SemioStoreClosePhase::HistoryMutations { edit_index } => {
                        let Some(index) = *edit_index else {
                            self.phase = SemioStoreClosePhase::HistoryEdits;
                            return Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                        };
                        match store.take_history_mutation_at(index).map_err(|error| error.to_string())? {
                            Some(retirement) => {
                                *self.active = Some(retirement);
                                Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                            }
                            None => {
                                *edit_index = index.checked_sub(1);
                                Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                            }
                        }
                    }
                    SemioStoreClosePhase::HistoryEdits => match store.take_last_history_edit_retirement().map_err(|error| error.to_string())? {
                        Some(retirement) => {
                            *self.active = Some(retirement);
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                        None => {
                            self.phase = SemioStoreClosePhase::HistoryMetadata { lane: 0 };
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                    },
                    SemioStoreClosePhase::HistoryMetadata { lane } => {
                        let authority = match *lane {
                            0 => dsl::ArtifactStoreHistoryMetadataLane::Changes,
                            1 => dsl::ArtifactStoreHistoryMetadataLane::Checkpoints,
                            2 => dsl::ArtifactStoreHistoryMetadataLane::Alternatives,
                            _ => {
                                self.phase = SemioStoreClosePhase::MessageLedgers { lane: 0 };
                                return Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                            }
                        };
                        match store.take_history_metadata_retirement(authority) {
                            Some(retirement) => {
                                *self.active = Some(retirement);
                                Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                            }
                            None => {
                                *lane += 1;
                                Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                            }
                        }
                    }
                    SemioStoreClosePhase::MessageLedgers { lane } => {
                        let authority = match *lane {
                            0 => dsl::ArtifactStoreMessageLedgerLane::Durable,
                            _ => {
                                self.phase = SemioStoreClosePhase::Conflicts;
                                return Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                            }
                        };
                        match store.take_message_ledger_retirement(authority) {
                            Some(retirement) => {
                                *self.active = Some(retirement);
                                Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                            }
                            None => {
                                *lane += 1;
                                Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                            }
                        }
                    }
                    SemioStoreClosePhase::Conflicts => match store.take_conflict_retirement() {
                        Some(retirement) => {
                            *self.active = Some(retirement);
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                        None => {
                            self.phase = SemioStoreClosePhase::PendingReport;
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                    },
                    SemioStoreClosePhase::PendingReport => match store.take_pending_report_retirement() {
                        Some(retirement) => {
                            *self.active = Some(retirement);
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                        None => {
                            self.phase = SemioStoreClosePhase::RuntimeStrings { lane: 0 };
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                    },
                    SemioStoreClosePhase::RuntimeStrings { lane } => {
                        let authority = match *lane {
                            0 => dsl::ArtifactStoreCloseStringLane::AppliedEditIds,
                            1 => dsl::ArtifactStoreCloseStringLane::RedoEditIds,
                            2 => dsl::ArtifactStoreCloseStringLane::AppliedRevisionIds,
                            3 => dsl::ArtifactStoreCloseStringLane::RedoRevisionIds,
                            4 => dsl::ArtifactStoreCloseStringLane::CurrentCheckpointId,
                            5 => dsl::ArtifactStoreCloseStringLane::LocalActorId,
                            6 => dsl::ArtifactStoreCloseStringLane::TailUndoEditId,
                            _ => {
                                self.phase = SemioStoreClosePhase::EnvelopeMetadata;
                                return Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                            }
                        };
                        match store.take_runtime_string_retirement(authority) {
                            Some(retirement) => {
                                *self.active = Some(retirement);
                                Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                            }
                            None => {
                                *lane += 1;
                                Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                            }
                        }
                    }
                    SemioStoreClosePhase::EnvelopeMetadata => match store.take_envelope_metadata_string_retirement() {
                        Some(retirement) => {
                            *self.active = Some(retirement);
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                        None => {
                            self.phase = SemioStoreClosePhase::TailSnapshot;
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                    },
                    SemioStoreClosePhase::TailSnapshot => match store.take_tail_snapshot_retirement().map_err(|error| error.to_string())? {
                        dsl::ArtifactStoreSnapshotRootClose::Empty => {
                            self.phase = SemioStoreClosePhase::CurrentSnapshot;
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                        dsl::ArtifactStoreSnapshotRootClose::ReleasedShared => {
                            self.phase = SemioStoreClosePhase::CurrentSnapshot;
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                        }
                        dsl::ArtifactStoreSnapshotRootClose::Retirement(retirement) => {
                            *self.active = Some(retirement);
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                    },
                    SemioStoreClosePhase::CurrentSnapshot => match store.take_current_snapshot_retirement().map_err(|error| error.to_string())? {
                        Some(retirement) => {
                            *self.active = Some(retirement);
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                        None => {
                            self.phase = SemioStoreClosePhase::Backbone;
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                    },
                    SemioStoreClosePhase::Backbone => match store.take_backbone_retirement() {
                        Some(retirement) => {
                            *self.active = Some(retirement);
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                        None => {
                            self.phase = SemioStoreClosePhase::CausalIndex;
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                    },
                    SemioStoreClosePhase::CausalIndex => match store.take_causal_owner_retirement() {
                        Some(retirement) => {
                            *self.active = Some(retirement);
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                        None => {
                            self.phase = SemioStoreClosePhase::StructuralOwners;
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                    },
                    SemioStoreClosePhase::StructuralOwners if store.structural_owners_terminal_is_empty() => {
                        self.phase = SemioStoreClosePhase::FinalEnvelope;
                        Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                    }
                    SemioStoreClosePhase::StructuralOwners => Err("semio member store structural close reached an owner outside its exact phase cursor".into()),
                    SemioStoreClosePhase::FinalEnvelope => match store.take_final_envelope_retirement().map_err(|error| error.to_string())? {
                        Some(retirement) => {
                            *self.active = Some(retirement);
                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                        }
                        None => {
                            self.phase = SemioStoreClosePhase::Complete;
                            Ok(dsl::SnapshotRetirementStep::Complete)
                        }
                    },
                    SemioStoreClosePhase::Complete => Ok(dsl::SnapshotRetirementStep::Complete),
                }
            }

            fn terminal_is_empty(
                &self,
                store: &dsl::ArtifactStore<
                    subsets::$module::schema::snapshot::$snapshot,
                    subsets::$module::schema::mutations::$mutation,
                >,
            ) -> bool {
                matches!(self.phase, SemioStoreClosePhase::Complete) && self.active.is_none() && store.owned_roots_terminal_is_empty()
            }

            fn close_uninstalled_step(&mut self, maximum_items: usize) -> Result<dsl::SnapshotRetirementStep, String> {
                if self.started || self.active.is_some() { return Err("installed semio disposer cannot retire as uninstalled".into()); }
                if maximum_items == 0 { return Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
                self.phase = SemioStoreClosePhase::Complete;
                Ok(dsl::SnapshotRetirementStep::Complete)
            }

            fn uninstalled_terminal_is_empty(&self) -> bool {
                !self.started && matches!(self.phase, SemioStoreClosePhase::Complete) && self.active.is_none()
            }
        }

        impl dsl::MemberStoreOwner<subsets::$module::schema::mutations::$mutation>
            for subsets::$module::schema::snapshot::$snapshot
        {
            semio_snapshot_open!($module, subsets::$module::schema::snapshot::$snapshot);

            fn member_store_owners() -> dsl::MemberStoreOwners<
                Self,
                subsets::$module::schema::mutations::$mutation,
            > {
                dsl::MemberStoreOwners::new(Arc::new(
                    SemioSnapshotRetirementFactory::<Self>(PhantomData),
                ), Arc::new(SemioOwnedValueRetirementFactory::<Self>(PhantomData)), Arc::new(
                    SemioMutationRetirementFactory::<subsets::$module::schema::mutations::$mutation>(PhantomData),
                ), Box::new(SemioStoreOwnedDisposer::<Self, subsets::$module::schema::mutations::$mutation>::new()))
            }
        }
    )*};
}
semio_subset_table!(member_owners);
//#endregion 🧹️SnapshotRetirement

// 🧬️ Eighteen composable subsets, each bound to its exact kind, standard, subset and schema.
dsl::space_members! {
    pub enum SemioMembers, SemioMembersOpen {
        Animation("s.stdio.semio", "v1", "animation", "stdio.semio") => (subsets::animation::schema::snapshot::SemioAnimationSnapshot, subsets::animation::schema::mutations::SemioAnimationMutation),
        Audio("s.stdio.semio", "v1", "audio", "stdio.semio") => (subsets::audio::schema::snapshot::SemioAudioSnapshot, subsets::audio::schema::mutations::SemioAudioMutation),
        Brep("s.stdio.semio", "v1", "brep", "stdio.semio") => (subsets::brep::schema::snapshot::SemioBrepSnapshot, subsets::brep::schema::mutations::SemioBrepMutation),
        Cad("s.stdio.semio", "v1", "cad", "stdio.semio") => (subsets::cad::schema::snapshot::SemioCadSnapshot, subsets::cad::schema::mutations::SemioCadMutation),
        Document("s.stdio.semio", "v1", "document", "stdio.semio") => (subsets::document::schema::snapshot::SemioDocumentSnapshot, subsets::document::schema::mutations::SemioDocumentMutation),
        Drawing("s.stdio.semio", "v1", "drawing", "stdio.semio") => (subsets::drawing::schema::snapshot::SemioDrawingSnapshot, subsets::drawing::schema::mutations::SemioDrawingMutation),
        Flow("s.stdio.semio", "v1", "flow", "stdio.semio") => (subsets::flow::schema::snapshot::SemioFlowSnapshot, subsets::flow::schema::mutations::SemioFlowMutation),
        Graph("s.stdio.semio", "v1", "graph", "stdio.semio") => (subsets::graph::schema::snapshot::SemioGraphSnapshot, subsets::graph::schema::mutations::SemioGraphMutation),
        Image("s.stdio.semio", "v1", "image", "stdio.semio") => (subsets::image::schema::snapshot::SemioImageSnapshot, subsets::image::schema::mutations::SemioImageMutation),
        Kit("s.stdio.semio", "v1", "kit", "stdio.semio") => (subsets::kit::schema::snapshot::SemioKitSnapshot, subsets::kit::schema::mutations::SemioKitMutation),
        Mesh("s.stdio.semio", "v1", "mesh", "stdio.semio") => (subsets::mesh::schema::snapshot::SemioMeshSnapshot, subsets::mesh::schema::mutations::SemioMeshMutation),
        Model("s.stdio.semio", "v1", "model", "stdio.semio") => (subsets::model::schema::snapshot::SemioModelSnapshot, subsets::model::schema::mutations::SemioModelMutation),
        Object("s.stdio.semio", "v1", "object", "stdio.semio") => (subsets::object::schema::snapshot::SemioObjectSnapshot, subsets::object::schema::mutations::SemioObjectMutation),
        Presentation("s.stdio.semio", "v1", "presentation", "stdio.semio") => (subsets::presentation::schema::snapshot::SemioPresentationSnapshot, subsets::presentation::schema::mutations::SemioPresentationMutation),
        Table("s.stdio.semio", "v1", "table", "stdio.semio") => (subsets::table::schema::snapshot::SemioTableSnapshot, subsets::table::schema::mutations::SemioTableMutation),
        Text("s.stdio.semio", "v1", "text", "stdio.semio") => (subsets::text::schema::snapshot::SemioTextSnapshot, subsets::text::schema::mutations::SemioTextMutation),
        Value("s.stdio.semio", "v1", "value", "stdio.semio") => (subsets::value::schema::snapshot::SemioValueSnapshot, subsets::value::schema::mutations::SemioValueMutation),
        Video("s.stdio.semio", "v1", "video", "stdio.semio") => (subsets::video::schema::snapshot::SemioVideoSnapshot, subsets::video::schema::mutations::SemioVideoMutation),
    }
}

/// 🏭️ Mints a typed Semio child through its closed full-dialect factory.
pub async fn create_semio_member(id: &str, dialect: &dsl::os_io::ArtifactDialect, initial_pack: &[u8]) -> Result<SemioMembers, dsl::VcsError> {
    <SemioMembers as dsl::MemberFactory>::create(id, dialect, initial_pack).await
}

/// 📤️ Reopens a Semio member only when its persisted schema and dialect match the requested binding.
pub async fn open_semio_member(expected: &dsl::os_io::ArtifactRef, owner: Option<&dsl::OwnerRef>, envelope_pack: &[u8]) -> Result<SemioMembers, dsl::VcsError> {
    <SemioMembers as dsl::MemberFactory>::open(expected, owner, envelope_pack).await
}
//#endregion 🔖️Members

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v1::subsets::base::io::io_registry as v1;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("SemioComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v1::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod animation {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod duration {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/💡️inferences/⏱️duration/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
            }
            #[path = "."]
            pub mod base {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod kind {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/💡️inferences/🏷️kind/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧮️geometry/🦀️.rs"]
                    pub mod geometry;
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🪆️child/🦀️.rs"]
                    pub mod child;
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧰️triples/🦀️.rs"]
                    pub mod triples;
                }
            }
            #[path = "."]
            pub mod audio {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod duration {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/💡️inferences/⏱️duration/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
            }
            #[path = "."]
            pub mod brep {
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    #[cfg(feature = "conversion-brep")]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod step {
                                    #[path = "."]
                                    pub mod v_ap214 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    #[cfg(feature = "conversion-brep")]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod step {
                                    #[path = "."]
                                    pub mod v_ap214 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/⚙️engine/🦀️.rs"]
                    pub mod engine;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🏟️arena/🦀️.rs"]
                        pub mod arena;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🔁️body/🦀️.rs"]
                        pub mod body;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/➰️curve/🦀️.rs"]
                        pub mod curve;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🚨️error/🦀️.rs"]
                        pub mod error;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/〰️polynomial/🦀️.rs"]
                        pub mod polynomial;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🏄️surface/🦀️.rs"]
                        pub mod surface;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/📏️tolerance/🦀️.rs"]
                        pub mod tolerance;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🕸️topology/🦀️.rs"]
                        pub mod topology;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/➡️vector/🦀️.rs"]
                        pub mod vector;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🎨️blend/🦀️.rs"]
                        pub mod blend;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🔀️boolean/🦀️.rs"]
                        pub mod boolean;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🔺️euler/🦀️.rs"]
                        pub mod euler;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/✂️intersect/🦀️.rs"]
                        pub mod intersect;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/↔️offset/🦀️.rs"]
                        pub mod offset;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🧱️primitives/🦀️.rs"]
                        pub mod primitives;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🧵️sew/🦀️.rs"]
                        pub mod sew;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/➡️sweep/🦀️.rs"]
                        pub mod sweep;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🔁️transform/🦀️.rs"]
                        pub mod transform;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod validation_report {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/✅validation-report/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/🌳bounding-volume/🦀️.rs"]
                        pub mod bounding_volume;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/🏷️classification/🦀️.rs"]
                        pub mod classification;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/📏mass-properties/🦀️.rs"]
                        pub mod mass_properties;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs"]
                        pub mod tessellation;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "."]
                        pub mod delete_edge {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/✂️delete-edge/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/✂️delete-edge/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/✂️delete-edge/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_curve {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/➰replace-curve/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/➰replace-curve/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/➰replace-curve/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_vertex {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🏗️create-vertex/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🏗️create-vertex/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🏗️create-vertex/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_shell {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🐚create-shell/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🐚create-shell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🐚create-shell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_shell {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/💥delete-shell/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/💥delete-shell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/💥delete-shell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod move_vertex {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/📍move-vertex/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/📍move-vertex/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/📍move-vertex/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_edge {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🖇️create-edge/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🖇️create-edge/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🖇️create-edge/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_face {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🔷create-face/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🔷create-face/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🔷create-face/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_solid {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🕳️delete-solid/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🕳️delete-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🕳️delete-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_vertex {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🗑️delete-vertex/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🗑️delete-vertex/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🗑️delete-vertex/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_surface {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🗺️replace-surface/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🗺️replace-surface/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🗺️replace-surface/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_face {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🚮delete-face/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🚮delete-face/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🚮delete-face/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_solid {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🧊create-solid/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🧊create-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/🧊create-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
            }
            #[path = "."]
            pub mod cad {
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    #[cfg(feature = "conversion-cad")]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod dxf {
                                    #[path = "."]
                                    pub mod v_r12 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔄️dxf/🔖️r12/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1024 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod step {
                                    #[path = "."]
                                    pub mod v_ap214 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    #[cfg(feature = "conversion-cad")]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod dxf {
                                    #[path = "."]
                                    pub mod v_r12 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔄️dxf/🔖️r12/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1024 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod step {
                                    #[path = "."]
                                    pub mod v_ap214 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod bounds {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
            }
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    #[cfg(feature = "conversion-document")]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod docx {
                                    #[path = "."]
                                    pub mod v_ecma_376 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_7 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    #[cfg(feature = "conversion-document")]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod docx {
                                    #[path = "."]
                                    pub mod v_ecma_376 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_7 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod outline {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
            }
            #[path = "."]
            pub mod drawing {
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    #[cfg(feature = "conversion-drawing")]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod svg {
                                    #[path = "."]
                                    pub mod v1_1 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dxf {
                                    #[path = "."]
                                    pub mod v_r12 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔄️dxf/🔖️r12/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_7 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1024 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    #[cfg(feature = "conversion-drawing")]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod svg {
                                    #[path = "."]
                                    pub mod v1_1 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dxf {
                                    #[path = "."]
                                    pub mod v_r12 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔄️dxf/🔖️r12/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_7 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1024 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod flattened_scene {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/💡️inferences/🎛️flattened-scene/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "."]
                        pub mod create_node {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/➕create-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/➕create-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/➕create-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_node {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/➖delete-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/➖delete-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/➖delete-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_layer {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🌱create-layer/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🌱create-layer/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🌱create-layer/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod unflatten_node {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🎈unflatten-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🎈unflatten-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🎈unflatten-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod ungroup_node {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/💫ungroup-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/💫ungroup-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/💫ungroup-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod move_node {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/📍move-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/📍move-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/📍move-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod scale_node {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/📏scale-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/📏scale-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/📏scale-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_stroke_width {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/📐change-stroke-width/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/📐change-stroke-width/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/📐change-stroke-width/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod reorder_nodes {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🔀reorder-nodes/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🔀reorder-nodes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🔀reorder-nodes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod rotate_node {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🔄rotate-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🔄rotate-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🔄rotate-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_stroke_color {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🖌️change-stroke-color/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🖌️change-stroke-color/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🖌️change-stroke-color/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod drag_nodes {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🖐️drag-nodes/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🖐️drag-nodes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🖐️drag-nodes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_layer {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🗑️delete-layer/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🗑️delete-layer/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🗑️delete-layer/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_path {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🛤️replace-path/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🛤️replace-path/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🛤️replace-path/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod group_nodes {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🧷group-nodes/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🧷group-nodes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🧷group-nodes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_fill {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🪣replace-fill/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🪣replace-fill/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🪣replace-fill/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod flatten_node {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🫓flatten-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🫓flatten-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🫓flatten-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
            }
            #[path = "."]
            pub mod image {
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    #[cfg(feature = "conversion-image")]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod jpg {
                                    #[path = "."]
                                    pub mod v_jfif_1_01 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📸️jpg/🔖️jfif-1.01/♾️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod gif {
                                    #[path = "."]
                                    pub mod v89a {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎞️gif/🔖️89a/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod bmp {
                                    #[path = "."]
                                    pub mod v_v3 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod tiff {
                                    #[path = "."]
                                    pub mod v6_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖼️tiff/🔖️6.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    #[cfg(feature = "conversion-image")]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod jpg {
                                    #[path = "."]
                                    pub mod v_jfif_1_01 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📸️jpg/🔖️jfif-1.01/♾️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod gif {
                                    #[path = "."]
                                    pub mod v89a {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️gif/🔖️89a/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod bmp {
                                    #[path = "."]
                                    pub mod v_v3 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod tiff {
                                    #[path = "."]
                                    pub mod v6_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖼️tiff/🔖️6.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod dimensions {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/💡️inferences/📐dimensions/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
            }
            #[path = "."]
            pub mod mesh {
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    #[cfg(feature = "conversion-mesh")]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod gltf {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎬️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod stl {
                                    #[path = "."]
                                    pub mod v_ascii {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod obj {
                                    #[path = "."]
                                    pub mod v3_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod ply {
                                    #[path = "."]
                                    pub mod v1_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod las {
                                    #[path = "."]
                                    pub mod v1_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1024 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    #[cfg(feature = "conversion-mesh")]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod gltf {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎬️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod stl {
                                    #[path = "."]
                                    pub mod v_ascii {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod obj {
                                    #[path = "."]
                                    pub mod v3_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod ply {
                                    #[path = "."]
                                    pub mod v1_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod las {
                                    #[path = "."]
                                    pub mod v1_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1024 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod aabb {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/💡️inferences/📦aabb/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "."]
                        pub mod create_mesh {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🕸️create-mesh/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🕸️create-mesh/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🕸️create-mesh/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_mesh {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🗑️delete-mesh/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🗑️delete-mesh/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🗑️delete-mesh/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_primitive {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🔺create-primitive/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🔺create-primitive/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🔺create-primitive/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_primitive {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/✂️delete-primitive/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/✂️delete-primitive/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/✂️delete-primitive/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod set_primitive_topology {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🔀set-primitive-topology/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🔀set-primitive-topology/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🔀set-primitive-topology/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_primitive_geometry {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/📐replace-primitive-geometry/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/📐replace-primitive-geometry/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/📐replace-primitive-geometry/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod set_primitive_material {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🧲️set-primitive-material/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🧲️set-primitive-material/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🧲️set-primitive-material/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_material {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🎨create-material/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🎨create-material/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🎨create-material/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_material {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🚮delete-material/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🚮delete-material/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🚮delete-material/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_material_base_color {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🌈change-material-base-color/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🌈change-material-base-color/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🌈change-material-base-color/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_material_metallic {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/⚙️change-material-metallic/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/⚙️change-material-metallic/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/⚙️change-material-metallic/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_material_roughness {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🧱change-material-roughness/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🧱change-material-roughness/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🧱change-material-roughness/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_texture {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🖼️create-texture/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🖼️create-texture/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🖼️create-texture/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_texture {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🕳️delete-texture/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🕳️delete-texture/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🕳️delete-texture/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_texture_mime {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🏷️change-texture-mime/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🏷️change-texture-mime/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/🏷️change-texture-mime/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_texture_bytes {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/📀replace-texture-bytes/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/📀replace-texture-bytes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/📀replace-texture-bytes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod move_vertex {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/📍move-vertex/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/📍move-vertex/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/📍move-vertex/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
                #[path = "."]
                pub mod examples {
                    #[path = "."]
                    pub mod cube {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/📚️examples/🧊️cube/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod model {
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    #[cfg(feature = "conversion-model")]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod ifc {
                                    #[path = "."]
                                    pub mod v4 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🏗️ifc/🔖️4/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod bcf {
                                    #[path = "."]
                                    pub mod v2_1 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💬️bcf/🔖️2.1/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    #[cfg(feature = "conversion-model")]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod ifc {
                                    #[path = "."]
                                    pub mod v4 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🏗️ifc/🔖️4/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod bcf {
                                    #[path = "."]
                                    pub mod v2_1 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💬️bcf/🔖️2.1/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod bounds {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
            }
            #[path = "."]
            pub mod value {
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    #[cfg(feature = "conversion-value")]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod xml {
                                    #[path = "."]
                                    pub mod v1_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    #[cfg(feature = "conversion-value")]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod xml {
                                    #[path = "."]
                                    pub mod v1_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/➕️algebra-internals/🦀️.rs"]
                    pub mod algebra_internals;
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🌫️fuzzy-internals/🦀️.rs"]
                    pub mod fuzzy_internals;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod census {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/💡️inferences/🌳census/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
            }
            #[path = "."]
            pub mod presentation {
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    #[cfg(feature = "conversion-presentation")]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod pptx {
                                    #[path = "."]
                                    pub mod v_ecma_376 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    #[cfg(feature = "conversion-presentation")]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod pptx {
                                    #[path = "."]
                                    pub mod v_ecma_376 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod outline {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
            }
            #[path = "."]
            pub mod video {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod duration {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/💡️inferences/⏱️duration/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
            }
            #[path = "."]
            pub mod flow {
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    #[cfg(feature = "conversion-flow")]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    #[cfg(feature = "conversion-flow")]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod topology {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
            }
            #[path = "."]
            pub mod text {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod profile {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/💡️inferences/📊profile/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod insert_run {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/📥insert-run/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/📥insert-run/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/📥insert-run/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod remove_run {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/🗑️remove-run/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/🗑️remove-run/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/🗑️remove-run/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod edit_run {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/✏️edit-run/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/✏️edit-run/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/✏️edit-run/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_run_language {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/🌐change-run-language/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/🌐change-run-language/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/🌐change-run-language/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod reorder_runs {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/🔀reorder-runs/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/🔀reorder-runs/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/🔀reorder-runs/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod add_mark {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/➕add-mark/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/➕add-mark/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/➕add-mark/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod remove_mark {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/➖remove-mark/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/➖remove-mark/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/➖remove-mark/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod table {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧭️causal-internals/🦀️.rs"]
                    pub mod causal_internals;
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🌀️entropy-internals/🦀️.rs"]
                    pub mod entropy_internals;
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🎲️probability-internals/🦀️.rs"]
                    pub mod probability_internals;
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📊️statistics-internals/🦀️.rs"]
                    pub mod statistics_internals;
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📋️tabular-internals/🦀️.rs"]
                    pub mod tabular_internals;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod shape {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/💡️inferences/📐shape/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod moments {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/💡️inferences/📊moments/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod entropy {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/💡️inferences/🎲entropy/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod create_column {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🏗️create-column/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🏗️create-column/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🏗️create-column/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_column {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🗑️delete-column/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🗑️delete-column/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🗑️delete-column/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod rename_column {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🏷️rename-column/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🏷️rename-column/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🏷️rename-column/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod reorder_columns {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🔀reorder-columns/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🔀reorder-columns/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🔀reorder-columns/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod insert_row {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/📥insert-row/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/📥insert-row/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/📥insert-row/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod remove_row {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/➖remove-row/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/➖remove-row/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/➖remove-row/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod reorder_rows {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🔃reorder-rows/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🔃reorder-rows/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🔃reorder-rows/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod edit_cell {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/✏️edit-cell/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/✏️edit-cell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/✏️edit-cell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
                #[path = "."]
                pub mod examples {
                    #[path = "."]
                    pub mod sheet {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/📚️examples/📃️sheet/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod graph {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🔧️operators-internals/🦀️.rs"]
                    pub mod operators_internals;
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🚶️traversal-internals/🦀️.rs"]
                    pub mod traversal_internals;
                    #[path = "."]
                    pub mod normal_internals {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/➕️normal-internals/➡️directed/🦀️.rs"]
                        pub mod directed;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/➕️normal-internals/↔️undirected/🦀️.rs"]
                        pub mod undirected;
                    }
                    #[path = "."]
                    pub mod ports_internals {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🔌️ports-internals/↔️undirected/🦀️.rs"]
                        pub mod undirected;
                        #[path = "."]
                        pub mod directed {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🔌️ports-internals/➡️directed/➕️normal/🦀️.rs"]
                            pub mod normal;
                        }
                    }
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod topology {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod connectivity {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/💡️inferences/🕸️connectivity/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod create_node {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🏗️create-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🏗️create-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🏗️create-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_node {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🗑️delete-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🗑️delete-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🗑️delete-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_node_kind {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔧change-node-kind/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔧change-node-kind/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔧change-node-kind/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_node_label {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🖍️change-node-label/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🖍️change-node-label/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🖍️change-node-label/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod move_node {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/📍move-node/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/📍move-node/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/📍move-node/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod add_node_port {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔌add-node-port/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔌add-node-port/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔌add-node-port/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod remove_node_port {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔚remove-node-port/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔚remove-node-port/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🔚remove-node-port/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod add_node_property {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/➕add-node-property/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/➕add-node-property/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/➕add-node-property/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod remove_node_property {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/➖remove-node-property/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/➖remove-node-property/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/➖remove-node-property/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_edge {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🌉️create-edge/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🌉️create-edge/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🌉️create-edge/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_edge {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/✂️delete-edge/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/✂️delete-edge/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/✂️delete-edge/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
                #[path = "."]
                pub mod examples {
                    #[path = "."]
                    pub mod wires {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/📚️examples/🕸️wires/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod object {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod composition {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/💡️inferences/🧩composition/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod move_object {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🚚move-object/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🚚move-object/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🚚move-object/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod rotate_object {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🔄rotate-object/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🔄rotate-object/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🔄rotate-object/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod scale_object {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/📏scale-object/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/📏scale-object/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/📏scale-object/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_brep {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🧱create-brep/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🧱create-brep/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🧱create-brep/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_brep {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/💥delete-brep/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/💥delete-brep/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/💥delete-brep/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_mesh {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🕸️create-mesh/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🕸️create-mesh/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🕸️create-mesh/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_mesh {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🧨delete-mesh/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🧨delete-mesh/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🧨delete-mesh/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_properties {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🏷️create-properties/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🏷️create-properties/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🏷️create-properties/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_properties {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🚫delete-properties/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🚫delete-properties/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/🚫delete-properties/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
                #[path = "."]
                pub mod examples {
                    #[path = "."]
                    pub mod crate_ {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/📚️examples/📦️crate/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod kit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod entries {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/💡️inferences/🗃️entries/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod create_object {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🏗️create-object/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🏗️create-object/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🏗️create-object/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_object {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🪓delete-object/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🪓delete-object/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🪓delete-object/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_model {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🏛️create-model/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🏛️create-model/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🏛️create-model/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_model {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/💣delete-model/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/💣delete-model/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/💣delete-model/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod create_properties {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🏷️create-properties/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🏷️create-properties/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🏷️create-properties/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod delete_properties {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🚫delete-properties/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🚫delete-properties/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🚫delete-properties/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod bind_representation {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🪢️bind-representation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🪢️bind-representation/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🪢️bind-representation/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod unbind_representation {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/✂️unbind-representation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/✂️unbind-representation/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/✂️unbind-representation/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_representation_pin {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/📌change-representation-pin/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/📌change-representation-pin/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/📌change-representation-pin/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod add_type {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/➕add-type/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/➕add-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/➕add-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod remove_type {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/➖remove-type/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/➖remove-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/➖remove-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod rename_type {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/✏️rename-type/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/✏️rename-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/✏️rename-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod add_design {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🆕add-design/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🆕add-design/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🆕add-design/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod remove_design {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🗑️remove-design/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🗑️remove-design/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🗑️remove-design/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod edit_design {
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🖊️edit-design/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🖊️edit-design/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/🖊️edit-design/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
                #[path = "."]
                pub mod examples {
                    #[path = "."]
                    pub mod furniture {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/📚️examples/🪑️furniture/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/📚️examples/🎬️demo/🦀️.rs"]
    pub mod demo;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/📚️examples/📃️note/🦀️.rs"]
    pub mod note;
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod semio_animation {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_base {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_audio {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_brep {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_cad {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_document {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_drawing {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_flow {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_graph {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_image {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_kit {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_mesh {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_model {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_object {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_presentation {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_table {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_text {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_value {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_video {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod semio_animation {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_base {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_audio {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_brep {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_cad {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_document {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_drawing {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_flow {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_graph {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_image {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_kit {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_mesh {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_model {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_object {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_presentation {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_table {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_text {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_value {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod semio_video {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
