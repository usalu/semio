use super::*;

static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn guard() -> std::sync::MutexGuard<'static, ()> {
    match TEST_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn drain() {
    while !PreparedGpuPresentCursor::close_abandoned_step() {}
}

#[test]
fn interrupted_present_cursor_hands_back_generation_and_fixed_owners() {
    let _guard = guard();
    drain();
    let cursor = match PreparedGpuPresentCursor::begin(7, 3) {
        Some(cursor) => cursor,
        None => panic!("fixed present cursor admission"),
    };
    drop(cursor);
    assert!(!PreparedGpuPresentCursor::close_abandoned_step());
    assert!(PreparedGpuPresentCursor::close_abandoned_step());
    assert!(PREPARED_GPU_ABANDONMENT_STATE.iter().all(|state| state.load(Ordering::Acquire) == 0));
}

#[test]
fn present_cursor_generation_and_capacity_boundaries_refuse_before_ownership() {
    let _guard = guard();
    drain();
    assert!(PreparedGpuPresentCursor::begin(0, 3).is_none());
    assert!(PreparedGpuPresentCursor::begin(7, u64::MAX).is_none());
    let mut owners: [Option<PreparedGpuPresentCursor>; PREPARED_GPU_ABANDONMENT_SLOTS] = std::array::from_fn(|_| None);
    for owner in &mut owners {
        *owner = PreparedGpuPresentCursor::begin(7, 3);
        assert!(owner.is_some());
    }
    assert!(PreparedGpuPresentCursor::begin(7, 3).is_none());
    for owner in owners.iter_mut().filter_map(Option::as_mut) {
        owner.begin_close();
        while !owner.close_step() {}
        assert!(owner.terminal_is_empty());
    }
}

/// ⚖️ One over-ceiling opportunity is a MEASUREMENT; only a run of
/// `SUSTAINED_OVERRUN_QUARANTINE_STEPS` consecutive ones is terminal. Driven by the neutral fixture
/// `🖱️ui/🧫️fixtures/🖥️prepared-gpu-opportunity/🔣️.json`, whose `coldStart` row is the measured
/// first frame that used to quarantine the browser surface before it had ever presented.
#[test]
fn a_single_over_ceiling_gpu_opportunity_is_recorded_and_only_a_run_is_terminal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖥️prepared-gpu-opportunity/🔣️.json")).expect("prepared gpu opportunity fixture");
    assert_eq!(fixture["ceilingUs"].as_u64(), Some(PREPARED_GPU_OPPORTUNITY_CEILING_US));
    assert_eq!(fixture["sustainedOverrunOpportunities"].as_u64(), Some(u64::from(semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS)));
    let cold = fixture["coldStart"]["elapsedUs"].as_u64().expect("cold start sample");
    assert!(cold > PREPARED_GPU_OPPORTUNITY_CEILING_US);
    assert_eq!(admit_prepared_gpu_opportunity(0, cold), Ok(1));
    for row in fixture["runs"].as_array().expect("run rows") {
        let mut run = 0u32;
        let mut terminal = None;
        for sample in row["elapsedUs"].as_array().expect("run samples") {
            match admit_prepared_gpu_opportunity(run, sample.as_u64().expect("sample")) {
                Ok(next) => run = next,
                Err(reached) => {
                    terminal = Some(reached);
                    break;
                }
            }
        }
        assert_eq!(terminal.is_some(), row["terminal"].as_bool().expect("terminal"), "{row}");
        assert_eq!(u64::from(terminal.unwrap_or(run)), row["run"].as_u64().expect("run"), "{row}");
    }
}

