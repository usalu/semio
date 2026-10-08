//! 🧬️ Note diff schema — sparse typed delta over the artifact: assignable document scalars, ordered block-tree rows
//! (add, remove, move, field patch) and keyed asset rows. The central applier is the only caller of
//! [`MutationDiff::apply`]; every row names exactly the entity and fields it changes.

use crate::schema::{block_id, find_block, find_block_location, flatten_blocks};
use crate::{NoteBlockNode, NoteImageAsset, NoteSnapshot, NoteTableCell, NoteTextChild};
use framework_schema::ArtifactSchema;
use protocol::{ApplyCapability, DiffAlgebra, MutationApplyError, MutationApplyResult, MutationDiff};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the note artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.note.note")]
pub struct NoteDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub title: Option<NoteAssigned<Option<String>>>,
    #[state(artifact)]
    pub blocks: Option<NoteBlocksDelta>,
    #[state(artifact)]
    pub grid_visible: Option<NoteAssigned<Option<bool>>>,
    #[state(artifact)]
    pub grid_spacing: Option<NoteAssigned<Option<f64>>>,
    #[state(artifact)]
    pub grid_subdivisions: Option<NoteAssigned<Option<f64>>>,
    #[state(artifact)]
    pub grid_opacity: Option<NoteAssigned<Option<f64>>>,
    #[state(artifact)]
    pub snap_enabled: Option<NoteAssigned<Option<bool>>>,
    #[state(artifact)]
    pub snap_grid_spacing: Option<NoteAssigned<Option<f64>>>,
    #[state(artifact)]
    pub pencil_width: Option<NoteAssigned<Option<f64>>>,
    #[state(artifact)]
    pub eraser_radius: Option<NoteAssigned<Option<f64>>>,
    #[state(artifact)]
    pub assets: Option<NoteAssetsDelta>,
    /// 🔗️ The `R:any` forward link slot — schema/codec-complete, currently unset by any mutation.
    #[state(artifact)]
    pub linked_artifact: Option<NoteAssigned<Option<store::ArtifactLink>>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🎯️ An explicitly assigned optional value: wraps the value so assigning `None` stays distinct from leaving the field untouched on the wire (a bare nested `Option` collapses both to `null`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct NoteAssigned<T> {
    pub value: T,
}

impl<T> NoteAssigned<T> {
    /// 🏗️ Wraps the assigned value.
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

/// 🧩 Ordered block-tree delta: the rows run in order, each naming one block by identity.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct NoteBlocksDelta {
    pub rows: Vec<NoteBlockRow>,
}

/// 🧱️ One block-tree row. `parent_id` is `None` for the document root; `index` is the destination position in the container as it stands when the row runs (for a move: after the block left its old container).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "row", rename_all = "camelCase")]
pub enum NoteBlockRow {
    #[value(rename = "add", rename_all = "camelCase")]
    Add { parent_id: Option<String>, index: usize, block: NoteBlockNode },
    #[value(rename = "remove", rename_all = "camelCase")]
    Remove { id: String },
    #[value(rename = "move", rename_all = "camelCase")]
    Move { id: String, parent_id: Option<String>, index: usize },
    #[value(rename = "patch", rename_all = "camelCase")]
    Patch { id: String, patch: NoteBlockPatch },
}

/// 🩹 Sparse field patch of one block: every slot is optional, the common slots fit every block kind and a kind slot fits only its kind.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct NoteBlockPatch {
    pub name: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub rotation: Option<f64>,
    pub visible: Option<bool>,
    pub locked: Option<bool>,
    pub content: Option<NoteTextChild>,
    pub font_size: Option<f64>,
    pub font_weight: Option<String>,
    pub align: Option<String>,
    pub image_key: Option<String>,
    pub table: Option<Vec<NoteTableEdit>>,
    pub tex: Option<String>,
    pub display_mode: Option<bool>,
    pub points: Option<Vec<[f64; 2]>>,
    pub stroke_width: Option<f64>,
    pub color: Option<[f64; 4]>,
}

/// 📊️ One ordered structural edit of a table block's grid.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "edit", rename_all = "camelCase")]
pub enum NoteTableEdit {
    #[value(rename = "insertRow", rename_all = "camelCase")]
    InsertRow { index: usize, cells: Vec<NoteTableCell> },
    #[value(rename = "removeRow", rename_all = "camelCase")]
    RemoveRow { index: usize },
    #[value(rename = "insertColumn", rename_all = "camelCase")]
    InsertColumn { index: usize, name: String, cells: Vec<NoteTableCell> },
    #[value(rename = "removeColumn", rename_all = "camelCase")]
    RemoveColumn { index: usize },
}

