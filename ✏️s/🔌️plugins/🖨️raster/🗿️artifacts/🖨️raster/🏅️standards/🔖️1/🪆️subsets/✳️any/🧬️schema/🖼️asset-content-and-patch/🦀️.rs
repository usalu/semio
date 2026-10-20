//! 🖼️ Image assets, pixel and mask content, adjustment parameters, layer patches, ids and cold retirement.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct RasterImageAsset {
    pub mime: String,
    #[dsl(base64)]
    #[value(with = "asset_data_base64")]
    pub data: Vec<u8>,
}

pub type RasterAssetChild = store::os_store::artifact_child::ArtifactChild<SemioImageSnapshot>;

fn mint_asset_child_handle(asset_id: &str, content: &[u8]) -> RasterAssetChild {
    let child_id = store::content_id("raster-asset", content);
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: format!("{asset_id}-image"), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

/// 🕸️ Deterministic content-addressed CHILD handle, hashed off the RAW `(mime, data)` bytes — the
/// fallback shape used only when the bytes can't be decoded into real `SemioImageSnapshot` content
/// (see `mint_raster_asset_child`), and by pure-codec tests that need SOME stable handle without
/// exercising the real png bridge. Prefer `mint_raster_asset_child` at every real call site.
pub fn image_asset_child_handle(asset_id: &str, asset: &RasterImageAsset) -> RasterAssetChild {
    mint_asset_child_handle(asset_id, &[asset.mime.as_bytes(), b"\x1f", &asset.data].concat())
}

/// 🕸️ Deterministic content-addressed CHILD handle, hashed off the composed child's own CANONICAL
/// content (`SemioImageSnapshot`'s real pack bytes) rather than the source encoding's raw bytes —
/// this is the handle `mint_raster_asset_child` actually persists whenever decode succeeds. Necessary
/// because two different (but pixel-identical) PNG byte streams — e.g. a hand-authored fixture vs.
/// this plugin's own re-encode of the SAME decoded content — are NOT byte-identical in general
/// (different encoders/compression settings), so hashing raw bytes would mint two different handles
/// for what is honestly the same image; hashing the canonical DECODED content instead makes
/// `decode → cache → re-encode → decode` idempotent at the handle level, which `add-layer-asset`'s
/// inverse (`🧬️mutations/🖇️add-layer-asset/↩️inverse`) depends on to restore the exact prior handle.
pub fn image_content_child_handle(asset_id: &str, image: &SemioImageSnapshot) -> RasterAssetChild {
    let child_id = raster_image_content_id("raster-asset", image);
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: format!("{asset_id}-image"), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

/// 🪆 Carries one asset child's immutable materialization onto another handle for the SAME asset —
/// the single move every fixed-capacity, field-by-field rebuild of a handle goes through (the
/// retained `add-layer-asset` apply and the snapshot clone authority in `🧬️mutations/💾️binary`).
/// Those authorities cannot `Clone` a child (they rebuild each string into exact-capacity storage),
/// and a rebuilt handle without its owner is a handle whose pixels are gone: `raster_asset` returns
/// `None`, `assets_json` renders `{}` and the composite surface stays empty (react boot of ticket
/// 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP). A source without materialized content leaves the
/// target untouched rather than clearing it.
pub fn adopt_raster_asset_owner(source: &RasterAssetChild, target: &mut RasterAssetChild) {
    if let Some(owner) = source.local_owner::<SemioImageSnapshot>() {
        target.set_local_owner(owner);
    }
}

/// 🖼️ Nullable pixel attachment and display extent, preserved exactly by undo.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct RasterPixelContent {
    pub image_key: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// 🎭️ Explicit mask replacement, including removal.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct RasterMaskContent {
    #[dsl(block)]
    pub mask: Option<RasterLayerMask>,
}

/// 🔢️ Bounded tone number retaining integer/decimal identity for exact history.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RasterAdjustmentNumber(semio_framework_value::Number);

impl RasterAdjustmentNumber {
    pub fn decimal(value:f64)->Self {Self(semio_framework_value::Number::Float(value))}
    pub fn get(self)->f64 {self.0.as_f64()}
    pub fn from_parameter(value:&semio_framework_value::DslValue)->Option<Self> {if let semio_framework_value::DslValue::Number(number)=value {Some(Self(*number))} else {None}}
    pub fn literal(self)->semio_framework_value::DslValue {semio_framework_value::DslValue::Number(self.0)}
    pub fn digest(self)->[u8;9] {
        let mut result=[0;9];
        let (tag,bytes)=match self.0 {semio_framework_value::Number::UInt(value)=>(1,value.to_be_bytes()),semio_framework_value::Number::Int(value)=>(2,value.to_be_bytes()),semio_framework_value::Number::Float(value)=>(3,value.to_bits().to_be_bytes())};
        result[0]=tag;result[1..].copy_from_slice(&bytes);result
    }
}

semio_framework_value::artifact_retire_leaf!(RasterAdjustmentNumber);

impl semio_framework_pack_json::ArtifactCanonicalJsonTree for RasterAdjustmentNumber {
    fn canonical_tree_node(&self)->Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>,semio_framework_value::ValueError> {
        Ok(match self.0 {
            semio_framework_value::Number::UInt(value)=>semio_framework_pack_json::ArtifactCanonicalJsonNode::U64(value),
            semio_framework_value::Number::Int(value)=>semio_framework_pack_json::ArtifactCanonicalJsonNode::I64(value),
            semio_framework_value::Number::Float(value)=>semio_framework_pack_json::ArtifactCanonicalJsonNode::F64(value),
        })
    }
}

impl semio_framework_value::ToValue for RasterAdjustmentNumber {fn to_value(&self)->semio_framework_value::DslValue {self.literal()}}

impl semio_framework_value::FromValue for RasterAdjustmentNumber {
    fn from_value(value:semio_framework_value::DslValue)->Result<Self,semio_framework_value::ValueError> {
        Self::from_parameter(&value).filter(|n|n.get().is_finite()&&(-1.0..=1.0).contains(&n.get())).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "adjustment number must be within -1 and 1"))
    }
}

