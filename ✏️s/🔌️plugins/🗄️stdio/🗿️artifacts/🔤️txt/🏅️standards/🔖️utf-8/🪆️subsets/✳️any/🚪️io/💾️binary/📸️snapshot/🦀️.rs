//! 🧱️ binary rep for stdio.txt 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_utf_8::subsets::any::schema::snapshot::*;
use crate::STDIO_TXT_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

impl pack::snapshot::ArtifactPack for TxtSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;

        let raw = self.to_body();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error|store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, raw.as_bytes()))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        let body = String::from_utf8(inner).map_err(|error|store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string())))?;
        Ok(Self::from_body(&body))
    }
    /// 🧬️ The structural fingerprint `ArtifactCodec::pack_schema_hash` and the hub's trusted
    /// catalog pin this kind by. The `dsl::DslRecord` derive above already generates the spec —
    /// the same one `register_schema_spec("stdio.txt", TxtSnapshot::__dsl_spec)` publishes to the
    /// DSL registry — but a HAND-ROLLED `ArtifactPack` never picks up the derive's `record_spec`
    /// override the way `DslArtifact` does, so this kind silently answered the trait's `None`
    /// opt-out. That opt-out is what `codec.pack-schema-hash(stdio.txt)` reported as
    /// `artifact codec schema has no structural record specification`, and what the catalog
    /// builder refuses as the zero fingerprint (ticket 26/09/18 slice TC4). The body's raw
    /// line-joined pack encoding is unaffected: this hash fingerprints the SNAPSHOT RECORD's
    /// fields, not the pack container.
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot for TxtSnapshot {
    fn reference_snapshot_codec()->Option<::semio_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}
}
use crate::standards::v_utf_8::subsets::any::schema::snapshot::TxtSnapshot;

/// 🔤️ The original receiving tuple of one text body: schema, ordered lines, pending line and the typed publication.
pub(crate) type TextReceiving=(String,Vec<String>,String,Option<TxtSnapshot>);

impl store::ArtifactPackReceiving for TxtSnapshot {
    /// 🫴️ Keeps original line storage and typed publication in the supplied native receiving owner.
    fn receive_pack(bytes:&[u8],output:&mut Option<Self>,owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->semio_framework_value::RetainedWorkOutcome<(),store::NativeSnapshotFailure>{
        owner.receive::<TextReceiving,Self,store::NativeSnapshotFailure>(output,&mut |slot,published,_marker,_unwind,native,body|{
            let before=native.actual_work();
            let result=Self::bind_pack_rows(bytes,slot,native,body,None).map(|snapshot|{*published=Some(snapshot);}).map_err(store::NativeSnapshotFailure::from);
            semio_framework_value::RetainedWorkOutcome{result,work:native.actual_work().saturating_sub(before)}
        })
    }
}

impl TxtSnapshot {
    /// 🧾️ Applies an actual relational row ceiling before any typed line storage is born.
    pub(crate) fn bind_pack_rows(bytes:&[u8],slot:&mut Option<TextReceiving>,native:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,body:&mut store::NativeSnapshotBodyWallet,maximum_rows:Option<usize>)->Result<Self,semio_framework_value::ValueError>{
        native.checkpoint()?;
        let bytes=store::semio_format::unwrap_binary_controlled(bytes,crate::STDIO_TXT_DOCUMENT_SCHEMA,store::semio_format::Component::Pack,1,native)?;
        let text=native.borrow_text(bytes)?;
        Self::bind_text(text,slot,native,body,maximum_rows)
    }

    /// 🔤️ Binds either admitted carrier through the same original line ownership lifecycle.
    pub(crate) fn bind_text(text:&str,slot:&mut Option<TextReceiving>,native:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,body:&mut store::NativeSnapshotBodyWallet,maximum_rows:Option<usize>)->Result<Self,semio_framework_value::ValueError>{
        use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
        use crate::standards::v_utf_8::subsets::any::schema::snapshot::LineEnding;
        {
            let inline=size_of::<TextReceiving>();
            body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:inline,maximum_depth:1,..Default::default()})?;native.checkpoint()?;
            *slot=Some((String::new(),Vec::new(),String::new(),None));body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:inline,..Default::default()})?;
            let mut lf=0usize;let mut crlf=0usize;let mut previous=0u8;
            native.scoped_stage(|native|{
                native.begin_stage(text.len())?;
                for chunk in text.as_bytes().chunks(65536){
                    body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()})?;
                    native.checkpoint()?;for &byte in chunk{if byte==b'\n'{lf=lf.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"Txt line count overflow"))?;if previous==b'\r'{crlf=crlf.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"Txt line count overflow"))?;}}previous=byte;}
                    let step=native.advance(chunk.len());body.record_progress(RetainedCloneProgress{copied_items:1,..Default::default()})?;step?;
                }
                Ok::<_,ValueError>(())
            })?;
            let line_ending=if crlf>0{LineEnding::CrLf}else{LineEnding::Lf};let separator=line_ending.as_str();let trailing_newline=!text.is_empty()&&text.ends_with(separator);
            let count=if text.is_empty(){0}else{(if crlf>0{crlf}else{lf}).checked_add(1).and_then(|count|count.checked_sub(usize::from(trailing_newline))).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"Txt line count overflow"))?};
            let rows=count.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"Txt entity count overflow"))?;
            if maximum_rows.is_some_and(|maximum|rows>maximum){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native Txt exceeds original row limit"))}
            let content=if trailing_newline{&text[..text.len()-separator.len()]}else{text};
            let(schema,lines,pending,typed)=slot.as_mut().unwrap();
            body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:crate::STDIO_TXT_DOCUMENT_SCHEMA.len(),maximum_capacity_bytes:crate::STDIO_TXT_DOCUMENT_SCHEMA.len(),maximum_depth:2,..Default::default()})?;native.checkpoint()?;
            body.copy_text_into(native,crate::STDIO_TXT_DOCUMENT_SCHEMA,schema,2)?;
            let capacity=count.checked_mul(size_of::<String>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Txt row backing extent overflow"))?;
            body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<String>>(),maximum_capacity_bytes:capacity,maximum_depth:3,..Default::default()})?;native.checkpoint()?;body.allocate_vec_into(native,count,lines,3)?;
            if !text.is_empty(){for line in content.split(separator){
                body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:line.len(),maximum_capacity_bytes:line.len(),maximum_depth:3,..Default::default()})?;native.checkpoint()?;body.copy_text_into(native,line,pending,3)?;
                body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<String>(),maximum_depth:3,..Default::default()})?;native.checkpoint()?;
                lines.push(std::mem::take(pending));body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<String>(),..Default::default()})?;
            }}
            let inline=size_of::<Self>();body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:inline,maximum_depth:1,..Default::default()})?;native.checkpoint()?;
            *typed=Some(Self{schema:std::mem::take(schema),lines:std::mem::take(lines),trailing_newline,line_ending});body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:inline,..Default::default()})?;
            Ok(typed.take().unwrap())
        }
    }
}
