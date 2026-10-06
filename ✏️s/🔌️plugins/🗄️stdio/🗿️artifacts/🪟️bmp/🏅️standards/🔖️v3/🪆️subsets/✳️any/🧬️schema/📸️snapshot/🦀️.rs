//! 🧬️ Exact BMP byte authority and handcrafted document codecs.

use crate::STDIO_BMP_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum BmpRowOrder {
    #[default]
    BottomUp,
    TopDown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct BmpPaletteEntry {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub reserved: u8,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp")]
pub struct BmpSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    #[dsl(base64)]
    pub bytes: Vec<u8>,
}

impl Default for BmpSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_BMP_DOCUMENT_SCHEMA.into(), bytes: crate::standards::v_v3::subsets::any::io::empty_bmp_bytes() }
    }
}







#[cfg(test)]
#[path = "🧪️tests/🔤️source-hex/🦀️.rs"]
mod source_hex_tests;
