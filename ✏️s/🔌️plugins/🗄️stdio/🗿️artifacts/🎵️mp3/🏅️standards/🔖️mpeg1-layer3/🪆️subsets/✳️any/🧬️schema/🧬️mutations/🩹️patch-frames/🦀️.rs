//! 🩹️ `patch-frames` — one splice of the MPEG frame lane at frame boundaries (trim, cut, insert or replace a range), with the exact inverse built from the replaced frames.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct PatchFrames {
    pub index: u64,
    pub remove_count: u64,
    #[value(default)]
    pub frames: Vec<Mp3Frame>,
}

impl PatchFrames {
    /// ✂️ The range this patch replaces in `base`'s frame lane, or why it leaves the lane.
    fn range(&self, base: &Mp3Snapshot) -> Result<std::ops::Range<usize>, String> {
        let start = usize::try_from(self.index).map_err(|_| "MP3 frame index exceeds this platform".to_string())?;
        let count = usize::try_from(self.remove_count).map_err(|_| "MP3 frame removal count exceeds this platform".to_string())?;
        let end = start.checked_add(count).filter(|end| *end <= base.frames.len()).ok_or_else(|| "MP3 frame patch is outside the frame lane".to_string())?;
        Ok(start..end)
    }
}

impl protocol::MutationKind<Mp3Snapshot, Mp3Mutation> for PatchFrames {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "frames", kind: "patch-frames", record: "PatchFrames" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Patch frames", "Frames bearbeiten")
    }
    fn target(&self) -> Vec<String> {
        vec!["frames".into(), self.index.to_string()]
    }
}

impl protocol::ColdMutationKind<Mp3Snapshot, Mp3Mutation> for PatchFrames {
    fn diff(&self, base: &Mp3Snapshot) -> protocol::MutationOutcome<<Mp3Mutation as protocol::ColdMutation<Mp3Snapshot>>::ReferenceDiff> {
        match self.range(base) {
            Ok(range) if self.frames.is_empty() && range.is_empty() || base.frames[range.clone()] == self.frames[..] => protocol::MutationOutcome::new(Mp3Diff::default()),
            Ok(_) => protocol::MutationOutcome::new(diff_patch_frames(self.index, self.remove_count, self.frames.clone())),
            Err(message) => protocol::MutationOutcome::error("mutation.target-mismatch", message, ["frames".into(), self.index.to_string()]),
        }
    }
    fn inverse(&self, base: &Mp3Snapshot) -> Result<Vec<Mp3Mutation>, semio_framework_value::ValueError> {
        let Ok(range) = self.range(base) else { return Ok(Vec::new()) };
        if self.frames.is_empty() && range.is_empty() || base.frames[range.clone()] == self.frames[..] {
            return Ok(Vec::new());
        }
        Ok(vec![Mp3Mutation::PatchFrames(PatchFrames { index: self.index, remove_count: self.frames.len() as u64, frames: base.frames[range].to_vec() })])
    }
}
//#endregion 🔖️Payload
