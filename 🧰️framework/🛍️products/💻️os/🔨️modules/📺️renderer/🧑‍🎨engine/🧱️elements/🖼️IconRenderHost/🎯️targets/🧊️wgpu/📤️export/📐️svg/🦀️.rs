//! 📐️ Bounded first-party vector export from one decoded Icon mesh asset.
use std::{cmp::Ordering, collections::BTreeMap};

use infinite_world::world::{World3dMeshAsset, World3dPrimitiveMaterial};
use ui_wgpu::wgpu::{Mat4, Mesh3dField, Mesh3dLease, SceneAuthoredMaterial3d, SceneMaterialAlpha3d, SceneViewportMask3d, Vec3, Vec3Math};

use super::scene::{validate_request, IconExportFormat};
use super::super::{IconRenderCameraFields, IconRenderMaterialFields, IconRenderProjection, IconRenderRequestFields};

const SVG_OUTPUT_CAPACITY: usize = 64 * 1024 * 1024;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Faces,
    Lines,
    Retire,
    Sort,
    Header,
    Body,
    Footer,
    Complete,
    Close,
}

struct SvgItem {
    style: String,
    path: String,
}

#[derive(Clone, Copy)]
struct SvgSortKey {
    depth: f32,
    sequence: u64,
}

impl PartialEq for SvgSortKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for SvgSortKey {}

impl PartialOrd for SvgSortKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SvgSortKey {
    fn cmp(&self, other: &Self) -> Ordering {
        other.depth.total_cmp(&self.depth).then_with(|| self.sequence.cmp(&other.sequence))
    }
}

struct SvgScene {
    view_proj: Mat4,
    viewport_mask: SceneViewportMask3d,
    clear_color: Option<[f32; 4]>,
    light_dir: [f32; 3],
    ambient_color: [f32; 3],
    sun_color: [f32; 3],
    sun_intensity: f32,
    material_override: Option<SceneAuthoredMaterial3d>,
    outline_color: Option<[f32; 4]>,
    primitives: Vec<World3dPrimitiveMaterial>,
}

pub(crate) struct IconSvgRejected {
    fault: String,
    asset: Option<World3dMeshAsset>,
}

impl std::fmt::Debug for IconSvgRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("IconSvgRejected").field("fault", &self.fault).field("owns_asset", &self.asset.is_some()).finish()
    }
}

impl IconSvgRejected {
    pub(crate) fn fault(&self) -> &str {
        &self.fault
    }

    pub(crate) fn close_step(&mut self) -> bool {
        let Some(asset) = self.asset.as_mut() else { return true };
        if !asset.close_step() {
            return false;
        }
        self.asset = None;
        true
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.asset.is_none()
    }
}

pub(crate) struct IconSvgExport {
    phase: Phase,
    scene: Option<SvgScene>,
    mesh: Mesh3dLease,
    asset: Option<World3dMeshAsset>,
    dimensions: (u32, u32),
    draw: usize,
    index: u32,
    edge: u32,
    items: BTreeMap<SvgSortKey, SvgItem>,
    item_bytes: usize,
    item_sequence: u64,
    work_completed: usize,
    work_total: usize,
    body_style: Option<String>,
    output: Option<Vec<u8>>,
    fault: Option<String>,
}

