//! 🎚️ `set-fmt` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFmt {
    pub fmt: WavFmt,
}

impl protocol::MutationKind<WavSnapshot, WavMutation> for SetFmt {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "fmt", kind: "set-fmt", record: "SetFmt" };

    fn diff(&self, base: &WavSnapshot) -> protocol::MutationOutcome<<WavMutation as Mutation<WavSnapshot>>::Diff> {
        let Self { fmt } = self;
        match validate_wav_serialization(&WavSnapshot { fmt: fmt.clone(), ..base.clone() }) {
            Ok(()) => protocol::MutationOutcome::new(sparse_against(base, diff_set_fmt(fmt.clone()))),
            Err(issue) => protocol::MutationOutcome::error("mutation.target-mismatch", format!("{}: {}", issue.code, issue.message), issue.target),
        }
    }
    fn inverse(&self, base: &WavSnapshot) -> Result<Vec<WavMutation>, semio_framework_value::ValueError> {
        Ok((base.fmt != self.fmt).then(|| WavMutation::SetFmt(set_fmt::SetFmt { fmt: base.fmt.clone() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set fmt", "fmt-Chunk setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
