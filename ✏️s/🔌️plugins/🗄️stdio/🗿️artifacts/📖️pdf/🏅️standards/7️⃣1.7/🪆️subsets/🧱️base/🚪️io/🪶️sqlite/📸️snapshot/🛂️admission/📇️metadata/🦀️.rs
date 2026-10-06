//! 📇️ Borrowed dates, information, files and document preference cells.
use super::*;
pub(super) fn date(c:&mut Census<'_,'_>,value:&D)->Result<(),ValueError>{c.row("pdf_date",&[Int,Int,Int,Int,Int,Int,integer(field(value,"offsetMinutes")?)?])}
pub(super) fn info(c:&mut Census<'_,'_>,value:&D)->Result<(),ValueError>{
 let creation=field(value,"creationDate")?;let modification=field(value,"modificationDate")?;
 if !matches!(creation,D::Null){date(c,creation)?;}if !matches!(modification,D::Null){date(c,modification)?;}
 objects::dictionary(c,field(value,"extra")?)?;
 c.row("pdf_document_info",&[optional_text(field(value,"title")?)?,optional_text(field(value,"author")?)?,optional_text(field(value,"subject")?)?,optional_text(field(value,"keywords")?)?,optional_text(field(value,"creator")?)?,optional_text(field(value,"producer")?)?,if matches!(creation,D::Null){Null}else{Int},if matches!(modification,D::Null){Null}else{Int},optional_text(field(value,"trapped")?)?,Int])
}
pub(super) fn file(c:&mut Census<'_,'_>,value:&D)->Result<(),ValueError>{
 let creation=field(value,"creationDate")?;let modification=field(value,"modificationDate")?;
 if !matches!(creation,D::Null){date(c,creation)?;}if !matches!(modification,D::Null){date(c,modification)?;}
 let bytes=c.blob(field(value,"data")?)?;
 c.row("pdf_embedded_file",&[Text(text(field(value,"id")?)?),Text(text(field(value,"fileName")?)?),optional_text(field(value,"description")?)?,optional_text(field(value,"mimeType")?)?,bytes,if matches!(creation,D::Null){Null}else{Int},if matches!(modification,D::Null){Null}else{Int},optional_text(field(value,"relationship")?)?,Int])
}
pub(super) fn intent(c:&mut Census<'_,'_>,value:&D)->Result<(),ValueError>{
 let bytes=c.blob(field(value,"profile")?)?;
 c.row("pdf_output_intent",&[Text(text(field(value,"subtype")?)?),Text(text(field(value,"conditionIdentifier")?)?),optional_text(field(value,"condition")?)?,optional_text(field(value,"registryName")?)?,optional_text(field(value,"info")?)?,bytes])
}
pub(super) fn encryption(c:&mut Census<'_,'_>,value:&D)->Result<(),ValueError>{
 let algorithm=text(field(value,"algorithm")?)?;
 let algorithm=match algorithm{"rc4_40"=>"rc4_40","rc4_128"=>"rc4_128","aes128"=>"aes128","aes256"=>"aes256",_=>return Err(invalid())};
 c.row("pdf_encryption",&[Text(algorithm),Int,Text(text(field(value,"userPassword")?)?),optional_text(field(value,"ownerPassword")?)?,Int])
}
pub(super) fn preferences(c:&mut Census<'_,'_>,value:&D)->Result<(),ValueError>{
 objects::dictionary(c,field(value,"extra")?)?;
 c.row("pdf_viewer_preferences",&[Int,Int,Int,Int,Int,Int,optional_text(field(value,"nonFullScreenPageMode")?)?,optional_text(field(value,"direction")?)?,optional_text(field(value,"viewArea")?)?,optional_text(field(value,"viewClip")?)?,optional_text(field(value,"printArea")?)?,optional_text(field(value,"printClip")?)?,optional_text(field(value,"printScaling")?)?,optional_text(field(value,"duplex")?)?,Int,integer(field(value,"numCopies")?)?,Int])?;
 for _ in list(field(value,"printPageRange")?)?{c.relation("pdf_print_page_range")?;}Ok(())
}
pub(super) fn mark(c:&mut Census<'_,'_>,_:&D)->Result<(),ValueError>{c.row("pdf_mark_info",&[Int;3])}

