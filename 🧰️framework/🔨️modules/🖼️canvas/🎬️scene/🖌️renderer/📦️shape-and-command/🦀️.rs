//! 📦️ Shape and command of the scene: the recorded shape and the scene command with its replay into a vello scene. See [`super`].

use super::*;

/// 📹️ An owned copy of a [`ShapeRef`] variant, kept exact (never flattened to a polyline at
/// record time) so a later transform — e.g. zooming an already-recorded scene — cannot make a
/// circle facet; the real curve only gets tessellated once, by `vello`, at rasterization time.
#[derive(Clone)]
#[cfg_attr(all(target_arch = "wasm32", target_env = "p2"), allow(dead_code, reason = "Fields are written by every guest build/measure/retire call but read back only by host-only replay (Scene::vello_scene) — never on this target, by design."))]
pub(crate) enum RecordedShape {
    Rect(Rect),
    RoundedRect(RoundedRect),
    Circle(Circle),
    Line(Line),
    Arc(Arc),
    CubicBez(CubicBez),
    BezPath(BezPath),
}

impl RecordedShape {
    pub(super) fn retirement_step(&mut self) -> bool {
        match self {
            Self::BezPath(path) => path.retirement_step(),
            Self::Rect(_) | Self::RoundedRect(_) | Self::Circle(_) | Self::Line(_) | Self::Arc(_) | Self::CubicBez(_) => true,
        }
    }

    pub(super) fn retirement_backing_bytes(&self) -> usize {
        match self {
            Self::BezPath(path) => path.retirement_backing_bytes(),
            Self::Rect(_) | Self::RoundedRect(_) | Self::Circle(_) | Self::Line(_) | Self::Arc(_) | Self::CubicBez(_) => 0,
        }
    }
}

/// 🖥️ Host/browser-only: the `ShapeRef` view is needed only to drive `geometry::with_shape_ref!`
/// inside `SceneCommand::replay_into` below.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl RecordedShape {
    fn as_shape_ref(&self) -> ShapeRef<'_> {
        match self {
            Self::Rect(s) => ShapeRef::Rect(s),
            Self::RoundedRect(s) => ShapeRef::RoundedRect(s),
            Self::Circle(s) => ShapeRef::Circle(s),
            Self::Line(s) => ShapeRef::Line(s),
            Self::Arc(s) => ShapeRef::Arc(s),
            Self::CubicBez(s) => ShapeRef::CubicBez(s),
            Self::BezPath(s) => ShapeRef::BezPath(s),
        }
    }
}

impl From<ShapeRef<'_>> for RecordedShape {
    fn from(value: ShapeRef<'_>) -> Self {
        match value {
            ShapeRef::Rect(s) => Self::Rect(*s),
            ShapeRef::RoundedRect(s) => Self::RoundedRect(*s),
            ShapeRef::Circle(s) => Self::Circle(*s),
            ShapeRef::Line(s) => Self::Line(*s),
            ShapeRef::Arc(s) => Self::Arc(*s),
            ShapeRef::CubicBez(s) => Self::CubicBez(*s),
            ShapeRef::BezPath(s) => Self::BezPath(s.clone()),
        }
    }
}

