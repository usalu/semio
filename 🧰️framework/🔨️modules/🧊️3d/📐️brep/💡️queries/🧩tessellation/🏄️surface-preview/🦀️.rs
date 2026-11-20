//! 🏄️ Budgeted preview of a standalone analytic or rational surface.

use super::*;
use semio_framework_mesh_engine::MeshData;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress as Progress,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;

use std::f64::consts::{FRAC_PI_2,TAU};

const MAXIMUM_TRIANGLES:usize=200_000;
const MAXIMUM_SPAN_PAIRS:usize=200_000;

#[derive(Clone,Copy)]
enum Turn{Counting,Reserve(usize),Visit,Sampling}
semio_framework_value::artifact_retire_leaf!(Turn);

#[derive(Clone,Copy)]
struct AnalyticPatch {ranges:[[f64;2];2],subdivisions:[usize;2]}
semio_framework_value::artifact_retire_leaf!(AnalyticPatch);

impl AnalyticPatch {
    fn for_surface(surface:&Surface,tolerance:f64)->Result<Option<Self>,ValueError>{
        let (frame,meridian,hessian,minimum)=match surface{
            Surface::Cylinder{frame,radius} if *radius>0.0=>(frame,[-*radius,*radius],[*radius,0.0,0.0],[8,1]),
            Surface::Cone{frame,half_angle} if *half_angle>0.0&&*half_angle<FRAC_PI_2=>{
                let slope=half_angle.tan();
                (frame,[0.0,1.0],[slope,slope,0.0],[8,1])
            }
            Surface::Sphere{frame,radius} if *radius>0.0=>(frame,[-FRAC_PI_2,FRAC_PI_2],[*radius;3],[8,4]),
            Surface::Torus{frame,major_radius,minor_radius} if *minor_radius>0.0&&major_radius>minor_radius=>
                (frame,[0.0,TAU],[major_radius+minor_radius,*minor_radius,*minor_radius],[8,8]),
            Surface::Nurbs{..}=>return Ok(None),
            _=>return Err(invalid()),
        };
        let scale=frame.x.norm().hypot(frame.y.norm()).hypot(frame.z.norm());
        let length=meridian[1]-meridian[0];
        let mixed=hessian[1]*TAU*length;
        let bound=[hessian[0]*TAU*TAU+mixed,hessian[2]*length*length+mixed];
        let subdivisions=std::array::from_fn(|axis|{
            let n=(2.0*scale*bound[axis]/tolerance).sqrt().ceil();
            (n as usize).max(minimum[axis])
        });
        if !scale.is_finite()||scale<=0.0||bound.iter().any(|value|!value.is_finite()){return Err(invalid());}
        Ok(Some(Self{ranges:[[0.0,TAU],meridian],subdivisions}))
    }

    fn sample(surface:&Surface,u:f64,v:f64)->Option<(Pnt3,Option<Vec3>)>{
        let u=if u==TAU{0.0}else{u};
        if let Surface::Sphere{frame,radius}=surface{
            if v.abs()==FRAC_PI_2{
                let radial=frame.z*v.signum();
                let orientation=frame.x.cross(frame.y).dot(frame.z).signum();
                return Some((frame.origin + radial * *radius,(radial*orientation).normalized()));
            }
        }
        let v=if matches!(surface,Surface::Torus{..})&&v==TAU{0.0}else{v};
        Some((surface.point_original(u,v)?,surface.normal_original(u,v)))
    }
}

pub(super) struct SurfacePatchPreview {
    label:u64,
    first:[usize;2],
    end:[usize;2],
    span:[usize;2],
    ranges:[[f64;2];2],
    stage:Turn,
    patches:usize,
    subdivisions:[usize;2],
    analytic:Option<AnalyticPatch>,
    triangle:usize,
    extent:usize,
    tolerance:f64,
    mesh:MeshData,
}
semio_framework_value::artifact_retire_struct!(SurfacePatchPreview{label,first,end,span,ranges,stage,patches,subdivisions,analytic,triangle,extent,tolerance,mesh});

