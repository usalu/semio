//! 🧊️ The aggregate `GltfDiff` with its touched regions, validation, apply, inverse, absorb and demo cases.

use crate::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::snapshot::*;
use crate::standards::v2_0::subsets::any::schema::snapshot_root::GltfSnapshot;
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::{ColdDiffAlgebra, DiffAlgebra};
use protocol::{ColdMutationDiff, MutationDiff};

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.gltf`. No `snapshot: Option<GltfSnapshot>` full-replace slot anywhere.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gltf.diff")]
pub struct GltfDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<GltfAssetDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scene: Option<Option<usize>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scenes: Option<GltfScenesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<GltfNodesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub meshes: Option<GltfMeshesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub accessors: Option<GltfAccessorsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub buffer_views: Option<GltfBufferViewsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub buffers: Option<GltfBuffersDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub buffer_bytes: Option<GltfBufferBytesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub materials: Option<GltfMaterialsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub textures: Option<GltfTexturesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<GltfImagesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub samplers: Option<GltfSamplersDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub skins: Option<GltfSkinsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub animations: Option<GltfAnimationsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cameras: Option<GltfCamerasDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions_used: Option<GltfStringsDelta>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extensions_required: Option<GltfStringsDelta>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source_form: Option<GltfSourceForm>,
}

impl GltfDiff {
    pub fn is_empty_diff(&self) -> bool {
        self.asset.as_ref().is_none_or(GltfAssetDiff::is_empty)
            && self.scene.is_none()
            && self.scenes.as_ref().is_none_or(GltfScenesDiff::is_empty)
            && self.nodes.as_ref().is_none_or(GltfNodesDiff::is_empty)
            && self.meshes.as_ref().is_none_or(GltfMeshesDiff::is_empty)
            && self.accessors.as_ref().is_none_or(GltfAccessorsDiff::is_empty)
            && self.buffer_views.as_ref().is_none_or(GltfBufferViewsDiff::is_empty)
            && self.buffers.as_ref().is_none_or(GltfBuffersDiff::is_empty)
            && self.buffer_bytes.as_ref().is_none_or(GltfBufferBytesDiff::is_empty)
            && self.materials.as_ref().is_none_or(GltfMaterialsDiff::is_empty)
            && self.textures.as_ref().is_none_or(GltfTexturesDiff::is_empty)
            && self.images.as_ref().is_none_or(GltfImagesDiff::is_empty)
            && self.samplers.as_ref().is_none_or(GltfSamplersDiff::is_empty)
            && self.skins.as_ref().is_none_or(GltfSkinsDiff::is_empty)
            && self.animations.as_ref().is_none_or(GltfAnimationsDiff::is_empty)
            && self.cameras.as_ref().is_none_or(GltfCamerasDiff::is_empty)
            && self.extensions_used.as_ref().is_none_or(|delta| delta.is_empty())
            && self.extensions_required.as_ref().is_none_or(|delta| delta.is_empty())
            && self.extensions.is_none()
            && self.extras.is_none()
            && self.source_form.is_none()
    }
}

//#region 🗺️TouchedRegions
impl protocol::DiffRegions for GltfDiff {
    fn touches(&self) -> protocol::TouchedPaths {
        let mut paths = Vec::new();
        if self.asset.as_ref().is_some_and(|diff| !diff.is_empty()) {
            paths.push("document/asset".to_string());
        }
        if self.scene.is_some() {
            paths.push("document/scene".to_string());
        }
        macro_rules! collection_paths {
            ($field:ident, $name:literal) => {
                if let Some(diff) = &self.$field {
                    if !diff.removed.is_empty() || !diff.added.is_empty() {
                        paths.push(concat!("document/", $name).to_string());
                    } else {
                        paths.extend(diff.modified.iter().map(|entry| format!(concat!("document/", $name, "/{}"), entry.index)));
                    }
                }
            };
        }
        collection_paths!(scenes, "scenes");
        if let Some(diff) = &self.nodes {
            if !diff.removed.is_empty() || !diff.added.is_empty() {
                paths.push("document/nodes".to_string());
            } else {
                for entry in &diff.modified {
                    let root = format!("document/nodes/{}", entry.index);
                    if entry.diff.children.is_some() {
                        paths.push(format!("{root}/hierarchy"));
                    }
                    if entry.diff.matrix.is_some() || entry.diff.translation.is_some() || entry.diff.rotation.is_some() || entry.diff.scale.is_some() {
                        paths.push(format!("{root}/transform"));
                    }
                    if entry.diff.mesh.is_some() {
                        paths.push(format!("{root}/mesh"));
                    }
                    if entry.diff.skin.is_some() {
                        paths.push(format!("{root}/skin"));
                    }
                    if entry.diff.weights.is_some() {
                        paths.push(format!("{root}/weights"));
                    }
                    if entry.diff.camera.is_some() {
                        paths.push(format!("{root}/camera"));
                    }
                    if entry.diff.name.is_some() {
                        paths.push(format!("{root}/name"));
                    }
                    if entry.diff.extensions.is_some() {
                        paths.push(format!("{root}/extensions"));
                    }
                    if entry.diff.extras.is_some() {
                        paths.push(format!("{root}/extras"));
                    }
                }
            }
        }
        if let Some(diff) = &self.meshes {
            if !diff.removed.is_empty() || !diff.added.is_empty() {
                paths.push("document/meshes".to_string());
            } else {
                for entry in &diff.modified {
                    let root = format!("document/meshes/{}", entry.index);
                    if entry.diff.primitives.is_some() {
                        paths.push(format!("{root}/primitives"));
                    }
                    if entry.diff.weights.is_some() {
                        paths.push(format!("{root}/weights"));
                    }
                    if entry.diff.name.is_some() {
                        paths.push(format!("{root}/name"));
                    }
                    if entry.diff.extensions.is_some() {
                        paths.push(format!("{root}/extensions"));
                    }
                    if entry.diff.extras.is_some() {
                        paths.push(format!("{root}/extras"));
                    }
                }
            }
        }
        collection_paths!(accessors, "accessors");
        collection_paths!(buffer_views, "bufferViews");
        collection_paths!(buffers, "buffers");
        if let Some(diff) = &self.buffer_bytes {
            if !diff.removed.is_empty() || !diff.added.is_empty() {
                paths.push("buffers".to_string());
            } else {
                paths.extend(diff.modified.iter().map(|entry| format!("buffers/{}", entry.index)));
            }
        }
        collection_paths!(materials, "materials");
        collection_paths!(textures, "textures");
        collection_paths!(images, "images");
        collection_paths!(samplers, "samplers");
        collection_paths!(skins, "skins");
        collection_paths!(animations, "animations");
        collection_paths!(cameras, "cameras");
        if self.extensions_used.is_some() {
            paths.push("document/extensionsUsed".to_string());
        }
        if self.extensions_required.is_some() {
            paths.push("document/extensionsRequired".to_string());
        }
        if self.extensions.is_some() {
            paths.push("document/extensions".to_string());
        }
        if self.extras.is_some() {
            paths.push("document/extras".to_string());
        }
        if self.source_form.is_some() {
            paths.push("sourceForm".to_string());
        }
        paths.sort();
        paths.dedup();
        protocol::TouchedPaths { paths }
    }
}
//#endregion 🗺️TouchedRegions

impl MutationDiff<GltfSnapshot> for GltfDiff {}

impl protocol::ColdMutationDiff<GltfSnapshot> for GltfDiff {
    fn absorb(&mut self, other: Self) {
        match (&mut self.asset, other.asset) {
            (Some(mine), Some(theirs)) => mine.absorb(theirs),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
        if other.scene.is_some() {
            self.scene = other.scene;
        }
        macro_rules! absorb_collection {
            ($field:ident) => {
                match (&mut self.$field, other.$field) {
                    (Some(mine), Some(theirs)) => mine.absorb(theirs),
                    (slot @ None, Some(theirs)) => *slot = Some(theirs),
                    _ => {}
                }
            };
        }
        absorb_collection!(scenes);
        absorb_collection!(nodes);
        absorb_collection!(meshes);
        absorb_collection!(accessors);
        absorb_collection!(buffer_views);
        absorb_collection!(buffers);
        absorb_collection!(buffer_bytes);
        absorb_collection!(materials);
        absorb_collection!(textures);
        absorb_collection!(images);
        absorb_collection!(samplers);
        absorb_collection!(skins);
        absorb_collection!(animations);
        absorb_collection!(cameras);
        absorb_collection!(extensions_used);
        absorb_collection!(extensions_required);
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
        if other.source_form.is_some() {
            self.source_form = other.source_form;
        }
    }

    fn apply(&self, unpublished: &mut GltfSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let applied = (|base: &GltfSnapshot| -> protocol::MutationApplyResult<GltfSnapshot> {
        let mut next = base.clone();
        let doc = &mut next.document;
        if let Some(d) = &self.asset {
            doc.asset = d.apply(&doc.asset, capability);
        }
        if let Some(v) = self.scene {
            doc.scene = v;
        }
        if let Some(d) = &self.scenes {
            doc.scenes = d.apply(&doc.scenes, "document/scenes", capability)?;
        }
        if let Some(d) = &self.nodes {
            doc.nodes = d.apply(&doc.nodes, "document/nodes", capability)?;
        }
        if let Some(d) = &self.meshes {
            doc.meshes = d.apply(&doc.meshes, "document/meshes", capability)?;
        }
        if let Some(d) = &self.accessors {
            doc.accessors = d.apply(&doc.accessors, "document/accessors", capability)?;
        }
        if let Some(d) = &self.buffer_views {
            doc.buffer_views = d.apply(&doc.buffer_views, "document/bufferViews", capability)?;
        }
        if let Some(d) = &self.buffers {
            doc.buffers = d.apply(&doc.buffers, "document/buffers", capability)?;
        }
        if let Some(d) = &self.buffer_bytes {
            next.buffers = d.apply(&next.buffers, "buffers", capability)?;
        }
        if let Some(d) = &self.materials {
            doc.materials = d.apply(&doc.materials, "document/materials", capability)?;
        }
        if let Some(d) = &self.textures {
            doc.textures = d.apply(&doc.textures, "document/textures", capability)?;
        }
        if let Some(d) = &self.images {
            doc.images = d.apply(&doc.images, "document/images", capability)?;
        }
        if let Some(d) = &self.samplers {
            doc.samplers = d.apply(&doc.samplers, "document/samplers", capability)?;
        }
        if let Some(d) = &self.skins {
            doc.skins = d.apply(&doc.skins, "document/skins", capability)?;
        }
        if let Some(d) = &self.animations {
            doc.animations = d.apply(&doc.animations, "document/animations", capability)?;
        }
        if let Some(d) = &self.cameras {
            doc.cameras = d.apply(&doc.cameras, "document/cameras", capability)?;
        }
        if let Some(d) = &self.extensions_used {
            doc.extensions_used = d.commit_onto(&doc.extensions_used, capability).map_err(|error| error.under(["document/extensionsUsed"]))?;
        }
        if let Some(d) = &self.extensions_required {
            doc.extensions_required = d.commit_onto(&doc.extensions_required, capability).map_err(|error| error.under(["document/extensionsRequired"]))?;
        }
        if let Some(v) = &self.extensions {
            doc.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            doc.extras = v.clone();
        }
        if let Some(v) = self.source_form {
            next.source_form = v;
        }
        if let Some(scene) = next.document.scene {
            if scene >= next.document.scenes.len() {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-reference", format!("default scene index {scene} does not address a scene")).at(["document/scene"]));
            }
        }
        Ok(next)
    
        })(&*unpublished)?;
        *unpublished = applied;
        Ok(())
    }
}

impl DiffAlgebra<GltfSnapshot> for GltfDiff {
    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}

impl protocol::ColdDiffAlgebra<GltfSnapshot> for GltfDiff {
    fn inverse(&self, base: &GltfSnapshot) -> Self {
        let doc = &base.document;
        Self {
            asset: self.asset.as_ref().map(|d| d.inverse(&doc.asset)),
            scene: self.scene.map(|_| doc.scene),
            scenes: self.scenes.as_ref().map(|d| d.inverse(&doc.scenes)),
            nodes: self.nodes.as_ref().map(|d| d.inverse(&doc.nodes)),
            meshes: self.meshes.as_ref().map(|d| d.inverse(&doc.meshes)),
            accessors: self.accessors.as_ref().map(|d| d.inverse(&doc.accessors)),
            buffer_views: self.buffer_views.as_ref().map(|d| d.inverse(&doc.buffer_views)),
            buffers: self.buffers.as_ref().map(|d| d.inverse(&doc.buffers)),
            buffer_bytes: self.buffer_bytes.as_ref().map(|d| d.inverse(&base.buffers)),
            materials: self.materials.as_ref().map(|d| d.inverse(&doc.materials)),
            textures: self.textures.as_ref().map(|d| d.inverse(&doc.textures)),
            images: self.images.as_ref().map(|d| d.inverse(&doc.images)),
            samplers: self.samplers.as_ref().map(|d| d.inverse(&doc.samplers)),
            skins: self.skins.as_ref().map(|d| d.inverse(&doc.skins)),
            animations: self.animations.as_ref().map(|d| d.inverse(&doc.animations)),
            cameras: self.cameras.as_ref().map(|d| d.inverse(&doc.cameras)),
            extensions_used: self.extensions_used.as_ref().map(|delta| delta.inverse(&doc.extensions_used)),
            extensions_required: self.extensions_required.as_ref().map(|delta| delta.inverse(&doc.extensions_required)),
            extensions: self.extensions.as_ref().map(|_| doc.extensions.clone()),
            extras: self.extras.as_ref().map(|_| doc.extras.clone()),
            source_form: self.source_form.map(|_| base.source_form),
        }
    }
}

/// 🧪️ P2-FG3: representative `GltfDiff` cases — the empty (`None`-everywhere) diff PLUS one rich diff that names a row in every one
/// of `GltfDiff`'s top-level clauses and one positional list row under a scene, a node and the used-extension list, declared
/// row by row against the lengths of `demo_gltf_snapshot()` — used by this artifact's own
/// `diff_grammar_conformance_law`/`protocol_walk_law` conformance tests (⚙️engine/component.rs).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_diff_cases() -> Vec<GltfDiff> {
    let base = crate::standards::v2_0::subsets::any::schema::demo_gltf_snapshot();
    let document = &base.document;
    fn appended<T, D>(len: usize, item: T) -> GltfCollectionDiff<T, D> {
        GltfCollectionDiff { removed: Vec::new(), modified: Vec::new(), added: vec![GltfAdded { index: len, item }], amended: Vec::new() }
    }
    let rich = GltfDiff {
        asset: Some(GltfAssetDiff { generator: Some(Some("semio-fg3".into())), ..Default::default() }),
        scene: Some(Some(1)),
        scenes: Some(appended(document.scenes.len(), GltfScene { nodes: vec![], name: Some("second-scene".into()), ..Default::default() })),
        nodes: Some(GltfNodesDiff {
            modified: vec![GltfModified {
                index: 0,
                diff: GltfNodeDiff { name: Some(Some("renamed-node".into())), children: Some(GltfRefsDelta::insertion(0, GltfRef(0))), ..Default::default() },
            }],
            ..Default::default()
        }),
        meshes: Some(appended(document.meshes.len(), GltfMesh::default())),
        accessors: Some(GltfAccessorsDiff { modified: vec![GltfModified { index: 0, diff: GltfAccessorDiff { count: Some(6), ..Default::default() } }], ..Default::default() }),
        buffer_views: Some(appended(document.buffer_views.len(), GltfBufferView { buffer: 0, byte_offset: 0, byte_length: 12, byte_stride: None, target: None, name: None, extensions: None, extras: None })),
        buffers: Some(appended(document.buffers.len(), GltfBuffer { byte_length: 4, uri: None, name: Some("extra".into()), extensions: None, extras: None })),
        buffer_bytes: Some(appended(base.buffers.len(), vec![1, 2, 3, 4])),
        materials: Some(GltfMaterialsDiff { modified: vec![GltfModified { index: 0, diff: GltfMaterialDiff { double_sided: Some(true), ..Default::default() } }], ..Default::default() }),
        textures: Some(appended(document.textures.len(), GltfTexture { sampler: None, source: None, name: Some("tex2".into()), extensions: None, extras: None })),
        images: Some(appended(document.images.len(), GltfImage { uri: Some("second.png".into()), ..Default::default() })),
        samplers: Some(appended(document.samplers.len(), GltfSampler::default())),
        skins: Some(appended(document.skins.len(), GltfSkin { joints: vec![0], ..Default::default() })),
        animations: Some(appended(document.animations.len(), GltfAnimation::default())),
        cameras: Some(appended(
            document.cameras.len(),
            GltfCamera {
                projection: GltfCameraProjection::Orthographic(GltfOrthographic { xmag: 1.0, ymag: 1.0, zfar: 10.0, znear: 0.1, extensions: None, extras: None }),
                name: Some("ortho-cam".into()),
                extensions: None,
                extras: None,
            },
        )),
        extensions_used: Some(GltfStringsDelta::insertion(document.extensions_used.len(), "KHR_texture_transform".to_string())),
        extensions_required: Some(GltfStringsDelta::insertion(document.extensions_required.len(), "KHR_texture_transform".to_string())),
        extensions: Some(Some(GltfJson::Bool(true))),
        extras: Some(None),
        source_form: Some(GltfSourceForm::Glb),
    };
    vec![GltfDiff::default(), rich]
}

//#endregion 🔖️Diff