/// 🗂️ Keyed asset delta: at most one row per key (a replace is one `replace` row), rows kept in key order.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct NoteAssetsDelta {
    pub rows: Vec<NoteAssetRow>,
}

/// 🖼️ One keyed asset row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "row", rename_all = "camelCase")]
pub enum NoteAssetRow {
    #[value(rename = "insert", rename_all = "camelCase")]
    Insert { key: String, asset: NoteImageAsset },
    #[value(rename = "replace", rename_all = "camelCase")]
    Replace { key: String, asset: NoteImageAsset },
    #[value(rename = "remove", rename_all = "camelCase")]
    Remove { key: String },
}

impl NoteAssetRow {
    fn key(&self) -> &str {
        match self {
            Self::Insert { key, .. } | Self::Replace { key, .. } | Self::Remove { key } => key,
        }
    }
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️BlockFields
struct CommonMut<'a> {
    name: &'a mut String,
    x: &'a mut f64,
    y: &'a mut f64,
    width: &'a mut f64,
    height: &'a mut f64,
    rotation: &'a mut f64,
    visible: &'a mut bool,
    locked: &'a mut bool,
}

fn common_mut(block: &mut NoteBlockNode) -> CommonMut<'_> {
    match block {
        NoteBlockNode::Text { name, x, y, width, height, rotation, visible, locked, .. }
        | NoteBlockNode::Image { name, x, y, width, height, rotation, visible, locked, .. }
        | NoteBlockNode::Table { name, x, y, width, height, rotation, visible, locked, .. }
        | NoteBlockNode::Math { name, x, y, width, height, rotation, visible, locked, .. }
        | NoteBlockNode::Ink { name, x, y, width, height, rotation, visible, locked, .. }
        | NoteBlockNode::Group { name, x, y, width, height, rotation, visible, locked, .. } => CommonMut { name, x, y, width, height, rotation, visible, locked },
    }
}

fn common_values(block: &NoteBlockNode) -> NoteBlockPatch {
    let (name, x, y, width, height, rotation, visible, locked) = match block {
        NoteBlockNode::Text { name, x, y, width, height, rotation, visible, locked, .. }
        | NoteBlockNode::Image { name, x, y, width, height, rotation, visible, locked, .. }
        | NoteBlockNode::Table { name, x, y, width, height, rotation, visible, locked, .. }
        | NoteBlockNode::Math { name, x, y, width, height, rotation, visible, locked, .. }
        | NoteBlockNode::Ink { name, x, y, width, height, rotation, visible, locked, .. }
        | NoteBlockNode::Group { name, x, y, width, height, rotation, visible, locked, .. } => (name, x, y, width, height, rotation, visible, locked),
    };
    NoteBlockPatch { name: Some(name.clone()), x: Some(*x), y: Some(*y), width: Some(*width), height: Some(*height), rotation: Some(*rotation), visible: Some(*visible), locked: Some(*locked), ..Default::default() }
}

fn mismatch(slot: &str) -> MutationApplyError {
    MutationApplyError::new("mutation.apply.invalid-target", format!("block kind has no `{slot}` slot")).at([slot.to_string()])
}

fn write_table(columns: &mut Vec<String>, rows: &mut Vec<Vec<NoteTableCell>>, edits: &[NoteTableEdit]) -> Result<(), MutationApplyError> {
    let at = |row: usize| ["table".to_string(), row.to_string()];
    for (row, edit) in edits.iter().enumerate() {
        match edit {
            NoteTableEdit::InsertRow { index, cells } => {
                if *index > rows.len() || cells.len() != columns.len() {
                    return Err(MutationApplyError::new("mutation.apply.invalid-index", "table row insertion is out of range or does not span the columns").at(at(row)));
                }
                rows.insert(*index, cells.clone());
            }
            NoteTableEdit::RemoveRow { index } => {
                if *index >= rows.len() {
                    return Err(MutationApplyError::new("mutation.apply.missing-target", "removed table row does not exist").at(at(row)));
                }
                rows.remove(*index);
            }
            NoteTableEdit::InsertColumn { index, name, cells } => {
                if *index > columns.len() || cells.len() != rows.len() {
                    return Err(MutationApplyError::new("mutation.apply.invalid-index", "table column insertion is out of range or does not span the rows").at(at(row)));
                }
                columns.insert(*index, name.clone());
                for (line, cell) in rows.iter_mut().zip(cells) {
                    line.insert(*index, cell.clone());
                }
            }
            NoteTableEdit::RemoveColumn { index } => {
                if *index >= columns.len() {
                    return Err(MutationApplyError::new("mutation.apply.missing-target", "removed table column does not exist").at(at(row)));
                }
                columns.remove(*index);
                for line in rows.iter_mut() {
                    line.remove(*index);
                }
            }
        }
    }
    Ok(())
}

