//! 🧵️ Hand-projected canonical trees for the glTF records whose wire fields carry custom presence or codec roles.
use super::*;
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText as Text, ArtifactCanonicalJsonTree as Tree};
use semio_framework_value::{ValueError, ValueRefusalKind};

type Field<'a> = Option<(&'static str, &'a dyn Tree)>;

fn absent() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, "canonical glTF ordinal is absent")
}

fn node<const N: usize>(fields: &[Field<'_>; N]) -> Result<Node<'static>, ValueError> {
    Ok(Node::Object(fields.iter().flatten().count()))
}

fn child<'a, const N: usize>(fields: &[Field<'a>; N], ordinal: usize) -> Result<&'a dyn Tree, ValueError> {
    fields.iter().flatten().nth(ordinal).map(|(_, value)| *value).ok_or_else(absent)
}

fn key<const N: usize>(fields: &[Field<'_>; N], ordinal: usize) -> Result<Text<'static>, ValueError> {
    fields.iter().flatten().nth(ordinal).map(|(name, _)| (*name).into()).ok_or_else(absent)
}

macro_rules! canonical_record {
    (@unit $field:ident) => { () };
    ($type:ty { $($field:ident => $wire:literal if $absent:expr;)* }) => {
        impl $type {
            fn canonical_fields(&self) -> [Field<'_>; <[()]>::len(&[$(canonical_record!(@unit $field)),*])] {
                [$(if ($absent)(&self.$field) { None } else { Some(($wire, &self.$field as &dyn Tree)) }),*]
            }
        }
        impl Tree for $type {
            fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
                node(&self.canonical_fields())
            }
            fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
                child(&self.canonical_fields(), ordinal)
            }
            fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
                key(&self.canonical_fields(), ordinal)
            }
        }
    };
}

