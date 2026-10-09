use crate::standards::v1::subsets::any::schema::default_drawing_document;
use crate::standards::v1::subsets::any::schema::create_drawing_path_layer;
use super::*;
use crate::DrawingCircle;

#[semio_framework_async_macros::async_test]
async fn default_document_has_path_layer() {
    let doc = default_drawing_document("test", None);
    assert_eq!(doc.layers.len(), 1);
    assert!(matches!(doc.layers[0], DrawingLayerNode::Path(_)));
}

#[semio_framework_async_macros::async_test]
async fn scene_nodes_include_shape_bounds() {
    let layer = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Rect")).to_string().into()).expect("nonempty authored identity"), "Rect");
    let doc = DrawingSnapshot { layers: vec![layer].into(), ..default_drawing_document("scene", None) };
    let nodes = flatten_drawing_document_to_scene_nodes(&doc);
    assert_eq!(nodes.len(), 1);
    assert!(!nodes[0].segments.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn resolve_boolean_layer_segments_unions_two_rects() {
    let mut doc = default_drawing_document("bool-test", None);
    doc.layers.clear();
    let mut rect_a = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("A")).to_string().into()).expect("nonempty authored identity"), "A");
    if let DrawingLayerNode::Shape(shape) = &mut rect_a {
        shape.rect = Some(DrawingRect { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
    }
    let id_a = layer_id(&rect_a).clone();
    let mut rect_b = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("B")).to_string().into()).expect("nonempty authored identity"), "B");
    if let DrawingLayerNode::Shape(shape) = &mut rect_b {
        shape.rect = Some(DrawingRect { x: 5.0, y: 5.0, width: 10.0, height: 10.0 });
    }
    let id_b = layer_id(&rect_b).clone();
    doc.layers.push(rect_a);
    doc.layers.push(rect_b);
    let boolean = create_drawing_boolean_layer(crate::schema::identity::DrawingIdentity::admit((("Union")).to_string().into()).expect("nonempty authored identity"), "Union", "union", [id_a, id_b].into_iter().collect());
    let boolean_id = layer_id(&boolean).to_string();
    doc.layers.push(boolean);
    let nodes = flatten_drawing_document_to_scene_nodes(&doc);
    let boolean_node = nodes.iter().find(|node| node.id == boolean_id).expect("boolean scene node");
    assert!(!boolean_node.segments.is_empty());
    assert_eq!(boolean_node.fill_rule.as_deref(), Some("evenodd"));
}

#[semio_framework_async_macros::async_test]
async fn resolve_boolean_layer_segments_flattens_arcs_before_boolean_operation() {
    let mut doc = default_drawing_document("bool-arc-test", None);
    doc.layers.clear();
    let path_a =
        create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("A")).to_string().into()).expect("nonempty authored identity"), "A", vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [10.0, 0.0] }, PathSegment::Arc { rx: 10.0, ry: 10.0, rotation: 0.0, large_arc: false, sweep: true, to: [0.0, 10.0] }, PathSegment::Close].into());
    let id_a = layer_id(&path_a).clone();
    let rect_b = {
        let mut layer = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("B")).to_string().into()).expect("nonempty authored identity"), "B");
        if let DrawingLayerNode::Shape(shape) = &mut layer {
            shape.rect = Some(DrawingRect { x: 2.0, y: 2.0, width: 4.0, height: 4.0 });
        }
        layer
    };
    let id_b = layer_id(&rect_b).clone();
    doc.layers.push(path_a);
    doc.layers.push(rect_b);
    let boolean = create_drawing_boolean_layer(crate::schema::identity::DrawingIdentity::admit((("Union")).to_string().into()).expect("nonempty authored identity"), "Union", "union", [id_a, id_b].into_iter().collect());
    let boolean_id = layer_id(&boolean).to_string();
    doc.layers.push(boolean);
    let nodes = flatten_drawing_document_to_scene_nodes(&doc);
    let boolean_node = nodes.iter().find(|node| node.id == boolean_id).expect("boolean scene node");
    assert!(!boolean_node.segments.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn arc_segment_flattens_to_cubics_preserving_endpoints() {
    let segments = vec![PathSegment::Move { to: [10.0, 0.0] }, PathSegment::Arc { rx: 10.0, ry: 10.0, rotation: 0.0, large_arc: false, sweep: true, to: [0.0, 10.0] }];
    let flattened = flatten_curve_segments(&segments);
    assert!(flattened.iter().all(|segment| !matches!(segment, PathSegment::Arc { .. })));
    match flattened.last() {
        Some(PathSegment::Cubic { to, .. }) => {
            assert!((to[0] - 0.0).abs() < 1e-6);
            assert!((to[1] - 10.0).abs() < 1e-6);
        }
        other => panic!("expected trailing cubic segment, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn resolve_trace_layer_segments_traces_solid_square_png() {
    let mut image_buffer = semio_framework_pixels::RasterImage::new(8, 8);
    for y in 2..6u32 {
        for x in 2..6u32 {
            let idx = ((y * image_buffer.width + x) * 4) as usize;
            image_buffer.pixels[idx..idx + 4].copy_from_slice(&[255, 255, 255, 255]);
        }
    }
    let bytes = semio_framework_pixels::encode_png(&image_buffer).expect("encode png");
    let mut doc = default_drawing_document("trace-test", None);
    doc.layers.clear();
    let mut assets = semio_framework_value::paged::PagedMap::default();
    assets.insert("source", crate::standards::v1::subsets::any::io::image::drawing_image_from_png(&bytes));
    doc.assets = assets;
    doc.artboard = Some(DrawingArtboard { width: 16.0, height: 16.0 });
    doc.layers.push(create_drawing_trace_layer(crate::schema::identity::DrawingIdentity::admit((("Trace")).to_string().into()).expect("nonempty authored identity"), "Trace", "source"));
    let nodes = flatten_drawing_document_to_scene_nodes(&doc);
    assert_eq!(nodes.len(), 1);
    assert!(!nodes[0].segments.is_empty());
    assert_eq!(nodes[0].fill_rule.as_deref(), Some("evenodd"));
}

#[semio_framework_async_macros::async_test]
async fn resolve_drawing_artboard_skips_group_boolean_trace_kinds() {
    let mut doc = default_drawing_document("artboard-test", None);
    doc.artboard = None;
    doc.layers.clear();
    let mut rect = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("R")).to_string().into()).expect("nonempty authored identity"), "R");
    if let DrawingLayerNode::Shape(shape) = &mut rect {
        shape.rect = Some(DrawingRect { x: 0.0, y: 0.0, width: 20.0, height: 30.0 });
    }
    doc.layers.push(rect);
    doc.layers.push(create_drawing_trace_layer(crate::schema::identity::DrawingIdentity::admit((("Trace")).to_string().into()).expect("nonempty authored identity"), "Trace", "missing-source"));
    let artboard = resolve_drawing_artboard(&doc).expect("artboard bounds");
    assert_eq!(artboard.width, 20.0);
    assert_eq!(artboard.height, 30.0);
}