fn undo_table(columns: &[String], rows: &[Vec<NoteTableCell>], edits: &[NoteTableEdit]) -> Vec<NoteTableEdit> {
    let mut columns = columns.to_vec();
    let mut rows = rows.to_vec();
    let mut undo = Vec::new();
    for edit in edits {
        match edit {
            NoteTableEdit::InsertRow { index, .. } => undo.push(NoteTableEdit::RemoveRow { index: *index }),
            NoteTableEdit::RemoveRow { index } => {
                if let Some(cells) = rows.get(*index) {
                    undo.push(NoteTableEdit::InsertRow { index: *index, cells: cells.clone() });
                }
            }
            NoteTableEdit::InsertColumn { index, .. } => undo.push(NoteTableEdit::RemoveColumn { index: *index }),
            NoteTableEdit::RemoveColumn { index } => {
                if let Some(name) = columns.get(*index) {
                    undo.push(NoteTableEdit::InsertColumn { index: *index, name: name.clone(), cells: rows.iter().filter_map(|line| line.get(*index).cloned()).collect() });
                }
            }
        }
        if write_table(&mut columns, &mut rows, std::slice::from_ref(edit)).is_err() {
            break;
        }
    }
    undo.reverse();
    undo
}

fn write_patch(block: &mut NoteBlockNode, patch: &NoteBlockPatch) -> Result<(), MutationApplyError> {
    let common = common_mut(block);
    macro_rules! set {
        ($($slot:ident),+) => {
            $(if let Some(value) = &patch.$slot {
                *common.$slot = value.clone();
            })+
        };
    }
    set!(name, x, y, width, height, rotation, visible, locked);
    macro_rules! kind {
        ($slot:ident, $variant:ident, $field:ident) => {
            if let Some(value) = &patch.$slot {
                match block {
                    NoteBlockNode::$variant { $field, .. } => *$field = value.clone(),
                    _ => return Err(mismatch(stringify!($slot))),
                }
            }
        };
    }
    kind!(content, Text, content);
    kind!(font_size, Text, font_size);
    kind!(font_weight, Text, font_weight);
    kind!(align, Text, align);
    kind!(image_key, Image, image_key);
    kind!(tex, Math, tex);
    kind!(display_mode, Math, display_mode);
    kind!(points, Ink, points);
    kind!(stroke_width, Ink, stroke_width);
    kind!(color, Ink, color);
    if let Some(edits) = &patch.table {
        match block {
            NoteBlockNode::Table { columns, rows, .. } => write_table(columns, rows, edits)?,
            _ => return Err(mismatch("table")),
        }
    }
    Ok(())
}

impl NoteBlockPatch {
    /// ➕️ Composes `later` over this patch: the later value of a slot wins and table edits run in sequence (an adjacent insert and remove of one line cancel).
    pub fn merge(&mut self, later: Self) {
        macro_rules! take {
            ($($slot:ident),+) => {
                $(if later.$slot.is_some() {
                    self.$slot = later.$slot;
                })+
            };
        }
        take!(name, x, y, width, height, rotation, visible, locked, content, font_size, font_weight, align, image_key, tex, display_mode, points, stroke_width, color);
        if let Some(edits) = later.table {
            let table = self.table.get_or_insert_with(Vec::new);
            for edit in edits {
                let cancels = match (table.last(), &edit) {
                    (Some(NoteTableEdit::InsertRow { index, .. }), NoteTableEdit::RemoveRow { index: removed }) | (Some(NoteTableEdit::InsertColumn { index, .. }), NoteTableEdit::RemoveColumn { index: removed }) => index == removed,
                    _ => false,
                };
                if cancels {
                    table.pop();
                } else {
                    table.push(edit);
                }
            }
            if table.is_empty() {
                self.table = None;
            }
        }
    }

