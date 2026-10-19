//! 🧱️ binary rep for stdio.gltf 🔺️diff

use crate::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::diff_aggregate::*;
#[cfg(test)]
use crate::standards::v2_0::subsets::any::schema::snapshot::GltfDocument;
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use protocol::MutationDiff;
use crate::standards::v2_0::subsets::any::io::binary::diff::wire::{gltf_bin_err, read_bin_accessor, read_bin_accessor_diff, read_bin_animation, read_bin_animation_diff, read_bin_asset_diff, read_bin_blob, read_bin_buffer, read_bin_buffer_diff, read_bin_buffer_view, read_bin_buffer_view_diff, read_bin_camera, read_bin_collection_blob, read_bin_image, read_bin_image_diff, read_bin_json, read_bin_material, read_bin_material_diff, read_bin_mesh, read_bin_mesh_diff, read_bin_node, read_bin_node_diff, read_bin_option, read_bin_sampler, read_bin_scene, read_bin_scene_diff, read_bin_skin, read_bin_skin_diff, read_bin_source_form, read_bin_strings_delta, read_bin_texture, read_bin_texture_diff, read_bin_tri, write_bin_accessor, write_bin_accessor_diff, write_bin_animation, write_bin_animation_diff, write_bin_asset_diff, write_bin_blob, write_bin_buffer, write_bin_buffer_diff, write_bin_buffer_view, write_bin_buffer_view_diff, write_bin_camera, write_bin_collection_blob, write_bin_image, write_bin_image_diff, write_bin_json, write_bin_material, write_bin_material_diff, write_bin_mesh, write_bin_mesh_diff, write_bin_node, write_bin_node_diff, write_bin_option, write_bin_sampler, write_bin_scene, write_bin_scene_diff, write_bin_skin, write_bin_skin_diff, write_bin_source_form, write_bin_strings_delta, write_bin_texture, write_bin_texture_diff, write_bin_tri};

