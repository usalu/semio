//! 🧱️ Document records shared by the block artifact family.

//#region 🔖️Identity
/// 🪪️ The single kind definition a block document edits — name/label/variant/description/icon/unit
/// apply uniformly whether the document is a `NodeKind` (2d), `ObjectKind` (3d) or `PartKind` (5d).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockKindIdentity {
    pub id: String,
    pub name: String,
    pub label: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub variant: Option<String>,
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    pub description: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub icon: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub unit: Option<String>,
}
//#endregion 🔖️Identity

//#region 🔖️Metadata
/// 🏷️ One free-form key/value attribute on a kind (optionally naming the attribute definition it
/// instantiates).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockAttribute {
    pub key: String,
    pub value: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub definition: Option<String>,
}

/// 👤️ One author credited on a kind.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockAuthor {
    pub id: String,
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub email: Option<String>,
}

/// 🔗️ One allowed (or, unidirectional, one-way-allowed) compatibility pair between two handle/vortex/
/// grip kind ids — the `id` lets ops remove a specific row without re-keying on `(source, target)`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockCompatibilityRule {
    pub id: String,
    pub source: String,
    pub target: String,
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    pub bidirectional: bool,
}

/// 🧱️ One representation (mesh at a LOD/tag combination) a kind ships with — semio_compose_rs's "Representation".
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockRepresentation {
    pub id: String,
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub mesh_url: Option<String>,
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    pub tags: Vec<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default, skip_serializing_if = "Option::is_none"))]
    pub lod: Option<String>,
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    pub description: String,
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    #[dsl(table)]
    pub attributes: Vec<BlockAttribute>,
}
//#endregion 🔖️Metadata

//#region 🔖️Cameras
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockCamera2d {
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    pub x: f64,
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    pub y: f64,
    #[value(default = "block_one_f64")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default = "block_one_f64"))]
    pub zoom: f64,
}

impl Default for BlockCamera2d {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockCamera3d {
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[value(default = "block_one_f64")]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default = "block_one_f64"))]
    pub zoom: f64,
}

impl Default for BlockCamera3d {
    fn default() -> Self {
        Self { position: [0.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], zoom: 1.0 }
    }
}

fn block_one_f64() -> f64 {
    1.0
}
//#endregion 🔖️Cameras

//#region 🔖️Meta
/// 📝️ Free-text description carried alongside a block document (distinct from the kind's own
/// `BlockKindIdentity::description`, which describes the kind; this describes the editing session).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(any(test, feature = "test-serde"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "test-serde"), serde(rename_all = "camelCase"))]
pub struct BlockMeta {
    #[value(default)]
    #[cfg_attr(any(test, feature = "test-serde"), serde(default))]
    pub description: String,
}
//#endregion 🔖️Meta
