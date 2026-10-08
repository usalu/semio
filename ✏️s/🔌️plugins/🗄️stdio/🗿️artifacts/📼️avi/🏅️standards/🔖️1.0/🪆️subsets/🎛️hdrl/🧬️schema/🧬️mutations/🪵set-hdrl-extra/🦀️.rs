//! 🪵 `set-hdrl-extra` — replaces the retained `hdrl` auxiliary chunks (`JUNK`, `vprp`, ...) as one unit. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetHdrlExtra {
    pub chunks: Vec<RiffChunk>,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for SetHdrlExtra {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "hdrl-extra", kind: "set-hdrl-extra", record: "SetHdrlExtra" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as Mutation<AviSnapshot>>::Diff> {
        protocol::MutationOutcome::new(AviDiff { hdrl_extra: (base.hdrl_extra != self.chunks).then(|| self.chunks.clone()), ..AviDiff::default() })
    }
    fn inverse(&self, base: &AviSnapshot) -> Result<Vec<AviMutation>, semio_framework_value::ValueError> {
        Ok((base.hdrl_extra != self.chunks).then(|| AviMutation::SetHdrlExtra(set_hdrl_extra::SetHdrlExtra { chunks: base.hdrl_extra.clone() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set hdrl extra chunks", "Zusätzliche hdrl-Chunks setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
