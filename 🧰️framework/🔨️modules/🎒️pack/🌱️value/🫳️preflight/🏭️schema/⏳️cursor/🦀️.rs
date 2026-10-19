//! 🧵️ Canonical borrowed schema discovery and quotient hashing retain every work frontier.
use super::{R,H,K};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError};
use semio_framework_value_edit::{sort::HeapSortCursor, sort::HeapSortWork};
use std::{cmp::Ordering,mem::size_of};
use semio_framework_value_persistent::list::PagedList;
type List<T> = PagedList<T,{usize::MAX}>;
const SPAN:usize=256;
#[derive(Clone,Copy)]
enum Token{Byte(u8),Number(u64),Text(&'static str),Edge{target:usize,local:usize}}
struct Record{spec:R,address:Option<usize>,tokens:List<Token>,edges:List<usize>,order:List<usize>,digest:[u8;32],class:usize,old:usize,ordinal:usize}
impl Record{fn new(spec:R,address:Option<usize>)->Self{Self{spec,address,tokens:List::empty(),edges:List::empty(),order:List::empty(),digest:[0;32],class:0,old:0,ordinal:usize::MAX}}}
#[derive(Clone,Copy)]
struct Frame{shape:H,depth:usize,stage:u8,chosen:Option<usize>}
#[derive(Clone,Copy,Default)]
struct Sort{cursor:Option<HeapSortCursor>,offset:usize,position:usize,initialized:bool,reset:bool}
#[derive(Clone,Copy)]
struct Edge{make:fn()->R,scan:usize,target:usize,stage:u8}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Birth,Build,InitialHash,ClassifyBase,CopyOld,RefineHash,ClassifyRefined,BfsBirth,Bfs,FinalHash,Complete,Closing,Closed}
#[derive(Clone,Copy)]
enum Owner{None,Records,Tokens,Edges,Frames,Queue,Hasher,FieldOrder,Variants}
/// 🧾️ One original backing or semantic work event, with no hidden output ownership.
#[derive(Default,Debug)]
pub struct BorrowedSchemaHashStep{pub progress:RetainedCloneProgress,pub complete:bool}
/// 🕸️ Paged static metadata locators carry discovery, exact refinement and canonical BFS between turns.
pub struct BorrowedSchemaHashCursor{
 records:List<Record>,frames:List<Frame>,queue:List<usize>,variants:List<usize>,phase:Phase,root:Option<R>,node:usize,
 field_stage:u8,field_scan:usize,field_sort:Option<HeapSortCursor>,field:usize,
 sort:Sort,edge:Option<Edge>,hasher:List<semio_framework_hash::Hasher>,token:usize,offset:usize,
 candidate:usize,equal:usize,equal_offset:usize,count:usize,previous:usize,refine_stage:u8,
 bfs_node:usize,bfs_edge:usize,pending_queue:Option<usize>,digest:Option<[u8;32]>,
}
impl Default for BorrowedSchemaHashCursor{fn default()->Self{Self{records:List::empty(),frames:List::empty(),queue:List::empty(),variants:List::empty(),phase:Phase::Birth,root:None,node:0,field_stage:7,field_scan:0,field_sort:None,field:0,sort:Sort::default(),edge:None,hasher:List::empty(),token:0,offset:0,candidate:0,equal:0,equal_offset:0,count:0,previous:0,refine_stage:0,bfs_node:0,bfs_edge:0,pending_queue:None,digest:None}}}
fn invariant(reason:&'static str)->ValueError{ValueError::literal(K::InvariantViolated,reason)}
fn mapped<T>(result:Result<T,semio_framework_value_persistent::list::PagedListError>)->Result<T,ValueError>{result.map_err(|error|invariant(error.reason))}
fn pushed<T>(result:Result<(),T>)->Result<(),ValueError>{result.map_err(|_|invariant("retained schema original reserved slot is absent"))}
fn varint(mut value:u64)->([u8;10],usize){let mut bytes=[0;10];let mut n=0;loop{bytes[n]=(value&127)as u8;value>>=7;if value!=0{bytes[n]|=128}n+=1;if value==0{return(bytes,n)}}}
fn scalar_tag(shape:H)->u8{match shape{H::Bool=>1,H::Int=>2,H::UInt=>3,H::Float=>4,H::Text=>5,H::Bytes64=>6,H::Enum(_)=>7,H::Tuple(_,_)=>8,H::List(_)=>9,H::Record(_)=>10,H::Block(_)=>11,H::Statements(_)=>12,H::Map(_)=>13,H::Value=>14,H::Table(_)=>15,H::Wire=>16,H::Quantity(_)=>17,H::Angle(_)=>18,H::Ref(_)=>19,H::Coord(_)=>20,H::Dir=>21,H::Dim(_)=>22,H::Range=>23,H::Count=>24,H::Expr=>25,H::Embed(_)=>26,H::EmbedFrom(_)=>27}}
fn sort_length(shape:H)->usize{match shape{H::Enum(items)=>items.len(),H::Statements(items)=>items.len(),_=>0}}
fn sort_key(shape:H,index:usize)->(u32,&'static str){match shape{H::Enum(items)=>(items[index].1,items[index].0),H::Statements(items)=>(0,items[index].0),_=>(0,"")}}
fn comparison(shape:H,a:usize,b:usize,offset:usize)->(Option<Ordering>,usize){let(ao,at)=sort_key(shape,a);let(bo,bt)=sort_key(shape,b);if ao!=bo{return(Some(ao.cmp(&bo)),0)}let n=at.len().min(bt.len()).saturating_sub(offset).min(SPAN);let order=at.as_bytes()[offset..offset+n].cmp(&bt.as_bytes()[offset..offset+n]);if order!=Ordering::Equal{return(Some(order),n*2)}if offset+n==at.len().min(bt.len()){let order=at.len().cmp(&bt.len());return(Some(if order==Ordering::Equal{a.cmp(&b)}else{order}),n*2)}(None,n*2)}
impl BorrowedSchemaHashCursor{
 #[cfg(test)]
 pub(super) fn stage_for_test(&self)->u8{self.phase as u8}
 pub fn digest(&self)->Option<[u8;32]>{self.digest}
 pub fn terminal_is_empty(&self)->bool{self.phase==Phase::Closed&&self.records.terminal_is_empty()&&self.frames.terminal_is_empty()&&self.queue.terminal_is_empty()&&self.hasher.terminal_is_empty()&&self.variants.terminal_is_empty()}
 fn record(&self)->&Record{self.records.get(self.node).expect("retained schema node")}
 fn owner(&self)->Owner{match self.phase{
  Phase::Birth=>Owner::Records,
  Phase::InitialHash|Phase::FinalHash|Phase::RefineHash if self.hasher.is_empty()=>Owner::Hasher,
  Phase::Build=>{
   if let Some(edge)=self.edge{return match edge.stage{1=>Owner::Records,2=>Owner::Tokens,3=>Owner::Edges,_=>Owner::None}}
   if let Some(frame)=self.frames.last(){if matches!(frame.shape,H::Enum(_)|H::Statements(_))&&frame.stage==2&&!self.sort.reset&&!self.sort.initialized&&self.variants.len()<sort_length(frame.shape){return Owner::Variants}return match(frame.shape,frame.stage){
    (_,0)=>Owner::Tokens,(H::Enum(_)|H::Statements(_),1|3)=>Owner::Tokens,(H::Enum(_),4)=>Owner::Tokens,
    (H::Tuple(_,_)|H::List(_)|H::Block(_)|H::Map(_),1)=>Owner::Frames,
    (H::Tuple(_,Some(_)),2|3)|(H::Tuple(_,None),2)=>Owner::Tokens,
    (H::Quantity(_)|H::Angle(_)|H::Ref(_)|H::Embed(_)|H::EmbedFrom(_)|H::Coord(_)|H::Dim(_),1)=>Owner::Tokens,_=>Owner::None}}
   match self.field_stage{0|2|3|4=>Owner::Tokens,5=>Owner::Frames,7 if self.record().order.len()<self.record().spec.fields.len()=>Owner::FieldOrder,_=>Owner::None}
  },Phase::BfsBirth=>Owner::Queue,Phase::Bfs if self.pending_queue.is_some()=>Owner::Queue,_=>Owner::None}}
 fn list_demand<T>(list:&List<T>)->Result<(usize,usize,usize),ValueError>{if list.len()<list.capacity(){Ok((0,size_of::<T>(),mapped(list.next_pop_depth_demand())?.max(3)))}else{Ok((mapped(list.next_allocation_bytes())?,mapped(list.next_reserve_copy_byte_demand())?,mapped(list.next_reserve_depth_demand())?))}}
 fn hash_copy(&self)->usize{let Some(token)=self.record().tokens.get(self.token).copied()else{return if self.phase==Phase::InitialHash||self.bfs_node+1==self.queue.len(){32}else{0}};match token{Token::Byte(_)=>1,Token::Number(n)=>varint(n).1,Token::Edge{target,local}=>varint(if self.phase==Phase::FinalHash{self.records.get(self.records.get(target).unwrap().class).unwrap().ordinal}else{local}as u64).1,Token::Text(text)=>if self.offset==0{varint(text.len()as u64).1}else{text.len().saturating_sub(self.offset-1).min(SPAN)}}}
 fn equality_copy(&self)->usize{if self.phase==Phase::ClassifyRefined||self.node==self.records.len()||self.candidate>=self.node{return 0}let record=self.record();let other=self.records.get(self.candidate).unwrap();if record.digest!=other.digest||record.tokens.len()!=other.tokens.len(){return 0}let Some(a)=record.tokens.get(self.equal).copied()else{return 0};let Some(b)=other.tokens.get(self.equal).copied()else{return 0};if let(Token::Text(a),Token::Text(b))=(a,b){if a.len()!=b.len(){0}else{a.len().saturating_sub(self.equal_offset).min(SPAN)*2}}else{0}}
 fn semantic_copy(&self)->usize{match self.phase{
  Phase::Build if self.frames.last().is_some_and(|frame|matches!(frame.shape,H::Enum(_)|H::Statements(_))&&frame.stage==2)=>{
   let shape=self.frames.last().unwrap().shape;if self.sort.reset{usize::from(!self.variants.is_empty())*size_of::<usize>()}else{match self.sort.cursor.map(|cursor|cursor.next()){Some(HeapSortWork::Compare{left,right})=>{let(a,at)=sort_key(shape,*self.variants.get(left).unwrap());let(b,bt)=sort_key(shape,*self.variants.get(right).unwrap());if a==b{at.len().min(bt.len()).saturating_sub(self.sort.offset).min(SPAN)*2}else{0}},Some(HeapSortWork::Swap{..})=>2*size_of::<usize>(),_=>0}}},
  Phase::Build if self.field_stage==8&&self.frames.is_empty()&&self.edge.is_none()=>if matches!(self.field_sort.map(|cursor|cursor.next()),Some(HeapSortWork::Swap{..})){2*size_of::<usize>()}else{0},
  Phase::InitialHash|Phase::FinalHash if !self.hasher.is_empty()=>self.hash_copy(),
  Phase::InitialHash|Phase::FinalHash=>if self.phase==Phase::FinalHash{varint(self.queue.len()as u64).1}else{0},
  Phase::RefineHash=>if self.hasher.is_empty(){0}else{match self.refine_stage{0=>varint(self.record().old as u64).1,1=>varint(self.record().edges.len()as u64).1,2=>self.record().edges.get(self.token).map(|target|varint(self.records.get(*target).unwrap().old as u64).1).unwrap_or(0),_=>32}},
  Phase::ClassifyBase|Phase::ClassifyRefined=>self.equality_copy(),_=>0}}
 fn demands(&self)->Result<(usize,usize,usize),ValueError>{let owner=self.owner();let demand=match owner{Owner::Records=>Self::list_demand(&self.records)?,Owner::Tokens=>Self::list_demand(&self.record().tokens)?,Owner::Edges=>Self::list_demand(&self.record().edges)?,Owner::FieldOrder=>Self::list_demand(&self.record().order)?,Owner::Variants=>Self::list_demand(&self.variants)?,Owner::Frames=>Self::list_demand(&self.frames)?,Owner::Queue=>Self::list_demand(&self.queue)?,Owner::Hasher=>Self::list_demand(&self.hasher)?,Owner::None=>(0,self.semantic_copy(),1)};let child_depth=if let Some(record)=self.records.get(self.node){mapped(record.tokens.next_get_depth_demand(0))?+mapped(record.edges.next_get_depth_demand(0))?+mapped(record.order.next_get_depth_demand(0))?}else{0};let copy=demand.1+if matches!(owner,Owner::Hasher)&&demand.0==0&&self.phase==Phase::FinalHash{varint(self.queue.len()as u64).1}else{0};Ok((demand.0,copy,demand.2+mapped(self.records.next_get_depth_demand(0))?+child_depth+mapped(self.frames.next_get_depth_demand(0))?+mapped(self.queue.next_get_depth_demand(0))?+mapped(self.hasher.next_get_depth_demand(0))?+mapped(self.variants.next_get_depth_demand(0))?+2))}
 pub fn next_capacity_byte_demand(&self)->Result<usize,ValueError>{self.demands().map(|d|d.0)}
 pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.demands().map(|d|d.1)}
 pub fn next_depth_demand(&self)->Result<usize,ValueError>{self.demands().map(|d|d.2)}
 fn reserve<T>(list:&mut List<T>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{if list.len()<list.capacity(){return Ok(None)}let progress=list.reserve_one_funded(RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes.min(4096),..grant}).map_err(|error|invariant(error.reason))?;Ok(Some(progress))}
 fn token_push(&mut self,token:Token)->Result<(),ValueError>{pushed(self.records.get_mut(self.node).unwrap().tokens.push_reserved(token))}
 fn frame_push(&mut self,shape:H,depth:usize)->Result<(),ValueError>{if depth>64{return Err(ValueError::literal(K::DepthLimit,"retained borrowed schema exceeds shape depth"))}pushed(self.frames.push_reserved(Frame{shape,depth,stage:0,chosen:None}))}
 fn build(&mut self,spec:R)->Result<(),ValueError>{
  if self.phase==Phase::Birth{pushed(self.records.push_reserved(Record::new(spec,None)))?;self.root=Some(spec);self.phase=Phase::Build;return Ok(())}
  if let Some(mut edge)=self.edge{
   match edge.stage{
    0=>{if edge.scan==self.records.len(){edge.stage=1;}else if self.records.get(edge.scan).unwrap().address==Some(edge.make as usize){edge.target=edge.scan;edge.stage=2;}else{edge.scan+=1;}},
    1=>{edge.target=self.records.len();pushed(self.records.push_reserved(Record::new((edge.make)(),Some(edge.make as usize))))?;edge.stage=2;},
    2=>{self.token_push(Token::Edge{target:edge.target,local:self.record().edges.len()})?;edge.stage=3;},
    3=>{pushed(self.records.get_mut(self.node).unwrap().edges.push_reserved(edge.target))?;edge.stage=4;},
    _=>{self.frames.last_mut().unwrap().stage+=1;self.edge=None;return Ok(())}
   }self.edge=Some(edge);return Ok(())
  }
  if let Some(frame)=self.frames.last().copied(){return self.build_shape(frame)}
  let spec=self.record().spec;
  match self.field_stage{
   0=>{self.token_push(Token::Number(spec.fields.len()as u64))?;self.field_stage=1;},
   1=>{if let Some(field)=self.record().order.get(self.field_scan).copied(){self.field=field;self.field_stage=2;}else{self.node+=1;self.field_stage=7;self.field_scan=0;if self.node==self.records.len(){self.phase=Phase::InitialHash;self.node=0;self.refine_stage=1;}}},
   2=>{self.token_push(Token::Number(spec.fields[self.field].id as u64))?;self.field_stage=3;},
   3=>{self.token_push(Token::Text(spec.fields[self.field].key))?;self.field_stage=4;},
   4=>{let field=spec.fields[self.field];self.token_push(Token::Byte(u8::from(field.optional)|(u8::from(field.flatten)<<1)))?;self.field_stage=5;},
   5=>{self.frame_push(spec.fields[self.field].shape,0)?;self.field_stage=6;},
   6=>{self.field_scan+=1;self.field_stage=1;},
   7=>{let length=self.record().order.len();if length<spec.fields.len(){pushed(self.records.get_mut(self.node).unwrap().order.push_reserved(length))?;}else{self.field_sort=Some(HeapSortCursor::new(length));self.field_stage=8;}},
   _=>{let work=self.field_sort.unwrap().next();let observation=match work{HeapSortWork::Complete=>{self.field_sort=None;self.field_stage=0;return Ok(())},HeapSortWork::Compare{left,right}=>{let a=*self.record().order.get(left).unwrap();let b=*self.record().order.get(right).unwrap();Some((spec.fields[a].id,a).cmp(&(spec.fields[b].id,b)))},HeapSortWork::Swap{left,right}=>{self.records.get_mut(self.node).unwrap().order.swap(left,right);None},HeapSortWork::Advance=>None};self.field_sort.as_mut().unwrap().advance(observation).map_err(|_|invariant("retained field sorting lost its pending observation"))?;}
  }Ok(())
 }
 fn build_shape(&mut self,frame:Frame)->Result<(),ValueError>{
  if frame.stage==0{self.token_push(Token::Byte(scalar_tag(frame.shape)))?;self.frames.last_mut().unwrap().stage=1;return Ok(())}
  match(frame.shape,frame.stage){
   (H::Enum(items),1)=>{self.token_push(Token::Number(items.len()as u64))?;self.frames.last_mut().unwrap().stage=2;self.sort=Sort{reset:true,..Default::default()};},
   (H::Statements(items),1)=>{self.token_push(Token::Number(items.len()as u64))?;self.frames.last_mut().unwrap().stage=2;self.sort=Sort{reset:true,..Default::default()};},
   (H::Enum(_)|H::Statements(_),2)=>self.sort_turn(frame.shape)?,
   (H::Enum(items),3)=>{self.token_push(Token::Number(items[frame.chosen.unwrap()].1 as u64))?;self.frames.last_mut().unwrap().stage=4;},
   (H::Enum(items),4)=>{self.token_push(Token::Text(items[frame.chosen.unwrap()].0))?;self.frames.last_mut().unwrap().stage=5;},
   (H::Statements(items),3)=>{self.token_push(Token::Text(items[frame.chosen.unwrap()].0))?;self.frames.last_mut().unwrap().stage=4;},
   (H::Statements(items),4)=>{self.edge=Some(Edge{make:items[frame.chosen.unwrap()].1,scan:0,target:0,stage:0});},
   (H::Enum(_)|H::Statements(_),5)=>{self.sort.position+=1;self.frames.last_mut().unwrap().stage=2;},
   (H::Tuple(make,_)|H::List(make)|H::Block(make)|H::Map(make),1)=>{let parent=self.frames.len()-1;self.frame_push(make(),frame.depth+1)?;self.frames.get_mut(parent).unwrap().stage=2;},
   (H::Tuple(_,Some(_)),2)=>{self.token_push(Token::Byte(1))?;self.frames.last_mut().unwrap().stage=3;},
   (H::Tuple(_,None),2)=>{self.token_push(Token::Byte(0))?;self.frames.last_mut().unwrap().stage=4;},
   (H::Tuple(_,Some(count)),3)=>{self.token_push(Token::Number(count as u64))?;self.frames.last_mut().unwrap().stage=4;},
   (H::Record(make)|H::Table(make),1)=>{self.edge=Some(Edge{make,scan:0,target:0,stage:0});},
   (H::Quantity(unit)|H::Angle(unit),1)=>{self.token_push(Token::Text(unit.symbol))?;self.frames.last_mut().unwrap().stage=2;},
   (H::Ref(text)|H::Embed(text)|H::EmbedFrom(text),1)=>{self.token_push(Token::Text(text))?;self.frames.last_mut().unwrap().stage=2;},
   (H::Coord(n)|H::Dim(n),1)=>{self.token_push(Token::Byte(n))?;self.frames.last_mut().unwrap().stage=2;},
   _=>{self.frames.pop();}
  }Ok(())
 }
 fn sort_turn(&mut self,shape:H)->Result<(),ValueError>{
  if self.sort.reset{if self.variants.pop().is_none(){self.sort.reset=false;}return Ok(())}
  if !self.sort.initialized{let length=self.variants.len();if length<sort_length(shape){pushed(self.variants.push_reserved(length))?;}else{self.sort.cursor=Some(HeapSortCursor::new(length));self.sort.initialized=true;}return Ok(())}
  let observation=match self.sort.cursor.unwrap().next(){
   HeapSortWork::Complete=>{if let Some(chosen)=self.variants.get(self.sort.position).copied(){let frame=self.frames.last_mut().unwrap();frame.chosen=Some(chosen);frame.stage=3;}else{self.frames.pop();}return Ok(())},
   HeapSortWork::Compare{left,right}=>{let a=*self.variants.get(left).unwrap();let b=*self.variants.get(right).unwrap();let Some(order)=comparison(shape,a,b,self.sort.offset).0 else{self.sort.offset+=SPAN;return Ok(())};self.sort.offset=0;Some(order)},
   HeapSortWork::Swap{left,right}=>{self.variants.swap(left,right);None},HeapSortWork::Advance=>None};
  self.sort.cursor.as_mut().unwrap().advance(observation).map_err(|_|invariant("retained variant sorting lost its pending observation"))
 }
 fn hash_number(&mut self,n:u64){let(bytes,len)=varint(n);self.hasher.last_mut().unwrap().update(&bytes[..len]);}
 fn hash_token(&mut self,canonical:bool)->Result<bool,ValueError>{
  let Some(token)=self.record().tokens.get(self.token).copied()else{return Ok(true)};
  match token{
   Token::Byte(byte)=>{self.hasher.last_mut().unwrap().update(&[byte]);self.token+=1;},
   Token::Number(n)=>{self.hash_number(n);self.token+=1;},
   Token::Edge{target,local}=>{let n=if canonical{let class=self.records.get(target).unwrap().class;self.records.get(class).unwrap().ordinal}else{local};self.hash_number(n as u64);self.token+=1;},
   Token::Text(text)=>{if self.offset==0{self.hash_number(text.len()as u64);self.offset=1;}else{let start=self.offset-1;let end=text.len().min(start+SPAN);self.hasher.last_mut().unwrap().update(&text.as_bytes()[start..end]);if end==text.len(){self.offset=0;self.token+=1;}else{self.offset=end+1;}}}
  }Ok(false)
 }
 fn hash_turn(&mut self)->Result<(),ValueError>{
  let canonical=self.phase==Phase::FinalHash;
  if self.hasher.is_empty(){pushed(self.hasher.push_reserved(semio_framework_hash::Hasher::new())).expect("retained schema hasher capacity");if canonical{self.hash_number(self.queue.len()as u64);self.node=*self.queue.get(0).unwrap();}self.token=0;self.offset=0;self.refine_stage=1;return Ok(())}
  if !self.hash_token(canonical)?{return Ok(())}
  if canonical{self.bfs_node+=1;if self.bfs_node==self.queue.len(){self.digest=Some(*self.hasher.pop().unwrap().finalize().as_bytes());self.phase=Phase::Complete;}else{self.node=*self.queue.get(self.bfs_node).unwrap();self.token=0;self.offset=0;}}else{let digest=*self.hasher.pop().unwrap().finalize().as_bytes();self.records.get_mut(self.node).unwrap().digest=digest;self.node+=1;if self.node==self.records.len(){self.start_classification(false);}}Ok(())
 }
 fn start_classification(&mut self,refined:bool){self.phase=if refined{Phase::ClassifyRefined}else{Phase::ClassifyBase};self.node=0;self.candidate=0;self.equal=0;self.equal_offset=0;self.count=0;}
 fn next_candidate(&mut self){self.candidate+=1;self.equal=0;self.equal_offset=0;}
 fn classify_turn(&mut self){
  if self.node==self.records.len(){if self.phase==Phase::ClassifyRefined&&self.count==self.previous{self.phase=Phase::BfsBirth;self.node=0;}else{self.previous=self.count;self.node=0;self.phase=Phase::CopyOld;}return}
  if self.candidate==self.node{self.records.get_mut(self.node).unwrap().class=self.node;self.count+=1;self.node+=1;self.candidate=0;self.equal=0;self.equal_offset=0;return}
  let a=self.records.get(self.node).unwrap();let b=self.records.get(self.candidate).unwrap();
  if a.digest!=b.digest||(self.phase==Phase::ClassifyRefined&&a.old!=b.old){self.next_candidate();return}
  let equal=if self.phase==Phase::ClassifyRefined{
   if a.edges.len()!=b.edges.len(){Some(false)}else if self.equal==a.edges.len(){Some(true)}else{let left=self.records.get(*a.edges.get(self.equal).unwrap()).unwrap().old;let right=self.records.get(*b.edges.get(self.equal).unwrap()).unwrap().old;self.equal+=1;if left==right{None}else{Some(false)}}
  }else if a.tokens.len()!=b.tokens.len(){Some(false)}else if self.equal==a.tokens.len(){Some(true)}else{
   match(*a.tokens.get(self.equal).unwrap(),*b.tokens.get(self.equal).unwrap()){
    (Token::Text(a),Token::Text(b))=>{if a.len()!=b.len(){Some(false)}else{let end=a.len().min(self.equal_offset+SPAN);if a.as_bytes()[self.equal_offset..end]!=b.as_bytes()[self.equal_offset..end]{Some(false)}else{self.equal_offset=end;if end==a.len(){self.equal+=1;self.equal_offset=0;}None}}},
    (Token::Byte(a),Token::Byte(b))if a==b=>{self.equal+=1;None},(Token::Number(a),Token::Number(b))if a==b=>{self.equal+=1;None},(Token::Edge{local:a,..},Token::Edge{local:b,..})if a==b=>{self.equal+=1;None},_=>Some(false)
   }
  };
  match equal{Some(true)=>{let class=self.records.get(self.candidate).unwrap().class;self.records.get_mut(self.node).unwrap().class=class;self.node+=1;self.candidate=0;self.equal=0;self.equal_offset=0;},Some(false)=>self.next_candidate(),None=>{}}
 }
 fn refine_turn(&mut self){
  if self.hasher.is_empty(){pushed(self.hasher.push_reserved(semio_framework_hash::Hasher::new())).expect("retained schema hasher capacity");self.refine_stage=0;self.token=0;return}
  match self.refine_stage{
   0=>{self.hash_number(self.record().old as u64);self.refine_stage=1;},
   1=>{self.hash_number(self.record().edges.len()as u64);self.refine_stage=2;},
   2=>{if let Some(target)=self.record().edges.get(self.token).copied(){self.hash_number(self.records.get(target).unwrap().old as u64);self.token+=1;}else{self.refine_stage=3;}},
   _=>{let digest=*self.hasher.pop().unwrap().finalize().as_bytes();self.records.get_mut(self.node).unwrap().digest=digest;self.node+=1;if self.node==self.records.len(){self.start_classification(true);}}
  }
 }
 fn bfs_turn(&mut self)->Result<(),ValueError>{
  if self.phase==Phase::BfsBirth{let class=self.records.get(0).unwrap().class;self.records.get_mut(class).unwrap().ordinal=0;pushed(self.queue.push_reserved(0))?;self.phase=Phase::Bfs;self.bfs_node=0;self.bfs_edge=0;return Ok(())}
  if let Some(target)=self.pending_queue{let class=self.records.get(target).unwrap().class;self.records.get_mut(class).unwrap().ordinal=self.queue.len();pushed(self.queue.push_reserved(target))?;self.pending_queue=None;return Ok(())}
  if self.bfs_node==self.queue.len(){self.phase=Phase::FinalHash;self.refine_stage=0;self.bfs_node=0;self.hasher.pop();return Ok(())}
  let node=*self.queue.get(self.bfs_node).unwrap();if let Some(target)=self.records.get(node).unwrap().edges.get(self.bfs_edge).copied(){self.bfs_edge+=1;let class=self.records.get(target).unwrap().class;if self.records.get(class).unwrap().ordinal==usize::MAX{self.pending_queue=Some(target);}}else{self.bfs_node+=1;self.bfs_edge=0;}Ok(())
 }
 /// 🎟️ Borrows the same original schema per turn, admitting physical allocation before mutation.
 pub fn advance(&mut self,spec:R,grant:RetainedCloneGrant)->Result<BorrowedSchemaHashStep,ValueError>{
  if matches!(self.phase,Phase::Closing|Phase::Closed){return Err(invariant("retained schema cursor is closing"))}
  if self.root.is_some_and(|root|root.fields.as_ptr()!=spec.fields.as_ptr()||root.fields.len()!=spec.fields.len()){return Err(invariant("retained schema original authority changed"))}
  if self.phase==Phase::Complete{return Ok(BorrowedSchemaHashStep{complete:true,..Default::default()})}
  let(capacity,copy,depth)=self.demands()?;if grant.maximum_items==0||grant.maximum_capacity_bytes<capacity||grant.maximum_copy_bytes<copy||grant.maximum_depth<depth{return Ok(Default::default())}
  let reserve=match self.owner(){Owner::Records=>Self::reserve(&mut self.records,grant)?,Owner::Tokens=>Self::reserve(&mut self.records.get_mut(self.node).unwrap().tokens,grant)?,Owner::Edges=>Self::reserve(&mut self.records.get_mut(self.node).unwrap().edges,grant)?,Owner::FieldOrder=>Self::reserve(&mut self.records.get_mut(self.node).unwrap().order,grant)?,Owner::Variants=>Self::reserve(&mut self.variants,grant)?,Owner::Frames=>Self::reserve(&mut self.frames,grant)?,Owner::Queue=>Self::reserve(&mut self.queue,grant)?,Owner::Hasher=>Self::reserve(&mut self.hasher,grant)?,Owner::None=>None};
  if let Some(progress)=reserve{
   return Ok(BorrowedSchemaHashStep{progress,complete:false})
  }
  match self.phase{Phase::Birth|Phase::Build=>self.build(spec)?,Phase::InitialHash|Phase::FinalHash=>self.hash_turn()?,Phase::ClassifyBase|Phase::ClassifyRefined=>self.classify_turn(),Phase::CopyOld=>{let record=self.records.get_mut(self.node).unwrap();record.old=record.class;self.node+=1;if self.node==self.records.len(){self.node=0;self.phase=Phase::RefineHash;self.hasher.pop();}},Phase::RefineHash=>self.refine_turn(),Phase::BfsBirth|Phase::Bfs=>self.bfs_turn()?,_=>return Err(invariant("retained schema work frontier is invalid"))}
  Ok(BorrowedSchemaHashStep{progress:RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()},complete:self.phase==Phase::Complete})
 }
 fn close_list_demand<T>(list:&List<T>)->Result<RetirementDemand,ValueError>{if !list.is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<T>(),depth:mapped(list.next_pop_depth_demand())?,..Default::default()})}else{Ok(RetirementDemand{release_bytes:mapped(list.next_release_allocation_bytes())?,depth:mapped(list.next_release_depth_demand())?,..Default::default()})}}
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{let mut demand=if let Some(record)=self.records.last(){if !record.order.terminal_is_empty(){Self::close_list_demand(&record.order)?}else if !record.tokens.terminal_is_empty(){Self::close_list_demand(&record.tokens)?}else if !record.edges.terminal_is_empty(){Self::close_list_demand(&record.edges)?}else{Self::close_list_demand(&self.records)?}}else if !self.variants.terminal_is_empty(){Self::close_list_demand(&self.variants)?}else if !self.hasher.terminal_is_empty(){Self::close_list_demand(&self.hasher)?}else if !self.frames.terminal_is_empty(){Self::close_list_demand(&self.frames)?}else if !self.queue.terminal_is_empty(){Self::close_list_demand(&self.queue)?}else if !self.records.terminal_is_empty(){Self::close_list_demand(&self.records)?}else{RetirementDemand::default()};demand.depth+=mapped(self.records.next_pop_depth_demand())?+1;Ok(demand)}
 fn close_list<T>(list:&mut List<T>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if !list.is_empty(){list.pop();Ok(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<T>(),..Default::default()})}else{let step=mapped(list.release_empty_page(grant.maximum_release_bytes))?;Ok(RetainedCloneProgress{copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..Default::default()})}}
 /// 🍂️ Cancels or closes one original metadata payload or backing page without recursive disposal.
 pub fn close(&mut self,grant:RetainedCloneGrant)->Result<BorrowedSchemaHashStep,ValueError>{
  if self.terminal_is_empty(){return Ok(BorrowedSchemaHashStep{complete:true,..Default::default()})}let demand=self.retirement_demands()?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Default::default())}self.phase=Phase::Closing;
  let progress=if let Some(record)=self.records.last_mut(){if !record.order.terminal_is_empty(){Self::close_list(&mut record.order,grant)?}else if !record.tokens.terminal_is_empty(){Self::close_list(&mut record.tokens,grant)?}else if !record.edges.terminal_is_empty(){Self::close_list(&mut record.edges,grant)?}else{Self::close_list(&mut self.records,grant)?}}else if !self.variants.terminal_is_empty(){Self::close_list(&mut self.variants,grant)?}else if !self.hasher.terminal_is_empty(){Self::close_list(&mut self.hasher,grant)?}else if !self.frames.terminal_is_empty(){Self::close_list(&mut self.frames,grant)?}else if !self.queue.terminal_is_empty(){Self::close_list(&mut self.queue,grant)?}else if !self.records.terminal_is_empty(){Self::close_list(&mut self.records,grant)?}else{self.root=None;self.hasher.pop();self.edge=None;self.pending_queue=None;self.digest=None;self.phase=Phase::Closed;RetainedCloneProgress{copied_items:1,..Default::default()}};
  Ok(BorrowedSchemaHashStep{progress,complete:self.terminal_is_empty()})
 }
}
