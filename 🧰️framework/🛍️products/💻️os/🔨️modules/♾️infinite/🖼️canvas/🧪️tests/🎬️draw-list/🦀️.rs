//! 🎬️ Law: one node-graph picture, painted with the canvas's own primitives at a real camera,
//! encodes to exactly the draw list every other implementation of this contract must produce.
//!
//! The fixture (`🔣️.json`) is the language-agnostic half — camera, viewport, grid, wires, nodes,
//! ports — and `📐️expected-draw-list.json` is the encoding both sides are pinned to. The
//! JavaScript twin (`🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🟨️.js`) reads the SAME two files and
//! asserts the 2D-context calls a replay of that draw list makes, so a drift in either direction
//! fails on one side or the other.

use super::camera::{camera_content_affine, Camera, Viewport};
use super::draw_list::{scene_draw_list_json, DrawListOptions};
use super::{Circle, Color, FillRule, Line, Point, Rect, RoundedRect, RoundedRectRadii, Scene, Stroke};
use geometry::CubicBez;
use serde_json::Value;

const DRAW_LIST_FIXTURE: &str = include_str!("../../🧫️fixtures/🎬️draw-list/🔣️.json");
const DRAW_LIST_EXPECTATION: &str = include_str!("../../🧫️fixtures/🎬️draw-list/📐️expected-draw-list.json");

fn number(value: &Value) -> f64 {
    value.as_f64().expect("fixture number")
}

fn numbers(value: &Value) -> Vec<f64> {
    value.as_array().expect("fixture number array").iter().map(number).collect()
}

fn color(value: &Value) -> Color {
    let parts = numbers(value);
    Color::from_rgba8(parts[0] as u8, parts[1] as u8, parts[2] as u8, parts[3] as u8)
}

/// 🖌️ Paints the fixture's graph in the documented order. This is deliberately the same primitive
/// vocabulary `DagHost::paint_scene` uses — rounded-rect node bodies, circular ports, cubic wires,
/// straight grid lines, all under one viewport clip — so the encoding is exercised on the shapes
/// the real node-graph painter actually emits, not on a synthetic sampler.
fn paint_fixture_scene(fixture: &Value) -> Scene {
    let camera = Camera { x: number(&fixture["camera"]["x"]), y: number(&fixture["camera"]["y"]), zoom: number(&fixture["camera"]["zoom"]) };
    let viewport = Viewport { width: number(&fixture["viewport"]["width"]) as u32, height: number(&fixture["viewport"]["height"]) as u32, dpr: number(&fixture["viewport"]["dpr"]) };
    let aff = camera_content_affine(&camera, &viewport);
    let mut scene = Scene::new();

    let clip = Rect::new(0.0, 0.0, f64::from(viewport.width), f64::from(viewport.height));
    scene.push_clip_layer(FillRule::NonZero, super::Affine::IDENTITY, &clip);

    let grid = &fixture["grid"];
    let grid_color = color(&grid["color"]);
    let grid_stroke = Stroke::new(number(&grid["strokeWidth"]));
    for line in grid["lines"].as_array().expect("fixture grid lines") {
        let p = numbers(line);
        scene.stroke(&grid_stroke, aff, grid_color, None, &Line::new(Point::new(p[0], p[1]), Point::new(p[2], p[3])));
    }

    for wire in fixture["wires"].as_array().expect("fixture wires") {
        let c = numbers(&wire["curve"]);
        let curve = CubicBez { p0: Point::new(c[0], c[1]), p1: Point::new(c[2], c[3]), p2: Point::new(c[4], c[5]), p3: Point::new(c[6], c[7]) };
        scene.stroke(&Stroke::new(number(&wire["strokeWidth"])), aff, color(&wire["color"]), None, &curve);
    }

    for node in fixture["nodes"].as_array().expect("fixture nodes") {
        let (x, y) = (number(&node["x"]), number(&node["y"]));
        let (half_width, half_height) = (number(&node["width"]) * 0.5, number(&node["height"]) * 0.5);
        let radius = number(&node["radius"]);
        let body = RoundedRect::new(Rect::new(x - half_width, y - half_height, x + half_width, y + half_height), RoundedRectRadii::new(radius, radius, radius, radius));
        scene.fill(FillRule::NonZero, aff, color(&node["fill"]), None, &body);
        scene.stroke(&Stroke::new(number(&node["strokeWidth"])), aff, color(&node["stroke"]), None, &body);
    }

    for port in fixture["ports"].as_array().expect("fixture ports") {
        let disc = Circle::new(Point::new(number(&port["x"]), number(&port["y"])), number(&port["radius"]));
        scene.fill(FillRule::NonZero, aff, color(&port["fill"]), None, &disc);
        scene.stroke(&Stroke::new(number(&port["strokeWidth"])), aff, color(&port["stroke"]), None, &disc);
    }

    scene.pop_layer();
    scene
}

