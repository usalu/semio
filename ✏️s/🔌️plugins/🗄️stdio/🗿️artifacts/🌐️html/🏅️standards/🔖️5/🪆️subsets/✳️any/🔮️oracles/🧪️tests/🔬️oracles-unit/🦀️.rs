mod tests {
    use super::*;

    const REAL_FIXTURE: &[u8] = include_bytes!("../../../🧫️fixtures/🏚️zukunft-bau-entwerfen-mit-bestand/🌐️.html");

    fn obj(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    //#region 🔖️RealFixtureShape
    /// 🧪️ Confirms the real, derived fixture (the committed real TYPO3-produced page, with its own
    /// two already-`<link>`/`<script src>`-referenced real external files `overwrite.css`/
    /// `default_frontend.js` inlined once — see this case's `component.feature` for the full
    /// derivation note) parses into the shape every other test in this module addresses by a
    /// literal `NodePath`: `html` root, `head` before `body`, both present.
    #[test]
    fn real_fixture_parses_into_the_shape_every_other_test_addresses() {
        let doc = parse(REAL_FIXTURE).expect("real fixture parses");
        assert_eq!(doc.doctype.as_deref(), Some("DOCTYPE html"));
        let HNode::Element { name, children, .. } = doc.root.as_ref().expect("root element") else { panic!("root must be an element") };
        assert_eq!(name, "html");
        let head = children
            .iter()
            .enumerate()
            .find_map(|(i, c)| match c {
                HNode::Element { name, .. } if name == "head" => Some(i),
                _ => None,
            })
            .expect("head element present");
        let body = children
            .iter()
            .enumerate()
            .find_map(|(i, c)| match c {
                HNode::Element { name, .. } if name == "body" => Some(i),
                _ => None,
            })
            .expect("body element present");
        assert!(head < body, "head must precede body");
        assert!(matches!(resolve(doc.root.as_ref(), &[0, 5]), Some(HNode::Comment(_))), "[0,5] must be the real TYPO3 license comment");
        assert!(matches!(resolve(doc.root.as_ref(), &[0, 47, 0]), Some(HNode::RawText { script: false, .. })), "[0,47,0] must be the inlined overwrite.css raw-text");
        assert!(matches!(resolve(doc.root.as_ref(), &[2, 29, 0]), Some(HNode::RawText { script: true, .. })), "[2,29,0] must be the inlined default_frontend.js raw-text");
        assert!(matches!(resolve(doc.root.as_ref(), &[0, 9, 0]), Some(HNode::Text(_))), "[0,9,0] must be the real <title> text");
        assert!(matches!(resolve(doc.root.as_ref(), &[2, 9]), Some(HNode::Element { name, .. }) if name == "div"), "[2,9] must be the real sidebars div");
    }
    //#endregion 🔖️RealFixtureShape

    //#region 🔖️SmallFixtureUnitLaws
    #[test]
    fn no_mutation_is_a_true_semantic_identity() {
        let input = b"<!doctype html>\n<html><body>hi</body></html>\n";
        let output = apply_mutation(input, "no-mutation", &Json::Object(vec![])).unwrap();
        assert_eq!(parse(input).unwrap(), parse(&output).unwrap());
    }

    #[test]
    fn insert_and_remove_node_are_inverse() {
        let input = b"<!doctype html>\n<html><body><p>a</p></body></html>";
        let node = || obj(vec![("kind", Json::String("element".into())), ("name", Json::String("span".into())), ("attributes", Json::Array(vec![])), ("children", Json::Array(vec![]))]);
        let inserted = apply_mutation(input, "insert-node", &obj(vec![("parent", Json::Array(vec![Json::Number(1.0)])), ("index", Json::Number(0.0)), ("node", node())])).unwrap();
        match resolve(parse(&inserted).unwrap().root.as_ref(), &[1]) {
            Some(HNode::Element { children, .. }) => assert_eq!(children.len(), 2, "body should now hold the original <p> plus the inserted <span>"),
            other => panic!("unexpected: {other:?}"),
        }
        let round_tripped = apply_mutation_inverse(input, "insert-node", &obj(vec![("parent", Json::Array(vec![Json::Number(1.0)])), ("index", Json::Number(0.0)), ("node", node())])).unwrap();
        assert_eq!(parse(&round_tripped).unwrap(), parse(input).unwrap());
    }

    #[test]
    fn set_element_name_and_its_inverse_round_trip() {
        let input = b"<!doctype html>\n<html><body><div>x</div></body></html>";
        let round_tripped = apply_mutation_inverse(input, "set-element-name", &obj(vec![("path", Json::Array(vec![Json::Number(1.0), Json::Number(0.0)])), ("name", Json::String("section".into()))])).unwrap();
        assert_eq!(parse(&round_tripped).unwrap(), parse(input).unwrap());
    }

    #[test]
    fn set_attribute_and_its_inverse_round_trip() {
        let input = b"<!doctype html>\n<html><body id=\"a\"></body></html>";
        let round_tripped = apply_mutation_inverse(input, "set-attribute", &obj(vec![("path", Json::Array(vec![Json::Number(1.0)])), ("name", Json::String("id".into())), ("value", Json::String("b".into()))])).unwrap();
        assert_eq!(parse(&round_tripped).unwrap(), parse(input).unwrap());
    }

    #[test]
    fn set_text_and_its_inverse_round_trip() {
        let input = b"<!doctype html>\n<html><body><p>hi</p></body></html>";
        let round_tripped = apply_mutation_inverse(input, "set-text", &obj(vec![("path", Json::Array(vec![Json::Number(1.0), Json::Number(0.0), Json::Number(0.0)])), ("text", Json::String("bye".into()))])).unwrap();
        assert_eq!(parse(&round_tripped).unwrap(), parse(input).unwrap());
    }

    #[test]
    fn set_comment_and_its_inverse_round_trip() {
        let input = b"<!doctype html>\n<html><!-- old --><body></body></html>";
        let round_tripped = apply_mutation_inverse(input, "set-comment", &obj(vec![("path", Json::Array(vec![Json::Number(0.0)])), ("text", Json::String(" new ".into()))])).unwrap();
        assert_eq!(parse(&round_tripped).unwrap(), parse(input).unwrap());
    }

    #[test]
    fn set_doctype_and_its_inverse_round_trip() {
        let input = b"<!doctype html>\n<html><body></body></html>";
        let round_tripped = apply_mutation_inverse(input, "set-doctype", &obj(vec![("doctype", Json::String("DOCTYPE htmlWave7".into()))])).unwrap();
        assert_eq!(parse(&round_tripped).unwrap(), parse(input).unwrap());
    }

    #[test]
    fn set_snapshot_and_its_inverse_round_trip() {
        let input = b"<!doctype html>\n<html><body>original</body></html>";
        let root = obj(vec![
            ("kind", Json::String("element".into())),
            ("name", Json::String("html".into())),
            ("attributes", Json::Array(vec![])),
            (
                "children",
                Json::Array(vec![obj(vec![
                    ("kind", Json::String("element".into())),
                    ("name", Json::String("body".into())),
                    ("attributes", Json::Array(vec![])),
                    ("children", Json::Array(vec![obj(vec![("kind", Json::String("text".into())), ("text", Json::String("replaced".into()))])])),
                ])]),
            ),
        ]);
        let round_tripped = apply_mutation_inverse(input, "set-snapshot", &obj(vec![("doctype", Json::String("DOCTYPE html".into())), ("root", root)])).unwrap();
        assert_eq!(parse(&round_tripped).unwrap(), parse(input).unwrap());
    }

    #[test]
    fn script_and_style_content_survive_as_raw_text_and_its_inverse_round_trips() {
        let input = b"<!doctype html>\n<html><head><style>.a { color: red; }</style><script>if (1 < 2) { console.log(1); }</script></head><body></body></html>";
        let doc = parse(input).unwrap();
        let HNode::Element { children, .. } = doc.root.as_ref().unwrap() else { panic!("root") };
        let HNode::Element { children: head_children, .. } = &children[0] else { panic!("head") };
        let HNode::Element { name: style_name, children: style_c, .. } = &head_children[0] else { panic!("style") };
        assert_eq!(style_name, "style");
        assert!(matches!(&style_c[0], HNode::RawText { script: false, text } if text.contains("color: red")));
        let HNode::Element { children: script_children, .. } = &head_children[1] else { panic!("script") };
        assert!(matches!(&script_children[0], HNode::RawText { script: true, text } if text.contains("console.log")));

        let round_tripped = apply_mutation_inverse(input, "set-raw-text", &obj(vec![("path", Json::Array(vec![Json::Number(0.0), Json::Number(1.0), Json::Number(0.0)])), ("text", Json::String("console.log(2);".into()))])).unwrap();
        assert_eq!(parse(&round_tripped).unwrap(), doc);
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let input = b"<!doctype html>\n<html></html>";
        let result = apply_mutation(input, "not-a-real-kind", &Json::Object(vec![]));
        assert!(result.is_err(), "an unrecognised kind must fail loudly");
    }

    #[test]
    fn projection_ignores_attribute_order_but_not_children_order() {
        let a = project(b"<!doctype html>\n<html a=\"1\" b=\"2\"></html>").unwrap();
        let b = project(b"<!doctype html>\n<html b=\"2\" a=\"1\"></html>").unwrap();
        assert_eq!(a, b, "attribute order must not affect the projection — real writer freedom");

        let c = project(b"<!doctype html>\n<html><body><p>1</p><p>2</p></body></html>").unwrap();
        let d = project(b"<!doctype html>\n<html><body><p>2</p><p>1</p></body></html>").unwrap();
        assert_ne!(c, d, "sibling order IS normative and must never be sorted away");
    }
    //#endregion 🔖️SmallFixtureUnitLaws

    //#region 🔖️RealFixtureExhaustiveSweep
    /// 🎯️ Runs every kind/params pair this case's `component.feature` Examples table declares
    /// against the REAL fixture (mutate, then its own computed inverse) and asserts the law the
    /// wave brief names directly: `apply(inverse(m, base), apply(m, base)) == base`'s PROJECTION.
    /// Keeping this list in lock-step with the feature file is what makes this test worth
    /// anything — a params typo here is caught by `cargo test`, not just by the exhaustive runner.
    #[test]
    fn real_fixture_every_declared_kind_mutates_and_inverts_cleanly() {
        let base_projection = project(REAL_FIXTURE).unwrap();
        let cases: Vec<(&str, Json)> = vec![
            ("no-mutation", obj(vec![])),
            (
                "set-snapshot",
                obj(vec![
                    ("doctype", Json::String("DOCTYPE html".into())),
                    (
                        "root",
                        obj(vec![
                            ("kind", Json::String("element".into())),
                            ("name", Json::String("html".into())),
                            ("attributes", Json::Array(vec![obj(vec![("name", Json::String("lang".into())), ("value", Json::String("de".into()))])])),
                            (
                                "children",
                                Json::Array(vec![
                                    obj(vec![
                                        ("kind", Json::String("element".into())),
                                        ("name", Json::String("head".into())),
                                        ("attributes", Json::Array(vec![])),
                                        (
                                            "children",
                                            Json::Array(vec![obj(vec![
                                                ("kind", Json::String("element".into())),
                                                ("name", Json::String("title".into())),
                                                ("attributes", Json::Array(vec![])),
                                                ("children", Json::Array(vec![obj(vec![("kind", Json::String("text".into())), ("text", Json::String("Wave 7 Snapshot Title".into()))])])),
                                            ])]),
                                        ),
                                    ]),
                                    obj(vec![
                                        ("kind", Json::String("element".into())),
                                        ("name", Json::String("body".into())),
                                        ("attributes", Json::Array(vec![])),
                                        ("children", Json::Array(vec![obj(vec![("kind", Json::String("text".into())), ("text", Json::String("Wave 7 snapshot replacement content".into()))])])),
                                    ]),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
            ("set-doctype", obj(vec![("doctype", Json::String("DOCTYPE htmlWave7".into()))])),
            (
                "insert-node",
                obj(vec![
                    ("parent", Json::Array(vec![Json::Number(2.0)])),
                    ("index", Json::Number(0.0)),
                    (
                        "node",
                        obj(vec![
                            ("kind", Json::String("element".into())),
                            ("name", Json::String("div".into())),
                            ("attributes", Json::Array(vec![obj(vec![("name", Json::String("id".into())), ("value", Json::String("wave7-marker".into()))])])),
                            ("children", Json::Array(vec![obj(vec![("kind", Json::String("text".into())), ("text", Json::String("Wave 7 mutation testing".into()))])])),
                        ]),
                    ),
                ]),
            ),
            ("remove-node", obj(vec![("parent", Json::Array(vec![Json::Number(2.0)])), ("index", Json::Number(9.0))])),
            ("set-element-name", obj(vec![("path", Json::Array(vec![Json::Number(2.0), Json::Number(9.0)])), ("name", Json::String("aside".into()))])),
            ("set-attribute", obj(vec![("path", Json::Array(vec![Json::Number(2.0), Json::Number(9.0)])), ("name", Json::String("class".into())), ("value", Json::String("sidebars-wave7".into()))])),
            ("set-text", obj(vec![("path", Json::Array(vec![Json::Number(0.0), Json::Number(9.0), Json::Number(0.0)])), ("text", Json::String("Wave 7 Mutation Testing".into()))])),
            ("set-comment", obj(vec![("path", Json::Array(vec![Json::Number(0.0), Json::Number(5.0)])), ("text", Json::String(" Wave 7 replaced comment ".into()))])),
            ("set-raw-text", obj(vec![("path", Json::Array(vec![Json::Number(2.0), Json::Number(29.0), Json::Number(0.0)])), ("text", Json::String("console.log('wave7');".into()))])),
        ];
        assert_eq!(cases.len(), super::super::KINDS.len(), "this sweep must cover every declared kind exactly once");
        for (kind, params) in &cases {
            let mutated = apply_mutation(REAL_FIXTURE, kind, params).unwrap_or_else(|error| panic!("mutate {kind:?} failed: {error}"));
            let mutated_projection = project(&mutated).unwrap();
            if *kind != "no-mutation" {
                assert_ne!(&mutated_projection, &base_projection, "mutate {kind:?} produced no visible change in the real document");
            }
            let restored = apply_mutation_inverse(REAL_FIXTURE, kind, params).unwrap_or_else(|error| panic!("inverse {kind:?} failed: {error}"));
            let restored_projection = project(&restored).unwrap();
            assert_eq!(restored_projection, base_projection, "inverse {kind:?} did not restore the real document's projection");
        }
    }
    //#endregion 🔖️RealFixtureExhaustiveSweep
}
