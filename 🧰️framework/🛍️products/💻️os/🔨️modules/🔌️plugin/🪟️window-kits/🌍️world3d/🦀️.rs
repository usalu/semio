//! 🌐️ Shared world-3d scene payload builders for plugin apps.
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::f64::consts::PI;
// 🧬️ M2 fallout (terra-sdk-wire): `scene-surface` relocated `World3dScene`/
// `world3d_default_selection_json` into `semio-framework-ui-scene`; `world3d_camera_json`/
// `ActionDescriptor`/`MeasureSelectItem`/`WindowMeasure` were not part of that move and stay here.
use semio_framework_ui_scene::world3d_default_selection_json;
pub use semio_framework_ui_scene::{
    scene_lane_hash, SceneLanePayload, SceneLaneRef, World3dScene, World3dSceneLane, World3dAngle, World3dAnnotation, World3dAnnotationLayer, World3dColorRamp, World3dDimension, World3dHighlight, World3dLeader, World3dLegendTick, World3dMarker, World3dMarkerShape, World3dModellingOptions, World3dPickGranularity, World3dPickTargets, World3dScalarDomain, World3dScalarField, World3dScalarLegend, World3dScalarRange, World3dSection, World3dSectionCap, World3dSubElementStyle, World3dText, World3dTone, WORLD3D_ANNOTATIONS_MAX, WORLD3D_SCENE_LANE_BODY_KEYS, WORLD3D_SCENE_LANE_FIELDS, WORLD3D_SCENE_LANE_KEY_PREFIX, WORLD3D_SCENE_LANE_NAMES, WORLD3D_SCENE_LANE_OPTIONAL,
};
use ui_wgpu::wgpu::{world3d_camera_json, ActionDescriptor, MeasureSelectItem, WindowMeasure};
/** 🌞️ Plugin-owned directional-light state for a `world-3d` scene; off by default so meshes render flat until a dev opts in via the window-options Sun toggle. */
#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
pub struct WorldSunConfig {
    pub enabled: bool,
    pub azimuth: f64,
    pub elevation: f64,
    pub intensity: f64,
    pub color: String,
}
impl Default for WorldSunConfig {
    fn default() -> Self {
        Self { enabled: false, azimuth: 45.0, elevation: 35.0, intensity: 0.85, color: "#ffffff".into() }
    }
}
/** 🌞️ Builds the `environment_json` payload consumed by `world-3d-host.tsx`'s `WorldEnvironmentRecord.sun`. */
pub fn world3d_environment_json(sun: &WorldSunConfig) -> String {
    json!({ "sun": sun }).to_string()
}
/** 🌳️ Measure group with optional open state and no header slider fields. */
fn measure_group_with_open(id: String, label: impl Into<String>, default_open: Option<bool>, children: Vec<WindowMeasure>) -> WindowMeasure {
    WindowMeasure::Group { id, label: label.into(), default_open, active_utility_id: None, value: None, min: None, max: None, step: None, ready: None, loading: None, waiting: None, on_change: None, children }
}
/** 🌞️ Shared "Sun" window-options group (enable toggle + azimuth/elevation/intensity sliders), see `lowpoly_window_measures`'s "Show Edges" toggle for the sibling pattern.
 * Open by default like every sibling group (grid/LOD/select): `WindowMeasureTreeGroup` does not render a
 * collapsed group's children at all, so a closed-by-default Sun group put `<prefix>-measure-sun-enabled`
 * outside the DOM entirely and the toggle read as missing rather than merely folded. */