/// 🎬️ One recorded drawing instruction — the first-party replacement for `vello::Scene`'s
/// internal `vello_encoding::Encoding` buffers. `Scene` (below) is just `Vec<SceneCommand>`;
/// a real `vello::Scene` is built from it only at the host/browser rasterization call sites
/// (`Scene::vello_scene`), so `vello`/`vello_encoding` never need to compile for
/// `wasm32-wasip2` even though `Scene` itself is built, measured and disposed unconditionally.
/// `VelloFragment` is the one host-only escape hatch, for `SvgDocument::append_to_scene`
/// (icon/label painting, itself already host/browser-gated — see its docstring below).
/// Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,
/// `🔍️research/📓️vello-scene-first-party.md`.
#[derive(Clone)]
pub(crate) enum SceneCommand {
    Fill {
        rule: FillRule,
        transform: Affine,
        paint: Paint,
        brush_transform: Option<Affine>,
        shape: RecordedShape,
    },
    Stroke {
        stroke: Stroke,
        transform: Affine,
        paint: Paint,
        brush_transform: Option<Affine>,
        shape: RecordedShape,
    },
    DrawImage {
        image: RasterImage,
        transform: Affine,
    },
    PushLayer {
        rule: FillRule,
        blend: BlendMode,
        alpha: f32,
        transform: Affine,
        clip: RecordedShape,
    },
    PushClipLayer {
        rule: FillRule,
        transform: Affine,
        clip: RecordedShape,
    },
    PopLayer,
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    VelloFragment {
        scene: SharedArc<backend::Scene>,
        transform: Affine,
    },
}

