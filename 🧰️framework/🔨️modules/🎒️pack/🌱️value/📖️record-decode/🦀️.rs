//! 📖️ Wire decoding: the symref and chunk source, the materialization admission owners and the tag-driven value decoders.

use semio_framework_value::{DslValue, Number, ValueError, ValueRefusalKind};
use semio_framework_pack_error::PackRefusal;
use semio_framework_dsl_record::{FieldValue, RecordSpec, RecordSpecProducer, RecordValue, Shape, WireEdgeLabel, WireNode, WireValue};
use semio_framework_replication_codec::{ByteReader, ChunkId, PackLimits};
use semio_framework_pack_record_schema::{TAG_ABSENT, TAG_BLOCK, TAG_BYTES, TAG_BYTES_CHUNKED, TAG_ENUM, TAG_EXPR, TAG_F64, TAG_FALSE, TAG_INT, TAG_LIST, TAG_MAP, TAG_NULL, TAG_PACKED_F64, TAG_PACKED_VARINT, TAG_RECORD, TAG_STATEMENTS, TAG_STR, TAG_STR_INLINE, TAG_TABLE_SOA, TAG_TRUE, TAG_TUPLE, TAG_UINT, TAG_VALUE, TAG_WIRE, block_inner_shape, elem_shape_of, map_inner_shape, statements_variants, table_spec_of};
use super::{check_depth, decode_table_soa, is_tuple_shape};

//#region 🔖️Decode
/// 🧭️ Where symrefs and chunk ids resolve during one decode: a full opened `PackFile`
/// (the `decode_document` path) or a container-less inline symbol table (the
/// `decode_record_body` path, which has no chunk table by construction).
pub(super) enum DecSource<'a> {
    File(&'a semio_framework_pack_format::PackFile<&'a [u8]>),
    Inline { symbols: Vec<String> },
}

/// 📖️ Mutable state threaded through one `decode_document`/`decode_record_body` call: the
/// symref/chunk resolution source, the caller's limits/verification/preserve-unknown choices, and
/// the accumulated unknown-field-id report.
pub(super) struct DecCtx<'a> {
    pub(super) source: DecSource<'a>,
    pub(super) limits: PackLimits,
    pub(super) verification: semio_framework_pack_format::VerificationLevel,
    pub(super) preserve_unknown: bool,
    pub(super) unknown_field_ids: Vec<u16>,
    pub(super) materialization: &'a dyn MaterializationAdmission,
}

/// 🧮️ Cumulative concrete requested storage for owned UTF-8 and typed collection slots.
pub(super) struct ValueMaterialization {
    pub(super) used: std::cell::Cell<u64>,
    pub(super) maximum: u64,
}

impl ValueMaterialization {
    pub(super) fn charge(&self, bytes: u64) -> Result<(), PackRefusal> {
        let used = self.used.get().checked_add(bytes).filter(|used| *used <= self.maximum).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "wire value materialization exceeds max_total_alloc" })?;
        self.used.set(used);
        Ok(())
    }
}

/// 🧾️ Transfers octets after controlled validation of the same immutable source range.
/// Both private admission owners use the exact span copy; the immutable borrow prevents its
/// validated scalar words from changing between validation and owned transfer.
fn copy_validated_span_utf8<M:MaterializationAdmission+?Sized>(materialization:&M,bytes:semio_framework_replication_codec::ByteSpan<'_>,offset:u64,what:&'static str)->Result<String,PackRefusal>{
    materialization.check_span_utf8(bytes,offset,what)?;
    let bytes=materialization.copy_span_bytes(bytes)?;
    Ok(unsafe{String::from_utf8_unchecked(bytes)})
}

