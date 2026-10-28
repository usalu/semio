//! 🧱️ Store owners and member visitors of the time-travel session: the owners implementation of the store state, the composed member run, commit, settle and query visitors and the selection and generation argument readers. See [`super`].

use super::*;

//#region 🔖️StoreState
impl<P, Mu> TimeTravelOwners for TimeTravelStoreState<P, Mu>
where
    P: semio_framework_value::ToValue  + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mu: semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
{
    fn replaying(&self) -> bool {
        self.replay.is_some()
    }

    fn preview_progress(&self) -> Option<store::os_store::replay::ReplayProgress> {
        self.preview_job.as_ref().map(store::os_store::snapshot_read::ArtifactDerivedHistoryPreview::progress).or_else(||self.preview_derived.as_ref().filter(|owner|!owner.is_closing()).map(|owner|store::os_store::replay::ReplayProgress{done:u32::try_from(owner.progress().items).unwrap_or(u32::MAX),total:0}))
    }

    fn processed(&self) -> Option<u32> {
        self.replay.as_ref().map(|replay| replay.operation_preparation_progress().items).filter(|items| *items > 0).map(|items| u32::try_from(items).unwrap_or(u32::MAX))
    }

    fn has_pending_work(&self) -> bool {
        self.finalizing()||self.finished_cancelled&&self.finished.is_some()||self.snapshot_aliases.iter().any(Option::is_some) || self.snapshot_close.is_some() || self.snapshot_issuers.is_some() || self.snapshot_issuer_close.iter().any(Option::is_some) || self.discard_effects.as_ref().is_some_and(|owner|owner.closing) || self.preview_job.is_some()||self.preview_derived.as_ref().is_some_and(|owner|owner.is_closing())||self.preview_cancelled&&(self.preview_original.is_some()||self.preview_input.is_some()||self.preview_source.is_some()||self.preview_native_input.is_some())||self.preview_input_close.is_some()||self.preview_original_close.is_some() || self.retirements.capacity()!=0 || !self.discarded.is_empty() || self.discarded_has_physical_capacity() || self.discarded_pending.is_some() || self.discarded_active.is_some() || self.discarded_factory.is_some() || self.discarded_factory_close.is_some()
    }

    fn terminal_is_empty(&self) -> bool {
        !self.finalizing()&&self.snapshot_aliases.iter().all(Option::is_none) && self.snapshot_close.is_none() && self.snapshot_issuers.is_none() && self.snapshot_issuer_close.iter().all(Option::is_none) && self.discard_effects.is_none() && self.staged.is_none() && self.kind.is_none() && self.preview.is_none() && self.preview_job.is_none() && self.preview_input.is_none()&&self.preview_base.is_none()&&self.preview_original.is_none()&&self.preview_source.is_none()&&self.preview_native_input.is_none()&&self.preview_derived.is_none()&&self.preview_input_close.is_none()&&self.preview_original_close.is_none() && self.replay.is_none() && self.finished.is_none() && self.head.is_none() && self.retirements.capacity()==0 && self.discarded_terminal_is_empty() && self.discarded_pending.is_none() && self.discarded_active.is_none() && self.discarded_factory.is_none() && self.discarded_factory_close.is_none()
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.original_retirement_demands(body) }
    fn command_pending(&self)->bool{self.discard_effects.is_some()||self.finalizing()}
    fn finalizing(&self)->bool{TimeTravelStoreState::finalizing(self)}
    fn cancel_prepared_command(&mut self){self.commit_closing=self.finalizing();if let Some(owner)=self.discard_effects.as_mut(){owner.closing=true}}
    fn requires_original_control(&self)->bool{self.preview_derived.as_ref().is_some_and(|owner|owner.is_closing()&&owner.requires_original_control())||self.retirements.iter().any(|owner|owner.requires_original_control())}
    fn retire_step_controlled(&mut self,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<Option<RetainedCloneStep>,Fault>{self.original_retirement_step_controlled(grant,control).map_err(ValueError::into_fault)}
    fn retire_step(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, Fault> { self.original_retirement_step(grant).map_err(ValueError::into_fault) }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// ✍️ One typed history-edit command against a composed member's store (design §12): creates the member's owners on the
/// first visit, then runs the command on them.
#[doc(hidden)]
pub  struct TimeTravelMemberRun<'a,'c> {
    pub owners: &'a mut Option<Box<dyn TimeTravelOwners>>,
    pub command: TimeTravelStoreCommand,pub receiving:Option<&'a mut semio_framework_job::StepContext<'c>>,
}

impl store::MemberStoreVisitorMut for TimeTravelMemberRun<'_,'_> {
    type Output = Result<TimeTravelStoreOutput, Fault>;

    async fn visit_mut<P, Mu>(self, store: &mut ArtifactStore<P, Mu>) -> Self::Output
    where
        P: semio_framework_value::ToValue  + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        for documents in <Mu as ::protocol::Mutation<P>>::INPUT_SCHEMA_DOCUMENTS {
            semio_framework_schema_registry::register_referenced_schema_documents(documents);
        }
        let owners = self.owners.get_or_insert_with(|| Box::new(TimeTravelStoreState::<P, Mu>::new(|op| protocol::SemanticMutation::<P>::label(op))) as Box<dyn TimeTravelOwners>);
        let state = owners.as_any_mut().downcast_mut::<TimeTravelStoreState<P, Mu>>().ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.member-store-kind"), "a composed member's history-edit owners belong to another store kind"))?;
        state.run(store, self.command,self.receiving)
    }
}

/// 🪪️ Transfers the actual finalization request into the original composed member's retained Store state.
#[doc(hidden)]
pub struct TimeTravelMemberCommit<'a,'c>{pub owners:&'a mut Option<Box<dyn TimeTravelOwners>>,pub request:&'a mut Option<store::HistoryFinalization>,pub actor:Option<&'a str>,pub receiving:&'a mut semio_framework_job::StepContext<'c>}
impl store::MemberStoreVisitorMut for TimeTravelMemberCommit<'_,'_>{type Output=Result<TimeTravelCommit,Fault>;async fn visit_mut<P,Mu>(self,store:&mut ArtifactStore<P,Mu>)->Self::Output where P:semio_framework_value::ToValue+store::os_store::serialization::ArtifactStoreSnapshot+semio_framework_schema_composition::ArtifactCompositionFields+Send+Sync+'static,Mu:semio_framework_value::ToValue+semio_framework_value::FromValue+protocol::Mutation<P>+protocol::SemanticMutation<P>+protocol::OpBinary+protocol::OpText+Send+'static{let state=self.owners.as_deref_mut().and_then(|owners|owners.as_any_mut().downcast_mut::<TimeTravelStoreState<P,Mu>>()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original member finalization Store state is absent").into_fault())?;state.commit_finished(store,self.request,self.actor,self.receiving)}}

/// 🧽️ Settles a composed member's owners for `stage` against its store (design §12).
#[doc(hidden)]
pub struct TimeTravelMemberSettle<'a> {
    pub owners: &'a mut dyn TimeTravelOwners,
    pub stage: TimeTravelStage,
}

impl store::MemberStoreVisitor for TimeTravelMemberSettle<'_> {
    type Output = Result<(), Fault>;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: semio_framework_value::ToValue  + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        self.owners.as_any_mut().downcast_mut::<TimeTravelStoreState<P, Mu>>().ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.member-store-kind"), "a composed member's history-edit owners belong to another store kind"))?.settle(store, self.stage)
    }
}

/// 🔎️ A store-only read of a composed member's store (design §12).
#[doc(hidden)]
pub struct TimeTravelMemberQuery<'a> {
    pub owners: Option<&'a dyn TimeTravelOwners>,
    pub query: TimeTravelStoreQuery,
}

impl store::MemberStoreVisitor for TimeTravelMemberQuery<'_> {
    type Output = TimeTravelQueryOutput;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: semio_framework_value::ToValue  + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        TimeTravelStoreState::<P, Mu>::query(self.owners.and_then(|owners| owners.as_any().downcast_ref::<TimeTravelStoreState<P, Mu>>()), store, self.query)
    }
}
//#endregion 🔖️StoreState

