//! 🎯️ Canonical component labels retain one measurable ordered backing allocation.

use std::mem::size_of;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ComponentReferenceTable {
    entries: Vec<(String, Vec<String>)>,
}

impl ComponentReferenceTable {
    pub fn from_entries(mut entries: Vec<(String, Vec<String>)>) -> Self {
        entries.sort_unstable_by(|a,b|a.0.cmp(&b.0));
        Self { entries }
    }
    pub fn into_entries(self) -> Vec<(String, Vec<String>)> { self.entries }
    pub fn iter(&self) -> std::slice::Iter<'_,(String,Vec<String>)> { self.entries.iter() }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn get(&self, key: &str) -> Option<&Vec<String>> { self.entries.binary_search_by(|entry|entry.0.as_str().cmp(key)).ok().map(|index|&self.entries[index].1) }
    pub fn original_capacity_bytes(&self) -> Option<usize> {
        self.entries.iter().try_fold(self.entries.capacity().checked_mul(size_of::<(String,Vec<String>)>())?,|sum,(key,labels)|labels.iter().try_fold(sum.checked_add(key.capacity())?.checked_add(labels.capacity().checked_mul(size_of::<String>())?)?,|sum,label|sum.checked_add(label.capacity())))
    }
    pub fn directory_capacity_bytes(&self)->Option<usize> {
        self.entries.iter().try_fold(self.entries.capacity().checked_mul(size_of::<(String,Vec<String>)>())?,|sum,(key,labels)|sum.checked_add(key.capacity())?.checked_add(labels.capacity().checked_mul(size_of::<String>())?))
    }
}

impl<'a> IntoIterator for &'a ComponentReferenceTable {
    type Item = &'a (String,Vec<String>);
    type IntoIter = std::slice::Iter<'a,(String,Vec<String>)>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl std::ops::Index<&str> for ComponentReferenceTable {
    type Output = Vec<String>;
    fn index(&self, key:&str) -> &Self::Output { self.get(key).expect("component domain") }
}
impl semio_framework_value::ToValue for ComponentReferenceTable {
    fn to_value(&self) -> semio_framework_value::DslValue {
        use semio_framework_value::ToValue;
        semio_framework_value::DslValue::Object(self.entries.iter().map(|(key,labels)|(key.clone(),labels.to_value())).collect())
    }
}
impl semio_framework_value::FromValue for ComponentReferenceTable {
    fn from_value(value:semio_framework_value::DslValue) -> Result<Self,semio_framework_value::ValueError> {
        use semio_framework_value::FromValue;
        let mut entries=Vec::new();
        for(key,value)in value.into_object()? { entries.push((key,Vec::<String>::from_value(value)?)); }
        let table=Self::from_entries(entries);
        if table.entries.windows(2).any(|pair|pair[0].0==pair[1].0) { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"duplicate component domain")); }
        Ok(table)
    }
}
semio_framework_value::artifact_retire_struct!(ComponentReferenceTable { entries });

#[cfg(any(test, feature = "testkit"))]
impl serde::Serialize for ComponentReferenceTable {
    fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error> {
        use serde::ser::SerializeMap;
        let mut map=serializer.serialize_map(Some(self.len()))?;
        for(key,labels)in self { map.serialize_entry(key,labels)?; }
        map.end()
    }
}
#[cfg(any(test, feature = "testkit"))]
impl<'de> serde::Deserialize<'de> for ComponentReferenceTable {
    fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error> {
        let map=<std::collections::BTreeMap<String,Vec<String>> as serde::Deserialize>::deserialize(deserializer)?;
        Ok(Self::from_entries(map.into_iter().collect()))
    }
}

use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText as Text, ArtifactCanonicalJsonTree as Tree};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn absent(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
impl Tree for ComponentReferenceTable {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(self.len()))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{self.iter().nth(ordinal).map(|(_,labels)|labels as&dyn Tree).ok_or_else(||absent("canonical component reference ordinal is absent"))}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{self.iter().nth(ordinal).map(|(key,_)|Text::from(key.as_str())).ok_or_else(||absent("canonical component reference key is absent"))}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
