//! 📦️ Every partial frame field remains in the actual receiving owner until paid transfer or retirement.
use super::*;
#[derive(Default,semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value")]
pub(super) struct Fields{pub numbers:[Option<u64>;8],pub flags:[Option<u8>;4],pub texts:[Option<String>;4],pub bytes:[Option<Vec<u8>>;7],pub envelopes:Vec<protocol::causal::MutationEnvelope>,pub children:Vec<ChildPackEntry>,pub heads:Vec<ChildHeadPackEntry>,pub windows:Vec<WindowConfigPackEntry>,pub local_ops:Vec<Vec<u8>>,pub foreign:Vec<Vec<u8>>,pub archive:DocumentArchivePack,pub query:Option<protocol::LocalInteractionQueryReply>}
impl Fields{
 pub fn n(&mut self,index:usize)->u64{self.numbers[index].take().unwrap()}
 pub fn b(&mut self,index:usize)->Vec<u8>{self.bytes[index].take().unwrap()}
 pub fn t(&mut self,index:usize)->String{self.texts[index].take().unwrap()}
 fn optional(&mut self)->Option<u64>{if self.flags[0]==Some(1){Some(self.n(0))}else{None}}
 fn handle(&mut self)->MediaExportHandleWire{MediaExportHandleWire{app_instance_id:self.n(1)as u32,parent_document_id:self.t(0),operation_id:self.n(2),base_revision:self.n(3),generation:self.n(4)}}
 pub fn build(&mut self,tag:u8)->AppFrame{match tag{
 0=>AppFrame::Done{in_reply_to:self.n(0)},
 1=>AppFrame::Invocation{in_reply_to:self.n(0),output:self.b(0),diagnostics:self.b(1),ui_scope:self.b(2),history_patch:self.b(3),messages:self.b(4),mutations:self.b(5),inverse_group:self.b(6)},
 2=>AppFrame::DocumentChanged{envelopes:std::mem::take(&mut self.envelopes),origin:self.t(0)},
 3=>AppFrame::Document{in_reply_to:self.n(0),pack:self.b(0),spr:self.b(1),ops:self.t(0)},
 4=>AppFrame::Config{in_reply_to:self.n(0),pack:self.b(0),spr:self.b(1),ops:self.t(0)},
 5=>AppFrame::ConfigChanged{envelopes:std::mem::take(&mut self.envelopes),origin:self.t(0)},
 6=>AppFrame::ContextMenu{in_reply_to:self.n(0),items:self.b(0)},
 7=>AppFrame::Media{in_reply_to:self.n(0),port:self.t(0),descriptor:self.b(0),data:self.b(1)},
 8=>AppFrame::MediaFingerprint{in_reply_to:self.n(0),port:self.t(0),fingerprint:self.b(0)},
 9=>AppFrame::Error{in_reply_to:self.optional(),fault:self.b(0),report:self.b(1)},
 10=>AppFrame::Emit{in_reply_to:self.n(0),document_ops:self.b(0),config_ops:self.b(1),draft_ops:self.b(2),output:self.b(3),diagnostics:self.b(4),child_ops:self.b(5)},
 11=>AppFrame::Draft{in_reply_to:self.n(0),pack:self.b(0),spr:self.b(1),ops:self.t(0)},
 12=>AppFrame::Children{in_reply_to:self.n(0),entries:std::mem::take(&mut self.children)},
 13=>AppFrame::Ephemeral{presence:self.b(0),presence_generation:self.n(0),transient_generation:self.n(1),interaction:self.b(1),tool_run:self.b(2),history_edit:self.b(3)},
 14=>AppFrame::HistorySnapshot{in_reply_to:self.n(0),history_patch:self.b(0)},
 15=>AppFrame::TransactionProposal{in_reply_to:self.n(0),proposal_id:self.t(0),local_ops:std::mem::take(&mut self.local_ops),foreign:std::mem::take(&mut self.foreign)},
 16=>AppFrame::TransactionPrepared{txn_id:self.t(0),foreign:std::mem::take(&mut self.foreign),rejection:self.b(0)},
 17=>AppFrame::TransactionCommitted{txn_id:self.t(0),edit_id:self.t(1)},
 18=>AppFrame::TransactionRolledBack{txn_id:self.t(0)},
 19=>AppFrame::MergeReport{in_reply_to:self.optional(),report:self.b(0)},
 20=>AppFrame::Conflicts{in_reply_to:self.optional(),conflicts:self.b(0)},
 21=>AppFrame::UiPatch{in_reply_to:self.optional(),surface:self.t(0),kind:self.t(1),revision:self.n(1),base_revision:self.n(2),ops:self.b(0)},
 22=>AppFrame::UiSnapshotEnd{revision:self.n(0)},23=>AppFrame::LocalInteractionQuery{reply:self.query.take().unwrap()},
 24=>AppFrame::WindowConfigs{in_reply_to:self.n(0),entries:std::mem::take(&mut self.windows)},
 25=>AppFrame::OperationCompleted{operation:self.n(0),revision:self.n(1),ui_scope:self.b(0),history_patch:self.b(1)},
 26=>AppFrame::DocumentArchive{in_reply_to:self.n(0),archive:std::mem::take(&mut self.archive)},
 27=>AppFrame::DocumentArchiveLoad{in_reply_to:self.n(0),status:DocumentArchiveLoadStatus{operation:self.n(1),state:match self.flags[0].unwrap(){0=>DocumentArchiveLoadState::Pending,1=>DocumentArchiveLoadState::Running,2=>DocumentArchiveLoadState::Ready,3=>DocumentArchiveLoadState::Cancelled,4=>DocumentArchiveLoadState::Fault,_=>unreachable!()},completed:self.n(2),total:self.n(3),ahead:self.n(4),fault:self.b(0)}},
 28=>AppFrame::MediaExportSubmitted{in_reply_to:self.n(0),handle:self.handle()},
 29=>AppFrame::MediaExportStatus{in_reply_to:self.n(0),handle:self.handle(),state:match self.flags[0].unwrap(){0=>MediaExportStateWire::Running,1=>MediaExportStateWire::Complete,2=>MediaExportStateWire::Cancelled,3=>MediaExportStateWire::Failed,_=>unreachable!()},applied_progress:self.n(5),checkpoint_available:self.flags[1]==Some(1),mime_type:self.t(1),total_bytes:self.n(6),detail:self.t(2),descriptor:self.b(0)},
 30=>AppFrame::MediaExportChunk{in_reply_to:self.n(0),handle:self.handle(),data:self.b(0),terminal:self.flags[0]==Some(1)},
 31=>AppFrame::DocumentIdentity{in_reply_to:self.n(0),identity:AppDocumentIdentity{app_instance_id:self.n(1)as u32,parent_document_id:if self.flags[0]==Some(1){Some(self.t(0))}else{None},base_revision:self.n(2)}},
 32=>AppFrame::ChildHeads{in_reply_to:self.n(0),entries:std::mem::take(&mut self.heads)},_=>unreachable!()
 }}
}
