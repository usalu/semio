//! 📐️ The detail model: path segments, values, presentations, variants, capabilities and the provider trait.

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnapshotDetailPathSegment {
    Key(String),
    Index(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SnapshotDetailValue {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Bytes { len: usize },
    Array,
    Object,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotDetailPresentation {
    pub label: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SnapshotDetailVariant {
    pub label: String,
    pub value: DslValue,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnapshotDetailItemCapabilities {
    pub edit: bool,
    pub rename: bool,
    pub remove: bool,
    pub reorder: bool,
}

impl SnapshotDetailItemCapabilities {
    pub(super) fn for_path(path: &[SnapshotDetailPathSegment]) -> Self {
        Self { edit: true, rename: matches!(path.last(), Some(SnapshotDetailPathSegment::Key(_))), remove: !path.is_empty(), reorder: matches!(path.last(), Some(SnapshotDetailPathSegment::Index(_))) }
    }
}

pub trait SnapshotDetailsProvider {
    fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue>;
    fn child_count(&self, path: &[SnapshotDetailPathSegment]) -> usize;
    fn object_key(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<String>;
    fn has_source(&self) -> bool {
        false
    }
    fn source(&self) -> Option<String> {
        None
    }
    fn creation_template(&self, _path: &[SnapshotDetailPathSegment], _collection_item: bool) -> Option<DslValue> {
        None
    }
    fn missing_property_templates(&self, _path: &[SnapshotDetailPathSegment]) -> Vec<(String, DslValue)> {
        Vec::new()
    }
    fn missing_property_count(&self, path: &[SnapshotDetailPathSegment]) -> usize {
        self.missing_property_templates(path).len()
    }
    fn missing_property_template(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<(String, DslValue)> {
        self.missing_property_templates(path).into_iter().nth(index)
    }
    fn allows_untyped_creation(&self, _path: &[SnapshotDetailPathSegment]) -> bool {
        true
    }
    fn enum_values(&self, _path: &[SnapshotDetailPathSegment]) -> Vec<DslValue> {
        Vec::new()
    }
    fn presentation(&self, _path: &[SnapshotDetailPathSegment], _locale: Locale) -> Option<SnapshotDetailPresentation> {
        None
    }
    fn item_capabilities(&self, path: &[SnapshotDetailPathSegment]) -> SnapshotDetailItemCapabilities {
        SnapshotDetailItemCapabilities::for_path(path)
    }
    fn allows_collection_insert(&self, _path: &[SnapshotDetailPathSegment]) -> bool {
        true
    }
    fn collection_insertion_key(&self, _path: &[SnapshotDetailPathSegment]) -> Option<String> {
        None
    }
    fn variants(&self, _path: &[SnapshotDetailPathSegment], _locale: Locale) -> Vec<SnapshotDetailVariant> {
        Vec::new()
    }
    fn variant_count(&self, path: &[SnapshotDetailPathSegment], locale: Locale) -> usize {
        self.variants(path, locale).len()
    }
    fn variant(&self, path: &[SnapshotDetailPathSegment], locale: Locale, index: usize) -> Option<SnapshotDetailVariant> {
        self.variants(path, locale).into_iter().nth(index)
    }
}
