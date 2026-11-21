//! 📏️ Certifies an exposed simple boundary sweep for open mesh reconstruction.

use super::*;

#[derive(semio_framework_value::RetireOwned)]
pub struct MeshBoundarySweep{origin:[f64;3],normal:[f64;3],offset:[f64;3],pub epsilon:f64,axis:usize}

fn refuse(message:&str)->MeshKernelError {MeshKernelError::InvalidInput(message.into())}

impl MeshBoundarySweep {
    /// 📐️ Selects a supporting plane perpendicular to a usable displacement component.
    pub fn new(origin:[f64;3],end:[f64;3],mut normal:[f64;3],offset:[f64;3])->MeshResult<Self>{
        let edge:[f64;3]=sub3(end.into(),origin.into()).into();
        let scale=length3(edge.into()).max(length3(offset.into()));
        let epsilon=scale*1e-6;
        if length3(edge.into())<=epsilon{return Err(refuse("boundary extrusion contains a collapsed edge"));}
        let mut drift=dot3(offset.into(),normal.into());
        if drift.abs()<=epsilon {
            let along=dot3(edge.into(),offset.into())/dot3(edge.into(),edge.into());
            normal=std::array::from_fn(|axis|offset[axis]-edge[axis]*along);
            let length=length3(normal.into());
            if length<=epsilon{return Err(refuse("boundary offset is parallel to the selected edge"));}
            normal=normal.map(|component|component/length);
            drift=dot3(offset.into(),normal.into());
        }
        if drift.abs()<=epsilon{return Err(refuse("boundary offset does not leave the supporting plane"));}
        normal=if drift<0.0{normal.map(|n|-n)}else{normal};
        let axis=(0..3).max_by(|&a,&b|normal[a].abs().total_cmp(&normal[b].abs())).unwrap();
        Ok(Self{origin,normal,offset,epsilon,axis})
    }

    /// 🧭️ Measures signed height over the oriented supporting plane.
    pub fn height(&self,point:[f64;3])->f64{dot3(sub3(point.into(),self.origin.into()),self.normal.into())}

    /// 📍️ Transfers one on-plane point to a well-separated native point.
    pub fn translate(&self,point:[f64;3])->MeshResult<[f32;3]>{
        if self.height(point).abs()>self.epsilon{return Err(refuse("selected boundary does not lie on one supporting plane"));}
        let shifted=std::array::from_fn(|axis|(point[axis]+self.offset[axis]) as f32);
        let actual=shifted.map(f64::from);
        if !shifted.iter().all(|v|v.is_finite())||self.height(actual)<=self.epsilon
            ||(0..3).any(|axis|(actual[axis]-point[axis]-self.offset[axis]).abs()>self.epsilon*0.25){
            return Err(refuse("boundary offset cannot be represented reliably"));
        }
        Ok(shifted)
    }

    /// 📐️ Refuses crossings, nonadjacent contacts and a shared join that folds back.
    pub fn certify_pair(&self,points:[[f64;3];4],consecutive:bool)->MeshResult<()>{
        let [a,b,c,d]=points.map(|point|[point[(self.axis+1)%3],point[(self.axis+2)%3]]);
        let orientation=|a:[f64;2],b:[f64;2],c:[f64;2]|
            (b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0]);
        let length=|u:[f64;2],v:[f64;2]|(u[0]-v[0]).hypot(u[1]-v[1]);
        let area=self.epsilon*length(a,b).max(length(c,d));
        if consecutive{
            let retraces=(a[0]-b[0])*(d[0]-b[0])+(a[1]-b[1])*(d[1]-b[1])>0.0;
            if orientation(a,b,d).abs()<=area&&retraces{return Err(refuse("selected boundary folds back at a join"));}
        }else{
            let overlaps=(0..2).all(|i|a[i].min(b[i])<=c[i].max(d[i])+self.epsilon&&c[i].min(d[i])<=a[i].max(b[i])+self.epsilon);
            let straddles=|x:f64,y:f64|x.min(y)<=area&&x.max(y)>=-area;
            if overlaps&&straddles(orientation(a,b,c),orientation(a,b,d))&&straddles(orientation(c,d,a),orientation(c,d,b)){
                return Err(refuse("selected boundary crosses or touches itself"));
            }
        }
        Ok(())
    }

}
