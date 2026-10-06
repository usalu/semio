//! 🎯️ Borrowed destinations, linked actions and bookmark containment.
use super::*;
pub(super) fn destination(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 let tag=kind(v)?;let mut fit=[Null;8];let(page,name)=match tag{
  "named"=>(Null,Text(text(field(v,"name")?)?)),
  "page"|"remotePage"=>{let value=field(v,"fit")?;let typ=kind(value)?;fit[0]=Text(typ);match typ{
   "xyz"=>{fit[1]=real(field(value,"left")?)?;fit[2]=real(field(value,"top")?)?;fit[3]=real(field(value,"zoom")?)?;},
   "fit"|"fitBoundingBox"=>{},
   "fitHorizontal"|"fitBoundingBoxHorizontal"=>fit[2]=real(field(value,"top")?)?,
   "fitVertical"|"fitBoundingBoxVertical"=>fit[1]=real(field(value,"left")?)?,
   "fitRectangle"=>{let values=list(field(value,"rect")?)?;if values.len()!=4{return Err(invalid())}for(i,value)in values.iter().enumerate(){fit[4+i]=real(value)?;}},
   _=>return Err(invalid())
  }(Int,Null)},
  _=>return Err(invalid())
 };
 c.row("pdf_destination",&[Text(tag),page,name,fit[0],fit[1],fit[2],fit[3],fit[4],fit[5],fit[6],fit[7]])
}
pub(super) fn file(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{let tag=kind(v)?;let key=match tag{"path"=>"path","embedded"=>"file",_=>return Err(invalid())};c.row("pdf_file_specification",&[Text(tag),Text(text(field(v,key)?)?)])}
pub(super) fn action(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 if c.depth>=64{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"PDF borrowed semantic containment depth exceeded"))}c.depth+=1;let result=(||{
 let value=field(v,"kind")?;let tag=kind(value)?;let mut f=[Null;21];
 match tag{
  "goTo"|"goToEmbedded"|"goToRemote"=>{destination(c,field(value,"destination")?)?;f[0]=Int;if tag=="goToRemote"{file(c,field(value,"file")?)?;f[1]=Int;}if tag!="goTo"{f[2]=integer(field(value,"newWindow")?)?;}},
  "launch"|"importData"=>{file(c,field(value,"file")?)?;f[1]=Int;if tag=="launch"{f[2]=integer(field(value,"newWindow")?)?;}},
  "thread"=>{let file_value=field(value,"file")?;if !matches!(file_value,D::Null){file(c,file_value)?;f[1]=Int;}f[3]=Int;},
  "uri"=>{f[4]=Text(text(field(value,"uri")?)?);f[5]=Int;},
  "sound"=>{f[6]=Text(text(field(value,"sound")?)?);f[7]=real(field(value,"volume")?)?;f[8]=Int;f[9]=Int;f[10]=Int;},
  "movie"=>{f[11]=optional_text(field(value,"annotation")?)?;f[12]=optional_text(field(value,"operation")?)?;},
  "hide"=>f[13]=Int,"named"=>f[14]=Text(text(field(value,"name")?)?),
  "submitForm"=>{f[15]=Text(text(field(value,"url")?)?);f[16]=Int;},"resetForm"=>f[16]=Int,
  "javaScript"=>f[17]=Text(text(field(value,"script")?)?),
  "setOptionalContentState"=>{objects::dictionary(c,field(value,"states")?)?;f[18]=Int;f[19]=Int;},
  "rendition"|"transition"|"goTo3dView"|"unknown"=>{objects::dictionary(c,field(value,"entries")?)?;f[18]=Int;if tag=="unknown"{f[20]=Text(text(field(value,"subtype")?)?);}},
  _=>return Err(invalid())
 }
 let mut cells=[Null;22];cells[0]=Text(tag);cells[1..].copy_from_slice(&f);c.row("pdf_action",&cells)?;
 if matches!(tag,"hide"|"submitForm"|"resetForm"){for name in list(field(value,if tag=="hide"{"annotations"}else{"fields"})?)?{c.row("pdf_action_name",&[Int,Int,Text(text(name)?)])?;}}
 for next in list(field(v,"next")?)?{action(c,next)?;c.relation("pdf_action_next")?;}Ok(())
})();c.depth-=1;result
}
pub(super) fn outline(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 if c.depth>=64{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"PDF borrowed semantic containment depth exceeded"))}c.depth+=1;let result=(||{
 let dest=field(v,"destination")?;let action_value=field(v,"action")?;if !matches!(dest,D::Null){destination(c,dest)?;}if !matches!(action_value,D::Null){action(c,action_value)?;}objects::dictionary(c,field(v,"extra")?)?;
 let color=field(v,"color")?;let mut colors=[Null;3];if !matches!(color,D::Null){let values=list(color)?;if values.len()!=3{return Err(invalid())}for(i,v)in values.iter().enumerate(){colors[i]=real(v)?;}}
 c.row("pdf_outline",&[Text(text(field(v,"title")?)?),if matches!(dest,D::Null){Null}else{Int},if matches!(action_value,D::Null){Null}else{Int},colors[0],colors[1],colors[2],Int,Int,Int,Int])?;
 for next in list(field(v,"children")?)?{outline(c,next)?;c.relation("pdf_outline_child")?;}Ok(())
})();c.depth-=1;result
}
pub(super) fn named(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{destination(c,field(v,"destination")?)?;c.row("pdf_named_destination",&[Text(text(field(v,"name")?)?),Int])}
pub(super) fn label(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{c.row("pdf_page_label",&[Int,optional_text(field(v,"style")?)?,optional_text(field(v,"prefix")?)?,Int])}
pub(super) fn open(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{let tag=kind(v)?;let(dest,act)=match tag{"destination"=>{destination(c,field(v,"destination")?)?;(Int,Null)},"action"=>{action(c,field(v,"action")?)?;(Null,Int)},_=>return Err(invalid())};c.row("pdf_open_action",&[Text(tag),dest,act])}

