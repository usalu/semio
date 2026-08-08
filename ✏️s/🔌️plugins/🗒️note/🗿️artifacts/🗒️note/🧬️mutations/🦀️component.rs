//! 🧬️ Note artifact — document mutation dispatch.

use crate::artifacts::note::{NoteBlockNode, NoteDocument, NoteImageAsset};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for a `NoteDocument`. Every content change flows through one of these so
/// the `DocumentStore` records a true inverse (`backwards`). Scalar setters carry the field's own
/// `Option` shape (backwards is a plain prior-value read); block edits use a whole-tree `SetBlocks`
/// snapshot (the recursive reid/clone tree makes per-node operations far messier than a snapshot); asset
/// and full-document loads have dedicated variants.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum NoteMutation {
    SetGridVisible {
        visible: Option<bool>,
    },
    SetGridSpacing {
        spacing: Option<f64>,
    },
    SetGridSubdivisions {
        value: Option<f64>,
    },
    SetGridOpacity {
        opacity: Option<f64>,
    },
    SetSnapEnabled {
        enabled: Option<bool>,
    },
    SetSnapGridSpacing {
        spacing: Option<f64>,
    },
    SetPencilWidth {
        width: Option<f64>,
    },
    SetEraserRadius {
        radius: Option<f64>,
    },
    SetBlocks {
        #[dsl(statements, block)]
        blocks: Vec<NoteBlockNode>,
    },
    PutAsset {
        key: String,
        #[dsl(block)]
        asset: NoteImageAsset,
    },
    /// 🗑️ True composed inverse of a `PutAsset` that introduced the key — the whole-document
    /// `SetDocument` snapshot this used to invert to would have undone every other pending change too.
    RemoveAsset {
        key: String,
    },
    SetDocument {
        #[dsl(block)]
        document: NoteDocument,
    },
}




/// ▶️ Applies `operation` to `projection`, producing the next document — shared by `NoteDiff::apply` and
/// this file's own `Mutation::diff`/tests.
pub fn apply_note_mutation(projection: &NoteDocument, operation: &NoteMutation) -> NoteDocument {
    let mut next = projection.clone();
    match operation {
        NoteMutation::SetGridVisible { visible } => next.grid_visible = *visible,
        NoteMutation::SetGridSpacing { spacing } => next.grid_spacing = *spacing,
        NoteMutation::SetGridSubdivisions { value } => next.grid_subdivisions = *value,
        NoteMutation::SetGridOpacity { opacity } => next.grid_opacity = *opacity,
        NoteMutation::SetSnapEnabled { enabled } => next.snap_enabled = *enabled,
        NoteMutation::SetSnapGridSpacing { spacing } => next.snap_grid_spacing = *spacing,
        NoteMutation::SetPencilWidth { width } => next.pencil_width = *width,
        NoteMutation::SetEraserRadius { radius } => next.eraser_radius = *radius,
        NoteMutation::SetBlocks { blocks } => next.blocks = blocks.clone(),
        NoteMutation::PutAsset { key, asset } => {
            next.assets.insert(key.clone(), asset.clone());
        }
        NoteMutation::RemoveAsset { key } => {
            next.assets.remove(key);
        }
        NoteMutation::SetDocument { document } => next = document.clone(),
    }
    next
}

impl Mutation<NoteDocument> for NoteMutation {
    type Diff = crate::artifacts::note::diff::NoteDiff;

    fn diff(&self, _projection: &NoteDocument) -> Self::Diff {
        crate::artifacts::note::diff::NoteDiff { operation: Some(self.clone()) }
    }

