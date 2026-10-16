//! 🧪️ dag-model tests (🔬️dag-unit): compiled against the `testkit` surface of the board crates.

use semio_framework_os_infinite_board_schema::{DagLayoutOptions, DagLayoutOrientation, LayoutControl};
use semio_framework_os_infinite_board_engine::CanvasPalette;
use semio_framework_os_infinite_board_dag_model::{apply_dag_layout, channel_row_bounds, channel_row_divider_y, cluster_explode_hit, cluster_explode_hit_rect, computation_channel_row_count, computation_channel_row_divider_x_span, computation_column_divider_x, computation_input_column_x_bounds, computation_io_side_row_counts, computation_io_side_row_divider_indices, computation_name_world_center, computation_node_height, computation_node_width, computation_output_column_x_bounds, ComputationChannelRowSide, DAG_COMPONENT_WIDTH, dag_draw_lod, dag_edge_body_stroke, dag_handle_body_stroke, DAG_HANDLE_WORLD_RADIUS, DAG_IO_COLUMN_WIDTH, dag_label_compact_paint_px, DAG_LABEL_COMPACT_SCREEN_PX, dag_label_layout_px, dag_label_paint_px, DAG_LABEL_SCREEN_PX, dag_lod_band_floor_zoom, dag_node_body_stroke, dag_node_internal_chrome_stroke, dag_node_label_fill, dag_node_paint_fill, fit_node_size, input_port_row_hit_bounds, io_node_handle_angles, io_node_rect_port_angle, io_port_column_width, io_widget_height, io_widget_width, note_widget_size, output_port_row_hit_bounds, slider_track_bounds, slider_widget_height, slider_widget_width, truncate_label_to_fit_width, would_create_cycle};
use semio_framework_canvas::{Color, Point};
use graph::NodeShape;
use ::{graph, semio_framework_canvas as canvas};
use semio_framework_graph_drawing::routing::{handle_exterior_cap_fill_path, handle_outward_at_node_rim, handle_position_on_rectangle};
use semio_framework_os_infinite_directed_presentation::*;

#[test]
fn io_node_handle_angles_left_right() {
    let (in_a, out_a) = io_node_handle_angles(0, 2, 0, 1);
    assert!(in_a > std::f64::consts::FRAC_PI_2);
    assert!(out_a.abs() < std::f64::consts::FRAC_PI_2);
}

#[test]
fn app_instance_node_serializes_and_sizes_n_ports() {
    let node = DagNodeSpec {
        id: "node-a".into(),
        name: "Draw".into(),
        abbreviation: "drw".into(),
        icon: "emoji:draw".into(),
        x: 100.0,
        y: 80.0,
        width: 180.0,
        height: 92.0,
        kind: DagNodeKind::AppInstance {
            instance_id: "app-1".into(),
            plugin_id: "draw".into(),
            app_id: "draw".into(),
            icon: "emoji:draw".into(),
            inputs: vec![IoPortSpec::simple("in-a", "In")],
            outputs: vec![IoPortSpec::simple("out-a", "Out"), IoPortSpec::simple("out-b", "Mesh")],
        },
        ..Default::default()
    };
    assert_eq!(dag_node_kind_tag(&node.kind), "appInstance");
    assert_eq!(node.inputs().len(), 1);
    assert_eq!(node.outputs().len(), 2);
    let json = semio_framework_pack_json::to_json_string(&node);
    assert!(json.contains("appInstance"));
    assert!(json.contains("instanceId"));
    let mut sized = node.clone();
    fit_node_size(&mut sized);
    assert!(sized.height >= 56.0);
}

#[test]
fn cycle_detection_blocks_back_edge() {
    let edges = vec![("a".into(), "b".into()), ("b".into(), "c".into())];
    assert!(would_create_cycle(&edges, "c", "a"));
    assert!(!would_create_cycle(&edges, "a", "c"));
}

