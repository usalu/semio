//! 🔺️ The sparse diff of an [`OpcPackage`] — shared by every OOXML artifact (`📜️docx`, `📽️pptx`, `📕️xlsx`) so that none of them re-derives it. Every keyed list of the
//! package is a positional list delta of the kernel (`protocol::list_delta`, wire shape from `stdio_list_delta!`): the `[Content_Types].xml` defaults and overrides, the parts,
//! and the relationship owners (each owner patch nesting the positional delta of its relationship list). A delta never carries an `order` key list; the comment is a
//! plain setter.
//!
//! The relationship owners are a sorted set, so an owner row's index is its sorted rank; a delta that would break the order is refused.

use super::{OpcPackage, OpcPart, OpcRelationship, OpcRelationshipOwners, OpcTargetMode};
use semio_s_artifact_stdio_contract::kernel::list_delta::RowPatch;
use semio_s_artifact_stdio_contract::kernel::{ApplyCapability, MutationApplyError};
use semio_s_artifact_stdio_contract::list_delta::{compose_optional, Composable};

//#region 🔖️Rows
/// 🏷️ One `[Content_Types].xml` entry as a list row: the extension (default) or part name (override) and its content type.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct OpcContentTypeRow {
    pub name: String,
    pub content_type: String,
}

