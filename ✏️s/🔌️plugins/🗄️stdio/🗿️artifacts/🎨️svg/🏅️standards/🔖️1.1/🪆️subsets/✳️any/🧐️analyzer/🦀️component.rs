//! 🧐️ SvgAnalyzer (1.1/✳️any) — read-only analysis, successor to the pre-migration
//! SvgDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::svg::schema::snapshot::{svg_document_to_typed, SvgElement};
use crate::artifacts::svg::SvgSnapshot;
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, XmlNode};

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
pub struct SvgAnalyzer;

impl ArtifactAnalyzer for SvgAnalyzer {
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
                AnalyzeSource::Text(text) => match <SvgSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.analyze.text",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <SvgSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                    Ok(snapshot) => {
                        if let Ok(typed) = svg_document_to_typed(&snapshot.doc) {
                            parts.typed = Some(typed);
                        }
                        parts.snapshot = Some(snapshot);
                    }
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.analyze.binary",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
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
mod tests {
    use super::*;
    use crate::artifacts::svg::schema::snapshot::{parse_view_box, CommonAttrs, ViewBox};
    use crate::artifacts::svg::standards::v1_1::subsets::any::builder::{ElementBuilder, GradientStopSpec, PathBuilder, SvgBuilder};
    use semio_framework_plugin::ArtifactBuilder;

    #[test]
    fn sniff_recognizes_real_svg_and_rejects_non_svg() {
        let svg = r#"<?xml version="1.0"?><svg xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="1" height="1"/></svg>"#;
        assert_eq!(SvgAnalyzer::sniff(&AnalyzeSource::Text(svg)), IoConfidence::High);
        let not_svg = r#"<note><to>Tove</to></note>"#;
        assert_ne!(SvgAnalyzer::sniff(&AnalyzeSource::Text(not_svg)), IoConfidence::High);
    }

    #[test]
    fn builder_constructs_a_complete_document_from_scratch() {
        let stops = vec![GradientStopSpec::new("0%").with_color("#ffffff"), GradientStopSpec::new("100%").with_color("#000000")];
        let snapshot = SvgBuilder::empty()
            .set_view_box(0.0, 0.0, 200.0, 100.0)
            .set_dimensions("200", "100")
            .define_linear_gradient("grad1", Some(0.0), Some(0.0), Some(1.0), Some(0.0), stops)
            .add_group(CommonAttrs::new().with_id("layer1"), |g: ElementBuilder| {
                g.add_rect(10.0, 10.0, 80.0, 40.0, CommonAttrs::new().with_fill("url(#grad1)"))
                    .add_circle(150.0, 50.0, 30.0, CommonAttrs::new().with_fill("red").with_stroke("black"))
                    .add_path(
                        PathBuilder::new().move_to(10.0, 80.0).line_to(50.0, 80.0).arc_to(20.0, 20.0, 0.0, false, true, 90.0, 80.0).close(),
                        CommonAttrs::new().with_stroke("blue"),
                    )
            })
            .build()
            .expect("build succeeds");

        let typed = svg_document_to_typed(&snapshot.doc).expect("typed conversion");
        let (view_box, children) = match &typed {
            SvgElement::Svg { view_box, children, .. } => (view_box.clone(), children.clone()),
            other => panic!("expected Svg root, got {other:?}"),
        };
        assert_eq!(view_box, Some(ViewBox { min_x: 0.0, min_y: 0.0, width: 200.0, height: 100.0 }));
        assert_eq!(children.len(), 2, "expected linearGradient + group");
        assert!(matches!(children[0], SvgElement::LinearGradient { .. }));
        let group_children = match &children[1] {
            SvgElement::Group { children, .. } => children,
            other => panic!("expected Group, got {other:?}"),
        };
        assert_eq!(group_children.len(), 3);

        // Round trip through the real text codec confirms the built document is well-formed SVG.
        let text = store::ArtifactDsl::print_dsl(&snapshot);
        let reparsed = <SvgSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("reparse");
        let retyped = svg_document_to_typed(&reparsed.doc).expect("retyped");
        assert_eq!(retyped, typed);
        assert_eq!(parse_view_box("0 0 200 100").unwrap(), view_box.unwrap());
    }

