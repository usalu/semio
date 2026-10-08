//! 📸️ `set-viewpoint-snapshot` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetViewpointSnapshot {
    pub(crate) topic_guid: String,
    pub(crate) guid: String,
    pub(crate) snapshot: Option<Vec<u8>>,
}

impl SetViewpointSnapshot {
    /// 📸️ This setter as the aggregate mutation that carries it.
    fn into_mutation(self) -> BcfMutation {
        BcfMutation::SetViewpointSnapshot(self)
    }
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for SetViewpointSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "viewpoint-snapshot", kind: "set-viewpoint-snapshot", record: "SetViewpointSnapshot" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as Mutation<BcfSnapshot>>::Diff> {
        let Self { topic_guid, guid, snapshot } = self;
        protocol::MutationOutcome::new(wrap_viewpoint_diff(base, topic_guid, guid, BcfViewpointDiff { camera: None, components: None, snapshot: Some(snapshot.clone()) }))
    }
    fn inverse(&self, base: &BcfSnapshot) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
        let Self { topic_guid, guid, .. } = self;
        Ok({
            match find_viewpoint(base, topic_guid, guid) {
                Some(v) => vec![Self { topic_guid: topic_guid.clone(), guid: guid.clone(), snapshot: v.snapshot.clone() }.into_mutation()],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set viewpoint snapshot", "Blickpunkt-Schnappschuss setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