#[semio_framework_async_macros::async_test]
async fn default_drawing_document_has_artboard_dimensions() {
    let doc = default_drawing_document("blank", None);
    let artboard = doc.artboard.expect("default artboard");
    assert_eq!(artboard.width, 1024.0);
    assert_eq!(artboard.height, 1024.0);
}

#[semio_framework_async_macros::async_test]
async fn layer_id_base_and_kind_label_cover_all_seven_variants() {
    let shape = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Shape")).to_string().into()).expect("nonempty authored identity"), "Shape");
    let path = create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("Path")).to_string().into()).expect("nonempty authored identity"), "Path", Vec::new().into());
    let text = create_drawing_text_layer(crate::schema::identity::DrawingIdentity::admit((("Text")).to_string().into()).expect("nonempty authored identity"), "Text");
    let image = create_drawing_image_layer(crate::schema::identity::DrawingIdentity::admit((("Image")).to_string().into()).expect("nonempty authored identity"), "Image", "key");
    let group = create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
    let boolean = create_drawing_boolean_layer(crate::schema::identity::DrawingIdentity::admit((("Boolean")).to_string().into()).expect("nonempty authored identity"), "Boolean", "union", Vec::new().into());
    let trace = create_drawing_trace_layer(crate::schema::identity::DrawingIdentity::admit((("Trace")).to_string().into()).expect("nonempty authored identity"), "Trace", "src");
    for (layer, expected_kind) in [(&shape, "shape:rect"), (&path, "path"), (&text, "text"), (&image, "image"), (&group, "group"), (&boolean, "boolean"), (&trace, "trace")] {
        assert_eq!(layer_kind_label(layer), expected_kind);
        assert_eq!(layer_id(layer), &layer_base(layer).id);
    }
}

#[semio_framework_async_macros::async_test]
async fn find_drawing_layer_locates_nested_child_and_returns_none_for_missing() {
    let child = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Child")).to_string().into()).expect("nonempty authored identity"), "Child");
    let child_id = layer_id(&child).to_string();
    let mut group = create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
    if let DrawingLayerNode::Group(body) = &mut group {
        body.children.push(child);
    }
    let mut doc = default_drawing_document("nested", None);
    doc.layers = vec![group].into();
    assert!(find_drawing_layer(&doc, &child_id).is_some());
    assert!(find_drawing_layer(&doc, "missing-id").is_none());
}

