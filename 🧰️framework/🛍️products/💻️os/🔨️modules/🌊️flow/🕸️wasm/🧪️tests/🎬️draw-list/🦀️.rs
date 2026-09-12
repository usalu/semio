//! 🎬️ Laws over the REAL generation3d flow window payload: `render_frame` must hand the host the
//! scene it painted, encoded, camera-correct and inside the ABI reply body — the picture used to be
//! computed and then dropped, and the browser drew placeholder boxes at raw world coordinates.

use super::*;

const GENERATION_3D_FLOW_FIXTURE: &str = include_str!("🔣️.json");

fn generation_3d_flow_host() -> FlowHost {
    let payload: Value = serde_json::from_str(GENERATION_3D_FLOW_FIXTURE).expect("generation3d flow fixture");
    let fixture = FlowHost::parse_fixture_json(&payload["fixture"].to_string()).expect("parsed flow fixture");
    FlowHost::from_fixture(fixture)
}

/// 🧹️ `FlowFixture`'s ordered maps refuse to drop unretired (they panic inside the value layer), so
/// every lane that builds a real host must walk it through the same retirement ladder the session
/// close walks — see `FlowDomainAdapter::close_step`.
fn retire(host: FlowHost) {
    let mut retirement = FlowHostRetirement::new(host);
    for _ in 0..1_000_000 {
        if retirement.close_page(64, 65_536).expect("flow host retirement") {
            assert!(retirement.terminal_nonopaque_is_empty(), "flow host retired without reaching its terminal");
            return;
        }
    }
    panic!("flow host did not retire within bound");
}

fn generation_3d_draw_list(width: u32, height: u32) -> (usize, Value) {
    let mut host = generation_3d_flow_host();
    host.set_viewport(width, height, 1.0);
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, width, height, 1.0);
    let encoded = canvas::draw_list::scene_draw_list_json(&scene, canvas::draw_list::DrawListOptions::default());
    retire(host);
    (encoded.len(), serde_json::from_str(&encoded).expect("encoded draw list"))
}

#[test]
fn the_generation_3d_flow_window_paints_a_scene_that_survives_encoding() {
    let payload: Value = serde_json::from_str(GENERATION_3D_FLOW_FIXTURE).expect("generation3d flow fixture");
    let expect = &payload["provenance"]["expect"];
    let host = generation_3d_flow_host();
    assert_eq!(host.fixture.widgets.len(), expect["widgets"].as_u64().expect("widget count") as usize);
    assert_eq!(host.fixture.synapses.len(), expect["synapses"].as_u64().expect("synapse count") as usize);
    retire(host);

    let (bytes, encoded) = generation_3d_draw_list(966, 814);
    let commands = encoded["commands"].as_array().expect("commands");
    assert!(!commands.is_empty(), "the generation3d flow window painted nothing to encode");
    assert_eq!(encoded["truncated"], Value::Bool(false), "the live node-graph scene must fit the draw-list ceiling");
    assert!(bytes < protocol::FLOW_MAX_REQUEST_BYTES, "the encoded draw list ({bytes} bytes) must fit one ABI reply body");
    // 📏️ Measured on this exact fixture at 966x814: 47 commands / 22 593 bytes — two orders of
    // magnitude under the ABI reply body, which is what makes carrying the picture itself viable.
    assert!(bytes < 64 * 1024, "the live node-graph draw list grew past its measured envelope ({bytes} bytes)");
    let tags: Vec<&str> = commands.iter().filter_map(|command| command[0].as_str()).collect();
    assert!(tags.contains(&"f"), "a node graph must fill something");
    assert!(tags.contains(&"s"), "a node graph must stroke something");
}

#[test]
fn moving_the_camera_moves_the_encoded_picture() {
    let mut host = generation_3d_flow_host();
    host.set_viewport(966, 814, 1.0);
    let mut at_rest = canvas::Scene::new();
    host.paint_scene(&mut at_rest, 966, 814, 1.0);
    let at_rest = canvas::draw_list::scene_draw_list_json(&at_rest, canvas::draw_list::DrawListOptions::default());

    host.set_camera(420.0, -210.0, 0.75);
    let mut moved = canvas::Scene::new();
    host.paint_scene(&mut moved, 966, 814, 1.0);
    let moved = canvas::draw_list::scene_draw_list_json(&moved, canvas::draw_list::DrawListOptions::default());

    assert_ne!(at_rest, moved, "a camera move that leaves the encoded picture identical means the camera never reached the host");
    retire(host);
}

