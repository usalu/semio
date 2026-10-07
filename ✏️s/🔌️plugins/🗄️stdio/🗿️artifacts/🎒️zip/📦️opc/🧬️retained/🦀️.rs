//! 🧬️ Paged retained ownership for OPC snapshots and Store preparation.

use super::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode, OpcRelationshipOwners};
use semio_framework_value::{NativeEncodeControl, ValueError, ValueRefusalKind, list::PagedList, paged::{PagedBytes, PagedMap, PagedUtf8}};
#[path = "🚦️ownership/🦀️.rs"]
mod ownership;

pub const OPC_RETAINED_MAX_TEXT_BYTES: usize = u32::MAX as usize;
pub const OPC_RETAINED_MAX_PART_BYTES: usize = u32::MAX as usize;
pub const OPC_RETAINED_MAX_PARTS: usize = u32::MAX as usize;
pub const OPC_RETAINED_MAX_METADATA_ENTRIES: usize = u32::MAX as usize;
pub const OPC_RETAINED_MAX_RELATIONSHIPS: usize = u32::MAX as usize;
pub const OPC_RETAINED_MAX_RELATIONSHIP_OWNERS: usize = u32::MAX as usize;

pub type RetainedOpcText = PagedUtf8<OPC_RETAINED_MAX_TEXT_BYTES>;
pub type RetainedOpcBytes = PagedBytes<OPC_RETAINED_MAX_PART_BYTES>;
pub type RetainedOpcParts = PagedList<RetainedOpcPart, OPC_RETAINED_MAX_PARTS>;
pub type RetainedOpcMetadataEntries = PagedList<RetainedOpcMetadataEntry, OPC_RETAINED_MAX_METADATA_ENTRIES>;
pub type RetainedOpcRelationships = PagedList<RetainedOpcRelationship, OPC_RETAINED_MAX_RELATIONSHIPS>;
pub type RetainedOpcRelationshipOwners = PagedMap<RetainedOpcRelationships, OPC_RETAINED_MAX_RELATIONSHIP_OWNERS>;

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct RetainedOpcPart {
    pub path: RetainedOpcText,
    pub content_type: RetainedOpcText,
    pub bytes: RetainedOpcBytes,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct RetainedOpcMetadataEntry {
    pub name: RetainedOpcText,
    pub content_type: RetainedOpcText,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum RetainedOpcTargetMode {
    Internal,
    External,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct RetainedOpcRelationship {
    pub id: RetainedOpcText,
    pub rel_type: RetainedOpcText,
    pub target: RetainedOpcText,
    pub target_mode: RetainedOpcTargetMode,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct RetainedOpcContentTypes {
    pub defaults: RetainedOpcMetadataEntries,
    pub overrides: RetainedOpcMetadataEntries,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, value_derive::RetainedClone, value_derive::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct RetainedOpcPackage {
    pub parts: RetainedOpcParts,
    pub content_types: RetainedOpcContentTypes,
    pub relationships: RetainedOpcRelationshipOwners,
    pub comment: RetainedOpcText,
}

fn text(value: String) -> Result<RetainedOpcText, ValueError> {
    RetainedOpcText::try_from_str(&value)
}

impl RetainedOpcPart {
    fn try_from_package_part(part: OpcPart) -> Result<Self, ValueError> {
        Ok(Self { path: text(part.path)?, content_type: text(part.content_type)?, bytes: RetainedOpcBytes::try_from_slice(&part.bytes)? })
    }

    pub fn materialize(&self, control: &mut NativeEncodeControl<'_>) -> Result<OpcPart, ValueError> {
        Ok(OpcPart { path: self.path.to_string_owner_controlled(control)?, content_type: self.content_type.to_string_owner_controlled(control)?, bytes: self.bytes.to_vec_owner_controlled(control)? })
    }
}

impl RetainedOpcMetadataEntry {
    fn try_from_pair((name, content_type): (String, String)) -> Result<Self, ValueError> {
        Ok(Self { name: text(name)?, content_type: text(content_type)? })
    }

    fn materialize(&self, control: &mut NativeEncodeControl<'_>) -> Result<(String, String), ValueError> {
        Ok((self.name.to_string_owner_controlled(control)?, self.content_type.to_string_owner_controlled(control)?))
    }
}

impl From<OpcTargetMode> for RetainedOpcTargetMode {
    fn from(value: OpcTargetMode) -> Self {
        match value {
            OpcTargetMode::Internal => Self::Internal,
            OpcTargetMode::External => Self::External,
        }
    }
}

impl From<RetainedOpcTargetMode> for OpcTargetMode {
    fn from(value: RetainedOpcTargetMode) -> Self {
        match value {
            RetainedOpcTargetMode::Internal => Self::Internal,
            RetainedOpcTargetMode::External => Self::External,
        }
    }
}

impl RetainedOpcRelationship {
    fn try_from_relationship(value: OpcRelationship) -> Result<Self, ValueError> {
        Ok(Self { id: text(value.id)?, rel_type: text(value.rel_type)?, target: text(value.target)?, target_mode: value.target_mode.into() })
    }

    fn materialize(&self, control: &mut NativeEncodeControl<'_>) -> Result<OpcRelationship, ValueError> {
        Ok(OpcRelationship { id: self.id.to_string_owner_controlled(control)?, rel_type: self.rel_type.to_string_owner_controlled(control)?, target: self.target.to_string_owner_controlled(control)?, target_mode: self.target_mode.into() })
    }
}

impl RetainedOpcContentTypes {
    fn try_from_content_types(value: OpcContentTypes) -> Result<Self, ValueError> {
        Ok(Self {
            defaults: RetainedOpcMetadataEntries::try_from_fallible_iter(value.defaults.into_iter().map(RetainedOpcMetadataEntry::try_from_pair))?,
            overrides: RetainedOpcMetadataEntries::try_from_fallible_iter(value.overrides.into_iter().map(RetainedOpcMetadataEntry::try_from_pair))?,
        })
    }

    pub fn materialize(&self, control: &mut NativeEncodeControl<'_>) -> Result<OpcContentTypes, ValueError> {
        let mut defaults = control.allocate_vec(self.defaults.len())?;
        for entry in self.defaults.iter() {
            defaults.push(entry.materialize(control)?);
            control.step()?;
        }
        let mut overrides = control.allocate_vec(self.overrides.len())?;
        for entry in self.overrides.iter() {
            overrides.push(entry.materialize(control)?);
            control.step()?;
        }
        Ok(OpcContentTypes { defaults, overrides })
    }

    pub fn resolve(&self, part_path: &str) -> Option<&RetainedOpcText> {
        let absolute = format!("/{}", part_path.trim_start_matches('/'));
        if let Some(entry) = self.overrides.iter().find(|entry| entry.name.eq_str(&absolute)) {
            return Some(&entry.content_type);
        }
        let extension = part_path.rsplit('.').next()?;
        self.defaults.iter().find(|entry| entry.name.to_string_owner().eq_ignore_ascii_case(extension)).map(|entry| &entry.content_type)
    }


    pub fn set_override(&mut self, part_path: &str, content_type: &str) -> Result<(), ValueError> {
        let absolute = format!("/{}", part_path.trim_start_matches('/'));
        if let Some(entry) = self.overrides.iter_mut().find(|entry| entry.name.eq_str(&absolute)) {
            entry.content_type = RetainedOpcText::try_from_str(content_type)?;
            return Ok(());
        }
        self.overrides
            .try_push(RetainedOpcMetadataEntry { name: RetainedOpcText::try_from_str(&absolute)?, content_type: RetainedOpcText::try_from_str(content_type)? })
            .map_err(ValueError::from)
    }
}

impl RetainedOpcPackage {
    pub fn try_from_package(package: OpcPackage) -> Result<Self, ValueError> {
        Ok(Self {
            parts: RetainedOpcParts::try_from_fallible_iter(package.parts.into_iter().map(RetainedOpcPart::try_from_package_part))?,
            content_types: RetainedOpcContentTypes::try_from_content_types(package.content_types)?,
            relationships: RetainedOpcRelationshipOwners::try_from_fallible_entries(package.relationships.into_groups().map(|(owner, relationships)| {
                let relationships = RetainedOpcRelationships::try_from_fallible_iter(relationships.into_iter().map(RetainedOpcRelationship::try_from_relationship))?;
                Ok::<(PagedUtf8<{usize::MAX}>, RetainedOpcRelationships), ValueError>((PagedUtf8::try_from_str(&owner)?, relationships))
            }))?,
            comment: text(package.comment)?,
        })
    }

    pub fn materialize_package(&self, control: &mut NativeEncodeControl<'_>) -> Result<OpcPackage, ValueError> {
        control.scoped_stage(|control| {
            let relationship_count = self.relationships.values().try_fold(0usize, |total, relationships| total.checked_add(relationships.len())).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "retained OPC relationship workload overflow"))?;
            let total = self.parts.len().checked_add(self.content_types.defaults.len()).and_then(|value| value.checked_add(self.content_types.overrides.len())).and_then(|value| value.checked_add(self.relationships.len())).and_then(|value| value.checked_add(relationship_count)).and_then(|value| value.checked_add(1)).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "retained OPC materialization workload overflow"))?;
            control.begin_stage(total)?;
            let mut parts = control.allocate_vec(self.parts.len())?;
            for part in self.parts.iter() {
                parts.push(part.materialize(control)?);
                control.step()?;
            }
            let content_types = self.content_types.materialize(control)?;
            let mut groups = control.allocate_vec(self.relationships.len())?;
            for (owner, retained_relationships) in self.relationships.iter() {
                let owner = owner.to_string_owner_controlled(control)?;
                let mut values = control.allocate_vec(retained_relationships.len())?;
                for relationship in retained_relationships.iter() {
                    values.push(relationship.materialize(control)?);
                    control.step()?;
                }
                groups.push((owner, values));
                control.step()?;
            }
            let relationships = OpcRelationshipOwners::adopt_encoded_admitted(groups, control)?;
            let comment = self.comment.to_string_owner_controlled(control)?;
            control.step()?;
            Ok(OpcPackage { parts, content_types, relationships, comment })
        })
    }

    pub fn materialize_package_exact(&self) -> Result<OpcPackage, ValueError> {
        let maximum = self.materialization_owned_bytes()?;
        let mut callback = |_| true;
        self.materialize_package(&mut NativeEncodeControl::new(maximum, &mut callback))
    }

    pub fn edit_package(&mut self, edit: impl FnOnce(&mut OpcPackage)) -> Result<(), ValueError> {
        let mut package = self.materialize_package_exact()?;
        edit(&mut package);
        *self = Self::try_from_package(package)?;
        Ok(())
    }

    pub fn materialization_owned_bytes(&self) -> Result<usize, ValueError> {
        fn add(total: &mut usize, bytes: usize) -> Result<(), ValueError> {
            *total = total.checked_add(bytes).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained OPC materialization ownership overflow"))?;
            Ok(())
        }
        fn array<T>(total: &mut usize, count: usize) -> Result<(), ValueError> {
            add(total, count.checked_mul(std::mem::size_of::<T>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained OPC materialization array overflow"))?)
        }
        let mut total = 0usize;
        array::<OpcPart>(&mut total, self.parts.len())?;
        for part in self.parts.iter() {
            add(&mut total, part.path.len())?;
            add(&mut total, part.content_type.len())?;
            add(&mut total, part.bytes.len())?;
        }
        for entries in [&self.content_types.defaults, &self.content_types.overrides] {
            array::<(String, String)>(&mut total, entries.len())?;
            for entry in entries.iter() {
                add(&mut total, entry.name.len())?;
                add(&mut total, entry.content_type.len())?;
            }
        }
        array::<(String,Vec<OpcRelationship>)>(&mut total, self.relationships.len())?;
        for (owner, relationships) in self.relationships.iter() {
            add(&mut total, owner.len())?;
            array::<OpcRelationship>(&mut total, relationships.len())?;
            for relationship in relationships.iter() {
                add(&mut total, relationship.id.len())?;
                add(&mut total, relationship.rel_type.len())?;
                add(&mut total, relationship.target.len())?;
            }
        }
        add(&mut total, self.comment.len())?;
        Ok(total)
    }

    pub fn part(&self, path: &str) -> Option<&RetainedOpcPart> {
        let path = path.trim_start_matches('/');
        self.parts.iter().find(|part| part.path.eq_str(path))
    }


    pub fn set_part(&mut self, path: &str, content_type: &str, bytes: Vec<u8>) -> Result<(), ValueError> {
        let path = path.trim_start_matches('/');
        if let Some(part) = self.parts.iter_mut().find(|part| part.path.eq_str(path)) {
            part.content_type = RetainedOpcText::try_from_str(content_type)?;
            part.bytes = RetainedOpcBytes::try_from_slice(&bytes)?;
        } else {
            self.parts
                .try_push(RetainedOpcPart { path: RetainedOpcText::try_from_str(path)?, content_type: RetainedOpcText::try_from_str(content_type)?, bytes: RetainedOpcBytes::try_from_slice(&bytes)? })
                .map_err(ValueError::from)?;
        }
        self.content_types.set_override(path, content_type)
    }

    pub fn part_bytes(&self, path: &str) -> Option<&RetainedOpcBytes> {
        self.part(path).map(|part| &part.bytes)
    }

    pub fn relationships_for(&self, owner: &str) -> Option<&RetainedOpcRelationships> {
        self.relationships.get(owner)
    }

    pub fn resolve_relationship(&self, owner: &str, rel_type: &str) -> Option<String> {
        let relationship = self.relationships_for(owner)?.iter().find(|relationship| relationship.rel_type.eq_str(rel_type))?;
        Some(super::resolve_relationship_target(owner, &relationship.target.to_string_owner()))
    }
}

impl TryFrom<OpcPackage> for RetainedOpcPackage {
    type Error = ValueError;

    fn try_from(value: OpcPackage) -> Result<Self, Self::Error> {
        Self::try_from_package(value)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
