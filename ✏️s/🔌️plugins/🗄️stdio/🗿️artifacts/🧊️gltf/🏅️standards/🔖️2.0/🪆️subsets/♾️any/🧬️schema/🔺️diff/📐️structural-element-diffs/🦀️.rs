//! 📐️ The asset, scene, node, mesh and accessor element diffs.

use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfAccessorType, standards::v2_0::subsets::any::schema::snapshot::GltfComponentType};
use crate::{standards::v2_0::subsets::any::schema::snapshot::GltfAccessor, standards::v2_0::subsets::any::schema::snapshot::GltfAsset, standards::v2_0::subsets::any::schema::snapshot::GltfJson, standards::v2_0::subsets::any::schema::snapshot::GltfMesh, standards::v2_0::subsets::any::schema::snapshot::GltfNode, standards::v2_0::subsets::any::schema::snapshot::GltfScene, standards::v2_0::subsets::any::schema::snapshot::GltfSparseAccessor};
use crate::standards::v2_0::subsets::any::schema::diff::*;

//#region 🔖️AssetDiff
/// 🔺️ Sparse per-field diff for [`GltfAsset`] -- always present as a whole document (not a
/// collection item), so handled directly rather than through [`ItemDiff`].
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfAssetDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub generator: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub copyright: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub min_version: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl GltfAssetDiff {
    pub fn is_empty(&self) -> bool {
        self.version.is_none() && self.generator.is_none() && self.copyright.is_none() && self.min_version.is_none() && self.extensions.is_none() && self.extras.is_none()
    }
    pub fn apply(&self, base: &GltfAsset, _capability: protocol::ApplyCapability) -> GltfAsset {
        self.apply_fields(base)
    }
    pub fn apply_fields(&self, base: &GltfAsset) -> GltfAsset {
        let mut next = base.clone();
        if let Some(v) = &self.version {
            next.version = v.clone();
        }
        if let Some(v) = &self.generator {
            next.generator = v.clone();
        }
        if let Some(v) = &self.copyright {
            next.copyright = v.clone();
        }
        if let Some(v) = &self.min_version {
            next.min_version = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        next
    }
    pub fn inverse(&self, base: &GltfAsset) -> Self {
        Self {
            version: self.version.as_ref().map(|_| base.version.clone()),
            generator: self.generator.as_ref().map(|_| base.generator.clone()),
            copyright: self.copyright.as_ref().map(|_| base.copyright.clone()),
            min_version: self.min_version.as_ref().map(|_| base.min_version.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    pub fn absorb(&mut self, other: Self) {
        if other.version.is_some() {
            self.version = other.version;
        }
        if other.generator.is_some() {
            self.generator = other.generator;
        }
        if other.copyright.is_some() {
            self.copyright = other.copyright;
        }
        if other.min_version.is_some() {
            self.min_version = other.min_version;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️AssetDiff

//#region 🔖️SceneDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfSceneDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<GltfRefsDelta>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfScene> for GltfSceneDiff {
    fn apply(&self, base: &GltfScene, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfScene> {
        let mut next = base.clone();
        if let Some(v) = &self.nodes {
            next.nodes = unrefs(v.commit_onto(&refs(&base.nodes), capability).map_err(|error| error.under(["nodes"]))?);
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfScene) -> Self {
        Self {
            nodes: self.nodes.as_ref().map(|delta| delta.inverse(&refs(&base.nodes))),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        absorb_slot!(self.nodes, other.nodes);
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️SceneDiff

//#region 🔖️NodeDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfNodeDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<GltfRefsDelta>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub mesh: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub camera: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub skin: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub matrix: Option<Option<[f64; 16]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub translation: Option<Option<[f64; 3]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub rotation: Option<Option<[f64; 4]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scale: Option<Option<[f64; 3]>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfNode> for GltfNodeDiff {
    fn apply(&self, base: &GltfNode, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfNode> {
        let mut next = base.clone();
        if let Some(v) = &self.children {
            next.children = unrefs(v.commit_onto(&refs(&base.children), capability).map_err(|error| error.under(["children"]))?);
        }
        if let Some(v) = self.mesh {
            next.mesh = v;
        }
        if let Some(v) = self.camera {
            next.camera = v;
        }
        if let Some(v) = self.skin {
            next.skin = v;
        }
        if let Some(v) = self.matrix {
            next.matrix = v;
        }
        if let Some(v) = self.translation {
            next.translation = v;
        }
        if let Some(v) = self.rotation {
            next.rotation = v;
        }
        if let Some(v) = self.scale {
            next.scale = v;
        }
        if let Some(v) = &self.weights {
            next.weights = v.clone();
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfNode) -> Self {
        Self {
            children: self.children.as_ref().map(|delta| delta.inverse(&refs(&base.children))),
            mesh: self.mesh.map(|_| base.mesh),
            camera: self.camera.map(|_| base.camera),
            skin: self.skin.map(|_| base.skin),
            matrix: self.matrix.map(|_| base.matrix),
            translation: self.translation.map(|_| base.translation),
            rotation: self.rotation.map(|_| base.rotation),
            scale: self.scale.map(|_| base.scale),
            weights: self.weights.as_ref().map(|_| base.weights.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        absorb_slot!(self.children, other.children);
        if other.mesh.is_some() {
            self.mesh = other.mesh;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        if other.skin.is_some() {
            self.skin = other.skin;
        }
        if other.matrix.is_some() {
            self.matrix = other.matrix;
        }
        if other.translation.is_some() {
            self.translation = other.translation;
        }
        if other.rotation.is_some() {
            self.rotation = other.rotation;
        }
        if other.scale.is_some() {
            self.scale = other.scale;
        }
        if other.weights.is_some() {
            self.weights = other.weights;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️NodeDiff

//#region 🔖️MeshDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfMeshDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub primitives: Option<GltfPrimitivesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfMesh> for GltfMeshDiff {
    fn apply(&self, base: &GltfMesh, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfMesh> {
        let mut next = base.clone();
        if let Some(v) = &self.primitives {
            next.primitives = v.apply(&base.primitives, "primitives", capability)?;
        }
        if let Some(v) = &self.weights {
            next.weights = v.clone();
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfMesh) -> Self {
        Self {
            primitives: self.primitives.as_ref().map(|d| d.inverse(&base.primitives)),
            weights: self.weights.as_ref().map(|_| base.weights.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        match (&mut self.primitives, other.primitives) {
            (Some(mine), Some(theirs)) => mine.absorb(theirs),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
        if other.weights.is_some() {
            self.weights = other.weights;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️MeshDiff

//#region 🔖️AccessorDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfAccessorDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub buffer_view: Option<Option<usize>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub component_type: Option<GltfComponentType>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub normalized: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<GltfAccessorType>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub max: Option<Option<Vec<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub min: Option<Option<Vec<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub sparse: Option<Option<GltfSparseAccessor>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extensions: Option<Option<GltfJson>>,
    #[value(default, skip_serializing_if = "Option::is_none", with = "json_presence")]
    pub extras: Option<Option<GltfJson>>,
}

impl ItemDiff<GltfAccessor> for GltfAccessorDiff {
    fn apply(&self, base: &GltfAccessor, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GltfAccessor> {
        let mut next = base.clone();
        if let Some(v) = self.buffer_view {
            next.buffer_view = v;
        }
        if let Some(v) = self.byte_offset {
            next.byte_offset = v;
        }
        if let Some(v) = self.component_type {
            next.component_type = v;
        }
        if let Some(v) = self.normalized {
            next.normalized = v;
        }
        if let Some(v) = self.count {
            next.count = v;
        }
        if let Some(v) = self.kind {
            next.kind = v;
        }
        if let Some(v) = &self.max {
            next.max = v.clone();
        }
        if let Some(v) = &self.min {
            next.min = v.clone();
        }
        if let Some(v) = &self.sparse {
            next.sparse = v.clone();
        }
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(v) = &self.extensions {
            next.extensions = v.clone();
        }
        if let Some(v) = &self.extras {
            next.extras = v.clone();
        }
        Ok(next)
    }
    fn inverse(&self, base: &GltfAccessor) -> Self {
        Self {
            buffer_view: self.buffer_view.map(|_| base.buffer_view),
            byte_offset: self.byte_offset.map(|_| base.byte_offset),
            component_type: self.component_type.map(|_| base.component_type),
            normalized: self.normalized.map(|_| base.normalized),
            count: self.count.map(|_| base.count),
            kind: self.kind.map(|_| base.kind),
            max: self.max.as_ref().map(|_| base.max.clone()),
            min: self.min.as_ref().map(|_| base.min.clone()),
            sparse: self.sparse.as_ref().map(|_| base.sparse.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            extensions: self.extensions.as_ref().map(|_| base.extensions.clone()),
            extras: self.extras.as_ref().map(|_| base.extras.clone()),
        }
    }
    fn absorb_into(&mut self, other: Self) {
        if other.buffer_view.is_some() {
            self.buffer_view = other.buffer_view;
        }
        if other.byte_offset.is_some() {
            self.byte_offset = other.byte_offset;
        }
        if other.component_type.is_some() {
            self.component_type = other.component_type;
        }
        if other.normalized.is_some() {
            self.normalized = other.normalized;
        }
        if other.count.is_some() {
            self.count = other.count;
        }
        if other.kind.is_some() {
            self.kind = other.kind;
        }
        if other.max.is_some() {
            self.max = other.max;
        }
        if other.min.is_some() {
            self.min = other.min;
        }
        if other.sparse.is_some() {
            self.sparse = other.sparse;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.extensions.is_some() {
            self.extensions = other.extensions;
        }
        if other.extras.is_some() {
            self.extras = other.extras;
        }
    }
}
//#endregion 🔖️AccessorDiff
