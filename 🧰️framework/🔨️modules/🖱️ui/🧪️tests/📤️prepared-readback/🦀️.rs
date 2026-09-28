use super::*;

fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/📤️prepared-readback/🔣️.json")).unwrap()
}

#[test]
fn prepared_readback_admits_dimensions_before_allocating() {
    let fixture = contract();
    assert_eq!(fixture["maxSide"].as_u64(), Some(u64::from(MAX_IMAGE_SIDE)));
    assert_eq!(fixture["maxPixels"].as_u64(), Some(MAX_IMAGE_PIXELS as u64));
    for row in fixture["admission"].as_array().unwrap() {
        let layout = PreparedReadbackLayout::new(row["width"].as_u64().unwrap() as u32, row["height"].as_u64().unwrap() as u32, false);
        assert_eq!(layout.is_ok(), row["accepted"].as_bool().unwrap(), "{row}");
        if let Ok(layout) = layout {
            assert_eq!(u64::from(layout.padded_bytes_per_row), row["paddedBytesPerRow"].as_u64().unwrap());
            assert_eq!(layout.byte_length, u64::from(layout.padded_bytes_per_row) * u64::from(layout.height));
        }
    }
    assert!(PreparedReadbackLayout::new(u32::MAX, u32::MAX, false).is_err());
}

#[test]
fn prepared_readback_normalizes_bounded_rows_without_padding_or_channel_loss() {
    for row in contract()["cases"].as_array().unwrap() {
        let layout = PreparedReadbackLayout::new(row["width"].as_u64().unwrap() as u32, row["height"].as_u64().unwrap() as u32, row["format"] == "bgra8unorm").unwrap();
        let expected: Vec<u8> = row["rgba"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect();
        let mut padded = vec![0xa5; layout.byte_length as usize];
        for (index, pixel) in expected.chunks_exact(4).enumerate() {
            let x = index % layout.width as usize;
            let y = index / layout.width as usize;
            let offset = y * layout.padded_bytes_per_row as usize + x * 4;
            let channels = if layout.bgra { [pixel[2], pixel[1], pixel[0], pixel[3]] } else { pixel.try_into().unwrap() };
            padded[offset..offset + 4].copy_from_slice(&channels);
        }
        let mut actual = Vec::new();
        while actual.len() < expected.len() {
            let before = actual.len();
            layout.append_rgba(&padded, &mut actual, 8).unwrap();
            assert!(actual.len() > before && actual.len() <= before + 8);
        }
        assert_eq!(actual, expected, "{}", row["id"]);
        assert!(layout.append_rgba(&padded[..padded.len() - 1], &mut Vec::new(), 8).is_err());
        assert!(layout.append_rgba(&padded, &mut vec![0], 8).is_err());
        assert!(layout.append_rgba(&padded, &mut Vec::new(), 3).is_err());
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn prepared_readback_reads_the_production_target_and_never_publishes_cancelled_bytes() {
    let (device, queue) = semio_framework_async::block_on(async {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::PRIMARY, ..wgpu::InstanceDescriptor::new_without_display_handle() });
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: None, force_fallback_adapter: false }).await.expect("readback law requires a GPU adapter");
        adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("prepared_readback_law"), required_features: wgpu::Features::empty(), required_limits: wgpu::Limits::default(), memory_hints: Default::default(), trace: wgpu::Trace::Off, experimental_features: Default::default(),
        }).await.expect("readback law device")
    });
    let validation = device.push_error_scope(wgpu::ErrorFilter::Validation);
    let fixture = contract();
    for row in fixture["cases"].as_array().unwrap() {
        let bgra = row["format"] == "bgra8unorm";
        let format = if bgra { wgpu::TextureFormat::Bgra8UnormSrgb } else { wgpu::TextureFormat::Rgba8UnormSrgb };
        let layout = PreparedReadbackLayout::new(row["width"].as_u64().unwrap() as u32, row["height"].as_u64().unwrap() as u32, bgra).unwrap();
        let mut target = None;
        crate::wgpu::draw::PreparedCompositeTarget::ensure(&device, &mut target, layout.width, layout.height, format);
        let target = target.unwrap();
        for pixel in row["rgba"].as_array().unwrap().chunks_exact(4) {
            let rgba: Vec<u8> = pixel.iter().map(|byte| byte.as_u64().unwrap() as u8).collect();
            let color = wgpu::Color { r: f64::from(rgba[0]) / 255.0, g: f64::from(rgba[1]) / 255.0, b: f64::from(rgba[2]) / 255.0, a: f64::from(rgba[3]) / 255.0 };
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
            drop(encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("readback_byte_fixture"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: target.world_encoded_view(), resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(color), store: wgpu::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None, multiview_mask: None,
            }));
            queue.submit(Some(encoder.finish()));
            let mut readback = PreparedGpuReadback::begin(&device, &queue, &target, layout).unwrap();
            assert!(readback.take_rgba().is_none());
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
            while !readback.advance(&device).unwrap() {
                assert!(std::time::Instant::now() < deadline, "readback completes without a blocking device wait");
                std::thread::yield_now();
            }
            let pixels = readback.take_rgba().unwrap();
            let expected = rgba.repeat((layout.width * layout.height) as usize);
            assert_eq!(pixels, expected);
            let mut png = semio_framework_pixels::png_encoding::PngEncodeJob::new(semio_framework_pixels::RasterImage { width: layout.width, height: layout.height, pixels }).unwrap();
            assert!(png.result().is_err());
            while !png.advance().unwrap().done {}
            let decoded = semio_framework_pixels::decode_png(png.result().unwrap()).unwrap();
            assert_eq!(decoded.pixels, expected);
            assert_eq!((decoded.width, decoded.height), (layout.width, layout.height));
            assert!(readback.take_rgba().is_none());
        }
        let mut cancelled = PreparedGpuReadback::begin(&device, &queue, &target, layout).unwrap();
        cancelled.cancel();
        assert!(cancelled.take_rgba().is_none());
        assert!(cancelled.advance(&device).is_err());
        assert_eq!(cancelled.progress().1, 0);
    }
    device.poll(wgpu::PollType::Poll).unwrap();
    assert!(semio_framework_async::block_on(validation.pop()).is_none());
}
