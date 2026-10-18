//! 📏️ Domain-neutral world-3d modelling primitives a B-Rep or mesh modelling app needs: an annotation
//! layer, scalar field colouring with a legend, a pick granularity filter, a section plane and
//! sub-element highlight tokens. The language-neutral contract is
//! `🧬️schema/📏️world3d-modelling/🔣️.json` plus `🧪️tests/🧫️fixtures/📏️world3d-modelling/🔣️.json`; the
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

#[path = "🏷️text-tone-and-annotation-types/🦀️.rs"]
mod text_tone_and_annotation_types;
pub use text_tone_and_annotation_types::{World3dMarkerShape, World3dTone};
pub use text_tone_and_annotation_types::{
    World3dAngle, World3dAnnotation, World3dDimension, World3dLeader, World3dMarker, World3dText,
};

#[path = "📐️annotation-codecs/🦀️.rs"]
mod annotation_codecs;
pub use annotation_codecs::World3dAnnotationLayer;

#[path = "🌡️scalar-field/🦀️.rs"]
mod scalar_field;
pub use scalar_field::{World3dColorRamp, World3dScalarDomain};
pub use scalar_field::{
    World3dLegendTick, World3dScalarField, World3dScalarLegend, World3dScalarRange,
};

#[path = "🎯️interaction-options/🦀️.rs"]
mod interaction_options;
pub use interaction_options::World3dPickGranularity;
pub use interaction_options::{
    World3dHighlight, World3dModellingOptions, World3dPickTargets, World3dSection,
    World3dSectionCap, World3dSubElementStyle,
};

#[path = "🧭️overlay-projection/🦀️.rs"]
mod overlay_projection;
pub use overlay_projection::{
    project_world3d_annotations, World3dLabelAlign, World3dProjectedAnnotation, World3dProjectedArc,
    World3dProjectedArrow, World3dProjectedLabel, World3dProjectedMarker,
    WORLD3D_ANNOTATION_ARROW_PX, WORLD3D_ANNOTATION_LABEL_GAP_PX, WORLD3D_ANNOTATION_MARKER_PX,
    WORLD3D_ANNOTATION_STROKE_PX,
};

#[path = "🔤️legend-strings-and-sub-element-paint/🦀️.rs"]
mod legend_strings_and_sub_element_paint;
pub use legend_strings_and_sub_element_paint::{
    format_world3d_legend_value, world3d_modelling_from_lane_text, world3d_modelling_lane_text,
    world3d_modelling_strings, world3d_sub_element_paint, World3dLegendLine, World3dLegendLines,
    World3dModellingStrings, World3dSubElementDefaults, World3dSubElementPaint,
};

//#endregion 🔖️Text

#[cfg(test)]
#[path = "../🧪️tests/🔬️world3d-modelling-unit/🦀️.rs"]
mod tests;
