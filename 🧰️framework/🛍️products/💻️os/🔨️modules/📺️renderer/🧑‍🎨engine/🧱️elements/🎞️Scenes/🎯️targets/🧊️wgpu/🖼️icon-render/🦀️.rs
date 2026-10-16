//! 🖼️ Icon render scene frame, layout and chrome.

use super::*;

//#region IconRender
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IconRenderCameraFields {
    pub(crate) position: [f64; 3],
    pub(crate) target: [f64; 3],
    #[serde(default = "icon_render_default_zoom")]
    pub(crate) zoom: f64,
    #[serde(default)]
    fov: Option<f64>,
    #[serde(default)]
    pub(crate) up: Option<[f64; 3]>,
    #[serde(default)]
    projection: IconRenderProjection,
}

#[derive(Clone, Copy, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
enum IconRenderProjection {
    #[default]
    Perspective,
    Orthographic,
}

#[derive(Deserialize)]
struct IconRenderFitFields {
    enabled: bool,
    #[serde(default)]
    pub(crate) padding: Option<f64>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct IconRenderLightsFields {
    #[serde(default)]
    ambient_intensity: f64,
    #[serde(default)]
    ambient_color: Option<String>,
    #[serde(default)]
    sun_azimuth: f64,
    #[serde(default)]
    sun_elevation: f64,
    #[serde(default)]
    sun_intensity: f64,
    #[serde(default)]
    sun_color: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IconRenderMaterialFields {
    #[serde(default)]
    pub(crate) color: Option<String>,
    #[serde(default)]
    metalness: Option<f64>,
    #[serde(default)]
    roughness: Option<f64>,
    #[serde(default)]
    emissive: Option<String>,
    #[serde(default)]
    emissive_intensity: Option<f64>,
    #[serde(default)]
    pub(crate) stroke: Option<String>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
enum IconRenderFormat {
    #[serde(rename = "svg")]
    Svg,
    #[serde(rename = "png")]
    Png,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IconRenderRequestFields {
    asset_url: String,
    pub(crate) format: IconRenderFormat,
    pub(crate) camera: IconRenderCameraFields,
    #[serde(default)]
    pub(crate) fit: Option<IconRenderFitFields>,
    #[serde(default)]
    lights: Option<IconRenderLightsFields>,
    pub(crate) width: f64,
    pub(crate) height: f64,
    #[serde(default)]
    shape: Option<String>,
    #[serde(default)]
    pub(crate) background: Option<String>,
    #[serde(default)]
    shadow_enabled: Option<bool>,
    #[serde(default)]
    material: Option<IconRenderMaterialFields>,
}

fn icon_render_default_zoom() -> f64 {
    1.0
}

fn icon_render_camera_json(request: &IconRenderRequestFields, subject_bounds: Option<([f32; 3], [f32; 3])>, preview_scale: f64) -> String {
    let camera = &request.camera;
    let parallel = camera.projection == IconRenderProjection::Orthographic;
    let fov = camera.fov.unwrap_or(50.0).max(1.0);
    let mut zoom = if camera.zoom.abs() > 1e-6 { camera.zoom } else { 1.0 };
    let mut position = camera.position;
    let mut target = camera.target;
    if let Some((fit, (minimum, maximum))) = request.fit.as_ref().filter(|fit| fit.enabled).zip(subject_bounds) {
        let radius = (0..3).map(|axis| (f64::from(maximum[axis]) - f64::from(minimum[axis])).powi(2)).sum::<f64>().sqrt() * 0.5;
        if radius.is_finite() && radius > 0.0 {
            let padding = fit.padding.unwrap_or(1.25).max(1.0);
            let mut direction = [0, 1, 2].map(|axis| position[axis] - target[axis]);
            if direction.iter().map(|value| value * value).sum::<f64>() < 1e-12 {
                direction = [1.0, -1.0, 0.85];
            }
            let length = direction.iter().map(|value| value * value).sum::<f64>().sqrt();
            let distance = if parallel {
                zoom = (request.width.min(request.height) * 0.5 / (radius * padding).max(0.5)).max(1e-3);
                if length > 1e-6 { length } else { (radius * 4.0).max(2.0) }
            } else {
                let vertical = (fov.to_radians() * 0.5).clamp(0.02, 1.5);
                let horizontal = (vertical.tan() * (request.width / request.height).max(0.05)).atan().clamp(0.02, 1.5);
                (radius.max(1e-4) / vertical.min(horizontal).sin() * padding).max(0.5)
            };
            target = [0, 1, 2].map(|axis| (f64::from(minimum[axis]) + f64::from(maximum[axis])) * 0.5);
            position = [0, 1, 2].map(|axis| target[axis] + direction[axis] / length * distance);
        }
    }
    let effective_fov = if !parallel && (zoom - 1.0).abs() > 1e-6 {
        let half = (fov * 0.5).to_radians();
        (2.0 * (half.tan() / zoom).atan()).to_degrees()
    } else {
        fov
    };
    let up = camera.up.unwrap_or([0.0, 0.0, 1.0]);
    let mode = if parallel { json!({ "kind": "orthographic" }) } else { json!({ "kind": "threePoint", "fov": effective_fov }) };
    json!({
        "position": position,
        "target": target,
        "up": up,
        "fov": effective_fov,
        "zoom": if parallel { zoom * preview_scale } else { 1.0 },
        "projection": { "mode": mode, "orientation": { "type": "free" } },
        "projectionFrame": "preserveCamera",
    })
    .to_string()
}

fn icon_render_environment_json(request: &IconRenderRequestFields) -> String {
    let lights = request.lights.clone().unwrap_or_default();
    let mut value = json!({
        "ambient": { "intensity": lights.ambient_intensity, "color": lights.ambient_color },
        "sun": {
            "enabled": true,
            "azimuth": lights.sun_azimuth,
            "elevation": lights.sun_elevation,
            "intensity": lights.sun_intensity,
            "color": lights.sun_color,
        },
        "shadow": { "enabled": request.shadow_enabled.unwrap_or(false) },
    });
    if let Some(object) = value.as_object_mut() {
        let material = request.material.as_ref();
        object.insert(
            "material".into(),
            json!({
                "color": material.map(|material| material.color.as_deref().unwrap_or("#9aa0ab")),
                "metalness": material.and_then(|material| material.metalness),
                "roughness": material.and_then(|material| material.roughness),
                "emissive": material.and_then(|material| material.emissive.as_deref()),
                "emissiveIntensity": material.and_then(|material| material.emissive_intensity),
                "stroke": material.and_then(|material| material.stroke.as_deref()).unwrap_or("#000000"),
            }),
        );
        if let Some(background) = &request.background {
            object.insert("background".into(), json!(background));
        }
    }
    value.to_string()
}

fn icon_render_shadow_profile(request: &IconRenderRequestFields) -> infinite_world::world::World3dShadowProfile {
    if matches!(request.format, IconRenderFormat::Svg) {
        infinite_world::world::World3dShadowProfile::IconSvg
    } else if request.material.is_some() {
        infinite_world::world::World3dShadowProfile::IconPng
    } else {
        infinite_world::world::World3dShadowProfile::Unshadowed
    }
}

fn render_icon_render_empty(bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, message: &str) {
    let theme = ctx.theme;
    let size = theme.font_size_body;
    let width = ctx.atlas.measure_text(message, size).0;
    draw_text(ctx, message, bounds.x + (bounds.w - width) * 0.5, bounds.y + bounds.h * 0.5, size, theme.text_muted);
}

fn icon_render_footer_layout(bounds: Rect, footer: Option<&str>, atlas: &mut ui_wgpu::wgpu::FontAtlas, size: f32) -> (f32, Vec<std::ops::Range<usize>>) {
    let Some(footer) = footer.filter(|footer| !footer.is_empty()) else { return (0.0, Vec::new()) };
    let lines = atlas.wrap_lines(footer, (bounds.w - 24.0).max(0.0), size);
    (lines.len() as f32 * ui_wgpu::wgpu::text::line_height(size) + 8.0, lines)
}

fn icon_render_frame(bounds: Rect, width: f64, height: f64, footer_height: f32) -> Rect {
    let width = width.max(1.0) as f32;
    let height = height.max(1.0) as f32;
    let body_height = (bounds.h - footer_height).max(0.0);
    let scale = (bounds.w / width).min(body_height / height).max(0.0);
    Rect::new(bounds.x + (bounds.w - width * scale) * 0.5, bounds.y + (body_height - height * scale) * 0.5, width * scale, height * scale)
}

fn icon_render_content_frame(frame: Rect) -> Rect {
    Rect::new(frame.x + 2.0, frame.y + 2.0, (frame.w - 4.0).max(0.0), (frame.h - 4.0).max(0.0))
}

fn icon_render_badge_layout(frame: Rect, text: &str, atlas: &mut ui_wgpu::wgpu::FontAtlas) -> Rect {
    let preferred = atlas.measure_text_face(TextFace::Mono, text, 10.0).0 + 8.0;
    let minimum = text.split_whitespace().map(|word| atlas.measure_text_face(TextFace::Mono, word, 10.0).0).fold(0.0_f32, f32::max) + 8.0;
    let width = (frame.w - 8.0).max(minimum).min(preferred);
    let height = atlas.wrap_lines_face(TextFace::Mono, text, width - 8.0, 10.0).len() as f32 * 15.0;
    Rect::new(frame.x + frame.w - width - 6.0, frame.y + frame.h - height - 6.0, width, height)
}

fn with_icon_render_mask(ctx: &mut FrameworkWidgetContext<'_>, frame: Rect, shape: &str, paint: impl FnOnce(&mut FrameworkWidgetContext<'_>)) {
    ctx.draw.push_scissor(frame);
    let first_layer = ctx.draw.layers.len().saturating_sub(1);
    let first_instance = ctx.draw.layers.last().map_or(0, |layer| layer.ui_instances.len());
    paint(ctx);
    if shape == "ellipse" {
        for (index, layer) in ctx.draw.layers.iter_mut().enumerate().skip(first_layer) {
            for instance in layer.ui_instances.iter_mut().skip(if index == first_layer { first_instance } else { 0 }) {
                instance.clip_ellipse = [frame.x, frame.y, frame.w, frame.h];
            }
        }
    }
    ctx.draw.pop_scissor();
}

/// 🖼️ React's shot is an OFFSCREEN render through `iconRenderPort`, so its host has three visible
/// states: the error text, the finished `<img>`, and `ui.host.rendering` while the promise is in
/// flight (`🖼️IconRenderHost/🟦️.tsx:55-61`). This twin draws the GLB straight into the frame, so it
/// used to have exactly one — a silently EMPTY shot frame for the whole time the mesh was being
/// fetched, and forever if the fetch never landed. The residency of the one subject mesh is the
/// same predicate: no lease yet is "rendering", a recorded snapshot fault is the error arm.
/// 🎬️ Shared preview and export scene construction at the requested camera scale.
fn icon_render_world_scene(request: &IconRenderRequestFields, asset_url: &str, mesh_id: &str, subject_bounds: Option<([f32; 3], [f32; 3])>, preview_scale: f64) -> ui_wgpu::wgpu::World3dScene {
    let instances_json = json!([{
        "id": "icon-render-subject",
        "meshId": mesh_id,
        "position": [0.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
    }])
    .to_string();
    let mut synthetic_world = semio_framework_plugin::world3d_scene(
        icon_render_camera_json(request, subject_bounds, preview_scale),
        semio_framework_plugin::world3d_meshes_json_from_urls(std::slice::from_ref(&asset_url.to_string())),
        instances_json,
        semio_framework_plugin::default_world3d_selection(),
        &semio_framework_plugin::WorldSunConfig::default(),
    );
    synthetic_world.environment_json = Some(icon_render_environment_json(request));
    synthetic_world.presentation_json = Some(json!({ "showGrid": false, "showGizmo": false, "interactive": false, "viewportMask": if request.shape.as_deref() == Some("ellipse") { "ellipse" } else { "rectangle" }, "clear": "transparent", "sourceAspect": request.width.max(1.0) / request.height.max(1.0) }).to_string());

    synthetic_world
}

/// 📤️ Wraps the shared Icon request scene in the exact component envelope consumed by World3d.
fn icon_render_world_component_scene(
    request: &IconRenderRequestFields,
    asset_url: &str,
    mesh_id: &str,
    subject_bounds: Option<([f32; 3], [f32; 3])>,
    preview_scale: f64,
    host_id: String,
    surface_id: String,
    controller_id: String,
) -> UiComponentSceneNode {
    UiComponentSceneNode {
        presence: UiPresence::default(),
        host_id,
        surface_id,
        controller_id,
        component_kind: SurfaceKind::World3d,
        pane_id: None,
        binding_id: None,
        canvas_2d: None,
        world_3d: Some(icon_render_world_scene(request, asset_url, mesh_id, subject_bounds, preview_scale)),
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        diff_view: None,
        event_feed: None,
        block_list: None,
        menu: None,
    }
}

/** 🖼️ Native counterpart of framework/renderer/react/components/icon-render-host.tsx: reframes the request into a synthetic World3dScene and delegates the actual GLB draw to infinite_world::world::render_world_3d, then paints the aspect-fit frame/badge/footer chrome on top. */
pub fn render_icon_render(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, hosts: &mut SceneEngineHosts<'_>) {
    let Some(icon_render) = &scene.icon_render else {
        return render_icon_render_empty(bounds, ctx, hosts.chrome_labels.icon_render.empty_scene);
    };
    let Ok(request) = serde_json::from_str::<IconRenderRequestFields>(&icon_render.request_json) else {
        return render_icon_render_empty(bounds, ctx, hosts.chrome_labels.icon_render.empty_scene);
    };

    let shape = request.shape.clone().unwrap_or_else(|| "rectangle".into());
    let (footer_height, _) = icon_render_footer_layout(bounds, icon_render.footer.as_deref(), ctx.atlas, ctx.theme.font_size_small);
    let frame = icon_render_frame(bounds, request.width, request.height, footer_height);
    let content_frame = icon_render_content_frame(frame);

    let asset_url = crate::mesh_assets::mesh_asset_transport_url(&request.asset_url);
    let mesh_id = semio_framework_plugin::world3d_mesh_id_from_url(&asset_url);
    let surface_id = scene.surface_id.clone();
    let controller_id = scene.controller_id.clone();
    let Some(state) = hosts.world3d_states.get_or_insert_with(scene.host_id.clone(), || infinite_world::world::World3dState::new(surface_id, controller_id)) else {
        return render_icon_render_empty(bounds, ctx, hosts.chrome_labels.icon_render.empty_scene);
    };
    let subject_bounds = state.mesh_lease(&mesh_id).and_then(|mesh| mesh.aabb().ok());
    let synthetic_scene = icon_render_world_component_scene(
        &request,
        &asset_url,
        &mesh_id,
        subject_bounds,
        f64::from(content_frame.w) / request.width.max(1.0),
        scene.host_id.clone(),
        scene.surface_id.clone(),
        scene.controller_id.clone(),
    );

    infinite_world::world::render_world_3d(&synthetic_scene, content_frame, ctx, state, hosts.world_resources, icon_render_shadow_profile(&request));
    let status = icon_render_subject_status(state, &mesh_id, &asset_url);
    if let Some(message) = icon_render_status_message(status, hosts.chrome_labels.icon_render) {
        let color = if status == IconRenderStatus::Failed { ctx.theme.error } else { ctx.theme.text_muted };
        with_icon_render_mask(ctx, content_frame, &shape, |ctx| render_icon_render_status(ctx, content_frame, message, color));
    }
    paint_icon_render_chrome(ctx, bounds, frame, &request, &shape, icon_render.footer.as_deref());
}

/// 🖼️ The three states React's `IconRenderHost` shows, resolved from the ONE subject mesh's residency.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IconRenderStatus {
    Ready,
    Rendering,
    Failed,
}

/// 🖼️ `Ready` once the subject's GLB lease is published (the frame now draws real geometry),
/// `Failed` when the state recorded a snapshot/capacity fault while it was still missing, else
/// `Rendering` — the asset lane is still fetching or decoding it. Takes the two predicates rather
/// than the state so the three-way mapping is testable without a live `World3dState`.
fn icon_render_status(mesh_resident: bool, faulted: bool) -> IconRenderStatus {
    if mesh_resident {
        IconRenderStatus::Ready
    } else if faulted {
        IconRenderStatus::Failed
    } else {
        IconRenderStatus::Rendering
    }
}

fn icon_render_subject_status(state: &infinite_world::world::World3dState, mesh_id: &str, url: &str) -> IconRenderStatus {
    icon_render_status(state.mesh_lease(mesh_id).is_some(), state.snapshot_fault().is_some() || infinite_world::world::world3d_asset_url_missed(&state.assets, url))
}

fn icon_render_status_message(status: IconRenderStatus, labels: IconRenderChromeLabels) -> Option<&'static str> {
    match status {
        IconRenderStatus::Ready => None,
        IconRenderStatus::Rendering => Some(labels.rendering),
        IconRenderStatus::Failed => Some(labels.failed),
    }
}

/// 🖼️ Centres one status line inside the shot frame, over the (empty) world draw — React puts the
/// same text inside `IconShotFrame`, not beside it.
fn render_icon_render_status(ctx: &mut FrameworkWidgetContext<'_>, frame: Rect, message: &str, color: Rgba) {
    let size = ctx.theme.font_size_body;
    let (width, height) = ctx.atlas.measure_text(message, size);
    draw_text(ctx, message, frame.x + (frame.w - width) * 0.5, frame.y + (frame.h + height) * 0.5, size, color);
}

/// 🖼️ 2px, matching `IconShotFrame`'s `border-2 border-accent` in `icon-render-host.tsx` —
/// `theme.stroke_hairline` (1px) previously halved the frame width relative to React.
/// 🏷️ `bg-background/80`, not `panel` — the badge chip sits on the transparent canvas frame in
/// `icon-render-host.tsx`, not on a panel surface.
fn paint_icon_render_chrome(ctx: &mut FrameworkWidgetContext<'_>, bounds: Rect, frame: Rect, request: &IconRenderRequestFields, shape: &str, footer: Option<&str>) {
    let theme = ctx.theme;
    let hair = 2.0_f32;
    if shape == "ellipse" {
        let centre = [frame.x + frame.w * 0.5, frame.y + frame.h * 0.5];
        let outer = [frame.w * 0.5, frame.h * 0.5];
        let inner = [(outer[0] - hair).max(0.0), (outer[1] - hair).max(0.0)];
        let point = |angle: f32, radius: [f32; 2]| [centre[0] + radius[0] * angle.cos(), centre[1] + radius[1] * angle.sin()];
        for segment in 0..128 {
            let start = std::f32::consts::TAU * segment as f32 / 128.0;
            let end = std::f32::consts::TAU * (segment + 1) as f32 / 128.0;
            ctx.draw.push_triangle_fan(&[point(start, outer), point(end, outer), point(end, inner), point(start, inner)], theme.accent);
        }
    } else {
        ctx.draw.push_solid([frame.x, frame.y, frame.w, hair], theme.accent);
        ctx.draw.push_solid([frame.x, frame.y + frame.h - hair, frame.w, hair], theme.accent);
        ctx.draw.push_solid([frame.x, frame.y, hair, frame.h], theme.accent);
        ctx.draw.push_solid([frame.x + frame.w - hair, frame.y, hair, frame.h], theme.accent);
    }

    let badge = format!("{}×{} · {}", request.width.round() as i64, request.height.round() as i64, shape);
    let badge_rect = icon_render_badge_layout(frame, &badge, ctx.atlas);
    with_icon_render_mask(ctx, icon_render_content_frame(frame), shape, |ctx| {
        ctx.draw.push_solid([badge_rect.x, badge_rect.y, badge_rect.w, badge_rect.h], theme.background.with_alpha(0.8));
        for (index, range) in ctx.atlas.wrap_lines_face(TextFace::Mono, &badge, badge_rect.w - 8.0, 10.0).into_iter().enumerate() {
            draw_text_face(ctx, TextFace::Mono, badge[range].trim_end(), badge_rect.x + 4.0, badge_rect.y + 12.5 + index as f32 * 15.0, 10.0, theme.text_muted);
        }
    });

    if let Some(footer) = footer {
        let footer_size = theme.font_size_small;
        let (height, lines) = icon_render_footer_layout(bounds, Some(footer), ctx.atlas, footer_size);
        let line_height = ui_wgpu::wgpu::text::line_height(footer_size);
        for (index, range) in lines.into_iter().enumerate() {
            let text = &footer[range];
            let width = ctx.atlas.measure_text(text, footer_size).0;
            let baseline = bounds.y + bounds.h - height + index as f32 * line_height + (line_height + footer_size) * 0.5;
            draw_text(ctx, text, bounds.x + (bounds.w - width) * 0.5, baseline, footer_size, theme.text.with_alpha(0.6));
        }
    }
}

/// 🖼️ The aspect-fit frame border, size/shape badge, and optional footer caption painted on top of
/// the delegated `render_world_3d` GLB draw — split out from `render_icon_render` (which needs a
/// live `GpuContext` and so can't run in a headless unit test) so this chrome-only paint can be
/// exercised directly against a `DrawList`.

//#endregion IconRender

//#region IconRenderTests
#[path = "../../../../🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/🦀️.rs"]
pub mod icon_export;

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-icon-render/🦀️.rs"]
mod icon_render_tests;
//#endregion IconRenderTests
