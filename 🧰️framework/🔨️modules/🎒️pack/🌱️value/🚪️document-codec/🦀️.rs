//! 🚪️ Top-level document and record-body entry points, their controlled twins and the decode report.

use semio_framework_value::{DslValue, ValueError, ValueRefusalKind};
use semio_framework_pack_error::PackRefusal;
use semio_framework_dsl_record::{FieldValue, RecordSpec, RecordValue};
use semio_framework_replication_codec::{write_varint_u64, ByteReader, CodecId, PackLimits};
use std::collections::HashMap;
use semio_framework_pack_record_schema::{TAG_VALUE, schema_hash, schema_hash_controlled};
use super::{ControlledMaterialization, DecCtx, DecSource, DecodeOptions, EncCtx, EncodeOptions, MaterializationAdmission, ValueMaterialization, build_symbols, decode_dsl_value, decode_record_fields, encode_record_fields};

//#region 🔖️Document

/// 🩺️ What [`decode_document`] observed beyond the plain `RecordValue`: field ids present
/// on the wire but absent from the caller's `RecordSpec`, any unrecognized segment kinds,
/// whether the manifest's stored `schema_hash` disagrees with the caller's `spec`, and the
/// verification level actually applied.
#[derive(Clone, Debug)]
pub struct DecodeReport {
    pub unknown_field_ids: Vec<u16>,
    pub unknown_segments: Vec<u8>,
    pub schema_drift: bool,
    pub verified: semio_framework_pack_format::VerificationLevel,
}

/// 📄️ The single entry point every other `pack_*`/`vcs`/`dsl_derive` crate encodes a
/// `RecordValue` through. Pre-pass computes the deterministic symbol table, then writes
/// `Symbols`, one-or-more `Document` frames (split at `options.frame_size`), any `Bytes64` chunks
/// produced along the way, and finally the `Manifest`/`End`/`Footer` via `PackWriter::finish`.
pub fn encode_document(spec: &RecordSpec, record: &RecordValue, options: &EncodeOptions) -> Result<Vec<u8>, PackRefusal> {
    let symbols = build_symbols(spec, record);
    let mut symbol_index = HashMap::with_capacity(symbols.len());
    for (i, s) in symbols.iter().enumerate() {
        symbol_index.insert(s.clone(), i as u64);
    }

    let write_options = semio_framework_pack_format::WriteOptions { required_flags:if semio_framework_pack_record_encode::record_required_chunked(record,options)?{semio_framework_pack_format::REQUIRED_CHUNKED}else{0}, optional_flags: if options.canonical { semio_framework_pack_format::OPTIONAL_CANONICAL } else { 0 }, codec: options.codec };
    let mut writer = ::semio_framework_async::poll::resolve_ready(semio_framework_pack_format::PackWriter::begin(Vec::new(), &write_options))?;

    let symbols_payload = ::semio_framework_async::poll::resolve_ready(semio_framework_pack_format::encode_symbols(&symbols));
    ::semio_framework_async::poll::resolve_ready(writer.write_segment(semio_framework_replication_codec::KIND_SYMBOLS, &symbols_payload))?;

    let field_count = record.fields.values().filter(|v| !matches!(v, FieldValue::Absent)).count() as u64;
    let doc_payload = {
        let mut enc_ctx = EncCtx { symbol_index, writer: &mut writer, options };
        encode_record_fields(&mut enc_ctx, Some(spec), record, 0)?
    };

    let frame_size = options.frame_size.max(1) as usize;
    let doc_start = ::semio_framework_async::poll::resolve_ready(writer.position());
    let mut frame_count: u64 = 0;
    for frame in doc_payload.chunks(frame_size) {
        ::semio_framework_async::poll::resolve_ready(writer.write_segment(semio_framework_replication_codec::KIND_DOCUMENT, frame))?;
        frame_count += 1;
    }
    let doc_end = ::semio_framework_async::poll::resolve_ready(writer.position());

    let manifest = semio_framework_pack_format::Manifest {
        schema_name: String::new(),
        schema_hash: schema_hash(spec),
        doc_span: semio_framework_replication_codec::ByteRange { offset: doc_start, len: doc_end - doc_start },
        doc_frame_count: frame_count,
        symbols_span: semio_framework_replication_codec::ByteRange { offset: 0, len: 0 },
        chunk_table_span: semio_framework_replication_codec::ByteRange { offset: 0, len: 0 },
        field_index_span: semio_framework_replication_codec::ByteRange { offset: 0, len: 0 },
        uncompressed_body_len: doc_payload.len() as u64,
        field_count,
        chunk_count: 0,
        symbol_count: symbols.len() as u64,
    };
    ::semio_framework_async::poll::resolve_ready(writer.finish(&manifest))
}