    /// 🔁️ The patch that restores `before` for exactly the slots this patch names.
    pub fn restoring(&self, before: &NoteBlockNode) -> Self {
        let held = common_values(before);
        let mut restore = Self {
            name: self.name.as_ref().and(held.name),
            x: self.x.and(held.x),
            y: self.y.and(held.y),
            width: self.width.and(held.width),
            height: self.height.and(held.height),
            rotation: self.rotation.and(held.rotation),
            visible: self.visible.and(held.visible),
            locked: self.locked.and(held.locked),
            ..Default::default()
        };
        match before {
            NoteBlockNode::Text { content, font_size, font_weight, align, .. } => {
                restore.content = self.content.as_ref().map(|_| content.clone());
                restore.font_size = self.font_size.map(|_| *font_size);
                restore.font_weight = self.font_weight.as_ref().map(|_| font_weight.clone());
                restore.align = self.align.as_ref().map(|_| align.clone());
            }
            NoteBlockNode::Image { image_key, .. } => restore.image_key = self.image_key.as_ref().map(|_| image_key.clone()),
            NoteBlockNode::Table { columns, rows, .. } => restore.table = self.table.as_ref().map(|edits| undo_table(columns, rows, edits)),
            NoteBlockNode::Math { tex, display_mode, .. } => {
                restore.tex = self.tex.as_ref().map(|_| tex.clone());
                restore.display_mode = self.display_mode.map(|_| *display_mode);
            }
            NoteBlockNode::Ink { points, stroke_width, color, .. } => {
                restore.points = self.points.as_ref().map(|_| points.clone());
                restore.stroke_width = self.stroke_width.map(|_| *stroke_width);
                restore.color = self.color.map(|_| *color);
            }
            NoteBlockNode::Group { .. } => {}
        }
        restore
    }
}
//#endregion 🔖️BlockFields

//#region 🔖️Tree
fn path_to(blocks: &[NoteBlockNode], id: &str) -> Option<Vec<usize>> {
    for (index, block) in blocks.iter().enumerate() {
        if block_id(block) == id {
            return Some(vec![index]);
        }
        if let NoteBlockNode::Group { children, .. } = block {
            if let Some(mut tail) = path_to(children, id) {
                tail.insert(0, index);
                return Some(tail);
            }
        }
    }
    None
}

fn node_at_mut<'a>(blocks: &'a mut Vec<NoteBlockNode>, path: &[usize]) -> Option<&'a mut NoteBlockNode> {
    let (first, rest) = path.split_first()?;
    let node = blocks.get_mut(*first)?;
    match (rest.is_empty(), node) {
        (true, node) => Some(node),
        (false, NoteBlockNode::Group { children, .. }) => node_at_mut(children, rest),
        (false, _) => None,
    }
}

fn container_mut<'a>(blocks: &'a mut Vec<NoteBlockNode>, parent: Option<&str>) -> Result<&'a mut Vec<NoteBlockNode>, MutationApplyError> {
    let Some(parent) = parent else {
        return Ok(blocks);
    };
    let path = path_to(blocks, parent).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "block parent does not exist").at(["parentId"]))?;
    match node_at_mut(blocks, &path) {
        Some(NoteBlockNode::Group { children, .. }) => Ok(children),
        _ => Err(MutationApplyError::new("mutation.apply.invalid-target", "block parent is not a group").at(["parentId"])),
    }
}

fn insert_row(blocks: &mut Vec<NoteBlockNode>, parent: Option<&str>, index: usize, block: NoteBlockNode) -> Result<(), MutationApplyError> {
    let container = container_mut(blocks, parent)?;
    if index > container.len() {
        return Err(MutationApplyError::new("mutation.apply.invalid-index", format!("block insertion index {index} exceeds length {}", container.len())).at(["index"]));
    }
    container.insert(index, block);
    Ok(())
}

fn take_row(blocks: &mut Vec<NoteBlockNode>, id: &str) -> Result<NoteBlockNode, MutationApplyError> {
    let path = path_to(blocks, id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "addressed block does not exist").at([id.to_string()]))?;
    let (last, parent) = path.split_last().expect("a located block has a path");
    let container = if parent.is_empty() {
        blocks
    } else {
        match node_at_mut(blocks, parent) {
            Some(NoteBlockNode::Group { children, .. }) => children,
            _ => return Err(MutationApplyError::new("mutation.apply.invalid-target", "block container is not a group").at([id.to_string()])),
        }
    };
    Ok(container.remove(*last))
}