impl IconSvgExport {
    #[expect(clippy::result_large_err, reason = "SVG refusal returns the exact decoded asset owner for bounded retirement.")]
    pub(crate) fn new(request_json: &str, asset: World3dMeshAsset) -> Result<Self, IconSvgRejected> {
        let validated = match validate_request(request_json) {
            Ok(validated) => validated,
            Err(fault) => return Err(IconSvgRejected { fault, asset: Some(asset) }),
        };
        if validated.format != IconExportFormat::Svg {
            return Err(IconSvgRejected { fault: "icon SVG export received a non-SVG request".into(), asset: Some(asset) });
        }
        let mesh = asset.mesh;
        let schema = match mesh.schema() {
            Ok(schema) if schema.indices > 0 && schema.indices.is_multiple_of(3) => schema,
            Ok(_) => return Err(IconSvgRejected { fault: "icon SVG mesh geometry was incomplete".into(), asset: Some(asset) }),
            Err(fault) => return Err(IconSvgRejected { fault: format!("icon SVG mesh schema was unavailable: {fault:?}"), asset: Some(asset) }),
        };
        if asset.appearance.primitives().iter().any(|primitive| primitive.first_index.checked_add(primitive.index_count).is_none_or(|end| end > schema.indices)) {
            return Err(IconSvgRejected { fault: "icon SVG primitive range exceeded its mesh".into(), asset: Some(asset) });
        }
        let bounds = match mesh.aabb() {
            Ok(bounds) => bounds,
            Err(fault) => return Err(IconSvgRejected { fault: format!("icon SVG mesh bounds were unavailable: {fault:?}"), asset: Some(asset) }),
        };
        let scene = svg_scene(&validated.request, validated.dimensions, bounds, asset.appearance.primitives().to_vec());
        let Some(work_total) = scene
            .primitives
            .iter()
            .try_fold(0usize, |total, primitive| total.checked_add(primitive.index_count as usize / 3))
            .and_then(|total| total.checked_add(if scene.outline_color.is_some() { schema.edges as usize } else { 0 }))
        else {
            return Err(IconSvgRejected { fault: "icon SVG work count overflowed".into(), asset: Some(asset) });
        };
        Ok(Self {
            phase: Phase::Faces,
            scene: Some(scene),
            mesh,
            asset: Some(asset),
            dimensions: validated.dimensions,
            draw: 0,
            index: 0,
            edge: 0,
            items: BTreeMap::new(),
            item_bytes: 0,
            item_sequence: 0,
            work_completed: 0,
            work_total,
            body_style: None,
            output: None,
            fault: None,
        })
    }

    pub(crate) fn advance(&mut self) -> Result<bool, String> {
        if self.phase == Phase::Complete {
            return Ok(true);
        }
        if self.phase == Phase::Close {
            return Err(self.fault.clone().unwrap_or_else(|| "icon SVG export was cancelled".into()));
        }
        let result = match self.phase {
            Phase::Faces => self.face_step(),
            Phase::Lines => self.line_step(),
            Phase::Retire => self.retire_step(),
            Phase::Sort => self.sort_step(),
            Phase::Header => self.header_step(),
            Phase::Body => self.body_step(),
            Phase::Footer => self.footer_step(),
            Phase::Complete | Phase::Close => Ok(()),
        };
        if let Err(fault) = result {
            self.fault = Some(fault.clone());
            self.phase = Phase::Close;
            return Err(fault);
        }
        Ok(self.phase == Phase::Complete)
    }

    pub(crate) fn progress(&self) -> (u8, usize, usize) {
        (self.phase as u8, self.work_completed.min(self.work_total), self.work_total)
    }

    pub(crate) fn take_svg(&mut self) -> Option<Vec<u8>> {
        (self.phase == Phase::Complete).then(|| self.output.take()).flatten()
    }

    pub(crate) fn cancel(&mut self) {
        self.fault = None;
        self.phase = Phase::Close;
    }

    pub(crate) fn close_step(&mut self) -> bool {
        self.phase = Phase::Close;
        if let Some(asset) = self.asset.as_mut() {
            if !asset.close_step() {
                return false;
            }
            self.asset = None;
            return false;
        }
        if self.output.as_mut().is_some_and(|output| !output.is_empty()) {
            let output = self.output.as_mut().expect("checked SVG output");
            output.truncate(output.len().saturating_sub(4096));
            return false;
        }
        self.output = None;
        self.body_style = None;
        if self.items.pop_first().is_some() {
            return false;
        }
        let Some(scene) = self.scene.as_mut() else { return true };
        if scene.primitives.pop().is_some() {
            return false;
        }
        self.scene = None;
        true
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.asset.is_none() && self.scene.is_none() && self.items.is_empty() && self.body_style.is_none() && self.output.is_none()
    }