/// 📍️ The single entry point every other `pack_*`/`vcs`/`dsl_derive` crate decodes a
/// `RecordValue` through. Opens the pack file at manifest level, reads and concatenates the
/// `Document` frame(s), then decodes the top-level record body against `spec` — self-describing
/// enough that any field id `spec` doesn't recognize still decodes and is preserved (subject to
/// `options.preserve_unknown`) and reported.
pub fn decode_document(bytes: &[u8], spec: &RecordSpec, options: &DecodeOptions) -> Result<(RecordValue, DecodeReport), PackRefusal> {
    let pack_file = ::semio_framework_async::poll::resolve_ready(semio_framework_pack_format::PackFile::open_manifest(bytes, &options.limits, options.verification))?;
    let manifest = pack_file.manifest().ok_or_else(|| PackRefusal::from(ValueError::new(ValueRefusalKind::InvariantViolated,"manifest not loaded")))?;
    let schema_drift = manifest.schema_hash != schema_hash(spec);
    let body = ::semio_framework_async::poll::resolve_ready(pack_file.body_bytes(options.verification))?;

    let mut reader = ByteReader::new(&body);
    let retained=pack_file.owned_catalog_bytes()?.checked_add(body.capacity() as u64).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "document materialization allocation overflow" })?;
    if retained>options.limits.max_total_alloc{return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "document materialization exceeds max_total_alloc" });}
    let budget=ValueMaterialization{used:std::cell::Cell::new(retained),maximum:options.limits.max_total_alloc};
    let mut dec_ctx = DecCtx { source: DecSource::File(&pack_file), limits: options.limits.clone(), verification: options.verification, preserve_unknown: options.preserve_unknown, unknown_field_ids: Vec::new(), materialization: &budget };
    let record = decode_record_fields(&mut reader, Some(spec), &mut dec_ctx, 0)?;

    let report = DecodeReport { unknown_field_ids: dec_ctx.unknown_field_ids, unknown_segments: Vec::new(), schema_drift, verified: options.verification };
    Ok((record, report))
}

/// 🛬️ Controls catalog/source/inflation/primitive ownership using the caller's typed-construction budget.
pub fn decode_document_controlled(bytes:&[u8],spec:&RecordSpec,options:&DecodeOptions,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(RecordValue,DecodeReport),PackRefusal>{
    let file=::semio_framework_async::poll::resolve_ready(semio_framework_pack_format::PackFile::open_manifest_controlled(bytes,&options.limits,options.verification,control))?;
    let manifest=file.manifest().ok_or_else(||PackRefusal::from(ValueError::new(ValueRefusalKind::InvariantViolated,"controlled manifest absent")))?;let schema_drift=manifest.schema_hash!=schema_hash_controlled(spec,control)?;
    let body=::semio_framework_async::poll::resolve_ready(file.body_bytes_controlled(options.verification,control))?;
    control.begin_stage(0).map_err(PackRefusal::from)?;let materialization=ControlledMaterialization{control:std::cell::RefCell::new(control),maximum:options.limits.max_total_alloc};
    let mut context=DecCtx{source:DecSource::File(&file),limits:options.limits.clone(),verification:options.verification,preserve_unknown:options.preserve_unknown,unknown_field_ids:Vec::new(),materialization:&materialization};let mut reader=ByteReader::new(&body);
    let record=decode_record_fields(&mut reader,Some(spec),&mut context,0)?;
    if reader.remaining()!=0{return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"document",offset:reader.position() as u64,detail:"trailing document body bytes".into()});}
    Ok((record,DecodeReport{unknown_field_ids:context.unknown_field_ids,unknown_segments:Vec::new(),schema_drift,verified:options.verification}))
}

