//! 🧷️ `set-pad-bytes` — sets the two RIFF alignment bytes the `fmt ` and `data` chunks carry after an odd payload. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPadBytes {
    #[value(default)]
    pub fmt_pad_byte: u8,
    #[value(default)]
    pub data_pad_byte: u8,
}

impl SetPadBytes {
    /// 🔺️ The sparse diff this payload asks of `base`: each pad byte that differs.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn wanted(&self, base: &WavSnapshot) -> WavDiff {
        WavDiff { fmt_pad_byte: (base.fmt_pad_byte != self.fmt_pad_byte).then_some(self.fmt_pad_byte), data_pad_byte: (base.data_pad_byte != self.data_pad_byte).then_some(self.data_pad_byte), ..WavDiff::default() }
    }
}

impl protocol::MutationKind<WavSnapshot, WavMutation> for SetPadBytes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "pad-bytes", kind: "set-pad-bytes", record: "SetPadBytes" };

    fn diff(&self, base: &WavSnapshot) -> protocol::MutationOutcome<<WavMutation as Mutation<WavSnapshot>>::Diff> {
        match validate_wav_serialization(&WavSnapshot { fmt_pad_byte: self.fmt_pad_byte, data_pad_byte: self.data_pad_byte, ..base.clone() }) {
            Ok(()) => protocol::MutationOutcome::new(self.wanted(base)),
            Err(issue) => protocol::MutationOutcome::error("mutation.target-mismatch", format!("{}: {}", issue.code, issue.message), issue.target),
        }
    }
    fn inverse(&self, base: &WavSnapshot) -> Result<Vec<WavMutation>, semio_framework_value::ValueError> {
        Ok((!protocol::DiffAlgebra::is_empty(&self.wanted(base))).then(|| WavMutation::SetPadBytes(set_pad_bytes::SetPadBytes { fmt_pad_byte: base.fmt_pad_byte, data_pad_byte: base.data_pad_byte })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set pad bytes", "Füllbytes setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
