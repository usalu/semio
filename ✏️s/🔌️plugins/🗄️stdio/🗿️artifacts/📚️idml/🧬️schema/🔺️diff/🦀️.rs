//! 🔺️ Ordered sparse IDML member changes with atomic package validation.
use std::collections::{HashMap, HashSet};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_os_kernel as protocol;
use crate::{IdmlSnapshot, schema::IdmlPart};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct IdmlPartPatch { pub path: String, pub bytes: Option<Vec<u8>> }

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct IdmlPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dom_version: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub part_order: Option<Vec<String>>,
    pub parts: Vec<IdmlPartPatch>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct IdmlDiff { pub steps: Vec<IdmlPatch> }

impl IdmlPatch {
    fn changing(base: &IdmlSnapshot, next: &IdmlSnapshot) -> Self {
        let before: HashMap<_, _> = base.parts.iter().map(|part| (part.path.as_str(), part)).collect();
        let after: HashSet<_> = next.parts.iter().map(|part| part.path.as_str()).collect();
        let mut parts: Vec<_> = base.parts.iter().filter(|part| !after.contains(part.path.as_str())).map(|part| IdmlPartPatch { path: part.path.clone(), bytes: None }).collect();
        parts.extend(next.parts.iter().filter(|part| before.get(part.path.as_str()).is_none_or(|previous| previous.bytes != part.bytes)).map(|part| IdmlPartPatch { path: part.path.clone(), bytes: Some(part.bytes.clone()) }));
        let order = |document: &IdmlSnapshot| document.parts.iter().map(|part| part.path.clone()).collect::<Vec<_>>();
        Self { dom_version: (base.dom_version != next.dom_version).then(|| next.dom_version.clone()), part_order: (order(base) != order(next)).then(|| order(next)), parts }
    }
    fn is_empty(&self) -> bool { self.dom_version.is_none() && self.part_order.is_none() && self.parts.is_empty() }
    fn candidate(&self, base: &IdmlSnapshot) -> Result<IdmlSnapshot, semio_framework_value::ValueError> {
        let mut next = base.clone();
        let mut seen = HashSet::new();
        for patch in &self.parts {
            if !seen.insert(patch.path.as_str()) { return Err(crate::schema::snapshot::invalid("stdio.idml.duplicate-part-patch")); }
            let at = next.parts.iter().position(|part| part.path == patch.path);
            match (&patch.bytes, at) {
                (Some(bytes), Some(at)) => next.parts[at].bytes = bytes.clone(),
                (Some(bytes), None) => next.parts.push(IdmlPart { path: patch.path.clone(), bytes: bytes.clone() }),
                (None, Some(at)) => { next.parts.remove(at); }
                (None, None) => return Err(crate::schema::snapshot::invalid("stdio.idml.missing-part-patch")),
            }
        }
        if let Some(order) = &self.part_order {
            if order.len() != next.parts.len() || order.iter().collect::<HashSet<_>>().len() != order.len() { return Err(crate::schema::snapshot::invalid("stdio.idml.part-order")); }
            let mut parts: HashMap<_, _> = next.parts.into_iter().map(|part| (part.path.clone(), part)).collect();
            next.parts = order.iter().map(|path| parts.remove(path).ok_or_else(|| crate::schema::snapshot::invalid("stdio.idml.part-order"))).collect::<Result<_, _>>()?;
        }
        if let Some(version) = &self.dom_version { next.dom_version = version.clone(); }
        next.package()?;
        Ok(next)
    }
}

impl IdmlDiff {
    /// 🧱️ Projects an admitted semantic candidate onto ordered member and version fields.
    pub fn changing(base: &IdmlSnapshot, next: &IdmlSnapshot) -> Self {
        let patch = IdmlPatch::changing(base, next);
        Self { steps: if patch.is_empty() { Vec::new() } else { vec![patch] } }
    }
}
impl protocol::DiffAlgebra<IdmlSnapshot> for IdmlDiff {
    fn is_empty(&self) -> bool { self.steps.iter().all(IdmlPatch::is_empty) }
}
impl protocol::MutationDiff<IdmlSnapshot> for IdmlDiff {}
impl protocol::ColdMutationDiff<IdmlSnapshot> for IdmlDiff {
    fn absorb(&mut self, other: Self) { self.steps.extend(other.steps); }
    fn apply(&self, unpublished: &mut IdmlSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let mut next = unpublished.clone();
        for patch in &self.steps { next = patch.candidate(&next).map_err(|error| protocol::MutationApplyError::new("mutation.invariant", error.to_string()))?; }
        protocol::PreparedField::new(next, drop, capability).install(unpublished);
        Ok(())
    }
}
impl protocol::ColdDiffAlgebra<IdmlSnapshot> for IdmlDiff {
    fn inverse(&self, base: &IdmlSnapshot) -> Self {
        let mut next = base.clone();
        let mut inverses = Vec::new();
        for patch in &self.steps {
            let Ok(after) = patch.candidate(&next) else { return Self::default(); };
            inverses.push(IdmlPatch::changing(&after, &next));
            next = after;
        }
        inverses.reverse();
        Self { steps: inverses }
    }
}

#[path="🧳️retained/🦀️.rs"]pub mod retained;
