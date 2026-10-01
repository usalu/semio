//! 📦️ CommonMark logical records retain every typed block and inline independently of wire Markdown.
use super::*;
#[path="🧱️block/🦀️.rs"]
mod block;
#[path="🧩️inline/🦀️.rs"]
mod inline;
#[derive(dsl::DslRecord)]
struct Snapshot{schema:String,roots:Vec<u64>,blocks:Vec<block::Block>,inlines:Vec<inline::Inline>}
fn indices<'a,T>(values:&'a[T],pending:&mut std::collections::VecDeque<&'a T>,next:&mut u64)->Vec<u64>{values.iter().map(|value|{let key=*next;*next+=1;pending.push_back(value);key}).collect()}
fn children<T>(values:&mut[Option<T>],parent:Option<usize>,keys:Vec<u64>)->Result<Vec<T>,String>{keys.into_iter().map(|key|{let key=usize::try_from(key).map_err(|_|"CommonMark child index exceeds native domain")?;if key>=values.len()||parent.is_some_and(|parent|key<=parent){return Err("CommonMark forward child topology differs".into())}values[key].take().ok_or_else(||"CommonMark child has multiple owners".into())}).collect()}
impl From<&MdSnapshot> for Snapshot{
 fn from(value:&MdSnapshot)->Self{let mut pending=std::collections::VecDeque::new();let mut next=0;let roots=indices(&value.blocks,&mut pending,&mut next);let mut blocks=Vec::new();let mut inline_pending=std::collections::VecDeque::new();let mut next_inline=0;while let Some(value)=pending.pop_front(){blocks.push(block::project(value,&mut pending,&mut next,&mut inline_pending,&mut next_inline))}let mut inlines=Vec::new();while let Some(value)=inline_pending.pop_front(){inlines.push(inline::project(value,&mut inline_pending,&mut next_inline))}Self{schema:value.schema.clone(),roots,blocks,inlines}}
}
impl TryFrom<Snapshot> for MdSnapshot{
 type Error=String;
 fn try_from(value:Snapshot)->Result<Self,String>{let mut inlines=inline::reconstruct(value.inlines)?;let mut blocks=block::reconstruct(value.blocks,&mut inlines)?;let roots=children(&mut blocks,None,value.roots)?;if blocks.iter().any(Option::is_some)||inlines.iter().any(Option::is_some){return Err("CommonMark logical record contains unowned entities".into())}Ok(Self{schema:value.schema,blocks:roots})}
}
impl store::ArtifactDsl for MdSnapshot{
 const EXTENSION:&'static str="md";
 fn envelope_id()->&'static str{"stdio.md"}
 fn parse_dsl(text:&str)->Result<Self,store::TextError>{let body=match store::semio_format::split_text_preamble(text){Ok((envelope,body))=>{if !envelope.matches_identity("stdio.md",store::semio_format::Component::Dsl,1){return Err(dsl::__rt::field_error("CommonMark text identity differs"))}body},Err(_)=>text};let record=dsl::parse(body,&Snapshot::__dsl_spec(),&dsl::ParseOptions::default())?;let snapshot=Snapshot::__dsl_from_record(&record)?;snapshot.try_into().map_err(dsl::__rt::field_error)}
 fn print_dsl(&self)->String{let body=dsl::print(&Snapshot::from(self).__dsl_to_record(),&Snapshot::__dsl_spec(),dsl::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.md",store::semio_format::Component::Dsl,1).expect("valid CommonMark identity");store::semio_format::wrap_text(&envelope,&body)}
}
impl store::ArtifactPack for MdSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn record_spec()->Option<dsl::RecordSpec>{Some(Snapshot::__dsl_spec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let inner=store::pack_rt::encode_document(&Snapshot::__dsl_spec(),&Snapshot::from(self).__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.md",store::semio_format::Component::Pack,1).map_err(|error|store::PackError::Schema(error.to_string()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::Schema(error.to_string()))?;if !envelope.matches_identity("stdio.md",store::semio_format::Component::Pack,1){return Err(store::PackError::Schema("CommonMark pack identity differs".into()))}let(record,_)=store::pack_rt::decode_document(&inner,&Snapshot::__dsl_spec(),options)?;let snapshot=Snapshot::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?;snapshot.try_into().map_err(store::PackError::Schema)}
}
