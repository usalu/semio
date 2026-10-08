//! 🔤️ Packs framework font assets as `[u32le count][u32le len, bytes]*` for host asset publication.

use std::{env, fs};

fn pack_asset_blobs<'a>(blobs: impl Iterator<Item = &'a [u8]>) -> Vec<u8> {
    let items: Vec<&[u8]> = blobs.collect();
    let mut out = Vec::new();
    out.extend_from_slice(&(items.len() as u32).to_le_bytes());
    for item in items {
        out.extend_from_slice(&(item.len() as u32).to_le_bytes());
        out.extend_from_slice(item);
    }
    out
}

fn main() {
    let out_path = env::args().nth(1).expect("usage: pack-typst-font-assets <output-path>");
    let blob = pack_asset_blobs(typst_assets::fonts());
    fs::write(&out_path, &blob).unwrap_or_else(|error| panic!("failed to write {out_path}: {error}"));
    println!("wrote {} bytes to {out_path}", blob.len());
}
