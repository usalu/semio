//! 🏗️ SvgBuilder — local ArtifactBuilder until SDK Wave 3. Also THE typed-constructor surface for
//! building a complete SVG 1.1 document from scratch (plan D2's svg reference requirement):
//! `SvgBuilder::empty().set_view_box(...).add_rect(...).add_group(...).build()`.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::svg::schema::snapshot::{
    set_element_attr, svg_element_to_xml_node, view_box_to_string, CommonAttrs, PathCommand, SvgElement, ViewBox,
};
use crate::artifacts::svg::{SvgDiff, SvgMutation, SvgSnapshot};
use crate::artifacts::xml::schema::snapshot::XmlNode;

//#region 🔖️PathBuilder
/// 🖊️ Fluent constructor for a `d` attribute's typed command list -- mirrors the path mini-language
/// 1:1 (`move_to`/`line_to`/... absolute, `move_by`/`line_by`/... relative) so a hand-written chain
/// reads like the path grammar itself.
#[derive(Clone, Debug, Default)]
pub struct PathBuilder {
    cmds: Vec<PathCommand>,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// 🧩 Seeds the builder from an already-typed command list (used to reconstruct a path
    /// programmatically, e.g. from an analyzer's output, without re-parsing/re-stringifying it).
    pub fn from_commands(cmds: Vec<PathCommand>) -> Self {
        Self { cmds }
    }
    pub fn move_to(mut self, x: f64, y: f64) -> Self { self.cmds.push(PathCommand::MoveTo { x, y, relative: false }); self }
    pub fn move_by(mut self, dx: f64, dy: f64) -> Self { self.cmds.push(PathCommand::MoveTo { x: dx, y: dy, relative: true }); self }
    pub fn line_to(mut self, x: f64, y: f64) -> Self { self.cmds.push(PathCommand::LineTo { x, y, relative: false }); self }
    pub fn line_by(mut self, dx: f64, dy: f64) -> Self { self.cmds.push(PathCommand::LineTo { x: dx, y: dy, relative: true }); self }
    pub fn horizontal_to(mut self, x: f64) -> Self { self.cmds.push(PathCommand::HorizontalLineTo { x, relative: false }); self }
    pub fn horizontal_by(mut self, dx: f64) -> Self { self.cmds.push(PathCommand::HorizontalLineTo { x: dx, relative: true }); self }
    pub fn vertical_to(mut self, y: f64) -> Self { self.cmds.push(PathCommand::VerticalLineTo { y, relative: false }); self }
    pub fn vertical_by(mut self, dy: f64) -> Self { self.cmds.push(PathCommand::VerticalLineTo { y: dy, relative: true }); self }
    pub fn cubic_to(mut self, x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64) -> Self {
        self.cmds.push(PathCommand::CurveTo { x1, y1, x2, y2, x, y, relative: false }); self
    }
    pub fn cubic_by(mut self, x1: f64, y1: f64, x2: f64, y2: f64, dx: f64, dy: f64) -> Self {
        self.cmds.push(PathCommand::CurveTo { x1, y1, x2, y2, x: dx, y: dy, relative: true }); self
    }
    pub fn smooth_cubic_to(mut self, x2: f64, y2: f64, x: f64, y: f64) -> Self {
        self.cmds.push(PathCommand::SmoothCurveTo { x2, y2, x, y, relative: false }); self
    }
    pub fn smooth_cubic_by(mut self, x2: f64, y2: f64, dx: f64, dy: f64) -> Self {
        self.cmds.push(PathCommand::SmoothCurveTo { x2, y2, x: dx, y: dy, relative: true }); self
    }
    pub fn quadratic_to(mut self, x1: f64, y1: f64, x: f64, y: f64) -> Self {
        self.cmds.push(PathCommand::QuadraticCurveTo { x1, y1, x, y, relative: false }); self
    }
    pub fn quadratic_by(mut self, x1: f64, y1: f64, dx: f64, dy: f64) -> Self {
        self.cmds.push(PathCommand::QuadraticCurveTo { x1, y1, x: dx, y: dy, relative: true }); self
    }
    pub fn smooth_quadratic_to(mut self, x: f64, y: f64) -> Self { self.cmds.push(PathCommand::SmoothQuadraticCurveTo { x, y, relative: false }); self }
    pub fn smooth_quadratic_by(mut self, dx: f64, dy: f64) -> Self { self.cmds.push(PathCommand::SmoothQuadraticCurveTo { x: dx, y: dy, relative: true }); self }
    pub fn arc_to(mut self, rx: f64, ry: f64, x_axis_rotation: f64, large_arc: bool, sweep: bool, x: f64, y: f64) -> Self {
        self.cmds.push(PathCommand::Arc { rx, ry, x_axis_rotation, large_arc, sweep, x, y, relative: false }); self
    }
    pub fn arc_by(mut self, rx: f64, ry: f64, x_axis_rotation: f64, large_arc: bool, sweep: bool, dx: f64, dy: f64) -> Self {
        self.cmds.push(PathCommand::Arc { rx, ry, x_axis_rotation, large_arc, sweep, x: dx, y: dy, relative: true }); self
    }
    pub fn close(mut self) -> Self { self.cmds.push(PathCommand::ClosePath); self }
    pub fn build(self) -> Vec<PathCommand> { self.cmds }
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
    pub fn new(offset: impl Into<String>) -> Self {
        Self { offset: offset.into(), color: None, opacity: None }
    }
    pub fn with_color(mut self, color: impl Into<String>) -> Self { self.color = Some(color.into()); self }
    pub fn with_opacity(mut self, opacity: impl Into<String>) -> Self { self.opacity = Some(opacity.into()); self }
    fn into_element(self) -> SvgElement {
        SvgElement::Stop { common: CommonAttrs::default(), offset: self.offset, stop_color: self.color, stop_opacity: self.opacity }
    }
}
//#endregion 🔖️GradientStopSpec

