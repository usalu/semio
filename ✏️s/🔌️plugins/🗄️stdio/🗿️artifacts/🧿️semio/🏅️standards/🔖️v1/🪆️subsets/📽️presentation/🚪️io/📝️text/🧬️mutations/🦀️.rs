//! 📝️ Text representation codec surface for `stdio.semio.presentation` (mutations).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::presentation::schema::mutations::*;
use crate::standards::v1::subsets::document::schema::snapshot::DocBlock;
use crate::standards::v1::subsets::presentation::schema::diff::{diff_insert_layout, diff_insert_master, diff_insert_shape, diff_insert_slide, diff_remove_layout, diff_remove_master, diff_remove_shape, diff_remove_slide, diff_set_layout_master, diff_set_shape_frame, diff_set_slide_layout, diff_set_slide_notes, diff_set_snapshot, diff_set_textbox_blocks, frame_of, SemioPresentationDiff};
use crate::standards::v1::subsets::presentation::io::text::diff::{dec_shape};
use crate::standards::v1::subsets::presentation::io::text::diff::{enc_shape};
use crate::standards::v1::subsets::presentation::io::text::diff::{dec_slide};
use crate::standards::v1::subsets::presentation::io::text::diff::{enc_slide};
use crate::standards::v1::subsets::presentation::io::text::diff::{dec_layout};
use crate::standards::v1::subsets::presentation::io::text::diff::{enc_layout};
use crate::standards::v1::subsets::presentation::io::text::diff::{dec_master};
use crate::standards::v1::subsets::presentation::io::text::diff::{enc_master};
use crate::standards::v1::subsets::presentation::io::text::diff::{dec_frame};
use crate::standards::v1::subsets::presentation::io::text::diff::{enc_frame};
use crate::standards::v1::subsets::document::io::text::diff::{dec_block};
use crate::standards::v1::subsets::document::io::text::diff::{enc_block};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{decode_option};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{encode_option};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::presentation::schema::snapshot::{SemioPresentationSnapshot, Slide, SlideFrame, SlideLayout, SlideMaster, SlideShape};
/// 🔧️ `OpBinary`/`OpText` both unconditional (not `#[cfg(test)]`-gated): the real
/// `impl protocol::OpBinary for SemioPresentationMutation` below (production code) calls
/// `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs both traits in scope.
use protocol::{Mutation, OpBinary, OpText};

