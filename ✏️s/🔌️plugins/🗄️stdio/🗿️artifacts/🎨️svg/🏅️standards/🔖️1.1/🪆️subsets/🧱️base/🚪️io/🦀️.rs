//! 🚪️ IO stdio.svg (1.1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_1::subsets::base::io::SvgAnalyzer;
    use crate::SvgSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

    pub struct SvgComposerComposition;

    impl ArtifactComposition for SvgComposerComposition {
        type Snapshot = SvgSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_XML]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_XML)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "SvgComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SvgAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SvgComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): moved
// here verbatim (destination rule 3 — `io_registry`/`ComposerEntry` → `🚪️io/`). This is the REAL
// `&'static [ComposerEntry]` entries (NOT the artifact root's own shadowing `io_registry` module,
// which returns `&[&ComposerEntry]` — a different type; the root's `.composers(...)` reaches this
// module by its fully-qualified path, never the bare `io_registry::entries()` shortcut, per this
// ticket's own "silent rebind" hazard).
pub mod io_registry {
    use crate::standards::v1_1::subsets::base::io::SvgComposer as SvgRawAnyComposer;
    use crate::standards::v1_1::subsets::basic::io::SvgBasicComposer;
    use crate::standards::v1_1::subsets::tiny::io::SvgTinyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<SvgRawAnyComposer>(), composer_entry_of::<SvgTinyComposer>(), composer_entry_of::<SvgBasicComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::schema::snapshot::{set_element_attr, CommonAttrs, PathCommand, SvgElement, ViewBox};
    use crate::standards::v1_1::subsets::base::io::text::snapshot::{svg_element_to_xml_node,svg_document_to_typed,native_svg_document,bind_svg_node,bind_svg_attribute};
    use crate::{SvgDiff, SvgMutation, SvgSnapshot};
    use semio_framework_plugin::ArtifactBuilder;
    use crate::schema::snapshot::SvgNode;

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
    /// `SvgDocument` directly (unchanged), so both entry points compose.
    #[derive(Clone, Debug, Default)]
    pub struct SvgBuilderConstruction {
        snapshot: SvgSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
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
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SvgSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SvgSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let (next, diff) = store::apply_outcome(&self.snapshot, protocol::Mutation::diff(&mutation, &self.snapshot));
            self.snapshot = next;
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        /// 🏗️ Lowers any pending typed constructor calls into `snapshot.doc`'s root `<svg>` children
        /// before returning -- this is what lets `SvgBuilderConstruction::empty().set_view_box(...).add_rect(...)`
        /// produce a complete, valid SVG 1.1 document purely from typed calls.
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            let mut snapshot = self.snapshot;
            let mut diagnostics = self.diagnostics;
            let pending = self.elements.build();
            if !pending.is_empty() || self.view_box.is_some() || self.width.is_some() || self.height.is_some() || self.xmlns.is_some() {
                if snapshot.doc.root.is_none() {
                    snapshot.doc.root = Some(SvgNode::Element { name: "svg".into(), attrs: vec![], children: vec![] });
                }
                if let Some(root) = snapshot.doc.root.as_mut() {
                    if let Some(xmlns) = &self.xmlns {
                        set_element_attr(root, "xmlns", Some(crate::schema::snapshot::SvgAttributeValue::Text(xmlns.clone())));
                    }
                    if let Some(vb) = &self.view_box {
                        set_element_attr(root, "viewBox", Some(crate::schema::snapshot::SvgAttributeValue::ViewBox(*vb)));
                    }
                    if let Some(w) = &self.width {
                        set_element_attr(root, "width", Some(bind_svg_attribute("width",w).map_err(|detail|vec![semio_framework_diagnostic::Diagnostic{code:semio_framework_diagnostic::FaultCode::new("svg.builder.invalid-attribute"),severity:semio_framework_diagnostic::Severity::Error,span:semio_framework_diagnostic::TextSpan::at(1,1),message:detail,expected:None,scope:semio_framework_diagnostic::FaultScope::default()}])?));
                    }
                    if let Some(h) = &self.height {
                        set_element_attr(root, "height", Some(bind_svg_attribute("height",h).map_err(|detail|vec![semio_framework_diagnostic::Diagnostic{code:semio_framework_diagnostic::FaultCode::new("svg.builder.invalid-attribute"),severity:semio_framework_diagnostic::Severity::Error,span:semio_framework_diagnostic::TextSpan::at(1,1),message:detail,expected:None,scope:semio_framework_diagnostic::FaultScope::default()}])?));
                    }
                    if let SvgNode::Element { children, .. } = root {
                        children.extend(pending.iter().map(|element|bind_svg_node(svg_element_to_xml_node(element)).expect("typed SVG builder element")));
                    }
                }
            }
            if let Err(error) = snapshot.doc.validate_boundaries() {
                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.svg.boundary", semio_framework_diagnostic::TextSpan::at(1, 1), error));
            }
            if diagnostics.is_empty() {
                Ok(snapshot)
            } else {
                Err(diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::schema::snapshot::SvgElement;
    use crate::standards::v1_1::subsets::base::io::text::snapshot::{svg_document_to_typed,native_svg_document};
    use crate::SvgSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
    use crate::schema::snapshot::{SvgNode};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::{xml_document_from_text};

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
        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Text(text) => match xml_document_from_text(text) {
                    Ok(doc) => match &doc.root {
                        Some(semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { name, .. }) if name == "svg" || name.ends_with(":svg") => semio_framework_plugin::io::Confidence::High,
                        Some(_) => semio_framework_plugin::io::Confidence::Low,
                        None => semio_framework_plugin::io::Confidence::Low,
                    },
                    // 🚧️ Malformed XML: still `Low` rather than a hard rejection, since a truncated
                    // real `.svg` file is a plausible source this artifact still owns.
                    Err(_) => semio_framework_plugin::io::Confidence::Low,
                },
                AnalyzeSource::Binary(_) => semio_framework_plugin::io::Confidence::Low,
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SvgParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => {
                        match if store::semio_format::split_text_preamble(text).is_ok() { <SvgSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string()) } else { SvgSnapshot::import_utf8(text.as_bytes()) } {
                            Ok(snapshot) => {
                                match svg_document_to_typed(&native_svg_document(&snapshot.doc)) {
                                    Ok(typed) => parts.typed = Some(typed),
                                    Err(err) => {
                                        confidence = semio_framework_plugin::io::Confidence::Low;
                                        diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.typed", semio_framework_diagnostic::TextSpan::at(1, 1), err));
                                    }
                                }
                                parts.snapshot = Some(snapshot);
                            }
                            Err(err) => {
                                confidence = semio_framework_plugin::io::Confidence::Low;
                                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                            }
                        }
                    }
                    AnalyzeSource::Binary(bytes) => match <SvgSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => {
                            if let Ok(typed) = svg_document_to_typed(&native_svg_document(&snapshot.doc)) {
                                parts.typed = Some(typed);
                            }
                            parts.snapshot = Some(snapshot);
                        }
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
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

semio_framework_plugin::derive_artifact_facets!(
    pub spec SvgBuilderFacets {
        construction: SvgBuilderConstruction,
        analysis: SvgAnalyzerAnalysis,
        composition: crate::standards::v1_1::subsets::base::io::derived_composition::SvgComposerComposition,
    }
    builder: SvgBuilder,
    analyzer: SvgAnalyzer,
    composer: SvgComposer,
);
