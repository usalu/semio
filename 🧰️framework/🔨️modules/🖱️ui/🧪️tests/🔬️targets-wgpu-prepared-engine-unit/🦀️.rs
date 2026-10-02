use super::*;

/// 🖥️ LAW: grid uniforms stay in logical scene units at every device scale; only the
/// encoder viewport and scissor cross the logical-to-physical boundary.
#[test]
fn procedural_grid_uniforms_are_dpr_invariant_while_viewport_and_scissor_scale_physically() {
    let _guard = prepared_process_guard();
    use crate::wgpu::draw::{physical_scissor_rect, physical_viewport_rect, World3dGridUniforms};
    use crate::wgpu::draw_types::ScissorRect;
    use crate::wgpu::kernel_3d_scene::ProceduralGrid3d;
    let grid = ProceduralGrid3d { plane_z: 2.001, camera_plane_projection: [7.0, -4.0, 2.001], cell_size: 2.5, fade_distance: 18.0, cell_color: [0.2, 0.3, 0.4] };
    let at_one = World3dGridUniforms::from_grid(&grid);
    let at_two = World3dGridUniforms::from_grid(&grid);
    assert_eq!(size_of::<World3dGridUniforms>(), 256, "one fixed GPU uniform allocation owns the grid scalar");
    assert_eq!(bytemuck::bytes_of(&at_one), bytemuck::bytes_of(&at_two), "DPR cannot enter logical grid uniforms");
    assert_eq!(at_one.plane_cell, [2.001, 2.5, 0.6, 18.0]);
    assert_eq!(at_one.camera_fade, [7.0, -4.0, 2.001, 1.5]);
    assert_eq!(at_one.cell_color, [0.2, 0.3, 0.4, 0.0]);
    let viewport = [11.0, 13.0, 120.0, 80.0];
    assert_eq!(physical_viewport_rect(viewport, 1.0), viewport);
    assert_eq!(physical_viewport_rect(viewport, 2.0), [22.0, 26.0, 240.0, 160.0]);
    let scissor = ScissorRect { x: 11, y: 13, w: 120, h: 80 };
    assert_eq!(physical_scissor_rect(scissor, 1.0), scissor);
    assert_eq!(physical_scissor_rect(scissor, 2.0), ScissorRect { x: 22, y: 26, w: 240, h: 160 });
}

