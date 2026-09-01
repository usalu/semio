//! 🖼️ Raster artifact — document entities (constitutional: general).

//#region 🔖️Constants
pub const RASTER_DOCUMENT_SCHEMA: &str = "raster.document";
pub const RASTER_OWNED_MAP_CAPACITY: usize = 64;
pub const RASTER_OWNED_MAP_PAGE_CAPACITY: usize = 8;
pub const RASTER_OWNED_MAP_PAGE_BACKING_BYTES: usize = 16 * 1024;
const RASTER_OWNED_MAP_PAGE_COUNT: usize = RASTER_OWNED_MAP_CAPACITY / RASTER_OWNED_MAP_PAGE_CAPACITY;
//#endregion 🔖️Constants

//#region 🗂️OwnedMap
struct RasterOwnedMapPage<V> {
    entries: [Option<(String, V)>; RASTER_OWNED_MAP_PAGE_CAPACITY],
}

impl<V> RasterOwnedMapPage<V> {
    fn new() -> Self {
        Self { entries: std::array::from_fn(|_| None) }
    }
}

pub(crate) struct RasterOwnedMapPageBacking<V> {
    page: Option<Box<RasterOwnedMapPage<V>>>,
}

impl<V> RasterOwnedMapPageBacking<V> {
    /// 🧮 Returns the conservative admitted credit for one owned page, independent of allocator layout.
    pub(crate) fn conservative_credit_bytes(&self) -> usize {
        RASTER_OWNED_MAP_PAGE_BACKING_BYTES
    }

    pub(crate) fn release(mut self) {
        drop(self.page.take());
    }
}

impl<V> Drop for RasterOwnedMapPageBacking<V> {
    fn drop(&mut self) {
        assert!(self.page.is_none(), "Raster owned map page backing reached Drop before exact release");
    }
}

#[derive(Debug)]
pub struct RasterOwnedMapRejected<V> {
    pub key: String,
    pub value: V,
    pub reason: &'static str,
}

pub struct RasterOwnedMapEntry<V> {
    owner: std::mem::ManuallyDrop<Option<(String, V)>>,
}

impl<V> RasterOwnedMapEntry<V> {
    fn new(key: String, value: V) -> Self {
        Self { owner: std::mem::ManuallyDrop::new(Some((key, value))) }
    }

    pub fn take(&mut self) -> (String, V) {
        self.owner.take().expect("Raster owned map entry remains available exactly once")
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.owner.is_none()
    }
}

impl<V> Drop for RasterOwnedMapEntry<V> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Raster owned map entry reached Drop before exact pair handback");
    }
}

pub(crate) enum RasterOwnedMapInsert<V> {
    Inserted,
    Replaced(RasterOwnedMapEntry<V>),
}

pub struct RasterOwnedMap<V> {
    pages: std::mem::ManuallyDrop<[Option<Box<RasterOwnedMapPage<V>>>; RASTER_OWNED_MAP_PAGE_COUNT]>,
    order: [u8; RASTER_OWNED_MAP_CAPACITY],
    length: usize,
}

