//! 🧬️ Raster artifact schema — every field of the artifact with its state class.

use base64::Engine as _;
use crate::artifacts::raster::{RasterImageAsset, RasterLayerNode, RasterViewportSize, RASTER_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full raster artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.raster.raster")]
pub struct RasterArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub id: String,
    #[state(persistent)] pub title: Option<String>,
    #[state(persistent)] pub layers: Vec<RasterLayerNode>,
    #[state(persistent)] pub assets: BTreeMap<String, RasterImageAsset>,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub brush_size: f64,
    #[state(local_ui)] pub brush_opacity: f64,
    #[state(local_ui)] pub composite_viewport: Option<RasterViewportSize>,
    #[state(local_ui)] pub camera_x: f64,
    #[state(local_ui)] pub camera_y: f64,
    #[state(local_ui)] pub camera_zoom: f64,
    #[state(local_ui)] pub locale: String,
    #[state(preview)] pub hovered_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for RasterArtifact {
    fn default() -> Self {
        Self {
            schema: RASTER_DOCUMENT_SCHEMA.into(),
            id: String::new(),
            title: None,
            layers: Vec::new(),
            assets: BTreeMap::new(),
            selected_ids: Vec::new(),
            active_utility_id: "selectMarquee".into(),
            brush_size: 24.0,
            brush_opacity: 1.0,
            composite_viewport: None,
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            locale: "en-US".into(),
            hovered_id: None,
        }
    }
}

