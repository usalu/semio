//! 🪣️ Bounded four-connected RGBA coverage with retained workspace and explicit retirement.
use super::{validate_extent,PixelEditError};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};

pub const FLOOD_PAGE_PIXELS:usize=4096;
#[derive(Debug,PartialEq,Eq)]
pub struct FloodMask<'a>{pages:&'a[Vec<u8>],length:usize}
impl FloodMask<'_>{pub fn len(&self)->usize{self.length}pub fn is_empty(&self)->bool{self.length==0}pub fn at(&self,index:usize)->Option<u8>{(index<self.length).then(||self.pages[index/FLOOD_PAGE_PIXELS][index%FLOOD_PAGE_PIXELS])}pub fn iter(&self)->impl Iterator<Item=&u8>{self.pages.iter().flat_map(|page|page.iter())}}
#[derive(Clone,Copy)]
pub enum FloodSource<'a>{Rgba(&'a[u8]),Uniform([u8;4])}
impl FloodSource<'_>{
 fn color(self,index:usize)->[u8;4]{match self{Self::Rgba(bytes)=>bytes[index*4..index*4+4].try_into().expect("validated RGBA extent"),Self::Uniform(color)=>color}}
 fn identity(self,count:usize)->Result<(usize,usize),PixelEditError>{match self{Self::Rgba(bytes)if bytes.len()==count*4=>Ok((bytes.as_ptr()as usize,bytes.len())),Self::Uniform(color)=>Ok((u32::from_le_bytes(color)as usize,0)),_=>Err(PixelEditError::Invalid("RGBA8 length does not match flood extent"))}}
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct FloodProgress{pub work:u64,pub completed:usize,pub total:usize,pub done:bool,pub progress:RetainedCloneProgress}
pub struct FloodRegionCursor{
 width:usize,height:usize,count:usize,seed:usize,target:[u8;4],source:Option<(usize,usize)>,seen:ManuallyDrop<Option<Vec<Vec<u8>>>>,mask:ManuallyDrop<Option<Vec<Vec<u8>>>>,queue:ManuallyDrop<Option<Vec<Vec<u32>>>>,held:usize,queue_len:usize,initialized:usize,head:usize,phase:u8,cancelled:bool,retirement:RetainedCloneClose,
}
impl FloodRegionCursor{
 pub fn new(width:u32,height:u32,x:u32,y:u32)->Result<Self,PixelEditError>{let count=validate_extent(width,height)?;if x>=width||y>=height{return Err(PixelEditError::Invalid("Invalid flood seed"))}Ok(Self{width:width as usize,height:height as usize,count,seed:y as usize*width as usize+x as usize,target:[0;4],source:None,seen:ManuallyDrop::new(None),mask:ManuallyDrop::new(None),queue:ManuallyDrop::new(None),held:0,queue_len:0,initialized:0,head:0,phase:0,cancelled:false,retirement:Default::default()})}
 pub fn done(&self)->bool{self.phase==3&&!self.cancelled}
 pub fn workspace_bytes(&self)->usize{self.held}
 fn page_count(&self)->usize{self.count.div_ceil(FLOOD_PAGE_PIXELS)}
 fn allocation_capacity(&self)->usize{if self.seen.is_none()||self.mask.is_none()||self.queue.is_none(){return self.page_count()*size_of::<Vec<u8>>()}if self.seen.as_ref().unwrap().len()<self.page_count(){return self.page_length(self.seen.as_ref().unwrap().len())}if self.mask.as_ref().unwrap().len()<self.page_count(){return self.page_length(self.mask.as_ref().unwrap().len())}self.page_length(self.queue.as_ref().unwrap().len())*size_of::<u32>()}
 fn page_length(&self,page:usize)->usize{FLOOD_PAGE_PIXELS.min(self.count-page*FLOOD_PAGE_PIXELS)}
 pub fn demands(&self,maximum_work:usize)->Result<RetirementDemand,ValueError>{
  if maximum_work==0||maximum_work>65536{return Err(invalid("invalid flood work budget"))}let copy=size_of::<Self>()*2;
  Ok(match self.phase{0=>RetirementDemand{copy_bytes:copy+size_of::<Vec<u8>>(),capacity_bytes:self.allocation_capacity(),depth:1,..Default::default()},1=>RetirementDemand{copy_bytes:copy+maximum_work.min(self.count-self.initialized)*2+5,depth:1,..Default::default()},2=>RetirementDemand{copy_bytes:copy+maximum_work.min(self.count-self.head)*32,depth:1,..Default::default()},_=>Default::default()})
 }
 pub fn advance(&mut self,source:FloodSource<'_>,maximum_work:usize,grant:RetainedCloneGrant,cancelled:bool)->RetainedWorkOutcome<FloodProgress>{
  let mut work=0;let mut progress=RetainedCloneProgress::default();
  let result:Result<FloodProgress,ValueError>=(||{
  self.cancelled|=cancelled;if self.cancelled{return Err(ValueError::literal(ValueRefusalKind::Canceled,"Flood cancelled"))}if self.phase>=4{return Err(invalid("Flood is closing"))}if !(1..=65536).contains(&maximum_work){return Err(invalid("Invalid flood work budget"))}
  let demand=self.demands(maximum_work)?;if grant.maximum_items==0||!retained_clone_grant_admits(grant,demand){return Ok(FloodProgress{work:0,completed:self.initialized+self.head,total:self.count*2,done:self.done(),progress:Default::default()})}
  let identity=source.identity(self.count).map_err(|_|invalid("Invalid flood source extent"))?;if self.source.is_some_and(|previous|previous!=identity){return Err(invalid("Original flood source changed"))}
  match self.phase{
   0=>{work=1;self.source=Some(identity);self.target=source.color(self.seed);let capacity;
    if self.seen.is_none(){let mut pages=Vec::new();pages.try_reserve_exact(self.page_count()).map_err(|_|allocation())?;capacity=pages.capacity()*size_of::<Vec<u8>>();*self.seen=Some(pages);}
    else if self.mask.is_none(){let mut pages=Vec::new();pages.try_reserve_exact(self.page_count()).map_err(|_|allocation())?;capacity=pages.capacity()*size_of::<Vec<u8>>();*self.mask=Some(pages);}
    else if self.queue.is_none(){let mut pages=Vec::new();pages.try_reserve_exact(self.page_count()).map_err(|_|allocation())?;capacity=pages.capacity()*size_of::<Vec<u32>>();*self.queue=Some(pages);}
    else if self.seen.as_ref().unwrap().len()<self.page_count(){let mut page=Vec::new();page.try_reserve_exact(self.page_length(self.seen.as_ref().unwrap().len())).map_err(|_|allocation())?;capacity=page.capacity();self.seen.as_mut().unwrap().push(page);progress.copied_bytes+=size_of::<Vec<u8>>();}
    else if self.mask.as_ref().unwrap().len()<self.page_count(){let mut page=Vec::new();page.try_reserve_exact(self.page_length(self.mask.as_ref().unwrap().len())).map_err(|_|allocation())?;capacity=page.capacity();self.mask.as_mut().unwrap().push(page);progress.copied_bytes+=size_of::<Vec<u8>>();}
    else{let mut page=Vec::new();page.try_reserve_exact(self.page_length(self.queue.as_ref().unwrap().len())).map_err(|_|allocation())?;capacity=page.capacity()*size_of::<u32>();self.queue.as_mut().unwrap().push(page);progress.copied_bytes+=size_of::<Vec<u32>>();if self.queue.as_ref().unwrap().len()==self.page_count(){self.phase=1;}}
    progress.retained_capacity_bytes=capacity;self.held+=capacity;if capacity>demand.capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Flood allocation exceeds its quoted capacity"))}}
   1=>{let end=self.count.min(self.initialized+maximum_work);let count=end-self.initialized;for index in self.initialized..end{self.seen.as_mut().unwrap()[index/FLOOD_PAGE_PIXELS].push(0);self.mask.as_mut().unwrap()[index/FLOOD_PAGE_PIXELS].push(0);}self.initialized=end;work=count as u64;progress.copied_bytes=count*2;
    if end==self.count{self.queue.as_mut().unwrap()[0].push(self.seed as u32);self.queue_len=1;self.seen.as_mut().unwrap()[self.seed/FLOOD_PAGE_PIXELS][self.seed%FLOOD_PAGE_PIXELS]=1;self.phase=2;progress.copied_bytes+=5;}}
   2=>{while work<(maximum_work as u64)&&self.head<self.queue_len{let index=self.queue.as_ref().unwrap()[self.head/FLOOD_PAGE_PIXELS][self.head%FLOOD_PAGE_PIXELS]as usize;self.head+=1;work+=1;progress.copied_bytes+=size_of::<usize>();if source.color(index)!=self.target{continue}self.mask.as_mut().unwrap()[index/FLOOD_PAGE_PIXELS][index%FLOOD_PAGE_PIXELS]=255;progress.copied_bytes+=1;let(x,y)=(index%self.width,index/self.width);
     for(next,inside)in[(index.wrapping_sub(1),x>0),(index+1,x+1<self.width),(index.wrapping_sub(self.width),y>0),(index+self.width,y+1<self.height)]{if inside&&self.seen.as_ref().unwrap()[next/FLOOD_PAGE_PIXELS][next%FLOOD_PAGE_PIXELS]==0{self.seen.as_mut().unwrap()[next/FLOOD_PAGE_PIXELS][next%FLOOD_PAGE_PIXELS]=1;self.queue.as_mut().unwrap()[self.queue_len/FLOOD_PAGE_PIXELS].push(next as u32);self.queue_len+=1;progress.copied_bytes+=5;}}
    }if self.head==self.queue_len{self.phase=3;}}
   _=>{}
  }
  Ok(FloodProgress{work,completed:self.initialized+self.head,total:self.count*2,done:self.done(),progress})
  })();
  if work!=0{progress.copied_items=1;progress.copied_bytes+=size_of::<Self>()*2;}
  RetainedWorkOutcome{result:result.map(|mut step|{step.progress=progress;step}).map_err(|error|error.with_retained_progress(progress)),work}
 }
 pub fn result(&self)->Result<FloodMask<'_>,PixelEditError>{if !self.done(){return Err(if self.cancelled{PixelEditError::Cancelled}else{PixelEditError::Incomplete})}Ok(FloodMask{pages:self.mask.as_ref().unwrap(),length:self.count})}
 pub fn begin_close(&mut self){self.phase=4;}
 pub fn terminal_is_empty(&self)->bool{self.seen.is_none()&&self.mask.is_none()&&self.queue.is_none()&&self.retirement.is_empty()&&self.held==0}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{let body=body.max(size_of::<Self>()*2);if self.phase!=4{return Err(invalid("flood close has not begun"))}if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.queue.is_some(){return self.retirement.next_owner_demands::<Vec<Vec<u32>>>(true,&None,body)}if self.mask.is_some()||self.seen.is_some(){return self.retirement.next_owner_demands::<Vec<Vec<u8>>>(true,&None,body)}Ok(Default::default())}
 fn account_close(&mut self,turn:RetainedWorkOutcome)->RetainedWorkOutcome{let progress=match &turn.result{Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>*progress,Err(error)=>error.retained_progress()};self.held+=progress.retained_capacity_bytes;self.held=self.held.checked_sub(progress.released_bytes).expect("flood close releases only retained capacity");turn}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if self.phase!=4{return RetainedWorkOutcome{result:Err(invalid("flood close has not begun")),work:0}}if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}let demand=match self.close_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if grant.maximum_items==0||!retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if !self.retirement.is_empty(){let turn=match self.retirement.select(grant.maximum_copy_bytes){Ok(selected)=>selected.step(grant),Err(error)=>RetainedWorkOutcome{result:Err(error),work:0}};return self.account_close(turn)}
  let turn=if self.queue.is_some(){self.retirement.begin_granted(&mut self.queue,grant)}else if self.mask.is_some(){self.retirement.begin_granted(&mut self.mask,grant)}else if self.seen.is_some(){self.retirement.begin_granted(&mut self.seen,grant)}else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};
  self.account_close(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work})
 }
}
fn allocation()->ValueError{ValueError::literal(ValueRefusalKind::AllocationFailed,"Flood allocation refused")}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
impl Drop for FloodRegionCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"flood workspace requires funded closure")}}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
