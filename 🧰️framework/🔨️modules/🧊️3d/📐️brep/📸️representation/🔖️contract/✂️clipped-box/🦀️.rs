//! ✂️ Bounded closed clipping of a rectangular stock by one authored plane.

use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};

const NODES:usize=20;
const VERTICES:usize=10;
const EDGES:usize=15;
const FACES:usize=7;

#[derive(Clone,Copy)]
pub struct ClippedBox {
    origin:Pnt3,
    axes:[Vec3;3],
    size:[f64;3],
    normal:Vec3,
    offset:f64,
}

#[derive(Clone,Copy)]
struct Ring {
    anchor:Pnt3,
    normal:Vec3,
    vertices:[usize;6],
    edges:[usize;6],
}

pub struct ClippedBoxBoundary {
    points:[Pnt3;NODES],
    vertices:[usize;VERTICES],
    edges:[[usize;2];EDGES],
    uses:[[u8;2];EDGES],
    rings:[Ring;FACES],
    sizes:[usize;FACES],
    vertex_count:usize,
    edge_count:usize,
    face_count:usize,
}

impl ClippedBox {
    /// 🪓️ Builds both independent halves in positive and negative authored plane order.
    pub fn split(origin:Pnt3,axes:[Vec3;3],size:[f64;3],plane:Pnt3,normal:Vec3)->Option<[(ClippedBoxBoundary,Self);2]> {
        let tol=Tol::DEFAULT.value();
        if !size.into_iter().all(|x|x.is_finite()&&x>tol) {return None;}
        let size_max=size.into_iter().fold(0.,f64::max);
        let angular=(tol/(4.*size_max)).min(64.*f64::EPSILON);
        if axes.iter().any(|&axis|!((axis.norm()-1.).abs()<=angular))
            || [(0,1),(0,2),(1,2)].into_iter().any(|(i,j)|axes[i].dot(axes[j]).abs()>angular){return None;}
        let scale=normal.to_array().into_iter().map(f64::abs).fold(0.,f64::max);
        if !scale.is_finite()||scale==0. {return None;}
        let normal=Vec3::from_array(normal.to_array().map(|x|x/scale)).normalized()?;
        let offset=(plane-origin).dot(normal);
        if !offset.is_finite(){return None;}
        let positive=Self{origin,axes,size,normal,offset};
        let negative=Self{normal:-normal,offset:-offset,..positive};
        Some([(positive.boundary()?,positive),(negative.boundary()?,negative)])
    }