pub fn world3d_sun_measures(id_prefix: &str, sun: &WorldSunConfig, is_de: bool, action: impl Fn(&str, Option<semio_framework::DslValue>) -> ActionDescriptor) -> WindowMeasure {
    measure_group_with_open(
        format!("{id_prefix}-measure-sun"),
        if is_de { "Sonne" } else { "Sun" },
        Some(true),
        vec![
            WindowMeasure::Toggle { id: format!("{id_prefix}-measure-sun-enabled"), icon_id: "sun".into(), label: Some(if is_de { "Aktiviert" } else { "Enabled" }.into()), pressed: sun.enabled, text: None, on_change: action("toggleSun", None) },
            WindowMeasure::Slider {
                id: format!("{id_prefix}-measure-sun-azimuth"),
                label: Some(if is_de { "Azimut" } else { "Azimuth" }.into()),
                value: sun.azimuth,
                min: 0.0,
                max: 360.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                on_change: action("setSunAzimuth", None),
            },
            WindowMeasure::Slider {
                id: format!("{id_prefix}-measure-sun-elevation"),
                label: Some(if is_de { "Höhe" } else { "Elevation" }.into()),
                value: sun.elevation,
                min: 0.0,
                max: 90.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                on_change: action("setSunElevation", None),
            },
            WindowMeasure::Slider {
                id: format!("{id_prefix}-measure-sun-intensity"),
                label: Some(if is_de { "Intensität" } else { "Intensity" }.into()),
                value: sun.intensity,
                min: 0.0,
                max: 4.0,
                step: Some(0.05),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                on_change: action("setSunIntensity", None),
            },
        ],
    )
}
/** 🌞️ Applies a sun-related action id to `sun`, returning whether it was handled — mirrors `lowpoly`'s `"toggleShowEdges"` action-handler shape. */
pub fn apply_world3d_sun_action(sun: &mut WorldSunConfig, action_id: &str, args: Option<&semio_framework_pack_json::Value>) -> bool {
    match action_id {
        "toggleSun" => {
            sun.enabled = !sun.enabled;
            true
        }
        "setSunAzimuth" => {
            if let Some(value) = args.and_then(|value| value.get("value")).and_then(semio_framework_pack_json::Value::as_f64) {
                sun.azimuth = value;
            }
            true
        }
        "setSunElevation" => {
            if let Some(value) = args.and_then(|value| value.get("value")).and_then(semio_framework_pack_json::Value::as_f64) {
                sun.elevation = value;
            }
            true
        }
        "setSunIntensity" => {
            if let Some(value) = args.and_then(|value| value.get("value")).and_then(semio_framework_pack_json::Value::as_f64) {
                sun.intensity = value;
            }
            true
        }
        _ => false,
    }
}
/** 📐️ Plugin-owned projection state for a `world-3d` scene camera — the full classical taxonomy
 * (Parallel: Orthographic/Axonometric/Oblique, Perspective: 1/2/3-Point/Curvilinear). Flat so
 * switching `kind` and back restores whatever a dev last dialed in on the other kinds. See
 * https://en.wikipedia.org/wiki/Axonometric_projection and https://en.wikipedia.org/wiki/Oblique_projection. */
