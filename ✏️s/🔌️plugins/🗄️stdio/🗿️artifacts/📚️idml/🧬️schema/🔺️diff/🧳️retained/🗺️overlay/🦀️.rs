//! 🗺️ Ordered IDML batches retain numeric field addresses over the live post and original diff.
use crate::{IdmlSnapshot,IdmlDiff};
use crate::io::admission::{IdmlAdmissionParts,IdmlAdmissionSource,IdmlAdmissionCursor,IdmlAdmissionGrant};
use std::mem::size_of;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct IdmlFieldAddress{pub step:Option<usize>,pub part:usize}
#[derive(Clone,Copy)]
struct Row{path:IdmlFieldAddress,bytes:IdmlFieldAddress,previous:Option<usize>,next:Option<usize>,touched:usize,ordered:usize,live:bool}
#[derive(Clone,Copy,Debug,Default)]
pub struct IdmlOverlayStep{pub work:usize,pub copied_bytes:usize,pub retained_capacity_bytes:usize,pub released_bytes:usize,pub done:bool,pub error:Option<&'static str>}
fn name<'a>(post:&'a IdmlSnapshot,diff:&'a IdmlDiff,a:IdmlFieldAddress)->&'a str{match a.step{Some(step)=>&diff.steps[step].parts[a.part].path,None=>&post.parts[a.part].path}}
fn data<'a>(post:&'a IdmlSnapshot,diff:&'a IdmlDiff,a:IdmlFieldAddress)->&'a [u8]{match a.step{Some(step)=>diff.steps[step].parts[a.part].bytes.as_deref().expect("live authored payload"),None=>&post.parts[a.part].bytes}}
struct View<'a>{rows:&'a[Row],order:&'a[usize],post:&'a IdmlSnapshot,diff:&'a IdmlDiff,version:Option<usize>,identity:usize}
impl IdmlAdmissionParts for View<'_>{fn source_identity(&self)->usize{self.identity}fn part_count(&self)->usize{self.order.len()}fn part_path(&self,index:usize)->&str{name(self.post,self.diff,self.rows[self.order[index]].path)}fn part_bytes(&self,index:usize)->&[u8]{data(self.post,self.diff,self.rows[self.order[index]].bytes)}fn dom_version(&self)->Option<&str>{Some(match self.version{Some(step)=>self.diff.steps[step].dom_version.as_deref().expect("selected authored version"),None=>&self.post.dom_version})}}
/// 🧭 Source payloads remain borrowed; metadata and admission pages have separate grants.
pub struct IdmlOverlayCursor{post:Option<usize>,diff:Option<usize>,mode:&'static str,count:usize,max_rows:usize,slots:usize,hash:Vec<u32>,rows:Vec<Row>,order:Vec<usize>,admission:IdmlAdmissionCursor,step:usize,part:usize,seed:usize,first:Option<usize>,last:Option<usize>,active:usize,version:Option<usize>,key:IdmlFieldAddress,hash_value:u32,offset:usize,slot:usize,compare:usize,found:Option<usize>,operation:&'static str,walk:Option<usize>,order_first:Option<usize>,order_last:Option<usize>,order_count:usize,error:Option<&'static str>,closing:bool}
impl Default for IdmlOverlayCursor{fn default()->Self{Self::new()}}
impl IdmlOverlayCursor{
 /// 🌱 Empty fixed state reads no source and allocates no metadata.
 pub fn new()->Self{Self{post:None,diff:None,mode:"count",count:0,max_rows:0,slots:0,hash:Vec::new(),rows:Vec::new(),order:Vec::new(),admission:IdmlAdmissionCursor::new(),step:0,part:0,seed:0,first:None,last:None,active:0,version:None,key:IdmlFieldAddress{step:None,part:0},hash_value:2166136261,offset:0,slot:0,compare:0,found:None,operation:"seed",walk:None,order_first:None,order_last:None,order_count:0,error:None,closing:false}}
 fn lookup(&mut self,key:IdmlFieldAddress,operation:&'static str){self.key=key;self.hash_value=2166136261;self.offset=0;self.operation=operation;self.mode="hash";}
 fn unlink(&mut self,index:usize){let row=self.rows[index];if let Some(previous)=row.previous{self.rows[previous].next=row.next}else{self.first=row.next}if let Some(next)=row.next{self.rows[next].previous=row.previous}else{self.last=row.previous}self.rows[index].live=false;self.active-=1;}
 fn append(&mut self,index:usize){self.rows[index].previous=self.last;self.rows[index].next=None;if let Some(last)=self.last{self.rows[last].next=Some(index)}else{self.first=Some(index)}self.last=Some(index);self.rows[index].live=true;self.active+=1;}
 fn new_row(&mut self,path:IdmlFieldAddress)->usize{let index=self.rows.len();self.rows.push(Row{path,bytes:path,previous:None,next:None,touched:0,ordered:0,live:false});self.hash[self.slot]=index as u32+1;index}
 fn key<'a>(&self,post:&'a IdmlSnapshot,diff:&'a IdmlDiff)->&'a str{if self.operation=="order"{&diff.steps[self.step].part_order.as_ref().expect("selected order")[self.order_count]}else{name(post,diff,self.key)}}
 /// 📏 Quotes numeric arena, order and open-address name backing before any reservation.
 pub fn next_capacity_bytes(&self)->usize{if self.mode=="allocate"{self.max_rows*(size_of::<Row>()+size_of::<usize>())+self.slots*size_of::<u32>()}else if self.mode=="admit"{self.admission.next_capacity_bytes()}else{0}}
 pub fn advance_release_bytes(&self)->usize{if self.mode=="release"{self.admission.next_release_bytes()}else{0}}
 /// 📝 Numeric initialization and fixed cursor reset are independently visible.
 pub fn next_copy_bytes(&self)->usize{if self.mode=="allocate"{self.next_capacity_bytes()}else if self.mode=="reset"{size_of::<IdmlAdmissionCursor>()*2}else if self.mode=="admit"{self.admission.next_copy_bytes()}else{0}}
 /// 👁 One final member exposes only its two original immutable coordinates.
 pub fn member(&self,index:usize)->Option<(IdmlFieldAddress,IdmlFieldAddress)>{if self.mode!="done"||self.error.is_some(){return None;}self.order.get(index).map(|&index|(self.rows[index].path,self.rows[index].bytes))}
 pub fn member_count(&self)->Option<usize>{if self.mode=="done"&&self.error.is_none(){Some(self.order.len())}else{None}}
 pub fn version_step(&self)->Option<usize>{self.version}
 /// ▶ Every source byte comparison, numeric update and page turn debits the supplied work.
 pub fn advance(&mut self,post:&IdmlSnapshot,diff:&IdmlDiff,grant:IdmlAdmissionGrant)->IdmlOverlayStep{
  let mut result=IdmlOverlayStep::default();if self.closing{result.error=Some("stdio.idml.overlay-closed");return result;}let p=post as*const _ as usize;let d=diff as*const _ as usize;if self.post.is_none(){self.post=Some(p);self.diff=Some(d);self.max_rows=post.parts.len();}else if self.post!=Some(p)||self.diff!=Some(d){result.error=Some("stdio.idml.overlay-source");return result;}
  while result.work<grant.maximum_work&&self.error.is_none()&&self.mode!="done"{
   match self.mode{
    "count"=>{if self.count==diff.steps.len(){if self.max_rows>32768{self.error=Some("stdio.idml.member-limit");continue;}self.slots=(self.max_rows.max(1)*2).next_power_of_two();self.mode="allocate";}else{self.max_rows=self.max_rows.saturating_add(diff.steps[self.count].parts.len());self.count+=1;if self.max_rows>32768{self.error=Some("stdio.idml.member-limit");}}result.work+=1;},
    "allocate"=>{let bytes=self.next_capacity_bytes();if grant.maximum_capacity_bytes<bytes||grant.maximum_copy_bytes<bytes{break;}let reserved=self.rows.try_reserve_exact(self.max_rows).and_then(|_|self.order.try_reserve_exact(self.max_rows)).and_then(|_|self.hash.try_reserve_exact(self.slots));result.retained_capacity_bytes=self.rows.capacity()*size_of::<Row>()+self.order.capacity()*size_of::<usize>()+self.hash.capacity()*size_of::<u32>();if reserved.is_err(){self.error=Some("stdio.idml.overlay-allocation");break;}if result.retained_capacity_bytes>grant.maximum_capacity_bytes{self.error=Some("stdio.idml.overlay-capacity");break;}self.hash.resize(self.slots,0);result.copied_bytes+=bytes;self.mode="seed";result.work+=1;break;},
    "seed"=>{if self.seed==post.parts.len(){self.mode="patch";continue;}self.lookup(IdmlFieldAddress{step:None,part:self.seed},"seed");result.work+=1;},
    "hash"=>{let key=self.key(post,diff).as_bytes();if self.offset==key.len(){self.slot=self.hash_value as usize&(self.slots-1);self.mode="probe";}else{self.hash_value=(self.hash_value^u32::from(key[self.offset])).wrapping_mul(16777619);self.offset+=1;}result.work+=1;},
    "probe"=>{let value=self.hash[self.slot];if value==0{self.found=None;self.mode="resolved";}else{self.found=Some(value as usize-1);self.compare=0;self.mode="compare";}result.work+=1;},
    "compare"=>{let key=self.key(post,diff).as_bytes();let other=name(post,diff,self.rows[self.found.expect("occupied hash slot")].path).as_bytes();if other.len()!=key.len()||(self.compare<other.len()&&other[self.compare]!=key[self.compare]){self.slot=(self.slot+1)&(self.slots-1);self.mode="probe";}else if self.compare==other.len(){self.mode="resolved";}else{self.compare+=1;}result.work+=1;},
    "resolved"=>{
     if self.operation=="seed"{if self.found.is_some(){self.error=Some("stdio.idml.duplicate-member");continue;}let index=self.new_row(IdmlFieldAddress{step:None,part:self.seed});self.append(index);self.seed+=1;self.mode="seed";}
     else if self.operation=="part"{let patch=&diff.steps[self.step].parts[self.part];if self.found.is_some_and(|index|self.rows[index].touched==self.step+1){self.error=Some("stdio.idml.duplicate-part-patch");continue;}if patch.bytes.is_none(){let Some(index)=self.found.filter(|&index|self.rows[index].live)else{self.error=Some("stdio.idml.missing-part-patch");continue;};self.rows[index].touched=self.step+1;self.unlink(index);}else{let index=match self.found{Some(index)=>index,None=>self.new_row(IdmlFieldAddress{step:Some(self.step),part:self.part})};self.rows[index].touched=self.step+1;self.rows[index].bytes=IdmlFieldAddress{step:Some(self.step),part:self.part};if !self.rows[index].live{self.rows[index].path=IdmlFieldAddress{step:Some(self.step),part:self.part};self.append(index);}}self.part+=1;self.mode="patch";}
     else{let Some(index)=self.found.filter(|&index|self.rows[index].live&&self.rows[index].ordered!=self.step+1)else{self.error=Some("stdio.idml.part-order");continue;};self.rows[index].ordered=self.step+1;self.rows[index].previous=self.order_last;self.rows[index].next=None;if let Some(last)=self.order_last{self.rows[last].next=Some(index)}else{self.order_first=Some(index)}self.order_last=Some(index);self.order_count+=1;self.mode="order";}result.work+=1;
    },
    "patch"=>{if self.step==diff.steps.len(){self.order.clear();self.walk=self.first;self.mode="final";continue;}let patch=&diff.steps[self.step];if self.part<patch.parts.len(){self.lookup(IdmlFieldAddress{step:Some(self.step),part:self.part},"part");continue;}if patch.dom_version.is_some(){self.version=Some(self.step);}if let Some(order)=&patch.part_order{if order.len()!=self.active{self.error=Some("stdio.idml.part-order");continue;}self.order_count=0;self.order_first=None;self.order_last=None;self.mode="order";}else{self.order.clear();self.walk=self.first;self.mode="view";}result.work+=1;},
    "order"=>{let order=diff.steps[self.step].part_order.as_ref().expect("selected source order");if self.order_count==order.len(){self.first=self.order_first;self.last=self.order_last;self.order.clear();self.walk=self.first;self.mode="view";}else{self.lookup(IdmlFieldAddress{step:Some(self.step),part:0},"order");}result.work+=1;},
    "view"|"final"=>{if let Some(index)=self.walk{self.order.push(index);self.walk=self.rows[index].next;result.work+=1;continue;}self.mode=if self.mode=="final"{"done"}else{"admit"};result.work+=1;},
    "admit"=>{let view=View{rows:&self.rows,order:&self.order,post,diff,version:self.version,identity:self as*const Self as usize};let source=IdmlAdmissionSource{snapshot:&view,replacement:None,compare_version:true};let step=self.admission.advance(source,IdmlAdmissionGrant{maximum_work:grant.maximum_work-result.work,maximum_copy_bytes:grant.maximum_copy_bytes-result.copied_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes-result.retained_capacity_bytes,..grant});result.work+=step.work;result.copied_bytes+=step.copied_bytes;result.retained_capacity_bytes+=step.retained_capacity_bytes;if let Some(error)=step.error{self.error=Some(error);}if step.done{self.admission.begin_close();self.mode="release";}if step.work==0{break;}},
    "release"=>{let bytes=self.admission.next_release_bytes();if grant.maximum_release_bytes-result.released_bytes<bytes{break;}let step=self.admission.close_one(grant.maximum_release_bytes-result.released_bytes);result.released_bytes+=step.released_bytes;result.work+=1;if step.done{self.mode="reset";}},
    "reset"=>{let bytes=self.next_copy_bytes();if grant.maximum_copy_bytes-result.copied_bytes<bytes{break;}self.admission=IdmlAdmissionCursor::new();self.step+=1;self.part=0;self.mode="patch";result.copied_bytes+=bytes;result.work+=1;},
    _=>{self.error=Some("stdio.idml.overlay-state");}
   }
  }result.done=self.mode=="done";result.error=self.error;result
 }
 /// 🛑 No source payload or live post is destroyed during cancellation.
 pub fn begin_close(&mut self){self.closing=true;self.admission.begin_close();}
 /// ♻ Numeric original allocations close independently after syntax-index pages.
 pub fn next_release_bytes(&self)->usize{let page=self.admission.next_release_bytes();if page!=0{return page;}if self.hash.capacity()!=0{return self.hash.capacity()*size_of::<u32>();}if self.order.capacity()!=0{return self.order.capacity()*size_of::<usize>();}self.rows.capacity()*size_of::<Row>()}
 /// 🪜 Closes one exact numeric allocation, leaving immutable payload ownership with its source.
 pub fn close_one(&mut self,maximum_release_bytes:usize)->usize{if !self.closing{return 0;}let page=self.admission.next_release_bytes();if page!=0{return self.admission.close_one(maximum_release_bytes).released_bytes;}let bytes=self.next_release_bytes();if maximum_release_bytes<bytes{return 0;}if self.hash.capacity()!=0{self.hash=Vec::new();}else if self.order.capacity()!=0{self.order=Vec::new();}else if self.rows.capacity()!=0{self.rows=Vec::new();}else{self.post=None;self.diff=None;}bytes}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.hash.capacity()==0&&self.order.capacity()==0&&self.rows.capacity()==0&&self.post.is_none()&&self.admission.terminal_is_empty()}
}
impl Drop for IdmlOverlayCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"IDML numeric overlay requires granted close");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;
