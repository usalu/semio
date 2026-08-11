//! 🧬️ BmpArtifact schema — full artifact state.

use crate::artifacts::bmp::schema::snapshot::{BmpPaletteEntry, BmpRowOrder};
use crate::artifacts::bmp::BmpSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.bmp` artifact state — mirrors `BmpSnapshot`'s complete BITMAPINFOHEADER +
/// palette + pixels model field-for-field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp")]
pub struct BmpArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub header_size: u32,
    #[state(persistent)]
    pub width: u32,
    #[state(persistent)]
    pub height: u32,
    #[state(persistent)]
    pub row_order: BmpRowOrder,
    #[state(persistent)]
    pub planes: u16,
    #[state(persistent)]
    pub bits_per_pixel: u16,
    #[state(persistent)]
    pub compression: u32,
    #[state(persistent)]
    pub image_size: u32,
    #[state(persistent)]
    pub x_pixels_per_meter: i32,
    #[state(persistent)]
    pub y_pixels_per_meter: i32,
    #[state(persistent)]
    pub colors_used: u32,
    #[state(persistent)]
    pub colors_important: u32,
    #[state(persistent)]
    #[serde(default)]
    pub palette: Vec<BmpPaletteEntry>,
    #[state(persistent)]
    #[serde(default)]
    pub pixels: Vec<u8>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for BmpArtifact {
    fn default() -> Self {
        Self::from_snapshot(BmpSnapshot::default())
    }
}

impl BmpArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> BmpSnapshot {
        BmpSnapshot {
            schema: self.schema.clone(),
            header_size: self.header_size,
            width: self.width,
            height: self.height,
            row_order: self.row_order,
            planes: self.planes,
            bits_per_pixel: self.bits_per_pixel,
            compression: self.compression,
            image_size: self.image_size,
            x_pixels_per_meter: self.x_pixels_per_meter,
            y_pixels_per_meter: self.y_pixels_per_meter,
            colors_used: self.colors_used,
            colors_important: self.colors_important,
            palette: self.palette.clone(),
            pixels: self.pixels.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: BmpSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            header_size: snapshot.header_size,
            width: snapshot.width,
            height: snapshot.height,
            row_order: snapshot.row_order,
            planes: snapshot.planes,
            bits_per_pixel: snapshot.bits_per_pixel,
            compression: snapshot.compression,
            image_size: snapshot.image_size,
            x_pixels_per_meter: snapshot.x_pixels_per_meter,
            y_pixels_per_meter: snapshot.y_pixels_per_meter,
            colors_used: snapshot.colors_used,
            colors_important: snapshot.colors_important,
            palette: snapshot.palette,
            pixels: snapshot.pixels,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: BmpSnapshot) {
        self.schema = snapshot.schema;
        self.header_size = snapshot.header_size;
        self.width = snapshot.width;
        self.height = snapshot.height;
        self.row_order = snapshot.row_order;
        self.planes = snapshot.planes;
        self.bits_per_pixel = snapshot.bits_per_pixel;
        self.compression = snapshot.compression;
        self.image_size = snapshot.image_size;
        self.x_pixels_per_meter = snapshot.x_pixels_per_meter;
        self.y_pixels_per_meter = snapshot.y_pixels_per_meter;
        self.colors_used = snapshot.colors_used;
        self.colors_important = snapshot.colors_important;
        self.palette = snapshot.palette;
        self.pixels = snapshot.pixels;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.bmp`.
pub fn bmp_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.bmp",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