#[derive(semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree, Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
pub struct WorldProjectionConfig {
    pub kind: String,
    pub orthographic_view: String,
    pub axonometric_variant: String,
    pub axonometric_angle_a: f64,
    pub axonometric_angle_b: f64,
    pub axonometric_quadrant: String,
    pub oblique_variant: String,
    pub oblique_angle: f64,
    pub oblique_depth: f64,
    pub one_point_axis: String,
    pub fov: f64,
    pub two_point_shift: f64,
    pub curvilinear_fov: f64,
    pub curvilinear_strength: f64,
    pub curvilinear_mapping: String,
}
impl Default for WorldProjectionConfig {
    fn default() -> Self {
        Self {
            kind: "threePoint".into(),
            orthographic_view: "top".into(),
            axonometric_variant: "isometric".into(),
            axonometric_angle_a: 15.0,
            axonometric_angle_b: 12.0,
            axonometric_quadrant: "ne".into(),
            oblique_variant: "cavalier".into(),
            oblique_angle: 45.0,
            oblique_depth: 1.0,
            one_point_axis: "y".into(),
            fov: 50.0,
            two_point_shift: 0.0,
            curvilinear_fov: 120.0,
            curvilinear_strength: 1.0,
            curvilinear_mapping: "fisheye".into(),
        }
    }
}
/** 📐️ Serializes mode ⊗ orientation — the `WorldProjectionSpec` shape the JS world layer parses. */
pub fn world3d_projection_spec_json(p: &WorldProjectionConfig) -> Value {
    let mode = match p.kind.as_str() {
        "orthographic" => json!({ "kind": "orthographic" }),
        "axonometric" => {
            let (angle_a, angle_b) = match p.axonometric_variant.as_str() {
                "isometric" => (30.0, 30.0),
                "dimetric" => (p.axonometric_angle_a, p.axonometric_angle_a),
                _ => (p.axonometric_angle_a, p.axonometric_angle_b),
            };
            json!({ "kind": "axonometric", "variant": p.axonometric_variant, "angleA": angle_a, "angleB": angle_b })
        }
        "oblique" => json!({ "kind": "oblique", "variant": p.oblique_variant, "angle": p.oblique_angle, "depthScale": p.oblique_depth }),
        "onePoint" => json!({ "kind": "onePoint", "fov": p.fov }),
        "twoPoint" => json!({ "kind": "twoPoint", "fov": p.fov, "verticalShift": p.two_point_shift }),
        "curvilinear" => json!({ "kind": "curvilinear", "fov": p.curvilinear_fov, "strength": p.curvilinear_strength, "mapping": p.curvilinear_mapping }),
        _ => json!({ "kind": "threePoint", "fov": p.fov }),
    };
    let orientation = match p.kind.as_str() {
        "axonometric" => json!({ "type": "corner", "quadrant": p.axonometric_quadrant, "hemisphere": "upper" }),
        "twoPoint" | "threePoint" | "curvilinear" => json!({ "type": "free" }),
        "onePoint" => {
            let view = match p.one_point_axis.as_str() {
                "x" => "left",
                "z" => "top",
                _ => "front",
            };
            json!({ "type": "cardinal", "view": view })
        }
        "oblique" if p.oblique_variant == "military" => json!({ "type": "cardinal", "view": "plan" }),
        "oblique" => json!({ "type": "cardinal", "view": "front" }),
        _ => json!({ "type": "cardinal", "view": p.orthographic_view }),
    };
    json!({ "mode": mode, "orientation": orientation })
}
/** 📐️ `camera_json` with `position`/`target`/`up`/`zoom` plus the active-kind `projection` spec object — replaces the plain `world3d_camera_json` for worlds that carry the full taxonomy. */
pub fn world3d_camera_projection_json(position: [f64; 3], target: [f64; 3], up: Option<[f64; 3]>, zoom: f64, p: &WorldProjectionConfig) -> String {
    let mut value = json!({
        "position": position,
        "target": target,
        "zoom": zoom,
        "projection": world3d_projection_spec_json(p),
    });
    if let Some(object) = value.as_object_mut() {
        if let Some(up) = up {
            object.insert("up".into(), json!(up));
        }
    }
    value.to_string()
}
/** 📐️ Canonical camera pose (`position`, `up`) for a projection config, orbiting `target` at `distance` — mirrors
 * `computeWorldProjectionPose` in `infinite/world/r3f/index.tsx`; used to snap the viewport on kind/view changes. */