#[test]
fn render_frame_carries_the_draw_list_the_host_must_paint() {
    let mut adapter = FlowDomainAdapter::default();
    let payload: Value = serde_json::from_str(GENERATION_3D_FLOW_FIXTURE).expect("generation3d flow fixture");
    let fixture = FlowHost::parse_fixture_json(&payload["fixture"].to_string()).expect("parsed flow fixture");
    let FlowDomainHost::Open(empty) = std::mem::replace(&mut adapter.host, FlowDomainHost::Open(FlowHost::from_fixture(fixture))) else { panic!("open flow host") };
    retire(empty);
    adapter.width = 966;
    adapter.height = 814;
    adapter.dpr = 1.0;
    adapter.host.set_viewport(966, 814, 1.0);
    adapter.surface = Some(FlowSurface {
        id: SurfaceId::try_new(1).unwrap(),
        generation: SurfaceGeneration::try_new(1).unwrap(),
        metrics: CanvasMetrics::try_new(966, 814, 1.0).unwrap(),
        state: SurfaceState::Ready,
    });

    let frame: Value = serde_json::from_slice(&adapter.render_frame().expect("render frame")).expect("render frame payload");
    assert_eq!(frame["present"], Value::from("2d"), "with no attached WebGPU canvas a frame must present through the encoded draw list");
    assert_eq!(frame["draw"]["version"], Value::from(canvas::draw_list::DRAW_LIST_VERSION));
    assert!(!frame["draw"]["commands"].as_array().expect("commands").is_empty(), "render_frame dropped the scene it painted");
    assert_eq!(frame["clear"].as_array().expect("clear colour").len(), 4, "the replayer needs the host's own clear colour");
    assert!(frame["fixture"]["widgets"].as_array().expect("widgets").len() == 7, "the fixture half of the frame must survive the change");
    assert!(frame["labels"].is_object(), "the label overlay half of the frame must survive the change");
    adapter.surface = None;
    let FlowDomainHost::Open(host) = std::mem::replace(&mut adapter.host, FlowDomainHost::Closed) else { panic!("open flow host") };
    retire(host);
}

//#region 🙈️DevicelessReplay
/// 📐️ Screen-space bounds of one encoded command: its shape's own coordinates through its own
/// affine. The tag layout is the encoding's (`canvas::draw_list`): fill `["f",rule,paint,affine,
/// shape]`, stroke `["s",width,cap,cap,dash,dashOffset,paint,affine,shape]`, image `["i",w,h,
/// bytes,affine]`, layers `["pl",rule,blend,alpha,affine,clip]` / `["pc",rule,affine,clip]`.
fn command_screen_bounds(command: &Value) -> Option<(f64, f64, f64, f64)> {
    let tag = command[0].as_str()?;
    let (affine_at, shape_at) = match tag {
        "f" => (3, Some(4)),
        "s" => (7, Some(8)),
        "i" => (4, None),
        "pl" => (4, Some(5)),
        "pc" => (2, Some(3)),
        _ => return None,
    };
    let affine: Vec<f64> = command[affine_at].as_array()?.iter().filter_map(Value::as_f64).collect();
    if affine.len() != 6 {
        return None;
    }
    let apply = |x: f64, y: f64| (affine[0] * x + affine[2] * y + affine[4], affine[1] * x + affine[3] * y + affine[5]);
    let local: Vec<(f64, f64)> = match shape_at {
        None => {
            let (width, height) = (command[1].as_f64()?, command[2].as_f64()?);
            vec![(0.0, 0.0), (width, height)]
        }
        Some(index) => {
            let shape = command[index].as_array()?;
            let numbers: Vec<f64> = shape[1..].iter().filter_map(Value::as_f64).collect();
            match shape[0].as_str()? {
                "ci" => {
                    let [cx, cy, r] = [numbers[0], numbers[1], numbers[2]];
                    vec![(cx - r, cy - r), (cx + r, cy + r)]
                }
                "p" => shape[1].as_array()?.chunks(3).filter(|verb| verb.len() == 3).filter_map(|verb| Some((verb[1].as_f64()?, verb[2].as_f64()?))).collect(),
                _ => numbers.chunks(2).filter(|pair| pair.len() == 2).map(|pair| (pair[0], pair[1])).collect(),
            }
        }
    };
    let mut bounds: Option<(f64, f64, f64, f64)> = None;
    for (x, y) in local {
        let (sx, sy) = apply(x, y);
        bounds = Some(match bounds {
            None => (sx, sy, sx, sy),
            Some((x0, y0, x1, y1)) => (x0.min(sx), y0.min(sy), x1.max(sx), y1.max(sy)),
        });
    }
    bounds
}