fn invalid()->ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue,"surface preview requires regular finite analytic or positive-weight rational geometry")
}

fn limit()->ValueError {
    ValueError::literal(ValueRefusalKind::WorkLimit,"surface preview exceeds the triangle or knot range limit")
}

fn reserve<T>(buffer:&mut Vec<T>,count:usize)->Result<usize,ValueError>{
    buffer.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"surface preview backing allocation failed"))?;
    Ok(buffer.capacity()*size_of::<T>())
}

fn fraction([start,end]:[f64;2],index:usize,count:usize)->f64{
    if index==0{start}else if index==count{end}
    else{(start+(end-start)*index as f64/count as f64).clamp(start,end)}
}

impl SurfacePatchPreview {
    pub(super) fn new(surface:&Surface,label:u64,tolerance:f64)->Result<Self,ValueError>{
        if !tolerance.is_finite()||tolerance<=0.0{return Err(invalid());}
        let analytic=AnalyticPatch::for_surface(surface,tolerance)?;
        let (first,end)=if analytic.is_some(){([0,0],[1,1])}else{surface.original_spline_spans().ok_or_else(invalid)?};
        (end[0]-first[0]).checked_mul(end[1]-first[1]).filter(|pairs|*pairs<=MAXIMUM_SPAN_PAIRS).ok_or_else(limit)?;
        Ok(Self{label,first,end,span:first,ranges:[[0.0;2];2],stage:Turn::Counting,patches:0,subdivisions:[1,1],analytic,
            triangle:0,extent:0,tolerance,mesh:Default::default()})
    }

    pub(super) fn is_meshing(&self)->bool{!matches!(self.stage,Turn::Counting)}

    pub(super) fn demand(&self)->RetirementDemand{
        let (copy_bytes,capacity_bytes)=match self.stage{
            Turn::Reserve(0|1)=>(0,self.extent*9*size_of::<f32>()),
            Turn::Reserve(_)=>(0,self.extent*3*size_of::<u32>()),
            Turn::Sampling=>(18*size_of::<f32>()+3*size_of::<u32>(),0),
            _=>(0,0),
        };
        RetirementDemand{copy_bytes,capacity_bytes,depth:1,..Default::default()}
    }

    fn next_span(&mut self){
        self.span[1]+=1;
        if self.span[1]==self.end[1] {self.span[1]=self.first[1];self.span[0]+=1;}
    }

    fn count(&mut self,surface:&Surface)->Result<(),ValueError>{
        if self.span[0]==self.end[0]{
            if self.patches==0{return Err(invalid());}
            self.extent=self.patches*2*self.subdivisions[0]*self.subdivisions[1];
            self.span=self.first;
            self.stage=Turn::Reserve(0);
            return Ok(());
        }
        let (ranges,subdivisions)=if let Some(analytic)=self.analytic {(analytic.ranges,analytic.subdivisions)}
        else{
            let (ranges,bound)=surface.original_spline_patch_bound(self.span).ok_or_else(invalid)?;
            let n=(2.0*bound/self.tolerance).sqrt().ceil().max(1.0) as usize;
            (ranges,[n,n])
        };
        if ranges.into_iter().all(|[a,b]|a<b){
            self.patches+=1;
            let [u,v]=std::array::from_fn(|axis|self.subdivisions[axis].max(subdivisions[axis]));
            self.subdivisions=[u,v];
            self.patches.checked_mul(2).and_then(|p|p.checked_mul(u)).and_then(|p|p.checked_mul(v))
                .filter(|count|*count<=MAXIMUM_TRIANGLES).ok_or_else(limit)?;
        }
        self.next_span();
        Ok(())
    }