/// 🫧 Every authored glass page addresses one live region; an out-of-range page is stale.
#[test]
fn every_glass_command_page_addresses_one_live_region() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖥️prepared-gpu-opportunity/🔣️.json")).expect("prepared gpu opportunity fixture");
    for row in fixture["glassCommandPages"]["rows"].as_array().expect("glass rows") {
        let region = usize::try_from(row["region"].as_u64().expect("region")).expect("region fits");
        let len = usize::try_from(row["len"].as_u64().expect("len")).expect("len fits");
        match address_prepared_glass_region(region, len) {
            Ok(resolved) => {
                assert!(!row["stale"].as_bool().expect("stale"), "{row}");
                assert_eq!(resolved as u64, row["addressed"].as_u64().expect("exact region"), "{row}");
            }
            Err(reported) => {
                assert!(row["stale"].as_bool().expect("stale"), "{row}");
                assert_eq!(reported, len, "{row}");
            }
        }
    }
}

/// 🐕️ Every advancing cursor remains visible to the presentation watchdog.
#[test]
fn every_ladder_index_moves_the_watchdog_signature() {
    let _guard = guard();
    drain();
    let mut cursor = PreparedGpuPresentCursor::begin(7, 3).expect("fixed present cursor admission");
    let mut seen = cursor.progress();
    cursor.command += 1;
    assert_ne!(cursor.progress(), seen, "the draw command index is visible");
    seen = cursor.progress();
    cursor.clip_piece += 1;
    assert_ne!(cursor.progress(), seen, "the bounded clip-piece index is visible");
    seen = cursor.progress();
    cursor.blur_mip += 1;
    assert_ne!(cursor.progress(), seen, "the blur mip is visible");
    seen = cursor.progress();
    cursor.phase = PreparedGpuPresentPhase::Present;
    assert_ne!(cursor.progress(), seen, "and so is the ladder phase");
    cursor.begin_close();
    while !cursor.close_step() {}
    assert!(cursor.terminal_is_empty(), "a closed cursor zeroes every index it walked");
}

fn clip_fixture_rect(value: &serde_json::Value) -> crate::wgpu::draw_types::ScissorRect {
    crate::wgpu::draw_types::ScissorRect {
        x: value[0].as_u64().expect("rect x") as u32,
        y: value[1].as_u64().expect("rect y") as u32,
        w: value[2].as_u64().expect("rect width") as u32,
        h: value[3].as_u64().expect("rect height") as u32,
    }
}

fn clip_fixture_pieces(row: &serde_json::Value, surface: crate::wgpu::draw_types::ScissorRect) -> Result<Vec<crate::wgpu::draw_types::ScissorRect>, ()> {
    let mut pieces = match row["clip"].as_array() {
        Some(values) => values.iter().map(clip_fixture_rect).collect::<Vec<_>>(),
        None => vec![surface],
    };
    for left in 0..pieces.len() {
        for right in left + 1..pieces.len() {
            let overlap = pieces[left].intersect(&pieces[right]);
            if overlap.w > 0 && overlap.h > 0 {
                return Err(());
            }
        }
    }
    let scissor = (!row["scissor"].is_null()).then(|| clip_fixture_rect(&row["scissor"]));
    let viewport = (!row["sceneViewport"].is_null()).then(|| clip_fixture_rect(&row["sceneViewport"]));
    pieces = pieces
        .into_iter()
        .map(|piece| piece.intersect(&surface))
        .map(|piece| scissor.map_or(piece, |scissor| piece.intersect(&scissor)))
        .map(|piece| viewport.map_or(piece, |viewport| piece.intersect(&viewport)))
        .filter(|piece| piece.w > 0 && piece.h > 0)
        .collect();
    Ok(pieces)
}