    fn face_step(&mut self) -> Result<(), String> {
        let scene = self.scene.as_ref().ok_or("icon SVG scene was missing")?;
        let Some(draw) = scene.primitives.get(self.draw) else {
            self.phase = Phase::Lines;
            return Ok(());
        };
        let schema = self.mesh.schema().map_err(|fault| format!("icon SVG mesh schema became unavailable: {fault:?}"))?;
        let available = schema.indices.saturating_sub(draw.first_index);
        let count = draw.index_count.min(available);
        if self.index + 2 >= count {
            self.draw += 1;
            self.index = 0;
            return Ok(());
        }
        let offset = draw.first_index + self.index;
        self.index += 3;
        self.work_completed = self.work_completed.saturating_add(1);
        let indices = [0, 1, 2].map(|lane| self.mesh.u32(Mesh3dField::Indices, offset + lane).map_err(|fault| format!("icon SVG index read failed: {fault:?}"))).into_iter().collect::<Result<Vec<_>, _>>()?;
        if indices.iter().any(|index| *index >= schema.vertices) {
            return Err("icon SVG triangle referenced a missing vertex".into());
        }
        let world = indices
            .iter()
            .map(|index| self.mesh.vec3(Mesh3dField::Positions, *index).map(vec3).map_err(|fault| format!("icon SVG position read failed: {fault:?}")))
            .collect::<Result<Vec<_>, _>>()?;
        let world = [world[0], world[1], world[2]];
        let normal = world[2].sub_m(world[1]).cross_m(world[0].sub_m(world[1])).normalize_m();
        let material = authored_material(scene.material_override.as_ref().unwrap_or(&draw.material));
        let vertex_color = if material.vertex_color && schema.colors == schema.vertices {
            self.mesh.vec4(Mesh3dField::Colors, indices[0]).map_err(|fault| format!("icon SVG vertex color read failed: {fault:?}"))?
        } else {
            [1.0; 4]
        };
        let diffuse = [0, 1, 2].map(|axis| material.diffuse[axis] * vertex_color[axis]);
        let fill = lit_color(scene, diffuse, material.emissive, normal);
        let opacity = material.opacity;
        if opacity <= 0.0 {
            return Ok(());
        }
        let clip = world.map(|point| transform_clip(scene.view_proj, point));
        let polygon = clip_near_far(&clip);
        if polygon.len() < 3 {
            return Ok(());
        }
        let projected: Vec<_> = polygon.iter().map(|point| project(*point, self.dimensions)).collect();
        for index in 1..projected.len() - 1 {
            let mut triangle = [projected[0], projected[index], projected[index + 1]];
            if !material.double_sided && !front_facing(triangle) {
                continue;
            }
            if !visible(triangle, self.dimensions) {
                continue;
            }
            overdraw(&mut triangle, 0.5);
            let depth = triangle.iter().map(|point| point[2]).sum::<f32>() / 3.0;
            self.insert_item(
                depth,
                SvgItem {
                    style: format!("fill:{};fill-opacity:{}", css_rgb(fill), scalar(opacity)),
                    path: format!("M{},{}L{},{}L{},{}z", scalar(triangle[0][0]), scalar(triangle[0][1]), scalar(triangle[1][0]), scalar(triangle[1][1]), scalar(triangle[2][0]), scalar(triangle[2][1])),
                },
            )?;
        }
        Ok(())
    }

    fn line_step(&mut self) -> Result<(), String> {
        let scene = self.scene.as_ref().ok_or("icon SVG scene was missing")?;
        let schema = self.mesh.schema().map_err(|fault| format!("icon SVG mesh schema became unavailable: {fault:?}"))?;
        let Some(color) = scene.outline_color else {
            self.phase = Phase::Retire;
            return Ok(());
        };
        if self.edge >= schema.edges {
            self.phase = Phase::Retire;
            return Ok(());
        }
        let edge = self.mesh.edge(self.edge).map_err(|fault| format!("icon SVG edge read failed: {fault:?}"))?;
        self.edge += 1;
        self.work_completed = self.work_completed.saturating_add(1);
        let clip = edge.map(|position| transform_clip(scene.view_proj, vec3(position)));
        let Some(clipped) = clip_line_near_far(clip) else { return Ok(()) };
        let points = clipped.map(|point| project(point, self.dimensions));
        if !visible_line(points, self.dimensions) {
            return Ok(());
        }
        self.insert_item(
            points[0][2].max(points[1][2]),
            SvgItem {
                style: format!("fill:none;stroke:{};stroke-opacity:{};stroke-width:1;stroke-linecap:round", css_rgb([color[0], color[1], color[2]]), scalar(color[3])),
                path: format!("M{},{}L{},{}", scalar(points[0][0]), scalar(points[0][1]), scalar(points[1][0]), scalar(points[1][1])),
            },
        )?;
        Ok(())
    }