/// 🧪️ Two-node/one-edge `dag.hostDocument` literal shared by the layout tests below.
fn ab_edge_layout_fixture()->DagHostSnapshot{DagHostSnapshot{schema:"dag.hostDocument".into(),nodes:["a","b"].into_iter().map(|id|DagNodeSpec{id:id.into(),..Default::default()}).collect(),edges:vec![DagHostSnapshotEdge{id:"e1".into(),source:"a".into(),target:"b".into(),..Default::default()}],camera:DagCamera{x:0.0,y:0.0,zoom:1.0}}}

macro_rules! dag_layout {($snapshot:expr,$options:expr)=>{{let mut progress=|_|true;let mut control=LayoutControl::new(1_000_000,&mut progress);apply_dag_layout($snapshot,$options,&mut control)}};}

#[test]
fn dag_layout_left_right_orders_depth_on_x() {
    let mut fixture = ab_edge_layout_fixture();
    dag_layout!(&mut fixture, &DagLayoutOptions::default()).unwrap();
    let a_x = fixture.nodes[0].x;
    let b_x = fixture.nodes[1].x;
    assert!(b_x > a_x + 1.0);
}

#[test]
fn dag_layout_top_bottom_orders_depth_on_y() {
    let mut fixture = ab_edge_layout_fixture();
    let opts = DagLayoutOptions { orientation: DagLayoutOrientation::TopBottom, ..DagLayoutOptions::default() };
    dag_layout!(&mut fixture, &opts).unwrap();
    let a_y = fixture.nodes[0].y;
    let b_y = fixture.nodes[1].y;
    assert!(b_y > a_y + 1.0);
}

#[test]
fn dag_layout_spacing_scales_coordinates() {
    let mut fixture = ab_edge_layout_fixture();
    dag_layout!(&mut fixture, &DagLayoutOptions::default()).unwrap();
    let default_gap = (fixture.nodes[1].x - fixture.nodes[0].x).abs();
    let mut wide = fixture.clone();
    dag_layout!(&mut wide, &DagLayoutOptions { layer_spacing: 240.0, sibling_gap: 80.0, ..DagLayoutOptions::default() }).unwrap();
    let wide_gap = (wide.nodes[1].x - wide.nodes[0].x).abs();
    assert!(wide_gap > default_gap * 1.5);
}

#[test]
fn slider_track_bounds_stay_inside_node_rect() {
    let output = IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() };
    let node = DagNodeSpec {
        id: "slider".into(),
        name: "Amount".into(),
        abbreviation: "Amount".into(),
        icon: "emoji:🎚️".into(),
        x: 100.0,
        y: 50.0,
        width: slider_widget_width("Amount", &output),
        height: slider_widget_height(),
        kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 2.0, output },
        ..Default::default()
    };
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let (left, top, right, bottom) = slider_track_bounds(&node);
    assert!(left >= node.x - hw);
    assert!(right <= node.x + hw);
    assert!((top + bottom) * 0.5 - node.y < 1e-6);
    assert!(top >= node.y - hh);
    assert!(bottom <= node.y + hh);
}

#[test]
fn input_port_row_hit_bounds_span_input_channel() {
    let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "b".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let width = computation_node_width("Node", &inputs, &outputs);
    let height = computation_node_height(2, 1, false, false);
    let node = DagNodeSpec::computation("n".into(), "Node", "Node", "emoji:🔢️".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
    let hw = width * 0.5;
    let divider_x = computation_column_divider_x(&node).expect("divider");
    let (x0, _, x1, _) = input_port_row_hit_bounds(&node, 1).expect("row");
    assert!((x0 - (node.x - hw)).abs() < 1e-9);
    assert!((x1 - divider_x).abs() < 1e-9);
}

#[test]
fn output_port_row_hit_bounds_span_output_channel() {
    let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "x".into(), label: "x".into(), ..Default::default() }, IoPortSpec { id: "y".into(), label: "y".into(), ..Default::default() }];
    let width = computation_node_width("Node", &inputs, &outputs);
    let height = computation_node_height(1, 2, false, false);
    let node = DagNodeSpec::computation("n".into(), "Node", "Node", "emoji:🔢️".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
    let hw = width * 0.5;
    let divider_x = computation_column_divider_x(&node).expect("divider");
    let (x0, _, x1, _) = output_port_row_hit_bounds(&node, 1).expect("row");
    assert!((x0 - divider_x).abs() < 1e-9);
    assert!((x1 - (node.x + hw)).abs() < 1e-9);
}