/// 🎯️ Container-less twin of [`encode_document`] for small payloads (operation/command
/// records): `symbol_count varint, (len varint, utf8)*, record fields` — no header, segments,
/// manifest, or footer, and never any `Bytes64` chunking (oversized bytes stay inline via
/// `TAG_BYTES`). Deterministic by the same purity rules as the document path: byte-identical
/// output for equal `(spec, record)` regardless of map iteration order.
pub fn encode_record_body(spec: &RecordSpec, record: &RecordValue, options: &EncodeOptions) -> Result<Vec<u8>, PackRefusal> {
    let symbols = build_symbols(spec, record);
    let mut symbol_index = HashMap::with_capacity(symbols.len());
    for (i, s) in symbols.iter().enumerate() {
        symbol_index.insert(s.clone(), i as u64);
    }
    let mut out = Vec::new();
    write_varint_u64(&mut out, symbols.len() as u64);
    for s in &symbols {
        write_varint_u64(&mut out, s.len() as u64);
        out.extend_from_slice(s.as_bytes());
    }
    let mut body_options = options.clone();
    body_options.chunk_threshold = u64::MAX;
    let write_options = semio_framework_pack_format::WriteOptions { required_flags: 0, optional_flags: 0, codec: CodecId(0) };
    let mut writer = ::semio_framework_async::poll::resolve_ready(semio_framework_pack_format::PackWriter::begin(Vec::new(), &write_options))?;
    let fields = {
        let mut enc_ctx = EncCtx { symbol_index, writer: &mut writer, options: &body_options };
        encode_record_fields(&mut enc_ctx, Some(spec), record, 0)?
    };
    out.extend_from_slice(&fields);
    Ok(out)
}

/// 🎒️ Decodes an [`encode_record_body`] payload against `spec`. Unknown fields decode,
/// are preserved (subject to `options.preserve_unknown`), and are reported exactly like the
/// document path; a `TAG_BYTES_CHUNKED` value is malformed here by construction.
pub fn decode_record_body(bytes: &[u8], spec: &RecordSpec, options: &DecodeOptions) -> Result<(RecordValue, DecodeReport), PackRefusal> {
    decode_record_body_inner(bytes, spec, options, false)
}

/// 🔒️ Decodes exactly one terminal record body, rejecting unknown fields and trailing bytes.
pub fn decode_record_body_exact(bytes: &[u8], spec: &RecordSpec, options: &DecodeOptions) -> Result<RecordValue, PackRefusal> {
    decode_record_body_inner(bytes, spec, options, true).map(|(record, _)| record)
}

/// 📹️ Parses a terminal container-less record under the same control used by its typed constructor.
pub fn decode_record_body_exact_controlled(bytes:&[u8],spec:&RecordSpec,options:&DecodeOptions,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<RecordValue,PackRefusal>{
    if bytes.len() as u64>options.limits.max_file_len{return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "record body exceeds max_file_len" });}
    control.begin_stage(0).map_err(PackRefusal::from)?;
    let mut reader=ByteReader::new(bytes);
    let materialization=ControlledMaterialization{control:std::cell::RefCell::new(control),maximum:options.limits.max_total_alloc};
    let symbols=decode_inline_symbols(&mut reader,&options.limits,&materialization)?;
    let mut ctx=DecCtx{source:DecSource::Inline{symbols},limits:options.limits.clone(),verification:options.verification,preserve_unknown:false,unknown_field_ids:Vec::new(),materialization:&materialization};
    let record=decode_record_fields(&mut reader,Some(spec),&mut ctx,0)?;
    if reader.position()!=bytes.len(){return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"record body",offset:reader.position() as u64,detail:"trailing bytes after terminal record".into()});}
    if !ctx.unknown_field_ids.is_empty(){return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"record body",offset:reader.position() as u64,detail:"unknown terminal record fields".into()});}
    Ok(record)
}

