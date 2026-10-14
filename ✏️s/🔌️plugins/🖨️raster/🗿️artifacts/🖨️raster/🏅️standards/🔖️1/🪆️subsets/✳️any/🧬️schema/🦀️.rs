//! 🧬️ Raster artifact schema — every field of the artifact with its state class.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;


use protocol::{Mutation, OpBinary};
use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
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
use crate::snapshot::RasterSnapshot;


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
pub mod testing;
#[cfg(test)]
pub use testing::raster_image_test_snapshot;


/// 📚️ The committed example document behind one registered example id, or `None` when the id is not
/// one this subset registers — the lookup `🎮️commands/🎬️set-active-example` resolves against.


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
pub use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;


//#endregion 🔁️Re-exports

#[cfg(test)]
#[path="🧪️tests/🔒️protection/🦀️.rs"]
mod protection_tests;

/// 🖼️ A one-frame straight-alpha RGBA8 image of `width × height` over `rgba8`.
pub fn semio_image_from_rgba8(width: u32, height: u32, rgba8: Vec<u8>) -> SemioImageSnapshot {
    SemioImageSnapshot { schema: "s.stdio.semio.image".into(), width, height, colorspace: semio_s_artifact_stdio_semio_image::schema::snapshot::SemioColorspace::Rgba, bit_depth: 8, frames: vec![semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageFrame { delay_ms: 0, rgba8 }], icc: None, metadata: Vec::new() }
}


#[cfg(test)]
#[path="🧪️tests/🖼️decoded-image/🦀️.rs"]
mod decoded_image_tests;