impl<V> RasterOwnedMap<V> {
    pub fn new() -> Self {
        Self { pages: std::mem::ManuallyDrop::new(std::array::from_fn(|_| None)), order: [0; RASTER_OWNED_MAP_CAPACITY], length: 0 }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub(crate) fn allocated_page_count(&self) -> usize {
        self.pages.iter().filter(|page| page.is_some()).count()
    }

    /// 🧮 Returns the conservative admitted credit for one page, not an allocator-size observation.
    pub(crate) fn conservative_page_credit_bytes() -> usize {
        RASTER_OWNED_MAP_PAGE_BACKING_BYTES
    }

    fn entry(&self, slot: usize) -> Option<&(String, V)> {
        self.pages.get(slot / RASTER_OWNED_MAP_PAGE_CAPACITY)?.as_ref()?.entries.get(slot % RASTER_OWNED_MAP_PAGE_CAPACITY)?.as_ref()
    }

    fn entry_mut(&mut self, slot: usize) -> Option<&mut (String, V)> {
        self.pages.get_mut(slot / RASTER_OWNED_MAP_PAGE_CAPACITY)?.as_mut()?.entries.get_mut(slot % RASTER_OWNED_MAP_PAGE_CAPACITY)?.as_mut()
    }

    fn ordered_slot(&self, index: usize) -> Option<usize> {
        (index < self.length).then(|| self.order[index] as usize)
    }

    pub(crate) fn entry_at(&self, index: usize) -> Option<(&String, &V)> {
        let slot = self.ordered_slot(index)?;
        let (key, value) = self.entry(slot)?;
        Some((key, value))
    }

    fn ordered_position(&self, key: &str) -> Result<usize, usize> {
        let mut left = 0;
        let mut right = self.length;
        while left < right {
            let middle = left + (right - left) / 2;
            let slot = self.order[middle] as usize;
            match self.entry(slot).expect("Raster owned map order addresses an occupied slot").0.as_str().cmp(key) {
                std::cmp::Ordering::Less => left = middle + 1,
                std::cmp::Ordering::Greater => right = middle,
                std::cmp::Ordering::Equal => return Ok(middle),
            }
        }
        Err(left)
    }

    fn free_slot(&self) -> Option<usize> {
        (0..RASTER_OWNED_MAP_CAPACITY).find(|slot| self.entry(*slot).is_none() && self.pages[*slot / RASTER_OWNED_MAP_PAGE_CAPACITY].is_some())
    }

    pub(crate) fn page_required_for_insert(&self, key: &str) -> bool {
        self.ordered_position(key).is_err() && self.free_slot().is_none() && self.length < RASTER_OWNED_MAP_CAPACITY
    }

    pub(crate) fn admit_one_page(&mut self) -> Result<(), &'static str> {
        if std::mem::size_of::<RasterOwnedMapPage<V>>() > RASTER_OWNED_MAP_PAGE_BACKING_BYTES {
            return Err("raster-map.page-backing-capacity");
        }
        let page = self.pages.iter_mut().find(|page| page.is_none()).ok_or("raster-map.page-capacity")?;
        *page = Some(Box::new(RasterOwnedMapPage::new()));
        Ok(())
    }

    pub(crate) fn insert_pre_admitted(&mut self, key: String, value: V) -> Result<RasterOwnedMapInsert<V>, RasterOwnedMapRejected<V>> {
        match self.ordered_position(&key) {
            Ok(position) => {
                let slot = self.order[position] as usize;
                let entry = self.entry_mut(slot).expect("Raster owned map replacement slot remains occupied");
                let (previous_key, previous_value) = std::mem::replace(entry, (key, value));
                Ok(RasterOwnedMapInsert::Replaced(RasterOwnedMapEntry::new(previous_key, previous_value)))
            }
            Err(position) => {
                if self.length >= RASTER_OWNED_MAP_CAPACITY {
                    return Err(RasterOwnedMapRejected { key, value, reason: "raster-map.item-capacity" });
                }
                let Some(slot) = self.free_slot() else {
                    return Err(RasterOwnedMapRejected { key, value, reason: "raster-map.page-not-admitted" });
                };
                for index in (position..self.length).rev() {
                    self.order[index + 1] = self.order[index];
                }
                self.pages[slot / RASTER_OWNED_MAP_PAGE_CAPACITY].as_mut().expect("Raster owned map page is admitted").entries[slot % RASTER_OWNED_MAP_PAGE_CAPACITY] = Some((key, value));
                self.order[position] = slot as u8;
                self.length += 1;
                Ok(RasterOwnedMapInsert::Inserted)
            }
        }
    }

