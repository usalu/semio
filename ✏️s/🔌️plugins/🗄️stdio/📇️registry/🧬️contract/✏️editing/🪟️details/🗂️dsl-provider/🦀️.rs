//! 🗂️ The DSL snapshot details provider over a typed snapshot value.

use super::*;

enum DslSnapshotDetailsValue<'a, S> {
    Snapshot(&'a S),
    Materialized(DslValue),
}

pub struct DslSnapshotDetailsProvider<'a, S> {
    pub(super) value: DslSnapshotDetailsValue<'a, S>,
    pub(super) schema: Option<Arc<DslValue>>,
}

impl<'a, S: ArtifactDsl + ToValue> DslSnapshotDetailsProvider<'a, S> {
    pub fn new(snapshot: &'a S) -> Self {
        Self::new_with_schema_hint(snapshot, None)
    }

    pub fn new_with_schema_hint(snapshot: &'a S, _controller_id: Option<&str>) -> Self {
        let schema = match snapshot.value_at_path(&["schema"]) {
            Ok(DslValue::String(schema_id)) => snapshot_schema(&schema_id),
            _ => None,
        };
        Self { value: DslSnapshotDetailsValue::Snapshot(snapshot), schema }
    }

    pub fn from_value(value: DslValue) -> Self {
        let schema = match value.value_at_path(&["schema"]) {
            Ok(DslValue::String(schema_id)) => snapshot_schema(&schema_id),
            _ => None,
        };
        Self { value: DslSnapshotDetailsValue::Materialized(value), schema }
    }

