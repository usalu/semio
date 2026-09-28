use super::*;
use semio_framework_job::{drive_step, root_cancel_token, Generation, InteractiveJob, InteractiveStage, OperationId, StepBudget, StepOutcome};
use ui_wgpu::wgpu::{DrawList, PreparedRenderInput, PreparedRenderJob};
use ui_wgpu::wgpu::{draw_types::UiInstance, theme::Rgba};

fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/📤️gpu-export/🔣️.json")).unwrap()
}

fn packet(fixture: &serde_json::Value) -> PreparedRenderPacket {
    let mut draw = DrawList::default();
    for row in fixture["rectangles"].as_array().unwrap() {
        let bounds = std::array::from_fn(|i| row["bounds"][i].as_f64().unwrap() as f32);
        let rgba: [f32; 4] = std::array::from_fn(|i| row["rgba"][i].as_f64().unwrap() as f32 / 255.0);
        draw.layers[0].ui_instances.push(UiInstance::solid(bounds, Rgba::new(rgba[0], rgba[1], rgba[2], rgba[3])));
    }
    let input = PreparedRenderInput::try_new(7, 3, draw, None, 0.0).unwrap_or_else(|mut rejected| {
        let fault = rejected.fault().to_string();
        while !rejected.close_step() {}
        panic!("{fault}");
    });
    let mut job = PreparedRenderJob::try_new(input).unwrap_or_else(|mut rejected| {
        let fault = rejected.fault().to_string();
        while !rejected.close_step() {}
        panic!("{fault}");
    });
    let mut preview = 0;
    let mut result = None;
    for _ in 0..4096 {
        let mut outcome = drive_step(&mut job, "icon-export-law", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), || Some(1), &mut preview, &mut None);
        if matches!(outcome, StepOutcome::Complete(_)) {
            result = job.take_packet();
            while outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) != semio_framework_job::JobPayloadCloseStep::Complete {}
            break;
        }
        assert!(matches!(outcome, StepOutcome::Yield));
    }
    InteractiveJob::begin_close(&mut job);
    while !PreparedRenderJob::close_step(&mut job) {}
    result.expect("prepared fixture packet")
}

fn begin(source: &GpuContext, fixture: &serde_json::Value) -> IconGpuPngExport {
    match IconGpuPngExport::new(source, fixture["width"].as_u64().unwrap() as u32, fixture["height"].as_u64().unwrap() as u32, packet(fixture)) {
        Ok(job) => job,
        Err(mut rejected) => {
            while !rejected.close_step() {}
            panic!("{}", rejected.fault);
        }
    }
}

fn finish(job: &mut IconGpuPngExport) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while !job.advance().unwrap() {
        assert!(job.take_png().is_none());
        assert!(std::time::Instant::now() < deadline, "export stalled at {:?}", job.progress());
        std::thread::yield_now();
    }
    assert!(job.terminal());
    assert!(job.gpu.mesh_table_terminal_is_empty());
    assert!(job.gpu.raster_table_terminal_is_empty());
    assert!(job.packet.is_none());
}

#[test]
fn icon_gpu_export_matches_canvas_pixels_and_delivers_once_after_retirement() {
    let fixture = contract();
    let source = semio_framework_async::block_on(GpuContext::headless(1, 1)).unwrap();
    let mut job = begin(&source, &fixture);
    finish(&mut job);
    let bytes = job.take_png().unwrap();
    let image = semio_framework_pixels::decode_png(&bytes).unwrap();
    assert_eq!((image.width, image.height), (fixture["width"].as_u64().unwrap() as u32, fixture["height"].as_u64().unwrap() as u32));
    for probe in fixture["probes"].as_array().unwrap() {
        let offset = (probe["position"][1].as_u64().unwrap() as usize * image.width as usize + probe["position"][0].as_u64().unwrap() as usize) * 4;
        let expected: Vec<u8> = probe["rgba"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u8).collect();
        assert_eq!(&image.pixels[offset..offset + 4], expected.as_slice(), "{probe}");
    }
    assert_eq!(fixture["deliveries"].as_u64(), Some(1));
    assert!(job.take_png().is_none());
    eprintln!("[DEBUG] icon GPU export produced {}x{} PNG with {} bytes and retired all packet resources", image.width, image.height, bytes.len());
}

#[test]
fn icon_gpu_export_cancellation_retires_every_phase_without_publication() {
    let fixture = contract();
    let source = semio_framework_async::block_on(GpuContext::headless(1, 1)).unwrap();
    for phase in fixture["cancel"].as_array().unwrap() {
        let target = phase.as_str().unwrap();
        let mut job = begin(&source, &fixture);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        while job.phase() != target {
            assert!(!job.advance().unwrap(), "missed cancellation phase {target}");
            assert!(std::time::Instant::now() < deadline);
        }
        job.cancel();
        assert!(job.take_png().is_none());
        finish(&mut job);
        assert!(job.take_png().is_none());
        eprintln!("[DEBUG] icon GPU export cancelled and retired from {target}");
    }
    let mut rejected = match IconGpuPngExport::new(&source, 0, 1, packet(&fixture)) {
        Ok(_) => panic!("zero-width export admitted"),
        Err(rejected) => rejected,
    };
    while !rejected.close_step() {}
}