#[semio_framework_async_macros::async_test]
async fn flatten_drawing_layers_includes_nested_group_children() {
    let child_a = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("A")).to_string().into()).expect("nonempty authored identity"), "A");
    let child_b = create_drawing_text_layer(crate::schema::identity::DrawingIdentity::admit((("B")).to_string().into()).expect("nonempty authored identity"), "B");
    let mut group = create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
    if let DrawingLayerNode::Group(body) = &mut group {
        body.children.push(child_a);
        body.children.push(child_b);
    }
    let layers = [group].into_iter().collect();
    let flat = flatten_drawing_layers(&layers);
    assert_eq!(flat.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn drawing_matrix_to_transform_round_trips_and_handles_zero_scale_x() {
    let transform = DrawingTransform { x: 1.0, y: 2.0, scale_x: 2.0, scale_y: 3.0, rotation: std::f64::consts::FRAC_PI_6, shear: 0.0 };
    let matrix = drawing_transform_to_matrix(&transform);
    let back = drawing_matrix_to_transform(matrix);
    assert!((back.x - transform.x).abs() < 1e-9);
    assert!((back.y - transform.y).abs() < 1e-9);
    assert!((back.scale_x - transform.scale_x).abs() < 1e-9);
    assert!((back.scale_y - transform.scale_y).abs() < 1e-9);
    assert!((back.rotation - transform.rotation).abs() < 1e-9);

    let degenerate = drawing_matrix_to_transform([0.0, 0.0, 5.0, 5.0, 1.0, 2.0]);
    assert_eq!(degenerate.scale_x, 0.0);
    assert!((degenerate.scale_y - 5.0_f64.hypot(5.0)).abs() < 1e-12);
    assert_eq!(degenerate.shear, 0.0);
}

#[semio_framework_async_macros::async_test]
async fn drawing_play_layers_tree_row_id_formats_and_parses_back() {
    let shape = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Shape")).to_string().into()).expect("nonempty authored identity"), "Shape");
    let id = layer_id(&shape).to_string();
    let row_id = drawing_play_layers_tree_row_id(&shape);
    assert_eq!(row_id, format!("drawing-play-layers.shape.{id}"));
    assert_eq!(drawing_play_layer_id_from_tree_row_id(&row_id), Some(id));

    let child_row = drawing_play_boolean_child_row_id("bool-1", "child-1");
    assert_eq!(child_row, "drawing-play-layers.boolean-child.6.bool-1child-1");
    assert_eq!(drawing_play_layer_id_from_tree_row_id(&child_row), Some("child-1".to_string()));
    for (parent, child) in [("a.child.b", "c.child.d"), ("🌳.child", "葉.1")] {
        assert_eq!(drawing_play_layer_id_from_tree_row_id(&drawing_play_boolean_child_row_id(parent, child)), Some(child.into()));
    }

    assert_eq!(drawing_play_layer_id_from_tree_row_id("not-a-row-id"), None);
    assert_eq!(drawing_play_layer_id_from_tree_row_id("drawing-play-layers."), None);
}

#[semio_framework_async_macros::async_test]
async fn layer_to_path_segments_covers_every_shape_kind_and_empty_polygon_and_unknown_kind() {
    let rect = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Rect")).to_string().into()).expect("nonempty authored identity"), "Rect");
    assert!(!layer_to_path_segments(&rect).is_empty());

    let line = DrawingLayerNode::Shape(DrawingShapeBody { base: default_layer_base(crate::schema::identity::DrawingIdentity::admit((("Line")).to_string().into()).expect("nonempty authored identity"), "Line"), shape_kind: "line".into(), rect: None, ellipse: None, circle: None, line: Some(DrawingLine { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0 }), polygon: None });
    assert_eq!(layer_to_path_segments(&line).len(), 2);

    let empty_polygon = DrawingLayerNode::Shape(DrawingShapeBody { base: default_layer_base(crate::schema::identity::DrawingIdentity::admit((("Poly")).to_string().into()).expect("nonempty authored identity"), "Poly"), shape_kind: "polygon".into(), rect: None, ellipse: None, circle: None, line: None, polygon: Some(DrawingPolygon { points: Vec::new().into() }) });
    assert!(layer_to_path_segments(&empty_polygon).is_empty());

    let polygon = DrawingLayerNode::Shape(DrawingShapeBody {
        base: default_layer_base(crate::schema::identity::DrawingIdentity::admit((("Poly")).to_string().into()).expect("nonempty authored identity"), "Poly"),
        shape_kind: "polygon".into(),
        rect: None,
        ellipse: None,
        circle: None,
        line: None,
        polygon: Some(DrawingPolygon { points: vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]].into() }),
    });
    assert_eq!(layer_to_path_segments(&polygon).len(), 4);

    let ellipse =
        DrawingLayerNode::Shape(DrawingShapeBody { base: default_layer_base(crate::schema::identity::DrawingIdentity::admit((("Ellipse")).to_string().into()).expect("nonempty authored identity"), "Ellipse"), shape_kind: "ellipse".into(), rect: None, ellipse: Some(DrawingEllipse { cx: 0.0, cy: 0.0, rx: 1.0, ry: 1.0 }), circle: None, line: None, polygon: None });
    assert_eq!(layer_to_path_segments(&ellipse).len(), 6);

    let circle = DrawingLayerNode::Shape(DrawingShapeBody { base: default_layer_base(crate::schema::identity::DrawingIdentity::admit((("Circle")).to_string().into()).expect("nonempty authored identity"), "Circle"), shape_kind: "circle".into(), rect: None, ellipse: None, circle: Some(DrawingCircle { cx: 0.0, cy: 0.0, r: 1.0 }), line: None, polygon: None });
    assert_eq!(layer_to_path_segments(&circle).len(), 6);

    let unknown_kind = DrawingLayerNode::Shape(DrawingShapeBody { base: default_layer_base(crate::schema::identity::DrawingIdentity::admit((("Unknown")).to_string().into()).expect("nonempty authored identity"), "Unknown"), shape_kind: "star".into(), rect: None, ellipse: None, circle: None, line: None, polygon: None });
    assert!(layer_to_path_segments(&unknown_kind).is_empty());

    let rect_missing_data = DrawingLayerNode::Shape(DrawingShapeBody { base: default_layer_base(crate::schema::identity::DrawingIdentity::admit((("RectNoData")).to_string().into()).expect("nonempty authored identity"), "RectNoData"), shape_kind: "rect".into(), rect: None, ellipse: None, circle: None, line: None, polygon: None });
    assert!(layer_to_path_segments(&rect_missing_data).is_empty());

    let group = create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
    assert!(layer_to_path_segments(&group).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn drawing_layer_world_bounds_covers_text_image_default_and_none_branches() {
    let text = DrawingLayerNode::Text(DrawingTextBody { base: default_layer_base(crate::schema::identity::DrawingIdentity::admit((("T")).to_string().into()).expect("nonempty authored identity"), "T"), x: 0.0, y: 0.0, content: "hi".into(), size: 10.0 });
    let (tx, ty, tw, th) = drawing_layer_world_bounds(&text).expect("text bounds");
    assert_eq!((tx, ty), (0.0, 0.0));
    assert!(tw > 0.0 && th > 0.0);

    let image = create_drawing_image_layer(crate::schema::identity::DrawingIdentity::admit((("Img")).to_string().into()).expect("nonempty authored identity"), "Img", "key");
    let (_, _, iw, ih) = drawing_layer_world_bounds(&image).expect("image bounds");
    assert_eq!((iw, ih), (256.0, 256.0));

    let empty_path = create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("Empty")).to_string().into()).expect("nonempty authored identity"), "Empty", Vec::new().into());
    assert!(drawing_layer_world_bounds(&empty_path).is_none());

    let close_only = create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("CloseOnly")).to_string().into()).expect("nonempty authored identity"), "CloseOnly", vec![PathSegment::Close].into());
    assert!(drawing_layer_world_bounds(&close_only).is_none());
}