    pub fn insert(&mut self, key: String, value: V) -> Result<(), RasterOwnedMapRejected<V>> {
        if self.contains_key(&key) {
            return Err(RasterOwnedMapRejected { key, value, reason: "raster-map.duplicate-key" });
        }
        if self.page_required_for_insert(&key) {
            if self.admit_one_page().is_err() {
                return Err(RasterOwnedMapRejected { key, value, reason: "raster-map.page-capacity" });
            }
        }
        match self.insert_pre_admitted(key, value)? {
            RasterOwnedMapInsert::Inserted => Ok(()),
            RasterOwnedMapInsert::Replaced(_) => unreachable!("Raster unique insert cannot replace an occupied key"),
        }
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        let position = self.ordered_position(key).ok()?;
        Some(&self.entry(self.order[position] as usize)?.1)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        let position = self.ordered_position(key).ok()?;
        Some(&mut self.entry_mut(self.order[position] as usize)?.1)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.ordered_position(key).is_ok()
    }

    pub fn remove_entry(&mut self, key: &str) -> Option<RasterOwnedMapEntry<V>> {
        let position = self.ordered_position(key).ok()?;
        let slot = self.order[position] as usize;
        let entry = self.pages[slot / RASTER_OWNED_MAP_PAGE_CAPACITY].as_mut()?.entries[slot % RASTER_OWNED_MAP_PAGE_CAPACITY].take()?;
        for index in position + 1..self.length {
            self.order[index - 1] = self.order[index];
        }
        self.length -= 1;
        Some(RasterOwnedMapEntry::new(entry.0, entry.1))
    }

    pub(crate) fn take_last_entry(&mut self) -> Option<(String, V)> {
        let slot = self.ordered_slot(self.length.checked_sub(1)?)?;
        let entry = self.pages[slot / RASTER_OWNED_MAP_PAGE_CAPACITY].as_mut()?.entries[slot % RASTER_OWNED_MAP_PAGE_CAPACITY].take()?;
        self.length -= 1;
        Some(entry)
    }

    pub(crate) fn take_empty_page_backing(&mut self) -> Option<RasterOwnedMapPageBacking<V>> {
        let page = self.pages.iter_mut().find(|page| page.as_ref().is_some_and(|page| page.entries.iter().all(Option::is_none)))?;
        Some(RasterOwnedMapPageBacking { page: page.take() })
    }

    pub fn iter(&self) -> RasterOwnedMapIter<'_, V> {
        RasterOwnedMapIter { map: self, index: 0 }
    }

    pub fn keys(&self) -> RasterOwnedMapKeys<'_, V> {
        RasterOwnedMapKeys { inner: self.iter() }
    }
}

impl<V> Default for RasterOwnedMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> Drop for RasterOwnedMap<V> {
    fn drop(&mut self) {
        assert!(self.length == 0 && self.pages.iter().all(Option::is_none), "Raster owned map reached Drop before every entry and page backing was explicitly retired");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.pages) };
    }
}

impl<V: Clone> Clone for RasterOwnedMap<V> {
    fn clone(&self) -> Self {
        assert!(self.is_empty(), "Populated Raster owned maps require the retained page clone authority");
        Self::new()
    }
}

impl<V: std::fmt::Debug> std::fmt::Debug for RasterOwnedMap<V> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_map().entries(self.iter()).finish()
    }
}

impl<V: PartialEq> PartialEq for RasterOwnedMap<V> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<V: Eq> Eq for RasterOwnedMap<V> {}

pub struct RasterOwnedMapIter<'a, V> {
    map: &'a RasterOwnedMap<V>,
    index: usize,
}

impl<'a, V> Iterator for RasterOwnedMapIter<'a, V> {
    type Item = (&'a String, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let slot = self.map.ordered_slot(self.index)?;
        self.index += 1;
        let (key, value) = self.map.entry(slot)?;
        Some((key, value))
    }
}

pub struct RasterOwnedMapKeys<'a, V> {
    inner: RasterOwnedMapIter<'a, V>,
}

impl<'a, V> Iterator for RasterOwnedMapKeys<'a, V> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(key, _)| key)
    }
}

