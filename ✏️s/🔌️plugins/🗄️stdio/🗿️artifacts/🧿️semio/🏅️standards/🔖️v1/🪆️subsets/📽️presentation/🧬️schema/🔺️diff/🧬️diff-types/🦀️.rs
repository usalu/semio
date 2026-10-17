//! 🧬️ Presentation diff types: the collection diff aliases, the sparse frame, picture, table, shape, master, layout and slide diff records and the presentation diff. See [`super`].

use super::*;

//#region 🔖️CollectionDiffAliases
pub type SlideShapesDiff = IndexedTripleDiff<SlideShapeDiff, SlideShape>;

/// 🧱️ `document::DocBlock` treated as its own diff (`D = T`) — see module doc comment.
pub type DocBlocksDiff = IndexedTripleDiff<DocBlock, DocBlock>;

pub type SlideTableRowsDiff = IndexedTripleDiff<SlideTableRowDiff, SlideTableRow>;

pub type SlideTableCellsDiff = IndexedTripleDiff<SlideTableCellDiff, SlideTableCell>;

pub type SlideMastersDiff = NamedTripleDiff<String, SlideMasterDiff, NamedAdded<SlideMaster>>;

pub type SlideLayoutsDiff = NamedTripleDiff<String, SlideLayoutDiff, NamedAdded<SlideLayout>>;

pub type SlidesDiff = IndexedTripleDiff<SlideDiff, Slide>;
//#endregion 🔖️CollectionDiffAliases

//#region 🔖️DiffTypes
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideFrameDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<SemioPoint2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlidePictureImageDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideTableCellDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<DocBlocksDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideTableRowDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cells: Option<SlideTableCellsDiff>,
}

/// 🌳️ Per-shape diff, shaped like `SlideShape` (`Replace` covers a shape-KIND change, e.g.
/// `TextBox` -> `Picture`, same convention as docx's `DocxBlockDiff::Replace`). Tag is
/// `shapeKind` (not `kind`) for the same field/tag-collision reason as `SlideShape` itself (see
/// that type's doc comment) — `Placeholder`'s own `kind` field would otherwise collide.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "shapeKind", rename_all = "camelCase")]
pub enum SlideShapeDiff {
    TextBox {
        #[value(default, skip_serializing_if = "Option::is_none")]
        frame: Option<SlideFrameDiff>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        blocks: Option<DocBlocksDiff>,
    },
    Picture {
        #[value(default, skip_serializing_if = "Option::is_none")]
        frame: Option<SlideFrameDiff>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        image: Option<SlidePictureImageDiff>,
    },
    Table {
        #[value(default, skip_serializing_if = "Option::is_none")]
        frame: Option<SlideFrameDiff>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        rows: Option<SlideTableRowsDiff>,
    },
    Placeholder {
        #[value(default, skip_serializing_if = "Option::is_none")]
        frame: Option<SlideFrameDiff>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        kind: Option<PlaceholderKind>,
    },
    Replace {
        shape: SlideShape,
    },
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideMasterDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shapes: Option<SlideShapesDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideLayoutDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub master_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shapes: Option<SlideShapesDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SlideDiff {
    /// 🪪️ The slide's own identity at this index, when it changes: an index-keyed slot whose slide is
    /// replaced by a different one (a reorder, a whole-deck `set-snapshot`) takes the new slide's `id`
    /// with its content, so applying `between(base, next)` to `base` yields exactly `next`.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = layout cleared, `Some(Some(id))` = set.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layout_id: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shapes: Option<SlideShapesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<DocBlocksDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.presentation.diff")]
pub struct SemioPresentationDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub masters: Option<SlideMastersDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layouts: Option<SlideLayoutsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub slides: Option<SlidesDiff>,
}
//#endregion 🔖️GenericNamedEngine