    pub fn from_value_and_schema(value: DslValue, schema: &str) -> Self {
        Self {
            value: DslSnapshotDetailsValue::Materialized(value),
            schema: semio_framework_pack_json::from_json_str(schema, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().map(|schema| Arc::new(bundle_snapshot_schema(schema, Vec::new()))),
        }
    }

    fn path(path: &[SnapshotDetailPathSegment]) -> Vec<String> {
        path.iter()
            .map(|segment| match segment {
                SnapshotDetailPathSegment::Key(key) => key.clone(),
                SnapshotDetailPathSegment::Index(index) => index.to_string(),
            })
            .collect()
    }

    fn with_path<R>(&self, path: &[SnapshotDetailPathSegment], read: impl FnOnce(&dyn ToValue, &[&str]) -> Result<R, semio_framework_value::ValueError>) -> Option<R> {
        let path = Self::path(path);
        let path = path.iter().map(String::as_str).collect::<Vec<_>>();
        match &self.value {
            DslSnapshotDetailsValue::Snapshot(snapshot) => read(*snapshot, &path).ok(),
            DslSnapshotDetailsValue::Materialized(value) => read(value, &path).ok(),
        }
    }

    fn shape(&self, path: &[SnapshotDetailPathSegment]) -> Option<ValueShape> {
        self.with_path(path, |value, path| value.value_shape_at_path(path))
    }

    fn scalar(&self, path: &[SnapshotDetailPathSegment]) -> Option<DslValue> {
        self.with_path(path, |value, path| value.value_at_path(path))
    }

    fn schema_at_path<'b>(&'b self, root: &'b DslValue, path: &[SnapshotDetailPathSegment]) -> Option<&'b DslValue> {
        let mut read = |path: &[SnapshotDetailPathSegment]| self.scalar(path);
        let mut shape = |path: &[SnapshotDetailPathSegment]| self.shape(path);
        schema_at_path(root, path, &mut read, &mut shape)
    }

    fn object_schema_at_path<'b>(&'b self, root: &'b DslValue, path: &[SnapshotDetailPathSegment]) -> Option<&'b DslValue> {
        let schema = self.schema_at_path(root, path)?;
        let mut read = |path: &[SnapshotDetailPathSegment]| self.scalar(path);
        let mut shape = |path: &[SnapshotDetailPathSegment]| self.shape(path);
        object_schema(root, schema, path, &mut read, &mut shape)
    }

    fn creation_schema_at_path<'b>(&'b self, root: &'b DslValue, path: &[SnapshotDetailPathSegment]) -> Option<&'b DslValue> {
        let mut read = |path: &[SnapshotDetailPathSegment]| self.scalar(path);
        let mut shape = |path: &[SnapshotDetailPathSegment]| self.shape(path);
        creation_schema_at_path(root, path, &mut read, &mut shape)
    }

    fn variant_from_member(&self, root: &DslValue, current: Option<&DslValue>, member: &DslValue, path: &[SnapshotDetailPathSegment], locale: Locale) -> Option<SnapshotDetailVariant> {
        let target = resolved_schema(root, member, 0)?;
        let mut value = template_from_schema(root, target, 0)?;
        let DslValue::Object(fields) = &mut value else { return None };
        let DslValue::Object(target_properties) = object_field(target, "properties")? else { return None };
        let current_properties = current.and_then(|schema| resolved_schema(root, schema, 0)).and_then(|schema| object_field(schema, "properties"));
        if let Some(DslValue::Object(current_properties)) = current_properties {
            for (key, current_schema) in current_properties {
                let Some((_, target_schema)) = target_properties.iter().find(|(target, _)| target == key) else { continue };
                if resolved_schema(root, current_schema, 0) != resolved_schema(root, target_schema, 0) {
                    continue;
                }
                let mut field_path = path.to_vec();
                field_path.push(SnapshotDetailPathSegment::Key(key.clone()));
                let Some(field_shape) = self.shape(&field_path) else { continue };
                if !matches!(field_shape, ValueShape::Null | ValueShape::Bool | ValueShape::Number | ValueShape::String) {
                    continue;
                }
                let Some(field_value) = self.scalar(&field_path) else { continue };
                if !schema_accepts_scalar(root, target_schema, &field_shape, &field_value) {
                    continue;
                }
                if let Some((_, value)) = fields.iter_mut().find(|(field, _)| field == key) {
                    *value = field_value;
                } else {
                    fields.push((key.clone(), field_value));
                }
            }
        }
        if !template_candidate_can_be_offered(root, target, &value, 0) {
            return None;
        }
        Some(SnapshotDetailVariant { label: variant_label(root, target, locale)?, value })
    }
}

impl<S: ArtifactDsl + ToValue> SnapshotDetailsProvider for DslSnapshotDetailsProvider<'_, S> {
    fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue> {
        Some(match self.shape(path)? {
            ValueShape::Null => SnapshotDetailValue::Null,
            ValueShape::Bool => match self.scalar(path)? {
                DslValue::Bool(value) => SnapshotDetailValue::Bool(value),
                _ => return None,
            },
            ValueShape::Number => match self.scalar(path)? {
                DslValue::Number(value) => SnapshotDetailValue::Number(value),
                _ => return None,
            },
            ValueShape::String => match self.scalar(path)? {
                DslValue::String(value) => SnapshotDetailValue::String(value),
                _ => return None,
            },
            ValueShape::Array { .. } => SnapshotDetailValue::Array,
            ValueShape::Bytes { len } => SnapshotDetailValue::Bytes { len },
            ValueShape::Object { .. } => SnapshotDetailValue::Object,
        })
    }

    fn child_count(&self, path: &[SnapshotDetailPathSegment]) -> usize {
        match self.shape(path) {
            Some(ValueShape::Array { len } | ValueShape::Object { len }) => len,
            _ => 0,
        }
    }

