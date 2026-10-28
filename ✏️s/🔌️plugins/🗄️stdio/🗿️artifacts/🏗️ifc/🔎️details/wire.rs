//! 🧵️ Borrowed forward and inverse Store wire admission advances the original native codec.
use super::*;
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText as Text, ArtifactCanonicalJsonTree as Tree};
use semio_framework_plugin::plugin_app_close_prelude::store;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneSource, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::sync::Arc;
#[path = "inverse.rs"]
mod inverse;
#[path = "preimage.rs"]
mod preimage;

pub type SnapshotSource = RetainedCloneSource<IfcSnapshot, Arc<IfcSnapshot>>;

#[derive(Clone, Copy, Default)]
enum Phase {
    #[default]
    Preflight,
    Start,
    Prepare,
    Forward,
    CloseForward,
    RestartForward,
    Inverse,
    CloseInverse,
    RestartInverse,
    Done,
}

#[derive(Clone, Copy, Default)]
enum Inverse {
    #[default]
    None,
    Description,
    FileName,
    Schema,
    Remove,
    Insert,
    Name,
    Argument,
    InsertArgument,
    RemoveArgument,
    Complex,
}

#[derive(Default)]
pub struct IfcInverseWire {
    pub source: Option<SnapshotSource>,
    inverse: Inverse,
    id: u64,
    index: usize,
    position: usize,
    overlay: preimage::Overlay,
}

pub struct IfcPublication {
    pub view: IfcInverseWire,
    codec: store::ArtifactPreparedOperationCursor,
    chunk: [u8; 64],
    live: bool,
    phase: Phase,
    mutation: usize,
    bytes: usize,
    rows: usize,
    preflight_mutation: usize,
    mutation_source: Option<(usize, usize)>,
    #[cfg(test)]
    cancel_at: Option<usize>,
    #[cfg(test)]
    turns: usize,
}

impl Default for IfcPublication {
    fn default() -> Self {
        Self {
            view: Default::default(),
            codec: Default::default(),
            chunk: [0; 64],
            live: false,
            phase: Phase::Preflight,
            mutation: 0,
            bytes: 0,
            rows: 0,
            preflight_mutation: 0,
            mutation_source: None,
            #[cfg(test)]
            cancel_at: None,
            #[cfg(test)]
            turns: 0,
        }
    }
}

fn refusal(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}

fn receipt(progress: store::ArtifactPreparedOperationProgress) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: progress.processed_items, copied_bytes: progress.copied_bytes, retained_capacity_bytes: progress.retained_capacity_bytes, released_bytes: progress.released_bytes }
}

fn entity_id(mutation: &IfcMutation) -> Option<u64> {
    match mutation {
        IfcMutation::InsertEntity(value) => Some(value.entity.id),
        IfcMutation::RemoveEntity(value) => Some(value.id),
        IfcMutation::SetEntityName(value) => Some(value.id),
        IfcMutation::SetEntityArg(value) => Some(value.id),
        IfcMutation::InsertEntityArg(value) => Some(value.id),
        IfcMutation::RemoveEntityArg(value) => Some(value.id),
        IfcMutation::SetEntityComplex(value) => Some(value.id),
        _ => None,
    }
}

impl IfcInverseWire {
    fn snapshot(&self) -> Result<&IfcSnapshot, ValueError> {
        self.source.as_ref().ok_or_else(|| refusal("IFC publication source missing"))?.try_authority().map(Arc::as_ref)
    }