pub fn create_pixel_layer(name: &str, width: u32, height: u32) -> RasterLayerNode {
    RasterLayerNode::Pixel { id: create_raster_id("layer"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(width), height: Some(height), image_key: None }
}

pub const RASTER_DOCUMENT_SCHEMA: &str = "raster.document";

pub const RASTER_OWNED_MAP_CAPACITY: usize = 64;

pub const RASTER_OWNED_MAP_PAGE_CAPACITY: usize = 8;

pub const RASTER_OWNED_MAP_PAGE_BACKING_BYTES: usize = 16 * 1024;

const RASTER_OWNED_MAP_PAGE_COUNT: usize = RASTER_OWNED_MAP_CAPACITY / RASTER_OWNED_MAP_PAGE_CAPACITY;

struct RasterOwnedMapPage<V> {
    entries: [Option<(String, V)>; RASTER_OWNED_MAP_PAGE_CAPACITY],
}

impl<V> RasterOwnedMapPage<V> {
    fn new() -> Self {
        Self { entries: std::array::from_fn(|_| None) }
    }
}

pub struct RasterOwnedMapPageBacking<V> {
    page: Option<Box<RasterOwnedMapPage<V>>>,
}

impl<V> RasterOwnedMapPageBacking<V> {
    /// 🧮 The credit one owned page costs — the EXACT size of the boxed page, not the declared
    /// ceiling [`RASTER_OWNED_MAP_PAGE_BACKING_BYTES`] the admission guards compare against.
    /// `Box<RasterOwnedMapPage<V>>` allocates exactly `size_of::<RasterOwnedMapPage<V>>()`, so this is
    /// an exact figure and not an allocator-layout observation. Reporting the 16 KiB ceiling instead
    /// starved every close ladder whose per-turn byte grant is the framework's 4 KiB
    /// `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` — `artifact_app_laws::close_registered_fixture_app` and
    /// the runtime's own `ArtifactDocumentStoreDisposer` both grant exactly that, so a POPULATED
    /// raster document could never reach terminal-empty: the page release answered `Pending { 0, 0 }`
    /// forever (measured 2026-09-21, `renders_layers_tree` /
    /// `document_tree_binds_the_layers_interaction_domain`).
    pub fn conservative_credit_bytes(&self) -> usize {
        size_of::<RasterOwnedMapPage<V>>()
    }

    pub fn release(mut self) {
        drop(self.page.take());
    }
}

/// 🔒️ Fail-closed, but never a SECOND panic while an earlier one unwinds: a destructor that panics
/// during cleanup is a non-unwinding abort that kills the whole process (SIGABRT) and erases every
/// other test's result. Same `std::thread::panicking()` guard the framework's `OrderedMap`,
/// `ValueRetirement`, `Dictionary` and `MutationDag` destructors carry.
impl<V> Drop for RasterOwnedMapPageBacking<V> {
    fn drop(&mut self) {
        assert!(self.page.is_none() || std::thread::panicking(), "Raster owned map page backing reached Drop before exact release");
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
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Raster owned map entry reached Drop before exact pair handback");
    }
}

pub enum RasterOwnedMapInsert<V> {
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

    pub fn allocated_page_count(&self) -> usize {
        self.pages.iter().filter(|page| page.is_some()).count()
    }

    /// 🧮 The credit one page costs — see [`RasterOwnedMapPageBacking::conservative_credit_bytes`],
    /// whose figure this must equal for preflight and release to agree.
    pub fn conservative_page_credit_bytes() -> usize {
        size_of::<RasterOwnedMapPage<V>>()
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

    pub fn entry_at(&self, index: usize) -> Option<(&String, &V)> {
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

    pub fn page_required_for_insert(&self, key: &str) -> bool {
        self.ordered_position(key).is_err() && self.free_slot().is_none() && self.length < RASTER_OWNED_MAP_CAPACITY
    }

    pub fn validate_additional_unique_entries(&self, count: usize) -> Result<(), &'static str> {
        if count > RASTER_OWNED_MAP_CAPACITY.saturating_sub(self.length) {
            return Err("raster-map.item-capacity");
        }
        if count > 0 && size_of::<RasterOwnedMapPage<V>>() > RASTER_OWNED_MAP_PAGE_BACKING_BYTES {
            return Err("raster-map.page-backing-capacity");
        }
        Ok(())
    }

    pub fn admit_one_page(&mut self) -> Result<(), &'static str> {
        if size_of::<RasterOwnedMapPage<V>>() > RASTER_OWNED_MAP_PAGE_BACKING_BYTES {
            return Err("raster-map.page-backing-capacity");
        }
        let page = self.pages.iter_mut().find(|page| page.is_none()).ok_or("raster-map.page-capacity")?;
        *page = Some(Box::new(RasterOwnedMapPage::new()));
        Ok(())
    }

    pub fn insert_pre_admitted(&mut self, key: String, value: V) -> Result<RasterOwnedMapInsert<V>, RasterOwnedMapRejected<V>> {
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
        if self.page_required_for_insert(&key) && self.admit_one_page().is_err() {
            return Err(RasterOwnedMapRejected { key, value, reason: "raster-map.page-capacity" });
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

    pub fn take_last_entry(&mut self) -> Option<(String, V)> {
        let slot = self.ordered_slot(self.length.checked_sub(1)?)?;
        let entry = self.pages[slot / RASTER_OWNED_MAP_PAGE_CAPACITY].as_mut()?.entries[slot % RASTER_OWNED_MAP_PAGE_CAPACITY].take()?;
        self.length -= 1;
        Some(entry)
    }

    pub fn take_empty_page_backing(&mut self) -> Option<RasterOwnedMapPageBacking<V>> {
        let page = self.pages.iter_mut().find(|page| page.as_ref().is_some_and(|page| page.entries.iter().all(Option::is_none)))?;
        Some(RasterOwnedMapPageBacking { page: page.take() })
    }

    pub fn iter(&self) -> RasterOwnedMapIter<'_, V> {
        RasterOwnedMapIter { map: self, index: 0 }
    }

    pub fn keys(&self) -> RasterOwnedMapKeys<'_, V> {
        RasterOwnedMapKeys { inner: self.iter() }
    }

    /// 🧹️ Empties this map entry by entry and releases every page backing, leaving it in the only
    /// state its `Drop` guard admits. Every route that can abandon a half-built map (a codec whose
    /// next entry refuses, a layer forest that is not planted) closes it through here rather than
    /// hand-rolling the same two loops.
    pub fn retire(&mut self) {
        while self.take_last_entry().is_some() {}
        while let Some(page) = self.take_empty_page_backing() {
            page.release();
        }
    }
}

impl<V> Default for RasterOwnedMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

/// 🔒️ Fail-closed on a bare drop, but silent while another panic unwinds — a second panic inside a
/// destructor during cleanup aborts the process instead of failing one test (that is exactly how one
/// red raster test used to take the whole binary down with SIGABRT). The pages are still handed back
/// to the allocator on the unwinding path so nothing leaks.
impl<V> Drop for RasterOwnedMap<V> {
    fn drop(&mut self) {
        assert!((self.length == 0 && self.pages.iter().all(Option::is_none)) || std::thread::panicking(), "Raster owned map reached Drop before every entry and page backing was explicitly retired");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.pages) };
    }
}

