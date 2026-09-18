//! 🖼️ framework/products/os/modules/renderer/engine/elements/IconRenderHost/component.rs — wgpu
//! icon atlas implementation for the IconRenderHost element, extracted from lib.rs's inline
//! `pub mod icon_atlas { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired via
//! the renderer's exact `../../../🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs` module path
//! in place of the former inline block; the module name `icon_atlas` is unchanged, so every
//! existing `crate::icon_atlas::...` call site elsewhere in the crate keeps resolving with zero
//! other changes.
//! 🖼️ CPU-rasterized Lucide icon atlas for native and web wgpu shells.

use ui_wgpu::wgpu::IconAtlas;

const ICON_SIZE: u32 = 24;
const ATLAS_COLS: u32 = 16;
const ICON_ATLAS_TEXTURE_SIZE: u32 = 2048;

include!(concat!(env!("OUT_DIR"), "/🧩️icons.rs"));

/// 📐️ Device-pixel cell edge for a surface scale factor. The atlas is a fixed
/// [`ICON_ATLAS_TEXTURE_SIZE`]-normalised UV space, so the cell may grow but the 16-column grid must
/// still fit: 250 icons at 16 columns is 16 rows, which caps the usable raster scale at 3.
fn icon_cell_size(scale_factor: f32) -> u32 {
    let scale = if scale_factor.is_finite() { scale_factor.round().clamp(1.0, 3.0) as u32 } else { 1 };
    ICON_SIZE * scale
}

fn rasterize_svg(svg: &str, tint_mask: bool, cell: u32) -> Option<Vec<u8>> {
    let mut options = usvg::Options::default();
    options.fontdb_mut().load_system_fonts();
    let tree = usvg::Tree::from_str(svg, &options).ok()?;
    let mut pixmap = tiny_skia::Pixmap::new(cell, cell)?;
    let scale = (cell as f32 / tree.size().width()).min(cell as f32 / tree.size().height());
    let transform = tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    let mut pixels = pixmap.take();
    if tint_mask {
        for chunk in pixels.chunks_mut(4) {
            let alpha = chunk[3];
            chunk[0] = 255;
            chunk[1] = 255;
            chunk[2] = 255;
            chunk[3] = alpha;
        }
    }
    Some(pixels)
}

/// 🖼️ The 1× atlas — every icon cell rasterised at [`ICON_SIZE`] device pixels.
pub fn build_icon_atlas() -> IconAtlas {
    build_icon_atlas_scaled(1.0)
}

//#region 🔁️IconRaster

/// 🔁️ Icons rasterised per step of a density change. ~250 `usvg`/`resvg` rasterisations at once blow
/// the interactive deadline, so a post-boot display swap pays for them across frames, the same way
/// the glyph atlas re-rasterises lazily per glyph. Sized so one step stays well inside a frame.
pub const ICON_RASTER_STEP_BUDGET: usize = 16;

/// 🔁️ What one [`IconAtlasRebuild::step`] did.
pub enum IconAtlasRebuildStep {
    /// 🖌️ This many sources were rasterised; call again.
    Rasterized(usize),
    /// ✅️ Every source is rasterised and packed — install this atlas and re-upload it.
    Complete(IconAtlas),
}

/// 🔁️ A bounded, resumable re-rasterisation of the whole icon atlas at a new device density. Its own
/// state machine rather than a loop, so the caller decides how much of a frame to spend: dragging a
/// window onto a higher-density display used to keep boot-density icons forever, because
/// `build_icon_atlas_scaled` was only ever called once at boot (ticket 26/09/17 packet W2k, W1g gap 1).
pub struct IconAtlasRebuild {
    scale_factor: f32,
    cell: u32,
    next: usize,
    loaded: Vec<(&'static str, Vec<u8>)>,
}

impl IconAtlasRebuild {
    /// 🔁️ A rebuild for `scale_factor`, with nothing rasterised yet.
    pub fn new(scale_factor: f32) -> Self {
        Self { scale_factor, cell: icon_cell_size(scale_factor), next: 0, loaded: Vec::with_capacity(icon_atlas_source_count()) }
    }

