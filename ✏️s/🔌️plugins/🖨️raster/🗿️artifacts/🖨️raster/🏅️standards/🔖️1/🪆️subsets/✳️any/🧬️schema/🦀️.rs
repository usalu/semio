//! 🧬️ Raster artifact schema — every field of the artifact with its state class.

use crate::{RasterAssetChild, RasterOwnedMap, RASTER_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ raster document artifact state.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.raster.raster")]
pub struct RasterArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    pub layers: Vec<RasterLayerNode>,
    #[state(artifact)]
    pub assets: RasterOwnedMap<RasterAssetChild>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for RasterArtifact {
    fn default() -> Self {
        Self { schema: RASTER_DOCUMENT_SCHEMA.into(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() }
    }
}

impl RasterArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> RasterSnapshot {
        RasterSnapshot { schema: self.schema.clone(), id: self.id.clone(), title: self.title.clone(), layers: self.layers.clone(), assets: self.assets.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: RasterSnapshot) -> Self {
        Self { schema: snapshot.schema, id: snapshot.id, title: snapshot.title, layers: snapshot.layers, assets: snapshot.assets }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: RasterSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.title = snapshot.title;
        self.layers = snapshot.layers;
        self.assets = snapshot.assets;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.raster.raster` — twenty handcrafted schema leaves.
pub fn raster_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.raster.raster",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"), typescript: include_str!("🔺️diff/🟦️.ts"), graphql: include_str!("🔺️diff/🔗️.graphql"), json_schema: include_str!("🔺️diff/🔣️.json"), proto: include_str!("🔺️diff/🛰️.proto")
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
/// 🌱️ Relocated verbatim from `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
/// rule 3: pure helpers over document types live in `🧬️schema/`). Every external call site now reads
/// `crate::standards::v1::subsets::any::schema::…` (the artifact root's own pre-existing `pub mod schema { pub
/// use super::standards::v1::subsets::any::schema::*; }` shim keeps that path resolving).
use crate::{RasterSnapshot, RasterTransform};




//#region 🔖️Tree
pub fn layer_node_id(layer: &RasterLayerNode) -> &str {
    match layer {
        RasterLayerNode::Pixel { id, .. } | RasterLayerNode::Group { id, .. } | RasterLayerNode::Adjustment { id, .. } => id,
    }
}

pub fn layer_name(layer: &RasterLayerNode) -> &str {
    match layer {
        RasterLayerNode::Pixel { name, .. } | RasterLayerNode::Group { name, .. } | RasterLayerNode::Adjustment { name, .. } => name,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all="camelCase")]
pub struct LayerProtection {pub locked:bool,pub inherited:bool,pub descendant:bool,pub editable:bool,pub structural:bool,pub can_change_lock:bool}
pub fn layer_locked(layer:&RasterLayerNode)->bool {match layer {RasterLayerNode::Pixel {locked,..}|RasterLayerNode::Group {locked,..}|RasterLayerNode::Adjustment {locked,..}=>*locked}}
pub fn layer_protection(layers:&[RasterLayerNode],id:&str)->Option<LayerProtection> {
    fn below(layers:&[RasterLayerNode])->bool {layers.iter().any(|layer|layer_locked(layer)||matches!(layer,RasterLayerNode::Group {children,..} if below(children)))}
    fn visit(layers:&[RasterLayerNode],id:&str,inherited:bool)->Option<LayerProtection> {
        for layer in layers {
            let locked=layer_locked(layer);
            if layer_node_id(layer)==id {let descendant=matches!(layer,RasterLayerNode::Group {children,..} if below(children));let editable=!locked&&!inherited;return Some(LayerProtection {locked,inherited,descendant,editable,structural:editable&&!descendant,can_change_lock:!inherited});}
            if let RasterLayerNode::Group {children,..}=layer {if let Some(found)=visit(children,id,inherited||locked){return Some(found);}}
        }
        None
    }
    visit(layers,id,false)
}

pub fn require_layer_edit(layers:&[RasterLayerNode],id:&str,structural:bool)->Result<(),&'static str> {
    let protection=layer_protection(layers,id).ok_or("raster-layer-not-found")?;
    if if structural {protection.structural}else{protection.editable} {Ok(())}else{Err("raster-layer-locked")}
}

pub fn layer_visible(layer: &RasterLayerNode) -> bool {
    match layer {
        RasterLayerNode::Pixel { visible, .. } | RasterLayerNode::Group { visible, .. } | RasterLayerNode::Adjustment { visible, .. } => *visible,
    }
}

pub fn layer_opacity(layer: &RasterLayerNode) -> f32 {
    match layer {
        RasterLayerNode::Pixel { opacity, .. } | RasterLayerNode::Group { opacity, .. } | RasterLayerNode::Adjustment { opacity, .. } => *opacity,
    }
}

pub fn layer_blend_mode(layer: &RasterLayerNode) -> &str {
    match layer {
        RasterLayerNode::Pixel { blend_mode, .. } | RasterLayerNode::Group { blend_mode, .. } | RasterLayerNode::Adjustment { blend_mode, .. } => blend_mode,
    }
}

pub fn layer_transform(layer: &RasterLayerNode) -> &RasterTransform {
    match layer {
        RasterLayerNode::Pixel { transform, .. } | RasterLayerNode::Group { transform, .. } | RasterLayerNode::Adjustment { transform, .. } => transform,
    }
}

pub fn find_layer<'a>(layers: &'a [RasterLayerNode], target_id: &str) -> Option<&'a RasterLayerNode> {
    for layer in layers {
        if layer_node_id(layer) == target_id {
            return Some(layer);
        }
        if let RasterLayerNode::Group { children, .. } = layer {
            if let Some(found) = find_layer(children, target_id) {
                return Some(found);
            }
        }
    }
    None
}

/// 🧭️ Finds a layer's parent-group id (`None` at the root) and its index among its siblings.
pub fn locate_layer(layers: &[RasterLayerNode], target_id: &str) -> Option<(Option<String>, usize)> {
    fn walk(layers: &[RasterLayerNode], parent: Option<&str>, target_id: &str) -> Option<(Option<String>, usize)> {
        for (index, layer) in layers.iter().enumerate() {
            if layer_node_id(layer) == target_id {
                return Some((parent.map(str::to_string), index));
            }
            if let RasterLayerNode::Group { id, children, .. } = layer {
                if let Some(found) = walk(children, Some(id), target_id) {
                    return Some(found);
                }
            }
        }
        None
    }
    walk(layers, None, target_id)
}

pub fn flatten_raster_layers(layers: &[RasterLayerNode]) -> Vec<&RasterLayerNode> {
    let mut out = Vec::new();
    fn visit<'a>(layers: &'a [RasterLayerNode], out: &mut Vec<&'a RasterLayerNode>) {
        for layer in layers {
            out.push(layer);
            if let RasterLayerNode::Group { children, .. } = layer {
                visit(children, out);
            }
        }
    }
    visit(layers, &mut out);
    out
}
//#endregion 🔖️Tree