pub(super) trait MaterializationAdmission {
    fn schema(&self,producer:&RecordSpecProducer)->Result<RecordSpec,PackRefusal>{Ok((producer.ordinary)())}
    fn charge(&self,bytes:u64)->Result<(),PackRefusal>;
    fn step(&self)->Result<(),PackRefusal>{Ok(())}
    fn controlled(&self)->bool{false}
    fn copy_text(&self,text:&str)->Result<String,PackRefusal>{self.charge(text.len() as u64)?;let mut output=String::new();output.try_reserve_exact(text.len()).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::AllocationFailed, limit: "decoded string allocation" })?;output.push_str(text);Ok(output)}
    fn copy_bytes(&self,bytes:&[u8])->Result<Vec<u8>,PackRefusal>{self.charge(bytes.len() as u64)?;let mut output=Vec::new();output.try_reserve_exact(bytes.len()).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::AllocationFailed, limit: "decoded octet allocation" })?;output.extend_from_slice(bytes);Ok(output)}
    fn octet_buffer(&self,count:usize)->Result<Vec<u8>,PackRefusal>{self.charge(count as u64)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::AllocationFailed, limit: "decoded octet allocation" })?;Ok(output)}
    fn copy_span_bytes(&self,bytes:semio_framework_replication_codec::ByteSpan<'_>)->Result<Vec<u8>,PackRefusal>{
        if let Some(bytes)=bytes.contiguous(){return self.copy_bytes(bytes)}
        let mut output=self.octet_buffer(bytes.len())?;
        for byte in bytes.iter(){self.step()?;output.push(byte);}
        Ok(output)
    }
    fn check_span_utf8(&self,bytes:semio_framework_replication_codec::ByteSpan<'_>,offset:u64,what:&'static str)->Result<(),PackRefusal>{
        let invalid=||PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what,offset,detail:"invalid utf8"};
        let mut position=0;
        while position<bytes.len(){
            self.step()?;
            let first=bytes[position];
            let length=match first{0..=127=>1,194..=223=>2,224..=239=>3,240..=244=>4,_=>return Err(invalid())};
            if position+length>bytes.len(){return Err(invalid())}
            let mut word=[0;4];
            for(index,byte)in word[..length].iter_mut().enumerate(){*byte=bytes[position+index];}
            std::str::from_utf8(&word[..length]).map_err(|_|invalid())?;
            position+=length;
        }
        Ok(())
    }
    fn copy_span_utf8(&self,bytes:semio_framework_replication_codec::ByteSpan<'_>,offset:u64,what:&'static str)->Result<String,PackRefusal>{
        if let Some(bytes)=bytes.contiguous(){return self.copy_utf8(bytes,offset,what)}
        copy_validated_span_utf8(self,bytes,offset,what)
    }
    fn append_chunk(&self,_file:&semio_framework_pack_format::PackFile<&[u8]>,_id:ChunkId,_verification:semio_framework_pack_format::VerificationLevel,_output:&mut Vec<u8>)->Result<(),PackRefusal>{Err(PackRefusal::from(ValueError::new(ValueRefusalKind::UnsupportedOwner,"chunk owner has no controlled admission")))}
    fn expression(&self,text:&str,_maximum_depth:u16)->Result<semio_framework_dsl_record::ExprValue,PackRefusal>{semio_framework_dsl_record::parse_expr_text(text).map_err(PackRefusal::from)}
    fn copy_utf8(&self,bytes:&[u8],offset:u64,what:&'static str)->Result<String,PackRefusal>{let value=std::str::from_utf8(bytes).map_err(|_|PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what,offset,detail:"invalid utf8".into()})?;self.copy_text(value)}
}

impl MaterializationAdmission for ValueMaterialization {fn charge(&self,bytes:u64)->Result<(),PackRefusal>{ValueMaterialization::charge(self,bytes)}}