    fn tag(&self) -> &'static str {
        match self.inverse {
            Inverse::Description => "setFileDescription",
            Inverse::FileName => "setFileName",
            Inverse::Schema => "setFileSchema",
            Inverse::Remove => "removeEntity",
            Inverse::Insert => "insertEntity",
            Inverse::Name => "setEntityName",
            Inverse::Argument => "setEntityArg",
            Inverse::InsertArgument => "insertEntityArg",
            Inverse::RemoveArgument => "removeEntityArg",
            Inverse::Complex => "setEntityComplex",
            Inverse::None => "",
        }
    }

    fn fields(&self) -> &'static [&'static str] {
        match self.inverse {
            Inverse::Description | Inverse::FileName | Inverse::Schema => &["values"],
            Inverse::Remove => &["id"],
            Inverse::Insert => &["index", "entity"],
            Inverse::Name => &["id", "name"],
            Inverse::Argument | Inverse::InsertArgument => &["id", "index", "value"],
            Inverse::RemoveArgument => &["id", "index"],
            Inverse::Complex => &["id", "complex"],
            Inverse::None => &[],
        }
    }

    fn child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        let snapshot = self.snapshot()?;
        let entity = || snapshot.entities.get(self.position).ok_or_else(|| refusal("IFC inverse entity missing"));
        match (self.inverse, ordinal) {
            (Inverse::Description, 0) => Ok(&snapshot.header.file_description),
            (Inverse::FileName, 0) => Ok(&snapshot.header.file_name),
            (Inverse::Schema, 0) => Ok(&snapshot.header.file_schema),
            (Inverse::Insert, 0) => Ok(&self.position),
            (Inverse::Insert, 1) => Ok(entity()?),
            (Inverse::Remove | Inverse::Name | Inverse::Argument | Inverse::InsertArgument | Inverse::RemoveArgument | Inverse::Complex, 0) => Ok(&self.id),
            (Inverse::Name, 1) => Ok(&entity()?.name),
            (Inverse::Argument | Inverse::InsertArgument | Inverse::RemoveArgument, 1) => Ok(&self.index),
            (Inverse::Argument | Inverse::InsertArgument, 2) => entity()?.args.get(self.index).map(|value| value as &dyn Tree).ok_or_else(|| refusal("IFC inverse argument missing")),
            (Inverse::Complex, 1) => Ok(&entity()?.complex),
            _ => Err(refusal("IFC inverse field missing")),
        }
    }

    fn descendant(&self, path: &[usize]) -> Result<&dyn Tree, ValueError> {
        let (first, rest) = path.split_first().ok_or_else(|| refusal("IFC inverse child path missing"))?;
        let mut tree = self.child(first.checked_sub(1).ok_or_else(|| refusal("IFC inverse tag has no children"))?)?;
        for ordinal in rest {
            tree = tree.canonical_tree_child(*ordinal)?;
        }
        Ok(tree)
    }
}

impl IfcPublication {
    fn begin_inverse(&mut self, mutation: &IfcMutation) {
        let view = &mut self.view;
        view.position = 0;
        view.index = 0;
        view.id = 0;
        view.inverse = match mutation {
            IfcMutation::SetFileDescription(_) => Inverse::Description,
            IfcMutation::SetFileName(_) => Inverse::FileName,
            IfcMutation::SetFileSchema(_) => Inverse::Schema,
            IfcMutation::InsertEntity(value) => {
                view.id = value.entity.id;
                Inverse::Remove
            }
            IfcMutation::RemoveEntity(value) => {
                view.id = value.id;
                Inverse::Insert
            }
            IfcMutation::SetEntityName(value) => {
                view.id = value.id;
                Inverse::Name
            }
            IfcMutation::SetEntityArg(value) => {
                view.id = value.id;
                view.index = value.index;
                Inverse::Argument
            }
            IfcMutation::RemoveEntityArg(value) => {
                view.id = value.id;
                view.index = value.index;
                Inverse::InsertArgument
            }
            IfcMutation::InsertEntityArg(value) => {
                view.id = value.id;
                view.index = value.index;
                Inverse::RemoveArgument
            }
            IfcMutation::SetEntityComplex(value) => {
                view.id = value.id;
                Inverse::Complex
            }
        };
        view.overlay.begin(self.mutation);
        self.phase = Phase::Prepare;
    }

    fn counted(&mut self, count: usize) -> Result<(), ValueError> {
        self.bytes = self.bytes.checked_add(count).ok_or_else(|| refusal("IFC operation wire size overflow"))?;
        if self.bytes > protocol::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES {
            return Err(refusal("IFC edit or its exact inverse exceeds the native publication item limit"));
        }
        Ok(())
    }

    pub fn demands(&self) -> Result<RetirementDemand, ValueError> {
        match self.phase {
            Phase::Preflight if self.mutation_source.is_none() => Ok(RetirementDemand { copy_bytes: std::mem::size_of_val(&self.mutation_source), depth: 1, ..Default::default() }),
            Phase::Prepare => self.view.overlay.demands(self.mutation_source.map_or(0, |source| source.1)),
            Phase::Forward | Phase::Inverse => self.codec.canonical_json_work_demands(),
            Phase::CloseForward | Phase::CloseInverse => self.codec.retirement_demands(),
            Phase::RestartForward | Phase::RestartInverse => Ok(self.codec.canonical_json_restart_demands()),
            _ => Ok(RetirementDemand { depth: 1, ..Default::default() }),
        }
    }

