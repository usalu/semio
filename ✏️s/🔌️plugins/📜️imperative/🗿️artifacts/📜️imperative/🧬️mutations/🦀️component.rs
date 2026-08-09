//! 🧬️ imperative artifact — document mutation dispatch.


use crate::artifacts::imperative::diff::schema::{ImperativeDiff, ImperativePathDelta};
use crate::artifacts::imperative::diff::steps_delta_from_collection_mutation;
use crate::artifacts::imperative::{Dictionary, ImperativeSnapshot, PathRef, Step};

//#region 🔖️Mutations
/// @emoji ✂️ A step-collection edit at a `PathRef` — root path or a nested `control.*` step's slot.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImperativeMutation {
    pub path_ref: PathRef,
    pub collection: protocol::CollectionMutation<String, Step, Dictionary>,
}

impl protocol::Mutation<ImperativeSnapshot> for ImperativeMutation {
    type Diff = crate::artifacts::imperative::diff::ImperativeDiff;

    fn diff(&self, projection: &ImperativeSnapshot) -> Self::Diff {
        let steps = resolve_steps(projection, &self.path_ref).unwrap_or(&[]);
        ImperativeDiff {
            path: Some(ImperativePathDelta {
                path_ref: self.path_ref.clone(),
                steps: steps_delta_from_collection_mutation(steps, &self.collection),
            }),
            ..Default::default()
        }
    }

    fn inverse(&self, projection: &ImperativeSnapshot) -> Vec<Self> {
        match resolve_steps(projection, &self.path_ref) {
            Some(steps) => vec![ImperativeMutation { path_ref: self.path_ref.clone(), collection: protocol::inverse_collection_mutation(steps, &self.collection) }],
            None => Vec::new(),
        }
    }
}

/// 🔎️ Resolves the step list a `PathRef` addresses; a not-yet-materialized nested slot reads as empty.
pub fn resolve_steps<'a>(snapshot: &'a ImperativeSnapshot, path_ref: &PathRef) -> Option<&'a [Step]> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return Some(&snapshot.path.steps);
    }
    let owner = path_ref.owner.as_ref()?;
    let slot = path_ref.slot.as_ref()?;
    let owner_step = snapshot.path.steps.iter().find(|step| &step.id == owner)?;
    Some(owner_step.bodies.get(slot).map_or(&[] as &[Step], |path| path.steps.as_slice()))
}
//#endregion 🔖️Mutations
