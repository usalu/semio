//! 🎬️ World3d host render: `render_world_3d` and the component face overlay build.

use super::*;

// 🖥️ Native/browser rendering-host entry point only (called from `📺️renderer`'s engine, never from
// wasip2 plugin guest logic — confirmed by grepping every `render_world_3d` call site in the repo).
// `WidgetContext` bundles the font/icon atlases, which are genuinely GPU-adjacent (real `wgpu`
// crate reachable through `wgpu-engine`), so this one function stays excluded from
// `wasm32-wasip2`: `target_arch = "wasm32"` is TRUE for wasip2 too, hence `not(target_env = "p2")`.
/// 👻️ Mirrors `BrushPreviewGhost`: renders whenever `origin` is present, regardless of
/// `meshUrl` — a translucent unit box is the fallback ghost when there's no mesh URL (or
/// its GLB hasn't resolved into `state.meshes` yet), not "nothing at all". The box itself is
/// a placeholder LEASE, so the ghost appears on the frame that lease lands, never before it.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn render_world_3d(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut semio_framework_ui_wgpu_widget_kit::WidgetContext<'_, ActionDescriptor>,
    state: &mut World3dState,
    gpu: &mut World3dBuildContext,
    shadow_profile: World3dShadowProfile,
) {
    use semio_framework_ui_wgpu_widget_kit::{draw_text, gizmo as gpu_gizmo};
    let theme = ctx.theme;
    let modelling = scene.world_3d.as_ref();
    let section = modelling.and_then(|world| world.modelling_options.as_ref()).and_then(|options| options.section);
    let sub_element_paint = world3d_sub_element_paint_native(theme, modelling.and_then(|world| world.modelling_options.as_ref()).and_then(|options| options.highlight.as_ref()));
    if state.overlay.sub_element_paint != sub_element_paint {
        state.overlay.sub_element_paint = sub_element_paint;
        advance_world3d_view_revision(&mut state.session, &mut state.scene);
    }
    step_world_placeholder_mesh(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene);
    state.session.pick_bounds = ctx.pick_clip.unwrap_or(bounds);
    sync_world3d_state(state, scene, bounds);
    let _ = step_world3d_camera_fit(state);
    apply_terrain_style_if_changed(&mut state.session, &mut state.scene, &mut state.terrain, gpu);
    let current_lod = scene_lod(&state.session, &state.camera, &state.scene);
    let lod_changed = state.camera.resolved_lod_pick.is_none_or(|previous| (previous - current_lod).abs() > WORLD_LOD_EPSILON);
    if lod_changed {
        state.camera.resolved_lod_pick = Some(current_lod);
    }
    apply_runtime_draw_flags(&mut state.selection, &mut state.interaction, &mut state.scene);
    let inner = bounds;
    let clear_color = match state.session.presentation.clear {
        World3dPresentationClear::Theme => Some(environment_clear_color(&state.scene.environment, theme.canvas_clear)),
        World3dPresentationClear::Transparent => environment_explicit_clear_color(&state.scene.environment),
    };
    if state.session.presentation.viewport_mask == SceneViewportMask3d::Rectangle {
        if let Some(clear_color) = clear_color {
            ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], clear_color);
        }
    }
    let camera = state.camera.orbit.to_camera();
    let light_dir = environment_light_dir(&state.scene.environment);
    let lighting = environment_scene_lighting(&state.scene.environment);
    let neutral_material = environment_scene_material(&state.scene.environment);
    let shadow = environment_scene_shadow(&state.scene.environment, light_dir, shadow_profile);
    let mut terrain_draws = sync_terrain(&mut state.session, &mut state.authority, &mut state.scene, &mut state.terrain, &mut state.assets, gpu, &camera);
    for draw in &mut terrain_draws {
        draw.shadow_role = world3d_shadow_role(World3dShadowGeometry::Terrain, shadow.enabled);
    }
    update_visible_chunks(&mut state.scene, camera.position);
    let projection_height = state.session.presentation.source_aspect.map_or(inner.h, |aspect| inner.w / aspect as f32);
    let view_proj = ui_scene::math::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, projection_height);
    let planes = frustum_planes(view_proj);
    let shadow_planes = directional_shadow_frustum_planes(light_dir);
    let mut culled_draws = Vec::new();
    let mut material_draws = Vec::new();
    let mut shadow_draws = Vec::new();
    let mut culled_count = 0u32;
    let mut needed_mesh_keys = HashSet::new();
    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    for (draw_index, draw) in state.scene.draws.iter().enumerate() {
        let Some(&mesh) = state.scene.meshes.get(&draw.mesh_key) else {
            continue;
        };
        let Ok((mesh_min, mesh_max)) = mesh.aabb() else { continue };
        let mesh_version = live_world3d_mesh_version(&state.scene, &draw.mesh_key);
        let mut instances = Vec::new();
        let mut painted_instances = Vec::new();
        let mut celebration_instances = Vec::new();
        let mut authored_instances = Vec::new();
        let mut shadow_instances = Vec::new();
        let schema = mesh.schema().ok();
        let has_vertex_colors = schema.is_some_and(|schema| schema.colors > 0);
        let paint_texture = schema.filter(|schema| schema.uvs == schema.vertices && schema.uvs > 0).and_then(|_| state.scene.mesh_paint_textures.get(&draw.mesh_key)).map(|raster| raster.identity);
        let [metalness, roughness] = world3d_standard_material_for_mesh(&state.scene, &draw.mesh_key, neutral_material);
        for (instance_index, source) in draw.instances.iter().enumerate() {
            let style = world_instance_style(&state.selection, source);
            let mut instance = world3d_style_paint(theme, style, has_vertex_colors, source.clone());
            if section.is_some() {
                instance.material.section = SceneSectionRole3d::Clipped;
            }
            let style_kind = resolve_mesh_style(style);
            instance.material.metalness = metalness;
            instance.material.roughness = roughness;
            instance.model = retained_gumball_preview_model(&state.interaction, &state.authority, draw_index, instance_index, instance.model);
            let position = state.scene.instance_positions.get(&instance.id).copied().unwrap_or([0.0, 0.0, 0.0]);
            if !instance_chunk_visible(&state.scene, position) {
                continue;
            }
            let (min, max) = transform_aabb(instance.model, mesh_min, mesh_max);
            if shadow.enabled && aabb_intersects_frustum(&shadow_planes, min, max) {
                shadow_instances.push(instance.clone());
            }
            if aabb_intersects_frustum(&planes, min, max) {
                if matches!(style_kind,MeshStyleKind::Neutral|MeshStyleKind::Disabled) && state.scene.mesh_appearances.contains_key(&draw.mesh_key) {
                    authored_instances.push(instance);
                } else if style_kind == MeshStyleKind::Celebrated && !instance.material.preserve_vertex_color {
                    celebration_instances.push(instance);
                } else if paint_texture.is_some() {
                    painted_instances.push(instance);
                } else {
                    instances.push(instance);
                }
            } else {
                culled_count += 1;
            }
        }
        if !instances.is_empty() || !painted_instances.is_empty() || !celebration_instances.is_empty() || !authored_instances.is_empty() || !shadow_instances.is_empty() {
            needed_mesh_keys.insert(draw.mesh_key.clone());
            gpu.ensure_mesh(&draw.mesh_key, mesh_version, mesh);
        }
        if paint_texture.is_some() && !painted_instances.is_empty() {
            let texture_key = format!("mesh-paint:{}", draw.mesh_key);
            if let Some(lease) = world_raster_upload(&state.scene, &draw.mesh_key, &texture_key, true) {
                gpu.ensure_world_plane_texture(&texture_key, lease);
            }
        }
        if !authored_instances.is_empty() {
            let pool = world_scene_raster_pool();
            let override_material = environment_authored_material(&state.scene.environment);
            if let Some(appearance) = state.scene.mesh_appearances.get(&draw.mesh_key) {
                for primitive in appearance.primitives() {
                    let material = override_material.clone().unwrap_or_else(|| primitive.material.clone());
                    for texture_key in SceneMaterialKind3d::Authored(material.clone()).texture_keys() {if let Some(lease)=appearance.texture_upload(texture_key,&pool){gpu.ensure_world_plane_texture(texture_key,lease);}}
                    material_draws.push(SceneMaterialDraw3d {
                        mesh_key: draw.mesh_key.clone(),
                        mesh_version,
                        first_index: primitive.first_index,
                        index_count: primitive.index_count,
                        instances: authored_instances.clone(),
                        translucent: material.alpha == SceneMaterialAlpha3d::Blend,
                        material: SceneMaterialKind3d::Authored(material),
                    });
                }
            }
        }
        let glb_shadow_role = world3d_shadow_role(World3dShadowGeometry::Glb, shadow.enabled);
        if !shadow_instances.is_empty() {
            shadow_draws.push(SceneDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, instances: shadow_instances, shadow_role: glb_shadow_role });
        }
        let instances = instances.into_iter().map(|instance| {
            if state.selection.provisional_instance_ids.contains(&instance.id) {
                Instance3d { selected: false, hovered: false, ..instance }
            } else {
                instance
            }
        });
        let (translucent, opaque): (Vec<_>, Vec<_>) = instances.partition(|instance| instance.color[3] < 1.0);
        if !opaque.is_empty() {
            culled_draws.push(SceneDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, instances: opaque, shadow_role: glb_shadow_role });
        }
        if !translucent.is_empty() {
            material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: translucent, material: SceneMaterialKind3d::Standard, translucent: true });
        }
        if !painted_instances.is_empty() {
            let texture_key = format!("mesh-paint:{}", draw.mesh_key);
            let (opaque, translucent): (Vec<_>, Vec<_>) = painted_instances.into_iter().partition(|instance| instance.color[3] >= 1.0);
            if !opaque.is_empty() {
                material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: opaque, material: SceneMaterialKind3d::Painted { texture_key: texture_key.clone() }, translucent: false });
            }
            if !translucent.is_empty() {
                material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: translucent, material: SceneMaterialKind3d::Painted { texture_key }, translucent: true });
            }
        }
        if !celebration_instances.is_empty() {
            let stops = mesh_style_paint(theme, MeshStyleKind::Celebrated).conic.expect("celebrated style owns the conic triad");
            let angle = (ctx.draw.clock_seconds() / CELEBRATE_CONIC_SPIN_SECONDS).rem_euclid(1.0) * std::f32::consts::TAU;
            let (opaque, translucent): (Vec<_>, Vec<_>) = celebration_instances.into_iter().partition(|instance| instance.color[3] >= 1.0);
            if !opaque.is_empty() {
                material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: opaque, material: SceneMaterialKind3d::Celebration { stops, angle }, translucent: false });
            }
            if !translucent.is_empty() {
                material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: translucent, material: SceneMaterialKind3d::Celebration { stops, angle }, translucent: true });
            }
        }
    }
    sync_mesh_pool(&mut state.scene, &mut state.assets, &needed_mesh_keys, gpu);
    let procedural_grid = if state.session.presentation.show_grid && state.scene.lod.show_grid {
        let datum = state.scene.lod.grid_datum.unwrap_or([0.0, 0.0, 0.0]);
        lod_grid_step_world(current_lod, state.scene.lod.grid_factor).and_then(|step| {
            let cell_size = step as f32;
            if !cell_size.is_finite() || cell_size <= 0.0 {
                return None;
            }
            let plane_z = datum[2] as f32 + 0.001;
            Some(ProceduralGrid3d {
                plane_z,
                camera_plane_projection: [camera.position.x, camera.position.y, plane_z],
                cell_size,
                fade_distance: camera_grid_fade_distance(&camera, datum[2] as f32, step, inner.w, inner.h),
                cell_color: [theme.text_element.r, theme.text_element.g, theme.text_element.b],
            })
        })
    } else {
        None
    };
    let mut line_vertices = Vec::new();
    append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay, theme, &camera, inner, &mut line_vertices);
    append_engagement_preview_lines(&state.scene, &mut line_vertices, [theme.row_hover.r, theme.row_hover.g, theme.row_hover.b, 1.0]);
    append_pick_target_hover_lines(&state.selection, theme, &camera, inner, &mut line_vertices);
    for attraction in &state.scene.attractions {
        let Some(from) = attraction.from else { continue };
        let Some(to) = attraction.to else { continue };
        let color = parse_color(attraction.color.as_deref().unwrap_or("#60a5fa"));
        line_vertices.push(LineVertex3d { position: [from[0] as f32, from[1] as f32, from[2] as f32], color });
        line_vertices.push(LineVertex3d { position: [to[0] as f32, to[1] as f32, to[2] as f32], color });
    }
    for volume in &state.scene.target_volumes {
        append_box_wireframe(&mut line_vertices, volume.origin.unwrap_or([0.0, 0.0, 0.0]), volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), volume.scale.unwrap_or([1.0, 1.0, 1.0]), parse_color(volume.color.as_deref().unwrap_or("#f472b6")));
    }
    let mut extra_draws = Vec::new();
    append_vortex_arrow_draws(&mut state.session, &mut state.selection, &mut state.authority, &mut state.retirement, &mut state.scene, gpu, &mut extra_draws);
    let vertex_instances = append_component_vertex_spheres();
    if !vertex_instances.is_empty() {
        let mesh_version = live_world3d_mesh_version(&state.scene, VERTEX_MARKER_MESH);
        if let Some(mesh) = state.scene.meshes.get(VERTEX_MARKER_MESH) {
            gpu.ensure_mesh(VERTEX_MARKER_MESH, mesh_version, *mesh);
            extra_draws.push(SceneDraw3d { mesh_key: VERTEX_MARKER_MESH.into(), mesh_version, instances: vertex_instances, shadow_role: Default::default() });
        }
    }
    let mut translucent_draws = Vec::new();
    append_component_face_translucent_overlays(state, gpu, &mut translucent_draws);
    append_tool_run_trace_draws(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, &mut state.trace, gpu, theme, &mut translucent_draws);
    if let Some(preview) = state.gumball.catalogue_drop_preview.clone() {
        let mesh_id = ghost_mesh_id(&state.scene, preview.mesh_url.as_deref());
        if !state.scene.meshes.contains_key(&mesh_id) {
            begin_world_placeholder_mesh(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, &mesh_id, WorldPlaceholderKind::Box);
        }
        let mesh_version = live_world3d_mesh_version(&state.scene, &mesh_id);
        if let Some(mesh) = state.scene.meshes.get(&mesh_id).copied() {
            gpu.ensure_mesh(&mesh_id, mesh_version, mesh);
            translucent_draws.push(SceneDraw3d {
                mesh_key: mesh_id,
                mesh_version,
                instances: vec![Instance3d { component_source: None,
                    id: "catalogue-drop-preview".into(),
                    model: Instance3d::model_from_trs([preview.origin[0] as f32, preview.origin[1] as f32, preview.origin[2] as f32], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]),
                    color: {
                        let mut color = parse_color("#59bfff");
                        color[3] = 0.55;
                        color
                    },
                    selected: false,
                    hovered: false,
                 material: Default::default() }],
                shadow_role: Default::default(),
            });
        }
    }
    if let Some(preview) = state.gumball.brush_preview.clone() {
        if let Some(origin) = preview.origin {
            let mesh_id = ghost_mesh_id(&state.scene, preview.mesh_url.as_deref());
            if !state.scene.meshes.contains_key(&mesh_id) {
                begin_world_placeholder_mesh(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, &mesh_id, WorldPlaceholderKind::Box);
            }
            let rotation = preview.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            let scale = preview_scale(preview.scale.as_ref());
            let mesh_version = live_world3d_mesh_version(&state.scene, &mesh_id);
            if let Some(mesh) = state.scene.meshes.get(&mesh_id).copied() {
                gpu.ensure_mesh(&mesh_id, mesh_version, mesh);
                translucent_draws.push(SceneDraw3d {
                    mesh_key: mesh_id,
                    mesh_version,
                    instances: vec![Instance3d { component_source: None,
                        id: "brush-preview".into(),
                        model: Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [rotation[0] as f32, rotation[1] as f32, rotation[2] as f32, rotation[3] as f32], scale),
                        color: {
                            let mut color = parse_color(preview.color.as_deref().unwrap_or("#59bfff"));
                            color[3] = 0.45;
                            color
                        },
                        selected: false,
                        hovered: false,
                     material: Default::default() }],
                    shadow_role: Default::default(),
                });
            }
        }
    }
    let mut textured_draws = Vec::new();
    if !state.scene.meshes.contains_key("reference-plane") {
        begin_world_placeholder_mesh(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, "reference-plane", WorldPlaceholderKind::Plane);
    }
    let mut textured_instances = Vec::new();
    let reference_image_urls: Vec<String> = state.scene
        .references
        .iter()
        .filter(|reference| !reference.hidden.unwrap_or(false))
        .filter_map(|reference| reference.url.clone())
        .filter(|url| !world_reference_raster_ready(&state.scene, url))
        .filter(|url| !state.assets.asset_url_misses.contains(url))
        .collect();
    for url in reference_image_urls {
        let _ = reserve_world3d_asset_request(&mut state.session, &mut state.assets, WorldAssetRequestKind::ReferenceImage, &url);
    }
    let visible_references: Vec<WorldReferenceRecord> = state.scene.references.iter().filter(|reference| !reference.hidden.unwrap_or(false)).cloned().collect();
    for reference in visible_references {
        let Some(url) = reference.url.as_deref() else { continue };
        let origin = reference.origin.unwrap_or([0.0, 0.0, 0.0]);
        let width = world_reference_width(&reference);
        let [width, height] = world_reference_plane_size(&state.scene, url, width);
        let visual = world_reference_visual(&state.selection, &reference, theme);
        textured_instances.push(TexturedInstance3d {
            texture_key: url.to_string(),
            model: Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [0.0, 0.0, 0.0, 1.0], [width, height, 1.0]),
            background: visual.background,
            appearance: [visual.content_opacity, 0.0, 0.0, 0.0],
        });
        if let Some(outline) = visual.outline {
            append_world_reference_outline(&mut line_vertices, origin, width, height, outline);
        }
        if let Some(lease) = reference_underlay_upload(&state.scene, url) {
            gpu.ensure_world_plane_texture(url, lease);
        }
    }
    if !textured_instances.is_empty() {
        textured_draws.push(TexturedDraw3d { instances: textured_instances });
    }
    if !state.scene.meshes.contains_key(GUMBALL_PLANE_MESH) {
        begin_world_placeholder_mesh(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, GUMBALL_PLANE_MESH, WorldPlaceholderKind::Plane);
    }
    if !state.selection.selected_ids.is_empty() && state.session.active_utility == "select" && world3d_offers_transform_gumball(&state.session) {
        append_gumball_geometry(&mut line_vertices, &mut translucent_draws, gpu, &state.selection, &state.gumball, &state.scene, &camera, &state.scene.meshes, &state.scene.mesh_versions);
    }
    let paint_shadow_role = world3d_shadow_role(World3dShadowGeometry::Paint, shadow.enabled);
    for draw in extra_draws.iter_mut().chain(translucent_draws.iter_mut()).filter(|draw| draw.shadow_role == SceneShadowRole3d::default()) {
        draw.shadow_role = paint_shadow_role;
    }
    if let Some(section) = section.filter(|section| section.cap.is_some()) {
        append_world3d_section_cap(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, gpu, theme, &section, &culled_draws, &mut translucent_draws);
    }
    culled_draws.extend(extra_draws);
    culled_draws.extend(terrain_draws);
    sort_world3d_translucent_material_draws(&state.scene.meshes, view_proj, &mut material_draws);
    retain_ensured_world_draws(gpu, &mut shadow_draws);
    retain_ensured_world_draws(gpu, &mut culled_draws);
    retain_ensured_world_draws(gpu, &mut translucent_draws);
    retain_ensured_world_material_draws(gpu, &mut material_draws);
    ctx.draw.push_scene_pass(ScenePass3d {
        viewport: [inner.x, inner.y, inner.w, inner.h],
        viewport_mask: state.session.presentation.viewport_mask,
        render_profile: shadow_profile.render_profile(),
        clear_color: (state.session.presentation.viewport_mask == SceneViewportMask3d::Ellipse).then(|| clear_color.map(|color| [color.r, color.g, color.b, color.a])).flatten(),
        view_proj: view_proj.to_cols_array(),
        camera_position: camera.position.to_array(),
        light_dir,
        lighting,
        neutral_material,
        shadow,
        section_clip: section.map(|section| section.clip_plane().map(|component| component as f32)),
        procedural_grid,
        shadow_draws,
        draws: culled_draws,
        line_draws: if line_vertices.is_empty() { Vec::new() } else { vec![LineDraw3d { vertices: line_vertices }] },
        translucent_draws,
        material_draws,
        textured_draws,
        curvilinear: match state.camera.projection_spec.mode {
            Viewport3dProjectionMode::Curvilinear { fov, strength, .. } => Some(ui_scene::math::SceneCurvilinear3d { fov_radians: (fov as f32).min(160.0).to_radians(), strength: strength as f32 }),
            _ => None,
        },
        ..Default::default()
    });
    if let Some(world) = modelling {
        let paints_legend = world.scalar_field.is_some() && matches!(state.overlay.scalar_field_status, World3dScalarFieldStatus::Applied | World3dScalarFieldStatus::Mismatch);
        if world.annotations.is_some() || paints_legend {
            let locale = state.session.modelling_locale;
            ctx.draw.push_scissor(inner);
            if let Some(layer) = world.annotations.as_ref() {
                paint_world3d_annotations(ctx, layer, &view_proj.to_cols_array(), inner, locale);
            }
            if let Some(field) = world.scalar_field.as_ref() {
                match state.overlay.scalar_field_status {
                    World3dScalarFieldStatus::Applied => paint_world3d_legend(ctx, field, inner, locale),
                    World3dScalarFieldStatus::Mismatch => paint_world3d_scalar_mismatch(ctx, inner, locale),
                    World3dScalarFieldStatus::None | World3dScalarFieldStatus::Missing => {}
                }
            }
            ctx.draw.pop_scissor();
        }
    }
    if state.session.presentation.interactive {
        if let Some(points) = world_marquee_overlay_points(&state.selection, &state.authority) {
        let lasso = state.selection.selection_method == "lasso";
        let crossing = marquee_is_crossing_from_path(&points, lasso);
        paint_selection_marquee(ctx.draw, theme, crossing, lasso, &points, true);
        }
    }
    if state.session.presentation.interactive && state.session.presentation.show_gizmo {
        gpu_gizmo::paint_orbit_view_gizmo(ctx, &camera, inner, state.camera.gizmo_hovered_tip);
    }
    for (index, status) in state.overlay.prepared_status.iter().flatten().enumerate() {
        if let Some(text) = status.text() {
            let y = inner.y + 20.0 + index as f32 * (theme.font_size_small + 6.0);
            draw_text(ctx, text, inner.x + 12.0, y, theme.font_size_small, theme.text);
        }
    }
    if let Some(status) = state.overlay.prepared_status[0] {
        let total = f64::from(status.total).max(1.0);
        let ratio = (status.progress[2].min(total) / total) as f32;
        let tone = if status.state == 6 { theme.text } else { theme.text_element };
        ctx.draw.push_solid([inner.x + 12.0, inner.y + 54.0, 160.0 * ratio, 3.0], tone);
    }
    if scene.world_3d.is_none() {
        draw_text(ctx, "world-3d (empty)", inner.x + 12.0, inner.y + 20.0, theme.font_size_small, theme.text_muted);
    }
    if world3d_cursor_work_pending(state) {
        gpu.request_cursor_wake();
    }
    if state.session.presentation.interactive {
        ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(state.session.surface_id.clone()), kind: HitKind::World3d, drag_axis: None, drag_data: None });
    }
}

