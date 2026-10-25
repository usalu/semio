//! 🏷️ Text, tone and annotation types: the bilingual text, the theme tone, the marker shape, the dimension, angle, marker and leader annotations and the annotation enum. See [`super`].

use super::*;

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

    /// 🌐️ The entry for the active locale (`de`, `de-CH` → German; everything else → English, the first language).
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
    /// 🚩️ The mark a point annotation is drawn with.
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

/// ⚓️ A text label connected to a world anchor by a leader line; `label_offset_px` displaces the label on screen.
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

/// 📌️ One annotation of the layer, discriminated on the wire by `kind`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum World3dAnnotation {
    Dimension(World3dDimension),
    Angle(World3dAngle),
    Marker(World3dMarker),
    Leader(World3dLeader),
}
//#endregion 🔖️Annotations
