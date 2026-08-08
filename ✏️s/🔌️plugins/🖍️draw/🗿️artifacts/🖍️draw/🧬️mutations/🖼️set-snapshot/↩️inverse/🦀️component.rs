//! ↩️ Inverse of `SetSnapshot`.
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;

pub fn inverse(base: &DrawSnapshot) -> Vec<DrawMutation> {
    vec![DrawMutation::SetSnapshot { snapshot: base.clone() }]
}
