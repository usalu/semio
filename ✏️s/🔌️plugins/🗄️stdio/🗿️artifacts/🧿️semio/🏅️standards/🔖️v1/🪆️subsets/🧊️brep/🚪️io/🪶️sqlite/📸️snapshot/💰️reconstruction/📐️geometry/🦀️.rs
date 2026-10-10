//! 📐️ Borrows analytic declarations into the original topology geometry before NURBS allocation.
use super::{BrepCurve,BrepCurve2,BrepSurface,RowIndex,SqliteSnapshotControl,ValueError,invalid,shape,point2,point3,sequence_into};
pub(super)struct Curves3<'a>{pub rows:RowIndex<'a>,pub points:RowIndex<'a>,pub weights:RowIndex<'a>,pub knots:RowIndex<'a>,pub orders:[&'a[usize];3]}
impl Curves3<'_>{
pub fn fill(&mut self,target:&mut BrepCurve,id:i64,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let row=self.rows.take(id,c)?.ok_or_else(||invalid("missing multiply owned or dangling edge curve"))?;*target=match row.text(1)?{
"line"=>{shape(row,&[2,3,4,5,6,7])?;BrepCurve::Line{origin:point3(row,2)?,direction:point3(row,5)?}},
"circle"=>{shape(row,&[8,9,10,11,12,13,14])?;BrepCurve::Circle{center:point3(row,8)?,axis:point3(row,11)?,radius:row.real(14)?}},
"ellipse"=>{shape(row,&[8,9,10,11,12,13,15,16])?;BrepCurve::Ellipse{center:point3(row,8)?,axis:point3(row,11)?,radius_major:row.real(15)?,radius_minor:row.real(16)?}},
"nurbs"=>{shape(row,&[17])?;BrepCurve::Nurbs{control_points:Vec::new(),weights:Vec::new(),degree:u32::try_from(row.integer(17)?).map_err(|_|invalid("invalid BRep curve degree"))?,knots:Vec::new()}},
_=>return Err(invalid("unknown BRep 3D curve kind"))
};if let BrepCurve::Nurbs{control_points,weights,knots,..}=target{sequence_into(control_points,&mut self.points,self.orders[0],id,c,|row|point3(row,3))?;sequence_into(weights,&mut self.weights,self.orders[1],id,c,|row|row.real(3))?;sequence_into(knots,&mut self.knots,self.orders[2],id,c,|row|row.real(3))?;}Ok(())}
pub fn remaining(&self)->bool{[self.rows.remaining(),self.points.remaining(),self.weights.remaining(),self.knots.remaining()].iter().any(|v|*v!=0)}
}
pub(super)struct Curves2<'a>{pub rows:RowIndex<'a>,pub points:RowIndex<'a>,pub weights:RowIndex<'a>,pub knots:RowIndex<'a>,pub orders:[&'a[usize];3]}
impl Curves2<'_>{
pub fn fill(&mut self,target:&mut BrepCurve2,id:i64,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let row=self.rows.take(id,c)?.ok_or_else(||invalid("missing multiply owned or dangling p-curve"))?;*target=match row.text(1)?{
"line"=>{shape(row,&[2,3,4,5])?;BrepCurve2::Line{origin:point2(row,2)?,direction:point2(row,4)?}},
"circle"=>{shape(row,&[6,7,10])?;BrepCurve2::Circle{center:point2(row,6)?,radius:row.real(10)?}},
"ellipse"=>{shape(row,&[6,7,8,9,11,12])?;BrepCurve2::Ellipse{center:point2(row,6)?,x_axis:point2(row,8)?,radius_major:row.real(11)?,radius_minor:row.real(12)?}},
"nurbs"=>{shape(row,&[13])?;BrepCurve2::Nurbs{control_points:Vec::new(),weights:Vec::new(),degree:u32::try_from(row.integer(13)?).map_err(|_|invalid("invalid BRep p-curve degree"))?,knots:Vec::new()}},
_=>return Err(invalid("unknown BRep p-curve kind"))
};if let BrepCurve2::Nurbs{control_points,weights,knots,..}=target{sequence_into(control_points,&mut self.points,self.orders[0],id,c,|row|point2(row,3))?;sequence_into(weights,&mut self.weights,self.orders[1],id,c,|row|row.real(3))?;sequence_into(knots,&mut self.knots,self.orders[2],id,c,|row|row.real(3))?;}Ok(())}
pub fn remaining(&self)->bool{[self.rows.remaining(),self.points.remaining(),self.weights.remaining(),self.knots.remaining()].iter().any(|v|*v!=0)}
}
pub(super)struct Surfaces<'a>{pub rows:RowIndex<'a>,pub points:RowIndex<'a>,pub weights:RowIndex<'a>,pub knots_u:RowIndex<'a>,pub knots_v:RowIndex<'a>,pub orders:[&'a[usize];4]}
impl Surfaces<'_>{
pub fn fill(&mut self,target:&mut BrepSurface,id:i64,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let row=self.rows.take(id,c)?.ok_or_else(||invalid("missing multiply owned or dangling face surface"))?;*target=match row.text(1)?{
"plane"=>{shape(row,&[2,3,4,11,12,13])?;BrepSurface::Plane{origin:point3(row,2)?,normal:point3(row,11)?}},
"cylinder"=>{shape(row,&[2,3,4,5,6,7,14])?;BrepSurface::Cylinder{origin:point3(row,2)?,axis:point3(row,5)?,radius:row.real(14)?}},
"cone"=>{shape(row,&[2,3,4,5,6,7,14,15])?;BrepSurface::Cone{origin:point3(row,2)?,axis:point3(row,5)?,radius:row.real(14)?,half_angle:row.real(15)?}},
"sphere"=>{shape(row,&[8,9,10,14])?;BrepSurface::Sphere{center:point3(row,8)?,radius:row.real(14)?}},
"torus"=>{shape(row,&[5,6,7,8,9,10,16,17])?;BrepSurface::Torus{center:point3(row,8)?,axis:point3(row,5)?,major_radius:row.real(16)?,minor_radius:row.real(17)?}},
"nurbs"=>{shape(row,&[18,19,20,21])?;BrepSurface::Nurbs{control_points:Vec::new(),weights:Vec::new(),u_count:u32::try_from(row.integer(18)?).map_err(|_|invalid("invalid BRep U count"))?,v_count:u32::try_from(row.integer(19)?).map_err(|_|invalid("invalid BRep V count"))?,degree_u:u32::try_from(row.integer(20)?).map_err(|_|invalid("invalid BRep U degree"))?,degree_v:u32::try_from(row.integer(21)?).map_err(|_|invalid("invalid BRep V degree"))?,knots_u:Vec::new(),knots_v:Vec::new()}},
_=>return Err(invalid("unknown BRep surface kind"))
};if let BrepSurface::Nurbs{control_points,weights,knots_u,knots_v,..}=target{sequence_into(control_points,&mut self.points,self.orders[0],id,c,|row|point3(row,3))?;sequence_into(weights,&mut self.weights,self.orders[1],id,c,|row|row.real(3))?;sequence_into(knots_u,&mut self.knots_u,self.orders[2],id,c,|row|row.real(3))?;sequence_into(knots_v,&mut self.knots_v,self.orders[3],id,c,|row|row.real(3))?;}Ok(())}
pub fn remaining(&self)->bool{[self.rows.remaining(),self.points.remaining(),self.weights.remaining(),self.knots_u.remaining(),self.knots_v.remaining()].iter().any(|v|*v!=0)}
}
                                           