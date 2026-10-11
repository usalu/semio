//! 🔁️ `set-loop-count` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-loop-count")]
#[value(rename_all = "camelCase")]
pub struct SetLoopCount {
    pub(crate) loop_count: Option<u16>,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetLoopCount {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "loop-count", kind: "set-loop-count", record: "SetLoopCount" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { loop_count } = self;
        protocol::MutationOutcome::new(GifDiff { loop_count: (*loop_count != base.loop_count).then_some(*loop_count), ..Default::default() })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        Ok(vec![GifMutation::SetLoopCount(set_loop_count::SetLoopCount { loop_count: base.loop_count })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set loop count", "Wiederholungsanzahl setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