pub fn step_component_face_overlay_build(state: &mut World3dState) {
    let Some(mut cursor) = state.overlay.face_overlay_build.take() else { return };
    match cursor.step(&state.selection, &state.gumball, &state.scene) {
        WorldFaceOverlayMeshStep::Pending => state.overlay.face_overlay_build = Some(cursor),
        WorldFaceOverlayMeshStep::Ready { index, color, key, lease } => match publish_world3d_mesh_lease(&mut state.interaction, &mut state.scene, key, lease) {
            Ok(()) => {
                cursor.published_colors[index] = Some(color);
                state.overlay.face_overlay_build = Some(cursor);
            }
            Err(rejected) => {
                cursor.owner = WorldPlaceholderOwner::Ready(rejected.value);
                cursor.retry_key = rejected.id;
                cursor.phase = WorldFaceOverlayPhase::Publish;
                cursor.close_started = false;
                state.overlay.face_overlay_build = Some(cursor);
            }
        },
        WorldFaceOverlayMeshStep::Complete { generation, revision, draw_generation, colors } => {
            state.overlay.face_overlay_retired_generation = state.overlay.face_overlay_generation.replace(generation);
            state.overlay.face_overlay_colors = colors;
            state.overlay.face_overlay_applied_revision = revision;
            state.overlay.face_overlay_applied_draw_generation = draw_generation;
        }
        WorldFaceOverlayMeshStep::Stale => {}
        WorldFaceOverlayMeshStep::Fault => mark_world_dynamic_fault(&mut state.session, &mut state.authority, &mut state.scene, WorldDynamicFault::Closing),
    }
}