    fn inverse(&self, projection: &NoteDocument) -> Vec<Self> {
        match self {
            NoteMutation::SetGridVisible { .. } => vec![NoteMutation::SetGridVisible { visible: projection.grid_visible }],
            NoteMutation::SetGridSpacing { .. } => vec![NoteMutation::SetGridSpacing { spacing: projection.grid_spacing }],
            NoteMutation::SetGridSubdivisions { .. } => vec![NoteMutation::SetGridSubdivisions { value: projection.grid_subdivisions }],
            NoteMutation::SetGridOpacity { .. } => vec![NoteMutation::SetGridOpacity { opacity: projection.grid_opacity }],
            NoteMutation::SetSnapEnabled { .. } => vec![NoteMutation::SetSnapEnabled { enabled: projection.snap_enabled }],
            NoteMutation::SetSnapGridSpacing { .. } => vec![NoteMutation::SetSnapGridSpacing { spacing: projection.snap_grid_spacing }],
            NoteMutation::SetPencilWidth { .. } => vec![NoteMutation::SetPencilWidth { width: projection.pencil_width }],
            NoteMutation::SetEraserRadius { .. } => vec![NoteMutation::SetEraserRadius { radius: projection.eraser_radius }],
            NoteMutation::SetBlocks { .. } => vec![NoteMutation::SetBlocks { blocks: projection.blocks.clone() }],
            // 🗑️ Composed, not a snapshot: restores the prior value at `key` if one existed, else removes
            // the key that didn't exist before this `PutAsset` — every other asset/field is untouched.
            NoteMutation::PutAsset { key, .. } => match projection.assets.get(key) {
                Some(prior) => vec![NoteMutation::PutAsset { key: key.clone(), asset: prior.clone() }],
                None => vec![NoteMutation::RemoveAsset { key: key.clone() }],
            },
            NoteMutation::RemoveAsset { key } => match projection.assets.get(key) {
                Some(prior) => vec![NoteMutation::PutAsset { key: key.clone(), asset: prior.clone() }],
                // Removing a key that was already absent is a no-op — nothing to restore.
                None => Vec::new(),
            },
            NoteMutation::SetDocument { .. } => vec![NoteMutation::SetDocument { document: projection.clone() }],
        }
    }
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_text_round_trips_every_variant() {
        use crate::artifacts::note::{NoteBlockNode, NoteImageAsset};

        store::test_support::assert_op_text_binary_equivalence(&NoteMutation::SetGridVisible { visible: Some(true) });
        store::test_support::assert_op_text_binary_equivalence(&NoteMutation::SetGridVisible { visible: None });
        store::test_support::assert_op_text_binary_equivalence(&NoteMutation::SetGridSpacing { spacing: Some(16.0) });
        store::test_support::assert_op_text_binary_equivalence(&NoteMutation::SetGridSpacing { spacing: None });
        store::test_support::assert_op_text_binary_equivalence(&NoteMutation::SetGridSubdivisions { value: Some(8.0) });
        store::test_support::assert_op_text_binary_equivalence(&NoteMutation::SetGridOpacity { opacity: Some(0.6) });
        store::test_support::assert_op_text_binary_equivalence(&NoteMutation::SetSnapEnabled { enabled: Some(false) });
        store::test_support::assert_op_text_binary_equivalence(&NoteMutation::SetSnapGridSpacing { spacing: Some(4.0) });
        store::test_support::assert_op_text_binary_equivalence(&NoteMutation::SetPencilWidth { width: Some(5.0) });
        store::test_support::assert_op_text_binary_equivalence(&NoteMutation::SetEraserRadius { radius: Some(20.0) });

        let stroke_with_points = NoteBlockNode::Ink {
            id: "stroke-1".into(),
            name: "Ink".into(),
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            points: vec![[0.0, 0.0], [1.5, 2.5], [3.0, -1.0]],
            stroke_width: 3.0,
            color: [0.0, 0.0, 0.0, 1.0],
        };
        let text_block = NoteBlockNode::Text {
            id: "text-1".into(),
            name: "Text".into(),
            x: 5.0,
            y: 5.0,
            width: 280.0,
            height: 120.0,
            rotation: 0.0,
            visible: true,
            locked: false,
            paragraphs: Vec::new(),
            font_size: 18.0,
            font_weight: "normal".into(),
            align: "left".into(),
        };
        store::test_support::assert_op_text_binary_equivalence(&NoteMutation::SetBlocks { blocks: vec![text_block, stroke_with_points] });

        store::test_support::assert_op_text_binary_equivalence(&NoteMutation::PutAsset { key: "asset-2".into(), asset: NoteImageAsset { mime: "image/jpeg".into(), data: "data:image/jpeg;base64,xyz".into(), width: None, height: None } });
        store::test_support::assert_op_text_binary_equivalence(&NoteMutation::RemoveAsset { key: "asset-2".into() });

        store::test_support::assert_op_text_binary_equivalence(&NoteMutation::SetDocument { document: crate::artifacts::note::engine::empty_note_document() });
    }

