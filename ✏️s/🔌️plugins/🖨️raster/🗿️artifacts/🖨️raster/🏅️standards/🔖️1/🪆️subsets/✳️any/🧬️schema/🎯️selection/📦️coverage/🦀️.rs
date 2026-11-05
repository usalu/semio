//! 📦️ Immutable packed coverage shares page roots while keeping every page backing within a four-KiB release grant.
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::persistent::{PersistentList,PersistentListIter};
use super::RasterSelectionSpan;
use std::{fmt,ops::Index,mem::size_of};

pub const RASTER_SELECTION_PAGE_BYTES:usize=4096;
pub const RASTER_SELECTION_PAGE_ROWS:usize=(RASTER_SELECTION_PAGE_BYTES-1024)/size_of::<RasterSelectionSpan>();

#[derive(Clone,Copy,semio_framework_value::RetireOwned)]
pub struct RasterSpanPage{rows:[RasterSelectionSpan;RASTER_SELECTION_PAGE_ROWS],len:usize}
impl RasterSpanPage{
    pub fn new()->Self{Self{rows:[RasterSelectionSpan{start:0,length:0,coverage:0};RASTER_SELECTION_PAGE_ROWS],len:0}}
    pub fn len(&self)->usize{self.len}
    pub fn is_full(&self)->bool{self.len==RASTER_SELECTION_PAGE_ROWS}
    pub fn get(&self,index:usize)->Option<&RasterSelectionSpan>{self.rows[..self.len].get(index)}
    pub fn as_slice(&self)->&[RasterSelectionSpan]{&self.rows[..self.len]}
    pub fn iter(&self)->std::slice::Iter<'_,RasterSelectionSpan>{self.as_slice().iter()}
    pub fn push(&mut self,value:RasterSelectionSpan)->Result<(),ValueError>{if self.is_full(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"selection coverage page is full"));}self.rows[self.len]=value;self.len+=1;Ok(())}
}
impl Default for RasterSpanPage{fn default()->Self{Self::new()}}

#[derive(Clone,Default,semio_framework_value::RetireOwned,semio_framework_value::RetainedClone)]
pub struct RasterSelectionSpans{pages:PersistentList<RasterSpanPage>,len:usize,maximum_end:u64}
impl RasterSelectionSpans{
    pub fn page_count(rows:usize)->usize{rows.div_ceil(RASTER_SELECTION_PAGE_ROWS)}
    fn from_pages(pages:PersistentList<RasterSpanPage>,len:usize,maximum_end:u64)->Result<Self,(ValueError,PersistentList<RasterSpanPage>)>{if pages.len()!=Self::page_count(len){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"selection coverage root differs from its authored row count"),pages));}Ok(Self{pages,len,maximum_end})}
    pub fn len(&self)->usize{self.len}
    pub fn maximum_end(&self)->u64{self.maximum_end}
    pub fn pages(&self)->&PersistentList<RasterSpanPage>{&self.pages}
    pub fn is_empty(&self)->bool{self.len==0}
    pub fn get(&self,index:usize)->Option<&RasterSelectionSpan>{if index>=self.len{return None;}self.pages.get(index/RASTER_SELECTION_PAGE_ROWS)?.get(index%RASTER_SELECTION_PAGE_ROWS)}
    pub fn get_with_work(&self,index:usize)->(Option<&RasterSelectionSpan>,u64){if index>=self.len{return(None,0);}let(page,work)=self.pages.get_with_work(index/RASTER_SELECTION_PAGE_ROWS);(page.and_then(|page|page.get(index%RASTER_SELECTION_PAGE_ROWS)),work+1)}
    pub fn page_slice(&self,index:usize)->Option<&[RasterSelectionSpan]>{if index>=self.len{return None;}Some(&self.pages.get(index/RASTER_SELECTION_PAGE_ROWS)?.as_slice()[index%RASTER_SELECTION_PAGE_ROWS..])}
    pub fn page_slice_with_work(&self,index:usize)->(Option<&[RasterSelectionSpan]>,u64){if index>=self.len{return(None,0);}let(page,work)=self.pages.get_with_work(index/RASTER_SELECTION_PAGE_ROWS);(page.map(|page|&page.as_slice()[index%RASTER_SELECTION_PAGE_ROWS..]),work+1)}
    pub fn rank_observation_demand(&self,index:usize)->Result<RetirementDemand,ValueError>{if index>=self.len{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"selection coverage rank is outside its original owner"));}self.pages.rank_observation_demand(index/RASTER_SELECTION_PAGE_ROWS)}
    pub fn iter(&self)->RasterSelectionSpanIter<'_>{RasterSelectionSpanIter{pages:self.pages.iter(),current:None,index:0,remaining:self.len}}
}
impl Index<usize> for RasterSelectionSpans{type Output=RasterSelectionSpan;fn index(&self,index:usize)->&Self::Output{self.get(index).expect("selection coverage rank is in bounds")}}
impl fmt::Debug for RasterSelectionSpans{fn fmt(&self,formatter:&mut fmt::Formatter<'_>)->fmt::Result{formatter.debug_list().entries(self.iter()).finish()}}
impl PartialEq for RasterSelectionSpans{fn eq(&self,other:&Self)->bool{self.len==other.len&&self.maximum_end==other.maximum_end&&(self.pages.same_root(&other.pages)||self.iter().eq(other.iter()))}}
impl Eq for RasterSelectionSpans{}

pub struct RasterSelectionSpanIter<'a>{pages:PersistentListIter<'a,RasterSpanPage>,current:Option<&'a RasterSpanPage>,index:usize,remaining:usize}
impl<'a> Iterator for RasterSelectionSpanIter<'a>{
    type Item=&'a RasterSelectionSpan;
    fn next(&mut self)->Option<Self::Item>{if self.remaining==0{return None;}if self.current.is_none_or(|page|self.index==page.len()){self.current=self.pages.next();self.index=0;}let result=self.current?.get(self.index)?;self.index+=1;self.remaining-=1;Some(result)}
    fn size_hint(&self)->(usize,Option<usize>){(self.remaining,Some(self.remaining))}
}
impl ExactSizeIterator for RasterSelectionSpanIter<'_>{}
impl<'a> IntoIterator for &'a RasterSelectionSpans{type Item=&'a RasterSelectionSpan;type IntoIter=RasterSelectionSpanIter<'a>;fn into_iter(self)->Self::IntoIter{self.iter()}}

#[path="🔁️codec/🦀️.rs"]
mod codec;

#[path="🧵️builder/🦀️.rs"]
mod builder;
pub use builder::RasterSelectionSpansBuilder;

#[path="🛬️receiving/🦀️.rs"]
mod receiving;
