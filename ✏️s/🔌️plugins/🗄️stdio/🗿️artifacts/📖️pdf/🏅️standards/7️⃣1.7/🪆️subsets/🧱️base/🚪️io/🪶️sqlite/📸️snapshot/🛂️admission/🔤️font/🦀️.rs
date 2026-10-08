//! 🔤️ Borrowed font entities, embedded programs, CMaps and ordered glyph metrics.
use super::*;
fn encoding(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{c.row("pdf_font_encoding",&[optional_text(field(v,"base")?)?])?;for d in list(field(v,"differences")?)?{c.row("pdf_encoding_difference",&[Int,Int,Int,Text(text(field(d,"glyph")?)?)])?;}Ok(())}
fn descriptor(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 objects::dictionary(c,field(v,"extra")?)?;let bbox_value=field(v,"fontBbox")?;let mut bbox=[Real(0.0);4];if !matches!(bbox_value,D::Null){let values=list(bbox_value)?;if values.len()!=4{return Err(invalid())}for(i,v)in values.iter().enumerate(){bbox[i]=real(v)?;}}
 let mut cells=[Null;22];cells[0]=Text(text(field(v,"fontName")?)?);cells[1]=Int;for(i,v)in bbox.iter().enumerate(){cells[2+i]=*v;}
 for(i,key)in["italicAngle","ascent","descent","capHeight","stemV","stemH","xHeight","leading","avgWidth","maxWidth","missingWidth"].iter().enumerate(){cells[6+i]=if i<5{real_or(field(v,key)?,0.0)?}else{real(field(v,key)?)?};}
 cells[17]=optional_text(field(v,"fontFamily")?)?;cells[18]=optional_text(field(v,"fontStretch")?)?;cells[19]=real(field(v,"fontWeight")?)?;cells[20]=optional_text(field(v,"charSet")?)?;cells[21]=Int;c.row("pdf_font_descriptor",&cells)
}
pub(super) fn program(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{let tag=kind(v)?;if !matches!(tag,"type1"|"trueType"|"cff"|"cidCff"|"openType"){return Err(invalid())}artifact_reference(c,field(v,"reference")?)?;c.row("pdf_font_program",&[Text(tag),Int])}
pub(super) fn unicode(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 c.row("pdf_to_unicode",&[Int])?;for v in list(field(v,"mappings")?)?{let tag=kind(v)?;let cols=match tag{"char"=>[Int,Null,Null],"range"=>[Null,Int,Int],_=>return Err(invalid())};c.row("pdf_unicode_mapping",&[Int,Int,Text(tag),cols[0],cols[1],cols[2],Text(text(field(v,"text")?)?)])?;}Ok(())
}
pub(super) fn cmap(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 match kind(v)?{
  "predefined"=>c.row("pdf_cmap",&[Text("predefined"),Text(text(field(v,"name")?)?),Null,Null]),
  "embedded"=>embedded_cmap(c,field(v,"cmap")?),_=>Err(invalid())
 }
}
fn cid(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 descriptor(c,field(v,"descriptor")?)?;let program_value=field(v,"program")?;if !matches!(program_value,D::Null){program(c,program_value)?;}
 objects::dictionary(c,field(v,"extra")?)?;let map=field(v,"cidToGid")?;let(typ,data)=if matches!(map,D::Null){(Null,Null)}else{match kind(map)?{"identity"=>(Text("identity"),Null),"map"=>{for glyph in list(field(map,"glyphs")?)? {c.row("pdf_cid_glyph",&[Int,Int,integer(glyph)?])?;}(Text("map"),Int)},_=>return Err(invalid())}};
 let system=field(v,"systemInfo")?;let vertical=field(v,"defaultVertical")?;let vertical=if matches!(vertical,D::Null){[Null;2]}else{let values=list(vertical)?;if values.len()!=2{return Err(invalid())}[real(&values[0])?,real(&values[1])?]};
 c.row("pdf_cid_font",&[Int,Text(text(field(v,"baseFont")?)?),Text(if matches!(system,D::Null){"Adobe"}else{text(field(system,"registry")?)?}),Text(if matches!(system,D::Null){"Identity"}else{text(field(system,"ordering")?)?}),Int,Int,real_or(field(v,"defaultWidth")?,1000.0)?,vertical[0],vertical[1],typ,data,if matches!(program_value,D::Null){Null}else{Int},Int])?;
 for run in list(field(v,"widths")?)?{c.relation("pdf_cid_width_run")?;for width in list(field(run,"widths")?)?{c.row("pdf_cid_width",&[Int,Int,real(width)?])?;}}
 for run in list(field(v,"verticalMetrics")?)?{c.relation("pdf_cid_vertical_run")?;for metric in list(field(run,"metrics")?)?{let values=list(metric)?;if values.len()!=3{return Err(invalid())}c.row("pdf_cid_vertical_metric",&[Int,Int,real(&values[0])?,real(&values[1])?,real(&values[2])?])?;}}Ok(())
}
pub(super) fn font(c:&mut Census<'_,'_>,v:&D)->Result<(),ValueError>{
 let subtype=field(v,"kind")?;let tag=kind(subtype)?;let mut fields=[Null;17];
 match tag{
  "type1"|"trueType"|"type3"=>{if tag!="type3"{fields[0]=Text(text(field(subtype,"baseFont")?)?);}encoding(c,field(subtype,"encoding")?)?;fields[1]=Int;fields[2]=Int;let value=field(subtype,"descriptor")?;if !matches!(value,D::Null){descriptor(c,value)?;fields[3]=Int;}let value=field(subtype,"program")?;if tag!="type3"&&!matches!(value,D::Null){program(c,value)?;fields[4]=Int;}if tag=="type3"{let matrix=list(field(subtype,"fontMatrix")?)?;let bbox=list(field(subtype,"fontBbox")?)?;if matrix.len()!=6||bbox.len()!=4{return Err(invalid())}for(i,value)in matrix.iter().chain(bbox).enumerate(){fields[5+i]=real(value)?;}}},
  "type0"=>{fields[0]=Text(text(field(subtype,"baseFont")?)?);cmap(c,field(subtype,"cmap")?)?;cid(c,field(subtype,"descendant")?)?;fields[15]=Int;fields[16]=Int;},
  _=>return Err(invalid())
 }
 let unicode_value=field(v,"toUnicode")?;if !matches!(unicode_value,D::Null){unicode(c,unicode_value)?;}objects::dictionary(c,field(v,"extra")?)?;
 let mut cells=[Null;21];cells[0]=Text(text(field(v,"id")?)?);cells[1]=Text(tag);cells[2..19].copy_from_slice(&fields);cells[19]=if matches!(unicode_value,D::Null){Null}else{Int};cells[20]=Int;c.row("pdf_font",&cells)?;
 if tag!="type0"{for width in list(field(subtype,"widths")?)?{c.row("pdf_font_width",&[Int,Int,real(width)?])?;}}
 if tag=="type3"{for procedure in list(field(subtype,"charProcs")?)?{render::ops(c,field(procedure,"content")?)?;c.row("pdf_char_proc",&[Int,Int,Text(text(field(procedure,"name")?)?),Int])?;}}Ok(())
}

pub(super) fn embedded_cmap(c:&mut Census<'_ ,'_>,v:&D)->Result<(),ValueError>{c.row("pdf_cmap",&[Text("embedded"),Text(text(field(v,"name")?)?),Int,optional_text(field(v,"useCmap")?)?])?;for _ in list(field(v,"codespace")?)?{c.row("pdf_codespace_range",&[Int;5])?;}for v in list(field(v,"mappings")?)?{let tag=kind(v)?;let cols=match tag{"char"=>[Int,Null,Null],"range"=>[Null,Int,Int],_=>return Err(invalid())};c.row("pdf_cid_mapping",&[Int,Int,Text(tag),cols[0],cols[1],cols[2],Int])?;}Ok(())}
