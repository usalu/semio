//! 🌐️ `stdio.semio` artifact root — the inbuilt semio semantic artifact (standard `v1`, 13
//! schema-owning domain subsets + the `✳️any` envelope union). See the master plan's
//! "Architecture > The semio artifact" section.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::semio::standards::v1::subsets::any::schema::SemioArtifact;
pub use crate::artifacts::semio::standards::v1::subsets::any::schema::diff::SemioDiff;
pub use crate::artifacts::semio::standards::v1::subsets::any::schema::mutations::SemioMutation;
pub use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::SemioSnapshot;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_SEMIO_DOCUMENT_SCHEMA: &str = "stdio.semio";

/// 🧬️ Artifact schema descriptor id.
pub const SEMIO_ARTIFACT_SCHEMA_ID: &str = "s.stdio.semio";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::definition_only_assembly("semio", definition)
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
/// 🗂️ Registers all 19 of `v1`'s subsets' IO composers (14 domain subsets + `text` + `✳️any`
/// itself) — dissolved out of the former standard-level `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES). `semio` is one of stdio's 10
/// deliberate imperative-`register()` artifacts (never converted to the `ArtifactDeclaration`
/// builder pattern, per `crate::plugin()`'s own call — unchanged in call order/behavior, only
/// the function's file moved with the deleted directory).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    crate::artifacts::semio::standards::v1::subsets::brep::io::register();
    crate::artifacts::semio::standards::v1::subsets::mesh::io::register();
    crate::artifacts::semio::standards::v1::subsets::model::io::register();
    crate::artifacts::semio::standards::v1::subsets::value::io::register();
    crate::artifacts::semio::standards::v1::subsets::document::io::register();
    crate::artifacts::semio::standards::v1::subsets::cad::io::register();
    crate::artifacts::semio::standards::v1::subsets::drawing::io::register();
    crate::artifacts::semio::standards::v1::subsets::image::io::register();
    crate::artifacts::semio::standards::v1::subsets::video::io::register();
    crate::artifacts::semio::standards::v1::subsets::audio::io::register();
    crate::artifacts::semio::standards::v1::subsets::animation::io::register();
    crate::artifacts::semio::standards::v1::subsets::presentation::io::register();
    crate::artifacts::semio::standards::v1::subsets::flow::io::register();
    crate::artifacts::semio::standards::v1::subsets::text::io::register();
    crate::artifacts::semio::standards::v1::subsets::table::io::register();
    crate::artifacts::semio::standards::v1::subsets::graph::io::register();
    crate::artifacts::semio::standards::v1::subsets::object::io::register();
    crate::artifacts::semio::standards::v1::subsets::kit::io::register();
    crate::artifacts::semio::standards::v1::subsets::any::io::register();
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

use crate::artifacts::semio::standards::v1::subsets;

//#region 🧹️SnapshotRetirement
use std::{marker::PhantomData, sync::Arc};

trait RetireOwned: Send + 'static {
    fn retirement(self) -> Box<dyn RetirementCursor>;
}

enum RetirementStep {
    Child(Box<dyn RetirementCursor>),
    Bytes(usize),
    Complete,
    BudgetExhausted,
}

trait RetirementCursor: Send {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep;
}

struct Leaf<T: Copy + Send + 'static>(Option<T>, usize);

impl<T: Copy + Send + 'static> RetirementCursor for Leaf<T> {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
        if self.1 > 0 {
            if maximum_bytes == 0 {
                return RetirementStep::BudgetExhausted;
            }
            let bytes = maximum_bytes.min(self.1);
            self.1 -= bytes;
            return RetirementStep::Bytes(bytes);
        }
        self.0.take();
        RetirementStep::Complete
    }
}

macro_rules! retire_leaf {
    ($($type:ty),+ $(,)?) => {$ (
        impl RetireOwned for $type {
            fn retirement(self) -> Box<dyn RetirementCursor> {
                Box::new(Leaf(Some(self), std::mem::size_of::<Self>()))
            }
        }
    )+ };
}

retire_leaf!(bool, u8, u32, u64, i32, i64, f32, f64);

struct Bytes(Vec<u8>);