impl semio_framework_dsl_record::DslField for RasterAdjustmentNumber {
    fn shape()->semio_framework_dsl_record::Shape {semio_framework_dsl_record::Shape::Value}
    fn to_value(&self)->semio_framework_dsl_record::FieldValue {semio_framework_dsl_record::FieldValue::Value(self.literal())}
    fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String> {
        let semio_framework_dsl_record::FieldValue::Value(value)=value else {return Err("expected adjustment number".into());};
        <Self as semio_framework_value::FromValue>::from_value(value.clone()).map_err(|error|error.to_string())
    }
}

/// 🎚️ One parameter replacement; an absent value restores its implicit default.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct RasterAdjustmentParameter {pub parameter:String,pub value:Option<RasterAdjustmentNumber>}

/// 🩹️ Sparse patch applied to a single `RasterLayerNode` — the `PatchLayer` operation's payload, and
/// (with fields swapped for their prior values) its own mechanical inverse.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerPatch {
    #[dsl(block)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub adjustment_parameters: Option<Vec<RasterAdjustmentParameter>>,
    #[dsl(block)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub mask_content: Option<RasterMaskContent>,
    #[dsl(block)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub pixel_content: Option<RasterPixelContent>,
    #[dsl(block)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub transform: Option<RasterTransform>,
    pub name: Option<String>,
    pub visible: Option<bool>,
    pub locked: Option<bool>,
    pub opacity: Option<f32>,
    #[dsl(key = "blend")]
    pub blend_mode: Option<String>,
    #[dsl(key = "x")]
    pub transform_x: Option<f64>,
    #[dsl(key = "y")]
    pub transform_y: Option<f64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    #[dsl(key = "kind")]
    pub adjustment_kind: Option<String>,
}

/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// raster surface (editor and viewer) shares — lives at the ARTIFACT level, not under `editor`/
/// `viewer`, so a viewer file can read it without ever importing through the sibling `editor` module.
/// `artifact_kind` matches `definition()`'s own `"s.raster.schema.artifact"` row descriptor
/// (`"s.raster.raster"`); `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location.
pub const RASTER_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.raster.raster", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