/// 🎥️ "plan" | "top"
pub fn world3d_projection_pose(p: &WorldProjectionConfig, target: [f64; 3], distance: f64) -> ([f64; 3], [f64; 3]) {
    let [tx, ty, tz] = target;
    match p.kind.as_str() {
        "orthographic" => match p.orthographic_view.as_str() {
            "bottom" => ([tx, ty, tz - distance], [0.0, 1.0, 0.0]),
            "front" => ([tx, ty - distance, tz], [0.0, 0.0, 1.0]),
            "back" => ([tx, ty + distance, tz], [0.0, 0.0, 1.0]),
            "left" => ([tx - distance, ty, tz], [0.0, 0.0, 1.0]),
            "right" => ([tx + distance, ty, tz], [0.0, 0.0, 1.0]),
            _ => ([tx, ty, tz + distance], [0.0, 1.0, 0.0]),
        },
        "axonometric" => {
            let spec = world3d_projection_spec_json(p);
            let angle_a = spec.get("angleA").and_then(Value::as_f64).unwrap_or(30.0).to_radians();
            let angle_b = spec.get("angleB").and_then(Value::as_f64).unwrap_or(30.0).to_radians();
            let elevation = (angle_a.tan() * angle_b.tan()).sqrt().asin();
            let azimuth = (angle_a.tan() / angle_b.tan()).sqrt().atan();
            let (sx, sy) = match p.axonometric_quadrant.as_str() {
                "nw" => (-1.0, 1.0),
                "se" => (1.0, -1.0),
                "sw" => (-1.0, -1.0),
                _ => (1.0, 1.0),
            };
            let dir = [sx * elevation.cos() * azimuth.sin(), sy * elevation.cos() * azimuth.cos(), elevation.sin()];
            ([tx + dir[0] * distance, ty + dir[1] * distance, tz + dir[2] * distance], [0.0, 0.0, 1.0])
        }
        "oblique" => {
            if p.oblique_variant == "military" {
                let angle = p.oblique_angle.to_radians();
                ([tx, ty, tz + distance], [angle.sin(), angle.cos(), 0.0])
            } else {
                ([tx, ty - distance, tz], [0.0, 0.0, 1.0])
            }
        }
        "onePoint" => match p.one_point_axis.as_str() {
            "x" => ([tx - distance, ty, tz], [0.0, 0.0, 1.0]),
            "z" => ([tx, ty, tz + distance], [0.0, 1.0, 0.0]),
            _ => ([tx, ty - distance, tz], [0.0, 0.0, 1.0]),
        },
        "twoPoint" => {
            let azimuth = PI / 4.0;
            ([tx + azimuth.sin() * distance, ty - azimuth.cos() * distance, tz], [0.0, 0.0, 1.0])
        }
        _ => ([tx + distance * 0.6, ty - distance * 0.6, tz + distance * 0.45], [0.0, 0.0, 1.0]),
    }
}
/** 📐️ Shared "Projection" window-measures tree: Parallel > Orthographic/Axonometric/Oblique, Perspective > 1/2/3-Point/Curvilinear —
 * every leaf targets `action`, parameter sliders gated to the kind/variant they apply to (see `puzzle3d_fill_utility_options` for the sibling gating pattern).
 * Every id is `<prefix>-measure-projection…`, the same `<prefix>-measure-<family>` shape `world3d_sun_measures` uses — the
 * bare `<prefix>-projection…` spelling it carried before matched nothing any window-options contract names. */
