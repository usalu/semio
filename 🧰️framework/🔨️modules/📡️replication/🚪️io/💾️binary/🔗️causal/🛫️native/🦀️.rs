//! 🛫️ Bounded causal-wire encoding borrows original fields and retains offsets alone.
use semio_framework_value::ByteReadSource;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_framework_replication_ids::HybridLogicalTimestamp;

/// 🏷️ Original envelope fields retain their protocol declaration order.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CausalEnvelopeField{MutationId,DocumentId,Actor,Dependency(usize),Observed,Target(usize),DiffSchema,DiffPayload,InverseSchema,InversePayload,TransactionId,TransactionTool,Verb,Line}

/// 📋️ Constant-size original row metadata selects each variable field independently.
#[derive(Clone,Copy)]
pub struct CausalEnvelopeShape{pub dependencies:usize,pub targets:usize,pub observed:bool,pub transaction:bool,pub verb:bool,pub line:bool,pub timestamp:HybridLogicalTimestamp}

/// 👁️ The embedding owner retains its actual source while the byte cursor reborrows bounded fields.
pub trait CausalEnvelopeWireSource{
    fn envelope_count(&self)->usize;
    fn shape(&self,row:usize)->Result<CausalEnvelopeShape,ValueError>;
    fn byte_len(&self,row:usize,field:CausalEnvelopeField)->Result<usize,ValueError>;
    fn read_bytes(&self,row:usize,field:CausalEnvelopeField,offset:usize,output:&mut[u8])->Result<usize,ValueError>;
}

/// 🧭️ One cursor advances one header, field fragment or row transition per paid action.
#[derive(Default)]
pub struct CausalEnvelopeWireCursor{row:usize,field:u8,index:usize,offset:usize,pending:[u8;10],pending_length:usize,pending_offset:usize,payload:bool,header:bool,complete:bool}

enum Selected{Number(u64),Bytes(CausalEnvelopeField),Skip,Next,Complete}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original causal wire field or extent differs")}

/// 🧾️ Every byte and metadata action belongs to the embedding original issuer.
pub struct CausalEnvelopeWireStep{pub written_bytes:usize,pub progress:RetainedCloneProgress,pub complete:bool}

impl CausalEnvelopeWireCursor{
    /// 📏️ Quotes a bounded field fragment plus actual cursor and shape metadata.
    pub fn demand(&self,body:usize)->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<Self>()+std::mem::size_of::<CausalEnvelopeShape>()+body.min(64),depth:1,..Default::default()}}
    pub fn is_complete(&self)->bool{self.complete}
    pub fn row_index(&self)->usize{self.row}

    fn selected(&self,source:&dyn CausalEnvelopeWireSource)->Result<Selected,ValueError>{
        if !self.header{return Ok(Selected::Number(source.envelope_count().try_into().map_err(|_|invalid())?))}
        if self.row==source.envelope_count(){return Ok(Selected::Complete)}
        if self.row>source.envelope_count(){return Err(invalid())}let s=source.shape(self.row)?;
        use CausalEnvelopeField as F;
        Ok(match self.field{
            0=>Selected::Bytes(F::MutationId),1=>Selected::Bytes(F::DocumentId),2=>Selected::Bytes(F::Actor),
            3=>Selected::Number(s.dependencies.try_into().map_err(|_|invalid())?),
            4 if self.index<s.dependencies=>Selected::Bytes(F::Dependency(self.index)),4=>Selected::Skip,
            5=>Selected::Number(u64::from(s.observed)),6 if s.observed=>Selected::Bytes(F::Observed),6=>Selected::Skip,
            7=>Selected::Number(s.targets.try_into().map_err(|_|invalid())?),
            8 if self.index<s.targets=>Selected::Bytes(F::Target(self.index)),8=>Selected::Skip,
            9=>Selected::Bytes(F::DiffSchema),10=>Selected::Bytes(F::DiffPayload),11=>Selected::Bytes(F::InverseSchema),12=>Selected::Bytes(F::InversePayload),
            13=>Selected::Number(s.timestamp.actor),14=>Selected::Number(s.timestamp.physical_ms),15=>Selected::Number(s.timestamp.logical),
            16=>Selected::Number(u64::from(s.transaction)|u64::from(s.verb)<<1|u64::from(s.line)<<2),
            17 if s.transaction=>Selected::Bytes(F::TransactionId),18 if s.transaction=>Selected::Bytes(F::TransactionTool),17|18=>Selected::Skip,
            19 if s.verb=>Selected::Bytes(F::Verb),19=>Selected::Skip,20 if s.line=>Selected::Bytes(F::Line),20=>Selected::Skip,
            21=>Selected::Next,_=>return Err(invalid()),
        })
    }

    fn next(&mut self){
        self.pending_length=0;self.pending_offset=0;self.payload=false;self.offset=0;
        if !self.header{self.header=true;return}
        if matches!(self.field,4|8){self.index+=1}else{self.field+=1;self.index=0}
    }

    fn number(&mut self,mut value:u64,output:&mut[u8])->usize{
        if self.pending_length==0{loop{let byte=(value&127)as u8;value>>=7;self.pending[self.pending_length]=byte|if value==0{0}else{128};self.pending_length+=1;if value==0{break}}}
        let count=(self.pending_length-self.pending_offset).min(output.len());output[..count].copy_from_slice(&self.pending[self.pending_offset..self.pending_offset+count]);self.pending_offset+=count;count
    }

    /// ✍️ Encodes the existing causal format without building an owned envelope or whole-value mirror.
    pub fn advance(&mut self,source:&dyn CausalEnvelopeWireSource,output:&mut[u8],grant:RetainedCloneGrant)->Result<CausalEnvelopeWireStep,ValueError>{
        if self.complete{return Ok(CausalEnvelopeWireStep{written_bytes:0,progress:Default::default(),complete:true})}
        let maximum=output.len().min(64);let d=self.demand(maximum);if grant.maximum_items==0||grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_depth<d.depth{return Ok(CausalEnvelopeWireStep{written_bytes:0,progress:Default::default(),complete:false})}
        let selected=self.selected(source)?;let mut written=0;
        match selected{
            Selected::Complete=>self.complete=true,
            Selected::Skip=>{self.field+=1;self.index=0;self.pending_length=0;self.pending_offset=0;self.payload=false;self.offset=0;},
            Selected::Next=>{self.row+=1;self.field=0;self.index=0;},
            Selected::Number(value)=>{if maximum!=0{written=self.number(value,&mut output[..maximum]);if self.pending_offset==self.pending_length{self.next();}}},
            Selected::Bytes(field)=>{
                let length=source.byte_len(self.row,field)?;
                if !self.payload{if maximum!=0{written=self.number(length.try_into().map_err(|_|invalid())?,&mut output[..maximum]);if self.pending_offset==self.pending_length{self.pending_length=0;self.pending_offset=0;self.payload=true;}}}
                else if self.offset==length{self.next();}
                else if self.offset>length{return Err(invalid())}
                else if maximum!=0{let count=(length-self.offset).min(maximum);written=source.read_bytes(self.row,field,self.offset,&mut output[..count])?;if written==0||written>count{return Err(invalid())}self.offset+=written;if self.offset==length{self.next();}}
            },
        }
        if self.header&&self.row==source.envelope_count(){self.complete=true;}
        Ok(CausalEnvelopeWireStep{written_bytes:written,progress:RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Self>()+std::mem::size_of::<CausalEnvelopeShape>()+written,..Default::default()},complete:self.complete})
    }
}