#[test]
fn a_scene_pass_is_prepared_between_the_ui_scalars_authored_around_it() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌌️prepared-scene-ui-stacking/🔣️.json")).expect("neutral scene/UI stacking fixture");
    let steps = fixture["steps"].as_array().expect("authored stacking steps");
    let mut draw = DrawList::default();
    for step in steps {
        let rect = std::array::from_fn(|index| step["rect"][index].as_f64().expect("rect scalar") as f32);
        let rgba = std::array::from_fn::<_, 4, _>(|index| step["rgba"][index].as_u64().expect("color scalar") as f32 / 255.0);
        match step["op"].as_str().expect("operation") {
            "solid" => draw.push_solid(rect, crate::wgpu::theme::Rgba::new(rgba[0], rgba[1], rgba[2], rgba[3])),
            "scene" => draw.push_scene_pass(crate::wgpu::kernel_3d_scene::ScenePass3d { viewport: rect, ..Default::default() }),
            op => panic!("unknown fixture operation {op}"),
        }
    }

    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
    assert!(matches!(drive_preparation_until_terminal(&mut job), StepOutcome::Complete(_)));
    let mut packet = job.take_packet().expect("accepted packet");
    let mut order = Vec::new();
    for index in 0..packet.command_pages().len() {
        let Some(command) = packet.command_pages().get(index) else { continue };
        let authored = match command.draw_cursor() {
            Some(DrawMeasureCursor::LayerUi { layer, item, overlay: false }) => {
                let value = &packet.draw.layers[layer].ui_instances[item];
                steps.iter().find(|step| step["op"] == "solid" && (0..4).all(|index| step["rect"][index].as_f64().unwrap() as f32 == value.rect[index]) && (0..4).all(|index| step["rgba"][index].as_u64().unwrap() as f32 / 255.0 == value.color[index]))
            }
            Some(DrawMeasureCursor::PassHeader(pass)) if pass < packet.draw.scene_passes.len() => {
                let value = &packet.draw.scene_passes[pass];
                steps.iter().find(|step| step["op"] == "scene" && (0..4).all(|index| step["rect"][index].as_f64().unwrap() as f32 == value.viewport[index]))
            }
            _ => None,
        };
        if let Some(step) = authored {
            order.push(step["id"].as_str().expect("step id").to_string());
        }
    }

    let width = fixture["size"][0].as_u64().expect("width") as u32;
    let height = fixture["size"][1].as_u64().expect("height") as u32;
    let mut reference = tiny_skia::Pixmap::new(width, height).expect("independent raster oracle");
    for step in steps {
        let mut paint = tiny_skia::Paint::default();
        paint.anti_alias = false;
        paint.set_color_rgba8(step["rgba"][0].as_u64().unwrap() as u8, step["rgba"][1].as_u64().unwrap() as u8, step["rgba"][2].as_u64().unwrap() as u8, step["rgba"][3].as_u64().unwrap() as u8);
        let rect =
            tiny_skia::Rect::from_xywh(step["rect"][0].as_f64().unwrap() as f32, step["rect"][1].as_f64().unwrap() as f32, step["rect"][2].as_f64().unwrap() as f32, step["rect"][3].as_f64().unwrap() as f32).expect("nondegenerate fixture rect");
        reference.fill_rect(rect, &paint, tiny_skia::Transform::identity(), None);
    }
    for sample in fixture["samples"].as_array().expect("pixel samples") {
        let color = reference.pixel(sample["at"][0].as_u64().unwrap() as u32, sample["at"][1].as_u64().unwrap() as u32).expect("sample in bounds");
        assert_eq!(serde_json::json!([color.red(), color.green(), color.blue(), color.alpha()]), sample["rgba"], "independent tiny-skia authored-order sample");
    }
    let expected = fixture["expectedOrder"].as_array().expect("expected order").iter().map(|id| id.as_str().expect("expected id").to_string()).collect::<Vec<_>>();
    while !packet.retire_step() {}
    while !job.close_step() {}
    assert_eq!(order, expected, "an opaque pane authored after World must remain after that exact scene pass in prepared commands");
}