impl<'a, V> IntoIterator for &'a RasterOwnedMap<V> {
    type Item = (&'a String, &'a V);
    type IntoIter = RasterOwnedMapIter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// 🛑 Keeps generic Raster serde derives compile-time independent from populated map serialization.
pub(crate) fn serialize_empty_owned_map<S: serde::Serializer, V>(map: &RasterOwnedMap<V>, serializer: S) -> Result<S::Ok, S::Error> {
    if !map.is_empty() {
        return Err(serde::ser::Error::custom("Populated Raster owned map serialization is forbidden; interactive production routes require the retained page output authority"));
    }
    use serde::ser::SerializeMap;
    serializer.serialize_map(Some(0))?.end()
}

impl<'de, V: serde::Deserialize<'de>> serde::Deserialize<'de> for RasterOwnedMap<V> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor<V>(std::marker::PhantomData<V>);

        impl<'de, V: serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<V> {
            type Value = RasterOwnedMap<V>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a bounded Raster string map")
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(self, _source: A) -> Result<Self::Value, A::Error> {
                Err(serde::de::Error::custom("Raster maps require the retained page decoder"))
            }
        }

        deserializer.deserialize_map(Visitor(std::marker::PhantomData))
    }
}

impl<V: dsl::DslField> dsl::DslField for RasterOwnedMap<V> {
    fn shape() -> dsl::Shape {
        dsl::Shape::Map(Box::new(V::shape()))
    }

    fn to_value(&self) -> dsl::FieldValue {
        assert!(self.is_empty(), "Populated Raster owned map DSL materialization is forbidden; interactive production routes require the retained page output authority");
        dsl::FieldValue::Map(Vec::new())
    }

    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let dsl::FieldValue::Map(entries) = value else { return Err(format!("expected Map, found {value:?}")) };
        if !entries.is_empty() {
            return Err("populated Raster maps require the retained page decoder".into());
        }
        Ok(Self::new())
    }
}
//#endregion 🗂️OwnedMap

//#region 🔖️Types
pub fn default_one() -> f64 {
    1.0
}

pub fn default_true() -> bool {
    true
}

/// 🎞️ Non-destructive raster document: a nested layer tree (pixel/group/adjustment) plus embedded
/// image assets. This is the authoritative projection shared by the wasm compositor bridge and the
/// `raster-plugin` `ArtifactApp`. Ephemeral tool/brush/selection/camera state lives in the app's
/// `RasterConfig`, never here.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RasterViewportSize {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RasterCamera {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "default_one")]
    pub zoom: f64,
}

impl Default for RasterCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

pub fn one_f32() -> f32 {
    1.0
}

pub fn default_blend() -> String {
    "normal".into()
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RasterTransform {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "default_one")]
    pub scale_x: f64,
    #[serde(default = "default_one")]
    pub scale_y: f64,
    #[dsl(angle = "deg")]
    #[serde(default)]
    pub rotation: f64,
}

impl Default for RasterTransform {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, scale_x: 1.0, scale_y: 1.0, rotation: 0.0 }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RasterLayerMask {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub linked: bool,
    #[serde(default)]
    pub invert: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RasterLayerNode {
    #[serde(rename = "pixel", rename_all = "camelCase")]
    Pixel {
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default = "one_f32")]
        opacity: f32,
        #[dsl(key = "blend")]
        #[serde(default = "default_blend")]
        blend_mode: String,
        #[dsl(block)]
        #[serde(default)]
        transform: RasterTransform,
        #[dsl(block)]
        mask: Option<RasterLayerMask>,
        width: Option<u32>,
        height: Option<u32>,
        #[dsl(key = "image")]
        image_key: Option<String>,
    },
    #[serde(rename = "group", rename_all = "camelCase")]
    Group {
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default = "one_f32")]
        opacity: f32,
        #[dsl(key = "blend")]
        #[serde(default = "default_blend")]
        blend_mode: String,
        #[dsl(block)]
        #[serde(default)]
        transform: RasterTransform,
        #[dsl(block)]
        mask: Option<RasterLayerMask>,
        #[dsl(statements, block)]
        children: Vec<RasterLayerNode>,
    },
    #[serde(rename = "adjustment", rename_all = "camelCase")]
    Adjustment {
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default = "one_f32")]
        opacity: f32,
        #[dsl(key = "blend")]
        #[serde(default = "default_blend")]
        blend_mode: String,
        #[dsl(block)]
        #[serde(default)]
        transform: RasterTransform,
        #[dsl(key = "kind")]
        adjustment_kind: String,
        #[serde(serialize_with = "crate::artifacts::raster::serialize_empty_owned_map")]
        #[serde(default)]
        params: RasterOwnedMap<dsl::DslValue>,
    },
}

