use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::step_ceiling;
use super::*;

#[test]
fn raster_to_png_asset_normalizes_valid_cells_only() {
    let mut raster = remodeling_geo::Raster::new(2, 2, 1.0, [0.0, 0.0]);
    raster.set(0, 0, 0.0);
    raster.set(1, 1, 10.0);
    let asset = raster_to_png_asset(&raster);
    assert_eq!(asset.mime, "image/png");
    assert_eq!(asset.width, 2);
    assert_eq!(asset.height, 2);
    assert!(!asset.data.is_empty());
}

#[test]
fn maximum_terminal_raster_png_is_worker_step_bounded() {
    let mut raster = remodeling_geo::Raster::new(512, 512, 1.0, [0.0, 0.0]);
    for index in 0..raster.values.len() {
        raster.values[index] = ((index * 7_919) % 65_521) as f32;
        raster.valid[index] = index % 11 != 0;
    }
    let chunks = std::thread::spawn(move || {
        let mut encoder = RasterPngPreparation::new(raster);
        let mut chunks = Vec::new();
        loop {
            let started = std::time::Instant::now();
            let progress = encoder.advance(4_096);
            step_ceiling::admit_step(started.elapsed(), format_args!("bounded raster PNG worker step exceeded 8 ms"));
            match progress {
                RasterPngProgress::Working => {}
                RasterPngProgress::Chunk(chunk) => {
                    assert!(chunk.len() <= RASTER_PNG_DURABLE_CHUNK_BYTES);
                    chunks.push(chunk);
                }
                RasterPngProgress::Complete => break chunks,
                RasterPngProgress::Failed => panic!("bounded raster PNG accounting failed"),
            }
        }
    })
    .join()
    .expect("raster encoder worker");
    let bytes = chunks.into_iter().flatten().collect::<Vec<_>>();
    let decoder = png::Decoder::new(bytes.as_slice());
    let mut reader = decoder.read_info().expect("bounded PNG is valid");
    let mut pixels = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut pixels).expect("bounded PNG decodes");
    assert_eq!((info.width, info.height), (512, 512));
}

#[test]
fn raster_digest_and_chunk_overflow_fail_without_wrapping() {
    let raster = remodeling_geo::Raster::new(1, 1, 1.0, [0.0, 0.0]);
    let mut digest = RasterPngPreparation::new(raster.clone());
    digest.phase = RasterPngPhase::Header;
    digest.digest_len = u64::MAX;
    assert!(matches!(digest.advance(1), RasterPngProgress::Failed));

    let mut chunks = RasterPngPreparation::new(raster);
    chunks.phase = RasterPngPhase::Header;
    chunks.chunk_count = u64::MAX;
    assert!(matches!(chunks.advance(1), RasterPngProgress::Failed));
}

#[test]
fn raster_png_portable_vectors_match_independent_decoder() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️raster-png.json")).expect("raster PNG vectors");
    for vector in fixture["vectors"].as_array().expect("vectors") {
        let id = vector["id"].as_str().expect("vector id");
        let mut raster = remodeling_geo::Raster::new(1, 1, 1.0, [0.0, 0.0]);
        raster.width = vector["width"].as_u64().expect("width") as u32;
        raster.height = vector["height"].as_u64().expect("height") as u32;
        if let Some(fill) = vector.get("fill") {
            let count = fill["count"].as_u64().expect("fill count") as usize;
            raster.values = vec![fill["value"].as_f64().expect("fill value") as f32; count];
            raster.valid = vec![fill["valid"].as_bool().expect("fill validity"); count];
        } else {
            raster.values = vector["values"].as_array().expect("values").iter().map(|value| value.as_f64().expect("value") as f32).collect();
            raster.valid = vector["valid"].as_array().expect("valid").iter().map(|value| value.as_bool().expect("validity")).collect();
        }
        let mut encoder = RasterPngPreparation::new(raster);
        let mut bytes = Vec::new();
        let mut failed = false;
        let mut completed = false;
        for _ in 0..64 {
            match encoder.advance(1) {
                RasterPngProgress::Working => {}
                RasterPngProgress::Chunk(chunk) => bytes.extend(chunk),
                RasterPngProgress::Complete => { completed = true; break; }
                RasterPngProgress::Failed => { failed = true; break; }
            }
        }
        assert_eq!(failed, vector["failed"].as_bool().unwrap_or(false), "{id}");
        if failed {
            assert!(bytes.is_empty(), "{id}: invalid raster emitted bytes");
            continue;
        }
        assert!(completed, "{id}: encoder did not terminate");
        let mut reader = png::Decoder::new(bytes.as_slice()).read_info().expect("independent PNG decoder");
        let mut pixels = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut pixels).expect("independent PNG pixels");
        assert_eq!(info.bit_depth, png::BitDepth::Sixteen, "{id}");
        let expected = vector["pixels"].as_array().expect("pixels").iter().flat_map(|value| (value.as_u64().expect("sample") as u16).to_be_bytes()).collect::<Vec<_>>();
        assert_eq!(&pixels[..info.buffer_size()], expected.as_slice(), "{id}");
    }
}
