//! 🧬️ Native sample admission and physical lowering of owned TIFF pages.
use super::*;
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, ValueError, ValueRefusalKind};

fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }
fn expand_ycbcr(raw:&[u8],width:usize,height:usize,h:usize,v:usize,c:&mut NativeDecodeControl<'_>)->Result<Vec<u8>,ValueError>{
 let length=width.checked_mul(height).and_then(|n|n.checked_mul(3)).ok_or_else(||invalid("tiff: expanded chroma extent"))?;let mut output=c.allocate_vec::<u8>(length)?;output.resize(length,0);let unit=h*v+2;let mut at=0;c.begin_stage(width*height)?;
 for y in (0..height).step_by(v){for x in (0..width).step_by(h){let values=raw.get(at..at+unit).ok_or_else(||invalid("tiff: incomplete chroma unit"))?;for dy in 0..v.min(height-y){for dx in 0..h.min(width-x){let target=((y+dy)*width+x+dx)*3;output[target]=values[dy*h+dx];output[target+1]=values[h*v];output[target+2]=values[h*v+1];c.step()?;}}at+=unit;}}if at!=raw.len(){return Err(invalid("tiff: excess chroma units"))}Ok(output)
}
fn physical(tag:u16)->bool { matches!(tag,259|266|273|278|279|284|317|322|323|324|325|292|293|347|512|513|514|515|517|518|519|520|521|530) }
fn bits(ifd:&NativeIfd,channels:usize)->Result<Vec<u32>,ValueError>{
 let mut values=tag_u32_list(ifd,TAG_BITS_PER_SAMPLE);if values.is_empty(){values.push(1)}if values.len()==1{values.resize(channels,values[0]);}if values.len()!=channels||values.iter().any(|bits|*bits==0||*bits>64){return Err(invalid("tiff: unsupported sample precision"))}Ok(values)
}
fn sample_at(bytes:&[u8],offset:usize,bits:u32,order:TiffByteOrder,fill:u32)->Result<u64,ValueError>{
 if bits>=8&&bits%8==0&&offset%8==0{let mut word=[0u8;8];let count=bits as usize/8;let native=bytes.get(offset/8..offset/8+count).ok_or_else(||invalid("tiff: truncated sample"))?;return Ok(match order{TiffByteOrder::LittleEndian=>{for(index,byte)in native.iter().enumerate(){word[index]=if fill==2{byte.reverse_bits()}else{*byte};}u64::from_le_bytes(word)},TiffByteOrder::BigEndian=>{for(index,byte)in native.iter().enumerate(){word[8-count+index]=if fill==2{byte.reverse_bits()}else{*byte};}u64::from_be_bytes(word)}})}
 let mut value=0u64;for bit in offset..offset+bits as usize{let byte=*bytes.get(bit/8).ok_or_else(||invalid("tiff: truncated packed sample"))?;let position=if fill==2{bit%8}else{7-bit%8};value=(value<<1)|u64::from((byte>>position)&1);}Ok(value)
}
fn store_sample(bytes:&mut[u8],offset:usize,bits:u32,word:u64,order:TiffByteOrder)->Result<(),String>{
 if bits>=8&&bits%8==0&&offset%8==0{let count=bits as usize/8;let native=match order{TiffByteOrder::LittleEndian=>word.to_le_bytes(),TiffByteOrder::BigEndian=>word.to_be_bytes()};bytes.get_mut(offset/8..offset/8+count).ok_or("tiff: native sample output extent")?.copy_from_slice(if order==TiffByteOrder::LittleEndian{&native[..count]}else{&native[8-count..]});return Ok(())}
 for lane in 0..bits as usize{let bit=offset+lane;let byte=bytes.get_mut(bit/8).ok_or("tiff: packed sample output extent")?;*byte|=(((word>>(bits as usize-1-lane))&1)as u8)<<(7-bit%8);}Ok(())
}
fn lzw_decode(input:&[u8],expected:usize,c:&mut NativeDecodeControl<'_>)->Result<Vec<u8>,ValueError>{
 c.charge(4096*(std::mem::size_of::<u16>()+2))?;
 let mut parents=[0u16;4096];let mut tails=[0u8;4096];let mut stack=[0u8;4096];for index in 0..256{tails[index]=index as u8;}
 let mut output=c.allocate_vec::<u8>(expected)?;let(mut cursor,mut width,mut next,mut previous)=(0usize,9usize,258usize,None::<usize>);
 c.begin_stage(expected)?;
 while cursor+width<=input.len()*8{
  let mut code=0usize;for bit in cursor..cursor+width{code=(code<<1)|usize::from((input[bit/8]>>(7-bit%8))&1);}cursor+=width;
  if code==256{width=9;next=258;previous=None;continue}if code==257{break}if code>next||code>=4096{return Err(invalid("tiff: LZW dictionary code"))}
  let mut node=if code==next{previous.ok_or_else(||invalid("tiff: LZW first code"))?}else{code};let mut count=0usize;
  while node>=256{if node>=next||count>=4095{return Err(invalid("tiff: LZW dictionary cycle"))}stack[count]=tails[node];count+=1;node=parents[node]as usize;}stack[count]=node as u8;count+=1;let first=stack[count-1];
  let extra=usize::from(code==next);if output.len().checked_add(count+extra).is_none_or(|size|size>expected){return Err(invalid("tiff: LZW decoded extent"))}
  for byte in stack[..count].iter().rev(){output.push(*byte);}if extra!=0{output.push(first)}c.advance(count+extra)?;
  if let Some(prior)=previous{if next<4096{parents[next]=prior as u16;tails[next]=first;next+=1;if next==(1<<width)-1&&width<12{width+=1;}}}
  previous=Some(code);
 }
 if output.len()!=expected{return Err(invalid("tiff: LZW decoded length mismatch"))}Ok(output)
}
fn lzw_encode_literal(input:&[u8])->Vec<u8>{
 let mut output=Vec::new();let mut partial=0u32;let mut bits=0usize;
 let push=|code:u16,output:&mut Vec<u8>,partial:&mut u32,bits:&mut usize|{*partial=(*partial<<9)|u32::from(code);*bits+=9;while *bits>=8{*bits-=8;output.push((*partial>>*bits)as u8);}*partial&=(1u32<<*bits)-1;};
 for chunk in input.chunks(200){push(256,&mut output,&mut partial,&mut bits);for byte in chunk{push(u16::from(*byte),&mut output,&mut partial,&mut bits);}}
 if input.is_empty(){push(256,&mut output,&mut partial,&mut bits)}push(257,&mut output,&mut partial,&mut bits);if bits!=0{output.push((partial<<(8-bits))as u8)}output
}
fn jpeg_samples(input:&[u8],tables:Option<&[u8]>,width:usize,height:usize,channels:usize,c:&mut NativeDecodeControl<'_>)->Result<Vec<u8>,ValueError>{
 use semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::decoded_components::JpgComponentDecoder;
 let mut merged=Vec::new();let source=if let Some(tables)=tables.filter(|value|!value.is_empty()){
  if !tables.starts_with(&[255,216])||!tables.ends_with(&[255,217]){return Err(invalid("tiff: JPEG table stream framing"))}
  let skip=usize::from(input.starts_with(&[255,216]))*2;let size=tables.len().checked_sub(2).and_then(|n|n.checked_add(input.len()-skip)).ok_or_else(||invalid("tiff: JPEG assembly extent"))?;merged=c.allocate_vec::<u8>(size)?;merged.extend_from_slice(&tables[..tables.len()-2]);merged.extend_from_slice(&input[skip..]);merged.as_slice()
 }else{input};
 let bound=width.checked_add(31).and_then(|w|height.checked_add(31).and_then(|h|w.checked_mul(h))).and_then(|n|n.checked_mul(channels)).and_then(|n|n.checked_mul(8)).and_then(|n|n.checked_add(source.len().checked_mul(8)?)).and_then(|n|n.checked_add(width.checked_mul(height)?.checked_mul(channels)?)).ok_or_else(||invalid("tiff: JPEG working extent"))?;
 c.charge(bound)?;let mut decoder={let mut cancel=||c.checkpoint().is_err();JpgComponentDecoder::new(&source,bound,&mut cancel)?};let(_,total)=decoder.progress();c.begin_stage(total)?;let mut previous=0;
 loop{let result={let mut cancel=||c.checkpoint().is_err();decoder.step(&source,256,&mut cancel)?};let(done,_)=decoder.progress();c.advance(done-previous)?;previous=done;if let Some(output)=result{if output.width as usize!=width||output.height as usize!=height||output.component_ids.len()!=channels{return Err(invalid("tiff: JPEG components differ from native page extent"))}return Ok(output.samples)}}
}
fn decompress(input:&[u8],compression:u32,expected:usize,width:usize,height:usize,fill:u32,options:u32,channels:usize,tables:Option<&[u8]>,c:&mut NativeDecodeControl<'_>)->Result<Vec<u8>,ValueError>{
 match compression{
  1=>{if input.len()<expected{return Err(invalid("tiff: native raster truncated"))}c.copy_bytes(&input[..expected])},
  32773=>{c.begin_stage(expected)?;c.charge(expected)?;let output=packbits_decode(input,expected).map_err(invalid)?;c.advance(expected)?;Ok(output)},
  2|3|4=>super::fax::decode(input,compression,width,height,fill,options,c),
  5=>lzw_decode(input,expected,c),
  6|7=>{let output=jpeg_samples(input,tables,width,height,channels,c)?;if output.len()!=expected{return Err(invalid("tiff: JPEG exact raster extent"))}Ok(output)},
  8|32946=>semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::binary::snapshot::decompress_zlib(input,expected,c),
  _=>Err(invalid(format!("tiff: native compression {compression} needs its physical decoder")))
 }
}
pub(super) fn admit(native:NativeSnapshot,c:&mut NativeDecodeControl<'_>,maximum_rows:usize)->Result<TiffSnapshot,ValueError>{
 let mut ifds=c.allocate_vec::<TiffIfd>(native.ifds.len())?;let mut rows=0usize;
 for directory in native.ifds{
  let mut blocks=c.allocate_vec::<TiffSampleBlock>(usize::from(!directory.storage.chunks.is_empty()))?;
  if !directory.storage.chunks.is_empty(){
   let width=tag_u32(&directory,TAG_IMAGE_WIDTH).ok_or_else(||invalid("tiff: raster width missing"))?;let height=tag_u32(&directory,TAG_IMAGE_LENGTH).ok_or_else(||invalid("tiff: raster height missing"))?;
   let channels=tag_u32(&directory,TAG_SAMPLES_PER_PIXEL).unwrap_or(1)as usize;if width==0||height==0||channels==0||channels>u16::MAX as usize{return Err(invalid("tiff: native image extent"))}
   let depths=bits(&directory,channels)?;let plane=tag_u32(&directory,284).unwrap_or(1);if !matches!(plane,1|2){return Err(invalid("tiff: native planar configuration"))}
   let fill=tag_u32(&directory,266).unwrap_or(1);if !matches!(fill,1|2){return Err(invalid("tiff: native fill order"))}
   let predictor=tag_u32(&directory,317).unwrap_or(1);if !matches!(predictor,1|2|3){return Err(invalid("tiff: native predictor needs physical decoder"))}
   let compression=tag_u32(&directory,TAG_COMPRESSION).unwrap_or(1);if matches!(compression,6|7)&&(plane!=1||depths.iter().any(|value|*value!=8)){return Err(invalid("tiff: JPEG native component interpretation"))}let subsampling=tag_u32_list(&directory,530);let sub_h=subsampling.first().copied().unwrap_or(2)as usize;let sub_v=subsampling.get(1).copied().unwrap_or(2)as usize;let chroma=tag_u32(&directory,TAG_PHOTOMETRIC)==Some(6)&&!matches!(compression,6|7);if chroma&&(plane!=1||channels!=3||depths.iter().any(|v|*v!=8)||predictor!=1||!matches!(sub_h,1|2|4)||!matches!(sub_v,1|2|4)){return Err(invalid("tiff: native YCbCr sampling"))}let tables=match tag_values(&directory,347){Some(TiffValues::Byte(value))|Some(TiffValues::Undefined(value))=>Some(value.as_slice()),_=>None};
   let(chunk_width,chunk_height)=match directory.storage.kind{NativeStorageKind::Tiles=>(tag_u32(&directory,TAG_TILE_WIDTH).ok_or_else(||invalid("tiff: tile width"))?,tag_u32(&directory,TAG_TILE_LENGTH).ok_or_else(||invalid("tiff: tile height"))?),NativeStorageKind::Strips=>(width,tag_u32(&directory,TAG_ROWS_PER_STRIP).unwrap_or(height).min(height)),_=>return Err(invalid("tiff: native storage kind"))};
   if chunk_width==0||chunk_height==0{return Err(invalid("tiff: native chunk extent"))}let across=width.div_ceil(chunk_width);let down=height.div_ceil(chunk_height);let per_plane=across.checked_mul(down).ok_or_else(||invalid("tiff: native chunk count"))?as usize;
   let count=(width as usize).checked_mul(height as usize).and_then(|n|n.checked_mul(channels)).ok_or_else(||invalid("tiff: exact sample count"))?;
   rows=rows.checked_add(count).ok_or_else(||invalid("tiff: sample rows overflow"))?;if rows>maximum_rows{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"TIFF precise sample row limit"))}
   if directory.storage.chunks.len()!=per_plane*(if plane==2{channels}else{1}){return Err(invalid("tiff: native chunk cardinality"))}
   let mut samples=c.allocate_vec::<TiffWord64>(count)?;samples.resize(count,TiffWord64::default());c.begin_stage(count)?;
   for(index,chunk)in directory.storage.chunks.iter().enumerate(){
    let lane=if plane==2{index/per_plane}else{0};let tile=index%per_plane;let x=tile as u32%across*chunk_width;let y=tile as u32/across*chunk_height;let visible_width=chunk_width.min(width-x);let visible_height=chunk_height.min(height-y);
    let stored_height=if directory.storage.kind==NativeStorageKind::Tiles{chunk_height}else{visible_height};let row_bits=if plane==2{chunk_width as usize*depths[lane]as usize}else{chunk_width as usize*depths.iter().map(|n|*n as usize).sum::<usize>()};let row_bytes=row_bits.div_ceil(8);let expected=row_bytes.checked_mul(stored_height as usize).ok_or_else(||invalid("tiff: decoded chunk extent"))?;
    let mut decoded=c.scoped_stage(|c|decompress(chunk,compression,if chroma{(chunk_width as usize).div_ceil(sub_h)*(stored_height as usize).div_ceil(sub_v)*(sub_h*sub_v+2)}else{expected},chunk_width as usize,stored_height as usize,fill,tag_u32(&directory,292).unwrap_or(0),if plane==2{1}else{channels},tables,c))?;
    if chroma{decoded=expand_ycbcr(&decoded,chunk_width as usize,stored_height as usize,sub_h,sub_v,c)?;}
    if predictor==3{let depth=depths[lane];if !matches!(depth,16|32|64)||depths.iter().any(|value|*value!=depth){return Err(invalid("tiff: floating predictor precision"))}let stride=if plane==2{1}else{channels};let count=chunk_width as usize*stride;let bytes=depth as usize/8;let mut scratch=c.allocate_vec::<u8>(row_bytes)?;scratch.resize(row_bytes,0);for row in decoded.chunks_mut(row_bytes){for at in stride..row.len(){row[at]=row[at].wrapping_add(row[at-stride]);}for sample in 0..count{for byte in 0..bytes{scratch[sample*bytes+byte]=row[byte*count+sample];}}row.copy_from_slice(&scratch);c.checkpoint()?;}}
    let lanes=if plane==2{lane..lane+1}else{0..channels};
    let mut positions=c.allocate_vec::<u64>(channels)?;positions.resize(channels,0);c.begin_stage(visible_width as usize*visible_height as usize*lanes.len())?;
    for row in 0..visible_height as usize{
     positions.fill(0);let mut bit_cursor=row*row_bytes*8;
     for column in 0..chunk_width as usize{
      for channel in lanes.clone(){
       let mut word=sample_at(&decoded,bit_cursor,depths[channel],if predictor==3{TiffByteOrder::BigEndian}else{native.byte_order},if matches!(compression,2|3|4){1}else{fill})?;bit_cursor+=depths[channel]as usize;
       if predictor==2&&column!=0{let previous=positions[channel];let mask=if depths[channel]==64{u64::MAX}else{(1u64<<depths[channel])-1};word=word.wrapping_add(previous)&mask;}positions[channel]=word;
       if column<visible_width as usize{let at=((y as usize+row)*width as usize+x as usize+column)*channels+channel;samples[at]=TiffWord64::from_word(word);c.step()?;}
      }
     }
    }
   }
   blocks.push(TiffSampleBlock{x:0,y:0,width,height,channels:channels as u16,samples});
  }
  let mut entries=c.allocate_vec::<TiffTag>(directory.entries.iter().filter(|entry|!physical(entry.tag)).count())?;c.begin_stage(0)?;for entry in directory.entries{if !physical(entry.tag){entries.push(entry);c.step()?;}}
  let ifd=TiffIfd{entries,blocks};ifd.validate().map_err(invalid)?;ifds.push(ifd);
 }
 let snapshot=TiffSnapshot{schema:native.schema,ifds};snapshot.validate().map_err(invalid)?;Ok(snapshot)
}
pub(super) fn lower(snapshot:&TiffSnapshot,options:TiffNativeOptions,c:&mut NativeEncodeControl<'_>)->Result<NativeSnapshot,ValueError>{
 snapshot.validate().map_err(invalid)?;let mut ifds=c.allocate_vec::<NativeIfd>(snapshot.ifds.len())?;
 for ifd in &snapshot.ifds{
  let mut entries=c.allocate_vec::<TiffTag>(ifd.entries.len()+4)?;c.begin_stage(ifd.entries.len())?;for tag in &ifd.entries{c.charge(super::values_owned_bytes(&tag.values)?)?;entries.push(tag.clone());c.step()?;}let mut storage=NativeStorage::default();
  if let Some(block)=ifd.blocks.first(){
   let channels=block.channels as usize;let source=ifd.integers(TAG_BITS_PER_SAMPLE);c.charge(source.len()*4)?;let mut depths=c.allocate_vec::<u32>(channels)?;depths.extend(source);if depths.is_empty(){depths.push(1)}if depths.len()==1{depths.resize(channels,depths[0]);}
   let(chunk_width,chunk_height,kind)=match options.layout{TiffNativeLayout::SingleStrip=>(block.width,block.height,NativeStorageKind::Strips),TiffNativeLayout::Strips{rows}=>(block.width,rows.min(block.height),NativeStorageKind::Strips),TiffNativeLayout::Tiles{width,height}=>(width,height,NativeStorageKind::Tiles)};
   if chunk_width==0||chunk_height==0{return Err(invalid("tiff: output chunk dimensions must be positive"))}
   let across=block.width.div_ceil(chunk_width);let down=block.height.div_ceil(chunk_height);let row_bytes=(chunk_width as usize*depths.iter().map(|n|*n as usize).sum::<usize>()).div_ceil(8);let mut chunks=c.allocate_vec::<Vec<u8>>(across.checked_mul(down).ok_or_else(||invalid("tiff: chunk count overflow"))?as usize)?;
   for tile_y in 0..down{for tile_x in 0..across{
    let visible_width=chunk_width.min(block.width-tile_x*chunk_width);let visible_height=chunk_height.min(block.height-tile_y*chunk_height);let stored_height=if kind==NativeStorageKind::Tiles{chunk_height}else{visible_height};let length=row_bytes.checked_mul(stored_height as usize).ok_or_else(||invalid("tiff: native raster size overflow"))?;let mut raw=c.allocate_vec::<u8>(length)?;raw.resize(length,0);c.begin_stage(visible_width as usize*visible_height as usize*channels)?;
    for y in 0..visible_height as usize{let mut bit=y*row_bytes*8;for x in 0..visible_width as usize{let at=((tile_y as usize*chunk_height as usize+y)*block.width as usize+tile_x as usize*chunk_width as usize+x)*channels;for channel in 0..channels{store_sample(&mut raw,bit,depths[channel],block.samples[at+channel].word(),options.byte_order).map_err(invalid)?;bit+=depths[channel]as usize;c.step()?;}}}
    chunks.push(match options.compression{TiffCompression::None=>raw,TiffCompression::ModifiedHuffman|TiffCompression::Group3|TiffCompression::Group4=>{if channels!=1||depths[0]!=1||!matches!(ifd.integer(TAG_PHOTOMETRIC),Some(0|1)){return Err(invalid("tiff: CCITT output requires bilevel grayscale samples"))}super::fax::encode(&raw,match options.compression{TiffCompression::ModifiedHuffman=>2,TiffCompression::Group3=>3,_=>4},chunk_width as usize,stored_height as usize,c)?},TiffCompression::PackBits=>{c.charge(raw.len().checked_mul(2).ok_or_else(||invalid("TIFF PackBits output extent"))?)?;packbits_encode(&raw)},TiffCompression::Lzw=>{c.charge(raw.len().checked_mul(2).and_then(|n|n.checked_add(8)).ok_or_else(||invalid("TIFF LZW output extent"))?)?;lzw_encode_literal(&raw)},TiffCompression::Deflate=>{c.charge(raw.len().checked_mul(2).and_then(|n|n.checked_add(64)).ok_or_else(||invalid("TIFF Deflate output extent"))?)?;semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(&raw).map_err(invalid)?}});
   }}
   c.charge(2+4*usize::from(kind==NativeStorageKind::Strips)+8*usize::from(kind==NativeStorageKind::Tiles))?;let compression=match options.compression{TiffCompression::None=>1,TiffCompression::PackBits=>32773,TiffCompression::Lzw=>5,TiffCompression::Deflate=>8,TiffCompression::ModifiedHuffman=>2,TiffCompression::Group3=>3,TiffCompression::Group4=>4};
   if ifd.integer(TAG_PHOTOMETRIC)==Some(6){c.charge(4)?;entries.push(TiffTag{tag:530,values:TiffValues::Short(vec![1,1])});}
   entries.push(TiffTag{tag:TAG_COMPRESSION,values:TiffValues::Short(vec![compression])});
   match kind{NativeStorageKind::Strips=>entries.push(TiffTag{tag:TAG_ROWS_PER_STRIP,values:TiffValues::Long(vec![chunk_height])}),NativeStorageKind::Tiles=>{entries.push(TiffTag{tag:TAG_TILE_WIDTH,values:TiffValues::Long(vec![chunk_width])});entries.push(TiffTag{tag:TAG_TILE_LENGTH,values:TiffValues::Long(vec![chunk_height])});},_=>unreachable!()}
   entries.sort_by_key(|tag|tag.tag);storage=NativeStorage{kind,offsets_kind:TiffFieldType::Long,byte_counts_kind:TiffFieldType::Long,chunks};
  }
  ifds.push(NativeIfd{entries,storage});
 }
 Ok(NativeSnapshot{schema:c.copy_text(&snapshot.schema)?,byte_order:options.byte_order,ifds})
}
