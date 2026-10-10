//! 🎼️ `set-frames` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFrames {
    pub frames: Vec<Mp3Frame>,
}

impl protocol::MutationKind<Mp3Snapshot, Mp3Mutation> for SetFrames {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "frames", kind: "set-frames", record: "SetFrames" };

    fn diff(&self, base: &Mp3Snapshot) -> protocol::MutationOutcome<<Mp3Mutation as Mutation<Mp3Snapshot>>::Diff> {
        let Self { frames } = self;
        protocol::MutationOutcome::new(diff_set_frames(frames.clone()))
    }
    fn inverse(&self, base: &Mp3Snapshot) -> Result<Vec<Mp3Mutation>, semio_framework_value::ValueError> {
        Ok(vec![Mp3Mutation::SetFrames(set_frames::SetFrames { frames: base.frames.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set frames", "Frames setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
