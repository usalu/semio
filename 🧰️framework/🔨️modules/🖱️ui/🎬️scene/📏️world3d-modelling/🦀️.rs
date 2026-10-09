//! 📏️ Domain-neutral world-3d modelling primitives a B-Rep or mesh modelling app needs: an annotation
//! layer, scalar field colouring with a legend, a pick granularity filter, a section plane and
//! sub-element highlight tokens. The language-neutral contract is
//! `🧬️schema/📏️world3d-modelling/🔣️.json` plus `🧫️fixtures/📏️world3d-modelling/🔣️.json`; the
//! TypeScript twin is `🟦️.ts` beside this file, and both are pinned against those two files.
//!
//! Every type has two codecs. `serde` carries the structure for [`crate::World3dScene`]'s own JSON
//! shape; `ToValue`/`FromValue` is the lane codec and also enforces the semantic rules the schema
//! cannot express (unique ids, distinct dimension ends, non-zero directions and normals, ordered
//! ranges), so a payload that decodes through a lane is always valid.
//!
//! 🚫️async: E6 sync payload construction.

use protocol::value::{DslValue, FromValue, ToValue, ValueError, ValueRefusalKind};
use semio_framework_pack_json::{from_json_str, to_json_string, JsonMemberPolicy};
use serde::{Deserialize, Serialize};

use crate::World3dScene;

pub const WORLD3D_ANNOTATIONS_MAX: usize = 512;
pub const WORLD3D_SCALAR_VALUES_MAX: usize = 4_000_000;
pub const WORLD3D_LEGEND_TICKS_DEFAULT: u8 = 5;
pub const WORLD3D_ANGLE_RADIUS_PX_DEFAULT: f64 = 48.0;
pub const WORLD3D_SCALAR_NO_DATA_RGB: [u8; 3] = [0x80, 0x80, 0x80];

/// 📍️ A world-space point or direction.
pub type World3dVec3 = [f64; 3];

//#region 🔖️Codec
fn refuse(message: impl Into<String>) -> ValueError {
    ValueError::new(ValueRefusalKind::InvalidValue, message)
}

struct Fields(Vec<(String, DslValue)>);

impl Fields {
    fn read(value: DslValue, allowed: &[&str]) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        if let Some((key, _)) = entries.iter().find(|(key, _)| !allowed.contains(&key.as_str())) {
            return Err(refuse(format!("unknown field `{key}`")));
        }
        Ok(Self(entries))
    }

    fn take_value(&mut self, key: &str) -> Option<DslValue> {
        let position = self.0.iter().position(|(entry, _)| entry == key)?;
        Some(self.0.remove(position).1)
    }

    fn require<T: FromValue>(&mut self, key: &str) -> Result<T, ValueError> {
        let value = self.take_value(key).ok_or_else(|| refuse(format!("missing field `{key}`")))?;
        T::from_value(value).map_err(|error| error.under(key))
    }

    fn option<T: FromValue>(&mut self, key: &str) -> Result<Option<T>, ValueError> {
        match self.take_value(key) {
            None => Ok(None),
            Some(value) => T::from_value(value).map(Some).map_err(|error| error.under(key)),
        }
    }

    fn or<T: FromValue>(&mut self, key: &str, default: T) -> Result<T, ValueError> {
        Ok(self.option(key)?.unwrap_or(default))
    }
}

fn push(entries: &mut Vec<(String, DslValue)>, key: &str, value: &impl ToValue) {
    entries.push((key.to_string(), value.to_value()));
}

fn push_option(entries: &mut Vec<(String, DslValue)>, key: &str, value: &Option<impl ToValue>) {
    if let Some(value) = value {
        push(entries, key, value);
    }
}

macro_rules! string_enum {
    ($(#[$meta:meta])* $name:ident { $($variant:ident => $text:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub enum $name {
            $(#[serde(rename = $text)] $variant),+
        }

        impl $name {
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];

            pub fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $text),+
                }
            }

            pub fn parse(text: &str) -> Option<Self> {
                match text {
                    $($text => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }

        impl ToValue for $name {
            fn to_value(&self) -> DslValue {
                DslValue::String(self.as_str().to_string())
            }
        }

        impl FromValue for $name {
            fn from_value(value: DslValue) -> Result<Self, ValueError> {
                let text = String::from_value(value)?;
                Self::parse(&text).ok_or_else(|| refuse(format!("unknown {} `{text}`", stringify!($name))))
            }
        }
    };
}

fn vec3_finite(vector: &World3dVec3, label: &str) -> Result<(), ValueError> {
    if vector.iter().all(|component| component.is_finite()) {
        Ok(())
    } else {
        Err(refuse(format!("{label} must be finite")))
    }
}

fn vec3_nonzero(vector: &World3dVec3, label: &str) -> Result<(), ValueError> {
    vec3_finite(vector, label)?;
    if vector.iter().any(|component| *component != 0.0) {
        Ok(())
    } else {
        Err(refuse(format!("{label} must not be zero")))
    }
}
//#endregion 🔖️Codec

//#region 🔖️Text
/// 🌍️ Caller-supplied text in every supported language; the renderer shows the entry for the active locale.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct World3dText {
    pub en: String,
    pub de: String,
}

