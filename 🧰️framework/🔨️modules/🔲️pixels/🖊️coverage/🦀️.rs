//! 🖊️ Exact transformed polygon coverage for https://www.w3.org/TR/SVG11/painting.html#FillProperties.
use crate::editing::{validate_extent,PixelEditError};
use std::cmp::Ordering;
use semio_framework_2d::physical_work_retirement;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CoverageRule {NonZero,EvenOdd}
#[derive(Clone,Debug,PartialEq)]
#[derive(semio_framework_value::RetireOwned)]
pub struct CoverageInput {pub width:u32,pub height:u32,pub transform:[f64;6],pub rule:CoverageRule,pub contours:Vec<Vec<[f64;2]>>}
#[derive(Clone,Debug,PartialEq,Eq)]
#[derive(semio_framework_value::RetireOwned)]
pub struct CoverageMask {pub width:u32,pub height:u32,pub coverage:Vec<u8>}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CoveragePhase {Preparing,Sorting,Rasterizing,Complete}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct CoverageProgress {pub phase:CoveragePhase,pub completed:usize,pub total:usize,pub work:u64,pub done:bool}
#[derive(Clone,Copy)]
struct Edge {x:f64,y:f64,dx:f64,winding:i32}
#[derive(Clone,Copy)]
struct SweepEvent {y:f64,edge:usize,add:bool}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Stage {Prepare,Events,Apply,Copy,Sort,Cross,Wind,Span,Finish,Write,Done}
const EPS:f64=1e-10;
const MAX_POINTS:usize=65536;
const MAX_CONTOURS:usize=65536;
const MAX_COORDINATE:f64=1e9;
const NONE:usize=usize::MAX;
fn coordinate(value:f64)->bool {value.is_finite()&&value.abs()<=MAX_COORDINATE}
fn primitive(value:f64)->f64 {if value<=0.0 {0.0} else if value<1.0 {value*value/2.0} else {value-0.5}}
fn integral(a:f64,b:f64,height:f64)->f64 {
    if a.max(b)<=0.0 {0.0} else if a.min(b)>=1.0 {height} else if (b-a).abs()<EPS {((a+b)/2.0).clamp(0.0,1.0)*height} else {(primitive(b)-primitive(a))/(b-a)*height}
}
fn x_at(edge:Edge,y:f64)->f64 {edge.x+(y-edge.y)*edge.dx}

#[derive(semio_framework_value::RetireOwned)]
struct MergeSort {source:Vec<usize>,target:Vec<usize>,width:usize,left:usize,i:usize,j:usize,k:usize,mid:usize,end:usize,writes:usize}
impl MergeSort {
    fn new(source:Vec<usize>)->Self {
        let mut sorter=Self {target:vec![0;source.len()],source,width:1,left:0,i:0,j:0,k:0,mid:0,end:0,writes:0};sorter.setup();sorter
    }
    fn done(&self)->bool {self.width>=self.source.len()}
    fn setup(&mut self) {self.i=self.left;self.mid=(self.left+self.width).min(self.source.len());self.j=self.mid;self.k=self.left;self.end=(self.left+2*self.width).min(self.source.len());}
    fn step(&mut self,compare:impl Fn(usize,usize)->Ordering) {
        if self.done() {return;}
        self.target[self.k]=if self.i<self.mid&&(self.j>=self.end||compare(self.source[self.i],self.source[self.j])!=Ordering::Greater) {let value=self.source[self.i];self.i+=1;value} else {let value=self.source[self.j];self.j+=1;value};
        self.k+=1;self.writes+=1;if self.k<self.end {return;}
        self.left+=2*self.width;if self.left>=self.source.len() {std::mem::swap(&mut self.source,&mut self.target);self.width*=2;self.left=0;}self.setup();
    }
}