#[test]
fn output_only_slider_divider_splits_output_hit_row() {
    let output = IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() };
    let node = DagNodeSpec {
        id: "slider".into(),
        name: "Amount".into(),
        abbreviation: "Amount".into(),
        icon: "emoji:🎚️".into(),
        x: 80.0,
        y: 40.0,
        width: slider_widget_width("Amount", &output),
        height: slider_widget_height(),
        kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 2.0, output },
        ..Default::default()
    };
    let divider_x = computation_column_divider_x(&node).expect("divider");
    assert!((divider_x - node.x).abs() < 1e-9);
    let hw = node.width * 0.5;
    let (x0, _, x1, _) = output_port_row_hit_bounds(&node, 0).expect("output row");
    assert!((x0 - divider_x).abs() < 1e-9);
    assert!((x1 - (node.x + hw)).abs() < 1e-9);
}

#[test]
fn component_width_is_twice_channel_width() {
    assert_eq!(DAG_COMPONENT_WIDTH, DAG_IO_COLUMN_WIDTH * 2.0);
}

#[test]
fn computation_node_width_is_uniform_for_all_components() {
    let inputs = vec![
        IoPortSpec { id: "cornerA".into(), label: "cornerA".into(), ..Default::default() },
        IoPortSpec { id: "cornerB".into(), label: "cornerB".into(), ..Default::default() },
        IoPortSpec { id: "height".into(), label: "height".into(), ..Default::default() },
    ];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
    let width = computation_node_width("Box", &inputs, &outputs);
    assert_eq!(width, DAG_COMPONENT_WIDTH);
}

#[test]
fn computation_io_columns_use_uniform_channel_width() {
    let inputs_short = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }];
    let inputs_long = vec![IoPortSpec { id: "cornerA".into(), label: "cornerA".into(), ..Default::default() }, IoPortSpec { id: "cornerB".into(), label: "cornerB".into(), ..Default::default() }];
    let outputs_short = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let outputs_long = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
    let short = computation_node_width("n", &inputs_short, &outputs_short);
    let long = computation_node_width("n", &inputs_long, &outputs_long);
    assert_eq!(short, DAG_COMPONENT_WIDTH);
    assert_eq!(long, DAG_COMPONENT_WIDTH);
    assert_eq!(io_port_column_width(&inputs_long, DAG_LABEL_COMPACT_SCREEN_PX), DAG_IO_COLUMN_WIDTH);
    assert_eq!(io_port_column_width(&outputs_long, DAG_LABEL_COMPACT_SCREEN_PX), DAG_IO_COLUMN_WIDTH);
}