impl World3dText {
    pub fn new(en: impl Into<String>, de: impl Into<String>) -> Self {
        Self { en: en.into(), de: de.into() }
    }

    /// 🌍️ The entry for the active locale (`de`, `de-CH` → German; everything else → English, the first language).
    pub fn resolve(&self, locale: &str) -> &str {
        if locale.to_ascii_lowercase().split('-').next() == Some("de") {
            &self.de
        } else {
            &self.en
        }
    }

    pub fn validate(&self) -> Result<(), ValueError> {
        if self.en.trim().is_empty() || self.de.trim().is_empty() {
            return Err(refuse("text needs a non-blank en and de"));
        }
        Ok(())
    }
}

impl ToValue for World3dText {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push(&mut entries, "en", &self.en);
        push(&mut entries, "de", &self.de);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dText {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["en", "de"])?;
        let text = Self { en: fields.require("en")?, de: fields.require("de")? };
        text.validate()?;
        Ok(text)
    }
}

string_enum! {
    /// 🎨️ A theme colour token: `neutral` is the foreground, the others the palette token of the same name.
    World3dTone { Neutral => "neutral", Primary => "primary", Secondary => "secondary", Tertiary => "tertiary", Success => "success", Warning => "warning", Danger => "danger", Info => "info" }
}

impl Default for World3dTone {
    fn default() -> Self {
        Self::Neutral
    }
}
//#endregion 🔖️Text

//#region 🔖️Annotations
string_enum! {
    /// 📍️ The mark a point annotation is drawn with.
    World3dMarkerShape { Dot => "dot", Cross => "cross", Ring => "ring" }
}

impl Default for World3dMarkerShape {
    fn default() -> Self {
        Self::Dot
    }
}

/// 📏️ A linear dimension between two world points, drawn displaced by `offset` with arrowheads and text of screen-constant size.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dDimension {
    pub id: String,
    pub from: World3dVec3,
    pub to: World3dVec3,
    pub offset: World3dVec3,
    pub text: World3dText,
    #[serde(default)]
    pub tone: World3dTone,
}

/// 📐️ An angle at `vertex` between two non-zero directions, drawn as an arc of `radius_px` screen pixels.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dAngle {
    pub id: String,
    pub vertex: World3dVec3,
    pub direction_a: World3dVec3,
    pub direction_b: World3dVec3,
    #[serde(default = "angle_radius_default")]
    pub radius_px: f64,
    pub text: World3dText,
    #[serde(default)]
    pub tone: World3dTone,
}

fn angle_radius_default() -> f64 {
    WORLD3D_ANGLE_RADIUS_PX_DEFAULT
}

/// 📍️ A point marker; its text is the accessible alternative and the tooltip.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dMarker {
    pub id: String,
    pub position: World3dVec3,
    #[serde(default)]
    pub shape: World3dMarkerShape,
    pub text: World3dText,
    #[serde(default)]
    pub tone: World3dTone,
}

/// 🏷️ A text label connected to a world anchor by a leader line; `label_offset_px` displaces the label on screen.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dLeader {
    pub id: String,
    pub anchor: World3dVec3,
    pub label_offset_px: [f64; 2],
    pub text: World3dText,
    #[serde(default)]
    pub tone: World3dTone,
}

/// 📏️ One annotation of the layer, discriminated on the wire by `kind`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum World3dAnnotation {
    Dimension(World3dDimension),
    Angle(World3dAngle),
    Marker(World3dMarker),
    Leader(World3dLeader),
}

impl World3dAnnotation {
    pub fn dimension(id: impl Into<String>, from: World3dVec3, to: World3dVec3, offset: World3dVec3, text: World3dText) -> Self {
        Self::Dimension(World3dDimension { id: id.into(), from, to, offset, text, tone: World3dTone::Neutral })
    }

    pub fn angle(id: impl Into<String>, vertex: World3dVec3, direction_a: World3dVec3, direction_b: World3dVec3, text: World3dText) -> Self {
        Self::Angle(World3dAngle { id: id.into(), vertex, direction_a, direction_b, radius_px: WORLD3D_ANGLE_RADIUS_PX_DEFAULT, text, tone: World3dTone::Neutral })
    }

    pub fn marker(id: impl Into<String>, position: World3dVec3, text: World3dText) -> Self {
        Self::Marker(World3dMarker { id: id.into(), position, shape: World3dMarkerShape::Dot, text, tone: World3dTone::Neutral })
    }

    pub fn leader(id: impl Into<String>, anchor: World3dVec3, label_offset_px: [f64; 2], text: World3dText) -> Self {
        Self::Leader(World3dLeader { id: id.into(), anchor, label_offset_px, text, tone: World3dTone::Neutral })
    }

