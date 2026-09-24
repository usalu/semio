//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    #[cfg(feature = "conversion-drawing")]
    use crate::standards::v1::subsets::drawing::io::export::serializers::artifacts::dxf::v_r12::any::SemioDrawingToDxf;
    #[cfg(feature = "conversion-drawing")]
    use crate::standards::v1::subsets::drawing::io::export::serializers::artifacts::pdf::v1_7::any::SemioDrawingToPdf;
    #[cfg(feature = "conversion-drawing")]
    use crate::standards::v1::subsets::drawing::io::export::serializers::artifacts::png::v1_2::any::SemioDrawingToPng;
    #[cfg(feature = "conversion-drawing")]
    use crate::standards::v1::subsets::drawing::io::export::serializers::artifacts::svg::v1_1::any::SemioDrawingToSvg;
    #[cfg(feature = "conversion-drawing")]
    use crate::standards::v1::subsets::drawing::io::import::deserializers::artifacts::dxf::v_r12::any::SemioDrawingFromDxf;
    #[cfg(feature = "conversion-drawing")]
    use crate::standards::v1::subsets::drawing::io::import::deserializers::artifacts::pdf::v1_7::any::SemioDrawingFromPdf;
    #[cfg(feature = "conversion-drawing")]
    use crate::standards::v1::subsets::drawing::io::import::deserializers::artifacts::svg::v1_1::any::SemioDrawingFromSvg;
    use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};
    use crate::standards::v1::subsets::drawing::schema::SemioDrawingAnalyzer;
    #[cfg(feature = "conversion-drawing")]
    use semio_framework_plugin::{deserializer_entry_of, register_composer_entries, serializer_entry_of, ComposerEntry};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };

    //#region 🔖️Composer
    pub struct SemioDrawingComposerComposition;

    impl ArtifactComposition for SemioDrawingComposerComposition {
        type Snapshot = SemioDrawingSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "SemioDrawingComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioDrawingAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioDrawingComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decodes the payload as `SemioDrawingSnapshot` and checks two real referential invariants
    /// (both real cross-collection lookups, not decode-only): (1) every `Path`/`Text` node's
    /// `style` reference resolves to a name present in `styles` (dangling-ref detection); (2) every
    /// `DrawLayer.id` is unique across `layers` (duplicate-id detection).
    pub struct SemioDrawingValidator;

    impl SubsetValidator for SemioDrawingValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioDrawingSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioDrawingSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_drawing_invariants(&snapshot),
                None => vec![dsl::Diagnostic::error("stdio.semio_drawing.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioDrawingValidator: payload did not decode as a SemioDrawingSnapshot".to_string())],
            }
        }
    }

    /// 🔎️ Real referential-invariant checks over `SemioDrawingSnapshot`'s own collections (no
    /// cross-artifact lookups needed -- both invariants are internal to this subset).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_drawing_invariants(snapshot: &SemioDrawingSnapshot) -> Vec<dsl::Diagnostic> {
        let mut diagnostics = Vec::new();

        let mut seen_layer_ids = std::collections::HashSet::new();
        for layer in &snapshot.layers {
            if !seen_layer_ids.insert(layer.id.clone()) {
                diagnostics.push(dsl::Diagnostic::error("stdio.semio_drawing.duplicate-layer-id", dsl::TextSpan::at(1, 1), format!("SemioDrawingValidator: duplicate layer id {:?}", layer.id)));
            }
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn walk(node: &DrawNode, style_names: &std::collections::HashSet<&str>, diagnostics: &mut Vec<dsl::Diagnostic>) {
            match node {
                DrawNode::Path { style: Some(name), .. } | DrawNode::Text { style: Some(name), .. } => {
                    if !style_names.contains(name.as_str()) {
                        diagnostics.push(dsl::Diagnostic::error("stdio.semio_drawing.dangling-style-ref", dsl::TextSpan::at(1, 1), format!("SemioDrawingValidator: node references undefined style {name:?}")));
                    }
                }
                DrawNode::Group { children, .. } => {
                    for child in children {
                        walk(child, style_names, diagnostics);
                    }
                }
                _ => {}
            }
        }
        let style_names: std::collections::HashSet<&str> = snapshot.styles.iter().map(|s| s.name.as_str()).collect();
        for layer in &snapshot.layers {
            walk(&layer.root, &style_names, &mut diagnostics);
        }

        diagnostics
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioDrawingValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    /// 🚪️ W4 (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT, group
    /// G4): drawing↔svg/dxf/pdf plus drawing→png. svg is the richest (recursive scene-graph↔scene-graph);
    /// dxf is a real entity↔path translation (exact circles, sampled-flattened curves on export); pdf
    /// export paints paths and text as content-stream operators while pdf import reads text only; png
    /// is this repository's own anti-aliased rasterizer — see each pair's own leaf doc comment.
    #[cfg(feature = "conversion-drawing")]
    static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    #[cfg(feature = "conversion-drawing")]
    fn io_entries() -> &'static [ComposerEntry] {
        IO_ENTRIES
            .get_or_init(|| {
                vec![
                    deserializer_entry_of::<SemioDrawingFromSvg>(),
                    serializer_entry_of::<SemioDrawingToSvg>(),
                    deserializer_entry_of::<SemioDrawingFromDxf>(),
                    serializer_entry_of::<SemioDrawingToDxf>(),
                    deserializer_entry_of::<SemioDrawingFromPdf>(),
                    serializer_entry_of::<SemioDrawingToPdf>(),
                    serializer_entry_of::<SemioDrawingToPng>(),
                ]
            })
            .as_slice()
    }
    //#endregion 🔖️IoEntries

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and (W4) its
    /// semio↔format io bridges. Called from this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::drawing::schema::semio_drawing_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioDrawingSnapshot, crate::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation>(
            crate::standards::v1::subsets::drawing::schema::snapshot::STDIO_SEMIODRAWING_DOCUMENT_SCHEMA,
        ))
        .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-drawing")]
        register_composer_entries(io_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.drawing.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::drawing::schema::inferences::semio_drawing_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(all(test, feature = "conversion-drawing"))]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🧾️Encoding