    fn sample_triangle(&mut self,surface:&Surface)->Result<usize,ValueError>{
        let [nu,nv]=self.subdivisions;
        let cell=self.triangle/2;
        let u=cell/nv;
        let v=cell%nv;
        let uv=if self.triangle%2==0{[(u,v),(u+1,v),(u+1,v+1)]}else{[(u,v),(u+1,v+1),(u,v+1)]};
        let mut points=[[0.0_f32;3];3];
        let mut lifted=[Pnt3::new(0.0,0.0,0.0);3];
        let mut normals=[None;3];
        for (index,(u,v)) in uv.into_iter().enumerate(){
            let u=fraction(self.ranges[0],u,nu);
            let v=fraction(self.ranges[1],v,nv);
            let sample=if self.analytic.is_some(){AnalyticPatch::sample(surface,u,v)}
                else{surface.original_spline_patch_sample(self.span,u,v)};
            let (point,normal)=sample.ok_or_else(invalid)?;
            let rounded=[point.x as f32,point.y as f32,point.z as f32];
            let [x,y,z]=rounded.map(f64::from);
            let lifted_point=Pnt3::new(x,y,z);
            if rounded.into_iter().any(|v|!v.is_finite())||point.distance(lifted_point)>self.tolerance*0.5{return Err(invalid());}
            points[index]=rounded;
            lifted[index]=lifted_point;
            normals[index]=normal;
        }
        self.triangle+=1;
        if self.triangle==2*nu*nv{
            self.next_span();
            self.stage=Turn::Visit;
        }
        let Some(normal)=(lifted[1]-lifted[0]).cross(lifted[2]-lifted[0]).normalized() else{return Ok(0);};
        let base=(self.mesh.positions.len()/3)as u32;
        for (point,normal) in points.into_iter().zip(normals.map(|value|value.unwrap_or(normal))){
            self.mesh.positions.extend(point);
            self.mesh.normals.extend([normal.x as f32,normal.y as f32,normal.z as f32]);
        }
        self.mesh.indices.extend([base,base+1,base+2]);
        Ok(18*size_of::<f32>()+3*size_of::<u32>())
    }

    /// 🧊️ Advances one bound estimate, allocation, or evaluated triangle.
    pub(super) fn advance(&mut self,surface:&Surface,label:u64,grant:RetainedCloneGrant)->Result<(Option<MeshData>,Progress),ValueError>{
        if self.label!=label{return Err(ValueError::invariant("original surface preview source changed"));}
        let demand=self.demand();
        if !retained_clone_grant_admits(grant,demand){return Ok((None,Default::default()));}
        let mut receipt=Progress{copied_items:1,..Default::default()};
        match self.stage {
            Turn::Counting=>self.count(surface)?,
            Turn::Reserve(slot)=>{
                receipt.retained_capacity_bytes=match slot{
                    0=>reserve(&mut self.mesh.positions,self.extent*9)?,
                    1=>reserve(&mut self.mesh.normals,self.extent*9)?,
                    _=>reserve(&mut self.mesh.indices,self.extent*3)?,
                };
                self.stage=if slot==2{Turn::Visit}else{Turn::Reserve(slot+1)};
            }
            Turn::Visit=>{
                if self.span[0]==self.end[0]{
                    if self.mesh.indices.is_empty(){return Err(invalid());}
                    return Ok((Some(std::mem::take(&mut self.mesh)),receipt));
                }
                self.ranges=if let Some(analytic)=self.analytic{analytic.ranges}else{surface.original_spline_patch_range(self.span).ok_or_else(invalid)?};
                if self.ranges.into_iter().any(|[a,b]|a==b){self.next_span();}
                else{self.triangle=0;self.stage=Turn::Sampling;}
            }
            Turn::Sampling=>receipt.copied_bytes=self.sample_triangle(surface)?,
        }
        if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"surface preview exceeded its retained grant").with_retained_progress(receipt));}
        Ok((None,receipt))
    }
}