    pub fn with_tone(mut self, tone: World3dTone) -> Self {
        match &mut self {
            Self::Dimension(item) => item.tone = tone,
            Self::Angle(item) => item.tone = tone,
            Self::Marker(item) => item.tone = tone,
            Self::Leader(item) => item.tone = tone,
        }
        self
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Dimension(item) => &item.id,
            Self::Angle(item) => &item.id,
            Self::Marker(item) => &item.id,
            Self::Leader(item) => &item.id,
        }
    }

    pub fn text(&self) -> &World3dText {
        match self {
            Self::Dimension(item) => &item.text,
            Self::Angle(item) => &item.text,
            Self::Marker(item) => &item.text,
            Self::Leader(item) => &item.text,
        }
    }

    pub fn validate(&self) -> Result<(), ValueError> {
        if self.id().is_empty() {
            return Err(refuse("annotation id must not be empty"));
        }
        self.text().validate()?;
        match self {
            Self::Dimension(item) => {
                for (vector, label) in [(&item.from, "from"), (&item.to, "to"), (&item.offset, "offset")] {
                    vec3_finite(vector, label)?;
                }
                if item.from == item.to {
                    return Err(refuse("dimension from and to must differ"));
                }
            }
            Self::Angle(item) => {
                vec3_finite(&item.vertex, "vertex")?;
                vec3_nonzero(&item.direction_a, "directionA")?;
                vec3_nonzero(&item.direction_b, "directionB")?;
                if !item.radius_px.is_finite() || !(16.0..=256.0).contains(&item.radius_px) {
                    return Err(refuse("radiusPx must be within 16..256"));
                }
            }
            Self::Marker(item) => vec3_finite(&item.position, "position")?,
            Self::Leader(item) => {
                vec3_finite(&item.anchor, "anchor")?;
                if !item.label_offset_px.iter().all(|component| component.is_finite()) {
                    return Err(refuse("labelOffsetPx must be finite"));
                }
            }
        }
        Ok(())
    }
}

impl ToValue for World3dAnnotation {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        match self {
            Self::Dimension(item) => {
                push(&mut entries, "kind", &"dimension");
                push(&mut entries, "id", &item.id);
                push(&mut entries, "from", &item.from);
                push(&mut entries, "to", &item.to);
                push(&mut entries, "offset", &item.offset);
                push(&mut entries, "text", &item.text);
                push(&mut entries, "tone", &item.tone);
            }
            Self::Angle(item) => {
                push(&mut entries, "kind", &"angle");
                push(&mut entries, "id", &item.id);
                push(&mut entries, "vertex", &item.vertex);
                push(&mut entries, "directionA", &item.direction_a);
                push(&mut entries, "directionB", &item.direction_b);
                push(&mut entries, "radiusPx", &item.radius_px);
                push(&mut entries, "text", &item.text);
                push(&mut entries, "tone", &item.tone);
            }
            Self::Marker(item) => {
                push(&mut entries, "kind", &"marker");
                push(&mut entries, "id", &item.id);
                push(&mut entries, "position", &item.position);
                push(&mut entries, "shape", &item.shape);
                push(&mut entries, "text", &item.text);
                push(&mut entries, "tone", &item.tone);
            }
            Self::Leader(item) => {
                push(&mut entries, "kind", &"leader");
                push(&mut entries, "id", &item.id);
                push(&mut entries, "anchor", &item.anchor);
                push(&mut entries, "labelOffsetPx", &item.label_offset_px);
                push(&mut entries, "text", &item.text);
                push(&mut entries, "tone", &item.tone);
            }
        }
        DslValue::Object(entries)
    }
}

impl FromValue for World3dAnnotation {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let kind = value.get("kind").and_then(DslValue::as_str).map(str::to_string).ok_or_else(|| refuse("annotation needs a kind"))?;
        let annotation = match kind.as_str() {
            "dimension" => {
                let mut fields = Fields::read(value, &["kind", "id", "from", "to", "offset", "text", "tone"])?;
                Self::Dimension(World3dDimension { id: fields.require("id")?, from: fields.require("from")?, to: fields.require("to")?, offset: fields.require("offset")?, text: fields.require("text")?, tone: fields.or("tone", World3dTone::Neutral)? })
            }
            "angle" => {
                let mut fields = Fields::read(value, &["kind", "id", "vertex", "directionA", "directionB", "radiusPx", "text", "tone"])?;
                Self::Angle(World3dAngle {
                    id: fields.require("id")?,
                    vertex: fields.require("vertex")?,
                    direction_a: fields.require("directionA")?,
                    direction_b: fields.require("directionB")?,
                    radius_px: fields.or("radiusPx", WORLD3D_ANGLE_RADIUS_PX_DEFAULT)?,
                    text: fields.require("text")?,
                    tone: fields.or("tone", World3dTone::Neutral)?,
                })
            }
            "marker" => {
                let mut fields = Fields::read(value, &["kind", "id", "position", "shape", "text", "tone"])?;
                Self::Marker(World3dMarker { id: fields.require("id")?, position: fields.require("position")?, shape: fields.or("shape", World3dMarkerShape::Dot)?, text: fields.require("text")?, tone: fields.or("tone", World3dTone::Neutral)? })
            }
            "leader" => {
                let mut fields = Fields::read(value, &["kind", "id", "anchor", "labelOffsetPx", "text", "tone"])?;
                Self::Leader(World3dLeader { id: fields.require("id")?, anchor: fields.require("anchor")?, label_offset_px: fields.require("labelOffsetPx")?, text: fields.require("text")?, tone: fields.or("tone", World3dTone::Neutral)? })
            }
            other => return Err(refuse(format!("unknown annotation kind `{other}`"))),
        };
        annotation.validate()?;
        Ok(annotation)
    }
}

