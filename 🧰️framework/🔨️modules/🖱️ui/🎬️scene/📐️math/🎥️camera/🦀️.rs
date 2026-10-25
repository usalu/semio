//! 🪐️ Orbit and parallel cameras, projection-spec matrices, framing, gumball and ray math, and the level-of-detail grid — the renderer-neutral camera half of the scene math.
//!
//! Carved out of the former `semio-framework-ui-scene` math module by ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`.

use semio_framework_geometry::{Mat4, Vec3};
use semio_framework_ui_scene_algebra::{mat4_look_at_m, mat4_orthographic_m, mat4_perspective_m, vec3_new_m, Mat4Math, Vec3Math};
use semio_framework_ui_viewport::{
    Viewport3dAxonometricHemisphere, Viewport3dAxonometricQuadrant, Viewport3dAxonometricVariant, Viewport3dObliqueVariant, Viewport3dOrthographicView, Viewport3dProjectionMode, Viewport3dProjectionOrientation, Viewport3dProjectionSpec,
};

//#region Camera
/// ⚛️ The two camera families React mounts — drei's `<PerspectiveCamera>` and its
/// `<OrthographicCamera>`. React derives it from the pane's projection spec
/// (`worldProjectionFamily`, `🎨️r3f/🟦️.tsx`): `orthographic | axonometric | oblique` are parallel,
/// every other mode is perspective.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CameraProjection3d {
    #[default]
    Perspective,
    Orthographic,
}

impl CameraProjection3d {
    /// ➡️ React's `worldProjectionFamily` over the wire's mode kind.
    pub fn from_mode_kind(kind: &str) -> Self {
        match kind {
            "orthographic" | "axonometric" | "oblique" => Self::Orthographic,
            _ => Self::Perspective,
        }
    }

    pub fn is_parallel(self) -> bool {
        matches!(self, Self::Orthographic)
    }
}

/// 🎬️ Resolves the camera family from the renderer-neutral projection schema.
pub fn projection_spec_family(spec: Viewport3dProjectionSpec) -> CameraProjection3d {
    match spec.mode {
        Viewport3dProjectionMode::Orthographic {} | Viewport3dProjectionMode::Axonometric { .. } | Viewport3dProjectionMode::Oblique { .. } => CameraProjection3d::Orthographic,
        _ => CameraProjection3d::Perspective,
    }
}

/// 👁️ Resolves the effective perspective field of view used by React's projection rig.
pub fn projection_spec_fov_degrees(spec: Viewport3dProjectionSpec) -> f32 {
    match spec.mode {
        Viewport3dProjectionMode::OnePoint { fov } | Viewport3dProjectionMode::TwoPoint { fov, .. } | Viewport3dProjectionMode::ThreePoint { fov } => fov as f32,
        Viewport3dProjectionMode::Curvilinear { fov, .. } => (fov as f32).min(160.0),
        _ => 50.0,
    }
}

/// 🪞️ Builds React's oblique receding-axis shear in the scene matrix convention.
pub fn projection_spec_oblique_shear(mode: Viewport3dProjectionMode) -> Mat4 {
    let Viewport3dProjectionMode::Oblique { variant, angle, depth_scale } = mode else { return Mat4::identity() };
    let alpha = if variant == Viewport3dObliqueVariant::Military { std::f32::consts::FRAC_PI_2 } else { (angle as f32).to_radians() };
    let length = if variant == Viewport3dObliqueVariant::Military { 1.0 } else { depth_scale as f32 };
    let mut shear = Mat4::identity();
    shear.cols[2][0] = -length * alpha.cos();
    shear.cols[2][1] = -length * alpha.sin();
    shear
}

/// 🏗️ Builds the exact Three projection matrix for a complete viewport projection spec.
pub fn projection_spec_matrix(camera: &Camera3d, spec: Viewport3dProjectionSpec, width: f32, height: f32) -> Mat4 {
    let mut camera = camera.clone();
    camera.projection = projection_spec_family(spec);
    camera.fov_y = projection_spec_fov_degrees(spec).to_radians();
    let mut projection = camera.projection_matrix(width, height);
    if matches!(spec.mode, Viewport3dProjectionMode::Oblique { .. }) {
        projection = projection.mul_m(projection_spec_oblique_shear(spec.mode));
    } else if let Viewport3dProjectionMode::TwoPoint { vertical_shift, .. } = spec.mode {
        projection.cols[2][1] += vertical_shift as f32;
    }
    projection
}

/// 🛰️ Composes one accepted projection spec with the live orbit view matrix.
pub fn projection_spec_view_proj(camera: &Camera3d, spec: Viewport3dProjectionSpec, width: f32, height: f32) -> Mat4 {
    projection_spec_matrix(camera, spec, width, height).mul_m(camera.view_matrix())
}

/// 🐟️ Maps visible curvilinear NDC back into the linear capture used by render and picking.
pub fn projection_spec_unproject_ndc(spec: Viewport3dProjectionSpec, ndc: [f32; 2], aspect: f32) -> [f32; 2] {
    let Viewport3dProjectionMode::Curvilinear { fov, strength, .. } = spec.mode else { return ndc };
    let half_fov = ((fov as f32).min(160.0).to_radians()) * 0.5;
    let scaled = [ndc[0] * aspect, ndc[1]];
    let radius = scaled[0].hypot(scaled[1]);
    if radius < 1e-6 {
        return ndc;
    }
    let rectilinear_radius = (radius * half_fov).tan() / half_fov.tan();
    let source_radius = radius + (rectilinear_radius - radius) * strength as f32;
    let scale = source_radius / radius;
    [scaled[0] * scale / aspect, scaled[1] * scale]
}

/// 📍️ Maps one linear capture-space NDC point onto the visible curvilinear surface.
pub fn projection_spec_project_ndc(spec: Viewport3dProjectionSpec, ndc: [f32; 2], aspect: f32) -> [f32; 2] {
    let Viewport3dProjectionMode::Curvilinear { fov, strength, .. } = spec.mode else { return ndc };
    let aspect = aspect.max(1e-6);
    let scaled = [ndc[0] * aspect, ndc[1]];
    let source_radius = scaled[0].hypot(scaled[1]);
    let strength = strength as f32;
    if source_radius < 1e-6 || strength < 1e-6 {
        return ndc;
    }
    let half_fov = ((fov as f32).min(160.0).to_radians()) * 0.5;
    let tangent = half_fov.tan();
    let limit = (std::f32::consts::FRAC_PI_2 - 1e-5) / half_fov.max(1e-6);
    let center_slope = 1.0 - strength + strength * half_fov / tangent;
    let mut visible_radius = (source_radius / center_slope.max(1e-6)).clamp(0.0, limit);
    for _ in 0..8 {
        let warped = (visible_radius * half_fov).tan();
        let capture_radius = visible_radius + (warped / tangent - visible_radius) * strength;
        let slope = 1.0 - strength + strength * half_fov * (1.0 + warped * warped) / tangent;
        visible_radius = (visible_radius - (capture_radius - source_radius) / slope.max(1e-6)).clamp(0.0, limit);
    }
    let scale = visible_radius / source_radius;
    [scaled[0] * scale / aspect, scaled[1] * scale]
}

