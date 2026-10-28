//! 🧸️ The shared native Window fixture retains real Pack receiving and Store semantic preparation.
use super::*;
use semio_framework_dsl_record::{BorrowedDslRecord,native_encoding::FieldProjectionSource};
#[derive(Clone,Copy,Debug,PartialEq,semio_framework_value::RetireOwned,semio_framework_value::RetainedClone,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::CanonicalJsonTree,semio_framework_dsl_record_derive::DslRecord,semio_framework_dsl_record_derive::DslReceiving,store::DslArtifact)]
#[canonical_json(owner=store)]
#[artifact(id="test.window.genesis")]
#[dsl(layout="lines")]
pub(crate) struct State{pub(crate) zoom:f64}
impl Default for State{fn default()->Self{Self{zoom:1.25}}}
impl pack::snapshot::ArtifactDsl for State{
 const EXTENSION:&'static str="windowgenesis";
 fn envelope_id()->&'static str{Self::__DSL_ENVELOPE_ID}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{let body=store::semio_format::split_text_preamble(text).map_or(text,|(_,body)|body);let record=semio_framework_dsl_record::parse(body,&Self::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:Default::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;Self::__dsl_from_record(&record)}
 fn print_dsl(&self)->String{let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::__DSL_ENVELOPE_ID,store::semio_format::Component::Dsl,1).unwrap();store::semio_format::wrap_text(&envelope,&body)}
}
impl pack::snapshot::ArtifactPack for State{
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let body=store::pack_rt::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::__DSL_ENVELOPE_ID,store::semio_format::Component::Pack,1).map_err(|e|store::PackError::from(e.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&body))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,body)=store::semio_format::unwrap_binary(bytes).map_err(|e|store::PackError::from(e.into_value_error()))?;if !envelope.matches_identity(Self::__DSL_ENVELOPE_ID,store::semio_format::Component::Pack,1){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Window fixture envelope differs").into())}let(record,_)=store::pack_rt::decode_document(&body,&Self::__dsl_spec(),options)?;Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)}
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Self::__dsl_spec())}
 fn record_spec_producer()->Option<store::RecordSpecProducer>{Some(Self::__artifact_record_spec_producer())}
 fn borrowed_record_spec_producer()->Option<store::BorrowedRecordSpecProducer>{Some(Self::__artifact_borrowed_record_spec_producer())}
}
impl semio_framework_os_store_codec_registry::native_snapshot::OriginalArtifactPackReceiving for State{
 type Receiving=semio_framework_os_store_codec_registry::native_snapshot::OriginalSnapshotRecordReceiving<Self>;
 fn original_pack_birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self::Receiving>(),depth:1,..Default::default()}}
 fn admit_original_pack(source:&[u8],limits:protocol::PackLimits,native:&semio_framework_value::NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<Option<(Self::Receiving,RetainedCloneProgress)>,ValueError>{Self::Receiving::admit(source,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),limits,native,grant)}
}
impl store::os_store::serialization::ArtifactStoreSnapshot for State{
    fn reference_snapshot_codec()->Option<store::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(store::os_store::serialization::reference_snapshot_codec::<Self>())}
}
impl store::ConfigRecord for State{}
#[derive(Clone,Copy,Debug,PartialEq,Default,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::FromValue)]
pub(crate) struct Diff{zoom:Option<f64>}
impl protocol::MutationDiff<State> for Diff{}
impl protocol::DiffAlgebra<State> for Diff{fn is_empty(&self)->bool{self.zoom.is_none()}}
#[derive(Clone,Debug,PartialEq,semio_framework_value::RetireOwned,semio_framework_value::RetainedClone,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::CanonicalJsonTree,semio_framework_dsl_record_derive::DslEnum,semio_framework_dsl_record_derive::DslReceiving,semio_framework_value::NativeReceive)]
#[canonical_json(owner=store)]
#[value(tag="kind",rename_all="kebab-case")]
pub(crate) enum Mutation{#[dsl(key="zoom")] Zoom{zoom:f64}}
impl protocol::Mutation<State> for Mutation{
 type Diff=Diff;
 fn may_emit_foreign_steps(&self)->bool{false}
 const DESCRIPTORS:&'static[protocol::MutationLeafDescriptor]=&[protocol::MutationLeafDescriptor{schema_version:1,owner:"test.window.genesis",semantic_kind:"set-zoom",display_name:"Set Zoom",emoji:"🔎️",aggregate_variant:"Zoom",payload_schema:"test.window.genesis.zoom",text_opcode:None,binary_tag:None,invertibility:protocol::MutationInvertibility::ExplicitMutation,diff_participation:protocol::MutationDiffParticipation::Detect,outcome_classes:&[protocol::MutationOutcomeClass::Applied],composition:protocol::MutationComposition::Atomic,required_language_surfaces:&[protocol::MutationLanguageSurface::Rust]}];
 fn descriptor(&self)->&'static protocol::MutationLeafDescriptor{&Self::DESCRIPTORS[0]}
}
impl protocol::OpBinary for Mutation{fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{dsl::variants_binary::encode_op(self)}fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{dsl::variants_binary::decode_op(bytes)}}
impl protocol::OpText for Mutation{
 fn parse_op(line:&str)->Result<Self,semio_framework_diagnostic::TextError>{let spec=<Self as semio_framework_dsl_record::DslVariants>::variants()[0].1;let record=semio_framework_dsl_record::parse(line,&(spec.ordinary)(),&semio_framework_dsl_record::ParseOptions{limits:Default::default(),mode:semio_framework_dsl_record::SourceMode::Inline})?;<Self as semio_framework_dsl_record::DslVariants>::from_named_record("zoom",&record)}
 fn print_op(&self)->String{let(_,record)=<Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);let spec=<Self as semio_framework_dsl_record::DslVariants>::variants()[0].1;semio_framework_dsl_record::print(&record,&(spec.ordinary)(),semio_framework_dsl_record::JoinMode::Inline)}
}
impl WindowConfigApplyMutation<State> for Mutation{fn exchange(self,post:&mut State)->Result<Self,(ValueError,Self)>{let Self::Zoom{zoom}=self;Ok(Self::Zoom{zoom:std::mem::replace(&mut post.zoom,zoom)})}fn admissible(&self)->bool{let Self::Zoom{zoom}=self;zoom.is_finite()&&*zoom>0.0}}
impl semio_framework_os_store_operation_wire::ArtifactOperationTargets for Mutation{
 fn len(&self)->usize{1}
 fn segment_len(&self,index:usize)->Result<usize,ValueError>{if index==0{Ok(4)}else{Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Window target ordinal is absent"))}}
 fn read_segment(&self,index:usize,offset:usize,output:&mut[u8])->Result<usize,ValueError>{if index!=0||offset>4{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Window target ordinal is absent"))}let source=&b"zoom"[offset..];let count=source.len().min(output.len());output[..count].copy_from_slice(&source[..count]);Ok(count)}
}
pub(crate) struct Owner;
impl WindowConfigOwner for Owner{
 const WINDOW_KIND_ID:&'static str="genesis-camera";const SCHEMA:&'static str="test.window.genesis";const MAXIMUM_PUBLICATION_BYTES:usize=4096;
 type State=State;type Mutation=Mutation;type Edit=WindowConfigApplyEdit<State,Mutation>;const MAXIMUM_PREPARATION_DEPTH:usize=64;
 fn native_pair_demands()->Result<RetirementDemand,ValueError>{Ok(window_config_native_pair_demands::<Self::State>())}
 fn admit_native_pair(pack:&[u8],spr:&[u8],pack_limits:protocol::PackLimits,history_limits:semio_framework_os_store_codec_registry::native_snapshot::OriginalArtifactHistoryReadLimits,native:&mut semio_framework_value::NativeDecodeControl<'_>,grant:RetainedCloneGrant,output:&mut Option<Box<dyn WindowConfigNativePairReceiving<Self::State>>>)->Result<RetainedCloneProgress,semio_framework_os_store_codec_registry::native_snapshot::OriginalArtifactPairFailure>{admit_window_config_native_pair::<Self::State>(pack,spr,pack_limits,history_limits,native,grant,output)}
 fn native_operation_wire()->semio_framework_os_store_operation_wire::ArtifactOperationWire<Self::Mutation>{semio_framework_os_store_operation_wire::original_edit::with_original_edit_message(semio_framework_os_store_operation_wire::ArtifactOperationWire::canonical().with_original_receiving().with_original_target(|operation|Some(operation)))}
 fn initial_state_demands()->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:2*std::mem::size_of::<State>(),depth:1,..Default::default()})}
 fn initial_state(grant:RetainedCloneGrant)->Result<(State,RetainedCloneProgress),ValueError>{let d=Self::initial_state_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Window initial fixed state transfer was not funded"))}Ok((State::default(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 fn initial_state_pack_source(state:&State)->Result<(&dyn FieldProjectionSource,semio_framework_dsl_record::BorrowedRecordSpec),ValueError>{Ok((state,State::RECORD))}
 fn build_retained_edit()->Arc<Self::Edit>{Arc::new(Self::Edit::new())}
 fn preparation_source_birth()->semio_framework_value::retained_clone::RetainedCloneBirthDemand{bounded_window_config_preparation_source_birth::<Self>()}
 fn build_store_owners(grant:RetainedCloneGrant)->Result<(store::os_store::document_store_owners::DocumentStoreOwners<State,Mutation>,RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<State,Mutation>>{bounded_window_config_store_owners::<Self>(grant)}
 fn build_one_item_preparation_factory()->Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<State,Mutation>>{bounded_window_config_preparation_factory::<Self>()}
 fn store_disposer_birth()->Result<RetirementDemand,ValueError>{bounded_window_config_store_disposer_birth::<Self>()}
 fn build_store_disposer()->Box<dyn ArtifactOwnedDisposer<store::ConfigStore<State,Mutation>>>{bounded_window_config_store_disposer::<Self>()}
}