/// 📏️ The annotation layer; every item is listed for assistive technology through its text.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dAnnotationLayer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<World3dText>,
    pub items: Vec<World3dAnnotation>,
}

impl World3dAnnotationLayer {
    pub fn new(items: Vec<World3dAnnotation>) -> Self {
        Self { title: None, items }
    }

    pub fn titled(mut self, title: World3dText) -> Self {
        self.title = Some(title);
        self
    }

    pub fn validate(&self) -> Result<(), ValueError> {
        if self.items.len() > WORLD3D_ANNOTATIONS_MAX {
            return Err(refuse(format!("at most {WORLD3D_ANNOTATIONS_MAX} annotations")));
        }
        if let Some(title) = &self.title {
            title.validate().map_err(|error| error.under("title"))?;
        }
        let mut identities = std::collections::HashSet::new();
        for (index, item) in self.items.iter().enumerate() {
            item.validate().map_err(|error| error.under(format!("items[{index}]")))?;
            if !identities.insert(item.id()) {
                return Err(refuse(format!("duplicate annotation id `{}`", item.id())));
            }
        }
        Ok(())
    }
}

impl ToValue for World3dAnnotationLayer {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push_option(&mut entries, "title", &self.title);
        push(&mut entries, "items", &self.items);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dAnnotationLayer {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["title", "items"])?;
        let layer = Self { title: fields.option("title")?, items: fields.require("items")? };
        layer.validate()?;
        Ok(layer)
    }
}
//#endregion 🔖️Annotations

//#region 🔖️ScalarField
string_enum! {
    /// 🌡️ Which mesh element each scalar value belongs to.
    World3dScalarDomain { Vertex => "vertex", Face => "face" }
}

string_enum! {
    /// 🎨️ A named colour ramp.
    World3dColorRamp { Viridis => "viridis", Inferno => "inferno", Coolwarm => "coolwarm", Grayscale => "grayscale" }
}

impl World3dColorRamp {
    /// 🎨️ Equally spaced sRGB stops; colours interpolate linearly per channel.
    pub fn stops(self) -> &'static [[u8; 3]] {
        match self {
            Self::Viridis => &[[0x44, 0x01, 0x54], [0x3b, 0x52, 0x8b], [0x21, 0x91, 0x8c], [0x5e, 0xc9, 0x62], [0xfd, 0xe7, 0x25]],
            Self::Inferno => &[[0x00, 0x00, 0x04], [0x57, 0x10, 0x6e], [0xbc, 0x37, 0x54], [0xf9, 0x8e, 0x09], [0xfc, 0xff, 0xa4]],
            Self::Coolwarm => &[[0x3b, 0x4c, 0xc0], [0x8d, 0xb0, 0xfe], [0xdd, 0xdd, 0xdd], [0xf4, 0x98, 0x7a], [0xb4, 0x04, 0x26]],
            Self::Grayscale => &[[0x00, 0x00, 0x00], [0xff, 0xff, 0xff]],
        }
    }

    /// 🎨️ The ramp colour at `t` (clamped to 0..1).
    pub fn sample(self, t: f64) -> [u8; 3] {
        let stops = self.stops();
        let segments = stops.len() - 1;
        let scaled = if t.is_nan() { 0.0 } else { t.clamp(0.0, 1.0) } * segments as f64;
        let index = (scaled.floor() as usize).min(segments - 1);
        let fraction = scaled - index as f64;
        let (from, to) = (stops[index], stops[index + 1]);
        let channel = |component: usize| {
            let (a, b) = (f64::from(from[component]) / 255.0, f64::from(to[component]) / 255.0);
            ((a + (b - a) * fraction) * 255.0 + 0.5).floor().clamp(0.0, 255.0) as u8
        };
        [channel(0), channel(1), channel(2)]
    }
}

/// 🌡️ The value interval the ramp spans; values outside clamp to its ends.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct World3dScalarRange {
    pub min: f64,
    pub max: f64,
}

impl ToValue for World3dScalarRange {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push(&mut entries, "min", &self.min);
        push(&mut entries, "max", &self.max);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dScalarRange {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["min", "max"])?;
        let range = Self { min: fields.require("min")?, max: fields.require("max")? };
        if !range.min.is_finite() || !range.max.is_finite() || range.min >= range.max {
            return Err(refuse("range min must be finite and below max"));
        }
        Ok(range)
    }
}

/// 🏷️ The on-screen legend of a scalar field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dScalarLegend {
    pub title: World3dText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default = "legend_ticks_default")]
    pub ticks: u8,
}

fn legend_ticks_default() -> u8 {
    WORLD3D_LEGEND_TICKS_DEFAULT
}

impl ToValue for World3dScalarLegend {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push(&mut entries, "title", &self.title);
        push_option(&mut entries, "unit", &self.unit);
        push(&mut entries, "ticks", &self.ticks);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dScalarLegend {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["title", "unit", "ticks"])?;
        let legend = Self { title: fields.require("title")?, unit: fields.option("unit")?, ticks: fields.or("ticks", WORLD3D_LEGEND_TICKS_DEFAULT)? };
        if !(2..=9).contains(&legend.ticks) {
            return Err(refuse("ticks must be within 2..9"));
        }
        if legend.unit.as_deref() == Some("") {
            return Err(refuse("unit must not be empty"));
        }
        Ok(legend)
    }
}