/// 🙈️ With NO WebGPU adapter the frame must still carry a picture — a non-empty draw list that
/// actually covers the node layout, not an empty one the host silently drops.
///
/// This is the live defect this row pins: `wgpu`'s canvas bring-up bound a `webgpu` context to the
/// element BEFORE discovering there was no adapter, so the frame correctly fell back to
/// `present="2d"` onto a canvas that could never give a 2D context again, and the node graph stayed
/// blank forever. The fallback is only real if the list it hands over is one a replayer can paint
/// the graph from, at the places the host itself says the nodes are (`entity_screen_json`).
#[test]
fn without_a_gpu_adapter_the_frame_replays_a_list_that_covers_the_node_layout() {
    let (width, height) = (966_u32, 814_u32);
    let mut adapter = FlowDomainAdapter::default();
    let payload: Value = serde_json::from_str(GENERATION_3D_FLOW_FIXTURE).expect("generation3d flow fixture");
    let fixture = FlowHost::parse_fixture_json(&payload["fixture"].to_string()).expect("parsed flow fixture");
    let FlowDomainHost::Open(empty) = std::mem::replace(&mut adapter.host, FlowDomainHost::Open(FlowHost::from_fixture(fixture))) else { panic!("open flow host") };
    retire(empty);
    adapter.width = width;
    adapter.height = height;
    adapter.dpr = 1.0;
    adapter.host.set_viewport(width, height, 1.0);
    adapter.surface = Some(FlowSurface {
        id: SurfaceId::try_new(1).unwrap(),
        generation: SurfaceGeneration::try_new(1).unwrap(),
        metrics: CanvasMetrics::try_new(width, height, 1.0).unwrap(),
        state: SurfaceState::Ready,
    });

    let frame: Value = serde_json::from_slice(&adapter.render_frame().expect("render frame")).expect("render frame payload");
    assert_eq!(frame["present"], Value::from("2d"), "with no adapter the frame must take the encoded-draw-list exit");
    let commands = frame["draw"]["commands"].as_array().expect("commands").clone();
    assert!(!commands.is_empty(), "the deviceless exit handed the host an empty list — nothing could be replayed");

    let mut ink: Option<(f64, f64, f64, f64)> = None;
    for command in &commands {
        let Some((x0, y0, x1, y1)) = command_screen_bounds(command) else { continue };
        ink = Some(match ink {
            None => (x0, y0, x1, y1),
            Some((ax0, ay0, ax1, ay1)) => (ax0.min(x0), ay0.min(y0), ax1.max(x1), ay1.max(y1)),
        });
    }
    let ink = ink.expect("the draw list carried no drawable coordinates at all");
    assert!(ink.2 - ink.0 > f64::from(width) * 0.5 && ink.3 - ink.1 > f64::from(height) * 0.5, "the replayable picture {ink:?} covers less than half a {width}x{height} viewport");

    // 🎯️ Every node the host reports as on screen must have something drawn at its own screen rect:
    // that is what makes this a CAMERA law and not just a "the list is long" law.
    let widgets = frame["fixture"]["widgets"].as_array().expect("widgets").clone();
    let mut covered = 0_usize;
    for widget in &widgets {
        let id = widget["id"].as_str().expect("widget id");
        let geometry: Value = serde_json::from_str(&adapter.host.entity_screen_json("node", id)).expect("entity screen json");
        if geometry["visible"] != Value::Bool(true) {
            continue;
        }
        let rect: Vec<f64> = geometry["rect"].as_array().expect("entity rect").iter().filter_map(Value::as_f64).collect();
        let (rx0, ry0, rx1, ry1) = (rect[0], rect[1], rect[0] + rect[2], rect[1] + rect[3]);
        if rx1 < 0.0 || ry1 < 0.0 || rx0 > f64::from(width) || ry0 > f64::from(height) {
            continue;
        }
        let node_area = (rx1 - rx0) * (ry1 - ry0);
        let drawn_here = commands.iter().filter_map(command_screen_bounds).any(|(x0, y0, x1, y1)| {
            let inside = x0 >= rx0 - 8.0 && y0 >= ry0 - 8.0 && x1 <= rx1 + 8.0 && y1 <= ry1 + 8.0;
            inside && (x1 - x0) * (y1 - y0) >= node_area * 0.25
        });
        assert!(drawn_here, "nothing node-sized was drawn inside on-screen node {id} at {rect:?} — the replay would paint an empty canvas where the host says a node is");
        covered += 1;
    }
    assert!(covered > 0, "no node of the live fixture was on screen at {width}x{height}, so this law measured nothing");

    adapter.surface = None;
    let FlowDomainHost::Open(host) = std::mem::replace(&mut adapter.host, FlowDomainHost::Closed) else { panic!("open flow host") };
    retire(host);
}
//#endregion 🙈️DevicelessReplay

