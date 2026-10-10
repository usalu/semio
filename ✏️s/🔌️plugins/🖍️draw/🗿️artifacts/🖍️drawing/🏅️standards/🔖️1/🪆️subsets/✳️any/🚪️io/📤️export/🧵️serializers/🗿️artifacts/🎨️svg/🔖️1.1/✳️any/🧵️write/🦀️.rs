//! 🧵️ Resolved scene custody emits scalar SVG fragments and segmented byte owners.
use crate::schema::scene_preparation::{DocumentScenePlan,DocumentSceneContent};
use crate::{FillStyle,PathSegment};
use semio_framework_pixels::{png_encoding::PngEncodeJob,retirement::RasterLease};
use semio_framework_value::{list::PagedList,retirement::{RetireOwned,RetirementCursor}};
use std::fmt::Write;
use crate::standards::v1::subsets::any::io::byte_writer::{Fragment,PAGE,reserve};
type Pages=PagedList<Vec<u8>,{usize::MAX}>;
type Encoder=PngEncodeJob<RasterLease>;
#[derive(Clone,Copy)]
pub struct SvgWriteLimits{pub view_box:[f64;4],pub max_output_bytes:usize,pub max_work:u64}
#[derive(Clone,Copy,Debug)]
pub struct SvgWriteProgress{pub phase:&'static str,pub nodes:usize,pub bytes:usize,pub work:u64,pub done:bool}
#[derive(semio_framework_value::RetireOwned)]
struct Owners{plan:DocumentScenePlan,pages:Pages,encoder:Option<Encoder>,image_bytes:Vec<u8>,retired_encoders:PagedList<Encoder,{usize::MAX}>,retired_bytes:Pages,rejected_image:Option<RasterLease>,failure:Option<String>}
pub struct SvgWriteJob{owners:Owners,limits:SvgWriteLimits,pending:Fragment,phase:u8,node:usize,common:usize,depth:usize,groups:[(usize,usize);32],scalar:usize,part:usize,asset:usize,stops:[usize;64],stop_count:usize,constant:Option<[f64;4]>,image_at:usize,bytes:usize,work:u64,cancelled:bool}
impl RetireOwned for SvgWriteJob{
 fn retirement(self)->Box<dyn RetirementCursor>{self.owners.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.owners.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{true}
}
semio_framework_2d::physical_work_retirement!(SvgWriteRetirement,SvgWriteJob,String,|value:&str|value.to_owned());
fn escaped(out:&mut Fragment,at:&mut usize,value:&str)->Result<bool,std::fmt::Error>{let Some(c)=value.get(*at..).and_then(|value|value.chars().next())else{*at=0;return Ok(true);};if matches!(c,'\u{0}'..='\u{8}'|'\u{b}'|'\u{c}'|'\u{e}'..='\u{1f}'|'\u{fffe}'|'\u{ffff}'){return Err(std::fmt::Error);}*at+=c.len_utf8();match c{'&'=>out.write_str("&amp;")?,'"'=>out.write_str("&quot;")?,'<'=>out.write_str("&lt;")?,'>'=>out.write_str("&gt;")?,_=>out.write_char(c)?};Ok(false)}
fn rgb(out:&mut Fragment,color:&[f64;4])->std::fmt::Result{write!(out,"rgb({},{},{})",color[0]*255.0,color[1]*255.0,color[2]*255.0)}
fn blend(value:&str)->&str{match value{"colorDodge"=>"color-dodge","colorBurn"=>"color-burn","hardLight"=>"hard-light","softLight"=>"soft-light",_=>value}}
fn path(out:&mut Fragment,segment:&PathSegment)->std::fmt::Result{match segment{PathSegment::Move{to:[x,y]}=>write!(out,"M{x} {y} "),PathSegment::Line{to:[x,y]}=>write!(out,"L{x} {y} "),PathSegment::Quad{ctrl:[x1,y1],to:[x,y]}=>write!(out,"Q{x1} {y1} {x} {y} "),PathSegment::Cubic{ctrl1:[x1,y1],ctrl2:[x2,y2],to:[x,y]}=>write!(out,"C{x1} {y1} {x2} {y2} {x} {y} "),PathSegment::Arc{rx,ry,rotation,large_arc,sweep,to:[x,y]}=>write!(out,"A{rx} {ry} {rotation} {} {} {x} {y} ",u8::from(*large_arc),u8::from(*sweep)),PathSegment::Close=>out.write_str("Z ")}}
impl SvgWriteJob{
 pub fn new(plan:DocumentScenePlan,limits:SvgWriteLimits)->Self{let failure=(!limits.view_box.iter().all(|n|n.is_finite())||limits.view_box[2]<=0.0||limits.view_box[3]<=0.0||!(64..=67108864).contains(&limits.max_output_bytes)||!(1..=1000000000).contains(&limits.max_work)).then(||"Invalid SVG writer limits".into());Self{owners:Owners{plan,pages:Pages::new(),encoder:None,image_bytes:Vec::new(),retired_encoders:PagedList::new(),retired_bytes:Pages::new(),rejected_image:None,failure},limits,pending:Fragment::default(),phase:0,node:0,common:0,depth:0,groups:[(0,0);32],scalar:0,part:0,asset:0,stops:[0;64],stop_count:0,constant:None,image_at:0,bytes:0,work:0,cancelled:false}}
 fn done(&self)->bool{self.phase==30&&self.pending.at==self.pending.len}
 fn step(&mut self)->Result<(),String>{
  if self.pending.at<self.pending.len{let full=self.owners.pages.last().is_none_or(|page|page.len()==page.capacity());if full{if self.bytes==self.limits.max_output_bytes{return Err("SVG output byte limit exceeded".into());}if !reserve(&mut self.owners.pages)?{return Ok(());}let mut page=Vec::new();page.try_reserve_exact(PAGE.min(self.limits.max_output_bytes-self.bytes)).map_err(|_|"SVG page allocation refused")?;self.owners.pages.push_reserved(page).unwrap_or_else(|_|unreachable!());return Ok(());}let page=self.owners.pages.last_mut().unwrap();let count=(self.pending.len-self.pending.at).min(page.capacity()-page.len());page.extend_from_slice(&self.pending.bytes[self.pending.at..self.pending.at+count]);self.pending.at+=count;self.bytes+=count;return Ok(());}
  self.pending.len=0;self.pending.at=0;let out=&mut self.pending;let plan=&self.owners.plan;
  let node=plan.nodes.get(self.node);
  let formatting=(||->std::fmt::Result{match self.phase{
   0=>{let[x,y,w,h]=self.limits.view_box;write!(out,"<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"{w}\" height=\"{h}\" viewBox=\"{x} {y} {w} {h}\">")?;self.phase=1;},
   1=>{if let Some(node)=node{if node.groups.len()>32{self.phase=32;return Ok(());}if !node.visible||node.opacity<=0.0||matches!(node.content,DocumentSceneContent::Group{..}){self.node+=1;}else{self.common=0;self.phase=2;}}else{self.phase=29;}},
   2=>{let node=node.unwrap();if self.common<self.depth&&self.common<node.groups.len(){let(n,g)=self.groups[self.common];if plan.nodes[n].groups[g]==node.groups[self.common]{self.common+=1;return Ok(());}}self.phase=3;},
   3=>{if self.depth>self.common{self.depth-=1;out.write_str("</g>")?;}else{self.phase=4;}},
   4=>{if self.depth<node.unwrap().groups.len(){out.write_str("<g data-group-id=\"")?;self.phase=5;}else{out.write_str("<g data-layer-id=\"")?;self.phase=7;}},
   5=>{if escaped(out,&mut self.scalar,&node.unwrap().groups[self.depth].id)?{self.phase=6;}},
   6=>{let group=&node.unwrap().groups[self.depth];write!(out,"\" opacity=\"{}\" style=\"isolation:isolate;mix-blend-mode:{}\">",group.opacity,blend(&group.blend_mode))?;self.groups[self.depth]=(self.node,self.depth);self.depth+=1;self.phase=4;},
   7=>{if escaped(out,&mut self.scalar,&node.unwrap().id)?{self.phase=8;}},
   8=>{let node=node.unwrap();let[a,b,c,d,e,f]=node.transform;write!(out,"\" transform=\"matrix({a} {b} {c} {d} {e} {f})\" opacity=\"{}\" style=\"mix-blend-mode:{}\">",node.opacity,blend(&node.blend_mode))?;self.stop_count=0;self.part=0;self.constant=None;self.phase=9;},
   9=>{let fill=match &node.unwrap().content{DocumentSceneContent::Path{fill,..}|DocumentSceneContent::Glyphs{fill,..}=>fill.as_ref(),_=>None};if let Some(fill)=fill{if let FillStyle::Solid{color}=fill{self.constant=Some(*color);self.phase=11;}else{let(stops,degenerate)=match fill{FillStyle::LinearGradient{x1,y1,x2,y2,stops}=>(stops,x1==x2&&y1==y2),FillStyle::RadialGradient{r,stops,..}=>(stops,*r==0.0),_=>unreachable!()};if stops.len()>64{self.phase=32;return Ok(());}if self.part<stops.len(){let index=self.part;self.part+=1;let mut at=self.stop_count;while at>0&&stops.get(self.stops[at-1]).unwrap().offset>stops.get(index).unwrap().offset{self.stops[at]=self.stops[at-1];at-=1;}self.stops[at]=index;self.stop_count+=1;}else if self.stop_count<=1||degenerate{self.constant=Some(if self.stop_count==0{[0.0;4]}else{stops.get(self.stops[if degenerate{self.stop_count-1}else{0}]).unwrap().color});self.phase=11;}else{let kind=match fill{FillStyle::LinearGradient{..}=>"linearGradient",_=>"radialGradient"};write!(out,"<defs><{kind} id=\"draw-gradient-{}\" gradientUnits=\"userSpaceOnUse\" ",self.node)?;match fill{FillStyle::LinearGradient{x1,y1,x2,y2,..}=>write!(out,"x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\">"),FillStyle::RadialGradient{cx,cy,r,..}=>write!(out,"cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\">"),_=>unreachable!()}?;self.part=0;self.phase=10;}}}else{self.phase=11;}},
   10=>{let fill=match &node.unwrap().content{DocumentSceneContent::Path{fill,..}|DocumentSceneContent::Glyphs{fill,..}=>fill.as_ref().unwrap(),_=>unreachable!()};let(stops,kind)=match fill{FillStyle::LinearGradient{stops,..}=>(stops,"linearGradient"),FillStyle::RadialGradient{stops,..}=>(stops,"radialGradient"),_=>unreachable!()};if self.part<self.stop_count{let stop=stops.get(self.stops[self.part]).unwrap();self.part+=1;write!(out,"<stop offset=\"{}\" stop-color=\"",stop.offset.clamp(0.0,1.0))?;rgb(out,&stop.color)?;write!(out,"\" stop-opacity=\"{}\"/>",stop.color[3])?;}else{write!(out,"</{kind}></defs>")?;self.phase=11;}},
   11=>{self.part=0;match &node.unwrap().content{DocumentSceneContent::Path{..}|DocumentSceneContent::Glyphs{..}=>{out.write_str("<path d=\"")?;self.phase=12;},DocumentSceneContent::Image{..}=>{self.asset=0;self.phase=20;},_=>{self.phase=31;}}},
   12=>{if let DocumentSceneContent::Path{segments,..}|DocumentSceneContent::Glyphs{segments,..}=&node.unwrap().content{if let Some(segment)=segments.get(self.part){path(out,segment)?;self.part+=1;}else{out.write_char('"')?;self.phase=13;}}},
   13=>{let(fill_rule,fill,stroke)=match &node.unwrap().content{DocumentSceneContent::Path{fill_rule,fill,stroke,..}|DocumentSceneContent::Glyphs{fill_rule,fill,stroke,..}=>(fill_rule,fill,stroke),_=>unreachable!()};out.write_str(" fill=\"")?;if let Some(color)=self.constant{rgb(out,&color)?;}else if fill.is_some(){write!(out,"url(#draw-gradient-{})",self.node)?;}else{out.write_str("none")?;}write!(out,"\" fill-opacity=\"{}\" fill-rule=\"{}\" stroke=\"",self.constant.map_or(1.0,|c|c[3]),fill_rule.as_str())?;if let Some(stroke)=stroke{rgb(out,&stroke.color)?;write!(out,"\" stroke-opacity=\"{}\" stroke-width=\"{}\" stroke-linecap=\"{}\" stroke-linejoin=\"{}\"",stroke.color[3],stroke.width,stroke.cap.as_str(),stroke.join.as_str())?;self.phase=if stroke.dash.as_ref().is_some_and(|dash|!dash.is_empty()){14}else{16};}else{out.write_str("none\"")?;self.phase=16;}self.part=0;},
   14=>{out.write_str(" stroke-dasharray=\"")?;self.phase=15;},
   15=>{let stroke=match &node.unwrap().content{DocumentSceneContent::Path{stroke,..}|DocumentSceneContent::Glyphs{stroke,..}=>stroke.as_ref().unwrap(),_=>unreachable!()};let dash=stroke.dash.as_ref().unwrap();if let Some(value)=dash.get(self.part){if self.part>0{out.write_char(' ')?;}write!(out,"{value}")?;self.part+=1;}else{out.write_char('"')?;self.phase=16;}},
   16=>{out.write_str("/></g>")?;self.node+=1;self.phase=1;},
   23=>{if let DocumentSceneContent::Image{width,height,..}=&node.unwrap().content{write!(out,"<image x=\"0\" y=\"0\" width=\"{width}\" height=\"{height}\" preserveAspectRatio=\"none\" href=\"data:image/png;base64,")?;self.phase=24;}},
   24=>{if self.image_at<self.owners.image_bytes.len(){let end=(self.image_at+PAGE).min(self.owners.image_bytes.len());const ABC:&[u8]=b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";for bytes in self.owners.image_bytes[self.image_at..end].chunks(3){let n=(u32::from(bytes[0])<<16)|(u32::from(*bytes.get(1).unwrap_or(&0))<<8)|u32::from(*bytes.get(2).unwrap_or(&0));for c in [ABC[(n>>18)as usize],ABC[(n>>12&63)as usize],if bytes.len()>1{ABC[(n>>6&63)as usize]}else{b'='},if bytes.len()>2{ABC[(n&63)as usize]}else{b'='}]{out.write_char(c as char)?;}}self.image_at=end;}else{self.phase=25;}},
   27=>{out.write_str("\"/></g>")?;self.node+=1;self.phase=1;},
   29=>{if self.depth>0{self.depth-=1;out.write_str("</g>")?;}else{out.write_str("</svg>")?;self.phase=30;}},
   _=>{},
  }Ok(())})();formatting.map_err(|_|"SVG scalar fragment exceeds capacity")?;
  match self.phase{
   20=>{let DocumentSceneContent::Image{asset,..}=&self.owners.plan.nodes[self.node].content else{unreachable!()};let Some(source)=self.owners.plan.assets.get(self.asset)else{return Err("Missing authoritative SVG image".into());};if &source.id!=asset{self.asset+=1;return Ok(());}match PngEncodeJob::with_maximum_bytes(RasterLease(source.image.clone()),self.limits.max_output_bytes){Ok(job)=>self.owners.encoder=Some(job),Err((error,image))=>{self.owners.rejected_image=Some(image);return Err(error.to_string());}}self.phase=21;},
   21=>{let job=self.owners.encoder.as_mut().unwrap();if job.advance().map_err(|error|error.to_string())?.done{self.owners.image_bytes=job.take_result().map_err(|error|error.to_string())?.data;self.image_at=0;self.phase=22;}},
   22=>{if reserve(&mut self.owners.retired_encoders)?{self.owners.retired_encoders.push_reserved(self.owners.encoder.take().unwrap()).unwrap_or_else(|_|unreachable!());self.phase=23;}},
   25=>{if reserve(&mut self.owners.retired_bytes)?{self.owners.retired_bytes.push_reserved(std::mem::take(&mut self.owners.image_bytes)).unwrap_or_else(|_|unreachable!());self.phase=27;}},
   31=>return Err("SVG requires resolved authored geometry".into()),32=>return Err("SVG retained source capacity exceeded".into()),
   _=>{},
  }Ok(())
 }
 pub fn advance(&mut self,grant:usize)->Result<SvgWriteProgress,String>{if grant==0||grant as u128>9007199254740991{return Err("Invalid SVG work grant".into());}if self.cancelled{return Err("SVG writer cancelled".into());}if let Some(error)=&self.owners.failure{return Err(error.clone());}for _ in 0..grant{if self.done(){break;}if self.work>=self.limits.max_work{self.owners.failure=Some("SVG work limit exceeded".into());break;}if let Err(error)=self.step(){self.owners.failure=Some(error);break;}self.work+=1;}if let Some(error)=&self.owners.failure{return Err(error.clone());}Ok(SvgWriteProgress{phase:if self.done(){"complete"}else{"writing"},nodes:self.node,bytes:self.bytes,work:self.work,done:self.done()})}
 pub fn result(&self)->Result<&Pages,&str>{if self.cancelled||self.owners.failure.is_some()||!self.done(){Err("SVG output incomplete or cancelled")}else{Ok(&self.owners.pages)}}
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn into_retirement(mut self)->(SvgWriteRetirement,Option<Pages>){let pages=(!self.cancelled&&self.owners.failure.is_none()&&self.done()).then(||std::mem::take(&mut self.owners.pages));self.cancelled=true;(SvgWriteRetirement::new(self),pages)}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
