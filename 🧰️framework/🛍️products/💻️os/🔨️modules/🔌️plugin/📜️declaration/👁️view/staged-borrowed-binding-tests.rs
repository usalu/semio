//! 🧪️ Borrowed staged bindings reproduce neutral wire while paid payload cursors own all construction.
use super::*;
use semio_framework::action_staging::*;
use semio_framework_plugin_decl_staged_form::*;

struct Borrowed<'a> { binding: &'a StagedFormBinding }
impl Borrowed<'_> {
    fn path(&self) -> &[String] {
        match &self.binding.edits[0] {
            StagedFormBindingEdit::Fixed { edit: StagedFormEdit::Set { path, .. } | StagedFormEdit::Append { path, .. } | StagedFormEdit::Remove { path } } | StagedFormBindingEdit::Text { path } | StagedFormBindingEdit::Choice { path, .. } => path,
        }
    }
    fn value(&self) -> &DslValue {
        match &self.binding.edits[0] {
            StagedFormBindingEdit::Fixed { edit: StagedFormEdit::Set { value, .. } | StagedFormEdit::Append { value, .. } } => value,
            StagedFormBindingEdit::Choice { options, .. } => &options[0].value,
            _ => unreachable!(),
        }
    }
}
impl StagedBindingPathSource for Borrowed<'_> {
    fn len(&self) -> usize { self.path().len() }
    fn segment(&self, index: usize) -> Option<&str> { self.path().get(index).map(String::as_str) }
}
impl StagedBindingOptionsSource for Borrowed<'_> {
    fn len(&self) -> usize { 1 }
    fn option(&self, index: usize) -> Option<StagedBindingOptionView<'_>> {
        let StagedFormBindingEdit::Choice { options, .. } = &self.binding.edits[0] else { return None };
        let option = options.get(index)?;
        Some(StagedBindingOptionView { key: &option.key, value: StagedBindingValueView::Payload(self) })
    }
}
impl StagedPayloadSource for Borrowed<'_> {
    fn identity(&self) -> usize { self.binding as *const _ as usize }
    fn node(&self, path: &[usize]) -> Result<StagedPayloadNode<'_>, semio_framework_value::ValueError> {
        let invalid = || semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "test.binding");
        let mut value = self.value(); let mut key = None;
        for index in path { match value { DslValue::Object(fields) => { let field = fields.get(*index).ok_or_else(invalid)?; key = Some(field.0.as_str()); value = &field.1; }, DslValue::Array(values) => { key = None; value = values.get(*index).ok_or_else(invalid)?; }, _ => return Err(invalid()) } }
        let value = match value { DslValue::Null => StagedPayloadKind::Null, DslValue::Bool(value) => StagedPayloadKind::Bool(*value), DslValue::String(value) => StagedPayloadKind::Text(value), DslValue::Number(value) => StagedPayloadKind::Number(value.as_f64()), DslValue::Object(fields) => StagedPayloadKind::Map(fields.len()), DslValue::Array(values) => StagedPayloadKind::List(values.len()), _ => return Err(invalid()) };
        Ok(StagedPayloadNode { key, value })
    }
}
impl StagedFormBindingSource for Borrowed<'_> {
    fn token(&self) -> &StagedFormToken { &self.binding.token }
    fn layout(&self) -> StagedFormLayout { self.binding.layout }
    fn edits(&self) -> usize { 1 }
    fn edit(&self, index: usize) -> Result<StagedBindingEditView<'_>, semio_framework_value::ValueError> {
        if index != 0 { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "test.binding")); }
        Ok(match &self.binding.edits[0] {
            StagedFormBindingEdit::Text { .. } => StagedBindingEditView::Text { path: self },
            StagedFormBindingEdit::Choice { kind, .. } => StagedBindingEditView::Choice { kind: *kind, path: self, options: self },
            StagedFormBindingEdit::Fixed { edit } => StagedBindingEditView::Fixed(match edit {
                StagedFormEdit::Set { .. } => StagedBindingFixedView::Set { path: self, value: StagedBindingValueView::Payload(self) },
                StagedFormEdit::Append { .. } => StagedBindingFixedView::Append { path: self, value: StagedBindingValueView::Payload(self) },
                StagedFormEdit::Remove { .. } => StagedBindingFixedView::Remove { path: self },
            }),
        })
    }
}
#[test]
fn borrowed_binding_sources_match_neutral_exact_payloads_without_intermediate_allocation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/binding-cases.json")).unwrap();
    let pages: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/page-cases.json")).unwrap();
    let mut warm = UiMapBuilder::try_new().unwrap(); warm.try_insert("warm".into(), UiValue::Null).unwrap(); close(UiValue::Map(warm.finish()));
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 0, maximum_release_bytes: 32768, maximum_depth: 64 };
    for row in fixture["cases"].as_array().unwrap() {
        let expected = serde_json::json!({"token":pages["token"],"layout":if row["edit"]["source"]=="text"{"preserve"}else{"replace"},"edits":[row["edit"]]});
        let binding: StagedFormBinding = serde_json::from_value(expected.clone()).unwrap();
        let source = Borrowed { binding: &binding };
        validate_staged_binding_source(&source).unwrap();
        let payload = StagedBindingPayload::Action(&source);
        let mut cursor = StagedPayloadCursor::default(); let mut output = None;
        for _ in 0..1024 {
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.step(&payload, &mut output, grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            if matches!(result.unwrap(), RetainedCloneStep::Complete(_)) { break; }
        }
        let output = output.unwrap(); assert_eq!(serde_json::to_value(&output).unwrap(), expected); close(output);
        assert!(cursor.terminal_is_empty());
    }
}

#[test]
fn heap_witness_observes_actual_allocation_before_borrowed_binding_claims() {
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
        let allocation = vec![std::hint::black_box(7u8); std::hint::black_box(73)];
        std::hint::black_box(&allocation);
        drop(allocation);
    });
    assert!(heap.requested_bytes >= 73 && heap.released_bytes >= 73);
}
