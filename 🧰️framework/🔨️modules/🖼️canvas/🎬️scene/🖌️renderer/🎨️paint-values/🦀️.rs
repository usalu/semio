//! 🖌️ Paint values of the scene: the colour, the fill rule, the blend mode, the paint and the raster image with their peniko conversions. See [`super`].

use super::*;

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
pub struct Color(pub(super) [f32; 4]);

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
impl FillRule {
    pub(super) fn to_peniko(self) -> peniko::Fill {
        match self {
            FillRule::NonZero => peniko::Fill::NonZero,
            FillRule::EvenOdd => peniko::Fill::EvenOdd,
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
impl BlendMode {
    pub(super) fn to_peniko(self) -> peniko::Mix {
        match self {
            BlendMode::Normal => peniko::Mix::Normal,
            BlendMode::Multiply => peniko::Mix::Multiply,
            BlendMode::Screen => peniko::Mix::Screen,
            BlendMode::Overlay => peniko::Mix::Overlay,
            BlendMode::Darken => peniko::Mix::Darken,
            BlendMode::Lighten => peniko::Mix::Lighten,
            BlendMode::ColorDodge => peniko::Mix::ColorDodge,
            BlendMode::ColorBurn => peniko::Mix::ColorBurn,
            BlendMode::HardLight => peniko::Mix::HardLight,
            BlendMode::SoftLight => peniko::Mix::SoftLight,
            BlendMode::Difference => peniko::Mix::Difference,
            BlendMode::Exclusion => peniko::Mix::Exclusion,
            BlendMode::Hue => peniko::Mix::Hue,
            BlendMode::Saturation => peniko::Mix::Saturation,
            BlendMode::Color => peniko::Mix::Color,
            BlendMode::Luminosity => peniko::Mix::Luminosity,
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
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) data: SharedArc<Vec<u8>>,
}

impl semio_framework_value::retirement::RetireOwned for RasterImage {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::{sequence,deferred,arc_custody::SharedControlledRetirement};sequence(vec![deferred(self.width),deferred(self.height),Box::new(SharedControlledRetirement::lease(self.data))])}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for,arc_custody::shared_retirement_birth_bytes};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.width),deferred_birth_bytes_for(&self.height),shared_retirement_birth_bytes::<Vec<u8>>()])}
    fn controlled_retirement_supported()->bool{true}
}

impl RasterImage {
    /// 🖍️ Builds an RGBA8 raster image for scene drawing.
    pub fn rgba8(width: u32, height: u32, data: SharedArc<Vec<u8>>) -> Self {
        Self { width, height, data }
    }
    pub(super) fn retirement_step(&mut self) -> bool {
        SharedArc::get_mut(&mut self.data).is_none_or(|data| data.pop().is_none())
    }
    pub(super) fn retirement_backing_bytes(&self) -> usize {
        if SharedArc::strong_count(&self.data) == 1 {
            self.data.capacity()
        } else {
            0
        }
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
    pub fn retirement_exclusive_backing_bytes(&self) -> usize {
        if SharedArc::strong_count(&self.data) == 1 {
            self.data.capacity()
        } else {
            0
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl RasterImage {
    /// ☁️ Host/browser-only: rebuilds a real `peniko::ImageData`, only ever needed at the
    /// `SceneCommand::replay_into` rasterization boundary.
    pub(crate) fn to_peniko(&self) -> peniko::ImageData {
        peniko::ImageData { data: peniko::Blob::new(self.data.clone()), format: peniko::ImageFormat::Rgba8, alpha_type: peniko::ImageAlphaType::Alpha, width: self.width, height: self.height }
    }
}