#[semio_framework_async_macros::async_test]
async fn canvas_layer_records_excludes_groups_and_includes_bounds() {
    let child = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Child")).to_string().into()).expect("nonempty authored identity"), "Child");
    let mut group = create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
    if let DrawingLayerNode::Group(body) = &mut group {
        body.children.push(child);
    }
    let mut doc = default_drawing_document("records", None);
    doc.layers = vec![group].into();
    let records = canvas_layer_records(&doc);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].kind, "shape:rect");
    assert!(records[0].width.is_some());
}

#[semio_framework_async_macros::async_test]
async fn clone_drawing_layer_node_assigns_new_ids_recursively_and_appends_suffix_only_at_top() {
    let shape = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Rect")).to_string().into()).expect("nonempty authored identity"), "Rect");
    let clone = clone_drawing_layer_node(&shape," copy",&vec![crate::schema::identity::DrawingIdentityAssignment{source:"Rect".into(),target:"rect-copy".into()}].into()).unwrap();
    assert_ne!(layer_id(&shape), layer_id(&clone));
    assert_eq!(layer_base(&clone).name, "Rect copy");

    let child = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Child")).to_string().into()).expect("nonempty authored identity"), "Child");
    let child_id = layer_id(&child).to_string();
    let mut group = create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
    if let DrawingLayerNode::Group(body) = &mut group {
        body.children.push(child);
    }
    let group_clone = clone_drawing_layer_node(&group," copy",&clone_assignments(&group)).unwrap();
    let DrawingLayerNode::Group(cloned_body) = &group_clone else { panic!("expected group") };
    assert_eq!(cloned_body.base.name, "Group copy");
    assert!(!layer_id(&cloned_body.children[0]).eq_str(&child_id));
    assert_eq!(layer_base(&cloned_body.children[0]).name, "Child");
}

#[semio_framework_async_macros::async_test]
async fn transform_path_segments_transforms_every_segment_kind() {
    let segments = vec![
        PathSegment::Move { to: [1.0, 0.0] },
        PathSegment::Line { to: [1.0, 0.0] },
        PathSegment::Quad { ctrl: [1.0, 0.0], to: [1.0, 0.0] },
        PathSegment::Cubic { ctrl1: [1.0, 0.0], ctrl2: [1.0, 0.0], to: [1.0, 0.0] },
        PathSegment::Arc { rx: 1.0, ry: 1.0, rotation: 0.0, large_arc: false, sweep: true, to: [1.0, 0.0] },
        PathSegment::Close,
    ];
    let transform = DrawingTransform { x: 10.0, y: 20.0, scale_x: 2.0, scale_y: 2.0, rotation: 0.0, shear: 0.0 };
    let transformed = transform_path_segments(&segments, &transform);
    match &transformed[0] {
        PathSegment::Move { to } => assert_eq!(*to, [12.0, 20.0]),
        other => panic!("expected move, got {other:?}"),
    }
    assert_eq!(transformed[4], PathSegment::Line { to: [12.0, 20.0] });
    assert!(matches!(transformed[5], PathSegment::Close));
}