#[test]
fn computation_column_divider_splits_io_columns() {
    let inputs = vec![IoPortSpec { id: "cornerA".into(), label: "cornerA".into(), ..Default::default() }, IoPortSpec { id: "cornerB".into(), label: "cornerB".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
    let width = computation_node_width("Box", &inputs, &outputs);
    let height = computation_node_height(2, 1, false, false);
    let node = DagNodeSpec::computation("box".into(), "Box", "Box", "emoji:📦️".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
    let divider_x = computation_column_divider_x(&node).expect("divider");
    let hw = width * 0.5;
    assert!(divider_x > node.x - hw + 1.0);
    assert!(divider_x < node.x + hw - 1.0);
}

#[test]
fn computation_name_sits_above_rectangle_centered() {
    let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let width = computation_node_width("Box", &inputs, &outputs);
    let height = computation_node_height(1, 1, false, false);
    let node = DagNodeSpec::computation("box".into(), "Box", "Box", "emoji:📦️".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
    let paint_px = dag_label_paint_px(1.0, 3);
    let (label_x, label_y) = computation_name_world_center(&node, "Box", paint_px, 1.0);
    assert!((label_x - node.x).abs() < 1e-6);
    let top = node.y - height * 0.5;
    assert!(label_y < top);
    let world_offset = top - label_y;
    let (_, label_h) = canvas::text::label_extent("Box", paint_px);
    let screen_offset = world_offset * 1.0;
    assert!((screen_offset - (DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_GAP_COMPACT_RATIO + label_h * 0.5)).abs() < 1e-6);
}

#[test]
fn io_widget_width_matches_component_width() {
    let width = io_widget_width("Amount");
    let height = io_widget_height("Amount");
    assert_eq!(width, DAG_COMPONENT_WIDTH);
    assert!(height >= 40.0);
}

#[test]
fn slider_widget_size_matches_function_row_metrics() {
    let input = IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() };
    let output = IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() };
    let width = slider_widget_width("Amount", &output);
    let height = slider_widget_height();
    assert_eq!(width, DAG_COMPONENT_WIDTH);
    assert_eq!(width, computation_node_width("Amount", &[input], &[output]));
    assert_eq!(height, DAG_CHANNEL_ROW_HEIGHT);
    assert!(width > height, "slider track should be wider than tall");
}

#[test]
fn computation_channel_row_count_matches_io_rows() {
    let node = DagNodeSpec::computation(
        "box".into(),
        "Box",
        "Box",
        "emoji:📦️".into(),
        vec![
            IoPortSpec { id: "cornerA".into(), label: "cornerA".into(), ..Default::default() },
            IoPortSpec { id: "cornerB".into(), label: "cornerB".into(), ..Default::default() },
            IoPortSpec { id: "height".into(), label: "height".into(), ..Default::default() },
        ],
        vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }],
        false,
        false,
        0.0,
        0.0,
        120.0,
        42.0,
    );
    assert_eq!(computation_channel_row_count(&node), 3);
}

#[test]
fn computation_channel_row_dividers_stop_at_last_port_on_shorter_side() {
    let three_inputs =
        vec![IoPortSpec { id: "a".into(), label: "cornerA".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "cornerB".into(), ..Default::default() }, IoPortSpec { id: "c".into(), label: "height".into(), ..Default::default() }];
    let one_output = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
    let three_outputs = vec![
        IoPortSpec { id: "outA".into(), label: "geometry".into(), ..Default::default() },
        IoPortSpec { id: "outB".into(), label: "mesh".into(), ..Default::default() },
        IoPortSpec { id: "outC".into(), label: "curve".into(), ..Default::default() },
    ];
    let one_input = vec![IoPortSpec { id: "a".into(), label: "cornerA".into(), ..Default::default() }];
    let more_inputs = DagNodeSpec::computation(
        "more-in".into(),
        "Box",
        "Box",
        "emoji:📦️".into(),
        three_inputs.clone(),
        one_output.clone(),
        false,
        false,
        0.0,
        0.0,
        computation_node_width("Box", &three_inputs, &one_output),
        computation_node_height(3, 1, false, false),
    );
    let more_outputs = DagNodeSpec::computation(
        "more-out".into(),
        "Box",
        "Box",
        "emoji:📦️".into(),
        one_input.clone(),
        three_outputs.clone(),
        false,
        false,
        0.0,
        0.0,
        computation_node_width("Box", &one_input, &three_outputs),
        computation_node_height(1, 3, false, false),
    );
    let grid = computation_channel_row_count(&more_inputs);
    assert_eq!(grid, 3);
    assert_eq!(computation_io_side_row_counts(&more_inputs), (3, 1));
    assert_eq!(computation_io_side_row_divider_indices(3, grid).collect::<Vec<_>>(), vec![1, 2]);
    assert_eq!(computation_io_side_row_divider_indices(1, grid).collect::<Vec<_>>(), vec![1]);
    assert_eq!(computation_io_side_row_counts(&more_outputs), (1, 3));
    assert_eq!(computation_io_side_row_divider_indices(1, grid).collect::<Vec<_>>(), vec![1]);
    assert_eq!(computation_io_side_row_divider_indices(3, grid).collect::<Vec<_>>(), vec![1, 2]);
}

