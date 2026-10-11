//! 🔷️ Scalar authored shape coordinates support sparse semantic diffs.
use crate::DrawingShapeBody;
#[derive(Clone,Copy,Debug,PartialEq,Eq,semio_framework_value::RetainedClone,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
#[value(rename_all="camelCase")]
#[cfg_attr(test,serde(rename_all="camelCase"))]
pub enum ShapeCoordinateField {RectX,RectY,RectWidth,RectHeight,EllipseCx,EllipseCy,EllipseRx,EllipseRy,CircleCx,CircleCy,CircleR,LineX1,LineY1,LineX2,LineY2,PolygonX,PolygonY}
impl ShapeCoordinateField {
    pub fn as_str(self)->&'static str {match self {Self::RectX=>"rectX",Self::RectY=>"rectY",Self::RectWidth=>"rectWidth",Self::RectHeight=>"rectHeight",Self::EllipseCx=>"ellipseCx",Self::EllipseCy=>"ellipseCy",Self::EllipseRx=>"ellipseRx",Self::EllipseRy=>"ellipseRy",Self::CircleCx=>"circleCx",Self::CircleCy=>"circleCy",Self::CircleR=>"circleR",Self::LineX1=>"lineX1",Self::LineY1=>"lineY1",Self::LineX2=>"lineX2",Self::LineY2=>"lineY2",Self::PolygonX=>"polygonX",Self::PolygonY=>"polygonY"}}
    pub fn parse(field:&str)->Result<Self,&'static str> {match field {"rectX"=>Ok(Self::RectX),"rectY"=>Ok(Self::RectY),"rectWidth"=>Ok(Self::RectWidth),"rectHeight"=>Ok(Self::RectHeight),"ellipseCx"=>Ok(Self::EllipseCx),"ellipseCy"=>Ok(Self::EllipseCy),"ellipseRx"=>Ok(Self::EllipseRx),"ellipseRy"=>Ok(Self::EllipseRy),"circleCx"=>Ok(Self::CircleCx),"circleCy"=>Ok(Self::CircleCy),"circleR"=>Ok(Self::CircleR),"lineX1"=>Ok(Self::LineX1),"lineY1"=>Ok(Self::LineY1),"lineX2"=>Ok(Self::LineX2),"lineY2"=>Ok(Self::LineY2),"polygonX"=>Ok(Self::PolygonX),"polygonY"=>Ok(Self::PolygonY),_=>Err("Unknown shape coordinate")}}
    pub fn shape_kind(self)->&'static str {match self {Self::RectX|Self::RectY|Self::RectWidth|Self::RectHeight=>"rect",Self::EllipseCx|Self::EllipseCy|Self::EllipseRx|Self::EllipseRy=>"ellipse",Self::CircleCx|Self::CircleCy|Self::CircleR=>"circle",Self::LineX1|Self::LineY1|Self::LineX2|Self::LineY2=>"line",Self::PolygonX|Self::PolygonY=>"polygon"}}
    pub fn nonnegative(self)->bool {matches!(self,Self::RectWidth|Self::RectHeight|Self::EllipseRx|Self::EllipseRy|Self::CircleR)}
    fn validate(self,shape:&DrawingShapeBody,index:Option<usize>)->Result<(),&'static str> {
        if shape.shape_kind!=self.shape_kind() {return Err("The coordinate does not belong to this shape");}
        if matches!(self,Self::PolygonX|Self::PolygonY)!=index.is_some() {return Err("Only polygon coordinates require a vertex index");}
        Ok(())
    }
}
/// 📏️ Borrows one authored scalar without materializing the layer or polygon.
pub fn shape_coordinate(shape:&DrawingShapeBody,field:&ShapeCoordinateField,index:Option<usize>)->Result<f64,&'static str> {
    use ShapeCoordinateField::*;let field=*field;field.validate(shape,index)?;
    Ok(match field {
        RectX=>shape.rect.as_ref().ok_or("Missing rectangle geometry")?.x,
        RectY=>shape.rect.as_ref().ok_or("Missing rectangle geometry")?.y,
        RectWidth=>shape.rect.as_ref().ok_or("Missing rectangle geometry")?.width,
        RectHeight=>shape.rect.as_ref().ok_or("Missing rectangle geometry")?.height,
        EllipseCx=>shape.ellipse.as_ref().ok_or("Missing ellipse geometry")?.cx,
        EllipseCy=>shape.ellipse.as_ref().ok_or("Missing ellipse geometry")?.cy,
        EllipseRx=>shape.ellipse.as_ref().ok_or("Missing ellipse geometry")?.rx,
        EllipseRy=>shape.ellipse.as_ref().ok_or("Missing ellipse geometry")?.ry,
        CircleCx=>shape.circle.as_ref().ok_or("Missing circle geometry")?.cx,
        CircleCy=>shape.circle.as_ref().ok_or("Missing circle geometry")?.cy,
        CircleR=>shape.circle.as_ref().ok_or("Missing circle geometry")?.r,
        LineX1=>shape.line.as_ref().ok_or("Missing line geometry")?.x1,
        LineY1=>shape.line.as_ref().ok_or("Missing line geometry")?.y1,
        LineX2=>shape.line.as_ref().ok_or("Missing line geometry")?.x2,
        LineY2=>shape.line.as_ref().ok_or("Missing line geometry")?.y2,
        PolygonX|PolygonY=>shape.polygon.as_ref().ok_or("Missing polygon geometry")?.points.get(index.unwrap()).ok_or("Missing polygon vertex")?[usize::from(field==PolygonY)],
    })
}
/// 🩹️ Changes exactly one admitted coordinate; rejected edits leave every field intact.
pub fn set_shape_coordinate(shape:&mut DrawingShapeBody,field:ShapeCoordinateField,index:Option<usize>,value:f64)->Result<(),&'static str> {
    use ShapeCoordinateField::*;field.validate(shape,index)?;
    if !value.is_finite()||(field.nonnegative()&&value<0.0) {return Err("Enter a finite coordinate and nonnegative dimensions");}
    let target=match field {
        RectX=>&mut shape.rect.as_mut().ok_or("Missing rectangle geometry")?.x,
        RectY=>&mut shape.rect.as_mut().ok_or("Missing rectangle geometry")?.y,
        RectWidth=>&mut shape.rect.as_mut().ok_or("Missing rectangle geometry")?.width,
        RectHeight=>&mut shape.rect.as_mut().ok_or("Missing rectangle geometry")?.height,
        EllipseCx=>&mut shape.ellipse.as_mut().ok_or("Missing ellipse geometry")?.cx,
        EllipseCy=>&mut shape.ellipse.as_mut().ok_or("Missing ellipse geometry")?.cy,
        EllipseRx=>&mut shape.ellipse.as_mut().ok_or("Missing ellipse geometry")?.rx,
        EllipseRy=>&mut shape.ellipse.as_mut().ok_or("Missing ellipse geometry")?.ry,
        CircleCx=>&mut shape.circle.as_mut().ok_or("Missing circle geometry")?.cx,
        CircleCy=>&mut shape.circle.as_mut().ok_or("Missing circle geometry")?.cy,
        CircleR=>&mut shape.circle.as_mut().ok_or("Missing circle geometry")?.r,
        LineX1=>&mut shape.line.as_mut().ok_or("Missing line geometry")?.x1,
        LineY1=>&mut shape.line.as_mut().ok_or("Missing line geometry")?.y1,
        LineX2=>&mut shape.line.as_mut().ok_or("Missing line geometry")?.x2,
        LineY2=>&mut shape.line.as_mut().ok_or("Missing line geometry")?.y2,
        PolygonX|PolygonY=>&mut shape.polygon.as_mut().ok_or("Missing polygon geometry")?.points.get_mut(index.unwrap()).ok_or("Missing polygon vertex")?[usize::from(field==PolygonY)],
    };*target=value;Ok(())
}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
