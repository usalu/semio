//! 🧬️ Owned JPEG image content, density, thumbnail, and opaque metadata.

use crate::STDIO_JPG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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

//#region 🌲️CanonicalTree
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as TreeNode, ArtifactCanonicalJsonText as TreeText, ArtifactCanonicalJsonTree as Tree};
use semio_framework_value::{ValueError as TreeError, ValueRefusalKind as TreeRefusal};

fn tree_absent(reason: &'static str) -> TreeError {
    TreeError::literal(TreeRefusal::InvariantViolated, reason)
}

/// 🌲️ Octet fields keep the canonical JSON array projection their `pack::value::bytes` role already emits.
impl Tree for JfifThumbnail {
    fn canonical_tree_node(&self) -> Result<TreeNode<'_>, TreeError> {
        Ok(TreeNode::Object(3))
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, TreeError> {
        match ordinal {
            0 => Ok(&self.width),
            1 => Ok(&self.height),
            2 => Ok(&self.rgb_data),
            _ => Err(tree_absent("canonical JFIF thumbnail ordinal is absent")),
        }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<TreeText<'_>, TreeError> {
        ["width", "height", "rgbData"].get(ordinal).map(|key| TreeText::from(*key)).ok_or_else(|| tree_absent("canonical JFIF thumbnail key is absent"))
    }
}

impl Tree for JpgSegment {
    fn canonical_tree_node(&self) -> Result<TreeNode<'_>, TreeError> {
        Ok(TreeNode::Object(2))
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, TreeError> {
        match ordinal {
            0 => Ok(&self.marker),
            1 => Ok(&self.data),
            _ => Err(tree_absent("canonical JPEG segment ordinal is absent")),
        }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<TreeText<'_>, TreeError> {
        ["marker", "data"].get(ordinal).map(|key| TreeText::from(*key)).ok_or_else(|| tree_absent("canonical JPEG segment key is absent"))
    }
}

impl JpgImage {
    fn canonical_fields(&self) -> ([u8; 9], usize) {
        let mut fields = [0u8; 9];
        let mut count = 0;
        for field in 0..9u8 {
            if field == 7 && self.jfif_thumbnail.is_none() {
                continue;
            }
            fields[count] = field;
            count += 1;
        }
        (fields, count)
    }
}

/// 🌲️ Projects exactly the fields `ToValue` emits; the thumbnail is skipped when absent.
impl Tree for JpgImage {
    fn canonical_tree_node(&self) -> Result<TreeNode<'_>, TreeError> {
        Ok(TreeNode::Object(self.canonical_fields().1))
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, TreeError> {
        let (fields, count) = self.canonical_fields();
        if ordinal >= count {
            return Err(tree_absent("canonical JPEG image ordinal is absent"));
        }
        match fields[ordinal] {
            0 => Ok(&self.width),
            1 => Ok(&self.height),
            2 => Ok(&self.pixels),
            3 => Ok(&self.jfif_version),
            4 => Ok(&self.jfif_density_units),
            5 => Ok(&self.jfif_x_density),
            6 => Ok(&self.jfif_y_density),
            7 => self.jfif_thumbnail.as_ref().map(|value| value as &dyn Tree).ok_or_else(|| tree_absent("canonical JPEG thumbnail is absent")),
            _ => Ok(&self.other_segments),
        }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<TreeText<'_>, TreeError> {
        let (fields, count) = self.canonical_fields();
        if ordinal >= count {
            return Err(tree_absent("canonical JPEG image key is absent"));
        }
        Ok(TreeText::from(["width", "height", "pixels", "jfifVersion", "jfifDensityUnits", "jfifXDensity", "jfifYDensity", "jfifThumbnail", "otherSegments"][fields[ordinal] as usize]))
    }
}
//#endregion 🌲️CanonicalTree