fn create_group_layer() -> RasterLayerNode {
    RasterLayerNode::Group { id: create_raster_id("group"), name: "Group".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() }
}

fn create_adjustment_layer() -> RasterLayerNode {
    RasterLayerNode::Adjustment {
        id: create_raster_id("adjust"),
        name: "Adjustment".into(),
        visible: true, locked: false,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: RasterTransform::default(),
        adjustment_kind: "brightnessContrast".into(),
        params: RasterOwnedMap::new(),
    }
}

pub fn create_layer_of_kind(kind: &str) -> RasterLayerNode {
    match kind {
        "group" => create_group_layer(),
        "adjustment" => create_adjustment_layer(),
        _ => create_pixel_layer("Layer", 512, 512),
    }
}



#[cfg(test)]
#[path = "🧪️testing/🦀️.rs"]
mod testing;
#[cfg(test)]
pub use testing::raster_image_test_snapshot;



/// 📚️ The committed example document behind one registered example id, or `None` when the id is not
/// one this subset registers — the lookup `🎮️commands/🎬️set-active-example` resolves against.
pub fn raster_example_document(example_id: &str) -> Option<RasterSnapshot> {
    (example_id == crate::examples::art_raster_demo::ID).then(default_raster_document)
}

/// 📄️ Duplicates a layer subtree with freshly minted ids (a new document node, not an operation inverse).
pub fn clone_layer(layer: &RasterLayerNode) -> RasterLayerNode {
    match layer {
        RasterLayerNode::Pixel { name, visible, locked, opacity, blend_mode, transform, mask, width, height, image_key, .. } => RasterLayerNode::Pixel {
            id: create_raster_id("layer"),
            name: format!("{name} copy"),
            visible: *visible,
            locked: *locked,
            opacity: *opacity,
            blend_mode: blend_mode.clone(),
            transform: transform.clone(),
            mask: mask.clone(),
            width: *width,
            height: *height,
            image_key: image_key.clone(),
        },
        RasterLayerNode::Group { name, visible, locked, opacity, blend_mode, transform, mask, children, .. } => RasterLayerNode::Group {
            id: create_raster_id("group"),
            name: format!("{name} copy"),
            visible: *visible,
            locked: *locked,
            opacity: *opacity,
            blend_mode: blend_mode.clone(),
            transform: transform.clone(),
            mask: mask.clone(),
            children: children.iter().map(clone_layer).collect(),
        },
        RasterLayerNode::Adjustment { name, visible, locked, opacity, blend_mode, transform, adjustment_kind, params, .. } => RasterLayerNode::Adjustment {
            id: create_raster_id("adjust"),
            name: format!("{name} copy"),
            visible: *visible,
            locked: *locked,
            opacity: *opacity,
            blend_mode: blend_mode.clone(),
            transform: transform.clone(),
            adjustment_kind: adjustment_kind.clone(),
            params: params.clone(),
        },
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️boot-document/🦀️.rs"]
mod boot_document_tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::RasterImageAsset;
pub use crate::RasterLayerNode;
pub use crate::RasterViewportSize;
//#endregion 🔁️Re-exports

#[cfg(test)]
#[path="🧪️tests/🔒️protection/🦀️.rs"]
mod protection_tests;