    fn retire_step(&mut self) -> Result<(), String> {
        let Some(asset) = self.asset.as_mut() else {
            self.phase = Phase::Sort;
            return Ok(());
        };
        if asset.close_step() {
            self.asset = None;
        }
        Ok(())
    }

    fn sort_step(&mut self) -> Result<(), String> {
        self.phase = Phase::Header;
        Ok(())
    }

    fn header_step(&mut self) -> Result<(), String> {
        let scene = self.scene.as_ref().ok_or("icon SVG scene was missing")?;
        let (width, height) = self.dimensions;
        let half_width = width as f32 * 0.5;
        let half_height = height as f32 * 0.5;
        let background = scene.clear_color.map(|color| css_rgb([color[0], color[1], color[2]]));
        let mut header = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {} {}\" width=\"{}\" height=\"{}\"",
            scalar(-half_width), scalar(-half_height), width, height, width, height
        );
        if scene.viewport_mask == SceneViewportMask3d::Rectangle {
            if let Some(background) = background.as_deref() {
                header.push_str(&format!(" style=\"background-color:{}\"", background));
            }
            header.push('>');
        } else {
            header.push_str(&format!(
                "><defs><clipPath id=\"semio-icon-ellipse-clip\"><ellipse cx=\"0\" cy=\"0\" rx=\"{}\" ry=\"{}\"/></clipPath></defs><g clip-path=\"url(#semio-icon-ellipse-clip)\">",
                scalar(half_width), scalar(half_height)
            ));
            if let Some(background) = background.as_deref() {
                header.push_str(&format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"/>",
                    scalar(-half_width), scalar(-half_height), width, height, background
                ));
            }
        }
        append_output(&mut self.output, &header)?;
        self.phase = Phase::Body;
        Ok(())
    }

    fn body_step(&mut self) -> Result<(), String> {
        let Some((_, item)) = self.items.first_key_value() else {
            if let Some(style) = self.body_style.take() {
                append_output(&mut self.output, &format!("\" style=\"{}\"/>", style))?;
                return Ok(());
            }
            self.phase = Phase::Footer;
            return Ok(());
        };
        if self.body_style.as_deref().is_some_and(|style| style != item.style.as_str()) {
            let style = self.body_style.take().expect("checked SVG body style");
            append_output(&mut self.output, &format!("\" style=\"{}\"/>", style))?;
            return Ok(());
        }
        if self.body_style.is_none() {
            self.body_style = Some(item.style.clone());
            append_output(&mut self.output, "<path d=\"")?;
        }
        let (_, item) = self.items.pop_first().expect("checked SVG body item");
        append_output(&mut self.output, &item.path)?;
        Ok(())
    }

    fn insert_item(&mut self, depth: f32, item: SvgItem) -> Result<(), String> {
        let item_bytes = item.style.len().checked_add(item.path.len()).and_then(|bytes| bytes.checked_add(32)).ok_or("icon SVG candidate byte count overflowed")?;
        let next_bytes = self.item_bytes.checked_add(item_bytes).ok_or("icon SVG candidate byte count overflowed")?;
        if next_bytes > SVG_OUTPUT_CAPACITY {
            return Err("icon SVG candidates exceeded the output byte budget".into());
        }
        let sequence = self.item_sequence;
        self.item_sequence = self.item_sequence.checked_add(1).ok_or("icon SVG painter sequence overflowed")?;
        self.item_bytes = next_bytes;
        self.items.insert(SvgSortKey { depth, sequence }, item);
        Ok(())
    }

    fn footer_step(&mut self) -> Result<(), String> {
        let scene = self.scene.as_ref().ok_or("icon SVG scene was missing")?;
        append_output(&mut self.output, if scene.viewport_mask == SceneViewportMask3d::Ellipse { "</g></svg>" } else { "</svg>" })?;
        self.phase = Phase::Complete;
        Ok(())
    }
}

