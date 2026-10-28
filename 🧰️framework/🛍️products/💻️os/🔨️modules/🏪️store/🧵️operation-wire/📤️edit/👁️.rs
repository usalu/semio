//! 👁️ The causal encoder reads actual retained Edit fields and original encoded operation pages.
use super::*;
use semio_framework_replication_causal::binary::{DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES,DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS,DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES,DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES};

/// 🪪️ The Store lends these unchanged original headers under its real publication fence.
#[derive(Clone,Copy)]
pub struct OriginalEditWireHeaders<'a>{pub document:&'a str,pub schema:&'a str,pub observed:Option<&'a str>}

pub(super) struct EditWireView<'a,M:Sync>{pub edit:&'a Edit<M>,pub wire:ArtifactOperationWire<M>,pub headers:OriginalEditWireHeaders<'a>,pub current:usize,pub forward:Option<&'a ArtifactOriginalOperationBytes<M>>,pub inverse:Option<&'a ArtifactOriginalOperationBytes<M>>}

impl<M:Sync> EditWireView<'_,M>{
    fn meta(&self,row:usize)->Result<&semio_framework_os_spr::MutationMeta,ValueError>{self.edit.mutation_meta.get(row).ok_or_else(invalid)}
    fn targets(&self,row:usize)->Result<Option<&dyn semio_framework_os_store_one_item_publication::lifted_prepared_operation_wire::ArtifactOperationTargets>,ValueError>{self.wire.targets(self.edit.forwards.get(row).ok_or_else(invalid)?)}
    fn original_text(&self,row:usize,field:CausalEnvelopeField)->Result<&str,ValueError>{
        let meta=self.meta(row)?;use CausalEnvelopeField as F;Ok(match field{
            F::MutationId=>&meta.mutation_id.as_ref().ok_or_else(invalid)?.0,F::DocumentId=>self.headers.document,
            F::Actor=>meta.author_id.as_ref().map(|actor|actor.0.as_str()).or_else(||self.edit.actor.as_deref()).ok_or_else(invalid)?,
            F::Dependency(index)=>&meta.dependencies.get(index).ok_or_else(invalid)?.0,F::Observed=>self.headers.observed.ok_or_else(invalid)?,
            F::DiffSchema|F::InverseSchema=>self.headers.schema,
            F::TransactionId=>&meta.transaction.as_ref().ok_or_else(invalid)?.id,F::TransactionTool=>&meta.transaction.as_ref().ok_or_else(invalid)?.tool,
            F::Verb=>self.edit.verb.as_deref().ok_or_else(invalid)?,F::Line=>self.edit.line.as_deref().ok_or_else(invalid)?,_=>return Err(invalid()),
        })
    }
}

impl<M:Sync> CausalEnvelopeWireSource for EditWireView<'_,M>{
    fn envelope_count(&self)->usize{self.edit.forwards.len()}
    fn shape(&self,row:usize)->Result<CausalEnvelopeShape,ValueError>{let meta=self.meta(row)?;let targets=self.targets(row)?.map_or(0,|targets|targets.len());if meta.dependencies.len()>DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES||targets>DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS{return Err(invalid())}Ok(CausalEnvelopeShape{dependencies:meta.dependencies.len(),targets,observed:self.headers.observed.is_some(),transaction:meta.transaction.is_some(),verb:self.edit.verb.is_some(),line:self.edit.line.is_some(),timestamp:meta.timestamp})}
    fn byte_len(&self,row:usize,field:CausalEnvelopeField)->Result<usize,ValueError>{
        use CausalEnvelopeField as F;let length=match field{
            F::DiffPayload=>{if row!=self.current{return Err(invalid())}self.forward.ok_or_else(invalid)?.payload_len()?},
            F::InversePayload=>{if row!=self.current{return Err(invalid())}self.inverse.map(|owner|owner.payload_len()).transpose()?.unwrap_or(0)},
            F::Target(index)=>self.targets(row)?.ok_or_else(invalid)?.segment_len(index)?,_=>self.original_text(row,field)?.len(),
        };let maximum=if matches!(field,F::DiffPayload|F::InversePayload){DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES}else{DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES};if length>maximum{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original causal field exceeds its authored backbone bound"))}Ok(length)
    }
    fn read_bytes(&self,row:usize,field:CausalEnvelopeField,offset:usize,output:&mut[u8])->Result<usize,ValueError>{
        use CausalEnvelopeField as F;match field{
            F::DiffPayload=>{if row!=self.current{return Err(invalid())}self.forward.ok_or_else(invalid)?.read_payload(offset,output)},
            F::InversePayload=>{if row!=self.current{return Err(invalid())}self.inverse.ok_or_else(invalid)?.read_payload(offset,output)},
            F::Target(index)=>self.targets(row)?.ok_or_else(invalid)?.read_segment(index,offset,output),
            _=>{let bytes=self.original_text(row,field)?.as_bytes().get(offset..).ok_or_else(invalid)?;let count=bytes.len().min(output.len());output[..count].copy_from_slice(&bytes[..count]);Ok(count)},
        }
    }
}
