//! 📤️ Owned native image fields projected directly to semantic rows.
use super::*;
pub(crate) fn project(snapshot:&BmpSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase> {admit(snapshot,PROJECT,control)?;let mut output=RowWriter::new(SQL,control)?;write(snapshot,&mut output)?;output.finish()}
pub(super) fn write(snapshot:&BmpSnapshot,output:&mut RowWriter<'_,'_>)->Result<()> {
 let image = &snapshot.image;

 output.insert("bmp_image", &[Cell::Text(&snapshot.schema),Cell::Integer(image.width.into()),Cell::Integer(image.height.into()),Cell::Text(if image.row_order == BmpRowOrder::TopDown { "topDown" } else { "bottomUp" }),Cell::Text(image.profile.id()),Cell::Integer(image.masks[0].into()),Cell::Integer(image.masks[1].into()),Cell::Integer(image.masks[2].into()),Cell::Integer(image.masks[3].into()),Cell::Integer(image.x_pixels_per_meter.into()),Cell::Integer(image.y_pixels_per_meter.into()),Cell::Integer(image.colors_used.into()),Cell::Integer(image.colors_important.into()),Cell::Integer(image.reserved_1.into()),Cell::Integer(image.reserved_2.into())])?;
 for (index,entry) in image.palette.iter().enumerate() { output.insert("bmp_palette_entry", &[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Integer(entry.b.into()),Cell::Integer(entry.g.into()),Cell::Integer(entry.r.into()),Cell::Integer(entry.reserved.into())])?; }
 match &image.pixels {
  BmpPixels::Indexed { indices } => { for (index,value) in indices.iter().enumerate() { output.insert("bmp_pixel_index", &[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Integer((*value).into())])?; } }
  BmpPixels::Direct { samples } => { for (index,sample) in samples.iter().enumerate() { output.insert("bmp_pixel_sample", &[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Integer(sample.red.into()),Cell::Integer(sample.green.into()),Cell::Integer(sample.blue.into()),Cell::Integer(sample.alpha.into()),Cell::Integer(sample.reserved.into())])?; } }
 }
 for (table,bytes) in [("bmp_gap_octet",&image.opaque_gap),("bmp_trailer_octet",&image.opaque_trailer)] { for (index,value) in bytes.iter().enumerate() { output.insert(table,&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Integer((*value).into())])?; } }
 Ok(())
}