struct Material {
    diffuse: [f32; 3],
    emissive: [f32; 3],
    opacity: f32,
    double_sided: bool,
    vertex_color: bool,
}

fn authored_material(material: &SceneAuthoredMaterial3d) -> Material {
    Material {
        diffuse: [material.base_color[0], material.base_color[1], material.base_color[2]],
        emissive: material.emissive,
        opacity: material.base_color[3],
        double_sided: material.double_sided,
        vertex_color: material.preserve_vertex_color,
    }
}

fn lit_color(scene: &SvgScene, diffuse: [f32; 3], emissive: [f32; 3], normal: Vec3) -> [f32; 3] {
    let light = vec3(scene.light_dir).normalize_m();
    let amount = normal.dot_m(light).max(0.0) * scene.sun_intensity;
    std::array::from_fn(|axis| (scene.ambient_color[axis] + scene.sun_color[axis] * amount) * diffuse[axis] + emissive[axis])
}

fn svg_scene(request: &IconRenderRequestFields, dimensions: (u32, u32), bounds: ([f32; 3], [f32; 3]), primitives: Vec<World3dPrimitiveMaterial>) -> SvgScene {
    let camera = resolved_camera(request, dimensions, bounds);
    let lights = request.lights.clone().unwrap_or_default();
    let azimuth = lights.sun_azimuth.to_radians() as f32;
    let elevation = lights.sun_elevation.to_radians() as f32;
    let light_dir = [elevation.cos() * azimuth.cos(), elevation.cos() * azimuth.sin(), elevation.sin()];
    let material_override = request.material.as_ref().map(material_override);
    let outline_color = request.material.as_ref().and_then(|material| material.stroke.as_deref()).map(str::trim).map_or_else(
        || Some(parse_color("#000000")),
        |stroke| if stroke.eq_ignore_ascii_case("none") || stroke.eq_ignore_ascii_case("transparent") { None } else { Some(parse_color(stroke)) },
    );
    SvgScene {
        view_proj: camera_view_projection(&camera, dimensions),
        viewport_mask: if request.shape.as_deref() == Some("ellipse") { SceneViewportMask3d::Ellipse } else { SceneViewportMask3d::Rectangle },
        clear_color: request.background.as_deref().filter(|background| !background.eq_ignore_ascii_case("transparent") && !background.is_empty()).map(parse_color),
        light_dir,
        ambient_color: parse_color(lights.ambient_color.as_deref().unwrap_or("#ffffff"))[..3].try_into().expect("fixed color prefix"),
        sun_color: parse_color(lights.sun_color.as_deref().unwrap_or("#ffffff"))[..3].try_into().expect("fixed color prefix"),
        sun_intensity: lights.sun_intensity as f32,
        material_override,
        outline_color,
        primitives,
    }
}

fn material_override(material: &IconRenderMaterialFields) -> SceneAuthoredMaterial3d {
    let base_color = parse_color(material.color.as_deref().unwrap_or("#9aa0ab"));
    let emissive = material.emissive.as_deref().map(parse_color).map_or([0.0; 3], |color| color[..3].try_into().expect("fixed color prefix"));
    SceneAuthoredMaterial3d {
        base_color,
        emissive,
        metalness: material.metalness.unwrap_or(0.0) as f32,
        roughness: material.roughness.unwrap_or(1.0) as f32,
        alpha: if base_color[3] < 1.0 { SceneMaterialAlpha3d::Blend } else { SceneMaterialAlpha3d::Opaque },
        alpha_cutoff: 0.5,
        double_sided: false,
        preserve_vertex_color: false,
        base_color_texture: None,
        texture_sampler: Default::default(),
        metallic_roughness_texture:None,normal_texture:None,occlusion_texture:None,emissive_texture:None,additional_texture_samplers:[Default::default();4],
    normal_scale:[1.0;2],occlusion_strength:1.0,}
}

