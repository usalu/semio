//! 🎨️ Prepared local-space paint with stable stops and logarithmic lookup.
use crate::{FillStyle,GradientStop};
use semio_framework_2d::retirement::{WorkRetirementCounter,WorkRetirementProgress};

pub const MAX_PAINT_STOPS:usize=4096;
fn unit(value:f64)->bool {value.is_finite()&&(0.0..=1.0).contains(&value)}
fn valid_color(color:&[f64;4])->bool {color.iter().all(|value|unit(*value))}

pub struct GradientRamp {stops:Vec<GradientStop>}
impl GradientRamp {
    pub fn new(stops:&[GradientStop])->Result<Self,&'static str> {
        if stops.len()>MAX_PAINT_STOPS||stops.iter().any(|stop|!unit(stop.offset)||!valid_color(&stop.color)) {return Err("Invalid gradient stops");}
        let mut stops=stops.to_vec();
        stops.sort_by(|a,b|a.offset.partial_cmp(&b.offset).expect("finite offsets"));
        Ok(Self {stops})
    }
    pub fn constant_color(&self)->Option<[f64;4]> {
        (self.stops.len()<=1).then(||self.stops.first().map_or([0.0;4],|stop|stop.color))
    }
    pub fn sample(&self,offset:f64)->Result<[f64;4],&'static str> {
        if offset.is_nan() {return Err("Invalid gradient position");}
        let Some(first)=self.stops.first() else {return Ok([0.0;4]);};
        let right=self.stops.partition_point(|stop|stop.offset<=offset);
        if right==0 {return Ok(first.color);}
        let left=&self.stops[right-1];
        let Some(right)=self.stops.get(right) else {return Ok(left.color);};
        let t=(offset-left.offset)/(right.offset-left.offset);
        Ok(std::array::from_fn(|index|left.color[index]+(right.color[index]-left.color[index])*t))
    }
    /// 🧹️ Consumes the actual fixed-width stop buffer before retiring the ramp header.
    pub fn into_retirement(self)->GradientRampRetirement {GradientRampRetirement {ramp:Some(self),slot:0,counter:WorkRetirementCounter::default()}}
}

pub struct GradientRampRetirement {ramp:Option<GradientRamp>,slot:u8,counter:WorkRetirementCounter}
impl GradientRampRetirement {
    pub fn terminal_is_empty(&self)->bool {self.ramp.is_none()}
    pub fn advance(&mut self,grant:usize)->Result<WorkRetirementProgress,&'static str> {
        let ramp=&mut self.ramp;let slot=&mut self.slot;
        self.counter.advance(grant,||{if *slot==0{ramp.as_mut().unwrap().stops=Vec::new();*slot=1;false}else{*ramp=None;true}})
    }
}

enum Geometry {Solid([f64;4]),Linear {origin:[f64;2],unit:[f64;2],length:f64},Radial {center:[f64;2],radius:f64}}
pub struct PreparedFill {geometry:Geometry,ramp:Option<GradientRamp>}
impl PreparedFill {
    pub fn new(fill:&FillStyle)->Result<Self,&'static str> {
        let (geometry,ramp)=match fill {
            FillStyle::Solid {color}=>{
                if !valid_color(color) {return Err("Invalid fill color");}
                (Geometry::Solid(*color),None)
            }
            FillStyle::LinearGradient {x1,y1,x2,y2,stops}=>{
                let dx=x2-x1;let dy=y2-y1;let length=dx.hypot(dy);
                if ![x1,y1,x2,y2,&length].iter().all(|value|value.is_finite()) {return Err("Invalid linear gradient geometry");}
                (Geometry::Linear {origin:[*x1,*y1],unit:if length==0.0 {[0.0;2]}else{[dx/length,dy/length]},length},Some(GradientRamp::new(stops)?))
            }
            FillStyle::RadialGradient {cx,cy,r,stops}=>{
                if ![cx,cy,r].iter().all(|value|value.is_finite())||*r<0.0 {return Err("Invalid radial gradient geometry");}
                (Geometry::Radial {center:[*cx,*cy],radius:*r},Some(GradientRamp::new(stops)?))
            }
        };
        Ok(Self {geometry,ramp})
    }
    pub fn constant_color(&self)->Option<[f64;4]> {
        match self.geometry {
            Geometry::Solid(color)=>Some(color),
            Geometry::Linear {length:0.0,..}|Geometry::Radial {radius:0.0,..}=>self.ramp.as_ref().expect("gradient geometry owns ramp").sample(1.0).ok(),
            _=>self.ramp.as_ref().expect("gradient geometry owns ramp").constant_color(),
        }
    }
    pub fn sample(&self,point:[f64;2])->Result<[f64;4],&'static str> {
        if !point.iter().all(|value|value.is_finite()) {return Err("Fill sample position must be finite");}
        let offset=match self.geometry {
            Geometry::Solid(color)=>return Ok(color),
            Geometry::Linear {origin,unit,length}=>if length==0.0 {1.0}else{((point[0]-origin[0])*unit[0]+(point[1]-origin[1])*unit[1])/length},
            Geometry::Radial {center,radius}=>if radius==0.0 {1.0}else{(point[0]-center[0]).hypot(point[1]-center[1])/radius},
        };
        self.ramp.as_ref().expect("gradient geometry owns ramp").sample(offset)
    }
    /// 🖌️ Adopts its genuine ramp frontier and retains paint until that child is terminal.
    pub fn into_retirement(mut self)->PreparedFillRetirement {
        let ramp=self.ramp.take().map(GradientRamp::into_retirement);PreparedFillRetirement {fill:Some(self),ramp,slot:0,counter:WorkRetirementCounter::default()}
    }
}

pub struct PreparedFillRetirement {fill:Option<PreparedFill>,ramp:Option<GradientRampRetirement>,slot:u8,counter:WorkRetirementCounter}
impl PreparedFillRetirement {
    pub fn terminal_is_empty(&self)->bool {self.fill.is_none()&&self.ramp.is_none()}
    pub fn advance(&mut self,grant:usize)->Result<WorkRetirementProgress,&'static str> {
        let fill=&mut self.fill;let ramp=&mut self.ramp;let slot=&mut self.slot;
        self.counter.advance(grant,||{if *slot==0 {if let Some(child)=ramp {if !child.terminal_is_empty(){child.advance(1).expect("valid ramp unit grant");return false;}}*ramp=None;*slot=1;false}else{*fill=None;true}})
    }
}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