impl RasterArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::raster::RasterSnapshot {
        crate::artifacts::raster::RasterSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            title: self.title.clone(),
            layers: self.layers.clone(),
            assets: self.assets.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::raster::RasterSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            title: snapshot.title,
            layers: snapshot.layers,
            assets: snapshot.assets,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::raster::RasterSnapshot) {
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
pub fn raster_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.raster.raster",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::raster::{RasterDiff, RasterMutation, RasterSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct RasterBuilderConstruction {
        snapshot: RasterSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for RasterBuilderConstruction {
        type Snapshot = RasterSnapshot;
        type Mutation = RasterMutation;
        type Diff = RasterDiff;
        fn empty() -> Self { Self { snapshot: RasterSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<RasterSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<RasterSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = crate::artifacts::raster::schema::mutations::apply_raster_mutation(&self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <RasterDiff as protocol::MutationDiff<RasterSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::raster::RasterSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct RasterParts {
        pub snapshot: Option<RasterSnapshot>,
    }

    pub struct RasterAnalyzerAnalysis;

    impl ArtifactAnalysis for RasterAnalyzerAnalysis {
        type Parts = RasterParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.raster", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = RasterParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <RasterSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <RasterSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec RasterBuilderFacets {
        construction: derived_construction::RasterBuilderConstruction,
        analysis: derived_analysis::RasterAnalyzerAnalysis,
        composition: super::super::io::derived_composition::RasterComposerComposition,
    }
    builder: RasterBuilder,
    analyzer: RasterAnalyzer,
    composer: RasterComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🌱️ Relocated verbatim from `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
/// rule 3: pure helpers over document types live in `🧬️schema/`). Every external call site now reads
/// `crate::artifacts::raster::schema::…` (the artifact root's own pre-existing `pub mod schema { pub
/// use super::standards::v1::subsets::any::schema::*; }` shim keeps that path resolving).
use crate::artifacts::raster::{RasterSnapshot, RasterTransform};

/// 📄️ The `semio` example document, handcrafted in the `.raster` DSL — {@link semio_example_document}/
/// {@link semio_example_json} are the only ways it should be consumed.
const SEMIO_RASTER_EXAMPLE_TEXT: &str = crate::artifacts::raster::dsl::SEMIO_RASTER_EXAMPLE_TEXT;

pub fn create_raster_id(prefix: &str) -> String {
    let next = {
        let hex = blake3::hash(concat!(file!(), line!()).as_bytes()).to_hex();
        u64::from_str_radix(&hex[..8], 16).unwrap_or(1)
    };
    format!("{prefix}-{next}")
}

pub fn empty_raster_snapshot() -> RasterSnapshot {
    RasterSnapshot { schema: RASTER_DOCUMENT_SCHEMA.into(), id: "raster".into(), title: Some("Untitled".into()), layers: Vec::new(), assets: BTreeMap::new() }
}

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

/// 🖼️ `pub` (not `fn` as it was inside `⚙️engine`, where crate-locality made privacy moot): now called
/// cross-module from `🚪️io/🦀️component.rs`'s `MediaImport` region (`raster_document_json_from_dwg`,
/// `raster_image_layer_and_asset`), which need a specific name/width/height rather than
/// `create_layer_of_kind`'s generic defaults.
pub fn create_pixel_layer(name: &str, width: u32, height: u32) -> RasterLayerNode {
    RasterLayerNode::Pixel { id: create_raster_id("layer"), name: name.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(width), height: Some(height), image_key: None }
}

fn create_group_layer() -> RasterLayerNode {
    RasterLayerNode::Group { id: create_raster_id("group"), name: "Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() }
}

fn create_adjustment_layer() -> RasterLayerNode {
    RasterLayerNode::Adjustment {
        id: create_raster_id("adjust"),
        name: "Adjustment".into(),
        visible: true,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: RasterTransform::default(),
        adjustment_kind: "brightnessContrast".into(),
        params: BTreeMap::new(),
    }
}

pub fn create_layer_of_kind(kind: &str) -> RasterLayerNode {
    match kind {
        "group" => create_group_layer(),
        "adjustment" => create_adjustment_layer(),
        _ => create_pixel_layer("Layer", 512, 512),
    }
}

pub fn empty_raster_document() -> RasterSnapshot {
    let mut document = empty_raster_snapshot();
    document.id = "empty".into();
    document.layers = vec![create_pixel_layer("Background", 512, 512)];
    document
}

pub fn semio_fixture_snapshot() -> RasterSnapshot {
    let mut assets = BTreeMap::new();
    assets.insert(
        "semio-emblem".into(),
        RasterImageAsset { mime: "image/png".into(), data: base64::engine::general_purpose::STANDARD.decode("iVBORw0KGgo=").unwrap_or_default() },
    );
    let mut params = BTreeMap::new();
    params.insert("brightness".into(), dsl::to_dsl_value(&serde_json::json!(0.12)).expect("dsl value"));
    params.insert("contrast".into(), dsl::to_dsl_value(&serde_json::json!(0.08)).expect("dsl value"));
    RasterSnapshot {
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        id: "semio-demo".into(),
        title: Some("Semio Raster Demo".into()),
        layers: vec![
            RasterLayerNode::Pixel {
                id: "backdrop".into(),
                name: "Backdrop".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: None,
                width: Some(1024),
                height: Some(1024),
                image_key: Some("semio-emblem".into()),
            },
            RasterLayerNode::Adjustment {
                id: "brighten".into(),
                name: "Brighten".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                adjustment_kind: "brightnessContrast".into(),
                params,
            },
        ],
        assets,
    }
}

/// 📄️ The `semio` example document used by the app manifest and tests.
pub fn semio_example_document() -> RasterSnapshot {
    semio_fixture_snapshot()
}

/// 📄️ JSON re-serialization of {@link semio_example_document}, for the framework-generic call sites that
/// contractually require JSON text (`App::example`'s manifest `document_json`) — out of scope to change,
/// since it is defined in `framework/plugin`.
pub fn semio_example_json() -> String {
    serde_json::to_string(&semio_example_document()).expect("serialize semio example document")
}

/// 📄️ Duplicates a layer subtree with freshly minted ids (a new document node, not an operation inverse).
pub fn clone_layer(layer: &RasterLayerNode) -> RasterLayerNode {
    match layer {
        RasterLayerNode::Pixel { name, visible, opacity, blend_mode, transform, mask, width, height, image_key, .. } => RasterLayerNode::Pixel {
            id: create_raster_id("layer"),
            name: format!("{name} copy"),
            visible: *visible,
            opacity: *opacity,
            blend_mode: blend_mode.clone(),
            transform: transform.clone(),
            mask: mask.clone(),
            width: *width,
            height: *height,
            image_key: image_key.clone(),
        },
        RasterLayerNode::Group { name, visible, opacity, blend_mode, transform, mask, children, .. } => RasterLayerNode::Group {
            id: create_raster_id("group"),
            name: format!("{name} copy"),
            visible: *visible,
            opacity: *opacity,
            blend_mode: blend_mode.clone(),
            transform: transform.clone(),
            mask: mask.clone(),
            children: children.iter().map(clone_layer).collect(),
        },
        RasterLayerNode::Adjustment { name, visible, opacity, blend_mode, transform, adjustment_kind, params, .. } => RasterLayerNode::Adjustment {
            id: create_raster_id("adjust"),
            name: format!("{name} copy"),
            visible: *visible,
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
mod tests {
    use super::*;

    #[test]
    fn imports_dwg_polyline_into_raster_document() {
        let mut drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(semio_s_plugin_stdio::artifacts::dwg::DwgEntity {
            layer,
            color: semio_s_plugin_stdio::artifacts::dwg::DwgColor::ByLayer,
            geometry: semio_s_plugin_stdio::artifacts::dwg::DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]], bulges: vec![0.0, 0.0, 0.0, 0.0] },
        });
        drawing.extmin = [0.0, 0.0, 0.0];
        drawing.extmax = [10.0, 10.0, 0.0];
        let value = crate::artifacts::raster::io::raster_document_json_from_dwg(&drawing).expect("dwg import");
        let document: RasterSnapshot = serde_json::from_value(value).expect("valid raster document");
        assert_eq!(document.layers.len(), 1);
        let RasterLayerNode::Pixel { image_key, .. } = &document.layers[0] else {
            panic!("expected pixel layer");
        };
        let asset_key = image_key.as_ref().expect("image key set");
        let asset = document.assets.get(asset_key).expect("asset present");
        assert_eq!(asset.mime, "image/png");
        assert!(!asset.data.is_empty());
    }

    #[test]
    fn imports_empty_dwg_into_blank_raster_document() {
        let drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing::default();
        let value = crate::artifacts::raster::io::raster_document_json_from_dwg(&drawing).expect("empty dwg import");
        let document: RasterSnapshot = serde_json::from_value(value).expect("valid raster document");
        assert_eq!(document.layers.len(), 1);
        let RasterLayerNode::Pixel { image_key, width, height, .. } = &document.layers[0] else {
            panic!("expected pixel layer");
        };
        assert_eq!(*width, Some(1));
        assert_eq!(*height, Some(1));
        let asset_key = image_key.as_ref().expect("image key set");
        let asset = document.assets.get(asset_key).expect("asset present");
        assert!(!asset.data.is_empty());
    }
}
//#endregion 🧪️Tests