#[test]
fn computation_channel_row_dividers_align_with_row_bounds() {
    let inputs =
        vec![IoPortSpec { id: "a".into(), label: "cornerA".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "cornerB".into(), ..Default::default() }, IoPortSpec { id: "c".into(), label: "height".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "outA".into(), label: "geometry".into(), ..Default::default() }, IoPortSpec { id: "outB".into(), label: "mesh".into(), ..Default::default() }];
    let width = computation_node_width("Box", &inputs, &outputs);
    let height = computation_node_height(3, 2, false, false);
    let node = DagNodeSpec::computation("box".into(), "Box", "Box", "emoji:📦️".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
    let (input_rows, output_rows) = computation_io_side_row_counts(&node);
    assert_eq!(input_rows, 3);
    assert_eq!(output_rows, 2);
    let (input_left, input_right) = computation_input_column_x_bounds(&node).expect("input column");
    let (output_left, output_right) = computation_output_column_x_bounds(&node).expect("output column");
    assert!(input_left < input_right);
    assert!(output_left < output_right);
    let divider_x = computation_column_divider_x(&node).expect("divider");
    let (input_span_left, input_span_right) = computation_channel_row_divider_x_span(&node, ComputationChannelRowSide::Input);
    let (output_span_left, output_span_right) = computation_channel_row_divider_x_span(&node, ComputationChannelRowSide::Output);
    assert!((input_span_left - (node.x - width * 0.5)).abs() < 1e-6);
    assert!((input_span_right - divider_x).abs() < 1e-6);
    assert!((output_span_left - divider_x).abs() < 1e-6);
    assert!((output_span_right - (node.x + width * 0.5)).abs() < 1e-6);
    assert!((divider_x - node.x).abs() < 1e-6, "2× channel width nodes split IO at center");
    let divider_y = channel_row_divider_y(node.y, node.height, 1);
    let (_, _row0_top, _, row0_bottom) = channel_row_bounds(&node, 0);
    let (_, row1_top, _, _row1_bottom) = channel_row_bounds(&node, 1);
    assert!((divider_y - row0_bottom).abs() < 1e-6);
    assert!((divider_y - row1_top).abs() < 1e-6);
}

#[test]
fn computation_node_size_fits_io_labels() {
    let inputs = vec![
        IoPortSpec { id: "width".into(), label: "width".into(), ..Default::default() },
        IoPortSpec { id: "depth".into(), label: "depth".into(), ..Default::default() },
        IoPortSpec { id: "height".into(), label: "height".into(), ..Default::default() },
    ];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
    let width = computation_node_width("brep.prim3d.box", &inputs, &outputs);
    let height = computation_node_height(3, 1, false, false);
    assert!(height <= 42.0, "expected compact height, got {height}");
    assert!(height < 96.0, "expected shorter than legacy 4-row layout");
    assert_eq!(width, DAG_COMPONENT_WIDTH);
}

#[test]
fn io_node_rect_port_angles_on_edges() {
    use graph::handle_position_on_rectangle;
    use canvas::Point;
    let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "b".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let width = computation_node_width("node", &inputs, &outputs);
    let height = computation_node_height(2, 1, false, false);
    let hw = width * 0.5;
    let left = io_node_rect_port_angle(0.0, 0.0, width, height, 0, 2, true);
    let right = io_node_rect_port_angle(0.0, 0.0, width, height, 0, 1, false);
    let left_pos = handle_position_on_rectangle(Point::new(0.0, 0.0), width, height, left);
    let right_pos = handle_position_on_rectangle(Point::new(0.0, 0.0), width, height, right);
    assert!(left_pos.x < -hw + 1.0);
    assert!(right_pos.x > hw - 1.0);
    assert!(left_pos.y < 0.0);
    assert!(right_pos.y < 0.0);
}

#[test]
fn computation_port_handle_caps_bulge_outward() {
    use graph::{NodeShape, handle_exterior_cap_fill_path, handle_outward_at_node_rim, handle_position_on_rectangle};
    use canvas::Point;
    let inputs = vec![IoPortSpec { id: "0".into(), label: "0".into(), ..Default::default() }, IoPortSpec { id: "1".into(), label: "1".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "dictionary".into(), ..Default::default() }];
    let width = computation_node_width("Merge", &inputs, &outputs);
    let height = computation_node_height(2, 1, true, false);
    let center = Point::new(100.0, 50.0);
    let left_angle = io_node_rect_port_angle(center.x, center.y, width, height, 0, 2, true);
    let right_angle = io_node_rect_port_angle(center.x, center.y, width, height, 0, 1, false);
    let left_pos = handle_position_on_rectangle(center, width, height, left_angle);
    let right_pos = handle_position_on_rectangle(center, width, height, right_angle);
    let left_out = handle_outward_at_node_rim(left_pos, center, NodeShape::Rectangle, 0.0, width, height).expect("left outward");
    let right_out = handle_outward_at_node_rim(right_pos, center, NodeShape::Rectangle, 0.0, width, height).expect("right outward");
    assert!((left_out.x + 1.0).abs() < 1e-9 && left_out.y.abs() < 1e-9);
    assert!((right_out.x - 1.0).abs() < 1e-9 && right_out.y.abs() < 1e-9);
    let left_cap = handle_exterior_cap_fill_path(left_pos, left_out, DAG_HANDLE_WORLD_RADIUS);
    let right_cap = handle_exterior_cap_fill_path(right_pos, right_out, DAG_HANDLE_WORLD_RADIUS);
    assert!(left_cap.bounding_box().x0() < left_pos.x - 1.0, "left input cap must bulge outside the west edge");
    assert!(right_cap.bounding_box().x1() > right_pos.x + 1.0, "right output cap must bulge outside the east edge");
}

#[test]
fn dag_draw_lod_maps_zoom_to_board_bands() {
    assert_eq!(dag_draw_lod(0.1), DagDrawLod::Minimap);
    assert_eq!(dag_draw_lod(0.45), DagDrawLod::Overview);
    assert_eq!(dag_draw_lod(0.65), DagDrawLod::Compact);
    assert_eq!(dag_draw_lod(1.2), DagDrawLod::Normal);
    assert_eq!(dag_draw_lod(2.5), DagDrawLod::Detail);
    assert_eq!(dag_draw_lod(3.0), DagDrawLod::Micro);
    assert_eq!(dag_draw_lod(5.0), DagDrawLod::Micro);
}

#[test]
fn dag_handle_world_radius_is_zoom_invariant() {
    assert_eq!(DAG_HANDLE_WORLD_RADIUS, 5.0);
}

#[test]
fn dag_label_paint_px_scales_with_zoom_inside_lod_band() {
    assert_eq!(dag_label_layout_px(), DAG_LABEL_SCREEN_PX);
    let normal = 3usize;
    let normal_floor = dag_lod_band_floor_zoom(normal);
    assert!((dag_label_paint_px(normal_floor, normal) - DAG_LABEL_SCREEN_PX).abs() < 1e-9);
    assert!((dag_label_paint_px(1.1, normal) - DAG_LABEL_SCREEN_PX * 1.1 / normal_floor).abs() < 1e-9);
    assert!((dag_label_paint_px(normal_floor, normal) - dag_label_paint_px(1.1, normal) * normal_floor / 1.1).abs() < 1e-9);
    assert!((dag_label_compact_paint_px(1.1, normal) - DAG_LABEL_COMPACT_SCREEN_PX * 1.1 / normal_floor).abs() < 1e-9);
}

#[test]
fn dag_node_paint_fill_matches_board_lod_chrome() {
    use canvas::Color;
    let theme = CanvasPalette {
        node_fill: Color::from_rgba8(10, 20, 30, 255),
        node_stroke: Color::from_rgba8(200, 210, 220, 255),
        node_fill_hovered: Color::from_rgba8(40, 50, 60, 255),
        node_stroke_hovered: Color::from_rgba8(90, 100, 110, 255),
        node_fill_selected: Color::from_rgba8(70, 80, 90, 255),
        node_stroke_selected: Color::from_rgba8(120, 130, 140, 255),
        node_fill_selection_exit: Color::from_rgba8(196, 228, 213, 255),
        node_stroke_selection_exit: Color::from_rgba8(80, 140, 110, 255),
        ..CanvasPalette::default()
    };
    assert_eq!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, false, false).expect("minimap neutral").to_rgba8(), theme.node_stroke.to_rgba8());
    assert!(dag_node_paint_fill(DagDrawLod::Overview, &theme, false, false, false, false).is_none());
    assert!(dag_node_paint_fill(DagDrawLod::Normal, &theme, false, false, false, false).is_none());
    assert_eq!(dag_node_paint_fill(DagDrawLod::Normal, &theme, false, true, false, false).expect("selected").to_rgba8(), theme.node_fill_selected.to_rgba8());
    assert_eq!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, false, true).expect("minimap hovered").to_rgba8(), theme.node_stroke_hovered.to_rgba8());
    assert_eq!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, true, false).expect("minimap highlighted").to_rgba8(), theme.node_stroke_selection_exit.to_rgba8());
    assert_eq!(dag_node_body_stroke(&theme, false, true, false, true).to_rgba8(), theme.node_stroke_selected.to_rgba8());
    assert_eq!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, true, false, true).expect("minimap selected").to_rgba8(), theme.node_stroke_selected.to_rgba8());
    assert_ne!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, false, true).expect("minimap hovered").to_rgba8(), theme.node_fill_hovered.to_rgba8());
    assert_eq!(dag_node_body_stroke(&theme, false, false, false, true).to_rgba8(), theme.node_stroke_hovered.to_rgba8());
    assert_eq!(dag_node_body_stroke(&theme, false, false, true, false).to_rgba8(), theme.node_stroke_selection_exit.to_rgba8());
    assert_eq!(dag_node_label_fill(&theme, false, false, false, true).to_rgba8(), theme.label_fill_hovered.to_rgba8());
    assert_eq!(dag_node_label_fill(&theme, false, false, true, false).to_rgba8(), theme.node_stroke_selection_exit.to_rgba8());
    assert_eq!(dag_node_label_fill(&theme, false, false, false, false).to_rgba8(), theme.label_fill.to_rgba8());
    assert_eq!(dag_node_label_fill(&theme, false, true, false, false).to_rgba8(), theme.label_fill_hovered.to_rgba8());
    let body = dag_node_body_stroke(&theme, false, false, false, false);
    let label = dag_node_label_fill(&theme, false, false, false, true);
    assert_eq!(dag_node_internal_chrome_stroke(body, label, true).to_rgba8(), label.to_rgba8());
    assert_eq!(dag_node_internal_chrome_stroke(body, label, false).to_rgba8(), body.to_rgba8());
    let body_selected = dag_node_body_stroke(&theme, false, true, false, false);
    let label_selected = dag_node_label_fill(&theme, false, true, false, false);
    assert_eq!(dag_node_internal_chrome_stroke(body_selected, label_selected, true).to_rgba8(), label_selected.to_rgba8());
    assert_eq!(dag_node_internal_chrome_stroke(body_selected, label_selected, false).to_rgba8(), body_selected.to_rgba8());
}