#[test]
fn prepared_gpu_color_commands_advance_one_bounded_clip_piece_at_a_time() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖥️prepared-gpu-clip-pieces/🔣️.json")).expect("neutral prepared GPU clip fixture");
    let surface = crate::wgpu::draw_types::ScissorRect { x: 0, y: 0, w: fixture["surface"][0].as_u64().expect("surface width") as u32, h: fixture["surface"][1].as_u64().expect("surface height") as u32 };
    for row in fixture["cases"].as_array().expect("clip cases") {
        let expected = &row["expected"];
        let pieces = clip_fixture_pieces(row, surface);
        assert_eq!(pieces.is_ok(), expected["accepted"].as_bool().expect("accepted"), "{}", row["id"]);
        let Ok(pieces) = pieces else { continue };
        let logical = pieces.iter().map(|piece| serde_json::json!([piece.x, piece.y, piece.w, piece.h])).collect::<Vec<_>>();
        assert_eq!(serde_json::Value::Array(logical), expected["logicalPieces"].clone(), "{}", row["id"]);
        let dpr = row["dpr"].as_f64().expect("DPR") as f32;
        let physical = pieces.iter().map(|piece| crate::wgpu::draw::physical_scissor_rect(*piece, dpr)).map(|piece| serde_json::json!([piece.x, piece.y, piece.w, piece.h])).collect::<Vec<_>>();
        assert_eq!(serde_json::Value::Array(physical), expected["physicalPieces"].clone(), "{}", row["id"]);
        let kind = row["kind"].as_str().expect("command kind");
        let color_encodes = if matches!(kind, "ui" | "world") { pieces.len() } else { 0 };
        assert_eq!(color_encodes as u64, expected["colorEncodes"].as_u64().expect("color encodes"), "{}", row["id"]);
        assert_eq!(u64::from(kind == "glass" && !pieces.is_empty()), expected["snapshots"].as_u64().expect("snapshots"), "{}", row["id"]);
        assert_eq!(if kind == "glass" { pieces.len() as u64 } else { 0 }, expected["glassComposites"].as_u64().expect("glass composites"), "{}", row["id"]);
        assert_eq!(u64::from(kind == "world"), expected["shadowBegins"].as_u64().expect("shadow begins"), "{}", row["id"]);
        assert_eq!(expected["commandAdvances"].as_u64(), Some(1), "{}", row["id"]);
        let mut draw = crate::wgpu::draw_types::DrawList::default();
        draw.layers[0].scissor = (!row["scissor"].is_null()).then(|| clip_fixture_rect(&row["scissor"]));
        draw.layers[0].clip = row["clip"].as_array().map(|values| crate::wgpu::draw_types::ClipRegion { scissors: values.iter().map(clip_fixture_rect).collect() });
        let cursor = match kind {
            "ui" => DrawMeasureCursor::LayerUi { layer: 0, item: 0, overlay: false },
            "glass" => {
                draw.glass_regions.push(crate::wgpu::draw_types::GlassRegion {
                    layer_index: 0,
                    rect: [0.0, 0.0, surface.w as f32, surface.h as f32],
                    radius: 0.0,
                    tint: crate::wgpu::theme::Rgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
                    alpha: 1.0,
                    blur_px: 8.0,
                    saturate: 1.0,
                });
                DrawMeasureCursor::Glass(0)
            }
            "world" => {
                draw.scene_passes.push(crate::wgpu::kernel_3d_scene::ScenePass3d { layer_index: 0, viewport: row["sceneViewport"].as_array().map(|viewport| [viewport[0].as_f64().unwrap() as f32, viewport[1].as_f64().unwrap() as f32, viewport[2].as_f64().unwrap() as f32, viewport[3].as_f64().unwrap() as f32]).unwrap(), ..Default::default() });
                DrawMeasureCursor::PassGrid { pass: 0 }
            }
            _ => unreachable!(),
        };
        let mut actual = Vec::new();
        let mut piece = 0usize;
        loop {
            match prepared_command_clip_piece(&draw, cursor, piece, surface.w as f32, surface.h as f32).expect("production clip-piece resolver") {
                PreparedCommandClipPiece::Scissor(scissor) => actual.push(scissor),
                PreparedCommandClipPiece::Empty => {}
                PreparedCommandClipPiece::Complete => break,
                PreparedCommandClipPiece::Unclipped => panic!("every color scalar has an exact clip piece"),
            }
            piece += 1;
            assert!(piece <= values_capacity_for_clip_law(row), "bounded clip progression");
        }
        assert_eq!(actual, pieces, "production clip progression matches the language-neutral oracle for {}", row["id"]);
        let mut oracle = tiny_skia::Pixmap::new(surface.w, surface.h).expect("independent clip raster oracle");
        let mut paint = tiny_skia::Paint::default();
        paint.anti_alias = false;
        paint.set_color_rgba8(255, 255, 255, 255);
        for piece in &pieces {
            let rect = tiny_skia::Rect::from_xywh(piece.x as f32, piece.y as f32, piece.w as f32, piece.h as f32).expect("nonempty clip piece");
            oracle.fill_rect(rect, &paint, tiny_skia::Transform::identity(), None);
        }
        for sample in row["samples"].as_array().expect("clip samples") {
            let pixel = oracle.pixel(sample["at"][0].as_u64().unwrap() as u32, sample["at"][1].as_u64().unwrap() as u32).expect("sample in surface");
            assert_eq!(pixel.alpha() > 0, sample["painted"].as_bool().expect("painted"), "{}", row["id"]);
        }
    }
}

