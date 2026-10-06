//! 📝️ Borrowed form and optional content trees measure semantic cells.
use super::*;
fn names(c:&mut Census<'_,'_>,role:&str,v:&D)->Result<(),ValueError>{for name in list(v)?{c.row("pdf_field_name",&[Int,Int,Text(role),Text(text(name)?)])?;}Ok(())}
fn form_field(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 if c.depth>=64{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"PDF borrowed semantic containment depth exceeded"))}c.depth+=1;let result=(||{
 let k=field(v,"kind")?;let tag=kind(k)?;let mut f=[Null;6];
 match tag{"button"=>{f[0]=optional_text(field(k,"value")?)?;f[1]=optional_text(field(k,"defaultValue")?)?;names(c,"option",field(k,"options")?)?;},"text"=>{f[0]=optional_text(field(k,"value")?)?;f[1]=optional_text(field(k,"defaultValue")?)?;f[2]=integer(field(k,"maxLength")?)?;f[3]=optional_text(field(k,"richValue")?)?;},"choice"=>{f[4]=integer(field(k,"topIndex")?)?;names(c,"value",field(k,"values")?)?;names(c,"default",field(k,"defaultValues")?)?;for option in list(field(k,"options")?)?{let a=list(option)?;c.row("pdf_choice_option",&[Int,Int,Text(text(&a[0])?),Text(text(&a[1])?)])?;}},"signature"=>{let value=field(k,"value")?;if !matches!(value,D::Null){objects::dictionary(c,value)?;f[5]=Int;}},"container"=>{},_=>return Err(invalid())}
 objects::dictionary(c,field(v,"additionalActions")?)?;objects::dictionary(c,field(v,"extra")?)?;
 let mut cells=Cells::from(&[Text(text(field(v,"name")?)?),Text(tag),Int,optional_text(field(v,"alternateName")?)?,optional_text(field(v,"mappingName")?)?,optional_text(field(v,"defaultAppearance")?)?,integer(field(v,"quadding")?)?]);cells.extend(f);cells.extend([Int,Int]);c.row("pdf_form_field",&cells)?;
 for _ in list(field(v,"widgets")?)?{c.row("pdf_field_widget",&[Int;4])?;}
 for child in list(field(v,"children")?)?{form_field(c,child)?;c.relation("pdf_field_child")?;}Ok(())
})();c.depth-=1;result
}
pub(super) fn acro(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{objects::dictionary(c,field(v,"extra")?)?;c.row("pdf_acro_form",&[Int,Int,optional_text(field(v,"defaultAppearance")?)?,integer(field(v,"quadding")?)?,Int])?;for child in list(field(v,"fields")?)?{form_field(c,child)?;c.relation("pdf_acro_field")?;}for name in list(field(v,"defaultFonts")?)?{c.row("pdf_acro_font",&[Int,Int,Text(text(name)?)])?;}Ok(())}
pub(super) fn optional(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{objects::dictionary(c,field(v,"extra")?)?;c.row("pdf_optional_content",&[optional_text(field(v,"name")?)?,Int,Int])?;
 for group in list(field(v,"groups")?)?{objects::dictionary(c,field(group,"usage")?)?;c.row("pdf_optional_group",&[Int,Int,Text(text(field(group,"id")?)?),Text(text(field(group,"name")?)?),Int])?;for intent in list(field(group,"intent")?)?{c.row("pdf_optional_group_intent",&[Int,Int,Text(text(intent)?)])?;}}
 for role in ["on","off"]{for name in list(field(v,role)?)?{c.row("pdf_optional_name",&[Int,Int,Text(role),Text(text(name)?)])?;}}
 for value in list(field(v,"order")?)?{objects::object(c,value)?;c.relation("pdf_optional_order")?;}Ok(())
}
