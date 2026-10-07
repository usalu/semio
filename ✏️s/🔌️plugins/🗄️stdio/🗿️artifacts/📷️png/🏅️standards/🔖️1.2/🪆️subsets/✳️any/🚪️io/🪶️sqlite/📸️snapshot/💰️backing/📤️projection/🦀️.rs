//! 📤️ Precise owned PNG image fields project directly to semantic rows.
use super::*;
pub(crate) fn project(snapshot:&PngSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase> {admit(snapshot,PROJECT,control)?;let mut output=RowWriter::new(SQL,control)?;write(snapshot,&mut output)?;output.finish()}
pub(super) fn write(snapshot:&PngSnapshot,output:&mut RowWriter<'_,'_>)->Result<()> {
 let image=&snapshot.image;
 output.insert("png_image",&[Cell::Text(&snapshot.schema),Cell::Integer(image.width.into()),Cell::Integer(image.height.into()),Cell::Integer(image.bit_depth.into()),Cell::Integer(image.color_type.to_u8().into()),Cell::Integer(i64::from(image.interlace)),Cell::Integer(i64::from(image.palette.is_some()))])?;
 for (index,sample) in image.samples.iter().enumerate() {output.insert("png_sample",&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Integer((*sample).into())])?;}
 if let Some(palette)=&image.palette {for (index,entry) in palette.iter().enumerate() {output.insert("png_palette_entry",&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Integer(entry.r.into()),Cell::Integer(entry.g.into()),Cell::Integer(entry.b.into())])?;}}
 if let Some(transparency)=&image.transparency {let(kind,first,second,third)=match transparency {PngTransparency::Indexed {alpha}=>{for (index,value) in alpha.iter().enumerate() {output.insert("png_alpha",&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Integer((*value).into())])?;}(3,0,0,0)},PngTransparency::Grayscale {gray}=>(0,*gray,0,0),PngTransparency::Rgb {r,g,b}=>(2,*r,*g,*b)};output.insert("png_transparency",&[Cell::Integer(kind),Cell::Integer(first.into()),Cell::Integer(second.into()),Cell::Integer(third.into())])?;}
 if let Some(background)=&image.background {let(kind,first,second,third)=match background {PngBackground::Indexed {index}=>(3,u16::from(*index),0,0),PngBackground::Grayscale {gray}=>(0,*gray,0,0),PngBackground::Rgb {r,g,b}=>(2,*r,*g,*b)};output.insert("png_background",&[Cell::Integer(kind),Cell::Integer(first.into()),Cell::Integer(second.into()),Cell::Integer(third.into())])?;}
 if let Some(gamma)=image.gamma {output.insert("png_gamma",&[Cell::Integer(gamma.into())])?;}
 if let Some(c)=image.chromaticities {output.insert("png_chromaticities",&[Cell::Integer(c.white_x.into()),Cell::Integer(c.white_y.into()),Cell::Integer(c.red_x.into()),Cell::Integer(c.red_y.into()),Cell::Integer(c.green_x.into()),Cell::Integer(c.green_y.into()),Cell::Integer(c.blue_x.into()),Cell::Integer(c.blue_y.into())])?;}
 if let Some(intent)=image.srgb {output.insert("png_srgb",&[Cell::Integer(intent.to_u8().into())])?;}
 if let Some(p)=image.physical_dims {output.insert("png_physical_dims",&[Cell::Integer(p.ppu_x.into()),Cell::Integer(p.ppu_y.into()),Cell::Integer(i64::from(p.unit_is_meter))])?;}
 if let Some(t)=image.timestamp {output.insert("png_timestamp",&[Cell::Integer(t.year.into()),Cell::Integer(t.month.into()),Cell::Integer(t.day.into()),Cell::Integer(t.hour.into()),Cell::Integer(t.minute.into()),Cell::Integer(t.second.into())])?;}
 for (index,text) in image.text_chunks.iter().enumerate() {output.insert("png_text",&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Text(&text.keyword),Cell::Text(&text.value),Cell::Integer(match text.kind {PngTextKind::Text=>0,PngTextKind::ZText=>1,PngTextKind::IText=>2}),Cell::Integer(i64::from(text.compressed)),Cell::Text(&text.language_tag),Cell::Text(&text.translated_keyword)])?;}
 for (index,chunk) in image.ancillary_chunks.iter().enumerate() {output.insert("png_ancillary_chunk",&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Blob(&chunk.kind),Cell::Integer(i64::from(chunk.after_raster)),Cell::Blob(&chunk.data)])?;}
 Ok(())
}