    /// 🔁 Core acceptance pattern (plan D2): parse a real document, walk it via the analyzer's
    /// typed output, reconstruct it via ONLY typed builder calls, and confirm the typed structures
    /// are equivalent. Uses a subset of elements the typed builder can construct (rect/circle/path
    /// w/ arc, nested group, linear gradient + stops) -- text/tspan/use/Unknown reconstruction is
    /// outside this particular round trip by design (those are covered by parse-fidelity tests
    /// elsewhere, not by builder reconstruction, since `Unknown` is deliberately not
    /// builder-constructible: it exists purely as a lossless parse escape hatch).
    #[test]
    fn analyzer_to_builder_round_trip() {
        // 🚧️ `r##"..."##`: the fixture's `stop-color="#ff0000"` contains the literal sequence
        // `"#`, which would otherwise close a single-hash raw string early.
        let source_text = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <defs>
    <linearGradient id="g1" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0%" stop-color="#ff0000"/>
      <stop offset="100%" stop-color="#0000ff"/>
    </linearGradient>
  </defs>
  <g id="shapes" transform="translate(5,5)">
    <rect x="0" y="0" width="40" height="20" fill="url(#g1)"/>
    <circle cx="60" cy="20" r="15" style="fill: green; stroke: black; stroke-width: 2"/>
    <path d="M10,50 L40,50 A5 5 0 108 8 Z"/>
  </g>
</svg>"##;

        let analysis = SvgAnalyzer::analyze(&[AnalyzeSource::Text(source_text)]);
        assert!(analysis.diagnostics.is_empty(), "diagnostics: {:?}", analysis.diagnostics);
        let original_typed = analysis.parts.typed.expect("typed parts present");

        // 🧵 The fixture is pretty-printed, so raw children include whitespace-only text nodes
        // between elements (preserved losslessly by design) -- filter those before indexing.
        fn elements_only(v: &[SvgElement]) -> Vec<SvgElement> {
            v.iter().filter(|c| !matches!(c, SvgElement::TextNode(_))).cloned().collect()
        }
        /// 🧹 Strips whitespace-only text nodes recursively, so structural comparison between a
        /// parsed (pretty-printed, whitespace-bearing) document and a builder-reconstructed one
        /// (which never emits layout whitespace) is apples-to-apples.
        fn strip_whitespace(el: &SvgElement) -> SvgElement {
            fn strip_all(children: &[SvgElement]) -> Vec<SvgElement> {
                elements_only(children).iter().map(strip_whitespace).collect()
            }
            match el.clone() {
                SvgElement::Svg { common, view_box, width, height, xmlns, children } => {
                    SvgElement::Svg { common, view_box, width, height, xmlns, children: strip_all(&children) }
                }
                SvgElement::Group { common, children } => SvgElement::Group { common, children: strip_all(&children) },
                SvgElement::Defs { common, children } => SvgElement::Defs { common, children: strip_all(&children) },
                SvgElement::LinearGradient { common, id, x1, y1, x2, y2, children } => {
                    SvgElement::LinearGradient { common, id, x1, y1, x2, y2, children: strip_all(&children) }
                }
                SvgElement::RadialGradient { common, id, cx, cy, r, fx, fy, children } => {
                    SvgElement::RadialGradient { common, id, cx, cy, r, fx, fy, children: strip_all(&children) }
                }
                other => other,
            }
        }

        let (defs_children, group_common, group_children, xmlns) = match &original_typed {
            SvgElement::Svg { children, xmlns, .. } => {
                let top = elements_only(children);
                let defs_children = match &top[0] {
                    SvgElement::Defs { children, .. } => elements_only(children),
                    other => panic!("expected Defs, got {other:?}"),
                };
                let (group_common, group_children) = match &top[1] {
                    SvgElement::Group { common, children } => (common.clone(), elements_only(children)),
                    other => panic!("expected Group, got {other:?}"),
                };
                (defs_children, group_common, group_children, xmlns.clone())
            }
            other => panic!("expected Svg root, got {other:?}"),
        };

        let (grad_id, grad_x1, grad_y1, grad_x2, grad_y2, stop_specs) = match &defs_children[0] {
            SvgElement::LinearGradient { id, x1, y1, x2, y2, children, .. } => (
                id.clone(),
                x1.as_ref().and_then(|s| s.parse::<f64>().ok()),
                y1.as_ref().and_then(|s| s.parse::<f64>().ok()),
                x2.as_ref().and_then(|s| s.parse::<f64>().ok()),
                y2.as_ref().and_then(|s| s.parse::<f64>().ok()),
                children
                    .iter()
                    .filter_map(|c| match c {
                        SvgElement::Stop { offset, stop_color, stop_opacity, .. } => {
                            let mut spec = GradientStopSpec::new(offset.clone());
                            if let Some(c) = stop_color { spec = spec.with_color(c.clone()); }
                            if let Some(o) = stop_opacity { spec = spec.with_opacity(o.clone()); }
                            Some(spec)
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
            ),
            other => panic!("expected LinearGradient, got {other:?}"),
        };

        let mut builder = SvgBuilder::empty().set_view_box(0.0, 0.0, 100.0, 100.0);
        if let Some(xmlns) = xmlns {
            builder = builder.set_xmlns(xmlns);
        }
        let rebuilt_snapshot = builder
            .add_defs(CommonAttrs::default(), |d: ElementBuilder| {
                d.define_linear_gradient(grad_id.unwrap_or_default(), grad_x1, grad_y1, grad_x2, grad_y2, stop_specs)
            })
            .add_group(group_common, |mut g: ElementBuilder| {
                for child in &group_children {
                    g = rebuild_one(g, child);
                }
                g
            })
            .build()
            .expect("rebuild succeeds");

        let rebuilt_typed = svg_document_to_typed(&rebuilt_snapshot.doc).expect("typed rebuilt");
        assert_eq!(strip_whitespace(&original_typed), strip_whitespace(&rebuilt_typed));
    }

    /// 🔁 Drives ONE typed builder call per typed element, recursing into containers. Used only by
    /// `analyzer_to_builder_round_trip` above.
    fn rebuild_one(eb: ElementBuilder, el: &SvgElement) -> ElementBuilder {
        match el {
            SvgElement::Rect { common, x, y, width, height, rx, ry } => match (rx, ry) {
                (Some(rx), Some(ry)) => eb.add_rect_rounded(*x, *y, *width, *height, *rx, *ry, common.clone()),
                _ => eb.add_rect(*x, *y, *width, *height, common.clone()),
            },
            SvgElement::Circle { common, cx, cy, r } => eb.add_circle(*cx, *cy, *r, common.clone()),
            SvgElement::Ellipse { common, cx, cy, rx, ry } => eb.add_ellipse(*cx, *cy, *rx, *ry, common.clone()),
            SvgElement::Line { common, x1, y1, x2, y2 } => eb.add_line(*x1, *y1, *x2, *y2, common.clone()),
            SvgElement::Polyline { common, points } => eb.add_polyline(points.clone(), common.clone()),
            SvgElement::Polygon { common, points } => eb.add_polygon(points.clone(), common.clone()),
            SvgElement::Path { common, d } => eb.add_path(PathBuilder::from_commands(d.clone()), common.clone()),
            SvgElement::Group { common, children } => eb.add_group(common.clone(), |mut inner| {
                for c in children {
                    inner = rebuild_one(inner, c);
                }
                inner
            }),
            SvgElement::Use { common, href, x, y, width, height } => eb.add_use(href.clone(), *x, *y, *width, *height, common.clone()),
            // 🚪 Text/Unknown/raw-node variants aren't part of this round trip's fixture (see the
            // test's doc comment); left as identity so the match stays exhaustive and honest.
            _ => eb,
        }
    }
}
//#endregion 🧪️Tests
