//! ♻️ Exact controlled retirement of admitted DAG fact records.
use super::*;
use semio_framework_value::retirement::*;
semio_framework_value::artifact_retire_struct!(DagSelectionDomains{nodes,edges,handles});
semio_framework_value::artifact_retire_struct!(DagChannelRef{widget_id,port,direction});
semio_framework_value::artifact_retire_leaf!(DagChannelDirection);
semio_framework_value::artifact_retire_struct!(DagWireTypeRefusal{source,source_types,target,target_types});
semio_framework_value::artifact_retire_struct!(DagHoverFacts{channel,refusal});
semio_framework_value::artifact_retire_struct!(DagScreenHit{node_id,channel,minimap,minimap_viewport,port_insert,handle,widget});
semio_framework_value::artifact_retire_struct!(DagScreenHitJson{node,draggable,handle,direction,widget,minimap,minimap_viewport,port_insert});
semio_framework_value::artifact_retire_struct!(DagComputingProgress{active,stale});
impl RetireOwned for DagNodeEvaluationStatus{
    fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::Ok{}|Self::Queued{}|Self::Computing{}=>sequence(vec![]),Self::Error{message}=>sequence(vec![deferred(message)]),Self::Blocked{ports}=>sequence(vec![deferred(ports)])}}
    fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Ok{}|Self::Queued{}|Self::Computing{}=>sequence_birth_bytes(&[]),Self::Error{message}=>sequence_birth_bytes(&[deferred_birth_bytes_for(message)]),Self::Blocked{ports}=>sequence_birth_bytes(&[deferred_birth_bytes_for(ports)])}}
    fn controlled_retirement_supported()->bool{true}
}