mod asset_data_base64 {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&base64_codec::base64_standard_encode(bytes))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        let encoded = String::deserialize(deserializer)?;
        base64_codec::base64_standard_decode(encoded.as_bytes()).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RasterImageAsset {
    pub mime: String,
    #[serde(with = "asset_data_base64")]
    #[dsl(base64)]
    pub data: Vec<u8>,
}

/// 📸️ Persisted raster snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
//#endregion 🔖️Types

//#region 🧩️Composition
/// 🧩️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (design map §4: "raster→C:image layers
/// R:drawing"): a pixel layer's real image bytes used to live INLINE on `RasterSnapshot.assets:
/// BTreeMap<String, RasterImageAsset>` (a duplicated bytes-blob type, never `s.stdio.semio/v1/image`
/// itself). `assets` is now `BTreeMap<String, RasterAssetChild>` — one composed `s.stdio.semio.image`
/// CHILD per asset id, content-addressed, never embedded bytes. `image_key: Option<String>` on
/// `RasterLayerNode::Pixel` is UNCHANGED — it still addresses into this same id-keyed collection, only
/// the map's VALUE type changed from bytes to a handle. `drawing` (`SemioDrawingSnapshot`, used by
/// `🚪️io`'s SVG export/DWG import bridge) was checked and found to be ALREADY a pure, non-persisted IO
/// conversion — raster never owns/duplicates a `drawing` field, it only ever calls stdio's real
/// `SemioDrawingSnapshot`/`DrawNode` types directly at conversion time (`drawing_snapshot_from_raster`/
/// `drawing_snapshot_from_dwg`, `🚪️io/🦀️component.rs`). That already satisfies "consumes/reads drawing
/// content but doesn't own it" — no `ArtifactLink` was needed because there was no persisted/duplicated
/// drawing field to convert.
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;

pub type RasterAssetChild = store::ArtifactChild<SemioImageSnapshot>;

