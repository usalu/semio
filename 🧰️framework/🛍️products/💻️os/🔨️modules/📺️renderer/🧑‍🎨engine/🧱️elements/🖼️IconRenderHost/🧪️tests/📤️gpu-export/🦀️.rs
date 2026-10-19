use super::{IconGpuPngExport, IconGpuPngRejected};
use ui_gpu::GpuContext;
use semio_framework_ui_prepared::PreparedRenderPacket;
use semio_framework_job::{Generation, JobOutcomeKind, OperationId, OriginalJobFixtureCaller, RetainedCloneGrant};
use semio_framework_ui_prepared::testing::{close_job_step, drain_abandoned_preparations, drive_preparation_until_terminal};
use semio_framework_ui_draw_contract::DrawList;
use semio_framework_ui_draw_contract::UiInstance;
use semio_framework_ui_prepared::{PreparedRenderInput, PreparedRenderJob};
use semio_framework_ui_theme::Rgba;

const TURN: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65536, maximum_capacity_bytes: 65536, maximum_release_bytes: 16 * 1024 * 1024, maximum_depth: 64 };
const STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 24, maximum_copy_bytes: usize::MAX / 4, maximum_capacity_bytes: usize::MAX / 4, maximum_release_bytes: usize::MAX / 4, maximum_depth: 64 };

fn clock() -> Option<u64> {
    Some(0)
}

pub fn caller() -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(OperationId(1), Generation(3), STOCK, clock, 1 << 40, 1 << 40)
}

pub fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/📤️gpu-export/🔣️.json")).unwrap()
}

pub fn packet(fixture: &serde_json::Value) -> PreparedRenderPacket {
    let mut draw = DrawList::default();
    for row in fixture["rectangles"].as_array().unwrap() {
        let bounds = std::array::from_fn(|i| row["bounds"][i].as_f64().unwrap() as f32);
        let rgba: [f32; 4] = std::array::from_fn(|i| row["rgba"][i].as_f64().unwrap() as f32 / 255.0);
        draw.layers[0].ui_instances.push(UiInstance::solid(bounds, Rgba::new(rgba[0], rgba[1], rgba[2], rgba[3])));
    }
    let input = PreparedRenderInput::try_new(7, 3, draw, None, 0.0).unwrap_or_else(|rejected| {
        let fault = rejected.fault().to_string();
        drop(rejected);
        drain_abandoned_preparations();
        panic!("{fault}");
    });
    let mut job = PreparedRenderJob::try_new(input).unwrap_or_else(|rejected| {
        let fault = rejected.fault().to_string();
        drop(rejected);
        drain_abandoned_preparations();
        panic!("{fault}");
    });
    assert_eq!(drive_preparation_until_terminal(&mut job), Some(JobOutcomeKind::Complete));
    let result = job.take_packet();
    for _ in 0..4096 {
        if close_job_step(&mut job) {
            break;
        }
    }
    result.expect("prepared fixture packet")
}

pub fn advance(caller: &mut OriginalJobFixtureCaller, job: &mut IconGpuPngExport) -> bool {
    caller.turn(TURN, |cx| Ok(job.advance(cx))).unwrap().unwrap()
}

pub fn close_rejected(caller: &mut OriginalJobFixtureCaller, rejected: &mut IconGpuPngRejected) {
    for _ in 0..4096 {
        if caller.turn(TURN, |cx| Ok(rejected.close_step(cx))).unwrap().unwrap() {
            return;
        }
    }
    panic!("rejected icon export did not retire within fixed test credits");
}

pub fn begin(caller: &mut OriginalJobFixtureCaller, source: &GpuContext, fixture: &serde_json::Value) -> IconGpuPngExport {
    match IconGpuPngExport::new(source, fixture["width"].as_u64().unwrap() as u32, fixture["height"].as_u64().unwrap() as u32, packet(fixture)) {
        Ok(job) => job,
        Err(mut rejected) => {
            close_rejected(caller, &mut rejected);
            panic!("{}", rejected.fault);
        }
    }
}

pub fn finish(caller: &mut OriginalJobFixtureCaller, job: &mut IconGpuPngExport) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while !advance(caller, job) {
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
    let mut caller = caller();
    let mut job = begin(&mut caller, &source, &fixture);
    finish(&mut caller, &mut job);
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
    caller.retire(TURN);
}

#[test]
fn icon_gpu_export_cancellation_retires_every_phase_without_publication() {
    let fixture = contract();
    let source = semio_framework_async::block_on(GpuContext::headless(1, 1)).unwrap();
    let mut caller = caller();
    for phase in fixture["cancel"].as_array().unwrap() {
        let target = phase.as_str().unwrap();
        let mut job = begin(&mut caller, &source, &fixture);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        while job.phase() != target {
            assert!(!advance(&mut caller, &mut job), "missed cancellation phase {target}");
            assert!(std::time::Instant::now() < deadline);
        }
        job.cancel();
        assert!(job.take_png().is_none());
        finish(&mut caller, &mut job);
        assert!(job.take_png().is_none());
    }
    let mut rejected = match IconGpuPngExport::new(&source, 0, 1, packet(&fixture)) {
        Ok(_) => panic!("zero-width export admitted"),
        Err(rejected) => rejected,
    };
    close_rejected(&mut caller, &mut rejected);
    caller.retire(TURN);
}