fn values_capacity_for_clip_law(row: &serde_json::Value) -> usize {
    row["clip"].as_array().map_or(1, |pieces| pieces.len()).saturating_add(1)
}

#[test]
fn overlapping_silhouette_pieces_are_refused_before_retained_draw_ownership() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖥️prepared-gpu-clip-pieces/🔣️.json")).expect("neutral prepared GPU clip fixture");
    let row = fixture["cases"].as_array().expect("clip cases").iter().find(|row| row["id"] == "overlapping-pieces-refused-before-alpha").expect("overlap refusal case");
    let rects = row["clip"].as_array().expect("overlapping pieces").iter().map(|value| {
        let rect = clip_fixture_rect(value);
        crate::wgpu::geometry::Rect::new(rect.x as f32, rect.y as f32, rect.w as f32, rect.h as f32)
    }).collect::<Vec<_>>();
    let mut draw = crate::wgpu::draw_types::DrawList::default();
    draw.begin_retained_output(16, 4_096).expect("bounded retained output grant");
    draw.begin_silhouette_clip(&rects);
    assert_eq!(draw.finish_retained_output(), Err(crate::wgpu::draw_types::RetainedOutputError::LimitExceeded), "overlapping alpha pieces must be rejected before they can double-blend a pixel");
}

/// 🪟️ Glass insertion ends earlier content and retains the enclosing foreground.
#[test]
fn authored_glass_boundaries_preserve_the_enclosing_layer() {
    let theme = crate::wgpu::theme::Theme::default();
    let mut draw = crate::wgpu::draw_types::DrawList::default();
    draw.push_solid([0.0, 0.0, 100.0, 100.0], theme.accent);
    let panel = draw.push_glass([0.0, 0.0, 100.0, 100.0], 0.0, theme.glass(crate::wgpu::theme::Level::Window));
    draw.begin_glass_content(panel);
    draw.begin_silhouette_clip(&[crate::wgpu::geometry::Rect::new(0.0, 0.0, 80.0, 80.0)]);
    draw.push_scissor(crate::wgpu::geometry::Rect::new(4.0, 4.0, 60.0, 60.0));
    draw.push_solid([0.0, 0.0, 100.0, 20.0], theme.accent);
    let previous = draw.layers.last().expect("panel text layer");
    let scissor = previous.scissor;
    let clip = previous.clip.clone();
    let foreground = previous.foreground_of;
    let layer = draw.layers.len();
    let popup = draw.push_glass([40.0, 0.0, 80.0, 40.0], 0.0, theme.glass(crate::wgpu::theme::Level::Menu));
    assert_eq!(draw.glass_regions[popup].layer_index, layer);
    assert_eq!(draw.layers.len(), layer + 1);
    assert_eq!(draw.layers[layer].scissor, scissor);
    assert_eq!(draw.layers[layer].clip, clip);
    assert_eq!(draw.layers[layer].foreground_of, foreground);
    assert!(draw.layers[layer].ui_instances.is_empty());
    draw.pop_scissor();
    draw.end_silhouette_clip();
    draw.end_glass_content();
}

