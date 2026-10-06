//! 🚪️ Native artifact representation codecs.
use super::super::super::*;
use semio_framework_os_kernel::{os_spr as protocol,os_store as store};

impl store::ArtifactPack for Snapshot {
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{store::pack_rt::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options)}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(record,_)=store::pack_rt::decode_document(bytes,&Self::__dsl_spec(),options)?;Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)}
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Self::__dsl_spec())}
}
