//! 🧬️ SvgArtifact schema — full artifact state.

use crate::{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full `stdio.svg` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.svg")]
pub struct SvgArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub doc: semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for SvgArtifact {
    fn default() -> Self {
        Self::from_snapshot(SvgSnapshot::default())
    }
}

impl SvgArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SvgSnapshot {
        SvgSnapshot { schema: self.schema.clone(), doc: self.doc.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SvgSnapshot) -> Self {
        Self { schema: snapshot.schema, doc: snapshot.doc }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SvgSnapshot) {
        self.schema = snapshot.schema;
        self.doc = snapshot.doc;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.svg`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn svg_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.stdio.svg",
        artifact: framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
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
pub mod derived_construction {
    use crate::schema::snapshot::{set_element_attr, svg_element_to_xml_node, view_box_to_string, CommonAttrs, PathCommand, SvgElement, ViewBox};
    use crate::{SvgDiff, SvgMutation, SvgSnapshot};
    use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️PathBuilder
    /// 🖊️ Fluent constructor for a `d` attribute's typed command list -- mirrors the path mini-language
    /// 1:1 (`move_to`/`line_to`/... absolute, `move_by`/`line_by`/... relative) so a hand-written chain
    /// reads like the path grammar itself.
    #[derive(Clone, Debug, Default)]
    pub struct PathBuilder {
        cmds: Vec<PathCommand>,
    }

    impl PathBuilder {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            Self::default()
        }
        /// 🧩 Seeds the builder from an already-typed command list (used to reconstruct a path
        /// programmatically, e.g. from an analyzer's output, without re-parsing/re-stringifying it).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn from_commands(cmds: Vec<PathCommand>) -> Self {
            Self { cmds }
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn move_to(mut self, x: f64, y: f64) -> Self {
            self.cmds.push(PathCommand::MoveTo { x, y, relative: false });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn move_by(mut self, dx: f64, dy: f64) -> Self {
            self.cmds.push(PathCommand::MoveTo { x: dx, y: dy, relative: true });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn line_to(mut self, x: f64, y: f64) -> Self {
            self.cmds.push(PathCommand::LineTo { x, y, relative: false });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn line_by(mut self, dx: f64, dy: f64) -> Self {
            self.cmds.push(PathCommand::LineTo { x: dx, y: dy, relative: true });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn horizontal_to(mut self, x: f64) -> Self {
            self.cmds.push(PathCommand::HorizontalLineTo { x, relative: false });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn horizontal_by(mut self, dx: f64) -> Self {
            self.cmds.push(PathCommand::HorizontalLineTo { x: dx, relative: true });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn vertical_to(mut self, y: f64) -> Self {
            self.cmds.push(PathCommand::VerticalLineTo { y, relative: false });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn vertical_by(mut self, dy: f64) -> Self {
            self.cmds.push(PathCommand::VerticalLineTo { y: dy, relative: true });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn cubic_to(mut self, x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64) -> Self {
            self.cmds.push(PathCommand::CurveTo { x1, y1, x2, y2, x, y, relative: false });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn cubic_by(mut self, x1: f64, y1: f64, x2: f64, y2: f64, dx: f64, dy: f64) -> Self {
            self.cmds.push(PathCommand::CurveTo { x1, y1, x2, y2, x: dx, y: dy, relative: true });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn smooth_cubic_to(mut self, x2: f64, y2: f64, x: f64, y: f64) -> Self {
            self.cmds.push(PathCommand::SmoothCurveTo { x2, y2, x, y, relative: false });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn smooth_cubic_by(mut self, x2: f64, y2: f64, dx: f64, dy: f64) -> Self {
            self.cmds.push(PathCommand::SmoothCurveTo { x2, y2, x: dx, y: dy, relative: true });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn quadratic_to(mut self, x1: f64, y1: f64, x: f64, y: f64) -> Self {
            self.cmds.push(PathCommand::QuadraticCurveTo { x1, y1, x, y, relative: false });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn quadratic_by(mut self, x1: f64, y1: f64, dx: f64, dy: f64) -> Self {
            self.cmds.push(PathCommand::QuadraticCurveTo { x1, y1, x: dx, y: dy, relative: true });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn smooth_quadratic_to(mut self, x: f64, y: f64) -> Self {
            self.cmds.push(PathCommand::SmoothQuadraticCurveTo { x, y, relative: false });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn smooth_quadratic_by(mut self, dx: f64, dy: f64) -> Self {
            self.cmds.push(PathCommand::SmoothQuadraticCurveTo { x: dx, y: dy, relative: true });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn arc_to(mut self, (rx, ry): (f64, f64), x_axis_rotation: f64, large_arc: bool, sweep: bool, x: f64, y: f64) -> Self {
            self.cmds.push(PathCommand::Arc { rx, ry, x_axis_rotation, large_arc, sweep, x, y, relative: false });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn arc_by(mut self, (rx, ry): (f64, f64), x_axis_rotation: f64, large_arc: bool, sweep: bool, dx: f64, dy: f64) -> Self {
            self.cmds.push(PathCommand::Arc { rx, ry, x_axis_rotation, large_arc, sweep, x: dx, y: dy, relative: true });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn close(mut self) -> Self {
            self.cmds.push(PathCommand::ClosePath);
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn build(self) -> Vec<PathCommand> {
            self.cmds
        }
    }
    //#endregion 🔖️PathBuilder

    //#region 🔖️GradientStopSpec
    /// 🎨 One `<stop>` for `define_linear_gradient`/`define_radial_gradient`.
    #[derive(Clone, Debug)]
    pub struct GradientStopSpec {
        pub offset: String,
        pub color: Option<String>,
        pub opacity: Option<String>,
    }

    impl GradientStopSpec {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new(offset: impl Into<String>) -> Self {
            Self { offset: offset.into(), color: None, opacity: None }
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_color(mut self, color: impl Into<String>) -> Self {
            self.color = Some(color.into());
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_opacity(mut self, opacity: impl Into<String>) -> Self {
            self.opacity = Some(opacity.into());
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn into_element(self) -> SvgElement {
            SvgElement::Stop { common: CommonAttrs::default(), offset: self.offset, stop_color: self.color, stop_opacity: self.opacity }
        }
    }
    //#endregion 🔖️GradientStopSpec

    //#region 🔖️ElementBuilder
    /// 🧩 Fluent, typed constructor for a list of sibling `SvgElement`s -- shared by `SvgBuilderConstruction`'s
    /// root-level children AND by `add_group`/`add_defs`'s nested scopes, so groups compose exactly
    /// like the top level does.
    #[derive(Clone, Debug, Default)]
    pub struct ElementBuilder {
        children: Vec<SvgElement>,
    }

    impl ElementBuilder {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            Self::default()
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_rect(mut self, x: f64, y: f64, width: f64, height: f64, common: CommonAttrs) -> Self {
            self.children.push(SvgElement::Rect { common, x, y, width, height, rx: None, ry: None });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_rect_rounded(mut self, x: f64, y: f64, width: f64, height: f64, (rx, ry): (f64, f64), common: CommonAttrs) -> Self {
            self.children.push(SvgElement::Rect { common, x, y, width, height, rx: Some(rx), ry: Some(ry) });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_circle(mut self, cx: f64, cy: f64, r: f64, common: CommonAttrs) -> Self {
            self.children.push(SvgElement::Circle { common, cx, cy, r });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_ellipse(mut self, cx: f64, cy: f64, rx: f64, ry: f64, common: CommonAttrs) -> Self {
            self.children.push(SvgElement::Ellipse { common, cx, cy, rx, ry });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_line(mut self, x1: f64, y1: f64, x2: f64, y2: f64, common: CommonAttrs) -> Self {
            self.children.push(SvgElement::Line { common, x1, y1, x2, y2 });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_polyline(mut self, points: Vec<(f64, f64)>, common: CommonAttrs) -> Self {
            self.children.push(SvgElement::Polyline { common, points });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_polygon(mut self, points: Vec<(f64, f64)>, common: CommonAttrs) -> Self {
            self.children.push(SvgElement::Polygon { common, points });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_path(mut self, path: PathBuilder, common: CommonAttrs) -> Self {
            self.children.push(SvgElement::Path { common, d: path.build() });
            self
        }
        /// 🧬 Nests a `<g>` group: `build` receives a fresh `ElementBuilder` scoped to the group.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_group(mut self, common: CommonAttrs, build: impl FnOnce(ElementBuilder) -> ElementBuilder) -> Self {
            let inner = build(ElementBuilder::new());
            self.children.push(SvgElement::Group { common, children: inner.children });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_defs(mut self, common: CommonAttrs, build: impl FnOnce(ElementBuilder) -> ElementBuilder) -> Self {
            let inner = build(ElementBuilder::new());
            self.children.push(SvgElement::Defs { common, children: inner.children });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_text(mut self, x: Option<f64>, y: Option<f64>, text: impl Into<String>, common: CommonAttrs) -> Self {
            self.children.push(SvgElement::Text { common, x, y, children: vec![SvgElement::TextNode(text.into())] });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_use(mut self, href: impl Into<String>, x: Option<f64>, y: Option<f64>, width: Option<f64>, height: Option<f64>, common: CommonAttrs) -> Self {
            self.children.push(SvgElement::Use { common, href: href.into(), x, y, width, height });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn define_linear_gradient(mut self, id: impl Into<String>, x1: Option<f64>, y1: Option<f64>, x2: Option<f64>, y2: Option<f64>, stops: Vec<GradientStopSpec>) -> Self {
            self.children.push(SvgElement::LinearGradient {
                common: CommonAttrs::default(),
                id: Some(id.into()),
                x1: x1.map(|v| v.to_string()),
                y1: y1.map(|v| v.to_string()),
                x2: x2.map(|v| v.to_string()),
                y2: y2.map(|v| v.to_string()),
                children: stops.into_iter().map(GradientStopSpec::into_element).collect(),
            });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn define_radial_gradient(mut self, id: impl Into<String>, cx: Option<f64>, cy: Option<f64>, r: Option<f64>, (fx, fy): (Option<f64>, Option<f64>), stops: Vec<GradientStopSpec>) -> Self {
            self.children.push(SvgElement::RadialGradient {
                common: CommonAttrs::default(),
                id: Some(id.into()),
                cx: cx.map(|v| v.to_string()),
                cy: cy.map(|v| v.to_string()),
                r: r.map(|v| v.to_string()),
                fx: fx.map(|v| v.to_string()),
                fy: fy.map(|v| v.to_string()),
                children: stops.into_iter().map(GradientStopSpec::into_element).collect(),
            });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn build(self) -> Vec<SvgElement> {
            self.children
        }
    }
    //#endregion 🔖️ElementBuilder

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.svg` snapshot. `set_view_box`/`add_*`/`define_*` accumulate typed elements
    /// (via an internal `ElementBuilder`) that are lowered into `snapshot.doc` only at `build()` time;
    /// `from_snapshot`/`from_text`/`from_binary`/`mutate` continue to operate on the persisted
    /// `XmlDocument` directly (unchanged), so both entry points compose.
    #[derive(Clone, Debug, Default)]
    pub struct SvgBuilderConstruction {
        snapshot: SvgSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
        elements: ElementBuilder,
        view_box: Option<ViewBox>,
        width: Option<String>,
        height: Option<String>,
        xmlns: Option<String>,
    }

    impl SvgBuilderConstruction {
        //#region TypedConstructors
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_view_box(mut self, min_x: f64, min_y: f64, width: f64, height: f64) -> Self {
            self.view_box = Some(ViewBox { min_x, min_y, width, height });
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_dimensions(mut self, width: impl Into<String>, height: impl Into<String>) -> Self {
            self.width = Some(width.into());
            self.height = Some(height.into());
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_xmlns(mut self, xmlns: impl Into<String>) -> Self {
            self.xmlns = Some(xmlns.into());
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_rect(mut self, x: f64, y: f64, width: f64, height: f64, common: CommonAttrs) -> Self {
            self.elements = self.elements.add_rect(x, y, width, height, common);
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_rect_rounded(mut self, x: f64, y: f64, width: f64, height: f64, (rx, ry): (f64, f64), common: CommonAttrs) -> Self {
            self.elements = self.elements.add_rect_rounded(x, y, width, height, (rx, ry), common);
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_circle(mut self, cx: f64, cy: f64, r: f64, common: CommonAttrs) -> Self {
            self.elements = self.elements.add_circle(cx, cy, r, common);
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_ellipse(mut self, cx: f64, cy: f64, rx: f64, ry: f64, common: CommonAttrs) -> Self {
            self.elements = self.elements.add_ellipse(cx, cy, rx, ry, common);
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_line(mut self, x1: f64, y1: f64, x2: f64, y2: f64, common: CommonAttrs) -> Self {
            self.elements = self.elements.add_line(x1, y1, x2, y2, common);
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_polyline(mut self, points: Vec<(f64, f64)>, common: CommonAttrs) -> Self {
            self.elements = self.elements.add_polyline(points, common);
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_polygon(mut self, points: Vec<(f64, f64)>, common: CommonAttrs) -> Self {
            self.elements = self.elements.add_polygon(points, common);
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_path(mut self, path: PathBuilder, common: CommonAttrs) -> Self {
            self.elements = self.elements.add_path(path, common);
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_group(mut self, common: CommonAttrs, build: impl FnOnce(ElementBuilder) -> ElementBuilder) -> Self {
            self.elements = self.elements.add_group(common, build);
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_defs(mut self, common: CommonAttrs, build: impl FnOnce(ElementBuilder) -> ElementBuilder) -> Self {
            self.elements = self.elements.add_defs(common, build);
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_text(mut self, x: Option<f64>, y: Option<f64>, text: impl Into<String>, common: CommonAttrs) -> Self {
            self.elements = self.elements.add_text(x, y, text, common);
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_use(mut self, href: impl Into<String>, x: Option<f64>, y: Option<f64>, width: Option<f64>, height: Option<f64>, common: CommonAttrs) -> Self {
            self.elements = self.elements.add_use(href, x, y, width, height, common);
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn define_linear_gradient(mut self, id: impl Into<String>, x1: Option<f64>, y1: Option<f64>, x2: Option<f64>, y2: Option<f64>, stops: Vec<GradientStopSpec>) -> Self {
            self.elements = self.elements.define_linear_gradient(id, x1, y1, x2, y2, stops);
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn define_radial_gradient(mut self, id: impl Into<String>, cx: Option<f64>, cy: Option<f64>, r: Option<f64>, (fx, fy): (Option<f64>, Option<f64>), stops: Vec<GradientStopSpec>) -> Self {
            self.elements = self.elements.define_radial_gradient(id, cx, cy, r, (fx, fy), stops);
            self
        }
        //#endregion TypedConstructors
    }

    impl ArtifactBuilder for SvgBuilderConstruction {
        type Snapshot = SvgSnapshot;
        type Mutation = SvgMutation;
        type Diff = SvgDiff;
        fn empty() -> Self {
            Self { snapshot: SvgSnapshot::default(), diagnostics: Vec::new(), elements: ElementBuilder::new(), view_box: None, width: None, height: None, xmlns: None }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new(), elements: ElementBuilder::new(), view_box: None, width: None, height: None, xmlns: None }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SvgSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SvgSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_svg_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SvgDiff as protocol::MutationDiff<SvgSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        /// 🏗️ Lowers any pending typed constructor calls into `snapshot.doc`'s root `<svg>` children
        /// before returning -- this is what lets `SvgBuilderConstruction::empty().set_view_box(...).add_rect(...)`
        /// produce a complete, valid SVG 1.1 document purely from typed calls.
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            let mut snapshot = self.snapshot;
            let pending = self.elements.build();
            if !pending.is_empty() || self.view_box.is_some() || self.width.is_some() || self.height.is_some() || self.xmlns.is_some() {
                if snapshot.doc.root.is_none() {
                    snapshot.doc.root = Some(XmlNode::Element { name: "svg".into(), attrs: vec![], children: vec![] });
                }
                if let Some(root) = snapshot.doc.root.as_mut() {
                    if let Some(xmlns) = &self.xmlns {
                        set_element_attr(root, "xmlns", Some(xmlns.clone()));
                    }
                    if let Some(vb) = &self.view_box {
                        set_element_attr(root, "viewBox", Some(view_box_to_string(vb)));
                    }
                    if let Some(w) = &self.width {
                        set_element_attr(root, "width", Some(w.clone()));
                    }
                    if let Some(h) = &self.height {
                        set_element_attr(root, "height", Some(h.clone()));
                    }
                    if let XmlNode::Element { children, .. } = root {
                        children.extend(pending.iter().map(svg_element_to_xml_node));
                    }
                }
            }
            if self.diagnostics.is_empty() {
                Ok(snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::schema::snapshot::{svg_document_to_typed, SvgElement};
    use crate::SvgSnapshot;
    use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_from_text, XmlNode};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.svg` parts. `typed` is the real 1.1 semantic model (`SvgElement` tree),
    /// derived from `snapshot.doc` once parsing succeeds -- callers that only need the generic/lossless
    /// XML view can still use `snapshot`; callers that want typed elements (shapes, paths, gradients,
    /// ...) use `typed`.
    #[derive(Clone, Debug, Default)]
    pub struct SvgParts {
        pub snapshot: Option<SvgSnapshot>,
        pub typed: Option<SvgElement>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.svg` (1.1/✳️any) sources.
    pub struct SvgAnalyzerAnalysis;

    impl ArtifactAnalysis for SvgAnalyzerAnalysis {
        type Parts = SvgParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };

        /// 🕵️ Real sniff: parses the (possibly DOCTYPE/prolog-prefixed) XML and checks the root
        /// element's LOCAL name is `svg` (namespace-prefixed roots like `ns:svg` count too) -- not a
        /// constant. Binary sources aren't XML text, so they're never claimed here.
        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Text(text) => match xml_document_from_text(text) {
                    Ok(doc) => match &doc.root {
                        Some(XmlNode::Element { name, .. }) if name == "svg" || name.ends_with(":svg") => IoConfidence::High,
                        Some(_) => IoConfidence::Low,
                        None => IoConfidence::Low,
                    },
                    // 🚧️ Malformed XML: still `Low` rather than a hard rejection, since a truncated
                    // real `.svg` file is a plausible source this artifact still owns.
                    Err(_) => IoConfidence::Low,
                },
                AnalyzeSource::Binary(_) => IoConfidence::Low,
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SvgParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => {
                        match if store::semio_format::split_text_preamble(text).is_ok() { <SvgSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string()) } else { SvgSnapshot::import_utf8(text.as_bytes()) } {
                            Ok(snapshot) => {
                                match svg_document_to_typed(&snapshot.doc) {
                                    Ok(typed) => parts.typed = Some(typed),
                                    Err(err) => {
                                        confidence = IoConfidence::Low;
                                        diagnostics.push(dsl::Diagnostic::error("stdio.analyze.typed", dsl::TextSpan::at(1, 1), err));
                                    }
                                }
                                parts.snapshot = Some(snapshot);
                            }
                            Err(err) => {
                                confidence = IoConfidence::Low;
                                diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                            }
                        }
                    }
                    AnalyzeSource::Binary(bytes) => match <SvgSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => {
                            if let Ok(typed) = svg_document_to_typed(&snapshot.doc) {
                                parts.typed = Some(typed);
                            }
                            parts.snapshot = Some(snapshot);
                        }
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    //#region 🧪️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec SvgBuilderFacets {
        construction: SvgBuilderConstruction,
        analysis: SvgAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SvgComposerComposition,
    }
    builder: SvgBuilder,
    analyzer: SvgAnalyzer,
    composer: SvgComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
// `empty_svg_snapshot`/`demo_svg_snapshot` relocated here verbatim (pure helpers over the
// document type, destination rule 5); `SvgEngine` (zero construction sites) deleted outright;
// codecs/`io_registry` moved to `../🚪️io`; tests moved beside what they now test (see that
// file's own `mod tests`).
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_svg_snapshot() -> SvgSnapshot {
    SvgSnapshot::default()
}

/// 📄️ The demo `stdio.svg` document -- exercises every real-syntax construct the W0 census row
/// names (svg's snapshot IS an `XmlDocument`, so this mirrors `📰️xml`'s own `demo_xml_snapshot`
/// construct-for-construct): an XML declaration, a simple `<!DOCTYPE svg>`, a namespaced
/// (`:`-qualified) attribute name (`xmlns:xlink`), entity decode (`Tom &amp; Jerry`), a
/// self-closing element (carrying an attribute so its trailing `/` never fuses with the preceding
/// ident), `<![CDATA[...]]>`, `<!--...-->`, and a `<?target data?>` processing instruction. The
/// single source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/
/// `🎒️.pack.semio` (both are literally this snapshot's `print_dsl`/`encode_pack` output,
/// asserted equal by `fixture_honesty_law` in `../🚪️io`'s own tests).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_svg_snapshot() -> SvgSnapshot {
    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDocument, XmlNode};
    let root = XmlNode::Element {
        name: "svg".into(),
        attrs: vec![XmlAttr { name: "xmlns".into(), value: "http://www.w3.org/2000/svg".into() }, XmlAttr { name: "xmlns:xlink".into(), value: "http://www.w3.org/1999/xlink".into() }, XmlAttr { name: "viewBox".into(), value: "0 0 100 100".into() }],
        children: vec![
            XmlNode::Comment { text: " demo scene ".into() },
            XmlNode::ProcessingInstruction { target: "xml-stylesheet".into(), data: "text".into() },
            XmlNode::Element {
                name: "rect".into(),
                attrs: vec![
                    XmlAttr { name: "x".into(), value: "0".into() },
                    XmlAttr { name: "y".into(), value: "0".into() },
                    XmlAttr { name: "width".into(), value: "10".into() },
                    XmlAttr { name: "height".into(), value: "10".into() },
                    XmlAttr { name: "fill".into(), value: "red".into() },
                ],
                children: vec![],
            },
            XmlNode::Element { name: "text".into(), attrs: vec![XmlAttr { name: "x".into(), value: "5".into() }], children: vec![XmlNode::Text { text: "Tom & Jerry".into() }] },
            XmlNode::Element { name: "circle".into(), attrs: vec![XmlAttr { name: "cx".into(), value: "1".into() }], children: vec![] },
            XmlNode::CData { text: "raw markup".into() },
        ],
    };
    let snapshot = SvgSnapshot {
        schema: STDIO_SVG_DOCUMENT_SCHEMA.into(),
        doc: XmlDocument { declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true) }), doctype: Some("<!DOCTYPE svg>".into()), prolog: Vec::new(), root: Some(root) },
    };
    let _text = crate::schema::snapshot::write_svg_xml(&snapshot.doc);
    snapshot
}
//#endregion 🔖️DocumentHelpers
