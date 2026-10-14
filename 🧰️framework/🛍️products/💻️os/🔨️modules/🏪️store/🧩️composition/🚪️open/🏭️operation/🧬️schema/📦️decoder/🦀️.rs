//! 📦️ The selected codec decodes the original request-owned Pack into its actual typed snapshot.
use crate::os_pack::PackError;
use pack::snapshot::ArtifactPack;
use std::marker::PhantomData;

/// 🏭️ Original typed output admission follows the member request's existing source authority check.
pub trait MemberSnapshotDecoder<P> {
 fn decode_snapshot(pack:&[u8])->Result<P,PackError>;
}

/// 🎒️ Selects a snapshot owner's authored first-party Pack implementation.
pub struct ArtifactPackSnapshotDecoder<P>(PhantomData<fn()->P>);
impl<P:ArtifactPack> MemberSnapshotDecoder<P> for ArtifactPackSnapshotDecoder<P> {
 fn decode_snapshot(pack:&[u8])->Result<P,PackError>{P::decode_pack(pack)}
}
