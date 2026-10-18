//! 🧭️ Overlay projection: the annotation pixel constants, the projected arrow, arc, marker and label records, the annotation projection and the `project_world3d_annotations` pass. See [`super`].

use super::*;

//#region 🔖️Overlay
/// 📏️ Arrowhead length of a dimension, in screen pixels; it never scales with zoom or distance.
pub const WORLD3D_ANNOTATION_ARROW_PX: f64 = 9.0;

/// 📍️ Edge length of a point marker, in screen pixels.
pub const WORLD3D_ANNOTATION_MARKER_PX: f64 = 10.0;

/// 🏷️ Distance of a dimension or angle label from its geometry, in screen pixels.
pub const WORLD3D_ANNOTATION_LABEL_GAP_PX: f64 = 14.0;

/// ✒️ Stroke width of annotation geometry, in screen pixels.
pub const WORLD3D_ANNOTATION_STROKE_PX: f64 = 1.5;

/// 🧱️ An arrowhead tip at a screen point, pointing along `angle` (radians, `y` down).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World3dProjectedArrow {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

/// 🌙️ The arc of an angle annotation: centre, radius in pixels, start angle and signed sweep in radians.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World3dProjectedArc {
    pub cx: f64,
    pub cy: f64,
    pub radius: f64,
    pub start: f64,
    pub sweep: f64,
}

/// 📍️ A point mark at a screen position.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World3dProjectedMarker {
    pub x: f64,
    pub y: f64,
    pub shape: World3dMarkerShape,
}

/// ↔️ How a label hangs off its anchor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dLabelAlign {
    Middle,
    Start,
    End,
}

/// 🏷️ The screen anchor of an annotation's text.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World3dProjectedLabel {
    pub x: f64,
    pub y: f64,
    pub align: World3dLabelAlign,
}

/// 🖼️ One annotation resolved to screen pixels (`x` right, `y` down, origin top-left). An annotation with any anchor behind the camera or outside the depth range is not visible.
#[derive(Clone, Debug, PartialEq)]
pub struct World3dProjectedAnnotation {
    pub id: String,
    pub kind: &'static str,
    pub tone: World3dTone,
    pub visible: bool,
    pub lines: Vec<[f64; 4]>,
    pub arrows: Vec<World3dProjectedArrow>,
    pub arc: Option<World3dProjectedArc>,
    pub marker: Option<World3dProjectedMarker>,
    pub label: Option<World3dProjectedLabel>,
}

impl World3dAnnotation {
    /// 🔖️ The wire discriminator of the annotation.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Dimension(_) => "dimension",
            Self::Angle(_) => "angle",
            Self::Marker(_) => "marker",
            Self::Leader(_) => "leader",
        }
    }

    /// ♿️ The text alternative a screen reader gets: the kind, a colon and the caller's text in the active language.
    pub fn accessible_name(&self, locale: &str) -> String {
        let strings = world3d_modelling_strings(locale);
        let kind = match self {
            Self::Dimension(_) => strings.dimension,
            Self::Angle(_) => strings.angle,
            Self::Marker(_) => strings.marker,
            Self::Leader(_) => strings.leader,
        };
        format!("{kind}: {}", self.text().resolve(locale))
    }

    pub(super) fn tone(&self) -> World3dTone {
        match self {
            Self::Dimension(item) => item.tone,
            Self::Angle(item) => item.tone,
            Self::Marker(item) => item.tone,
            Self::Leader(item) => item.tone,
        }
    }
}

