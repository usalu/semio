//! 📝️ Encodes typed raster, density, thumbnail and ordered metadata.
use crate::{JpgImage,JpgMutation};
use crate::schema::mutations::ReplaceImage;
use crate::standards::v_jfif_1_01::subsets::document::io::text::diff::*;
use crate::standards::v_jfif_1_01::subsets::document::io::text::mutations::Entry;
pub const TEXT_OPCODE:&str="replace-image";
pub const CODEC:Entry=Entry{opcode:TEXT_OPCODE,print,parse};
pub fn print(value:&JpgMutation)->Option<String>{
 let JpgMutation::ReplaceImage(payload)=value else{return None};let image=&payload.image;
 Some(format!("replace-image width={} height={} pixels={} version={} density-units={} x-density={} y-density={} thumbnail={} segments=[{}]",image.width,image.height,hex_encode(&image.pixels),enc_version(&image.jfif_version),enc_density_units(&image.jfif_density_units),image.jfif_x_density,image.jfif_y_density,encode_option(&image.jfif_thumbnail,enc_thumbnail),image.other_segments.iter().map(enc_segment).collect::<Vec<_>>().join(",")))
}
pub fn parse(line:&str)->Result<JpgMutation,semio_framework_diagnostic::TextError>{
 let parse=||->Result<JpgImage,String>{let(keyword,rest)=line.split_once(' ').ok_or("missing image fields")?;if keyword!=TEXT_OPCODE{return Err("expected replace-image".into());}
 let args:std::collections::BTreeMap<&str,&str>=rest.split(' ').map(|token|token.split_once('=').ok_or_else(||format!("bad image field {token}"))).collect::<Result<_,_>>()?;
 if args.len()!=9{return Err("expected exactly nine image fields".into());}
 let arg=|key:&str|args.get(key).copied().ok_or_else(||format!("missing {key}"));
 let segments=arg("segments")?;let inner=segments.strip_prefix('[').and_then(|value|value.strip_suffix(']')).ok_or("expected image segment list")?;
 let other_segments=if inner.is_empty(){Vec::new()}else{split_top_level(inner,',').into_iter().map(dec_segment).collect::<Result<_,_>>()?};
 Ok(JpgImage{width:arg("width")?.parse().map_err(|e:std::num::ParseIntError|e.to_string())?,height:arg("height")?.parse().map_err(|e:std::num::ParseIntError|e.to_string())?,pixels:hex_decode(arg("pixels")?)?,jfif_version:dec_version(arg("version")?)?,jfif_density_units:dec_density_units(arg("density-units")?)?,jfif_x_density:arg("x-density")?.parse().map_err(|e:std::num::ParseIntError|e.to_string())?,jfif_y_density:arg("y-density")?.parse().map_err(|e:std::num::ParseIntError|e.to_string())?,jfif_thumbnail:decode_option(arg("thumbnail")?,dec_thumbnail)?,other_segments})};
 parse().map(|image|JpgMutation::ReplaceImage(ReplaceImage{image})).map_err(|message|semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message,semio_framework_diagnostic::TextSpan::at(1,1)))
}