    pub(super) fn boundary(self)->Option<ClippedBoxBoundary>{
        let tol=Tol::DEFAULT.value();
        let local:[[f64;3];8]=std::array::from_fn(|corner|std::array::from_fn(|axis|if corner&(1<<axis)!=0 {self.size[axis]} else{0.}));
        let projected=self.axes.map(|axis|axis.dot(self.normal));
        let signed=local.map(|p|p.into_iter().zip(projected).map(|(x,n)|x*n).sum::<f64>()-self.offset);
        if !signed.into_iter().all(f64::is_finite) || !signed.iter().any(|&d|d>tol) || !signed.iter().any(|&d|d< -tol) {return None;}
        let sign=signed.map(|d|if d.abs()<=tol/4. {0} else if d>0. {1} else {-1});
        let mut points=[self.origin;NODES];
        let mut found=[false;12];
        let mut place=|id:usize,p:[f64;3]|{
            let local=self.axes[0]*p[0]+self.axes[1]*p[1]+self.axes[2]*p[2];
            let world=self.origin+local;
            if !world.to_array().into_iter().all(f64::is_finite) || !(((world-self.origin)-local).norm()<=tol/4.) {return None;}
            points[id]=world;
            Some(())
        };
        for (i,p) in local.into_iter().enumerate(){place(i,p)?;}
        for axis in 0..3{
            for a in (0..8).filter(|a| a&(1<<axis)==0){
                let b=a|(1<<axis);
                if sign[a]*sign[b] != -1 {continue;}
                let id=edge_node(a,b);
                let magnitude=signed[a].abs().max(signed[b].abs());
                let ratio=(signed[a].abs()/magnitude)/(signed[a].abs()/magnitude+signed[b].abs()/magnitude);
                let mut p=local[a];
                p[axis]=self.size[axis]*ratio;
                place(id,p)?;
                found[id-8]=true;
            }
        }

        let mut shape=ClippedBoxBoundary::new(points);
        let mut cap=[None;NODES];
        let on_plane=|id:usize|id>=8||sign[id]==0;
        let reversed=self.axes[0].cross(self.axes[1]).dot(self.axes[2])<0.;
        for face in 0..6 {
            let mut corners=super::box_finish_layout::face(face).vertices.map(|id|id/3);
            if reversed{corners.reverse();}
            let mut ring=[0;6];
            let mut count=0;
            for i in 0..4{
                let [a,b]=[corners[i],corners[(i+1)%4]];
                if sign[a]>=0 {ring[count]=a;count+=1;}
                if sign[a]*sign[b]==-1 {
                    let id=edge_node(a,b);
                    if !found[id-8]{return None;}
                    ring[count]=id;count+=1;
                }
            }
            let ring=&ring[..count];
            if ring.len()<3{continue;}
            for i in 0..ring.len(){
                let [a,b]=[ring[i],ring[(i+1)%ring.len()]];
                if on_plane(a)&&on_plane(b)&&cap[b].replace(a).is_some(){return None;}
            }
            let normal=if face%2==0{-self.axes[face/2]}else{self.axes[face/2]};
            shape.add_face(normal,ring,tol)?;
        }
        let mut ring=[0;6];
        let mut count=0;
        let start=cap.iter().position(Option::is_some)?;
        let mut node=start;
        loop{
            *ring.get_mut(count)?=node;
            if !(((points[node]-self.origin).dot(self.normal)-self.offset).abs()<=tol){return None;}
            count+=1;
            node=cap[node].take()?;
            if node==start{break;}
        }
        if count<3||cap.iter().any(Option::is_some){return None;}
        shape.add_face(-self.normal,&ring[..count],tol)?;
        let on_cap=points[start];
        let offset_error=(on_cap-self.origin).dot(self.normal)-self.offset;
        shape.rings[shape.face_count-1].anchor=on_cap-self.normal*offset_error;
        (shape.uses[..shape.edge_count].iter().all(|uses|uses==&[1,1])
            && shape.vertex_count+shape.face_count==shape.edge_count+2).then_some(shape)
    }

    pub fn bounds(self)->Option<[Pnt3;2]>{
        let shape=self.boundary()?;
        Some([false,true].map(|max|Pnt3::from_array(std::array::from_fn(|axis|{
            (0..shape.vertex_count).map(|id|shape.vertex(id).to_array()[axis]).reduce(|a,b|if max {a.max(b)}else{a.min(b)}).unwrap()
        }))))
    }

    /// 📍️ Measures Euclidean distance to the closed clipped box, negative in material.
    pub fn signed_boundary_distance(self,point:Pnt3)->Option<f64>{
        let shape=self.boundary()?;
        let side=|p:Pnt3,face:usize|{
            let (origin,normal)=shape.plane(face);
            (p-origin).dot(normal)
        };
        let farthest=(0..shape.face_count).map(|i|side(point,i)).fold(f64::NEG_INFINITY,f64::max);
        if farthest<=0. {return Some(farthest);}
        let mut best=f64::INFINITY;
        for face in 0..shape.face_count{
            let distance=side(point,face);
            if distance<=0.{continue;}
            let candidate=point-shape.rings[face].normal*distance;
            if (0..shape.face_count).all(|f|f==face||side(candidate,f)<=Tol::DEFAULT.value()/16.){
                best=best.min(distance);
            }
        }
        for &[i,j] in &shape.edges[..shape.edge_count]{
            let [a,b]=[i,j].map(|id|shape.vertex(id));
            let edge=b-a;
            let length=edge.norm();
            let direction=edge*(1./length);
            let local=point-a;
            let distance=(local-direction*local.dot(direction).clamp(0.,length)).norm();
            best=best.min(distance);
        }
        best.is_finite().then_some(best)
    }
}