/// 🧬️ A real deep copy: the map is a FIXED-capacity paged owner (64 entries over 8 pages), so
/// copying it is bounded by construction and needs no retained page authority. It used to answer an
/// EMPTY map for a populated source, which silently deleted every asset and every adjustment
/// parameter on any `RasterSnapshot::clone` — and `ArtifactStore` clones the open document.
impl<V: Clone> Clone for RasterOwnedMap<V> {
    fn clone(&self) -> Self {
        let mut copy = Self::new();
        for (key, value) in self.iter() {
            copy.insert(key.clone(), value.clone()).map_err(|rejected| rejected.reason).expect("a Raster owned map copy fits the capacity its source already fits");
        }
        copy
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

/// 🗂️ The map's real DSL projection, key-ordered exactly as [`RasterOwnedMap::iter`] walks it.
impl<V: semio_framework_dsl_record::DslField> semio_framework_dsl_record::DslField for RasterOwnedMap<V> {
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Map(Box::new(V::shape()))
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Map(self.iter().map(|(key, value)| (key.clone(), value.to_value())).collect())
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Map(entries) = value else { return Err(format!("expected Map, found {value:?}")) };
        let mut out = Self::new();
        for (key, item) in entries {
            let admitted = V::from_value(item).and_then(|item| out.insert(key.clone(), item).map_err(|rejected| rejected.reason.to_string()));
            if let Err(error) = admitted {
                out.retire();
                return Err(error);
            }
        }
        Ok(out)
    }
}

/// 🗂️ The same real projection on `dsl::ToValue`/`dsl::FromValue`, so a `RasterOwnedMap` field
/// carries its true contents inside a `#[derive(dsl::ToValue, dsl::FromValue)]` type.
///
/// 🩸️ Both directions used to refuse a populated map ("interactive production routes require the
/// retained page output authority") — a retained page authority this artifact never grew. Every
/// whole-document route therefore trapped or silently emptied the document: `ArtifactStore`'s own
/// `encode_pack` ("default-options pack encode is infallible"), the `.raster` DSL printer whose
/// committed demo carrier is itself populated, the `s.stdio.json` export/import serializers, and the
/// `apply_raster_mutation_json` oracle bridge whose committed before-documents carry both layers and
/// assets. The map is FIXED-capacity (64 entries over 8 pages of 16 KiB), so a whole-map projection
/// is bounded by construction and no paging authority is owed.
impl<V: semio_framework_value::ToValue> semio_framework_value::ToValue for RasterOwnedMap<V> {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::Object(self.iter().map(|(key, value)| (key.clone(), value.to_value())).collect())
    }
}

impl<V: semio_framework_value::FromValue> semio_framework_value::FromValue for RasterOwnedMap<V> {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(entries) = value else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for a Raster owned map")) };
        let mut out = Self::new();
        for (key, item) in entries {
            let admitted = V::from_value(item).and_then(|item| out.insert(key, item).map_err(|rejected| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, rejected.reason)));
            if let Err(error) = admitted {
                out.retire();
                return Err(error);
            }
        }
        Ok(out)
    }
}

