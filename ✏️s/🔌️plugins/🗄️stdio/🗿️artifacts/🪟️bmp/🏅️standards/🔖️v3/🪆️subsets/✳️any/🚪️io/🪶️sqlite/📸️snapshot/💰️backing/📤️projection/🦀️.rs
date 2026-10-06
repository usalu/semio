//! 📤️ Interpreted header, sample, layout and literal occurrence projection.
use super::*;
pub(crate) fn project(snapshot:&BmpSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase>{
 let bytes=capture(&snapshot.bytes,control)?;let layout=layout::inspect(&bytes);
 let diagnostic=match &layout{Ok(_)=>None,Err(failure)=>Some(super::diagnostic(*failure,control)?)};
 let mut output=Projection::new(SQL,control)?;
 output.insert("bmp_document",&[Cell::Text(&snapshot.schema),Cell::Text(if layout.is_ok(){"valid_layout"}else{"literal_octets"}),Cell::Text(diagnostic.as_deref().unwrap_or(""))])?;
 let layout=match layout{Ok(layout)=>layout,Err(_)=>{for(index,value)in bytes.iter().enumerate(){output.insert("bmp_literal_octet",&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Integer(i64::from(*value))])?;}return output.finish();}};
 output.insert("bmp_file_header",&[Cell::Integer(1),Cell::Integer(i64::from(layout.file_size)),Cell::Integer(i64::from(layout.reserved_1)),Cell::Integer(i64::from(layout.reserved_2)),Cell::Integer(ordinal(layout.data_offset)?)])?;
 let signed_height=if layout.row_order==BmpRowOrder::TopDown{-i64::from(layout.height)}else{i64::from(layout.height)};
 output.insert("bmp_info_header",&[Cell::Integer(1),Cell::Integer(40),Cell::Integer(i64::from(layout.width)),Cell::Integer(signed_height),Cell::Integer(i64::from(layout.planes)),Cell::Integer(i64::from(layout.bits_per_pixel)),Cell::Integer(i64::from(layout.compression)),Cell::Integer(i64::from(layout.image_size)),Cell::Integer(i64::from(layout.x_pixels_per_meter)),Cell::Integer(i64::from(layout.y_pixels_per_meter)),Cell::Integer(i64::from(layout.colors_used)),Cell::Integer(i64::from(layout.colors_important))])?;
 let masks=masks(&layout);let union=masks[0]|masks[1]|masks[2];
 if layout.compression==3{for index in 0..3{output.insert("bmp_channel_mask",&[Cell::Integer(1),Cell::Integer(index as i64),Cell::Text(CHANNELS[index]),Cell::Integer(i64::from(masks[index]))])?;}}
 for index in 0..layout.palette_entries{let at=layout.palette_offset+index*4;output.insert("bmp_palette_entry",&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Integer(i64::from(bytes[at])),Cell::Integer(i64::from(bytes[at+1])),Cell::Integer(i64::from(bytes[at+2])),Cell::Integer(i64::from(bytes[at+3]))])?;}
 let bpp=usize::from(layout.bits_per_pixel);let tail=(8-mul(layout.width as usize,bpp)?%8)%8;
 for y in 0..layout.height as usize{
  let row=physical(&layout,y);
  for x in 0..layout.width as usize{
   if bpp<=8{let bit=x*bpp;let value=(bytes[row+bit/8]>>(8-bpp-bit%8))&((1u16<<bpp)-1)as u8;output.insert("bmp_pixel_index",&[Cell::Integer(1),Cell::Integer(ordinal(x)?),Cell::Integer(ordinal(y)?),Cell::Integer(i64::from(value))])?;}
   else{let value=word(&bytes,row+x*bpp/8,bpp/8);let channels=masks.map(|mask|(value&mask)>>mask.trailing_zeros());output.insert("bmp_pixel_sample",&[Cell::Integer(1),Cell::Integer(ordinal(x)?),Cell::Integer(ordinal(y)?),Cell::Integer(i64::from(channels[0])),Cell::Integer(i64::from(channels[1])),Cell::Integer(i64::from(channels[2])),Cell::Integer(i64::from(value&!union))])?;}
  }
  if tail!=0{output.insert("bmp_row_tail_bits",&[Cell::Integer(1),Cell::Integer(ordinal(y)?),Cell::Integer(i64::from(bytes[row+layout.row_payload-1]&((1u16<<tail)-1)as u8))])?;}
  for index in 0..layout.row_stride-layout.row_payload{output.insert("bmp_row_padding_octet",&[Cell::Integer(1),Cell::Integer(ordinal(y)?),Cell::Integer(ordinal(index)?),Cell::Integer(i64::from(bytes[row+layout.row_payload+index]))])?;}
 }
 for index in 0..layout.data_offset-layout.metadata_end{output.insert("bmp_gap_octet",&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Integer(i64::from(bytes[layout.metadata_end+index]))])?;}
 for index in 0..bytes.len()-layout.pixel_end{output.insert("bmp_trailer_octet",&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Integer(i64::from(bytes[layout.pixel_end+index]))])?;}
 output.finish()
}