/// 🎙️ Hand-rolled `OpText`/`OpBinary` (same reasoning as `DocxMutation`'s: the payload types are
/// data-carrying enums the `dsl::DslOps` derive cannot bridge) — reuses the diff file's
/// `pub(crate)` grammar primitives rather than duplicating them. Grammar: `keyword arg=value ...`
/// (space-separated), matching the docx/gif/svg convention. `no-mutation` is no longer a keyword
/// this codec parses (there is nothing left to construct for it); a `🧪️tests/mutate-*` adapter that
/// must still honor the `no-mutation` scenario id maps it to the identity `set-snapshot` mutation
/// itself, ahead of this codec.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_presentation_mutation(m: &SemioPresentationMutation) -> String {
    match m {
        SemioPresentationMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", crate::standards::v1::subsets::presentation::io::text::snapshot::enc_presentation_snapshot(snapshot)),
        SemioPresentationMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),
        SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index, slide }) => format!("insert-slide index={index} slide={}", enc_slide(slide)),
        SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index }) => format!("remove-slide index={index}"),
        SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index, layout_id }) => format!("set-slide-layout index={index} layout-id={}", encode_option(layout_id, |v| enc_str(v))),
        SemioPresentationMutation::SetSlideNotes(set_slide_notes::SetSlideNotes { index, notes }) => format!("set-slide-notes index={index} notes={}", enc_list(notes, enc_block)),
        SemioPresentationMutation::InsertShape(insert_shape::InsertShape { slide_index, shape_index, shape }) => format!("insert-shape slide-index={slide_index} shape-index={shape_index} shape={}", enc_shape(shape)),
        SemioPresentationMutation::RemoveShape(remove_shape::RemoveShape { slide_index, shape_index }) => format!("remove-shape slide-index={slide_index} shape-index={shape_index}"),
        SemioPresentationMutation::SetShapeFrame(set_shape_frame::SetShapeFrame { slide_index, shape_index, frame }) => format!("set-shape-frame slide-index={slide_index} shape-index={shape_index} frame={}", enc_frame(frame)),
        SemioPresentationMutation::SetTextBoxBlocks(set_textbox_blocks::SetTextBoxBlocks { slide_index, shape_index, blocks }) => {
            format!("set-text-box-blocks slide-index={slide_index} shape-index={shape_index} blocks={}", enc_list(blocks, enc_block))
        }
        SemioPresentationMutation::InsertMaster(insert_master::InsertMaster { master }) => format!("insert-master master={}", enc_master(master)),
        SemioPresentationMutation::RemoveMaster(remove_master::RemoveMaster { id }) => format!("remove-master id={}", enc_str(id)),
        SemioPresentationMutation::InsertLayout(insert_layout::InsertLayout { layout }) => format!("insert-layout layout={}", enc_layout(layout)),
        SemioPresentationMutation::RemoveLayout(remove_layout::RemoveLayout { id }) => format!("remove-layout id={}", enc_str(id)),
        SemioPresentationMutation::SetLayoutMaster(set_layout_master::SetLayoutMaster { id, master_id }) => format!("set-layout-master id={} master-id={}", enc_str(id), enc_str(master_id)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_presentation_mutation(line: &str) -> Result<SemioPresentationMutation, String> {
    if let Some(source) = line.strip_prefix("patch-snapshot patch=") {
        let patch = semio_s_artifact_stdio_contract::editing::snapshot_patch_from_hex(source)?;
        return Ok(SemioPresentationMutation::PatchSnapshot(crate::standards::v1::subsets::presentation::schema::mutations::patch_snapshot::PatchSnapshot { patch }));
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> =
        rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("presentation mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("presentation mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(SemioPresentationMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: crate::standards::v1::subsets::presentation::io::text::snapshot::dec_presentation_snapshot(arg("snapshot")?)? })),
        "insert-slide" => Ok(SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index: usize_arg("index")?, slide: dec_slide(arg("slide")?)? })),
        "remove-slide" => Ok(SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index: usize_arg("index")? })),
        "set-slide-layout" => Ok(SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index: usize_arg("index")?, layout_id: decode_option(arg("layout-id")?, dec_str)? })),
        "set-slide-notes" => Ok(SemioPresentationMutation::SetSlideNotes(set_slide_notes::SetSlideNotes { index: usize_arg("index")?, notes: dec_list(arg("notes")?, dec_block)? })),
        "insert-shape" => Ok(SemioPresentationMutation::InsertShape(insert_shape::InsertShape { slide_index: usize_arg("slide-index")?, shape_index: usize_arg("shape-index")?, shape: dec_shape(arg("shape")?)? })),
        "remove-shape" => Ok(SemioPresentationMutation::RemoveShape(remove_shape::RemoveShape { slide_index: usize_arg("slide-index")?, shape_index: usize_arg("shape-index")? })),
        "set-shape-frame" => Ok(SemioPresentationMutation::SetShapeFrame(set_shape_frame::SetShapeFrame { slide_index: usize_arg("slide-index")?, shape_index: usize_arg("shape-index")?, frame: dec_frame(arg("frame")?)? })),
        "set-text-box-blocks" => Ok(SemioPresentationMutation::SetTextBoxBlocks(set_textbox_blocks::SetTextBoxBlocks { slide_index: usize_arg("slide-index")?, shape_index: usize_arg("shape-index")?, blocks: dec_list(arg("blocks")?, dec_block)? })),
        "insert-master" => Ok(SemioPresentationMutation::InsertMaster(insert_master::InsertMaster { master: dec_master(arg("master")?)? })),
        "remove-master" => Ok(SemioPresentationMutation::RemoveMaster(remove_master::RemoveMaster { id: dec_str(arg("id")?)? })),
        "insert-layout" => Ok(SemioPresentationMutation::InsertLayout(insert_layout::InsertLayout { layout: dec_layout(arg("layout")?)? })),
        "remove-layout" => Ok(SemioPresentationMutation::RemoveLayout(remove_layout::RemoveLayout { id: dec_str(arg("id")?)? })),
        "set-layout-master" => Ok(SemioPresentationMutation::SetLayoutMaster(set_layout_master::SetLayoutMaster { id: dec_str(arg("id")?)?, master_id: dec_str(arg("master-id")?)? })),
        other => Err(format!("presentation mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioPresentationMutation {
    fn print_op(&self) -> String {
        print_presentation_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_presentation_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