/// ♻️ Retires one fixed-capacity map entry by entry, then each exact page allocation, through the framework's admitted cursor protocol.
struct RasterOwnedMapRetirement<V: semio_framework_value::retirement::RetireOwned> {
    map: std::mem::ManuallyDrop<RasterOwnedMap<V>>,
}

impl<V: semio_framework_value::retirement::RetireOwned> RasterOwnedMapRetirement<V> {
    fn last_entry(&self) -> Option<&(String, V)> {
        let slot = self.map.ordered_slot(self.map.length.checked_sub(1)?)?;
        self.map.entry(slot)
    }

    fn page_bytes(&self) -> usize {
        if self.map.allocated_page_count() == 0 { 0 } else { size_of::<RasterOwnedMapPage<V>>() }
    }
}

impl<V: semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetirementCursor for RasterOwnedMapRetirement<V> {
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_value::retirement::RetirementStep {
        use semio_framework_value::retirement::{RetireOwned, RetirementStep};
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return RetirementStep::BudgetExhausted;
        }
        if self.map.length > 0 {
            let entry = self.map.take_last_entry().expect("Raster owned map retains the entry it quoted");
            return RetirementStep::Child(entry.retirement());
        }
        let bytes = self.page_bytes();
        if bytes == 0 {
            return RetirementStep::Complete;
        }
        if bytes > grant.maximum_release_bytes {
            return RetirementStep::BudgetExhausted;
        }
        let page = self.map.take_empty_page_backing().expect("Raster owned map retains the page it quoted");
        page.release();
        RetirementStep::Bytes(bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.map.length == 0 && self.map.allocated_page_count() == 0
    }

    fn next_close_byte_demand(&self) -> Option<usize> {
        Some(if self.map.length == 0 { self.page_bytes() } else { 0 })
    }

    fn next_birth_bytes(&self, _maximum_bytes: usize) -> Option<usize> {
        if self.map.length == 0 {
            return Some(0);
        }
        semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(self.last_entry()?)
    }

    fn terminal_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

impl<V: semio_framework_value::retirement::RetireOwned> Drop for RasterOwnedMapRetirement<V> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || semio_framework_value::retirement::RetirementCursor::terminal_is_empty(self), "Raster owned map retirement abandoned before terminal-empty");
        if semio_framework_value::retirement::RetirementCursor::terminal_is_empty(self) {
            unsafe { std::mem::ManuallyDrop::drop(&mut self.map) };
        }
    }
}

impl<V: semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetireOwned for RasterOwnedMap<V> {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        Box::new(RasterOwnedMapRetirement { map: std::mem::ManuallyDrop::new(self) })
    }

    fn retirement_birth_bytes(&self) -> Option<usize> {
        Some(size_of::<RasterOwnedMapRetirement<V>>())
    }

    fn controlled_retirement_supported() -> bool {
        true
    }
}