impl RetirementCursor for Bytes {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
        if self.0.is_empty() {
            return RetirementStep::Complete;
        }
        if maximum_bytes == 0 {
            return RetirementStep::BudgetExhausted;
        }
        let bytes = maximum_bytes.min(self.0.len());
        self.0.truncate(self.0.len() - bytes);
        RetirementStep::Bytes(bytes)
    }
}

impl RetireOwned for String {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(Bytes(self.into_bytes()))
    }
}

struct Collection<T: RetireOwned>(Vec<T>);

impl<T: RetireOwned> RetirementCursor for Collection<T> {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        self.0.pop().map_or(RetirementStep::Complete, |value| RetirementStep::Child(value.retirement()))
    }
}

impl<T: RetireOwned> RetireOwned for Vec<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(Collection(self))
    }
}

impl<T: RetireOwned> RetireOwned for Option<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        self.map_or_else(|| sequence(Vec::new()), RetireOwned::retirement)
    }
}

struct Sequence(Vec<Box<dyn RetirementCursor>>);

impl RetirementCursor for Sequence {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        self.0.pop().map_or(RetirementStep::Complete, RetirementStep::Child)
    }
}

fn sequence(fields: Vec<Box<dyn RetirementCursor>>) -> Box<dyn RetirementCursor> {
    Box::new(Sequence(fields))
}

macro_rules! seq {
    ($($field:expr),* $(,)?) => { sequence(vec![$(RetireOwned::retirement($field)),*]) };
}

macro_rules! retire_struct {
    ($type:ty { $($field:ident),+ $(,)? }) => {
        impl RetireOwned for $type {
            fn retirement(self) -> Box<dyn RetirementCursor> {
                let Self { $($field),+ } = self;
                seq![$($field),+]
            }
        }
    };
}

use subsets::any::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform, SemioUv};
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

retire_struct!(brep::BrepVertex { id, point });
retire_struct!(brep::BrepEdge { id, start_vertex, end_vertex, curve });
retire_struct!(brep::BrepLoopEdge { edge, orientation });
retire_struct!(brep::BrepLoop { id, edges });
retire_struct!(brep::BrepFace { id, outer_loop, inner_loops, surface, orientation });
retire_struct!(brep::BrepShellFace { face, orientation });
retire_struct!(brep::BrepShell { id, faces });
retire_struct!(brep::BrepSolidShell { shell, is_void });
retire_struct!(brep::BrepSolid { id, shells });
retire_struct!(brep::SemioBrepSnapshot { schema, vertices, edges, loops, faces, shells, solids });
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

impl RetireOwned for dsl::os_io::ArtifactDialect {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let Self { artifact_kind, standard, subset } = self;
        seq![artifact_kind, standard, subset]
    }
}
impl RetireOwned for dsl::os_io::ArtifactRef {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let Self { artifact_id, dialect } = self;
        seq![artifact_id, dialect]
    }
}
impl<S: Send + 'static> RetireOwned for dsl::ArtifactChild<S> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let child_id = self.child_id;
        let target = self.target;
        seq![child_id, target]
    }
}
retire_struct!(dsl::BlobRef { hash, size, media_type });
retire_struct!(dsl::ArtifactLink { target, pin, role });
impl RetireOwned for dsl::LinkPin {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Head => seq![],
            Self::Checkpoint { id } => id.retirement(),
            Self::Snapshot { blob } => blob.retirement(),
        }
    }
}

