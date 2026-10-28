//! 🎲️ Space members macro: the generator of a per-plugin space member and member factory enum over several document kinds. See [`super`].

//#region 🔖️BlobStore
//#region 🔖️Space
//#region SpaceMember
/// 🧬️ Generates a per-plugin `SpaceMember`+`MemberFactory` enum spanning several document
/// kinds composed under one `SpaceHost`/`CompositionCoordinator` — the O1 replacement for a
/// `Box<dyn SpaceMember>` heterogeneous registry. Delegation arms are written ONCE here, next to
/// `trait SpaceMember` itself, so drift between the trait and the macro is a compile error, never a
/// silent bug. Usage:
/// ```ignore
/// space_members! {
///     native_producer:&semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
///     history_limits: HistoryReadLimits { file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64 },
///     maximum_history_allocation_bytes:8388608,
///     pub enum NoteMembers, NoteMembersOpen {
///         Text("s.note.text", "1", "*", "note.text/v1") => (TextSnapshot, TextMutation, TextMemberStoreProducer, TextMemberStoreProducer),
///         Sketch("s.note.sketch", "1", "*", "note.sketch/v1") => (SketchSnapshot, SketchMutation, SketchMemberStoreProducer, SketchMemberStoreProducer),
///     }
/// }
/// ```
/// expands to the enum, `impl SpaceMember for NoteMembers` and `impl ReferenceSpaceMember for NoteMembers`
/// (match-delegation over every required method), and `impl MemberFactory for NoteMembers` with an exact coordinate per typed store.
#[macro_export]
macro_rules! space_members {
    (native_producer:$native_producer:expr,history_limits:$history_limits:expr,maximum_history_allocation_bytes:$history_allocation_bytes:expr,pub enum $enum_name:ident, $open_name:ident { $($variant:ident($kind:literal, $standard:literal, $subset:literal, $schema:literal) => ($snapshot:ty, $mutation:ty, $catalog:ty, $catalog_value:expr)),+ $(,)? }) => {
        pub enum $enum_name {
            $($variant(Box<$crate::os_store::ArtifactStore<$snapshot, $mutation>>)),+
        }

        pub enum $open_name {
            $($variant(Box<$crate::os_store::InitialMemberStoreOpen<$enum_name, $snapshot, $mutation,$catalog>>)),+
            , ReturningSource(::semio_framework_job::OriginalCallerSourceLoan), Closed
        }

        impl $crate::os_store::MemberOpenOperation for $open_name {
            type Member = $enum_name;

            fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>, grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<$crate::os_store::MemberOpenStep<Self::Member>,::semio_framework_job::OriginalTurnFailure> {
                Ok(match self {
                    $(Self::$variant(open) => match open.step_store(cx, grant)? {
                        $crate::os_store::MemberOpenStep::Pending(progress) => $crate::os_store::MemberOpenStep::Pending(progress),
                        $crate::os_store::MemberOpenStep::Ready(member) => $crate::os_store::MemberOpenStep::Ready($enum_name::$variant(member)),
                        $crate::os_store::MemberOpenStep::Rejected(diagnostic) => $crate::os_store::MemberOpenStep::Rejected(diagnostic),
                    }),+,
                    Self::ReturningSource(_) | Self::Closed => $crate::os_store::MemberOpenStep::Rejected($crate::os_store::MemberOpenDiagnostic::Stale),
                })
            }

            fn close_step(&mut self,parent:&::semio_framework_job::OriginalCallerSourceParent,cx:&mut ::semio_framework_job::StepContext<'_>)->Result<::semio_framework_value::retained_clone::RetainedCloneStep,::semio_framework_job::OriginalTurnFailure>{
                if let Self::ReturningSource(loan)=self{let step=loan.close_step(parent,cx)?;let progress=step.progress();if loan.terminal_is_empty(){*self=Self::Closed}return Ok(if matches!(self,Self::Closed){::semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress)}else{::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)})}
                let extent=match self{$(Self::$variant(open)if open.payload_terminal_is_empty()=>{open.validate_source_context(cx)?;Some(::std::mem::size_of_val(open.as_ref()))},)+Self::Closed=>return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())),_=>None};
                if let Some(extent)=extent{let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<extent{return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()))}let loan=match self{$(Self::$variant(open)=>open.take_source_before_shell_return(cx)?,)+_=>unreachable!()};let original=::std::mem::replace(self,Self::ReturningSource(loan));drop(original);return $crate::os_store::member_open::receive_member_close(cx,Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(::semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,released_bytes:extent,..Default::default()})))}
                match self{$(Self::$variant(open)=>$crate::os_store::MemberOpenOperation::close_step(open.as_mut(),parent,cx).map(|step|::semio_framework_value::retained_clone::RetainedCloneStep::Progress(step.progress())),)+Self::ReturningSource(_)|Self::Closed=>unreachable!()}
            }
            fn next_copy_byte_demand(&self) -> Result<usize, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(open) if !open.payload_terminal_is_empty() => $crate::os_store::MemberOpenOperation::next_copy_byte_demand(open.as_ref()),)+ _ => Ok(0) }
            }
            fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(open) if !open.payload_terminal_is_empty() => $crate::os_store::MemberOpenOperation::next_capacity_byte_demand(open.as_ref(), body),)+ _ => Ok(0) }
            }
            fn next_release_byte_demand(&self) -> Result<usize, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(open) => if open.payload_terminal_is_empty() { Ok(::std::mem::size_of_val(open.as_ref())) } else { $crate::os_store::MemberOpenOperation::next_release_byte_demand(open.as_ref()) },)+ Self::ReturningSource(_)|Self::Closed => Ok(0) }
            }
            fn next_depth_demand(&self) -> Result<usize, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(open) => if open.payload_terminal_is_empty() { Ok(1) } else { $crate::os_store::MemberOpenOperation::next_depth_demand(open.as_ref()) },)+ Self::ReturningSource(_) => Ok(1),Self::Closed => Ok(0) }
            }

            fn terminal_is_empty(&self) -> bool {
                matches!(self, Self::Closed)
            }
            fn terminal_drop_byte_demand(&self) -> Option<usize> {
                matches!(self, Self::Closed).then_some(0)
            }
        }

        impl $crate::os_store::SpaceMember for $enum_name {
            fn local_actor_id(&self) -> &$crate::os_spr::ActorId {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::local_actor_id(m.as_ref())),+ }
            }
            async fn document_id(&self) -> &str {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::document_id(m.as_ref()).await),+ }
            }
            fn artifact_ref(&self) -> Option<::semio_framework_artifact_reference::ArtifactRef> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::artifact_ref(m.as_ref())),+ }
            }
            fn owner_ref(&self) -> Option<$crate::os_store::OwnerRef> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::owner_ref(m.as_ref())),+ }
            }
            fn child_restore_projection(&self) -> Result<$crate::semio_framework_composition_admission::ChildRestoreProjection<'_>, $crate::semio_framework_composition_admission::ChildRestoreProjectionError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::child_restore_projection(m.as_ref())),+ }
            }
            fn child_restore_projection_for_read<'read>(&self,read:&'read $crate::os_store::snapshot_read::ErasedSnapshotRead)->Result<$crate::semio_framework_composition_admission::ChildRestoreProjection<'read>,$crate::semio_framework_composition_admission::ChildRestoreProjectionError>{
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::child_restore_projection_for_read(m.as_ref(),read)),+ }
            }
            fn one_item_publication_identity(&self) -> (u64, [u8; 32]) {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::one_item_publication_identity(m.as_ref())),+ }
            }
            fn one_item_wire_publication_supported(&self) -> bool {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::one_item_wire_publication_supported(m.as_ref())),+ }
            }
            fn owned_publication_birth_bytes(&self, request: &$crate::os_store::MemberStoreOwnedBatchRequest) -> Result<usize, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::owned_publication_birth_bytes(m.as_ref(), request)),+ }
            }
            fn begin_one_item_owned_publication(&self, request: &mut $crate::os_store::MemberStoreOwnedBatchRequest, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<Option<Box<dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication>>, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::begin_one_item_owned_publication(m.as_ref(), request, grant)),+ }
            }
            fn advance_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, body: usize, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant, receiving:&mut  $crate::os_store::NativeDecodeControl<'_>) -> Result<$crate::os_store::one_item_publication::ArtifactStoreOneItemAdvance, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::advance_one_item_publication(m.as_mut(), publication, body, grant, receiving)),+ }
            }
            fn prepare_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, body: usize, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant, receiving:&mut  $crate::os_store::NativeDecodeControl<'_>) -> Result<$crate::os_store::one_item::ArtifactStoreOneItemPreparationStep, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::prepare_one_item_publication(m.as_mut(), publication, body, grant, receiving)),+ }
            }
            fn stage_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::one_item::ArtifactStoreOneItemPreparationStep, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::stage_one_item_publication(m.as_mut(), publication, visibility, grant)),+ }
            }
            fn adopt_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::one_item::ArtifactStoreOneItemPreparationStep, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::adopt_one_item_publication(m.as_mut(), publication, visibility, grant)),+ }
            }
            fn snapshot_read_erased_for_publication(&self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<Option<(u64, [u8; 32], $crate::os_store::snapshot_read::ErasedSnapshotRead, ::semio_framework_value::retained_clone::RetainedCloneProgress)>, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_erased_for_publication(m.as_ref(), publication, visibility, grant)),+ }
            }
            fn return_prepared_snapshot_read_erased(&self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, snapshot: $crate::os_store::snapshot_read::ErasedSnapshotRead, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::RetainedCloneStep, (::semio_framework_value::ValueError, $crate::os_store::snapshot_read::ErasedSnapshotRead)> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::return_prepared_snapshot_read_erased(m.as_ref(), publication, visibility, snapshot, grant)),+ }
            }
            fn abort_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, original:&mut ::semio_framework_value::OriginalRetirementCloseLoan<'_>, grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<::semio_framework_value::retained_clone::RetainedCloneStep, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::abort_one_item_publication(m.as_mut(), publication, original,grant)),+ }
            }
            fn snapshot_read_erased_now(&self) -> Result<$crate::os_store::snapshot_read::ErasedSnapshotRead, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_erased_now(m.as_ref())),+ }
            }
            async fn snapshot_read_erased(&self) -> Result<$crate::os_store::snapshot_read::ErasedSnapshotRead, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_erased(m.as_ref()).await),+ }
            }
            fn retire_snapshot_read_erased(&mut self, snapshot: &mut Option<$crate::os_store::snapshot_read::ErasedSnapshotRead>, grant: $crate::os_store::RetainedCloneGrant) -> Result<(Option<Box<dyn $crate::os_store::ErasedSnapshotRetirement>>, $crate::os_store::RetainedCloneProgress), ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::retire_snapshot_read_erased(m.as_mut(), snapshot, grant)),+ }
            }
            fn take_returned_snapshot_read_retirement(&mut self, grant: $crate::os_store::RetainedCloneGrant) -> Result<(Option<Box<dyn $crate::os_store::ErasedSnapshotRetirement>>, $crate::os_store::RetainedCloneProgress), ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::take_returned_snapshot_read_retirement(m.as_mut(), grant)),+ }
            }
            fn snapshot_read_retirement_demand(&self) -> ::semio_framework_value::RetirementDemand {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_retirement_demand(m.as_ref())),+ }
            }
            fn returned_snapshot_read_retirement_demand(&self) -> Result<::semio_framework_value::RetirementDemand, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::returned_snapshot_read_retirement_demand(m.as_ref())),+ }
            }
            fn snapshot_read_leases_terminal_is_empty(&self) -> bool {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_leases_terminal_is_empty(m.as_ref())),+ }
            }
            fn close_owned_step(&mut self, grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<::semio_framework_value::retained_clone::RetainedCloneStep, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::close_owned_step(m.as_mut(), grant)),+ }
            }
            fn close_owned_terminal_is_empty(&self) -> bool {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::close_owned_terminal_is_empty(m.as_ref())),+ }
            }
            fn close_owned_demands(&self, maximum_body_bytes: usize) -> Result<::semio_framework_value::RetirementDemand, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::close_owned_demands(m.as_ref(), maximum_body_bytes)),+ }
            }
            fn content_revision_now(&self) -> [u8; 32] {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::content_revision_now(m.as_ref())),+ }
            }
            async fn content_revision(&self) -> [u8; 32] {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::content_revision(m.as_ref()).await),+ }
            }
            async fn is_dirty(&self) -> bool {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::is_dirty(m.as_ref()).await),+ }
            }
            async fn current_checkpoint_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::current_checkpoint_id(m.as_ref()).await),+ }
            }
            async fn current_alternative_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::current_alternative_id(m.as_ref()).await),+ }
            }
            async fn last_local_edit_timestamp(&self) -> Option<$crate::os_spr::HybridLogicalTimestamp> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::last_local_edit_timestamp(m.as_ref()).await),+ }
            }
            async fn last_undone_local_edit_timestamp(&self) -> Option<$crate::os_spr::HybridLogicalTimestamp> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::last_undone_local_edit_timestamp(m.as_ref()).await),+ }
            }
            async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::as_any_mut(m.as_mut()).await),+ }
            }
            async fn tail_group_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::tail_group_id(m.as_ref()).await),+ }
            }
            async fn tail_edit_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::tail_edit_id(m.as_ref()).await),+ }
            }
            async fn redo_tail(&self) -> Option<(String, Option<String>)> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::redo_tail(m.as_ref()).await),+ }
            }
            async fn merge_policy(&self) -> $crate::os_spr::MergePolicy {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::merge_policy(m.as_ref()).await),+ }
            }
        }

        impl $crate::os_store::ReferenceSpaceMember for $enum_name {
            fn begin_one_item_wire_publication(&self, request: $crate::os_store::one_item_publication::MemberStoreOneItemWireRequest) -> Result<Box<dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication>, $crate::os_store::one_item_publication::ArtifactStoreBatchAdmissionRejected<$crate::os_store::one_item_publication::MemberStoreOneItemWire>> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::begin_one_item_wire_publication(m.as_ref(), request)),+ }
            }
            async fn commit_checkpoint(&mut self, message: String, authors: Vec<$crate::os_store::Author>, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::commit_checkpoint(m.as_mut(), message, authors, identity).await),+ }
            }
            async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::checkout(m.as_mut(), checkpoint_id, alternative_id, identity).await),+ }
            }
            async fn create_alternative(&mut self, name: String, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::create_alternative(m.as_mut(), name, identity).await),+ }
            }
            async fn undo(&mut self, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::undo(m.as_mut(), identity).await),+ }
            }
            async fn redo(&mut self, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::redo(m.as_mut(), identity).await),+ }
            }
            async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<$crate::os_spr::MutationMessage> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::preview_wire(m.as_ref(), ops).await),+ }
            }
            async fn dispatch_wire(&mut self, cmd_bytes: &[u8], identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<$crate::os_store::CommandReceipt, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::dispatch_wire(m.as_mut(), cmd_bytes, identity).await),+ }
            }
            async fn dispatch_wire_with_policy(&mut self, cmd_bytes: &[u8], policy: $crate::os_spr::MergePolicy, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<$crate::os_store::CommandReceipt, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::dispatch_wire_with_policy(m.as_mut(), cmd_bytes, policy, identity).await),+ }
            }
            async fn event_log_payload(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::event_log_payload(m.as_ref()).await),+ }
            }
            async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::announce_tail_edit_payload(m.as_mut()).await),+ }
            }
            async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::ingest_remote_payload(m.as_mut(), envelopes).await),+ }
            }
            async fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::stamp_tail_group_id(m.as_mut(), group_id).await),+ }
            }
            async fn stamp_tail_origin(&mut self, origin: $crate::os_spr::MutationOrigin) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::stamp_tail_origin(m.as_mut(), origin).await),+ }
            }
            async fn set_owner(&mut self, owner: Option<$crate::os_store::OwnerRef>) {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::set_owner(m.as_mut(), owner).await),+ }
            }
            async fn document_pack_bytes(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::document_pack_bytes(m.as_ref()).await),+ }
            }
            async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::envelope_pack_bytes(m.as_ref()).await),+ }
            }
            async fn merge_persisted_envelope(&mut self, envelope_pack: &[u8]) -> Result<$crate::os_store::replay::PairMerge, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::merge_persisted_envelope(m.as_mut(), envelope_pack).await),+ }
            }
            async fn pack_at_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::pack_at_checkpoint(m.as_ref(), checkpoint_id).await),+ }
            }
        }

        impl $crate::os_store::MemberFactory for $enum_name {
            const OPEN_DECLARATIONS: &'static [$crate::os_store::MemberOpenDeclaration] = &[
                $($crate::os_store::MemberOpenDeclaration { kind: $kind, standard: $standard, subset: $subset, schema: $schema,history_limits:$history_limits,maximum_history_allocation_bytes:$history_allocation_bytes }),+
            ];

            type Open = $open_name;

            fn open_birth_demand(request: &$crate::os_store::MemberOpenRequest) -> Result<$crate::os_store::RetainedCloneBirthDemand, $crate::os_store::MemberOpenDiagnostic> {
                let dialect = &request.admitted_expected()?.dialect;
                match (dialect.artifact_kind.as_str(), dialect.standard.as_str(), dialect.subset.as_str()) {
                    $(($kind, $standard, $subset) => {
                        let child = $crate::os_store::InitialMemberStoreOpen::<Self, $snapshot, $mutation,$catalog>::begin_birth_demand(request,&$catalog_value)?;
                        let capacity_bytes = child.capacity_bytes.checked_add(::std::mem::size_of::<$crate::os_store::InitialMemberStoreOpen<Self, $snapshot, $mutation,$catalog>>()).ok_or($crate::os_store::MemberOpenDiagnostic::Capacity)?;
                        let depth = child.depth.checked_add(1).ok_or($crate::os_store::MemberOpenDiagnostic::Capacity)?;
                        Ok($crate::os_store::RetainedCloneBirthDemand { capacity_bytes, depth })
                    },)+
                    _ => Err($crate::os_store::MemberOpenDiagnostic::Identity),
                }
            }

            fn open_copy_byte_demand(request:&$crate::os_store::MemberOpenRequest)->Result<usize,$crate::os_store::MemberOpenDiagnostic>{
                let dialect=&request.admitted_expected()?.dialect;
                match(dialect.artifact_kind.as_str(),dialect.standard.as_str(),dialect.subset.as_str()){
                    $(($kind,$standard,$subset)=>$crate::os_store::InitialMemberStoreOpen::<Self,$snapshot,$mutation,$catalog>::begin_copy_byte_demand(request,&$catalog_value)?.checked_add(2*::std::mem::size_of::<Self::Open>()).ok_or($crate::os_store::MemberOpenDiagnostic::Capacity),)+
                    _=>Err($crate::os_store::MemberOpenDiagnostic::Identity),
                }
            }

            fn member_frame_bytes(&self) -> usize {
                match self { $(Self::$variant(member) => ::std::mem::size_of_val(member.as_ref())),+ }
            }

            fn begin_open(admission:&mut $crate::os_store::MemberOpenSourceAdmission,opened:&mut Option<Self::Open>,cx:&mut ::semio_framework_job::StepContext<'_>,parent:&::semio_framework_job::OriginalCallerSourceParent)->Result<bool,::semio_framework_job::OriginalTurnFailure>{
                if opened.is_some(){return Err($crate::os_store::member_open::member_admission_diagnostic($crate::os_store::MemberOpenDiagnostic::Stale).into())}
                if !admission.admit_source(parent,cx)?||cx.should_yield(){return Ok(false)}
                let retained=admission.request().ok_or_else(||$crate::os_store::member_open::member_admission_diagnostic($crate::os_store::MemberOpenDiagnostic::Stale))?;
                let required_copy=Self::open_copy_byte_demand(retained).map_err($crate::os_store::member_open::member_admission_diagnostic)?;
                let required=Self::open_birth_demand(retained).map_err($crate::os_store::member_open::member_admission_diagnostic)?;
                let grant=cx.retained_grant();
                if grant.maximum_items==0||grant.maximum_copy_bytes<required_copy||grant.maximum_depth<required.depth||grant.maximum_capacity_bytes<required.capacity_bytes{return Ok(false)}
                let dialect=&retained.admitted_expected().map_err($crate::os_store::member_open::member_admission_diagnostic)?.dialect;
                match(dialect.artifact_kind.as_str(),dialect.standard.as_str(),dialect.subset.as_str()){
                    $(($kind,$standard,$subset)=>{let(open,mut progress)=$crate::os_store::InitialMemberStoreOpen::begin(admission,cx,::semio_framework_value::retained_clone::RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-2*::std::mem::size_of::<Self::Open>(),maximum_capacity_bytes:grant.maximum_capacity_bytes-::std::mem::size_of::<$crate::os_store::InitialMemberStoreOpen<Self,$snapshot,$mutation,$catalog>>(),maximum_depth:grant.maximum_depth-1,..grant},$catalog_value)?;progress.retained_capacity_bytes+=::std::mem::size_of_val(&open);progress.copied_bytes+=2*::std::mem::size_of::<Self::Open>();*opened=Some($open_name::$variant(Box::new(open)));$crate::os_store::member_open::receive_member_close(cx,Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)))?;Ok(true)},)+
                    _=>Err($crate::os_store::member_open::member_admission_diagnostic($crate::os_store::MemberOpenDiagnostic::Identity).into()),
                }
            }

            async fn create(id: &str, dialect: &::semio_framework_artifact_reference::ArtifactDialect, initial_pack: &[u8], actor: $crate::os_spr::ActorId) -> Result<Self, $crate::os_store::VcsError> {
                match (dialect.artifact_kind.as_str(), dialect.standard.as_str(), dialect.subset.as_str()) {
                    $(($kind, $standard, $subset) => Ok(Self::$variant(Box::new($crate::os_store::create_member_store($schema, id, dialect, initial_pack, actor, $native_producer,&$catalog_value).await?))),)+
                    _ => Err($crate::os_store::VcsError::ValidationFailed(format!("no member dialect '{}' registered in {}", ::semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText::to_coordinate(dialect), stringify!($enum_name)))),
                }
            }
            async fn open(expected: &::semio_framework_artifact_reference::ArtifactRef, owner: Option<&$crate::os_store::OwnerRef>, envelope_pack: &[u8], actor: $crate::os_spr::ActorId) -> Result<Self, $crate::os_store::VcsError> {
                match (expected.dialect.artifact_kind.as_str(), expected.dialect.standard.as_str(), expected.dialect.subset.as_str()) {
                    $(($kind, $standard, $subset) => Ok(Self::$variant(Box::new($crate::os_store::open_member_store($schema, expected, owner, envelope_pack, actor, $native_producer,&$catalog_value).await?))),)+
                    _ => Err($crate::os_store::VcsError::ValidationFailed(format!("no member dialect '{}' registered in {}", ::semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText::to_coordinate(&expected.dialect), stringify!($enum_name)))),
                }
            }
        }

        impl $crate::os_store::MemberVisit for $enum_name {
            fn visit_member<V: $crate::os_store::MemberStoreVisitor>(&self, visitor: V) -> V::Output {
                match self { $(Self::$variant(m) => visitor.visit(m.as_ref())),+ }
            }
            async fn visit_member_mut<V: $crate::os_store::MemberStoreVisitorMut>(&mut self, visitor: V) -> V::Output {
                match self { $(Self::$variant(m) => visitor.visit_mut(m.as_mut()).await),+ }
            }
        }
    };
}
//#endregion SpaceMember
//#endregion 🔖️Space
//#endregion 🧪️Tests