    pub fn advance(&mut self, mutations: &[IfcMutation], capacity: semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if self.mutation_source.is_some_and(|source| source != (mutations.as_ptr() as usize, mutations.len())) {
            return Err(refusal("IFC original mutation source changed during retained publication").into());
        }
        for _ in 0..64 {
            #[cfg(test)]
            {
                self.turns += 1;
                if self.cancel_at == Some(self.turns) {
                    cx.original_cancel_token().cancel_now();
                }
            }
            if cx.is_cancelled() {
                return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC publication validation cancelled").into());
            }
            let demand = self.demands()?;
            let grant = cx.retained_grant();
            if cx.should_yield()
                || grant.maximum_items == 0
                || demand.copy_bytes > grant.maximum_copy_bytes
                || demand.capacity_bytes > grant.maximum_capacity_bytes
                || demand.release_bytes > grant.maximum_release_bytes
                || demand.depth > grant.maximum_depth
            {
                return Ok(false);
            }
            if self.advance_one(mutations, capacity, cx)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn advance_one(&mut self, mutations: &[IfcMutation], capacity: semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC publication validation cancelled").into());
        }
        if cx.should_yield() {
            return Ok(false);
        }
        match self.phase {
            Phase::Preflight => {
                let identity = (mutations.as_ptr() as usize, mutations.len());
                if self.mutation_source.is_some_and(|source| source != identity) {
                    return Err(refusal("IFC original mutation source changed during preflight").into());
                }
                if self.mutation_source.is_none() {
                    self.mutation_source = Some(identity);
                    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of_val(&self.mutation_source), ..Default::default() })?;
                    cx.consume_fuel(1);
                    return Ok(false);
                }
                if mutations.len() > capacity.work_items() {
                    return Err(refusal("IFC edit exceeds its declared publication row capacity").into());
                }
                if self.preflight_mutation == mutations.len() {
                    self.phase = Phase::Start;
                } else {
                    self.rows = self.rows.checked_add(1).and_then(|rows| rows.checked_add(protocol::Mutation::inverse_rows(&mutations[self.preflight_mutation]))).ok_or_else(|| refusal("IFC publication row count overflow"))?;
                    if !capacity.admits(self.rows) {
                        return Err(refusal("IFC edit exceeds its declared publication row capacity").into());
                    }
                    self.preflight_mutation += 1;
                }
            }
            Phase::Done => return Ok(true),
            Phase::Start => {
                if self.mutation_source != Some((mutations.as_ptr() as usize, mutations.len())) {
                    return Err(refusal("IFC original mutation source changed before wire encoding").into());
                }
                let Some(mutation) = mutations.get(self.mutation) else {
                    self.phase = Phase::Done;
                    return Ok(true);
                };
                self.begin_inverse(mutation);
            }
            Phase::Prepare => {
                let source = self.view.source.as_ref().ok_or_else(|| refusal("IFC publication source missing"))?;
                let snapshot = source.try_authority()?.as_ref();
                if self.view.overlay.advance(snapshot, mutations, cx)? {
                    self.view.position = self.view.overlay.before.position;
                    if matches!(self.view.inverse, Inverse::Insert | Inverse::Name | Inverse::Complex) && self.view.overlay.before.origin.is_none()
                        || matches!(self.view.inverse, Inverse::Argument | Inverse::InsertArgument) && self.view.overlay.argument.is_none()
                    {
                        self.view.inverse = Inverse::None;
                    }
                    self.live = true;
                    self.bytes = 0;
                    self.phase = Phase::Forward;
                }
            }
            Phase::Forward | Phase::Inverse => {
                if matches!(self.phase, Phase::Inverse) && !self.live {
                    self.live = true;
                    self.bytes = 0;
                }
                let grant = cx.retained_grant();
                let result = if matches!(self.phase, Phase::Forward) {
                    let source = store::ArtifactPreparedOperationSource::CanonicalJson { header: b"op", body: store::ArtifactCanonicalTreeSource::view(&mutations[self.mutation]) };
                    self.codec.advance(source, &mut self.chunk, grant)
                } else {
                    let inverse = inverse::BorrowedInverse { view: &self.view, mutations };
                    self.codec.advance(store::ArtifactPreparedOperationSource::CanonicalJson { header: b"op", body: &inverse }, &mut self.chunk, grant)
                };
                match result {
                    Ok(progress) => {
                        let complete = progress.complete;
                        let written = progress.written_bytes;
                        cx.consume_retained(receipt(progress))?;
                        self.counted(written)?;
                        if complete {
                            self.phase = if matches!(self.phase, Phase::Forward) { Phase::CloseForward } else { Phase::CloseInverse };
                        }
                    }
                    Err(error) => {
                        cx.consume_retained(error.reason.retained_progress())?;
                        return Err(error.reason.into());
                    }
                }
            }
            Phase::CloseForward | Phase::CloseInverse => {
                let forward = matches!(self.phase, Phase::CloseForward);
                let progress = match self.codec.close(cx.retained_grant()) {
                    Ok(progress) => progress,
                    Err(error) => {
                        return Err(semio_framework_job::OriginalTurnFailure::debit_original(error.reason, cx));
                    }
                };
                let complete = progress.complete;
                cx.consume_retained(receipt(progress))?;
                if complete {
                    self.live = false;
                    self.phase = if forward { Phase::RestartForward } else { Phase::RestartInverse };
                }
            }
            Phase::RestartForward | Phase::RestartInverse => {
                let forward = matches!(self.phase, Phase::RestartForward);
                let Some(progress) = self.codec.restart_canonical_json(cx.retained_grant())? else { return Ok(false) };
                cx.consume_retained(progress)?;
                if forward {
                    if matches!(self.view.inverse, Inverse::None) {
                        self.mutation += 1;
                        self.phase = Phase::Start;
                    } else {
                        self.phase = Phase::Inverse;
                    }
                } else {
                    self.mutation += 1;
                    self.phase = Phase::Start;
                }
            }
        }
        cx.consume_fuel(1);
        Ok(false)
    }

    pub fn codec_live(&self) -> bool {
        !self.codec.closed || self.view.overlay.has_metadata()
    }
    #[cfg(test)]
    pub fn cancel_at_turn(&mut self, turn: usize) {
        self.cancel_at = Some(turn);
    }
    #[cfg(test)]
    pub fn turns(&self) -> usize {
        self.turns
    }
    #[cfg(test)]
    pub fn inverse_wire<'a>(&'a self, mutations: &'a [IfcMutation]) -> impl store::ArtifactCanonicalJson + 'a {
        inverse::BorrowedInverse { view: &self.view, mutations }
    }
    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.codec.closed { self.codec.retirement_demands() } else { self.view.overlay.close_demands() }
    }
    pub fn close_codec(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.codec.closed {
            return self.view.overlay.close(grant);
        }
        let progress = self.codec.close(grant).map_err(|error| error.reason)?;
        let complete = progress.complete;
        let progress = receipt(progress);
        if complete {
            self.live = false;
            Ok(if self.view.overlay.has_metadata() { RetainedCloneStep::Progress(progress) } else { RetainedCloneStep::Complete(progress) })
        } else {
            Ok(RetainedCloneStep::Progress(progress))
        }
    }
}

