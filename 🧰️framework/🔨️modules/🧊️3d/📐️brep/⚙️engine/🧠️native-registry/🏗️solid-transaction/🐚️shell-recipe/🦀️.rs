//! 🐚️ Certified cylindrical, spherical and orthogonal shell wall recipes.

use super::*;
use super::boolean_recipe::Bounds;
use semio_framework_3d_brep_boolean::BooleanOp;
use semio_framework_3d_brep_vector::{Vec3,matrix::Frame3};

fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}

/// 🧱️ The immutable certified geometry needed by one original Shell operation.
#[derive(Clone,Copy)]
pub enum ShellStock {
    Box { bounds:Bounds, frame:Option<Frame3>, thickness:f64 },
    Sphere(SphericalShell),
    Cylinder { frame:Frame3, radius:f64, height:f64, thickness:f64 },
}

impl ShellStock {
    pub fn new(primitive:SolidPrimitive,thickness:f64)->Result<Self,ValueError>{
        let tolerance=Tol::DEFAULT.value();
        if !thickness.is_finite()||thickness<=tolerance{return Err(invalid("shell thickness must be positive at kernel tolerance"));}
        match primitive{
            SolidPrimitive::Box{origin,width,depth,height}=>Ok(Self::Box{bounds:Bounds::box_primitive(origin,width,depth,height),frame:None,thickness}),
            SolidPrimitive::Sphere{frame,radius}=>SphericalShell::new(frame,radius,thickness,tolerance).map(Self::Sphere)
                .ok_or_else(||invalid("shell thickness collapsed the spherical cavity or wall")),
            SolidPrimitive::Cylinder{frame,radius,height}=>Ok(Self::Cylinder{frame,radius,height,thickness}),
            _=>Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Shell requires a certified box, cylinder or sphere")),
        }
    }

    /// 🧭️ Captures one local rectangular solid and its authored world frame.
    pub fn oriented_box(stock:OrientedBox,thickness:f64)->Result<Self,ValueError>{
        if !thickness.is_finite()||thickness<=Tol::DEFAULT.value(){return Err(invalid("shell thickness must be positive at kernel tolerance"));}
        let (origin,[x,y,z],hi)=stock.placement();
        Ok(Self::Box{bounds:Bounds{lo:[0.;3],hi},frame:Some(Frame3{origin,x,y,z}),thickness})
    }

    /// 🪟️ Maps a verified source face's outward plane normal to its opening bit.
    pub fn opening(self,normal:Vec3)->Result<usize,ValueError>{
        match self{
            Self::Box{frame,..}=>{
                let [x,y,z]=frame.map_or([Vec3::X,Vec3::Y,Vec3::Z],|frame|[frame.x,frame.y,frame.z]);
                [-x,x,-y,y,-z,z].into_iter().position(|axis|axis.dot(normal)>=1.-64.*f64::EPSILON)
                    .ok_or_else(||invalid("shell opening does not match a box face"))
            }
            Self::Cylinder{frame,..}=>{
                let cap=frame.z.dot(normal);
                (cap.abs()>=1.-64.*f64::EPSILON).then_some(usize::from(cap>0.))
                    .ok_or_else(||invalid("cylindrical Shell requires a cap opening"))
            }
            Self::Sphere(_)=>Err(invalid("closed spherical Shell requires no open faces")),
        }
    }

    pub fn finish(self,open_sides:u8)->Result<SolidBoundary,ValueError>{
        let (stock,frame,thickness)=match self{
            Self::Cylinder{frame,radius,height,thickness}=>{
                let floor=if open_sides&1==0{thickness}else{0.};
                let top=if open_sides&2==0{height-thickness}else{height};
                return AnnularRevolution::from_axial_cut(frame,radius-thickness,radius,height,[floor,top],Tol::DEFAULT.value())
                    .map(SolidBoundary::Annular).ok_or_else(||invalid("shell thickness collapsed the cylindrical cavity or wall"));
            }
            Self::Sphere(shell)=>{
                if open_sides!=0{return Err(invalid("closed spherical Shell requires no open faces"));}
                return Ok(SolidBoundary::SphericalShell(shell));
            }
            Self::Box{bounds,frame,thickness}=>(bounds,frame,thickness),
        };
        if open_sides==0b11_1111{return Err(invalid("removing all box faces leaves no shell"));}
        let inset=|side|if open_sides&(1u8<<side)==0{thickness}else{0.0};
        let cavity=Bounds{
            lo:std::array::from_fn(|axis|stock.lo[axis]+inset(2*axis)),
            hi:std::array::from_fn(|axis|stock.hi[axis]-inset(2*axis+1)),
        };
        if (0..3).any(|axis|cavity.hi[axis]-cavity.lo[axis]<=Tol::DEFAULT.value()
            ||(inset(2*axis)!=0.0&&cavity.lo[axis]<=stock.lo[axis])
            ||(inset(2*axis+1)!=0.0&&cavity.hi[axis]>=stock.hi[axis])){
            return Err(invalid("shell thickness collapsed an opening, cavity or wall"));
        }
        let boundary=OrthogonalBoundary::new([stock.lo,stock.hi],[cavity.lo,cavity.hi],BooleanOp::Cut)?;
        let placed=match frame{Some(frame)=>boundary.placed(frame).ok_or_else(||invalid("shell walls are outside the representable stock frame"))?,None=>boundary};
        Ok(SolidBoundary::Orthogonal(placed))
    }
}

pub(super) fn exterior(shape:SphericalShell)->SolidPrimitive{shape.boundary(false).into()}

/// 🕳️ Measures the nearer of a certified nested exterior and its inward cavity wall.
pub fn spherical_shell_distance(shape:SphericalShell,point:Pnt3)->f64{
    let inner=SolidPrimitive::from(shape.boundary(true));
    exterior(shape).signed_boundary_distance(point).max(-inner.signed_boundary_distance(point))
}

pub(super) fn rings(shape:SphericalShell)->&'static [usize]{
    if matches!(exterior(shape),SolidPrimitive::Box{..}){&[4;7]}else{&[4;2]}
}

pub(super) fn vertex(shape:SphericalShell,i:usize)->Pnt3{
    let outer=exterior(shape);
    let n=outer.vertex_count();
    if i<n{outer.vertex(i)}else{SolidPrimitive::from(shape.boundary(true)).vertex(i-n)}
}

pub(super) fn edge(shape:SphericalShell,i:usize)->PrimitiveEdge{
    let outer=exterior(shape);
    let n=outer.edge_count();
    if i<n{return outer.edge(i);}
    let mut edge=SolidPrimitive::from(shape.boundary(true)).edge(i-n);
    edge.vertices=edge.vertices.map(|v|v+outer.vertex_count());
    edge
}

pub(super) fn surface(shape:SphericalShell,i:usize)->Surface{
    let outer=exterior(shape);
    if i<outer.ring_sizes().len(){outer.surface(i)}else{SolidPrimitive::from(shape.boundary(true)).surface(0)}
}

pub(super) fn trim(shape:SphericalShell,i:usize,member:usize)->PrimitiveTrim{
    let outer=exterior(shape);
    if i<outer.ring_sizes().len(){return outer.trim(i,member);}
    let mut trim=SolidPrimitive::from(shape.boundary(true)).trim(0,member);
    trim.edge+=outer.edge_count();
    trim
}