impl NoteBlocksDelta {
    /// 🧬️ Runs the rows over `blocks`; private so that only [`MutationDiff::apply`] (the central applier's entry) reaches it.
    fn write_into(&self, blocks: &[NoteBlockNode]) -> MutationApplyResult<Vec<NoteBlockNode>> {
        let mut next = blocks.to_vec();
        for (row, entry) in self.rows.iter().enumerate() {
            let under = |error: MutationApplyError| error.under(["rows".to_string(), row.to_string()]);
            match entry {
                NoteBlockRow::Add { parent_id, index, block } => {
                    if flatten_blocks(std::slice::from_ref(block)).into_iter().any(|added| find_block(&next, block_id(added)).is_some()) {
                        return Err(under(MutationApplyError::new("mutation.apply.duplicate-target", "added block tree contains an existing identity")));
                    }
                    insert_row(&mut next, parent_id.as_deref(), *index, block.clone()).map_err(under)?;
                }
                NoteBlockRow::Remove { id } => {
                    take_row(&mut next, id).map_err(under)?;
                }
                NoteBlockRow::Move { id, parent_id, index } => {
                    let moved = take_row(&mut next, id).map_err(under)?;
                    if parent_id.as_deref().is_some_and(|parent| find_block(std::slice::from_ref(&moved), parent).is_some()) {
                        return Err(under(MutationApplyError::new("mutation.apply.invalid-target", "a block cannot move into its own subtree")));
                    }
                    insert_row(&mut next, parent_id.as_deref(), *index, moved).map_err(under)?;
                }
                NoteBlockRow::Patch { id, patch } => {
                    let path = path_to(&next, id).ok_or_else(|| under(MutationApplyError::new("mutation.apply.missing-target", "patched block does not exist")))?;
                    let block = node_at_mut(&mut next, &path).expect("a located block has a node");
                    write_patch(block, patch).map_err(under)?;
                }
            }
        }
        let ids: Vec<&str> = flatten_blocks(&next).into_iter().map(block_id).collect();
        if ids.iter().enumerate().any(|(index, id)| ids[..index].contains(id)) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "resulting block tree contains duplicate identities").at(["identities"]));
        }
        Ok(next)
    }

    /// ➕️ Sequentially composes `later` after `self`: adjacent rows of one block coalesce (add∘remove cancels, add∘move and move∘move keep the last destination, move∘remove keeps the remove) and patches of one block merge across other blocks' patches.
    pub fn absorb(&mut self, later: Self) {
        for row in later.rows {
            self.push(row);
        }
        sort_patch_runs(&mut self.rows);
    }

    fn push(&mut self, row: NoteBlockRow) {
        match row {
            NoteBlockRow::Remove { id } => {
                while matches!(self.rows.last(), Some(NoteBlockRow::Patch { id: patched, .. }) if patched == &id) {
                    self.rows.pop();
                }
                match self.rows.last() {
                    Some(NoteBlockRow::Add { block, .. }) if block_id(block) == id => {
                        self.rows.pop();
                    }
                    Some(NoteBlockRow::Move { id: moved, .. }) if moved == &id => {
                        self.rows.pop();
                        self.rows.push(NoteBlockRow::Remove { id });
                    }
                    _ => self.rows.push(NoteBlockRow::Remove { id }),
                }
            }
            NoteBlockRow::Move { id, parent_id, index } => match self.rows.last_mut() {
                Some(NoteBlockRow::Add { block, parent_id: parent, index: at }) if block_id(block) == id => {
                    *parent = parent_id;
                    *at = index;
                }
                Some(NoteBlockRow::Move { id: moved, parent_id: parent, index: at }) if moved == &id => {
                    *parent = parent_id;
                    *at = index;
                }
                _ => self.rows.push(NoteBlockRow::Move { id, parent_id, index }),
            },
            NoteBlockRow::Patch { id, patch } => {
                let mut cursor = self.rows.len();
                while cursor > 0 {
                    match &mut self.rows[cursor - 1] {
                        NoteBlockRow::Patch { id: prior, patch: prior_patch } if prior == &id => {
                            prior_patch.merge(patch);
                            return;
                        }
                        NoteBlockRow::Patch { .. } => cursor -= 1,
                        _ => break,
                    }
                }
                self.rows.push(NoteBlockRow::Patch { id, patch });
            }
            add @ NoteBlockRow::Add { .. } => self.rows.push(add),
        }
    }

    /// 🔁️ The negative delta against `base`: the reversed rows that put every block back where it was, patches restoring the pre-patch values.
    pub fn negative(&self, base: &[NoteBlockNode]) -> Self {
        let mut current = base.to_vec();
        let mut undo = Vec::new();
        for row in &self.rows {
            match row {
                NoteBlockRow::Add { parent_id, index, block } => {
                    undo.push(NoteBlockRow::Remove { id: block_id(block).to_string() });
                    let _ = insert_row(&mut current, parent_id.as_deref(), *index, block.clone());
                }
                NoteBlockRow::Remove { id } => {
                    if let Some((parent_id, index)) = find_block_location(&current, id) {
                        if let Ok(block) = take_row(&mut current, id) {
                            undo.push(NoteBlockRow::Add { parent_id, index, block });
                        }
                    }
                }
                NoteBlockRow::Move { id, parent_id, index } => {
                    if let Some((origin, position)) = find_block_location(&current, id) {
                        undo.push(NoteBlockRow::Move { id: id.clone(), parent_id: origin, index: position });
                        if let Ok(block) = take_row(&mut current, id) {
                            let _ = insert_row(&mut current, parent_id.as_deref(), *index, block);
                        }
                    }
                }
                NoteBlockRow::Patch { id, patch } => {
                    if let Some(path) = path_to(&current, id) {
                        if let Some(block) = node_at_mut(&mut current, &path) {
                            undo.push(NoteBlockRow::Patch { id: id.clone(), patch: patch.restoring(block) });
                            let _ = write_patch(block, patch);
                        }
                    }
                }
            }
        }
        undo.reverse();
        sort_patch_runs(&mut undo);
        Self { rows: undo }
    }

    /// 🧭️ The delta from `base` to `other` for sync and import: root blocks that differ are removed and re-added, kept ones are moved into `other`'s root order.
    pub fn between(base: &[NoteBlockNode], other: &[NoteBlockNode]) -> Self {
        let kept = |block: &NoteBlockNode| other.iter().any(|candidate| candidate == block);
        let mut rows: Vec<NoteBlockRow> = base.iter().filter(|block| !kept(block)).map(|block| NoteBlockRow::Remove { id: block_id(block).to_string() }).collect();
        let mut order: Vec<&str> = base.iter().filter(|block| kept(block)).map(block_id).collect();
        for (index, target) in other.iter().enumerate() {
            if order.get(index) == Some(&block_id(target)) {
                continue;
            }
            match order.iter().position(|id| *id == block_id(target)) {
                Some(position) => {
                    let id = order.remove(position);
                    order.insert(index, id);
                    rows.push(NoteBlockRow::Move { id: id.to_string(), parent_id: None, index });
                }
                None => {
                    order.insert(index, block_id(target));
                    rows.push(NoteBlockRow::Add { parent_id: None, index, block: target.clone() });
                }
            }
        }
        Self { rows }
    }

    /// 🕳️ Whether the delta names no row.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

