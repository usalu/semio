//! 🧬️ GltfArtifact schema — full artifact state.

use crate::schema::snapshot::{GltfAccessor, GltfBuffer, GltfBufferView, GltfDocument, GltfJson, GltfMesh, GltfPrimitive, GltfSourceForm};
use crate::{GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gltf")]
pub struct GltfArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub document: GltfDocument,
    #[state(artifact)]
    #[value(default)]
    pub buffers: Vec<Vec<u8>>,
    #[state(artifact)]
    #[value(default)]
    pub source_form: GltfSourceForm,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for GltfArtifact {
    fn default() -> Self {
        Self::from_snapshot(GltfSnapshot::default())
    }
}

impl GltfArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> GltfSnapshot {
        GltfSnapshot { schema: self.schema.clone(), document: self.document.clone(), buffers: self.buffers.clone(), source_form: self.source_form }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: GltfSnapshot) -> Self {
        Self { schema: snapshot.schema, document: snapshot.document, buffers: snapshot.buffers, source_form: snapshot.source_form }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: GltfSnapshot) {
        self.schema = snapshot.schema;
        self.document = snapshot.document;
        self.buffers = snapshot.buffers;
        self.source_form = snapshot.source_form;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gltf_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.gltf",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Dissolved out of `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — reached as
/// `crate::engine::empty_gltf_snapshot` through the `engine` barrel shim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_gltf_snapshot() -> GltfSnapshot {
    GltfSnapshot::default()
}

/// 🌳️ P2-FG3: a genuinely non-trivial persisted snapshot (one scene/node/mesh/accessor/material/
/// buffer PLUS one of every WEAK collection item — bufferView/texture/image/sampler/skin/
/// animation/camera — and populated `extensionsUsed`/`extras`) — used by this artifact's own
/// conformance-law tests AND by the shipped `.dsl.semio`/`.pack.semio` example fixtures (never a
/// bare fake like the pre-FG3 `{"hello":"stdio.gltf","n":1}` stub, `fixture_honesty_law`'s own
/// mandate). Mirrors `demo_json_snapshot`'s own role in json's pilot report.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_gltf_snapshot() -> GltfSnapshot {
    use crate::standards::v2_0::subsets::any::schema::snapshot::{GltfAccessorType, GltfComponentType};
    use crate::schema::snapshot::{
        GltfAnimation, GltfAnimationChannel, GltfAnimationChannelTarget, GltfAnimationPath, GltfAsset, GltfCamera, GltfCameraProjection, GltfImage, GltfInterpolation, GltfMaterial, GltfNode, GltfPbrMetallicRoughness, GltfPerspective, GltfSampler,
        GltfScene, GltfSkin, GltfTexture,
    };

    let document = GltfDocument {
        asset: GltfAsset { version: "2.0".into(), generator: Some("semio".into()), ..GltfAsset::default() },
        scene: Some(0),
        scenes: vec![GltfScene { nodes: vec![0], name: Some("root-scene".into()), ..GltfScene::default() }],
        nodes: vec![GltfNode { mesh: Some(0), camera: Some(0), name: Some("root-node".into()), ..GltfNode::default() }],
        meshes: vec![GltfMesh { primitives: vec![GltfPrimitive { attributes: vec![("POSITION".into(), 0)], indices: None, material: Some(0), mode: Some(4), targets: Vec::new(), extensions: None, extras: None }], ..GltfMesh::default() }],
        accessors: vec![GltfAccessor {
            buffer_view: Some(0),
            byte_offset: 0,
            component_type: GltfComponentType::Float,
            normalized: false,
            count: 3,
            kind: GltfAccessorType::Vec3,
            max: Some(vec![1.0, 1.0, 1.0]),
            min: Some(vec![0.0, 0.0, 0.0]),
            sparse: None,
            name: Some("positions".into()),
            extensions: None,
            extras: None,
        }],
        buffer_views: vec![GltfBufferView { buffer: 0, byte_offset: 0, byte_length: 36, byte_stride: None, target: Some(34962), name: None, extensions: None, extras: None }],
        // A real `data:` URI (not `None`) -- `None` would round-trip LOSSY through the TEXT
        // facet specifically (`serialize_gltf_document` embeds any uri-less buffer as a data URI
        // on print, so re-parsing would fabricate a `Some(..)` that never matches this demo's own
        // `None`, a real asymmetry discovered by `fixture_honesty_law` -- setting a genuine data
        // URI up front keeps BOTH the text (`parse_gltf_document`) and GLB (`decode_glb`, which
        // never embeds when a buffer already declares a `uri`) facets byte-for-byte lossless.
        buffers: vec![GltfBuffer { byte_length: 36, uri: Some("data:application/octet-stream;base64,AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".into()), name: Some("geometry".into()), extensions: None, extras: None }],
        materials: vec![GltfMaterial {
            name: Some("triangle-material".into()),
            pbr_metallic_roughness: Some(GltfPbrMetallicRoughness { base_color_factor: [1.0, 0.0, 0.0, 1.0], metallic_factor: 0.0, roughness_factor: 0.8, ..GltfPbrMetallicRoughness::default() }),
            ..GltfMaterial::default()
        }],
        textures: vec![GltfTexture { sampler: Some(0), source: Some(0), name: None, extensions: None, extras: None }],
        images: vec![GltfImage { uri: Some("texture.png".into()), ..GltfImage::default() }],
        samplers: vec![GltfSampler::default()],
        skins: vec![GltfSkin { joints: vec![0], name: Some("root-skin".into()), ..GltfSkin::default() }],
        animations: vec![GltfAnimation {
            channels: vec![GltfAnimationChannel { sampler: 0, target: GltfAnimationChannelTarget { node: Some(0), path: GltfAnimationPath::Translation, extensions: None, extras: None }, extensions: None, extras: None }],
            samplers: vec![crate::schema::snapshot::GltfAnimationSampler { input: 0, interpolation: GltfInterpolation::Linear, output: 0, extensions: None, extras: None }],
            name: Some("spin".into()),
            extensions: None,
            extras: None,
        }],
        cameras: vec![GltfCamera {
            projection: GltfCameraProjection::Perspective(GltfPerspective { aspect_ratio: Some(1.777), yfov: 0.8, zfar: Some(100.0), znear: 0.1, extensions: None, extras: None }),
            name: Some("main-camera".into()),
            extensions: None,
            extras: None,
        }],
        extensions_used: vec!["KHR_materials_unlit".into()],
        extensions_required: Vec::new(),
        extensions: None,
        extras: Some(GltfJson::Object(vec![("generator-note".into(), GltfJson::String("fg3 demo fixture".into()))])),
    };
    GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers: vec![vec![0u8; 36]], source_form: GltfSourceForm::Json }
}
//#endregion 🔖️DocumentHelpers