pub(super) struct ControlledMaterialization<'a,'callback>{pub(super) control:std::cell::RefCell<&'a mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'callback>>,pub(super) maximum:u64}
impl ControlledMaterialization<'_,'_>{
    fn check(&self,bytes:u64)->Result<(),PackRefusal>{if (self.control.borrow().owned_bytes() as u64).checked_add(bytes).filter(|next|*next<=self.maximum).is_none(){return Err(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"controlled native materialization exceeds max_total_alloc")));}Ok(())}
}
impl MaterializationAdmission for ControlledMaterialization<'_,'_>{
    fn schema(&self,producer:&RecordSpecProducer)->Result<RecordSpec,PackRefusal>{let maximum=usize::try_from(self.maximum).unwrap_or(usize::MAX);self.control.borrow_mut().scoped_maximum(maximum,|control|producer.decode(control)).map_err(PackRefusal::from)}
    fn charge(&self,bytes:u64)->Result<(),PackRefusal>{self.check(bytes)?;self.control.borrow_mut().charge(usize::try_from(bytes).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "native allocation exceeds address space" })?).map_err(PackRefusal::from)}
    fn step(&self)->Result<(),PackRefusal>{self.control.borrow_mut().step().map_err(PackRefusal::from)}
    fn controlled(&self)->bool{true}
    fn copy_text(&self,text:&str)->Result<String,PackRefusal>{self.check(text.len() as u64)?;self.control.borrow_mut().copy_text(text).map_err(PackRefusal::from)}
    fn copy_bytes(&self,bytes:&[u8])->Result<Vec<u8>,PackRefusal>{self.check(bytes.len() as u64)?;self.control.borrow_mut().copy_bytes(bytes).map_err(PackRefusal::from)}
    fn octet_buffer(&self,count:usize)->Result<Vec<u8>,PackRefusal>{self.check(count as u64)?;self.control.borrow_mut().allocate_vec(count).map_err(PackRefusal::from)}
    fn append_chunk(&self,file:&semio_framework_pack_format::PackFile<&[u8]>,id:ChunkId,verification:semio_framework_pack_format::VerificationLevel,output:&mut Vec<u8>)->Result<(),PackRefusal>{self.control.borrow_mut().scoped_stage(|control|::semio_framework_async::poll::resolve_ready(file.append_chunk_controlled(id,verification,output,control)))}
    fn expression(&self,text:&str,maximum_depth:u16)->Result<semio_framework_dsl_record::ExprValue,PackRefusal>{let maximum=usize::try_from(self.maximum).unwrap_or(usize::MAX);let mut limits=semio_framework_diagnostic::Limits::default();limits.max_depth=usize::from(maximum_depth);self.control.borrow_mut().scoped_maximum(maximum,|control|Ok::<_,ValueError>(semio_framework_dsl_record::parse_expr_text_controlled(text,&limits,control).map_err(PackRefusal::from))).map_err(PackRefusal::from)?}
    fn copy_utf8(&self,bytes:&[u8],_offset:u64,_what:&'static str)->Result<String,PackRefusal>{self.check(bytes.len() as u64)?;let mut control=self.control.borrow_mut();if bytes.len()>control.maximum_bytes().saturating_sub(control.owned_bytes()){return Err(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"native text exceeds caller allowance")));}let value=control.borrow_text(bytes).map_err(PackRefusal::from)?;control.copy_text(value).map_err(PackRefusal::from)}
    fn copy_span_utf8(&self,bytes:semio_framework_replication_codec::ByteSpan<'_>,offset:u64,what:&'static str)->Result<String,PackRefusal>{
        if let Some(bytes)=bytes.contiguous(){return self.copy_utf8(bytes,offset,what)}
        self.check(bytes.len()as u64)?;
        {let control=self.control.borrow();if bytes.len()>control.maximum_bytes().saturating_sub(control.owned_bytes()){return Err(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"native text exceeds caller allowance")))}}
        copy_validated_span_utf8(self,bytes,offset,what)
    }
}

fn copy_decoded_string(value:&str,materialization:&dyn MaterializationAdmission)->Result<String,PackRefusal>{materialization.copy_text(value)}

impl DecCtx<'_> {
    pub(super) fn check_items(&self,n:u64)->Result<(),PackRefusal>{if n>self.limits.max_items{return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "item count exceeds max_items" });}Ok(())}
    pub(super) fn value_slots<T>(&self,count:u64)->Result<Vec<T>,PackRefusal>{self.check_items(count)?;self.materialization.charge(count.checked_mul(size_of::<T>() as u64).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "wire value slot overflow" })?)?;let mut output=Vec::new();output.try_reserve_exact(usize::try_from(count).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "wire value slot count" })?).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::AllocationFailed, limit: "wire value slot allocation" })?;Ok(output)}
    pub(super) fn record_slots(&self,record:&mut RecordValue,count:u64)->Result<(),PackRefusal>{self.check_items(count)?;let count=usize::try_from(count).map_err(|_|PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"record slot count exceeds address space")))?.max(record.fields.len());if count<=record.fields.capacity(){return Ok(());}let bytes=count.checked_mul(size_of::<(u16,FieldValue)>()).ok_or_else(||PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"record slot storage overflow")))?;self.materialization.charge(bytes as u64)?;let mut entries=Vec::new();entries.try_reserve_exact(count).map_err(|_|PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::AllocationFailed,"record slot allocation failed")))?;record.fields.replace_empty_slots(entries);Ok(())}
}