/// 🆔️ Stable identity of decoded image fields, independent of every carrier encoding.
pub fn raster_image_content_id(prefix: &str, image: &SemioImageSnapshot) -> String {
    let mut hash = framework_hash::Sha256::new();
    let mut field = |bytes: &[u8]| {
        hash.update(&(bytes.len() as u64).to_le_bytes());
        hash.update(bytes);
    };
    field(image.schema.as_bytes());
    field(&image.width.to_le_bytes());
    field(&image.height.to_le_bytes());
    field(&[image.colorspace as u8, image.bit_depth]);
    field(&(image.frames.len() as u64).to_le_bytes());
    for frame in &image.frames {
        field(&frame.delay_ms.to_le_bytes());
        field(&frame.rgba8);
    }
    field(&[u8::from(image.icc.is_some())]);
    if let Some(icc) = &image.icc { field(icc); }
    field(&(image.metadata.len() as u64).to_le_bytes());
    for entry in &image.metadata { field(entry.key.as_bytes()); field(entry.value.as_bytes()); }
    format!("{prefix}-{}", framework_hash::hex_lower(&hash.finalize()[..8]))
}

/// 🪆️ Attaches decoded image content without reading or writing a physical representation.
pub fn mint_raster_image_child(asset_id: &str, image: &SemioImageSnapshot) -> RasterAssetChild {
    image_content_child_handle(asset_id, image).with_local_owner(std::sync::Arc::new(image.clone()))
}

/// 🖼️ Reads the decoded content owned by this exact snapshot child.
pub fn raster_image(assets: &RasterOwnedMap<RasterAssetChild>, asset_id: &str) -> Option<SemioImageSnapshot> {
    Some(assets.get(asset_id)?.local_owner::<SemioImageSnapshot>()?.as_ref().clone())
}

pub fn create_raster_id(prefix: &str) -> String {
    use std::hash::BuildHasher;
    use std::sync::{OnceLock,atomic::{AtomicU64,Ordering}};
    static NAMESPACE:OnceLock<[u64;2]>=OnceLock::new();
    static NEXT:AtomicU64=AtomicU64::new(0);
    let namespace=NAMESPACE.get_or_init(||{
        let state=std::collections::hash_map::RandomState::new();
        [state.hash_one(0_u8),state.hash_one(1_u8)]
    });
    let next=NEXT.try_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)).expect("Raster identity sequence exhausted");
    format!("{prefix}-{:016x}{:016x}{next:016x}",namespace[0],namespace[1])
}

impl<V:semio_framework_dsl_record::BorrowedDslField> semio_framework_dsl_record::BorrowedDslField for RasterOwnedMap<V>{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Map(semio_framework_dsl_record::borrowed_field_shape::<V>);}

impl semio_framework_dsl_record::BorrowedDslField for RasterAdjustmentNumber{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Value;}

pub const RASTER_OWNED_FIELD_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

/// 🧹️ Closes one cold-owned Raster value to terminal-empty through grants sized exactly by the retirement's own quotes.
pub fn retire_raster_value_cold<T: semio_framework_value::retirement::RetireOwned>(value: T) {
    let birth = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(), maximum_depth: 2, ..Default::default() };
    match semio_framework_value::retirement::admit_owned_retirement(value, birth) {
        Ok((owner, _)) => close_raster_owner_cold(owner),
        Err((error, _value)) => panic!("Raster cold retirement refused its frame: {error}"),
    }
}

/// 🧹️ Drives one admitted retirement frame to terminal-empty through grants sized exactly by its own quotes.
pub fn close_raster_owner_cold(mut owner: Box<dyn store::ErasedSnapshotRetirement>) {
    let mut turns = 0usize;
    while !owner.terminal_is_empty() {
        let demand = owner.next_demand(RASTER_OWNED_FIELD_BYTES).expect("Raster cold retirement quotes its next turn");
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: RASTER_OWNED_FIELD_BYTES.max(demand.copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        owner.close_step(grant).expect("Raster cold retirement closes within its quoted grant");
        turns += 1;
        assert!(turns < 1 << 24, "Raster cold retirement made no progress");
    }
}
