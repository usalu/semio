//! ↗️ Preserve SVG transform lists in the owned editable affine representation.
use crate::DrawingTransform;
use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::snapshot::{Matrix2D, TransformOp};
use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::{parse_transform_list};
pub fn parse_editable_svg_transform(source:&str)->Result<DrawingTransform,String> {
    validate_transform_grammar(source)?;
    editable_svg_transform_operations(&parse_transform_list(source)?)
}

/// ↗️ Consumes the native operation authority without printing and parsing transform text.
pub fn editable_svg_transform_operations(operations:&[TransformOp])->Result<DrawingTransform,String> {
    let mut matrix=Matrix2D::identity();
    for mut operation in operations.iter().cloned() {
        match &mut operation {
            TransformOp::Rotate {angle,..} => {*angle%=360.0;},
            TransformOp::SkewX {angle}|TransformOp::SkewY {angle} => {
                *angle%=180.0;
                if angle.abs()==90.0 {return Err("Undefined SVG skew angle".into());}
            },
            _=>{}
        }
        matrix=matrix.multiply(&operation.to_matrix());
        if ![matrix.a,matrix.b,matrix.c,matrix.d,matrix.e,matrix.f].iter().all(|value|value.is_finite()) {return Err("SVG transform exceeds finite coordinates".into());}
    }
    let result=crate::schema::drawing_matrix_to_transform([matrix.a,matrix.b,matrix.c,matrix.d,matrix.e,matrix.f]);
    if ![result.x,result.y,result.scale_x,result.scale_y,result.shear,result.rotation].iter().all(|value|value.is_finite()) {return Err("SVG transform cannot be represented".into());}
    Ok(result)
}
fn validate_transform_grammar(source:&str)->Result<(),String> {
    fn whitespace(bytes:&[u8],cursor:&mut usize) {while bytes.get(*cursor).is_some_and(|byte|matches!(byte,b' '|b'\t'|b'\r'|b'\n')) {*cursor+=1;}}
    fn digits(bytes:&[u8],cursor:&mut usize)->usize {let start=*cursor;while bytes.get(*cursor).is_some_and(u8::is_ascii_digit) {*cursor+=1;}*cursor-start}
    let (bytes,mut cursor)=(source.as_bytes(),0);
    let invalid=||"Invalid SVG transform grammar".to_owned();
    whitespace(bytes,&mut cursor);
    while cursor<bytes.len() {
        let start=cursor;
        while bytes.get(cursor).is_some_and(u8::is_ascii_alphabetic) {cursor+=1;}
        if cursor==start {return Err(invalid());}
        whitespace(bytes,&mut cursor);
        if bytes.get(cursor)!=Some(&b'(') {return Err(invalid());}cursor+=1;
        whitespace(bytes,&mut cursor);
        let mut count=0;
        loop {
            if matches!(bytes.get(cursor),Some(b'+'|b'-')) {cursor+=1;}
            let mut length=digits(bytes,&mut cursor);
            if bytes.get(cursor)==Some(&b'.') {cursor+=1;length+=digits(bytes,&mut cursor);}
            if length==0 {return Err(invalid());}
            if matches!(bytes.get(cursor),Some(b'e'|b'E')) {
                cursor+=1;if matches!(bytes.get(cursor),Some(b'+'|b'-')) {cursor+=1;}
                if digits(bytes,&mut cursor)==0 {return Err(invalid());}
            }
            count+=1;if count>6 {return Err(invalid());}
            let end=cursor;whitespace(bytes,&mut cursor);
            if bytes.get(cursor)==Some(&b')') {cursor+=1;break;}
            if bytes.get(cursor)==Some(&b',') {cursor+=1;whitespace(bytes,&mut cursor);}
            else if cursor==end {return Err(invalid());}
        }
        let end=cursor;whitespace(bytes,&mut cursor);
        if cursor<bytes.len() {
            if bytes.get(cursor)==Some(&b',') {
                while bytes.get(cursor)==Some(&b',') {cursor+=1;whitespace(bytes,&mut cursor);}
                if cursor==bytes.len() {return Err(invalid());}
            } else if cursor==end {return Err(invalid());}
        }
    }
    Ok(())
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