/// 📖️ Decodes one terminal Record directly from a checked source range under the caller's control.
pub fn decode_record_body_span_exact_controlled(bytes:semio_framework_replication_codec::ByteSpan<'_>,spec:&RecordSpec,options:&DecodeOptions,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<RecordValue,PackRefusal>{
    if bytes.len()as u64>options.limits.max_file_len{return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"record body exceeds max_file_len"})}
    control.begin_stage(0).map_err(PackRefusal::from)?;
    let mut reader=ByteReader::from_span(bytes);
    let materialization=ControlledMaterialization{control:std::cell::RefCell::new(control),maximum:options.limits.max_total_alloc};
    let symbols=decode_inline_symbols(&mut reader,&options.limits,&materialization)?;
    let mut context=DecCtx{source:DecSource::Inline{symbols},limits:options.limits.clone(),verification:options.verification,preserve_unknown:false,unknown_field_ids:Vec::new(),materialization:&materialization};
    let record=decode_record_fields(&mut reader,Some(spec),&mut context,0)?;
    if reader.position()!=bytes.len(){return Err(PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"record body",offset:reader.position()as u64,detail:"trailing bytes after terminal record"})}
    if !context.unknown_field_ids.is_empty(){return Err(PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"record body",offset:reader.position()as u64,detail:"unknown terminal record fields"})}
    Ok(record)
}

fn decode_inline_symbols(reader: &mut ByteReader<'_>, limits: &PackLimits, materialization: &dyn MaterializationAdmission) -> Result<Vec<String>, PackRefusal> {
    let symbol_count = reader.read_varint_u64()?;
    if symbol_count > u64::from(limits.max_symbols) {
        return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "record-body symbol count exceeds max_symbols" });
    }
    materialization.charge(symbol_count.checked_mul(size_of::<String>() as u64).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "wire symbol slot overflow" })?)?;
    let count = usize::try_from(symbol_count).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "wire symbol slot count" })?;
    let mut symbols = Vec::new();
    symbols.try_reserve_exact(count).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::AllocationFailed, limit: "wire symbol slot allocation" })?;
    for _ in 0..symbol_count {
        materialization.step()?;
        let len = reader.read_varint_u64()?;
        if len > limits.max_segment_len {
            return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "record-body symbol length exceeds max_segment_len" });
        }
        let raw = reader.read_span(usize::try_from(len).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "wire symbol byte length" })?)?;
        let symbol_offset = reader.position() as u64;
        symbols.push(materialization.copy_span_utf8(raw,symbol_offset,"symbol")?);
    }
    Ok(symbols)
}

/// 🛡️ Decodes one exact intrinsic field with cumulative concrete storage admission.
/// Owned UTF-8 and typed symbol, list and member slots are admitted before allocation.
pub fn decode_value_record_body_exact(bytes: &[u8], field_id: u16, limits: &PackLimits) -> Result<DslValue, PackRefusal> {
    if bytes.len() as u64 > limits.max_file_len {
        return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "wire value exceeds max_file_len" });
    }
    let mut reader = ByteReader::new(bytes);
    let budget = ValueMaterialization { used: std::cell::Cell::new(0), maximum: limits.max_total_alloc };
    let symbols = decode_inline_symbols(&mut reader, limits, &budget)?;
    if reader.read_varint_u64()? != 1 || reader.read_varint_u64()? != u64::from(field_id) || reader.read_u8()? != TAG_VALUE {
        return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "wire value", offset: reader.position() as u64, detail: "expected exactly one declared Value field".into() });
    }
    let mut ctx = DecCtx { source: DecSource::Inline { symbols }, limits: limits.clone(), verification: semio_framework_pack_format::VerificationLevel::Standard, preserve_unknown: false, unknown_field_ids: Vec::new(), materialization: &budget };
    let value = decode_dsl_value(&mut reader, &mut ctx, 0)?;
    if reader.position() != bytes.len() {
        return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "wire value", offset: reader.position() as u64, detail: "trailing bytes after the terminal value".into() });
    }
    Ok(value)
}

