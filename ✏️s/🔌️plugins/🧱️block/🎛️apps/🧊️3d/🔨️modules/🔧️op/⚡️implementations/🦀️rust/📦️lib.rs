//! ⚡️ Block 3D app — operation enum + laws (constitutional: op).

use block_3d::{Block3dDefinition, Block3dVortexKind, Block3dVortexTemplate};
use block_shared::{BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};

pub type Block3dEnvelope = store::DocumentEnvelope<Block3dDefinition, Block3dOperation>;
pub type Block3dStore = store::DocumentStore<Block3dDefinition, Block3dOperation>;

// #region 🔖️Collections
trait Block3dHasId {
    fn id(&self) -> &str;
}
impl Block3dHasId for BlockRepresentation {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block3dHasId for Block3dVortexKind {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block3dHasId for Block3dVortexTemplate {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block3dHasId for BlockCompatibilityRule {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block3dHasId for BlockAttribute {
    fn id(&self) -> &str {
        &self.key
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block3dCollectionDiff<T> {
    pub removed: Vec<String>,
    pub set: Vec<(usize, T)>,
}

/// 🧩️ Manual (not derived) `Default` — `#[derive(Default)]` on a generic struct bounds every type
/// parameter by `Default`, even though `Vec<(usize, T)>` never needs it.
impl<T> Default for Block3dCollectionDiff<T> {
    fn default() -> Self {
        Self { removed: Vec::new(), set: Vec::new() }
    }
}

fn apply_block3d_collection_diff<T: Block3dHasId + Clone>(items: &mut Vec<T>, removed: &[String], set: &[(usize, T)]) {
    for id in removed {
        items.retain(|item| item.id() != id);
    }
    for (index, item) in set {
        if let Some(pos) = items.iter().position(|entry| entry.id() == item.id()) {
            items[pos] = item.clone();
        } else {
            items.insert((*index).min(items.len()), item.clone());
        }
    }
}

fn block3d_index_of<T: Block3dHasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}
// #endregion 🔖️Collections

// #region 🔖️Operations
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block3dDiff {
    pub document: Option<Block3dDefinition>,
    pub object_kind: Option<BlockKindIdentity>,
    pub representations: Block3dCollectionDiff<BlockRepresentation>,
    pub vortex_kinds: Block3dCollectionDiff<Block3dVortexKind>,
    pub vortices: Block3dCollectionDiff<Block3dVortexTemplate>,
    pub compatibility: Block3dCollectionDiff<BlockCompatibilityRule>,
    pub attributes: Block3dCollectionDiff<BlockAttribute>,
    pub authors: Option<Vec<BlockAuthor>>,
    pub camera3d: Option<BlockCamera3d>,
    pub meta: Option<BlockMeta>,
}

fn block3d_diff_absorb(diff: &mut Block3dDiff, other: Block3dDiff) {
    if other.document.is_some() {
        *diff = Block3dDiff { document: other.document, ..Default::default() };
        return;
    }
    if other.object_kind.is_some() {
        diff.object_kind = other.object_kind;
    }
    diff.representations.removed.extend(other.representations.removed);
    diff.representations.set.extend(other.representations.set);
    diff.vortex_kinds.removed.extend(other.vortex_kinds.removed);
    diff.vortex_kinds.set.extend(other.vortex_kinds.set);
    diff.vortices.removed.extend(other.vortices.removed);
    diff.vortices.set.extend(other.vortices.set);
    diff.compatibility.removed.extend(other.compatibility.removed);
    diff.compatibility.set.extend(other.compatibility.set);
    diff.attributes.removed.extend(other.attributes.removed);
    diff.attributes.set.extend(other.attributes.set);
    if other.authors.is_some() {
        diff.authors = other.authors;
    }
    if other.camera3d.is_some() {
        diff.camera3d = other.camera3d;
    }
    if other.meta.is_some() {
        diff.meta = other.meta;
    }
}

impl OperationDiff<Block3dDefinition> for Block3dDiff {
    fn apply(&self, projection: &Block3dDefinition) -> Block3dDefinition {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(object_kind) = &self.object_kind {
            next.object_kind = object_kind.clone();
        }
        apply_block3d_collection_diff(&mut next.representations, &self.representations.removed, &self.representations.set);
        apply_block3d_collection_diff(&mut next.vortex_kinds, &self.vortex_kinds.removed, &self.vortex_kinds.set);
        apply_block3d_collection_diff(&mut next.vortices, &self.vortices.removed, &self.vortices.set);
        apply_block3d_collection_diff(&mut next.compatibility, &self.compatibility.removed, &self.compatibility.set);
        apply_block3d_collection_diff(&mut next.attributes, &self.attributes.removed, &self.attributes.set);
        if let Some(authors) = &self.authors {
            next.authors = authors.clone();
        }
        if let Some(camera3d) = &self.camera3d {
            next.camera3d = camera3d.clone();
        }
        if let Some(meta) = &self.meta {
            next.meta = meta.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        block3d_diff_absorb(self, other);
    }
}

/// 🧮️ Block-3d operation: id-keyed table edits plus scalar object_kind/camera3d/meta, each with a
/// true inverse computed from the pre-operation projection, and a whole-document replace for example
/// loads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Block3dOperation {
    #[dsl(key = "setObjectKind")]
    SetObjectKind {
        #[dsl(block)]
        object_kind: BlockKindIdentity,
    },
    #[dsl(key = "setRepresentation")]
    SetRepresentation {
        index: usize,
        #[dsl(block)]
        representation: BlockRepresentation,
    },
    #[dsl(key = "removeRepresentation")]
    RemoveRepresentation { id: String },
    #[dsl(key = "setVortexKind")]
    SetVortexKind {
        index: usize,
        #[dsl(block)]
        vortex_kind: Block3dVortexKind,
    },
    #[dsl(key = "removeVortexKind")]
    RemoveVortexKind { id: String },
    #[dsl(key = "setVortex")]
    SetVortex {
        index: usize,
        #[dsl(block)]
        vortex: Block3dVortexTemplate,
    },
    #[dsl(key = "removeVortex")]
    RemoveVortex { id: String },
    #[dsl(key = "setCompatibilityRule")]
    SetCompatibilityRule {
        index: usize,
        #[dsl(block)]
        rule: BlockCompatibilityRule,
    },
    #[dsl(key = "removeCompatibilityRule")]
    RemoveCompatibilityRule { id: String },
    #[dsl(key = "setAttribute")]
    SetAttribute {
        index: usize,
        #[dsl(block)]
        attribute: BlockAttribute,
    },
    #[dsl(key = "removeAttribute")]
    RemoveAttribute { key: String },
    #[dsl(key = "setAuthors")]
    SetAuthors { authors: Vec<BlockAuthor> },
    #[dsl(key = "setCamera3d")]
    SetCamera3d {
        #[dsl(block)]
        camera3d: BlockCamera3d,
    },
    #[dsl(key = "setMeta")]
    SetMeta {
        #[dsl(block)]
        meta: BlockMeta,
    },
    #[dsl(key = "setDocument")]
    SetDocument {
        #[dsl(block)]
        document: Block3dDefinition,
    },
}

fn block3d_operation_diff(operation: &Block3dOperation) -> Block3dDiff {
    let mut diff = Block3dDiff::default();
    match operation {
        Block3dOperation::SetObjectKind { object_kind } => diff.object_kind = Some(object_kind.clone()),
        Block3dOperation::SetRepresentation { index, representation } => diff.representations.set.push((*index, representation.clone())),
        Block3dOperation::RemoveRepresentation { id } => diff.representations.removed.push(id.clone()),
        Block3dOperation::SetVortexKind { index, vortex_kind } => diff.vortex_kinds.set.push((*index, vortex_kind.clone())),
        Block3dOperation::RemoveVortexKind { id } => diff.vortex_kinds.removed.push(id.clone()),
        Block3dOperation::SetVortex { index, vortex } => diff.vortices.set.push((*index, vortex.clone())),
        Block3dOperation::RemoveVortex { id } => diff.vortices.removed.push(id.clone()),
        Block3dOperation::SetCompatibilityRule { index, rule } => diff.compatibility.set.push((*index, rule.clone())),
        Block3dOperation::RemoveCompatibilityRule { id } => diff.compatibility.removed.push(id.clone()),
        Block3dOperation::SetAttribute { index, attribute } => diff.attributes.set.push((*index, attribute.clone())),
        Block3dOperation::RemoveAttribute { key } => diff.attributes.removed.push(key.clone()),
        Block3dOperation::SetAuthors { authors } => diff.authors = Some(authors.clone()),
        Block3dOperation::SetCamera3d { camera3d } => diff.camera3d = Some(camera3d.clone()),
        Block3dOperation::SetMeta { meta } => diff.meta = Some(meta.clone()),
        Block3dOperation::SetDocument { document } => diff.document = Some(document.clone()),
    }
    diff
}

impl Operation<Block3dDefinition> for Block3dOperation {
    type Diff = Block3dDiff;

