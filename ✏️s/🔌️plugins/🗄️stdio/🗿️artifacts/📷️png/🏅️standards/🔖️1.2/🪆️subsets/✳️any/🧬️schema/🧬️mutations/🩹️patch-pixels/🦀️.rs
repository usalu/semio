//! 🩹️ Bounded PNG pixel-range patch with an exact inverse. The canonical raster is `width`×`height` RGBA, and PNG's
//! image data holds exactly `height` scanlines of `width` pixels (PNG 1.2 §2.3, IHDR §4.1.1), so a patch that would
//! change the raster's byte length is refused rather than leaving a snapshot no encoder can write.
//! <https://www.w3.org/TR/PNG/#11IHDR>
use crate::schema::diff::*;
use crate::schema::mutations::PngMutation;
use crate::schema::snapshot::*;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PatchPixelsMutation {
    pub index: u64,
    pub remove_count: u64,
    pub pixels: Vec<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub move_to: Option<u64>,
}

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

fn apply(base: &[u8], patch: &PatchPixelsMutation) -> Result<Vec<u8>, String> {
    let index = usize::try_from(patch.index).map_err(|_| "PNG pixel index exceeds this platform".to_string())?;
    if let Some(move_to) = patch.move_to {
        let move_to = usize::try_from(move_to).map_err(|_| "PNG move destination exceeds this platform".to_string())?;
        if patch.remove_count != 0 || !patch.pixels.is_empty() || index >= base.len() || move_to >= base.len() {
            return Err("PNG pixel move is outside the pixel range or carries replacement data".into());
        }
        let mut next = base.to_vec();
        let value = next.remove(index);
        next.insert(move_to, value);
        return Ok(next);
    }
    let remove_count = usize::try_from(patch.remove_count).map_err(|_| "PNG removal count exceeds this platform".to_string())?;
    let end = index.checked_add(remove_count).ok_or_else(|| "PNG pixel range overflows".to_string())?;
    if index > base.len() || end > base.len() {
        return Err("PNG pixel patch is outside the pixel range".into());
    }
    if patch.pixels.len() != remove_count {
        return Err(format!("PNG pixel patch replaces {remove_count} byte(s) with {}, which would change the raster's byte length", patch.pixels.len()));
    }
    let mut next = base.to_vec();
    next.splice(index..end, patch.pixels.iter().copied());
    Ok(next)
}

impl protocol::MutationKind<PngSnapshot, PngMutation> for PatchPixelsMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "pixels", kind: "patch-pixels", record: "PatchPixels" };
    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        match apply(&base.pixels, self) {
            Ok(pixels) => protocol::MutationOutcome::new(crate::schema::mutations::replace_pixels::contribute(base, pixels)),
            Err(message) => protocol::MutationOutcome::error("mutation.target-mismatch", message, ["pixels".into(), self.index.to_string()]),
        }
    }
    fn inverse(&self, base: &PngSnapshot) -> Vec<PngMutation> {
        let Ok(index) = usize::try_from(self.index) else { return Vec::new() };
        if let Some(move_to) = self.move_to {
            if apply(&base.pixels, self).is_err() { return Vec::new(); }
            return vec![PngMutation::PatchPixels(PatchPixelsMutation { index: move_to, remove_count: 0, pixels: Vec::new(), move_to: Some(self.index) })];
        }
        let Ok(remove_count) = usize::try_from(self.remove_count) else { return Vec::new() };
        let Some(end) = index.checked_add(remove_count) else { return Vec::new() };
        if end > base.pixels.len() || apply(&base.pixels, self).is_err() { return Vec::new(); }
        vec![PngMutation::PatchPixels(PatchPixelsMutation { index: self.index, remove_count: self.pixels.len() as u64, pixels: base.pixels[index..end].to_vec(), move_to: None })]
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Patch pixels", "Pixel bearbeiten") }
    fn target(&self) -> Vec<String> { vec!["pixels".into(), self.index.to_string()] }
}

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    PngMutation::PatchPixels(PatchPixelsMutation { index: 1, remove_count: 1, pixels: vec![9], move_to: None })
}
