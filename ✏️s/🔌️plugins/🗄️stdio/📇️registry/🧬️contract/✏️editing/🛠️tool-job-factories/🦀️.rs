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

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: E::NATIVE_MAXIMUM_RAW_BYTES, checkpoint_resume: E::NATIVE_CHECKPOINT_RESUME);
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

pub fn build_bounded_native_edit_tool_job<E: BoundedNativeEditingEditor>(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<E>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
    let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
    if !E::NATIVE_TOOL_IDS.contains(&original.tool_id.as_str()) {
        return Ok(false);
    }
    if E::command_id(&original.command) != original.tool_id {
        return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "bounded native edit command does not match its addressed tool").into());
    }
    let tool_id = E::command_id(&original.command);
    semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, E::command_id, E::NATIVE_MAXIMUM_RAW_BYTES, E::NATIVE_MAXIMUM_WORK_ITEMS, E::NATIVE_CHECKPOINT_RESUME, |_request| E::native_edit_work(tool_id))
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
    if <SnapshotEditEvent as OpBinary>::encode_op(event).map_or(true, |bytes| bytes.len() > SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES) {
        return false;
    }
    let mut remaining = SNAPSHOT_EDIT_MAXIMUM_VALUE_NODES;
    if matches!(event, SnapshotEditEvent::SetValue {value,..}|SnapshotEditEvent::InsertValue {value,..} if !value_is_admitted(value,&mut remaining,0)){return false;}
    snapshot_edit_path_is_admitted(event,snapshot)
}

/// 🧭️ Borrow only snapshot shape after a retained worker has admitted the original event payload.
pub fn snapshot_edit_path_is_admitted<S:ToValue>(event:&SnapshotEditEvent,snapshot:&S)->bool {
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
    match event {
        SnapshotEditEvent::SetValue { path: pointer, .. } => {
            let Some(path) = path(pointer) else { return false };
            path.is_empty() || shape(snapshot, &path).is_some()
        }
        SnapshotEditEvent::InsertValue { path: pointer, .. } => {
            let Some(path) = path(pointer) else { return false };
            insertion_parent_is_admitted(snapshot, &path, pointer)
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

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES, checkpoint_resume: false);
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
    E::snapshot_edit_retained_extent(event,_snapshot)
}

/// 📏️ Refuses a resolved edit whose mutation or exact undo cannot be carried as one native publication item.
pub fn check_publication_limits<S, M: Mutation<S> + kernel::ColdMutation<S> + OpBinary>(snapshot: &S, mutations: &[M]) -> Result<(), Fault> {
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

pub(super) fn snapshot_edit_contextual_once<T>(consumed:&mut bool,cx:&mut semio_framework_job::StepContext<'_>,reduce:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<T,Fault>)->Result<T,Fault>{
    if *consumed{return Err(Fault::from("retained-command-bounded-work-repeated"));}
    *consumed=true;reduce(cx)
}

pub(crate) struct SnapshotEditWork<E:SnapshotEditingEditor>{tool_id:&'static str,consumed:bool,marker:std::marker::PhantomData<fn()->E>}
impl<E:SnapshotEditingEditor> SnapshotEditWork<E>{pub(crate) fn new(tool_id:&'static str)->Self{Self{tool_id,consumed:false,marker:std::marker::PhantomData}}}
impl<E:SnapshotEditingEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<E>> for SnapshotEditWork<E>{
    fn tool_id(&self)->&'static str{self.tool_id}
    fn extent(&self,command:&E::Command,snapshot:&E::Snapshot,interaction:&kernel::InteractionState,_context:Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<E>>>)->Option<usize>{snapshot_edit_extent::<E>(command,snapshot,interaction)}
    fn work_demands(&self,_input:&semio_framework_plugin::retained_command::ArtifactCommandInputs<'_,EditorApp<E>>,_maximum_copy_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}
    fn terminal_frame_release_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self>())}
    fn reports_progress(&self)->bool{false}
    fn step(&mut self,input:&semio_framework_plugin::retained_command::ArtifactCommandInputs<'_,EditorApp<E>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<E>>,semio_framework_job::OriginalTurnFailure>{
        snapshot_edit_contextual_once(&mut self.consumed,cx,|cx|{
            let event=E::snapshot_edit_event(input.command).ok_or_else(||edit_fault("snapshot-edit.command-mismatch","snapshot edit factory received a native command"))?;
            E::snapshot_edit_emit_in_context(event,input.snapshot,cx).map(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete)
        }).map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

pub fn build_snapshot_edit_tool_job<E: SnapshotEditingEditor>(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<E>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
    let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
    if !SNAPSHOT_EDIT_ACTION_IDS.contains(&original.tool_id.as_str()) {
        return Ok(false);
    }
    let Some(event) = E::snapshot_edit_event(&original.command) else {
        return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "snapshot edit tool received a native command").into());
    };
    if event.action_id() != original.tool_id {
        return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "snapshot edit command does not match its addressed tool").into());
    }
    let tool_id = event.action_id();
    semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, snapshot_edit_command_id::<E>, SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES, SNAPSHOT_EDIT_WORK_CAPACITY.work_items(), false, |request| BoundedArtifactCommandWork::new(tool_id, snapshot_edit_reduce::<E>, snapshot_edit_extent::<E>))
}

pub fn snapshot_edit_bounded_first_step_proofs<E: SnapshotEditingEditor>(owner_file: &'static str, controller_id: &'static str, artifact_schema: &'static str) -> Vec<ArtifactBoundedFirstStepProof> {
    let factory_type = std::any::type_name::<SnapshotEditToolJobFactory<E>>();
    let factory = factory_type.rsplit("::").next().unwrap_or(factory_type);
    SNAPSHOT_EDIT_ACTION_IDS
        .iter()
        .map(|tool| ArtifactBoundedFirstStepProof::new::<EditorApp<E>>(owner_file, controller_id, factory, tool, artifact_schema, snapshot_edit_execution_contract()).with_factory_type::<EditorApp<E>, SnapshotEditToolJobFactory<E>>())
        .collect()
}