fn mint_asset_child_handle(asset_id: &str, content_hash: u64) -> RasterAssetChild {
    let child_id = format!("raster-asset-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "image".into() };
    let target = store::os_io::ArtifactRef { artifact_id: format!("{asset_id}-image"), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🕸️ Deterministic content-addressed CHILD handle, hashed off the RAW `(mime, data)` bytes — the
/// fallback shape used only when the bytes can't be decoded into real `SemioImageSnapshot` content
/// (see `mint_raster_asset_child`), and by pure-codec tests that need SOME stable handle without
/// exercising the real png bridge. Prefer `mint_raster_asset_child` at every real call site.
pub fn image_asset_child_handle(asset_id: &str, asset: &RasterImageAsset) -> RasterAssetChild {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    asset.mime.hash(&mut hasher);
    asset.data.hash(&mut hasher);
    mint_asset_child_handle(asset_id, hasher.finish())
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
fn image_content_child_handle(asset_id: &str, image: &SemioImageSnapshot) -> RasterAssetChild {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    <SemioImageSnapshot as store::ArtifactPack>::encode_pack(image).hash(&mut hasher);
    mint_asset_child_handle(asset_id, hasher.finish())
}

/// 🌉️ The single funnel-through "add real content" primitive: converts the real bytes into the
/// composed child's own real content (`SemioImageSnapshot`, via the real `🚪️io` png↔semio/image
/// bridge — never a stub), mints the CANONICAL content-addressed handle off that decoded content
/// (`image_content_child_handle`), and attaches immutable content to that exact child owner. An
/// unsupported mime or undecodable payload falls back to a raw-bytes handle without fabricating a
/// materialization. Every call site receives one self-contained child owner; no process-global cache
/// can leak content between snapshots or retain abandoned payloads.
pub fn mint_raster_asset_child(asset_id: &str, asset: &RasterImageAsset) -> RasterAssetChild {
    match crate::artifacts::raster::io::semio_image_snapshot_from_raster_asset(asset) {
        Ok(image) => {
            image_content_child_handle(asset_id, &image).with_local_owner(std::sync::Arc::new(image))
        }
        Err(_) => image_asset_child_handle(asset_id, asset),
    }
}

/// 🌉️ Resolves only content owned by the exact child in this snapshot. A wire-decoded child that
/// has not yet been materialized by the host resolver fails soft without consulting shared state.
pub fn raster_asset(assets: &RasterOwnedMap<RasterAssetChild>, asset_id: &str) -> Option<RasterImageAsset> {
    let handle = assets.get(asset_id)?;
    let image = handle.local_owner::<SemioImageSnapshot>()?;
    crate::artifacts::raster::io::raster_asset_from_semio_image_snapshot(image.as_ref()).ok()
}
//#endregion 🧩️Composition

//#region 🔖️Operations
/// 🩹️ Sparse patch applied to a single `RasterLayerNode` — the `PatchLayer` operation's payload, and
/// (with fields swapped for their prior values) its own mechanical inverse.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct RasterLayerPatch {
    pub name: Option<String>,
    pub visible: Option<bool>,
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
//#endregion 🔖️Operations

pub use crate::artifacts::raster::schema::diff::RasterDiff;
pub use crate::artifacts::raster::schema::mutations::RasterMutation;
pub use crate::artifacts::raster::schema::snapshot::RasterSnapshot;

//#region 🔖️Dialect
/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// raster surface (editor and viewer) shares — lives at the ARTIFACT level, not under `editor`/
/// `viewer`, so a viewer file can read it without ever importing through the sibling `editor` module.
/// `artifact_kind` matches `definition()`'s own `"s.raster.schema.artifact"` row descriptor
/// (`"s.raster.raster"`); `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location.
pub const RASTER_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.raster.raster", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🏷️ The `2d.raster` artifact kind — lifted out of `create_raster_app`'s `.artifact_kind(…)` call so
/// both the app manifest and (in the future) any other consumer can share one definition.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "2d.raster".into(),
        name: "2D Raster".into(),
        source_format: "raster.document".into(),
        component_kind: "raster".into(),
        dimension: "2d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Raster },
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.svg", "stdio.png"],
        import_stdio_kinds: vec!["stdio.svg", "stdio.png"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) —
/// replaces the old side-effecting `register()`, which called four different global registries
/// directly from a plugin `.setup()` callback. `crate::editor::raster::config::schema::
/// register_app_schema()` is the one exception, still called from `🖨️raster/🦀️component.rs`'s own
/// `.setup()`: it registers the `RasterPlayApp` CONFIG schema, an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) —
/// `register_app_schema_descriptor` is not in the W1 census's artifact-scoped function set.
/// Relocated from `⚙️engine/🦀️component.rs` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// reloc-g3): `⚙️engine` was removed from the taxonomy and `declaration()` describes the artifact,
/// not engine behaviour, so its home is the artifact root alongside `artifact_kind()`. The
/// `io_registry::entries()` call below is now re-qualified onto `subsets::any::io::io_registry`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): the whole `⚙️engine` file this
/// function moved out of has since been dissolved into `🧬️schema/`/`🚪️io/`/the app, per rule 5.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.raster.standard.v1", "standard", "1", &[], None),
        ("s.raster.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.raster.schema.artifact", "schema", "s.raster.raster", &[("schema", "s.raster.raster")], None),
        ("s.raster.inference.artifact", "inference", "s.raster.raster.inference", &[("schema", "s.raster.raster.inference")], None),
        ("s.raster.composer.native", "composer", "s.raster@1/*", &[("dialect", "s.raster@1/*")], None),
        ("s.raster.composer.format-1", "composer", "s.stdio.gif@87a/*", &[("dialect", "s.stdio.gif@87a/*")], None),
        ("s.raster.composer.format-2", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.raster.composer.format-3", "composer", "s.stdio.pdf@1.4/*", &[("dialect", "s.stdio.pdf@1.4/*")], None),
        ("s.raster.composer.format-4", "composer", "s.stdio.jpg@jfif-1.01/*", &[("dialect", "s.stdio.jpg@jfif-1.01/*")], None),
        ("s.raster.composer.format-5", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.raster.composer.format-6", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.raster.composer.format-7", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.raster.composer.format-8", "composer", "s.stdio.bmp@v3/*", &[("dialect", "s.stdio.bmp@v3/*")], None),
        ("s.raster.composer.format-9", "composer", "s.stdio.tiff@6.0/*", &[("dialect", "s.stdio.tiff@6.0/*")], None),
        ("s.raster.grammar.1", "grammar", "raster.document", &[("grammar", "raster.document")], None),
        ("s.raster.grammar.2", "grammar", "raster.op", &[("grammar", "raster.op")], None),
        ("s.raster.grammar.3", "grammar", "raster.document.diff", &[("grammar", "raster.document.diff")], None),
        ("s.raster.grammar.4", "grammar", "raster.pack", &[("grammar", "raster.pack")], None),
        ("s.raster.grammar.5", "grammar", "raster.spr", &[("grammar", "raster.spr")], None),
        ("s.raster.codec.document-1", "codec", "raster.document:raster", &[("codec", "raster.document"), ("extension", "raster")], None),
        ("s.raster.localization.en", "localization", "Raster", &[], Some(("en", "Raster"))),
        ("s.raster.localization.de", "localization", "Raster", &[], Some(("de", "Raster"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.raster")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::raster::schema::raster_artifact_schema_descriptor())
        .inferences([crate::artifacts::raster::schema::inferences::raster_artifact_inference_descriptor()])
        .composers(crate::artifacts::raster::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::app::EditorApp<crate::editor::raster::RasterPlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// 🗒️note's own `pilot_languages()` convention.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "raster.document",
                    extension: Some("raster"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::raster::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::raster::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::raster::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::raster::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("raster.document"),
                },
                dsl::LanguageSpec {
                    id: "raster.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::raster::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::raster::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::raster::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::raster::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("raster.op"),
                },
                dsl::LanguageSpec {
                    id: "raster.document.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::raster::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::raster::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("raster.document.diff"),
                },
                dsl::LanguageSpec {
                    id: "raster.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::raster::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::raster::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("raster.pack"),
                },
                dsl::LanguageSpec {
                    id: "raster.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::raster::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::raster::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("raster.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    trait RasterChildOwnerOracle {
        fn expected() -> serde_json::Value;
    }

    struct SerdeJsonRasterChildOwnerOracle;

    impl RasterChildOwnerOracle for SerdeJsonRasterChildOwnerOracle {
        fn expected() -> serde_json::Value {
            serde_json::from_str(include_str!("🧪️fixtures/🎯️child-owner-isolation.json")).expect("language-neutral Raster child-owner fixture")
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_kind_keeps_the_media_schema_matching_the_store_schema() {
        assert_eq!(artifact_kind().schema, RASTER_DOCUMENT_SCHEMA);
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_materialization_is_owned_by_the_exact_snapshot_child() {
        let content = SemioImageSnapshot::default();
        let owned = image_content_child_handle("isolated", &content).with_local_owner(std::sync::Arc::new(content));
        let wire = serde_json::to_vec(&owned).expect("Raster child wire identity");
        let reconstructed: RasterAssetChild = serde_json::from_slice(&wire).expect("Raster child wire roundtrip");
        let observed = serde_json::json!({
            "ownedHasMaterialization": owned.local_owner::<SemioImageSnapshot>().is_some(),
            "wireIdentityMatches": owned == reconstructed,
            "wireHasMaterialization": reconstructed.local_owner::<SemioImageSnapshot>().is_some(),
        });

        assert_eq!(observed, SerdeJsonRasterChildOwnerOracle::expected());
    }
}
//#endregion 🧪️Tests