    #[test]
    fn operation_backwards_restores_pre_state() {
        let pre = crate::artifacts::note::engine::empty_note_document();
        store::test_support::assert_operation_round_trip(&pre, NoteMutation::SetGridSpacing { spacing: Some(48.0) });
    }

    /// 🗑️ `PutAsset`'s inverse must be composed (touch only the one key), not a whole-document
    /// `SetDocument` snapshot that would also clobber every other pending change.
    #[test]
    fn put_asset_backwards_is_composed_not_a_whole_document_snapshot() {
        use crate::artifacts::note::NoteImageAsset;

        let mut pre = crate::artifacts::note::engine::empty_note_document();
        pre.grid_spacing = Some(99.0); // an unrelated field that a snapshot inverse would wrongly revert too
        let asset = NoteImageAsset { mime: "image/png".into(), data: "data:image/png;base64,abc".into(), width: None, height: None };

        // Introducing a brand-new key inverts to removing it.
        let put_new = NoteMutation::PutAsset { key: "asset-1".into(), asset: asset.clone() };
        let inverse = put_new.inverse(&pre);
        assert_eq!(inverse, vec![NoteMutation::RemoveAsset { key: "asset-1".into() }]);
        let mut after_put = apply_note_mutation(&pre, &put_new);
        assert_eq!(after_put.assets.get("asset-1"), Some(&asset));
        for op in &inverse {
            after_put = apply_note_mutation(&after_put, op);
        }
        assert_eq!(after_put, pre, "undoing a fresh PutAsset must restore exactly the pre-state, unrelated fields included");

        // Replacing an existing key inverts to restoring the prior asset at that key.
        pre.assets.insert("asset-1".into(), asset.clone());
        let replacement = NoteImageAsset { mime: "image/jpeg".into(), data: "data:image/jpeg;base64,def".into(), width: None, height: None };
        let put_replace = NoteMutation::PutAsset { key: "asset-1".into(), asset: replacement.clone() };
        let inverse = put_replace.inverse(&pre);
        assert_eq!(inverse, vec![NoteMutation::PutAsset { key: "asset-1".into(), asset: asset.clone() }]);
        let mut after_replace = apply_note_mutation(&pre, &put_replace);
        assert_eq!(after_replace.assets.get("asset-1"), Some(&replacement));
        for op in &inverse {
            after_replace = apply_note_mutation(&after_replace, op);
        }
        assert_eq!(after_replace, pre);

        // Removing a key inverts to restoring it.
        let remove = NoteMutation::RemoveAsset { key: "asset-1".into() };
        let inverse = remove.inverse(&pre);
        assert_eq!(inverse, vec![NoteMutation::PutAsset { key: "asset-1".into(), asset }]);

        // Removing an already-absent key is a no-op with a no-op (empty) inverse.
        let remove_missing = NoteMutation::RemoveAsset { key: "never-existed".into() };
        assert_eq!(remove_missing.inverse(&pre), Vec::new());
    }
}
//#endregion 🧪️Tests

