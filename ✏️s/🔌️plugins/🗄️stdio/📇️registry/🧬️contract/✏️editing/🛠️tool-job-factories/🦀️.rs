//! 🛠️ The bounded native and snapshot edit tool job factories, admission and first-step proofs.

use super::*;

pub struct BoundedNativeEditToolJobFactory<E: BoundedNativeEditingEditor> {
    keys: Vec<ToolFactoryKey>,
    marker: std::marker::PhantomData<fn() -> E>,
}

impl<E: BoundedNativeEditingEditor> BoundedNativeEditToolJobFactory<E> {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: E::NATIVE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect(), marker: std::marker::PhantomData }
    }
}

impl<E: BoundedNativeEditingEditor> ToolJobFactory for BoundedNativeEditToolJobFactory<E> {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<E>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<E>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        E::NATIVE_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        E::native_edit_execution_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > E::NATIVE_MAXIMUM_RAW_BYTES {
            return Err((ToolJobFactoryError::new("bounded native edit rejects oversized wire"), input, checkpoint));
        }
        match checkpoint {
            Some(checkpoint) if E::NATIVE_CHECKPOINT_RESUME => Ok(ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint)),
            Some(checkpoint) => Err((ToolJobFactoryError::new("bounded native edit does not declare checkpoint resumption"), input, Some(checkpoint))),
            None => Ok(ArtifactRetainedCommandJob::from_wire(payload, input)),
        }
    }
}

impl<E: BoundedNativeEditingEditor> ArtifactOwnedToolJobFactory for BoundedNativeEditToolJobFactory<E> {
    type Owner = EditorApp<E>;
    const TOOL_IDS: &'static [&'static str] = E::NATIVE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = E::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = E::NATIVE_PUBLICATION_CONTRACTS;
}