pub(super) fn resolve_symref(ctx: &DecCtx<'_>, symref: u64) -> Result<String, PackRefusal> {
    let value = match &ctx.source {
        DecSource::File(pack_file) => pack_file.symbol(symref)?,
        DecSource::Inline { symbols } => symbols.get(usize::try_from(symref).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::InvalidValue, limit: "symbol reference index" })?).map(String::as_str).ok_or_else(|| PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue,
            what: "symref",
            offset: 0,
            detail: format!("symref {symref} out of range for inline table of {}", symbols.len()),
        })?,
    };
    copy_decoded_string(value, ctx.materialization)
}

/// 📏️ Reads a `varint` length then that many raw bytes, rejecting an oversized length
/// against `limits.max_segment_len` BEFORE allocating/slicing.
fn read_len_prefixed_bytes<'b>(reader: &mut ByteReader<'b>, limits: &PackLimits) -> Result<semio_framework_replication_codec::ByteSpan<'b>, PackRefusal> {
    let len = reader.read_varint_u64()?;
    if len > limits.max_segment_len {
        return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "inline blob length exceeds max_segment_len" });
    }
    reader.read_span(usize::try_from(len).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "inline blob byte length" })?)
}

fn read_inline_string(reader: &mut ByteReader<'_>, ctx: &DecCtx<'_>) -> Result<String, PackRefusal> {
    let bytes = read_len_prefixed_bytes(reader, &ctx.limits)?;
    let offset = reader.position() as u64;
    ctx.materialization.copy_span_utf8(bytes,offset,"text")
}

fn read_inline_bytes(reader: &mut ByteReader<'_>, ctx: &DecCtx<'_>) -> Result<Vec<u8>, PackRefusal> {
    let bytes=read_len_prefixed_bytes(reader,&ctx.limits)?;
    if bytes.len()as u64>ctx.limits.max_total_alloc{return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "decoded octets exceed max_total_alloc" });}
    ctx.materialization.copy_span_bytes(bytes)
}

/// 🧱️ Reads `count` chunk ids and concatenates their decoded (and, per `verification`,
/// integrity-checked) content via the open `PackFile`'s chunk table.
fn read_chunked_bytes(reader: &mut ByteReader<'_>, ctx: &DecCtx<'_>) -> Result<Vec<u8>, PackRefusal> {
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    if ctx.materialization.controlled(){
        let DecSource::File(file)=&ctx.source else{return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::UnsupportedOwner,what:"chunk_id",offset:reader.position() as u64,detail:"chunked octets require an owned chunk catalog".into()});};let mut sizes=reader.fork();let mut length=0u64;
        for _ in 0..count{ctx.materialization.step()?;let raw_id=sizes.read_varint_u64()?;let id=u32::try_from(raw_id).map_err(|_|PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"chunk_id",offset:sizes.position() as u64,detail:"chunk id exceeds u32".into()})?;let bytes=file.chunk_decoded_len(ChunkId(id))?;if bytes>ctx.limits.max_segment_len{return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "chunk length exceeds max_segment_len" });}length=length.checked_add(bytes).filter(|length|*length<=ctx.limits.max_total_alloc).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "controlled octet length exceeds caller limits" })?;}
        let length=usize::try_from(length).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "chunk octets exceed address space" })?;let mut output=ctx.materialization.octet_buffer(length)?;
        for _ in 0..count{ctx.materialization.step()?;let id=u32::try_from(reader.read_varint_u64()?).map_err(|_|PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"chunk_id",offset:reader.position() as u64,detail:"chunk id exceeds u32".into()})?;ctx.materialization.append_chunk(file,ChunkId(id),ctx.verification,&mut output)?;}
        if output.len()!=length{return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvariantViolated,what:"chunks",offset:reader.position() as u64,detail:"chunk output differs from admitted length".into()});}return Ok(output);
    }
    let mut out = Vec::new();
    for _ in 0..count {
        let id = reader.read_varint_u64()?;
        if id > u32::MAX as u64 {
            return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "chunk_id", offset: reader.position() as u64, detail: "chunk id exceeds u32".to_string() });
        }
        let piece = match &ctx.source {
            DecSource::File(pack_file) => {
                let length=pack_file.chunk_decoded_len(ChunkId(id as u32))?;
                let total=(out.len()as u64).checked_add(length).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "decoded octet length overflow" })?;
                if total>ctx.limits.max_total_alloc||usize::try_from(total).is_err(){return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "decoded octets exceed max_total_alloc" });}
                ctx.materialization.charge(length)?;
                ::semio_framework_async::poll::resolve_ready(pack_file.read_chunk(ChunkId(id as u32),ctx.verification))?
            },
            DecSource::Inline { .. } => {
                return Err(PackRefusal::Malformed { kind: ValueRefusalKind::UnsupportedOwner, what: "chunk_id", offset: reader.position() as u64, detail: "chunked bytes are not representable in a container-less record body".to_string() });
            }
        };
        let next=out.len().checked_add(piece.len()).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "decoded octet length overflow" })?;
        if next as u64>ctx.limits.max_total_alloc{return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "decoded octets exceed max_total_alloc" });}
        out.try_reserve_exact(piece.len()).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::AllocationFailed, limit: "decoded octet allocation" })?;
        out.extend_from_slice(&piece);
    }
    Ok(out)
}

