//! 🧮️ Local Hessian bounds for positive-weight rational surface patches.

use super::*;
use semio_framework_3d_brep_curve::bspline::ORIGINAL_SPLINE_MAXIMUM_DEGREE;

#[derive(Clone,Copy)]
struct Homogeneous {vector:Vec3,weight:f64}

impl Homogeneous {
    fn derivative(self,next:Self,knots:&KnotVector,control:usize,order:usize)->Option<Self>{
        let divisor=knots.knots[control+knots.degree+1]-knots.knots[control+order];
        if !divisor.is_finite()||divisor<0.0{return None;}
        let scale=if divisor==0.0{0.0}else{(knots.degree+1-order)as f64/divisor};
        let vector=(next.vector-self.vector)*scale;
        let weight=(next.weight-self.weight)*scale;
        [vector.x,vector.y,vector.z,weight].into_iter().all(f64::is_finite).then_some(Self{vector,weight})
    }
}

fn norm(vector:Vec3)->f64 {vector.x.hypot(vector.y).hypot(vector.z)}

impl Surface {
    /// 🔳️ Returns the inclusive starting and exclusive ending span indices on each spline axis.
    pub fn original_spline_spans(&self)->Option<([usize;2],[usize;2])>{
        let Self::Nurbs{u_knots,v_knots,controls,weights}=self else{return None;};
        let first=[u_knots.degree,v_knots.degree];
        let end=[controls.len(),controls.first()?.len()];
        if weights.len()!=end[0]{return None;}
        for (axis,knots) in [u_knots,v_knots].into_iter().enumerate(){
            let degree=first[axis];
            if degree==0||degree>ORIGINAL_SPLINE_MAXIMUM_DEGREE||degree>=end[axis]||
                knots.knots.len()!=end[axis].checked_add(degree)?.checked_add(1)? {return None;}
        }
        Some((first,end))
    }

    /// 🧱️ Returns the original untrimmed parameter rectangle, including any collapsed axis span.
    pub fn original_spline_patch_range(&self,spans:[usize;2])->Option<[[f64;2];2]>{
        let Self::Nurbs{u_knots,v_knots,..}=self else{return None;};
        let (first,end)=self.original_spline_spans()?;
        if (0..2).any(|a| !(first[a]..end[a]).contains(&spans[a])){return None;}
        let [u,v]=spans;
        let ranges=[[u_knots.knots[u],u_knots.knots[u+1]],[v_knots.knots[v],v_knots.knots[v+1]]];
        if ranges.into_iter().any(|[a,b]|!a.is_finite()||!b.is_finite()||a>b){return None;}
        Some(ranges)
    }

    /// 📏️ Bounds linear interpolation error over a parameter span of a positive-weight patch.
    pub fn original_spline_patch_bound(&self,spans:[usize;2])->Option<([[f64;2];2],f64)>{
        let Self::Nurbs{u_knots,v_knots,controls,weights}=self else{return None;};
        let (first,end)=self.original_spline_spans()?;
        let ranges=self.original_spline_patch_range(spans)?;
        let [u,v]=spans;
        let [du,dv]=ranges.map(|[a,b]|b-a);
        if [du,dv].into_iter().any(|width|!width.is_finite()||width<0.0){return None;}
        if du==0.0||dv==0.0{return Some((ranges,0.0));}
        let [p,q]=first;
        let [i0,j0]=[u-p,v-q];
        let anchor=*controls.get(i0)?.get(j0)?;
        let mut radius=0.0_f64;
        let mut w_min=f64::INFINITY;
        let mut w_max=0.0_f64;
        for i in i0..=u{
            let points=controls.get(i)?;
            let row=weights.get(i)?;
            if points.len()!=end[1]||row.len()!=end[1]{return None;}
            for j in j0..=v{
                let w=row[j];
                let length=norm(points[j]-anchor);
                if !w.is_finite()||w<=0.0||!length.is_finite(){return None;}
                radius=radius.max(length);
                w_min=w_min.min(w);
                w_max=w_max.max(w);
            }
        }
        let w_min=w_min/w_max;
        if w_min<=0.0{return None;}
        let point=|i:usize,j:usize|{
            let weight=weights[i][j]/w_max;
            Homogeneous{vector:(controls[i][j]-anchor)*weight,weight}
        };
        let along_u=|i,j|point(i,j).derivative(point(i+1,j),u_knots,i,1);
        let along_v=|i,j|point(i,j).derivative(point(i,j+1),v_knots,j,1);
        let mut h=[0.0_f64;5];
        let mut w=[0.0_f64;5];
        let mut accumulate=|axis:usize,value:Homogeneous|{
            h[axis]=h[axis].max(norm(value.vector));
            w[axis]=w[axis].max(value.weight.abs());
        };
        for i in i0..=u{
            for j in j0..=v{
                if i<u {accumulate(0,along_u(i,j)?);}
                if j<v {accumulate(1,along_v(i,j)?);}
                if i+1<u {accumulate(2,along_u(i,j)?.derivative(along_u(i+1,j)?,u_knots,i,2)?);}
                if i<u&&j<v {accumulate(3,along_u(i,j)?.derivative(along_u(i,j+1)?,v_knots,j,1)?);}
                if j+1<v {accumulate(4,along_v(i,j)?.derivative(along_v(i,j+1)?,v_knots,j,2)?);}
            }
        }
        let pu=(h[0]+radius*w[0])/w_min;
        let pv=(h[1]+radius*w[1])/w_min;
        let puu=(h[2]+radius*w[2]+2.0*pu*w[0])/w_min;
        let puv=(h[3]+radius*w[3]+pu*w[1]+pv*w[0])/w_min;
        let pvv=(h[4]+radius*w[4]+2.0*pv*w[1])/w_min;
        let bound=0.5*(puu*du*du+2.0*puv*du*dv+pvv*dv*dv);
        bound.is_finite().then_some((ranges,bound))
    }
}
