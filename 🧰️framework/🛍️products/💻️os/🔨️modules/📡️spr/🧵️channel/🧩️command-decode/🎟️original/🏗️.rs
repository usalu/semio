//! 🏗️ Validated original fields move into their actual command without allocation or a fallible tail.
use super::*;
impl OriginalCommandDecode{
 pub(super) fn build(&mut self)->AppCommand{
  let seq=self.seq.unwrap();match self.tag.unwrap(){
   0=>AppCommand::ConfigCommand{seq,command:self.bytes[0].take().unwrap()},
   1=>AppCommand::Command{seq,command:self.bytes[0].take().unwrap(),view_state:self.bytes[1].take().unwrap()},
   2=>AppCommand::CommandText{seq,line:self.texts[0].take().unwrap()},
   3=>AppCommand::ContextMenu{seq,request:self.bytes[0].take().unwrap()},
   4=>AppCommand::ArtifactCommand{seq,command:self.bytes[0].take().unwrap()},
   7=>AppCommand::ReadDocument{seq},8=>AppCommand::LoadConfig{seq,pack:self.bytes[0].take().unwrap(),spr:self.bytes[1].take().unwrap()},9=>AppCommand::ReadConfig{seq},
   10=>AppCommand::MediaIn{seq,port:self.texts[0].take().unwrap(),descriptor:self.bytes[0].take().unwrap(),data:self.bytes[1].take().unwrap()},
   11=>AppCommand::MediaOut{seq,port:self.texts[0].take().unwrap(),request:self.bytes[0].take().unwrap()},12=>AppCommand::MediaFingerprint{seq,port:self.texts[0].take().unwrap()},
   13=>AppCommand::PureCommand{seq,command:self.bytes[0].take().unwrap(),head:self.bytes[1].take().unwrap()},15=>AppCommand::ReadChildren{seq},16=>AppCommand::ReadHistory{seq},
   17=>AppCommand::TransactionPrepare{seq,txn_id:self.texts[0].take().unwrap(),mutation_id:self.texts[1].take().unwrap(),payload:self.bytes[0].take().unwrap(),prepared_ops:std::mem::take(&mut self.ops),origin:self.bytes[1].take().unwrap(),prepared_child_ops:self.bytes[2].take().unwrap()},
   18=>AppCommand::TransactionCommit{seq,txn_id:self.texts[0].take().unwrap()},19=>AppCommand::TransactionRollback{seq,txn_id:self.texts[0].take().unwrap()},20=>AppCommand::TransactionUndo{seq,group_id:self.texts[0].take().unwrap()},21=>AppCommand::TransactionRedo{seq,group_id:self.texts[0].take().unwrap()},
   27=>AppCommand::ReadConflicts{seq},29=>AppCommand::LocalInteractionQuery{seq,command:self.query.take().unwrap()},
   30=>AppCommand::LoadWindowConfig{seq,entry:WindowConfigPackEntry{window_id:self.texts[0].take().unwrap(),window_kind_id:self.texts[1].take().unwrap(),envelope_pack:self.bytes[0].take().unwrap()}},31=>AppCommand::ReadWindowConfigs{seq},
   32=>AppCommand::LoadDocumentArchive{seq,archive:std::mem::take(&mut self.archive)},43=>AppCommand::MergeDocumentArchive{seq,archive:std::mem::take(&mut self.archive)},33=>AppCommand::ReadDocumentArchive{seq},
   34=>AppCommand::PollDocumentArchiveLoad{seq,operation:self.numbers[0].unwrap()},35=>AppCommand::CancelDocumentArchiveLoad{seq,operation:self.numbers[0].unwrap()},36=>AppCommand::AcknowledgeDocumentArchiveLoad{seq,operation:self.numbers[0].unwrap()},
   37=>AppCommand::SubmitMediaExport{seq,port:self.texts[0].take().unwrap(),expected_parent_document_id:self.texts[1].take().unwrap(),expected_base_revision:self.numbers[0].unwrap(),checkpoint_id:self.texts[2].take()},
   38..=40=>{let handle=MediaExportHandleWire{app_instance_id:self.numbers[0].unwrap()as u32,parent_document_id:self.texts[0].take().unwrap(),operation_id:self.numbers[1].unwrap(),base_revision:self.numbers[2].unwrap(),generation:self.numbers[3].unwrap()};match self.tag.unwrap(){38=>AppCommand::PollMediaExport{seq,handle},39=>AppCommand::CancelMediaExport{seq,handle},_=>AppCommand::TakeMediaExportChunk{seq,handle}}},
   41=>AppCommand::ReadDocumentIdentity{seq},42=>AppCommand::ReadChildHeads{seq},_=>unreachable!("original field plan admitted its exact command kind")
  }
 }
}