pub use semio_s_artifact_stdio_gltf_model_wire::standards::v2_0::subsets::any::io::binary::diff::wire;

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
impl protocol::DiffBinary for GltfDiff {
/// ⚡️ P2-FG3: real binary diff-frame — upgraded from the F6-era `print_diff().into_bytes()`
/// text-as-binary shortcut (100% of stdio's `DiffCodec` impls were still on that shortcut per
/// the P2-W0 census; the FG1 wave's own closer report flagged leaving this un-upgraded as a
/// real defect to not repeat, and FG2's gif89a upgrade is this file's literal template).
/// Matches `../💾️binary/📡️.protocol.semio`'s real flag-per-field layout exactly,
/// field for field, in `GltfDiff`'s own struct declaration order (2-way flag for plain
/// `Option<T>` fields, 3-way flag for the 3 tri-state fields `scene`/`extensions`/`extras`).
/// Every one of the 14 collection fields is one length-prefixed blob wrapping its own real
/// binary `removed`/`modified`/`added` encoding (`write_bin_collection_blob`) — the blob's
/// OWN internal shape isn't further protocol-walkable (`Prim::Ref` recursion gap), but this
/// Rust side IS genuinely, fully structured real binary throughout, never text-as-bytes.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut w = dsl::ByteWriter::new();
    // `asset`/`extensions_used`/`extensions_required`/`extensions`/`extras` are each wrapped
    // in a length-prefixed blob (matching `../💾️binary/📡️.protocol.semio`'s
    // `Array(u8, Field(<name>_len))` shape exactly) — NOT bare-inline like `scene`/
    // `source_form`'s fixed-width payloads — because they are NOT the last field in the frame
    // and their own internal shape has no fixed width `walk_protocol` could otherwise skip
    // past without knowing its byte length up front.
    write_bin_option(&mut w, &self.asset, |w, v| {
        write_bin_blob(w, &{
            let mut inner = dsl::ByteWriter::new();
            write_bin_asset_diff(&mut inner, v);
            inner.into_bytes()
        });
    });
    write_bin_tri(&mut w, &self.scene, |w, v| w.write_varint_u64(*v as u64));
    write_bin_option(&mut w, &self.scenes, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_scene, write_bin_scene_diff)));
    write_bin_option(&mut w, &self.nodes, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_node, write_bin_node_diff)));
    write_bin_option(&mut w, &self.meshes, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_mesh, write_bin_mesh_diff)));
    write_bin_option(&mut w, &self.accessors, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_accessor, write_bin_accessor_diff)));
    write_bin_option(&mut w, &self.buffer_views, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_buffer_view, write_bin_buffer_view_diff)));
    write_bin_option(&mut w, &self.buffers, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_buffer, write_bin_buffer_diff)));
    write_bin_option(&mut w, &self.buffer_bytes, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, |w, b: &Vec<u8>| write_bin_blob(w, b), |w, b: &Vec<u8>| write_bin_blob(w, b))));
    write_bin_option(&mut w, &self.materials, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_material, write_bin_material_diff)));
    write_bin_option(&mut w, &self.textures, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_texture, write_bin_texture_diff)));
    write_bin_option(&mut w, &self.images, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_image, write_bin_image_diff)));
    write_bin_option(&mut w, &self.samplers, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_sampler, write_bin_sampler)));
    write_bin_option(&mut w, &self.skins, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_skin, write_bin_skin_diff)));
    write_bin_option(&mut w, &self.animations, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_animation, write_bin_animation_diff)));
    write_bin_option(&mut w, &self.cameras, |w, v| write_bin_blob(w, &write_bin_collection_blob(v, write_bin_camera, write_bin_camera)));
    write_bin_option(&mut w, &self.extensions_used, |w, v| {
        write_bin_blob(w, &{
            let mut inner = dsl::ByteWriter::new();
            write_bin_strings_delta(&mut inner, v);
            inner.into_bytes()
        });
    });
    write_bin_option(&mut w, &self.extensions_required, |w, v| {
        write_bin_blob(w, &{
            let mut inner = dsl::ByteWriter::new();
            write_bin_strings_delta(&mut inner, v);
            inner.into_bytes()
        });
    });
    write_bin_tri(&mut w, &self.extensions, |w, v| {
        write_bin_blob(w, &{
            let mut inner = dsl::ByteWriter::new();
            write_bin_json(&mut inner, v);
            inner.into_bytes()
        });
    });
    write_bin_tri(&mut w, &self.extras, |w, v| {
        write_bin_blob(w, &{
            let mut inner = dsl::ByteWriter::new();
            write_bin_json(&mut inner, v);
            inner.into_bytes()
        });
    });
    write_bin_option(&mut w, &self.source_form, |w, v| write_bin_source_form(w, *v));
    Ok(w.into_bytes())
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut r = dsl::ByteReader::new(bytes);
    let asset = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        let mut inner = dsl::ByteReader::new(&b);
        read_bin_asset_diff(&mut inner)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let scene = read_bin_tri(&mut r, |r| Ok(r.read_varint_u64()? as usize)).map_err(|error| gltf_bin_err(&error))?;
    let scenes = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_scene, read_bin_scene_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let nodes = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_node, read_bin_node_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let meshes = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_mesh, read_bin_mesh_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let accessors = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_accessor, read_bin_accessor_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let buffer_views = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_buffer_view, read_bin_buffer_view_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let buffers = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_buffer, read_bin_buffer_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let buffer_bytes = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_blob, read_bin_blob)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let materials = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_material, read_bin_material_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let textures = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_texture, read_bin_texture_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let images = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_image, read_bin_image_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let samplers = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_sampler, read_bin_sampler)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let skins = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_skin, read_bin_skin_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let animations = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_animation, read_bin_animation_diff)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let cameras = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        read_bin_collection_blob(&b, read_bin_camera, read_bin_camera)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let extensions_used = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        let mut inner = dsl::ByteReader::new(&b);
        read_bin_strings_delta(&mut inner)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let extensions_required = read_bin_option(&mut r, |r| {
        let b = read_bin_blob(r)?;
        let mut inner = dsl::ByteReader::new(&b);
        read_bin_strings_delta(&mut inner)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let extensions = read_bin_tri(&mut r, |r| {
        let b = read_bin_blob(r)?;
        let mut inner = dsl::ByteReader::new(&b);
        read_bin_json(&mut inner)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let extras = read_bin_tri(&mut r, |r| {
        let b = read_bin_blob(r)?;
        let mut inner = dsl::ByteReader::new(&b);
        read_bin_json(&mut inner)
    })
    .map_err(|error| gltf_bin_err(&error))?;
    let source_form = read_bin_option(&mut r, read_bin_source_form).map_err(|error| gltf_bin_err(&error))?;
    Ok(GltfDiff { asset, scene, scenes, nodes, meshes, accessors, buffer_views, buffers, buffer_bytes, materials, textures, images, samplers, skins, animations, cameras, extensions_used, extensions_required, extensions, extras, source_form })
}
}