impl<V: semio_framework_pack_json::ArtifactCanonicalJsonTree> semio_framework_pack_json::ArtifactCanonicalJsonTree for RasterOwnedMap<V> {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
        Ok(semio_framework_pack_json::ArtifactCanonicalJsonNode::Object(self.len()))
    }

    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree, semio_framework_value::ValueError> {
        self.entry_at(ordinal).map(|(_, value)| value as &dyn semio_framework_pack_json::ArtifactCanonicalJsonTree).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical Raster map ordinal is absent"))
    }

    fn canonical_tree_key(&self, ordinal: usize) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonText<'_>, semio_framework_value::ValueError> {
        self.entry_at(ordinal).map(|(key, _)| key.as_str().into()).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical Raster map key is absent"))
    }
}

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
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct RasterViewportSize {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct RasterCamera {
    #[value(default)]
    pub x: f64,
    #[value(default)]
    pub y: f64,
    #[value(default = "default_one")]
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

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct RasterTransform {
    pub x: f64,
    pub y: f64,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

impl Default for RasterTransform {
    fn default() -> Self { Self { x:0.0,y:0.0,a:1.0,b:0.0,c:0.0,d:1.0 } }
}

impl RasterTransform {
    /// 📐️ Coefficients in the shared compositor's affine order.
    pub fn as_affine(&self)->semio_framework_pixels::compositing::CompositeAffine { [self.a,self.b,self.c,self.d,self.x,self.y] }
    /// 🧭️ Keeps the full placement returned by coordinate-frame conversion.
    pub fn from_affine(value:semio_framework_pixels::compositing::CompositeAffine)->Self {
        let [a,b,c,d,x,y]=value;Self {x,y,a,b,c,d}
    }
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerMask {
    #[value(default = "default_true")]
    pub enabled: bool,
    #[value(default = "default_true")]
    pub linked: bool,
    #[value(default)]
    pub invert: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub image_key: Option<String>,
    #[dsl(block)]
    pub transform: RasterTransform,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum RasterLayerNode {
    #[value(rename = "pixel", rename_all = "camelCase")]
    Pixel {
        id: String,
        name: String,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        #[value(default = "one_f32")]
        opacity: f32,
        #[dsl(key = "blend")]
        #[value(default = "default_blend")]
        blend_mode: String,
        #[dsl(block)]
        #[value(default)]
        transform: RasterTransform,
        #[dsl(block)]
        mask: Option<RasterLayerMask>,
        width: Option<u32>,
        height: Option<u32>,
        #[dsl(key = "image")]
        image_key: Option<String>,
    },
    #[value(rename = "group", rename_all = "camelCase")]
    Group {
        id: String,
        name: String,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        #[value(default = "one_f32")]
        opacity: f32,
        #[dsl(key = "blend")]
        #[value(default = "default_blend")]
        blend_mode: String,
        #[dsl(block)]
        #[value(default)]
        transform: RasterTransform,
        #[dsl(block)]
        mask: Option<RasterLayerMask>,
        #[dsl(statements, block)]
        children: Vec<RasterLayerNode>,
    },
    #[value(rename = "adjustment", rename_all = "camelCase")]
    Adjustment {
        id: String,
        name: String,
        #[value(default = "default_true")]
        visible: bool,
        #[value(default)]
        locked: bool,
        #[value(default = "one_f32")]
        opacity: f32,
        #[dsl(key = "blend")]
        #[value(default = "default_blend")]
        blend_mode: String,
        #[dsl(block)]
        #[value(default)]
        transform: RasterTransform,
        #[dsl(key = "kind")]
        adjustment_kind: String,
        #[value(default)]
        params: RasterOwnedMap<semio_framework_value::DslValue>,
    },
}

/// 🧽️ Closes one layer subtree's owned roots. An `Adjustment` layer carries a
/// `RasterOwnedMap<dsl::DslValue>` of parameters whose `Drop` is fail-closed, and a `Group` hides
/// arbitrarily many of them below it, so every seam that throws a layer away — a displaced replay
/// projection, a cold diff, an inverted `create-layer` — closes it through here instead of dropping
/// it and aborting the guest.
pub fn retire_raster_layer(layer: RasterLayerNode) {
    match layer {
        RasterLayerNode::Adjustment { mut params, .. } => params.retire(),
        RasterLayerNode::Group { children, .. } => retire_raster_layers(children),
        RasterLayerNode::Pixel { .. } => {}
    }
}

/// 🧼️ [`retire_raster_layer`] over a whole forest, in document order.
pub fn retire_raster_layers(layers: Vec<RasterLayerNode>) {
    for layer in layers {
        retire_raster_layer(layer);
    }
}

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
fn image_content_child_handle(asset_id: &str, image: &SemioImageSnapshot) -> RasterAssetChild {
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

const RASTER_OWNED_FIELD_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

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

//#region 🪢️TaxonomyMounts
#[path = "💡️inferences/🦀️.rs"]
pub mod inferences;
#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
    #[path = "."]
    pub mod sqlite {
        #[path = "../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
    #[path = "."]
    pub mod text {
        #[path = "../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
}
#[path = "📸️snapshot/🦀️.rs"]
pub mod snapshot;
//#endregion 🪢️TaxonomyMounts