impl CausalEnvelopeWireSource for Vec<crate::MutationEnvelope>{
    fn envelope_count(&self)->usize{self.len()}
    fn shape(&self,row:usize)->Result<CausalEnvelopeShape,ValueError>{let row=self.get(row).ok_or_else(invalid)?;Ok(CausalEnvelopeShape{dependencies:row.dependencies.len(),targets:row.target.len(),observed:row.observed.is_some(),transaction:row.transaction.is_some(),verb:row.verb.is_some(),line:row.line.is_some(),timestamp:row.timestamp})}
    fn byte_len(&self,row:usize,field:CausalEnvelopeField)->Result<usize,ValueError>{Ok(self.original_field(row,field)?.byte_len())}
    fn read_bytes(&self,row:usize,field:CausalEnvelopeField,offset:usize,output:&mut[u8])->Result<usize,ValueError>{let bytes=self.original_field(row,field)?;if offset>bytes.byte_len(){return Err(invalid())}Ok(bytes.read_bytes(offset,output))}
}

enum OriginalFieldBytes<'a>{Contiguous(&'a[u8]),Paged(&'a crate::CausalPayload)}
impl ByteReadSource for OriginalFieldBytes<'_>{fn byte_len(&self)->usize{match self{Self::Contiguous(bytes)=>bytes.len(),Self::Paged(bytes)=>bytes.len()}}fn byte_at(&self,index:usize)->Option<u8>{match self{Self::Contiguous(bytes)=>bytes.get(index).copied(),Self::Paged(bytes)=>bytes.byte_at(index)}}fn read_bytes(&self,offset:usize,output:&mut[u8])->usize{match self{Self::Contiguous(bytes)=>bytes.read_bytes(offset,output),Self::Paged(bytes)=>bytes.read_bytes(offset,output)}}}

trait OriginalFields{fn original_field(&self,row:usize,field:CausalEnvelopeField)->Result<OriginalFieldBytes<'_>,ValueError>;}
impl OriginalFields for [crate::MutationEnvelope]{
    fn original_field(&self,row:usize,field:CausalEnvelopeField)->Result<OriginalFieldBytes<'_>,ValueError>{let row=self.get(row).ok_or_else(invalid)?;use CausalEnvelopeField as F;Ok(OriginalFieldBytes::Contiguous(match field{
        F::MutationId=>row.mutation_id.0.as_bytes(),F::DocumentId=>row.document_id.0.as_bytes(),F::Actor=>row.actor.0.as_str().as_bytes(),F::Dependency(index)=>row.dependencies.get(index).ok_or_else(invalid)?.0.as_bytes(),F::Observed=>row.observed.as_ref().ok_or_else(invalid)?.0.as_bytes(),F::Target(index)=>row.target.get(index).ok_or_else(invalid)?.as_bytes(),F::DiffSchema=>row.diff.schema.0.as_bytes(),F::DiffPayload=>return Ok(OriginalFieldBytes::Paged(&row.diff.payload)),F::InverseSchema=>row.inverse.schema.0.as_bytes(),F::InversePayload=>return Ok(OriginalFieldBytes::Paged(&row.inverse.payload)),F::TransactionId=>row.transaction.as_ref().ok_or_else(invalid)?.id.as_bytes(),F::TransactionTool=>row.transaction.as_ref().ok_or_else(invalid)?.tool.as_bytes(),F::Verb=>row.verb.as_ref().ok_or_else(invalid)?.as_bytes(),F::Line=>row.line.as_ref().ok_or_else(invalid)?.as_bytes(),
    }))}
}

#[cfg(test)]
#[path="🧪️.rs"]
mod tests;