fn edge_node(a:usize,b:usize)->usize{
    let axis=(a^b).trailing_zeros() as usize;
    let base=a.min(b);
    let lower=base&((1<<axis)-1);
    8+4*axis+lower+(base>>(axis+1)<<axis)
}

impl ClippedBoxBoundary {
    fn new(points:[Pnt3;NODES])->Self {
        Self{points,vertices:[0;VERTICES],edges:[[0;2];EDGES],uses:[[0;2];EDGES],rings:[Ring{anchor:points[0],normal:Vec3::ZERO,vertices:[0;6],edges:[0;6]};FACES],sizes:[0;FACES],vertex_count:0,edge_count:0,face_count:0}
    }

    fn add_face(&mut self,normal:Vec3,ring:&[usize],tol:f64)->Option<()>{
        let local=|i:usize|self.points[ring[i]]-self.points[ring[0]];
        let twice_area=(1..ring.len()-1).map(|i|local(i).cross(local(i+1)).dot(normal)).sum::<f64>();
        if !twice_area.is_finite()||twice_area<=tol*tol {return None;}
        let face=self.rings.get_mut(self.face_count)?;
        face.anchor=self.points[ring[0]];
        face.normal=normal;
        for (slot,&id) in face.vertices.iter_mut().zip(ring){
            *slot=match self.vertices[..self.vertex_count].iter().position(|&v|v==id){
                Some(index)=>index,
                None=>{
                    *self.vertices.get_mut(self.vertex_count)?=id;
                    self.vertex_count+=1;
                    self.vertex_count-1
                }
            };
        }
        for i in 0..ring.len(){
            let [a,b]=[face.vertices[i],face.vertices[(i+1)%ring.len()]];
            if (self.points[self.vertices[a]]-self.points[self.vertices[b]]).norm()<=tol{return None;}
            let key=[a.min(b),a.max(b)];
            let edge=match self.edges[..self.edge_count].iter().position(|&e|e==key){
                Some(i)=>i,
                None=>{
                    *self.edges.get_mut(self.edge_count)?=key;
                    self.edge_count+=1;
                    self.edge_count-1
                }
            };
            self.uses[edge][usize::from(a<b)]+=1;
            face.edges[i]=edge;
        }
        self.sizes[self.face_count]=ring.len();
        self.face_count+=1;
        Some(())
    }

    pub fn vertex_count(&self)->usize{self.vertex_count}
    pub fn edge_count(&self)->usize{self.edge_count}
    pub fn ring_sizes(&self)->&[usize]{&self.sizes[..self.face_count]}
    pub fn vertex(&self,id:usize)->Pnt3{self.points[self.vertices[id]]}

    pub fn plane(&self,face:usize)->(Pnt3,Vec3){
        let row=self.rings[face];
        (row.anchor,row.normal)
    }

    pub fn frame(&self,face:usize)->Frame3{
        let (point,normal)=self.plane(face);
        Frame3::from_normal(point,normal).expect("certified planar support")
    }

    pub fn edge(&self,id:usize)->([usize;2],Curve3){
        let vertices=self.edges[id];
        let [origin,end]=vertices.map(|id|self.vertex(id));
        (vertices,Curve3::Line{origin,dir:end-origin})
    }

    pub fn member(&self,face:usize,index:usize)->(usize,bool){
        let ring=self.rings[face];
        let edge=ring.edges[index];
        (edge,ring.vertices[index]==self.edges[edge][0])
    }
}

