//! 📈️ Exact bounded quadratic arcs in elliptic-cone complements.

use super::*;

#[derive(Clone,Copy)]
enum Support { Parabola(f64), Hyperbola { major:f64, minor:f64, direction:f64 } }

impl Support {
    fn parameter(self,distance:f64,sine:f64)->f64{
        match self{
            Self::Parabola(bend)=>(distance/(sine*bend.abs())).sqrt(),
            Self::Hyperbola{major,..}=>(1.+distance/(sine*major)).acosh(),
        }
    }

    fn arc(self,first:f64,last:f64)->SectionArc{
        match self{
            Self::Parabola(bend)=>SectionArc{
                controls:[Pnt2::new(bend*first*first,first),Pnt2::new(bend*first*last,(first+last)*0.5),Pnt2::new(bend*last*last,last)],
                middle_weight:1.,
            },
            Self::Hyperbola{major,minor,direction}=>{
                let middle=(first+last)*0.5;
                let weight=((last-first)*0.5).cosh();
                let point=|theta:f64,divisor:f64|Pnt2::new(direction*major*(theta.cosh()/divisor-1.),minor*theta.sinh()/divisor);
                SectionArc{controls:[point(first,1.),point(middle,weight),point(last,1.)],middle_weight:weight}
            }
        }
    }
}

pub(super) fn section(frame:Frame3,radius:f64,height:f64,slope:f64,plane:Frame3,coefficient:f64)
    ->Result<(Frame3,Conic,ConicBoundary),ValueError>{
    let tol=Tol::DEFAULT.value();
    let apex_level=-radius/slope;
    let apex=frame.origin+frame.z*apex_level;
    let offset=(plane.origin-apex).dot(plane.z);
    if !(offset.abs()>tol&&offset.is_finite()){return Err(section_miss());}
    let cross=plane.z.cross(frame.z);
    let sine=cross.norm();
    let cosine=plane.z.dot(frame.z).clamp(-1.,1.);
    let y=cross.normalized().ok_or_else(invalid_section)?;
    let x=y.cross(plane.z);
    let factor=1.+slope*slope;
    let linear=factor*cosine*offset*sine;
    let direction=slope.signum();
    let (vertex,support)=if coefficient==0.{
        if linear.signum()!=direction||linear==0.{return Err(section_miss());}
        let constant=(factor*cosine*cosine-1.)*offset*offset;
        (-constant/(2.*linear),Support::Parabola((2.*linear).recip()))
    }else{
        let positive=-coefficient;
        let minor=(slope*offset).abs()/positive.sqrt();
        let major=minor/positive.sqrt();
        (-linear/positive+direction*major,Support::Hyperbola{major,minor,direction})
    };
    let level=apex_level+cosine*offset+sine*vertex;
    let [start,end]=if direction>0.{[(-level).max(0.),height-level]}else{[(level-height).max(0.),level]};
    if !(level.is_finite()&&end-start>tol&&end.is_finite()){return Err(section_miss());}
    let lo=support.parameter(start,sine)*direction;
    let hi=support.parameter(end,sine)*direction;
    if !(hi.is_finite()&&lo.is_finite()){return Err(invalid_section());}
    let first=support.arc(hi,if start==0.{-hi}else{lo});
    let second=if start==0.{first}else{support.arc(-lo,-hi)};
    let frame=Frame3{origin:apex+plane.z*offset+x*vertex,x,y,z:plane.z};
    for arc in [first,second]{
        if !(arc.middle_weight>0.&&arc.middle_weight.is_finite())||arc.controls.into_iter().any(|p|
            !(frame.origin+frame.x*p.x+frame.y*p.y).to_array().into_iter().all(f64::is_finite)){
            return Err(invalid_section());
        }
        if (arc.controls[0].y-arc.controls[2].y).abs()<=tol{return Err(section_miss());}
    }
    let span=(0.,1.);
    let boundary=ConicBoundary::clipped(span,(start!=0.).then_some(span));
    Ok((frame,Conic::Quadratic([first,second]),boundary))
}