#[semio_framework_async_macros::async_test]
async fn scale_path_segments_returns_untouched_clone_for_identity_scale_and_scales_otherwise() {
    let segments = vec![PathSegment::Move { to: [1.0, 2.0] }, PathSegment::Line { to: [3.0, 4.0] }];
    assert_eq!(scale_path_segments(&segments, 1.0, 1.0), segments);
    let scaled = scale_path_segments(&segments, 2.0, 3.0);
    match &scaled[1] {
        PathSegment::Line { to } => assert_eq!(*to, [6.0, 12.0]),
        other => panic!("expected line, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn split_path_segments_by_contour_splits_on_move_and_handles_empty_input() {
    let segments = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [1.0, 0.0] }, PathSegment::Move { to: [5.0, 5.0] }, PathSegment::Line { to: [6.0, 5.0] }, PathSegment::Close];
    let contours = split_path_segments_by_contour(&segments);
    assert_eq!(contours.len(), 2);
    assert_eq!(contours[1].len(), 3);

    let empty_contours = split_path_segments_by_contour(&[]);
    assert_eq!(empty_contours, vec![Vec::<PathSegment>::new()]);
}

#[semio_framework_async_macros::async_test]
async fn path_segments_bounds_is_none_when_no_segment_carries_an_endpoint() {
    assert!(path_segments_bounds(&[PathSegment::Close]).is_none());
    let bounds = path_segments_bounds(&[PathSegment::Move { to: [1.0, 1.0] }, PathSegment::Line { to: [4.0, 5.0] }]).expect("bounds");
    assert_eq!(bounds, (1.0, 1.0, 3.0, 4.0));
}

#[semio_framework_async_macros::async_test]
async fn filter_path_segments_by_contour_area_keeps_all_for_non_positive_min_area_and_drops_small_contours() {
    let small = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [1.0, 0.0] }, PathSegment::Line { to: [1.0, 1.0] }, PathSegment::Close];
    let big = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [10.0, 0.0] }, PathSegment::Line { to: [10.0, 10.0] }, PathSegment::Close];
    let mut combined = small;
    combined.extend(big.clone());

    assert_eq!(filter_path_segments_by_contour_area(&combined, 0.0), combined);

    let filtered = filter_path_segments_by_contour_area(&combined, 4.0);
    assert_eq!(filtered, big);
}

#[semio_framework_async_macros::async_test]
async fn flatten_curve_segments_falls_back_to_line_for_degenerate_arc_and_passes_other_kinds_through() {
    let segments = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Arc { rx: 0.0, ry: 0.0, rotation: 0.0, large_arc: false, sweep: true, to: [5.0, 5.0] }, PathSegment::Quad { ctrl: [1.0, 1.0], to: [2.0, 2.0] }, PathSegment::Close];
    let flattened = flatten_curve_segments(&segments);
    assert!(matches!(flattened[1], PathSegment::Line { to } if to == [5.0, 5.0]));
    assert!(matches!(flattened[2], PathSegment::Quad { .. }));
    assert!(matches!(flattened[3], PathSegment::Close));
}

#[semio_framework_async_macros::async_test]
async fn flatten_segments_to_lines_samples_quad_and_cubic_into_lines() {
    let segments = vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Quad { ctrl: [1.0, 1.0], to: [2.0, 0.0] }, PathSegment::Cubic { ctrl1: [2.0, 1.0], ctrl2: [3.0, 1.0], to: [4.0, 0.0] }];
    let flattened = flatten_segments_to_lines(&segments);
    assert!(flattened.iter().all(|segment| matches!(segment, PathSegment::Move { .. } | PathSegment::Line { .. })));
    assert_eq!(flattened.len(), 1 + CURVE_LINE_SAMPLE_STEPS * 2);
    match flattened.last().unwrap() {
        PathSegment::Line { to } => assert!((to[0] - 4.0).abs() < 1e-9 && (to[1] - 0.0).abs() < 1e-9),
        other => panic!("expected line, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn drawing_layer_descendant_leaf_ids_flattens_nested_groups_to_leaves() {
    let leaf_a = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("A")).to_string().into()).expect("nonempty authored identity"), "A");
    let leaf_a_id = layer_id(&leaf_a).to_string();
    let leaf_b = create_drawing_trace_layer(crate::schema::identity::DrawingIdentity::admit((("B")).to_string().into()).expect("nonempty authored identity"), "B", "src");
    let leaf_b_id = layer_id(&leaf_b).to_string();
    let mut inner_group = create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Inner")).to_string().into()).expect("nonempty authored identity"), "Inner");
    if let DrawingLayerNode::Group(body) = &mut inner_group {
        body.children.push(leaf_a);
        body.children.push(leaf_b);
    }
    let leaf_c = create_drawing_text_layer(crate::schema::identity::DrawingIdentity::admit((("C")).to_string().into()).expect("nonempty authored identity"), "C");
    let leaf_c_id = layer_id(&leaf_c).to_string();
    let mut outer_group = create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Outer")).to_string().into()).expect("nonempty authored identity"), "Outer");
    if let DrawingLayerNode::Group(body) = &mut outer_group {
        body.children.push(inner_group);
        body.children.push(leaf_c);
    }
    assert_eq!(drawing_layer_descendant_leaf_ids(&outer_group), vec![leaf_a_id, leaf_b_id, leaf_c_id]);

    let leaf = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Solo")).to_string().into()).expect("nonempty authored identity"), "Solo");
    let leaf_id_value = layer_id(&leaf).to_string();
    assert_eq!(drawing_layer_descendant_leaf_ids(&leaf), vec![leaf_id_value]);
}

