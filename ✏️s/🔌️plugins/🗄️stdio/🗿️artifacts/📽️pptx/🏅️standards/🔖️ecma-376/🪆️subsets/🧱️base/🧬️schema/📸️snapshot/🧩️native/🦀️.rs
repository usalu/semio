//! 🎞️ Literal PPTX OPC, XML parts and typed presentation fields under one native control.
use super::{PptxSnapshot,PptxXmlPart,PptxPresentation,PptxSlide,PptxShape,PptxTransform,PptxParagraph,PptxRun};
use semio_framework_os_kernel::{NativeEncodeControl,NativeDecodeControl,DecodedValue,sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,SqliteDatabaseLimits},io_schema::IoPayload};
use semio_s_artifact_stdio_zip::opc::native::{OpcNativeWriter,OpcNativeReader};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument,XmlNativeEmission,XmlNativeInput,emit_xml_native_node,read_xml_native_node,sqlite::{XmlDocumentView,retire_xml_document}};
struct Parts(Vec<PptxXmlPart>);
impl Drop for Parts{fn drop(&mut self){for part in self.0.drain(..){retire_xml_document(part.document);}}}
fn retire_shape(shape:PptxShape){if let PptxShape::Other{node}=shape{retire_xml_document(XmlDocument{root:Some(node),..XmlDocument::default()});}}
fn retire_slide(slide:PptxSlide){for shape in slide.shapes{retire_shape(shape);}}
struct Shapes(Vec<PptxShape>);
impl Drop for Shapes{fn drop(&mut self){for shape in self.0.drain(..){retire_shape(shape);}}}
struct Slides(Vec<PptxSlide>);
impl Drop for Slides{fn drop(&mut self){for slide in self.0.drain(..){retire_slide(slide);}}}
fn signed(value:i64,writer:&mut OpcNativeWriter<'_, '_, '_>)->Result<(),String>{if writer.encoding==SnapshotEncoding::Binary{return writer.unsigned(value as u64);}if value<0{writer.raw(b"-")?;}writer.unsigned(value.unsigned_abs())}
fn transform(value:&PptxTransform,writer:&mut OpcNativeWriter<'_, '_, '_>)->Result<(),String>{writer.delimiter(b"[")?;signed(value.x,writer)?;writer.delimiter(b",")?;signed(value.y,writer)?;writer.delimiter(b",")?;signed(value.cx,writer)?;writer.delimiter(b",")?;signed(value.cy,writer)?;writer.delimiter(b"]")}
fn paragraphs(values:&[PptxParagraph],writer:&mut OpcNativeWriter<'_, '_, '_>)->Result<(),String>{writer.list(values,|writer,paragraph|{writer.delimiter(b"[")?;writer.list(&paragraph.runs,|writer,run|{writer.delimiter(b"[")?;writer.string(&run.text)?;writer.delimiter(b",")?;writer.unsigned(u64::from(run.bold))?;writer.delimiter(b",")?;writer.unsigned(u64::from(run.italic))?;writer.delimiter(b",[")?;writer.unsigned(u64::from(run.font_size.is_some()))?;if let Some(size)=run.font_size{writer.delimiter(b",")?;writer.unsigned(u64::from(size))?;}writer.delimiter(b"]]")})?;writer.delimiter(b"]")})}
fn shape(value:&PptxShape,writer:&mut OpcNativeWriter<'_, '_, '_>)->Result<(),String>{
 writer.rows(1)?;writer.delimiter(b"[")?;match value{
 PptxShape::TextBox{text_frame,position}=>{writer.unsigned(0)?;writer.delimiter(b",")?;paragraphs(text_frame,writer)?;writer.delimiter(b",")?;transform(position,writer)?;}
 PptxShape::Picture{blip_rel_id,position}=>{writer.unsigned(1)?;writer.delimiter(b",")?;writer.string(blip_rel_id)?;writer.delimiter(b",")?;transform(position,writer)?;}
 PptxShape::Placeholder{kind,text_frame,position}=>{writer.unsigned(2)?;writer.delimiter(b",")?;writer.string(kind)?;writer.delimiter(b",")?;paragraphs(text_frame,writer)?;writer.delimiter(b",")?;transform(position,writer)?;}
 PptxShape::Other{node}=>{writer.rows(1)?;writer.unsigned(3)?;writer.delimiter(b",")?;emit_xml_native_node(node,XmlNativeEmission{control:&mut *writer.control,output:writer.output.as_deref_mut(),count:&mut writer.count,rows:&mut writer.rows,limits:writer.limits,encoding:writer.encoding})?;}
 }writer.delimiter(b"]")
}
fn write(snapshot:&PptxSnapshot,writer:&mut OpcNativeWriter<'_, '_, '_>)->Result<(),String>{
 writer.rows(1)?;if writer.encoding==SnapshotEncoding::Binary{let token=b"stdio.pptx.pack v1";writer.raw(b"\x89SEM\r\n\x1a\n")?;writer.raw(&(token.len()as u32).to_le_bytes())?;writer.raw(token)?;writer.raw(&[1])?;}else{writer.raw(b"semio stdio.pptx.dsl v1\n[")?;}
 writer.string(&snapshot.schema)?;writer.delimiter(b",")?;writer.package(&snapshot.opc)?;writer.delimiter(b",")?;
 writer.list(&snapshot.xml_parts,|writer,part|{writer.delimiter(b"[")?;writer.string(&part.path)?;writer.delimiter(b",")?;writer.string(&part.content_type)?;writer.delimiter(b",")?;writer.document(XmlDocumentView::from(&part.document))?;writer.delimiter(b"]")})?;writer.delimiter(b",")?;
 writer.rows(1)?;writer.delimiter(b"[")?;writer.list(&snapshot.presentation.slides,|writer,slide|{writer.delimiter(b"[")?;writer.list(&slide.shapes,|writer,value|shape(value,writer))?;writer.delimiter(b"]")})?;writer.delimiter(b"]]")
}
pub(super) fn encode(snapshot:&PptxSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<IoPayload,String>{
 let limits=control.limits();control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;let mut callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|control.checkpoint(SqliteSnapshotPhase::EncodeNative,event.completed,event.total).is_ok();let mut native=NativeEncodeControl::new(limits.max_value_bytes,&mut callback);native.begin_stage(0)?;
 let mut writer=OpcNativeWriter{control:&mut native,output:None,count:0,rows:0,limits,encoding};write(snapshot,&mut writer)?;let total=writer.count;native.begin_stage(total)?;let mut bytes=native.allocate_vec::<u8>(total)?;
 let mut writer=OpcNativeWriter{control:&mut native,output:Some(&mut bytes),count:0,rows:0,limits,encoding};write(snapshot,&mut writer)?;if writer.count!=total||bytes.len()!=total{return Err("PPTX measured native field size differs".into());}native.checkpoint()?;
 match encoding{SnapshotEncoding::Binary=>Ok(IoPayload::Binary(bytes)),SnapshotEncoding::Text=>Ok(IoPayload::Text(String::from_utf8(bytes).map_err(|_|"PPTX native output is not UTF-8")?))}
}
fn read_signed(reader:&mut OpcNativeReader<'_, '_, '_>)->Result<i64,String>{if reader.binary{return Ok(reader.unsigned()?as i64);}let negative=reader.bytes.get(reader.position)==Some(&b'-');if negative{reader.take(1)?;}let magnitude=reader.unsigned()?;if negative{if magnitude==0{return Err("PPTX signed coordinate is not canonical".into());}if magnitude==1u64<<63{Ok(i64::MIN)}else{Ok(-i64::try_from(magnitude).map_err(|_|"PPTX signed coordinate exceeds64bits")?)}}else{i64::try_from(magnitude).map_err(|_|"PPTX signed coordinate exceeds64bits".into())}}
fn read_transform(reader:&mut OpcNativeReader<'_, '_, '_>)->Result<PptxTransform,String>{reader.delimiter(b'[')?;let x=read_signed(reader)?;reader.delimiter(b',')?;let y=read_signed(reader)?;reader.delimiter(b',')?;let cx=read_signed(reader)?;reader.delimiter(b',')?;let cy=read_signed(reader)?;reader.delimiter(b']')?;Ok(PptxTransform{x,y,cx,cy})}
fn flag(reader:&mut OpcNativeReader<'_, '_, '_>)->Result<bool,String>{match reader.unsigned()?{0=>Ok(false),1=>Ok(true),_=>Err("PPTX native boolean differs".into())}}
fn read_paragraphs(reader:&mut OpcNativeReader<'_, '_, '_>)->Result<Vec<PptxParagraph>,String>{reader.list(|reader|{reader.delimiter(b'[')?;let runs=reader.list(|reader|{reader.delimiter(b'[')?;let text=reader.string()?;reader.delimiter(b',')?;let bold=flag(reader)?;reader.delimiter(b',')?;let italic=flag(reader)?;reader.delimiter(b',')?;reader.delimiter(b'[')?;let font_size=if flag(reader)?{reader.delimiter(b',')?;Some(u32::try_from(reader.unsigned()?).map_err(|_|"PPTX font size exceeds unsigned32")?)}else{None};reader.delimiter(b']')?;reader.delimiter(b']')?;Ok(PptxRun{text,bold,italic,font_size})})?;reader.delimiter(b']')?;Ok(PptxParagraph{runs})})}
fn read_shape(reader:&mut OpcNativeReader<'_, '_, '_>)->Result<PptxShape,String>{
 reader.rows(1)?;reader.delimiter(b'[')?;let tag=reader.unsigned()?;reader.delimiter(b',')?;let value=match tag{
 0=>{let text_frame=read_paragraphs(reader)?;reader.delimiter(b',')?;PptxShape::TextBox{text_frame,position:read_transform(reader)?}}
 1=>{let blip_rel_id=reader.string()?;reader.delimiter(b',')?;PptxShape::Picture{blip_rel_id,position:read_transform(reader)?}}
 2=>{let kind=reader.string()?;reader.delimiter(b',')?;let text_frame=read_paragraphs(reader)?;reader.delimiter(b',')?;PptxShape::Placeholder{kind,text_frame,position:read_transform(reader)?}}
 3=>{reader.rows(1)?;PptxShape::Other{node:read_xml_native_node(XmlNativeInput{bytes:reader.bytes,position:&mut reader.position,rows:&mut reader.rows,limits:reader.limits,binary:reader.binary,control:&mut *reader.control})?}}
 _=>return Err("PPTX native shape discriminant differs".into())};let value=DecodedValue::new(value,retire_shape);reader.delimiter(b']')?;Ok(value.take())
}
fn length(reader:&mut OpcNativeReader<'_, '_, '_>)->Result<usize,String>{reader.delimiter(b'[')?;let count=if reader.binary{usize::try_from(reader.unsigned()?).map_err(|_|"PPTX collection exceeds platform width")?}else{0};if reader.binary{reader.rows(count)?;}Ok(count)}
fn more(reader:&OpcNativeReader<'_, '_, '_>,position:usize,count:usize)->bool{if reader.binary{position<count}else{reader.bytes.get(reader.position)!=Some(&b']')}}
fn read_presentation(reader:&mut OpcNativeReader<'_, '_, '_>)->Result<Slides,String>{
 reader.rows(1)?;reader.delimiter(b'[')?;let count=length(reader)?;let mut slides=Slides(reader.control.allocate_vec::<PptxSlide>(count)?);
 while more(reader,slides.0.len(),count){if !reader.binary{reader.rows(1)?;reader.reserve(&mut slides.0)?;}if !slides.0.is_empty(){reader.delimiter(b',')?;}reader.delimiter(b'[')?;let count=length(reader)?;let mut shapes=Shapes(reader.control.allocate_vec::<PptxShape>(count)?);
  while more(reader,shapes.0.len(),count){if !reader.binary{reader.rows(1)?;reader.reserve(&mut shapes.0)?;}if !shapes.0.is_empty(){reader.delimiter(b',')?;}shapes.0.push(read_shape(reader)?);}
  reader.delimiter(b']')?;reader.delimiter(b']')?;slides.0.push(PptxSlide{shapes:std::mem::take(&mut shapes.0)});
 }reader.delimiter(b']')?;reader.delimiter(b']')?;Ok(slides)
}
fn read(reader:&mut OpcNativeReader<'_, '_, '_>)->Result<PptxSnapshot,String>{
 reader.rows(1)?;if reader.binary{if reader.take(1)?!=[1]{return Err("PPTX native revision differs".into());}}else{reader.delimiter(b'[')?;}
 let schema=reader.string()?;reader.delimiter(b',')?;let opc=reader.package()?;reader.delimiter(b',')?;let count=length(reader)?;let mut parts=Parts(reader.control.allocate_vec::<PptxXmlPart>(count)?);
 while more(reader,parts.0.len(),count){if !reader.binary{reader.rows(1)?;reader.reserve(&mut parts.0)?;}if !parts.0.is_empty(){reader.delimiter(b',')?;}reader.delimiter(b'[')?;let path=reader.string()?;reader.delimiter(b',')?;let content_type=reader.string()?;reader.delimiter(b',')?;let document=DecodedValue::new(reader.document()?,retire_xml_document);reader.delimiter(b']')?;parts.0.push(PptxXmlPart{path,content_type,document:document.take()});}
 reader.delimiter(b']')?;reader.delimiter(b',')?;let mut slides=read_presentation(reader)?;reader.delimiter(b']')?;if reader.position!=reader.bytes.len(){return Err("PPTX native input has trailing fields".into());}reader.control.checkpoint()?;Ok(PptxSnapshot{schema,opc,xml_parts:std::mem::take(&mut parts.0),presentation:PptxPresentation{slides:std::mem::take(&mut slides.0)}})
}
fn input(bytes:&[u8],binary:bool,control:&mut SqliteSnapshotControl<'_>)->Result<PptxSnapshot,String>{
 let limits=control.limits();if bytes.len()>limits.max_file_bytes{return Err("PPTX native input exceeds caller file ceiling".into());}control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,bytes.len())?;let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|control.checkpoint(SqliteSnapshotPhase::DecodeNative,event.completed,event.total).is_ok();let mut native=NativeDecodeControl::new(limits.max_value_bytes,&mut callback);
 let body=if binary{store::semio_format::unwrap_binary_controlled(bytes,"stdio.pptx",store::semio_format::Component::Pack,1,&mut native).map_err(|error|error.to_string())?}else{let text=native.borrow_text(bytes)?;store::semio_format::split_text_preamble_controlled(text,"stdio.pptx",store::semio_format::Component::Dsl,1,&mut native).map_err(|error|error.to_string())?.as_bytes()};native.begin_stage(body.len())?;read(&mut OpcNativeReader{bytes:body,position:0,rows:0,limits,binary,control:&mut native})
}
pub(super) fn decode(payload:&IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<PptxSnapshot,String>{match payload{IoPayload::Binary(bytes)=>input(bytes,true,control),IoPayload::Text(text)=>input(text.as_bytes(),false,control)}}
pub(super) fn decode_text(text:&str,control:&mut SqliteSnapshotControl<'_>)->Result<PptxSnapshot,String>{input(text.as_bytes(),false,control)}
pub(super) fn decode_binary(bytes:&[u8],control:&mut SqliteSnapshotControl<'_>)->Result<PptxSnapshot,String>{input(bytes,true,control)}
pub(super) fn pack_limits(limits:&store::mounted_pack_rt::PackLimits)->SqliteDatabaseLimits{SqliteDatabaseLimits{max_file_bytes:usize::try_from(limits.max_file_len).unwrap_or(usize::MAX),max_value_bytes:usize::try_from(limits.max_total_alloc).unwrap_or(usize::MAX),max_rows:usize::try_from(limits.max_items).unwrap_or(usize::MAX),..SqliteDatabaseLimits::default()}}
