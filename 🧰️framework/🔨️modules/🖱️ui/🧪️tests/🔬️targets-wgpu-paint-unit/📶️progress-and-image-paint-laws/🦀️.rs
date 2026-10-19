// 📶️ Progress and image paint laws: the progress fraction and track paint and the image source decoding, upload identity, content rect, raster quads and placeholders.

//#region 📶️ProgressPaint
fn progress(completed: f64, total: Option<f64>) -> UiProgressNode {
    UiProgressNode { id: "progress".into(), completed, total, value_text: Label::data("progress"), presence: UiPresence::default(), menu: None }
}

/// 📶️ Every case of the shared progress law fills exactly the fraction it declares, on a centred
/// `--size-tiny` track — React's bar is `h-tiny w-full` (`🗣️Interpreter/🟦️.tsx`'s `ProgressView`),
/// and the same fraction sizes its fill.
#[test]
fn a_progress_bar_fills_the_fraction_the_shared_fixture_declares() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧬️contract/🧪️tests/🧫️fixtures/📶️progress.json")).expect("📶️ the progress fixture parses");
    let theme = Theme::default();
    let bounds = Rect::new(10.0, 20.0, 300.0, 40.0);
    for case in fixture["cases"].as_array().expect("progress cases") {
        let component = &case["component"];
        let node = progress(component["completed"].as_f64().expect("completed"), component["total"].as_f64());
        let (track, fill) = progress_bar_rects(&node, bounds, &theme);
        let height = semio_framework_ui_wgpu_layout::chrome::SIZE_TINY;
        assert_eq!(track, [10.0, 20.0 + (40.0 - height) * 0.5, 300.0, height], "{}: the track spans the bounds, vertically centred", case["id"]);
        match case["fraction"].as_f64() {
            Some(fraction) => assert_eq!(fill, [track[0], track[1], 300.0 * fraction as f32, track[3]], "{}: determinate fill", case["id"]),
            None => assert_eq!(fill, [10.0 + 100.0, track[1], 100.0, track[3]], "{}: a still, centred indeterminate sweep", case["id"]),
        }
    }
}

/// 🎨️ A progress bar paints with theme tokens only: the `separator` track, then the `accent` fill
/// React's own `bg-muted`/`bg-accent` pair resolves to — and an empty determinate bar paints no
/// zero-width fill instance at all.
#[test]
fn a_progress_bar_paints_its_track_and_fill_from_theme_tokens() {
    let theme = Theme::default();
    let bounds = Rect::new(0.0, 0.0, 200.0, 24.0);
    let colors = |node: &UiProgressNode| {
        let mut draw = DrawList::default();
        paint_progress(node, bounds, &theme, &mut draw);
        draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.color).collect::<Vec<_>>()
    };
    let token = |color: Rgba| [color.r, color.g, color.b, color.a];
    assert_eq!(colors(&progress(12.0, Some(100.0))), vec![token(theme.muted), token(theme.accent)]);
    assert_eq!(colors(&progress(3.0, None)), vec![token(theme.muted), token(theme.accent)]);
    assert_eq!(colors(&progress(0.0, Some(100.0))), vec![token(theme.muted)]);
    assert_ne!(token(theme.muted), token(theme.separator), "React's track is `bg-muted`, a distinct token from the separator stroke it stood in for until packet W2k");
}
//#endregion 📶️ProgressPaint

//#region 🖼️UiImagePaint
// 🖼️ W1n: `Component::Image`'s default (no-`SceneHost`) path used to paint a grey placeholder box
// forever — `paint_image` was `#[cfg(test)]`-gated and nothing decoded `src` at all. These cover the
// three halves of the fix: decode, the `object-contain` rect, and the raster quad the paint step emits.

fn encode_base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = u32::from(chunk[0]);
        let b1 = chunk.get(1).copied().map_or(0, u32::from);
        let b2 = chunk.get(2).copied().map_or(0, u32::from);
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(ALPHABET[(triple >> 18) as usize & 0x3F] as char);
        out.push(ALPHABET[(triple >> 12) as usize & 0x3F] as char);
        out.push(if chunk.len() > 1 { ALPHABET[(triple >> 6) as usize & 0x3F] as char } else { '=' });
        out.push(if chunk.len() > 2 { ALPHABET[triple as usize & 0x3F] as char } else { '=' });
    }
    out
}