/// 🧾️ A standard file format a drawing is written as through this subset's own export leaves.
#[cfg(feature = "conversion-drawing")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemioDrawingFormat {
    Svg,
    Dxf,
    Dwg,
    Pdf { version: &'static str },
    Png,
}

/// 🧾️ The file bytes of `drawing` in `format` — the one call every domain artifact that projects
/// into a drawing makes, so each standard format has exactly one writer in the repository.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(feature = "conversion-drawing")]
pub fn encode_drawing(drawing: &crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot, format: SemioDrawingFormat) -> Result<Vec<u8>, String> {
    use crate::standards::v1::subsets::drawing::io::export::serializers::artifacts::{dwg::v_ac1024::any::SemioDrawingToDwg, dxf::v_r12::any::SemioDrawingToDxf, pdf::v1_7::any::drawing_to_pdf, png::v1_2::any::SemioDrawingToPng, svg::v1_1::any::SemioDrawingToSvg};
    use semio_framework_plugin::{resolve_ready, ArtifactSerializer};
    match format {
        SemioDrawingFormat::Svg => resolve_ready(SemioDrawingToSvg::serialize(drawing)).map_err(|e| e.to_string())?.export_utf8(),
        SemioDrawingFormat::Dxf => Ok(semio_s_artifact_stdio_dxf::schema::snapshot::print_dxf_document(&resolve_ready(SemioDrawingToDxf::serialize(drawing)).map_err(|e| e.to_string())?).into_bytes()),
        SemioDrawingFormat::Dwg => semio_s_artifact_stdio_dwg::engine::dwg_to_bytes(&resolve_ready(SemioDrawingToDwg::serialize(drawing)).map_err(|e| e.to_string())?.drawing.to_native()?),
        SemioDrawingFormat::Pdf { version } => {
            let mut pdf = drawing_to_pdf(drawing)?;
            pdf.declared_version = version.to_string();
            semio_s_artifact_stdio_pdf::io::encode_pdf(&pdf).map_err(|e| e.to_string())
        }
        SemioDrawingFormat::Png => semio_s_artifact_stdio_png::io::encode_png(&resolve_ready(SemioDrawingToPng::serialize(drawing)).map_err(|e| e.to_string())?),
    }
}

