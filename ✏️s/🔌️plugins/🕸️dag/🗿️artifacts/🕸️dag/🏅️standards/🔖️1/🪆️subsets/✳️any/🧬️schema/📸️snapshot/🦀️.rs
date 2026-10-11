//! 🕸️ Persisted DAG marker and its composed `graph` content child handle; the content lives in the child's own store.
use crate::DagContentChild;
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted DAG document snapshot — schema tag plus the composed `graph` content child.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[dsl(id = "dag.dag", layout = "lines")]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: DagContentChild,
}

impl semio_framework_value::FromValue for DagSnapshot {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let mut schema = None;
        let mut content = None;
        for (key, value) in semio_framework_value::DslValue::into_object(value)? {
            match key.as_str() {
                "schema" if schema.is_none() => schema = Some(semio_framework_value::FromValue::from_value(value)?),
                "content" if content.is_none() => content = Some(semio_framework_value::FromValue::from_value(value)?),
                _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown or duplicate Dag field {key}"))),
            }
        }
        let result = Self { schema: schema.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Dag schema"))?, content: content.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Dag content"))? };
        result.validate().map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message))?;
        Ok(result)
    }
    fn from_value_controlled(value: &semio_framework_value::DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        let entries = value.object_controlled(control)?;
        semio_framework_value::DslValue::deny_fields_controlled(entries, &["schema", "content"], control)?;
        control.charge(std::mem::size_of::<Self>())?;
        let schema = semio_framework_value::DslValue::field_controlled(entries, "schema", control)?.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Dag schema"))?;
        let schema = <String as semio_framework_value::FromValue>::from_value_controlled(schema, control)?;
        let child = semio_framework_value::DslValue::field_controlled(entries, "content", control)?.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Dag content"))?;
        let child = <DagContentChild as semio_framework_value::FromValue>::from_value_controlled(child, control)?;
        let result = semio_framework_value::DecodedValue::new(Self { schema, content: child }, Self::retire_decoded);
        result.get().validate().map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message))?;
        control.checkpoint()?;
        Ok(result.take())
    }
    fn retire_decoded(self) {
        let factory = semio_framework_value::retirement::OwnedValueRetirementFactory::<DagContentChild>::default();
        let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: store::ArtifactOwnedValueRetirementFactory::retirement_birth_bytes(&factory, &self.content), maximum_depth: 2, ..Default::default() };
        let (mut cursor, _) = store::ArtifactOwnedValueRetirementFactory::retire_owned(&factory, self.content, grant).unwrap_or_else(|(error, _)| panic!("DAG child retirement admission refused: {error}"));
        store::test_support::drive_retirement(cursor.as_mut()).expect("DAG child closes with its exact owner");
        assert!(cursor.terminal_is_empty());
    }
}

impl DagSnapshot {
    /// 🪆️ Enforces the document marker and exact owned-child coordinates.
    pub fn validate(&self) -> Result<(), String> {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if self.schema != "dag.dag" {
            return Err("invalid Dag document marker".into());
        }
        validate_semio_child_identity(&self.content.child_id, &self.content.target, "graph")?;
        Ok(())
    }
}

impl Default for DagSnapshot {
    fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app and examples.
pub fn default_snapshot() -> DagSnapshot {
    crate::examples::demo::snapshot()
}
/// 🫙️ The empty document — schema marker plus an owned `graph` child with no nodes or edges. The
/// example picker's "no example" selection loads this, so it must satisfy `validate()` exactly the
/// way `default_snapshot()` does.
pub fn empty_snapshot() -> DagSnapshot {
    DagSnapshot { schema: crate::DAG_DOCUMENT_SCHEMA.into(), content: crate::dag_content_child_handle(&crate::DagScene::default()) }
}
//#endregion 🔖️Snapshot


//#region 🌉️ExternalCodecBridge








//#endregion 🌉️ExternalCodecBridge



