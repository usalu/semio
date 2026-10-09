//! ♻️ Composes the actual operation and history fields retained by typed command jobs.

use crate::app::{AppOperationContext,AlternativeView,CommandView,HistoryCommandFilter,HistoryView,InverseAction,MutationView};

semio_framework_value::artifact_retire_struct!(AppOperationContext{app_instance_id,parent_document_id,operation_id,generation,canonical_base_revision,retained,authoring_seed});
semio_framework_value::artifact_retire_struct!(HistoryView{columns,can_undo,can_redo,active_alternative_id,alternatives,current_checkpoint_id,commands,command_filter});
semio_framework_value::artifact_retire_struct!(AlternativeView{id,name,current,author,branched_at,edited});
semio_framework_value::artifact_retire_struct!(CommandView{seq,action_id,label,kind,timestamp,edit_id,child_edit_ids,transition_id,author,op_lines,op_count,applied,revertible,count,inverse,transaction,mutations});
semio_framework_value::artifact_retire_struct!(MutationView{mutation_id,position,op_index,label,worst,messages,superseded,withdrawn,editable,withdrawable,store});
semio_framework_value::artifact_retire_struct!(InverseAction{action_id,args});
semio_framework_value::artifact_retire_leaf!(HistoryCommandFilter);

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