/// 📥️ `bytes` in `format` read into a drawing through this subset's own import leaves (png has none:
/// a raster carries no vector geometry).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(feature = "conversion-drawing")]
pub fn decode_drawing(bytes: &[u8], format: SemioDrawingFormat) -> Result<crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot, String> {
    use crate::standards::v1::subsets::drawing::io::import::deserializers::artifacts::{dwg::v_ac1024::any::SemioDrawingFromDwg, dxf::v_r12::any::SemioDrawingFromDxf, pdf::v1_7::any::SemioDrawingFromPdf, svg::v1_1::any::SemioDrawingFromSvg};
    use semio_framework_plugin::{resolve_ready, ArtifactDeserializer};
    let text = || std::str::from_utf8(bytes).map_err(|e| e.to_string());
    match format {
        SemioDrawingFormat::Svg => resolve_ready(SemioDrawingFromSvg::deserialize(&semio_s_artifact_stdio_svg::SvgSnapshot::import_utf8(bytes)?)).map_err(|e| e.to_string()),
        SemioDrawingFormat::Dxf => resolve_ready(SemioDrawingFromDxf::deserialize(&semio_s_artifact_stdio_dxf::schema::snapshot::parse_dxf_document(text()?)?)).map_err(|e| e.to_string()),
        SemioDrawingFormat::Dwg => resolve_ready(SemioDrawingFromDwg::deserialize(&semio_s_artifact_stdio_dwg::schema::snapshot::decode_dwg(bytes)?)).map_err(|e| e.to_string()),
        SemioDrawingFormat::Pdf { .. } => resolve_ready(SemioDrawingFromPdf::deserialize(&semio_s_artifact_stdio_pdf::io::decode_pdf(bytes).map_err(|e| e.to_string())?)).map_err(|e| e.to_string()),
        SemioDrawingFormat::Png => Err("semio/drawing←png: a raster carries no vector geometry".into()),
    }
}
//#endregion 🧾️Encoding

//#region 🕸️Diagram
/// 🔷️ The outline of one diagram node, centred on the node's position.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SemioDiagramShape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

/// 🔷️ One node of a node-link diagram in the owner's own coordinates (y down).
#[derive(Clone, Debug, PartialEq)]
pub struct SemioDiagramNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub shape: SemioDiagramShape,
    pub label: Option<String>,
}

/// 🔗️ A straight connection between two nodes by id, drawn boundary to boundary.
#[derive(Clone, Debug, PartialEq)]
pub struct SemioDiagramLink {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
}

/// 🔲️ A labelled axis-aligned frame (a region, a group, a target area) with its top-left corner.
#[derive(Clone, Debug, PartialEq)]
pub struct SemioDiagramFrame {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub label: Option<String>,
}

/// 🕸️ A node-link diagram — the common shape of every board, graph and wiring artifact, so each of
/// them draws (and therefore exports to svg, pdf, png, dxf and dwg) through one routine.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SemioDiagram {
    pub nodes: Vec<SemioDiagramNode>,
    pub links: Vec<SemioDiagramLink>,
    pub frames: Vec<SemioDiagramFrame>,
}

const DIAGRAM_PAD: f64 = 32.0;
const DIAGRAM_TEXT_ADVANCE: f64 = 6.0;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diagram_boundary(node: &SemioDiagramNode, toward: [f64; 2]) -> [f64; 2] {
    let (dx, dy) = (toward[0] - node.x, toward[1] - node.y);
    let length = (dx * dx + dy * dy).sqrt();
    if length == 0.0 {
        return [node.x, node.y];
    }
    let reach = match node.shape {
        SemioDiagramShape::Circle { radius } => radius,
        SemioDiagramShape::Rectangle { width, height } => {
            let horizontal = if dx == 0.0 { f64::INFINITY } else { width / 2.0 * length / dx.abs() };
            let vertical = if dy == 0.0 { f64::INFINITY } else { height / 2.0 * length / dy.abs() };
            horizontal.min(vertical)
        }
    };
    [node.x + dx / length * reach.min(length), node.y + dy / length * reach.min(length)]
}

