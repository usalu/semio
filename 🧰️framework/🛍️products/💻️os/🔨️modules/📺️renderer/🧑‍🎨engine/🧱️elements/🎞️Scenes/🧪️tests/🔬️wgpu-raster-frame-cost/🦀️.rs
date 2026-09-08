
use super::*;

fn pending_raster_len(surface_id: &str) -> usize {
    PENDING_RASTER_STATE.with(|cell| cell.borrow().get(surface_id).map_or(0, |surface| surface.queue.len()))
}

fn clear_pending_rasters(surface_id: &str) {
    PENDING_RASTER_STATE.with(|cell| {
        if let Some(surface) = cell.borrow_mut().get_mut(surface_id) {
            surface.queue.close_all();
            if let Some(reservation) = surface.admission.take() {
                let mut rejected = reservation.reject("test admission retirement", Vec::new());
                while !rejected.close_step() {}
            }
            if let Some(mut retiring) = surface.retiring.take() {
                while !retiring.close_step() {}
            }
            if let Some(mut rejected) = surface.rejected.take() {
                while !rejected.close_step() {}
            }
            if let Some(mut producer) = surface.closing.take() {
                while !producer.close_step() {}
            }
        }
    });
}

fn reset_pending_raster_authority() {
    PENDING_RASTER_STATE.with(|cell| {
        assert!(cell.borrow().terminal_is_empty());
        *cell.borrow_mut() = AdmittedSurfaceMap::default();
    });
    PENDING_RASTER_CLOSE_OWNER.with(|cell| assert!(cell.borrow().is_none()));
}

#[test]
fn pending_raster_ring_is_fixed_fifo_and_returns_cap_plus_one_owner() {
    let mut queue = PendingRasterQueue::default();
    let mut generations = Vec::new();
    for index in 0..RASTER_UPLOADS_PER_SURFACE_CAPACITY {
        let (producer, _) = PreparedRasterProducer::try_admit(format!("raster-{index}"), vec![index as u8; 4], 1, 1).expect("fixed queue admission");
        generations.push(producer.source_generation());
        queue.push_back(producer).ok().expect("fixed FIFO slot");
    }
    let (overflow, _) = PreparedRasterProducer::try_admit("overflow".into(), vec![9; 4], 1, 1).expect("ledger still owns the cap-plus-one producer");
    let overflow_generation = overflow.source_generation();
    let mut overflow = queue.push_back(overflow).expect_err("ring cap plus one returns exact producer");
    assert_eq!(overflow.source_generation(), overflow_generation);
    overflow.begin_close();
    while !overflow.close_step() {}
    for expected in generations {
        let token = queue.checkout_front().expect("FIFO checkout token");
        let mut producer = queue.take_checked_out(token).expect("FIFO producer owner");
        assert_eq!(producer.source_generation(), expected);
        producer.begin_close();
        while !producer.close_step() {}
    }
    assert_eq!(queue.len(), 0);
}

#[test]
fn checked_out_drop_hands_back_exact_fifo_owner_and_rejects_aba() {
    let surface_id = "raster-checkout-handback";
    let data_url = tiny_png_data_url(1, 2, 3);
    assert!(queue_canvas_image_upload(surface_id, "layer", &data_url).is_some());
    let mut cursor = PendingRasterUploadCursor::default();
    let checked = loop {
        match cursor.step() {
            PendingRasterUploadStep::Pending => {}
            PendingRasterUploadStep::Upload(checked) => break checked,
            _ => panic!("exact checkout"),
        }
    };
    let stale = PendingRasterQueueToken { slot: checked.queue.slot, epoch: checked.queue.epoch.saturating_sub(1) };
    assert!(!PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().get_token_mut(checked.surface).is_some_and(|surface| surface.queue.hand_back(stale))), "stale queue generation cannot reopen the checked-out slot");
    let generation = PENDING_RASTER_STATE.with(|cell| cell.borrow().get(surface_id).and_then(|surface| surface.queue.slots[usize::from(surface.queue.head)].as_ref()).map(PreparedRasterProducer::source_generation).unwrap());
    drop(checked);
    let mut second = PendingRasterUploadCursor::default();
    let checked = loop {
        match second.step() {
            PendingRasterUploadStep::Pending => {}
            PendingRasterUploadStep::Upload(checked) => break checked,
            _ => panic!("returned checkout"),
        }
    };
    let mut producer = checked.take().unwrap_or_else(|_| panic!("same owner remains in FIFO"));
    assert_eq!(producer.source_generation(), generation);
    producer.begin_close();
    while !producer.close_step() {}
}