fn sort_patch_runs(rows: &mut [NoteBlockRow]) {
    let mut start = 0;
    while start < rows.len() {
        if !matches!(rows[start], NoteBlockRow::Patch { .. }) {
            start += 1;
            continue;
        }
        let mut end = start;
        while end < rows.len() && matches!(rows[end], NoteBlockRow::Patch { .. }) {
            end += 1;
        }
        rows[start..end].sort_by(|a, b| match (a, b) {
            (NoteBlockRow::Patch { id: left, .. }, NoteBlockRow::Patch { id: right, .. }) => left.cmp(right),
            _ => std::cmp::Ordering::Equal,
        });
        start = end;
    }
}
//#endregion 🔖️Tree

//#region 🔖️Assets
/// 🧩 Composes two rows of one asset key: `Ok(Some(row))` replaces the earlier row, `Ok(None)` cancels both, `Err(later)` hands the later row back when the pair is impossible.
fn coalesce_asset_rows(earlier: &NoteAssetRow, later: NoteAssetRow) -> Result<Option<NoteAssetRow>, NoteAssetRow> {
    match (earlier, later) {
        (NoteAssetRow::Insert { key, .. }, NoteAssetRow::Replace { asset, .. }) => Ok(Some(NoteAssetRow::Insert { key: key.clone(), asset })),
        (NoteAssetRow::Insert { .. }, NoteAssetRow::Remove { .. }) => Ok(None),
        (NoteAssetRow::Replace { key, .. }, NoteAssetRow::Replace { asset, .. }) => Ok(Some(NoteAssetRow::Replace { key: key.clone(), asset })),
        (NoteAssetRow::Replace { key, .. }, NoteAssetRow::Remove { .. }) => Ok(Some(NoteAssetRow::Remove { key: key.clone() })),
        (NoteAssetRow::Remove { key }, NoteAssetRow::Insert { asset, .. }) => Ok(Some(NoteAssetRow::Replace { key: key.clone(), asset })),
        (_, rejected) => Err(rejected),
    }
}

