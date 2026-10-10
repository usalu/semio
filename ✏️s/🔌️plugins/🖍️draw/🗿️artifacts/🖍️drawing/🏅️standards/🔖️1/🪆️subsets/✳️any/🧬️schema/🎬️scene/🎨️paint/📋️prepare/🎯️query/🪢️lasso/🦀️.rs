//! 🪢️ Painted boundaries and internal lasso exclusions advance over borrowed geometry.
use super::PreparedPaint;
struct SegmentPartitionCursor{from:[f64;2],to:[f64;2],cuts:Vec<f64>,at:usize}
impl SegmentPartitionCursor{
 fn new(from:[f64;2],to:[f64;2])->Self{Self{from,to,cuts:vec![0.0,1.0],at:0}}
 fn advance(&mut self,polygon:&[[f64;2]])->Option<Vec<f64>>{
  if self.at==polygon.len(){self.cuts.sort_by(f64::total_cmp);self.cuts.dedup();return Some(std::mem::take(&mut self.cuts));}
  let a=polygon[self.at];self.at+=1;let b=polygon[self.at%polygon.len()];let dx=self.to[0]-self.from[0];let dy=self.to[1]-self.from[1];let ex=b[0]-a[0];let ey=b[1]-a[1];let ox=a[0]-self.from[0];let oy=a[1]-self.from[1];let denominator=dx*ey-dy*ex;let length=dx.hypot(dy);
  if length==0.0{return None;}
  if denominator.abs()>f64::EPSILON*16.0*length*ex.hypot(ey){let t=(ox*ey-oy*ex)/denominator;let u=(ox*dy-oy*dx)/denominator;if t>0.0&&t<1.0&&(0.0..=1.0).contains(&u){self.cuts.push(t);}}
  else if(ox*dy-oy*dx).abs()<=f64::EPSILON*8.0*a.into_iter().chain(self.from).fold(1.0_f64,|n,v|n.max(v.abs()))*length{for p in [a,b]{let t=if dx.abs()>=dy.abs(){(p[0]-self.from[0])/dx}else{(p[1]-self.from[1])/dy};if t>0.0&&t<1.0{self.cuts.push(t);}}}None
 }
}
pub(super) struct SegmentEnclosureCursor{from:[f64;2],to:[f64;2],phase:u8,cuts:Vec<f64>,partition:Option<SegmentPartitionCursor>,at:usize,sample:usize,inside:bool,boundary:bool,enclosed:bool}
impl SegmentEnclosureCursor{
 pub(super) fn new(from:[f64;2],to:[f64;2])->Self{Self{from,to,phase:0,cuts:vec![0.0,1.0],partition:Some(SegmentPartitionCursor::new(from,to)),at:0,sample:0,inside:false,boundary:false,enclosed:true}}
 pub(super) fn advance(&mut self,polygon:&[[f64;2]])->Option<bool>{
  if self.phase==2{return Some(self.enclosed);}
  if self.phase==0{if let Some(cuts)=self.partition.as_mut().unwrap().advance(polygon){self.cuts=cuts;self.partition=None;self.phase=1;}return None;}
  if self.at==polygon.len(){if !self.inside&&!self.boundary{self.enclosed=false;self.phase=2;return Some(false);}self.sample+=1;if self.sample==self.cuts.len()*2-1{self.phase=2;return Some(true);}self.at=0;self.inside=false;self.boundary=false;return None;}
  let index=self.sample/2;let t=if self.sample%2==1{(self.cuts[index]+self.cuts[index+1])/2.0}else{self.cuts[index]};let p=[self.from[0]*(1.0-t)+self.to[0]*t,self.from[1]*(1.0-t)+self.to[1]*t];let a=polygon[self.at];let b=polygon[(self.at+1)%polygon.len()];let dx=b[0]-a[0];let dy=b[1]-a[1];let length=dx.hypot(dy);let along=if length>0.0{(((p[0]-a[0])*dx+(p[1]-a[1])*dy)/length).clamp(0.0,length)}else{0.0};let rounding=a.into_iter().chain(b).chain(p).fold(1.0_f64,|n,v|n.max(v.abs()))*f64::EPSILON*8.0;
  self.boundary|=(p[0]-a[0]-if length>0.0{dx*along/length}else{0.0}).hypot(p[1]-a[1]-if length>0.0{dy*along/length}else{0.0})<=rounding;
  if(a[1]>p[1])!=(b[1]>p[1])&&p[0]<(b[0]-a[0])*(p[1]-a[1])/(b[1]-a[1])+a[0]{self.inside=!self.inside;}self.at+=1;None
 }
}
fn sign(value:f64)->i64{if value>0.0{1}else if value<0.0{-1}else{0}}
fn side_winding(p:[f64;2],normal:[f64;2],side:f64,a:[f64;2],b:[f64;2])->i64{
 let above=|y:f64|if y==p[1]{sign(-side*normal[1])}else{sign(y-p[1])};let ay=above(a[1]);let by=above(b[1]);let dx=b[0]-a[0];let dy=b[1]-a[1];let px=p[0]-a[0];let py=p[1]-a[1];let cross=dx*py-dy*px;let error=((dx*py).abs()+(dy*px).abs())*f64::EPSILON*8.0;let orientation=if cross.abs()<=error{sign(side*(dx*normal[1]-dy*normal[0]))}else{sign(cross)};
 if ay<=0&&by>0&&orientation>0{1}else if by<=0&&ay>0&&orientation<0{-1}else{0}
}
/// 🕳️ Infinitesimal lasso sides reject internal exclusions without inventing epsilon-sized holes.
pub(super) struct LassoInteriorCursor{bounds:[f64;4],phase:u8,outer:usize,partition:Option<SegmentPartitionCursor>,cuts:Vec<f64>,interval:usize,point:[f64;2],normal:[f64;2],at:usize,slot:usize,contour:usize,edge:usize,winding:[i64;2],outside:[bool;2],painted:[bool;2],enclosed:bool}
impl LassoInteriorCursor{
 pub(super) fn new(bounds:[f64;4])->Self{Self{bounds,phase:0,outer:0,partition:None,cuts:Vec::new(),interval:0,point:[0.0;2],normal:[0.0;2],at:0,slot:0,contour:0,edge:0,winding:[0;2],outside:[false;2],painted:[false;2],enclosed:true}}
 fn sample(&mut self,polygon:&[[f64;2]]){let a=polygon[self.outer];let b=polygon[(self.outer+1)%polygon.len()];let t=(self.cuts[self.interval]+self.cuts[self.interval+1])/2.0;self.point=[a[0]*(1.0-t)+b[0]*t,a[1]*(1.0-t)+b[1]*t];self.normal=[a[1]-b[1],b[0]-a[0]];self.at=0;self.winding=[0;2];self.phase=1;}
 fn next(&mut self,polygon:&[[f64;2]]){self.interval+=1;if self.interval+1==self.cuts.len(){self.outer+=1;self.cuts=Vec::new();self.phase=0;}else{self.sample(polygon);}}
 pub(super) fn advance(&mut self,polygon:&[[f64;2]],paint:&PreparedPaint)->Option<bool>{
  if self.phase==3{return Some(self.enclosed);}
  if self.phase==0{if self.outer==polygon.len(){self.phase=3;return Some(true);}let a=polygon[self.outer];let b=polygon[(self.outer+1)%polygon.len()];let[x,y,r,d]=self.bounds;if a==b||a[0].max(b[0])<x||a[0].min(b[0])>r||a[1].max(b[1])<y||a[1].min(b[1])>d{self.outer+=1;return None;}if self.partition.is_none(){self.partition=Some(SegmentPartitionCursor::new(a,b));}if let Some(cuts)=self.partition.as_mut().unwrap().advance(polygon){self.cuts=cuts;self.partition=None;self.interval=0;self.sample(polygon);}return None;}
  if self.phase==1{if self.at<polygon.len(){let a=polygon[self.at];self.at+=1;let b=polygon[self.at%polygon.len()];for side in 0..2{self.winding[side]+=side_winding(self.point,self.normal,if side==0{-1.0}else{1.0},a,b);}return None;}self.outside=self.winding.map(|n|n%2==0);if !self.outside.into_iter().any(|outside|outside){self.next(polygon);return None;}self.slot=0;self.contour=0;self.edge=0;self.winding=[0;2];self.painted=[false;2];self.phase=2;return None;}
  if self.slot==if matches!(paint,PreparedPaint::Path(_)){2}else{1}{if(0..2).any(|side|self.outside[side]&&self.painted[side]){self.enclosed=false;self.phase=3;return Some(false);}self.next(polygon);return None;}
  let contour=match paint{PreparedPaint::Path(regions)=>(if self.slot==0{&regions.fill}else{&regions.stroke}).get(self.contour).map(Vec::as_slice),PreparedPaint::Image(quad)=>if self.contour==0{Some(quad.as_slice())}else{None},PreparedPaint::Empty=>None};
  let Some(contour)=contour else{for side in 0..2{let filled=if self.slot==0&&matches!(paint,PreparedPaint::Path(regions)if regions.fill_rule==crate::FillRule::Evenodd){self.winding[side]%2!=0}else{self.winding[side]!=0};self.painted[side]|=filled;}self.slot+=1;self.contour=0;self.edge=0;self.winding=[0;2];return None;};
  if self.edge==contour.len(){self.contour+=1;self.edge=0;return None;}
  let a=contour[self.edge];self.edge+=1;let b=contour[self.edge%contour.len()];for side in 0..2{self.winding[side]+=side_winding(self.point,self.normal,if side==0{-1.0}else{1.0},a,b);}None
 }
}