/// 🖼️ A `data:image/png;base64` source of `width`x`height` solid opaque red, built through the same
/// owned codec the paint registry decodes with — the third-party-free round trip AGENTS.md asks for.
fn png_data_url(width: u32, height: u32) -> String {
    let mut image = semio_framework_pixels::RasterImage::new(width, height);
    for pixel in image.pixels.chunks_mut(4) {
        pixel.copy_from_slice(&[255, 0, 0, 255]);
    }
    let encoded = semio_framework_pixels::encode_png(&image).expect("owned png encode");
    format!("data:image/png;base64,{}", encode_base64(&encoded))
}

fn image_node(id: &str, src: &str) -> UiNode {
    UiNode::Image(UiImageNode { id: id.into(), src: src.into(), alt: Some(Label::data("alt text")), presence: UiPresence::default(), menu: None })
}

#[test]
fn a_data_url_png_source_decodes_to_its_natural_size() {
    let src = png_data_url(6, 3);
    assert_eq!(admit_ui_image(&src), UiImageAdmission::Ready);
    assert_eq!(ui_image_natural_size(&src), Some((6, 3)));
    assert_eq!(admit_ui_image(&src), UiImageAdmission::Ready, "re-admitting an already-decoded source is a lookup, not a second decode");
}

#[test]
fn an_admitted_source_publishes_exactly_one_pending_upload() {
    let src = png_data_url(2, 2);
    assert_eq!(admit_ui_image(&src), UiImageAdmission::Ready);
    let upload = std::iter::from_fn(take_ui_image_upload).find(|upload| upload.key == src).expect("the decoded source is queued for the raster table");
    assert_eq!((upload.width, upload.height), (2, 2));
    assert_eq!(upload.pixels.len(), 2 * 2 * 4, "RGBA8, the exact layout PreparedRasterProducer admits");
    assert_eq!(upload.pixels[..4], [255, 0, 0, 255]);
}

#[test]
fn a_url_source_is_deferred_to_the_host_and_a_jpeg_data_url_is_unsupported() {
    assert_eq!(admit_ui_image("https://example.test/picture.png"), UiImageAdmission::Deferred, "a fetch is the host asset pipeline's job, not this crate's");
    assert_eq!(admit_ui_image("assets/picture.png"), UiImageAdmission::Deferred);
    assert_eq!(admit_ui_image("data:image/jpeg;base64,/9j/4AAQ"), UiImageAdmission::Unsupported, "no third-party JPEG codec may enter the runtime graph");
    assert_eq!(admit_ui_image(""), UiImageAdmission::Refused);
}

/// 📐️ React's `<img className="max-h-64 max-w-full object-contain">` (`🗣️Interpreter/🟦️.tsx:1974`):
/// the box clamps per axis and the natural aspect ratio is letterboxed inside it, centred.
#[test]
fn the_image_content_rect_matches_react_object_contain() {
    let bounds = Rect::new(10.0, 20.0, 100.0, 100.0);
    let wide = ui_image_content_rect(bounds, 200, 100);
    assert_eq!((wide.w, wide.h), (100.0, 50.0), "a wide picture fills the width and letterboxes the height");
    assert_eq!(wide.x, 10.0);
    assert_eq!(wide.y, 20.0 + (100.0 - 50.0) * 0.5);

    let small = ui_image_content_rect(bounds, 40, 20);
    assert_eq!((small.w, small.h), (40.0, 20.0), "a picture smaller than its box is never upscaled");

    let tall = ui_image_content_rect(Rect::new(0.0, 0.0, 400.0, 1_000.0), 400, 800);
    assert_eq!(tall.h, UI_IMAGE_MAX_BOX_HEIGHT, "max-h-64 caps the box at 16rem no matter how tall the layout rect is");
    assert_eq!(tall.w, UI_IMAGE_MAX_BOX_HEIGHT / 2.0);
}

#[test]
fn a_decoded_image_paints_a_raster_quad_instead_of_the_placeholder() {
    let src = png_data_url(4, 2);
    assert_eq!(admit_ui_image(&src), UiImageAdmission::Ready);
    let (tree, root, theme, mut atlas) = setup(&image_node("picture", &src));
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    for _ in 0..8 {
        if paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor) == RetainedNodePaintStep::Complete {
            break;
        }
    }
    let rasters: Vec<_> = draw.layers.iter().flat_map(|layer| layer.raster_instances.iter()).collect();
    assert_eq!(rasters.len(), 1, "exactly one KIND_RASTER quad, keyed by src");
    assert_eq!(rasters[0].0, src);
    let layout = tree.accepted_layout(root).expect("laid out image");
    let expected = ui_image_content_rect(Rect::new(layout.x, layout.y, layout.width, layout.height), 4, 2);
    assert_eq!(rasters[0].1.rect, [expected.x, expected.y, expected.w, expected.h]);
    assert_eq!(draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).count(), 0, "no alt-text fallback once the bitmap is real");
}