//#region 📷️OpeningCamera
/// 📷️ The live defect, as a law: the camera the generation3d document carries does NOT frame its own
/// graph in the 483x814 pane the Flow window actually gets, so the opening camera must be the fit —
/// and the fit must put every one of the seven widgets on screen.
///
/// Measured before the fix: `x=94.75 y=-97.5 zoom=1.78` showed the `extrude` node alone while the
/// minimap drew the whole seven-node column.
#[test]
fn the_generation_3d_document_camera_does_not_frame_its_own_graph_and_loses_to_the_fit() {
    let (width, height) = (483_u32, 814_u32);
    let mut host = generation_3d_flow_host();
    host.set_viewport(width, height, 1.0);

    let content = host.dag.content_world_bounds().expect("the generation3d graph has nodes to frame");
    let stored = canvas::camera::Camera { x: 94.755_815_717_374_45, y: -97.508_331_346_796_68, zoom: 1.784_432_561_601_11 };
    let viewport = canvas::camera::Viewport { width, height, dpr: 1.0 };
    let stored_coverage = canvas::camera::content_coverage(&content, &stored, &viewport);
    assert!(
        stored_coverage < canvas::camera::CONTENT_FRAMED_MIN_COVERAGE,
        "the document camera already frames {stored_coverage} of the graph — this law no longer measures the defect it was written for"
    );

    let fitted = host.dag.adopt_camera_or_fit(stored.x, stored.y, stored.zoom);
    assert!(fitted, "the opening camera kept a stored camera that shows {stored_coverage} of the graph");
    assert!((host.dag.camera_content_coverage() - 1.0).abs() < 1e-6, "after the fit the whole graph must be on screen");

    // 🎯️ Not "a number came out right": every widget the host can place has to land inside the pane.
    let mut on_screen = 0_usize;
    for widget in host.fixture.widgets.iter() {
        let id = semio_framework_artifact_flow_flow::widget_id_for(widget).to_string();
        let geometry: Value = serde_json::from_str(&host.entity_screen_json("node", &id)).expect("entity screen json");
        assert_eq!(geometry["visible"], Value::Bool(true), "{id} is not even placed");
        let rect: Vec<f64> = geometry["rect"].as_array().expect("entity rect").iter().filter_map(Value::as_f64).collect();
        let (x0, y0, x1, y1) = (rect[0], rect[1], rect[0] + rect[2], rect[1] + rect[3]);
        assert!(
            x0 >= 0.0 && y0 >= 0.0 && x1 <= f64::from(width) && y1 <= f64::from(height),
            "after the fit {id} is still off screen at {rect:?} in a {width}x{height} pane"
        );
        on_screen += 1;
    }
    assert_eq!(on_screen, 7, "the bundled example has seven widgets and the fit must frame all of them");
    retire(host);
}

/// 🔀️ An example switch that moves the graph out from under a live camera re-fits; an ordinary edit
/// under a camera that still shows the graph never does.
#[test]
fn a_graph_that_left_the_view_refits_and_one_that_did_not_is_left_alone() {
    let mut host = generation_3d_flow_host();
    host.set_viewport(483, 814, 1.0);
    host.dag.fit_camera_to_content();
    assert!(!host.dag.refit_camera_if_content_left_view(), "a framed graph must never be re-fitted under the viewer");

    let content = host.dag.content_world_bounds().expect("content");
    host.set_camera(content.max_x + 100_000.0, content.max_y + 100_000.0, 1.0);
    assert!(host.dag.camera_content_coverage() <= canvas::camera::CONTENT_REFIT_MAX_COVERAGE);
    assert!(host.dag.refit_camera_if_content_left_view(), "a graph nothing of which is on screen must be re-fitted");
    assert!((host.dag.camera_content_coverage() - 1.0).abs() < 1e-6);
    retire(host);
}
//#endregion 📷️OpeningCamera

