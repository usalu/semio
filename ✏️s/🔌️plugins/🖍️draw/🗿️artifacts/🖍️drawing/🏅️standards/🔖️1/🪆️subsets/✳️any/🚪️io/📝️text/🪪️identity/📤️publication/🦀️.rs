//! 🪪️ Controlled lowercase Drawing identity publication.
use crate::standards::v1::subsets::any::schema::identity::{DrawingIdentity,DrawingIdentityKind,DrawingIdentityCommitment};
use semio_framework_value::{NativeEncodeControl,ValueError};
pub fn encode_identity_text_into<'a>(commitment:&DrawingIdentityCommitment,bytes:&'a mut[u8;76],control:&mut NativeEncodeControl<'_>)->Result<&'a str,ValueError>{
 control.begin_stage(32)?;let prefix=match commitment.kind{DrawingIdentityKind::Layer=>"layer",DrawingIdentityKind::Path=>"path",DrawingIdentityKind::Group=>"group",DrawingIdentityKind::Boolean=>"boolean",DrawingIdentityKind::Trace=>"trace",DrawingIdentityKind::Shape=>"shape",DrawingIdentityKind::Text=>"text",DrawingIdentityKind::Image=>"image",DrawingIdentityKind::Svg=>"svg",DrawingIdentityKind::ImageAsset=>"image-asset"};
 bytes[..prefix.len()].copy_from_slice(prefix.as_bytes());bytes[prefix.len()]=b'-';let digits=b"0123456789abcdef";for(index,byte)in commitment.digest.iter().enumerate(){control.step()?;bytes[prefix.len()+1+index*2]=digits[usize::from(byte>>4)];bytes[prefix.len()+2+index*2]=digits[usize::from(byte&15)];}std::str::from_utf8(&bytes[..prefix.len()+65]).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Drawing identity spelling is not UTF8"))
}
pub fn encode_identity_text(commitment:&DrawingIdentityCommitment,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let mut bytes=[0u8;76];let text=encode_identity_text_into(commitment,&mut bytes,control)?;control.copy_text(text)}

pub fn admit_identity(kind:DrawingIdentityKind,parts:&[&[u8]],control:&mut NativeEncodeControl<'_>)->Result<DrawingIdentity,ValueError>{
 let commitment=crate::standards::v1::subsets::any::io::binary::identity::commit_identity(kind,parts,control)?;let key=encode_identity_text(&commitment,control)?;DrawingIdentity::admit(key.into())
}
