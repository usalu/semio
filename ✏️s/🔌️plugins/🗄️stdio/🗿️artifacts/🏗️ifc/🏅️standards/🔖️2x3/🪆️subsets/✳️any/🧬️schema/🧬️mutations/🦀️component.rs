//! 🧬️ Ifc2x3Mutation — document mutation dispatch. Richer than `4`'s `{NoMutation, SetSnapshot}`
//! stub: real per-instance vocabulary (`UpsertInstance`/`RemoveInstance`/`SetHeader`) matching
//! `Ifc2x3Diff`'s own id-keyed shape.

use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::{
    diff_remove_instance, diff_set_header, diff_set_snapshot, diff_upsert_instance, Ifc2x3Diff,
};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::step::engine::part21::{Part21Header, Part21Instance};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.ifc.2x3`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Ifc2x3Mutation {
    #[default]
    NoMutation,
    SetSnapshot { snapshot: Ifc2x3Snapshot },
    UpsertInstance { instance: Part21Instance },
    RemoveInstance { id: u64 },
    SetHeader { header: Part21Header },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning the diff (computed against the PRE-mutation
/// state, per `Mutation::diff`'s contract).
pub fn apply_ifc2x3_mutation(snapshot: &mut Ifc2x3Snapshot, mutation: &Ifc2x3Mutation) -> Ifc2x3Diff {
    let __diff = <Ifc2x3Mutation as Mutation<Ifc2x3Snapshot>>::diff(mutation, snapshot);
    match mutation {
        Ifc2x3Mutation::NoMutation => {}
        Ifc2x3Mutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
        Ifc2x3Mutation::UpsertInstance { instance } => {
            snapshot.document.instances.retain(|i| i.id != instance.id);
            snapshot.document.instances.push(instance.clone());
        }
        Ifc2x3Mutation::RemoveInstance { id } => {
            snapshot.document.instances.retain(|i| i.id != *id);
        }
        Ifc2x3Mutation::SetHeader { header } => {
            snapshot.document.header = header.clone();
        }
    }
    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<Ifc2x3Snapshot> for Ifc2x3Mutation {
    type Diff = Ifc2x3Diff;

    fn diff(&self, base: &Ifc2x3Snapshot) -> Self::Diff {
        match self {
            Ifc2x3Mutation::NoMutation => Ifc2x3Diff::default(),
            Ifc2x3Mutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            Ifc2x3Mutation::UpsertInstance { instance } => diff_upsert_instance(instance),
            Ifc2x3Mutation::RemoveInstance { id } => diff_remove_instance(*id),
            Ifc2x3Mutation::SetHeader { header } => diff_set_header(header),
        }
    }

    fn inverse(&self, base: &Ifc2x3Snapshot) -> Vec<Self> {
        match self {
            Ifc2x3Mutation::NoMutation => vec![Ifc2x3Mutation::NoMutation],
            Ifc2x3Mutation::SetSnapshot { .. } => vec![Ifc2x3Mutation::SetSnapshot { snapshot: base.clone() }],
            Ifc2x3Mutation::UpsertInstance { instance } => match base.document.instance(instance.id) {
                Some(old) => vec![Ifc2x3Mutation::UpsertInstance { instance: old.clone() }],
                None => vec![Ifc2x3Mutation::RemoveInstance { id: instance.id }],
            },
            Ifc2x3Mutation::RemoveInstance { id } => match base.document.instance(*id) {
                Some(old) => vec![Ifc2x3Mutation::UpsertInstance { instance: old.clone() }],
                None => vec![Ifc2x3Mutation::NoMutation],
            },
            Ifc2x3Mutation::SetHeader { .. } => vec![Ifc2x3Mutation::SetHeader { header: base.document.header.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for Ifc2x3Mutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for Ifc2x3Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Malformed { what: "op encode", offset: 0, detail: e.to_string() })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op decode", offset: 0, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::step::engine::part21::Part21Value;

    fn inst(id: u64, name: &str) -> Part21Instance {
        Part21Instance { id, entities: vec![(name.to_string(), vec![Part21Value::Int(id as i64)])] }
    }

    #[test]
    fn upsert_then_inverse_restores_absent_id_via_remove() {
        let mut snap = Ifc2x3Snapshot::default();
        let mutation = Ifc2x3Mutation::UpsertInstance { instance: inst(1, "IFCWALL") };
        let base = snap.clone();
        apply_ifc2x3_mutation(&mut snap, &mutation);
        assert_eq!(snap.document.instances.len(), 1);
        let inv = <Ifc2x3Mutation as Mutation<Ifc2x3Snapshot>>::inverse(&mutation, &base);
        assert_eq!(inv, vec![Ifc2x3Mutation::RemoveInstance { id: 1 }]);
    }

    #[test]
    fn remove_then_inverse_restores_prior_instance() {
        let mut snap = Ifc2x3Snapshot::default();
        snap.document.instances.push(inst(2, "IFCDOOR"));
        let base = snap.clone();
        let mutation = Ifc2x3Mutation::RemoveInstance { id: 2 };
        apply_ifc2x3_mutation(&mut snap, &mutation);
        assert!(snap.document.instances.is_empty());
        let inv = <Ifc2x3Mutation as Mutation<Ifc2x3Snapshot>>::inverse(&mutation, &base);
        assert_eq!(inv, vec![Ifc2x3Mutation::UpsertInstance { instance: inst(2, "IFCDOOR") }]);
    }

    #[test]
    fn op_text_round_trips() {
        let mutation = Ifc2x3Mutation::SetHeader { header: Part21Header::default() };
        let printed = protocol::OpText::print_op(&mutation);
        let parsed = <Ifc2x3Mutation as protocol::OpText>::parse_op(&printed).expect("parse");
        assert_eq!(parsed, mutation);
    }
}
//#endregion 🧪️Tests