//#region 🏷️NodeCaptions
/// 🏷️ The caption law over the REAL payload: at every zoom band that captions a node at all, the
/// caption is that node's NAME. Before the fix the `detail` band (which you reach by zooming IN past
/// `normal`) served `DagNodeSpec::abbreviation`, and generation3d abbreviates a document-derived
/// operator kind to its first letter — which is how `extrude` was drawn as `E`.
#[test]
fn every_captioned_zoom_band_names_the_node_instead_of_abbreviating_it() {
    let mut host = generation_3d_flow_host();
    host.set_viewport(483, 814, 1.0);
    // 🔡️ Exactly what the plugin publishes for a kind its catalogue does not carry
    // (`document_operator_records` in `🧊️generation3d/…/🪟️windows/🕸️flow/🦀️.rs`): the name is the
    // last kind segment and the abbreviation is its first letter.
    let records: Vec<ui_wgpu::wgpu::NodeGraphOperatorRecord> = ["brep.curve.polygon", "math.vector", "brep.solid.extrude"]
        .into_iter()
        .map(|kind| {
            let (extension, name) = kind.rsplit_once('.').expect("dotted kind");
            ui_wgpu::wgpu::NodeGraphOperatorRecord {
                id: kind.to_string(),
                extension: extension.to_string(),
                name: name.to_string(),
                abbreviation: name.chars().next().map(|ch| ch.to_uppercase().to_string()).unwrap_or_default(),
                icon: "box".into(),
                summary: String::new(),
                inputs: Vec::new(),
                outputs: Vec::new(),
                variadic_input: None,
                variadic_output: None,
                group: vec![extension.to_string()],
            }
        })
        .collect();
    host.set_neuron_kind_infos(&records);

    let mut captioned_bands = 0_usize;
    for zoom in [0.2_f64, 0.5, 0.9, 1.3, 1.784_432_561_601_11, 2.0, 3.0, 6.0] {
        host.set_camera(0.0, -120.0, zoom);
        let state: Value = serde_json::from_str(&host.label_overlay_paint_state_json().expect("label overlay state")).expect("label json");
        let titles: Vec<&str> = state["labels"]
            .as_array()
            .expect("rows")
            .iter()
            .filter(|row| row.get("kind").is_none())
            .filter_map(|row| row["text"].as_str())
            .collect();
        if titles.is_empty() {
            continue;
        }
        captioned_bands += 1;
        let lod = host.dag.draw_lod_label();
        for title in &titles {
            assert!(title.chars().count() > 1, "at zoom {zoom} (lod {lod}) the caption {title:?} is a single glyph — the abbreviation tier is back");
        }
        // 🔤️ `normalize_node_display` title-cases what the operator record carries, so the live
        // captions read `Extrude`/`Vector`/`Polygon` — never the `E`/`V`/`P` the abbreviation is.
        assert!(titles.contains(&"Extrude"), "at zoom {zoom} (lod {lod}) the extrude node is captioned {titles:?} instead of its name");
        assert!(titles.contains(&"Vector") && titles.contains(&"Polygon"), "at zoom {zoom} (lod {lod}) the captions are {titles:?}");
    }
    assert!(captioned_bands >= 4, "only {captioned_bands} zoom bands captioned anything, so this law measured almost nothing");
    retire(host);
}
/// 📐️ Every caption row carries its OWN screen-width budget, and a title drawn above the node body
/// is not clipped to the body: the computation layout puts it there (`computation_name_world_center`),
/// so a budget of one node width silently truncated every operator name to three or four glyphs.
#[test]
fn a_title_above_the_node_body_is_budgeted_wider_than_the_body() {
    let mut host = generation_3d_flow_host();
    host.set_viewport(483, 814, 1.0);
    host.dag.fit_camera_to_content();
    let state: Value = serde_json::from_str(&host.label_overlay_paint_state_json().expect("label overlay state")).expect("label json");
    let camera_zoom = state["camera"]["zoom"].as_f64().expect("camera zoom");
    let rows = state["labels"].as_array().expect("rows");
    let mut titles = 0_usize;
    let mut ports = 0_usize;
    for row in rows {
        let budget = row["maxScreenW"].as_f64().expect("every caption row carries its own width budget");
        assert!(budget > 0.0, "a caption budget of {budget} can never draw anything");
        if row.get("kind").is_some() {
            ports += 1;
            continue;
        }
        titles += 1;
        let node_width = row["nodeW"].as_f64().expect("nodeW");
        let body = node_width * camera_zoom * 0.88;
        let vertical = row["layout"] == Value::from("vertical");
        if !vertical {
            assert!(
                budget > body * 1.5,
                "a title drawn above the body was budgeted {budget} against a {body}-wide body — that is the truncation this law closes"
            );
        }
    }
    assert!(titles >= 6 && ports >= 6, "the live graph must contribute both titles ({titles}) and port labels ({ports})");
    retire(host);
}
//#endregion 🏷️NodeCaptions
