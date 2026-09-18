
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