#[semio_framework_async_macros::async_test]
async fn resolve_boolean_layer_segments_returns_empty_for_missing_children_and_invalid_operation() {
    let mut doc = default_drawing_document("bool-empty", None);
    doc.layers.clear();
    let boolean_missing = DrawingBooleanBody { base: default_layer_base(crate::schema::identity::DrawingIdentity::admit((("B")).to_string().into()).expect("nonempty authored identity"), "B"), operation: "union".into(), children: vec!["missing".into()].into() };
    assert!(resolve_boolean_layer_segments(&doc, &boolean_missing).is_empty());

    let mut rect_a = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("A")).to_string().into()).expect("nonempty authored identity"), "A");
    if let DrawingLayerNode::Shape(shape) = &mut rect_a {
        shape.rect = Some(DrawingRect { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
    }
    let id_a = layer_id(&rect_a).clone();
    let mut rect_b = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("B")).to_string().into()).expect("nonempty authored identity"), "B");
    if let DrawingLayerNode::Shape(shape) = &mut rect_b {
        shape.rect = Some(DrawingRect { x: 2.0, y: 2.0, width: 5.0, height: 5.0 });
    }
    let id_b = layer_id(&rect_b).clone();
    doc.layers.push(rect_a);
    doc.layers.push(rect_b);
    let boolean_invalid = DrawingBooleanBody { base: default_layer_base(crate::schema::identity::DrawingIdentity::admit((("B")).to_string().into()).expect("nonempty authored identity"), "B"), operation: "not-a-real-op".into(), children: [id_a, id_b].into_iter().collect() };
    assert!(resolve_boolean_layer_segments(&doc, &boolean_invalid).is_empty());
}

#[test]
fn drawing_image_asset_luma_uses_intrinsic_alpha_weighted_samples_and_refuses_wrong_extent(){
 let asset=DrawingImageAsset{width:2,height:1,samples:vec![[255,255,255,255],[255,0,0,128]].into()};
 let (width,height,luma)=drawing_image_asset_luma(&asset).unwrap();assert_eq!((width,height),(2,1));assert_eq!(luma,vec![255,38]);
 let malformed=DrawingImageAsset{width:3,height:1,samples:asset.samples.clone()};assert!(drawing_image_asset_luma(&malformed).is_none());assert!(drawing_image_samples(&malformed).is_none());
}

#[semio_framework_async_macros::async_test]
async fn resolve_drawing_artboard_falls_back_to_layer_bounds_and_returns_none_when_no_bounds() {
    let mut doc = default_drawing_document("artboard-fallback", None);
    doc.artboard = Some(DrawingArtboard { width: 0.0, height: 0.0 });
    doc.layers.clear();
    let mut rect = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("R")).to_string().into()).expect("nonempty authored identity"), "R");
    if let DrawingLayerNode::Shape(shape) = &mut rect {
        shape.rect = Some(DrawingRect { x: 0.0, y: 0.0, width: 15.0, height: 25.0 });
    }
    doc.layers.push(rect);
    let artboard = resolve_drawing_artboard(&doc).expect("fallback bounds");
    assert_eq!((artboard.width, artboard.height), (15.0, 25.0));

    doc.artboard = None;
    doc.layers.clear();
    doc.layers.push(create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("EmptyGroup")).to_string().into()).expect("nonempty authored identity"), "EmptyGroup"));
    assert!(resolve_drawing_artboard(&doc).is_none());
}