//#region 🔖️Pointer
/// 🎯️ The draft value of reference input `input` read from `selection`, the current selection of its declared domain:
/// the selected ids typed by the input's id type (ids the type cannot spell are skipped), all of them up to `max_items`
/// for a many-reference, else the first. Refused `unknown-input` for an input that is no reference or names no domain,
/// `no-selection` when the domain selects nothing at the input's granularity.
#[doc(hidden)]
pub fn time_travel_selection_value(input: &ActionArgDef, selection: Option<&protocol::DomainSelection>) -> Result<DslValue, TimeTravelActionRefusal> {
    let ArgSchema::Reference { domain: Some(_), granularity, many, max_items, id_type, .. } = &input.schema else { return Err(TimeTravelActionRefusal::UnknownInput) };
    let selection = selection.filter(|selection| granularity.as_ref().is_none_or(|granularity| *granularity == selection.granularity)).ok_or(TimeTravelActionRefusal::NoSelection)?;
    let mut ids = selection.ids.iter().filter_map(|id| id_type.id_value(id));
    if !*many {
        return ids.next().ok_or(TimeTravelActionRefusal::NoSelection);
    }
    let ids: Vec<DslValue> = ids.take(max_items.map_or(usize::MAX, |max| max as usize)).collect();
    match ids.is_empty() {
        true => Err(TimeTravelActionRefusal::NoSelection),
        false => Ok(DslValue::Array(ids)),
    }
}
//#endregion 🔖️Pointer

//#region 🔖️Driver
/// 🧿️ The session generation a verb names, in every shape the wire hands it over; `None` addresses the live session.
#[doc(hidden)]
pub fn time_travel_arg_generation(args: Option<&DslValue>) -> Option<u32> {
    semio_framework_plugin_tool_run::tool_run_arg_u64(args, HISTORY_EDIT_ARG_GENERATION).and_then(|generation| u32::try_from(generation).ok())
}
//#endregion 🔖️Driver