pub fn register_bounded_native_edit_tool_factory<E: BoundedNativeEditingEditor>(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<E>>) -> Result<(), Fault> {
    let controller = registry.controller_id().to_string();
    registry.register(BoundedNativeEditToolJobFactory::<E>::new(&controller))
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
pub(super) fn bounded_native_edit_reduce<E: BoundedNativeEditingEditor>(
    command: &E::Command,
    snapshot: &E::Snapshot,
    _config: &E::Config,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &kernel::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<E>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<E::Mutation, E::ConfigMutation, E::DraftMutation>, Fault> {
    if !E::NATIVE_TOOL_IDS.contains(&E::command_id(command)) {
        return Err(edit_fault("bounded-native-edit.command-mismatch", "bounded native edit factory received a command for another tool"));
    }
    E::native_edit_mutations(command, snapshot)
}

pub fn admit_bounded_native_command<C: OpBinary>(command: &C, maximum_bytes: usize) -> Result<usize, Fault> {
    let encoded = command.encode_op().map_err(|error| edit_fault("bounded-native-edit.command-encoding", error.to_string()))?;
    if encoded.len() > maximum_bytes {
        return Err(edit_fault("bounded-native-edit.command-too-large", format!("bounded native edit command is {} bytes; maximum is {maximum_bytes}", encoded.len())));
    }
    Ok(encoded.len())
}

pub fn build_bounded_native_edit_tool_job<E: BoundedNativeEditingEditor>(request: ArtifactOwnedToolJobRequest<EditorApp<E>>) -> Result<Option<ToolOperationSpec>, Fault> {
    if !E::NATIVE_TOOL_IDS.contains(&request.tool_id.as_str()) {
        return Ok(None);
    }
    if E::command_id(&request.command) != request.tool_id {
        return Err(edit_fault("app.command.tool-mismatch", "bounded native edit command does not match its addressed tool"));
    }
    admit_bounded_native_command(request.command.as_ref(), E::NATIVE_MAXIMUM_RAW_BYTES)?;
    let tool_id = E::command_id(&request.command);
    let operation = AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id,
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
        retained: request.retained,
        authoring_seed: request.authoring_seed.clone(),
    };
    let payload = ArtifactRetainedCommandPayload::new(
        ArtifactRetainedCommandInputs {
            command: *request.command,
            snapshot: request.snapshot,
            snapshot_read: request.snapshot_read,
            config: request.config,
            history: request.history,
            interaction_state: request.interaction_state,
            interaction_hover: request.interaction_hover,
            context: Some(request.context),
            operation,
            completion: request.completion,
        },
        E::command_id,
        E::NATIVE_MAXIMUM_RAW_BYTES,
        E::NATIVE_MAXIMUM_WORK_ITEMS,
        E::native_edit_work(tool_id),
    );
    Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}

pub fn bounded_native_edit_bounded_first_step_proofs<E: BoundedNativeEditingEditor>(owner_file: &'static str, controller_id: &'static str, artifact_schema: &'static str) -> Vec<ArtifactBoundedFirstStepProof> {
    let factory_type = std::any::type_name::<BoundedNativeEditToolJobFactory<E>>();
    let factory = factory_type.rsplit("::").next().unwrap_or(factory_type);
    E::NATIVE_TOOL_IDS
        .iter()
        .map(|tool_id| ArtifactBoundedFirstStepProof::new::<EditorApp<E>>(owner_file, controller_id, factory, tool_id, artifact_schema, E::native_edit_execution_contract()).with_factory_type::<EditorApp<E>, BoundedNativeEditToolJobFactory<E>>())
        .collect()
}

pub fn snapshot_edit_value_is_admitted<S: ToValue>(event: &SnapshotEditEvent, snapshot: &S) -> bool {
    fn value_is_admitted(value: &DslValue, remaining: &mut usize, depth: usize) -> bool {
        if *remaining == 0 || depth > 128 {
            return false;
        }
        *remaining -= 1;
        match value {
            DslValue::Number(semio_framework_value::Number::Float(value)) => value.is_finite(),
            DslValue::Array(items) => items.iter().all(|item| value_is_admitted(item, remaining, depth + 1)),
            DslValue::Object(entries) => {
                let mut keys = BTreeSet::new();
                entries.iter().all(|(key, item)| keys.insert(key.as_str()) && value_is_admitted(item, remaining, depth + 1))
            }
            _ => true,
        }
    }
    fn path(path: &str) -> Option<Vec<String>> {
        if path.len() > SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES {
            return None;
        }
        let segments = decode_pointer(path).ok()?;
        (segments.len() <= 128).then_some(segments)
    }
    fn shape<S: ToValue>(snapshot: &S, path: &[String]) -> Option<semio_framework_value::ValueShape> {
        let segments = path.iter().map(String::as_str).collect::<Vec<_>>();
        snapshot.value_shape_at_path(&segments).ok()
    }
    fn insertion_parent_is_admitted<S: ToValue>(snapshot: &S, path: &[String], pointer: &str) -> bool {
        let Some((last, parent)) = path.split_last() else { return false };
        match shape(snapshot, parent) {
            Some(semio_framework_value::ValueShape::Array { len }) => array_index(last, len, pointer, true).is_ok(),
            Some(semio_framework_value::ValueShape::Object { .. }) => true,
            _ => false,
        }
    }
    if <SnapshotEditEvent as OpBinary>::encode_op(event).map_or(true, |bytes| bytes.len() > SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES) {
        return false;
    }
    let mut remaining = SNAPSHOT_EDIT_MAXIMUM_VALUE_NODES;
    match event {
        SnapshotEditEvent::SetValue { path: pointer, value } => {
            let Some(path) = path(pointer) else { return false };
            value_is_admitted(value, &mut remaining, 0) && (path.is_empty() || shape(snapshot, &path).is_some())
        }
        SnapshotEditEvent::InsertValue { path: pointer, value } => {
            let Some(path) = path(pointer) else { return false };
            value_is_admitted(value, &mut remaining, 0) && insertion_parent_is_admitted(snapshot, &path, pointer)
        }
        SnapshotEditEvent::RemoveValue { path: pointer } => {
            let Some(path) = path(pointer) else { return false };
            !path.is_empty() && shape(snapshot, &path).is_some()
        }
        SnapshotEditEvent::MoveValue { from, path: pointer } => {
            let (Some(from_path), Some(path)) = (path(from), path(pointer)) else { return false };
            !from_path.is_empty() && !path.is_empty() && !(path.len() > from_path.len() && path.starts_with(&from_path)) && shape(snapshot, &from_path).is_some()
        }
        SnapshotEditEvent::RenameKey { path: pointer, key } => {
            if key.len() > SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES {
                return false;
            }
            let Some(path) = path(pointer) else { return false };
            let Some((_, parent)) = path.split_last() else { return false };
            matches!(shape(snapshot, parent), Some(semio_framework_value::ValueShape::Object { .. })) && shape(snapshot, &path).is_some()
        }
        SnapshotEditEvent::ReplaceSource { source } => source.len() <= SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES,
    }
}

pub fn snapshot_edit_execution_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES, 4_096, 1, SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES, 7_500)
}

pub struct SnapshotEditToolJobFactory<E: SnapshotEditingEditor> {
    keys: Vec<ToolFactoryKey>,
    marker: std::marker::PhantomData<fn() -> E>,
}

impl<E: SnapshotEditingEditor> SnapshotEditToolJobFactory<E> {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: SNAPSHOT_EDIT_ACTION_IDS.iter().map(|tool| ToolFactoryKey::new(controller_id, *tool)).collect(), marker: std::marker::PhantomData }
    }
}

impl<E: SnapshotEditingEditor> ToolJobFactory for SnapshotEditToolJobFactory<E> {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<E>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<E>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SNAPSHOT_EDIT_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        snapshot_edit_execution_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("snapshot edit rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl<E: SnapshotEditingEditor> ArtifactOwnedToolJobFactory for SnapshotEditToolJobFactory<E> {
    type Owner = EditorApp<E>;
    const TOOL_IDS: &'static [&'static str] = SNAPSHOT_EDIT_ACTION_IDS;
    const DOCUMENT_SCHEMA: &'static str = E::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = SNAPSHOT_EDIT_PUBLICATION_CONTRACTS;
}