impl NoteAssetsDelta {
    fn write_into(&self, assets: &BTreeMap<String, NoteImageAsset>) -> MutationApplyResult<BTreeMap<String, NoteImageAsset>> {
        let mut next = assets.clone();
        for (row, entry) in self.rows.iter().enumerate() {
            let at = ["rows".to_string(), row.to_string(), entry.key().to_string()];
            match entry {
                NoteAssetRow::Insert { key, asset } => {
                    if next.insert(key.clone(), asset.clone()).is_some() {
                        return Err(MutationApplyError::new("mutation.apply.duplicate-target", "inserted asset key already exists").at(at));
                    }
                }
                NoteAssetRow::Replace { key, asset } => {
                    if next.insert(key.clone(), asset.clone()).is_none() {
                        return Err(MutationApplyError::new("mutation.apply.missing-target", "replaced asset does not exist").at(at));
                    }
                }
                NoteAssetRow::Remove { key } => {
                    if next.remove(key).is_none() {
                        return Err(MutationApplyError::new("mutation.apply.missing-target", "removed asset does not exist").at(at));
                    }
                }
            }
        }
        Ok(next)
    }

    /// ➕️ Sequentially composes `later` after `self` per key: insert∘replace is one insert, insert∘remove cancels, replace∘replace keeps the last, replace∘remove is a remove, remove∘insert is a replace; an impossible pair stays as two rows so the sequence keeps being rejected.
    pub fn absorb(&mut self, later: Self) {
        for row in later.rows {
            let Some(position) = self.rows.iter().position(|prior| prior.key() == row.key()) else {
                self.rows.push(row);
                continue;
            };
            match coalesce_asset_rows(&self.rows[position], row) {
                Ok(Some(merged)) => self.rows[position] = merged,
                Ok(None) => {
                    self.rows.remove(position);
                }
                Err(rejected) => self.rows.push(rejected),
            }
        }
        self.rows.sort_by(|a, b| a.key().cmp(b.key()));
    }

    /// 🔁️ The negative delta against `base`: each row undone against the value it displaced, rows kept in key order.
    pub fn negative(&self, base: &BTreeMap<String, NoteImageAsset>) -> Self {
        let mut current = base.clone();
        let mut undo = Vec::new();
        for entry in &self.rows {
            match entry {
                NoteAssetRow::Insert { key, asset } => {
                    undo.push(NoteAssetRow::Remove { key: key.clone() });
                    current.insert(key.clone(), asset.clone());
                }
                NoteAssetRow::Replace { key, asset } => {
                    if let Some(prior) = current.insert(key.clone(), asset.clone()) {
                        undo.push(NoteAssetRow::Replace { key: key.clone(), asset: prior });
                    }
                }
                NoteAssetRow::Remove { key } => {
                    if let Some(prior) = current.remove(key) {
                        undo.push(NoteAssetRow::Insert { key: key.clone(), asset: prior });
                    }
                }
            }
        }
        undo.reverse();
        undo.sort_by(|a, b| a.key().cmp(b.key()));
        Self { rows: undo }
    }

    /// 🧭️ The delta from `base` to `other` for sync and import.
    pub fn between(base: &BTreeMap<String, NoteImageAsset>, other: &BTreeMap<String, NoteImageAsset>) -> Self {
        let mut rows = Vec::new();
        for (key, asset) in other {
            match base.get(key) {
                None => rows.push(NoteAssetRow::Insert { key: key.clone(), asset: asset.clone() }),
                Some(prior) if prior != asset => rows.push(NoteAssetRow::Replace { key: key.clone(), asset: asset.clone() }),
                Some(_) => {}
            }
        }
        rows.extend(base.keys().filter(|key| !other.contains_key(*key)).map(|key| NoteAssetRow::Remove { key: key.clone() }));
        rows.sort_by(|a, b| a.key().cmp(b.key()));
        Self { rows }
    }

    /// 🕳️ Whether the delta names no row.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
//#endregion 🔖️Assets

//#region 🔖️Builders
impl NoteDiff {
    /// ➕️ A diff carrying the given block-tree rows.
    pub fn block_rows(rows: Vec<NoteBlockRow>) -> Self {
        Self { blocks: Some(NoteBlocksDelta { rows }), ..Default::default() }
    }

    /// 🩹 A diff patching the given blocks, one row each, in id order.
    pub fn block_patches(entries: impl IntoIterator<Item = (String, NoteBlockPatch)>) -> Self {
        let mut rows: Vec<NoteBlockRow> = entries.into_iter().map(|(id, patch)| NoteBlockRow::Patch { id, patch }).collect();
        sort_patch_runs(&mut rows);
        Self::block_rows(rows)
    }