/// 📖️ Reads one self-describing string value (`TAG_STR` or `TAG_STR_INLINE`) — used for
/// `Map`/object keys and `DslValue::String`, where the tag itself (not any external shape) is
/// what disambiguates interned vs inline.
fn decode_string(reader: &mut ByteReader<'_>, ctx: &DecCtx<'_>) -> Result<String, PackRefusal> {
    let tag = reader.read_u8()?;
    match tag {
        TAG_STR => {
            let idx = reader.read_varint_u64()?;
            resolve_symref(ctx, idx)
        }
        TAG_STR_INLINE => read_inline_string(reader, ctx),
        other => Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "string", offset: reader.position() as u64, detail: format!("expected a string tag, found {other:#04x}") }),
    }
}

/// 🧾️ Decodes one record's fields: `field_count, (field_id, value)*`. Any field id not
/// found in `spec` is decoded generically (`shape = None`) and reported into
/// `ctx.unknown_field_ids`; when `ctx.preserve_unknown` is `false` it is still consumed (to stay
/// byte-aligned) but dropped from the returned `RecordValue`. Every `spec` field not seen on the
/// wire is inserted as `Absent` — the decode-side half of canonical mode's "omit `Absent`" rule.
pub(super) fn decode_record_fields(reader: &mut ByteReader<'_>, spec: Option<&RecordSpec>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<RecordValue, PackRefusal> {
    ctx.materialization.step()?;
    check_depth(ctx.limits.max_depth, depth)?;
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    let mut record = RecordValue::default();
    ctx.record_slots(&mut record,count)?;
    for _ in 0..count {
        let id_raw = reader.read_varint_u64()?;
        if id_raw > u16::MAX as u64 {
            return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "field_id", offset: reader.position() as u64, detail: "field id exceeds u16".to_string() });
        }
        let id = id_raw as u16;
        let field_shape = spec.and_then(|s| s.fields.iter().find(|f| f.id == id)).map(|f| &f.shape);
        let value = decode_value(reader, field_shape, ctx, depth + 1)?;
        if field_shape.is_none() {
            ctx.materialization.charge(8)?;
            ctx.unknown_field_ids.try_reserve_exact(1).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::AllocationFailed, limit: "unknown field report allocation" })?;
            ctx.unknown_field_ids.push(id);
            if ctx.preserve_unknown {
                record.fields.insert(id, value);
            }
        } else {
            record.fields.insert(id, value);
        }
    }
    if let Some(spec) = spec {
        let missing=spec.fields.iter().filter(|field|!record.fields.contains_key(&field.id)).count();
        let count=record.fields.len().checked_add(missing).ok_or_else(||PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"native record default storage overflow")))?;
        ctx.record_slots(&mut record,count as u64)?;
        for field in &spec.fields {
            if !record.fields.contains_key(&field.id){record.fields.insert(field.id,FieldValue::Absent);}
        }
    }
    Ok(record)
}