/// 🌐️ Projects a world point through the linear camera and the accepted curvilinear image pass.
pub fn projection_spec_project_point(view_proj: Mat4, spec: Viewport3dProjectionSpec, point: Vec3, width: f32, height: f32) -> Option<[f32; 2]> {
    let clip = view_proj.transform_point_m(point);
    if clip.z < 0.0 || clip.z > 1.0 {
        return None;
    }
    let visible = projection_spec_project_ndc(spec, [clip.x, clip.y], width.max(1.0) / height.max(1.0));
    Some([(visible[0] * 0.5 + 0.5) * width, (1.0 - (visible[1] * 0.5 + 0.5)) * height])
}

/// 🧵️ World and screen endpoints of a segment within the accepted camera depth range.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectedSegment3d {
    pub points: [Vec3; 2],
    pub screen: [[f32; 2]; 2],
}

/// 🪡️ Projects an authored segment through the accepted camera and image transform.
pub fn projection_spec_project_segment(view_proj: Mat4, spec: Viewport3dProjectionSpec, a: Vec3, b: Vec3, width: f32, height: f32) -> Option<ProjectedSegment3d> {
    if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 { return None; }
    let homogeneous = |point: Vec3| std::array::from_fn::<_, 4, _>(|row| f64::from(view_proj.cols[0][row]) * f64::from(point.x) + f64::from(view_proj.cols[1][row]) * f64::from(point.y) + f64::from(view_proj.cols[2][row]) * f64::from(point.z) + f64::from(view_proj.cols[3][row]));
    let start = homogeneous(a);
    let end = homogeneous(b);
    if start.iter().chain(end.iter()).any(|value| !value.is_finite()) { return None; }
    let mut lower = 0.0_f64;
    let mut upper = 1.0_f64;
    for (first, last) in [(start[2], end[2]), (start[3] - start[2], end[3] - end[2])] {
        if first < 0.0 && last < 0.0 { return None; }
        if first < 0.0 { lower = lower.max(first / (first - last)); }
        if last < 0.0 { upper = upper.min(first / (first - last)); }
    }
    if lower > upper { return None; }
    let project = |parameter: f64| {
        let clip: [f64; 4] = std::array::from_fn(|axis| start[axis] + (end[axis] - start[axis]) * parameter);
        if clip[3] <= 0.0 { return None; }
        let visible = projection_spec_project_ndc(spec, [(clip[0] / clip[3]) as f32, (clip[1] / clip[3]) as f32], width / height);
        let screen = [(visible[0] * 0.5 + 0.5) * width, (1.0 - (visible[1] * 0.5 + 0.5)) * height];
        screen.iter().all(|value| value.is_finite()).then_some(screen)
    };
    let point = |parameter: f64| vec3_new_m((f64::from(a.x) + (f64::from(b.x) - f64::from(a.x)) * parameter) as f32, (f64::from(a.y) + (f64::from(b.y) - f64::from(a.y)) * parameter) as f32, (f64::from(a.z) + (f64::from(b.z) - f64::from(a.z)) * parameter) as f32);
    Some(ProjectedSegment3d { points: [point(lower), point(upper)], screen: [project(lower)?, project(upper)?] })
}

/// 🏠️ Returns the shared default for a newly mounted perspective World surface.
pub fn default_projection_spec() -> Viewport3dProjectionSpec {
    Viewport3dProjectionSpec { mode: Viewport3dProjectionMode::ThreePoint { fov: 50.0 }, orientation: Viewport3dProjectionOrientation::Free {} }
}

/// 🧭️ Resolves React's mode × orientation look basis without collapsing corner parameters.
pub fn projection_spec_orientation_look(spec: Viewport3dProjectionSpec) -> (Vec3, Vec3) {
    let cardinal = |view| match view {
        Viewport3dOrthographicView::Plan | Viewport3dOrthographicView::Top => (vec3_new_m(0.0, 0.0, 1.0), vec3_new_m(0.0, 1.0, 0.0)),
        Viewport3dOrthographicView::Bottom => (vec3_new_m(0.0, 0.0, -1.0), vec3_new_m(0.0, -1.0, 0.0)),
        Viewport3dOrthographicView::Front => (vec3_new_m(0.0, -1.0, 0.0), vec3_new_m(0.0, 0.0, 1.0)),
        Viewport3dOrthographicView::Back => (vec3_new_m(0.0, 1.0, 0.0), vec3_new_m(0.0, 0.0, 1.0)),
        Viewport3dOrthographicView::Left => (vec3_new_m(-1.0, 0.0, 0.0), vec3_new_m(0.0, 0.0, 1.0)),
        Viewport3dOrthographicView::Right => (vec3_new_m(1.0, 0.0, 0.0), vec3_new_m(0.0, 0.0, 1.0)),
    };
    match spec.orientation {
        Viewport3dProjectionOrientation::Cardinal { view } => {
            if let Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Military, angle, .. } = spec.mode {
                if matches!(view, Viewport3dOrthographicView::Plan | Viewport3dOrthographicView::Top) {
                    let rotation = (angle as f32).to_radians();
                    return (vec3_new_m(0.0, 0.0, 1.0), vec3_new_m(rotation.sin(), rotation.cos(), 0.0));
                }
            }
            cardinal(view)
        }
        Viewport3dProjectionOrientation::Corner { quadrant, hemisphere } => {
            let (angle_a, angle_b) = match spec.mode {
                Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Isometric, .. } => (30.0, 30.0),
                Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Dimetric, angle_a, .. } => (angle_a as f32, angle_a as f32),
                Viewport3dProjectionMode::Axonometric { angle_a, angle_b, .. } => (angle_a as f32, angle_b as f32),
                _ => (30.0, 30.0),
            };
            let (angle_a, angle_b) = (angle_a.to_radians(), angle_b.to_radians());
            let elevation = (angle_a.tan() * angle_b.tan()).sqrt().asin();
            let azimuth = (angle_a.tan() / angle_b.tan()).sqrt().atan();
            let sign_x = if matches!(quadrant, Viewport3dAxonometricQuadrant::Nw | Viewport3dAxonometricQuadrant::Sw) { -1.0 } else { 1.0 };
            let sign_y = if matches!(quadrant, Viewport3dAxonometricQuadrant::Se | Viewport3dAxonometricQuadrant::Sw) { -1.0 } else { 1.0 };
            let lower = hemisphere == Some(Viewport3dAxonometricHemisphere::Lower);
            (vec3_new_m(sign_x * elevation.cos() * azimuth.sin(), sign_y * elevation.cos() * azimuth.cos(), elevation.sin() * if lower { -1.0 } else { 1.0 }), vec3_new_m(0.0, 0.0, if lower { -1.0 } else { 1.0 }))
        }
        Viewport3dProjectionOrientation::Free {} => match spec.mode {
            Viewport3dProjectionMode::TwoPoint { .. } => (vec3_new_m(std::f32::consts::FRAC_1_SQRT_2, -std::f32::consts::FRAC_1_SQRT_2, 0.0), vec3_new_m(0.0, 0.0, 1.0)),
            Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Military, angle, .. } => {
                let rotation = (angle as f32).to_radians();
                (vec3_new_m(0.0, 0.0, 1.0), vec3_new_m(rotation.sin(), rotation.cos(), 0.0))
            }
            Viewport3dProjectionMode::Oblique { .. } => (vec3_new_m(0.0, -1.0, 0.0), vec3_new_m(0.0, 0.0, 1.0)),
            _ => (vec3_new_m(0.75, -0.75, 0.55).normalize_m(), vec3_new_m(0.0, 0.0, 1.0)),
        },
    }
}

