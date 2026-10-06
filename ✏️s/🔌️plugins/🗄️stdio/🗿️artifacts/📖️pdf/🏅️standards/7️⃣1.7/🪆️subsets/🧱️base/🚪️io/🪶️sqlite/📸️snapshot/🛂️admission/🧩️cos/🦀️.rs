//! 🧩️ Borrowed COS nodes, ordered membership and actual stream filter rows.
use super::*;
pub(super) fn filters(c:&mut Census<'_,'_>,values:&D)->Result<(),ValueError>{
 c.row("pdf_filter_chain",&[])?;
 for value in list(values)?{
  let tag=kind(value)?;let mut cells=[Null;4];let predictor=match tag{
   "flate"=>field(value,"predictor")?,"lzw"=>{cells[0]=integer(field(value,"earlyChange")?)?;field(value,"predictor")?},
   "dct"=>{cells[1]=integer(field(value,"colorTransform")?)?;&D::Null},
   "jbig2"=>{cells[2]=c.blob(field(value,"globals")?)?;&D::Null},
   "crypt"=>{cells[3]=optional_text(field(value,"name")?)?;&D::Null},
   "asciiHex"|"ascii85"|"runLength"|"jpx"|"ccitt"=>&D::Null,_=>return Err(invalid())
  };
  c.row("pdf_stream_filter",&[Int,Int,Text(tag),cells[0],cells[1],cells[2],cells[3]])?;
  if !matches!(predictor,D::Null){c.row("pdf_filter_predictor",&[Int,Int,Int,Int])?;}
  if tag=="ccitt"{c.row("pdf_filter_ccitt",&[Int;8])?;}
 }
 Ok(())
}
pub(super) fn dictionary(c:&mut Census<'_,'_>,values:&D)->Result<(),ValueError>{
 if c.depth>=64{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"PDF borrowed semantic containment depth exceeded"))}c.depth+=1;let result=(||{
 c.row("pdf_cos_value",&[Text("dictionary"),Null,Null,Null,Null,Null,Null,Null,Null,Null,Null,Null])?;
 for entry in list(values)?{object(c,field(entry,"value")?)?;c.row("pdf_cos_dictionary_entry",&[Int,Int,Text(text(field(entry,"key")?)?),Int])?;}
 Ok(())
})();c.depth-=1;result
}
pub(super) fn object(c:&mut Census<'_,'_>,value:&D)->Result<(),ValueError>{
 if c.depth>=64{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"PDF borrowed semantic containment depth exceeded"))}c.depth+=1;let result=(||{
 let tag=kind(value)?;let mut fields=[Null;11];let kind=match tag{
  "null"=>"null","bool"=>{fields[0]=Int;"boolean"},"int"=>{fields[1]=Int;"integer"},
  "real"=>{fields[2]=Int;fields[3]=Text(text(field(value,"coefficient")?)?);fields[4]=Int;"decimal"},
  "str"=>{fields[5]=c.blob(field(value,"value")?)?;"string"},
  "name"=>{fields[6]=Text(text(field(value,"value")?)?);"name"},
  "array"=>"array","dict"=>"dictionary","ref"=>{fields[7]=Int;fields[8]=Int;"reference"},
  "stream"=>{fields[9]=c.blob(field(value,"data")?)?;filters(c,field(value,"filters")?)?;fields[10]=Int;"stream"},
  _=>return Err(invalid())
 };
 c.row("pdf_cos_value",&[Text(kind),fields[0],fields[1],fields[2],fields[3],fields[4],fields[5],fields[6],fields[7],fields[8],fields[9],fields[10]])?;
 match tag{
  "array"=>for value in list(field(value,"value")?)?{object(c,value)?;c.relation("pdf_cos_array_element")?;},
  "dict"|"stream"=>for entry in list(field(value,if tag=="dict"{"value"}else{"dict"})?)?{object(c,field(entry,"value")?)?;c.row("pdf_cos_dictionary_entry",&[Int,Int,Text(text(field(entry,"key")?)?),Int])?;},
  _=>{}
 }
 Ok(())
})();c.depth-=1;result
}

