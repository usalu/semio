//! 💾️ Structurally encodes every owned image field.
use crate::{JpgImage,JpgMutation};
use crate::schema::mutations::ReplaceImage;
use crate::standards::v_jfif_1_01::subsets::document::io::binary::diff::*;
use crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::Entry;
pub const BINARY_TAG:u8=dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"),"replace-image");
pub const CODEC:Entry=Entry{tag:BINARY_TAG,encode,decode};
pub fn encode(value:&JpgMutation)->Option<Result<Vec<u8>,protocol::ProtocolError>>{
 let JpgMutation::ReplaceImage(payload)=value else{return None};
 let image=&payload.image;let mut out=Vec::new();
 store::pack_rt::write_varint_u64(&mut out,image.width.into());store::pack_rt::write_varint_u64(&mut out,image.height.into());
 write_bytes_lp(&mut out,&image.pixels);enc_version_bin(&image.jfif_version,&mut out);enc_density_units_bin(&image.jfif_density_units,&mut out);
 store::pack_rt::write_varint_u64(&mut out,image.jfif_x_density.into());store::pack_rt::write_varint_u64(&mut out,image.jfif_y_density.into());
 write_opt(&mut out,&image.jfif_thumbnail,enc_thumbnail_bin);
 store::pack_rt::write_varint_u64(&mut out,image.other_segments.len() as u64);for segment in &image.other_segments{enc_segment_bin(segment,&mut out);}
 Some(Ok(out))
}
pub fn decode(bytes:&[u8])->Result<JpgMutation,protocol::ProtocolError>{
 let parse=||->Result<JpgImage,String>{let mut reader=store::ByteReader::new(bytes);
 let width=u32::try_from(reader.read_varint_u64().map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
 let height=u32::try_from(reader.read_varint_u64().map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
 let pixels=read_bytes_lp(&mut reader)?;let jfif_version=dec_version_bin(&mut reader)?;let jfif_density_units=dec_density_units_bin(&mut reader)?;
 let jfif_x_density=u16::try_from(reader.read_varint_u64().map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
 let jfif_y_density=u16::try_from(reader.read_varint_u64().map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
 let jfif_thumbnail=read_opt(&mut reader,dec_thumbnail_bin)?;
 let count=usize::try_from(reader.read_varint_u64().map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
 if count>bytes.len().saturating_sub(reader.position()){return Err("image segment count exceeds its payload".into());}
 let mut other_segments=Vec::new();for _ in 0..count{other_segments.push(dec_segment_bin(&mut reader)?);}
 if reader.position()!=bytes.len(){return Err("trailing image bytes".into());}
 Ok(JpgImage{width,height,pixels,jfif_version,jfif_density_units,jfif_x_density,jfif_y_density,jfif_thumbnail,other_segments})};
 parse().map(|image|JpgMutation::ReplaceImage(ReplaceImage{image})).map_err(|detail|protocol::ProtocolError::Malformed{what:"replace-image",offset:0,detail})
}
