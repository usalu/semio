//! 🖼️ Affine sampling with premultiplied bilinear and per-cell area filtering.
use crate::{RasterImage,editing::{validate_extent,validate_image},coverage::{CoverageJob,CoverageInput,CoverageRule,CoverageMask,CoverageRetirement}};
use crate::retirement::RasterLease;
use semio_framework_2d::physical_work_retirement;
use std::sync::Arc;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum AffineSampling{Nearest,Bilinear,Area,Auto}
#[derive(Clone,Debug)]
pub struct AffineImageInput{pub source:Arc<RasterImage>,pub width:u32,pub height:u32,pub origin:[f64;2],pub transform:[f64;6],pub sampling:AffineSampling}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct AffineImageProgress{pub phase:&'static str,pub completed:usize,pub total:usize,pub sampled:usize,pub sample_total:usize,pub work:u64,pub done:bool}
#[derive(Clone,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub enum AffineImageError{Invalid(String),Incomplete,Cancelled}
impl std::fmt::Display for AffineImageError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Invalid(v)=>f.write_str(v),Self::Incomplete=>f.write_str("Affine sample job is incomplete"),Self::Cancelled=>f.write_str("Affine sample job cancelled")}}}
impl std::error::Error for AffineImageError{}
fn invalid(v:impl ToString)->AffineImageError{AffineImageError::Invalid(v.to_string())}
fn coordinate(v:f64)->bool{v.is_finite()&&v.abs()<=1e9}
fn byte(v:f64)->u8{v.clamp(0.0,255.0).round()as u8}
/// ⏱️ Exact image boundary coverage with one source-cell visit per reduction work unit.
#[derive(semio_framework_value::RetireOwned)]
pub struct AffineImageJob{
 source:Option<RasterLease>,output:RasterImage,coverage:Option<CoverageJob>,mask:Option<CoverageMask>,inverse:Option<[f64;6]>,determinant:f64,filter:AffineSampling,
 phase:&'static str,at:usize,work:u64,cancelled:bool,failed:Option<AffineImageError>,sampled:usize,sample_total:usize,active:bool,x:usize,y:usize,left:usize,right:usize,bottom:usize,
 quad:[[f64;2];4],scratch_a:[[f64;2];12],scratch_b:[[f64;2];12],sums:[f64;4],
 coverage_retirement:Option<CoverageRetirement>,
}
impl AffineImageJob{
 pub fn new(input:AffineImageInput)->Result<Self,AffineImageError>{
  let count=validate_extent(input.width,input.height).map_err(invalid)?;validate_image(&input.source).map_err(invalid)?;
  if !input.origin.into_iter().all(coordinate)||!input.transform.into_iter().all(coordinate){return Err(invalid("Invalid affine image contract"));}
  let mut m=input.transform;m[4]-=input.origin[0];m[5]-=input.origin[1];if !m.into_iter().all(coordinate){return Err(invalid("Affine viewport exceeds coordinate budget"));}
  let [a,b,c,d,e,f]=m;let det=a*d-b*c;let mut filter=input.sampling;
  let inverse=if det!=0.0{let inverse=[d/det,-b/det,-c/det,a/det,(c*f-d*e)/det,(b*e-a*f)/det];if !inverse.into_iter().all(f64::is_finite){return Err(invalid("Affine inverse exceeds numeric limits"));}if filter==AffineSampling::Auto{let u=inverse[0]*inverse[0]+inverse[1]*inverse[1];let v=inverse[2]*inverse[2]+inverse[3]*inverse[3];let cross=inverse[0]*inverse[2]+inverse[1]*inverse[3];filter=if (u+v+(u-v).hypot(2.0*cross))/2.0>1.0+1e-12{AffineSampling::Area}else{AffineSampling::Bilinear};}Some(inverse)}else{None};
  let coverage=if inverse.is_some(){let w=f64::from(input.source.width);let h=f64::from(input.source.height);Some(CoverageJob::new(CoverageInput{width:input.width,height:input.height,transform:m,rule:CoverageRule::NonZero,contours:vec![vec![[0.0,0.0],[w,0.0],[w,h],[0.0,h]]]}).map_err(invalid)?)}else{None};
  Ok(Self{source:Some(RasterLease(input.source)),output:RasterImage{width:input.width,height:input.height,pixels:vec![0;count*4]},coverage,coverage_retirement:None,mask:None,inverse,determinant:det.abs(),filter,phase:if inverse.is_some(){"coverage"}else{"sampling"},at:0,work:0,cancelled:false,failed:None,sampled:0,sample_total:0,active:false,x:0,y:0,left:0,right:0,bottom:0,quad:[[0.0;2];4],scratch_a:[[0.0;2];12],scratch_b:[[0.0;2];12],sums:[0.0;4]})
 }
 fn point(&self,x:f64,y:f64)->Result<[f64;2],AffineImageError>{let m=self.inverse.unwrap();let p=[m[0]*x+m[2]*y+m[4],m[1]*x+m[3]*y+m[5]];if !p.into_iter().all(f64::is_finite){return Err(invalid("Sample footprint exceeds numeric limits"));}Ok(p)}
 fn write(&mut self,alpha:f64){let at=self.at*4;let a=byte(alpha);if a>0&&self.sums[3]>0.0{for c in 0..3{self.output.pixels[at+c]=byte(self.sums[c]/self.sums[3]);}self.output.pixels[at+3]=a;}self.at+=1;self.active=false;if self.at==self.output.width as usize*self.output.height as usize{self.phase="complete";}}
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
 fn step(&mut self)->Result<(),AffineImageError>{
  if self.phase=="coverage"{if self.coverage.as_mut().unwrap().advance(1).map_err(invalid)?.done{let(retired,mask)=self.coverage.take().unwrap().into_retirement();self.coverage_retirement=Some(retired);self.mask=mask;self.phase="coverageCleanup";}return Ok(());}
  if self.phase=="coverageCleanup"{self.phase="sampling";return Ok(());}
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
 pub fn advance(&mut self,budget:usize)->Result<AffineImageProgress,AffineImageError>{
  if budget==0||budget as u64>9_007_199_254_740_991{return Err(invalid("Affine sample work grant must be a positive integer"));}self.check()?;
  for _ in 0..budget{if self.phase=="complete"{break;}if let Err(e)=self.step(){self.failed=Some(e.clone());return Err(e);}self.work+=1;}
  Ok(AffineImageProgress{phase:self.phase,completed:self.at,total:self.output.width as usize*self.output.height as usize,sampled:self.sampled,sample_total:self.sample_total,work:self.work,done:self.phase=="complete"})
 }
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
physical_work_retirement!(AffineImageRetirement,AffineImageJob,AffineImageError,invalid);
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
