//! 🥦️ Transient diff: the window transient diff record, its diff algebra and the preflight and transfer hooks. See [`super`].

use super::*;

/// 🕳️ Tri-state decode of every `Option<Option<T>>` diff slot: a missing key is the unchanged slot (`None`) and a PRESENT
/// `null` is the clear `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into.
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}

/// 🔺️ Sparse typed delta of one Puzzle 5D window instance's ephemeral interaction state: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dWindowTransientDiff {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub suggestion_menu: Option<Option<Puzzle5dSuggestionMenu>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub engagement_input: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub brush_candidate_index: Option<usize>,
}

impl Puzzle5dWindowTransientDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle5dWindowTransient) -> Self {
        Self { suggestion_menu: Some(state.suggestion_menu.clone()), engagement_input: Some(state.engagement_input.clone()), brush_candidate_index: Some(state.brush_candidate_index) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle5dWindowTransient) -> Self {
        Self { suggestion_menu: self.suggestion_menu.as_ref().filter(|value| **value != base.suggestion_menu).cloned(), engagement_input: self.engagement_input.as_ref().filter(|value| **value != base.engagement_input).cloned(), brush_candidate_index: self.brush_candidate_index.as_ref().filter(|value| **value != base.brush_candidate_index).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle5dWindowTransient) -> Self {
        Self { suggestion_menu: self.suggestion_menu.as_ref().map(|_| base.suggestion_menu.clone()), engagement_input: self.engagement_input.as_ref().map(|_| base.engagement_input.clone()), brush_candidate_index: self.brush_candidate_index.as_ref().map(|_| base.brush_candidate_index) }
    }
}

impl protocol::MutationDiff<Puzzle5dWindowTransient> for Puzzle5dWindowTransientDiff {


}

impl protocol::ColdMutationDiff<Puzzle5dWindowTransient> for Puzzle5dWindowTransientDiff {
    fn absorb(&mut self, other: Self) {
        if other.suggestion_menu.is_some() {
            self.suggestion_menu = other.suggestion_menu;
        }
        if other.engagement_input.is_some() {
            self.engagement_input = other.engagement_input;
        }
        if other.brush_candidate_index.is_some() {
            self.brush_candidate_index = other.brush_candidate_index;
        }
    }

    fn apply(&self, next: &mut Puzzle5dWindowTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = &self.suggestion_menu {
            next.suggestion_menu = value.clone();
        }
        if let Some(value) = &self.engagement_input {
            next.engagement_input = value.clone();
        }
        if let Some(value) = &self.brush_candidate_index {
            next.brush_candidate_index = *value;
        }
        Ok(())
    }
}

impl protocol::DiffAlgebra<Puzzle5dWindowTransient> for Puzzle5dWindowTransientDiff {

    fn is_empty(&self) -> bool {
        self.suggestion_menu.is_none() && self.engagement_input.is_none() && self.brush_candidate_index.is_none()
    }
}

impl protocol::ColdDiffAlgebra<Puzzle5dWindowTransient> for Puzzle5dWindowTransientDiff {
    fn inverse(&self, base: &Puzzle5dWindowTransient) -> Self {
        self.restoring(base)
    }
}

semio_framework_value::artifact_retire_struct!(Puzzle5dWindowTransient { suggestion_menu, engagement_input, brush_candidate_index });

pub fn puzzle5d_window_transient_preflight(mutation: &Puzzle5dWindowTransientMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let Puzzle5dWindowTransientMutation::Snapshot { transient } = mutation;
    let menu = transient.suggestion_menu.as_ref().map_or(Some(0), |menu| menu.window_id.capacity().checked_add(menu.vortex_full_id.capacity()));
    let retained_bytes = menu
        .and_then(|menu| size_of::<Puzzle5dWindowTransient>().checked_add(transient.engagement_input.capacity())?.checked_add(menu))
        .ok_or_else(|| "Puzzle 5D window transient footprint overflowed".to_string())?;
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_ephemeral_item(retained_bytes))
}

pub(super) fn puzzle5d_window_transient_transfer(mutation: Puzzle5dWindowTransientMutation) -> Puzzle5dWindowTransient {
    match mutation {
        Puzzle5dWindowTransientMutation::Snapshot { transient } => transient,
    }
}