#[derive(Clone, Copy)]
struct ResolvedCamera {
    position: [f32; 3],
    target: [f32; 3],
    up: [f32; 3],
    zoom: f32,
    fov: f32,
    parallel: bool,
}

fn resolved_camera(request: &IconRenderRequestFields, dimensions: (u32, u32), bounds: ([f32; 3], [f32; 3])) -> ResolvedCamera {
    let IconRenderCameraFields { position, target, zoom, fov, up, projection } = &request.camera;
    let mut position = position.map(|value| value as f32);
    let mut target = target.map(|value| value as f32);
    let mut zoom = *zoom as f32;
    let fov = fov.unwrap_or(50.0) as f32;
    let parallel = *projection == IconRenderProjection::Orthographic;
    if let Some(fit) = request.fit.as_ref().filter(|fit| fit.enabled) {
        let radius = (0..3).map(|axis| (bounds.1[axis] - bounds.0[axis]).powi(2)).sum::<f32>().sqrt() * 0.5;
        if radius.is_finite() && radius > 0.0 {
            let padding = fit.padding.unwrap_or(1.25).max(1.0) as f32;
            let mut direction = vec3([position[0] - target[0], position[1] - target[1], position[2] - target[2]]);
            let length = direction.length_m();
            if length < 1e-6 {
                direction = vec3([1.0, -1.0, 0.85]);
            }
            direction = direction.normalize_m();
            target = std::array::from_fn(|axis| (bounds.0[axis] + bounds.1[axis]) * 0.5);
            let distance = if parallel {
                zoom = (dimensions.0.min(dimensions.1) as f32 * 0.5 / (radius * padding).max(0.5)).max(1e-3);
                if length > 1e-6 { length } else { (radius * 4.0).max(2.0) }
            } else {
                let vertical = (fov.to_radians() * 0.5).clamp(0.02, 1.5);
                let horizontal = (vertical.tan() * (dimensions.0 as f32 / dimensions.1 as f32).max(0.05)).atan().clamp(0.02, 1.5);
                (radius.max(1e-4) / vertical.min(horizontal).sin() * padding).max(0.5)
            };
            position = [target[0] + direction.x * distance, target[1] + direction.y * distance, target[2] + direction.z * distance];
        }
    }
    ResolvedCamera { position, target, up: up.unwrap_or([0.0, 0.0, 1.0]).map(|value| value as f32), zoom, fov, parallel }
}

fn camera_view_projection(camera: &ResolvedCamera, dimensions: (u32, u32)) -> Mat4 {
    let eye = vec3(camera.position);
    let target = vec3(camera.target);
    let up = vec3(camera.up);
    let mut z = eye.sub_m(target);
    if z.length_m() <= f32::EPSILON {
        z.z = 1.0;
    }
    z = z.normalize_m();
    let mut x = up.cross_m(z);
    if x.length_m() <= f32::EPSILON {
        if up.z.abs() == 1.0 {
            z.x += 0.0001;
        } else {
            z.z += 0.0001;
        }
        z = z.normalize_m();
        x = up.cross_m(z);
    }
    x = x.normalize_m();
    let y = z.cross_m(x);
    let view = Mat4 {
        cols: [
            [x.x, y.x, z.x, 0.0],
            [x.y, y.y, z.y, 0.0],
            [x.z, y.z, z.z, 0.0],
            [-x.dot_m(eye), -y.dot_m(eye), -z.dot_m(eye), 1.0],
        ],
    };
    let near = 0.1;
    let far = 10_000.0;
    let projection = if camera.parallel {
        let width = dimensions.0 as f32 / camera.zoom;
        let height = dimensions.1 as f32 / camera.zoom;
        Mat4 { cols: [[2.0 / width, 0.0, 0.0, 0.0], [0.0, 2.0 / height, 0.0, 0.0], [0.0, 0.0, -2.0 / (far - near), 0.0], [0.0, 0.0, -(far + near) / (far - near), 1.0]] }
    } else {
        let effective_fov = 2.0 * ((camera.fov.to_radians() * 0.5).tan() / camera.zoom).atan();
        let f = 1.0 / (effective_fov * 0.5).tan();
        let aspect = dimensions.0 as f32 / dimensions.1 as f32;
        Mat4 { cols: [[f / aspect, 0.0, 0.0, 0.0], [0.0, f, 0.0, 0.0], [0.0, 0.0, -(far + near) / (far - near), -1.0], [0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0]] }
    };
    mul4(projection, view)
}