/// 🌡️ One legend tick: a value and the ramp colour it maps to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World3dLegendTick {
    pub value: f64,
    pub rgb: [u8; 3],
}

/// 🌡️ A scalar analysis field painted as a heatmap on every instance of `mesh_id`: one value per vertex, or one per triangle. `None` is no data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dScalarField {
    pub mesh_id: String,
    pub domain: World3dScalarDomain,
    pub values: Vec<Option<f64>>,
    pub ramp: World3dColorRamp,
    pub range: World3dScalarRange,
    pub legend: World3dScalarLegend,
}

semio_framework_value::artifact_retire_leaf!(World3dScalarDomain,World3dColorRamp,World3dScalarRange);
semio_framework_value::artifact_retire_struct!(World3dText{en,de});
semio_framework_value::artifact_retire_struct!(World3dScalarLegend{title,unit,ticks});
semio_framework_value::artifact_retire_struct!(World3dScalarField{mesh_id,domain,values,ramp,range,legend});

impl World3dScalarField {
    pub fn new(mesh_id: impl Into<String>, domain: World3dScalarDomain, values: Vec<Option<f64>>, ramp: World3dColorRamp, range: World3dScalarRange, legend_title: World3dText) -> Self {
        Self { mesh_id: mesh_id.into(), domain, values, ramp, range, legend: World3dScalarLegend { title: legend_title, unit: None, ticks: WORLD3D_LEGEND_TICKS_DEFAULT } }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.legend.unit = Some(unit.into());
        self
    }

    pub fn with_ticks(mut self, ticks: u8) -> Self {
        self.legend.ticks = ticks;
        self
    }

    pub fn validate(&self) -> Result<(), ValueError> {
        if self.mesh_id.is_empty() {
            return Err(refuse("meshId must not be empty"));
        }
        if self.values.is_empty() || self.values.len() > WORLD3D_SCALAR_VALUES_MAX {
            return Err(refuse(format!("values must hold 1..{WORLD3D_SCALAR_VALUES_MAX} entries")));
        }
        if self.values.iter().flatten().any(|value| !value.is_finite()) {
            return Err(refuse("values must be finite or null"));
        }
        if !self.range.min.is_finite() || !self.range.max.is_finite() || self.range.min >= self.range.max {
            return Err(refuse("range min must be finite and below max"));
        }
        self.legend.title.validate().map_err(|error| error.under("legend.title"))?;
        if !(2..=9).contains(&self.legend.ticks) {
            return Err(refuse("ticks must be within 2..9"));
        }
        if self.legend.unit.as_deref() == Some("") {
            return Err(refuse("unit must not be empty"));
        }
        Ok(())
    }

    fn t(&self, value: f64) -> f64 {
        (value - self.range.min) / (self.range.max - self.range.min)
    }

    /// 🌡️ The heatmap colour of one value; `None` is the no-data grey.
    pub fn color_of(&self, value: Option<f64>) -> [u8; 3] {
        value.map_or(WORLD3D_SCALAR_NO_DATA_RGB, |value| self.ramp.sample(self.t(value)))
    }

    /// 🌡️ Every value of the field as packed `r, g, b` bytes, ready for a vertex or triangle colour attribute.
    pub fn color_bytes(&self) -> Vec<u8> {
        self.values.iter().flat_map(|value| self.color_of(*value)).collect()
    }

    /// 🏷️ `legend.ticks` evenly spaced values from range min to max, each with its ramp colour.
    pub fn legend_ticks(&self) -> Vec<World3dLegendTick> {
        let count = usize::from(self.legend.ticks.max(2));
        (0..count)
            .map(|index| {
                let value = self.range.min + (index as f64 * (self.range.max - self.range.min)) / (count - 1) as f64;
                World3dLegendTick { value, rgb: self.ramp.sample(self.t(value)) }
            })
            .collect()
    }
}

impl ToValue for World3dScalarField {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push(&mut entries, "meshId", &self.mesh_id);
        push(&mut entries, "domain", &self.domain);
        push(&mut entries, "values", &self.values);
        push(&mut entries, "ramp", &self.ramp);
        push(&mut entries, "range", &self.range);
        push(&mut entries, "legend", &self.legend);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dScalarField {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["meshId", "domain", "values", "ramp", "range", "legend"])?;
        let field = Self { mesh_id: fields.require("meshId")?, domain: fields.require("domain")?, values: fields.require("values")?, ramp: fields.require("ramp")?, range: fields.require("range")?, legend: fields.require("legend")? };
        field.validate()?;
        Ok(field)
    }
}
//#endregion 🔖️ScalarField

//#region 🔖️Interaction
string_enum! {
    /// 🎯️ The only granularity hover and selection reach.
    World3dPickGranularity { Shape => "shape", Face => "face", Edge => "edge", Vertex => "vertex" }
}

/// 🎯️ The sub-element kinds hover and selection may reach.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct World3dPickTargets {
    pub mesh: bool,
    pub face: bool,
    pub edge: bool,
    pub vertex: bool,
}

