//! 🖼️ framework/products/os/modules/renderer/engine/elements/IconRenderHost/component.rs — wgpu
//! icon atlas implementation for the IconRenderHost element, extracted from lib.rs's inline
//! `pub mod icon_atlas { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired via
//! `#[path = "../../../../🧱️elements/IconRenderHost/🧊️component.rs"] pub mod icon_atlas;` in lib.rs
//! in place of the former inline block; the module name `icon_atlas` is unchanged, so every
//! existing `crate::icon_atlas::...` call site elsewhere in the crate keeps resolving with zero
//! other changes.
//! 🖼️ CPU-rasterized Lucide icon atlas for native and web wgpu shells.

use ui_wgpu::IconAtlas;

const ICON_SIZE: u32 = 24;
const ATLAS_COLS: u32 = 16;
const ICON_ATLAS_TEXTURE_SIZE: u32 = 2048;

include!(concat!(env!("OUT_DIR"), "/icons_🤖️generated.rs"));

fn rasterize_svg(svg: &str, tint_mask: bool) -> Option<Vec<u8>> {
    let mut options = usvg::Options::default();
    options.fontdb_mut().load_system_fonts();
    let tree = usvg::Tree::from_str(svg, &options).ok()?;
    let mut pixmap = tiny_skia::Pixmap::new(ICON_SIZE, ICON_SIZE)?;
    let scale = (ICON_SIZE as f32 / tree.size().width()).min(ICON_SIZE as f32 / tree.size().height());
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

pub fn build_icon_atlas() -> IconAtlas {
    let mut loaded: Vec<(&str, Vec<u8>)> = Vec::new();
    for (id, svg) in ICON_SVGS {
        let Some(pixels) = rasterize_svg(svg, *id != "semio-logo") else {
            continue;
        };
        loaded.push((id, pixels));
    }
    if let Some(pixels) = rasterize_svg(SEMIO_LOGO_SVG, false) {
        loaded.push(("semio-logo", pixels));
    }
    let rows = loaded.len().div_ceil(ATLAS_COLS as usize);
    let width = ATLAS_COLS * ICON_SIZE;
    let height = (rows as u32).max(1) * ICON_SIZE;
    let mut pixels = vec![0u8; (width * height * 4) as usize];
    let mut entries = Vec::new();
    for (index, (id, icon_pixels)) in loaded.into_iter().enumerate() {
        let col = (index as u32) % ATLAS_COLS;
        let row = (index as u32) / ATLAS_COLS;
        let ox = col * ICON_SIZE;
        let oy = row * ICON_SIZE;
        for y in 0..ICON_SIZE {
            for x in 0..ICON_SIZE {
                let src = ((y * ICON_SIZE + x) * 4) as usize;
                let dst = (((oy + y) * width + (ox + x)) * 4) as usize;
                pixels[dst] = icon_pixels[src];
                pixels[dst + 1] = icon_pixels[src + 1];
                pixels[dst + 2] = icon_pixels[src + 2];
                pixels[dst + 3] = icon_pixels[src + 3];
            }
        }
        let texture = ICON_ATLAS_TEXTURE_SIZE as f32;
        entries.push((id.to_string(), [ox as f32 / texture, oy as f32 / texture, (ox + ICON_SIZE) as f32 / texture, (oy + ICON_SIZE) as f32 / texture]));
    }
    IconAtlas::from_packed(width, height, pixels, entries)
}
