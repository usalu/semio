//! 📐️ Lossless placement changes between invertible affine parent frames.
use super::{inverse,multiply,CompositeAffine,PixelEditError};

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct AffineControls {pub x:f64,pub y:f64,pub scale_x:f64,pub scale_y:f64,pub rotation:f64,pub shear_x:f64}

/// 🎛️ Composes translation, degree rotation, horizontal shear and signed scale.
pub fn compose(value:AffineControls)->Result<CompositeAffine,PixelEditError> {
    if ![value.x,value.y,value.scale_x,value.scale_y,value.rotation,value.shear_x].iter().all(|v|v.is_finite()){return Err(PixelEditError::Invalid("Affine controls require finite coordinates"));}
    let (s,c)=value.rotation.to_radians().sin_cos();
    let result=[c*value.scale_x,s*value.scale_x,(c*value.shear_x-s)*value.scale_y,(s*value.shear_x+c)*value.scale_y,value.x,value.y];
    inverse(result)?;Ok(result)
}

/// 🪞️ Canonical controls retain shear and place reflection in the signed vertical scale.
pub fn decompose(matrix:CompositeAffine)->Result<AffineControls,PixelEditError> {
    inverse(matrix)?;
    let [a,b,c,d,x,y]=matrix;let scale_x=a.hypot(b);let scale_y=(a*d-b*c)/scale_x;
    let result=AffineControls {x,y,scale_x,scale_y,rotation:b.atan2(a).to_degrees(),shear_x:((a/scale_x)*c+(b/scale_x)*d)/scale_y};
    compose(result)?;Ok(result)
}

/// 🧭️ Preserves world placement when changing an object's parent coordinate frame.
pub fn reframe(transform:CompositeAffine,source:CompositeAffine,target:CompositeAffine)->Result<CompositeAffine,PixelEditError> {
    inverse(transform)?;inverse(source)?;
    let result=multiply(inverse(target)?,multiply(source,transform));
    inverse(result)?;Ok(result)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
