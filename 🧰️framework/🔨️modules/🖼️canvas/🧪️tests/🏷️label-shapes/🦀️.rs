mod label_shapes_laws {
    //! 🗃️ Laws over the shaped-label cache fixture (`🧫️fixtures/🏷️label-shapes/🔣️.json`): its least-recently-used
    //! policy, and that a label painted or measured through the cache is exactly what usvg (the third-party shaper)
    //! produces for the same markup directly — shaped once, reused by every later paint and measure.

    use super::super::draw_list::{scene_draw_list_json, DrawListOptions};
    use super::*;
    use serde_json::Value;

    const FIXTURE: &str = include_str!("../../🧫️fixtures/🏷️label-shapes/🔣️.json");

    fn fixture() -> Value {
        serde_json::from_str(FIXTURE).expect("label-shapes fixture")
    }

    fn color(value: &Value) -> Color {
        let parts: Vec<u8> = value.as_array().expect("rgba").iter().map(|part| part.as_u64().expect("channel") as u8).collect();
        Color::from_rgba8(parts[0], parts[1], parts[2], parts[3])
    }

    fn bounds(value: &Value) -> LabelShapeBounds {
        let bound = |name: &str| value[name].as_u64().expect("bound") as usize;
        LabelShapeBounds { maximum_entries: bound("maximumEntries"), maximum_bytes: bound("maximumBytes"), maximum_entry_bytes: bound("maximumEntryBytes") }
    }

    fn origin(document: &Value) -> Point {
        let parts = document["origin"].as_array().expect("origin");
        Point::new(parts[0].as_f64().expect("x"), parts[1].as_f64().expect("y"))
    }

    fn draw_list(scene: &Scene) -> String {
        scene_draw_list_json(scene, DrawListOptions::default())
    }

    /// 🔮️ The oracle: `markup` shaped by usvg directly, placed exactly as the canvas places a label.
    fn usvg_direct(markup: &str, origin: Point, px: f64) -> Scene {
        let tree = usvg::Tree::from_str(markup, usvg_options_map_labels()).expect("usvg shapes the fixture markup");
        let (bx, by, bw, bh) = super::super::svg_icon::svg_icon_content_bounds(&tree);
        let mut scene = Scene::new();
        if bw <= 0.0 || bh <= 0.0 {
            return scene;
        }
        let mut label_scene = Scene::new();
        render_svg_tree_literal(&mut label_scene, &tree);
        let scale = (px * ui_styling::metrics::label::SCALE_RATIO / bh).min(ui_styling::metrics::label::SCALE_MAX);
        let aff = Affine::IDENTITY.translate(Vec2::new(origin.x() - bx * scale, origin.y() - by * scale - px * ui_styling::metrics::label::VERTICAL_OFFSET_RATIO)).scale(scale);
        scene.append(&label_scene, Some(aff));
        scene
    }

    /// 🔮️ The oracle of `label_byte_world_x`: the line box and the prefix box measured by usvg directly.
    fn usvg_direct_byte_x(line: &str, byte: usize, origin_x: f64, px: f64) -> f64 {
        let measure = |text: &str| {
            let tree = usvg::Tree::from_str(&label_measure_markup(text, px), usvg_options_map_labels()).expect("usvg measures the fixture markup");
            super::super::svg_icon::svg_icon_content_bounds(&tree)
        };
        let pad = px * ui_styling::metrics::label::PAD_RATIO;
        let (bx, _, _, bh) = measure(if line.is_empty() { " " } else { line });
        let scale = (px * ui_styling::metrics::label::SCALE_RATIO / bh).min(ui_styling::metrics::label::SCALE_MAX);
        let advance = if byte == 0 {
            0.0
        } else {
            let (pbx, _, pbw, _) = measure(&line[..byte]);
            (pbx + pbw) - pad
        };
        origin_x + (pad + advance - bx) * scale
    }

    #[test]
    fn the_cache_bounds_are_the_schema_consts() {
        let schema: Value = serde_json::from_str(LABEL_SHAPES_SCHEMA).expect("label-shapes schema");
        let declared = LabelShapeBounds::declared();
        assert_eq!(declared.maximum_entries as u64, schema["properties"]["maximumEntries"]["const"].as_u64().expect("maximumEntries const"));
        assert_eq!(declared.maximum_bytes as u64, schema["properties"]["maximumBytes"]["const"].as_u64().expect("maximumBytes const"));
        assert_eq!(declared.maximum_entry_bytes as u64, schema["properties"]["maximumEntryBytes"]["const"].as_u64().expect("maximumEntryBytes const"));
        assert_eq!(fixture()["schema"], schema["$id"]);
        assert!(LabelShapeBounds::from_schema(r#"{"properties":{"maximumEntries":{"const":0},"maximumBytes":{"const":1},"maximumEntryBytes":{"const":1}}}"#).is_err());
        assert!(LabelShapeBounds::from_schema(r#"{"properties":{"maximumEntries":{"const":1},"maximumBytes":{"const":1},"maximumEntryBytes":{"const":2}}}"#).is_err());
        assert!(LabelShapeBounds::from_schema(r#"{"properties":{"maximumEntries":{"const":1}}}"#).is_err());
    }

    #[test]
    fn every_lru_case_admits_and_evicts_as_the_fixture_names() {
        let document = fixture();
        let cases = document["lru"].as_array().expect("lru cases");
        assert!(cases.len() >= 3);
        for case in cases {
            let id = case["id"].as_str().expect("case id");
            let mut lru = BoundedLru::new(bounds(&case["bounds"]));
            for (index, step) in case["steps"].as_array().expect("steps").iter().enumerate() {
                let key = step["key"].as_str().expect("key");
                match step["do"].as_str().expect("do") {
                    "get" => {
                        let answer = if lru.get(key).is_some() { "hit" } else { "miss" };
                        assert_eq!(answer, step["expect"].as_str().expect("get expectation"), "{id} step {index}");
                    }
                    "admit" => {
                        let answer = lru.admit(key.to_owned(), (), step["bytes"].as_u64().expect("bytes") as usize);
                        let expected = if step["expect"] == "bypassed" {
                            LruAdmission::Bypassed
                        } else {
                            LruAdmission::Stored { evicted: step["expect"]["stored"].as_array().expect("stored").iter().map(|key| key.as_str().expect("evicted key").to_owned()).collect() }
                        };
                        assert_eq!(answer, expected, "{id} step {index}");
                    }
                    other => panic!("{id}: unknown step {other}"),
                }
            }
            let keys: Vec<String> = case["keys"].as_array().expect("keys").iter().map(|key| key.as_str().expect("key").to_owned()).collect();
            assert_eq!(lru.keys(), keys, "{id} resident keys");
            assert_eq!(lru.bytes() as u64, case["bytes"].as_u64().expect("bytes"), "{id} resident bytes");
        }
    }

    #[test]
    fn every_label_paints_what_usvg_shapes_and_shapes_it_once() {
        let document = fixture();
        let at = origin(&document);
        for row in document["labels"].as_array().expect("labels") {
            let id = row["id"].as_str().expect("id");
            let text = row["text"].as_str().expect("text");
            let px = row["px"].as_f64().expect("px");
            let (fill, halo) = (color(&row["fill"]), color(&row["halo"]));
            let oracle = draw_list(&usvg_direct(&label_paint_markup(text, px, fill, halo), at, px));
            let before = label_shape_stats();
            let mut first = Scene::new();
            append_label(&mut first, text, at, px, fill, halo);
            let shaped = label_shape_stats();
            let mut second = Scene::new();
            append_label(&mut second, text, at, px, fill, halo);
            let reused = label_shape_stats();
            assert!(!first.is_empty(), "{id}: painted nothing");
            assert_eq!(draw_list(&first), oracle, "{id}: first paint differs from usvg");
            assert_eq!(draw_list(&second), oracle, "{id}: cached paint differs from usvg");
            assert_eq!(shaped.shapes, before.shapes + 1, "{id}: the first paint shapes once");
            assert_eq!(reused.shapes, shaped.shapes, "{id}: the second paint reshapes");
            assert_eq!(reused.hits, shaped.hits + 1, "{id}: the second paint is a hit");
            for byte in row["measure"].as_array().expect("measure").iter().map(|byte| byte.as_u64().expect("byte") as usize) {
                let expected = usvg_direct_byte_x(text, byte, 0.0, px);
                let measured = label_byte_world_x(text, byte, 0.0, px);
                let settled = label_shape_stats();
                assert_eq!(measured, expected, "{id}: byte {byte} measured differs from usvg");
                assert_eq!(label_byte_world_x(text, byte, 0.0, px), expected, "{id}: cached byte {byte} differs from usvg");
                assert_eq!(label_shape_stats().shapes, settled.shapes, "{id}: re-measuring byte {byte} reshapes");
            }
        }
    }

    #[test]
    fn every_tspan_line_paints_what_usvg_shapes_and_shapes_it_once() {
        let document = fixture();
        let at = origin(&document);
        for row in document["tspans"].as_array().expect("tspans") {
            let id = row["id"].as_str().expect("id");
            let text = row["text"].as_str().expect("text");
            let px = row["px"].as_f64().expect("px");
            let halo = color(&row["halo"]);
            let spans: Vec<(usize, usize, Color)> = row["spans"].as_array().expect("spans").iter().map(|span| (span["start"].as_u64().expect("start") as usize, span["end"].as_u64().expect("end") as usize, color(&span["fill"]))).collect();
            let oracle = draw_list(&usvg_direct(&label_tspans_markup(text, &spans, px).expect("tspan markup"), at, px));
            let before = label_shape_stats();
            let mut first = Scene::new();
            append_label_tspans(&mut first, text, &spans, at, px, halo);
            let mut second = Scene::new();
            append_label_tspans(&mut second, text, &spans, at, px, halo);
            let after = label_shape_stats();
            assert!(!first.is_empty(), "{id}: painted nothing");
            assert_eq!(draw_list(&first), oracle, "{id}: first paint differs from usvg");
            assert_eq!(draw_list(&second), oracle, "{id}: cached paint differs from usvg");
            assert_eq!(after.shapes, before.shapes + 1, "{id}: two paints shape once");
        }
    }

    #[test]
    fn a_working_set_is_accounted_by_its_outline_bytes_within_the_declared_bounds() {
        let bounds = LabelShapeBounds::declared();
        let before = label_shape_stats();
        let mut scene = Scene::new();
        for index in 0..64 {
            append_label(&mut scene, &format!("label {index:04} of a working set"), Point::new(0.0, 0.0), 12.0, Color::from_rgba8(0, 0, 0, 255), Color::from_rgba8(255, 255, 255, 255));
        }
        let after = label_shape_stats();
        assert_eq!(after.shapes, before.shapes + 64);
        assert!(after.entries <= bounds.maximum_entries && after.bytes <= bounds.maximum_bytes, "{after:?} exceeds {bounds:?}");
        assert!(after.bytes >= 64 * 1024, "64 shaped labels hold their outlines, not just their keys: {after:?}");
    }
}
