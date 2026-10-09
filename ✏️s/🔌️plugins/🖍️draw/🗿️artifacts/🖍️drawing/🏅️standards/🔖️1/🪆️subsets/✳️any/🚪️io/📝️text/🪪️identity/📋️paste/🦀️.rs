//! 📋️ Counted clipboard identity preimages and bounded borrowed text publication.
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind};
use crate::schema::identity::DrawingIdentityKind;
pub fn prepare_paste_identity<'a>(kind:DrawingIdentityKind,document:&semio_framework_value::paged::PagedUtf8<{usize::MAX}>,source:&str,ordinal:usize,material:&mut[u8],output:&'a mut[u8;76],control:&mut NativeEncodeControl<'_>)->Result<&'a str,ValueError>{
 control.checkpoint()?;if document.is_empty()||document.len()>material.len()||source.is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Drawing clipboard identity source invalid"));}
 let mut at=0;for chunk in document.retained_chunks().iter(){let end=at+chunk.len();material[at..end].copy_from_slice(chunk.as_bytes());at=end;control.step()?;}
 let ordinal=(ordinal as u64).to_le_bytes();let commitment=crate::standards::v1::subsets::any::io::binary::identity::commit_identity(kind,&[&material[..at],source.as_bytes(),b"paste",&ordinal],control)?;
 super::publication::encode_identity_text_into(&commitment,output,control)
}
