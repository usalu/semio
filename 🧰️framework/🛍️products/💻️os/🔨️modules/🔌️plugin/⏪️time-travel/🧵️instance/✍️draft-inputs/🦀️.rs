//! ✍️ Draft inputs of the instance time-travel turns: the drafted input and the drafted selection of a history edit session. See [`super`].

use super::*;
use semio_framework::Effect;
use semio_framework::Fault;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TimeTravelCtx<'ctx, A, M, X> {
    /// 🎚️ Drafts one input: the host value coerced to the shape of the input `path` addresses in the variant the draft is
    /// in, set at `path` in the draft payload (a union's selector switches the variant: [`time_travel_switch_variant`]) —
    /// or, with a list `edit`, one item inserted at `path` (`<list>/<index>` or `<list>/-`; the host value coerced to the
    /// item, else the item's [`time_travel_default_value`]) or removed there — validated against the leaf payload schema
    /// (with its root discriminators spliced back; `minItems`/`maxItems` included), rebuilt into the same kind and
    /// canonically encoded. A refused value keeps the draft and names the reason on the editor (on the list's row for a
    /// list edit). Fails closed: a payload schema that compiles no validator or describes no inputs admits no draft
    /// (`timeTravel.schema-unavailable`), never an unvalidated payload.
    #[allow(clippy::too_many_arguments, reason = "the verb's path, value and list edit beside the session plumbing every draft verb carries")]
    pub(super) async fn draft_time_travel_input(&mut self, path: Option<&str>, value: Option<DslValue>, edit: Option<&str>, generation: u32, meta: &ActionMeta, effects: &mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TimeTravelActionOutcome, Fault> {
        if generation != self.time_travel.session.generation {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Stale)));
        }
        let Some(editor) = self.time_travel.editor.as_mut().filter(|_| self.time_travel.session.stage == TimeTravelStage::Editing) else {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal)));
        };
        let Some(schema) = editor.schema else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::NotEditable)) };
        let unavailable = match (&editor.validator, &editor.inputs_refused) {
            (Err(reason), _) => Some(format!("the payload schema compiles no validator, so no draft is admitted: {reason}")),
            (Ok(_), Some(error)) => Some(format!("the payload schema describes no inputs, so no draft is admitted: {}", error.detail)),
            (Ok(_), None) => None,
        };
        if let Some(reason) = unavailable {
            editor.refused = Some((path.unwrap_or_default().to_string(), reason));
            self.time_travel.ui_dirty = true;
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::SchemaUnavailable));
        }
        let Some(path) = path else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
        let refuse = |editor: &mut TimeTravelEditor, at: &str, reason: String| {
            editor.refused = Some((at.to_string(), reason));
            TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::InvalidInput)
        };
        let (at, candidate) = match edit {
            Some(edit) => {
                let Some((list, _)) = path.rsplit_once('/') else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
                let Some((list_input, false)) = time_travel_input_at(&editor.inputs, &editor.value, list) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
                let (item_input, element) = match &list_input.schema {
                    ArgSchema::Array { items, .. } => (ActionArgDef { schema: items.as_ref().clone(), required: true, nullable: false, default: None, ..list_input.clone() }, false),
                    ArgSchema::Reference { many: true, .. } => (list_input.clone(), true),
                    _ => return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)),
                };
                let mut candidate = editor.value.clone();
                let applied = match edit {
                    HISTORY_EDIT_INPUT_INSERT => {
                        let item = match &value {
                            Some(value) => time_travel_coerce(&item_input, element, value),
                            None => (!element).then(|| time_travel_default_value(&item_input.schema)),
                        };
                        let Some(item) = item else {
                            let outcome = refuse(editor, list, format!("{list} does not take this item"));
                            self.time_travel.ui_dirty = true;
                            return Ok(outcome);
                        };
                        time_travel_pointer_insert(&mut candidate, path, item)
                    }
                    HISTORY_EDIT_INPUT_REMOVE => time_travel_pointer_remove(&mut candidate, path),
                    _ => return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::InvalidInput)),
                };
                if !applied {
                    return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput));
                }
                (list, candidate)
            }
            None => {
                let Some((input, element)) = time_travel_input_at(&editor.inputs, &editor.value, path) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
                let Some(coerced) = value.as_ref().and_then(|value| time_travel_coerce(&input, element, value)) else {
                    let outcome = refuse(editor, path, format!("{path} does not take this value"));
                    self.time_travel.ui_dirty = true;
                    return Ok(outcome);
                };
                let coerced = match (&input.schema, coerced) {
                    (ArgSchema::Vector { dims: 4, .. }, DslValue::Array(mut components)) if components.len() == 3 => {
                        components.push(time_travel_pointer_get(&editor.value, &format!("{path}/3")).cloned().unwrap_or_else(|| DslValue::float(1.0)));
                        DslValue::Array(components)
                    }
                    (_, coerced) => coerced,
                };
                let switched = time_travel_variant_selector(&editor.inputs).filter(|selector| selector.id == path).zip(coerced.as_str()).map(|(selector, variant)| time_travel_switch_variant(&editor.inputs, selector, &editor.value, variant));
                let candidate = match switched {
                    Some(candidate) => candidate,
                    None => {
                        let mut candidate = editor.value.clone();
                        if !time_travel_pointer_set(&mut candidate, path, coerced) {
                            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput));
                        }
                        candidate
                    }
                };
                (path, candidate)
            }
        };
        let verdict = mutation_input_instance(schema, &registered_input_schema_document, &candidate)
            .map_err(|error| error.to_string())
            .and_then(|instance| editor.validator.as_ref().map_err(String::clone).and_then(|validator| validator.validate_json(&time_travel_json(&instance)).map(|_| ()).map_err(|error| error.to_string())));
        let rebuilt = match verdict {
            Ok(()) => match self.time_travel_run(TimeTravelStoreCommand::Rebuild(candidate.clone()),None).await? {
                TimeTravelStoreOutput::Rebuilt(rebuilt) => rebuilt,
                _ => Err("a history-edit rebuild answered another command".to_string()),
            },
            Err(reason) => Err(reason),
        };
        let replacement = match rebuilt {
            Ok(replacement) => replacement,
            Err(reason) => {
                let Some(editor) = self.time_travel.editor.as_mut() else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::EditorClosed)) };
                let outcome = refuse(editor, at, reason);
                self.time_travel.ui_dirty = true;
                return Ok(outcome);
            }
        };
        let outcome = self.apply_time_travel_event(TimeTravelEvent::Draft { generation, replacement }, Some(meta), effects, identity).await?;
        if matches!(outcome, TimeTravelActionOutcome::Applied(_)) {
            if let Some(editor) = self.time_travel.editor.as_mut() {
                editor.value = candidate;
                editor.refused = None;
            }
        }
        Ok(outcome)
    }

    /// 🎯️ Drafts the reference input at `path` from the current selection of its declared domain, every row mapped to the
    /// entity id it names (`ArtifactApp::selection_reference_id`) ([`time_travel_selection_value`]), then validates and
    /// rebuilds it like any other input.
    pub(super) async fn draft_time_travel_selection(&mut self, path: Option<&str>, generation: u32, meta: &ActionMeta, effects: &mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TimeTravelActionOutcome, Fault> {
        let Some(editor) = self.time_travel.editor.as_ref() else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal))) };
        let Some((input, false)) = path.and_then(|path| time_travel_input_at(&editor.inputs, &editor.value, path)) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
        let ArgSchema::Reference { domain: Some(domain), kinds, .. } = &input.schema else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
        let state = self.interaction.interaction_selection_snapshot(&*self.stores);
        let selection = state.selection.get(domain).map(|selection| protocol::DomainSelection { ids: selection.ids.iter().filter_map(|row| A::selection_reference_id(kinds, row)).collect(), ..selection.clone() });
        match time_travel_selection_value(&input, selection.as_ref()) {
            Ok(value) => self.draft_time_travel_input(path, Some(value), None, generation, meta, effects, identity).await,
            Err(refusal) => Ok(TimeTravelActionOutcome::Rejected(refusal)),
        }
    }

}