#[test]
fn dag_handle_and_edge_stroke_use_theme_defaults() {
    use canvas::Color;
    let theme = CanvasPalette {
        edge_stroke: Color::from_rgba8(100, 110, 120, 255),
        edge_stroke_hovered: Color::from_rgba8(10, 20, 30, 255),
        edge_stroke_selected: Color::from_rgba8(40, 50, 60, 255),
        handle_stroke: Color::from_rgba8(130, 140, 150, 255),
        handle_stroke_hovered: Color::from_rgba8(20, 30, 40, 255),
        handle_stroke_selected: Color::from_rgba8(50, 60, 70, 255),
        ..CanvasPalette::default()
    };
    assert_eq!(dag_edge_body_stroke(&theme, false, false, false, false).to_rgba8(), theme.edge_stroke.to_rgba8());
    assert_eq!(dag_handle_body_stroke(&theme, false, false, false, false).to_rgba8(), theme.handle_stroke.to_rgba8());
}

#[test]
fn note_widget_size_uses_uniform_component_width() {
    let short = note_widget_size("hi");
    let long = note_widget_size("some longer note text");
    assert_eq!(short.0, DAG_COMPONENT_WIDTH);
    assert_eq!(long.0, DAG_COMPONENT_WIDTH);
    assert_eq!(short.1, DAG_CHANNEL_ROW_HEIGHT);
    assert_eq!(long.1, DAG_CHANNEL_ROW_HEIGHT);
}