#[test]
fn admission_saturation_runs_no_hash_dimension_or_pixel_materialization() {
    use std::cell::Cell;

    let mut reservations = Vec::with_capacity(256);
    for index in 0..256 {
        reservations.push(PreparedRasterReservation::try_reserve(format!("held-{index}")).expect("exact live reservation slot"));
    }
    let dimensions_called = Cell::new(false);
    let decode_called = Cell::new(false);
    let result = queue_canvas_image_upload_with(
        "raster-predecode-saturation",
        "layer",
        b"exact-borrowed-source",
        || {
            dimensions_called.set(true);
            Ok((1, 1, Vec::new()))
        },
        |_| {
            decode_called.set(true);
            Some(vec![0; 4])
        },
    );
    assert!(result.is_none());
    assert!(!dimensions_called.get());
    assert!(!decode_called.get());
    for reservation in reservations {
        let mut rejected = reservation.reject("test slot retirement", Vec::new());
        while !rejected.close_step() {}
    }
    clear_pending_rasters("raster-predecode-saturation");
}

#[test]
fn realm_close_retires_pending_rasters_before_terminal_and_allows_clean_reopen_fixture() {
    let surface_id = "raster-realm-close";
    let data_url = tiny_png_data_url(7, 8, 9);
    assert!(queue_canvas_image_upload(surface_id, "layer", &data_url).is_some());
    let mut close = begin_pending_raster_authority_close();
    let mut turns = 0usize;
    while !close.close_step() {
        turns += 1;
        assert!(turns < 200_000);
    }
    assert!(turns > 1, "realm close advances one retained owner per grant");
    assert!(close.terminal_is_empty());
    reset_pending_raster_authority();
}

fn count_solids(draw: &ui_wgpu::wgpu::DrawList) -> usize {
    draw.layers.iter().map(|layer| layer.ui_instances.len()).sum()
}

fn tiny_png_data_url(r: u8, g: u8, b: u8) -> String {
    let img = image::RgbaImage::from_pixel(1, 1, image::Rgba([r, g, b, 255]));
    let mut bytes: Vec<u8> = Vec::new();
    image::DynamicImage::ImageRgba8(img).write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).expect("encode tiny test png");
    format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD.encode(&bytes))
}

#[test]
fn queue_canvas_image_upload_requeues_stable_key_without_unbounded_digest() {
    let surface_id = "raster-frame-cost-test-unchanged";
    let data_url = tiny_png_data_url(10, 20, 30);
    let first = queue_canvas_image_upload(surface_id, "layer-a", &data_url);
    assert!(first.is_some());
    clear_pending_rasters(surface_id);
    let second = queue_canvas_image_upload(surface_id, "layer-a", &data_url);
    assert_eq!(first, second, "key must stay stable across frames");
    let pending = pending_raster_len(surface_id);
    assert_eq!(pending, 1, "the admitted source must requeue without a whole-buffer identity scan");
    clear_pending_rasters(surface_id);
}

#[test]
fn queue_canvas_image_upload_redecodes_when_source_changes() {
    let surface_id = "raster-frame-cost-test-changed";
    let png_a = tiny_png_data_url(10, 20, 30);
    let png_b = tiny_png_data_url(200, 100, 50);
    queue_canvas_image_upload(surface_id, "layer-a", &png_a);
    clear_pending_rasters(surface_id);
    queue_canvas_image_upload(surface_id, "layer-a", &png_b);
    let pending = pending_raster_len(surface_id);
    assert_eq!(pending, 1, "changed data_url must re-decode and queue exactly one upload");
    clear_pending_rasters(surface_id);
}

#[test]
fn draw_checkerboard_clamps_to_visible_viewport() {
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let viewport = Viewport { x: 0.0, y: 0.0, zoom: 1.0 };
    let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    let theme = Theme::default();
    draw_checkerboard(&mut draw, &viewport, inner, &theme, 4096.0);
    let quads = count_solids(&draw);
    assert!(quads > 0, "checkerboard should still draw the visible cells");
    assert!(quads < 4000, "checkerboard must clamp to the viewport instead of the full ±extent/2 grid, got {quads}");
}

#[test]
fn draw_checkerboard_falls_back_to_full_extent_when_zoom_is_zero() {
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let viewport = Viewport { x: 0.0, y: 0.0, zoom: 0.0 };
    let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    let theme = Theme::default();
    draw_checkerboard(&mut draw, &viewport, inner, &theme, 64.0);
    let quads = count_solids(&draw);
    assert_eq!(quads, 16, "degenerate zoom must fall back to the full extent grid (4x4 cells for a 64-unit extent)");
}

