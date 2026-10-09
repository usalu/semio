//! 📦️ CommonMark logical records retain every typed block and inline independently of wire Markdown.
use crate::standards::v_commonmark::subsets::any::schema::snapshot::*;
#[path="🧱️block/🦀️.rs"]
mod block;
#[path="🧩️inline/🦀️.rs"]
mod inline;
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct Snapshot{schema:String,roots:Vec<u64>,blocks:Vec<block::Block>,inlines:Vec<inline::Inline>}
fn indices<'a,T>(values:&'a[T],pending:&mut std::collections::VecDeque<&'a T>,next:&mut u64)->Vec<u64>{values.iter().map(|value|{let key=*next;*next+=1;pending.push_back(value);key}).collect()}
fn children<T>(values:&mut[Option<T>],parent:Option<usize>,keys:Vec<u64>)->Result<Vec<T>,String>{keys.into_iter().map(|key|{let key=usize::try_from(key).map_err(|_|"CommonMark child index exceeds native domain")?;if key>=values.len()||parent.is_some_and(|parent|key<=parent){return Err("CommonMark forward child topology differs".into())}values[key].take().ok_or_else(||"CommonMark child has multiple owners".into())}).collect()}
impl From<&MdSnapshot> for Snapshot{
 fn from(value:&MdSnapshot)->Self{let mut pending=std::collections::VecDeque::new();let mut next=0;let roots=indices(&value.blocks,&mut pending,&mut next);let mut blocks=Vec::new();let mut inline_pending=std::collections::VecDeque::new();let mut next_inline=0;while let Some(value)=pending.pop_front(){blocks.push(block::project(value,&mut pending,&mut next,&mut inline_pending,&mut next_inline))}let mut inlines=Vec::new();while let Some(value)=inline_pending.pop_front(){inlines.push(inline::project(value,&mut inline_pending,&mut next_inline))}Self{schema:value.schema.clone(),roots,blocks,inlines}}
}
impl TryFrom<Snapshot> for MdSnapshot{
 type Error=String;
 fn try_from(value:Snapshot)->Result<Self,String>{let mut inlines=inline::reconstruct(value.inlines)?;let mut blocks=block::reconstruct(value.blocks,&mut inlines)?;let roots=children(&mut blocks,None,value.roots)?;if blocks.iter().any(Option::is_some)||inlines.iter().any(Option::is_some){return Err("CommonMark logical record contains unowned entities".into())}Ok(Self{schema:value.schema,blocks:roots})}
}

impl store::ArtifactPack for MdSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Snapshot::__dsl_spec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let inner=store::pack_rt::encode_document(&Snapshot::__dsl_spec(),&Snapshot::from(self).__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.md",store::semio_format::Component::Pack,1).map_err(|error|store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;if !envelope.matches_identity("stdio.md",store::semio_format::Component::Pack,1){return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"CommonMark pack identity differs")))}let(record,_)=store::pack_rt::decode_document(&inner,&Snapshot::__dsl_spec(),options)?;let snapshot=Snapshot::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?;snapshot.try_into().map_err(|error|store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error)))}
}

use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::NativeEncodeControl;
use semio_framework_value::NativeDecodeControl;

fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

