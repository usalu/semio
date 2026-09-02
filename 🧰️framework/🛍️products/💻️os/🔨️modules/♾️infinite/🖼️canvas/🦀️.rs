//! 🖼️ Application-neutral tile-based infinite canvas; extend via `CanvasExtension`.
#![allow(clippy::missing_errors_doc, reason = "Canvas bundle is internal infrastructure.")]

// #region 🔖️Renderer
mod renderer {
    // #region 🏷️VelloBackend
    pub(super) mod vello_backend {
        /// 🖊️ `vello`'s font/SVG/codec transitive tail (`vello_encoding`, `skrifa`, `read-fonts`,
        /// `font-types`, `png`, `guillotiere`, `moxcms`, `pxfm`, …) is only needed where a real
        /// `vello::Scene` is actually rasterized — the host/browser target table. `Scene` itself
        /// is a first-party command list (see `SceneCommand` below). `kurbo`/`peniko` (the
        /// value-type family `Stroke`/`Color`/`Fill`/`Mix`/`ImageData` are built on) are ALSO
        /// host/browser-only now: every guest-reachable value type (`Cap`, `Join`, `Stroke`,
        /// `Color`, `FillRule`, `BlendMode`, `RasterImage`) is first-party, converting to a real
        /// `kurbo`/`peniko` value only inside `SceneCommand::replay_into` — the same
        /// already-host-gated boundary `vello` itself crosses at. `target_arch = "wasm32"` alone
        /// is TRUE for `wasm32-wasip2`, so the gate is always the
        /// `not(all(target_arch = "wasm32", target_env = "p2"))` compound. Ticket
        /// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,
        /// `🔍️research/📓️kurbo-peniko-first-party.md` (extends
        /// `🔍️research/📓️vello-scene-first-party.md`).
        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        pub use vello;
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        pub use vello::util;
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        pub use vello::wgpu;
        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        pub use vello::Scene;
        /// 🎨️ `usvg` (via `vello_svg`) is a rendering/text-shaping dependency: every real
        /// consumer — `svg_icon`'s paint pipeline, `SvgDocument`, `mod text`'s label shaper — is
        /// reachable only from the host/browser paint tree or from `semio-framework-editor`
        /// (itself absent from every plugin's `wasm32-wasip2` dependency graph), never from a
        /// WASI guest. Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-
        /// ARTIFACTS`, `🔍️research/📓️infinite-text-shaping.md`.
        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        pub use vello_svg;
        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        pub use vello_svg::usvg;
    }
    // #endregion 🏷️VelloBackend

