//! 🔶️ Fixed planar faces and corner caps for an exact all-edge box chamfer.

use super::*;
use semio_framework_3d_brep_construction::primitives::planar_pcurve;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_vector::{Vec3, matrix::Frame3};

use semio_framework_3d_brep_representation::contract::box_finish_layout::{EDGES, VERTICES, TOPOLOGY, face};

/// 🧊️ One rectangular stock with twelve planar bevels and eight triangular corner closures.
pub struct ChamferedBox {
    origin:Pnt3,
    basis:[Vec3;3],
    axes:[[f64;4];3],
    reversed:bool,
}

impl ChamferedBox {
    pub fn new(stock: SolidPrimitive, distance: f64) -> Option<Self> {
        let SolidPrimitive::Box { origin, width, depth, height } = stock else { return None; };
        Self::placed(origin,[Vec3::X,Vec3::Y,Vec3::Z],[width,depth,height],distance)
    }

    /// 🧭️ Certifies that the bevel fits the local spans and remains representable in the stock frame.
    pub(super) fn placed(origin:Pnt3,basis:[Vec3;3],size:[f64;3],distance:f64)->Option<Self>{
        let tolerance=Tol::DEFAULT.value();
        if !distance.is_finite()||distance<=tolerance{return None;}
        let axes=size.map(|span|[0.,distance,span-distance,span]);
        if !axes.iter().all(|axis|axis[3].is_finite()&&axis.windows(2).all(|pair|pair[1]-pair[0]>tolerance)
            && ((axis[3]-axis[2])-distance).abs()<=tolerance/4.){return None;}
        let [x,y,z]=basis;
        let shape=Self{origin,basis,axes,reversed:x.cross(y).dot(z)<0.};
        let valid=(0..VERTICES).all(|id|{
            let offset=shape.rotate(shape.local_vertex(id));
            (((origin+offset)-origin)-offset).norm()<=tolerance/4.
        })&&TOPOLOGY.edges.iter().all(|&[a,b]|{
            let length=(shape.vertex(b)-shape.vertex(a)).norm();
            length.is_finite()&&length>tolerance
        });
        valid.then_some(shape)
    }

    fn rotate(&self,[x,y,z]:[f64;3])->Vec3{
        self.basis[0]*x+self.basis[1]*y+self.basis[2]*z
    }

    pub fn vertex_count(&self) -> usize { VERTICES }
    pub fn edge_count(&self) -> usize { EDGES }
    pub fn ring_sizes(&self) -> &'static [usize] { &TOPOLOGY.ring_sizes }

    fn local_vertex(&self,index:usize)->[f64;3]{
        let (corner,normal)=(index/3,index%3);
        std::array::from_fn(|axis|{
            let high=(corner>>axis)&1;
            self.axes[axis][if normal==axis{3*high}else{1+high}]
        })
    }

    pub fn vertex(&self,index:usize)->Pnt3{
        self.origin+self.rotate(self.local_vertex(index))
    }

    pub fn edge(&self, index: usize) -> PrimitiveEdge {
        let vertices = TOPOLOGY.edges[index];
        let [origin, end] = vertices.map(|id| self.vertex(id));
        PrimitiveEdge { vertices, curve: Curve3::Line { origin, dir: end - origin }, range: (0.0, 1.0) }
    }

    pub fn surface(&self, index: usize) -> Surface {
        let ring = face(index);
        let frame = Frame3::from_normal(self.vertex(ring.vertices[0]), self.rotate(ring.normal)).expect("fixed chamfer face normal");
        Surface::Plane { frame }
    }

    pub fn trim(&self, index: usize, member: usize) -> PrimitiveTrim {
        let ring=face(index);
        let slot=if self.reversed{ring.count-1-member}else{member};
        let edge=TOPOLOGY.members[index][slot];
        let spec=self.edge(edge);
        let Surface::Plane { frame } = self.surface(index) else { unreachable!(); };
        let forward=(ring.vertices[slot]==spec.vertices[0])!=self.reversed;
        PrimitiveTrim { edge, forward, curve: planar_pcurve(&spec.curve, frame), range: spec.range }
    }
}