    /// 🖼️ A diff carrying the given asset rows.
    pub fn asset_rows(rows: Vec<NoteAssetRow>) -> Self {
        Self { assets: Some(NoteAssetsDelta { rows }), ..Default::default() }
    }
}
//#endregion 🔖️Builders

//#region 🔖️Apply
impl MutationDiff<NoteSnapshot> for NoteDiff {
    fn apply(&self, base: &NoteSnapshot, _capability: ApplyCapability) -> MutationApplyResult<NoteSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(title) = &self.title {
            next.title = title.value.clone();
        }
        if let Some(delta) = &self.blocks {
            next.blocks = delta.write_into(&base.blocks).map_err(|error| error.under(["blocks"]))?;
        }
        macro_rules! assign {
            ($($field:ident),+) => {
                $(if let Some(assigned) = &self.$field {
                    next.$field = assigned.value;
                })+
            };
        }
        assign!(grid_visible, grid_spacing, grid_subdivisions, grid_opacity, snap_enabled, snap_grid_spacing, pencil_width, eraser_radius);
        if let Some(delta) = &self.assets {
            next.assets = delta.write_into(&base.assets).map_err(|error| error.under(["assets"]))?;
        }
        if let Some(link) = &self.linked_artifact {
            next.linked_artifact = link.value.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($($field:ident),+) => {
                $(if other.$field.is_some() {
                    self.$field = other.$field;
                })+
            };
        }
        take!(schema, id, title, grid_visible, grid_spacing, grid_subdivisions, grid_opacity, snap_enabled, snap_grid_spacing, pencil_width, eraser_radius, linked_artifact);
        match (self.blocks.as_mut(), other.blocks) {
            (Some(prior), Some(later)) => prior.absorb(later),
            (None, Some(later)) => self.blocks = Some(later),
            _ => {}
        }
        match (self.assets.as_mut(), other.assets) {
            (Some(prior), Some(later)) => prior.absorb(later),
            (None, Some(later)) => self.assets = Some(later),
            _ => {}
        }
    }
}

impl DiffAlgebra<NoteSnapshot> for NoteDiff {
    fn inverse(&self, base: &NoteSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            id: self.id.as_ref().map(|_| base.id.clone()),
            title: self.title.as_ref().map(|_| NoteAssigned::new(base.title.clone())),
            blocks: self.blocks.as_ref().map(|delta| delta.negative(&base.blocks)),
            grid_visible: self.grid_visible.as_ref().map(|_| NoteAssigned::new(base.grid_visible)),
            grid_spacing: self.grid_spacing.as_ref().map(|_| NoteAssigned::new(base.grid_spacing)),
            grid_subdivisions: self.grid_subdivisions.as_ref().map(|_| NoteAssigned::new(base.grid_subdivisions)),
            grid_opacity: self.grid_opacity.as_ref().map(|_| NoteAssigned::new(base.grid_opacity)),
            snap_enabled: self.snap_enabled.as_ref().map(|_| NoteAssigned::new(base.snap_enabled)),
            snap_grid_spacing: self.snap_grid_spacing.as_ref().map(|_| NoteAssigned::new(base.snap_grid_spacing)),
            pencil_width: self.pencil_width.as_ref().map(|_| NoteAssigned::new(base.pencil_width)),
            eraser_radius: self.eraser_radius.as_ref().map(|_| NoteAssigned::new(base.eraser_radius)),
            assets: self.assets.as_ref().map(|delta| delta.negative(&base.assets)),
            linked_artifact: self.linked_artifact.as_ref().map(|_| NoteAssigned::new(base.linked_artifact.clone())),
        }
    }

    fn between(base: &NoteSnapshot, other: &NoteSnapshot) -> Self {
        let blocks = NoteBlocksDelta::between(&base.blocks, &other.blocks);
        let assets = NoteAssetsDelta::between(&base.assets, &other.assets);
        macro_rules! differs {
            ($field:ident) => {
                (base.$field != other.$field).then(|| NoteAssigned::new(other.$field.clone()))
            };
        }
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            id: (base.id != other.id).then(|| other.id.clone()),
            title: differs!(title),
            blocks: (!blocks.is_empty()).then_some(blocks),
            grid_visible: differs!(grid_visible),
            grid_spacing: differs!(grid_spacing),
            grid_subdivisions: differs!(grid_subdivisions),
            grid_opacity: differs!(grid_opacity),
            snap_enabled: differs!(snap_enabled),
            snap_grid_spacing: differs!(snap_grid_spacing),
            pencil_width: differs!(pencil_width),
            eraser_radius: differs!(eraser_radius),
            assets: (!assets.is_empty()).then_some(assets),
            linked_artifact: differs!(linked_artifact),
        }
    }

    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
//#endregion 🔖️Apply

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::NoteArtifact;
//#endregion 🔁️Re-exports

#[cfg(test)]
#[path = "🧪️tests/🔬️diff-apply/🦀️.rs"]
mod diff_apply_tests;
