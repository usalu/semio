//! 🖼️ Affine sampling with premultiplied bilinear and per-cell area filtering.
use crate::{RasterImage,editing::{validate_extent,validate_image},coverage::{CoverageJob,CoverageInput,CoverageRule,CoverageMask,CoverageRetirement}};
use crate::retirement::RasterLease;
use semio_framework_2d::physical_work_retirement;
use std::sync::Arc;
use semio_framework_value::{work::RetainedWorkLedger,RetirementDemand};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum AffineSampling{Nearest,Bilinear,Area,Auto}
#[derive(Clone,Debug)]
pub struct AffineImageInput{pub source:Arc<RasterImage>,pub width:u32,pub height:u32,pub origin:[f64;2],pub transform:[f64;6],pub sampling:AffineSampling}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct AffineImageProgress{pub phase:&'static str,pub completed:usize,pub total:usize,pub sampled:usize,pub sample_total:usize,pub work:u64,pub done:bool}
#[derive(Clone,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub enum AffineImageError{Invalid(&'static str),Pixels(crate::editing::PixelEditError),Incomplete,Cancelled}
impl std::fmt::Display for AffineImageError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Invalid(v)=>f.write_str(v),Self::Pixels(v)=>std::fmt::Display::fmt(v,f),Self::Incomplete=>f.write_str("Affine sample job is incomplete"),Self::Cancelled=>f.write_str("Affine sample job cancelled")}}}
impl std::error::Error for AffineImageError{}
fn invalid(v:&'static str)->AffineImageError{AffineImageError::Invalid(v)}
impl AffineImageError{pub fn message(&self)->&str{match self{Self::Invalid(v)=>v,Self::Pixels(v)=>v.message(),Self::Incomplete=>"Affine sample job is incomplete",Self::Cancelled=>"Affine sample job cancelled"}}}
fn coordinate(v:f64)->bool{v.is_finite()&&v.abs()<=1e9}
fn byte(v:f64)->u8{v.clamp(0.0,255.0).round()as u8}
/// ⏱️ Exact image boundary coverage with one source-cell visit per reduction work unit.
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
pub struct AffineImageJob{
 #[doc(hidden)] pub source:Option<RasterLease>,#[doc(hidden)] pub output:RasterImage,#[doc(hidden)] pub coverage:Option<CoverageJob>,#[doc(hidden)] pub mask:Option<CoverageMask>,inverse:Option<[f64;6]>,determinant:f64,filter:AffineSampling,
 #[doc(hidden)] pub phase:&'static str,#[doc(hidden)] pub at:usize,#[doc(hidden)] pub work:u64,cancelled:bool,#[doc(hidden)] pub failed:Option<AffineImageError>,sampled:usize,sample_total:usize,active:bool,x:usize,y:usize,left:usize,right:usize,bottom:usize,
 pending:RetirementDemand,last_work:u64,last_semantic:u64,step_work:u64,step_semantic:u64,transform:[f64;6],shape:Vec<[f64;2]>,contours:Vec<Vec<[f64;2]>>,pending_write:Option<f64>,quad:[[f64;2];4],scratch_a:[[f64;2];12],scratch_b:[[f64;2];12],sums:[f64;4],
 #[doc(hidden)] pub coverage_retirement:Option<CoverageRetirement>,
}
impl AffineImageJob{
 pub fn new(input:AffineImageInput)->Self{
  let admitted=(||{validate_extent(input.width,input.height).map_err(AffineImageError::Pixels)?;validate_image(&input.source).map_err(AffineImageError::Pixels)?;if !input.origin.into_iter().all(coordinate)||!input.transform.into_iter().all(coordinate){return Err(invalid("Invalid affine image contract"));}let mut m=input.transform;m[4]-=input.origin[0];m[5]-=input.origin[1];if !m.into_iter().all(coordinate){return Err(invalid("Affine viewport exceeds coordinate budget"));}let[a,b,c,d,e,f]=m;let det=a*d-b*c;let mut filter=input.sampling;let inverse=if det!=0.0{let inverse=[d/det,-b/det,-c/det,a/det,(c*f-d*e)/det,(b*e-a*f)/det];if !inverse.into_iter().all(f64::is_finite){return Err(invalid("Affine inverse exceeds numeric limits"));}if filter==AffineSampling::Auto{let u=inverse[0]*inverse[0]+inverse[1]*inverse[1];let v=inverse[2]*inverse[2]+inverse[3]*inverse[3];let cross=inverse[0]*inverse[2]+inverse[1]*inverse[3];filter=if(u+v+(u-v).hypot(2.0*cross))/2.0>1.0+1e-12{AffineSampling::Area}else{AffineSampling::Bilinear};}Some(inverse)}else{None};Ok((m,inverse,det.abs(),filter))})();let(transform,inverse,determinant,filter,failed)=match admitted{Ok((m,i,d,f))=>(m,i,d,f,None),Err(error)=>(input.transform,None,0.0,input.sampling,Some(error))};
  Self{source:Some(RasterLease(input.source)),output:RasterImage{width:input.width,height:input.height,pixels:Vec::new()},coverage:None,coverage_retirement:None,mask:None,inverse,determinant,filter,phase:"preparing",at:0,work:0,cancelled:false,failed,sampled:0,sample_total:0,active:false,x:0,y:0,left:0,right:0,bottom:0,quad:[[0.0;2];4],scratch_a:[[0.0;2];12],scratch_b:[[0.0;2];12],sums:[0.0;4],pending:RetirementDemand{depth:1,..Default::default()},last_work:0,last_semantic:0,step_work:0,step_semantic:0,transform,shape:Vec::new(),contours:Vec::new(),pending_write:None}
 }
 fn prepare(&mut self,work:&mut RetainedWorkLedger)->Result<bool,AffineImageError>{let count=self.output.width as usize*self.output.height as usize*4;if self.output.pixels.capacity()<count{if !work.reserve_vec(&mut self.output.pixels,count).map_err(invalid)?{return Ok(false);}}if self.output.pixels.len()<count{let n=(count-self.output.pixels.len()).min(64);if !work.copy_bytes(n){return Ok(false);}self.output.pixels.resize(self.output.pixels.len()+n,0);self.step_work=1;return Ok(false);}if self.inverse.is_none(){self.phase="sampling";self.step_work=1;return Ok(false);}if self.contours.is_empty()&&self.shape.len()<4{if self.shape.capacity()<4&&!work.reserve_vec(&mut self.shape,4).map_err(invalid)?{return Ok(false);}if !work.vec_slot(&mut self.shape).map_err(invalid)?{return Ok(false);}let source=self.source.as_ref().ok_or(AffineImageError::Invalid("source absent"))?;let(w,h)=(source.width as f64,source.height as f64);let point=[[0.0,0.0],[w,0.0],[w,h],[0.0,h]][self.shape.len()];work.vec_push(&mut self.shape,point).map_err(|(e,_)|invalid(e))?;self.step_work=1;return Ok(false);}if self.contours.is_empty(){if self.contours.capacity()==0&&!work.reserve_vec(&mut self.contours,1).map_err(invalid)?{return Ok(false);}if !work.vec_slot(&mut self.contours).map_err(invalid)?{return Ok(false);}if let Err((error,value))=work.vec_push(&mut self.contours,std::mem::take(&mut self.shape)){self.shape=value;return Err(invalid(error));}self.step_work=1;return Ok(false);}self.coverage=Some(CoverageJob::new(CoverageInput{width:self.output.width,height:self.output.height,transform:self.transform,rule:CoverageRule::NonZero,contours:std::mem::take(&mut self.contours)}));self.phase="coverage";self.step_work=1;Ok(false)
 }
 fn point(&self,x:f64,y:f64)->Result<[f64;2],AffineImageError>{let m=self.inverse.unwrap();let p=[m[0]*x+m[2]*y+m[4],m[1]*x+m[3]*y+m[5]];if !p.into_iter().all(f64::is_finite){return Err(invalid("Sample footprint exceeds numeric limits"));}Ok(p)}
 fn write(&mut self,alpha:f64){self.pending_write=Some(alpha);self.active=false;}
 fn publish(&mut self,work:&mut RetainedWorkLedger)->Result<bool,AffineImageError>{if !work.copy_bytes(4){return Ok(false);}let at=self.at*4;let a=byte(self.pending_write.ok_or(AffineImageError::Invalid("pending_write absent"))?);if a>0&&self.sums[3]>0.0{for c in 0..3{self.output.pixels[at+c]=byte(self.sums[c]/self.sums[3]);}self.output.pixels[at+3]=a;}self.pending_write=None;self.at+=1;if self.at==self.output.width as usize*self.output.height as usize{self.phase="complete";}self.step_work=1;Ok(false)}
 fn accumulate(&mut self,x:usize,y:usize,weight:f64){let source=self.source.as_ref().unwrap();let at=(y*source.width as usize+x)*4;let alpha=f64::from(source.pixels[at+3])*weight;self.sums[3]+=alpha;for c in 0..3{self.sums[c]+=f64::from(source.pixels[at+c])*alpha;}}
 fn area(&mut self,x:usize,y:usize)->f64{
  self.scratch_a[..4].copy_from_slice(&self.quad);let mut count=4;
  for side in 0..4{if count==0{break;}let axis=if side<2{0}else{1};let bound=match side{0=>x,1=>x+1,2=>y,_=>y+1}as f64;let lower=side==0||side==2;let mut n=0;
   for i in 0..count{let p=self.scratch_a[(i+count-1)%count];let q=self.scratch_a[i];let inside_p=if lower{p[axis]>=bound}else{p[axis]<=bound};let inside_q=if lower{q[axis]>=bound}else{q[axis]<=bound};
    if inside_p!=inside_q{let t=(bound-p[axis])/(q[axis]-p[axis]);self.scratch_b[n]=[p[0]+t*(q[0]-p[0]),p[1]+t*(q[1]-p[1])];n+=1;}
    if inside_q{self.scratch_b[n]=q;n+=1;}
   }count=n;std::mem::swap(&mut self.scratch_a,&mut self.scratch_b);
  }
  let mut area=0.0;for i in 1..count.saturating_sub(1){let a=self.scratch_a[0];let p=self.scratch_a[i];let q=self.scratch_a[i+1];area+=(p[0]-a[0])*(q[1]-a[1])-(p[1]-a[1])*(q[0]-a[0]);}area.abs()/2.0
 }
 fn step(&mut self,work:&mut RetainedWorkLedger)->Result<(),AffineImageError>{
  if self.phase=="preparing"{self.prepare(work)?;return Ok(());}
  if self.pending_write.is_some(){self.publish(work)?;return Ok(());}
  if self.phase=="coverage"{let child=self.coverage.as_mut().ok_or(AffineImageError::Invalid("coverage absent"))?;if child.result().is_ok(){let(retired,mask)=self.coverage.take().ok_or(AffineImageError::Invalid("coverage absent"))?.into_retirement();self.coverage_retirement=Some(retired);self.mask=mask;self.phase="coverageCleanup";self.step_work=1;return Ok(());}let result=child.advance(1,work);self.step_work=child.last_turn_work();self.step_semantic=child.last_turn_semantic_work();result.map_err(AffineImageError::Pixels)?;return Ok(());}
  if self.phase=="coverageCleanup"{self.phase="sampling";self.step_work=1;return Ok(());}
  self.step_work=1;self.step_semantic=1;
  if self.active{let area=self.area(self.x,self.y);self.accumulate(self.x,self.y,area);self.sampled+=1;self.x+=1;if self.x>=self.right{self.x=self.left;self.y+=1;}if self.y>=self.bottom{self.write(self.sums[3]*self.determinant);}return Ok(());}
  let coverage=self.mask.as_ref().map(|m|m.coverage[self.at]).unwrap_or(0);self.sums=[0.0;4];if self.inverse.is_none()||coverage==0{self.write(0.0);return Ok(());}
  let source=self.source.as_ref().unwrap();let sw=source.width as usize;let sh=source.height as usize;let x=(self.at%self.output.width as usize)as f64;let y=(self.at/self.output.width as usize)as f64;
  if self.filter==AffineSampling::Area{let mut min=[f64::INFINITY;2];let mut max=[f64::NEG_INFINITY;2];for i in 0..4{let p=self.point(x+if i==1||i==2{1.0}else{0.0},y+if i>=2{1.0}else{0.0})?;self.quad[i]=p;for axis in 0..2{min[axis]=min[axis].min(p[axis]);max[axis]=max[axis].max(p[axis]);}}
   self.left=min[0].floor().clamp(0.0,sw as f64)as usize;self.right=max[0].ceil().clamp(0.0,sw as f64)as usize;self.y=min[1].floor().clamp(0.0,sh as f64)as usize;self.bottom=max[1].ceil().clamp(0.0,sh as f64)as usize;self.x=self.left;self.sampled=0;self.sample_total=(self.right-self.left)*(self.bottom-self.y);self.active=self.sample_total>0;if !self.active{self.write(0.0);}return Ok(());
  }
  let p=self.point(x+0.5,y+0.5)?;let clamp_x=|v:f64|v.clamp(0.0,(sw-1)as f64);let clamp_y=|v:f64|v.clamp(0.0,(sh-1)as f64);
  if self.filter==AffineSampling::Nearest{self.accumulate(clamp_x(p[0].floor())as usize,clamp_y(p[1].floor())as usize,1.0);}
  else{let sx=clamp_x(p[0]-0.5);let sy=clamp_y(p[1]-0.5);let left=sx.floor()as usize;let top=sy.floor()as usize;let dx=sx-left as f64;let dy=sy-top as f64;self.accumulate(left,top,(1.0-dx)*(1.0-dy));self.accumulate((left+1).min(sw-1),top,dx*(1.0-dy));self.accumulate(left,(top+1).min(sh-1),(1.0-dx)*dy);self.accumulate((left+1).min(sw-1),(top+1).min(sh-1),dx*dy);}
  self.write(self.sums[3]*f64::from(coverage)/255.0);Ok(())
 }
 fn check(&self)->Result<(),AffineImageError>{if self.cancelled{return Err(AffineImageError::Cancelled);}if let Some(e)=&self.failed{return Err(e.clone());}Ok(())}
 pub fn progress(&self)->AffineImageProgress{AffineImageProgress{phase:self.phase,completed:self.at,total:self.output.width as usize*self.output.height as usize,sampled:self.sampled,sample_total:self.sample_total,work:self.work,done:self.phase=="complete"}}
 pub fn next_work_demand(&self)->RetirementDemand{self.pending}
 pub fn last_turn_work(&self)->u64{self.last_work}
 pub fn last_turn_semantic_work(&self)->u64{self.last_semantic}
 pub fn failure_pending(&self)->bool{self.failed.is_some()}
 pub fn settlement_pending(&self)->bool{self.phase=="complete"||self.pending_write.is_some()||self.phase=="coverageCleanup"||self.coverage.as_ref().is_some_and(|child|child.result().is_ok()||child.settlement_pending())}
 pub fn advance(&mut self,budget:usize,work:&mut RetainedWorkLedger)->Result<AffineImageProgress,AffineImageError>{self.last_work=0;self.last_semantic=0;if budget==0||budget as u64>9_007_199_254_740_991{return Err(invalid("Affine sample work grant must be a positive integer"));}self.check()?;if !work.admits_absolute(self.pending){work.require_absolute(self.pending);return Ok(self.progress());}let Some(root)=work.enter()else{self.pending=work.next_demand();return Ok(self.progress());};let result=(||{for _ in 0..budget{if self.phase=="complete"||!work.can_step(){break;}self.step_work=0;self.step_semantic=0;let before=work.receipt();let result=self.step(work);if self.step_work>0||work.receipt()!=before{self.last_work+=self.step_work.max(1);work.note_source(root);}self.work+=self.step_semantic;self.last_semantic+=self.step_semantic;self.pending=work.next_demand();if let Err(error)=result{self.failed=Some(error.clone());return Err(error);}if work.blocked(){break;}}Ok(self.progress())})();work.exit();result}
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn result(&self)->Result<&RasterImage,AffineImageError>{self.check()?;if self.phase!="complete"{return Err(AffineImageError::Incomplete);}Ok(&self.output)}
 /// 🧹️ Moves complete pixels and schedules the genuine coverage and sampling owners.
 pub fn into_retirement(mut self)->(AffineImageRetirement,Option<RasterImage>){
  let output=if !self.cancelled&&self.failed.is_none()&&self.phase=="complete"{Some(RasterImage{width:self.output.width,height:self.output.height,pixels:std::mem::take(&mut self.output.pixels)})}else{None};self.cancelled=true;
  if let Some(mut child)=self.coverage.take(){child.cancel();self.coverage_retirement=Some(child.into_retirement().0);}
  (AffineImageRetirement::new(self),output)
 }
}
semio_framework_value::artifact_retire_leaf!(AffineSampling);
physical_work_retirement!(AffineImageRetirement,AffineImageJob);