#[test]
fn a_decoded_overlay_image_keeps_its_upload_identity_in_the_overlay_raster_lane() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🔽️retained-select-overlay-raster/🔣️.json")).expect("retained Select/overlay raster fixture");
    let image = &fixture["image"];
    let width = u32::try_from(image["decodedWidth"].as_u64().expect("decoded width")).expect("decoded width fits");
    let height = u32::try_from(image["decodedHeight"].as_u64().expect("decoded height")).expect("decoded height fits");
    let src = png_data_url(width, height);
    assert_eq!(admit_ui_image(&src), UiImageAdmission::Ready);
    let (tree, root, theme, mut atlas) = setup(&image_node("dialog-picture", &src));
    let mut draw = DrawList::default();
    let mut scene_cursor = RetainedNodePaintCursor::default();
    for _ in 0..8 {
        if paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut scene_cursor) == RetainedNodePaintStep::Complete {
            break;
        }
    }
    let mut cursor = RetainedNodePaintCursor::default();
    for _ in 0..8 {
        draw.begin_overlay_route();
        let step = paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor);
        draw.end_overlay_route();
        if step == RetainedNodePaintStep::Complete {
            break;
        }
    }
    let scenes: Vec<_> = draw.layers.iter().flat_map(|layer| layer.raster_instances.iter()).collect();
    let overlays: Vec<_> = draw.layers.iter().flat_map(|layer| layer.overlay_raster_instances.iter()).collect();
    assert_eq!(scenes.len(), 1);
    assert_eq!(overlays.len(), 1);
    assert_eq!(scenes[0].0, overlays[0].0, "normal and overlay draws retain one decoded upload key");
    assert_eq!(overlays[0].0, src);
    assert_eq!(ui_image_natural_size(&src), Some((width, height)), "routing does not alter decoded dimensions");
}

/// ⚖️ Law: the paint arm ADMITS the source itself. `admit_ui_image` had only test callers, so in
/// production the ledger stayed empty forever, `ui_image_natural_size` always answered `None`, and
/// EVERY `UiNode::Image` painted the `alt` placeholder however decodable its `src` was. No explicit
/// `admit_ui_image` here on purpose: this is exactly what a host that only paints does.
#[test]
fn the_paint_arm_admits_its_own_source_without_a_separate_admit_call() {
    let src = png_data_url(5, 5);
    assert_eq!(ui_image_natural_size(&src), None, "nothing has admitted this source yet");
    let (tree, root, theme, mut atlas) = setup(&image_node("self-admitting", &src));
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    for _ in 0..64 {
        if paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor) == RetainedNodePaintStep::Complete {
            break;
        }
    }
    assert_eq!(ui_image_natural_size(&src), Some((5, 5)), "the paint arm admitted the source");
    let rasters: Vec<_> = draw.layers.iter().flat_map(|layer| layer.raster_instances.iter()).collect();
    assert_eq!(rasters.len(), 1, "one KIND_RASTER quad, not the placeholder");
    assert_eq!(rasters[0].0, src);
    assert_eq!(draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).count(), 0, "no alt-text fallback once the bitmap is real");
    assert!(std::iter::from_fn(take_ui_image_upload).any(|upload| upload.key == src), "the decoded bitmap is queued for the host's raster-table drain");
}

#[test]
fn an_undecodable_source_still_paints_the_placeholder_and_alt_text() {
    let (tree, root, theme, mut atlas) = setup(&image_node("remote", "https://example.test/remote.png"));
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    for _ in 0..64 {
        if paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor) == RetainedNodePaintStep::Complete {
            break;
        }
    }
    assert_eq!(draw.layers.iter().flat_map(|layer| layer.raster_instances.iter()).count(), 0);
    assert!(draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01), "alt text is what React's broken <img> shows too");
}
//#endregion 🖼️UiImagePaint