impl World3dPickGranularity {
    /// 🎯️ The targets a filter admits; no filter admits all of them.
    pub fn targets(filter: Option<Self>) -> World3dPickTargets {
        match filter {
            None => World3dPickTargets { mesh: true, face: true, edge: true, vertex: true },
            Some(filter) => World3dPickTargets { mesh: filter == Self::Shape, face: filter == Self::Face, edge: filter == Self::Edge, vertex: filter == Self::Vertex },
        }
    }
}

/// ✂️ How a section plane closes the cut.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct World3dSectionCap {
    #[serde(default)]
    pub tone: World3dTone,
}

/// ✂️ A section plane through `origin`; geometry on the side `normal` points to is removed. `cap` closes the cut with a solid face.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct World3dSection {
    pub origin: World3dVec3,
    pub normal: World3dVec3,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cap: Option<World3dSectionCap>,
}

impl World3dSection {
    pub fn new(origin: World3dVec3, normal: World3dVec3) -> Self {
        Self { origin, normal, cap: None }
    }

    pub fn capped(mut self, tone: World3dTone) -> Self {
        self.cap = Some(World3dSectionCap { tone });
        self
    }

    pub fn validate(&self) -> Result<(), ValueError> {
        vec3_finite(&self.origin, "origin")?;
        vec3_nonzero(&self.normal, "normal")
    }

    /// ✂️ The plane `[nx, ny, nz, constant]` in three.js clipping convention (negative signed distance is clipped), removing the side `normal` points to.
    pub fn clip_plane(&self) -> [f64; 4] {
        let length = self.normal.iter().map(|component| component * component).sum::<f64>().sqrt();
        let unit = [self.normal[0] / length, self.normal[1] / length, self.normal[2] / length];
        let constant = unit[0] * self.origin[0] + unit[1] * self.origin[1] + unit[2] * self.origin[2];
        [-unit[0], -unit[1], -unit[2], constant]
    }
}

impl ToValue for World3dSectionCap {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push(&mut entries, "tone", &self.tone);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dSectionCap {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["tone"])?;
        Ok(Self { tone: fields.or("tone", World3dTone::Neutral)? })
    }
}

impl ToValue for World3dSection {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push(&mut entries, "origin", &self.origin);
        push(&mut entries, "normal", &self.normal);
        push_option(&mut entries, "cap", &self.cap);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dSection {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["origin", "normal", "cap"])?;
        let section = Self { origin: fields.require("origin")?, normal: fields.require("normal")?, cap: fields.option("cap")? };
        section.validate()?;
        Ok(section)
    }
}

/// 🖍️ Hover and selection tokens of one sub-element granularity; an absent token keeps the theme default (secondary for hover, primary for selection).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dSubElementStyle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hover: Option<World3dTone>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected: Option<World3dTone>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width_px: Option<f64>,
}

impl World3dSubElementStyle {
    pub fn validate(&self) -> Result<(), ValueError> {
        match self.width_px {
            Some(width) if !width.is_finite() || !(1.0..=8.0).contains(&width) => Err(refuse("widthPx must be within 1..8")),
            _ => Ok(()),
        }
    }
}

impl ToValue for World3dSubElementStyle {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push_option(&mut entries, "hover", &self.hover);
        push_option(&mut entries, "selected", &self.selected);
        push_option(&mut entries, "widthPx", &self.width_px);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dSubElementStyle {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["hover", "selected", "widthPx"])?;
        let style = Self { hover: fields.option("hover")?, selected: fields.option("selected")?, width_px: fields.option("widthPx")? };
        style.validate()?;
        Ok(style)
    }
}

/// 🖍️ Sub-element highlight tokens per granularity, resolved through the theme so every theme and dark mode customises them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct World3dHighlight {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face: Option<World3dSubElementStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edge: Option<World3dSubElementStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertex: Option<World3dSubElementStyle>,
}

impl ToValue for World3dHighlight {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push_option(&mut entries, "face", &self.face);
        push_option(&mut entries, "edge", &self.edge);
        push_option(&mut entries, "vertex", &self.vertex);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dHighlight {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["face", "edge", "vertex"])?;
        Ok(Self { face: fields.option("face")?, edge: fields.option("edge")?, vertex: fields.option("vertex")? })
    }
}

/// ⚙️ The scene options of a modelling viewport: pick granularity filter, section plane and highlight tokens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dModellingOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pick_filter: Option<World3dPickGranularity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<World3dSection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight: Option<World3dHighlight>,
}

impl World3dModellingOptions {
    pub fn validate(&self) -> Result<(), ValueError> {
        if let Some(section) = &self.section {
            section.validate().map_err(|error| error.under("section"))?;
        }
        Ok(())
    }
}