/// 📐️ Projects an annotation layer through a column-major view-projection matrix whose depth runs `0..1` (the engine's clip convention) onto a `width` x `height` pixel viewport.
/// Pure: the same matrix, size and layer always give the same geometry, and it is pinned against the three.js-produced `annotationProjection` fixture. Occlusion by the model is not tested, as in CAD dimensioning.
pub fn project_world3d_annotations(layer: &World3dAnnotationLayer, view_proj: &[f32; 16], width: f32, height: f32) -> Vec<World3dProjectedAnnotation> {
    let matrix: [f64; 16] = view_proj.map(f64::from);
    let (width, height) = (f64::from(width), f64::from(height));
    let point = |position: World3dVec3| -> Option<[f64; 2]> {
        let clip = |row: usize| matrix[row] * position[0] + matrix[4 + row] * position[1] + matrix[8 + row] * position[2] + matrix[12 + row];
        let w = clip(3);
        if !(w > 1e-12) {
            return None;
        }
        let (x, y, z) = (clip(0) / w, clip(1) / w, clip(2) / w);
        if !(x.is_finite() && y.is_finite() && (0.0..=1.0).contains(&z)) {
            return None;
        }
        Some([(x + 1.0) / 2.0 * width, (1.0 - y) / 2.0 * height])
    };
    let shifted = |position: World3dVec3, offset: World3dVec3| [position[0] + offset[0], position[1] + offset[1], position[2] + offset[2]];
    layer
        .items
        .iter()
        .map(|item| {
            let hidden = || World3dProjectedAnnotation { id: item.id().to_string(), kind: item.kind(), tone: item.tone(), visible: false, lines: Vec::new(), arrows: Vec::new(), arc: None, marker: None, label: None };
            let shown = |lines, arrows, arc, marker, label| World3dProjectedAnnotation { id: item.id().to_string(), kind: item.kind(), tone: item.tone(), visible: true, lines, arrows, arc, marker, label };
            match item {
                World3dAnnotation::Dimension(item) => {
                    let (Some(from), Some(to), Some(a), Some(b)) = (point(item.from), point(item.to), point(shifted(item.from, item.offset)), point(shifted(item.to, item.offset))) else { return hidden() };
                    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
                    let length = dx.hypot(dy);
                    let normal = if length > 1e-9 { [-dy / length, dx / length] } else { [0.0, -1.0] };
                    let up = if normal[1] <= 0.0 { 1.0 } else { -1.0 };
                    let heading = dy.atan2(dx);
                    shown(
                        vec![[from[0], from[1], a[0], a[1]], [to[0], to[1], b[0], b[1]], [a[0], a[1], b[0], b[1]]],
                        vec![World3dProjectedArrow { x: a[0], y: a[1], angle: heading + std::f64::consts::PI }, World3dProjectedArrow { x: b[0], y: b[1], angle: heading }],
                        None,
                        None,
                        Some(World3dProjectedLabel { x: (a[0] + b[0]) / 2.0 + up * normal[0] * WORLD3D_ANNOTATION_LABEL_GAP_PX, y: (a[1] + b[1]) / 2.0 + up * normal[1] * WORLD3D_ANNOTATION_LABEL_GAP_PX, align: World3dLabelAlign::Middle }),
                    )
                }
                World3dAnnotation::Angle(item) => {
                    let (Some(vertex), Some(ta), Some(tb)) = (point(item.vertex), point(shifted(item.vertex, item.direction_a)), point(shifted(item.vertex, item.direction_b))) else { return hidden() };
                    let (ax, ay, bx, by) = (ta[0] - vertex[0], ta[1] - vertex[1], tb[0] - vertex[0], tb[1] - vertex[1]);
                    if ax.hypot(ay) < 1e-6 || bx.hypot(by) < 1e-6 {
                        return hidden();
                    }
                    let start = ay.atan2(ax);
                    let mut sweep = by.atan2(bx) - start;
                    while sweep > std::f64::consts::PI {
                        sweep -= std::f64::consts::TAU;
                    }
                    while sweep <= -std::f64::consts::PI {
                        sweep += std::f64::consts::TAU;
                    }
                    let middle = start + sweep / 2.0;
                    let label_radius = item.radius_px + WORLD3D_ANNOTATION_LABEL_GAP_PX;
                    shown(
                        vec![
                            [vertex[0], vertex[1], vertex[0] + start.cos() * item.radius_px, vertex[1] + start.sin() * item.radius_px],
                            [vertex[0], vertex[1], vertex[0] + (start + sweep).cos() * item.radius_px, vertex[1] + (start + sweep).sin() * item.radius_px],
                        ],
                        Vec::new(),
                        Some(World3dProjectedArc { cx: vertex[0], cy: vertex[1], radius: item.radius_px, start, sweep }),
                        None,
                        Some(World3dProjectedLabel { x: vertex[0] + middle.cos() * label_radius, y: vertex[1] + middle.sin() * label_radius, align: World3dLabelAlign::Middle }),
                    )
                }
                World3dAnnotation::Marker(item) => match point(item.position) {
                    Some(position) => shown(Vec::new(), Vec::new(), None, Some(World3dProjectedMarker { x: position[0], y: position[1], shape: item.shape }), None),
                    None => hidden(),
                },
                World3dAnnotation::Leader(item) => {
                    let Some(anchor) = point(item.anchor) else { return hidden() };
                    let label = [anchor[0] + item.label_offset_px[0], anchor[1] + item.label_offset_px[1]];
                    shown(
                        vec![[anchor[0], anchor[1], label[0], label[1]]],
                        Vec::new(),
                        None,
                        Some(World3dProjectedMarker { x: anchor[0], y: anchor[1], shape: World3dMarkerShape::Dot }),
                        Some(World3dProjectedLabel { x: label[0], y: label[1], align: if item.label_offset_px[0] >= 0.0 { World3dLabelAlign::Start } else { World3dLabelAlign::End } }),
                    )
                }
            }
        })
        .collect()
}
//#endregion 🔖️Overlay