/// 📖️ Decodes one tag-prefixed value. `shape`, when known, disambiguates `Tuple` vs
/// `List`, resolves nested `Record`/`Block`/`Statements`/`Map` sub-shapes, and reinterprets
/// `PackedVarint` payloads as `UInt`/`Enum` where the shape says so; `None` decodes generically
/// straight from the wire tag — every tag is self-describing enough for this to always succeed,
/// which is what makes unknown-field decode possible without the original schema.
// 🔁️ Mutually recursive with `decode_record_fields`/`decode_seq_body`/`decode_map` (and directly
// self-recursive for `TAG_BLOCK`) — same `Box::pin(...).await` requirement as `encode_value`.
pub(super) fn decode_value(reader: &mut ByteReader<'_>, shape: Option<&Shape>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<FieldValue, PackRefusal> {
    ctx.materialization.step()?;
    check_depth(ctx.limits.max_depth, depth)?;
    let tag = reader.read_u8()?;
    match tag {
        TAG_ABSENT => Ok(FieldValue::Absent),
        TAG_FALSE => Ok(FieldValue::Bool(false)),
        TAG_TRUE => Ok(FieldValue::Bool(true)),
        TAG_INT => Ok(FieldValue::Int(reader.read_varint_i64()?)),
        TAG_UINT => Ok(FieldValue::UInt(reader.read_varint_u64()?)),
        TAG_F64 => Ok(FieldValue::Float(reader.read_f64_le()?)),
        TAG_STR => {
            let idx = reader.read_varint_u64()?;
            Ok(FieldValue::Text(resolve_symref(ctx, idx)?))
        }
        TAG_STR_INLINE => Ok(FieldValue::Text(read_inline_string(reader, ctx)?)),
        TAG_BYTES => Ok(FieldValue::Bytes64(read_inline_bytes(reader, ctx)?)),
        TAG_BYTES_CHUNKED => Ok(FieldValue::Bytes64(read_chunked_bytes(reader, ctx)?)),
        TAG_ENUM => Ok(FieldValue::Enum(reader.read_varint_u64()? as u32)),
        TAG_TUPLE => decode_seq_body(reader, elem_shape_of(shape), true, ctx, depth),
        TAG_LIST => {
            if let Some(spec_fn) = table_spec_of(shape) {
                let record_shape = Shape::Record(spec_fn);
                decode_seq_body(reader, Some(&record_shape), false, ctx, depth)
            } else {
                decode_seq_body(reader, elem_shape_of(shape), false, ctx, depth)
            }
        }
        TAG_RECORD => {
            let nested_spec = match shape{Some(Shape::Record(producer))=>Some(ctx.materialization.schema(producer)?),_=>None};
            Ok(FieldValue::Record(decode_record_fields(reader, nested_spec.as_ref(), ctx, depth + 1)?))
        }
        TAG_BLOCK => {
            ctx.materialization.charge(size_of::<FieldValue>() as u64)?;
            Ok(FieldValue::Block(Box::new(decode_value(reader, block_inner_shape(shape), ctx, depth + 1)?)))
        },
        TAG_STATEMENTS => decode_statements(reader, statements_variants(shape), ctx, depth),
        TAG_MAP => decode_map(reader, map_inner_shape(shape), ctx, depth),
        TAG_VALUE => Ok(FieldValue::Value(decode_dsl_value(reader, ctx, depth + 1)?)),
        TAG_WIRE => Ok(FieldValue::Wire(decode_wire(reader, ctx, depth + 1)?)),
        TAG_EXPR => {
            let text = decode_string(reader, ctx)?;
            let _offset = reader.position() as u64;
            ctx.materialization.expression(&text,ctx.limits.max_depth).map(FieldValue::Expr)
        }
        TAG_TABLE_SOA => Ok(FieldValue::List(decode_table_soa(reader, table_spec_of(shape), ctx, depth)?)),
        TAG_PACKED_F64 => decode_packed_f64_body(reader, is_tuple_shape(shape), ctx),
        TAG_PACKED_VARINT => decode_packed_varint_body(reader, elem_shape_of(shape).or(shape.filter(|s| !matches!(s, Shape::Tuple(_, _)))), is_tuple_shape(shape), ctx),
        TAG_NULL => Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "wire_tag", offset: reader.position() as u64, detail: "TAG_NULL is only valid inside a DslValue".to_string() }),
        other => Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "wire_tag", offset: reader.position() as u64, detail: format!("unrecognized tag {other:#04x}") }),
    }
}