fn add_rows(rows:&mut usize,count:usize,maximum:usize)->Result<(),ValueError>{*rows=rows.checked_add(count).filter(|value|*value<=maximum).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark domain rows exceed caller ceiling"))?;Ok(())}
fn enqueue<'a,T>(values:&'a[T],pending:&mut std::collections::VecDeque<&'a T>,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{let bytes=values.len().checked_mul(std::mem::size_of::<&T>()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"CommonMark borrowed frontier overflow"))?;control.charge(bytes)?;pending.try_reserve(values.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"CommonMark borrowed frontier allocation failed"))?;for value in values{pending.push_back(value);control.step()?;}Ok(())}
fn paid_indices<'a,T>(values:&'a[T],pending:&mut std::collections::VecDeque<&'a T>,next:&mut u64,control:&mut NativeEncodeControl<'_>)->Result<Vec<u64>,ValueError>{control.scoped_stage(|control|{control.begin_stage(values.len())?;let mut output=control.allocate_vec(values.len())?;let bytes=values.len().checked_mul(std::mem::size_of::<&T>()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"CommonMark borrowed frontier overflow"))?;control.charge(bytes)?;pending.try_reserve(values.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"CommonMark borrowed frontier allocation failed"))?;for value in values{let id=*next;*next=next.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark logical index overflow"))?;output.push(id);pending.push_back(value);control.step()?;}Ok(output)})}
fn paid_option(value:&Option<String>,control:&mut NativeEncodeControl<'_>)->Result<Option<String>,ValueError>{value.as_deref().map(|text|control.copy_text(text)).transpose()}

fn census(value:&MdSnapshot,maximum:usize,control:&mut NativeEncodeControl<'_>)->Result<(usize,usize,usize),ValueError>{
 let mut rows=0;add_rows(&mut rows,1,maximum)?;let mut pending=std::collections::VecDeque::new();let mut inline_pending=std::collections::VecDeque::new();control.begin_stage(0)?;enqueue(&value.blocks,&mut pending,control)?;let(mut blocks,mut inlines)=(0usize,0usize);
 while let Some(block)=pending.pop_front(){add_rows(&mut rows,3,maximum)?;blocks=blocks.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark block count overflow"))?;match block{MdBlock::Heading{inlines,..}|MdBlock::Paragraph{inlines}=>enqueue(inlines,&mut inline_pending,control)?,MdBlock::List{items,..}=>{add_rows(&mut rows,items.len(),maximum)?;control.scoped_stage(|control|{control.begin_stage(items.len())?;for item in items{control.scoped_stage(|control|{control.begin_stage(item.len())?;enqueue(item,&mut pending,control)})?;control.step()?;}Ok::<_,ValueError>(())})?;},MdBlock::BlockQuote{blocks}=>enqueue(blocks,&mut pending,control)?,MdBlock::CodeBlock{..}|MdBlock::ThematicBreak|MdBlock::HtmlBlock{..}=>{}}control.step()?;}
 while let Some(inline)=inline_pending.pop_front(){add_rows(&mut rows,3,maximum)?;inlines=inlines.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark inline count overflow"))?;match inline{MdInline::Emphasis{inlines}|MdInline::Strong{inlines}=>enqueue(inlines,&mut inline_pending,control)?,MdInline::Link{text,..}=>enqueue(text,&mut inline_pending,control)?,MdInline::Text{..}|MdInline::Code{..}|MdInline::Image{..}|MdInline::SoftBreak|MdInline::HardBreak|MdInline::HtmlInline{..}=>{}}control.step()?;}
 Ok((blocks,inlines,rows))
}

fn project_controlled(value:&MdSnapshot,maximum:usize,control:&mut NativeEncodeControl<'_>)->Result<Snapshot,ValueError>{
 let(block_count,inline_count,_)=control.scoped_stage(|control|census(value,maximum,control))?;let mut pending=std::collections::VecDeque::new();let mut inline_pending=std::collections::VecDeque::new();let(mut next,mut next_inline)=(0,0);let roots=paid_indices(&value.blocks,&mut pending,&mut next,control)?;let mut blocks=control.allocate_vec(block_count)?;let mut inlines=control.allocate_vec(inline_count)?;control.begin_stage(block_count.checked_add(inline_count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark logical work overflow"))?)?;
 while let Some(value)=pending.pop_front(){blocks.push(control.scoped_stage(|control|block::project_controlled(value,&mut pending,&mut next,&mut inline_pending,&mut next_inline,control))?);control.step()?;}
 while let Some(value)=inline_pending.pop_front(){inlines.push(control.scoped_stage(|control|inline::project_controlled(value,&mut inline_pending,&mut next_inline,control))?);control.step()?;}
 Ok(Snapshot{schema:control.copy_text(&value.schema)?,roots,blocks,inlines})
}

fn retirement_visit(){#[cfg(test)]retirement_test_visit();}
/// 🧵️ A continuation occupies only a popped node's spare vector slot and retains that slot's backing vector.
pub(crate) struct InlineContinuation{work:Vec<MdInline>,previous:*mut InlineContinuation}
/// 🧶️ Block continuations retain sibling nodes and link to a pending-item header in its own popped vector slot.
pub(crate) struct BlockContinuation{work:Vec<MdBlock>,items:*mut Vec<Vec<MdBlock>>,previous:*mut BlockContinuation}
const _:()=assert!(std::mem::size_of::<InlineContinuation>()<=std::mem::size_of::<MdInline>()&&std::mem::align_of::<InlineContinuation>()<=std::mem::align_of::<MdInline>());
const _:()=assert!(std::mem::size_of::<BlockContinuation>()<=std::mem::size_of::<MdBlock>()&&std::mem::align_of::<BlockContinuation>()<=std::mem::align_of::<MdBlock>());
const _:()=assert!(std::mem::size_of::<Vec<Vec<MdBlock>>>()<=std::mem::size_of::<Vec<MdBlock>>()&&std::mem::align_of::<Vec<Vec<MdBlock>>>()<=std::mem::align_of::<Vec<MdBlock>>());
/// 🪡️ Nonempty item storage has a spare slot from the item pop that yielded the active block vector.
fn save_items(items:&mut Vec<Vec<MdBlock>>)->*mut Vec<Vec<MdBlock>>{
 if items.capacity()==0{return std::ptr::null_mut()}
 assert!(items.len()<items.capacity());let slot=unsafe{items.as_mut_ptr().add(items.len())}.cast::<Vec<Vec<MdBlock>>>();let saved=std::mem::take(items);unsafe{std::ptr::write(slot,saved)}slot
}
fn drain_inlines(mut work:Vec<MdInline>){
 let mut previous=std::ptr::null_mut::<InlineContinuation>();
 loop{
  if let Some(value)=work.pop(){
   retirement_visit();let children=match value{MdInline::Emphasis{inlines}|MdInline::Strong{inlines}=>Some(inlines),MdInline::Link{text,..}=>Some(text),MdInline::Text{..}|MdInline::Code{..}|MdInline::Image{..}|MdInline::SoftBreak|MdInline::HardBreak|MdInline::HtmlInline{..}=>None};
   if let Some(children)=children{let slot=unsafe{work.as_mut_ptr().add(work.len())}.cast::<InlineContinuation>();let frame=InlineContinuation{work:std::mem::take(&mut work),previous};unsafe{std::ptr::write(slot,frame)}previous=slot;work=children;}
  }else if previous.is_null(){return}else{let frame=unsafe{std::ptr::read(previous)};previous=frame.previous;work=frame.work;}
 }
}
fn drain_blocks(mut work:Vec<MdBlock>){
 let mut items=Vec::new();let mut previous=std::ptr::null_mut::<BlockContinuation>();
 loop{
  if let Some(value)=work.pop(){
   retirement_visit();match value{
    MdBlock::Heading{inlines,..}|MdBlock::Paragraph{inlines}=>drain_inlines(inlines),
    MdBlock::BlockQuote{blocks}=>{let slot=unsafe{work.as_mut_ptr().add(work.len())}.cast::<BlockContinuation>();let frame=BlockContinuation{work:std::mem::take(&mut work),items:save_items(&mut items),previous};unsafe{std::ptr::write(slot,frame)}previous=slot;work=blocks;},
    MdBlock::List{items:children,..}=>{let slot=unsafe{work.as_mut_ptr().add(work.len())}.cast::<BlockContinuation>();let frame=BlockContinuation{work:std::mem::take(&mut work),items:save_items(&mut items),previous};unsafe{std::ptr::write(slot,frame)}previous=slot;items=children;},
    MdBlock::CodeBlock{..}|MdBlock::ThematicBreak|MdBlock::HtmlBlock{..}=>{}
   }
  }else if let Some(children)=items.pop(){retirement_visit();work=children;}
  else if previous.is_null(){return}else{let frame=unsafe{std::ptr::read(previous)};previous=frame.previous;work=frame.work;items=if frame.items.is_null(){Vec::new()}else{unsafe{std::ptr::read(frame.items)}};}
 }
}
pub(crate) fn retire_parts(blocks:Vec<MdBlock>,inlines:Vec<MdInline>){drain_blocks(blocks);drain_inlines(inlines)}
pub(crate) trait RetireNode{fn retire_nodes(values:Vec<Self>)where Self:Sized;fn retire_node(self);}
impl RetireNode for MdBlock{fn retire_nodes(values:Vec<Self>){retire_parts(values,Vec::new())}fn retire_node(self){match self{MdBlock::Heading{inlines,..}|MdBlock::Paragraph{inlines}=>retire_parts(Vec::new(),inlines),MdBlock::List{items,..}=>{for blocks in items{retire_parts(blocks,Vec::new())}},MdBlock::BlockQuote{blocks}=>retire_parts(blocks,Vec::new()),MdBlock::CodeBlock{..}|MdBlock::ThematicBreak|MdBlock::HtmlBlock{..}=>{}}}}
impl RetireNode for MdInline{fn retire_nodes(values:Vec<Self>){retire_parts(Vec::new(),values)}fn retire_node(self){match self{MdInline::Emphasis{inlines}|MdInline::Strong{inlines}=>retire_parts(Vec::new(),inlines),MdInline::Link{text,..}=>retire_parts(Vec::new(),text),MdInline::Text{..}|MdInline::Code{..}|MdInline::Image{..}|MdInline::SoftBreak|MdInline::HardBreak|MdInline::HtmlInline{..}=>{}}}}
pub(crate) struct OwnedNodes<T:RetireNode>(pub(crate) Vec<T>);
impl<T:RetireNode> std::ops::Deref for OwnedNodes<T>{type Target=Vec<T>;fn deref(&self)->&Vec<T>{&self.0}}
impl<T:RetireNode> std::ops::DerefMut for OwnedNodes<T>{fn deref_mut(&mut self)->&mut Vec<T>{&mut self.0}}
impl RetireNode for Vec<MdBlock>{fn retire_nodes(values:Vec<Self>){for blocks in values{retire_parts(blocks,Vec::new())}}fn retire_node(self){retire_parts(self,Vec::new())}}
impl<T:RetireNode> Drop for OwnedNodes<T>{fn drop(&mut self){T::retire_nodes(std::mem::take(&mut self.0))}}
pub(crate) struct OwnedSlots<T:RetireNode>(Vec<Option<T>>);
impl<T:RetireNode> Drop for OwnedSlots<T>{fn drop(&mut self){for value in std::mem::take(&mut self.0).into_iter().flatten(){value.retire_node()}}}
pub(crate) struct OwnedItems(Vec<Vec<MdBlock>>);
impl Drop for OwnedItems{fn drop(&mut self){for blocks in std::mem::take(&mut self.0){retire_parts(blocks,Vec::new())}}}
fn paid_children<T:RetireNode>(values:&mut[Option<T>],parent:Option<usize>,keys:Vec<u64>,control:&mut NativeDecodeControl<'_>)->Result<Vec<T>,ValueError>{control.scoped_stage(|control|{control.begin_stage(keys.len())?;let mut output=OwnedNodes(control.allocate_vec(keys.len())?);for key in keys{let key=usize::try_from(key).map_err(|_|invalid("CommonMark child index exceeds native domain"))?;if parent.is_some_and(|parent|key<=parent){return Err(invalid("CommonMark forward child topology differs"))}let value=values.get_mut(key).and_then(Option::take).ok_or_else(||invalid("CommonMark child is unknown or has multiple owners"))?;output.0.push(value);control.step()?;}Ok(std::mem::take(&mut output.0))})}
fn reconstruct_controlled(value:Snapshot,control:&mut NativeDecodeControl<'_>)->Result<MdSnapshot,ValueError>{
 #[cfg(test)]let _stage=crate::standards::v_commonmark::subsets::any::io::sqlite::snapshot::tests::MdReconstructTestScope::enter();
 let Snapshot{schema,roots,blocks,inlines}=value;let mut inlines=inline::reconstruct_controlled(inlines,control)?;let mut blocks=block::reconstruct_controlled(blocks,&mut inlines.0,control)?;let mut roots=OwnedNodes(paid_children(&mut blocks.0,None,roots,control)?);if blocks.0.iter().any(Option::is_some)||inlines.0.iter().any(Option::is_some){return Err(invalid("CommonMark logical record contains unowned entities"))}Ok(MdSnapshot{schema,blocks:std::mem::take(&mut roots.0)})
}

pub(crate) fn decode_owned(payload:&store::io_schema::IoPayload,control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<MdSnapshot,ValueError>{
 let limits=control.limits();if <MdSnapshot as store::ArtifactSqliteSnapshot>::SQLITE_SCHEMA.len()>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"CommonMark authored schema exceeds caller limit"))}store::decode_sqlite_snapshot_record_native(payload,"stdio.md",Snapshot::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {MdSnapshot::admit_sqlite_record(record,limits,native)?;let flat=Snapshot::__dsl_from_record_controlled(record,native)?;reconstruct_controlled(flat,native)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)
}
pub(crate) fn encode_owned(value:&MdSnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{
 value.admit_sqlite_values(control,store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative)?;
 let limits=control.limits();if <MdSnapshot as store::ArtifactSqliteSnapshot>::SQLITE_SCHEMA.len()>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"CommonMark authored schema exceeds caller limit"))}store::encode_sqlite_snapshot_record_native(encoding,"stdio.md",Snapshot::__dsl_spec_producer(),|native|{let flat=project_controlled(value,limits.max_rows,native)?;flat.__dsl_to_record_controlled(native)},control,native_owner)
}

pub(crate) fn retire_owned(value:MdSnapshot){retire_parts(value.blocks,Vec::new())}

#[cfg(test)]
pub(crate) fn duplicate_roots_payload_for_test(value:&MdSnapshot)->store::io_schema::IoPayload{let mut record=Snapshot::from(value).__dsl_to_record();let Some(semio_framework_dsl_record::FieldValue::List(roots))=record.fields.get_mut(&1)else{panic!("owned logical roots field must be a list")};assert_eq!(roots.len(),1);roots.push(semio_framework_dsl_record::FieldValue::UInt(0));let body=semio_framework_dsl_record::print(&record,&Snapshot::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.md",store::semio_format::Component::Dsl,1).unwrap();store::io_schema::IoPayload::Text(store::semio_format::wrap_text(&envelope,&body))}

#[cfg(test)]
std::thread_local!{
 static RETIRE_ALLOCATION_TRACKING:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};
 static RETIRE_ALLOCATION_REFUSAL:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};
 static RETIRE_ALLOCATION_ATTEMPTS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
 static RETIRE_LIVE_BYTES:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
 static RETIRE_VISITS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
}
#[cfg(test)]
fn retirement_test_visit(){RETIRE_VISITS.with(|count|count.set(count.get()+1));}
#[cfg(test)]
pub(crate) fn reset_retirement_visits_for_test(){RETIRE_VISITS.with(|count|count.set(0));}
#[cfg(test)]
pub(crate) fn retirement_visits_for_test()->usize{RETIRE_VISITS.with(std::cell::Cell::get)}
#[cfg(test)]
pub(crate) struct RetireObservedAllocator;
#[cfg(test)]
fn refuse_retirement_allocation()->bool{if RETIRE_ALLOCATION_REFUSAL.try_with(std::cell::Cell::get).unwrap_or(false){let _=RETIRE_ALLOCATION_ATTEMPTS.try_with(|count|count.set(count.get()+1));true}else{false}}
#[cfg(test)]
fn retirement_live_add(bytes:usize){if RETIRE_ALLOCATION_TRACKING.try_with(std::cell::Cell::get).unwrap_or(false){let _=RETIRE_LIVE_BYTES.try_with(|live|live.set(live.get()+bytes));}}
#[cfg(test)]
fn retirement_live_remove(bytes:usize){if RETIRE_ALLOCATION_TRACKING.try_with(std::cell::Cell::get).unwrap_or(false){let _=RETIRE_LIVE_BYTES.try_with(|live|live.set(live.get()-bytes));}}
#[cfg(test)]
unsafe impl std::alloc::GlobalAlloc for RetireObservedAllocator{
 unsafe fn alloc(&self,layout:std::alloc::Layout)->*mut u8{if refuse_retirement_allocation(){return std::ptr::null_mut()}let result=unsafe{std::alloc::GlobalAlloc::alloc(&std::alloc::System,layout)};if !result.is_null(){retirement_live_add(layout.size());}result}
 unsafe fn alloc_zeroed(&self,layout:std::alloc::Layout)->*mut u8{if refuse_retirement_allocation(){return std::ptr::null_mut()}let result=unsafe{std::alloc::GlobalAlloc::alloc_zeroed(&std::alloc::System,layout)};if !result.is_null(){retirement_live_add(layout.size());}result}
 unsafe fn realloc(&self,pointer:*mut u8,layout:std::alloc::Layout,size:usize)->*mut u8{if refuse_retirement_allocation(){return std::ptr::null_mut()}let result=unsafe{std::alloc::GlobalAlloc::realloc(&std::alloc::System,pointer,layout,size)};if !result.is_null(){retirement_live_remove(layout.size());retirement_live_add(size);}result}
 unsafe fn dealloc(&self,pointer:*mut u8,layout:std::alloc::Layout){retirement_live_remove(layout.size());unsafe{std::alloc::GlobalAlloc::dealloc(&std::alloc::System,pointer,layout)}}
}
#[cfg(test)]
#[global_allocator]
static RETIRE_OBSERVED_ALLOCATOR:RetireObservedAllocator=RetireObservedAllocator;

#[test]
fn sqlite_snapshot_md_retirement_releases_deep_and_wide_literals_when_all_allocations_refuse(){
 for case in 0..3{let(owned_bytes,remaining_bytes,attempts,visits,maximum_visits)=std::thread::Builder::new().stack_size(64*1024).spawn(move||{
  RETIRE_ALLOCATION_TRACKING.with(|flag|flag.set(true));let(blocks,inlines,maximum_visits)=match case{
   0=>{let mut inline=MdInline::Text{text:"retained text 世界".into()};for _ in 0..8192{inline=MdInline::Link{text:vec![inline],url:"literal URL 世界".into(),title:Some("literal title\0".into())};}(Vec::new(),vec![inline],2*(8192+1))},
   1=>{let mut block=MdBlock::CodeBlock{info:Some("literal info\0".into()),literal:"literal code 世界".into()};for depth in 0..8192{block=if depth%2==0{MdBlock::BlockQuote{blocks:vec![block,MdBlock::HtmlBlock{raw:"retained sibling 世界".into()}]}}else{MdBlock::List{ordered:true,start:Some(u32::MAX),tight:false,items:vec![Vec::new(),vec![block],Vec::new()]}};}(vec![block],Vec::new(),8*(8192+1)+4*(3*4096))},
   _=>{let items=(0..4096).map(|index|if index%2==0{Vec::new()}else{vec![MdBlock::Paragraph{inlines:vec![MdInline::Image{alt:"literal alt\0".into(),url:"literal image URL".into(),title:Some(String::new())},MdInline::Code{literal:"literal code 世界".into()},MdInline::HtmlInline{raw:"literal HTML\0".into()}]}]}).collect();(vec![MdBlock::List{ordered:false,start:None,tight:true,items}],Vec::new(),8*(1+4*2048)+4*4096)}
  };let mut blocks=blocks;if !inlines.is_empty(){blocks.push(MdBlock::Paragraph{inlines});}let snapshot=MdSnapshot{schema:"retained literal schema 世界\0".into(),blocks};let owned=RETIRE_LIVE_BYTES.with(std::cell::Cell::get);RETIRE_ALLOCATION_ATTEMPTS.with(|count|count.set(0));RETIRE_VISITS.with(|count|count.set(0));RETIRE_ALLOCATION_REFUSAL.with(|flag|flag.set(true));<MdSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(snapshot);RETIRE_ALLOCATION_REFUSAL.with(|flag|flag.set(false));let remaining=RETIRE_LIVE_BYTES.with(std::cell::Cell::get);RETIRE_ALLOCATION_TRACKING.with(|flag|flag.set(false));(owned,remaining,RETIRE_ALLOCATION_ATTEMPTS.with(std::cell::Cell::get),RETIRE_VISITS.with(std::cell::Cell::get),maximum_visits)
 }).unwrap().join().unwrap();assert!(owned_bytes>0);assert_eq!(remaining_bytes,0,"every owned string/vector must deallocate in case {case}");assert_eq!(attempts,0,"retirement must not request any allocation in case {case}");assert!(visits<=maximum_visits,"case {case} exceeds linear work frontier: {visits}>{maximum_visits}");}
}
