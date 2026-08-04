//! 🪶️ GUESTSLIM: writes the packed typst default font set to the path given as `argv[1]`, in the
//! same `[u32le count][u32le len, bytes]*` wire format `infinite_canvas::host_asset::split_blobs`
//! decodes guest-side and the native plugin host's `pack_asset_blobs` produces for `read-asset`.
//! Invoked once (cached) by os-dev `📜️script.ts`'s `ensureGuestSlimTypstFontsAsset` so the browser
//! dev pipeline can static-serve an identical blob for the jco/worker path.
use std::env;
use std::fs;

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
    let out_path = env::args().nth(1).expect("usage: dump-guestslim-typst-fonts <output-path>");
    let blob = pack_asset_blobs(typst_assets::fonts());
    fs::write(&out_path, &blob).unwrap_or_else(|error| panic!("failed to write {out_path}: {error}"));
    println!("wrote {} bytes to {out_path}", blob.len());
}