impl SceneCommand {
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn retire_vello_fragment(scene: &mut SharedArc<backend::Scene>) -> bool {
        let Some(scene) = SharedArc::get_mut(scene) else {
            return true;
        };
        let encoding = scene.encoding_mut();
        encoding.resources.patches.pop().is_none()
            && encoding.resources.color_stops.pop().is_none()
            && encoding.resources.glyphs.pop().is_none()
            && encoding.resources.glyph_runs.pop().is_none()
            && encoding.resources.normalized_coords.pop().is_none()
            && encoding.path_tags.pop().is_none()
            && encoding.path_data.pop().is_none()
            && encoding.draw_tags.pop().is_none()
            && encoding.draw_data.pop().is_none()
            && encoding.transforms.pop().is_none()
            && encoding.styles.pop().is_none()
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[allow(clippy::ptr_arg, reason = "retirement accounts allocation capacity, which a slice erases")]
    fn vector_backing_bytes<T>(values: &Vec<T>) -> usize {
        values.capacity().saturating_mul(size_of::<T>())
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn vello_fragment_backing_bytes(scene: &SharedArc<backend::Scene>) -> usize {
        if SharedArc::strong_count(scene) != 1 {
            return 0;
        }
        let encoding = scene.encoding();
        Self::vector_backing_bytes(&encoding.resources.patches)
            .saturating_add(Self::vector_backing_bytes(&encoding.resources.color_stops))
            .saturating_add(Self::vector_backing_bytes(&encoding.resources.glyphs))
            .saturating_add(Self::vector_backing_bytes(&encoding.resources.glyph_runs))
            .saturating_add(Self::vector_backing_bytes(&encoding.resources.normalized_coords))
            .saturating_add(Self::vector_backing_bytes(&encoding.path_tags))
            .saturating_add(Self::vector_backing_bytes(&encoding.path_data))
            .saturating_add(Self::vector_backing_bytes(&encoding.draw_tags))
            .saturating_add(Self::vector_backing_bytes(&encoding.draw_data))
            .saturating_add(Self::vector_backing_bytes(&encoding.transforms))
            .saturating_add(Self::vector_backing_bytes(&encoding.styles))
    }

    pub(super) fn retirement_step(&mut self) -> bool {
        match self {
            Self::Fill { shape, .. } => shape.retirement_step(),
            Self::Stroke { stroke, shape, .. } => stroke.retirement_step() && shape.retirement_step(),
            Self::DrawImage { image, .. } => image.retirement_step(),
            Self::PushLayer { clip, .. } | Self::PushClipLayer { clip, .. } => clip.retirement_step(),
            Self::PopLayer => true,
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            Self::VelloFragment { scene, .. } => Self::retire_vello_fragment(scene),
        }
    }

    pub(super) fn retirement_backing_bytes(&self) -> usize {
        match self {
            Self::Fill { shape, .. } => shape.retirement_backing_bytes(),
            Self::Stroke { stroke, shape, .. } => stroke.retirement_backing_bytes().saturating_add(shape.retirement_backing_bytes()),
            Self::DrawImage { image, .. } => image.retirement_backing_bytes(),
            Self::PushLayer { clip, .. } | Self::PushClipLayer { clip, .. } => clip.retirement_backing_bytes(),
            Self::PopLayer => 0,
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            Self::VelloFragment { scene, .. } => Self::vello_fragment_backing_bytes(scene),
        }
    }

    /// 🔁️ Rewrites this command's own transform(s) as `outer * existing` — the same
    /// "child-space-then-outer" composition `vello::Scene::append` itself performs on its
    /// internal per-op transform stack, applied here one first-party command at a time so
    /// `Scene::append` can flatten `other`'s commands directly into `self`'s list (keeping
    /// every command independently transferable into the retained retirement cursor, instead
    /// of nesting a whole sub-scene behind one non-incremental drop).
    pub(super) fn transformed(self, outer: Affine) -> Self {
        match self {
            Self::Fill { rule, transform, paint, brush_transform, shape } => Self::Fill { rule, transform: outer * transform, paint, brush_transform: brush_transform.map(|bt| outer * bt), shape },
            Self::Stroke { stroke, transform, paint, brush_transform, shape } => Self::Stroke { stroke, transform: outer * transform, paint, brush_transform: brush_transform.map(|bt| outer * bt), shape },
            Self::DrawImage { image, transform } => Self::DrawImage { image, transform: outer * transform },
            Self::PushLayer { rule, blend, alpha, transform, clip } => Self::PushLayer { rule, blend, alpha, transform: outer * transform, clip },
            Self::PushClipLayer { rule, transform, clip } => Self::PushClipLayer { rule, transform: outer * transform, clip },
            Self::PopLayer => Self::PopLayer,
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            Self::VelloFragment { scene, transform } => Self::VelloFragment { scene, transform: outer * transform },
        }
    }

    /// 🎞️ Host/browser-only: replays one command into a real, growing `vello::Scene`. The
    /// only place `vello`'s draw API is actually invoked — see the module docstring.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub(super) fn replay_into(&self, built: &mut backend::Scene) {
        match self {
            Self::Fill { rule, transform, paint, brush_transform, shape } => {
                let style: peniko::Fill = rule.to_peniko();
                let transform = affine_to_kurbo(*transform);
                let brush_transform = brush_transform.map(affine_to_kurbo);
                let Paint::Solid(color) = paint;
                built.fill(style, transform, color.to_peniko(), brush_transform, &KurboShapeAdapter(shape.as_shape_ref()));
            }
            Self::Stroke { stroke, transform, paint, brush_transform, shape } => {
                let stroke = stroke.to_kurbo();
                let transform = affine_to_kurbo(*transform);
                let brush_transform = brush_transform.map(affine_to_kurbo);
                let Paint::Solid(color) = paint;
                built.stroke(&stroke, transform, color.to_peniko(), brush_transform, &KurboShapeAdapter(shape.as_shape_ref()));
            }
            Self::DrawImage { image, transform } => {
                built.draw_image(&peniko::ImageBrush::new(image.to_peniko()), affine_to_kurbo(*transform));
            }
            Self::PushLayer { rule, blend, alpha, transform, clip } => {
                let style: peniko::Fill = rule.to_peniko();
                let mix: peniko::Mix = blend.to_peniko();
                let transform = affine_to_kurbo(*transform);
                built.push_layer(style, mix, *alpha, transform, &KurboShapeAdapter(clip.as_shape_ref()));
            }
            Self::PushClipLayer { rule, transform, clip } => {
                let style: peniko::Fill = rule.to_peniko();
                let transform = affine_to_kurbo(*transform);
                built.push_clip_layer(style, transform, &KurboShapeAdapter(clip.as_shape_ref()));
            }
            Self::PopLayer => built.pop_layer(),
            Self::VelloFragment { scene, transform } => built.append(scene, Some(affine_to_kurbo(*transform))),
        }
    }
}
