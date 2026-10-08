//! 🧬️ Owned JPEG image content, density, thumbnail, and opaque metadata.

use crate::STDIO_JPG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub enum JfifDensityUnits {
    #[default]
    Aspect,
    PixelsPerInch,
    PixelsPerCm,
}

impl JfifDensityUnits {
    pub fn from_u8(v: u8) -> Result<Self, String> {
        match v {
            0 => Ok(JfifDensityUnits::Aspect),
            1 => Ok(JfifDensityUnits::PixelsPerInch),
            2 => Ok(JfifDensityUnits::PixelsPerCm),
            _ => Err(format!("jfif: unsupported density unit {v}")),
        }
    }
    pub fn to_u8(self) -> u8 {
        match self {
            JfifDensityUnits::Aspect => 0,
            JfifDensityUnits::PixelsPerInch => 1,
            JfifDensityUnits::PixelsPerCm => 2,
        }
    }
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct JfifThumbnail {
    pub width: u8,
    pub height: u8,
    #[value(default, with = "pack::value::bytes", serialize_controlled_with = "pack::value::bytes::to_value_controlled")]
    pub rgb_data: Vec<u8>,
}

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JpgSegment {
    pub marker: u8,
    #[value(default, with = "pack::value::bytes", serialize_controlled_with = "pack::value::bytes::to_value_controlled")]
    pub data: Vec<u8>,
}

/// 🖼️ Owned decoded image and meaningful JFIF and application metadata.
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct JpgImage {
    pub width:u32,
    pub height:u32,
    #[value(with = "pack::value::bytes", serialize_controlled_with = "pack::value::bytes::to_value_controlled")]
    pub pixels:Vec<u8>,
    pub jfif_version:(u8,u8),
    pub jfif_density_units:JfifDensityUnits,
    pub jfif_x_density:u16,
    pub jfif_y_density:u16,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub jfif_thumbnail:Option<JfifThumbnail>,
    pub other_segments:Vec<JpgSegment>,
}
impl Default for JpgImage {
    fn default()->Self{Self{width:0,height:0,pixels:Vec::new(),jfif_version:(1,1),jfif_density_units:JfifDensityUnits::Aspect,jfif_x_density:1,jfif_y_density:1,jfif_thumbnail:None,other_segments:Vec::new()}}
}
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.stdio.jpg")]
pub struct JpgSnapshot {
    #[state(artifact)]
    pub schema:String,
    #[state(artifact)]
    pub image:JpgImage,
}
impl Default for JpgSnapshot {
    fn default()->Self{Self{schema:STDIO_JPG_DOCUMENT_SCHEMA.into(),image:JpgImage::default()}}
}