    use geometry::{Affine, Arc, BezPath, Circle, CubicBez, Line, Rect, RoundedRect, ShapeRef};
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    use geometry::{PathEl, Point};
    use std::mem::ManuallyDrop;
    use std::sync::{Arc as SharedArc, Mutex, OnceLock};
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    use vello_backend as backend;

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn affine_to_kurbo(value: Affine) -> kurbo::Affine {
        kurbo::Affine::new(value.as_coeffs())
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn point_to_kurbo(value: Point) -> kurbo::Point {
        kurbo::Point::new(value.x, value.y)
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn path_element_to_kurbo(value: PathEl) -> kurbo::PathEl {
        match value {
            PathEl::MoveTo(point) => kurbo::PathEl::MoveTo(point_to_kurbo(point)),
            PathEl::LineTo(point) => kurbo::PathEl::LineTo(point_to_kurbo(point)),
            PathEl::QuadTo(control, point) => kurbo::PathEl::QuadTo(point_to_kurbo(control), point_to_kurbo(point)),
            PathEl::CurveTo(control1, control2, point) => kurbo::PathEl::CurveTo(point_to_kurbo(control1), point_to_kurbo(control2), point_to_kurbo(point)),
            PathEl::ClosePath => kurbo::PathEl::ClosePath,
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn shape_path_elements(shape: ShapeRef<'_>, tolerance: f64) -> Vec<PathEl> {
        match shape {
            ShapeRef::Rect(shape) => shape.path_elements(tolerance),
            ShapeRef::RoundedRect(shape) => shape.path_elements(tolerance),
            ShapeRef::Circle(shape) => shape.path_elements(tolerance),
            ShapeRef::Line(shape) => shape.path_elements(tolerance),
            ShapeRef::Arc(shape) => shape.path_elements(tolerance),
            ShapeRef::CubicBez(shape) => shape.path_elements(tolerance),
            ShapeRef::BezPath(shape) => shape.elements(),
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    struct KurboShapeAdapter<'a>(ShapeRef<'a>);

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    impl kurbo::Shape for KurboShapeAdapter<'_> {
        type PathElementsIter<'iter>
            = std::vec::IntoIter<kurbo::PathEl>
        where
            Self: 'iter;

        fn path_elements(&self, tolerance: f64) -> Self::PathElementsIter<'_> {
            shape_path_elements(self.0, tolerance).into_iter().map(path_element_to_kurbo).collect::<Vec<_>>().into_iter()
        }

        fn area(&self) -> f64 {
            kurbo::Shape::area(&self.to_path(0.1))
        }

        fn perimeter(&self, accuracy: f64) -> f64 {
            kurbo::Shape::perimeter(&self.to_path(accuracy), accuracy)
        }

        fn winding(&self, point: kurbo::Point) -> i32 {
            kurbo::Shape::winding(&self.to_path(0.1), point)
        }

        fn bounding_box(&self) -> kurbo::Rect {
            kurbo::Shape::bounding_box(&self.to_path(0.1))
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Cap {
        Butt,
        Round,
        Square,
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    impl From<Cap> for kurbo::Cap {
        fn from(value: Cap) -> Self {
            match value {
                Cap::Butt => Self::Butt,
                Cap::Round => Self::Round,
                Cap::Square => Self::Square,
            }
        }
    }

    /// 🔗️ First-party stroke join style — mirrors `kurbo::Join`. No guest call site sets it
    /// explicitly today (every `Stroke` is built via `new`, matching `kurbo::Stroke::new`'s own
    /// `Join::Round` default), but the field is real so `Stroke::to_kurbo` (host-only, below) can
    /// rebuild a real `kurbo::Stroke` losslessly rather than hardcoding a join.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Join {
        Bevel,
        Miter,
        Round,
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    impl From<Join> for kurbo::Join {
        fn from(value: Join) -> Self {
            match value {
                Join::Bevel => Self::Bevel,
                Join::Miter => Self::Miter,
                Join::Round => Self::Round,
            }
        }
    }

    /// 🖊️ First-party stroke style — plain value fields, no `kurbo::Stroke` wrapper, so
    /// `wasm32-wasip2` never needs `kurbo` to build/measure/dispose one. `to_kurbo` (host-only,
    /// below) rebuilds a real `kurbo::Stroke` only at rasterization time, matching
    /// `kurbo::Stroke::new`'s own field defaults exactly.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Stroke {
        width: f64,
        join: Join,
        miter_limit: f64,
        start_cap: Cap,
        end_cap: Cap,
        dash_pattern: Vec<f64>,
        dash_offset: f64,
    }

    impl Stroke {
        pub fn new(width: f64) -> Self {
            Self { width, join: Join::Round, miter_limit: 4.0, start_cap: Cap::Round, end_cap: Cap::Round, dash_pattern: Vec::new(), dash_offset: 0.0 }
        }
        pub fn set_dash_pattern(&mut self, pattern: Vec<f64>) {
            self.dash_pattern = pattern;
        }
        pub fn set_start_cap(&mut self, cap: Cap) {
            self.start_cap = cap;
        }
        pub fn set_end_cap(&mut self, cap: Cap) {
            self.end_cap = cap;
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    impl Stroke {
        /// 🖥️ Host/browser-only: rebuilds a real `kurbo::Stroke`, the only place this crate needs
        /// one, at the same `SceneCommand::replay_into` boundary `vello` itself is reached from.
        pub(crate) fn to_kurbo(&self) -> kurbo::Stroke {
            kurbo::Stroke {
                width: self.width,
                join: self.join.into(),
                miter_limit: self.miter_limit,
                start_cap: self.start_cap.into(),
                end_cap: self.end_cap.into(),
                dash_pattern: self.dash_pattern.iter().copied().collect(),
                dash_offset: self.dash_offset,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Rgba8 {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }

    /// 🎨️ First-party straight-alpha sRGB color — the same representation `peniko::Color`
    /// (`color::AlphaColor<color::Srgb>`) uses internally (`[f32; 4]`, straight not
    /// premultiplied), reimplemented so `wasm32-wasip2` never needs `peniko`/`color` to build or
    /// pass one. `to_peniko` (host-only, below) is a lossless `peniko::Color::new` at the
    /// rasterization boundary; `to_rgba8`/`from_rgba8` byte-for-byte match
    /// `color::AlphaColor::{to_rgba8,from_rgba8}` — proven differentially in `color_tests` below.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Color([f32; 4]);

    impl Color {
        pub fn new(rgba: [f32; 4]) -> Self {
            Self(rgba)
        }
        pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
            Self([r, g, b, a].map(|c| f32::from(c) * (1.0 / 255.0)))
        }
        pub fn to_rgba8(self) -> Rgba8 {
            let [r, g, b, a] = self.0.map(|c| (c * 255.0 + 0.5) as u8);
            Rgba8 { r, g, b, a }
        }
        pub fn components(self) -> [f32; 4] {
            self.0
        }
        pub fn multiply_alpha(self, alpha: f32) -> Self {
            let [r, g, b, a] = self.0;
            Self([r, g, b, a * alpha])
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    impl Color {
        /// 🖥️ Host/browser-only: rebuilds a real `peniko::Color`, only ever needed at the
        /// `SceneCommand::replay_into` rasterization boundary.
        pub(crate) fn to_peniko(self) -> peniko::Color {
            peniko::Color::new(self.0)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FillRule {
        NonZero,
        EvenOdd,
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    impl From<FillRule> for peniko::Fill {
        fn from(value: FillRule) -> Self {
            match value {
                FillRule::NonZero => Self::NonZero,
                FillRule::EvenOdd => Self::EvenOdd,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BlendMode {
        Normal,
        Multiply,
        Screen,
        Overlay,
        Darken,
        Lighten,
        ColorDodge,
        ColorBurn,
        HardLight,
        SoftLight,
        Difference,
        Exclusion,
        Hue,
        Saturation,
        Color,
        Luminosity,
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    impl From<BlendMode> for peniko::Mix {
        fn from(value: BlendMode) -> Self {
            match value {
                BlendMode::Normal => Self::Normal,
                BlendMode::Multiply => Self::Multiply,
                BlendMode::Screen => Self::Screen,
                BlendMode::Overlay => Self::Overlay,
                BlendMode::Darken => Self::Darken,
                BlendMode::Lighten => Self::Lighten,
                BlendMode::ColorDodge => Self::ColorDodge,
                BlendMode::ColorBurn => Self::ColorBurn,
                BlendMode::HardLight => Self::HardLight,
                BlendMode::SoftLight => Self::SoftLight,
                BlendMode::Difference => Self::Difference,
                BlendMode::Exclusion => Self::Exclusion,
                BlendMode::Hue => Self::Hue,
                BlendMode::Saturation => Self::Saturation,
                BlendMode::Color => Self::Color,
                BlendMode::Luminosity => Self::Luminosity,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Paint {
        Solid(Color),
    }

    impl From<Color> for Paint {
        fn from(value: Color) -> Self {
            Self::Solid(value)
        }
    }

    /// 🖼️ First-party RGBA8 raster image — plain `width`/`height`/`data` fields, no
    /// `peniko::ImageData` wrapper, so `wasm32-wasip2` never needs `peniko` to build/measure/
    /// dispose one. `to_peniko` (host-only, below) rebuilds a real `peniko::ImageData` (format
    /// `Rgba8`, alpha type `Alpha` — the only combination `rgba8` ever produces) only at
    /// rasterization time.
    #[derive(Clone, Debug, PartialEq)]
    pub struct RasterImage {
        width: u32,
        height: u32,
        data: SharedArc<Vec<u8>>,
    }

    impl RasterImage {
        /// @emoji 🖼️ Builds an RGBA8 raster image for scene drawing.
        pub fn rgba8(width: u32, height: u32, data: SharedArc<Vec<u8>>) -> Self {
            Self { width, height, data }
        }
        pub fn clone_data(&self) -> Self {
            Self { width: self.width, height: self.height, data: SharedArc::clone(&self.data) }
        }
        pub fn width(&self) -> u32 {
            self.width
        }
        pub fn height(&self) -> u32 {
            self.height
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    impl RasterImage {
        /// 🖥️ Host/browser-only: rebuilds a real `peniko::ImageData`, only ever needed at the
        /// `SceneCommand::replay_into` rasterization boundary.
        pub(crate) fn to_peniko(&self) -> peniko::ImageData {
            peniko::ImageData { data: peniko::Blob::new(self.data.clone()), format: peniko::ImageFormat::Rgba8, alpha_type: peniko::ImageAlphaType::Alpha, width: self.width, height: self.height }
        }
    }

    /// 🧪️ Differential proof that the first-party `Color`/`Stroke`/`RasterImage` reimplementations
    /// agree with the real `kurbo`/`peniko` (`color`) types they replaced on `wasm32-wasip2` — the
    /// oracle is used directly (this crate already depends on `kurbo`/`peniko` on the host target
    /// these tests run on, via the boundary conversions above), same convention as
    /// `semio-framework-geometry`'s `path_seg_tests`.
    #[cfg(all(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
    mod color_stroke_raster_tests {
        use super::*;

        const FIXTURES: [[f32; 4]; 12] = [
            [0.0, 0.0, 0.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.5, 0.5, 0.5, 0.5],
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 0.75],
            [0.0, 0.0, 1.0, 0.25],
            [0.019_607_844, 0.0, 0.0, 1.0],
            [0.996, 0.004, 0.5, 0.9999],
            [0.313_725_5, 0.627_451, 0.941_176_5, 1.0],
            [1.5, -0.2, 0.5, 1.0],
            [0.001, 0.999, 0.333, 0.667],
        ];

        #[test]
        fn to_rgba8_agrees_with_peniko_color_across_fixtures() {
            for rgba in FIXTURES {
                let ours = Color::new(rgba).to_rgba8();
                let oracle = peniko::Color::new(rgba).to_rgba8();
                assert_eq!((ours.r, ours.g, ours.b, ours.a), (oracle.r, oracle.g, oracle.b, oracle.a), "to_rgba8 mismatch for {rgba:?}");
            }
        }

        #[test]
        fn from_rgba8_agrees_with_peniko_color_across_every_channel_byte() {
            for byte in 0..=255u8 {
                let ours = Color::from_rgba8(byte, 255 - byte, byte / 2, byte).components();
                let oracle = peniko::Color::from_rgba8(byte, 255 - byte, byte / 2, byte).components;
                assert_eq!(ours, oracle, "from_rgba8 mismatch for byte {byte}");
            }
        }

        #[test]
        fn multiply_alpha_agrees_with_peniko_color_across_fixtures() {
            for rgba in FIXTURES {
                for factor in [0.0_f32, 0.25, 0.5, 1.0, 1.5] {
                    let ours = Color::new(rgba).multiply_alpha(factor).components();
                    let oracle = peniko::Color::new(rgba).multiply_alpha(factor).components;
                    assert_eq!(ours, oracle, "multiply_alpha mismatch for {rgba:?} * {factor}");
                }
            }
        }

        #[test]
        fn stroke_to_kurbo_matches_kurbo_stroke_new_defaults() {
            let ours = Stroke::new(3.5).to_kurbo();
            let oracle = kurbo::Stroke::new(3.5);
            assert_eq!(ours.width, oracle.width);
            assert_eq!(ours.join, oracle.join);
            assert_eq!(ours.miter_limit, oracle.miter_limit);
            assert_eq!(ours.start_cap, oracle.start_cap);
            assert_eq!(ours.end_cap, oracle.end_cap);
            assert_eq!(ours.dash_pattern.as_slice(), oracle.dash_pattern.as_slice());
            assert_eq!(ours.dash_offset, oracle.dash_offset);
        }

        #[test]
        fn stroke_to_kurbo_reflects_dash_pattern_and_cap_setters() {
            let mut stroke = Stroke::new(2.0);
            stroke.set_dash_pattern(vec![4.0, 2.0]);
            stroke.set_start_cap(Cap::Butt);
            stroke.set_end_cap(Cap::Square);
            let built = stroke.to_kurbo();
            assert_eq!(built.dash_pattern.as_slice(), &[4.0, 2.0]);
            assert_eq!(built.start_cap, kurbo::Cap::Butt);
            assert_eq!(built.end_cap, kurbo::Cap::Square);
        }

        #[test]
        fn raster_image_to_peniko_preserves_dimensions_and_bytes() {
            let data = SharedArc::new(vec![1u8, 2, 3, 4, 5, 6, 7, 8]);
            let image = RasterImage::rgba8(1, 2, SharedArc::clone(&data));
            let built = image.to_peniko();
            assert_eq!(built.width, 1);
            assert_eq!(built.height, 2);
            assert_eq!(built.format, peniko::ImageFormat::Rgba8);
            assert_eq!(built.alpha_type, peniko::ImageAlphaType::Alpha);
            assert_eq!(built.data.data(), data.as_slice());
        }

        #[test]
        fn raster_image_clone_data_shares_the_same_backing_allocation() {
            let data = SharedArc::new(vec![9u8, 9, 9]);
            let image = RasterImage::rgba8(4, 4, SharedArc::clone(&data));
            let cloned = image.clone_data();
            assert!(SharedArc::ptr_eq(&image.data, &cloned.data));
            assert_eq!(cloned.width(), 4);
            assert_eq!(cloned.height(), 4);
        }
    }

    /// 📦️ An owned copy of a [`ShapeRef`] variant, kept exact (never flattened to a polyline at
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
        /// 🔁️ Rewrites this command's own transform(s) as `outer * existing` — the same
        /// "child-space-then-outer" composition `vello::Scene::append` itself performs on its
        /// internal per-op transform stack, applied here one first-party command at a time so
        /// `Scene::append` can flatten `other`'s commands directly into `self`'s list (keeping
        /// every command independently poppable by `retirement_step`, instead of nesting a whole
        /// sub-scene behind one non-incremental drop).
        fn transformed(self, outer: Affine) -> Self {
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

        /// 🖥️ Host/browser-only: replays one command into a real, growing `vello::Scene`. The
        /// only place `vello`'s draw API is actually invoked — see the module docstring.
        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        fn replay_into(&self, built: &mut backend::Scene) {
            match self {
                Self::Fill { rule, transform, paint, brush_transform, shape } => {
                    let style: peniko::Fill = (*rule).into();
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
                    let style: peniko::Fill = (*rule).into();
                    let mix: peniko::Mix = (*blend).into();
                    let transform = affine_to_kurbo(*transform);
                    built.push_layer(style, mix, *alpha, transform, &KurboShapeAdapter(clip.as_shape_ref()));
                }
                Self::PushClipLayer { rule, transform, clip } => {
                    let style: peniko::Fill = (*rule).into();
                    let transform = affine_to_kurbo(*transform);
                    built.push_clip_layer(style, transform, &KurboShapeAdapter(clip.as_shape_ref()));
                }
                Self::PopLayer => built.pop_layer(),
                Self::VelloFragment { scene, transform } => built.append(scene, Some(affine_to_kurbo(*transform))),
            }
        }
    }

    #[derive(Clone, Default)]
    pub struct Scene(pub(crate) Vec<SceneCommand>);

    const OPAQUE_SCENE_RETIREMENT_CAPACITY: usize = 1024;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct OpaqueSceneRetirementToken {
        slot: u16,
        generation: u64,
    }

    struct OpaqueSceneRetirementSlot {
        generation: u64,
        scene: Option<ManuallyDrop<Scene>>,
    }

    struct OpaqueSceneRetirementRegistry {
        slots: Box<[OpaqueSceneRetirementSlot; OPAQUE_SCENE_RETIREMENT_CAPACITY]>,
        next: usize,
        faulted: bool,
    }

    impl Default for OpaqueSceneRetirementRegistry {
        fn default() -> Self {
            Self { slots: Box::new(std::array::from_fn(|_| OpaqueSceneRetirementSlot { generation: 0, scene: None })), next: 0, faulted: false }
        }
    }

    impl OpaqueSceneRetirementRegistry {
        fn reserve(&mut self) -> Option<OpaqueSceneRetirementToken> {
            if self.next == OPAQUE_SCENE_RETIREMENT_CAPACITY {
                self.faulted = true;
                return None;
            }
            let index = self.next;
            self.next += 1;
            let slot = &mut self.slots[index];
            slot.generation = slot.generation.wrapping_add(1).max(1);
            Some(OpaqueSceneRetirementToken { slot: index as u16, generation: slot.generation })
        }

        fn token_is_current(&self, token: OpaqueSceneRetirementToken) -> bool {
            self.slots.get(usize::from(token.slot)).is_some_and(|slot| slot.generation == token.generation && slot.scene.is_none())
        }

        fn publish(&mut self, token: OpaqueSceneRetirementToken, scene: Scene) {
            assert!(self.token_is_current(token), "opaque scene retirement token remains current before ownership transfer");
            self.slots[usize::from(token.slot)].scene = Some(ManuallyDrop::new(scene));
        }
    }

    fn opaque_scene_retirements() -> &'static Mutex<OpaqueSceneRetirementRegistry> {
        static REGISTRY: OnceLock<Mutex<OpaqueSceneRetirementRegistry>> = OnceLock::new();
        REGISTRY.get_or_init(|| Mutex::new(OpaqueSceneRetirementRegistry::default()))
    }

    pub fn reserve_opaque_scene_retirement() -> Option<OpaqueSceneRetirementToken> {
        let mut registry = opaque_scene_retirements().lock().expect("worker opaque scene retirement registry");
        registry.reserve()
    }

    pub fn publish_opaque_scene_retirement(token: OpaqueSceneRetirementToken, scene: Scene) {
        let mut registry = opaque_scene_retirements().lock().expect("worker opaque scene retirement registry");
        registry.publish(token, scene);
    }

    pub fn opaque_scene_retirement_status() -> (usize, bool) {
        let registry = opaque_scene_retirements().lock().expect("worker opaque scene retirement registry");
        (registry.next, registry.faulted)
    }

    impl Scene {
        pub fn new() -> Self {
            Self(Vec::new())
        }
        /// 🐌️ Retires (drops) exactly one recorded command per call — O(1) per call, same
        /// "spread a big drop across ticks" contract the 11-buffer `vello_encoding::Encoding`
        /// version had, just at first-party command granularity instead of sub-command buffer
        /// granularity (glyph-run buffers never populated on `wasm32-wasip2` per
        /// `🔍️research/📓️infinite-vello-image-split.md`, so no guest-side representation for
        /// them was ever needed here).
        pub fn retirement_step(&mut self) -> bool {
            self.0.pop().is_none()
        }

        pub fn retirement_is_empty(&self) -> bool {
            self.0.is_empty()
        }
        pub fn fill<'a>(&mut self, rule: FillRule, transform: Affine, paint: impl Into<Paint>, brush_transform: Option<Affine>, shape: impl Into<ShapeRef<'a>>) {
            self.0.push(SceneCommand::Fill { rule, transform, paint: paint.into(), brush_transform, shape: shape.into().into() });
        }
        pub fn stroke<'a>(&mut self, stroke: &Stroke, transform: Affine, paint: impl Into<Paint>, brush_transform: Option<Affine>, shape: impl Into<ShapeRef<'a>>) {
            self.0.push(SceneCommand::Stroke { stroke: stroke.clone(), transform, paint: paint.into(), brush_transform, shape: shape.into().into() });
        }
        pub fn draw_image(&mut self, image: &RasterImage, transform: Affine) {
            self.0.push(SceneCommand::DrawImage { image: image.clone_data(), transform });
        }
        /// 🔗️ Flattens `other`'s commands directly into `self` (composing `transform` into each,
        /// see `SceneCommand::transformed`) rather than nesting a sub-scene, so every appended
        /// command stays independently poppable by `retirement_step`.
        pub fn append(&mut self, other: &Scene, transform: Option<Affine>) {
            match transform {
                Some(outer) => self.0.extend(other.0.iter().cloned().map(|command| command.transformed(outer))),
                None => self.0.extend(other.0.iter().cloned()),
            }
        }
        pub fn push_layer<'a>(&mut self, rule: FillRule, blend: BlendMode, alpha: f32, transform: Affine, clip: impl Into<ShapeRef<'a>>) {
            self.0.push(SceneCommand::PushLayer { rule, blend, alpha, transform, clip: clip.into().into() });
        }
        pub fn pop_layer(&mut self) {
            self.0.push(SceneCommand::PopLayer);
        }
        pub fn push_clip_layer<'a>(&mut self, rule: FillRule, transform: Affine, clip: impl Into<ShapeRef<'a>>) {
            self.0.push(SceneCommand::PushClipLayer { rule, transform, clip: clip.into().into() });
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        /// 🔢️ A diagnostic hint (see `encoded_scene_hint`'s own docstring), not an exact-value
        /// contract — counts commands that draw or clip a path, same intent as the old
        /// `vello_encoding::Encoding::path_tags.len()` count but at first-party command
        /// granularity rather than per-verb granularity.
        pub fn path_count(&self) -> usize {
            self.0.iter().filter(|command| matches!(command, SceneCommand::Fill { .. } | SceneCommand::Stroke { .. } | SceneCommand::PushLayer { .. } | SceneCommand::PushClipLayer { .. })).count()
        }
        /// 🖥️ Host/browser-only: replays every recorded command into a freshly built
        /// `vello::Scene`, owned by the caller — the sole place this crate constructs a real
        /// `vello::Scene` from `Scene`'s first-party command list. `wasm32-wasip2` never reaches
        /// this method (nothing in `🎲️board` calls it — see module docstring).
        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        pub fn vello_scene(&self) -> backend::Scene {
            let mut built = backend::Scene::new();
            for command in &self.0 {
                command.replay_into(&mut built);
            }
            built
        }
    }

    /// @emoji 🏷️ Parsed SVG document for icon and label rasterization. Host/browser only — see
    /// `vello_backend`'s `usvg` re-export docstring above; its only real callers are
    /// `IconPaintCache::get_or_build`'s native arm and `#[cfg(test)]` code.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub struct SvgDocument(pub(crate) backend::usvg::Tree);

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    impl SvgDocument {
        pub(crate) fn from_tree(tree: backend::usvg::Tree) -> Self {
            Self(tree)
        }

        /// @emoji 🏷️ Appends the SVG tree into a scene. Builds a standalone real `vello::Scene`
        /// from the tree, then records it as one `SceneCommand::VelloFragment` — `Scene` itself
        /// stays a first-party command list even on this host/browser-only path.
        pub fn append_to_scene(&self, scene: &mut Scene) {
            let mut fragment = backend::Scene::new();
            vello_svg::append_tree(&mut fragment, &self.0);
            scene.0.push(SceneCommand::VelloFragment { scene: SharedArc::new(fragment), transform: Affine::IDENTITY });
        }
    }

    /// @emoji 🏷️ Appends a parsed SVG document into a scene.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn append_svg_document(scene: &mut Scene, doc: &SvgDocument) {
        doc.append_to_scene(scene);
    }

    #[cfg(test)]
    mod opaque_scene_retirement_tests {
        use super::*;

        #[test]
        fn fixed_quarantine_saturates_before_scene_ownership_transfer() {
            let mut registry = OpaqueSceneRetirementRegistry::default();
            for _ in 0..OPAQUE_SCENE_RETIREMENT_CAPACITY {
                let token = registry.reserve().expect("fixed quarantine credit");
                registry.publish(token, Scene::new());
            }
            assert!(registry.reserve().is_none());
            assert!(registry.faulted);
            assert_eq!(registry.next, OPAQUE_SCENE_RETIREMENT_CAPACITY);
        }

        #[test]
        fn late_opaque_scene_token_is_rejected_before_owner_publication() {
            let mut registry = OpaqueSceneRetirementRegistry::default();
            let token = registry.reserve().expect("fixed quarantine credit");
            registry.publish(token, Scene::new());
            assert!(!registry.token_is_current(token));
            assert_eq!(registry.next, 1);
        }
    }
}

pub use geometry::{append_shape_to_path, geom_sel, Affine, Arc, BezPath, Circle, CubicBez, Line, PathEl, Point, Rect, RoundedRect, RoundedRectRadii, ShapeRef, Vec2};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(crate) use renderer::vello_backend::usvg;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub use renderer::{append_svg_document, SvgDocument};
pub use renderer::{opaque_scene_retirement_status, publish_opaque_scene_retirement, reserve_opaque_scene_retirement, BlendMode, Cap, Color, FillRule, OpaqueSceneRetirementToken, Paint, RasterImage, Rgba8, Scene, Stroke};
// #endregion 🔖️Renderer

/// 📐️ First-party intrinsic-dimension reader (`🧰️framework/🔨️modules/📐️intrinsic-size`) — the
/// `wasm32-wasip2` arms of `icon_codec::decode_raster_icon_bytes` and
/// `svg_icon::svg_icon_content_bounds_from_str` use it instead of `image`/`usvg` so those two
/// dimension-only call sites stop pulling the ~50-crate SVG/raster pipeline onto the guest
/// component. Native/browser keep the real `image`/`usvg` decode unchanged. Ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,
/// `🔍️research/📓️intrinsic-size-wiring.md`.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework_intrinsic_size as intrinsic_size;

// #region ⚠️ Errors
/// @emoji 🚨️ SVG-parse failures raised by canvas icon/label rendering.
#[derive(Clone, Debug, PartialEq)]
pub enum CanvasError {
    /// @emoji 🏷️ SVG source failed to parse into a `usvg` tree.
    SvgParse(String),
}

impl std::fmt::Display for CanvasError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SvgParse(message) => write!(formatter, "SVG parse failed: {message}"),
        }
    }
}

impl std::error::Error for CanvasError {}
// #endregion ⚠️ Errors

pub mod theme {
    // #region theme
    //! @emoji 🎨️ Default canvas paint helpers from centralized styling tokens.

    use super::Color;
    use ui_styling::{appearance::AppearanceName, CANVAS_LIGHT};

    /// @emoji 🌈️ Maps a linear-sRGB token color to `Color`.
    pub fn linear_color(rgba: [f32; 4]) -> Color {
        Color::new(rgba)
    }

    /// @emoji 🎨️ Shared default clear color for graph board canvases.
    pub fn default_raster_clear() -> Color {
        linear_color(CANVAS_LIGHT.raster_clear)
    }

    /// @emoji 🎨️ Default themed icon foreground paint.
    pub fn default_icon_fg() -> Color {
        linear_color(CANVAS_LIGHT.icon_fg)
    }

    /// @emoji 🎨️ Default themed icon background paint.
    pub fn default_icon_bg() -> Color {
        linear_color(CANVAS_LIGHT.icon_bg)
    }

    /// @emoji 🎨️ Resolves canvas paints for a theme name.
    pub fn canvas_clear_for(theme: AppearanceName) -> Color {
        linear_color(theme.canvas().raster_clear)
    }

    /// @emoji 🌈️ Parses an sRGB8888 JSON array into `Color`.
    pub fn color_from_json_rgba8(arr: &[serde_json::Value]) -> Option<Color> {
        let r = u8::try_from(arr.first()?.as_u64().unwrap_or(0).min(255)).ok()?;
        let g = u8::try_from(arr.get(1)?.as_u64().unwrap_or(0).min(255)).ok()?;
        let b = u8::try_from(arr.get(2)?.as_u64().unwrap_or(0).min(255)).ok()?;
        let a = u8::try_from(arr.get(3).and_then(|x| x.as_u64()).unwrap_or(255).min(255)).ok()?;
        Some(Color::from_rgba8(r, g, b, a))
    }

    /// @emoji 🎨️ Merges one camelCase color field from a canvas theme JSON object.
    pub fn merge_color_field(next: &mut Color, v: &serde_json::Value, key: &str) {
        if let Some(arr) = v.get(key).and_then(|x| x.as_array()) {
            if let Some(c) = color_from_json_rgba8(arr) {
                *next = c;
            }
        }
    }

    /// @emoji 🌓️ Returns whether a canvas clear color reads as a light background.
    pub fn clear_is_light(clear: Color) -> bool {
        let [r, g, b, _] = clear.components();
        0.2126 * f64::from(r) + 0.7152 * f64::from(g) + 0.0722 * f64::from(b) > 0.5
    }

    /// @emoji 🎨️ Checkerboard cell shades for transparent raster layers.
    pub fn checkerboard_shades_for_clear(clear: Color) -> (u8, u8) {
        if clear_is_light(clear) {
            (220, 180)
        } else {
            (64, 48)
        }
    }
    // #endregion theme
}

// #region 🏷️IconAssets

pub mod icon_assets {
    //! @emoji 📎️ Static bytes for icon rendering; `include_bytes!` paths are relative to this `lib.rs` file.

    pub static NOTO_COLOR_EMOJI_SUBSET_TTF: &[u8] = include_bytes!("🖼️assets/🔤️NotoColorEmoji-subset.ttf");

    pub static MAP_LABEL_SANS_TTF: &[u8] = include_bytes!("🖼️assets/🔤️MapLabelSans.ttf");
}

// #endregion 🏷️IconAssets

pub mod svg_icon {
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    use std::sync::{Arc, OnceLock};

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    use super::usvg;
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    use super::{Affine, BezPath, Color, FillRule, Point, Scene, ShapeRef, Stroke};

    // #region 🔖️IconUsvgParseOptions

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    static ICON_USVG_OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();

    /// @emoji 🔤️ Shared `usvg` parse options with bundled Noto Color Emoji so `<text>` in Typst `emoji:` SVG matches the Typst font book; avoids system fallback glyphs.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn usvg_options_icons() -> &'static usvg::Options<'static> {
        ICON_USVG_OPTIONS.get_or_init(|| {
            let mut db = usvg::fontdb::Database::new();
            db.load_font_data(super::icon_assets::NOTO_COLOR_EMOJI_SUBSET_TTF.to_vec());
            usvg::Options { fontdb: Arc::new(db), font_family: ui_styling::canvas_fonts::NOTO_COLOR_EMOJI.into(), ..Default::default() }
        })
    }

    // #endregion 🔖️IconUsvgParseOptions

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn to_affine(ts: &usvg::Transform) -> Affine {
        let usvg::Transform { sx, kx, ky, sy, tx, ty } = *ts;
        Affine::new([sx, ky, kx, sy, tx, ty].map(f64::from))
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn to_bez_path(path: &usvg::Path) -> BezPath {
        let mut local_path = BezPath::new();
        let mut just_closed = false;
        let mut most_recent_initial = (0_f64, 0_f64);
        for elt in path.data().segments() {
            match elt {
                usvg::tiny_skia_path::PathSegment::MoveTo(p) => {
                    if std::mem::take(&mut just_closed) {
                        local_path.move_to(most_recent_initial);
                    }
                    most_recent_initial = (p.x.into(), p.y.into());
                    local_path.move_to(most_recent_initial);
                }
                usvg::tiny_skia_path::PathSegment::LineTo(p) => {
                    if std::mem::take(&mut just_closed) {
                        local_path.move_to(most_recent_initial);
                    }
                    local_path.line_to(Point::new(p.x as f64, p.y as f64));
                }
                usvg::tiny_skia_path::PathSegment::QuadTo(p1, p2) => {
                    if std::mem::take(&mut just_closed) {
                        local_path.move_to(most_recent_initial);
                    }
                    local_path.quad_to(Point::new(p1.x as f64, p1.y as f64), Point::new(p2.x as f64, p2.y as f64));
                }
                usvg::tiny_skia_path::PathSegment::CubicTo(p1, p2, p3) => {
                    if std::mem::take(&mut just_closed) {
                        local_path.move_to(most_recent_initial);
                    }
                    local_path.curve_to(Point::new(p1.x as f64, p1.y as f64), Point::new(p2.x as f64, p2.y as f64), Point::new(p3.x as f64, p3.y as f64));
                }
                usvg::tiny_skia_path::PathSegment::Close => {
                    just_closed = true;
                    local_path.close_path();
                }
            }
        }
        local_path
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn map_solid_icon_paint(paint: &usvg::Paint, opacity: usvg::Opacity, fg: Color, bg: Color) -> Option<Color> {
        let usvg::Paint::Color(c) = paint else {
            return None;
        };
        let a = opacity.get();
        if c.red < 22 && c.green < 22 && c.blue < 22 {
            return Some(fg.multiply_alpha(a));
        }
        if c.red > 233 && c.green > 233 && c.blue > 233 {
            return Some(bg.multiply_alpha(a));
        }
        Some(Color::from_rgba8(c.red, c.green, c.blue, opacity.to_u8()))
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn stroke_path(scene: &mut Scene, path: &usvg::Path, transform: Affine, local_path: &BezPath, fg: Color, bg: Color) {
        if let Some(stroke) = path.stroke() {
            if let Some(color) = map_solid_icon_paint(stroke.paint(), stroke.opacity(), fg, bg) {
                let conv = Stroke::new(f64::from(stroke.width().get()));
                scene.stroke(&conv, transform, color, None, ShapeRef::BezPath(local_path));
            }
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn fill_path(scene: &mut Scene, path: &usvg::Path, transform: Affine, local_path: &BezPath, fg: Color, bg: Color) {
        if let Some(fill) = path.fill() {
            if let Some(color) = map_solid_icon_paint(fill.paint(), fill.opacity(), fg, bg) {
                scene.fill(
                    match fill.rule() {
                        usvg::FillRule::NonZero => FillRule::NonZero,
                        usvg::FillRule::EvenOdd => FillRule::EvenOdd,
                    },
                    transform,
                    color,
                    None,
                    ShapeRef::BezPath(local_path),
                );
            }
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn render_path(scene: &mut Scene, path: &usvg::Path, fg: Color, bg: Color, stroke_first: bool) {
        if !path.is_visible() {
            return;
        }
        let transform = to_affine(&path.abs_transform());
        let local_path = to_bez_path(path);
        if stroke_first {
            stroke_path(scene, path, transform, &local_path, fg, bg);
            fill_path(scene, path, transform, &local_path, fg, bg);
        } else {
            fill_path(scene, path, transform, &local_path, fg, bg);
            stroke_path(scene, path, transform, &local_path, fg, bg);
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn render_group(scene: &mut Scene, group: &usvg::Group, fg: Color, bg: Color, stroke_first: bool) {
        for node in group.children() {
            match node {
                usvg::Node::Group(g) => render_group(scene, g, fg, bg, stroke_first),
                usvg::Node::Path(path) => render_path(scene, path, fg, bg, stroke_first),
                usvg::Node::Text(t) => render_group(scene, t.flattened(), fg, bg, true),
                _ => {}
            }
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn literal_paint(paint: &usvg::Paint, opacity: usvg::Opacity) -> Option<Color> {
        let usvg::Paint::Color(c) = paint else {
            return None;
        };
        Some(Color::from_rgba8(c.red, c.green, c.blue, opacity.to_u8()))
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn stroke_path_literal(scene: &mut Scene, path: &usvg::Path, transform: Affine, local_path: &BezPath) {
        if let Some(stroke) = path.stroke() {
            if let Some(color) = literal_paint(stroke.paint(), stroke.opacity()) {
                let conv = Stroke::new(f64::from(stroke.width().get()));
                scene.stroke(&conv, transform, color, None, ShapeRef::BezPath(local_path));
            }
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn fill_path_literal(scene: &mut Scene, path: &usvg::Path, transform: Affine, local_path: &BezPath) {
        if let Some(fill) = path.fill() {
            if let Some(color) = literal_paint(fill.paint(), fill.opacity()) {
                scene.fill(
                    match fill.rule() {
                        usvg::FillRule::NonZero => FillRule::NonZero,
                        usvg::FillRule::EvenOdd => FillRule::EvenOdd,
                    },
                    transform,
                    color,
                    None,
                    ShapeRef::BezPath(local_path),
                );
            }
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn render_path_literal(scene: &mut Scene, path: &usvg::Path, stroke_first: bool) {
        if !path.is_visible() {
            return;
        }
        let transform = to_affine(&path.abs_transform());
        let local_path = to_bez_path(path);
        if stroke_first {
            stroke_path_literal(scene, path, transform, &local_path);
            fill_path_literal(scene, path, transform, &local_path);
        } else {
            fill_path_literal(scene, path, transform, &local_path);
            stroke_path_literal(scene, path, transform, &local_path);
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn render_group_literal(scene: &mut Scene, group: &usvg::Group, stroke_first: bool) {
        for node in group.children() {
            match node {
                usvg::Node::Group(g) => render_group_literal(scene, g, stroke_first),
                usvg::Node::Path(path) => render_path_literal(scene, path, stroke_first),
                usvg::Node::Text(t) => render_group_literal(scene, t.flattened(), true),
                _ => {}
            }
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    /// @emoji 🏷️ Renders SVG tree paints literally (no icon fg/bg remapping); used for map labels.
    pub fn render_svg_tree_literal(scene: &mut Scene, tree: &usvg::Tree) {
        render_group_literal(scene, tree.root(), false);
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn icon_rect_xywh(r: usvg::Rect) -> Option<(f64, f64, f64, f64)> {
        let w = f64::from(r.width());
        let h = f64::from(r.height());
        if !(w > 1e-6 && h > 1e-6 && w.is_finite() && h.is_finite()) {
            return None;
        }
        Some((f64::from(r.x()), f64::from(r.y()), w, h))
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn icon_rect_nonzero(r: usvg::tiny_skia_path::NonZeroRect) -> (f64, f64, f64, f64) {
        (f64::from(r.x()), f64::from(r.y()), f64::from(r.width()), f64::from(r.height()))
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn icon_union_xywh(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
        let ax1 = a.0 + a.2;
        let ay1 = a.1 + a.3;
        let bx1 = b.0 + b.2;
        let by1 = b.1 + b.3;
        let x0 = a.0.min(b.0);
        let y0 = a.1.min(b.1);
        let x1 = ax1.max(bx1);
        let y1 = ay1.max(by1);
        (x0, y0, x1 - x0, y1 - y0)
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn icon_union_rects_into(acc: &mut Option<(f64, f64, f64, f64)>, r: usvg::Rect) {
        if let Some(xy) = icon_rect_xywh(r) {
            *acc = Some(match acc.take() {
                None => xy,
                Some(a) => icon_union_xywh(a, xy),
            });
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn icon_visit_node_bounds(node: &usvg::Node, acc: &mut Option<(f64, f64, f64, f64)>) {
        match node {
            usvg::Node::Group(g) => {
                for c in g.children() {
                    icon_visit_node_bounds(c, acc);
                }
            }
            usvg::Node::Path(p) => {
                if !p.is_visible() {
                    return;
                }
                icon_union_rects_into(acc, p.abs_bounding_box());
                icon_union_rects_into(acc, p.abs_stroke_bounding_box());
            }
            usvg::Node::Image(img) => {
                if !img.is_visible() {
                    return;
                }
                icon_union_rects_into(acc, img.abs_bounding_box());
            }
            usvg::Node::Text(t) => {
                icon_union_rects_into(acc, t.abs_bounding_box());
                icon_union_rects_into(acc, t.abs_stroke_bounding_box());
            }
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    /// @emoji 📐️ Union of visible paint bounds (paths, raster images, text) in absolute SVG space for uniform scale-and-center fits.
    pub fn svg_icon_content_bounds(tree: &usvg::Tree) -> (f64, f64, f64, f64) {
        let mut acc = None::<(f64, f64, f64, f64)>;
        for c in tree.root().children() {
            icon_visit_node_bounds(c, &mut acc);
        }
        if let Some(u) = acc {
            let (_, _, bw, bh) = u;
            if bw > 1e-6 && bh > 1e-6 {
                return u;
            }
        }
        let root = tree.root();
        let mut u = icon_rect_nonzero(root.abs_layer_bounding_box());
        if let Some(r) = icon_rect_xywh(root.abs_stroke_bounding_box()) {
            u = icon_union_xywh(u, r);
        }
        if let Some(r) = icon_rect_xywh(root.abs_bounding_box()) {
            u = icon_union_xywh(u, r);
        }
        let (_, _, bw, bh) = u;
        if bw > 1e-6 && bh > 1e-6 {
            return u;
        }
        let w = f64::from(tree.size().width());
        let h = f64::from(tree.size().height());
        (0.0, 0.0, w.max(1.0), h.max(1.0))
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn render_svg_tree_themed(scene: &mut Scene, tree: &usvg::Tree, fg: Color, bg: Color) {
        render_group(scene, tree.root(), fg, bg, false);
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    /// @emoji 🏷️ Parses SVG source and renders it themed into `scene`.
    pub fn append_svg_str_themed(scene: &mut Scene, svg: &str, fg: Color, bg: Color) -> Result<(), super::CanvasError> {
        let tree = usvg::Tree::from_str(svg, usvg_options_icons()).map_err(|e| super::CanvasError::SvgParse(e.to_string()))?;
        render_svg_tree_themed(scene, &tree, fg, bg);
        Ok(())
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    /// @emoji 🏷️ Parses SVG source and renders it with the default icon theme into `scene`.
    pub fn append_svg_str(scene: &mut Scene, svg: &str) -> Result<(), super::CanvasError> {
        append_svg_str_themed(scene, svg, super::theme::default_icon_fg(), super::theme::default_icon_bg())
    }

    /// @emoji 📐️ Parses SVG and returns visible content bounds in absolute SVG space.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn svg_icon_content_bounds_from_str(svg: &str) -> Result<(f64, f64, f64, f64), super::CanvasError> {
        let tree = usvg::Tree::from_str(svg, usvg_options_icons()).map_err(|e| super::CanvasError::SvgParse(e.to_string()))?;
        Ok(svg_icon_content_bounds(&tree))
    }

    /// 📐️ `wasm32-wasip2` arm: declared `width`/`height`/`viewBox` box
    /// (`semio-framework-intrinsic-size`), NOT the painted-ink content bbox the native/browser
    /// arm above computes via `usvg` — a disclosed, intentional behavior difference (no `usvg`
    /// renderer is linked on this target). `x`/`y` are always `0.0`: a declared box has no
    /// painted-content offset the way a bounding box of rendered ink can. Its only caller is
    /// `preview_media_natural_size`'s dimension query (never icon painting — see
    /// `IconPaintCache::get_or_build`'s own `wasm32-wasip2` arm). Ticket
    /// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    pub fn svg_icon_content_bounds_from_str(svg: &str) -> Result<(f64, f64, f64, f64), super::CanvasError> {
        let (w, h) = super::intrinsic_size::svg_intrinsic_size(svg).map_err(|e| super::CanvasError::SvgParse(e.to_string()))?;
        Ok((0.0, 0.0, w, h))
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl SvgDocument {
    /// @emoji 🏷️ Parses icon SVG with bundled emoji font options.
    pub fn parse_icons(svg: &str) -> Result<Self, CanvasError> {
        let tree = usvg::Tree::from_str(svg, svg_icon::usvg_options_icons()).map_err(|e| CanvasError::SvgParse(e.to_string()))?;
        Ok(Self::from_tree(tree))
    }

    /// @emoji 📐️ Visible content bounds in absolute SVG space.
    pub fn content_bounds(&self) -> (f64, f64, f64, f64) {
        svg_icon::svg_icon_content_bounds(&self.0)
    }

    /// @emoji 🏷️ Renders themed icon paints into a scene.
    pub fn render_themed(&self, scene: &mut Scene, fg: Color, bg: Color) {
        svg_icon::render_svg_tree_themed(scene, &self.0, fg, bg);
    }

    /// @emoji 🏷️ Renders literal SVG paints into a scene.
    pub fn render_literal(&self, scene: &mut Scene) {
        svg_icon::render_svg_tree_literal(scene, &self.0);
    }
}

// #region 🔖️Text
pub mod text {
    //! 🏷️ Real glyph-shaped label rendering (`append_label`/`append_label_tspans`) and
    //! `usvg`-shaped advance measurement (`label_byte_world_x`'s native arm, via
    //! `label_line_layout`/`label_prefix_advance_svg`) are host/browser only: every real caller
    //! repo-wide is either inside `DagHost`/`BoardHost`/`TrinityBridge`'s paint tree (host-only,
    //! see `IconPaintCache::get_or_build`'s docstring), behind flow's `wasm_session` browser
    //! bridge (already `not(target_env = "p2")`-gated, e.g. `begin_note_edit`), or in
    //! `semio-framework-editor` (not a dependency of any plugin's `wasm32-wasip2` build at all).
    //! The `wasm32-wasip2` arms below use the character-width heuristic
    //! (`label_extent`/`label_advance`/`label_text_inset`) already established in this module as
    //! the "no glyph shaping available" fallback. Ticket
    //! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,
    //! `🔍️research/📓️infinite-text-shaping.md`.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    use std::sync::{Arc, OnceLock};

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    use super::svg_icon::render_svg_tree_literal;
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    use super::usvg;
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    use super::{Affine, Vec2};
    use super::{Color, Point, Scene};

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    static MAP_LABEL_USVG_OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();

    /// @emoji 🔤️ `usvg` options with bundled map label sans for place-name labels.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn usvg_options_map_labels() -> &'static usvg::Options<'static> {
        MAP_LABEL_USVG_OPTIONS.get_or_init(|| {
            let mut db = usvg::fontdb::Database::new();
            db.load_font_data(super::icon_assets::MAP_LABEL_SANS_TTF.to_vec());
            let family = db.faces().next().and_then(|face| face.families.first().map(|(name, _)| name.clone())).unwrap_or_else(|| ui_styling::canvas_fonts::MAP_LABEL_SANS_FALLBACK.into());
            usvg::Options { fontdb: Arc::new(db), font_family: family, ..Default::default() }
        })
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn escape_xml_attr(s: &str) -> String {
        s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn color_to_svg(c: Color) -> String {
        let rgba = c.to_rgba8();
        if rgba.a == 255 {
            format!("#{:02x}{:02x}{:02x}", rgba.r, rgba.g, rgba.b)
        } else {
            let a = f64::from(rgba.a) / 255.0;
            format!("rgba({},{},{},{a})", rgba.r, rgba.g, rgba.b)
        }
    }

    /// @emoji 📐️ Estimated label box size in screen px for layout (matches `append_label` padding).
    pub fn label_extent(label: &str, px: f64) -> (f64, f64) {
        let trimmed = label.trim();
        if trimmed.is_empty() || px < ui_styling::metrics::label::MIN_PX {
            return (0.0, 0.0);
        }
        let pad = px * ui_styling::metrics::label::PAD_RATIO;
        let w = (trimmed.len() as f64 * px * ui_styling::metrics::label::CHAR_WIDTH_RATIO + pad * 2.0).clamp(ui_styling::metrics::label::WIDTH_MIN, ui_styling::metrics::label::WIDTH_MAX);
        let h = (px * ui_styling::metrics::label::HEIGHT_RATIO + pad * 2.0).clamp(ui_styling::metrics::label::HEIGHT_MIN, ui_styling::metrics::label::HEIGHT_MAX);
        (w, h)
    }

    /// @emoji ↔ Horizontal text advance inside a label box (excludes outer padding).
    pub fn label_advance(label: &str, px: f64) -> f64 {
        if label.is_empty() || px < ui_styling::metrics::label::MIN_PX {
            return 0.0;
        }
        label.len() as f64 * px * ui_styling::metrics::label::CHAR_WIDTH_RATIO
    }

    /// @emoji 📏️ Left inset from label origin to first glyph baseline start.
    pub fn label_text_inset(px: f64) -> f64 {
        if px < ui_styling::metrics::label::MIN_PX {
            return 0.0;
        }
        px * ui_styling::metrics::label::PAD_RATIO
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[derive(Clone, Copy, Debug)]
    struct LabelLineLayout {
        bx: f64,
        scale: f64,
        pad: f64,
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn label_line_layout(line: &str, px: f64) -> Option<LabelLineLayout> {
        if px < ui_styling::metrics::label::MIN_PX {
            return None;
        }
        let pad = px * ui_styling::metrics::label::PAD_RATIO;
        let extent_line = if line.is_empty() { " " } else { line };
        let (w, h) = label_extent(extent_line, px);
        let text_y = pad + px;
        let family = usvg_options_map_labels().font_family.clone();
        let body = if line.is_empty() { " " } else { line };
        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><text x="{pad}" y="{text_y}" font-size="{px}" font-family="{family}">{text}</text></svg>"##,
            w = w,
            h = h,
            pad = pad,
            text_y = text_y,
            px = px,
            family = escape_xml_attr(&family),
            text = escape_xml_attr(body),
        );
        let tree = usvg::Tree::from_str(&svg, usvg_options_map_labels()).ok()?;
        let (bx, _, bw, bh) = super::svg_icon::svg_icon_content_bounds(&tree);
        if bw <= 0.0 || bh <= 0.0 {
            return None;
        }
        let scale = (px * ui_styling::metrics::label::SCALE_RATIO / bh).min(ui_styling::metrics::label::SCALE_MAX);
        Some(LabelLineLayout { bx, scale, pad })
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn label_prefix_advance_svg(line: &str, byte_end: usize, px: f64) -> f64 {
        let end = byte_end.min(line.len());
        if end == 0 {
            return 0.0;
        }
        if !line.is_char_boundary(end) {
            let prev = line[..end].char_indices().next_back().map_or(0, |(i, _)| i);
            return label_prefix_advance_svg(line, prev, px);
        }
        let prefix = &line[..end];
        let pad = px * ui_styling::metrics::label::PAD_RATIO;
        let (w, h) = label_extent(prefix, px);
        let text_y = pad + px;
        let family = usvg_options_map_labels().font_family.clone();
        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><text x="{pad}" y="{text_y}" font-size="{px}" font-family="{family}">{text}</text></svg>"##,
            w = w,
            h = h,
            pad = pad,
            text_y = text_y,
            px = px,
            family = escape_xml_attr(&family),
            text = escape_xml_attr(prefix),
        );
        let Ok(tree) = usvg::Tree::from_str(&svg, usvg_options_map_labels()) else {
            return label_advance(prefix, px);
        };
        let (bx, _, bw, bh) = super::svg_icon::svg_icon_content_bounds(&tree);
        if bw <= 0.0 || bh <= 0.0 {
            return label_advance(prefix, px);
        }
        (bx + bw) - pad
    }

    /// @emoji ↔ World x for a byte offset in a code line (matches `append_label_tspans` layout).
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn label_byte_world_x(line: &str, byte_offset: usize, origin_x: f64, px: f64) -> f64 {
        let Some(layout) = label_line_layout(line, px) else {
            return origin_x;
        };
        let advance = label_prefix_advance_svg(line, byte_offset, px);
        origin_x + (layout.pad + advance - layout.bx) * layout.scale
    }

    /// ↔ `wasm32-wasip2` arm: no `usvg` text shaper is linked on this target (see this module's
    /// top docstring — every real caller is host/browser-only or unreachable). Uses the same
    /// character-width heuristic (`label_advance`/`label_text_inset`) this module already falls
    /// back to when real glyph shaping is unavailable (see `label_prefix_advance_svg`'s own
    /// `usvg::Tree::from_str` failure arm above) — a disclosed, intentional precision difference,
    /// not a stub: byte offsets still map to a monotonically increasing world x. Ticket
    /// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    pub fn label_byte_world_x(line: &str, byte_offset: usize, origin_x: f64, px: f64) -> f64 {
        let end = byte_offset.min(line.len());
        let end = if line.is_char_boundary(end) { end } else { line[..end].char_indices().next_back().map_or(0, |(i, _)| i) };
        origin_x + label_text_inset(px) + label_advance(&line[..end], px)
    }

    /// @emoji ↔ World x range for a byte span in a code line.
    pub fn label_span_world_x(line: &str, byte_start: usize, byte_end: usize, origin_x: f64, px: f64) -> (f64, f64) {
        (label_byte_world_x(line, byte_start, origin_x, px), label_byte_world_x(line, byte_end, origin_x, px))
    }

    /// @emoji 🏷️ Renders a single map label via SVG text at `origin` (screen px, baseline).
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn append_label(scene: &mut Scene, label: &str, origin: Point, px: f64, fill: Color, halo: Color) {
        let trimmed = label.trim();
        if trimmed.is_empty() || px < ui_styling::metrics::label::MIN_PX {
            return;
        }
        let pad = px * ui_styling::metrics::label::PAD_RATIO;
        let (w, h) = label_extent(trimmed, px);
        let text_y = pad + px;
        let family = usvg_options_map_labels().font_family.clone();
        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><text x="{pad}" y="{text_y}" font-size="{px}" font-family="{family}" fill="{fill}" stroke="{halo}" stroke-width="{stroke}" paint-order="stroke">{text}</text></svg>"##,
            w = w,
            h = h,
            pad = pad,
            text_y = text_y,
            px = px,
            family = escape_xml_attr(&family),
            fill = color_to_svg(fill),
            halo = color_to_svg(halo),
            stroke = (px * ui_styling::metrics::label::HALO_STROKE_RATIO).max(ui_styling::metrics::label::HALO_STROKE_MIN),
            text = escape_xml_attr(trimmed),
        );
        let Ok(tree) = usvg::Tree::from_str(&svg, usvg_options_map_labels()) else {
            return;
        };
        let (bx, by, bw, bh) = super::svg_icon::svg_icon_content_bounds(&tree);
        if bw <= 0.0 || bh <= 0.0 {
            return;
        }
        let scale = (px * ui_styling::metrics::label::SCALE_RATIO / bh).min(ui_styling::metrics::label::SCALE_MAX);
        let mut label_scene = Scene::new();
        render_svg_tree_literal(&mut label_scene, &tree);
        let aff = Affine::IDENTITY.translate(Vec2::new(origin.x() - bx * scale, origin.y() - by * scale - px * ui_styling::metrics::label::VERTICAL_OFFSET_RATIO)).scale(scale);
        scene.append(&label_scene, Some(aff));
    }

    /// 🚫️ `wasm32-wasip2` arm: label *painting* (rasterizing shaped glyphs into `Scene`) is
    /// host-only, same reasoning as `IconPaintCache::get_or_build`'s `wasm32-wasip2` arm — a WASI
    /// guest has no display to paint onto, and this module's top docstring traces every real
    /// caller as host/browser-only or unreachable. A no-op, not a stub: nothing on this target
    /// could ever observe a painted label today. Ticket
    /// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    pub fn append_label(_scene: &mut Scene, _label: &str, _origin: Point, _px: f64, _fill: Color, _halo: Color) {}

    /// @emoji 🏷️ Renders one label with colored inline tspans (single padding box, no per-span gaps).
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn append_label_tspans(scene: &mut Scene, line: &str, spans: &[(usize, usize, Color)], origin: Point, px: f64, _halo: Color) {
        if line.is_empty() || spans.is_empty() || px < ui_styling::metrics::label::MIN_PX {
            return;
        }
        let pad = px * ui_styling::metrics::label::PAD_RATIO;
        let (w, h) = label_extent(line, px);
        let text_y = pad + px;
        let family = usvg_options_map_labels().font_family.clone();
        let mut inner = String::new();
        for &(start, end, fill) in spans {
            if start >= end || end > line.len() {
                continue;
            }
            let slice = &line[start..end];
            if slice.is_empty() {
                continue;
            }
            inner.push_str(&format!(r#"<tspan fill="{fill}">{text}</tspan>"#, fill = color_to_svg(fill), text = escape_xml_attr(slice),));
        }
        if inner.is_empty() {
            return;
        }
        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><text x="{pad}" y="{text_y}" font-size="{px}" font-family="{family}">{inner}</text></svg>"##,
            w = w,
            h = h,
            pad = pad,
            text_y = text_y,
            px = px,
            family = escape_xml_attr(&family),
            inner = inner,
        );
        let Ok(tree) = usvg::Tree::from_str(&svg, usvg_options_map_labels()) else {
            return;
        };
        let (bx, by, bw, bh) = super::svg_icon::svg_icon_content_bounds(&tree);
        if bw <= 0.0 || bh <= 0.0 {
            return;
        }
        let scale = (px * ui_styling::metrics::label::SCALE_RATIO / bh).min(ui_styling::metrics::label::SCALE_MAX);
        let mut label_scene = Scene::new();
        render_svg_tree_literal(&mut label_scene, &tree);
        let aff = Affine::IDENTITY.translate(Vec2::new(origin.x() - bx * scale, origin.y() - by * scale - px * ui_styling::metrics::label::VERTICAL_OFFSET_RATIO)).scale(scale);
        scene.append(&label_scene, Some(aff));
    }

    /// 🚫️ `wasm32-wasip2` arm: see `append_label`'s `wasm32-wasip2` arm docstring — identical
    /// reasoning, this is the multi-span sibling.
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    pub fn append_label_tspans(_scene: &mut Scene, _line: &str, _spans: &[(usize, usize, Color)], _origin: Point, _px: f64, _halo: Color) {}
}
// #endregion 🔖️Text

// #region 🔖️Camera
pub mod camera {
    use super::{Affine, Point};

    pub const CANVAS_CAMERA_ZOOM_MIN: f64 = ui_styling::metrics::camera::ZOOM_MIN;
    pub const CANVAS_CAMERA_ZOOM_MAX: f64 = ui_styling::metrics::camera::ZOOM_MAX;

    #[derive(Clone, Debug)]
    pub struct Camera {
        pub x: f64,
        pub y: f64,
        pub zoom: f64,
    }

    impl Default for Camera {
        fn default() -> Self {
            Self { x: 0.0, y: 0.0, zoom: 1.0 }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Viewport {
        pub width: u32,
        pub height: u32,
        pub dpr: f64,
    }

    impl Default for Viewport {
        fn default() -> Self {
            Self { width: 1, height: 1, dpr: 1.0 }
        }
    }

    impl Viewport {
        pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
            self.width = width.max(1);
            self.height = height.max(1);
            self.dpr = dpr.max(1.0);
        }

        pub fn physical_size(&self) -> (u32, u32) {
            let pw = ((self.width as f64 * self.dpr).round() as u32).max(1);
            let ph = ((self.height as f64 * self.dpr).round() as u32).max(1);
            (pw, ph)
        }
    }

    pub fn clamp_zoom(zoom: f64) -> f64 {
        zoom.clamp(CANVAS_CAMERA_ZOOM_MIN, CANVAS_CAMERA_ZOOM_MAX)
    }

    pub fn world_to_screen(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
        Point::new((p.x - camera.x) * camera.zoom + viewport.width as f64 / 2.0, (p.y - camera.y) * camera.zoom + viewport.height as f64 / 2.0)
    }

    pub fn screen_to_world(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
        Point::new((p.x - viewport.width as f64 / 2.0) / camera.zoom + camera.x, (p.y - viewport.height as f64 / 2.0) / camera.zoom + camera.y)
    }

    pub fn camera_content_affine(camera: &Camera, viewport: &Viewport) -> Affine {
        let z = camera.zoom;
        Affine::new([z, 0.0, 0.0, z, viewport.width as f64 * 0.5 - camera.x * z, viewport.height as f64 * 0.5 - camera.y * z])
    }

    pub fn wheel_screen(camera: &mut Camera, viewport: &Viewport, sx: f64, sy: f64, delta_y: f64) {
        let zoom_factor = if delta_y < 0.0 { ui_styling::metrics::camera::WHEEL_ZOOM_IN_FACTOR } else { ui_styling::metrics::camera::WHEEL_ZOOM_OUT_FACTOR };
        let next_zoom = clamp_zoom(camera.zoom * zoom_factor);
        let screen = Point::new(sx, sy);
        let world_before = screen_to_world(camera, viewport, screen);
        camera.x = world_before.x - (sx - viewport.width as f64 / 2.0) / next_zoom;
        camera.y = world_before.y - (sy - viewport.height as f64 / 2.0) / next_zoom;
        camera.zoom = next_zoom;
    }
}
// #endregion 🔖️Camera

// #region 🔖️Lod
pub mod lod {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Lod {
        pub id: &'static str,
        pub name: &'static str,
        pub description: &'static str,
        pub max_zoom: f64,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct LodScale {
        pub lods: &'static [Lod],
    }

    impl LodScale {
        pub fn resolve_index(self, zoom: f64) -> usize {
            for (i, lod) in self.lods.iter().enumerate() {
                if zoom < lod.max_zoom {
                    return i;
                }
            }
            self.lods.len().saturating_sub(1)
        }

        pub fn resolve(self, zoom: f64) -> &'static Lod {
            &self.lods[self.resolve_index(zoom)]
        }

        pub fn index_of(self, id: &str) -> Option<usize> {
            self.lods.iter().position(|lod| lod.id == id)
        }
    }

    /// @emoji 🔤️ Fixed screen label px for a LOD band; stays constant while zooming inside the band.
    pub fn band_label_screen_px(band_px: &[f64], band_index: usize, fallback: f64) -> f64 {
        band_px.get(band_index).copied().unwrap_or(fallback)
    }

    /// @emoji 🔤️ Lower camera-zoom bound for a LOD band (previous band `max_zoom`, or `zoom_min`).
    pub fn band_floor_zoom(band_floor_zoom: &[f64], band_index: usize, zoom_min: f64) -> f64 {
        band_floor_zoom.get(band_index).copied().unwrap_or(zoom_min).max(zoom_min)
    }

    /// @emoji 🔤️ Label screen px scaled with camera zoom inside one LOD band so text keeps the same proportion to world geometry.
    pub fn lod_band_label_screen_px(base_screen_px: f64, zoom: f64, band_floor_zoom: f64) -> f64 {
        let z = zoom.max(ui_styling::metrics::camera::LOD_ZOOM_FLOOR);
        let floor = band_floor_zoom.max(ui_styling::metrics::camera::LOD_ZOOM_FLOOR);
        base_screen_px * z / floor
    }
}
// #endregion 🔖️Lod

// #region 🔖️Raster
pub mod raster {
    use super::{Affine, RasterImage, Scene};
    use std::collections::HashMap;
    use std::sync::Arc;

    pub fn draw_image(scene: &mut Scene, image: &RasterImage, transform: Affine) {
        scene.draw_image(image, transform);
    }

    pub fn draw_image_arc(scene: &mut Scene, image: &Arc<RasterImage>, transform: Affine) {
        scene.draw_image(image, transform);
    }

    #[derive(Clone, Default)]
    pub struct RasterImageCache {
        entries: HashMap<String, Arc<RasterImage>>,
    }

    impl RasterImageCache {
        pub fn get(&self, key: &str) -> Option<Arc<RasterImage>> {
            self.entries.get(key).cloned()
        }

        pub fn insert(&mut self, key: String, image: RasterImage) -> Arc<RasterImage> {
            let arc = Arc::new(image);
            self.entries.insert(key, arc.clone());
            arc
        }
    }
}
// #endregion 🔖️Raster

// #region 🔖️Render
pub mod render {
    use super::{Affine, Scene};

    /// @emoji 📐️ Scales a logical-viewport scene to the physical GPU surface (device pixel ratio).
    pub fn scale_scene_for_device_pixel_ratio(scene: Scene, dpr: f64) -> Scene {
        let scale = dpr.max(1.0);
        if (scale - 1.0).abs() < f64::EPSILON {
            return scene;
        }
        let mut scaled = Scene::new();
        scaled.append(&scene, Some(Affine::IDENTITY.scale(scale)));
        scaled
    }
}
// #endregion 🔖️Render

// #region 🔖️CanvasContent
pub mod canvas_content {
    use super::{Color, Scene};

    pub trait CanvasContent {
        fn build_scene(&self) -> Scene;
        fn clear_color(&self) -> Color;
    }
}
// #endregion 🔖️CanvasContent

// #region 🔖️GpuSession
// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this is the browser WebGPU canvas
// session shared by every surface/editor session bridge, so it is narrowed to exclude the WASI
// component target (which attaches no `HtmlCanvasElement`).
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub mod gpu_session {
    use super::renderer::vello_backend::{util, vello, wgpu};
    use super::{Color, Scene};
    use wasm_bindgen::prelude::JsValue;
    use web_sys::HtmlCanvasElement;

    #[derive(Default)]
    pub struct CanvasGpuSession {
        #[allow(dead_code, reason = "Retains canvas for the WebGPU surface lifetime.")]
        canvas: Option<HtmlCanvasElement>,
        render_ctx: Option<util::RenderContext>,
        renderer: Option<vello::Renderer>,
        surface: Option<util::RenderSurface<'static>>,
    }

    impl CanvasGpuSession {
        pub fn gpu_ready(&self) -> bool {
            self.surface.is_some()
        }

        /// @emoji 🖥️ WebGPU surface bring-up; returns `String` (not `CanvasError`) because every call site is a wasm-bindgen boundary fn that immediately erases the error into a `JsValue` for JS — see `render_frame` below for the same convention.
        pub async fn create_canvas_surface(canvas: HtmlCanvasElement, pw: u32, ph: u32) -> Result<(util::RenderContext, vello::Renderer, util::RenderSurface<'static>), String> {
            let mut render_ctx = util::RenderContext::new();
            let surface = render_ctx.create_surface(wgpu::SurfaceTarget::Canvas(canvas), pw, ph, wgpu::PresentMode::AutoVsync).await.map_err(|err| format!("{err:?}"))?;
            let dev = &render_ctx.devices[surface.dev_id].device;
            let renderer =
                vello::Renderer::new(dev, vello::RendererOptions { use_cpu: false, antialiasing_support: vello::AaSupport::area_only(), num_init_threads: std::num::NonZeroUsize::new(1), pipeline_cache: None }).map_err(|err| format!("{err:?}"))?;
            Ok((render_ctx, renderer, surface))
        }

        pub fn finish_attach(&mut self, canvas: HtmlCanvasElement, render_ctx: util::RenderContext, renderer: vello::Renderer, surface: util::RenderSurface<'static>) {
            self.canvas = Some(canvas);
            self.render_ctx = Some(render_ctx);
            self.renderer = Some(renderer);
            self.surface = Some(surface);
        }

        pub fn resize_surface(&mut self, pw: u32, ph: u32) {
            if let (Some(surface), Some(render_ctx)) = (self.surface.as_mut(), self.render_ctx.as_mut()) {
                let cur_w = surface.config.width;
                let cur_h = surface.config.height;
                if cur_w != pw || cur_h != ph {
                    render_ctx.resize_surface(surface, pw, ph);
                }
            }
        }

        pub fn detach(&mut self) {
            self.canvas = None;
            self.render_ctx = None;
            self.renderer = None;
            self.surface = None;
        }

        pub fn render_frame(&mut self, scene: &Scene, clear_color: Color) -> Result<(), JsValue> {
            for _attempt in 0..3u8 {
                let (surface, renderer, render_ctx) = match (self.surface.as_mut(), self.renderer.as_mut(), self.render_ctx.as_mut()) {
                    (Some(s), Some(r), Some(rc)) => (s, r, rc),
                    _ => return Ok(()),
                };
                let dh = &render_ctx.devices[surface.dev_id];
                let pw = surface.config.width.max(1);
                let ph = surface.config.height.max(1);
                let params = vello::RenderParams { base_color: clear_color.to_peniko(), width: pw, height: ph, antialiasing_method: vello::AaConfig::Area };
                let vello_scene = scene.vello_scene();
                renderer.render_to_texture(&dh.device, &dh.queue, &vello_scene, &surface.target_view, &params).map_err(|err| JsValue::from_str(&format!("{err:?}")))?;

                let surface_tex = match surface.surface.get_current_texture() {
                    Ok(t) => t,
                    Err(wgpu::SurfaceError::Outdated) => {
                        surface.surface.configure(&dh.device, &surface.config);
                        continue;
                    }
                    Err(wgpu::SurfaceError::Timeout) | Err(wgpu::SurfaceError::Other) => return Ok(()),
                    Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::OutOfMemory) => {
                        return Err(JsValue::from_str("surface lost or validation error"));
                    }
                };
                let view = surface_tex.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = dh.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("infinite_canvas_surface_blit") });
                surface.blitter.copy(&dh.device, &mut encoder, &surface.target_view, &view);
                dh.queue.submit(std::iter::once(encoder.finish()));
                surface_tex.present();
                let _ = dh.device.poll(wgpu::PollType::Poll).ok();
                return Ok(());
            }
            Ok(())
        }
    }
}
// #endregion 🔖️GpuSession

// #region 🔖️IconCodec
#[path = "../../../../../🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🤖️generated/🦀️metabolism_icon_name.rs"]
mod metabolism_icon_name_gen;

#[path = "../../../../../🔨️modules/🖼️assets/🔣️icons/🤖️generated/🦀️icon_name.rs"]
mod catalog_icon_name_gen;

pub use catalog_icon_name_gen::IconName;
pub use metabolism_icon_name_gen::MetabolismIconName;

pub mod icon_codec {
    // #region icon_codec
    //! 🖼️ Generic icon encoding resolver for board nodes (url, shortcode, math, emoji, raster, inline SVG, catalog, themed, text).

    pub use super::{IconName, MetabolismIconName};

    use serde::{Deserialize, Serialize};
    use std::sync::Arc;

    mod icon_shortcodes {
        include!(concat!(env!("OUT_DIR"), "/icon_shortcode_match.rs"));
    }

    /// 🔍️ Optional lookup for domain-themed SVG icons (e.g. puzzle metabolism table).
    pub type ThemedSvgLookup = fn(&str) -> Option<&'static str>;

    // #region 🏷️IconUnion

    /// @emoji 🖼️ Canonical structured icon payload shared across canvases and UI chrome.
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum Icon {
        Url { url: String },
        Shortcode { code: String },
        Data { data: String },
        Emoji { emoji: String },
        Math { src: String },
        Text { text: String },
        Svg { svg: String },
        Catalog { key: IconName },
        Themed { key: MetabolismIconName },
    }

    fn is_raster_data_url_payload(s: &str) -> bool {
        let u = s.trim().to_ascii_lowercase();
        u.starts_with("data:image/png;base64,") || u.starts_with("data:image/jpeg;base64,") || u.starts_with("data:image/jpg;base64,") || u.starts_with("data:image/webp;base64,") || u.starts_with("data:image/gif;base64,")
    }

    fn is_svg_data_url_payload(s: &str) -> bool {
        s.trim().to_ascii_lowercase().starts_with("data:image/svg+xml")
    }

    fn looks_like_shortcode_token(t: &str) -> bool {
        t.len() >= 3 && t.starts_with(':') && t.ends_with(':') && t[1..t.len() - 1].chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '+' | '-'))
    }

    fn shortcode_inner(t: &str) -> Option<&str> {
        if !looks_like_shortcode_token(t) {
            return None;
        }
        Some(&t[1..t.len() - 1])
    }

    fn looks_like_ascii_catalogish_stem(s: &str) -> bool {
        let t = s.trim();
        if t.is_empty() || !t.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-')) {
            return false;
        }
        matches!(t.chars().next(), Some(c) if c.is_ascii_alphabetic() || c == '_') && (t.contains('.') || t.contains('_') || t.contains('-') || t.len() > 48)
    }

    fn is_extended_pictographic_char(c: char) -> bool {
        let cp = c as u32;
        matches!(cp, 0x1F1E6..=0x1F1FF | 0x1F300..=0x1FAFF | 0x2600..=0x27BF | 0x2300..=0x23FF) || matches!(c, '©' | '®' | '™' | '☺' | '☻' | '♥' | '♦' | '♣' | '♠' | '✓' | '✔' | '✕' | '✖' | '✗' | '✘') || c == '\u{FE0F}' || c == '\u{200D}'
    }

    fn looks_like_bare_emoji(s: &str) -> bool {
        let t = s.trim();
        !t.is_empty() && t.chars().any(is_extended_pictographic_char)
    }

    fn looks_like_bare_url(s: &str) -> bool {
        let lower = s.trim().to_ascii_lowercase();
        lower.starts_with("http://") || lower.starts_with("https://")
    }

    fn resolve_inline_svg(encoded: &str) -> Option<String> {
        let t = encoded.trim();
        if t.is_empty() {
            return None;
        }
        let lower = t.to_ascii_lowercase();
        if lower.starts_with("<?xml") || lower.contains("<svg") {
            return Some(t.to_string());
        }
        None
    }

    /// @emoji 🔤️ Decodes a canonical icon string into a structured {@link Icon}.
    pub fn decode_icon(encoded: &str) -> Option<Icon> {
        let t = encoded.trim();
        if t.is_empty() {
            return None;
        }
        if let Some(url) = t.strip_prefix("url:") {
            let url = url.trim();
            return (!url.is_empty()).then(|| Icon::Url { url: url.to_string() });
        }
        if looks_like_bare_url(t) {
            return Some(Icon::Url { url: t.to_string() });
        }
        if let Some(code) = shortcode_inner(t) {
            return Some(Icon::Shortcode { code: code.to_string() });
        }
        if let Some(src) = t.strip_prefix("math:") {
            let src = src.trim();
            return (!src.is_empty()).then(|| Icon::Math { src: src.to_string() });
        }
        // `$formula$` decode sugar — generic math-notation culture, not a Typst leftover (the icon
        // selector's own placeholder already teaches it). Unlike Typst's own `$...$` inline-math
        // marker, the compiler's grammar has no delimiter syntax of its own, so the `$`s must be
        // STRIPPED here rather than passed through verbatim.
        if t.len() >= 2 && t.starts_with('$') && t.ends_with('$') {
            let inner = t[1..t.len() - 1].trim();
            return (!inner.is_empty()).then(|| Icon::Math { src: inner.to_string() });
        }
        if let Some(em) = t.strip_prefix("emoji:") {
            let em = em.trim();
            return (!em.is_empty()).then(|| Icon::Emoji { emoji: em.to_string() });
        }
        if let Some(text) = t.strip_prefix("text:") {
            let text = text.trim();
            return (!text.is_empty()).then(|| Icon::Text { text: text.to_string() });
        }
        if is_raster_data_url_payload(t) || is_svg_data_url_payload(t) || t.to_ascii_lowercase().starts_with("data:") {
            return Some(Icon::Data { data: t.to_string() });
        }
        if let Some(svg) = resolve_inline_svg(t) {
            return Some(Icon::Svg { svg });
        }
        if let Some(key) = MetabolismIconName::from_str(t) {
            return Some(Icon::Themed { key });
        }
        if let Some(key) = IconName::from_str(t) {
            return Some(Icon::Catalog { key });
        }
        if looks_like_ascii_catalogish_stem(t) {
            return None;
        }
        if looks_like_bare_emoji(t) {
            return Some(Icon::Emoji { emoji: t.to_string() });
        }
        if t.chars().count() <= 16 {
            return Some(Icon::Text { text: t.to_string() });
        }
        None
    }

    /// @emoji 🔤️ Encodes a structured {@link Icon} into the canonical wire string.
    pub fn encode_icon(icon: &Icon) -> String {
        match icon {
            Icon::Url { url } => format!("url:{}", url.trim()),
            Icon::Shortcode { code } => format!(":{code}:"),
            Icon::Data { data } => data.trim().to_string(),
            Icon::Emoji { emoji } => format!("emoji:{}", emoji.trim()),
            // Always canonical, even for a `src` that was decoded from `$…$` sugar — the `$`
            // delimiters are input sugar only, never round-tripped back out.
            Icon::Math { src } => format!("math:{}", src.trim()),
            Icon::Text { text } => format!("text:{}", text.trim()),
            Icon::Svg { svg } => svg.trim().to_string(),
            Icon::Catalog { key } => key.as_str().to_string(),
            Icon::Themed { key } => key.as_str().to_string(),
        }
    }

    // #endregion 🏷️IconUnion

    #[derive(Debug)]
    pub enum BoardResolvedIcon {
        None,
        SvgThemed(String),
        SvgPlain(String),
        RasterRgba8 { rgba: Arc<[u8]>, w: u32, h: u32 },
    }

    struct RgbaImage {
        data: Arc<[u8]>,
        w: u32,
        h: u32,
    }

    fn decode_data_url_svg(s: &str) -> Option<String> {
        let t = s.trim();
        let lower = t.to_ascii_lowercase();
        if lower.starts_with("data:image/svg+xml;base64,") {
            let rest = t.split_once(',').map(|(_, b)| b.trim())?;
            let raw = base64_codec::base64_standard_decode(rest).ok()?;
            return String::from_utf8(raw).ok();
        }
        if lower.starts_with("data:image/svg+xml,") {
            let rest = t.split_once(',').map(|(_, b)| b.trim())?;
            return Some(percent_decode_utf8(rest));
        }
        None
    }

    fn percent_decode_utf8(input: &str) -> String {
        let bytes = input.as_bytes();
        let mut out = Vec::with_capacity(bytes.len());
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'%' && i + 2 < bytes.len() {
                if let (Some(h), Some(l)) = (hex_nibble(bytes[i + 1]), hex_nibble(bytes[i + 2])) {
                    out.push((h << 4) | l);
                    i += 3;
                    continue;
                }
            }
            out.push(bytes[i]);
            i += 1;
        }
        String::from_utf8_lossy(&out).into_owned()
    }

    fn hex_nibble(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        }
    }

    fn decode_raster_icon_bytes(t: &str) -> Option<RgbaImage> {
        let s = t.trim();
        let rest = s
            .strip_prefix("data:image/png;base64,")
            .or_else(|| s.strip_prefix("data:image/jpeg;base64,"))
            .or_else(|| s.strip_prefix("data:image/jpg;base64,"))
            .or_else(|| s.strip_prefix("data:image/webp;base64,"))
            .or_else(|| s.strip_prefix("data:image/gif;base64,"))?;
        let raw = base64_codec::base64_standard_decode(rest.trim()).ok()?;
        raster_icon_bytes_to_rgba(&raw)
    }

    /// 🖼️ Full pixel decode for icon *painting* — host/browser only. `IconPaintCache::get_or_build`
    /// (the only caller that ever reads `RgbaImage::data`) is unconditionally `None` on
    /// `wasm32-wasip2` (a WASI guest never paints pixels), so the real decode is never needed
    /// there. Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn raster_icon_bytes_to_rgba(raw: &[u8]) -> Option<RgbaImage> {
        let img = image::load_from_memory(raw).ok()?;
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        if w == 0 || h == 0 {
            return None;
        }
        Some(RgbaImage { data: Arc::from(rgba.into_raw().into_boxed_slice()), w, h })
    }

    /// 📐️ `wasm32-wasip2` arm: header-only dimension read (`semio-framework-intrinsic-size`),
    /// never a full pixel decode. `data` is deliberately empty — the only consumer that would
    /// read real pixels (`IconPaintCache::get_or_build`, icon painting) is itself unconditionally
    /// `None` on this target, so nothing ever reads it; the only live consumer of this arm's
    /// result is `preview_media_natural_size`'s dimension query, which reads `w`/`h` only. Ticket
    /// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    fn raster_icon_bytes_to_rgba(raw: &[u8]) -> Option<RgbaImage> {
        let (w, h) = super::intrinsic_size::raster_dimensions(raw).ok()?;
        if w == 0 || h == 0 {
            return None;
        }
        Some(RgbaImage { data: Arc::from([].as_slice()), w, h })
    }

    /// @emoji 🧮️ Compiles a semio math notation snippet ([`compiler::syntax::parse_formula`]) to SVG
    /// via the `compiler` module — replaces the former Typst-markup icon path. `None` on invalid
    /// notation syntax (graceful degradation, matching the old malformed-typst-markup behavior).
    fn resolve_math_src(src: &str) -> BoardResolvedIcon {
        let src = src.trim();
        if src.is_empty() {
            return BoardResolvedIcon::None;
        }
        let options = compiler::SnippetOptions { font_size_pt: ui_styling::metrics::math::ICON_FONT_SIZE_PT as f32, margin_pt: ui_styling::metrics::math::ICON_MARGIN_PT as f32 };
        match compiler::compile_snippet_to_svg(src, options) {
            Ok(snippet) => BoardResolvedIcon::SvgPlain(snippet.svg),
            Err(_) => BoardResolvedIcon::None,
        }
    }

    /// @emoji 😀️ Renders an arbitrary emoji string via `compiler::compile_emoji_to_svg` — infallible
    /// (the compiler's fonts are always embedded, no host-fetch failure mode remains).
    fn resolve_emoji_body(em: &str) -> BoardResolvedIcon {
        let em = em.trim();
        if em.is_empty() {
            return BoardResolvedIcon::None;
        }
        let options = compiler::SnippetOptions { font_size_pt: ui_styling::metrics::math::EMOJI_FONT_SIZE_PT as f32, margin_pt: ui_styling::metrics::math::EMOJI_MARGIN_PT as f32 };
        BoardResolvedIcon::SvgPlain(compiler::compile_emoji_to_svg(em, options).svg)
    }

    /// @emoji 📝️ Renders arbitrary text via `compiler::compile_text_to_svg` — infallible.
    fn resolve_text_body(text: &str) -> BoardResolvedIcon {
        let text = text.trim();
        if text.is_empty() {
            return BoardResolvedIcon::None;
        }
        let options = compiler::SnippetOptions { font_size_pt: ui_styling::metrics::math::TEXT_FONT_SIZE_PT as f32, margin_pt: ui_styling::metrics::math::TEXT_MARGIN_PT as f32 };
        BoardResolvedIcon::SvgPlain(compiler::compile_text_to_svg(text, options).svg)
    }

    fn resolve_icon_data(data: &str) -> BoardResolvedIcon {
        if let Some(svg) = decode_data_url_svg(data) {
            return BoardResolvedIcon::SvgPlain(svg);
        }
        if let Some(img) = decode_raster_icon_bytes(data) {
            return BoardResolvedIcon::RasterRgba8 { rgba: img.data, w: img.w, h: img.h };
        }
        BoardResolvedIcon::None
    }

    fn resolve_structured_icon(icon: &Icon, themed_lookup: ThemedSvgLookup) -> BoardResolvedIcon {
        match icon {
            Icon::Url { .. } => BoardResolvedIcon::None,
            Icon::Shortcode { code } => match icon_shortcodes::icon_shortcode_resolve(code) {
                Some(icon_shortcodes::ShortcodeResolved::Emoji(em)) => resolve_emoji_body(em),
                Some(icon_shortcodes::ShortcodeResolved::SvgPlain(svg)) => BoardResolvedIcon::SvgPlain(svg.to_string()),
                Some(icon_shortcodes::ShortcodeResolved::SvgThemed(svg)) => BoardResolvedIcon::SvgThemed(svg.to_string()),
                None => themed_lookup(code).map_or(BoardResolvedIcon::None, |svg| BoardResolvedIcon::SvgThemed(svg.to_string())),
            },
            Icon::Data { data } => resolve_icon_data(data),
            Icon::Emoji { emoji } => resolve_emoji_body(emoji),
            Icon::Math { src } => resolve_math_src(src),
            Icon::Text { text } => resolve_text_body(text),
            Icon::Svg { svg } => {
                if themed_lookup(svg).is_some() {
                    BoardResolvedIcon::SvgThemed(svg.clone())
                } else {
                    BoardResolvedIcon::SvgPlain(svg.clone())
                }
            }
            Icon::Catalog { key } => match icon_shortcodes::icon_shortcode_resolve(key.as_str()) {
                Some(icon_shortcodes::ShortcodeResolved::SvgPlain(svg)) => BoardResolvedIcon::SvgPlain(svg.to_string()),
                Some(icon_shortcodes::ShortcodeResolved::SvgThemed(svg)) => BoardResolvedIcon::SvgThemed(svg.to_string()),
                _ => themed_lookup(key.as_str()).map_or(BoardResolvedIcon::None, |svg| BoardResolvedIcon::SvgThemed(svg.to_string())),
            },
            Icon::Themed { key } => match icon_shortcodes::icon_shortcode_resolve(key.as_str()) {
                Some(icon_shortcodes::ShortcodeResolved::SvgThemed(svg)) => BoardResolvedIcon::SvgThemed(svg.to_string()),
                _ => themed_lookup(key.as_str()).map_or(BoardResolvedIcon::None, |svg| BoardResolvedIcon::SvgThemed(svg.to_string())),
            },
        }
    }

    /// @emoji 🔍️ Resolves an icon encoding to paintable content; `themed_lookup` marks SVG as themed when present.
    pub fn board_resolve_icon_kind(encoded: &str, themed_lookup: ThemedSvgLookup) -> BoardResolvedIcon {
        let Some(icon) = decode_icon(encoded) else {
            return BoardResolvedIcon::None;
        };
        resolve_structured_icon(&icon, themed_lookup)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn round_trip(s: &str) {
            let icon = decode_icon(s).expect("decode");
            let encoded = encode_icon(&icon);
            let again = decode_icon(&encoded).expect("re-decode");
            assert_eq!(icon, again, "round-trip failed for {s} -> {encoded}");
        }

        #[test]
        fn icon_codec_round_trips_all_kinds() {
            round_trip("url:https://example.com/icon.png");
            round_trip(":smile:");
            round_trip("data:image/png;base64,iVBORw0KGgo=");
            round_trip("emoji:☺️");
            round_trip("math:x^2");
            round_trip("text:Hi");
            round_trip(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10"/></svg>"#);
            round_trip("capsule_J");
        }

        #[test]
        fn icon_codec_bare_dollar_sugar_decodes_as_math() {
            // The `$…$` delimiters are input sugar only — stripped, not echoed into `src`.
            assert_eq!(decode_icon("$x^2$"), Some(Icon::Math { src: "x^2".to_string() }));
        }

        #[test]
        fn icon_codec_decodes_themed_stem() {
            assert!(matches!(decode_icon("capsule_J"), Some(Icon::Themed { .. })));
        }

        #[test]
        fn icon_codec_decodes_catalog_stem() {
            assert!(matches!(decode_icon("plus"), Some(Icon::Catalog { .. })));
        }

        #[test]
        fn icon_codec_rejects_unknown_catalogish_stem() {
            assert!(decode_icon("unknown_icon_stem_with_underscore").is_none());
        }

        #[test]
        fn icon_codec_resolves_emoji_shortcode_to_svg() {
            let r = board_resolve_icon_kind(":grinning:", |_| None);
            match r {
                BoardResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
                other => panic!("unexpected: {other:?}"),
            }
        }

        #[test]
        fn icon_codec_resolves_catalog_shortcode_to_svg() {
            let r = board_resolve_icon_kind(":plus:", |_| None);
            match r {
                BoardResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
                other => panic!("unexpected: {other:?}"),
            }
        }

        #[test]
        fn icon_codec_resolves_metabolism_shortcode_to_themed_svg() {
            let r = board_resolve_icon_kind(":capsule_J:", |_| None);
            match r {
                BoardResolvedIcon::SvgThemed(s) => assert!(s.contains("<svg")),
                other => panic!("unexpected: {other:?}"),
            }
        }

        #[test]
        fn icon_codec_resolves_text_to_svg() {
            let r = board_resolve_icon_kind("text:Hi", |_| None);
            match r {
                BoardResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
                other => panic!("unexpected: {other:?}"),
            }
        }

        #[test]
        fn icon_codec_url_returns_none_for_sync_resolver() {
            assert!(matches!(board_resolve_icon_kind("url:https://example.com/x.png", |_| None), BoardResolvedIcon::None));
        }
    }
    // #endregion icon_codec
}
pub use icon_codec::{board_resolve_icon_kind, decode_icon, encode_icon, BoardResolvedIcon, Icon, ThemedSvgLookup};
// #endregion 🔖️IconCodec

// #region 🔖️CanvasExtension
/// 🧩️ Extension hook for domain-specific canvas behavior (hit-test, paint, kinds).
pub trait CanvasExtension: Send + Sync {
    fn extension_id(&self) -> &str;
}

/// ⚙️ Generic infinite-canvas engine shell; domain logic lives in `E`.
pub struct CanvasEngine<E: CanvasExtension> {
    pub extension: E,
}

impl<E: CanvasExtension> CanvasEngine<E> {
    pub fn new(extension: E) -> Self {
        Self { extension }
    }
}
// #endregion 🔖️CanvasExtension

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::camera::{screen_to_world, world_to_screen, Camera, Viewport};
    use super::lod::{Lod, LodScale};
    use super::text;
    use super::theme;
    use super::{Point, Scene};

    #[test]
    fn scale_scene_for_device_pixel_ratio_scales_logical_scene() {
        let mut scene = Scene::new();
        text::append_label(&mut scene, "A", Point::new(10.0, 10.0), 12.0, theme::default_icon_fg(), theme::default_icon_bg());
        let logical = scene.path_count();
        let scaled = super::render::scale_scene_for_device_pixel_ratio(scene, 2.0);
        assert!(scaled.path_count() >= logical);
        let identity = super::render::scale_scene_for_device_pixel_ratio(Scene::new(), 1.0);
        assert_eq!(identity.path_count(), 0);
    }

    #[test]
    fn append_label_renders_glyphs() {
        let mut scene = Scene::new();
        text::append_label(&mut scene, "Zürich", Point::new(40.0, 40.0), 14.0, theme::default_icon_fg(), theme::default_icon_bg());
        assert!(!scene.path_count().eq(&0));
        let mut empty = Scene::new();
        text::append_label(&mut empty, "  ", Point::new(0.0, 0.0), 14.0, theme::default_icon_fg(), theme::default_icon_bg());
        assert!(empty.is_empty());
    }

    #[test]
    fn label_advance_excludes_outer_padding() {
        let px = 14.0;
        let (box_w, _) = text::label_extent("MATCH", px);
        let advance = text::label_advance("MATCH", px);
        assert!(box_w > advance);
        assert_eq!(advance, "MATCH".len() as f64 * px * ui_styling::metrics::label::CHAR_WIDTH_RATIO);
    }

    #[test]
    fn label_byte_world_x_is_narrower_than_char_width_estimate() {
        let line = "MATCH";
        let x0 = text::label_byte_world_x(line, 0, 0.0, 14.0);
        let x5 = text::label_byte_world_x(line, 5, 0.0, 14.0);
        let estimate = text::label_advance("MATCH", 14.0);
        assert!(x5 - x0 < estimate * 0.85);
    }

    #[test]
    fn label_extent_matches_append_label_box() {
        let (w, h) = text::label_extent("math.add", 12.0);
        assert!(w > 32.0);
        assert!(h >= 16.0);
        assert_eq!(text::label_extent("  ", 12.0), (0.0, 0.0));
    }

    #[test]
    fn camera_round_trip() {
        let camera = Camera { x: 10.0, y: -5.0, zoom: 2.0 };
        let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
        let world = Point::new(12.0, 3.0);
        let screen = world_to_screen(&camera, &viewport, world);
        let back = screen_to_world(&camera, &viewport, screen);
        assert!((back.x - world.x).abs() < 1e-9);
        assert!((back.y - world.y).abs() < 1e-9);
    }

    #[test]
    fn lod_scale_resolve() {
        const LODS: &[Lod] =
            &[Lod { id: "minimap", name: "Minimap", description: "min", max_zoom: 0.15 }, Lod { id: "overview", name: "Overview", description: "ov", max_zoom: 0.35 }, Lod { id: "micro", name: "Micro", description: "mi", max_zoom: f64::INFINITY }];
        let scale = LodScale { lods: LODS };
        assert_eq!(scale.resolve(0.1).id, "minimap");
        assert_eq!(scale.resolve(0.2).id, "overview");
        assert_eq!(scale.resolve(3.0).id, "micro");
        assert_eq!(scale.index_of("overview"), Some(1));
    }
}
// #endregion 🔖️Tests
