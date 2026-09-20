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

/// 🫧 The terminal glass command page — the measured step that retires the section, carrying
/// `index == glass_regions.len()` — addresses no region and is not a stale cursor. Driven by the
/// `glassCommandPages` rows of the same neutral fixture.
#[test]
fn the_terminal_glass_command_page_addresses_no_region_and_is_not_stale() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖥️prepared-gpu-opportunity/🔣️.json")).expect("prepared gpu opportunity fixture");
    for row in fixture["glassCommandPages"]["rows"].as_array().expect("glass rows") {
        let region = usize::try_from(row["region"].as_u64().expect("region")).expect("region fits");
        let len = usize::try_from(row["len"].as_u64().expect("len")).expect("len fits");
        let addressed = row["addressed"].as_u64().map(|index| usize::try_from(index).expect("addressed fits"));
        match address_prepared_glass_region(region, len) {
            Ok(resolved) => {
                assert!(!row["stale"].as_bool().expect("stale"), "{row}");
                assert_eq!(resolved, addressed, "{row}");
            }
            Err(reported) => {
                assert!(row["stale"].as_bool().expect("stale"), "{row}");
                assert_eq!(reported, len, "{row}");
            }
        }
    }
}

/// 🐕️ LAW: every cursor the present ladder walks moves the watchdog signature.
///
/// 🩸️ `progress()` is the ONLY thing `AppPresentStallWatch` can see inside one `AppPresentPhase::Render`,
/// and a ladder index missing from it is indistinguishable from a frozen cursor. `ForegroundCommands`
/// was added without its index and the host quarantined the surface on every boot —
/// `presentation stalled: phase=Render engine=0 upload=1 gpu-cursor=Some((6, 13770, 13770, 5))`
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY).
#[test]
fn every_ladder_index_moves_the_watchdog_signature() {
    let _guard = guard();
    drain();
    let mut cursor = PreparedGpuPresentCursor::begin(7, 3).expect("fixed present cursor admission");
    let mut seen = cursor.progress();
    cursor.command += 1;
    assert_ne!(cursor.progress(), seen, "the draw command index is visible");
    seen = cursor.progress();
    cursor.glass_command += 1;
    assert_ne!(cursor.progress(), seen, "the glass command index is visible");
    seen = cursor.progress();
    cursor.foreground_command += 1;
    assert_ne!(cursor.progress(), seen, "the glass-foreground command index is visible");
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

/// 🫧 LAW: only a layer opened by `begin_glass_content` is split off the scene target.
///
/// A glass region samples the scene and paints over it, so content measured into the scene inside a
/// glass rect is blurred away by the very region that carries it — the window cap's `Puzzle 3D`
/// title was painted and then erased on every frame (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY).
#[test]
fn only_a_glass_content_layer_is_split_off_the_scene_target() {
    let theme = crate::wgpu::theme::Theme::default();
    let mut draw = crate::wgpu::draw_types::DrawList::default();
    draw.push_rounded([0.0, 0.0, 100.0, 40.0], theme.accent, 0.0);
    let region = draw.push_glass([0.0, 0.0, 100.0, 40.0], 0.0, theme.glass(crate::wgpu::theme::Level::Window));
    draw.begin_glass_content(region);
    draw.push_rounded([4.0, 4.0, 40.0, 16.0], theme.accent, 0.0);
    draw.end_glass_content();
    draw.push_rounded([0.0, 60.0, 100.0, 40.0], theme.accent, 0.0);

    let foreground_layers: Vec<usize> = draw.layers.iter().enumerate().filter(|(_, layer)| layer.foreground_of.is_some()).map(|(index, _)| index).collect();
    assert_eq!(foreground_layers.len(), 1, "exactly one layer is glass content");
    let glass_layer = foreground_layers[0];

    for (index, _) in draw.layers.iter().enumerate() {
        let cursor = DrawMeasureCursor::LayerUi { layer: index, item: 0, overlay: false };
        assert_eq!(prepared_draw_scalar_is_glass_foreground(&draw, cursor), index == glass_layer, "layer {index} classification");
    }
    assert!(!prepared_draw_scalar_is_glass_foreground(&draw, DrawMeasureCursor::LayerUi { layer: draw.layers.len(), item: 0, overlay: false }), "a stale layer index is never a foreground scalar");
    assert!(!prepared_draw_scalar_is_glass_foreground(&draw, DrawMeasureCursor::Glass(0)), "a glass region itself is not its own foreground");
    assert!(!prepared_draw_scalar_is_glass_foreground(&draw, DrawMeasureCursor::PassInstance { pass: 0, draw: 0, instance: 0, translucent: false }), "a scene pass with no layer is never a foreground scalar");
}

#[test]
fn overlay_rasters_are_encoded_in_the_foreground_phase() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔽️retained-select-overlay-raster/🔣️.json")).expect("retained Select/overlay raster fixture");
    let image = &fixture["image"];
    let key = image["sharedKey"].as_str().expect("shared raster key");
    let mut draw = crate::wgpu::draw_types::DrawList::default();
    draw.push_raster_quad(key, [0.0, 0.0, 8.0, 8.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    draw.begin_overlay_route();
    draw.push_raster_quad(key, [8.0, 0.0, 8.0, 8.0], [0.0, 0.0, 1.0, 1.0], 1.0);
    draw.end_overlay_route();

    for row in image["draws"].as_array().expect("draw phase rows") {
        let overlay = row["route"].as_str() == Some("overlay");
        let foreground = prepared_draw_scalar_is_glass_foreground(&draw, DrawMeasureCursor::LayerRaster { layer: 0, raster: 0, overlay });
        assert_eq!(if foreground { "foreground" } else { "scene" }, row["expectedPhase"].as_str().expect("expected phase"));
    }
}

/// ⚖️ LAW: **a window cap goes UNDER the veil.** Glass content whose own region is fully enclosed by a
/// LATER glass region — the introduction veil over a cap, a dialog over a floating panel — is encoded
/// into the SCENE the blur chain mips, not into the composite after the glass pass. React's veil
/// covers the whole shell except the card, and this renderer's caps used to stay crisp over it
/// (`📓️w8a-tour-crispness-and-symbol-glyphs.md` §5, hand-off 1).
///
/// Containment, not overlap, is the predicate: a menu that clips a panel's corner must not push that
/// panel's whole content into the backdrop, and a spotlight step's veil BANDS enclose nothing that
/// straddles them, which leaves the introduced element crisp exactly as React elevates it.
#[test]
fn glass_content_under_a_later_enclosing_region_is_encoded_into_the_scene() {
    let theme = crate::wgpu::theme::Theme::default();
    let mut draw = crate::wgpu::draw_types::DrawList::default();
    let cap = draw.push_glass([0.0, 0.0, 200.0, 24.0], 0.0, theme.glass(crate::wgpu::theme::Level::Window));
    draw.begin_glass_content(cap);
    draw.push_rounded([4.0, 4.0, 40.0, 16.0], theme.accent, 0.0);
    draw.end_glass_content();
    let cap_layer = draw.layers.iter().position(|layer| layer.foreground_of == Some(cap)).expect("the cap opened one content layer");
    let cap_cursor = DrawMeasureCursor::LayerUi { layer: cap_layer, item: 0, overlay: false };

    assert!(!prepared_foreground_scalar_is_enclosed(&draw, None, cap_cursor), "with nothing over it the cap's chips stay crisp");

    let mut veiled = crate::wgpu::draw_types::DrawList::default();
    veiled.push_glass([0.0, 0.0, 800.0, 600.0], 0.0, theme.veil_glass(crate::wgpu::theme::Level::Dialog));
    assert!(prepared_foreground_scalar_is_enclosed(&draw, Some(&veiled), cap_cursor), "a full-viewport veil in the OVERLAY list encloses a cap of the main list");

    let mut banded = crate::wgpu::draw_types::DrawList::default();
    banded.push_glass([0.0, 40.0, 800.0, 560.0], 0.0, theme.veil_glass(crate::wgpu::theme::Level::Dialog));
    assert!(!prepared_foreground_scalar_is_enclosed(&draw, Some(&banded), cap_cursor), "a veil band that starts below the cap encloses nothing of it");

    let mut clipped = crate::wgpu::draw_types::DrawList::default();
    clipped.push_glass([150.0, 0.0, 400.0, 300.0], 0.0, theme.glass(crate::wgpu::theme::Level::Menu));
    assert!(!prepared_foreground_scalar_is_enclosed(&draw, Some(&clipped), cap_cursor), "a menu that merely clips the cap's corner never blurs the whole cap");

    let card = draw.push_glass([300.0, 300.0, 200.0, 120.0], 0.0, theme.glass(crate::wgpu::theme::Level::Dialog));
    draw.begin_glass_content(card);
    draw.push_rounded([310.0, 310.0, 40.0, 16.0], theme.accent, 0.0);
    draw.end_glass_content();
    let card_layer = draw.layers.iter().position(|layer| layer.foreground_of == Some(card)).expect("the card opened one content layer");
    let card_cursor = DrawMeasureCursor::LayerUi { layer: card_layer, item: 0, overlay: false };
    assert!(!prepared_foreground_scalar_is_enclosed(&draw, None, card_cursor), "the LAST region's own content has nothing after it and stays crisp");
    assert!(!prepared_foreground_scalar_is_enclosed(&draw, None, cap_cursor), "a later region that does not enclose the cap leaves it crisp");

    assert!(prepared_glass_region_covers([0.0, 0.0, 10.0, 10.0], [0.0, 0.0, 10.0, 10.0]), "an exactly coincident region encloses");
    assert!(!prepared_glass_region_covers([0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]), "a degenerate region encloses nothing");
    assert!(!prepared_foreground_scalar_is_enclosed(&draw, None, DrawMeasureCursor::Glass(0)), "a glass region itself is never enclosed content");
    eprintln!("[DEBUG] glass enclosure: cap under veil = scene, cap under band/menu = composite");
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
    assert!(!prepared_draw_scalar_is_glass_foreground(&draw, cursor), "a textured scalar under an ordinary layer stays on the scene target");
    assert!(!prepared_draw_scalar_is_glass_foreground(&draw, DrawMeasureCursor::PassTexturedInstance { pass: draw.scene_passes.len(), draw: 0, instance: 0 }), "a stale pass index is never a foreground scalar");
}

#[test]
fn every_world_color_cursor_uses_the_encoded_attachment_in_scene_and_foreground_phases() {
    let theme = crate::wgpu::theme::Theme::default();
    let mut draw = crate::wgpu::draw_types::DrawList::default();
    draw.push_rounded([0.0, 0.0, 100.0, 40.0], theme.accent, 0.0);
    draw.push_scene_pass(crate::wgpu::kernel_3d_scene::ScenePass3d::default());
    let region = draw.push_glass([0.0, 0.0, 100.0, 40.0], 0.0, theme.glass(crate::wgpu::theme::Level::Window));
    draw.begin_glass_content(region);
    draw.push_scene_pass(crate::wgpu::kernel_3d_scene::ScenePass3d::default());
    draw.end_glass_content();

    for (pass, foreground) in [(0, false), (1, true)] {
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
            assert_eq!(prepared_draw_scalar_is_glass_foreground(&draw, cursor), foreground, "the encoded view follows the pass into its ordinary or glass-foreground target");
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
    assert!(source.contains("if world_encoded { scene.world_encoded_view() } else { scene.mip_view(0) }"));
    assert!(source.contains("if world_encoded { composite.world_encoded_view() } else { composite.view() }"));
    let draw_source = include_str!("../../🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs");
    assert!(draw_source.contains("let world_encoded_format = format.remove_srgb_suffix();"));
    assert!(draw_source.contains("scene_color_world_encoded"));
    assert!(draw_source.contains("prepared_composite_world_encoded_view"));
    assert_eq!(draw_source.matches("format: world_encoded_format, blend:").count(), 9, "standard, depth-writing standard translucent, painted, celebration, line and textured pipelines all target the encoded UNORM view");
    assert!(draw_source.contains("shadow: [if pass.shadow.enabled { 1.0 } else { 0.0 }, 0.0, 0.0, 1.0]"), "the legacy WGPU producer declares that its World attachment expects encoded output");
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
