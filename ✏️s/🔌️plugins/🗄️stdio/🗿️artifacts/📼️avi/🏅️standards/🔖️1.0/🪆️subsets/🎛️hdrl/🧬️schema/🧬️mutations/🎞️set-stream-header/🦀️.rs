//! 🎞️ `set-stream-header` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetStreamHeader {
    pub stream_index: usize,
    pub strh: AviStreamHeader,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for SetStreamHeader {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "stream-header", kind: "set-stream-header", record: "SetStreamHeader" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as Mutation<AviSnapshot>>::Diff> {
        let Self { stream_index, strh } = self;
        protocol::MutationOutcome::new(stream_diff_for(*stream_index, AviStreamDiff { strh: Some(strh.clone()), ..AviStreamDiff::default() }))
    }
    fn inverse(&self, base: &AviSnapshot) -> Result<Vec<AviMutation>, semio_framework_value::ValueError> {
        let Self { stream_index, .. } = self;
        Ok({
            match base.streams.get(*stream_index) {
                Some(stream) => vec![AviMutation::SetStreamHeader(set_stream_header::SetStreamHeader { stream_index: *stream_index, strh: stream.strh.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set stream header", "Datenstrom-Header setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
