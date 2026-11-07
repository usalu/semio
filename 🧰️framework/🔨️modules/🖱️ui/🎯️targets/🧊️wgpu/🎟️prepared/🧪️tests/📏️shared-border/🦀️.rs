use crate::*;
use semio_framework_job::{root_cancel_token, Generation, OperationId, StepBudget};
use semio_ui_draw_contract::{SharedBorderState, UiInstance};
use semio_ui_theme::{geometry::Rect, Rgba};

#[test]
fn shared_border_preparation_resolves_independently_appended_frames() {
    let _guard = testing::prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap();
    let array = |value: &serde_json::Value| -> [f32; 4] { std::array::from_fn(|i| value[i].as_f64().unwrap() as f32) };
    let mut draw = DrawList::default();
    for frame in fixture["frames"].as_array().unwrap() {
        let [x, y, w, h] = array(&frame["rect"]);
        let [r, g, b, a] = array(&frame["color"]);
        let state = match frame["state"].as_str().unwrap() { "active" => SharedBorderState::Active, "hover" => SharedBorderState::Hover, _ => panic!("Unknown fixture state") };
        let mut candidate = DrawList::default();
        candidate.push_shared_border(Rect::new(x, y, w, h), 1.0, Rgba { r, g, b, a }, state);
        draw.append_retained_candidate(&mut candidate).unwrap();
    }
    let input = PreparedRenderInput::new(1, 3, draw, None, 0.0);
    let mut job = PreparedRenderJob::new(input, 1);
    assert_eq!(testing::drive_preparation_until_terminal(&mut job), Some(semio_framework_job::JobOutcomeKind::Complete));
    let packet = job.take_packet().unwrap();
    let visible: Vec<&UiInstance> = packet.draw.layers.iter().flat_map(|layer| &layer.ui_instances).filter(|instance| instance.rect[2] > 0.0 && instance.rect[3] > 0.0 && instance.color[3] > 0.0).collect();
    assert_eq!(visible.len(), fixture["visibleEdges"].as_u64().unwrap() as usize);
    let shared: Vec<_> = visible.iter().filter(|instance| instance.rect == array(&fixture["sharedPaint"])).collect();
    assert_eq!(shared.len(), 1);
    assert_eq!(shared[0].color, array(&fixture["sharedColor"]));
    assert!(!visible.iter().any(|instance| instance.rect == array(&fixture["suppressedPaint"])));
    drop(packet);
    drop(job);
    testing::drain_abandoned_preparations();
    assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(std::sync::atomic::Ordering::Acquire), 0);
}

#[test]
fn closing_shared_border_preparation_releases_in_progress_backing() {
    let _guard = testing::prepared_process_guard();
    let mut draw = DrawList::default();
    for index in 0..32 {
        draw.push_shared_border(Rect::new(index as f32 * 10.0, 0.0, 10.0, 10.0), 1.0, Rgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, SharedBorderState::Hover);
    }
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(1, 3, draw, None, 0.0), 1);
    let mut preview = 0;
    let grant = ui_contract::UI_WORKER_RETIREMENT_POLICY;
    for _ in 0..4_096 {
        let (outcome, receipt) = testing::observe_prepared_step(&mut job, OperationId(1), Generation(3), StepBudget::new(100, 10, grant), root_cancel_token(), testing::now_ms, &mut preview, grant);
        assert!(receipt.fits(grant));
        assert!(matches!(outcome, None | Some(JobOutcomeKind::Yield)));
        if job.border_reserved_units > 0 { break; }
    }
    assert!(job.border_reserved_units > 0);
    assert!(matches!(job.section, PreparationSection::Borders));
    semio_framework_job::InteractiveJob::begin_close(&mut job);
    drop(job);
    testing::drain_abandoned_preparations();
    assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(std::sync::atomic::Ordering::Acquire), 0);
}
