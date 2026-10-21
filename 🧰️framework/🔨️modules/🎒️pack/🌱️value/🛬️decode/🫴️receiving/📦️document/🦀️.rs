//! 📦️ Record materialization borrows either the authored Body or verified complete Document driver.
use super::*;
pub(super) enum RecordCursor{Body(RetainedRecordBodyCursor),Document(RetainedDocumentDriver)}
impl RecordCursor{
 pub(super) fn source_step_demands(&self)->Result<RetirementDemand,PackRefusal>{match self{Self::Body(cursor)if cursor.ingress_ready()=>Ok(RetirementDemand{copy_bytes:1,depth:1,..Default::default()}),Self::Body(cursor)=>cursor.source_step_demands(),Self::Document(cursor)=>cursor.normal_demands()}}
 pub(super) fn allocated_bytes(&self)->usize{match self{Self::Body(cursor)=>cursor.allocated_bytes(),Self::Document(cursor)=>cursor.allocated_bytes()}}
 pub(super) fn symbol_chars(&self,symbol:u64)->Result<usize,PackRefusal>{match self{Self::Body(cursor)=>cursor.symbol_chars(symbol),Self::Document(cursor)=>cursor.symbol_chars(symbol)}}
 pub(super) fn symbol_utf8_bytes(&self,symbol:u64)->Result<usize,PackRefusal>{match self{Self::Body(cursor)=>cursor.symbol_utf8_bytes(symbol),Self::Document(cursor)=>cursor.symbol_utf8_bytes(symbol)}}
 pub(super) fn symbol_char(&self,symbol:u64,index:usize)->Result<Option<char>,PackRefusal>{match self{Self::Body(cursor)=>cursor.symbol_char(symbol,index),Self::Document(cursor)=>cursor.symbol_char(symbol,index)}}
 pub(super) fn advance(&mut self,source:&[u8],position:&mut usize,sealed:&mut bool,maximum:usize)->Result<Option<Token>,PackRefusal>{match self{
  Self::Document(cursor)=>cursor.one(source,maximum),
  Self::Body(cursor)=>{if cursor.ingress_ready(){if *position<source.len(){cursor.admit_byte(*position as u64,source[*position]).map_err(|_|PackRefusal::from(invalid("original Record ingress handback refused")))?;*position+=1;}else if !*sealed{cursor.seal(*position as u64)?;*sealed=true;}else{return Err(PackRefusal::from(invalid("original Record lexer did not complete")))}}else if maximum>0{cursor.reserve_allocation(maximum).map_err(|error|error.fault)?;}else if let Some(RetainedRecordBodyToken::Value(token))=cursor.grant()?{return Ok(Some(token))}Ok(None)},
 }}
 pub(super) fn source_close_demands(&self)->Result<RetirementDemand,PackRefusal>{match self{Self::Body(cursor)=>cursor.source_close_demands(),Self::Document(cursor)=>cursor.close_demands(usize::MAX)}}
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,PackRefusal>{match self{Self::Document(cursor)=>cursor.close_step(grant),Self::Body(cursor)=>match cursor.close_step(grant.maximum_items,grant.maximum_release_bytes)?{semio_framework_pack_format_retained::RetainedPackCloseStep::Complete=>Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),semio_framework_pack_format_retained::RetainedPackCloseStep::Pending{released_items,released_bytes}=>Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()}))}}}
 pub(super) fn terminal_is_empty(&self)->bool{match self{Self::Body(cursor)=>cursor.terminal_is_empty(),Self::Document(cursor)=>cursor.terminal_is_empty()}}
}