struct SemioSnapshotRetirement<P: RetireOwned> {
    snapshot: Option<Arc<P>>,
    cursors: Vec<Box<dyn RetirementCursor>>,
}
impl<P: RetireOwned + Sync> dsl::ErasedSnapshotRetirement for SemioSnapshotRetirement<P> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<dsl::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let (mut turns, mut released_items, mut released_bytes) = (0, 0, 0);
        if let Some(snapshot) = self.snapshot.take() {
            if Arc::strong_count(&snapshot) > 1 {
                self.snapshot = Some(snapshot);
                return Ok(dsl::SnapshotRetirementStep::Blocked);
            }
            turns += 1;
            match Arc::try_unwrap(snapshot) {
                Ok(snapshot) => self.cursors.push(snapshot.retirement()),
                Err(shared) => {
                    self.snapshot = Some(shared);
                    return Ok(dsl::SnapshotRetirementStep::Blocked);
                }
            }
        }
        while turns < maximum_items {
            let Some(cursor) = self.cursors.last_mut() else { return Ok(dsl::SnapshotRetirementStep::Complete) };
            match cursor.close_step(maximum_bytes - released_bytes) {
                RetirementStep::Child(child) => self.cursors.push(child),
                RetirementStep::Bytes(bytes) => released_bytes += bytes,
                RetirementStep::Complete => {
                    self.cursors.pop();
                    released_items += 1;
                }
                RetirementStep::BudgetExhausted => break,
            }
            turns += 1;
        }
        if self.terminal_is_empty() { Ok(dsl::SnapshotRetirementStep::Complete) } else { Ok(dsl::SnapshotRetirementStep::Pending { released_items, released_bytes }) }
    }
    fn terminal_is_empty(&self) -> bool {
        self.snapshot.is_none() && self.cursors.is_empty()
    }
}
struct SemioSnapshotRetirementFactory<P>(PhantomData<fn() -> P>);
macro_rules! factories { ($($snapshot:ty),+ $(,)?) => {$ (impl dsl::SnapshotRetirementFactory<$snapshot> for SemioSnapshotRetirementFactory<$snapshot>{fn retire(&self,snapshot:Arc<$snapshot>)->Box<dyn dsl::ErasedSnapshotRetirement>{Box::new(SemioSnapshotRetirement{snapshot:Some(snapshot),cursors:Vec::new()})}})+}; }
factories!(
    animation::SemioAnimationSnapshot,
    audio::SemioAudioSnapshot,
    brep::SemioBrepSnapshot,
    cad::SemioCadSnapshot,
    document::SemioDocumentSnapshot,
    drawing::SemioDrawingSnapshot,
    flow::SemioFlowSnapshot,
    graph::SemioGraphSnapshot,
    image::SemioImageSnapshot,
    kit::SemioKitSnapshot,
    mesh::SemioMeshSnapshot,
    model::SemioModelSnapshot,
    object::SemioObjectSnapshot,
    presentation::SemioPresentationSnapshot,
    table::SemioTableSnapshot,
    text::SemioTextSnapshot,
    value::SemioValueSnapshot,
    video::SemioVideoSnapshot
);

fn install_semio_snapshot_retirement(member: &mut SemioMembers) -> Result<(), dsl::VcsError> {
    macro_rules! install {
        ($store:expr, $snapshot:ty) => {
            $store.install_snapshot_retirement_factory(Arc::new(SemioSnapshotRetirementFactory::<$snapshot>(PhantomData)))
        };
    }
    match member {
        SemioMembers::Animation(store) => install!(store, animation::SemioAnimationSnapshot),
        SemioMembers::Audio(store) => install!(store, audio::SemioAudioSnapshot),
        SemioMembers::Brep(store) => install!(store, brep::SemioBrepSnapshot),
        SemioMembers::Cad(store) => install!(store, cad::SemioCadSnapshot),
        SemioMembers::Document(store) => install!(store, document::SemioDocumentSnapshot),
        SemioMembers::Drawing(store) => install!(store, drawing::SemioDrawingSnapshot),
        SemioMembers::Flow(store) => install!(store, flow::SemioFlowSnapshot),
        SemioMembers::Graph(store) => install!(store, graph::SemioGraphSnapshot),
        SemioMembers::Image(store) => install!(store, image::SemioImageSnapshot),
        SemioMembers::Kit(store) => install!(store, kit::SemioKitSnapshot),
        SemioMembers::Mesh(store) => install!(store, mesh::SemioMeshSnapshot),
        SemioMembers::Model(store) => install!(store, model::SemioModelSnapshot),
        SemioMembers::Object(store) => install!(store, object::SemioObjectSnapshot),
        SemioMembers::Presentation(store) => install!(store, presentation::SemioPresentationSnapshot),
        SemioMembers::Table(store) => install!(store, table::SemioTableSnapshot),
        SemioMembers::Text(store) => install!(store, text::SemioTextSnapshot),
        SemioMembers::Value(store) => install!(store, value::SemioValueSnapshot),
        SemioMembers::Video(store) => install!(store, video::SemioVideoSnapshot),
    }
}
//#endregion 🧹️SnapshotRetirement

