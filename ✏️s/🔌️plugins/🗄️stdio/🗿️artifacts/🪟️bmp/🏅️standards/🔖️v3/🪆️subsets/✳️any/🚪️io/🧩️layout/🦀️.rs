//! 🪟️ Borrowed v3 layout grammar and allocation-free refusal facts.
use super::{BmpLayout,BmpProfile,BmpRowOrder,BMP_MAGIC,BITMAPINFOHEADER_SIZE,BI_RGB,BI_BITFIELDS};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum LayoutFailure{Static(&'static str),Truncated(&'static str),RangeOverflow(&'static str),Dib(u32),Planes(u16),RgbDepth(u16),BitfieldDepth(u16),Compression(u32),PixelEnd(usize,usize),FileSize(u32),ImageSize(u32,usize),MaskContiguous(usize),PaletteCount(usize,u16,usize),Offset(usize,usize)}
impl std::fmt::Display for LayoutFailure{
 fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{
  Self::Static(message)=>f.write_str(message),Self::Truncated(name)=>write!(f,"bmp: truncated {name}"),Self::RangeOverflow(name)=>write!(f,"bmp: {name} range overflow"),
  Self::Dib(size)=>write!(f,"bmp: v3 requires a 40-byte BITMAPINFOHEADER; DIB profile {size} must use its own standard"),
  Self::Planes(value)=>write!(f,"bmp: planes must be 1, got {value}"),Self::RgbDepth(value)=>write!(f,"bmp: unsupported BI_RGB bit depth {value}"),Self::BitfieldDepth(value)=>write!(f,"bmp: BI_BITFIELDS requires 16 or 32 bits per pixel, got {value}"),Self::Compression(value)=>write!(f,"bmp: compression profile {value} is outside the uncompressed v3 standard"),
  Self::PixelEnd(end,len)=>write!(f,"bmp: pixel storage ends at {end}, beyond {len} source bytes"),Self::FileSize(value)=>write!(f,"bmp: declared file size {value} does not contain the checked image and fit the source"),Self::ImageSize(value,checked)=>write!(f,"bmp: declared image size {value} is smaller than {checked} checked row bytes"),
  Self::MaskContiguous(index)=>write!(f,"bmp: channel mask {index} is not contiguous"),Self::PaletteCount(count,bpp,capacity)=>write!(f,"bmp: palette has {count} declared entries, beyond {bpp}-bit capacity {capacity}"),Self::Offset(offset,end)=>write!(f,"bmp: pixel offset {offset} overlaps metadata ending at {end}")
 }}
}
fn range<'a>(bytes:&'a[u8],at:usize,len:usize,name:&'static str)->Result<&'a[u8],LayoutFailure>{bytes.get(at..at.checked_add(len).ok_or(LayoutFailure::RangeOverflow(name))?).ok_or(LayoutFailure::Truncated(name))}
fn u16_(bytes:&[u8],at:usize,name:&'static str)->Result<u16,LayoutFailure>{let v=range(bytes,at,2,name)?;Ok(u16::from_le_bytes([v[0],v[1]]))}
fn u32_(bytes:&[u8],at:usize,name:&'static str)->Result<u32,LayoutFailure>{let v=range(bytes,at,4,name)?;Ok(u32::from_le_bytes([v[0],v[1],v[2],v[3]]))}
/// 📐️ Reads complete native layout facts without constructing an owned error message.
pub fn inspect(bytes:&[u8])->Result<BmpLayout,LayoutFailure>{
 if range(bytes,0,2,"signature")?!=BMP_MAGIC{return Err(LayoutFailure::Static("bmp: bad signature"));}
 let file_size=u32_(bytes,2,"file size")?;let reserved_1=u16_(bytes,6,"reserved 1")?;let reserved_2=u16_(bytes,8,"reserved 2")?;let data_offset=usize::try_from(u32_(bytes,10,"pixel offset")?).map_err(|_|LayoutFailure::Static("bmp: pixel offset exceeds address space"))?;
 let header_size=u32_(bytes,14,"DIB header size")?;if header_size!=BITMAPINFOHEADER_SIZE{return Err(LayoutFailure::Dib(header_size));}range(bytes,14,40,"BITMAPINFOHEADER")?;
 let width_field=u32_(bytes,18,"width")? as i32;let height_field=u32_(bytes,22,"height")? as i32;
 if width_field<0||height_field==i32::MIN{return Err(LayoutFailure::Static("bmp: width must be nonnegative and height must be representable"));}
 if(width_field==0)!=(height_field==0){return Err(LayoutFailure::Static("bmp: empty BMP dimensions must both be zero"));}
 let width=width_field as u32;let height=height_field.unsigned_abs();let row_order=if height_field<0{BmpRowOrder::TopDown}else{BmpRowOrder::BottomUp};let planes=u16_(bytes,26,"planes")?;let bits_per_pixel=u16_(bytes,28,"bits per pixel")?;let compression=u32_(bytes,30,"compression")?;
 if planes!=1{return Err(LayoutFailure::Planes(planes));}
 let profile=match(bits_per_pixel,compression){(1,BI_RGB)=>BmpProfile::IndexedRgb1,(4,BI_RGB)=>BmpProfile::IndexedRgb4,(8,BI_RGB)=>BmpProfile::IndexedRgb8,(16,BI_RGB)=>BmpProfile::DirectRgb16,(24,BI_RGB)=>BmpProfile::DirectRgb24,(32,BI_RGB)=>BmpProfile::DirectRgb32,(16,BI_BITFIELDS)=>BmpProfile::DirectBitfields16,(32,BI_BITFIELDS)=>BmpProfile::DirectBitfields32,(_,BI_RGB)=>return Err(LayoutFailure::RgbDepth(bits_per_pixel)),(_,BI_BITFIELDS)=>return Err(LayoutFailure::BitfieldDepth(bits_per_pixel)),(_,other)=>return Err(LayoutFailure::Compression(other))};
 let image_size=u32_(bytes,34,"image size")?;let x_pixels_per_meter=u32_(bytes,38,"horizontal resolution")? as i32;let y_pixels_per_meter=u32_(bytes,42,"vertical resolution")? as i32;let colors_used=u32_(bytes,46,"colors used")?;let colors_important=u32_(bytes,50,"important colors")?;
 let row_bits=u64::from(width).checked_mul(u64::from(bits_per_pixel)).ok_or(LayoutFailure::Static("bmp: row bit count overflow"))?;
 let row_stride=usize::try_from(row_bits.checked_add(31).ok_or(LayoutFailure::Static("bmp: row alignment overflow"))?/32*4).map_err(|_|LayoutFailure::Static("bmp: row stride exceeds address space"))?;
 let row_payload=usize::try_from(row_bits.checked_add(7).ok_or(LayoutFailure::Static("bmp: row payload overflow"))?/8).map_err(|_|LayoutFailure::Static("bmp: row payload exceeds address space"))?;
 let pixel_bytes=row_stride.checked_mul(height as usize).ok_or(LayoutFailure::Static("bmp: pixel storage length overflow"))?;let pixel_end=data_offset.checked_add(pixel_bytes).ok_or(LayoutFailure::Static("bmp: pixel range overflow"))?;
 if pixel_end>bytes.len(){return Err(LayoutFailure::PixelEnd(pixel_end,bytes.len()));}
 if file_size!=0&&(usize::try_from(file_size).map_err(|_|LayoutFailure::Static("bmp: declared file size exceeds address space"))?<pixel_end||file_size as usize>bytes.len()){return Err(LayoutFailure::FileSize(file_size));}
 if image_size!=0&&(image_size as usize)<pixel_bytes{return Err(LayoutFailure::ImageSize(image_size,pixel_bytes));}
 let mut metadata_end=54usize;let mut masks=match profile{BmpProfile::DirectRgb16=>[0x7c00,0x03e0,0x001f,0],BmpProfile::DirectRgb32=>[0xff0000,0xff00,0xff,0],_=>[0;4]};
 if compression==BI_BITFIELDS{masks[0]=u32_(bytes,metadata_end,"red mask")?;masks[1]=u32_(bytes,metadata_end+4,"green mask")?;masks[2]=u32_(bytes,metadata_end+8,"blue mask")?;
  let limit=if bits_per_pixel==32{u32::MAX}else{(1u32<<bits_per_pixel)-1};if masks[..3].iter().any(|mask|*mask==0||*mask&!limit!=0){return Err(LayoutFailure::Static("bmp: BI_BITFIELDS RGB masks must be nonzero and fit the sample width"));}
  for(index,mask)in masks[..3].iter().enumerate(){let shifted=*mask>>mask.trailing_zeros();if shifted&shifted.wrapping_add(1)!=0{return Err(LayoutFailure::MaskContiguous(index));}}
  if masks[0]&masks[1]!=0||masks[0]&masks[2]!=0||masks[1]&masks[2]!=0{return Err(LayoutFailure::Static("bmp: BI_BITFIELDS RGB masks overlap"));}metadata_end+=12;
 }
 let palette_offset=metadata_end;let palette_entries=if profile.is_indexed(){let capacity=1usize<<bits_per_pixel;let count=if colors_used==0{capacity}else{usize::try_from(colors_used).map_err(|_|LayoutFailure::Static("bmp: palette count exceeds address space"))?};if count>capacity{return Err(LayoutFailure::PaletteCount(count,bits_per_pixel,capacity));}count}else{0};
 metadata_end=metadata_end.checked_add(palette_entries.checked_mul(4).ok_or(LayoutFailure::Static("bmp: palette byte count overflow"))?).ok_or(LayoutFailure::Static("bmp: palette range overflow"))?;
 if data_offset<metadata_end{return Err(LayoutFailure::Offset(data_offset,metadata_end));}range(bytes,palette_offset,palette_entries*4,"palette")?;
 Ok(BmpLayout{profile,file_size,reserved_1,reserved_2,data_offset,width,height,row_order,planes,bits_per_pixel,compression,image_size,x_pixels_per_meter,y_pixels_per_meter,colors_used,colors_important,masks,palette_offset,palette_entries,metadata_end,row_stride,row_payload,pixel_bytes,pixel_end})
}
