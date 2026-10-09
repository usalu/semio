//! 📉️ Work-granted Douglas–Peucker simplification preserves authored curve segments.
use crate::PathSegment;
use semio_framework_value::list::PagedList;
type Segments=PagedList<PathSegment,{usize::MAX}>;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PathSimplifyPhase {Scanning,Reducing,Building,Complete}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct PathSimplifyProgress {pub phase:PathSimplifyPhase,pub completed:usize,pub total:usize,pub work:u64,pub done:bool}
#[derive(semio_framework_value::RetireOwned)]
struct Span {start:usize,end:usize,next:usize,farthest:usize,distance:f64,force:u8}
#[derive(Clone,Copy,semio_framework_value::RetireOwned)]
enum SimplifyFailure {Geometry,Contour,Capacity}
impl SimplifyFailure {fn message(self)->&'static str {match self {Self::Geometry=>"Invalid path geometry",Self::Contour=>"A contour must start with a move",Self::Capacity=>"Path simplification exceeds work capacity"}}}
fn distance(p:[f64;2],a:[f64;2],b:[f64;2])->f64 {
    let scale=p.into_iter().chain(a).chain(b).map(f64::abs).fold(1.0,f64::max);
    let a=a.map(|v|v/scale);let b=b.map(|v|v/scale);let p=p.map(|v|v/scale);
    let d=[b[0]-a[0],b[1]-a[1]];let length=d[0]*d[0]+d[1]*d[1];
    let t=if length==0.0 {0.0}else {(((p[0]-a[0])*d[0]+(p[1]-a[1])*d[1])/length).clamp(0.0,1.0)};
    (p[0]-a[0]-t*d[0]).hypot(p[1]-a[1]-t*d[1])*scale
}
/// ⏱️ Owns a private candidate until every line-run scan and distance test has completed.
#[derive(semio_framework_value::RetireOwned)]
pub struct PathSimplifyJob {source:Segments,keep:Vec<bool>,output:Segments,stack:PagedList<Span,{usize::MAX}>,span:Option<Span>,at:usize,run:Option<usize>,build:usize,tolerance:f64,work:u64,done:bool,cancelled:bool,contour:bool,failed:Option<SimplifyFailure>}
impl PathSimplifyJob {
    pub fn new(source:Vec<PathSegment>,tolerance:f64)->Result<Self,&'static str> {
        if !tolerance.is_finite()||!(1e-6..=1e6).contains(&tolerance)||source.len()>65536 {return Err("Invalid path simplification contract");}
        Self::new_paged(source.into(),tolerance)
    }
    pub fn new_paged(source:Segments,tolerance:f64)->Result<Self,&'static str> {
        if !tolerance.is_finite()||!(1e-6..=1e6).contains(&tolerance)||source.len()>65536 {return Err("Invalid path simplification contract");}
        let keep=vec![true;source.len()];
        Ok(Self {source,keep,output:Default::default(),stack:Default::default(),span:None,at:0,run:None,build:0,tolerance,work:0,done:false,cancelled:false,contour:false,failed:None})
    }
    fn point(&self,index:usize)->[f64;2] {super::endpoint(&self.source[index]).expect("admitted line-run endpoint")}
    fn span(start:usize,end:usize,force:u8)->Span {Span {start,end,next:start+1,farthest:start,distance:0.0,force}}
    fn step(&mut self)->Result<(),&'static str> {
        if self.span.is_none() {self.span=self.stack.pop();}
        if let Some(mut span)=self.span.take() {
            if span.next<span.end {
                let value=distance(self.point(span.next),self.point(span.start),self.point(span.end));
                if value>span.distance {span.distance=value;span.farthest=span.next;}
                span.next+=1;self.span=Some(span);return Ok(());
            }
            if span.farthest>span.start && (span.distance>self.tolerance || span.force>0) {
                self.keep[span.farthest]=true;let force=span.force.saturating_sub(1);
                self.stack.push(Self::span(span.farthest,span.end,force));self.stack.push(Self::span(span.start,span.farthest,force));
            }
            return Ok(());
        }
        if self.at<self.source.len() {
            let item=&self.source[self.at];
            if !matches!(item,PathSegment::Line {..}) {if let Some(start)=self.run.take() {
                let end=self.at-1;let force=if matches!(item,PathSegment::Close)&&matches!(self.source[start],PathSegment::Move {..}) {if self.point(start)==self.point(end) {2}else {1}}else {0};
                self.stack.push(Self::span(start,end,force));return Ok(());
            }}
            if !crate::schema::valid_path_segment(item) {return Err("Invalid path geometry");}
            match item {PathSegment::Move {..}=>self.contour=true,_ if !self.contour=>return Err("A contour must start with a move"),PathSegment::Close=>self.contour=false,_=>{}}
            if matches!(item,PathSegment::Line {..}) {
                if self.at==0||matches!(self.source[self.at-1],PathSegment::Close) {return Err("A contour must start with a move");}
                if self.run.is_none() {self.run=Some(self.at-1);}
                if self.at+1<self.source.len()&&matches!(self.source[self.at+1],PathSegment::Line {..}) {self.keep[self.at]=false;}
            }
            self.at+=1;return Ok(());
        }
        if let Some(start)=self.run.take() {self.stack.push(Self::span(start,self.at-1,0));return Ok(());}
        if self.build<self.source.len() {if self.keep[self.build] {self.output.push(self.source[self.build].clone());}self.build+=1;return Ok(());}
        self.done=true;Ok(())
    }
    pub fn advance(&mut self,grant:usize)->Result<PathSimplifyProgress,&'static str> {
        if grant==0 {return Err("Path simplification requires a positive work grant");}
        if self.cancelled {return Err("Path simplification cancelled");}
        if let Some(error)=self.failed {return Err(error.message());}
        for _ in 0..grant {if self.done {break;}if self.work>=10000000 {self.failed=Some(SimplifyFailure::Capacity);return Err(SimplifyFailure::Capacity.message());}if let Err(error)=self.step() {self.failed=Some(if error=="A contour must start with a move" {SimplifyFailure::Contour}else {SimplifyFailure::Geometry});return Err(error);}self.work+=1;}
        Ok(PathSimplifyProgress {phase:if self.done {PathSimplifyPhase::Complete}else if self.span.is_some()||!self.stack.is_empty() {PathSimplifyPhase::Reducing}else if self.at==self.source.len() {PathSimplifyPhase::Building}else {PathSimplifyPhase::Scanning},completed:self.at,total:self.source.len(),work:self.work,done:self.done})
    }
    pub fn cancel(&mut self) {self.cancelled=true;}
    pub fn result(&self)->Result<&Segments,&'static str> {if self.cancelled {Err("Path simplification cancelled")}else if let Some(error)=self.failed {Err(error.message())}else if !self.done {Err("Path simplification is incomplete")}else {Ok(&self.output)}}
    pub fn take_result(&mut self)->Result<Segments,&'static str> {self.result()?;Ok(std::mem::take(&mut self.output))}
    pub fn into_result(self)->Result<Vec<PathSegment>,&'static str> {self.result()?;Ok(self.output.into_iter().collect())}
}
