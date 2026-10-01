//! 🧬️ Authoritative replace-pixel-data mutation: the decoded canonical-RGBA raster is replaced, and the document's
//! storage follows the BITMAPINFOHEADER rules. A `BI_RGB` bitmap of 8 bpp or less stores every pixel as an index into
//! the colour table that follows the header, so the replacement stays indexed exactly when every one of its colours has
//! an entry in that table; otherwise the document becomes the 24-bit `BI_RGB` form, which has no colour table and
//! stores each pixel's RGB triplet directly, with `biClrUsed`/`biClrImportant` 0 and `biSizeImage` the 24-bit stride
//! times the height. Pixels are never narrowed onto the table.
//! <https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader>
use crate::schema::diff::*;
use crate::schema::mutations::{BmpMutation, SetSnapshot};
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplacePixelDataMutation {
    #[dsl(base64)]
    pub pixels: Vec<u8>,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<BmpSnapshot, BmpMutation> for ReplacePixelDataMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "pixel-data", kind: "replace-pixel-data", record: "ReplacePixelData" };
    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        match replaced(base, &self.pixels) {
            Some(next) => protocol::MutationOutcome::new(diff_set_snapshot(base, &next)),
            None => protocol::MutationOutcome::refuse("mutation.target-mismatch", format!("replace-pixel-data carries {} byte(s), not the {}x{} RGBA raster", self.pixels.len(), base.width, base.height), ["pixels".to_string()]),
        }
    }
    fn inverse(&self, base: &BmpSnapshot) -> Vec<BmpMutation> {
        match replaced(base, &self.pixels) {
            Some(next) if next == *base => Vec::new(),
            Some(next) if next.bits_per_pixel == base.bits_per_pixel => vec![BmpMutation::ReplacePixelData(ReplacePixelDataMutation { pixels: base.pixels.clone() })],
            Some(_) => vec![BmpMutation::SetSnapshot(SetSnapshot { snapshot: base.clone() })],
            None => Vec::new(),
        }
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Replace pixel data", "Pixeldaten ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["replace-pixel-data".into()]
    }
}

//#endregion Semantics

//#region Storage
/// 🔲️ `base` with its raster replaced by `pixels`, in the storage that raster needs: kept indexed while every pixel's RGB
/// has a colour-table entry, otherwise promoted to 24-bit `BI_RGB` without a colour table. `None` when `pixels` is not
/// the document's `width`×`height` RGBA raster.
fn replaced(base: &BmpSnapshot, pixels: &[u8]) -> Option<BmpSnapshot> {
    if pixels.len() != base.width as usize * base.height as usize * 4 {
        return None;
    }
    let mut next = base.clone();
    next.pixels = pixels.to_vec();
    let table: std::collections::HashSet<(u8, u8, u8)> = base.palette.iter().map(|entry| (entry.r, entry.g, entry.b)).collect();
    let indexed = matches!(base.bits_per_pixel, 1 | 4 | 8) && !base.palette.is_empty();
    if indexed && !pixels.chunks_exact(4).all(|pixel| table.contains(&(pixel[0], pixel[1], pixel[2]))) {
        next.bits_per_pixel = 24;
        next.palette = Vec::new();
        next.colors_used = 0;
        next.colors_important = 0;
        next.image_size = (base.width * 24).div_ceil(32) * 4 * base.height;
    }
    Some(next)
}
//#endregion Storage

#[cfg(test)]
pub(crate) fn test_case() -> BmpMutation {
    dsl::json::from_json_str(include_str!("../../../🧫️fixtures/🧬️mutations/🔲️replace-pixel-data/🎯️direct/🦠️mutation/🔣️.json")).expect("committed replace-pixel-data payload")
}
#[cfg(test)]
#[path = "🧪️tests/🎯️direct/🦀️.rs"]
mod tests_direct_behavior;
#[cfg(test)]
#[path = "🧪️tests/🎨️keeps/🦀️.rs"]
mod tests_keeps_indexed_storage;