/// 📚️ Decodes a plain (non-packed) `Tuple`/`List` body: `count, values*`.
fn decode_seq_body(reader: &mut ByteReader<'_>, elem_shape: Option<&Shape>, is_tuple: bool, ctx: &mut DecCtx<'_>, depth: u16) -> Result<FieldValue, PackRefusal> {
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    let mut items = ctx.value_slots(count)?;
    for _ in 0..count {
        items.push(decode_value(reader, elem_shape, ctx, depth + 1)?);
    }
    Ok(if is_tuple { FieldValue::Tuple(items) } else { FieldValue::List(items) })
}

fn decode_packed_f64_body(reader: &mut ByteReader<'_>, is_tuple: bool, ctx: &DecCtx<'_>) -> Result<FieldValue, PackRefusal> {
    let count = reader.read_varint_u64()?;
    let mut items = ctx.value_slots(count)?;
    for _ in 0..count {
        ctx.materialization.step()?;
        items.push(FieldValue::Float(reader.read_f64_le()?));
    }
    Ok(if is_tuple { FieldValue::Tuple(items) } else { FieldValue::List(items) })
}

/// 🔢️ Decodes a `PackedVarint` body. `elem_shape` (the field's `List(UInt)`/`List(Enum)`/
/// `Tuple(..)` element shape, when known) picks the reconstruction type; unknown context always
/// defaults to `Int`, which is also what makes an unknown field's homogeneous-`Int` list
/// re-encode to the exact same bytes (round-trip preserved even without the original schema).
fn decode_packed_varint_body(reader: &mut ByteReader<'_>, elem_shape: Option<&Shape>, is_tuple: bool, ctx: &DecCtx<'_>) -> Result<FieldValue, PackRefusal> {
    let count = reader.read_varint_u64()?;
    let mut items = ctx.value_slots(count)?;
    for _ in 0..count {
        ctx.materialization.step()?;
        let v = reader.read_varint_i64()?;
        let fv = match elem_shape {
            Some(Shape::UInt) => {
                if v < 0 {
                    return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "packed_varint", offset: reader.position() as u64, detail: "negative value under UInt shape".to_string() });
                }
                FieldValue::UInt(v as u64)
            }
            Some(Shape::Enum(_)) => {
                if v < 0 {
                    return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "packed_varint", offset: reader.position() as u64, detail: "negative value under Enum shape".to_string() });
                }
                FieldValue::Enum(v as u32)
            }
            _ => FieldValue::Int(v),
        };
        items.push(fv);
    }
    Ok(if is_tuple { FieldValue::Tuple(items) } else { FieldValue::List(items) })
}

fn decode_map(reader: &mut ByteReader<'_>, inner_shape: Option<&Shape>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<FieldValue, PackRefusal> {
    check_depth(ctx.limits.max_depth, depth)?;
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    let mut entries = ctx.value_slots(count)?;
    for _ in 0..count {
        let key = decode_string(reader, ctx)?;
        let value = decode_value(reader, inner_shape, ctx, depth + 1)?;
        entries.push((key, value));
    }
    Ok(FieldValue::Map(entries))
}

