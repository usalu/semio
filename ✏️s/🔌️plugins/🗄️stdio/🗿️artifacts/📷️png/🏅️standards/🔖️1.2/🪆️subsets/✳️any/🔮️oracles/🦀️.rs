//! 🔮️ Independent PNG oracle with exact owned samples and metadata.
use semio_repo_test_host::Json;
#[cfg(feature="oracles")]
#[path="🌱️owned-document/🦀️.rs"]
mod owned;
#[cfg(feature="oracles")]
pub struct PngSourceOracleProjection {pub width:u32,pub height:u32,pub bit_depth:u8,pub color_type:u8,pub interlaced:bool,pub rgba:Vec<u8>}
/// 🔬️ Reads the original profile and an independent display projection.
#[cfg(feature="oracles")]
pub fn project_png_source(input:&[u8])->Result<PngSourceOracleProjection,String>{
 let metadata=png::Decoder::new(std::io::Cursor::new(input)).read_info().map_err(|error|error.to_string())?;
 let info=metadata.info();let(width,height,bit_depth,color_type,interlaced)=(info.width,info.height,info.bit_depth as u8,info.color_type as u8,info.interlaced);
 let mut decoder=png::Decoder::new(std::io::Cursor::new(input));decoder.set_transformations(png::Transformations::ALPHA|png::Transformations::STRIP_16);
 let mut reader=decoder.read_info().map_err(|error|error.to_string())?;let mut buffer=vec![0;reader.output_buffer_size().ok_or("PNG oracle display extent")?];let frame=reader.next_frame(&mut buffer).map_err(|error|error.to_string())?;
 let bytes=&buffer[..frame.buffer_size()];let rgba=match frame.color_type{png::ColorType::Rgba=>bytes.to_vec(),png::ColorType::Rgb=>bytes.chunks_exact(3).flat_map(|value|[value[0],value[1],value[2],255]).collect(),png::ColorType::Grayscale=>bytes.iter().flat_map(|value|[*value,*value,*value,255]).collect(),png::ColorType::GrayscaleAlpha=>bytes.chunks_exact(2).flat_map(|value|[value[0],value[0],value[0],value[1]]).collect(),_=>return Err("PNG independent display transform retained indexed samples".into())};
 Ok(PngSourceOracleProjection{width,height,bit_depth,color_type,interlaced,rgba})
}
/// 🌱️ Admits the exact independent owned document.
#[cfg(feature="oracles")]
pub fn project_png_owned(input:&[u8])->Result<Json,String>{owned::reading(input)}
/// 🏭️ Materializes an exact owned image through the independent test writer.
#[cfg(feature="oracles")]
pub fn oracle_encode_owned(snapshot:&Json)->Result<Vec<u8>,String>{owned::encode(snapshot)}
/// 🔑️ Computes the structural revision from independent owned fields.
#[cfg(feature="oracles")]
pub fn owned_png_revision(snapshot:&Json)->Result<String,String>{owned::revision(snapshot)}
#[cfg(feature="oracles")]
pub fn oracle_apply_mutation(input:&[u8],spec:&Json)->Result<Vec<u8>,String>{owned::apply(input,spec)}
#[cfg(feature="oracles")]
pub fn oracle_undo_mutation(original:&[u8],_spec:&Json,mutated:&[u8])->Result<Vec<u8>,String>{owned::reading(mutated)?;owned::encode(&owned::reading(original)?)}
/// 👁️ Compares the native profile, all metadata, and exact sample digest.
#[cfg(feature="oracles")]
pub fn project_png_mutation(bytes:&[u8])->Result<Json,String>{owned::project(bytes)}
#[cfg(feature="oracles")]
pub fn oracle_identity_round_trip(input:&[u8])->Result<Vec<u8>,String>{owned::encode(&owned::reading(input)?)}
#[cfg(not(feature="oracles"))]
pub fn oracle_apply_mutation(_input:&[u8],_spec:&Json)->Result<Vec<u8>,String>{Err("the oracles feature is disabled".into())}
#[cfg(not(feature="oracles"))]
pub fn oracle_undo_mutation(_original:&[u8],_spec:&Json,_mutated:&[u8])->Result<Vec<u8>,String>{Err("the oracles feature is disabled".into())}
#[cfg(not(feature="oracles"))]
pub fn project_png_mutation(_bytes:&[u8])->Result<Json,String>{Err("the oracles feature is disabled".into())}
#[cfg(not(feature="oracles"))]
pub fn oracle_identity_round_trip(_input:&[u8])->Result<Vec<u8>,String>{Err("the oracles feature is disabled".into())}
#[cfg(not(feature="oracles"))]
pub fn project_png_owned(_input:&[u8])->Result<Json,String>{Err("the oracles feature is disabled".into())}
#[cfg(not(feature="oracles"))]
pub fn owned_png_revision(_snapshot:&Json)->Result<String,String>{Err("the oracles feature is disabled".into())}
#[cfg(not(feature="oracles"))]
pub fn oracle_encode_owned(_snapshot:&Json)->Result<Vec<u8>,String>{Err("the oracles feature is disabled".into())}