/// ⏱️ Owned, budgeted sweep with exact polygon area integration and cancellable candidates.
#[derive(semio_framework_value::RetireOwned)]
pub struct CoverageJob {
    input:CoverageInput,mask:CoverageMask,areas:Vec<f64>,edges:Vec<Edge>,events:Vec<SweepEvent>,positions:Vec<usize>,active:Vec<usize>,event_ids:Vec<usize>,
    sorted:Vec<usize>,copied:Vec<usize>,sorter:Option<MergeSort>,stage:Stage,cancelled:bool,failed:Option<PixelEditError>,work:u64,
    contour:usize,point:usize,prepared:usize,preparation_total:usize,first:Option<[f64;2]>,previous:Option<[f64;2]>,
    event:usize,copy:usize,scan:usize,row:usize,y:f64,end:f64,winding:i32,left:usize,span_left:usize,span_right:usize,pixel:usize,pixel_end:usize,dirty_start:usize,dirty_end:usize,
}
impl CoverageJob {
    pub fn new(input:CoverageInput)->Result<Self,PixelEditError> {
        let count=validate_extent(input.width,input.height)?;
        if !input.transform.into_iter().all(coordinate)||input.contours.len()>MAX_CONTOURS {return Err(PixelEditError::Invalid("Invalid vector coverage contract"));}
        let points=input.contours.iter().try_fold(0usize,|total,contour|total.checked_add(contour.len())).ok_or(PixelEditError::Invalid("Coverage geometry exceeds point budget"))?;
        if points>MAX_POINTS {return Err(PixelEditError::Invalid("Coverage geometry exceeds point budget"));}
        let mask=CoverageMask {width:input.width,height:input.height,coverage:vec![0;count]};let width=input.width as usize;let preparation_total=points+input.contours.len();
        Ok(Self {input,mask,areas:vec![0.0;width],edges:Vec::new(),events:Vec::new(),positions:Vec::new(),active:Vec::new(),event_ids:Vec::new(),sorted:Vec::new(),copied:Vec::new(),sorter:None,stage:Stage::Prepare,cancelled:false,failed:None,work:0,contour:0,point:0,prepared:0,preparation_total,first:None,previous:None,event:0,copy:0,scan:0,row:0,y:0.0,end:0.0,winding:0,left:NONE,span_left:NONE,span_right:NONE,pixel:0,pixel_end:0,dirty_start:width,dirty_end:0})
    }
    fn edge(&mut self,from:[f64;2],to:[f64;2])->Result<(),PixelEditError> {
        if from[1]==to[1] {return Ok(());}
        let (low,high)=if from[1]<to[1] {(from,to)} else {(to,from)};let start=low[1].max(0.0);let end=high[1].min(f64::from(self.input.height));
        if start>=end {return Ok(());}
        let dx=(high[0]-low[0])/(high[1]-low[1]);if !dx.is_finite() {return Err(PixelEditError::Invalid("Coverage edge exceeds numeric limits"));}
        let id=self.edges.len();self.edges.push(Edge {x:low[0],y:low[1],dx,winding:if from[1]<to[1] {1} else {-1}});self.positions.push(NONE);
        self.event_ids.extend([self.events.len(),self.events.len()+1]);self.events.extend([SweepEvent {y:start,edge:id,add:true},SweepEvent {y:end,edge:id,add:false}]);Ok(())
    }
    fn inside(&self,winding:i32)->bool {match self.input.rule {CoverageRule::NonZero=>winding!=0,CoverageRule::EvenOdd=>winding.abs()%2==1}}
    fn step(&mut self)->Result<(),PixelEditError> {
        match self.stage {
            Stage::Prepare=>{
                let Some(points)=self.input.contours.get(self.contour) else {self.sorter=Some(MergeSort::new(std::mem::take(&mut self.event_ids)));self.stage=Stage::Events;return Ok(());};
                if self.point<points.len() {
                    let value=points[self.point];self.point+=1;if !value.into_iter().all(coordinate) {return Err(PixelEditError::Invalid("Invalid coverage point"));}
                    let m=self.input.transform;let p=[m[0]*value[0]+m[2]*value[1]+m[4],m[1]*value[0]+m[3]*value[1]+m[5]];
                    if !p.into_iter().all(coordinate) {return Err(PixelEditError::Invalid("Transformed coverage exceeds coordinate budget"));}
                    if let Some(previous)=self.previous {self.edge(previous,p)?;} else {self.first=Some(p);}self.previous=Some(p);
                } else {if let (Some(previous),Some(first))=(self.previous,self.first) {self.edge(previous,first)?;}self.contour+=1;self.point=0;self.first=None;self.previous=None;}
                self.prepared+=1;
            }
            Stage::Events=>{
                if self.sorter.as_ref().unwrap().done() {self.sorted=self.sorter.take().unwrap().source;self.stage=Stage::Apply;}
                else {let events=&self.events;self.sorter.as_mut().unwrap().step(|a,b|events[a].y.total_cmp(&events[b].y).then_with(||events[a].add.cmp(&events[b].add)));}
            }
            Stage::Apply=>{
                if self.y>=f64::from(self.input.height) {self.stage=Stage::Done;return Ok(());}
                let event=self.sorted.get(self.event).map(|id|self.events[*id]);
                if let Some(event)=event.filter(|event|event.y<=self.y) {
                    self.event+=1;
                    if event.add {self.positions[event.edge]=self.active.len();self.active.push(event.edge);}
                    else {let at=self.positions[event.edge];let last=self.active.pop().unwrap();if at<self.active.len() {self.active[at]=last;self.positions[last]=at;}self.positions[event.edge]=NONE;}
                } else {self.end=((self.row+1) as f64).min(event.map_or(f64::from(self.input.height),|event|event.y)).min(f64::from(self.input.height));self.copy=0;self.copied.clear();self.stage=Stage::Copy;}
            }
            Stage::Copy=>{
                if self.copy<self.active.len() {self.copied.push(self.active[self.copy]);self.copy+=1;}
                else {self.sorter=Some(MergeSort::new(std::mem::take(&mut self.copied)));self.stage=Stage::Sort;}
            }
            Stage::Sort=>{
                if self.sorter.as_ref().unwrap().done() {self.copied=self.sorter.take().unwrap().source;self.scan=0;self.stage=Stage::Cross;}
                else {let edges=&self.edges;let y=self.y;self.sorter.as_mut().unwrap().step(|a,b| {let x=edges[a];let z=edges[b];let delta=x_at(x,y)-x_at(z,y);if delta.abs()>EPS {delta.total_cmp(&0.0)} else {x.dx.total_cmp(&z.dx).then_with(||a.cmp(&b))}});}
            }
            Stage::Cross=>{
                if self.scan+1<self.copied.len() {
                    let left=self.edges[self.copied[self.scan]];self.scan+=1;let right=self.edges[self.copied[self.scan]];let slope=left.dx-right.dx;
                    if slope>0.0 {let cross=self.y+(x_at(right,self.y)-x_at(left,self.y))/slope;if cross>self.y+EPS&&cross<self.end {self.end=cross;}}
                } else {self.scan=0;self.winding=0;self.left=NONE;self.stage=Stage::Wind;}
            }
            Stage::Wind=>{
                if self.scan>=self.copied.len() {self.stage=Stage::Finish;return Ok(());}
                let id=self.copied[self.scan];self.scan+=1;let before=self.inside(self.winding);self.winding+=self.edges[id].winding;let after=self.inside(self.winding);
                if !before&&after {self.left=id;}
                if before&&!after {
                    self.span_left=self.left;self.span_right=id;let left=self.edges[self.left];let right=self.edges[id];
                    self.pixel=x_at(left,self.y).min(x_at(left,self.end)).floor().max(0.0) as usize;
                    self.pixel_end=x_at(right,self.y).max(x_at(right,self.end)).ceil().clamp(0.0,f64::from(self.input.width)) as usize;self.stage=Stage::Span;
                }
            }
            Stage::Span=>{
                if self.pixel>=self.pixel_end {self.stage=Stage::Wind;return Ok(());}
                let x=self.pixel;self.pixel+=1;let left=self.edges[self.span_left];let right=self.edges[self.span_right];let height=self.end-self.y;
                self.areas[x]+=integral(x_at(right,self.y)-x as f64,x_at(right,self.end)-x as f64,height)-integral(x_at(left,self.y)-x as f64,x_at(left,self.end)-x as f64,height);
                self.dirty_start=self.dirty_start.min(x);self.dirty_end=self.dirty_end.max(x+1);
            }
            Stage::Finish=>{self.y=self.end;if self.y>=(self.row+1) as f64 {self.pixel=self.dirty_start;self.stage=Stage::Write;} else {self.stage=Stage::Apply;}}
            Stage::Write=>{
                if self.pixel<self.dirty_end {let x=self.pixel;self.pixel+=1;self.mask.coverage[self.row*self.input.width as usize+x]=(self.areas[x].clamp(0.0,1.0)*255.0).round() as u8;self.areas[x]=0.0;}
                else {self.row+=1;self.dirty_start=self.input.width as usize;self.dirty_end=0;self.stage=Stage::Apply;}
            }
            Stage::Done=>{}
        }
        Ok(())
    }
    pub fn advance(&mut self,budget:usize)->Result<CoverageProgress,PixelEditError> {
        if budget==0||budget as u128>9_007_199_254_740_991 {return Err(PixelEditError::Invalid("Coverage work grant must be a positive integer"));}
        if self.cancelled {return Err(PixelEditError::Cancelled);}if let Some(error)=&self.failed {return Err(error.clone());}
        for _ in 0..budget {if self.stage==Stage::Done {break;}if let Err(error)=self.step() {self.failed=Some(error.clone());return Err(error);}self.work+=1;}
        let phase=match self.stage {Stage::Prepare=>CoveragePhase::Preparing,Stage::Events=>CoveragePhase::Sorting,Stage::Done=>CoveragePhase::Complete,_=>CoveragePhase::Rasterizing};
        let (completed,total)=match phase {CoveragePhase::Preparing=>(self.prepared,self.preparation_total),CoveragePhase::Sorting=>{let sort=self.sorter.as_ref().unwrap();let count=sort.source.len();let passes=if count<2 {0} else {usize::BITS-(count-1).leading_zeros()};(sort.writes,count*passes as usize)},_=>(self.row*self.input.width as usize,self.mask.coverage.len())};
        Ok(CoverageProgress {phase,completed,total,work:self.work,done:self.stage==Stage::Done})
    }
    pub fn cancel(&mut self) {self.cancelled=true;}
    pub fn result(&self)->Result<&CoverageMask,PixelEditError> {
        if self.cancelled {return Err(PixelEditError::Cancelled);}if let Some(error)=&self.failed {return Err(error.clone());}if self.stage!=Stage::Done {return Err(PixelEditError::Incomplete);}Ok(&self.mask)
    }
    pub fn into_result(self)->Result<CoverageMask,PixelEditError> {self.result()?;Ok(self.mask)}
    /// 🧹️ Transfers complete coverage and admits private contour and buffer retirement through work grants.
    pub fn into_retirement(mut self)->(CoverageRetirement,Option<CoverageMask>) {
        let output=if self.stage==Stage::Done&&!self.cancelled&&self.failed.is_none() {Some(CoverageMask {width:self.mask.width,height:self.mask.height,coverage:std::mem::take(&mut self.mask.coverage)})} else {None};
        self.cancelled=true;(CoverageRetirement::new(self),output)
    }
}

semio_framework_value::artifact_retire_leaf!(CoverageRule,CoveragePhase,Edge,SweepEvent,Stage);
physical_work_retirement!(CoverageRetirement,CoverageJob,&'static str,|_:&str|"Coverage retirement refused its physical grant");
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
