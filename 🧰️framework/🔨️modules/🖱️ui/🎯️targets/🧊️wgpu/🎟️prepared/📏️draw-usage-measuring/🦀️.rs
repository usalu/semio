//! 📏️ Draw usage measuring: the draw usage census and the layer, scene pass, shadow and material cursor steps. See [`super`].

use super::*;

//#region ⚙️PreparationJob
impl PreparedRenderJob {
    pub fn next_draw_usage(draw: &DrawList, cursor: &mut DrawMeasureCursor) -> Option<PreparedRenderUsage> {
        let (usage, next) = match *cursor {
            DrawMeasureCursor::LayerHeader(layer) => {
                let Some(value) = draw.layers.get(layer) else {
                    *cursor = DrawMeasureCursor::Complete;
                    return Some(PreparedRenderUsage::default());
                };
                let _ = value;
                let region = draw.glass_regions.partition_point(|region| region.layer_index < layer);
                let next = if draw.glass_regions.get(region).is_some_and(|region| region.layer_index == layer) { DrawMeasureCursor::Glass(region) } else { Self::layer_channel_cursor(draw, layer, 0) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<DrawLayer>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::LayerUi { layer, item, overlay } => {
                let value = &draw.layers[layer];
                let items = if overlay { &value.overlay_ui_instances } else { &value.ui_instances };
                let next = if item + 1 < items.len() { DrawMeasureCursor::LayerUi { layer, item: item + 1, overlay } } else { Self::layer_channel_cursor(draw, layer, if overlay { 4 } else { 1 }) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_draw_contract::UiInstance>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::LayerVector { layer, item, overlay } => {
                let value = &draw.layers[layer];
                let items = if overlay { &value.overlay_vector_vertices } else { &value.vector_vertices };
                let next = if item + 1 < items.len() { DrawMeasureCursor::LayerVector { layer, item: item + 1, overlay } } else { Self::layer_channel_cursor(draw, layer, if overlay { 5 } else { 2 }) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_draw_contract::VectorVertex>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::LayerRaster { layer, raster, overlay } => {
                let instances = if overlay { &draw.layers[layer].overlay_raster_instances } else { &draw.layers[layer].raster_instances };
                let value = &instances[raster];
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_draw_contract::UiInstance>(), ..PreparedRenderUsage::default() };
                let next = if value.0.is_empty() { Self::next_layer_raster(draw, layer, raster, overlay) } else { DrawMeasureCursor::LayerRasterKey { layer, raster, byte: 0, overlay } };
                (usage, next)
            }
            DrawMeasureCursor::LayerRasterKey { layer, raster, byte, overlay } => {
                let instances = if overlay { &draw.layers[layer].overlay_raster_instances } else { &draw.layers[layer].raster_instances };
                let key = &instances[raster].0;
                let next = if byte + 1 < key.len() { DrawMeasureCursor::LayerRasterKey { layer, raster, byte: byte + 1, overlay } } else { Self::next_layer_raster(draw, layer, raster, overlay) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassHeader(pass) => {
                let Some(value) = draw.scene_passes.get(pass) else {
                    *cursor = DrawMeasureCursor::Complete;
                    return Some(PreparedRenderUsage::default());
                };
                let next = if value.viewport_mask == semio_framework_ui_scene_pass::SceneViewportMask3d::Ellipse { DrawMeasureCursor::PassBackdropSnapshot { pass } } else { Self::next_after_scene_prelude(draw, pass) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::ScenePass3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassBackdropSnapshot { pass } => (PreparedRenderUsage { draw_items: 1, ..PreparedRenderUsage::default() }, Self::next_after_scene_prelude(draw, pass)),
            DrawMeasureCursor::PassSceneClear { pass } => {
                let next = if draw.scene_passes[pass].shadow.enabled && draw.scene_passes[pass].shadow_draws.iter().any(|draw| draw.shadow_role.casts && !draw.instances.is_empty()) {
                    DrawMeasureCursor::PassShadowBegin(pass)
                } else {
                    Self::next_after_shadow(draw, pass)
                };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<[f32; 4]>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassShadowBegin(pass) => {
                let next = Self::first_shadow_instance(draw, pass).unwrap_or_else(|| Self::next_after_shadow(draw, pass));
                (PreparedRenderUsage { draw_items: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassShadowInstance { pass, draw: draw_index, instance } => {
                let draw_value = &draw.scene_passes[pass].shadow_draws[draw_index];
                let instance_value = &draw_value.instances[instance];
                let next = Self::next_shadow_instance(draw, pass, draw_index, instance);
                let draw_bytes = size_of::<semio_framework_ui_scene_mesh::Instance3d>() + instance_value.id.len() + if instance == 0 { size_of::<semio_framework_ui_scene_pass::SceneDraw3d>() + draw_value.mesh_key.len() } else { 0 };
                (PreparedRenderUsage { draw_items: 1, draw_bytes, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassDraw { pass, draw: draw_index, translucent } => {
                let pass_value = &draw.scene_passes[pass];
                let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
                let value = &draws[draw_index];
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::SceneDraw3d>(), ..PreparedRenderUsage::default() };
                let next = if !value.mesh_key.is_empty() {
                    DrawMeasureCursor::PassDrawKey { pass, draw: draw_index, byte: 0, translucent }
                } else if value.instances.is_empty() {
                    Self::next_pass_draw(draw, pass, draw_index, translucent)
                } else {
                    DrawMeasureCursor::PassInstance { pass, draw: draw_index, instance: 0, translucent }
                };
                (usage, next)
            }
            DrawMeasureCursor::PassDrawKey { pass, draw: draw_index, byte, translucent } => {
                let pass_value = &draw.scene_passes[pass];
                let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
                let value = &draws[draw_index];
                let next = if byte + 1 < value.mesh_key.len() {
                    DrawMeasureCursor::PassDrawKey { pass, draw: draw_index, byte: byte + 1, translucent }
                } else if value.instances.is_empty() {
                    Self::next_pass_draw(draw, pass, draw_index, translucent)
                } else {
                    DrawMeasureCursor::PassInstance { pass, draw: draw_index, instance: 0, translucent }
                };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassInstance { pass, draw: draw_index, instance, translucent } => {
                let pass_value = &draw.scene_passes[pass];
                let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
                let value = &draws[draw_index].instances[instance];
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_mesh::Instance3d>(), ..PreparedRenderUsage::default() };
                let next = if value.id.is_empty() { Self::next_pass_instance(draw, pass, draw_index, instance, translucent) } else { DrawMeasureCursor::PassInstanceKey { pass, draw: draw_index, instance, byte: 0, translucent } };
                (usage, next)
            }
            DrawMeasureCursor::PassInstanceKey { pass, draw: draw_index, instance, byte, translucent } => {
                let pass_value = &draw.scene_passes[pass];
                let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
                let key = &draws[draw_index].instances[instance].id;
                let next = if byte + 1 < key.len() { DrawMeasureCursor::PassInstanceKey { pass, draw: draw_index, instance, byte: byte + 1, translucent } } else { Self::next_pass_instance(draw, pass, draw_index, instance, translucent) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassLine { pass, draw: draw_index } => {
                let pass_value = &draw.scene_passes[pass];
                let next = if pass_value.line_draws[draw_index].vertices.is_empty() { Self::next_pass_line(draw, pass, draw_index) } else { DrawMeasureCursor::PassLineVertex { pass, draw: draw_index, vertex: 0 } };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::LineDraw3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassLineVertex { pass, draw: draw_index, vertex } => {
                let next = if vertex + 1 < draw.scene_passes[pass].line_draws[draw_index].vertices.len() { DrawMeasureCursor::PassLineVertex { pass, draw: draw_index, vertex: vertex + 1 } } else { Self::next_pass_line(draw, pass, draw_index) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::LineVertex3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassTextured { pass, draw: draw_index } => {
                let value = &draw.scene_passes[pass].textured_draws[draw_index];
                let next = if value.instances.is_empty() { Self::next_textured_draw(draw, pass, draw_index) } else { DrawMeasureCursor::PassTexturedInstance { pass, draw: draw_index, instance: 0 } };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::TexturedDraw3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassTexturedInstance { pass, draw: draw_index, instance } => {
                let value = &draw.scene_passes[pass].textured_draws[draw_index].instances[instance];
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::TexturedInstance3d>(), ..PreparedRenderUsage::default() };
                let next = if value.texture_key.is_empty() { Self::next_textured_instance(draw, pass, draw_index, instance) } else { DrawMeasureCursor::PassTexturedKey { pass, draw: draw_index, instance, byte: 0 } };
                (usage, next)
            }
            DrawMeasureCursor::PassTexturedKey { pass, draw: draw_index, instance, byte } => {
                let key = &draw.scene_passes[pass].textured_draws[draw_index].instances[instance].texture_key;
                let next = if byte + 1 < key.len() { DrawMeasureCursor::PassTexturedKey { pass, draw: draw_index, instance, byte: byte + 1 } } else { Self::next_textured_instance(draw, pass, draw_index, instance) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassGrid { pass } => {
                let next = Self::next_after_grid(draw, pass);
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::ProceduralGrid3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassMaterial { pass, draw: draw_index, translucent } => {
                let value = &draw.scene_passes[pass].material_draws[draw_index];
                let next = if !value.mesh_key.is_empty() { DrawMeasureCursor::PassMaterialMeshKey { pass, draw: draw_index, byte: 0, translucent } } else { Self::next_material_after_keys(draw, pass, draw_index, translucent) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::SceneMaterialDraw3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassMaterialMeshKey { pass, draw: draw_index, byte, translucent } => {
                let value = &draw.scene_passes[pass].material_draws[draw_index];
                let next = if byte + 1 < value.mesh_key.len() { DrawMeasureCursor::PassMaterialMeshKey { pass, draw: draw_index, byte: byte + 1, translucent } } else { Self::next_material_after_keys(draw, pass, draw_index, translucent) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassMaterialTextureKey { pass, draw: draw_index, byte, translucent } => {
                let texture_bytes:usize = draw.scene_passes[pass].material_draws[draw_index].material.texture_keys().map(str::len).sum();
                let next = if byte + 1 < texture_bytes {
                    DrawMeasureCursor::PassMaterialTextureKey { pass, draw: draw_index, byte: byte + 1, translucent }
                } else if draw.scene_passes[pass].material_draws[draw_index].instances.is_empty() {
                    Self::next_material_draw(draw, pass, draw_index, translucent)
                } else {
                    DrawMeasureCursor::PassMaterialInstance { pass, draw: draw_index, instance: 0, translucent }
                };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassMaterialInstance { pass, draw: draw_index, instance, translucent } => {
                let value = &draw.scene_passes[pass].material_draws[draw_index].instances[instance];
                let next = if value.id.is_empty() { Self::next_material_instance(draw, pass, draw_index, instance, translucent) } else { DrawMeasureCursor::PassMaterialInstanceKey { pass, draw: draw_index, instance, byte: 0, translucent } };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_mesh::Instance3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassMaterialInstanceKey { pass, draw: draw_index, instance, byte, translucent } => {
                let key = &draw.scene_passes[pass].material_draws[draw_index].instances[instance].id;
                let next = if byte + 1 < key.len() { DrawMeasureCursor::PassMaterialInstanceKey { pass, draw: draw_index, instance, byte: byte + 1, translucent } } else { Self::next_material_instance(draw, pass, draw_index, instance, translucent) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassPostprocess { pass } => {
                let next = Self::next_after_scene_pass_content(draw, pass);
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::SceneCurvilinear3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::Glass(index) => {
                let region = draw.glass_regions.get(index)?;
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_draw_contract::GlassRegion>(), ..PreparedRenderUsage::default() }, Self::layer_channel_cursor(draw, region.layer_index, 0))
            }
            DrawMeasureCursor::Complete => return None,
        };
        *cursor = next;
        Some(usage)
    }

    fn next_layer_raster(draw: &DrawList, layer: usize, raster: usize, overlay: bool) -> DrawMeasureCursor {
        let instances = if overlay { &draw.layers[layer].overlay_raster_instances } else { &draw.layers[layer].raster_instances };
        if raster + 1 < instances.len() {
            DrawMeasureCursor::LayerRaster { layer, raster: raster + 1, overlay }
        } else if overlay {
            Self::layer_channel_cursor(draw, layer, 6)
        } else {
            Self::next_after_layer(draw, layer)
        }
    }

    /// 🥞️ The six channels one `DrawLayer` publishes. Normal ui, vector, and raster channels
    /// terminate at that layer's scene passes; its overlay ui, vector, and raster channels resume
    /// after those passes and before the following layer.
    ///
    /// 🩸️ This walk used to put the RASTERS last, after both overlay channels, so within one layer an
    /// engine surface's opaque vello texture was composited ON TOP of the overlay glyphs painted over
    /// it. The two renderers therefore disagreed about what "overlay" means, and the browser one —
    /// which is the prepared one — lost every caption the shell paints over an engine raster: the live
    /// generation3d flow window on 6118 drew all seven node bodies, every port, every wire and the
    /// minimap, and not one word (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
    /// `📓️w14b-generation3d-labels-preview-layout.md`).
    fn layer_channel_cursor(draw: &DrawList, layer: usize, from_channel: u8) -> DrawMeasureCursor {
        let Some(value) = draw.layers.get(layer) else {
            return DrawMeasureCursor::Complete;
        };
        let terminal_channel = if from_channel < 3 { 3 } else { 6 };
        for channel in from_channel..terminal_channel {
            match channel {
                0 if !value.ui_instances.is_empty() => return DrawMeasureCursor::LayerUi { layer, item: 0, overlay: false },
                1 if !value.vector_vertices.is_empty() => return DrawMeasureCursor::LayerVector { layer, item: 0, overlay: false },
                2 if !value.raster_instances.is_empty() => return DrawMeasureCursor::LayerRaster { layer, raster: 0, overlay: false },
                3 if !value.overlay_ui_instances.is_empty() => return DrawMeasureCursor::LayerUi { layer, item: 0, overlay: true },
                4 if !value.overlay_vector_vertices.is_empty() => return DrawMeasureCursor::LayerVector { layer, item: 0, overlay: true },
                5 if !value.overlay_raster_instances.is_empty() => return DrawMeasureCursor::LayerRaster { layer, raster: 0, overlay: true },
                _ => {}
            }
        }
        if from_channel < 3 {
            Self::next_after_layer(draw, layer)
        } else {
            DrawMeasureCursor::LayerHeader(layer + 1)
        }
    }

    fn next_after_layer(draw: &DrawList, layer: usize) -> DrawMeasureCursor {
        let pass = draw.scene_passes.partition_point(|pass| pass.layer_index < layer);
        if draw.scene_passes.get(pass).is_some_and(|pass| pass.layer_index == layer) {
            DrawMeasureCursor::PassHeader(pass)
        } else {
            Self::layer_channel_cursor(draw, layer, 3)
        }
    }

    fn next_after_scene_pass(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        if draw.scene_passes.get(pass).is_some_and(|value| value.curvilinear.is_some() || value.viewport_mask == semio_framework_ui_scene_pass::SceneViewportMask3d::Ellipse) {
            return DrawMeasureCursor::PassPostprocess { pass };
        }
        Self::next_after_scene_pass_content(draw, pass)
    }

    fn next_after_scene_prelude(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        let value = &draw.scene_passes[pass];
        if value.clear_color.is_some() {
            DrawMeasureCursor::PassSceneClear { pass }
        } else if value.shadow.enabled && value.shadow_draws.iter().any(|draw| draw.shadow_role.casts && !draw.instances.is_empty()) {
            DrawMeasureCursor::PassShadowBegin(pass)
        } else {
            Self::next_after_shadow(draw, pass)
        }
    }

    fn next_after_scene_pass_content(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        let Some(value) = draw.scene_passes.get(pass) else { return DrawMeasureCursor::Complete };
        if draw.scene_passes.get(pass + 1).is_some_and(|next| next.layer_index == value.layer_index) {
            DrawMeasureCursor::PassHeader(pass + 1)
        } else {
            Self::layer_channel_cursor(draw, value.layer_index, 3)
        }
    }

    fn next_pass_instance(draw: &DrawList, pass: usize, draw_index: usize, instance: usize, translucent: bool) -> DrawMeasureCursor {
        let pass_value = &draw.scene_passes[pass];
        let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
        if instance + 1 < draws[draw_index].instances.len() {
            DrawMeasureCursor::PassInstance { pass, draw: draw_index, instance: instance + 1, translucent }
        } else {
            Self::next_pass_draw(draw, pass, draw_index, translucent)
        }
    }

    fn first_shadow_instance(draw: &DrawList, pass: usize) -> Option<DrawMeasureCursor> {
        draw.scene_passes[pass].shadow_draws.iter().position(|draw| draw.shadow_role.casts && !draw.instances.is_empty()).map(|draw| DrawMeasureCursor::PassShadowInstance { pass, draw, instance: 0 })
    }

    fn next_shadow_instance(draw: &DrawList, pass: usize, draw_index: usize, instance: usize) -> DrawMeasureCursor {
        let draws = &draw.scene_passes[pass].shadow_draws;
        if instance + 1 < draws[draw_index].instances.len() {
            return DrawMeasureCursor::PassShadowInstance { pass, draw: draw_index, instance: instance + 1 };
        }
        if let Some(next_draw) = draws.iter().enumerate().skip(draw_index + 1).find_map(|(index, draw)| (draw.shadow_role.casts && !draw.instances.is_empty()).then_some(index)) {
            return DrawMeasureCursor::PassShadowInstance { pass, draw: next_draw, instance: 0 };
        }
        Self::next_after_shadow(draw, pass)
    }

    fn next_after_shadow(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        if draw.scene_passes[pass].textured_draws.is_empty() {
            Self::next_after_textured(draw, pass)
        } else {
            DrawMeasureCursor::PassTextured { pass, draw: 0 }
        }
    }

    fn next_pass_line(draw: &DrawList, pass: usize, draw_index: usize) -> DrawMeasureCursor {
        if draw_index + 1 < draw.scene_passes[pass].line_draws.len() {
            DrawMeasureCursor::PassLine { pass, draw: draw_index + 1 }
        } else {
            Self::next_after_lines(draw, pass)
        }
    }

    fn next_textured_instance(draw: &DrawList, pass: usize, draw_index: usize, instance: usize) -> DrawMeasureCursor {
        if instance + 1 < draw.scene_passes[pass].textured_draws[draw_index].instances.len() {
            DrawMeasureCursor::PassTexturedInstance { pass, draw: draw_index, instance: instance + 1 }
        } else {
            Self::next_textured_draw(draw, pass, draw_index)
        }
    }

    fn first_material(draw: &DrawList, pass: usize, translucent: bool) -> Option<DrawMeasureCursor> {
        draw.scene_passes[pass].material_draws.iter().position(|value| value.translucent == translucent && !value.instances.is_empty()).map(|draw| DrawMeasureCursor::PassMaterial { pass, draw, translucent })
    }

    fn next_material_after_keys(draw: &DrawList, pass: usize, draw_index: usize, translucent: bool) -> DrawMeasureCursor {
        let value = &draw.scene_passes[pass].material_draws[draw_index];
        match &value.material {
            material if material.texture_keys().any(|texture_key| !texture_key.is_empty()) => DrawMeasureCursor::PassMaterialTextureKey { pass, draw: draw_index, byte: 0, translucent },
            _ if value.instances.is_empty() => Self::next_material_draw(draw, pass, draw_index, translucent),
            _ => DrawMeasureCursor::PassMaterialInstance { pass, draw: draw_index, instance: 0, translucent },
        }
    }

    fn next_material_instance(draw: &DrawList, pass: usize, draw_index: usize, instance: usize, translucent: bool) -> DrawMeasureCursor {
        if instance + 1 < draw.scene_passes[pass].material_draws[draw_index].instances.len() {
            DrawMeasureCursor::PassMaterialInstance { pass, draw: draw_index, instance: instance + 1, translucent }
        } else {
            Self::next_material_draw(draw, pass, draw_index, translucent)
        }
    }

    fn next_material_draw(draw: &DrawList, pass: usize, draw_index: usize, translucent: bool) -> DrawMeasureCursor {
        if let Some((next, _)) = draw.scene_passes[pass].material_draws.iter().enumerate().skip(draw_index + 1).find(|(_, value)| value.translucent == translucent && !value.instances.is_empty()) {
            DrawMeasureCursor::PassMaterial { pass, draw: next, translucent }
        } else if translucent {
            Self::next_after_scene_pass(draw, pass)
        } else if !draw.scene_passes[pass].draws.is_empty() {
            DrawMeasureCursor::PassDraw { pass, draw: 0, translucent: false }
        } else {
            Self::next_after_opaque(draw, pass)
        }
    }

    fn next_pass_draw(draw: &DrawList, pass: usize, draw_index: usize, translucent: bool) -> DrawMeasureCursor {
        let pass_value = &draw.scene_passes[pass];
        let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
        if draw_index + 1 < draws.len() {
            DrawMeasureCursor::PassDraw { pass, draw: draw_index + 1, translucent }
        } else if translucent {
            Self::next_after_translucent(draw, pass)
        } else {
            Self::next_after_opaque(draw, pass)
        }
    }

    fn next_after_opaque(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        let value = &draw.scene_passes[pass];
        if !value.line_draws.is_empty() {
            DrawMeasureCursor::PassLine { pass, draw: 0 }
        } else {
            Self::next_after_lines(draw, pass)
        }
    }

    fn next_after_lines(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        let value = &draw.scene_passes[pass];
        if !value.translucent_draws.is_empty() {
            DrawMeasureCursor::PassDraw { pass, draw: 0, translucent: true }
        } else {
            Self::next_after_translucent(draw, pass)
        }
    }

    /// 🖼️ A scene pass is measured — and therefore encoded — TEXTURED → opaque → lines → translucent.
    ///
    /// 🩸️ The textured underlay used to come last, so the world reference plane composited OVER the
    /// grid and the model and tinted everything it covered. React draws the same plane at
    /// `renderOrder = -10` (`🎨️r3f/🟦️.tsx`'s `WorldReferencePlaneItem`), under the grid's `-5` and the
    /// model's default `0`. The pass is a non-depth-writing `LessEqual` blend either way, so drawing it
    /// first is what makes the geometry in front of it actually occlude it (ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w7b-presenter-one-frame-per-boot.md` §4).
    fn next_after_textured(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        if draw.scene_passes[pass].procedural_grid.is_some() {
            DrawMeasureCursor::PassGrid { pass }
        } else {
            Self::next_after_grid(draw, pass)
        }
    }

    fn next_after_grid(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        if let Some(cursor) = Self::first_material(draw, pass, false) {
            cursor
        } else if !draw.scene_passes[pass].draws.is_empty() {
            DrawMeasureCursor::PassDraw { pass, draw: 0, translucent: false }
        } else {
            Self::next_after_opaque(draw, pass)
        }
    }

    fn next_after_translucent(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        Self::first_material(draw, pass, true).unwrap_or_else(|| Self::next_after_scene_pass(draw, pass))
    }

    fn next_textured_draw(draw: &DrawList, pass: usize, draw_index: usize) -> DrawMeasureCursor {
        if draw_index + 1 < draw.scene_passes[pass].textured_draws.len() {
            DrawMeasureCursor::PassTextured { pass, draw: draw_index + 1 }
        } else {
            Self::next_after_textured(draw, pass)
        }
    }

}
//#endregion ⚙️PreparationJob
