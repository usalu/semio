use super::*;

/// 🧵️ Original wire and point leases submit empty surface domains without fabricating triangles or requesting another mesh.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn original_zero_index_geometry_submits_without_fabricated_surface_or_missing_mesh() {
    let _guard = prepared_process_guard();
    use crate::wgpu::kernel_3d_scene::*;
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🎬️scene/🧫️fixtures/🎯️component-source/🔣️.json")).unwrap();
    let law = &fixture["zeroIndex"]["gpuSubmission"];
    let width = law["size"][0].as_u64().unwrap() as u32;
    let height = law["size"][1].as_u64().unwrap() as u32;
    let revision = law["sceneRevision"].as_u64().unwrap();
    let generation = law["previewGeneration"].as_u64().unwrap();
    let limit = law["retirementTurns"].as_u64().unwrap() as usize;
    let mut gpu = match semio_framework_async::block_on(crate::wgpu::gpu::GpuContext::headless(width, height)) {
        Ok(gpu) => gpu,
        Err(error) => { assert!(error.starts_with("offscreen adapter:"), "{error}"); eprintln!("[DEBUG] originalZeroIndexGpu adapterUnavailable=true runtimeQualified=false"); return; }
    };
    let reference = tiny_skia::Pixmap::new(width, height).unwrap();
    let rgba: [u8; 4] = serde_json::from_value(law["rgba"].clone()).unwrap();
    assert!(reference.data().chunks_exact(4).all(|pixel| pixel == rgba));
    let grant = |capacity, release, work, depth| RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: work, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: depth };
    for (case_index, row) in fixture["zeroIndex"]["accepted"].as_array().unwrap().iter().enumerate() {
        let positions: Vec<[f32; 3]> = serde_json::from_value(row["positions"].clone()).unwrap();
        let vertex_ids: Vec<u32> = serde_json::from_value(row["vertexIds"].clone()).unwrap();
        let edges: Vec<[[f32; 3]; 2]> = serde_json::from_value(row["edges"].clone()).unwrap();
        let edge_ids: Vec<u32> = serde_json::from_value(row["edgeIds"].clone()).unwrap();
        let schema = Mesh3dSchema { vertex_ids: vertex_ids.len() as u32, edges: edges.len() as u32, edge_ids: edge_ids.len() as u32, ..Mesh3dSchema::triangle_mesh(positions.len() as u32, 0) };
        let birth = grant(mesh3d_begin_capacity_byte_demand(), 0, 0, 1);
        let (token, progress) = mesh3d_begin(generation, revision, schema, birth).unwrap();
        assert!(progress.fits(birth));
        let mut allocated = false;
        for _ in 0..limit { let credits = grant(mesh3d_allocate_capacity_byte_demand(token).unwrap(), 0, 0, 1); let step = mesh3d_allocate_step(token, credits).unwrap(); assert!(step.progress.fits(credits)); if step.complete { allocated = true; break; } }
        assert!(allocated);
        for position in positions { mesh3d_write_vec3(token, Mesh3dField::Positions, position).unwrap(); mesh3d_write_vec3(token, Mesh3dField::Normals, [0.0, 0.0, 1.0]).unwrap(); }
        for id in vertex_ids { mesh3d_write_u32(token, Mesh3dField::VertexIds, id).unwrap(); }
        for edge in edges { mesh3d_write_edge(token, edge).unwrap(); }
        for id in edge_ids { mesh3d_write_u32(token, Mesh3dField::EdgeIds, id).unwrap(); }
        let lease = mesh3d_seal(token).unwrap();
        let key = row["kind"].as_str().unwrap();
        let mut uploaded = false;
        let mut upload_error = None;
        for _ in 0..limit { match gpu.ensure_mesh_step(key, revision, lease) { Ok(true) => { uploaded = true; break; }, Ok(false) => {}, Err(error) => { upload_error = Some(error); break; } } }
        if !uploaded {
            for _ in 0..limit { if gpu.close_mesh_upload_step() { break; } }
            assert!(gpu.mesh_upload_terminal_is_empty());
            let mut retired = false;
            for _ in 0..limit { if gpu.retire_mesh_exact_step(key, revision).unwrap() { retired = true; break; } }
            assert!(retired);
            mesh3d_begin_close(lease).unwrap();
            for _ in 0..limit { let demand = mesh3d_close_demands(lease, MESH3D_PAGE_BYTES).unwrap(); let credits = grant(demand.capacity_bytes, demand.release_bytes, MESH3D_PAGE_BYTES, demand.depth); let step = mesh3d_close_step(lease, credits).unwrap(); assert!(step.progress.fits(credits)); if step.complete { break; } }
            assert!(mesh3d_terminal_is_empty(lease));
            for _ in 0..limit { if gpu.close_mesh_table_step() { break; } }
            assert!(gpu.mesh_table_terminal_is_empty());
            for _ in 0..limit { if gpu.close_raster_table_step().unwrap() { break; } }
            assert!(gpu.raster_table_terminal_is_empty());
            panic!("{key}: original GPU upload failed after exact owner closure: {upload_error:?}");
        }
        assert_eq!(gpu.mesh_store_mut().get_versioned(key, revision).unwrap().index_count, law["indexCount"].as_u64().unwrap() as u32);
        let mut answers = Vec::new();
        for (version_index, version_law) in law["versions"].as_array().unwrap().iter().enumerate() {
            let drawn_version = revision + version_law["offset"].as_u64().unwrap();
            let mut draw = DrawList::default();
            let instance = Instance3d { component_source: None, id: key.into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() };
            let surface = SceneDraw3d { mesh_key: key.into(), mesh_version: drawn_version, instances: vec![instance.clone()], ..Default::default() };
            draw.push_scene_pass(ScenePass3d { viewport: [0.0, 0.0, width as f32, height as f32], view_proj: Mat4::identity().to_cols_array_m(), draws: vec![surface.clone()], translucent_draws: vec![surface], material_draws: vec![SceneMaterialDraw3d { mesh_key: key.into(), mesh_version: drawn_version, first_index: 0, index_count: 0, instances: vec![instance], material: SceneMaterialKind3d::Standard, translucent: false }], ..Default::default() });
            let mut job = PreparedRenderJob::new(PreparedRenderInput::new(revision, generation, draw, None, 0.0), 1);
            let outcome = drive_preparation_until_terminal(&mut job);
            assert!(matches!(outcome, Some(semio_framework_job::JobOutcomeKind::Complete)));
            let mut packet = job.take_packet().unwrap();
            let mut paths = [0; 3];
            for index in 0..packet.command_pages().len() { match packet.command_pages().get(index).and_then(|command| command.draw_cursor()) { Some(DrawMeasureCursor::PassInstance { translucent: false, .. }) => paths[0] += 1, Some(DrawMeasureCursor::PassInstance { translucent: true, .. }) => paths[1] += 1, Some(DrawMeasureCursor::PassMaterialInstance { .. }) => paths[2] += 1, _ => {} } }
            assert_eq!(paths, [1; 3]);
            assert_eq!(law["drawPaths"], serde_json::json!(["opaque", "translucent", "material"]));
            let witness = crate::wgpu::draw::RasterTextureWitness { scene_revision: revision, preview_generation: generation, operation: (case_index * law["versions"].as_array().unwrap().len() + version_index) as u64 + 1 };
            gpu.begin_raster_ownership(witness).unwrap();
            gpu.seal_raster_ownership(witness).unwrap();
            let mut cursor = gpu.begin_prepared_present(&packet, witness).unwrap();
            let rendered = (|| -> Result<Vec<u8>, String> {
                let mut complete = false;
                for _ in 0..limit { if gpu.prepared_present_step(&packet, &mut cursor)? { complete = true; break; } }
                if !complete { return Err("original GPU presentation exceeded bounded turns".into()); }
                let mut readback = gpu.begin_prepared_readback(&cursor)?;
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
                while std::time::Instant::now() < deadline { if gpu.prepared_readback_step(&mut readback)? { return readback.take_rgba().ok_or_else(|| "original GPU readback was absent".into()); } std::thread::yield_now(); }
                readback.cancel();
                Err("original GPU readback deadline expired".into())
            })();
            let missing = gpu.take_missing_world_mesh();
            let mut committed = false;
            for _ in 0..limit { if gpu.commit_presented_rasters_step(witness).unwrap() { committed = true; break; } }
            assert!(committed);
            for _ in 0..limit { if cursor.close_step() { break; } }
            assert!(cursor.terminal_is_empty());
            let mut packet_closed = false;
            for _ in 0..limit { if close_packet_step(&mut packet) { packet_closed = true; break; } }
            assert!(packet_closed);
            for _ in 0..limit { if close_job_step(&mut job) { break; } }
            assert!(job.terminal_is_empty());
            answers.push((rendered, missing, drawn_version, version_law["missing"].as_bool().unwrap()));
        }
        for _ in 0..limit { if gpu.close_mesh_upload_step() { break; } }
        assert!(gpu.mesh_upload_terminal_is_empty());
        let mut retired = false;
        for _ in 0..limit { if gpu.retire_mesh_exact_step(key, revision).unwrap() { retired = true; break; } }
        assert!(retired);
        mesh3d_begin_close(lease).unwrap();
        for _ in 0..limit { let demand = mesh3d_close_demands(lease, MESH3D_PAGE_BYTES).unwrap(); let credits = grant(demand.capacity_bytes, demand.release_bytes, MESH3D_PAGE_BYTES, demand.depth); let step = mesh3d_close_step(lease, credits).unwrap(); assert!(step.progress.fits(credits)); if step.complete { break; } }
        assert!(mesh3d_terminal_is_empty(lease));
        for (rendered, missing, drawn_version, absent) in answers {
            assert_eq!(rendered.unwrap(), reference.data(), "{key}: actual submitted pixels match independent tiny-skia empty surface");
            assert_eq!(missing, absent.then(|| (key.to_string(), drawn_version)), "{key}: exact original residency distinguishes the accepted empty domain from an absent version");
        }
    }
    for _ in 0..limit { if gpu.close_mesh_table_step() { break; } }
    assert!(gpu.mesh_table_terminal_is_empty());
    for _ in 0..limit { if gpu.close_raster_table_step().unwrap() { break; } }
    assert!(gpu.raster_table_terminal_is_empty());
    eprintln!("[DEBUG] originalZeroIndexGpu actualDevice=true uploaded={} submittedPaths={} residentPaths={} absentVersionPaths={} readbackTinySkia=true noFabricatedTriangles=true exactResidencyMarkers=true exactOwnersClosed=true", fixture["zeroIndex"]["accepted"].as_array().unwrap().len(), fixture["zeroIndex"]["accepted"].as_array().unwrap().len() * 6, fixture["zeroIndex"]["accepted"].as_array().unwrap().len() * 3, fixture["zeroIndex"]["accepted"].as_array().unwrap().len() * 3);
}

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
    assert!(matches!(drive_preparation_until_terminal(&mut job), Some(semio_framework_job::JobOutcomeKind::Complete)));
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
    while !close_packet_step(&mut packet) {}
    while !close_job_step(&mut job) {}
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
    assert!(matches!(drive_preparation_until_terminal(&mut job), Some(semio_framework_job::JobOutcomeKind::Complete)));
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
    while !close_packet_step(&mut packet) {}
    while !close_job_step(&mut job) {}
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
    assert!(matches!(drive_preparation_until_terminal(&mut job), Some(semio_framework_job::JobOutcomeKind::Complete)));
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
    while !close_packet_step(&mut packet) {}
    while !close_job_step(&mut job) {}
    assert_eq!(order, expected, "a partial popup covers earlier panel text, and resumed parent content follows nested content");
}