/// 🧬️ Closed set spanning `semio`'s 18 composable subsets — the O1 replacement for the deleted
/// `Box<dyn SpaceMember>` `ChildStoreFactory` registry (`store::MemberFactory`'s own doc explains the
/// general mechanism; `store::space_members!` generates the `SpaceMember`/`MemberFactory` delegation
/// below). Generated exactly as any other family would be, EXCEPT the string fed to `MemberFactory::
/// create`/`open` at the two call sites below is `dialect.subset` — never `dialect.artifact_kind` —
/// because all 18 variants share the SAME kind (`s.stdio.semio`, `SEMIO_ARTIFACT_SCHEMA_ID` above) and
/// differ only by subset. Nothing in `space_members!` requires its `kind: &str` parameter to actually
/// BE an `ArtifactKindId`; it only ever compares it against string literals, so this reuse is exact,
/// not approximate.
dsl::space_members! {
    pub enum SemioMembers {
        Animation("animation", "stdio.semio") => dsl::ArtifactStore<subsets::animation::schema::snapshot::SemioAnimationSnapshot, subsets::animation::schema::mutations::SemioAnimationMutation>,
        Audio("audio", "stdio.semio") => dsl::ArtifactStore<subsets::audio::schema::snapshot::SemioAudioSnapshot, subsets::audio::schema::mutations::SemioAudioMutation>,
        Brep("brep", "stdio.semio") => dsl::ArtifactStore<subsets::brep::schema::snapshot::SemioBrepSnapshot, subsets::brep::schema::mutations::SemioBrepMutation>,
        Cad("cad", "stdio.semio") => dsl::ArtifactStore<subsets::cad::schema::snapshot::SemioCadSnapshot, subsets::cad::schema::mutations::SemioCadMutation>,
        Document("document", "stdio.semio") => dsl::ArtifactStore<subsets::document::schema::snapshot::SemioDocumentSnapshot, subsets::document::schema::mutations::SemioDocumentMutation>,
        Drawing("drawing", "stdio.semio") => dsl::ArtifactStore<subsets::drawing::schema::snapshot::SemioDrawingSnapshot, subsets::drawing::schema::mutations::SemioDrawingMutation>,
        Flow("flow", "stdio.semio") => dsl::ArtifactStore<subsets::flow::schema::snapshot::SemioFlowSnapshot, subsets::flow::schema::mutations::SemioFlowMutation>,
        Graph("graph", "stdio.semio") => dsl::ArtifactStore<subsets::graph::schema::snapshot::SemioGraphSnapshot, subsets::graph::schema::mutations::SemioGraphMutation>,
        Image("image", "stdio.semio") => dsl::ArtifactStore<subsets::image::schema::snapshot::SemioImageSnapshot, subsets::image::schema::mutations::SemioImageMutation>,
        Kit("kit", "stdio.semio") => dsl::ArtifactStore<subsets::kit::schema::snapshot::SemioKitSnapshot, subsets::kit::schema::mutations::SemioKitMutation>,
        Mesh("mesh", "stdio.semio") => dsl::ArtifactStore<subsets::mesh::schema::snapshot::SemioMeshSnapshot, subsets::mesh::schema::mutations::SemioMeshMutation>,
        Model("model", "stdio.semio") => dsl::ArtifactStore<subsets::model::schema::snapshot::SemioModelSnapshot, subsets::model::schema::mutations::SemioModelMutation>,
        Object("object", "stdio.semio") => dsl::ArtifactStore<subsets::object::schema::snapshot::SemioObjectSnapshot, subsets::object::schema::mutations::SemioObjectMutation>,
        Presentation("presentation", "stdio.semio") => dsl::ArtifactStore<subsets::presentation::schema::snapshot::SemioPresentationSnapshot, subsets::presentation::schema::mutations::SemioPresentationMutation>,
        Table("table", "stdio.semio") => dsl::ArtifactStore<subsets::table::schema::snapshot::SemioTableSnapshot, subsets::table::schema::mutations::SemioTableMutation>,
        Text("text", "stdio.semio") => dsl::ArtifactStore<subsets::text::schema::snapshot::SemioTextSnapshot, subsets::text::schema::mutations::SemioTextMutation>,
        Value("value", "stdio.semio") => dsl::ArtifactStore<subsets::value::schema::snapshot::SemioValueSnapshot, subsets::value::schema::mutations::SemioValueMutation>,
        Video("video", "stdio.semio") => dsl::ArtifactStore<subsets::video::schema::snapshot::SemioVideoSnapshot, subsets::video::schema::mutations::SemioVideoMutation>,
    }
}