    /// 📐️ The device-pixel cell edge this rebuild rasterises at.
    pub fn cell_size(&self) -> u32 {
        self.cell
    }

    /// 📐️ The surface scale factor this rebuild is for.
    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    /// 📊️ Sources rasterised so far, out of [`icon_atlas_source_count`].
    pub fn progress(&self) -> (usize, usize) {
        (self.next, icon_atlas_source_count())
    }

    /// 🔁️ Rasterises at most `budget` more sources, then packs and answers the atlas once every
    /// source has been through. A source that fails to rasterise is skipped exactly as the one-shot
    /// build skips it, so a rebuild can never stall on a malformed SVG.
    pub fn step(&mut self, budget: usize) -> IconAtlasRebuildStep {
        let total = icon_atlas_source_count();
        let mut done = 0;
        while done < budget.max(1) && self.next < total {
            let index = self.next;
            self.next += 1;
            done += 1;
            let rasterized = if index < ICON_SVGS.len() {
                let (id, svg) = ICON_SVGS[index];
                rasterize_svg(svg, id != "semio-logo", self.cell).map(|pixels| (id, pixels))
            } else {
                rasterize_svg(SEMIO_LOGO_SVG, false, self.cell).map(|pixels| ("semio-logo", pixels))
            };
            if let Some(entry) = rasterized {
                self.loaded.push(entry);
            }
        }
        if self.next < total {
            return IconAtlasRebuildStep::Rasterized(done);
        }
        IconAtlasRebuildStep::Complete(pack_icon_atlas(std::mem::take(&mut self.loaded), self.cell))
    }
}

//#endregion 🔁️IconRaster

/// 🖼️ The atlas rasterised for a surface scale factor: cells are `ICON_SIZE * round(scale_factor)`
/// device pixels so an icon drawn at its LOGICAL size is pixel-crisp instead of an upscaled 1× cell.
/// UVs stay normalised against the fixed [`ICON_ATLAS_TEXTURE_SIZE`], so no draw call changes. Drives
/// [`IconAtlasRebuild`] to completion in one call, so there is exactly ONE rasterisation path.
pub fn build_icon_atlas_scaled(scale_factor: f32) -> IconAtlas {
    let mut rebuild = IconAtlasRebuild::new(scale_factor);
    loop {
        if let IconAtlasRebuildStep::Complete(atlas) = rebuild.step(icon_atlas_source_count()) {
            return atlas;
        }
    }
}

/// 🧩️ Packs rasterised cells into the 16-column atlas and normalises every UV against the fixed
/// [`ICON_ATLAS_TEXTURE_SIZE`].
fn pack_icon_atlas(loaded: Vec<(&'static str, Vec<u8>)>, cell: u32) -> IconAtlas {
    let rows = loaded.len().div_ceil(ATLAS_COLS as usize);
    let width = ATLAS_COLS * cell;
    let height = (rows as u32).max(1) * cell;
    let mut pixels = vec![0u8; (width * height * 4) as usize];
    let mut entries = Vec::new();
    for (index, (id, icon_pixels)) in loaded.into_iter().enumerate() {
        let col = (index as u32) % ATLAS_COLS;
        let row = (index as u32) / ATLAS_COLS;
        let ox = col * cell;
        let oy = row * cell;
        for y in 0..cell {
            for x in 0..cell {
                let src = ((y * cell + x) * 4) as usize;
                let dst = (((oy + y) * width + (ox + x)) * 4) as usize;
                pixels[dst] = icon_pixels[src];
                pixels[dst + 1] = icon_pixels[src + 1];
                pixels[dst + 2] = icon_pixels[src + 2];
                pixels[dst + 3] = icon_pixels[src + 3];
            }
        }
        let texture = ICON_ATLAS_TEXTURE_SIZE as f32;
        entries.push((id.to_string(), [ox as f32 / texture, oy as f32 / texture, (ox + cell) as f32 / texture, (oy + cell) as f32 / texture]));
    }
    IconAtlas::from_packed(width, height, pixels, entries)
}

pub fn icon_atlas_source_count() -> usize {
    ICON_SVGS.len() + 1
}

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;
