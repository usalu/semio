//! 🎯️ Canonical component labels retain one measurable ordered backing allocation.

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
        self.entries.iter().try_fold(self.entries.capacity().checked_mul(std::mem::size_of::<(String,Vec<String>)>())?,|sum,(key,labels)|labels.iter().try_fold(sum.checked_add(key.capacity())?.checked_add(labels.capacity().checked_mul(std::mem::size_of::<String>())?)?,|sum,label|sum.checked_add(label.capacity())))
    }
    pub fn directory_capacity_bytes(&self)->Option<usize> {
        self.entries.iter().try_fold(self.entries.capacity().checked_mul(std::mem::size_of::<(String,Vec<String>)>())?,|sum,(key,labels)|sum.checked_add(key.capacity())?.checked_add(labels.capacity().checked_mul(std::mem::size_of::<String>())?))
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
impl pack::value::ToValue for ComponentReferenceTable {
    fn to_value(&self) -> pack::value::DslValue {
        use pack::value::ToValue;
        pack::value::DslValue::Object(self.entries.iter().map(|(key,labels)|(key.clone(),labels.to_value())).collect())
    }
}
impl pack::value::FromValue for ComponentReferenceTable {
    fn from_value(value:pack::value::DslValue) -> Result<Self,pack::value::ValueError> {
        use pack::value::FromValue;
        let mut entries=Vec::new();
        for(key,value)in value.into_object()? { entries.push((key,Vec::<String>::from_value(value)?)); }
        let table=Self::from_entries(entries);
        if table.entries.windows(2).any(|pair|pair[0].0==pair[1].0) { return Err(pack::value::ValueError::new(pack::value::ValueRefusalKind::InvalidValue,"duplicate component domain")); }
        Ok(table)
    }
}
pack::value::artifact_retire_struct!(ComponentReferenceTable { entries });

#[cfg(test)]
impl serde::Serialize for ComponentReferenceTable {
    fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error> {
        use serde::ser::SerializeMap;
        let mut map=serializer.serialize_map(Some(self.len()))?;
        for(key,labels)in self { map.serialize_entry(key,labels)?; }
        map.end()
    }
}
#[cfg(test)]
impl<'de> serde::Deserialize<'de> for ComponentReferenceTable {
    fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error> {
        let map=<std::collections::BTreeMap<String,Vec<String>> as serde::Deserialize>::deserialize(deserializer)?;
        Ok(Self::from_entries(map.into_iter().collect()))
    }
}