/// 🏭️ Mints a new subset-typed `semio` child — the `create` half of the removed `ChildStoreFactory`.
/// Dispatch key is `dialect.subset` (see [`SemioMembers`]'s doc).
pub async fn create_semio_member(id: &str, dialect: &dsl::os_io::ArtifactDialect, initial_pack: &[u8]) -> Result<SemioMembers, dsl::VcsError> {
    let mut member = <SemioMembers as dsl::MemberFactory>::create(dialect.subset.as_str(), id, dialect, initial_pack).await?;
    install_semio_snapshot_retirement(&mut member)?;
    Ok(member)
}

/// 📤️ Reopens a persisted subset-typed `semio` child — the `open` half. The subset is recovered from
/// the envelope itself (`subset_of_persisted_envelope`), exactly as the removed `ChildStoreFactory::
/// open` did — `open` gets no dialect argument, so it has to; this only works because the `.spr`
/// composition overlay carries `dialect` (see this ticket's `REC_COMPOSITION`).
pub async fn open_semio_member(envelope_pack: &[u8]) -> Result<SemioMembers, dsl::VcsError> {
    let subset = subset_of_persisted_envelope(envelope_pack).await?;
    let mut member = <SemioMembers as dsl::MemberFactory>::open(subset.as_str(), envelope_pack).await?;
    install_semio_snapshot_retirement(&mut member)?;
    Ok(member)
}