pub fn register_snapshot_edit_tool_factory<E: SnapshotEditingEditor>(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<E>>) -> Result<(), Fault> {
    let controller = registry.controller_id().to_string();
    registry.register(SnapshotEditToolJobFactory::<E>::new(&controller))
}

fn snapshot_edit_command_id<E: SnapshotEditingEditor>(command: &E::Command) -> &'static str {
    E::snapshot_edit_event(command).map_or("unknownSnapshotEdit", SnapshotEditEvent::action_id)
}

fn snapshot_edit_extent<E: SnapshotEditingEditor>(command: &E::Command, _snapshot: &E::Snapshot, _interaction: &kernel::InteractionState) -> Option<usize> {
    let event = E::snapshot_edit_event(command)?;
    E::snapshot_edit_is_admitted(event, _snapshot).then(|| SNAPSHOT_EDIT_WORK_CAPACITY.rows_for_items(1)).flatten()
}

/// 📏️ Refuses a resolved edit whose mutation or exact undo cannot be carried as one native publication item.
pub fn check_publication_limits<S, M: Mutation<S> + OpBinary>(snapshot: &S, mutations: &[M]) -> Result<(), Fault> {
    mutations.iter().try_for_each(|mutation| {
        let inverses = mutation.inverse(snapshot).map_err(|error| edit_fault("snapshot-edit.inverse-refused", error.into_message()))?;
        std::iter::once(mutation).chain(inverses.iter()).try_for_each(|operation| {
            let bytes = operation.encode_op().map_err(|error| edit_fault("snapshot-edit.publication-codec", error.to_string()))?;
            (bytes.len() <= kernel::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).then_some(()).ok_or_else(|| edit_fault("snapshot-edit.publication-limit", "the edit or its exact undo exceeds the native publication item limit"))
        })
    })
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
fn snapshot_edit_reduce<E: SnapshotEditingEditor>(
    command: &E::Command,
    snapshot: &E::Snapshot,
    _config: &E::Config,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &kernel::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<E>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<E::Mutation, E::ConfigMutation, E::DraftMutation>, Fault> {
    let event = E::snapshot_edit_event(command).ok_or_else(|| edit_fault("snapshot-edit.command-mismatch", "snapshot edit factory received a native command"))?;
    E::snapshot_edit_emit(event, snapshot)
}

pub fn build_snapshot_edit_tool_job<E: SnapshotEditingEditor>(request: ArtifactOwnedToolJobRequest<EditorApp<E>>) -> Result<Option<ToolOperationSpec>, Fault> {
    if !SNAPSHOT_EDIT_ACTION_IDS.contains(&request.tool_id.as_str()) {
        return Ok(None);
    }
    let Some(event) = E::snapshot_edit_event(&request.command) else {
        return Err(edit_fault("snapshot-edit.command-mismatch", "snapshot edit tool received a native command"));
    };
    if event.action_id() != request.tool_id {
        return Err(edit_fault("app.command.tool-mismatch", "snapshot edit command does not match its addressed tool"));
    }
    let tool_id = event.action_id();
    let operation = AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id,
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
        retained: request.retained,
        authoring_seed: request.authoring_seed.clone(),
    };
    let payload = ArtifactRetainedCommandPayload::new(
        ArtifactRetainedCommandInputs {
            command: *request.command,
            snapshot: request.snapshot,
            snapshot_read: request.snapshot_read,
            config: request.config,
            history: request.history,
            interaction_state: request.interaction_state,
            interaction_hover: request.interaction_hover,
            context: Some(request.context),
            operation,
            completion: request.completion,
        },
        snapshot_edit_command_id::<E>,
        SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES,
        SNAPSHOT_EDIT_WORK_CAPACITY.work_items(),
        Box::new(BoundedArtifactCommandWork::new(tool_id, snapshot_edit_reduce::<E>, snapshot_edit_extent::<E>)),
    );
    Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}

pub fn snapshot_edit_bounded_first_step_proofs<E: SnapshotEditingEditor>(owner_file: &'static str, controller_id: &'static str, artifact_schema: &'static str) -> Vec<ArtifactBoundedFirstStepProof> {
    let factory_type = std::any::type_name::<SnapshotEditToolJobFactory<E>>();
    let factory = factory_type.rsplit("::").next().unwrap_or(factory_type);
    SNAPSHOT_EDIT_ACTION_IDS
        .iter()
        .map(|tool| ArtifactBoundedFirstStepProof::new::<EditorApp<E>>(owner_file, controller_id, factory, tool, artifact_schema, snapshot_edit_execution_contract()).with_factory_type::<EditorApp<E>, SnapshotEditToolJobFactory<E>>())
        .collect()
}
