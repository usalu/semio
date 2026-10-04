//! 📥️ Complete native reconstruction from individually validated semantic occurrences.
use super::*;

pub(crate) fn reconstruct(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<BmpSnapshot>{
 validate_sqlite_database_schema_controlled(database,SQL,RECONSTRUCT,control)?;
 control.check_database(database,RECONSTRUCT)?;
 let document=singleton(database.table("bmp_document")?)?;
 let schema=reconstruct_text(control,document.text(1)?)?;
 let state=document.text(2)?;let diagnostic=document.text(3)?;
 if state=="literal_octets"{
  if diagnostic.is_empty(){return Err(invalid("BMP literal state requires its actual layout diagnostic"));}
  for name in ["bmp_file_header","bmp_info_header","bmp_channel_mask","bmp_palette_entry","bmp_pixel_index","bmp_pixel_sample","bmp_row_tail_bits","bmp_row_padding_octet","bmp_gap_octet","bmp_trailer_octet"]{count(database.table(name)?,0)?;}
  let rows=ordered(database.table("bmp_literal_octet")?,control)?;
  control.admit_reconstruction_bytes(rows.len())?;let mut bytes=zeros(rows.len(),control)?;
  for(index,row)in rows.iter().enumerate(){bytes[index]=number(row,3,255)? as u8;if(index+1)%256==0{control.checkpoint(RECONSTRUCT,index+1,rows.len())?;}}
  match layout::inspect(&bytes){Err(failure)if diagnostic_matches(failure,diagnostic)=>{},_=>return Err(invalid("BMP literal state disagrees with its actual layout diagnostic"))}
  control.checkpoint(RECONSTRUCT,rows.len(),rows.len())?;
  return Ok(BmpSnapshot{schema,bytes});
 }
 if state!="valid_layout"||!diagnostic.is_empty(){return Err(invalid("BMP layout state is invalid"));}
 count(database.table("bmp_literal_octet")?,0)?;
 let file=singleton(database.table("bmp_file_header")?)?;let info=singleton(database.table("bmp_info_header")?)?;scoped(file)?;scoped(info)?;
 let file_size=number(file,2,u32::MAX)?;let reserved_1=number(file,3,65535)? as u16;let reserved_2=number(file,4,65535)? as u16;let data_offset=usize::try_from(number(file,5,u32::MAX)?).map_err(|_|ownership("BMP pixel offset exceeds address space"))?;
 let header_size=number(info,2,u32::MAX)?;let width=number(info,3,i32::MAX as u32)?;let signed_height=integer(info,4,-i64::from(i32::MAX),i64::from(i32::MAX))? as i32;let height=signed_height.unsigned_abs();let planes=number(info,5,65535)? as u16;let bits_per_pixel=number(info,6,65535)? as u16;let compression=number(info,7,u32::MAX)?;
 let image_size=number(info,8,u32::MAX)?;let x_pixels_per_meter=integer(info,9,i64::from(i32::MIN),i64::from(i32::MAX))? as i32;let y_pixels_per_meter=integer(info,10,i64::from(i32::MIN),i64::from(i32::MAX))? as i32;let colors_used=number(info,11,u32::MAX)?;let colors_important=number(info,12,u32::MAX)?;
 if header_size!=40||planes!=1||(width==0)!=(height==0)||!((compression==0&&matches!(bits_per_pixel,1|4|8|16|24|32))||(compression==3&&matches!(bits_per_pixel,16|32))){return Err(invalid("BMP header is outside the actual native grammar"));}
 let bpp=usize::from(bits_per_pixel);let indexed=bpp<=8;let max_sample=if bpp==32{u32::MAX}else{(1u32<<bpp)-1};
 let mut masks=if bpp==16{[0x7c00,0x03e0,0x001f]}else if bpp>=24{[0xff0000,0xff00,0xff]}else{[0;3]};
 if compression==3{
  let rows=ordered(database.table("bmp_channel_mask")?,control)?;if rows.len()!=3{return Err(invalid("BMP requires three explicit bitfield masks"));}
  for(index,row)in rows.iter().enumerate(){if row.text(3)?!=CHANNELS[index]{return Err(invalid("BMP mask ordinal has another channel"));}let mask=number(row,4,max_sample)?;if mask==0{return Err(invalid("BMP channel mask is zero"));}let shifted=mask>>mask.trailing_zeros();if shifted&shifted.wrapping_add(1)!=0{return Err(invalid("BMP channel mask is not contiguous"));}masks[index]=mask;}
  if masks[0]&masks[1]!=0||masks[0]&masks[2]!=0||masks[1]&masks[2]!=0{return Err(invalid("BMP channel masks overlap"));}
 }else{count(database.table("bmp_channel_mask")?,0)?;}
 let palette_entries=if indexed{if colors_used==0{1usize<<bpp}else{usize::try_from(colors_used).map_err(|_|ownership("BMP palette count exceeds address space"))?}}else{0};
 if indexed&&palette_entries>1usize<<bpp{return Err(invalid("BMP palette exceeds its sample capacity"));}
 let palette_offset=if compression==3{66}else{54};let metadata_end=add(palette_offset,mul(palette_entries,4)?)?;
 if data_offset<metadata_end{return Err(invalid("BMP metadata overlaps its pixel offset"));}
 let row_bits=mul(width as usize,bpp)?;let row_stride=mul(add(row_bits,31)?/32,4)?;let row_payload=add(row_bits,7)?/8;
 let pixel_bytes=mul(row_stride,height as usize)?;let pixel_end=add(data_offset,pixel_bytes)?;let total=add(pixel_end,database.table("bmp_trailer_octet")?.rows.len())?;
 if file_size!=0&&(u64::from(file_size)<pixel_end as u64||u64::from(file_size)>total as u64)||image_size!=0&&u64::from(image_size)<pixel_bytes as u64{return Err(invalid("BMP declared sizes do not contain the checked layout"));}
 let pixels=mul(width as usize,height as usize)?;let tail=(8-row_bits%8)%8;let padding_width=row_stride-row_payload;let padding_count=mul(padding_width,height as usize)?;
 count(database.table("bmp_palette_entry")?,palette_entries)?;count(database.table("bmp_gap_octet")?,data_offset-metadata_end)?;
 let rows=database.table(if indexed{"bmp_pixel_index"}else{"bmp_pixel_sample"})?;
 count(database.table(if indexed{"bmp_pixel_sample"}else{"bmp_pixel_index"})?,0)?;count(rows,pixels)?;
 count(database.table("bmp_row_tail_bits")?,if tail==0{0}else{height as usize})?;count(database.table("bmp_row_padding_octet")?,padding_count)?;
 control.admit_reconstruction_bytes(total)?;
 let mut bytes=zeros(total,control)?;bytes[0]=66;bytes[1]=77;
 store(&mut bytes,2,4,file_size);store(&mut bytes,6,2,u32::from(reserved_1));store(&mut bytes,8,2,u32::from(reserved_2));store(&mut bytes,10,4,data_offset as u32);
 for(at,value,width_)in [(14,40,4),(18,width,4),(22,signed_height as u32,4),(26,u32::from(planes),2),(28,u32::from(bits_per_pixel),2),(30,compression,4),(34,image_size,4),(38,x_pixels_per_meter as u32,4),(42,y_pixels_per_meter as u32,4),(46,colors_used,4),(50,colors_important,4)]{store(&mut bytes,at,width_,value);}
 if compression==3{for(index,mask)in masks.iter().enumerate(){store(&mut bytes,54+index*4,4,*mask);}}
 let palette=ordered(database.table("bmp_palette_entry")?,control)?;
 for(index,row)in palette.iter().enumerate(){for channel in 0..4{bytes[palette_offset+index*4+channel]=number(row,3+channel,255)? as u8;}}
 let physical=|y:usize|data_offset+(if signed_height<0{y}else{height as usize-1-y})*row_stride;
 let mut seen=zeros(pixels,control)?;let union=masks[0]|masks[1]|masks[2];
 for(index,row)in rows.rows.iter().enumerate(){
  scoped(row)?;let x=number(row,2,width.checked_sub(1).ok_or_else(||invalid("BMP empty layout has a pixel"))?)? as usize;let y=number(row,3,height.checked_sub(1).ok_or_else(||invalid("BMP empty layout has a pixel"))?)? as usize;
  let at=y*width as usize+x;if seen[at]!=0{return Err(invalid("BMP pixel coordinates are duplicated"));}seen[at]=1;
  if indexed{let sample=number(row,4,max_sample)?;let bit=x*bpp;bytes[physical(y)+bit/8]|=(sample<<(8-bpp-bit%8))as u8;}
  else{let mut value=number(row,7,max_sample)?;if value&union!=0{return Err(invalid("BMP unused sample bits overlap a channel"));}for(channel,mask)in masks.iter().enumerate(){let shift=mask.trailing_zeros();value|=number(row,4+channel,*mask>>shift)?<<shift;}store(&mut bytes,physical(y)+x*bpp/8,bpp/8,value);}
  if(index+1)%256==0{control.checkpoint(RECONSTRUCT,index+1,rows.rows.len())?;}
 }
 let mut tail_seen=zeros(if tail==0{0}else{height as usize},control)?;
 for (n,row) in database.table("bmp_row_tail_bits")?.rows.iter().enumerate(){scoped(row)?;let y=number(row,2,height.checked_sub(1).ok_or_else(||invalid("BMP empty layout has a tail"))?)? as usize;if tail_seen[y]!=0{return Err(invalid("BMP row tail occurs twice"));}tail_seen[y]=1;bytes[physical(y)+row_payload-1]|=number(row,3,(1u32<<tail)-1)? as u8;if (n+1)%256==0||n+1==tail_seen.len(){control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,n+1,tail_seen.len())?;}}
 let mut padding_seen=zeros(padding_count,control)?;
 for(index,row)in database.table("bmp_row_padding_octet")?.rows.iter().enumerate(){scoped(row)?;let y=number(row,2,height.checked_sub(1).ok_or_else(||invalid("BMP empty layout has padding"))?)? as usize;let ordinal_=usize::try_from(integer(row,3,0,ordinal(padding_width.checked_sub(1).ok_or_else(||invalid("BMP aligned row has padding"))?)?)?).map_err(|_|ownership("BMP padding ordinal exceeds address space"))?;let at=y*padding_width+ordinal_;if padding_seen[at]!=0{return Err(invalid("BMP row padding occurs twice"));}padding_seen[at]=1;bytes[physical(y)+row_payload+ordinal_]=number(row,4,255)? as u8;if(index+1)%256==0{control.checkpoint(RECONSTRUCT,index+1,padding_count)?;}}
 let gap=ordered(database.table("bmp_gap_octet")?,control)?;let trailer=ordered(database.table("bmp_trailer_octet")?,control)?;
 for(index,row)in gap.iter().enumerate(){bytes[metadata_end+index]=number(row,3,255)? as u8;if(index+1)%256==0{control.checkpoint(RECONSTRUCT,index+1,gap.len())?;}}
 for(index,row)in trailer.iter().enumerate(){bytes[pixel_end+index]=number(row,3,255)? as u8;if(index+1)%256==0{control.checkpoint(RECONSTRUCT,index+1,trailer.len())?;}}
 if layout::inspect(&bytes).is_err(){return Err(invariant("BMP semantic reconstruction violated its checked grammar"));}
 control.checkpoint(RECONSTRUCT,rows.rows.len(),rows.rows.len())?;
 Ok(BmpSnapshot{schema,bytes})
}
