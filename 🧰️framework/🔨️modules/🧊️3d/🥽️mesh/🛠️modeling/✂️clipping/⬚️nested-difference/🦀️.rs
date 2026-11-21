//! ⬚️ Publishes strictly nested Difference shells with opposite winding.

use super::*;

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct NestedDifference {
    output:MeshIntersectionSolid,outer_faces:usize,
    face:usize,corner:usize,phase:u8,volume:[f64;2],
}

impl NestedDifference {
    pub(super) fn new(mut geometry:Soup,faces:Vec<MeshIntersectionFace>,points:Vec<SourceVertex>)->Self{
        let outer_faces=geometry.faces.len();
        geometry.normals.clear();
        let output=MeshIntersectionSolid{geometry,faces,points,right_points:Vec::new(),right_interpolations:Vec::new(),retained_operand:None};
        Self{output,outer_faces,face:0,corner:0,phase:0,volume:[0.0;2]}
    }

    pub(super) fn advance(&mut self,right:&Soup,epsilon:f32)->MeshResult<bool>{
        let out=&mut self.output;
        match self.phase{
            0=>{
                let index=out.right_points.len();
                if let Some(&point)=right.positions.get(index){
                    let id=out.geometry.positions.len() as u32;
                    out.geometry.positions.push(point);out.right_points.push(id);
                }else{self.phase=1;}
            },
            1=>{
                if let Some(ring)=right.faces.get(self.face){
                    if self.corner==0{
                        out.geometry.faces.push(Vec::new());
                        out.faces.push(MeshIntersectionFace::ReversedRightPlane(self.face as u32));
                    }
                    let id=ring[ring.len()-1-self.corner];
                    out.geometry.faces.last_mut().unwrap().push(out.right_points[id as usize]);
                    self.corner+=1;
                    if self.corner==ring.len(){self.corner=0;self.face+=1;}
                }else{self.phase=2;self.face=0;}
            },
            2=>{
                if let Some(ring)=out.geometry.faces.get(self.face){
                    let shell=usize::from(self.face>=self.outer_faces);
                    let point=|id:u32|out.geometry.positions[id as usize];
                    let anchor=point(out.geometry.faces[shell*self.outer_faces][0]);
                    let a=point(ring[0]);
                    let b=point(ring[self.corner]);
                    let c=point(ring[(self.corner+1)%ring.len()]);
                    self.volume[shell]+=dot(difference(a,anchor),cross(difference(b,anchor),difference(c,anchor)))/6.0;
                    self.corner+=1;
                    if self.corner==ring.len(){self.corner=0;self.face+=1;}
                }else{
                    let tolerance=f64::from(epsilon).powi(3);
                    let [outer,inner]=self.volume;
                    if outer<=tolerance||inner>= -tolerance||outer+inner<=tolerance{
                        return Err(invalid("nested difference requires opposite closed shell volumes and positive net volume"));
                    }
                    return Ok(true);
                }
            },
            _=>unreachable!(),
        }
        Ok(false)
    }

    pub(super) fn take_output(&mut self)->MeshIntersectionSolid{
        MeshIntersectionSolid{geometry:std::mem::take(&mut self.output.geometry),faces:std::mem::take(&mut self.output.faces),
            points:std::mem::take(&mut self.output.points),right_points:std::mem::take(&mut self.output.right_points),right_interpolations:Vec::new(),retained_operand:None}
    }
}