    fn diff(&self, _projection: &Block3dDefinition) -> Block3dDiff {
        block3d_operation_diff(self)
    }

    fn backwards(&self, projection: &Block3dDefinition) -> Vec<Self> {
        match self {
            Block3dOperation::SetObjectKind { .. } => vec![Block3dOperation::SetObjectKind { object_kind: projection.object_kind.clone() }],
            Block3dOperation::SetRepresentation { representation, .. } => match block3d_index_of(&projection.representations, &representation.id) {
                Some(index) => vec![Block3dOperation::SetRepresentation { index, representation: projection.representations[index].clone() }],
                None => vec![Block3dOperation::RemoveRepresentation { id: representation.id.clone() }],
            },
            Block3dOperation::RemoveRepresentation { id } => {
                block3d_index_of(&projection.representations, id).map(|index| vec![Block3dOperation::SetRepresentation { index, representation: projection.representations[index].clone() }]).unwrap_or_default()
            }
            Block3dOperation::SetVortexKind { vortex_kind, .. } => match block3d_index_of(&projection.vortex_kinds, &vortex_kind.id) {
                Some(index) => vec![Block3dOperation::SetVortexKind { index, vortex_kind: projection.vortex_kinds[index].clone() }],
                None => vec![Block3dOperation::RemoveVortexKind { id: vortex_kind.id.clone() }],
            },
            Block3dOperation::RemoveVortexKind { id } => block3d_index_of(&projection.vortex_kinds, id).map(|index| vec![Block3dOperation::SetVortexKind { index, vortex_kind: projection.vortex_kinds[index].clone() }]).unwrap_or_default(),
            Block3dOperation::SetVortex { vortex, .. } => match block3d_index_of(&projection.vortices, &vortex.id) {
                Some(index) => vec![Block3dOperation::SetVortex { index, vortex: projection.vortices[index].clone() }],
                None => vec![Block3dOperation::RemoveVortex { id: vortex.id.clone() }],
            },
            Block3dOperation::RemoveVortex { id } => block3d_index_of(&projection.vortices, id).map(|index| vec![Block3dOperation::SetVortex { index, vortex: projection.vortices[index].clone() }]).unwrap_or_default(),
            Block3dOperation::SetCompatibilityRule { rule, .. } => match block3d_index_of(&projection.compatibility, &rule.id) {
                Some(index) => vec![Block3dOperation::SetCompatibilityRule { index, rule: projection.compatibility[index].clone() }],
                None => vec![Block3dOperation::RemoveCompatibilityRule { id: rule.id.clone() }],
            },
            Block3dOperation::RemoveCompatibilityRule { id } => block3d_index_of(&projection.compatibility, id).map(|index| vec![Block3dOperation::SetCompatibilityRule { index, rule: projection.compatibility[index].clone() }]).unwrap_or_default(),
            Block3dOperation::SetAttribute { attribute, .. } => match block3d_index_of(&projection.attributes, &attribute.key) {
                Some(index) => vec![Block3dOperation::SetAttribute { index, attribute: projection.attributes[index].clone() }],
                None => vec![Block3dOperation::RemoveAttribute { key: attribute.key.clone() }],
            },
            Block3dOperation::RemoveAttribute { key } => block3d_index_of(&projection.attributes, key).map(|index| vec![Block3dOperation::SetAttribute { index, attribute: projection.attributes[index].clone() }]).unwrap_or_default(),
            Block3dOperation::SetAuthors { .. } => vec![Block3dOperation::SetAuthors { authors: projection.authors.clone() }],
            Block3dOperation::SetCamera3d { .. } => vec![Block3dOperation::SetCamera3d { camera3d: projection.camera3d.clone() }],
            Block3dOperation::SetMeta { .. } => vec![Block3dOperation::SetMeta { meta: projection.meta.clone() }],
            Block3dOperation::SetDocument { .. } => vec![Block3dOperation::SetDocument { document: projection.clone() }],
        }
    }
}
// #endregion 🔖️Operations

//#region 🔖️ConfigOperations
/// 🧮️ `block_3d_engine::Block3dConfig`'s operation enum — one variant per settled interaction
/// (mirrors the pre-B1 `Block3dPlayApp` `RefCell` field writes), plus a generic `Snapshot` every
/// variant's `backwards()` returns — same "whole-config-snapshot inverse" shape
/// `shooting_op::ShootingConfigOperation` established for the pilot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Block3dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: block_3d_engine::Block3dConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "active-representation")]
    SetActiveRepresentation { representation_id: Option<String> },
    #[dsl(key = "wanted-tags")]
    SetWantedTags { tags: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "window-representations")]
    SetWindowRepresentations { window_id: String, representation_ids: Vec<String> },
    #[dsl(key = "toggle-window-representation")]
    ToggleWindowRepresentation { window_id: String, representation_id: String, visible: bool },
    #[dsl(key = "window-arrangement")]
    SetWindowArrangement { window_id: String, arrangement: String },
    #[dsl(key = "window-spacing")]
    SetWindowSpacing { window_id: String, spacing: f64 },
    #[dsl(key = "active-utility")]
    SetActiveUtility { window_id: String, utility_id: String },
    #[dsl(key = "brush-vortex-kind")]
    SetBrushVortexKind { vortex_kind_id: Option<String> },
    #[dsl(key = "brush-radius")]
    SetBrushRadius { radius: f64 },
    #[dsl(key = "brush-flip")]
    SetBrushFlip { flip: bool },
    #[dsl(key = "brush-preview")]
    SetBrushPreview { preview: Option<block_3d_engine::Block3dBrushPreview> },
    #[dsl(key = "camera")]
    SetCamera { camera: block_shared::BlockCamera3d },
    #[dsl(key = "hovered-vortex")]
    SetHoveredVortexFullId { full_id: Option<String> },
}

impl Operation<block_3d_engine::Block3dConfig> for Block3dConfigOperation {
    type Diff = block_3d_engine::Block3dConfig;