/// 🖼️ LAW: a TEXTURED world instance is encoded, and into the same target every other scene scalar
/// of its pass goes to.
///
/// 🩸️ `ScenePass3d::textured_draws` had been measured into command pages since the scene pass
/// existed — `advance_pipeline` walks `PassTextured`/`PassTexturedInstance`/`PassTexturedKey` and the
/// puzzle3d playground publishes 146 of those pages per frame — but `encode_prepared_draw_scalar`
/// had no arm for any of them, so the reference underlay could not paint on ANY target. Measured on
/// the live wgpu playground as `textured=1` on both World3d surfaces with nothing on the canvas
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W5c).
#[test]
fn a_textured_world_instance_is_encoded_into_its_pass_target() {
    let source = include_str!("../../🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs");
    assert!(source.contains("DrawMeasureCursor::PassTexturedInstance { pass, draw: draw_index, instance } =>"), "the ladder owns an arm for the textured instance cursor");
    assert!(source.contains("encode_prepared_world_textured("), "and that arm reaches the textured encoder");

    let theme = crate::wgpu::theme::Theme::default();
    let mut draw = crate::wgpu::draw_types::DrawList::default();
    draw.push_rounded([0.0, 0.0, 100.0, 40.0], theme.accent, 0.0);
    draw.push_scene_pass(crate::wgpu::kernel_3d_scene::ScenePass3d {
        viewport: [0.0, 0.0, 100.0, 40.0],
        textured_draws: vec![crate::wgpu::kernel_3d_scene::TexturedDraw3d {
            instances: vec![crate::wgpu::kernel_3d_scene::TexturedInstance3d {
                texture_key: "/reference.png".to_string(),
                model: crate::wgpu::kernel_3d_scene::Instance3d::model_from_trs([3.5, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [12.0, 8.0, 1.0]),
                background: [0.0; 4], appearance: [0.85, 0.0, 0.0, 0.0],
            }],
        }],
        ..Default::default()
    });
    assert_eq!(draw.scene_passes.len(), 1, "the pass carries its textured draw");
    let cursor = DrawMeasureCursor::PassTexturedInstance { pass: 0, draw: 0, instance: 0 };
    assert!(prepared_draw_scalar_uses_world_encoded_attachment(cursor), "the textured pass preserves its encoded color attachment");
}

#[test]
fn every_world_color_cursor_uses_the_encoded_composite_attachment() {
    let theme = crate::wgpu::theme::Theme::default();
    let mut draw = crate::wgpu::draw_types::DrawList::default();
    draw.push_rounded([0.0, 0.0, 100.0, 40.0], theme.accent, 0.0);
    draw.push_scene_pass(crate::wgpu::kernel_3d_scene::ScenePass3d::default());
    let region = draw.push_glass([0.0, 0.0, 100.0, 40.0], 0.0, theme.glass(crate::wgpu::theme::Level::Window));
    draw.begin_glass_content(region);
    draw.push_scene_pass(crate::wgpu::kernel_3d_scene::ScenePass3d::default());
    draw.end_glass_content();

    for pass in [0, 1] {
        let cursors = [
            DrawMeasureCursor::PassInstance { pass, draw: 0, instance: 0, translucent: false },
            DrawMeasureCursor::PassInstance { pass, draw: 0, instance: 0, translucent: true },
            DrawMeasureCursor::PassMaterialInstance { pass, draw: 0, instance: 0, translucent: false },
            DrawMeasureCursor::PassMaterialInstance { pass, draw: 0, instance: 0, translucent: true },
            DrawMeasureCursor::PassTexturedInstance { pass, draw: 0, instance: 0 },
            DrawMeasureCursor::PassLineVertex { pass, draw: 0, vertex: 1 },
        ];
        for cursor in cursors {
            assert!(prepared_draw_scalar_uses_world_encoded_attachment(cursor), "every World color family selects the encoded attachment");
        }
    }
    for cursor in [
        DrawMeasureCursor::LayerUi { layer: 0, item: 0, overlay: false },
        DrawMeasureCursor::LayerVector { layer: 0, item: 2, overlay: false },
        DrawMeasureCursor::LayerRaster { layer: 0, raster: 0, overlay: false },
        DrawMeasureCursor::PassShadowBegin(0),
        DrawMeasureCursor::PassShadowInstance { pass: 0, draw: 0, instance: 0 },
    ] {
        assert!(!prepared_draw_scalar_uses_world_encoded_attachment(cursor), "UI and depth-only cursors retain the linear sRGB attachment view");
    }

    let source = include_str!("../../🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs");
    assert!(source.contains("if world_encoded { composite.world_encoded_view() } else { composite.view() }"));
    let draw_source = include_str!("../../🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs");
    assert!(draw_source.contains("let world_encoded_format = format.remove_srgb_suffix();"));
    assert!(draw_source.contains("scene_color_world_encoded"));
    assert!(draw_source.contains("prepared_composite_world_encoded_view"));
    let encoded_world_pipelines = [
        "world3d_pipeline",
        "world3d_pipeline_translucent",
        "world3d_line_pipeline",
        "world3d_standard_translucent_pipeline",
        "world3d_painted_pipeline",
        "world3d_painted_pipeline_translucent",
        "world3d_celebration_pipeline",
        "world3d_celebration_pipeline_translucent",
        "world3d_textured_pipeline",
        "world3d_grid_pipeline",
    ];
    for label in encoded_world_pipelines {
        assert!(draw_source.contains(&format!("label: Some(\"{label}\")")), "the {label} encoded-color pipeline remains registered");
    }
    assert_eq!(draw_source.matches("format: world_encoded_format, blend:").count(), encoded_world_pipelines.len(), "standard, translucent, painted, celebration, line, textured and procedural-grid pipelines all target the encoded UNORM view exactly once");
    assert!(draw_source.contains("shadow: [if pass.shadow.enabled { 1.0 } else { 0.0 }, 0.0, 0.0, 1.0]"), "the WGPU producer declares that its World attachment expects encoded output");
}

/// 🫧 LAW: React's transparent `MeshStandardMaterial` remains front-sided and depth-writing; it is
/// distinct from overlay transparency and celebration's opacity-dependent depth policy.
#[test]
fn standard_translucent_world_pipeline_preserves_react_depth_and_side_policy() {
    let source = include_str!("../../🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs");
    let pipeline = source.split("label: Some(\"world3d_standard_translucent_pipeline\")").nth(1).expect("the Standard translucent pipeline");
    let pipeline = &pipeline[..pipeline.find("cache: None,").expect("the pipeline descriptor closes")];
    assert!(pipeline.contains("blend: Some(wgpu::BlendState::ALPHA_BLENDING)"));
    assert!(pipeline.contains("cull_mode: Some(wgpu::Face::Back)"), "React Standard defaults to FrontSide");
    assert!(pipeline.contains("depth_stencil: Some(material_depth(true))"), "React Standard keeps depthWrite=true when transparent");
}

/// 🖼️ LAW: the world textured pass is an UNDERLAY — it blends and never writes depth, and its quad
/// is a centred unit XY plane whose first texture row is its TOP.
///
/// A depth-writing underlay would punch its own semi-transparent quad through everything behind it,
/// and an upside-down `v` would mirror React's plan image. Both are invisible in a compile check and
/// expensive to see on a live boot, so they are pinned here.
#[test]
fn the_world_textured_pass_is_a_blended_underlay_on_a_centred_plane() {
    assert_eq!(size_of::<crate::wgpu::draw::World3dTexturedGpuInstance>(), 96, "the instance stride the shader contract declares");
    let vertices = crate::wgpu::draw::WORLD_PLANE_VERTICES;
    assert_eq!(vertices.len(), 30, "six vertices of position(3) + uv(2)");
    for chunk in vertices.chunks_exact(5) {
        assert!(chunk[0].abs() == 0.5 && chunk[1].abs() == 0.5 && chunk[2] == 0.0, "every corner is on the centred unit XY quad");
        assert_eq!(chunk[3], chunk[0] + 0.5, "u grows with +x");
        assert_eq!(chunk[4], 0.5 - chunk[1], "v grows with -y, so row 0 of the image is the TOP of the quad");
    }

    let source = include_str!("../../🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs");
    let pipeline = source.split("label: Some(\"world3d_textured_pipeline\")").nth(1).expect("the built textured pipeline");
    let pipeline = &pipeline[..pipeline.find("cache: None,").expect("the pipeline descriptor ends at its cache slot")];
    assert!(pipeline.contains("blend: Some(wgpu::BlendState::ALPHA_BLENDING)"), "the underlay composites over the ground");
    assert!(pipeline.contains("depth_write_enabled: Some(false)"), "and never writes depth");
    assert!(pipeline.contains("depth_compare: Some(wgpu::CompareFunction::LessEqual)"), "so geometry that already wrote depth still occludes it");
    assert!(pipeline.contains("cull_mode: None"), "React's reference plane is double-sided");
    assert!(pipeline.contains("array_stride: 20"), "position(3) + uv(2) per vertex");
}

/// ⏳️ Successful early transitions share timing admission; advancing does not excuse an overrun.
#[test]
fn every_successful_gpu_transition_is_measured_before_returning() {
    let _guard = guard();
    drain();
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖥️prepared-gpu-opportunity/🔣️.json")).unwrap();
    let mut cursor = PreparedGpuPresentCursor::begin(7, 3).unwrap();
    for transition in law["successfulTransitions"].as_array().unwrap() {
        cursor.overrun_run = transition["initialRun"].as_u64().unwrap() as u32;
        let mut ticks = [Some(10_000), Some(10_000 + transition["elapsedUs"].as_u64().unwrap())].into_iter();
        let complete = transition["complete"].as_bool().unwrap();
        let result = measure_prepared_gpu_opportunity(&mut cursor, None, || ticks.next().flatten(), |cursor| {
            cursor.command += 1;
            Ok(complete)
        });
        assert_eq!(result, Ok(complete), "{}", transition["id"]);
        assert_eq!(cursor.overrun_run, transition["expectedRun"].as_u64().unwrap() as u32, "{} resets the streak despite returning before a color encode", transition["id"]);
        assert!(ticks.next().is_none(), "every successful transition reads both clock edges");
    }
    cursor.overrun_run = 0;
    for (index, elapsed) in law["advancingOverruns"]["elapsedUs"].as_array().unwrap().iter().enumerate() {
        let mut ticks = [Some(10_000), Some(10_000 + elapsed.as_u64().unwrap())].into_iter();
        let result = measure_prepared_gpu_opportunity(&mut cursor, Some((17, Some(DrawMeasureCursor::LayerUi { layer: 0, item: index, overlay: false }))), || ticks.next().flatten(), |cursor| {
            cursor.command += 1;
            Ok(false)
        });
        let terminal = index as u64 + 1 == law["advancingOverruns"]["terminalAt"].as_u64().unwrap();
        assert_eq!(result.is_err(), terminal, "advancing color work still obeys the two millisecond ceiling");
        if let Err(error) = result {
            assert!(error.contains("4 consecutive opportunities"));
            assert!(error.contains("command_kind=Some(17)") && error.contains("LayerUi") && error.contains("progress="), "the fault identifies its exact work: {error}");
        }
    }
    cursor.begin_close();
    while !cursor.close_step() {}
}
