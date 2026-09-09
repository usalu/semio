use super::*;

#[semio_framework_async_macros::async_test]
async fn schema_facets_reject_source_and_raw_doctype_shadow_state() {
    let facets = [
        include_str!("../../🦀️.rs"),
        include_str!("../../🟦️.ts"),
        include_str!("../../🔣️.json"),
        include_str!("../../📝️text/🔗️.graphql"),
        include_str!("../../📝️text/🛰️.proto"),
        include_str!("../../../🔺️diff/📝️text/📖️.grammar.semio"),
        include_str!("../../../🔺️diff/💾️binary/📡️.protocol.semio"),
        include_str!("../../../🧬️mutations/📝️text/📖️.grammar.semio"),
        include_str!("../../../🧬️mutations/💾️binary/📡️.protocol.semio"),
    ];
    for facet in facets {
        for forbidden in [concat!("pub doctype: Option<", "String>"), concat!("raw ", "doctype"), concat!("source-", "field"), concat!("source-", "tok"), concat!("artifact-", "source"), concat!("semantic-", "blake3")] {
            assert!(!facet.to_ascii_lowercase().contains(&forbidden.to_ascii_lowercase()), "forbidden SVG shadow-state facet: {forbidden}");
        }
    }

    let persistence_facets = [
        include_str!("../../../🔺️diff/📝️text/🔤️.ebnf"),
        include_str!("../../../🔺️diff/📝️text/🅰️.g4"),
        include_str!("../../../🔺️diff/📝️text/🔗️.graphql"),
        include_str!("../../../🔺️diff/📝️text/🔣️.json"),
        include_str!("../../../🔺️diff/📝️text/🛰️.proto"),
        include_str!("../../../🔺️diff/📝️text/🟦️.ts"),
        include_str!("../../../🔺️diff/💾️binary/🌶️.spicy"),
        include_str!("../../../🔺️diff/💾️binary/🥋️.ksy"),
        include_str!("../../../🔺️diff/💾️binary/🔠️.abnf"),
        include_str!("../../../🔺️diff/💾️binary/🟦️.ts"),
        include_str!("../../../🧬️mutations/📝️text/🔤️.ebnf"),
        include_str!("../../../🧬️mutations/📝️text/🅰️.g4"),
        include_str!("../../../🧬️mutations/📝️text/🔗️.graphql"),
        include_str!("../../../🧬️mutations/📝️text/🔣️.json"),
        include_str!("../../../🧬️mutations/📝️text/🛰️.proto"),
        include_str!("../../../🧬️mutations/📝️text/🟦️.ts"),
        include_str!("../../../🧬️mutations/💾️binary/🌶️.spicy"),
        include_str!("../../../🧬️mutations/💾️binary/🥋️.ksy"),
        include_str!("../../../🧬️mutations/💾️binary/🔠️.abnf"),
        include_str!("../../../🧬️mutations/💾️binary/🟦️.ts"),
    ];
    for facet in persistence_facets {
        assert!(!facet.to_ascii_lowercase().contains("json"), "SVG diff/mutation persistence facet must describe the structured codec");
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_dsl_rejects_native_svg_without_a_semio_envelope() {
    assert!(<SvgSnapshot as store::ArtifactDsl>::parse_dsl(r#"<svg xmlns="http://www.w3.org/2000/svg"/>"#).is_err());
}

//#region PathGrammar
#[semio_framework_async_macros::async_test]
async fn path_implicit_lineto_repetition_after_moveto() {
    let cmds = parse_path_data("M 0 0 10 10 20 20").unwrap();
    assert_eq!(cmds, vec![PathCommand::MoveTo { x: 0.0, y: 0.0, relative: false }, PathCommand::LineTo { x: 10.0, y: 10.0, relative: false }, PathCommand::LineTo { x: 20.0, y: 20.0, relative: false },]);
}

#[semio_framework_async_macros::async_test]
async fn path_arc_flag_squeeze_decomposes_correctly() {
    // 🚩 THE classic bug: "A5 5 0 108 8" must decompose as flags 1,0 then x=8,y=8 -- not 10,8,8.
    let cmds = parse_path_data("M40,20 A5 5 0 108 8").unwrap();
    match cmds.last().unwrap() {
        PathCommand::Arc { rx, ry, x_axis_rotation, large_arc, sweep, x, y, relative } => {
            assert_eq!((*rx, *ry, *x_axis_rotation), (5.0, 5.0, 0.0));
            assert_eq!((*large_arc, *sweep), (true, false));
            assert_eq!((*x, *y, *relative), (8.0, 8.0, false));
        }
        other => panic!("expected Arc, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn path_relative_and_close() {
    let cmds = parse_path_data("m0,0 10,0 0,10z").unwrap();
    assert_eq!(cmds, vec![PathCommand::MoveTo { x: 0.0, y: 0.0, relative: true }, PathCommand::LineTo { x: 10.0, y: 0.0, relative: true }, PathCommand::LineTo { x: 0.0, y: 10.0, relative: true }, PathCommand::ClosePath,]);
}

#[semio_framework_async_macros::async_test]
async fn path_round_trips_through_string() {
    let cmds = parse_path_data("M0,0 C1,1 2,2 3,3 S4,4 5,5 A5 5 0 108 8 Z").unwrap();
    let text = path_data_to_string(&cmds);
    let reparsed = parse_path_data(&text).unwrap();
    assert_eq!(cmds, reparsed);
}

#[semio_framework_async_macros::async_test]
async fn path_missing_leading_command_is_error() {
    assert!(parse_path_data("10 10 L20 20").is_err());
}
//#endregion PathGrammar

//#region TransformGrammar
#[semio_framework_async_macros::async_test]
async fn transform_parses_all_functions() {
    let ops = parse_transform_list("translate(10,20) scale(2) rotate(90,5,5) skewX(30) skewY(-15) matrix(1,0,0,1,0,0)").unwrap();
    assert_eq!(
        ops,
        vec![
            TransformOp::Translate { x: 10.0, y: Some(20.0) },
            TransformOp::Scale { x: 2.0, y: None },
            TransformOp::Rotate { angle: 90.0, center: Some((5.0, 5.0)) },
            TransformOp::SkewX { angle: 30.0 },
            TransformOp::SkewY { angle: -15.0 },
            TransformOp::Matrix { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 },
        ]
    );
}

#[semio_framework_async_macros::async_test]
async fn transform_composition_order_matches_svg_semantics() {
    // translate(10,0) rotate(90) applied to (1,0) => (10,1): rotate happens in local space first.
    let m = transform_ops_to_matrix(&parse_transform_list("translate(10,0) rotate(90)").unwrap());
    let (px, py) = (m.a * 1.0 + m.c * 0.0 + m.e, m.b * 1.0 + m.d * 0.0 + m.f);
    assert!((px - 10.0).abs() < 1e-9 && (py - 1.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn transform_rotate_about_center_fixes_that_point() {
    let m = transform_ops_to_matrix(&parse_transform_list("rotate(45,7,3)").unwrap());
    let (px, py) = (m.a * 7.0 + m.c * 3.0 + m.e, m.b * 7.0 + m.d * 3.0 + m.f);
    assert!((px - 7.0).abs() < 1e-9 && (py - 3.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn transform_wrong_arity_is_error() {
    assert!(parse_transform_list("scale(1,2,3)").is_err());
    assert!(parse_transform_list("frobnicate(1)").is_err());
}
//#endregion TransformGrammar

//#region StyleAndGeometry
#[semio_framework_async_macros::async_test]
async fn style_declarations_parse_and_unrecognized_ones_are_retained() {
    let mut p = PresentationAttrs::default();
    for (k, v) in parse_style_decls("fill: red; stroke:blue ; opacity:0.5; letter-spacing: 2px") {
        if !apply_presentation_attr(&mut p, &k, &v) {
            p.extra_style.push((k, v));
        }
    }
    assert_eq!(p.fill.as_deref(), Some("red"));
    assert_eq!(p.stroke.as_deref(), Some("blue"));
    assert_eq!(p.opacity.as_deref(), Some("0.5"));
    assert_eq!(p.extra_style, vec![("letter-spacing".to_string(), "2px".to_string())]);
}

#[semio_framework_async_macros::async_test]
async fn view_box_and_points_parse() {
    assert_eq!(parse_view_box("0 0 100 50").unwrap(), ViewBox { min_x: 0.0, min_y: 0.0, width: 100.0, height: 50.0 });
    assert_eq!(parse_points("0,0 10,0 5,10").unwrap(), vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)]);
    assert!(parse_view_box("0 0 100").is_err());
    assert!(parse_points("0,0 10").is_err());
}
//#endregion StyleAndGeometry

//#region TypedParse
#[semio_framework_async_macros::async_test]
async fn typed_parse_of_multi_element_document_with_gradient_group_and_arc_path() {
    // 🚧️ `r##"..."##` (not `r#"..."#`) because the fixture's `stop-color="#ff0000"` contains
    // the literal 2-char sequence `"#`, which would otherwise close a single-hash raw string.
    let text = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
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
    let doc = xml_document_from_text(text).expect("xml parses");
    let typed = svg_document_to_typed(&doc).expect("typed conversion");

    // 🧵 The fixture is pretty-printed, so raw XML children include whitespace-only text nodes
    // between elements (preserved losslessly, per the typed model's design) -- filter those
    // out before indexing into the REAL element children.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn elements_only(v: &[SvgElement]) -> Vec<&SvgElement> {
        v.iter().filter(|c| !matches!(c, SvgElement::TextNode(_))).collect()
    }

    let (view_box, children) = match &typed {
        SvgElement::Svg { view_box, children, .. } => (view_box.clone(), elements_only(children)),
        other => panic!("expected Svg root, got {other:?}"),
    };
    assert_eq!(view_box, Some(ViewBox { min_x: 0.0, min_y: 0.0, width: 100.0, height: 100.0 }));
    assert_eq!(children.len(), 2, "expected <defs> and <g> as direct children");

    let defs_children = match children[0] {
        SvgElement::Defs { children, .. } => elements_only(children),
        other => panic!("expected Defs, got {other:?}"),
    };
    let (id, stops) = match defs_children[0] {
        SvgElement::LinearGradient { id, children, .. } => (id.clone(), elements_only(children)),
        other => panic!("expected LinearGradient, got {other:?}"),
    };
    assert_eq!(id.as_deref(), Some("g1"));
    assert_eq!(stops.len(), 2);
    match stops[0] {
        SvgElement::Stop { offset, stop_color, .. } => {
            assert_eq!(offset, "0%");
            assert_eq!(stop_color.as_deref(), Some("#ff0000"));
        }
        other => panic!("expected Stop, got {other:?}"),
    }

    let (group_common, group_children) = match children[1] {
        SvgElement::Group { common, children } => (common, elements_only(children)),
        other => panic!("expected Group, got {other:?}"),
    };
    assert_eq!(group_common.id.as_deref(), Some("shapes"));
    assert_eq!(group_common.transform, Some(vec![TransformOp::Translate { x: 5.0, y: Some(5.0) }]));
    assert_eq!(group_children.len(), 3);

    match group_children[0] {
        SvgElement::Rect { common, x, y, width, height, .. } => {
            assert_eq!((*x, *y, *width, *height), (0.0, 0.0, 40.0, 20.0));
            assert_eq!(common.presentation.fill.as_deref(), Some("url(#g1)"));
        }
        other => panic!("expected Rect, got {other:?}"),
    }
    match group_children[1] {
        SvgElement::Circle { common, cx, cy, r } => {
            assert_eq!((*cx, *cy, *r), (60.0, 20.0, 15.0));
            // 🎨 style="" must win and be REAL-parsed, not string-matched.
            assert_eq!(common.presentation.fill.as_deref(), Some("green"));
            assert_eq!(common.presentation.stroke.as_deref(), Some("black"));
            assert_eq!(common.presentation.stroke_width.as_deref(), Some("2"));
        }
        other => panic!("expected Circle, got {other:?}"),
    }
    match group_children[2] {
        SvgElement::Path { d, .. } => {
            assert_eq!(
                d,
                &vec![
                    PathCommand::MoveTo { x: 10.0, y: 50.0, relative: false },
                    PathCommand::LineTo { x: 40.0, y: 50.0, relative: false },
                    PathCommand::Arc { rx: 5.0, ry: 5.0, x_axis_rotation: 0.0, large_arc: true, sweep: false, x: 8.0, y: 8.0, relative: false },
                    PathCommand::ClosePath,
                ]
            );
        }
        other => panic!("expected Path, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn unknown_element_and_attrs_survive_losslessly() {
    let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><customThing data-x="1"><rect x="0" y="0" width="1" height="1"/></customThing></svg>"#;
    let doc = xml_document_from_text(text).unwrap();
    let typed = svg_document_to_typed(&doc).unwrap();
    let children = match &typed {
        SvgElement::Svg { children, .. } => children,
        other => panic!("expected Svg, got {other:?}"),
    };
    match &children[0] {
        SvgElement::Unknown { name, attrs, children } => {
            assert_eq!(name, "customThing");
            assert_eq!(attrs.iter().find(|a| a.name == "data-x").map(|a| a.value.as_str()), Some("1"));
            assert_eq!(children.len(), 1);
            assert!(matches!(children[0], SvgElement::Rect { .. }));
        }
        other => panic!("expected Unknown, got {other:?}"),
    }
    // Round trip: the typed model lowers back to an XmlDocument that reparses identically.
    let doc2 = typed_to_svg_document(&typed, None);
    let typed2 = svg_document_to_typed(&doc2).unwrap();
    assert_eq!(typed, typed2);
}
//#endregion TypedParse