    fn diff(&self, base: &block_3d_engine::Block3dConfig) -> block_3d_engine::Block3dConfig {
        let mut next = base.clone();
        match self {
            Block3dConfigOperation::Snapshot { config } => return config.clone(),
            Block3dConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            Block3dConfigOperation::SetActiveRepresentation { representation_id } => next.active_representation_id = representation_id.clone(),
            Block3dConfigOperation::SetWantedTags { tags } => next.wanted_tags = tags.clone(),
            Block3dConfigOperation::SetLocale { value } => next.locale = value.clone(),
            Block3dConfigOperation::SetWindowRepresentations { window_id, representation_ids } => {
                let index = block_3d_engine::upsert_window_view_index(&mut next.windows, window_id);
                next.windows[index].representation_ids = representation_ids.clone();
            }
            Block3dConfigOperation::ToggleWindowRepresentation { window_id, representation_id, visible } => {
                let index = block_3d_engine::upsert_window_view_index(&mut next.windows, window_id);
                let row = &mut next.windows[index];
                if *visible {
                    if !row.representation_ids.contains(representation_id) {
                        row.representation_ids.push(representation_id.clone());
                    }
                } else {
                    row.representation_ids.retain(|id| id != representation_id);
                }
            }
            Block3dConfigOperation::SetWindowArrangement { window_id, arrangement } => {
                let index = block_3d_engine::upsert_window_view_index(&mut next.windows, window_id);
                next.windows[index].arrangement = arrangement.clone();
            }
            Block3dConfigOperation::SetWindowSpacing { window_id, spacing } => {
                let index = block_3d_engine::upsert_window_view_index(&mut next.windows, window_id);
                next.windows[index].spacing = *spacing;
            }
            Block3dConfigOperation::SetActiveUtility { window_id, utility_id } => {
                let index = block_3d_engine::upsert_window_view_index(&mut next.windows, window_id);
                next.windows[index].active_utility = utility_id.clone();
            }
            Block3dConfigOperation::SetBrushVortexKind { vortex_kind_id } => next.brush_vortex_kind_id = vortex_kind_id.clone(),
            Block3dConfigOperation::SetBrushRadius { radius } => next.brush_radius = *radius,
            Block3dConfigOperation::SetBrushFlip { flip } => next.brush_flip = *flip,
            Block3dConfigOperation::SetBrushPreview { preview } => next.brush_preview = preview.clone(),
            Block3dConfigOperation::SetCamera { camera } => next.camera = Some(camera.clone()),
            Block3dConfigOperation::SetHoveredVortexFullId { full_id } => next.hovered_vortex_full_id = full_id.clone(),
        }
        next
    }

