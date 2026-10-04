//! 🛬️ Controlled paid reader for the existing external TIFF6 carrier boundary.
use super::*;
use semio_framework_value::{ValueError,ValueRefusalKind,native_decoding::NativeDecodeControl};

enum Refusal{Native(ValueError),Wire(String)}
impl From<ValueError> for Refusal{fn from(error:ValueError)->Self{Self::Native(error)}}
impl From<String> for Refusal{fn from(error:String)->Self{Self::Wire(error)}}
impl Refusal{fn into_value_error(self)->ValueError{match self{Self::Native(error)=>error,Self::Wire(message)=>ValueError::new(ValueRefusalKind::InvalidValue,message)}}}
type ReadResult<T>=Result<T,Refusal>;
fn invalid(message:&str)->Refusal{Refusal::Wire(message.into())}
fn window(data:&[u8],offset:usize,count:usize)->ReadResult<&[u8]>{data.get(offset..offset.checked_add(count).ok_or_else(||invalid("tiff: byte range overflow"))?).ok_or_else(||invalid("tiff: truncated byte range"))}
fn next(data:&[u8],offset:usize,e:Endian)->ReadResult<usize>{if offset==0{return Ok(0)}let count=usize::from(read_u16(data,offset,e)?);let length=count.checked_mul(12).and_then(|v|v.checked_add(2)).ok_or_else(||invalid("tiff: IFD size overflow"))?;window(data,offset,length.checked_add(4).ok_or_else(||invalid("tiff: IFD size overflow"))?)?;Ok(usize::try_from(read_u32(data,offset+length,e)?).map_err(|_|invalid("tiff: IFD offset width"))?)}
fn strip_component<'a>(data:&'a[u8],offset:usize,e:Endian,tag:u16,c:&mut NativeDecodeControl<'_>)->ReadResult<Option<(&'a[u8],usize,TiffFieldType)>>{
    let entries=usize::from(read_u16(data,offset,e)?);for index in 0..entries{c.step()?;let position=offset+2+index*12;if read_u16(data,position,e)?==tag{let kind=TiffFieldType::from_u16(read_u16(data,position+2,e)?)?;if !matches!(kind,TiffFieldType::Short|TiffFieldType::Long){return Ok(None)}let count=usize::try_from(read_u32(data,position+4,e)?).map_err(|_|invalid("tiff: strip count width"))?;let length=count.checked_mul(kind.element_size()).ok_or_else(||invalid("tiff: strip value size overflow"))?;let start=if length<=4{position+8}else{usize::try_from(read_u32(data,position+8,e)?).map_err(|_|invalid("tiff: strip offset width"))?};return Ok(Some((window(data,start,length)?,count,kind)))} }Ok(None)
}
fn has_secondary_pixels(data:&[u8],offset:usize,e:Endian,c:&mut NativeDecodeControl<'_>)->ReadResult<bool>{
    let Some((_,offsets,_))=strip_component(data,offset,e,TAG_STRIP_OFFSETS,c)?else{return Ok(false)};let Some((bytes,counts,kind))=strip_component(data,offset,e,TAG_STRIP_BYTE_COUNTS,c)?else{return Ok(false)};if offsets==0||counts!=offsets{return Ok(false)}for index in 0..counts{c.step()?;let value=if kind==TiffFieldType::Short{u32::from(e.u16(&bytes[index*2..index*2+2]))}else{e.u32(&bytes[index*4..index*4+4])};if value!=0{return Ok(true)}}Ok(false)
}
fn census(data:&[u8],first:usize,e:Endian,c:&mut NativeDecodeControl<'_>,maximum_rows:usize)->ReadResult<usize>{
    c.begin_stage(0)?;let(mut offset,mut slow,mut fast,mut count)=(first,first,first,0usize);
    let mut rows=2usize;
    while offset!=0{next(data,offset,e)?;let entries=usize::from(read_u16(data,offset,e)?);let normalized=count!=0&&has_secondary_pixels(data,offset,e,c)?;rows=rows.checked_add(2).ok_or_else(||invalid("tiff: row count overflow"))?;for index in 0..entries{let position=offset+2+index*12;let tag=read_u16(data,position,e)?;let kind=TiffFieldType::from_u16(read_u16(data,position+2,e)?)?;let values=if kind==TiffFieldType::Ascii{1}else{usize::try_from(read_u32(data,position+4,e)?).map_err(|_|invalid("tiff: value count width"))?};if !normalized||!matches!(tag,TAG_STRIP_OFFSETS|TAG_STRIP_BYTE_COUNTS){rows=rows.checked_add(1).and_then(|n|n.checked_add(values)).filter(|n|*n<=maximum_rows).ok_or_else(||Refusal::Native(ValueError::new(ValueRefusalKind::OwnershipLimit,"TIFF semantic row limit")))?;}c.step()?;}if rows>maximum_rows{return Err(Refusal::Native(ValueError::new(ValueRefusalKind::OwnershipLimit,"TIFF semantic row limit")))}offset=next(data,offset,e)?;count=count.checked_add(1).ok_or_else(||invalid("tiff: IFD count overflow"))?;slow=next(data,slow,e)?;fast=next(data,next(data,fast,e)?,e)?;if slow!=0&&slow==fast{return Err(invalid("tiff: IFD offset cycle detected"))}c.step()?;}Ok(count)
}
fn text_piece(bytes:&[u8],position:usize)->(&str,usize){
    let first=bytes[position];let width=match first{0..=127=>1,194..=223=>2,224..=239=>3,240..=244=>4,_=>1};let end=position.saturating_add(width).min(bytes.len());
    match std::str::from_utf8(&bytes[position..end]){Ok(text)=>(text,end-position),Err(error)=>("\u{fffd}",error.error_len().unwrap_or(end-position))}
}
fn text(bytes:&[u8],c:&mut NativeDecodeControl<'_>)->ReadResult<String>{
    let mut end=bytes.len();c.begin_stage(bytes.len())?;while end!=0&&bytes[end-1]==0{end-=1;c.step()?}let bytes=&bytes[..end];c.begin_stage(bytes.len())?;let(mut position,mut size)=(0usize,0usize);while position<bytes.len(){let(piece,count)=text_piece(bytes,position);size=size.checked_add(piece.len()).ok_or_else(||invalid("tiff: ASCII size overflow"))?;position+=count;c.advance(count)?;}
    let mut output=c.allocate_vec::<u8>(size)?;c.begin_stage(bytes.len())?;position=0;while position<bytes.len(){let(piece,count)=text_piece(bytes,position);output.extend_from_slice(piece.as_bytes());position+=count;c.advance(count)?;}String::from_utf8(output).map_err(|_|invalid("tiff: decoded ASCII invariant"))
}
fn sequence<T>(count:usize,c:&mut NativeDecodeControl<'_>,read:impl Fn(usize)->T)->ReadResult<Vec<T>>{let mut result=c.allocate_vec::<T>(count)?;c.begin_stage(count)?;for index in 0..count{result.push(read(index));c.step()?;}Ok(result)}
fn values(src:&[u8],kind:TiffFieldType,count:usize,e:Endian,c:&mut NativeDecodeControl<'_>)->ReadResult<TiffValues>{
    Ok(match kind{
        TiffFieldType::Byte=>TiffValues::Byte(c.copy_bytes(src)?),TiffFieldType::Undefined=>TiffValues::Undefined(c.copy_bytes(src)?),TiffFieldType::Ascii=>TiffValues::Ascii(c.copy_bytes(src)?),
        TiffFieldType::Short=>TiffValues::Short(sequence(count,c,|i|e.u16(&src[i*2..i*2+2]))?),TiffFieldType::Long=>TiffValues::Long(sequence(count,c,|i|e.u32(&src[i*4..i*4+4]))?),
        TiffFieldType::Rational=>TiffValues::Rational(sequence(count,c,|i|(e.u32(&src[i*8..i*8+4]),e.u32(&src[i*8+4..i*8+8])))?),
        TiffFieldType::SByte=>TiffValues::SByte(sequence(count,c,|i|i8::from_ne_bytes([src[i]]))?),TiffFieldType::SShort=>TiffValues::SShort(sequence(count,c,|i|i16::from_ne_bytes(e.u16(&src[i*2..i*2+2]).to_ne_bytes()))?),TiffFieldType::SLong=>TiffValues::SLong(sequence(count,c,|i|i32::from_ne_bytes(e.u32(&src[i*4..i*4+4]).to_ne_bytes()))?),
        TiffFieldType::SRational=>TiffValues::SRational(sequence(count,c,|i|(i32::from_ne_bytes(e.u32(&src[i*8..i*8+4]).to_ne_bytes()),i32::from_ne_bytes(e.u32(&src[i*8+4..i*8+8]).to_ne_bytes())))?),
        TiffFieldType::Float=>TiffValues::Float(sequence(count,c,|i|TiffBinary32{bits:e.u32(&src[i*4..i*4+4])})?),TiffFieldType::Double=>TiffValues::Double(sequence(count,c,|i|TiffBinary64{bits:e.u64(&src[i*8..i*8+8])})?),
    })
}
fn directories(data:&[u8],first:usize,e:Endian,c:&mut NativeDecodeControl<'_>,maximum_rows:usize)->ReadResult<Vec<TiffIfd>>{
    let count=census(data,first,e,c,maximum_rows)?;if count==0{return Err(invalid("tiff: no IFD present"))}let mut output=c.allocate_vec::<TiffIfd>(count)?;let mut offset=first;
    while offset!=0{let count=usize::from(read_u16(data,offset,e)?);let mut entries=c.allocate_vec::<TiffTag>(count)?;
        for index in 0..count{c.begin_stage(0)?;let position=offset+2+index*12;let tag=read_u16(data,position,e)?;let kind=TiffFieldType::from_u16(read_u16(data,position+2,e)?)?;let count=usize::try_from(read_u32(data,position+4,e)?).map_err(|_|invalid("tiff: value count width"))?;let size=count.checked_mul(kind.element_size()).ok_or_else(||invalid("tiff: tag value size overflow"))?;let start=if size<=4{position+8}else{usize::try_from(read_u32(data,position+8,e)?).map_err(|_|invalid("tiff: tag offset width"))?};let src=window(data,start,size)?;entries.push(TiffTag{tag,values:values(src,kind,count,e,c)?});}
        super::controlled_ordering::order(&mut entries,|entry|entry.tag,c)?;
        output.push(TiffIfd{entries,storage:TiffStorage::default()});offset=next(data,offset,e)?;
    }Ok(output)
}
fn numbers(ifd:&TiffIfd,tag:u16)->Option<&TiffValues>{match tag_values(ifd,tag){Some(value@TiffValues::Short(_))|Some(value@TiffValues::Long(_))=>Some(value),_=>None}}
fn number_count(value:Option<&TiffValues>)->usize{match value{Some(TiffValues::Short(v))=>v.len(),Some(TiffValues::Long(v))=>v.len(),_=>0}}
fn number(value:Option<&TiffValues>,index:usize)->Option<u32>{match value{Some(TiffValues::Short(v))=>v.get(index).map(|v|u32::from(*v)),Some(TiffValues::Long(v))=>v.get(index).copied(),_=>None}}
fn first(ifd:&TiffIfd,tag:u16)->Option<u32>{number(numbers(ifd,tag),0)}
fn raw_storage(data:&[u8],ifd:&TiffIfd,c:&mut NativeDecodeControl<'_>)->ReadResult<TiffStorage>{
    let strips=numbers(ifd,TAG_STRIP_OFFSETS);let tiles=numbers(ifd,TAG_TILE_OFFSETS);if number_count(strips)!=0&&number_count(tiles)!=0{return Err(invalid("tiff: one IFD cannot own both strips and tiles"))}
    let(kind,offset_tag,count_tag,offsets)=if number_count(tiles)!=0{(TiffStorageKind::Tiles,TAG_TILE_OFFSETS,TAG_TILE_BYTE_COUNTS,tiles)}else if number_count(strips)!=0{(TiffStorageKind::Strips,TAG_STRIP_OFFSETS,TAG_STRIP_BYTE_COUNTS,strips)}else{return Ok(TiffStorage::default())};
    let counts=numbers(ifd,count_tag);let count=number_count(offsets);if number_count(counts)!=count{return Err(invalid("tiff: storage offset/count cardinality mismatch"))}
    let offsets_kind=tag_values(ifd,offset_tag).ok_or_else(||invalid("tiff: missing storage offsets"))?.kind();let byte_counts_kind=tag_values(ifd,count_tag).ok_or_else(||invalid("tiff: missing storage byte counts"))?.kind();if !matches!(offsets_kind,TiffFieldType::Short|TiffFieldType::Long)||!matches!(byte_counts_kind,TiffFieldType::Short|TiffFieldType::Long){return Err(invalid("tiff: storage words must be SHORT or LONG"))}
    let mut chunks=c.allocate_vec::<Vec<u8>>(count)?;for index in 0..count{let start=usize::try_from(number(offsets,index).unwrap()).map_err(|_|invalid("tiff: storage offset width"))?;let length=usize::try_from(number(counts,index).unwrap()).map_err(|_|invalid("tiff: storage byte-count width"))?;chunks.push(c.copy_bytes(window(data,start,length)?)?);c.step()?;}Ok(TiffStorage{kind,offsets_kind,byte_counts_kind,chunks})
}
fn zeroes(count:usize,c:&mut NativeDecodeControl<'_>)->ReadResult<Vec<u8>>{let mut output=c.allocate_vec::<u8>(count)?;c.begin_stage(count)?;while output.len()<count{let length=(count-output.len()).min(65536);output.resize(output.len()+length,0);c.advance(length)?;}Ok(output)}
fn packed_strip(data:&[u8],output:&mut[u8],c:&mut NativeDecodeControl<'_>)->ReadResult<()>{
    c.begin_stage(output.len())?;let(mut input,mut position)=(0usize,0usize);while input<data.len()&&position<output.len(){let n=i8::from_ne_bytes([data[input]]);input+=1;if n>=0{let count=usize::from(n.unsigned_abs())+1;let target=output.get_mut(position..position.checked_add(count).ok_or_else(||invalid("tiff: PackBits length overflow"))?).ok_or_else(||invalid("tiff: PackBits output overrun"))?;target.copy_from_slice(window(data,input,count)?);input+=count;position+=count;c.advance(count)?;}else if n!=i8::MIN{let count=usize::from(n.unsigned_abs())+1;let byte=*data.get(input).ok_or_else(||invalid("tiff: PackBits repeat missing byte"))?;input+=1;let target=output.get_mut(position..position.checked_add(count).ok_or_else(||invalid("tiff: PackBits length overflow"))?).ok_or_else(||invalid("tiff: PackBits output overrun"))?;target.fill(byte);position+=count;c.advance(count)?;}else{c.checkpoint()?;}}
    if position!=output.len(){return Err(invalid("tiff: PackBits decoded length mismatch"))}Ok(())
}
fn pixels(data:&[u8],ifd:&TiffIfd,c:&mut NativeDecodeControl<'_>)->ReadResult<Vec<u8>>{
    let width=usize::try_from(first(ifd,TAG_IMAGE_WIDTH).ok_or_else(||invalid("tiff: missing ImageWidth"))?).map_err(|_|invalid("tiff: image width"))?;let height=usize::try_from(first(ifd,TAG_IMAGE_LENGTH).ok_or_else(||invalid("tiff: missing ImageLength"))?).map_err(|_|invalid("tiff: image height"))?;if width==0||height==0{return Err(invalid("tiff: zero dimension"))}
    if first(ifd,TAG_BITS_PER_SAMPLE).unwrap_or(8)!=8{return Err(invalid("tiff: unsupported BitsPerSample"))}let samples=usize::try_from(first(ifd,TAG_SAMPLES_PER_PIXEL).unwrap_or(1)).map_err(|_|invalid("tiff: sample width"))?;if !matches!(samples,1|3|4){return Err(invalid("tiff: unsupported SamplesPerPixel"))}let compression=first(ifd,TAG_COMPRESSION).unwrap_or(1);if !matches!(compression,1|32773){return Err(invalid("tiff: unsupported compression"))}
    let offsets=numbers(ifd,TAG_STRIP_OFFSETS);if number_count(offsets)==0{return Err(invalid("tiff: missing StripOffsets"))}let counts=numbers(ifd,TAG_STRIP_BYTE_COUNTS);let rows_per_strip=usize::try_from(first(ifd,TAG_ROWS_PER_STRIP).unwrap_or(u32::try_from(height).map_err(|_|invalid("tiff: height width"))?)).map_err(|_|invalid("tiff: strip rows width"))?;
    let pixels=width.checked_mul(height).ok_or_else(||invalid("tiff: pixel count overflow"))?;let row_bytes=width.checked_mul(samples).ok_or_else(||invalid("tiff: row length overflow"))?;let mut raster=zeroes(pixels.checked_mul(samples).ok_or_else(||invalid("tiff: raster length overflow"))?,c)?;let mut row=0;
    for index in 0..number_count(offsets){if row>=height{break}let rows=rows_per_strip.min(height-row);let length=rows.checked_mul(row_bytes).ok_or_else(||invalid("tiff: strip length overflow"))?;let start=usize::try_from(number(offsets,index).unwrap()).map_err(|_|invalid("tiff: strip offset width"))?;let target=&mut raster[row*row_bytes..row*row_bytes+length];if compression==32773{let count=usize::try_from(number(counts,index).ok_or_else(||invalid("tiff: missing StripByteCounts"))?).map_err(|_|invalid("tiff: strip length width"))?;packed_strip(window(data,start,count)?,target,c)?;}else{c.begin_stage(length)?;for(chunk,target)in window(data,start,length)?.chunks(65536).zip(target.chunks_mut(65536)){target.copy_from_slice(chunk);c.advance(chunk.len())?;}}row+=rows;}
    let photometric=first(ifd,TAG_PHOTOMETRIC).unwrap_or(1);let mut rgba=c.allocate_vec::<u8>(pixels.checked_mul(4).ok_or_else(||invalid("tiff: RGBA length overflow"))?)?;c.begin_stage(pixels)?;for index in 0..pixels{let src=&raster[index*samples..index*samples+samples];match samples{1=>{let gray=if photometric==0{255-src[0]}else{src[0]};rgba.extend_from_slice(&[gray,gray,gray,255]);},3=>rgba.extend_from_slice(&[src[0],src[1],src[2],255]),4=>rgba.extend_from_slice(src),_=>unreachable!()}c.step()?;}Ok(rgba)
}
fn read(data:&[u8],control:&mut NativeDecodeControl<'_>,maximum_rows:usize)->ReadResult<TiffSnapshot>{
    control.checkpoint()?;control.charge(std::mem::size_of::<TiffSnapshot>())?;window(data,0,8)?;let(e,byte_order)=match &data[..2]{b"II"=>(Endian::Little,TiffByteOrder::LittleEndian),b"MM"=>(Endian::Big,TiffByteOrder::BigEndian),_=>return Err(invalid("tiff: bad byte order"))};if read_u16(data,2,e)?!=42{return Err(invalid("tiff: bad magic"))}
    let mut ifds=directories(data,usize::try_from(read_u32(data,4,e)?).map_err(|_|invalid("tiff: first IFD width"))?,e,control,maximum_rows)?;for ifd in &mut ifds{ifd.storage=raw_storage(data,ifd,control)?;ifd.entries.retain(|entry|!matches!(entry.tag,TAG_STRIP_OFFSETS|TAG_STRIP_BYTE_COUNTS|TAG_TILE_OFFSETS|TAG_TILE_BYTE_COUNTS));control.step()?;}
    let schema=control.copy_text(STDIO_TIFF_DOCUMENT_SCHEMA)?;Ok(TiffSnapshot{schema,byte_order,ifds})
}

/// 📖️ Decodes only the actual TIFF carrier, admitting each field and work frontier.
pub fn decode_tiff_controlled(data:&[u8],control:&mut NativeDecodeControl<'_>,maximum_rows:usize)->Result<TiffSnapshot,ValueError>{read(data,control,maximum_rows).map_err(Refusal::into_value_error)}