pub fn world3d_projection_measures(id_prefix: &str, p: &WorldProjectionConfig, action: impl Fn(&str, Option<semio_framework::DslValue>) -> ActionDescriptor) -> WindowMeasure {
    let select = |id: String, value: String, items: Vec<(&str, &str)>, field: &str| WindowMeasure::Select {
        id,
        label: None,
        value,
        items: items.into_iter().map(|(v, label)| MeasureSelectItem { id: v.into(), value: v.into(), label: label.into() }).collect(),
        on_change: action("setProjection", Some(semio_framework::dsl_value!({ "field": field }))),
    };
    let slider = |id: String, label: &str, value: f64, min: f64, max: f64, step: f64, param: &str| WindowMeasure::Slider {
        id,
        label: Some(label.into()),
        value,
        min,
        max,
        step: Some(step),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        on_change: action("setProjectionParam", Some(semio_framework::dsl_value!({ "param": param }))),
    };
    let orthographic_view = if p.kind == "orthographic" { p.orthographic_view.clone() } else { String::new() };
    let orthographic = measure_group_with_open(
        format!("{id_prefix}-measure-projection-orthographic"),
        "Orthographic",
        Some(true),
        vec![select(
            format!("{id_prefix}-measure-projection-orthographic-view"),
            orthographic_view,
            vec![("plan", "Plan"), ("top", "Top"), ("bottom", "Bottom"), ("front", "Front"), ("back", "Back"), ("left", "Left"), ("right", "Right")],
            "orthographicView",
        )],
    );
    let mut axo_children = vec![
        select(
            format!("{id_prefix}-measure-projection-axonometric-variant"),
            if p.kind == "axonometric" { p.axonometric_variant.clone() } else { String::new() },
            vec![("isometric", "Isometric"), ("dimetric", "Dimetric"), ("trimetric", "Trimetric")],
            "axonometricVariant",
        ),
        select(
            format!("{id_prefix}-measure-projection-axonometric-quadrant"),
            if p.kind == "axonometric" { p.axonometric_quadrant.clone() } else { String::new() },
            vec![("ne", "NE"), ("nw", "NW"), ("se", "SE"), ("sw", "SW")],
            "axonometricQuadrant",
        ),
    ];
    if p.kind == "axonometric" && p.axonometric_variant != "isometric" {
        axo_children.push(slider(format!("{id_prefix}-measure-projection-axonometric-angle-a"), "Angle", p.axonometric_angle_a, 5.0, if p.axonometric_variant == "dimetric" { 60.0 } else { 75.0 }, 0.5, "axonometricAngleA"));
    }
    if p.kind == "axonometric" && p.axonometric_variant == "trimetric" {
        axo_children.push(slider(format!("{id_prefix}-measure-projection-axonometric-angle-b"), "Angle B", p.axonometric_angle_b, 5.0, 75.0, 0.5, "axonometricAngleB"));
    }
    let axonometric = measure_group_with_open(format!("{id_prefix}-measure-projection-axonometric"), "Axonometric", Some(false), axo_children);
    let mut oblique_children = vec![select(
        format!("{id_prefix}-measure-projection-oblique-variant"),
        if p.kind == "oblique" { p.oblique_variant.clone() } else { String::new() },
        vec![("cabinet", "Cabinet"), ("cavalier", "Cavalier"), ("military", "Military")],
        "obliqueVariant",
    )];
    if p.kind == "oblique" {
        oblique_children.push(slider(format!("{id_prefix}-measure-projection-oblique-angle"), "Angle", p.oblique_angle, 5.0, 90.0, 1.0, "obliqueAngle"));
        if p.oblique_variant != "military" {
            oblique_children.push(slider(format!("{id_prefix}-measure-projection-oblique-depth"), "Depth Scale", p.oblique_depth, 0.05, 1.0, 0.05, "obliqueDepth"));
        }
    }
    let oblique = measure_group_with_open(format!("{id_prefix}-measure-projection-oblique"), "Oblique", Some(false), oblique_children);
    let parallel = measure_group_with_open(format!("{id_prefix}-measure-projection-parallel"), "Parallel", Some(true), vec![orthographic, axonometric, oblique]);
    let perspective_kind_value = match p.kind.as_str() {
        "onePoint" => "onePoint",
        "twoPoint" => "twoPoint",
        "curvilinear" => "curvilinear",
        "threePoint" => "threePoint",
        _ => "",
    };
    let mut perspective_children =
        vec![select(format!("{id_prefix}-measure-projection-perspective-kind"), perspective_kind_value.into(), vec![("onePoint", "1-Point"), ("twoPoint", "2-Point"), ("threePoint", "3-Point"), ("curvilinear", "Curvilinear")], "perspectiveKind")];
    match p.kind.as_str() {
        "onePoint" => {
            perspective_children.push(select(format!("{id_prefix}-measure-projection-one-point-axis"), p.one_point_axis.clone(), vec![("y", "Front (Y)"), ("x", "Side (X)"), ("z", "Down (Z)")], "onePointAxis"));
            perspective_children.push(slider(format!("{id_prefix}-measure-projection-fov"), "Field of View", p.fov, 15.0, 120.0, 1.0, "fov"));
        }
        "twoPoint" => {
            perspective_children.push(slider(format!("{id_prefix}-measure-projection-fov"), "Field of View", p.fov, 15.0, 120.0, 1.0, "fov"));
            perspective_children.push(slider(format!("{id_prefix}-measure-projection-two-point-shift"), "Vertical Shift", p.two_point_shift, -1.0, 1.0, 0.01, "twoPointShift"));
        }
        "threePoint" => {
            perspective_children.push(slider(format!("{id_prefix}-measure-projection-fov"), "Field of View", p.fov, 15.0, 120.0, 1.0, "fov"));
        }
        "curvilinear" => {
            perspective_children.push(slider(format!("{id_prefix}-measure-projection-curvilinear-fov"), "Field of View", p.curvilinear_fov, 60.0, 160.0, 1.0, "curvilinearFov"));
            perspective_children.push(slider(format!("{id_prefix}-measure-projection-curvilinear-strength"), "Strength", p.curvilinear_strength, 0.0, 1.0, 0.01, "curvilinearStrength"));
            perspective_children.push(select(format!("{id_prefix}-measure-projection-curvilinear-mapping"), p.curvilinear_mapping.clone(), vec![("fisheye", "Fisheye"), ("panini", "Panini")], "curvilinearMapping"));
        }
        _ => {}
    }
    let perspective = measure_group_with_open(format!("{id_prefix}-measure-projection-perspective"), "Perspective", Some(true), perspective_children);
    measure_group_with_open(format!("{id_prefix}-measure-projection"), "Projection", Some(true), vec![parallel, perspective])
}
/** 📐️ Applies `setProjection`/`setProjectionParam` to `p`, returning whether the action was handled. */
pub fn apply_world3d_projection_action(p: &mut WorldProjectionConfig, action_id: &str, args: Option<&semio_framework_pack_json::Value>) -> bool {
    let field = args.and_then(|v| v.get("field")).and_then(semio_framework_pack_json::Value::as_str);
    let value_str = args.and_then(|v| v.get("value")).and_then(semio_framework_pack_json::Value::as_str);
    let value_f64 = args.and_then(|v| v.get("value")).and_then(semio_framework_pack_json::Value::as_f64);
    let param = args.and_then(|v| v.get("param")).and_then(semio_framework_pack_json::Value::as_str);
    match action_id {
        "setProjection" => {
            match (field, value_str) {
                (Some("orthographicView"), Some(value)) => {
                    p.kind = "orthographic".into();
                    p.orthographic_view = value.into();
                }
                (Some("axonometricVariant"), Some(value)) => {
                    p.kind = "axonometric".into();
                    p.axonometric_variant = value.into();
                }
                (Some("axonometricQuadrant"), Some(value)) => {
                    p.kind = "axonometric".into();
                    p.axonometric_quadrant = value.into();
                }
                (Some("obliqueVariant"), Some(value)) => {
                    p.kind = "oblique".into();
                    p.oblique_variant = value.into();
                }
                (Some("perspectiveKind"), Some(value)) => {
                    p.kind = value.into();
                }
                (Some("onePointAxis"), Some(value)) => {
                    p.one_point_axis = value.into();
                }
                (Some("curvilinearMapping"), Some(value)) => {
                    p.curvilinear_mapping = value.into();
                }
                _ => return false,
            }
            true
        }
        "setProjectionParam" => {
            let (Some(param), Some(value)) = (param, value_f64) else { return false };
            match param {
                "axonometricAngleA" => p.axonometric_angle_a = value,
                "axonometricAngleB" => p.axonometric_angle_b = value,
                "obliqueAngle" => p.oblique_angle = value,
                "obliqueDepth" => p.oblique_depth = value,
                "fov" => p.fov = value,
                "twoPointShift" => p.two_point_shift = value,
                "curvilinearFov" => p.curvilinear_fov = value,
                "curvilinearStrength" => p.curvilinear_strength = value,
                _ => return false,
            }
            true
        }
        _ => false,
    }
}
/** 📐️ Whether a `setProjection`/`setProjectionParam` action requires a pose recompute (kind/view/variant changes) vs. a pure in-place parameter tweak that keeps the current pose (oblique shear / two-point shift / fov / curvilinear re-shade live). */
pub fn world3d_projection_action_moves_pose(action_id: &str, args: Option<&semio_framework_pack_json::Value>) -> bool {
    if action_id != "setProjection" {
        return false;
    }
    matches!(args.and_then(|v| v.get("field")).and_then(semio_framework_pack_json::Value::as_str), Some("orthographicView") | Some("axonometricVariant") | Some("axonometricQuadrant") | Some("obliqueVariant") | Some("perspectiveKind"))
}
pub fn mesh_kind_from_json(mesh_json: &str) -> String {
    serde_json::from_str::<Value>(mesh_json).ok().and_then(|value| value.get("kind").and_then(|v| v.as_str()).map(str::to_string)).unwrap_or_else(|| "box".into())
}
/// 🌉️ Rewritten from a `.map(...).collect()` into an explicit loop (sync — `Iterator::map`
/// cannot take an async closure; mesh construction itself is synchronous CPU work.
pub fn world3d_meshes_json_from_kinds(kinds: &[String]) -> String {
    let mut meshes: Vec<semio_framework_pack_json::Value> = Vec::with_capacity(kinds.len());
    for kind in kinds.iter() {
        meshes.push(world3d_mesh_kind_entry(kind));
    }
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(meshes))
}
/// 🥽️ One `{ id, kind }` scene entry for a built-in mesh kind — a REFERENCE, never the geometry.
///
/// A built-in kind is procedural: both renderers already own its generator (`mesh_from_kind` in
/// `🔨️modules/🏗️mesh-engine/🦀️.rs` and the wgpu viewport's own `WorldPlaceholderKind`; the React
/// host resolves it in `🌐️World3dHost/🟦️.tsx`'s `meshDataFromKind`), so inlining the tessellation
/// re-sent a fixed, renderer-derivable buffer on EVERY refresh. `vortex-marker` alone
/// (`mesh_ico_sphere(0.12, 1)`, 80 triangles printed as JSON floats) cost ~26 KiB of the 32 KiB
/// `UiFixedBytes` surface payload, which is what made a puzzle 3d world with an open suggestion
/// popup fail `scene-surface.encode` outright. A GLB mesh already rode as `{ id, url }`; this makes
/// the procedural case symmetric.
///
/// The renderer-observable property a reference must reproduce, whichever generator resolves it, is
/// the kind's local-space bounding box — pinned on both ends by
/// `🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🥽️scene-mesh-kinds/🔣️.json`. The two generators
/// tessellate differently on purpose, so the triangle list is deliberately NOT part of that contract.
fn world3d_mesh_kind_entry(kind: &str) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::object([("id".to_string(), semio_framework_pack_json::Value::String(kind.to_string())), ("kind".to_string(), semio_framework_pack_json::Value::String(kind.to_string()))])
}
pub fn world3d_mesh_id_from_url(url: &str) -> String {
    let slug = url.trim_start_matches('/').rsplit('/').next().unwrap_or(url).trim_end_matches(".glb").trim_end_matches(".gltf");
    format!("mesh:{slug}")
}
pub fn world3d_meshes_json_from_urls(urls: &[String]) -> String {
    let mut meshes: Vec<Value> = Vec::with_capacity(urls.len());
    for url in urls.iter() {
        let id = world3d_mesh_id_from_url(url);
        meshes.push(json!({
            "id": id,
            "url": url,
        }));
    }
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}
/// 🌉️ Rewritten from a `.map(...).collect()` into an explicit loop (sync — `Iterator::map`
/// cannot take an async closure; mesh construction itself is synchronous CPU work.
pub fn world3d_meshes_json_from_kinds_and_urls(kinds: &[String], urls: &[String]) -> String {
    let mut meshes: Vec<semio_framework_pack_json::Value> = Vec::with_capacity(kinds.len());
    for kind in kinds.iter() {
        meshes.push(world3d_mesh_kind_entry(kind));
    }
    for url in urls {
        let id = world3d_mesh_id_from_url(url);
        if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(id.as_str())) {
            continue;
        }
        meshes.push(semio_framework_pack_json::object([("id".to_string(), semio_framework_pack_json::Value::String(id)), ("url".to_string(), semio_framework_pack_json::Value::String(url.clone()))]));
    }
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(meshes))
}
pub fn world3d_selection_json(method: &str, ids: &[String], hovered_id: Option<&str>) -> String {
    world3d_selection_json_with_granularity(method, ids, hovered_id, None)
}
pub fn world3d_selection_json_with_granularity(method: &str, ids: &[String], hovered_id: Option<&str>, granularity: Option<&str>) -> String {
    let mut value = json!({
        "method": method,
        "mode": "replace",
        "ids": ids,
        "hoveredId": hovered_id,
    });
    if let Some(entry) = granularity {
        if let Some(object) = value.as_object_mut() {
            object.insert("granularity".into(), json!(entry));
        }
    }
    value.to_string()
}
pub fn world3d_scene(camera_json: String, meshes_json: String, instances_json: String, selection_json: String, sun: &WorldSunConfig) -> World3dScene {
    let mut scene = World3dScene::base(camera_json, meshes_json, instances_json, selection_json);
    scene.environment_json = Some(world3d_environment_json(sun));
    scene
}
pub fn world3d_default_camera() -> String {
    world3d_camera_json([4.0, -4.0, 3.0], [0.0, 0.0, 0.0], 45.0)
}
/** ✅️ Ordered selection ids with O(1) membership — serializes as a plain JSON string array. */
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectionSet {
    ids: Vec<String>,
    index: std::collections::HashSet<String>,
}
// 🚫️async: E1 pure value type consumed by external-trait impls (`Serialize`/`Deserialize`/
// `FromIterator`/`IntoIterator`/`From<Vec<String>>` below, all in this same file) — see R9.
// Every method here is a plain in-memory `Vec`/`HashSet` operation with no suspension point.
impl SelectionSet {
    pub fn from_ids(ids: Vec<String>) -> Self {
        let index: std::collections::HashSet<String> = ids.iter().cloned().collect();
        Self { ids, index }
    }
    pub fn contains(&self, id: &str) -> bool {
        self.index.contains(id)
    }
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }
    pub fn len(&self) -> usize {
        self.ids.len()
    }
    pub fn clear(&mut self) {
        self.ids.clear();
        self.index.clear();
    }
    pub fn first(&self) -> Option<&str> {
        self.ids.first().map(String::as_str)
    }
    pub fn as_slice(&self) -> &[String] {
        &self.ids
    }
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.ids.iter()
    }
    pub fn push_unique(&mut self, id: String) {
        if self.index.insert(id.clone()) {
            self.ids.push(id);
        }
    }
    pub fn remove_id(&mut self, id: &str) {
        if self.index.remove(id) {
            self.ids.retain(|entry| entry != id);
        }
    }
    pub fn to_vec(&self) -> Vec<String> {
        self.ids.clone()
    }
}
impl IntoIterator for SelectionSet {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;
    fn into_iter(self) -> Self::IntoIter {
        self.ids.into_iter()
    }
}
impl<'a> IntoIterator for &'a SelectionSet {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.ids.iter()
    }
}
impl Serialize for SelectionSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.ids.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for SelectionSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ids = Vec::<String>::deserialize(deserializer)?;
        Ok(Self::from_ids(ids))
    }
}
impl From<Vec<String>> for SelectionSet {
    /// 🚫️async: E1 impl of externally-declared `From` — see R9.
    fn from(ids: Vec<String>) -> Self {
        Self::from_ids(ids)
    }
}
impl FromIterator<String> for SelectionSet {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self::from_ids(iter.into_iter().collect())
    }
}
/// 🎯️ The non-domain world path's set algebra — `worldPick`/`worldVortexSelect`, the surfaces that
/// bind no app `InteractionDefinition` and so never reach `next_selection`. It takes a decoded
/// [`protocol::MergeMode`], not a word: the vocabulary is the schema enum
/// (`🕹️interaction/🧬️schema/🔣️.json`, fixture `🎯️merge-modes.json`), the same five words the
/// domain path speaks, so the two paths can no longer disagree about what a shift-click means.
/// `Range` has no meaning without an ordered topology (this path carries none — see the fixture's
/// `range.unorderedDomains`), so it degrades to the documented single-batch replace exactly as
/// `next_selection` does for an unordered domain.
pub async fn merge_world_selection_ids(existing: &SelectionSet, incoming: &[String], merge: protocol::MergeMode) -> SelectionSet {
    match merge {
        protocol::MergeMode::Additive => {
            let mut merged = existing.clone();
            for id in incoming {
                merged.push_unique(id.clone());
            }
            merged
        }
        protocol::MergeMode::Invertive => {
            let mut merged = existing.clone();
            for id in incoming {
                if merged.index.remove(id) {
                    merged.ids.retain(|entry| entry != id);
                } else {
                    merged.push_unique(id.clone());
                }
            }
            merged
        }
        protocol::MergeMode::Subtractive => {
            let mut merged = existing.clone();
            for id in incoming {
                if merged.index.remove(id) {
                    merged.ids.retain(|entry| entry != id);
                }
            }
            merged
        }
        protocol::MergeMode::Replace | protocol::MergeMode::Range => SelectionSet::from_ids(incoming.to_vec()),
    }
}
pub fn default_world3d_selection() -> String {
    world3d_default_selection_json()
}
#[cfg(test)]
include!("../../🧪️tests/🔬️world3d-host-unit/🦀️.rs");