    fn backwards(&self, base: &block_3d_engine::Block3dConfig) -> Vec<Self> {
        vec![Block3dConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_vortex_then_remove_round_trips_through_true_inverse() {
        let mut projection = block_3d_engine::empty_block3d_definition();
        let set = Block3dOperation::SetVortex { index: 0, vortex: Block3dVortexTemplate { id: "v0".into(), vortex_kind: "door".into(), position: [1.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius: 0.3, label: None } };
        let inverse = set.backwards(&projection);
        projection = set.diff(&projection).apply(&projection);
        assert_eq!(projection.vortices.len(), 1);
        assert_eq!(inverse, vec![Block3dOperation::RemoveVortex { id: "v0".into() }]);
        for operation in &inverse {
            projection = operation.diff(&projection).apply(&projection);
        }
        assert_eq!(projection, block_3d_engine::empty_block3d_definition());
    }

    #[test]
    fn config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = block_3d_engine::Block3dConfig::default();
        let operation = Block3dConfigOperation::SetSelection { ids: vec!["r0".into()] };
        let next = operation.diff(&base);
        assert_eq!(next.selected_ids, vec!["r0".to_string()]);
        let inverse = operation.backwards(&base);
        assert_eq!(inverse, vec![Block3dConfigOperation::Snapshot { config: base.clone() }]);
        let restored = inverse[0].diff(&next);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
