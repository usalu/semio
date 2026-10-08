//! 🔺️ Interaction state changes as absolute per-domain rows.
use protocol::InteractionState;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

/// 🗝️ The absolute new value of one domain's slot in one of the interaction maps; `value: None` clears the slot.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DomainEdit<T> {
    pub domain: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,
}

impl<T: Clone + PartialEq> DomainEdit<T> {
    /// 🧭️ The rows that carry `held` to `next`: one per domain whose slot differs, in ascending domain order.
    pub fn changed(held: &BTreeMap<String, T>, next: &BTreeMap<String, T>) -> Vec<Self> {
        let mut domains: Vec<&String> = held.keys().chain(next.keys()).collect();
        domains.sort();
        domains.dedup();
        domains.into_iter().filter(|domain| held.get(*domain) != next.get(*domain)).map(|domain| Self { domain: domain.clone(), value: next.get(domain).cloned() }).collect()
    }

    fn restoring(rows: &[Self], held: &BTreeMap<String, T>) -> Vec<Self> {
        rows.iter().map(|row| Self { domain: row.domain.clone(), value: held.get(&row.domain).cloned() }).collect()
    }

    fn write(map: &mut BTreeMap<String, T>, rows: &[Self]) {
        for row in rows {
            match &row.value {
                Some(value) => {
                    map.insert(row.domain.clone(), value.clone());
                }
                None => {
                    map.remove(&row.domain);
                }
            }
        }
    }

    fn absorb(held: &mut Vec<Self>, rows: Vec<Self>) {
        for row in rows {
            match held.iter_mut().find(|candidate| candidate.domain == row.domain) {
                Some(candidate) => candidate.value = row.value,
                None => held.push(row),
            }
        }
    }
}

/// 🔺️ Sparse diff of [`InteractionState`]: one absolute row per touched domain of each map.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct InteractionStateDiff {
    #[value(skip_serializing_if = "Vec::is_empty")]
    pub selection: Vec<DomainEdit<protocol::DomainSelection>>,
    #[value(skip_serializing_if = "Vec::is_empty")]
    pub hover: Vec<DomainEdit<protocol::DomainHover>>,
    #[value(skip_serializing_if = "Vec::is_empty")]
    pub active_mode: Vec<DomainEdit<protocol::SelectionMode>>,
    #[value(skip_serializing_if = "Vec::is_empty")]
    pub active_granularity: Vec<DomainEdit<String>>,
}

impl protocol::MutationDiff<InteractionState> for InteractionStateDiff {
    fn apply(&self, base: &InteractionState, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<InteractionState> {
        let mut next = base.clone();
        DomainEdit::write(&mut next.selection, &self.selection);
        DomainEdit::write(&mut next.hover, &self.hover);
        DomainEdit::write(&mut next.active_mode, &self.active_mode);
        DomainEdit::write(&mut next.active_granularity, &self.active_granularity);
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        DomainEdit::absorb(&mut self.selection, other.selection);
        DomainEdit::absorb(&mut self.hover, other.hover);
        DomainEdit::absorb(&mut self.active_mode, other.active_mode);
        DomainEdit::absorb(&mut self.active_granularity, other.active_granularity);
    }
}

impl protocol::DiffAlgebra<InteractionState> for InteractionStateDiff {
    fn inverse(&self, base: &InteractionState) -> Self {
        Self {
            selection: DomainEdit::restoring(&self.selection, &base.selection),
            hover: DomainEdit::restoring(&self.hover, &base.hover),
            active_mode: DomainEdit::restoring(&self.active_mode, &base.active_mode),
            active_granularity: DomainEdit::restoring(&self.active_granularity, &base.active_granularity),
        }
    }

    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