/// 🎞️ Decodes one terminal intrinsic field under cumulative ownership and cancellation.
pub fn decode_value_record_body_exact_controlled(bytes:&[u8],field_id:u16,options:&DecodeOptions,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<DslValue,PackRefusal>{
    if bytes.len() as u64>options.limits.max_file_len{return Err(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::WorkLimit,"intrinsic wire exceeds max_file_len")));}
    control.scoped_stage(|control|{control.begin_stage(0).map_err(PackRefusal::ValueRefusal)?;let mut reader=ByteReader::new(bytes);let materialization=ControlledMaterialization{control:std::cell::RefCell::new(control),maximum:options.limits.max_total_alloc};let symbols=decode_inline_symbols(&mut reader,&options.limits,&materialization)?;if reader.read_varint_u64()?!=1||reader.read_varint_u64()?!=u64::from(field_id)||reader.read_u8()?!=TAG_VALUE{return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"intrinsic wire",offset:reader.position() as u64,detail:"expected exactly one declared Value field".into()});}let mut ctx=DecCtx{source:DecSource::Inline{symbols},limits:options.limits.clone(),verification:options.verification,preserve_unknown:false,unknown_field_ids:Vec::new(),materialization:&materialization};let value=decode_dsl_value(&mut reader,&mut ctx,0)?;if reader.position()!=bytes.len(){return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"intrinsic wire",offset:reader.position() as u64,detail:"trailing bytes after terminal intrinsic value".into()});}materialization.control.borrow_mut().checkpoint().map_err(PackRefusal::ValueRefusal)?;Ok(value)})
}

/// 💎️ Decodes one exact intrinsic body by borrowing the caller's complete immutable byte source.
pub fn decode_value_record_body_span_exact_controlled(bytes:semio_framework_replication_codec::ByteSpan<'_>,field_id:u16,options:&DecodeOptions,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<DslValue,PackRefusal>{
    if bytes.len() as u64>options.limits.max_file_len{return Err(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::WorkLimit,"intrinsic wire exceeds max_file_len")));}
    control.scoped_stage(|control|{control.begin_stage(0).map_err(PackRefusal::ValueRefusal)?;let mut reader=ByteReader::from_span(bytes);let materialization=ControlledMaterialization{control:std::cell::RefCell::new(control),maximum:options.limits.max_total_alloc};let symbols=decode_inline_symbols(&mut reader,&options.limits,&materialization)?;if reader.read_varint_u64()?!=1||reader.read_varint_u64()?!=u64::from(field_id)||reader.read_u8()?!=TAG_VALUE{return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"intrinsic wire",offset:reader.position() as u64,detail:"expected exactly one declared Value field".into()});}let mut ctx=DecCtx{source:DecSource::Inline{symbols},limits:options.limits.clone(),verification:options.verification,preserve_unknown:false,unknown_field_ids:Vec::new(),materialization:&materialization};let value=decode_dsl_value(&mut reader,&mut ctx,0)?;if reader.position()!=bytes.len(){return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"intrinsic wire",offset:reader.position() as u64,detail:"trailing bytes after terminal intrinsic value".into()});}materialization.control.borrow_mut().checkpoint().map_err(PackRefusal::ValueRefusal)?;Ok(value)})
}

fn decode_record_body_inner(bytes: &[u8], spec: &RecordSpec, options: &DecodeOptions, exact: bool) -> Result<(RecordValue, DecodeReport), PackRefusal> {
    if bytes.len() as u64 > options.limits.max_file_len { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "record body exceeds max_file_len" }); }
    let mut reader = ByteReader::new(bytes);
    let budget = ValueMaterialization { used: std::cell::Cell::new(0), maximum: options.limits.max_total_alloc };
    let symbols = decode_inline_symbols(&mut reader, &options.limits, &budget)?;
    let mut dec_ctx = DecCtx { source: DecSource::Inline { symbols }, limits: options.limits.clone(), verification: options.verification, preserve_unknown: options.preserve_unknown, unknown_field_ids: Vec::new(), materialization: &budget };
    let record = decode_record_fields(&mut reader, Some(spec), &mut dec_ctx, 0)?;
    if exact && reader.position() != bytes.len() {
        return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "record body", offset: reader.position() as u64, detail: "trailing bytes after the terminal record".into() });
    }
    if exact && !dec_ctx.unknown_field_ids.is_empty() {
        return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "record body", offset: reader.position() as u64, detail: format!("{} unknown terminal record fields, first {}", dec_ctx.unknown_field_ids.len(), dec_ctx.unknown_field_ids[0]) });
    }
    let report = DecodeReport { unknown_field_ids: dec_ctx.unknown_field_ids, unknown_segments: Vec::new(), schema_drift: false, verified: options.verification };
    Ok((record, report))
}
//#endregion 🔖️Document