/// 🕸️ The diagram as a drawing: frames, then links, then nodes, then labels, shifted so the bounding
/// box sits `32` units inside the canvas. Shapes use the exact circle normal form and closed
/// rectangles, so every drawing bridge writes them as real circles and polylines.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diagram_drawing(diagram: &SemioDiagram) -> crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot {
    use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
    use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
    let extent = |node: &SemioDiagramNode| match node.shape {
        SemioDiagramShape::Circle { radius } => [radius, radius],
        SemioDiagramShape::Rectangle { width, height } => [width / 2.0, height / 2.0],
    };
    let corners = diagram.nodes.iter().flat_map(|n| {
        let [ex, ey] = extent(n);
        [[n.x - ex, n.y - ey], [n.x + ex, n.y + ey]]
    });
    let (min, max) = corners.chain(diagram.frames.iter().flat_map(|f| [[f.x, f.y], [f.x + f.width, f.y + f.height]])).fold(([f64::MAX; 2], [f64::MIN; 2]), |(lo, hi), p| ([lo[0].min(p[0]), lo[1].min(p[1])], [hi[0].max(p[0]), hi[1].max(p[1])]));
    let (min, max) = if min[0] <= max[0] { (min, max) } else { ([0.0; 2], [0.0; 2]) };
    let at = |x: f64, y: f64| SemioPoint2 { x: x - min[0] + DIAGRAM_PAD, y: y - min[1] + DIAGRAM_PAD };
    let rgba = |r: f32, g: f32, b: f32| Some(SemioRgba { r, g, b, a: 1.0 });
    let styles = vec![
        DrawStyle { name: "diagram-frame".into(), fill: None, stroke: rgba(0.62, 0.66, 0.72), stroke_width: Some(1.0), opacity: None },
        DrawStyle { name: "diagram-link".into(), fill: None, stroke: rgba(0.38, 0.42, 0.48), stroke_width: Some(1.5), opacity: None },
        DrawStyle { name: "diagram-node".into(), fill: rgba(0.91, 0.94, 1.0), stroke: rgba(0.16, 0.29, 0.62), stroke_width: Some(1.5), opacity: None },
        DrawStyle { name: "diagram-label".into(), fill: rgba(0.1, 0.1, 0.12), stroke: None, stroke_width: None, opacity: None },
    ];
    let rectangle = |x: f64, y: f64, width: f64, height: f64, style: &str| DrawNode::Path {
        segments: vec![PathSegment::MoveTo { to: at(x, y) }, PathSegment::LineTo { to: at(x + width, y) }, PathSegment::LineTo { to: at(x + width, y + height) }, PathSegment::LineTo { to: at(x, y + height) }, PathSegment::Close],
        style: Some(style.into()),
    };
    let label = |text: &str, x: f64, y: f64| DrawNode::Text { value: text.to_string(), at: at(x - text.chars().count() as f64 * DIAGRAM_TEXT_ADVANCE / 2.0, y + 4.0), style: Some("diagram-label".into()) };
    let mut children = Vec::new();
    for frame in &diagram.frames {
        children.push(rectangle(frame.x, frame.y, frame.width, frame.height, "diagram-frame"));
        if let Some(text) = &frame.label {
            children.push(label(text, frame.x + frame.width / 2.0, frame.y + 12.0));
        }
    }
    let by_id: std::collections::HashMap<&str, &SemioDiagramNode> = diagram.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    for link in &diagram.links {
        let (Some(from), Some(to)) = (by_id.get(link.from.as_str()), by_id.get(link.to.as_str())) else { continue };
        let (start, end) = (diagram_boundary(from, [to.x, to.y]), diagram_boundary(to, [from.x, from.y]));
        children.push(DrawNode::Path { segments: vec![PathSegment::MoveTo { to: at(start[0], start[1]) }, PathSegment::LineTo { to: at(end[0], end[1]) }], style: Some("diagram-link".into()) });
        if let Some(text) = &link.label {
            children.push(label(text, (start[0] + end[0]) / 2.0, (start[1] + end[1]) / 2.0 - 8.0));
        }
    }
    for node in &diagram.nodes {
        children.push(match node.shape {
            SemioDiagramShape::Circle { radius } => DrawNode::Path {
                segments: vec![
                    PathSegment::MoveTo { to: at(node.x + radius, node.y) },
                    PathSegment::ArcTo { rx: radius, ry: radius, x_rotation: 0.0, large_arc: false, sweep: true, to: at(node.x - radius, node.y) },
                    PathSegment::ArcTo { rx: radius, ry: radius, x_rotation: 0.0, large_arc: false, sweep: true, to: at(node.x + radius, node.y) },
                    PathSegment::Close,
                ],
                style: Some("diagram-node".into()),
            },
            SemioDiagramShape::Rectangle { width, height } => rectangle(node.x - width / 2.0, node.y - height / 2.0, width, height, "diagram-node"),
        });
    }
    for node in &diagram.nodes {
        if let Some(text) = &node.label {
            children.push(label(text, node.x, node.y));
        }
    }
    SemioDrawingSnapshot {
        canvas: DrawCanvas { width: (max[0] - min[0] + 2.0 * DIAGRAM_PAD).max(64.0), height: (max[1] - min[1] + 2.0 * DIAGRAM_PAD).max(64.0), background: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }) },
        styles,
        layers: vec![DrawLayer { id: "diagram".into(), name: "diagram".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
        ..SemioDrawingSnapshot::default()
    }
}
//#endregion 🕸️Diagram

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️diagram-unit/🦀️.rs"]
mod diagram_tests;
//#endregion 🧪️Tests
