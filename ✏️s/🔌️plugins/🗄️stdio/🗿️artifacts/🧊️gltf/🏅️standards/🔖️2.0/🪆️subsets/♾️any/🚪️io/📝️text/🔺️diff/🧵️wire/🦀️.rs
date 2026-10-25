//! 🧵️ The text diff wire: per-element encoders and decoders the `GltfDiff` codec composes.

use crate::standards::v2_0::subsets::any::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v2_0::subsets::any::schema::snapshot::{GltfAccessorType, GltfComponentType};
use crate::standards::v2_0::subsets::any::schema::snapshot::{GltfAccessor, GltfAlphaMode, GltfAnimation, GltfAnimationChannel, GltfAnimationChannelTarget, GltfAnimationPath, GltfAnimationSampler, GltfAsset, GltfBuffer, GltfBufferView, GltfCamera, GltfCameraProjection, GltfImage, GltfInterpolation, GltfJson, GltfMaterial, GltfMesh, GltfMorphTarget, GltfNode, GltfNormalTextureInfo, GltfOcclusionTextureInfo, GltfOrthographic, GltfPbrMetallicRoughness, GltfPerspective, GltfPrimitive, GltfSampler, GltfScene, GltfSkin, GltfSourceForm, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues, GltfTexture, GltfTextureInfo};
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use protocol::MutationDiff;

#[path = "🧱️scalar-and-enum-codecs/🦀️.rs"]
mod scalar_and_enum_codecs;
pub use scalar_and_enum_codecs::*;

#[path = "🧊️scene-mesh-and-accessor-codecs/🦀️.rs"]
mod scene_mesh_and_accessor_codecs;
pub use scene_mesh_and_accessor_codecs::*;

#[path = "🎨️material-and-resource-codecs/🦀️.rs"]
mod material_and_resource_codecs;
pub use material_and_resource_codecs::*;

#[path = "🎬️animation-and-camera-codecs/🦀️.rs"]
mod animation_and_camera_codecs;
pub use animation_and_camera_codecs::*;

#[path = "🔺️delta-diff-and-print-codecs/🦀️.rs"]
mod delta_diff_and_print_codecs;
pub use delta_diff_and_print_codecs::*;