impl ToValue for World3dModellingOptions {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push_option(&mut entries, "pickFilter", &self.pick_filter);
        push_option(&mut entries, "section", &self.section);
        push_option(&mut entries, "highlight", &self.highlight);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dModellingOptions {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["pickFilter", "section", "highlight"])?;
        let options = Self { pick_filter: fields.option("pickFilter")?, section: fields.option("section")?, highlight: fields.option("highlight")? };
        options.validate()?;
        Ok(options)
    }
}
//#endregion 🔖️Interaction

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

    fn tone(&self) -> World3dTone {
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

/// 🏷️ The fixed strings of the modelling layer in one language.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct World3dModellingStrings {
    pub annotations: &'static str,
    pub dimension: &'static str,
    pub angle: &'static str,
    pub marker: &'static str,
    pub leader: &'static str,
    pub legend: &'static str,
    pub scale: &'static str,
    pub no_data: &'static str,
    pub mismatch: &'static str,
}

/// 🏷️ The fixed strings for a locale (German for `de*`, English otherwise); the React twin is `world3dModellingStrings`.
pub fn world3d_modelling_strings(locale: &str) -> World3dModellingStrings {
    if is_german(locale) {
        World3dModellingStrings { annotations: "Anmerkungen", dimension: "Bemaßung", angle: "Winkel", marker: "Markierung", leader: "Beschriftung", legend: "Legende", scale: "Skalenwerte", no_data: "Keine Daten", mismatch: "Das Skalarfeld passt nicht zum Netz" }
    } else {
        World3dModellingStrings { annotations: "Annotations", dimension: "Dimension", angle: "Angle", marker: "Marker", leader: "Label", legend: "Legend", scale: "Scale values", no_data: "No data", mismatch: "The scalar field does not match the mesh" }
    }
}

fn is_german(locale: &str) -> bool {
    locale.to_ascii_lowercase().split(['-', '_']).next() == Some("de")
}

/// 🧮️ A legend value with at most four significant digits, grouped and separated for the `en` or `de` language family — the same text as `Intl.NumberFormat(locale, { maximumSignificantDigits: 4 })`,
/// which pins it through the `legendValues` fixture. Rounding is half away from zero on the shortest decimal form of the double, as ICU does.
pub fn format_world3d_legend_value(value: f64, locale: &str) -> String {
    if !value.is_finite() {
        return value.to_string();
    }
    let (decimal, group) = if is_german(locale) { (',', '.') } else { ('.', ',') };
    if value == 0.0 {
        return "0".to_string();
    }
    let scientific = format!("{:e}", value.abs());
    let (mantissa, exponent) = scientific.split_once('e').unwrap_or((scientific.as_str(), "0"));
    let mut digits: Vec<u8> = mantissa.bytes().filter(u8::is_ascii_digit).map(|digit| digit - b'0').collect();
    let mut exponent: i32 = exponent.parse().unwrap_or(0);
    if digits.len() > 4 {
        let round_up = digits[4] >= 5;
        digits.truncate(4);
        if round_up {
            let mut index = 4;
            loop {
                if index == 0 {
                    digits.insert(0, 1);
                    digits.truncate(4);
                    exponent += 1;
                    break;
                }
                index -= 1;
                if digits[index] == 9 {
                    digits[index] = 0;
                } else {
                    digits[index] += 1;
                    break;
                }
            }
        }
    }
    while digits.len() > 1 && digits.last() == Some(&0) {
        digits.pop();
    }
    let text: String = digits.iter().map(|digit| char::from(b'0' + digit)).collect();
    let integer_digits = exponent + 1;
    let (integer, fraction) = if integer_digits <= 0 {
        ("0".to_string(), format!("{}{text}", "0".repeat(integer_digits.unsigned_abs() as usize)))
    } else if integer_digits as usize >= text.len() {
        (format!("{text}{}", "0".repeat(integer_digits as usize - text.len())), String::new())
    } else {
        (text[..integer_digits as usize].to_string(), text[integer_digits as usize..].to_string())
    };
    let mut grouped = String::with_capacity(integer.len() + integer.len() / 3);
    for (index, digit) in integer.chars().enumerate() {
        if index > 0 && (integer.len() - index) % 3 == 0 {
            grouped.push(group);
        }
        grouped.push(digit);
    }
    let sign = if value < 0.0 { "-" } else { "" };
    if fraction.is_empty() { format!("{sign}{grouped}") } else { format!("{sign}{grouped}{decimal}{fraction}") }
}

/// 🌡️ One legend tick line: its text and the ramp colour it names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct World3dLegendLine {
    pub text: String,
    pub rgb: [u8; 3],
}

/// 🌡️ The text of a scalar-field legend in one language: the title with its unit, one line per tick bottom to top, and the no-data caption when a value is missing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct World3dLegendLines {
    pub title: String,
    pub ticks: Vec<World3dLegendLine>,
    pub no_data: Option<&'static str>,
}

impl World3dScalarField {
    /// 🌡️ The legend text for a locale; the React twin is `world3dLegendLines`, pinned against the same `legendLines` fixture.
    pub fn legend_lines(&self, locale: &str) -> World3dLegendLines {
        let title = self.legend.title.resolve(locale);
        let unit = self.legend.unit.as_deref();
        World3dLegendLines {
            title: unit.map_or_else(|| title.to_string(), |unit| format!("{title} ({unit})")),
            ticks: self.legend_ticks().into_iter().map(|tick| World3dLegendLine { text: format!("{}{}", format_world3d_legend_value(tick.value, locale), unit.map_or_else(String::new, |unit| format!(" {unit}"))), rgb: tick.rgb }).collect(),
            no_data: self.values.iter().any(Option::is_none).then(|| world3d_modelling_strings(locale).no_data),
        }
    }
}