#[test]
fn an_inline_overlay_follows_its_layer_scene_and_precedes_the_following_layer() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️prepared-scene-overlay-stacking/🔣️.json")).expect("neutral scene/overlay stacking fixture");
    let steps = fixture["steps"].as_array().expect("source stacking steps");
    let mut draw = DrawList::default();
    for step in steps {
        let rect = std::array::from_fn(|index| step["rect"][index].as_f64().expect("rect scalar") as f32);
        let rgba = std::array::from_fn::<_, 4, _>(|index| step["rgba"][index].as_u64().expect("color scalar") as f32 / 255.0);
        match (step["op"].as_str().expect("operation"), step["route"].as_str().expect("route")) {
            ("solid", "normal") => draw.push_solid(rect, crate::wgpu::theme::Rgba::new(rgba[0], rgba[1], rgba[2], rgba[3])),
            ("solid", "overlay") => {
                draw.begin_overlay_route();
                draw.push_solid(rect, crate::wgpu::theme::Rgba::new(rgba[0], rgba[1], rgba[2], rgba[3]));
                draw.end_overlay_route();
            }
            ("raster", "normal") => draw.push_raster_quad(step["rasterKey"].as_str().expect("raster key"), rect, [0.0, 0.0, 1.0, 1.0], 1.0),
            ("scene", "scene") => draw.push_scene_pass(crate::wgpu::kernel_3d_scene::ScenePass3d { viewport: rect, ..Default::default() }),
            (op, route) => panic!("unknown fixture operation {op}/{route}"),
        }
    }

    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
    assert!(matches!(drive_preparation_until_terminal(&mut job), StepOutcome::Complete(_)));
    let mut packet = job.take_packet().expect("accepted packet");
    let mut order = Vec::new();
    for index in 0..packet.command_pages().len() {
        let Some(command) = packet.command_pages().get(index) else { continue };
        let authored = match command.draw_cursor() {
            Some(DrawMeasureCursor::LayerUi { layer, item, overlay }) => {
                let values = if overlay { &packet.draw.layers[layer].overlay_ui_instances } else { &packet.draw.layers[layer].ui_instances };
                let value = &values[item];
                steps.iter().find(|step| {
                    step["op"] == "solid"
                        && (step["route"] == "overlay") == overlay
                        && (0..4).all(|index| step["rect"][index].as_f64().unwrap() as f32 == value.rect[index])
                        && (0..4).all(|index| step["rgba"][index].as_u64().unwrap() as f32 / 255.0 == value.color[index])
                })
            }
            Some(DrawMeasureCursor::LayerRaster { layer, raster, overlay: false }) => {
                let (key, value) = &packet.draw.layers[layer].raster_instances[raster];
                steps.iter().find(|step| step["op"] == "raster" && step["rasterKey"] == key.as_str() && (0..4).all(|index| step["rect"][index].as_f64().unwrap() as f32 == value.rect[index]))
            }
            Some(DrawMeasureCursor::PassHeader(pass)) if pass < packet.draw.scene_passes.len() => {
                let value = &packet.draw.scene_passes[pass];
                steps.iter().find(|step| step["op"] == "scene" && (0..4).all(|index| step["rect"][index].as_f64().unwrap() as f32 == value.viewport[index]))
            }
            _ => None,
        };
        if let Some(step) = authored {
            order.push(step["id"].as_str().expect("step id").to_string());
        }
    }

    let width = fixture["size"][0].as_u64().expect("width") as u32;
    let height = fixture["size"][1].as_u64().expect("height") as u32;
    let expected = fixture["expectedOrder"].as_array().expect("expected order");
    let mut reference = tiny_skia::Pixmap::new(width, height).expect("independent raster oracle");
    for id in expected {
        let step = steps.iter().find(|step| step["id"] == *id).expect("expected step exists");
        let mut paint = tiny_skia::Paint::default();
        paint.anti_alias = false;
        paint.set_color_rgba8(step["rgba"][0].as_u64().unwrap() as u8, step["rgba"][1].as_u64().unwrap() as u8, step["rgba"][2].as_u64().unwrap() as u8, step["rgba"][3].as_u64().unwrap() as u8);
        let rect =
            tiny_skia::Rect::from_xywh(step["rect"][0].as_f64().unwrap() as f32, step["rect"][1].as_f64().unwrap() as f32, step["rect"][2].as_f64().unwrap() as f32, step["rect"][3].as_f64().unwrap() as f32).expect("nondegenerate fixture rect");
        reference.fill_rect(rect, &paint, tiny_skia::Transform::identity(), None);
    }
    for sample in fixture["samples"].as_array().expect("pixel samples") {
        let color = reference.pixel(sample["at"][0].as_u64().unwrap() as u32, sample["at"][1].as_u64().unwrap() as u32).expect("sample in bounds");
        assert_eq!(serde_json::json!([color.red(), color.green(), color.blue(), color.alpha()]), sample["rgba"], "independent tiny-skia route-order sample");
    }
    let expected = expected.iter().map(|id| id.as_str().expect("expected id").to_string()).collect::<Vec<_>>();
    while !packet.retire_step() {}
    while !job.close_step() {}
    assert_eq!(order, expected, "an inline overlay follows its own layer's scene but stays behind a later covering layer");
}