fn decode_statements(reader: &mut ByteReader<'_>, variants: Option<&Vec<(String, RecordSpecProducer)>>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<FieldValue, PackRefusal> {
    check_depth(ctx.limits.max_depth, depth)?;
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    let mut items = ctx.value_slots(count)?;
    for _ in 0..count {
        let symref = reader.read_varint_u64()?;
        let keyword = resolve_symref(ctx, symref)?;
        let spec = variants.and_then(|vs| vs.iter().find(|(k, _)| *k == keyword)).map(|(_, f)| ctx.materialization.schema(f)).transpose()?;
        let record = decode_record_fields(reader, spec.as_ref(), ctx, depth + 1)?;
        items.push((keyword, record));
    }
    Ok(FieldValue::Statements(items))
}

// 🔁️ Self-recursive (`TAG_LIST`/`TAG_MAP` arms) — boxed for the same reason as `decode_value`.
pub(super) fn decode_dsl_value(reader: &mut ByteReader<'_>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<DslValue, PackRefusal> {
    ctx.materialization.step()?;
    check_depth(ctx.limits.max_depth, depth)?;
    let tag = reader.read_u8()?;
    match tag {
        TAG_NULL => Ok(DslValue::Null),
        TAG_FALSE => Ok(DslValue::Bool(false)),
        TAG_TRUE => Ok(DslValue::Bool(true)),
        TAG_UINT => Ok(DslValue::Number(Number::UInt(reader.read_varint_u64()?))),
        TAG_INT => Ok(DslValue::Number(Number::Int(reader.read_varint_i64()?))),
        TAG_F64 => Ok(DslValue::Number(Number::Float(reader.read_f64_le()?))),
        TAG_STR => {
            let idx = reader.read_varint_u64()?;
            Ok(DslValue::String(resolve_symref(ctx, idx)?))
        }
        TAG_STR_INLINE => Ok(DslValue::String(read_inline_string(reader, ctx)?)),
        TAG_BYTES=>Ok(DslValue::Bytes(read_inline_bytes(reader,ctx)?)),
        TAG_BYTES_CHUNKED=>Ok(DslValue::Bytes(read_chunked_bytes(reader,ctx)?)),
        TAG_LIST => {
            let count = reader.read_varint_u64()?;
            let mut items = ctx.value_slots(count)?;
            for _ in 0..count {
                items.push(decode_dsl_value(reader, ctx, depth + 1)?);
            }
            Ok(DslValue::Array(items))
        }
        TAG_MAP => {
            let count = reader.read_varint_u64()?;
            let mut entries = ctx.value_slots(count)?;
            for _ in 0..count {
                let key = decode_string(reader, ctx)?;
                let value = decode_dsl_value(reader, ctx, depth + 1)?;
                entries.push((key, value));
            }
            Ok(DslValue::Object(entries))
        }
        other => Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "dsl_value", offset: reader.position() as u64, detail: format!("unexpected tag {other:#04x}") }),
    }
}

fn decode_wire(reader: &mut ByteReader<'_>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<WireValue, PackRefusal> {
    let presence = reader.read_u8()?;
    let from = decode_wire_node(reader, ctx)?;
    let edge = if presence & 0b01 != 0 {
        let directed = presence & 0b10 != 0;
        let to = decode_wire_node(reader, ctx)?;
        Some((directed, to))
    } else {
        None
    };
    let edge_label = if presence & 0b100 != 0 {
        let lp = reader.read_u8()?;
        let id = if lp & 0b01 != 0 { Some(decode_string(reader, ctx)?) } else { None };
        let kind = if lp & 0b10 != 0 { Some(decode_string(reader, ctx)?) } else { None };
        WireEdgeLabel { id, kind }
    } else {
        WireEdgeLabel::default()
    };
    let properties = decode_dsl_value(reader, ctx, depth + 1)?;
    Ok(WireValue { from, edge, edge_label, properties })
}

fn decode_wire_node(reader: &mut ByteReader<'_>, ctx: &mut DecCtx<'_>) -> Result<WireNode, PackRefusal> {
    ctx.materialization.step()?;
    let presence = reader.read_u8()?;
    let id = decode_string(reader, ctx)?;
    let kind = if presence & 0b01 != 0 { Some(decode_string(reader, ctx)?) } else { None };
    let port = if presence & 0b10 != 0 { Some(decode_string(reader, ctx)?) } else { None };
    Ok(WireNode { id, kind, port })
}
//#endregion 🔖️Decode
