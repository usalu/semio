//! 📝️ Text representation codec surface for `stdio.gltf` (diff).

use crate::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::diff_aggregate::*;
use protocol::{DiffText,DiffBinary};
#[cfg(test)]
use crate::standards::v2_0::subsets::any::schema::snapshot::GltfDocument;
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use protocol::MutationDiff;
use crate::standards::v2_0::subsets::any::io::text::diff::wire::*;

pub use semio_s_artifact_stdio_gltf_model_wire_2::standards::v2_0::subsets::any::io::text::diff::wire;

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
impl protocol::DiffText for GltfDiff {
fn print_diff(&self) -> String {
    print_gltf_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_gltf_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_gltf_diff(d: &GltfDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.asset {
        tokens.push(format!("asset={}", enc_asset_diff(v)));
    }
    if let Some(v) = d.scene {
        tokens.push(format!("scene={}", encode_option(&v, |x| x.to_string())));
    }
    if let Some(v) = &d.scenes {
        tokens.push(format!("scenes={}", enc_collection(v, enc_scene, enc_scene_diff)));
    }
    if let Some(v) = &d.nodes {
        tokens.push(format!("nodes={}", enc_collection(v, enc_node, enc_node_diff)));
    }
    if let Some(v) = &d.meshes {
        tokens.push(format!("meshes={}", enc_collection(v, enc_mesh, enc_mesh_diff)));
    }
    if let Some(v) = &d.accessors {
        tokens.push(format!("accessors={}", enc_collection(v, enc_accessor, enc_accessor_diff)));
    }
    if let Some(v) = &d.buffer_views {
        tokens.push(format!("buffer-views={}", enc_collection(v, enc_buffer_view, enc_buffer_view_diff)));
    }
    if let Some(v) = &d.buffers {
        tokens.push(format!("buffers={}", enc_collection(v, enc_buffer, enc_buffer_diff)));
    }
    if let Some(v) = &d.buffer_bytes {
        tokens.push(format!("buffer-bytes={}", enc_collection(v, |b: &Vec<u8>| enc_bytes(b), |b: &Vec<u8>| enc_bytes(b))));
    }
    if let Some(v) = &d.materials {
        tokens.push(format!("materials={}", enc_collection(v, enc_material, enc_material_diff)));
    }
    if let Some(v) = &d.textures {
        tokens.push(format!("textures={}", enc_collection(v, enc_texture, enc_texture_diff)));
    }
    if let Some(v) = &d.images {
        tokens.push(format!("images={}", enc_collection(v, enc_image, enc_image_diff)));
    }
    if let Some(v) = &d.samplers {
        tokens.push(format!("samplers={}", enc_collection(v, enc_sampler, enc_sampler)));
    }
    if let Some(v) = &d.skins {
        tokens.push(format!("skins={}", enc_collection(v, enc_skin, enc_skin_diff)));
    }
    if let Some(v) = &d.animations {
        tokens.push(format!("animations={}", enc_collection(v, enc_animation, enc_animation_diff)));
    }
    if let Some(v) = &d.cameras {
        tokens.push(format!("cameras={}", enc_collection(v, enc_camera, enc_camera)));
    }
    if let Some(v) = &d.extensions_used {
        tokens.push(format!("extensions-used={}", enc_strings_delta(v)));
    }
    if let Some(v) = &d.extensions_required {
        tokens.push(format!("extensions-required={}", enc_strings_delta(v)));
    }
    if let Some(v) = &d.extensions {
        tokens.push(format!("extensions={}", encode_option(v, enc_json)));
    }
    if let Some(v) = &d.extras {
        tokens.push(format!("extras={}", encode_option(v, enc_json)));
    }
    if let Some(v) = d.source_form {
        tokens.push(format!("source-form={}", enc_source_form(v)));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_gltf_diff(line: &str) -> Result<GltfDiff, String> {
    let mut d = GltfDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("asset=") {
            d.asset = Some(dec_asset_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("scene=") {
            d.scene = Some(decode_option(rest, parse_usize)?);
        } else if let Some(rest) = token.strip_prefix("scenes=") {
            d.scenes = Some(dec_collection(rest, dec_scene, dec_scene_diff)?);
        } else if let Some(rest) = token.strip_prefix("nodes=") {
            d.nodes = Some(dec_collection(rest, dec_node, dec_node_diff)?);
        } else if let Some(rest) = token.strip_prefix("meshes=") {
            d.meshes = Some(dec_collection(rest, dec_mesh, dec_mesh_diff)?);
        } else if let Some(rest) = token.strip_prefix("accessors=") {
            d.accessors = Some(dec_collection(rest, dec_accessor, dec_accessor_diff)?);
        } else if let Some(rest) = token.strip_prefix("buffer-views=") {
            d.buffer_views = Some(dec_collection(rest, dec_buffer_view, dec_buffer_view_diff)?);
        } else if let Some(rest) = token.strip_prefix("buffer-bytes=") {
            d.buffer_bytes = Some(dec_collection(rest, dec_bytes, dec_bytes)?);
        } else if let Some(rest) = token.strip_prefix("buffers=") {
            d.buffers = Some(dec_collection(rest, dec_buffer, dec_buffer_diff)?);
        } else if let Some(rest) = token.strip_prefix("materials=") {
            d.materials = Some(dec_collection(rest, dec_material, dec_material_diff)?);
        } else if let Some(rest) = token.strip_prefix("textures=") {
            d.textures = Some(dec_collection(rest, dec_texture, dec_texture_diff)?);
        } else if let Some(rest) = token.strip_prefix("images=") {
            d.images = Some(dec_collection(rest, dec_image, dec_image_diff)?);
        } else if let Some(rest) = token.strip_prefix("samplers=") {
            d.samplers = Some(dec_collection(rest, dec_sampler, dec_sampler)?);
        } else if let Some(rest) = token.strip_prefix("skins=") {
            d.skins = Some(dec_collection(rest, dec_skin, dec_skin_diff)?);
        } else if let Some(rest) = token.strip_prefix("animations=") {
            d.animations = Some(dec_collection(rest, dec_animation, dec_animation_diff)?);
        } else if let Some(rest) = token.strip_prefix("cameras=") {
            d.cameras = Some(dec_collection(rest, dec_camera, dec_camera)?);
        } else if let Some(rest) = token.strip_prefix("extensions-used=") {
            d.extensions_used = Some(dec_strings_delta(rest)?);
        } else if let Some(rest) = token.strip_prefix("extensions-required=") {
            d.extensions_required = Some(dec_strings_delta(rest)?);
        } else if let Some(rest) = token.strip_prefix("extensions=") {
            d.extensions = Some(decode_option(rest, dec_json)?);
        } else if let Some(rest) = token.strip_prefix("extras=") {
            d.extras = Some(decode_option(rest, dec_json)?);
        } else if let Some(rest) = token.strip_prefix("source-form=") {
            d.source_form = Some(dec_source_form(rest)?);
        } else {
            return Err(format!("gltf diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}