/// 📡️ Builds a pick ray through the same matrix and curvilinear inverse used by presentation.
pub fn projection_spec_ray_from_screen(camera: &Camera3d, spec: Viewport3dProjectionSpec, x: f32, y: f32, width: f32, height: f32) -> (Vec3, Vec3) {
    let visible_ndc = [(x / width.max(1.0)) * 2.0 - 1.0, 1.0 - (y / height.max(1.0)) * 2.0];
    let ndc = projection_spec_unproject_ndc(spec, visible_ndc, width.max(1.0) / height.max(1.0));
    let inv = projection_spec_view_proj(camera, spec, width, height).inverse_m();
    match projection_spec_family(spec) {
        CameraProjection3d::Orthographic => (inv.transform_point_m(vec3_new_m(ndc[0], ndc[1], 0.0)), camera.target.sub_m(camera.position).normalize_m()),
        CameraProjection3d::Perspective => {
            let middle = inv.transform_point_m(vec3_new_m(ndc[0], ndc[1], 0.5));
            (camera.position, middle.sub_m(camera.position).normalize_m())
        }
    }
}

/// 🎥️ A world camera in React's exact terms: a pose, a family, and a `zoom` whose meaning is the
/// family's (`fov` divisor under perspective, pixels-per-world-unit under the parallel frustum).
#[derive(Clone, Debug)]
pub struct Camera3d {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
    pub projection: CameraProjection3d,
    /// 🔎️ three.js camera `zoom`. Perspective divides `tan(fov/2)`; orthographic divides the pixel
    /// frustum's half-extent, so it reads as pixels per world unit.
    pub zoom: f32,
}

impl Default for Camera3d {
    fn default() -> Self {
        Self {
            position: vec3_new_m(4.0, -4.0, 3.0),
            target: Vec3::ZERO,
            up: vec3_new_m(0.0, 0.0, 1.0),
            fov_y: 45.0_f32.to_radians(),
            near: WORLD_ORBIT_CAMERA_NEAR,
            far: WORLD_ORBIT_CAMERA_MIN_FAR,
            projection: CameraProjection3d::Perspective,
            zoom: 1.0,
        }
    }
}

impl Camera3d {
    /// 🔳️ Half of the parallel frustum in world units — drei seeds `left = -width / 2` … from the
    /// canvas size in PIXELS, so a `zoom` of 1 shows exactly `width` world units across
    /// (`WorldProjectionContentFrame`, `🌐️World3dHost/🟦️.tsx`).
    pub fn orthographic_half_extent(&self, width: f32, height: f32) -> (f32, f32) {
        let zoom = self.zoom.max(1e-6);
        (width.max(1.0) * 0.5 / zoom, height.max(1.0) * 0.5 / zoom)
    }