pub(crate) fn append_component_face_translucent_overlays(state: &mut World3dState, gpu: &mut World3dBuildContext, translucent: &mut Vec<SceneDraw3d>) {
    if !face_component_mode_active(&state.selection) {
        state.overlay.face_overlay_applied_revision = u64::MAX;
        state.overlay.face_overlay_applied_draw_generation = u64::MAX;
        if let Some(cursor) = state.overlay.face_overlay_build.as_mut() {
            cursor.faulted = true;
            cursor.stale = true;
            step_component_face_overlay_build(state);
            return;
        }
        if state.overlay.face_overlay_generation.is_some() {
            state.overlay.face_overlay_retired_generation = state.overlay.face_overlay_generation.take();
            state.overlay.face_overlay_colors = [None; 3];
        }
        return;
    }
    let had_build = state.overlay.face_overlay_build.is_some();
    step_component_face_overlay_build(state);
    if !had_build
        && state.overlay.face_overlay_build.is_none()
        && state.overlay.face_overlay_retired_generation.is_none()
        && (state.overlay.face_overlay_applied_revision != state.session.interaction_revision || state.overlay.face_overlay_applied_draw_generation != state.scene.draw_generation)
        && state.scene.dynamic_mesh_close.is_none()
        && state.scene.dynamic_blocked_mesh.is_none()
        && state.session.snapshot_fault.is_none()
    {
        let generation = state.scene.placeholder_generation.checked_add(3);
        match generation.and_then(|next| WorldFaceOverlayMeshCursor::new(&state.session.surface_id, state.scene.placeholder_generation + 1, state.session.interaction_revision, state.scene.draw_generation).ok().map(|cursor| (next, cursor))) {
            Some((next, cursor)) => {
                state.scene.placeholder_generation = next;
                state.overlay.face_overlay_build = Some(cursor);
            }
            None => mark_world_dynamic_fault(&mut state.session, &mut state.authority, &mut state.scene, WorldDynamicFault::ByteCapacity),
        }
    }
    let Some(generation) = state.overlay.face_overlay_generation else { return };
    for index in 0..3 {
        let Some(color) = state.overlay.face_overlay_colors[index] else { continue };
        let mesh_key = format!("component-face-overlay:{}:{generation}:{index}", state.session.surface_id);
        let Some(mesh) = state.scene.meshes.get(&mesh_key) else { continue };
        let mesh_version = live_world3d_mesh_version(&state.scene, &mesh_key);
        gpu.ensure_mesh(&mesh_key, mesh_version, *mesh);
        translucent.push(SceneDraw3d { mesh_key, mesh_version, instances: vec![Instance3d { component_source: None, id: format!("face-overlay-{index}"), model: Mat4::identity(), color, selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    }
}