#[test]
fn prepared_glass_and_foreground_follow_authored_partial_and_nested_stacking() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️prepared-glass-stacking/🔣️.json")).expect("neutral glass stacking fixture");
    let steps = fixture["steps"].as_array().expect("authored steps");
    let mut draw = DrawList::default();
    let mut current_glass = None;
    for step in steps {
        let rect = || std::array::from_fn(|index| step["rect"][index].as_f64().expect("rect scalar") as f32);
        let rgba = || std::array::from_fn::<_, 4, _>(|index| step["rgba"][index].as_u64().expect("color scalar") as f32 / 255.0);
        match step["op"].as_str().expect("operation") {
            "solid" => {
                let color = rgba();
                draw.push_solid(rect(), crate::wgpu::theme::Rgba::new(color[0], color[1], color[2], color[3]));
            }
            "glass" => {
                let color = rgba();
                current_glass = Some(draw.push_glass(rect(), 0.0, crate::wgpu::theme::GlassStyle { tint: crate::wgpu::theme::Rgba::new(color[0], color[1], color[2], color[3]), alpha: color[3], blur_px: 0.0, saturate: 1.0 }));
            }
            "begin" => draw.begin_glass_content(current_glass.expect("preceding glass")),
            "end" => draw.end_glass_content(),
            op => panic!("unknown fixture operation {op}"),
        }
    }
    let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
    assert!(matches!(drive_preparation_until_terminal(&mut job), StepOutcome::Complete(_)));
    let mut packet = job.take_packet().expect("accepted packet");
    let mut order = Vec::new();
    for index in 0..packet.command_pages().len() {
        let Some(command) = packet.command_pages().get(index) else { continue };
        let item = match command.draw_cursor() {
            Some(DrawMeasureCursor::LayerUi { layer, item, overlay: false }) => {
                let value = &packet.draw.layers[layer].ui_instances[item];
                Some((value.rect, value.color))
            }
            Some(DrawMeasureCursor::Glass(region)) if region < packet.draw.glass_regions.len() => {
                let value = &packet.draw.glass_regions[region];
                Some((value.rect, [value.tint.r, value.tint.g, value.tint.b, value.alpha]))
            }
            _ => None,
        };
        if let Some((rect, color)) = item {
            let step = steps
                .iter()
                .find(|step| step.get("id").is_some() && (0..4).all(|index| step["rect"][index].as_f64().unwrap() as f32 == rect[index]) && (0..4).all(|index| step["rgba"][index].as_u64().unwrap() as f32 / 255.0 == color[index]))
                .expect("every emitted draw has an authored identity");
            order.push(step["id"].as_str().unwrap().to_string());
        }
    }
    let width = fixture["size"][0].as_u64().unwrap() as u32;
    let height = fixture["size"][1].as_u64().unwrap() as u32;
    let mut reference = tiny_skia::Pixmap::new(width, height).expect("independent raster oracle");
    for step in steps.iter().filter(|step| step.get("rect").is_some()) {
        let mut paint = tiny_skia::Paint::default();
        paint.anti_alias = false;
        paint.set_color_rgba8(step["rgba"][0].as_u64().unwrap() as u8, step["rgba"][1].as_u64().unwrap() as u8, step["rgba"][2].as_u64().unwrap() as u8, step["rgba"][3].as_u64().unwrap() as u8);
        let rect = tiny_skia::Rect::from_xywh(step["rect"][0].as_f64().unwrap() as f32, step["rect"][1].as_f64().unwrap() as f32, step["rect"][2].as_f64().unwrap() as f32, step["rect"][3].as_f64().unwrap() as f32).unwrap();
        reference.fill_rect(rect, &paint, tiny_skia::Transform::identity(), None);
    }
    for sample in fixture["samples"].as_array().unwrap() {
        let color = reference.pixel(sample["at"][0].as_u64().unwrap() as u32, sample["at"][1].as_u64().unwrap() as u32).unwrap();
        assert_eq!(serde_json::json!([color.red(), color.green(), color.blue(), color.alpha()]), sample["rgba"], "independent tiny-skia stacking sample");
    }
    let expected = fixture["expectedOrder"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_string()).collect::<Vec<_>>();
    while !packet.retire_step() {}
    while !job.close_step() {}
    assert_eq!(order, expected, "a partial popup covers earlier panel text, and resumed parent content follows nested content");
}