#[semio_framework_async_macros::async_test]
async fn resolve_trace_layer_segments_returns_empty_without_assets_or_source_or_valid_decode() {
    let mut doc = default_drawing_document("trace-empty", None);
    doc.layers.clear();
    doc.assets = Default::default();
    let trace_no_assets = DrawingTraceBody { base: default_layer_base(crate::schema::identity::DrawingIdentity::admit((("T")).to_string().into()).expect("nonempty authored identity"), "T"), source_key: "missing".into(), params: default_drawing_trace_params() };
    assert!(resolve_trace_layer_segments(&doc, &trace_no_assets).is_empty());

    let mut assets = semio_framework_value::paged::PagedMap::default();
    assets.insert("present", DrawingImageAsset { width: 1, height: 1, samples: Default::default() });
    doc.assets = assets;
    let trace_missing_key = DrawingTraceBody { base: default_layer_base(crate::schema::identity::DrawingIdentity::admit((("T")).to_string().into()).expect("nonempty authored identity"), "T"), source_key: "missing".into(), params: default_drawing_trace_params() };
    assert!(resolve_trace_layer_segments(&doc, &trace_missing_key).is_empty());

    let trace_bad_decode = DrawingTraceBody { base: default_layer_base(crate::schema::identity::DrawingIdentity::admit((("T")).to_string().into()).expect("nonempty authored identity"), "T"), source_key: "present".into(), params: default_drawing_trace_params() };
    assert!(resolve_trace_layer_segments(&doc, &trace_bad_decode).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn create_layer_by_kind_covers_all_known_kinds_and_fallbacks() {
    assert_eq!(layer_kind_label(&create_layer_by_kind(crate::schema::identity::DrawingIdentity::admit((("shape:rect")).to_string().into()).expect("nonempty authored identity"), "shape:rect")), "shape:rect");
    assert_eq!(layer_kind_label(&create_layer_by_kind(crate::schema::identity::DrawingIdentity::admit((("shape:ellipse")).to_string().into()).expect("nonempty authored identity"), "shape:ellipse")), "shape:ellipse");
    assert_eq!(layer_kind_label(&create_layer_by_kind(crate::schema::identity::DrawingIdentity::admit((("shape:line")).to_string().into()).expect("nonempty authored identity"), "shape:line")), "shape:line");
    assert_eq!(layer_kind_label(&create_layer_by_kind(crate::schema::identity::DrawingIdentity::admit((("shape:polygon")).to_string().into()).expect("nonempty authored identity"), "shape:polygon")), "shape:polygon");
    assert_eq!(layer_kind_label(&create_layer_by_kind(crate::schema::identity::DrawingIdentity::admit((("shape:unknown")).to_string().into()).expect("nonempty authored identity"), "shape:unknown")), "shape:rect");
    assert_eq!(layer_kind_label(&create_layer_by_kind(crate::schema::identity::DrawingIdentity::admit((("path")).to_string().into()).expect("nonempty authored identity"), "path")), "path");
    assert_eq!(layer_kind_label(&create_layer_by_kind(crate::schema::identity::DrawingIdentity::admit((("text")).to_string().into()).expect("nonempty authored identity"), "text")), "text");
    assert_eq!(layer_kind_label(&create_layer_by_kind(crate::schema::identity::DrawingIdentity::admit((("image")).to_string().into()).expect("nonempty authored identity"), "image")), "image");
    assert_eq!(layer_kind_label(&create_layer_by_kind(crate::schema::identity::DrawingIdentity::admit((("group")).to_string().into()).expect("nonempty authored identity"), "group")), "group");
    assert_eq!(layer_kind_label(&create_layer_by_kind(crate::schema::identity::DrawingIdentity::admit((("boolean")).to_string().into()).expect("nonempty authored identity"), "boolean")), "boolean");
    assert_eq!(layer_kind_label(&create_layer_by_kind(crate::schema::identity::DrawingIdentity::admit((("trace")).to_string().into()).expect("nonempty authored identity"), "trace")), "trace");
    assert_eq!(layer_kind_label(&create_layer_by_kind(crate::schema::identity::DrawingIdentity::admit((("nonsense")).to_string().into()).expect("nonempty authored identity"), "nonsense")), "path");
}



#[semio_framework_async_macros::async_test]
async fn find_drawing_layer_location_reports_parent_and_index_or_none_when_missing() {
    let child = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Child")).to_string().into()).expect("nonempty authored identity"), "Child");
    let child_id = layer_id(&child).to_string();
    let mut group = create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
    if let DrawingLayerNode::Group(body) = &mut group {
        body.children.push(child);
    }
    let group_id = layer_id(&group).to_string();
    let top_level = create_drawing_text_layer(crate::schema::identity::DrawingIdentity::admit((("Top")).to_string().into()).expect("nonempty authored identity"), "Top");
    let top_id = layer_id(&top_level).to_string();
    let mut doc = default_drawing_document("locate", None);
    doc.layers = vec![group, top_level].into();

    let child_location = find_drawing_layer_location(&doc, &child_id).expect("child location");
    assert!(child_location.parent_id.as_ref().is_some_and(|id| id.eq_str(&group_id)));
    assert_eq!(child_location.index, 0);

    let top_location = find_drawing_layer_location(&doc, &top_id).expect("top location");
    assert_eq!(top_location.parent_id, None);
    assert_eq!(top_location.index, 1);

    assert!(find_drawing_layer_location(&doc, "missing").is_none());
}

#[test]
fn nested_group_transform_reaches_scene_and_bounds() {
    let mut child = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Child")).to_string().into()).expect("nonempty authored identity"), "Child");
    if let DrawingLayerNode::Shape(shape) = &mut child { shape.rect = Some(DrawingRect { x: 0.0, y: 0.0, width: 10.0, height: 20.0 }); }
    layer_base_mut(&mut child).transform.x = 4.0;
    layer_base_mut(&mut child).transform.y = 5.0;
    let mut group = create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
    layer_base_mut(&mut group).transform = DrawingTransform { x: 10.0, y: 20.0, scale_x: 2.0, scale_y: 3.0, rotation: 0.0, shear: 0.0 };
    if let DrawingLayerNode::Group(body) = &mut group { body.children.push(child); }
    assert_eq!(drawing_layer_world_bounds(&group), Some((18.0, 35.0, 20.0, 60.0)));
    let document = DrawingSnapshot { layers: vec![group].into(), ..Default::default() };
    let scene = flatten_drawing_document_to_scene_nodes(&document);
    assert_eq!(scene[0].transform, [2.0, 0.0, 0.0, 3.0, 18.0, 35.0]);
}

#[test]
fn curve_bounds_include_the_visible_extremum() {
    let layer = create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("Curve")).to_string().into()).expect("nonempty authored identity"), "Curve", vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Cubic { ctrl1: [0.0, 100.0], ctrl2: [100.0, 100.0], to: [100.0, 0.0] }].into());
    assert_eq!(drawing_layer_world_bounds(&layer), Some((0.0, 0.0, 100.0, 75.0)));
}