fn mul4(left: Mat4, right: Mat4) -> Mat4 {
    Mat4 { cols: std::array::from_fn(|column| std::array::from_fn(|row| (0..4).map(|lane| left.cols[lane][row] * right.cols[column][lane]).sum())) }
}

fn parse_color(value: &str) -> [f32; 4] {
    let hex = value.trim().strip_prefix('#').unwrap_or("94a3b8");
    let expanded = if hex.len() == 3 { hex.chars().flat_map(|digit| [digit, digit]).collect::<String>() } else { hex.to_string() };
    if expanded.len() < 6 {
        return parse_color("#94a3b8");
    }
    let channels = [0, 2, 4].map(|offset| u8::from_str_radix(&expanded[offset..offset + 2], 16).unwrap_or(148) as f32 / 255.0);
    [srgb_to_linear(channels[0]), srgb_to_linear(channels[1]), srgb_to_linear(channels[2]), 1.0]
}

fn srgb_to_linear(channel: f32) -> f32 {
    if channel <= 0.04045 { channel / 12.92 } else { ((channel + 0.055) / 1.055).powf(2.4) }
}

fn transform_clip(matrix: Mat4, point: Vec3) -> [f32; 4] {
    [
        point.x * matrix.cols[0][0] + point.y * matrix.cols[1][0] + point.z * matrix.cols[2][0] + matrix.cols[3][0],
        point.x * matrix.cols[0][1] + point.y * matrix.cols[1][1] + point.z * matrix.cols[2][1] + matrix.cols[3][1],
        point.x * matrix.cols[0][2] + point.y * matrix.cols[1][2] + point.z * matrix.cols[2][2] + matrix.cols[3][2],
        point.x * matrix.cols[0][3] + point.y * matrix.cols[1][3] + point.z * matrix.cols[2][3] + matrix.cols[3][3],
    ]
}

fn clip_near_far(points: &[[f32; 4]; 3]) -> Vec<[f32; 4]> {
    let mut input = points.to_vec();
    for far in [false, true] {
        let mut output = Vec::with_capacity(5);
        for index in 0..input.len() {
            let a = input[index];
            let b = input[(index + 1) % input.len()];
            let da = if far { a[3] - a[2] } else { a[3] + a[2] };
            let db = if far { b[3] - b[2] } else { b[3] + b[2] };
            match (da >= 0.0, db >= 0.0) {
                (true, true) => output.push(a),
                (true, false) => {
                    output.push(a);
                    output.push(lerp4(a, b, da / (da - db)));
                }
                (false, true) => output.push(lerp4(a, b, da / (da - db))),
                (false, false) => {}
            }
        }
        input = output;
        if input.is_empty() {
            break;
        }
    }
    input
}

fn clip_line_near_far(mut points: [[f32; 4]; 2]) -> Option<[[f32; 4]; 2]> {
    for far in [false, true] {
        let da = if far { points[0][3] - points[0][2] } else { points[0][3] + points[0][2] };
        let db = if far { points[1][3] - points[1][2] } else { points[1][3] + points[1][2] };
        if da < 0.0 && db < 0.0 {
            return None;
        }
        if da < 0.0 {
            points[0] = lerp4(points[0], points[1], da / (da - db));
        } else if db < 0.0 {
            points[1] = lerp4(points[0], points[1], da / (da - db));
        }
    }
    Some(points)
}

fn lerp4(a: [f32; 4], b: [f32; 4], t: f32) -> [f32; 4] {
    std::array::from_fn(|axis| a[axis] + (b[axis] - a[axis]) * t)
}

