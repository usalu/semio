//! 🔏️ Versioned counted Drawing identity preimages and controlled SHA-256 commitments.
use crate::standards::v1::subsets::any::schema::identity::{DrawingIdentityKind,DrawingIdentityCommitment};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind};
const HEADER:&[u8;8]=b"DRAWID01";
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"Drawing identity input exceeds canonical limits")}
fn tag(kind:DrawingIdentityKind)->u8{match kind{DrawingIdentityKind::Layer=>0,DrawingIdentityKind::Path=>1,DrawingIdentityKind::Group=>2,DrawingIdentityKind::Boolean=>3,DrawingIdentityKind::Trace=>4,DrawingIdentityKind::Shape=>5,DrawingIdentityKind::Text=>6,DrawingIdentityKind::Image=>7,DrawingIdentityKind::Svg=>8,DrawingIdentityKind::ImageAsset=>9}}
fn length(parts:&[&[u8]])->Result<usize,ValueError>{if parts.len()>64{return Err(invalid());}parts.iter().try_fold(13usize,|total,part|total.checked_add(8).and_then(|n|n.checked_add(part.len())).filter(|n|*n<=65536).ok_or_else(invalid))}
pub fn encode_identity_preimage(kind:DrawingIdentityKind,parts:&[&[u8]],control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,ValueError>{
 control.checkpoint()?;let total=length(parts)?;control.begin_stage(total)?;let mut output=control.allocate_vec(total)?;output.extend_from_slice(HEADER);output.push(tag(kind));output.extend_from_slice(&(parts.len()as u32).to_le_bytes());control.advance(13)?;
 for part in parts{output.extend_from_slice(&(part.len()as u64).to_le_bytes());control.advance(8)?;for chunk in part.chunks(256){output.extend_from_slice(chunk);control.advance(chunk.len())?;}}Ok(output)
}
pub fn commit_identity(kind:DrawingIdentityKind,parts:&[&[u8]],control:&mut NativeEncodeControl<'_>)->Result<DrawingIdentityCommitment,ValueError>{
 control.checkpoint()?;let total=length(parts)?;control.begin_stage(total)?;let mut hash=semio_framework_hash::Sha256::new();hash.update(HEADER);hash.update(&[tag(kind)]);hash.update(&(parts.len()as u32).to_le_bytes());control.advance(13)?;
 for part in parts{hash.update(&(part.len()as u64).to_le_bytes());control.advance(8)?;for chunk in part.chunks(256){hash.update(chunk);control.advance(chunk.len())?;}}control.checkpoint()?;Ok(DrawingIdentityCommitment{kind,digest:hash.finalize()})
}