impl store::ArtifactCanonicalJson for IfcInverseWire {
    fn canonical_json_node(&self, path: &[usize]) -> Result<Node<'_>, ValueError> {
        match path {
            [] => Ok(Node::Object(self.fields().len() + 1)),
            [0] => Ok(Node::String(self.tag())),
            _ => self.descendant(path)?.canonical_tree_node(),
        }
    }
    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<Text<'_>, ValueError> {
        if path.is_empty() {
            if index == 0 {
                return Ok("mutation".into());
            }
            return self.fields().get(index - 1).map(|key| Text::from(*key)).ok_or_else(|| refusal("IFC inverse key missing"));
        }
        self.descendant(path)?.canonical_tree_key(index)
    }
}

#[cfg(test)]
#[path = "progressive_tests.rs"]
mod progressive_tests;
#[cfg(test)]
mod preimage_tests {
    use super::*;
    #[test]
    fn retained_preflight_counts_progressive_batches_before_any_wire_bytes() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/preimages.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let parsed = semio_framework_pack_json::parse(&case["mutations"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            let mutations = Vec::<IfcMutation>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).unwrap();
            let mut caller = semio_framework_plugin::app_testing::OriginalWorkFixture::with_identity(semio_framework_job::OperationId(891), semio_framework_job::Generation(1));
            let mut cursor = IfcPublication::default();
            let mut refused = false;
            for _ in 0..10_000 {
                if matches!(cursor.phase, Phase::Start) {
                    break;
                }
                let demand = cursor.demands().unwrap();
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
                match caller.work_turn(grant, 1, |cx| cursor.advance(&mutations, semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(32), cx)) {
                    Err(error) => {
                        assert!(error.primary.message().contains("requires progressive preimages"), "unexpected preflight refusal: {error:?}");
                        refused = true;
                        break;
                    }
                    Ok(complete) => assert!(!complete),
                }
            }
            assert_eq!(!refused, case["admitted"].as_bool().unwrap(), "{}", case["name"]);
            assert_eq!(cursor.bytes, 0);
            assert!(!cursor.live);
            assert!(matches!(cursor.phase, Phase::Preflight | Phase::Start));
            caller.finish();
        }
    }
}