canonical_record!(GltfAsset {
    version => "version" if |_| false;
    generator => "generator" if Option::is_none;
    copyright => "copyright" if Option::is_none;
    min_version => "minVersion" if Option::is_none;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfScene {
    nodes => "nodes" if Vec::is_empty;
    name => "name" if Option::is_none;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfNode {
    children => "children" if Vec::is_empty;
    mesh => "mesh" if Option::is_none;
    camera => "camera" if Option::is_none;
    skin => "skin" if Option::is_none;
    matrix => "matrix" if Option::is_none;
    translation => "translation" if Option::is_none;
    rotation => "rotation" if Option::is_none;
    scale => "scale" if Option::is_none;
    weights => "weights" if Vec::is_empty;
    name => "name" if Option::is_none;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfPrimitive {
    attributes => "attributes" if |_| false;
    indices => "indices" if Option::is_none;
    material => "material" if Option::is_none;
    mode => "mode" if Option::is_none;
    targets => "targets" if Vec::is_empty;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfMesh {
    primitives => "primitives" if Vec::is_empty;
    weights => "weights" if Vec::is_empty;
    name => "name" if Option::is_none;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfSparseIndices {
    buffer_view => "bufferView" if |_| false;
    byte_offset => "byteOffset" if is_zero_usize;
    component_type => "componentType" if |_| false;
});

canonical_record!(GltfSparseValues {
    buffer_view => "bufferView" if |_| false;
    byte_offset => "byteOffset" if is_zero_usize;
});

canonical_record!(GltfAccessor {
    buffer_view => "bufferView" if Option::is_none;
    byte_offset => "byteOffset" if is_zero_usize;
    component_type => "componentType" if |_| false;
    normalized => "normalized" if is_false;
    count => "count" if |_| false;
    kind => "type" if |_| false;
    max => "max" if Option::is_none;
    min => "min" if Option::is_none;
    sparse => "sparse" if Option::is_none;
    name => "name" if Option::is_none;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfBufferView {
    buffer => "buffer" if |_| false;
    byte_offset => "byteOffset" if is_zero_usize;
    byte_length => "byteLength" if |_| false;
    byte_stride => "byteStride" if Option::is_none;
    target => "target" if Option::is_none;
    name => "name" if Option::is_none;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfBuffer {
    byte_length => "byteLength" if |_| false;
    uri => "uri" if Option::is_none;
    name => "name" if Option::is_none;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfTextureInfo {
    index => "index" if |_| false;
    tex_coord => "texCoord" if is_zero_u64;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfNormalTextureInfo {
    index => "index" if |_| false;
    tex_coord => "texCoord" if is_zero_u64;
    scale => "scale" if is_one_f64;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfOcclusionTextureInfo {
    index => "index" if |_| false;
    tex_coord => "texCoord" if is_zero_u64;
    strength => "strength" if is_one_f64;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfPbrMetallicRoughness {
    base_color_factor => "baseColorFactor" if is_vec4_one;
    base_color_texture => "baseColorTexture" if Option::is_none;
    metallic_factor => "metallicFactor" if is_one_f64;
    roughness_factor => "roughnessFactor" if is_one_f64;
    metallic_roughness_texture => "metallicRoughnessTexture" if Option::is_none;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfMaterial {
    name => "name" if Option::is_none;
    pbr_metallic_roughness => "pbrMetallicRoughness" if Option::is_none;
    normal_texture => "normalTexture" if Option::is_none;
    occlusion_texture => "occlusionTexture" if Option::is_none;
    emissive_texture => "emissiveTexture" if Option::is_none;
    emissive_factor => "emissiveFactor" if is_vec3_zero;
    alpha_mode => "alphaMode" if is_opaque;
    alpha_cutoff => "alphaCutoff" if is_default_alpha_cutoff;
    double_sided => "doubleSided" if is_false;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfTexture {
    sampler => "sampler" if Option::is_none;
    source => "source" if Option::is_none;
    name => "name" if Option::is_none;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfImage {
    uri => "uri" if Option::is_none;
    mime_type => "mimeType" if Option::is_none;
    buffer_view => "bufferView" if Option::is_none;
    name => "name" if Option::is_none;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfSampler {
    mag_filter => "magFilter" if Option::is_none;
    min_filter => "minFilter" if Option::is_none;
    wrap_s => "wrapS" if is_default_wrap;
    wrap_t => "wrapT" if is_default_wrap;
    name => "name" if Option::is_none;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfSkin {
    inverse_bind_matrices => "inverseBindMatrices" if Option::is_none;
    skeleton => "skeleton" if Option::is_none;
    joints => "joints" if Vec::is_empty;
    name => "name" if Option::is_none;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfAnimationChannelTarget {
    node => "node" if Option::is_none;
    path => "path" if |_| false;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfAnimationChannel {
    sampler => "sampler" if |_| false;
    target => "target" if |_| false;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfAnimationSampler {
    input => "input" if |_| false;
    interpolation => "interpolation" if is_linear;
    output => "output" if |_| false;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfAnimation {
    channels => "channels" if Vec::is_empty;
    samplers => "samplers" if Vec::is_empty;
    name => "name" if Option::is_none;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfOrthographic {
    xmag => "xmag" if |_| false;
    ymag => "ymag" if |_| false;
    zfar => "zfar" if |_| false;
    znear => "znear" if |_| false;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfPerspective {
    aspect_ratio => "aspectRatio" if Option::is_none;
    yfov => "yfov" if |_| false;
    zfar => "zfar" if Option::is_none;
    znear => "znear" if |_| false;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

canonical_record!(GltfDocument {
    asset => "asset" if |_| false;
    scene => "scene" if Option::is_none;
    scenes => "scenes" if Vec::is_empty;
    nodes => "nodes" if Vec::is_empty;
    meshes => "meshes" if Vec::is_empty;
    accessors => "accessors" if Vec::is_empty;
    buffer_views => "bufferViews" if Vec::is_empty;
    buffers => "buffers" if Vec::is_empty;
    materials => "materials" if Vec::is_empty;
    textures => "textures" if Vec::is_empty;
    images => "images" if Vec::is_empty;
    samplers => "samplers" if Vec::is_empty;
    skins => "skins" if Vec::is_empty;
    animations => "animations" if Vec::is_empty;
    cameras => "cameras" if Vec::is_empty;
    extensions_used => "extensionsUsed" if Vec::is_empty;
    extensions_required => "extensionsRequired" if Vec::is_empty;
    extensions => "extensions" if Option::is_none;
    extras => "extras" if Option::is_none;
});

impl Tree for GltfJson {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        Ok(match self {
            Self::Null => Node::Null,
            Self::Bool(value) => Node::Bool(*value),
            Self::Number(value) => Node::F64(*value),
            Self::String(value) => Node::String(value),
            Self::Array(values) => Node::Array(values.len()),
            Self::Object(members) => Node::Object(members.len()),
        })
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        match self {
            Self::Array(values) => values.get(ordinal).map(|value| value as &dyn Tree).ok_or_else(absent),
            Self::Object(members) => members.get(ordinal).map(|(_, value)| value as &dyn Tree).ok_or_else(absent),
            _ => Err(absent()),
        }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
        match self {
            Self::Object(members) => members.get(ordinal).map(|(name, _)| name.as_str().into()).ok_or_else(absent),
            _ => Err(absent()),
        }
    }
}
