//! 🎨️ `set-stream-format` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetStreamFormat {
    pub stream_index: usize,
    pub strf: AviStreamFormat,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for SetStreamFormat {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "stream-format", kind: "set-stream-format", record: "SetStreamFormat" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as Mutation<AviSnapshot>>::Diff> {
        let Self { stream_index, strf } = self;
        protocol::MutationOutcome::new(stream_diff_for(*stream_index, AviStreamDiff { strf: Some(strf.clone()), ..AviStreamDiff::default() }))
    }
    fn inverse(&self, base: &AviSnapshot) -> Result<Vec<AviMutation>, semio_framework_value::ValueError> {
        let Self { stream_index, .. } = self;
        Ok({
            match base.streams.get(*stream_index) {
                Some(stream) => vec![AviMutation::SetStreamFormat(set_stream_format::SetStreamFormat { stream_index: *stream_index, strf: stream.strf.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set stream format", "Datenstromformat setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