fn project(point: [f32; 4], dimensions: (u32, u32)) -> [f32; 3] {
    let inverse_w = if point[3].abs() < 1e-8 { 1.0 } else { 1.0 / point[3] };
    [point[0] * inverse_w * dimensions.0 as f32 * 0.5, -point[1] * inverse_w * dimensions.1 as f32 * 0.5, point[2] * inverse_w]
}

/// 🔄️ three's `Projector.checkBackfaceCulling` judges winding in NDC, y up (`< 0` is a front face); [`project`] writes y-down
/// SVG space, which mirrors every winding, so a front face is positive here.
fn front_facing(points: [[f32; 3]; 3]) -> bool {
    (points[2][0] - points[0][0]) * (points[1][1] - points[0][1]) - (points[2][1] - points[0][1]) * (points[1][0] - points[0][0]) > 0.0
}

fn visible(points: [[f32; 3]; 3], dimensions: (u32, u32)) -> bool {
    let half = [dimensions.0 as f32 * 0.5, dimensions.1 as f32 * 0.5];
    let minimum = [0, 1].map(|axis| points.iter().map(|point| point[axis]).fold(f32::INFINITY, f32::min));
    let maximum = [0, 1].map(|axis| points.iter().map(|point| point[axis]).fold(f32::NEG_INFINITY, f32::max));
    maximum[0] >= -half[0] && minimum[0] <= half[0] && maximum[1] >= -half[1] && minimum[1] <= half[1]
}

fn visible_line(points: [[f32; 3]; 2], dimensions: (u32, u32)) -> bool {
    let half = [dimensions.0 as f32 * 0.5, dimensions.1 as f32 * 0.5];
    points[0][0].max(points[1][0]) >= -half[0]
        && points[0][0].min(points[1][0]) <= half[0]
        && points[0][1].max(points[1][1]) >= -half[1]
        && points[0][1].min(points[1][1]) <= half[1]
}

fn overdraw(points: &mut [[f32; 3]; 3], pixels: f32) {
    expand(points, 0, 1, pixels);
    expand(points, 1, 2, pixels);
    expand(points, 2, 0, pixels);
}

fn expand(points: &mut [[f32; 3]; 3], first: usize, second: usize, pixels: f32) {
    let dx = points[second][0] - points[first][0];
    let dy = points[second][1] - points[first][1];
    let length = dx.hypot(dy);
    if length <= f32::EPSILON {
        return;
    }
    let x = dx * pixels / length;
    let y = dy * pixels / length;
    points[second][0] += x;
    points[second][1] += y;
    points[first][0] -= x;
    points[first][1] -= y;
}

fn vec3(value: [f32; 3]) -> Vec3 {
    Vec3 { x: value[0], y: value[1], z: value[2] }
}

fn css_rgb(linear: [f32; 3]) -> String {
    let values = linear.map(|channel| {
        let channel = channel.clamp(0.0, 1.0);
        let encoded = if channel <= 0.003_130_8 { channel * 12.92 } else { 1.055 * channel.powf(1.0 / 2.4) - 0.055 };
        (encoded * 255.0).round() as u8
    });
    format!("rgb({},{},{})", values[0], values[1], values[2])
}

fn scalar(value: f32) -> String {
    if value.abs() < 0.000_000_5 {
        return "0".into();
    }
    let mut value = format!("{value:.6}");
    while value.ends_with('0') {
        value.pop();
    }
    if value.ends_with('.') {
        value.pop();
    }
    value
}

fn append_output(output: &mut Option<Vec<u8>>, value: &str) -> Result<(), String> {
    let output = output.get_or_insert_with(Vec::new);
    let next = output.len().checked_add(value.len()).ok_or("icon SVG byte count overflowed")?;
    if next > SVG_OUTPUT_CAPACITY {
        return Err("icon SVG exceeded the output byte budget".into());
    }
    output.extend_from_slice(value.as_bytes());
    Ok(())
}

#[cfg(test)]
#[path = "../../../../🧪️tests/📤️svg-export/🦀️.rs"]
mod tests;