impl World3dSection {
    /// ✂️ The unit normal of the removed side.
    pub fn unit_normal(&self) -> World3dVec3 {
        let length = self.normal.iter().map(|component| component * component).sum::<f64>().sqrt();
        [self.normal[0] / length, self.normal[1] / length, self.normal[2] / length]
    }
}

/// 🖍️ The theme defaults of sub-element hover and selection that a highlight token overrides.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World3dSubElementDefaults<T> {
    pub select: T,
    pub hover: T,
    pub edge_hover: T,
    pub edge_width: f64,
    pub vertex_mark_px: f64,
}

/// 🖍️ The one paint table of sub-element hover and selection per granularity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World3dSubElementPaint<T> {
    pub face_select: T,
    pub face_hover: T,
    pub edge_select: T,
    pub edge_hover: T,
    pub vertex_select: T,
    pub vertex_hover: T,
    pub edge_width: f64,
    pub vertex_mark_px: f64,
}

/// 🖍️ Resolves the highlight tokens through `tone` over the theme defaults: a token overrides exactly its granularity, an absent token keeps the default (secondary hover, primary selection in the engine theme);
/// the React twin is `world3dSubElementPaint` and both are pinned against the `subElementPaint` fixture.
pub fn world3d_sub_element_paint<T: Copy>(highlight: Option<&World3dHighlight>, tone: impl Fn(World3dTone) -> T, defaults: World3dSubElementDefaults<T>) -> World3dSubElementPaint<T> {
    let style = |pick: fn(&World3dHighlight) -> Option<World3dSubElementStyle>| highlight.and_then(pick);
    let (face, edge, vertex) = (style(|highlight| highlight.face), style(|highlight| highlight.edge), style(|highlight| highlight.vertex));
    World3dSubElementPaint {
        face_select: face.and_then(|style| style.selected).map_or(defaults.select, &tone),
        face_hover: face.and_then(|style| style.hover).map_or(defaults.hover, &tone),
        edge_select: edge.and_then(|style| style.selected).map_or(defaults.select, &tone),
        edge_hover: edge.and_then(|style| style.hover).map_or(defaults.edge_hover, &tone),
        vertex_select: vertex.and_then(|style| style.selected).map_or(defaults.select, &tone),
        vertex_hover: vertex.and_then(|style| style.hover).map_or(defaults.hover, &tone),
        edge_width: edge.and_then(|style| style.width_px).unwrap_or(defaults.edge_width),
        vertex_mark_px: vertex.and_then(|style| style.width_px).unwrap_or(defaults.vertex_mark_px),
    }
}
//#endregion 🔖️Overlay

//#region 🔖️Lanes
/// 🚚️ Encodes one modelling payload as its lane text.
pub fn world3d_modelling_lane_text<T: ToValue>(value: &T) -> String {
    to_json_string(value)
}

/// 🚚️ Decodes one lane text, refusing anything the schema or its semantic rules refuse.
pub fn world3d_modelling_from_lane_text<T: FromValue>(text: &str) -> Result<T, ValueError> {
    from_json_str(text, JsonMemberPolicy::Reject)
}

impl World3dScene {
    /// 📏️ Sets the annotation layer lane; refuses an invalid layer.
    pub fn with_annotations(mut self, layer: World3dAnnotationLayer) -> Result<Self, ValueError> {
        layer.validate()?;
        self.annotations = Some(layer);
        Ok(self)
    }

    /// 🌡️ Sets the scalar field lane; refuses an invalid field.
    pub fn with_scalar_field(mut self, field: World3dScalarField) -> Result<Self, ValueError> {
        field.validate()?;
        self.scalar_field = Some(field);
        Ok(self)
    }

    /// ⚙️ Replaces the modelling options lane; refuses invalid options.
    pub fn with_modelling_options(mut self, options: World3dModellingOptions) -> Result<Self, ValueError> {
        options.validate()?;
        self.modelling_options = Some(options);
        Ok(self)
    }

    /// 🎯️ Restricts hover and selection to one granularity.
    pub fn with_pick_filter(mut self, filter: World3dPickGranularity) -> Self {
        self.modelling_options.get_or_insert_with(Default::default).pick_filter = Some(filter);
        self
    }

    /// ✂️ Sets the section plane; refuses a degenerate plane.
    pub fn with_section(mut self, section: World3dSection) -> Result<Self, ValueError> {
        section.validate()?;
        self.modelling_options.get_or_insert_with(Default::default).section = Some(section);
        Ok(self)
    }

    /// 🖍️ Sets the sub-element highlight tokens; refuses an out-of-range width.
    pub fn with_highlight(mut self, highlight: World3dHighlight) -> Result<Self, ValueError> {
        for style in [highlight.face, highlight.edge, highlight.vertex].into_iter().flatten() {
            style.validate()?;
        }
        self.modelling_options.get_or_insert_with(Default::default).highlight = Some(highlight);
        Ok(self)
    }
}
//#endregion 🔖️Lanes

#[cfg(test)]
#[path = "../🧪️tests/🔬️world3d-modelling-unit/🦀️.rs"]
mod tests;