    pub fn projection_matrix(&self, width: f32, height: f32) -> Mat4 {
        match self.projection {
            CameraProjection3d::Orthographic => {
                let (half_width, half_height) = self.orthographic_half_extent(width, height);
                mat4_orthographic_m(half_width, half_height, self.near, self.far)
            }
            CameraProjection3d::Perspective => {
                let aspect = (width.max(1.0) / height.max(1.0)).max(0.05);
                let half = ((self.fov_y * 0.5).tan() / self.zoom.max(1e-6)).atan();
                mat4_perspective_m((half * 2.0).clamp(0.01, 3.0), aspect, self.near, self.far)
            }
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        mat4_look_at_m(self.position, self.target, self.stable_up())
    }

    pub fn view_proj(&self, width: f32, height: f32) -> Mat4 {
        self.projection_matrix(width, height).mul_m(self.view_matrix())
    }

    /// 🔦️ The world-space pick ray through a viewport pixel, built the way three's
    /// `Raycaster::setFromCamera` builds it.
    ///
    /// 🩸️ It used to be `unproject(ndc, z = 1) - unproject(ndc, z = 0)`. At React's adaptive far
    /// plane (`near = 0.2`, `far ≥ 524 288`) the inverse view-projection's near-plane entries are
    /// around `1e-6`, so that difference is numerically empty in `f32` and EVERY pick missed — which
    /// is why the far plane stayed pinned at 1 000 for a whole packet
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w7b-presenter-one-frame-per-boot.md` §5.1).
    /// three never has that problem: a perspective ray starts at the camera POSITION and takes one
    /// mid-depth unprojection, a parallel ray starts on the near plane and runs down the view axis.
    pub fn ray_from_screen(&self, x: f32, y: f32, width: f32, height: f32) -> (Vec3, Vec3) {
        let ndc_x = (x / width.max(1.0)) * 2.0 - 1.0;
        let ndc_y = 1.0 - (y / height.max(1.0)) * 2.0;
        let inv = self.view_proj(width, height).inverse_m();
        match self.projection {
            CameraProjection3d::Orthographic => {
                let origin = inv.transform_point_m(vec3_new_m(ndc_x, ndc_y, 0.0));
                (origin, self.target.sub_m(self.position).normalize_m())
            }
            CameraProjection3d::Perspective => {
                let middle = inv.transform_point_m(vec3_new_m(ndc_x, ndc_y, 0.5));
                (self.position, middle.sub_m(self.position).normalize_m())
            }
        }
    }

    /// ⬆️ The requested up vector, or the world twin that is not parallel to the view axis — a plan
    /// view whose up is `+Z` degenerates `look_at`'s cross product into `NaN`, which is exactly what
    /// a `Top` pane seeded with the orbit's hard-wired Z-up used to render.
    fn stable_up(&self) -> Vec3 {
        let forward = self.target.sub_m(self.position);
        let up = self.up;
        let length = up.length_m();
        let candidate = if length > 1e-6 { up.scale_m(1.0 / length) } else { vec3_new_m(0.0, 0.0, 1.0) };
        if forward.normalize_m().cross_m(candidate).length_m() > 1e-4 {
            candidate
        } else if candidate.z.abs() > 0.5 {
            vec3_new_m(0.0, 1.0, 0.0)
        } else {
            vec3_new_m(0.0, 0.0, 1.0)
        }
    }
}

#[derive(Clone, Debug)]
pub struct OrbitController {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_y: f32,
    pub projection: CameraProjection3d,
    pub zoom: f32,
    pub up: Vec3,
}

impl Default for OrbitController {
    fn default() -> Self {
        Self { target: Vec3::ZERO, distance: 8.0, yaw: 0.8, pitch: 0.5, fov_y: 45.0_f32.to_radians(), projection: CameraProjection3d::Perspective, zoom: 1.0, up: vec3_new_m(0.0, 0.0, 1.0) }
    }
}

impl OrbitController {
    pub fn from_camera(camera: &Camera3d) -> Self {
        let offset = camera.position.sub_m(camera.target);
        let distance = offset.length_m().max(0.5);
        Self { target: camera.target, distance, yaw: offset.y.atan2(offset.x), pitch: (offset.z / distance).clamp(-1.0, 1.0).asin(), fov_y: camera.fov_y, projection: camera.projection, zoom: camera.zoom, up: camera.up }
    }

    pub fn to_camera(&self) -> Camera3d {
        let cp = self.pitch.cos();
        let position = vec3_new_m(self.target.x + self.distance * cp * self.yaw.cos(), self.target.y + self.distance * cp * self.yaw.sin(), self.target.z + self.distance * self.pitch.sin());
        Camera3d { position, target: self.target, up: self.up, fov_y: self.fov_y, near: WORLD_ORBIT_CAMERA_NEAR, far: adaptive_orbit_camera_far(self.distance), projection: self.projection, zoom: self.zoom }
    }

    pub fn orbit(&mut self, dx: f32, dy: f32) {
        self.yaw -= dx * 0.01;
        self.pitch = (self.pitch + dy * 0.01).clamp(-1.5, 1.5);
    }

    /// 🖐️ Screen-parallel pan. Under the parallel frustum a pixel is exactly `1 / zoom` world units,
    /// which is three's `OrbitControls.panLeft/panUp` orthographic branch; under perspective the
    /// existing distance-proportional scale stands in for its `targetDistance * tan(fov/2)`.
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let camera = self.to_camera();
        let right = camera.position.sub_m(camera.target).cross_m(camera.stable_up()).normalize_m();
        let up = right.cross_m(camera.position.sub_m(camera.target)).normalize_m();
        let scale = if self.projection.is_parallel() { 1.0 / self.zoom.max(1e-6) } else { self.distance * 0.001 };
        self.target = self.target.add_m(right.scale_m(-dx * scale)).add_m(up.scale_m(dy * scale));
    }

    /// 🔍️ Wheel dolly. A parallel camera cannot dolly — three's `OrbitControls` scales its `zoom`
    /// instead, which is why React's `captureNavigationSnapshot` reports the live `camera.zoom` for
    /// an orthographic pane and a bare `1` for a perspective one.
    pub fn zoom(&mut self, delta: f32) {
        if self.projection.is_parallel() {
            self.zoom = (self.zoom * (1.0 + delta * 0.001)).clamp(WORLD_ORBIT_PARALLEL_ZOOM_MIN, WORLD_ORBIT_PARALLEL_ZOOM_MAX);
        } else {
            self.distance = (self.distance * (1.0 - delta * 0.001)).clamp(0.5, 500.0);
        }
    }
}

/// 🎯️ The headroom a boot framing leaves around the delivered bounds, as a multiple of the exact
/// fitting distance. 12 % is the difference between "the outermost vertex sits on the viewport
/// edge" and "the user sees the whole example with air around it".
pub const WORLD_FRAME_BOUNDS_MARGIN: f32 = 1.12;

/// 🔭️ How far a perspective eye must stand off a bounding sphere of `radius` for the WHOLE sphere
/// to project inside a `fov_y`/`aspect` frustum, times `margin`.
///
/// ⚖️ The BINDING half-angle is the smaller of the vertical and horizontal ones: a wide viewport
/// (`aspect > 1`) is limited by its vertical field, a tall one by its horizontal field. Fitting a
/// box by `radius * constant` — which is what every ad-hoc fit rule in this repo used to do — is
/// short by a factor of `1 / sin(half_angle)` ≈ 2.6 at the default 45° field, which is exactly how
/// a converged example arrived clipped at the corner of the preview
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
pub fn frame_distance_for_radius(radius: f32, fov_y: f32, aspect: f32, margin: f32) -> f32 {
    let vertical = (fov_y * 0.5).clamp(0.02, 1.5);
    let horizontal = (vertical.tan() * aspect.max(0.05)).atan().clamp(0.02, 1.5);
    let half = vertical.min(horizontal);
    (radius.max(1e-4) / half.sin() * margin.max(1.0)).max(0.5)
}

/// 📷️ The parallel `zoom` that fits a half-extent into drei's pixel frustum — React's
/// `worldProjectionOrthoZoom` (`🎨️r3f/🟦️.tsx`), the binding axis winning.
pub fn world_projection_ortho_zoom(half_width: f32, half_height: f32, viewport_width: f32, viewport_height: f32, padding: f32) -> f32 {
    let padded_half_w = (half_width * padding).max(0.5);
    let padded_half_h = (half_height * padding).max(0.5);
    let zoom_x = viewport_width.max(1.0) * 0.5 / padded_half_w;
    let zoom_y = viewport_height.max(1.0) * 0.5 / padded_half_h;
    zoom_x.min(zoom_y).max(1e-3)
}

/// 🎲️ The six cardinal views React's `WorldProjectionSpec` orientation can name
/// (`worldProjectionCardinalLook`, `🎨️r3f/🟦️.tsx`). `plan` is React's own alias for `top`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorldCardinalView {
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
}

impl WorldCardinalView {
    /// 🔤️ The wire spelling React authors (`orientation.view`), with `plan` folded onto `top`.
    pub fn from_wire(view: &str) -> Option<Self> {
        match view {
            "top" | "plan" => Some(Self::Top),
            "bottom" => Some(Self::Bottom),
            "front" => Some(Self::Front),
            "back" => Some(Self::Back),
            "left" => Some(Self::Left),
            "right" => Some(Self::Right),
            _ => None,
        }
    }
}

/// 🔄️ React's `WorldProjectionSpec["orientation"]` reduced to what the framing reads: which cardinal
/// view a pane is locked to, or a free one.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WorldProjectionOrientation {
    Cardinal(WorldCardinalView),
    #[default]
    Free,
}

/// 📺️ React's `worldProjectionViewHalfExtent` (`🎨️r3f/🟦️.tsx`) VERBATIM: the half-width/height of a
/// content box in the projection's own view plane, read off the box's CARDINAL half-extent.
///
/// ⚖️ A cardinal view takes the two axes its plane spans — `(hx, hy)` for top/bottom, `(hx, hz)` for
/// front/back, `(hy, hz)` for left/right. A FREE oblique (any variant but `military`) takes
/// `(hx, hz)`. Everything else — axonometric, a free perspective — takes the ISOTROPIC span
/// `max(hx, hy, hz)` on both axes, because a box seen down a corner has no stable view plane to
/// measure in and React refuses to pretend otherwise.
///
/// 🩸️ The wgpu twin used [`screen_half_extent`] instead — the eight corners projected onto the live
/// camera's screen basis. The two agree for an exactly cardinal camera and DIVERGE for every other
/// orientation, which is precisely the set the projection pane can now select
/// (`📓️w9b-projection-pane-framing-grid-materials.md` §2).
pub fn world_projection_view_half_extent(orientation: WorldProjectionOrientation, oblique_off_axis: bool, half_extent: [f32; 3]) -> (f32, f32) {
    let [hx, hy, hz] = half_extent;
    if let WorldProjectionOrientation::Cardinal(view) = orientation {
        return match view {
            WorldCardinalView::Front | WorldCardinalView::Back => (hx, hz),
            WorldCardinalView::Left | WorldCardinalView::Right => (hy, hz),
            WorldCardinalView::Top | WorldCardinalView::Bottom => (hx, hy),
        };
    }
    if oblique_off_axis {
        return (hx, hz);
    }
    let span = hx.max(hy).max(hz);
    (span, span)
}

/// 🔲️ React's `frameWorldProjectionPose` for a PARALLEL pane: the content box's view-plane half
/// extent through [`world_projection_view_half_extent`], then [`world_projection_ortho_zoom`] into
/// drei's pixel frustum. The look direction is the pane's own — React rebuilds it from the spec,
/// and this renderer is already sitting on the pose that spec delivered.
pub fn frame_projection_orbit_to_bounds(orbit: &OrbitController, orientation: WorldProjectionOrientation, oblique_off_axis: bool, minimum: [f32; 3], maximum: [f32; 3], width: f32, height: f32, padding: f32) -> OrbitController {
    let center = vec3_new_m((minimum[0] + maximum[0]) * 0.5, (minimum[1] + maximum[1]) * 0.5, (minimum[2] + maximum[2]) * 0.5);
    let half_extent = [0, 1, 2].map(|axis| (maximum[axis] - minimum[axis]) * 0.5);
    let (half_width, half_height) = world_projection_view_half_extent(orientation, oblique_off_axis, half_extent);
    OrbitController { target: center, zoom: world_projection_ortho_zoom(half_width, half_height, width, height, padding.max(1.0)), ..orbit.clone() }
}

/// 📸️ Frames a camera from the complete viewport projection spec, matching React's
/// `frameWorldProjectionPose`: the spec owns look direction, up, family and FOV; content owns target,
/// distance and, for a parallel family, the viewport-aware zoom.
pub fn frame_projection_orbit_to_spec_bounds(_orbit: &OrbitController, spec: Viewport3dProjectionSpec, minimum: [f32; 3], maximum: [f32; 3], width: f32, height: f32, padding: f32) -> OrbitController {
    let center = vec3_new_m((minimum[0] + maximum[0]) * 0.5, (minimum[1] + maximum[1]) * 0.5, (minimum[2] + maximum[2]) * 0.5);
    let [hx, hy, hz] = [0, 1, 2].map(|axis| (maximum[axis] - minimum[axis]) * 0.5);
    let plane = match spec.orientation {
        Viewport3dProjectionOrientation::Cardinal { view: Viewport3dOrthographicView::Front | Viewport3dOrthographicView::Back } => (hx, hz),
        Viewport3dProjectionOrientation::Cardinal { view: Viewport3dOrthographicView::Left | Viewport3dOrthographicView::Right } => (hy, hz),
        Viewport3dProjectionOrientation::Cardinal { .. } => (hx, hy),
        Viewport3dProjectionOrientation::Free {} if matches!(spec.mode, Viewport3dProjectionMode::Oblique { variant, .. } if variant != Viewport3dObliqueVariant::Military) => (hx, hz),
        _ => {
            let span = hx.max(hy).max(hz);
            (span, span)
        }
    };
    let padding = padding.max(1.0);
    let span = plane.0.max(plane.1) * 2.0 * padding;
    let distance = (span * 1.5).max(2.0);
    let (direction, up) = projection_spec_orientation_look(spec);
    let direction = direction.normalize_m();
    let projection = projection_spec_family(spec);
    OrbitController {
        target: center,
        distance,
        yaw: direction.y.atan2(direction.x),
        pitch: direction.z.clamp(-1.0, 1.0).asin(),
        fov_y: projection_spec_fov_degrees(spec).to_radians(),
        projection,
        zoom: if projection.is_parallel() { world_projection_ortho_zoom(plane.0, plane.1, width, height, padding) } else { 1.0 },
        up,
    }
}

/// 🖼️ Frames an orbit on an axis-aligned box while keeping its current look direction — the ONE
/// framing rule the wgpu world surface and its React twin (`world3dFrameOrbitToBounds`) both obey.
///
/// ⚖️ React's `WorldAutoFit` uses the sphere-fit formula for perspective and parallel
/// cameras. A parallel pane's independent projection-content frame owns `zoom`; auto-fit preserves
/// that value while moving the eye and target onto rendered geometry. Three's orthographic camera
/// exposes no aspect ratio, so that family's fit uses one.
pub fn frame_orbit_to_bounds(orbit: &OrbitController, minimum: [f32; 3], maximum: [f32; 3], width: f32, height: f32, margin: f32) -> OrbitController {
    let center = vec3_new_m((minimum[0] + maximum[0]) * 0.5, (minimum[1] + maximum[1]) * 0.5, (minimum[2] + maximum[2]) * 0.5);
    let radius = (((maximum[0] - minimum[0]).powi(2) + (maximum[1] - minimum[1]).powi(2) + (maximum[2] - minimum[2]).powi(2)).sqrt()) * 0.5;
    let aspect = if orbit.projection.is_parallel() { 1.0 } else { (width.max(1.0) / height.max(1.0)).max(0.05) };
    OrbitController { target: center, distance: frame_distance_for_radius(radius, orbit.fov_y, aspect, margin), ..orbit.clone() }
}

/// 🗃️ Half-extent of an axis-aligned box on the camera's own screen axes — React's
/// `worldProjectionViewHalfExtent` generalised past its cardinal cases: it reads `(hx, hy)` for a
/// plan view and `(hy, hz)` for a side one, which is exactly what projecting the eight corners onto
/// the view basis answers for those orientations, and stays correct for every other one.
pub fn screen_half_extent(camera: &Camera3d, minimum: [f32; 3], maximum: [f32; 3]) -> (f32, f32) {
    let forward = camera.target.sub_m(camera.position).normalize_m();
    let right = forward.cross_m(camera.stable_up()).normalize_m();
    let up = right.cross_m(forward).normalize_m();
    let centre = vec3_new_m((minimum[0] + maximum[0]) * 0.5, (minimum[1] + maximum[1]) * 0.5, (minimum[2] + maximum[2]) * 0.5);
    let mut half_width = 0.0_f32;
    let mut half_height = 0.0_f32;
    for corner in 0..8 {
        let point = vec3_new_m(if corner & 1 == 0 { minimum[0] } else { maximum[0] }, if corner & 2 == 0 { minimum[1] } else { maximum[1] }, if corner & 4 == 0 { minimum[2] } else { maximum[2] }).sub_m(centre);
        half_width = half_width.max(point.dot_m(right).abs());
        half_height = half_height.max(point.dot_m(up).abs());
    }
    (half_width.max(1e-4), half_height.max(1e-4))
}
//#endregion Camera

//#region GumballMath
pub fn vec3_from_f64(values: [f64; 3]) -> Vec3 {
    vec3_new_m(values[0] as f32, values[1] as f32, values[2] as f32)
}

pub fn gumball_extent(camera_distance: f32) -> f32 {
    (camera_distance * 0.15).clamp(0.25, 2.5)
}

pub fn gumball_eye(camera: &Camera3d, pivot: Vec3) -> Vec3 {
    camera.position.sub_m(pivot).normalize_m()
}

pub fn ray_plane_point(origin: Vec3, dir: Vec3, plane_point: Vec3, plane_normal: Vec3) -> Option<Vec3> {
    let denom = plane_normal.dot_m(dir);
    if denom.abs() < 1e-6 {
        return None;
    }
    let t = plane_normal.dot_m(plane_point.sub_m(origin)) / denom;
    if t < 0.0 {
        return None;
    }
    Some(origin.add_m(dir.scale_m(t)))
}

pub fn gumball_axis_drag_plane_normal(axis: Vec3, eye: Vec3) -> Vec3 {
    let axis = axis.normalize_m();
    let align = eye.cross_m(axis);
    if align.length_m() > 1e-6 {
        return align.cross_m(axis).normalize_m();
    }
    if axis.z.abs() < 0.9 {
        vec3_new_m(0.0, 0.0, 1.0).cross_m(axis).normalize_m()
    } else {
        vec3_new_m(0.0, 1.0, 0.0).cross_m(axis).normalize_m()
    }
}

pub fn gumball_project_ray_onto_axis(origin: Vec3, dir: Vec3, pivot: Vec3, axis: Vec3, eye: Vec3) -> Option<f32> {
    let plane_normal = gumball_axis_drag_plane_normal(axis, eye);
    let hit = ray_plane_point(origin, dir, pivot, plane_normal)?;
    Some(hit.sub_m(pivot).dot_m(axis.normalize_m()))
}

/// 🧷️ Closest finite geometry points retain the authored segment and the forward ray separately.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RaySegmentClosest {
    pub distance: f32,
    pub ray_point: Vec3,
    pub segment_point: Vec3,
}

/// 🥢️ Resolves the closest pair on a forward ray and a finite segment, including endpoint constraints.
pub fn ray_segment_closest(origin: Vec3, dir: Vec3, a: Vec3, b: Vec3) -> Option<RaySegmentClosest> {
    let ab = b.sub_m(a);
    let len_sq = ab.dot_m(ab);
    let dir_sq = dir.dot_m(dir);
    if len_sq <= 0.0 || dir_sq <= 0.0 || !len_sq.is_finite() || !dir_sq.is_finite() {
        return None;
    }
    let offset = origin.sub_m(a);
    let along_ray = offset.dot_m(dir);
    let along_segment = offset.dot_m(ab);
    let directions = dir.dot_m(ab);
    let mut best: Option<(f32, f32, f32)> = None;
    let mut consider = |ray: f32, segment: f32| {
        let gap = offset.add_m(dir.scale_m(ray)).sub_m(ab.scale_m(segment));
        let squared = gap.dot_m(gap);
        if ray.is_finite() && segment.is_finite() && squared.is_finite() && best.is_none_or(|previous| squared < previous.0 || (squared == previous.0 && ray < previous.1)) {
            best = Some((squared, ray, segment));
        }
    };
    consider((-along_ray / dir_sq).max(0.0), 0.0);
    consider(((directions - along_ray) / dir_sq).max(0.0), 1.0);
    consider(0.0, (along_segment / len_sq).clamp(0.0, 1.0));
    let cross = dir.cross_m(ab);
    let denominator = cross.dot_m(cross);
    if denominator > 0.0 {
        let ray = (directions * along_segment - len_sq * along_ray) / denominator;
        let segment = (dir_sq * along_segment - directions * along_ray) / denominator;
        if ray >= 0.0 && (0.0..=1.0).contains(&segment) {
            consider(ray, segment);
        }
    }
    let (squared, ray, segment) = best?;
    Some(RaySegmentClosest { distance: squared.sqrt(), ray_point: origin.add_m(dir.scale_m(ray)), segment_point: a.add_m(ab.scale_m(segment)) })
}

/// 📏️ Measures the perpendicular gap using the original constrained closest-pair query.
pub fn ray_segment_distance(origin: Vec3, dir: Vec3, a: Vec3, b: Vec3) -> Option<f32> {
    ray_segment_closest(origin, dir, a, b).map(|closest| closest.distance)
}

pub fn quat_from_basis(x: Vec3, y: Vec3, z: Vec3) -> [f32; 4] {
    let m00 = x.x;
    let m01 = y.x;
    let m02 = z.x;
    let m10 = x.y;
    let m11 = y.y;
    let m12 = z.y;
    let m20 = x.z;
    let m21 = y.z;
    let m22 = z.z;
    let trace = m00 + m11 + m22;
    if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0;
        [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s]
    } else if m00 > m11 && m00 > m22 {
        let s = (1.0 + m00 - m11 - m22).sqrt() * 2.0;
        [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]
    } else if m11 > m22 {
        let s = (1.0 + m11 - m00 - m22).sqrt() * 2.0;
        [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s]
    } else {
        let s = (1.0 + m22 - m00 - m11).sqrt() * 2.0;
        [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s]
    }
}

pub fn rotate_vector(vector: Vec3, axis: Vec3, angle: f32) -> Vec3 {
    let axis = axis.normalize_m();
    let cos = angle.cos();
    let sin = angle.sin();
    vector.scale_m(cos).add_m(axis.cross_m(vector).scale_m(sin)).add_m(axis.scale_m(axis.dot_m(vector) * (1.0 - cos)))
}

pub fn axis_rotate_angle(start: Vec3, current: Vec3, axis: Vec3) -> f32 {
    let axis = axis.normalize_m();
    let project = |v: Vec3| v.sub_m(axis.scale_m(v.dot_m(axis)));
    let a = project(start).normalize_m();
    let b = project(current).normalize_m();
    let mut angle = a.dot_m(b).clamp(-1.0, 1.0).acos();
    if axis.dot_m(a.cross_m(b)) < 0.0 {
        angle = -angle;
    }
    angle
}
//#endregion GumballMath

//#region LodGrid
pub const LOD_GRID_MAJOR_QUANTUM: f64 = 10.0;
pub const LOD_GRID_MEDIUM_QUANTUM: f64 = 2.5;
pub const LOD_GRID_SMALL_QUANTUM: f64 = 0.5;
pub const LOD_GRID_MICRO_QUANTUM: f64 = 0.1;
pub const WORLD_LOD_GRID_MAX_LOD: f64 = 1000.0;
/// 🧱️ React's `WORLD_LOD_GRID_BASE_LOD` — the LOD one grid factor already covers, so the drawn
/// spacing only starts growing past it.
pub const WORLD_LOD_GRID_BASE_LOD: f64 = 2.0;
pub const WORLD_LOD_GRID_MEDIUM_MAX_LOD: f64 = 50.0;
pub const WORLD_LOD_GRID_SMALL_MAX_LOD: f64 = 10.0;
pub const WORLD_LOD_GRID_MICRO_MAX_LOD: f64 = 2.0;
pub const LOD_GRID_LAYER_OPACITY: [f32; 4] = [1.0, 0.72, 0.48, 0.32];
/// 🌫️ The viewport-free coverage multiplier React's `cameraGridFadeDistance` uses on the camera's
/// height above the grid plane (`🎨️r3f/🟦️.tsx` `WORLD_LOD_GRID_FADE_HEIGHT_FACTOR`).
pub const WORLD_LOD_GRID_FADE_HEIGHT_FACTOR: f32 = 32.0;
/// 🔢️ The smallest number of cells the grid always covers, whatever the camera height.
pub const WORLD_LOD_GRID_MIN_FADE_CELLS: f64 = 24.0;
/// 💨️ The exponent the grid's alpha falls off with across its fade radius — React's drei `fadeStrength`.
pub const WORLD_LOD_GRID_FADE_STRENGTH: f32 = 1.5;
/// 🛡️ React's `WORLD_LOD_GRID_COVERAGE_MARGIN` — how far past the live frustum radius the grid plane
/// reaches so its corners never reveal a hard edge.
pub const WORLD_LOD_GRID_COVERAGE_MARGIN: f32 = 32.0;
/// 🔜️ The orbit camera's near plane — React's `near={0.2}` on both camera components.
pub const WORLD_ORBIT_CAMERA_NEAR: f32 = 0.2;
/// 🛣️ The floor React's `adaptiveOrbitCameraFar` never goes below (`WORLD_ORBIT_CAMERA_MIN_FAR`).
pub const WORLD_ORBIT_CAMERA_MIN_FAR: f32 = 524_288.0;
/// 🛤️ How many orbit distances React's far plane covers before it grows (`WORLD_ORBIT_CAMERA_FAR_DISTANCE_FACTOR`).
pub const WORLD_ORBIT_CAMERA_FAR_DISTANCE_FACTOR: f32 = 1_024.0;
/// 📶️ React's `WORLD_LOD_REFERENCE_FOV_DEG` — the field a parallel `zoom` is mapped through so the
/// LOD bands retune when an orthographic pane scrolls.
pub const WORLD_LOD_REFERENCE_FOV_DEG: f32 = 50.0;
/// 🎚️ React's `orbitCameraZoomForProjection` default for a freshly parallel pane.
pub const WORLD_ORBIT_PARALLEL_DEFAULT_ZOOM: f32 = 50.0;
pub const WORLD_ORBIT_PARALLEL_ZOOM_MIN: f32 = 1e-3;
pub const WORLD_ORBIT_PARALLEL_ZOOM_MAX: f32 = 1e5;
/// 🪟️ React's `WORLD_PROJECTION_FRAME_PADDING` — the air a parallel fit leaves around content.
pub const WORLD_PROJECTION_FRAME_PADDING: f32 = 1.35;

/// 🌌️ Quantized far plane that follows an arbitrarily large orbit distance — React's
/// `adaptiveOrbitCameraFar` (`🎨️r3f/🟦️.tsx`), run by `LodFrameRunner` on every frame.
pub fn adaptive_orbit_camera_far(distance: f32) -> f32 {
    let target = WORLD_ORBIT_CAMERA_MIN_FAR.max(distance.max(0.0) * WORLD_ORBIT_CAMERA_FAR_DISTANCE_FACTOR);
    2.0_f32.powf(target.max(1.0).log2().ceil())
}

/// 📐️ The parallel `zoom` reproducing a perspective camera's apparent scale at `distance` — React's
/// `worldProjectionMatchedOrthoZoom`.
pub fn world_projection_matched_ortho_zoom(fov_deg: f32, distance: f32, viewport_height: f32) -> f32 {
    viewport_height.max(1.0) / (2.0 * distance.max(1e-4) * (fov_deg.to_radians() * 0.5).tan())
}

/// ↔️ The inverse: the orbit distance a parallel `zoom` looks like — React's
/// `worldProjectionMatchedPerspectiveDistance`, the input automatic LOD reads.
pub fn world_projection_matched_perspective_distance(fov_deg: f32, zoom: f32, viewport_height: f32) -> f32 {
    viewport_height.max(1.0) / (2.0 * zoom.max(1e-6) * (fov_deg.to_radians() * 0.5).tan())
}

/// 🎛️ The distance automatic LOD bands on — React's `lodOrbitDistanceForCamera`: a parallel camera
/// never dollies, so its `zoom` stands in for the eye→target distance.
pub fn lod_orbit_distance_for_camera(camera: &Camera3d, orbit_distance: f32, viewport_height: f32) -> f32 {
    if camera.projection.is_parallel() {
        world_projection_matched_perspective_distance(WORLD_LOD_REFERENCE_FOV_DEG, camera.zoom, viewport_height)
    } else {
        orbit_distance.max(1e-6)
    }
}

/// 👀️ World-space radius of the camera frustum on the grid plane — React's `cameraGridVisibleRadius`.
pub fn camera_grid_visible_radius(camera: &Camera3d, plane_z: f32, width: f32, height: f32) -> f32 {
    let width = width.max(1.0);
    let height = height.max(1.0);
    if camera.projection.is_parallel() {
        let (half_width, half_height) = camera.orthographic_half_extent(width, height);
        return half_width.hypot(half_height);
    }
    let distance = (camera.position.z - plane_z).abs().max(1e-3);
    let half_height = distance * (camera.fov_y * 0.5).tan() / camera.zoom.max(1e-6);
    let half_width = half_height * (width / height);
    half_width.hypot(half_height)
}

/// 🌅️ World-space radius the grid reaches before it fades out — the Rust twin of React's
/// `cameraGridFadeDistance`, viewport branch and all, now that the orbit camera grows its own far
/// plane like React's ([`adaptive_orbit_camera_far`]).
pub fn camera_grid_fade_distance(camera: &Camera3d, plane_z: f32, step_world: f64, width: f32, height: f32) -> f32 {
    if !step_world.is_finite() || step_world <= 0.0 {
        return 0.0;
    }
    let coverage = f64::from(camera_grid_visible_radius(camera, plane_z, width, height) * WORLD_LOD_GRID_COVERAGE_MARGIN);
    let target = (step_world * WORLD_LOD_GRID_MIN_FADE_CELLS).max(coverage);
    let cells = 2.0_f64.powf((target / step_world).max(1.0).log2().ceil());
    let fade = step_world * cells;
    let far_cap = if camera.far.is_finite() && camera.far > 0.0 { f64::from(camera.far) * 0.25 } else { fade };
    coverage.max(fade.min(far_cap.max(coverage))) as f32
}

/// 🌁️ The grid line's alpha at `radius` from the grid centre — React's drei `fadeStrength` curve.
pub fn lod_grid_fade_alpha(radius: f32, fade_distance: f32) -> f32 {
    if !(fade_distance > 0.0) {
        return 0.0;
    }
    (1.0 - (radius / fade_distance).clamp(0.0, 1.0).powf(WORLD_LOD_GRID_FADE_STRENGTH)).clamp(0.0, 1.0)
}

pub fn lod_from_camera_distance(distance: f64, reference: f64) -> f64 {
    let d = distance.max(1e-6);
    let reference = reference.max(1e-6);
    d / reference
}

pub fn pick_closest_lod(available: &[f64], desired: f64) -> Option<f64> {
    if available.is_empty() || !desired.is_finite() || desired <= 0.0 {
        return None;
    }
    let mut best = available[0];
    if !best.is_finite() || best <= 0.0 {
        return None;
    }
    let mut best_dist = (best.ln() - desired.ln()).abs();
    for &rep in available.iter().skip(1) {
        if !rep.is_finite() || rep <= 0.0 {
            continue;
        }
        let dist = (rep.ln() - desired.ln()).abs();
        if dist < best_dist - 1e-12 || ((dist - best_dist).abs() <= 1e-12 && rep < best) {
            best = rep;
            best_dist = dist;
        }
    }
    Some(best)
}

pub fn pick_closest_mesh_url<'a>(entries: &'a [(f64, &'a str)], desired: f64, fallback: Option<&'a str>) -> Option<&'a str> {
    if entries.is_empty() {
        return fallback;
    }
    let lods: Vec<f64> = entries.iter().filter_map(|(lod, _)| (*lod > 0.0 && lod.is_finite()).then_some(*lod)).collect();
    let picked = pick_closest_lod(&lods, desired)?;
    entries.iter().find(|(lod, _)| (*lod - picked).abs() < 1e-12).map(|(_, url)| *url).or(fallback)
}

pub fn lod_grid_band_steps_world(grid_factor: f64) -> [f64; 4] {
    [LOD_GRID_MAJOR_QUANTUM * grid_factor, LOD_GRID_MEDIUM_QUANTUM * grid_factor, LOD_GRID_SMALL_QUANTUM * grid_factor, LOD_GRID_MICRO_QUANTUM * grid_factor]
}

pub fn lod_progressive_grid_layers(lod: f64, grid_factor: f64) -> Vec<(f64, f32)> {
    if !lod.is_finite() || lod <= 0.0 || lod > WORLD_LOD_GRID_MAX_LOD {
        return Vec::new();
    }
    let [large, medium, small, micro] = lod_grid_band_steps_world(grid_factor);
    let mut layers = vec![(large, LOD_GRID_LAYER_OPACITY[0])];
    if lod <= WORLD_LOD_GRID_MEDIUM_MAX_LOD {
        layers.push((medium, LOD_GRID_LAYER_OPACITY[1]));
    }
    if lod <= WORLD_LOD_GRID_SMALL_MAX_LOD {
        layers.push((small, LOD_GRID_LAYER_OPACITY[2]));
    }
    if lod <= WORLD_LOD_GRID_MICRO_MAX_LOD {
        layers.push((micro, LOD_GRID_LAYER_OPACITY[3]));
    }
    layers
}

pub fn lod_progressive_grid_layer_key(lod: f64, grid_factor: f64) -> String {
    let layers = lod_progressive_grid_layers(lod, grid_factor);
    if layers.is_empty() {
        return String::new();
    }
    layers.iter().map(|(step, _)| step.to_string()).collect::<Vec<_>>().join("|")
}

/// 🪜️ The ONE spacing React's `WorldLodGridHelper` draws — its `lodGridStepWorld`: the configured
/// factor times a regular 1 · 2.5 · 5 · 10 quantum of the band's own magnitude.
///
/// ⚖️ This is the drawn grid's contract. [`lod_progressive_grid_layers`] is a different, local idea
/// (four stacked bands at fixed quanta) that React has no counterpart for; drawing it put a 1-unit
/// grid under a 10-unit one and read as noise next to the reference
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w8b-orthographic-camera-and-3d-parity.md` §3).
pub fn lod_grid_step_world(lod: f64, grid_factor: f64) -> Option<f64> {
    if !lod.is_finite() || lod <= 0.0 || !grid_factor.is_finite() || grid_factor <= 0.0 {
        return None;
    }
    let target_multiplier = (lod / WORLD_LOD_GRID_BASE_LOD).max(1.0);
    let magnitude = 10.0_f64.powf(target_multiplier.log10().floor());
    let normalized = target_multiplier / magnitude;
    let quantum = if normalized <= 1.0 {
        1.0
    } else if normalized <= 2.5 {
        2.5
    } else if normalized <= 5.0 {
        5.0
    } else {
        10.0
    };
    Some(grid_factor * quantum * magnitude)
}

pub fn floating_origin_rebase(world: Vec3, anchor: Vec3) -> Vec3 {
    vec3_new_m(world.x - anchor.x, world.y - anchor.y, world.z - anchor.z)
}

pub fn grid_placement_anchor(orbit_target: Vec3, datum: [f64; 3]) -> Vec3 {
    vec3_new_m(orbit_target.x, orbit_target.y, datum[2] as f32)
}
//#endregion LodGrid

#[cfg(any(test, feature = "testkit"))]
#[path = "../../🧪️tests/🧰️camera-testkit/🦀️.rs"]
pub mod testkit;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️camera-unit/🦀️.rs"]
mod tests;