/// 🎯️ Reads a persisted child's subset out of its own `.spr` composition overlay — deliberately
/// snapshot-type-agnostic (it decodes only the history log, never the document body), because
/// choosing the snapshot type is exactly what this answer is needed FOR.
async fn subset_of_persisted_envelope(envelope_pack: &[u8]) -> Result<String, dsl::VcsError> {
    let (_, spr) = dsl::decode_document_pack_bytes(envelope_pack).await?;
    let log = dsl::decode_history(&spr, &dsl::os_spr::DecodeOptions::default()).await.map_err(|error| dsl::VcsError::Deserialize(error.to_string()))?;
    log.composition.and_then(|composition| composition.dialect).map(|(_, _, subset)| subset).ok_or_else(|| dsl::VcsError::Deserialize("semio child store: persisted child carries no dialect, so its subset is unknowable".to_string()))
}
//#endregion 🔖️Members

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::semio::standards::v1::subsets::any::io::io_registry as v1;
    use semio_framework_plugin::{ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource, register_composer_entries};
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
        let _ = register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
    use crate::dsl::{ArtifactPack, SpaceMember, os_io::ArtifactDialect};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn subset_dialect(subset: &str) -> ArtifactDialect {
        ArtifactDialect { artifact_kind: SEMIO_ARTIFACT_SCHEMA_ID.into(), standard: "v1".into(), subset: subset.into() }
    }

    /// 🧹️ A large nested snapshot is dismantled over bounded turns; completion is accepted only
    /// after the concrete cursor has surrendered every owned node and byte.
    #[test]
    fn large_nested_snapshot_retirement_is_multi_turn_and_terminal_empty() {
        let snapshot = text::SemioTextSnapshot { schema: "stdio.semio.text".into(), runs: (0..128).map(|index| text::SemioTextRun { language: format!("language-{index}"), content: "payload".repeat(128), marks: Vec::new() }).collect() };
        let factory = SemioSnapshotRetirementFactory::<text::SemioTextSnapshot>(PhantomData);
        let mut retirement = dsl::SnapshotRetirementFactory::retire(&factory, Arc::new(snapshot));
        let mut turns = 0;
        loop {
            turns += 1;
            match retirement.close_step(7, 31).expect("bounded close step") {
                dsl::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 7);
                    assert!(released_bytes <= 31);
                }
                dsl::SnapshotRetirementStep::Blocked => panic!("owned snapshot retirement has no external wait"),
                dsl::SnapshotRetirementStep::Complete => break,
            }
            assert!(turns < 20_000, "retirement stopped making progress");
        }
        assert!(turns > 1);
        assert!(retirement.terminal_is_empty());
    }

    /// 🧯️ Cancellation/app-close pumps may present an empty grant; ownership remains intact and
    /// resumable until a later granted turn.
    #[test]
    fn empty_retirement_grant_preserves_resumable_ownership() {
        let factory = SemioSnapshotRetirementFactory::<text::SemioTextSnapshot>(PhantomData);
        let mut retirement = dsl::SnapshotRetirementFactory::retire(&factory, Arc::new(text::SemioTextSnapshot::default()));
        assert_eq!(retirement.close_step(0, 0).expect("empty grant"), dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        assert!(!retirement.terminal_is_empty());
    }

    /// 🔒️ A cloned public read lease keeps the exact owner blocked; the disposer never drops one
    /// Arc and lets another reader become an unbounded last owner behind its terminal witness.
    #[test]
    fn shared_snapshot_read_blocks_until_the_exact_owner_is_unique() {
        let factory = SemioSnapshotRetirementFactory::<text::SemioTextSnapshot>(PhantomData);
        let snapshot = Arc::new(text::SemioTextSnapshot::default());
        let reader = snapshot.clone();
        let mut retirement = dsl::SnapshotRetirementFactory::retire(&factory, snapshot);
        assert_eq!(retirement.close_step(8, 64).expect("shared owner check"), dsl::SnapshotRetirementStep::Blocked);
        assert!(!retirement.terminal_is_empty());
        drop(reader);
        while retirement.close_step(8, 64).expect("unique owner retirement") != dsl::SnapshotRetirementStep::Complete {}
        assert!(retirement.terminal_is_empty());
    }

    /// 🧸️ Every composable subset must be reachable through `create_semio_member` — an unlisted
    /// subset would fail with an unhelpful error rather than a named one.
    #[semio_framework_async_macros::async_test]
    async fn every_composable_subset_dispatches_to_a_real_child_store() {
        for subset in composable_subsets().await {
            let dialect = subset_dialect(subset).await;
            // An empty pack is rejected by the production member, so this asserts the DISPATCH
            // reached a real typed variant rather than falling through to "no member kind".
            let error = match create_semio_member("probe", &dialect, &[]).await {
                Ok(_) => panic!("empty genesis pack must be rejected"),
                Err(error) => error,
            };
            assert!(!error.to_string().contains("no member kind"), "subset {subset} is not wired into the child-store dispatch");
        }
        let unknown = match create_semio_member("probe", &subset_dialect("not-a-subset").await, &[]).await {
            Ok(_) => panic!("unknown subset must be rejected"),
            Err(error) => error,
        };
        assert!(unknown.to_string().contains("no member kind"));
    }

    /// 🧸️ `create_semio_member` must MINT a real child store and `open_semio_member` must REOPEN it
    /// from its own persisted envelope — the whole point of "children have their own version
    /// history". The reopen half only works because the persisted `.spr` now carries the dialect the
    /// subset is recovered from.
    #[semio_framework_async_macros::async_test]
    async fn a_semio_member_mints_and_reopens_a_real_child_envelope() {
        let dialect = subset_dialect("mesh").await;

        let seed = SemioMeshSnapshot::default();
        let child = create_semio_member("mesh-child-1", &dialect, &seed.encode_pack().await).await.expect("create child");
        assert_eq!(child.document_id().await, "mesh-child-1");

        let reopened = open_semio_member(&child.envelope_pack_bytes().await.expect("envelope pack")).await.expect("reopen child");
        assert_eq!(reopened.document_pack_bytes().await.expect("head pack"), child.document_pack_bytes().await.expect("head pack"), "the reopened child diverged from the persisted one");
    }
}
//#endregion 🧪️Tests
