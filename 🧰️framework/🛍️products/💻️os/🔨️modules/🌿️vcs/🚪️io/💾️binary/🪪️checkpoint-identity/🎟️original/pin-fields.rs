//! 🧷️ Original typed composition pins lend canonical URI pieces to the domain-neutral history codec.
use super::*;
use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceUriView;
/// 🪢️ This stack view carries no ownership or execution authority; its enclosing publisher retains the actual roster.
pub struct OriginalCompositionPinFields<'a>{pins:&'a[CompositionPin]}
impl<'a> OriginalCompositionPinFields<'a>{
 /// 👁️ Borrows the actual ordered typed pin source without building protocol mirrors.
 pub fn new(pins:&'a[CompositionPin])->Self{Self{pins}}
}
impl protocol::HistoryTransitionPinSource for OriginalCompositionPinFields<'_>{
 fn source_identity(&self)->(usize,usize){(self.pins.as_ptr()as usize,self.pins.len())}
 fn len(&self)->usize{self.pins.len()}
 fn uri_part(&self,row:usize,part:usize)->Option<&str>{self.pins.get(row).and_then(|pin|ArtifactReferenceUriView::new(&pin.child_ref).part(part))}
 fn checkpoint_id(&self,row:usize)->Option<&str>{self.pins.get(row).map(|pin|pin.checkpoint_id.as_str())}
}

#[cfg(test)]
mod tests{
 use super::*;
 use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText;
 #[test]
 fn original_typed_pins_stream_repin_uri_fields_without_protocol_copies(){let pins=vec![CompositionPin{child_ref:semio_framework_artifact_reference::ArtifactRef{artifact_id:"energy-λ".repeat(8197),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.energy.model".into(),standard:"2026/10".into(),subset:"de".into()}},checkpoint_id:"child-checkpoint".into()}];let expected=protocol::encode_history_transition(&protocol::HistoryTransition::Repin{checkpoint_id:"checkpoint".into(),pinned_checkpoint_id:"pinned".into(),pins:pins.iter().map(|pin|protocol::TransitionPin{child_uri:pin.child_ref.to_uri(),checkpoint_id:pin.checkpoint_id.clone()}).collect()});for width in [1,7,64,4096]{let mut cursor=protocol::HistoryTransitionWireCursor::default();let mut output=Vec::new();let mut bytes=[0;4096];for index in 0..262144{if cursor.is_finished(){break}let(n,h)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.write_repin_view(&protocol::HistoryTransitionRepinView{checkpoint_id:"checkpoint",pinned_checkpoint_id:"pinned",pins:&OriginalCompositionPinFields::new(&pins)},31,&mut bytes[..width]).unwrap());assert_eq!((h.requested_bytes,h.released_bytes),(0,0));output.extend_from_slice(&bytes[..n]);assert!(index<262143);}assert_eq!(output,expected);}let fields=OriginalCompositionPinFields::new(&pins);let view=protocol::HistoryTransitionRepinView{checkpoint_id:"checkpoint",pinned_checkpoint_id:"pinned",pins:&fields};let mut cursor=protocol::HistoryTransitionWireCursor::default();let mut output=[37;1];assert_eq!(cursor.write_repin_view(&view,31,&mut output).unwrap(),1);output[0]=37;let foreign=pins.clone();for(epoch,fields)in [(32,OriginalCompositionPinFields::new(&pins)),(31,OriginalCompositionPinFields::new(&foreign))]{assert!(cursor.write_repin_view(&protocol::HistoryTransitionRepinView{pins:&fields,..view},epoch,&mut output).is_err());assert_eq!(output,[37]);}}
}