    fn object_key(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<String> {
        self.with_path(path, |value, path| value.value_key_at_path(path, index))
    }

    fn source(&self) -> Option<String> {
        match &self.value {
            DslSnapshotDetailsValue::Snapshot(snapshot) => Some(snapshot_edit_source(*snapshot)),
            DslSnapshotDetailsValue::Materialized(_) => None,
        }
    }

    fn has_source(&self) -> bool {
        matches!(&self.value, DslSnapshotDetailsValue::Snapshot(_))
    }

    fn creation_template(&self, path: &[SnapshotDetailPathSegment], collection_item: bool) -> Option<DslValue> {
        let root = self.schema.as_ref()?;
        let mut schema = self.creation_schema_at_path(root, path)?;
        if collection_item {
            let child_keyword = match self.shape(path)? {
                ValueShape::Array { .. } => "items",
                ValueShape::Object { .. } => "additionalProperties",
                _ => return None,
            };
            schema = if let Some(members) = union_members(schema) { members.iter().find_map(|member| resolved_schema(root, member, 0).and_then(|member| object_field(member, child_keyword)))? } else { object_field(schema, child_keyword)? };
            schema = resolved_schema(root, schema, 0)?;
        }
        template_from_schema(root, schema, 0)
    }

    fn missing_property_templates(&self, path: &[SnapshotDetailPathSegment]) -> Vec<(String, DslValue)> {
        (0..self.missing_property_count(path)).filter_map(|index| self.missing_property_template(path, index)).collect()
    }

    fn missing_property_count(&self, path: &[SnapshotDetailPathSegment]) -> usize {
        let Some(root) = self.schema.as_ref() else { return 0 };
        let Some(schema) = self.object_schema_at_path(root, path) else { return 0 };
        let Some(DslValue::Object(properties)) = object_field(schema, "properties") else { return 0 };
        properties
            .iter()
            .filter(|(key, schema)| {
                let mut property = path.to_vec();
                property.push(SnapshotDetailPathSegment::Key(key.clone()));
                self.shape(&property).is_none() && template_from_schema(root, schema, 0).is_some()
            })
            .count()
    }

    fn missing_property_template(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<(String, DslValue)> {
        let root = self.schema.as_ref()?;
        let schema = self.object_schema_at_path(root, path)?;
        let DslValue::Object(properties) = object_field(schema, "properties")? else { return None };
        properties
            .iter()
            .filter_map(|(key, schema)| {
                let mut property = path.to_vec();
                property.push(SnapshotDetailPathSegment::Key(key.clone()));
                self.shape(&property).is_none().then(|| template_from_schema(root, schema, 0).map(|value| (key.clone(), value))).flatten()
            })
            .nth(index)
    }

    fn allows_untyped_creation(&self, path: &[SnapshotDetailPathSegment]) -> bool {
        let Some(root) = self.schema.as_ref() else { return true };
        let Some(schema) = self.schema_at_path(root, path) else { return true };
        let Some(schema) = resolved_schema(root, schema, 0) else { return true };
        match self.shape(path) {
            Some(ValueShape::Array { .. }) => object_field(schema, "items").is_none(),
            Some(ValueShape::Object { .. }) => match object_field(schema, "additionalProperties") {
                Some(DslValue::Bool(allowed)) => *allowed,
                Some(DslValue::Object(_)) => false,
                None => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn enum_values(&self, path: &[SnapshotDetailPathSegment]) -> Vec<DslValue> {
        let Some(root) = self.schema.as_ref() else { return Vec::new() };
        let Some(schema) = self.schema_at_path(root, path) else { return Vec::new() };
        let mut values = Vec::new();
        schema_enum_values(root, schema, 0, &mut values);
        values
    }

    fn presentation(&self, path: &[SnapshotDetailPathSegment], locale: Locale) -> Option<SnapshotDetailPresentation> {
        let (SnapshotDetailPathSegment::Key(key), parent) = path.split_last()? else { return None };
        let root = self.schema.as_ref()?;
        let parent_schema = self.object_schema_at_path(root, parent)?;
        let properties = object_field(parent_schema, "properties")?;
        let schema = object_field(properties, key)?;
        let friendly = localized_schema_annotation(schema, "title", locale).unwrap_or_else(|| humanized_schema_key(key, locale));
        Some(SnapshotDetailPresentation { label: format!("{friendly} [{key}]"), description: localized_schema_annotation(schema, "description", locale) })
    }

    fn item_capabilities(&self, path: &[SnapshotDetailPathSegment]) -> SnapshotDetailItemCapabilities {
        let mut capabilities = SnapshotDetailItemCapabilities::for_path(path);
        if matches!(path.last(), Some(SnapshotDetailPathSegment::Key(key)) if key == "schema") {
            return SnapshotDetailItemCapabilities { edit: false, rename: false, remove: false, reorder: false };
        }
        let Some((segment, parent)) = path.split_last() else {
            capabilities.rename = false;
            capabilities.remove = false;
            capabilities.reorder = false;
            return capabilities;
        };
        let Some(root) = self.schema.as_ref() else { return capabilities };
        match segment {
            SnapshotDetailPathSegment::Key(key) => {
                let Some(schema) = self.object_schema_at_path(root, parent) else { return capabilities };
                if matches!(object_field(schema, "properties"), Some(properties) if object_field(properties, key).is_some()) {
                    capabilities.rename = false;
                }
                if object_field(schema, "properties").and_then(|properties| object_field(properties, key)).and_then(|property| resolved_schema(root, property, 0)).and_then(|property| object_field(property, "const")).is_some() {
                    capabilities.edit = false;
                }
                if schema_requires_property(schema, key) {
                    capabilities.remove = false;
                }
                if schema_usize(object_field(schema, "minProperties")).is_some_and(|minimum| self.child_count(parent) <= minimum) {
                    capabilities.remove = false;
                }
                let mut read = |path: &[SnapshotDetailPathSegment]| self.scalar(path);
                let mut shape = |path: &[SnapshotDetailPathSegment]| self.shape(path);
                if creation_schema_at_path(root, parent, &mut read, &mut shape).and_then(|schema| union_discriminator_key(root, schema)).as_deref() == Some(key) {
                    capabilities.edit = false;
                }
                capabilities.reorder = false;
            }
            SnapshotDetailPathSegment::Index(_) => {
                let Some(schema) = self.schema_at_path(root, parent) else { return capabilities };
                let length = self.child_count(parent);
                capabilities.rename = false;
                capabilities.remove = schema_usize(object_field(schema, "minItems")).is_none_or(|minimum| length > minimum);
            }
        }
        capabilities
    }

    fn allows_collection_insert(&self, path: &[SnapshotDetailPathSegment]) -> bool {
        let Some(root) = self.schema.as_ref() else { return true };
        let Some(schema) = self.schema_at_path(root, path) else { return true };
        let length = self.child_count(path);
        match self.shape(path) {
            Some(ValueShape::Array { .. }) => schema_usize(object_field(schema, "maxItems")).is_none_or(|maximum| length < maximum),
            Some(ValueShape::Object { .. }) => schema_usize(object_field(schema, "maxProperties")).is_none_or(|maximum| length < maximum),
            _ => false,
        }
    }

    fn collection_insertion_key(&self, path: &[SnapshotDetailPathSegment]) -> Option<String> {
        if !matches!(self.shape(path), Some(ValueShape::Object { .. })) {
            return None;
        }
        let length = self.child_count(path);
        for ordinal in [length.saturating_add(1), length.saturating_add(2), length.saturating_add(3), length.saturating_add(4)] {
            let key = format!("new{ordinal}");
            let mut candidate = path.to_vec();
            candidate.push(SnapshotDetailPathSegment::Key(key.clone()));
            if self.shape(&candidate).is_none() {
                return Some(key);
            }
        }
        None
    }

    fn variants(&self, path: &[SnapshotDetailPathSegment], locale: Locale) -> Vec<SnapshotDetailVariant> {
        (0..self.variant_count(path, locale)).filter_map(|index| self.variant(path, locale, index)).collect()
    }

    fn variant_count(&self, path: &[SnapshotDetailPathSegment], locale: Locale) -> usize {
        let Some(root) = self.schema.as_ref() else { return 0 };
        let Some(schema) = self.creation_schema_at_path(root, path) else { return 0 };
        let Some(members) = union_members(schema) else { return 0 };
        let mut read = |path: &[SnapshotDetailPathSegment]| self.scalar(path);
        let mut shape = |path: &[SnapshotDetailPathSegment]| self.shape(path);
        let current = selected_union_member(root, members, path, &mut read, &mut shape);
        members.iter().filter(|member| current.is_none_or(|current| !std::ptr::eq(*member, current))).filter(|member| self.variant_from_member(root, current, member, path, locale).is_some()).count()
    }

    fn variant(&self, path: &[SnapshotDetailPathSegment], locale: Locale, index: usize) -> Option<SnapshotDetailVariant> {
        let root = self.schema.as_ref()?;
        let schema = self.creation_schema_at_path(root, path)?;
        let members = union_members(schema)?;
        let mut read = |path: &[SnapshotDetailPathSegment]| self.scalar(path);
        let mut shape = |path: &[SnapshotDetailPathSegment]| self.shape(path);
        let current = selected_union_member(root, members, path, &mut read, &mut shape);
        members.iter().filter(|member| current.is_none_or(|current| !std::ptr::eq(*member, current))).filter_map(|member| self.variant_from_member(root, current, member, path, locale)).nth(index)
    }
}
