//! 🖼️ Borrowed resource structures account for owned SQL entities.
use super::*;
fn optional(c:&mut Census<'_,'_>,v:&D,visit:fn(&mut Census<'_,'_>,&D)->Result<(),ValueError>)->Result<Cell<'static>,ValueError>{if matches!(v,D::Null){Ok(Null)}else{visit(c,v)?;Ok(Int)}}
fn array<'a>(v:&'a D,n:usize)->Result<Cells<'a>,ValueError>{if matches!(v,D::Null){Ok(Cells::from(&[Null;16][..n]))}else{let a=list(v)?;if a.len()!=n{return Err(invalid())}{let mut cells=Cells::new();for value in a{cells.push(real(value)?);}Ok(cells)}}}
pub(super) fn group(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{let color=optional(c,field(v,"colorSpace")?,colors::color)?;c.row("pdf_transparency_group",&[color,Int,Int])}
pub(super) fn image(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 let color=optional(c,field(v,"colorSpace")?,colors::color)?;let codec=field(v,"codec")?;let tag=kind(codec)?;let transform=if tag=="dct"{integer(field(codec,"colorTransform")?)?}else{Null};let globals=if tag=="jbig2"{c.blob(field(codec,"globals")?)?}else{Null};c.row("pdf_image_codec",&[Text(tag),transform,globals])?;
 if tag=="ccitt"{c.row("pdf_image_ccitt",&[Int;8])?;}
 objects::dictionary(c,field(v,"extra")?)?;let mask=field(v,"mask")?;let (mask_kind,stencil)=if matches!(mask,D::Null){(Null,Null)}else{let k=kind(mask)?;(Text(k),if k=="stencil"{Text(text(field(mask,"image")?)?)}else{Null})};let data=c.blob(field(v,"data")?)?;
 c.row("pdf_image",&[Text(text(field(v,"id")?)?),Int,Int,color,Int,Int,Int,Int,data,optional_text(field(v,"softMask")?)?,integer(field(v,"softMaskInData")?)?,mask_kind,stencil,Int,optional_text(field(v,"intent")?)?,optional_text(field(v,"optionalContent")?)?,integer(field(v,"structParent")?)?,Int])?;
 colors::reals(c,"pdf_image_real","decode",field(v,"decode")?)?;colors::reals(c,"pdf_image_real","matte",field(v,"matte")?)?;
 if matches!(mask_kind,Text("colorKey")){for value in list(field(mask,"ranges")?)?{c.row("pdf_image_color_key",&[Int,Int,integer(value)?])?;}}Ok(())
}
pub(super) fn form(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{render::ops(c,field(v,"content")?)?;let group=optional(c,field(v,"group")?,group)?;objects::dictionary(c,field(v,"extra")?)?;let mut f=Cells::from(&[Text(text(field(v,"id")?)?)]);f.extend(array(field(v,"bbox")?,4)?);f.extend(matrix(field(v,"matrix")?)?);f.extend([Int,group,optional_text(field(v,"optionalContent")?)?,integer(field(v,"structParent")?)?,Int]);c.row("pdf_form_xobject",&f)}
pub(super) fn state(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 let mask=field(v,"softMask")?;let (mk,mg,mb,mt)=if matches!(mask,D::Null){(Null,Null,Null,Null)}else{let tag=kind(mask)?;(Text(tag),if tag=="none"{Null}else{Text(text(field(mask,"group")?)?)},if tag=="luminosity"{Int}else{Null},optional(c,field(mask,"transfer")?,colors::function)?)};objects::dictionary(c,field(v,"extra")?)?;
 let dash=list(field(v,"dash")?)?;let font=list(field(v,"font")?)?;if (!dash.is_empty()&&dash.len()!=2)||(!font.is_empty()&&font.len()!=2){return Err(invalid())}
 c.row("pdf_ext_g_state",&[Text(text(field(v,"id")?)?),real(field(v,"lineWidth")?)?,optional_text(field(v,"lineCap")?)?,optional_text(field(v,"lineJoin")?)?,real(field(v,"miterLimit")?)?,if dash.is_empty(){Null}else{real(&dash[1])?},optional_text(field(v,"renderingIntent")?)?,integer(field(v,"overprintStroke")?)?,integer(field(v,"overprintFill")?)?,integer(field(v,"overprintMode")?)?,if font.is_empty(){Null}else{Text(text(&font[0])?)},if font.is_empty(){Null}else{real(&font[1])?},Int,mk,mg,mb,mt,real(field(v,"strokeAlpha")?)?,real(field(v,"fillAlpha")?)?,integer(field(v,"alphaIsShape")?)?,integer(field(v,"strokeAdjust")?)?,real(field(v,"flatness")?)?,real(field(v,"smoothness")?)?,integer(field(v,"textKnockout")?)?,Int])?;
 if let Some(a)=dash.first(){colors::reals(c,"pdf_g_state_real","dash",a)?;}
 if matches!(mk,Text("luminosity")){colors::reals(c,"pdf_g_state_real","backdrop",field(mask,"backdrop")?)?;}
 for mode in list(field(v,"blendMode")?)?{c.row("pdf_blend_mode",&[Int,Int,Text(text(mode)?)])?;}Ok(())
}
pub(super) fn shading(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 colors::color(c,field(v,"colorSpace")?)?;objects::dictionary(c,field(v,"extra")?)?;let k=field(v,"kind")?;let tag=kind(k)?;let mut f=Cells::from(&[Text(text(field(v,"id")?)?),Int,Text(tag),Int]);f.extend(array(field(v,"bbox")?,4)?);f.extend([Int,Int]);c.row("pdf_shading",&f)?;colors::sequence(c,"pdf_shading_background",field(v,"background")?)?;
 let func=optional(c,field(k,"function")?,colors::function)?;
 match tag{
 "functionBased"=>{let mut f=array(field(k,"domain")?,4)?;f.extend(array(field(k,"matrix")?,6)?);f.push(func);c.row("pdf_function_shading",&f)?;},
 "axial"|"radial"=>{let mut f=array(field(k,"coords")?,if tag=="axial"{4}else{6})?;f.extend(array(field(k,"domain")?,2)?);f.extend([func,Int,Int]);c.row(if tag=="axial"{"pdf_axial_shading"}else{"pdf_radial_shading"},&f)?;},
 "mesh"=>{let data=c.blob(field(k,"data")?)?;c.row("pdf_mesh_shading",&[Int,Int,Int,integer(field(k,"bitsPerFlag")?)?,integer(field(k,"verticesPerRow")?)?,func,data])?;colors::sequence(c,"pdf_mesh_decode",field(k,"decode")?)?;},_=>return Err(invalid())
 }Ok(())
}
pub(super) fn pattern(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 objects::dictionary(c,field(v,"extra")?)?;let k=field(v,"kind")?;let tag=kind(k)?;let mut f=Cells::from(&[Text(text(field(v,"id")?)?),Text(tag)]);f.extend(matrix(field(v,"matrix")?)?);f.push(Int);c.row("pdf_pattern",&f)?;
 match tag{"tiling"=>{render::ops(c,field(k,"content")?)?;let mut f=Cells::from(&[Int,Int]);f.extend(array(field(k,"bbox")?,4)?);f.extend([real(field(k,"xStep")?)?,real(field(k,"yStep")?)?,Int]);c.row("pdf_tiling_pattern",&f)?;},"shading"=>c.row("pdf_shading_pattern",&[Text(text(field(k,"shading")?)?),optional_text(field(k,"extGState")?)?])?,_=>return Err(invalid())}Ok(())
}
pub(super) fn rect<'a>(v:&'a D,n:usize)->Result<Cells<'a>,ValueError>{array(v,n)}

fn matrix(v:&D)->Result<Cells<'_>,ValueError>{if matches!(v,D::Null){Ok(Cells::from(&[Real(1.0),Real(0.0),Real(0.0),Real(1.0),Real(0.0),Real(0.0)]))}else{array(v,6)}}