#[test]
fn truncate_label_to_fit_width_adds_ellipsis() {
    let px = 12.0;
    let max_w = 40.0;
    let truncated = truncate_label_to_fit_width("alpha beta gamma delta", max_w, px);
    assert!(truncated.ends_with('…'));
    assert!(truncated.len() < "alpha beta gamma delta".len());
}

#[test]
fn note_preview_action_port_accessors() {
    let note = DagNodeSpec {
        id: "note".into(),
        name: "Note".into(),
        abbreviation: "Note".into(),
        icon: "emoji:📝️".into(),
        x: 0.0,
        y: 0.0,
        width: note_widget_size("hi").0,
        height: note_widget_size("hi").1,
        kind: DagNodeKind::Note { text: "hi".into(), output: IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() } },
        ..Default::default()
    };
    assert!(note.inputs().is_empty());
    assert_eq!(note.outputs().len(), 1);
    let preview = DagNodeSpec {
        id: "preview".into(),
        name: "Preview".into(),
        abbreviation: "Preview".into(),
        icon: "emoji:👁️".into(),
        x: 0.0,
        y: 0.0,
        width: 120.0,
        height: 48.0,
        kind: DagNodeKind::Preview { content: DagPreviewContent::Scalar { text: "3".into() }, expanded: DagExpandedPaths::new(), input: IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() } },
        ..Default::default()
    };
    assert_eq!(preview.inputs().len(), 1);
    assert!(preview.outputs().is_empty());
    let export = DagNodeSpec {
        id: "export".into(),
        name: "Export SVG".into(),
        abbreviation: "SVG".into(),
        icon: "emoji:📤️".into(),
        x: 0.0,
        y: 0.0,
        width: 120.0,
        height: 48.0,
        kind: DagNodeKind::Export { label: "SVG".into(), format: "svg".into(), input: IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() } },
        ..Default::default()
    };
    assert_eq!(export.inputs().len(), 1);
    assert!(export.outputs().is_empty());
}

#[test]
fn cluster_explode_hit_rect_detects_top_right_affordance() {
    let inputs = vec![IoPortSpec::simple("a", "a")];
    let outputs = vec![IoPortSpec::simple("out", "out")];
    let node = DagNodeSpec::cluster("cluster".into(), "Cluster", "Cluster", "emoji:🧩️".into(), inputs, outputs, 0.0, 0.0, 120.0, 80.0);
    let (x0, y0, x1, y1) = cluster_explode_hit_rect(&node).expect("rect");
    assert!(cluster_explode_hit(&node, (x0 + x1) * 0.5, (y0 + y1) * 0.5));
    assert!(!cluster_explode_hit(&node, node.x - 50.0, node.y - 50.0));
}