/// 🗂️ One relationship owner as a list row: the owner part name (empty for the package root) and the relationships it owns.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct OpcOwnerRow {
    pub owner: String,
    pub relationships: Vec<OpcRelationship>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn content_type_rows(entries: &[(String, String)]) -> Vec<OpcContentTypeRow> {
    entries.iter().map(|(name, content_type)| OpcContentTypeRow { name: name.clone(), content_type: content_type.clone() }).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn content_type_entries(rows: Vec<OpcContentTypeRow>) -> Vec<(String, String)> {
    rows.into_iter().map(|row| (row.name, row.content_type)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn owner_rows(owners: &OpcRelationshipOwners) -> Vec<OpcOwnerRow> {
    owners.groups().map(|(owner, relationships)| OpcOwnerRow { owner: owner.clone(), relationships: relationships.clone() }).collect()
}
//#endregion 🔖️Rows

//#region 🔖️Patches
/// 🩹 The sparse patch of one `[Content_Types].xml` entry: the content type it is given.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct OpcContentTypePatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

/// 🩹 The sparse patch of one part: its content type and its bytes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct OpcPartPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

/// 🩹 The sparse patch of one relationship: its type, target and target mode.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct OpcRelationshipPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rel_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target_mode: Option<OpcTargetMode>,
}

/// 🩹 The sparse patch of one relationship owner: the positional delta of its relationship list.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct OpcOwnerPatch {
    #[value(default)]
    pub relationships: OpcRelationshipsDelta,
}

impl RowPatch<OpcContentTypeRow> for OpcContentTypePatch {
    fn commit_into(&self, row: &mut OpcContentTypeRow, _capability: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(content_type) = &self.content_type {
            row.content_type.clone_from(content_type);
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.content_type.is_some() {
            self.content_type = later.content_type;
        }
    }

    fn inverse(&self, row: &OpcContentTypeRow) -> Self {
        Self { content_type: self.content_type.as_ref().map(|_| row.content_type.clone()) }
    }

    fn is_empty(&self) -> bool {
        self.content_type.is_none()
    }
}

impl RowPatch<OpcPart> for OpcPartPatch {
    fn commit_into(&self, row: &mut OpcPart, _capability: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(content_type) = &self.content_type {
            row.content_type.clone_from(content_type);
        }
        if let Some(bytes) = &self.bytes {
            row.bytes.clone_from(bytes);
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.content_type.is_some() {
            self.content_type = later.content_type;
        }
        if later.bytes.is_some() {
            self.bytes = later.bytes;
        }
    }

    fn inverse(&self, row: &OpcPart) -> Self {
        Self { content_type: self.content_type.as_ref().map(|_| row.content_type.clone()), bytes: self.bytes.as_ref().map(|_| row.bytes.clone()) }
    }

    fn is_empty(&self) -> bool {
        self.content_type.is_none() && self.bytes.is_none()
    }
}

impl RowPatch<OpcRelationship> for OpcRelationshipPatch {
    fn commit_into(&self, row: &mut OpcRelationship, _capability: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(rel_type) = &self.rel_type {
            row.rel_type.clone_from(rel_type);
        }
        if let Some(target) = &self.target {
            row.target.clone_from(target);
        }
        if let Some(target_mode) = self.target_mode {
            row.target_mode = target_mode;
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.rel_type.is_some() {
            self.rel_type = later.rel_type;
        }
        if later.target.is_some() {
            self.target = later.target;
        }
        if later.target_mode.is_some() {
            self.target_mode = later.target_mode;
        }
    }

    fn inverse(&self, row: &OpcRelationship) -> Self {
        Self { rel_type: self.rel_type.as_ref().map(|_| row.rel_type.clone()), target: self.target.as_ref().map(|_| row.target.clone()), target_mode: self.target_mode.map(|_| row.target_mode) }
    }

    fn is_empty(&self) -> bool {
        self.rel_type.is_none() && self.target.is_none() && self.target_mode.is_none()
    }
}

impl RowPatch<OpcOwnerRow> for OpcOwnerPatch {
    fn commit_into(&self, row: &mut OpcOwnerRow, capability: ApplyCapability) -> Result<(), MutationApplyError> {
        row.relationships = self.relationships.commit_onto(&row.relationships, capability).map_err(|error| error.under(["relationships"]))?;
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        self.relationships.absorb(later.relationships);
    }

    fn inverse(&self, row: &OpcOwnerRow) -> Self {
        Self { relationships: self.relationships.inverse(&row.relationships) }
    }

    fn is_empty(&self) -> bool {
        self.relationships.is_empty()
    }
}
//#endregion 🔖️Patches

//#region 🔖️Deltas
semio_s_artifact_stdio_contract::stdio_list_delta! {
    /// 🪡️ The positional delta of a `[Content_Types].xml` entry list (defaults or overrides).
    pub OpcContentTypeEntriesDelta { removal: OpcContentTypeRemoval, insertion: OpcContentTypeInsertion, relocation: OpcContentTypeRelocation, modification: OpcContentTypeModification, row: OpcContentTypeRow, patch: OpcContentTypePatch, key: name }
}

semio_s_artifact_stdio_contract::stdio_list_delta! {
    /// 🪡️ The positional delta of the package's parts.
    pub OpcPartsDelta { removal: OpcPartRemoval, insertion: OpcPartInsertion, relocation: OpcPartRelocation, modification: OpcPartModification, row: OpcPart, patch: OpcPartPatch, key: path }
}

semio_s_artifact_stdio_contract::stdio_list_delta! {
    /// 🪡️ The positional delta of one owner's relationship list.
    pub OpcRelationshipsDelta { removal: OpcRelationshipRemoval, insertion: OpcRelationshipInsertion, relocation: OpcRelationshipRelocation, modification: OpcRelationshipModification, row: OpcRelationship, patch: OpcRelationshipPatch, key: id }
}

semio_s_artifact_stdio_contract::stdio_list_delta! {
    /// 🪡️ The positional delta of the relationship owners, in owner order.
    pub OpcOwnersDelta { removal: OpcOwnerRemoval, insertion: OpcOwnerInsertion, relocation: OpcOwnerRelocation, modification: OpcOwnerModification, row: OpcOwnerRow, patch: OpcOwnerPatch, key: owner }
}
//#endregion 🔖️Deltas

//#region 🔖️PackageDiff
/// 🔺️ The `[Content_Types].xml` part of an [`OpcDiff`]: the deltas of its two entry lists.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct OpcContentTypesDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<OpcContentTypeEntriesDelta>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<OpcContentTypeEntriesDelta>,
}

/// 🔺️ The sparse diff of an [`OpcPackage`]: its archive comment, content types, parts and relationship owners.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct OpcDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_types: Option<OpcContentTypesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<OpcPartsDelta>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub relationships: Option<OpcOwnersDelta>,
}

impl OpcDiff {
    /// ✍️ Writes the diff into `package`; reached only from an artifact diff's own `apply`, under the central applier's capability.
    pub fn commit_into(&self, package: &mut OpcPackage, capability: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(types) = &self.content_types {
            if let Some(delta) = &types.defaults {
                let rows = delta.commit_onto(&content_type_rows(&package.content_types.defaults), capability).map_err(|error| error.under(["contentTypes", "defaults"]))?;
                package.content_types.defaults = content_type_entries(rows);
            }
            if let Some(delta) = &types.overrides {
                let rows = delta.commit_onto(&content_type_rows(&package.content_types.overrides), capability).map_err(|error| error.under(["contentTypes", "overrides"]))?;
                package.content_types.overrides = content_type_entries(rows);
            }
        }
        if let Some(delta) = &self.parts {
            package.parts = delta.commit_onto(&package.parts, capability).map_err(|error| error.under(["parts"]))?;
        }
        if let Some(delta) = &self.relationships {
            let rows = delta.commit_onto(&owner_rows(&package.relationships), capability).map_err(|error| error.under(["relationships"]))?;
            if rows.windows(2).any(|pair| pair[0].owner >= pair[1].owner) {
                return Err(MutationApplyError::new("mutation.apply.invalid-order", "relationship owners are not in ascending owner order").at(["relationships"]));
            }
            package.relationships = OpcRelationshipOwners::new();
            for row in rows {
                package.relationships.replace_owner(row.owner, row.relationships);
            }
        }
        if let Some(comment) = &self.comment {
            package.comment.clone_from(comment);
        }
        Ok(())
    }

    /// 🔁️ The diff that, written after `self`, restores `base`: every delta rewinds from the base rows, the comment from the base comment.
    pub fn rewind(&self, base: &OpcPackage) -> Self {
        Self {
            comment: self.comment.as_ref().map(|_| base.comment.clone()),
            content_types: self.content_types.as_ref().map(|types| OpcContentTypesDiff {
                defaults: types.defaults.as_ref().map(|delta| delta.inverse(&content_type_rows(&base.content_types.defaults))),
                overrides: types.overrides.as_ref().map(|delta| delta.inverse(&content_type_rows(&base.content_types.overrides))),
            }),
            parts: self.parts.as_ref().map(|delta| delta.inverse(&base.parts)),
            relationships: self.relationships.as_ref().map(|delta| delta.inverse(&owner_rows(&base.relationships))),
        }
    }

    /// ➕️ Composes `self` (base→mid) with `later` (mid→after) into base→after, in place.
    pub fn absorb(&mut self, later: Self) {
        if later.comment.is_some() {
            self.comment = later.comment;
        }
        self.content_types = match (self.content_types.take(), later.content_types) {
            (None, value) | (value, None) => value,
            (Some(left), Some(right)) => Some(OpcContentTypesDiff { defaults: compose_optional(left.defaults, right.defaults), overrides: compose_optional(left.overrides, right.overrides) }),
        };
        self.parts = compose_optional(self.parts.take(), later.parts);
        self.relationships = compose_optional(self.relationships.take(), later.relationships);
    }
}
//#endregion 🔖️PackageDiff

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