#[test]
fn the_node_graph_fixture_encodes_to_the_shared_draw_list() {
    let fixture: Value = serde_json::from_str(DRAW_LIST_FIXTURE).expect("draw-list fixture");
    let expected: Value = serde_json::from_str(DRAW_LIST_EXPECTATION).expect("draw-list expectation");
    let encoded: Value = serde_json::from_str(&scene_draw_list_json(&paint_fixture_scene(&fixture), DrawListOptions::default())).expect("encoded draw list");
    assert_eq!(encoded, expected, "the encoded draw list drifted from the shared expectation the JavaScript twin replays");
}

#[test]
fn every_command_keeps_the_camera_affine_the_painter_baked_in() {
    let fixture: Value = serde_json::from_str(DRAW_LIST_FIXTURE).expect("draw-list fixture");
    let camera = Camera { x: number(&fixture["camera"]["x"]), y: number(&fixture["camera"]["y"]), zoom: number(&fixture["camera"]["zoom"]) };
    let viewport = Viewport { width: number(&fixture["viewport"]["width"]) as u32, height: number(&fixture["viewport"]["height"]) as u32, dpr: number(&fixture["viewport"]["dpr"]) };
    let coefficients = camera_content_affine(&camera, &viewport).as_coeffs();
    let encoded: Value = serde_json::from_str(&scene_draw_list_json(&paint_fixture_scene(&fixture), DrawListOptions::default())).expect("encoded draw list");
    let commands = encoded["commands"].as_array().expect("commands");

    let content: Vec<&Value> = commands.iter().filter(|command| matches!(command[0].as_str(), Some("f") | Some("s"))).collect();
    assert!(!content.is_empty(), "a node-graph draw list must carry drawing commands");
    for command in content {
        let transform = numbers(&command[if command[0] == "f" { 3 } else { 7 }]);
        for (index, expected) in coefficients.iter().enumerate() {
            assert!((transform[index] - expected).abs() < 1e-3, "command {command} lost the camera affine coefficient {index}");
        }
    }
    assert!(coefficients[0] > 1.0, "the fixture camera must actually zoom, or this law proves nothing");
}

#[test]
fn a_zoomed_camera_moves_every_encoded_coordinate() {
    let mut fixture: Value = serde_json::from_str(DRAW_LIST_FIXTURE).expect("draw-list fixture");
    let at_rest = scene_draw_list_json(&paint_fixture_scene(&fixture), DrawListOptions::default());
    fixture["camera"]["zoom"] = Value::from(2.5);
    fixture["camera"]["x"] = Value::from(-310.0);
    let zoomed = scene_draw_list_json(&paint_fixture_scene(&fixture), DrawListOptions::default());
    assert_ne!(at_rest, zoomed, "a camera change that does not change the draw list means the camera is not in the encoding");
}

#[test]
fn shapes_stay_primitives_so_a_replay_never_facets_a_circle() {
    let fixture: Value = serde_json::from_str(DRAW_LIST_FIXTURE).expect("draw-list fixture");
    let encoded: Value = serde_json::from_str(&scene_draw_list_json(&paint_fixture_scene(&fixture), DrawListOptions::default())).expect("encoded draw list");
    let commands = encoded["commands"].as_array().expect("commands");
    let tags: Vec<&str> = commands
        .iter()
        .filter_map(|command| match command[0].as_str() {
            Some("f") => command[4][0].as_str(),
            Some("s") => command[8][0].as_str(),
            _ => None,
        })
        .collect();
    let port_count = fixture["ports"].as_array().expect("ports").len();
    let node_count = fixture["nodes"].as_array().expect("nodes").len();
    assert_eq!(tags.iter().filter(|tag| **tag == "ci").count(), port_count * 2, "ports must encode as circles, not flattened paths");
    assert_eq!(tags.iter().filter(|tag| **tag == "rr").count(), node_count * 2, "node bodies must encode as rounded rectangles");
    assert_eq!(tags.iter().filter(|tag| **tag == "cb").count(), fixture["wires"].as_array().expect("wires").len(), "wires must encode as cubic segments");
    assert!(!tags.contains(&"p"), "no shape in this fixture needs a flattened verb stream");
}

#[test]
fn a_scene_over_the_command_ceiling_truncates_loudly() {
    let mut scene = Scene::new();
    for index in 0..8 {
        scene.fill(FillRule::NonZero, super::Affine::IDENTITY, Color::from_rgba8(1, 2, 3, 255), None, &Rect::new(0.0, 0.0, f64::from(index), 1.0));
    }
    let options = DrawListOptions { maximum_commands: 3, ..DrawListOptions::default() };
    let encoded: Value = serde_json::from_str(&scene_draw_list_json(&scene, options)).expect("encoded draw list");
    assert_eq!(encoded["truncated"], Value::Bool(true));
    assert_eq!(encoded["commands"].as_array().expect("commands").len(), 3);
    assert_eq!(encoded["version"], Value::from(super::draw_list::DRAW_LIST_VERSION));
}