//#region Canvas2dDrawRecordTests
#[test]
fn canvas_layer_should_render_filters_meta_role_and_invisible_records() {
    let meta: CanvasLayer = serde_json::from_str(r#"{"role":"meta","utility":"selectDirect"}"#).unwrap();
    assert!(!canvas_layer_should_render(&meta), "role: meta records are non-visual bookkeeping");
    let hidden: CanvasLayer = serde_json::from_str(r#"{"kind":"rect","visible":false}"#).unwrap();
    assert!(!canvas_layer_should_render(&hidden));
    let visible: CanvasLayer = serde_json::from_str(r#"{"kind":"rect"}"#).unwrap();
    assert!(canvas_layer_should_render(&visible), "records default to visible when the field is absent");
}

#[test]
fn canvas_gradient_color_at_interpolates_between_bracketing_stops() {
    let stops = vec![CanvasGradientStopJson { offset: 0.0, color: Some(vec![0.0, 0.0, 0.0, 1.0]) }, CanvasGradientStopJson { offset: 1.0, color: Some(vec![1.0, 1.0, 1.0, 1.0]) }];
    let start = canvas_gradient_color_at(&stops, 0.0, 1.0);
    let mid = canvas_gradient_color_at(&stops, 0.5, 1.0);
    let end = canvas_gradient_color_at(&stops, 1.0, 1.0);
    assert!(start.r < 0.001, "t=0 should sample the first stop");
    assert!((mid.r - 0.5).abs() < 0.001, "t=0.5 should sit halfway between stops");
    assert!(end.r > 0.999, "t=1 should sample the last stop");
}

#[test]
fn canvas_apply_blend_mode_normal_and_none_are_passthrough() {
    let src = Rgba::new(0.2, 0.4, 0.6, 0.8);
    let backdrop = Rgba::new(0.9, 0.9, 0.9, 1.0);
    assert_eq!(canvas_apply_blend_mode(None, backdrop, src).r, src.r);
    assert_eq!(canvas_apply_blend_mode(Some("normal"), backdrop, src).b, src.b);
}

#[test]
fn canvas_apply_blend_mode_multiply_matches_w3c_formula() {
    let src = Rgba::new(0.5, 0.5, 0.5, 1.0);
    let backdrop = Rgba::new(0.4, 0.8, 1.0, 1.0);
    let blended = canvas_apply_blend_mode(Some("multiply"), backdrop, src);
    assert!((blended.r - 0.2).abs() < 0.001, "multiply(0.4,0.5) should be 0.2, got {}", blended.r);
    assert!((blended.g - 0.4).abs() < 0.001, "multiply(0.8,0.5) should be 0.4, got {}", blended.g);
    assert!((blended.b - 0.5).abs() < 0.001, "multiply(1.0,0.5) should be 0.5, got {}", blended.b);
}

#[test]
fn canvas_apply_blend_mode_screen_matches_w3c_formula() {
    let src = Rgba::new(0.5, 0.2, 0.0, 1.0);
    let backdrop = Rgba::new(0.5, 0.5, 1.0, 1.0);
    let blended = canvas_apply_blend_mode(Some("screen"), backdrop, src);
    let expected_r = 0.5 + 0.5 - 0.5 * 0.5;
    assert!((blended.r - expected_r).abs() < 0.001, "screen(a,b)=a+b-ab, got {}", blended.r);
}

#[test]
fn push_linear_gradient_fill_emits_banded_triangle_fan_geometry() {
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let viewport = Viewport { x: 0.0, y: 0.0, zoom: 1.0 };
    let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    let clip = Rect::new(50.0, 50.0, 100.0, 40.0);
    let fill = CanvasFillJson {
        kind: Some("linearGradient".into()),
        x1: 0.0,
        y1: 0.0,
        x2: 100.0,
        y2: 0.0,
        stops: vec![CanvasGradientStopJson { offset: 0.0, color: Some(vec![1.0, 0.0, 0.0, 1.0]) }, CanvasGradientStopJson { offset: 1.0, color: Some(vec![0.0, 0.0, 1.0, 1.0]) }],
        ..Default::default()
    };
    push_linear_gradient_fill(&mut draw, &viewport, inner, clip, 0.0, 0.0, &fill, 1.0, None, Theme::default().canvas_clear);
    let verts: usize = draw.layers.iter().map(|layer| layer.vector_vertices.len()).sum();
    assert!(verts > 0, "linear gradient bands should push triangle-fan geometry");
}

#[test]
fn push_radial_gradient_fill_emits_concentric_ring_geometry() {
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let viewport = Viewport { x: 0.0, y: 0.0, zoom: 1.0 };
    let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    let clip = Rect::new(0.0, 0.0, 80.0, 80.0);
    let fill = CanvasFillJson {
        kind: Some("radialGradient".into()),
        cx: 40.0,
        cy: 40.0,
        r: 40.0,
        stops: vec![CanvasGradientStopJson { offset: 0.0, color: Some(vec![1.0, 1.0, 1.0, 1.0]) }, CanvasGradientStopJson { offset: 1.0, color: Some(vec![0.0, 0.0, 0.0, 1.0]) }],
        ..Default::default()
    };
    push_radial_gradient_fill(&mut draw, &viewport, inner, clip, 0.0, 0.0, &fill, 1.0, None, Theme::default().canvas_clear);
    let verts: usize = draw.layers.iter().map(|layer| layer.vector_vertices.len()).sum();
    assert!(verts > 0, "radial gradient rings should push triangle-fan geometry");
}

#[test]
fn render_canvas_shape_fill_draws_solid_fill_and_stroke_for_plain_records() {
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let viewport = Viewport { x: 0.0, y: 0.0, zoom: 1.0 };
    let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    let shape_rect = Rect::new(10.0, 10.0, 40.0, 20.0);
    let layer: CanvasLayer = serde_json::from_str(r#"{"kind":"rect","fill":{"color":[0.1,0.2,0.3,1.0]},"stroke":{"color":[1.0,1.0,1.0,1.0],"width":2.0}}"#).unwrap();
    render_canvas_shape_fill(&mut draw, &viewport, inner, shape_rect, &layer, 1.0, Rgba::new(0.0, 0.0, 0.0, 1.0), Theme::default().canvas_clear, false);
    let solids = count_solids(&draw);
    let verts: usize = draw.layers.iter().map(|l| l.vector_vertices.len()).sum();
    assert!(solids > 0, "solid fill should push a rounded-rect instance");
    assert!(verts > 0, "stroke should push line geometry");
}
//#endregion Canvas2dDrawRecordTests

//#region Paint2dNavigatorTests
#[test]
fn paint2d_navigator_fit_viewport_centers_and_scales_to_content_bounds() {
    let flat = vec![Paint2dFlatLayer { x: 100.0, y: 50.0, scale_x: 1.0, scale_y: 1.0, width: 200, height: 100 }];
    let inner = Rect::new(0.0, 0.0, 100.0, 100.0);
    let viewport = paint2d_navigator_fit_viewport(&flat, inner);
    assert!((viewport.x - 100.0).abs() < 0.01, "camera should center on content x, got {}", viewport.x);
    assert!((viewport.y - 50.0).abs() < 0.01, "camera should center on content y, got {}", viewport.y);
    assert!((viewport.zoom - 0.26).abs() < 0.01, "zoom should fit the padded viewport to content, got {}", viewport.zoom);
}

#[test]
fn paint2d_navigator_fit_viewport_falls_back_to_neutral_camera_when_document_is_empty() {
    let viewport = paint2d_navigator_fit_viewport(&[], Rect::new(0.0, 0.0, 100.0, 100.0));
    assert_eq!(viewport.x, 0.0);
    assert_eq!(viewport.y, 0.0);
    assert_eq!(viewport.zoom, 1.0);
}

#[test]
fn paint2d_navigator_overlay_rect_maps_main_viewport_into_navigator_screen_space() {
    let content_camera_json = r#"{"x":0,"y":0,"zoom":1}"#;
    let content_viewport_json = r#"{"width":400,"height":300}"#;
    let navigator_viewport = Viewport { x: 0.0, y: 0.0, zoom: 0.5 };
    let navigator_inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    let overlay = paint2d_navigator_overlay_rect(content_camera_json, Some(content_viewport_json), &navigator_viewport, navigator_inner).expect("overlay rect should resolve when a composite viewport size is present");
    assert!((overlay.w - 200.0).abs() < 0.5, "overlay width should track the main viewport's world width, got {}", overlay.w);
    assert!((overlay.h - 150.0).abs() < 0.5, "overlay height should track the main viewport's world height, got {}", overlay.h);
}

#[test]
fn paint2d_navigator_overlay_rect_is_none_without_a_reported_composite_viewport() {
    let navigator_viewport = Viewport { x: 0.0, y: 0.0, zoom: 1.0 };
    let navigator_inner = Rect::new(0.0, 0.0, 200.0, 200.0);
    assert!(paint2d_navigator_overlay_rect("{}", None, &navigator_viewport, navigator_inner).is_none());
}
//#endregion Paint2dNavigatorTests
