
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
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "bounded raster PNG worker step exceeded 8 ms");
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