#[test]
fn tree_row_ids_preserve_dotted_layer_ids() {
    assert_eq!(drawing_play_layer_id_from_tree_row_id("drawing-play-layers.shape.shape.a"), Some("shape.a".into()));
    assert_eq!(drawing_play_layer_id_from_tree_row_id("drawing-play-layers.group.group.a"), Some("group.a".into()));
}

#[test]
fn duplicated_group_remaps_internal_boolean_references() {
    let child = create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit((("Operand")).to_string().into()).expect("nonempty authored identity"), "Operand");
    let original_id = layer_id(&child).clone();
    let boolean = create_drawing_boolean_layer(crate::schema::identity::DrawingIdentity::admit((("Result")).to_string().into()).expect("nonempty authored identity"), "Result", "union", [original_id.clone(), "external".into()].into_iter().collect());
    let mut group = create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("Container")).to_string().into()).expect("nonempty authored identity"), "Container");
    if let DrawingLayerNode::Group(body) = &mut group { body.children = vec![child, boolean].into(); }
    let DrawingLayerNode::Group(copy) = clone_drawing_layer_node(&group," copy",&clone_assignments(&group)).unwrap() else { panic!("Expected a group") };
    let DrawingLayerNode::Boolean(boolean) = &copy.children[1] else { panic!("Expected Boolean operands") };
    assert_ne!(layer_id(&copy.children[0]), &original_id);
    assert_eq!(boolean.children, [layer_id(&copy.children[0]).clone(), "external".into()].into_iter().collect());
}

#[test]
fn text_line_layout_matches_the_shared_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/📝️text/🧫️fixtures/🔣️.json" )).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let content = case["content"].as_str().unwrap();
        let size = case["size"].as_f64().unwrap();
        let actual = semio_framework_2d::text::drawing_text_lines(content).collect::<Vec<_>>();
        assert_eq!(serde_json::json!(actual), case["lines"]);
        for (index, value) in semio_framework_2d::text::drawing_text_fallback_extent(content, size).iter().enumerate() { assert!((value - case["extent"][index].as_f64().unwrap()).abs() < 1e-10); }
    }
}

#[test]
fn text_scene_preserves_authored_coordinates_under_layer_transform() {
    let mut layer = create_drawing_text_layer(crate::schema::identity::DrawingIdentity::admit((("Positioned text")).to_string().into()).expect("nonempty authored identity"), "Positioned text");
    let DrawingLayerNode::Text(text) = &mut layer else { unreachable!() };
    text.x = 7.0;
    text.y = 11.0;
    text.base.transform.x = 100.0;
    text.base.transform.y = 200.0;
    text.base.transform.scale_x = 2.0;
    text.base.transform.scale_y = 3.0;
    let snapshot = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    let nodes = flatten_drawing_document_to_scene_nodes(&snapshot);
    assert_eq!(nodes[0].transform, [2.0, 0.0, 0.0, 3.0, 114.0, 233.0]);
}

#[test]
fn scene_preserves_authored_curve_segments_for_canvas_and_vector_export() {
    let segments = vec![PathSegment::Move { to:[0.0,0.0] },PathSegment::Arc { rx:5.0,ry:3.0,rotation:25.0,large_arc:true,sweep:false,to:[30.0,10.0] },PathSegment::Quad { ctrl:[5.0,9.0],to:[10.0,0.0] }];
    let snapshot = DrawingSnapshot { layers:vec![create_drawing_path_layer(crate::schema::identity::DrawingIdentity::admit((("Curves")).to_string().into()).expect("nonempty authored identity"), "Curves",segments.iter().cloned().collect())].into(),..Default::default() };
    assert_eq!(flatten_drawing_document_to_scene_nodes(&snapshot)[0].segments,segments);
}

fn clone_assignments(node:&DrawingLayerNode)->semio_framework_value::list::PagedList<crate::schema::identity::DrawingIdentityAssignment,{usize::MAX}>{
 fn visit(node:&DrawingLayerNode,output:&mut Vec<crate::schema::identity::DrawingIdentityAssignment>){let source=layer_id(node).clone();let target=format!("copy:{source}").into();output.push(crate::schema::identity::DrawingIdentityAssignment{source,target});if let DrawingLayerNode::Group(group)=node{for child in &group.children{visit(child,output);}}}
 let mut output=Vec::new();visit(node,&mut output);output.into()
}