//#region 🔖️ElementBuilder
/// 🧩 Fluent, typed constructor for a list of sibling `SvgElement`s -- shared by `SvgBuilder`'s
/// root-level children AND by `add_group`/`add_defs`'s nested scopes, so groups compose exactly
/// like the top level does.
#[derive(Clone, Debug, Default)]
pub struct ElementBuilder {
    children: Vec<SvgElement>,
}

impl ElementBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rect(mut self, x: f64, y: f64, width: f64, height: f64, common: CommonAttrs) -> Self {
        self.children.push(SvgElement::Rect { common, x, y, width, height, rx: None, ry: None });
        self
    }
    pub fn add_rect_rounded(mut self, x: f64, y: f64, width: f64, height: f64, rx: f64, ry: f64, common: CommonAttrs) -> Self {
        self.children.push(SvgElement::Rect { common, x, y, width, height, rx: Some(rx), ry: Some(ry) });
        self
    }
    pub fn add_circle(mut self, cx: f64, cy: f64, r: f64, common: CommonAttrs) -> Self {
        self.children.push(SvgElement::Circle { common, cx, cy, r });
        self
    }
    pub fn add_ellipse(mut self, cx: f64, cy: f64, rx: f64, ry: f64, common: CommonAttrs) -> Self {
        self.children.push(SvgElement::Ellipse { common, cx, cy, rx, ry });
        self
    }
    pub fn add_line(mut self, x1: f64, y1: f64, x2: f64, y2: f64, common: CommonAttrs) -> Self {
        self.children.push(SvgElement::Line { common, x1, y1, x2, y2 });
        self
    }
    pub fn add_polyline(mut self, points: Vec<(f64, f64)>, common: CommonAttrs) -> Self {
        self.children.push(SvgElement::Polyline { common, points });
        self
    }
    pub fn add_polygon(mut self, points: Vec<(f64, f64)>, common: CommonAttrs) -> Self {
        self.children.push(SvgElement::Polygon { common, points });
        self
    }
    pub fn add_path(mut self, path: PathBuilder, common: CommonAttrs) -> Self {
        self.children.push(SvgElement::Path { common, d: path.build() });
        self
    }
    /// 🧬 Nests a `<g>` group: `build` receives a fresh `ElementBuilder` scoped to the group.
    pub fn add_group(mut self, common: CommonAttrs, build: impl FnOnce(ElementBuilder) -> ElementBuilder) -> Self {
        let inner = build(ElementBuilder::new());
        self.children.push(SvgElement::Group { common, children: inner.children });
        self
    }
    pub fn add_defs(mut self, common: CommonAttrs, build: impl FnOnce(ElementBuilder) -> ElementBuilder) -> Self {
        let inner = build(ElementBuilder::new());
        self.children.push(SvgElement::Defs { common, children: inner.children });
        self
    }
    pub fn add_text(mut self, x: Option<f64>, y: Option<f64>, text: impl Into<String>, common: CommonAttrs) -> Self {
        self.children.push(SvgElement::Text { common, x, y, children: vec![SvgElement::TextNode(text.into())] });
        self
    }
    pub fn add_use(mut self, href: impl Into<String>, x: Option<f64>, y: Option<f64>, width: Option<f64>, height: Option<f64>, common: CommonAttrs) -> Self {
        self.children.push(SvgElement::Use { common, href: href.into(), x, y, width, height });
        self
    }
    pub fn define_linear_gradient(
        mut self,
        id: impl Into<String>,
        x1: Option<f64>,
        y1: Option<f64>,
        x2: Option<f64>,
        y2: Option<f64>,
        stops: Vec<GradientStopSpec>,
    ) -> Self {
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
    pub fn define_radial_gradient(
        mut self,
        id: impl Into<String>,
        cx: Option<f64>,
        cy: Option<f64>,
        r: Option<f64>,
        fx: Option<f64>,
        fy: Option<f64>,
        stops: Vec<GradientStopSpec>,
    ) -> Self {
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
pub struct SvgBuilder {
    snapshot: SvgSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
    elements: ElementBuilder,
    view_box: Option<ViewBox>,
    width: Option<String>,
    height: Option<String>,
    xmlns: Option<String>,
}

impl SvgBuilder {
    //#region TypedConstructors
    pub fn set_view_box(mut self, min_x: f64, min_y: f64, width: f64, height: f64) -> Self {
        self.view_box = Some(ViewBox { min_x, min_y, width, height });
        self
    }
    pub fn set_dimensions(mut self, width: impl Into<String>, height: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self.height = Some(height.into());
        self
    }
    pub fn set_xmlns(mut self, xmlns: impl Into<String>) -> Self {
        self.xmlns = Some(xmlns.into());
        self
    }
    pub fn add_rect(mut self, x: f64, y: f64, width: f64, height: f64, common: CommonAttrs) -> Self {
        self.elements = self.elements.add_rect(x, y, width, height, common);
        self
    }
    pub fn add_rect_rounded(mut self, x: f64, y: f64, width: f64, height: f64, rx: f64, ry: f64, common: CommonAttrs) -> Self {
        self.elements = self.elements.add_rect_rounded(x, y, width, height, rx, ry, common);
        self
    }
    pub fn add_circle(mut self, cx: f64, cy: f64, r: f64, common: CommonAttrs) -> Self {
        self.elements = self.elements.add_circle(cx, cy, r, common);
        self
    }
    pub fn add_ellipse(mut self, cx: f64, cy: f64, rx: f64, ry: f64, common: CommonAttrs) -> Self {
        self.elements = self.elements.add_ellipse(cx, cy, rx, ry, common);
        self
    }
    pub fn add_line(mut self, x1: f64, y1: f64, x2: f64, y2: f64, common: CommonAttrs) -> Self {
        self.elements = self.elements.add_line(x1, y1, x2, y2, common);
        self
    }
    pub fn add_polyline(mut self, points: Vec<(f64, f64)>, common: CommonAttrs) -> Self {
        self.elements = self.elements.add_polyline(points, common);
        self
    }
    pub fn add_polygon(mut self, points: Vec<(f64, f64)>, common: CommonAttrs) -> Self {
        self.elements = self.elements.add_polygon(points, common);
        self
    }
    pub fn add_path(mut self, path: PathBuilder, common: CommonAttrs) -> Self {
        self.elements = self.elements.add_path(path, common);
        self
    }
    pub fn add_group(mut self, common: CommonAttrs, build: impl FnOnce(ElementBuilder) -> ElementBuilder) -> Self {
        self.elements = self.elements.add_group(common, build);
        self
    }
    pub fn add_defs(mut self, common: CommonAttrs, build: impl FnOnce(ElementBuilder) -> ElementBuilder) -> Self {
        self.elements = self.elements.add_defs(common, build);
        self
    }
    pub fn add_text(mut self, x: Option<f64>, y: Option<f64>, text: impl Into<String>, common: CommonAttrs) -> Self {
        self.elements = self.elements.add_text(x, y, text, common);
        self
    }
    pub fn add_use(mut self, href: impl Into<String>, x: Option<f64>, y: Option<f64>, width: Option<f64>, height: Option<f64>, common: CommonAttrs) -> Self {
        self.elements = self.elements.add_use(href, x, y, width, height, common);
        self
    }
    pub fn define_linear_gradient(mut self, id: impl Into<String>, x1: Option<f64>, y1: Option<f64>, x2: Option<f64>, y2: Option<f64>, stops: Vec<GradientStopSpec>) -> Self {
        self.elements = self.elements.define_linear_gradient(id, x1, y1, x2, y2, stops);
        self
    }
    pub fn define_radial_gradient(mut self, id: impl Into<String>, cx: Option<f64>, cy: Option<f64>, r: Option<f64>, fx: Option<f64>, fy: Option<f64>, stops: Vec<GradientStopSpec>) -> Self {
        self.elements = self.elements.define_radial_gradient(id, cx, cy, r, fx, fy, stops);
        self
    }
    //#endregion TypedConstructors
}

impl ArtifactBuilder for SvgBuilder {
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
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = crate::artifacts::svg::schema::mutations::apply_svg_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SvgDiff as protocol::MutationDiff<SvgSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    /// 🏗️ Lowers any pending typed constructor calls into `snapshot.doc`'s root `<svg>` children
    /// before returning -- this is what lets `SvgBuilder::empty().set_view_box(...).add_rect(...)`
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
        if self.diagnostics.is_empty() { Ok(snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder
