//! 🧩️ OS product grammars and binary mutation dispatch over canonical Record and Value owners.

use semio_framework_dsl::LanguageRole;
use semio_framework_dsl::LanguageSpec;
use semio_framework_dsl::language;
use semio_framework_dsl::language_for_extension;
use semio_framework_dsl::language_for_role_extension;
use semio_framework_diagnostic::TextSpan;
use semio_framework_dsl::UnitSpec;
use semio_framework_dsl::unit_by_symbol;
// The derive macros emit `::crate::os_dsl::...` paths so generated code reads identically regardless of
// which technology crate invokes them. That only resolves for the crates that depend on `dsl` as
// an external crate — which is every real consumer, but NOT this crate's own tests (a crate is
// never its own dependency). `// extern crate self removed after merge` is the standard fix: it makes `::dsl`
// resolve to this crate even when the derive is exercised in-crate, as the `🧪️Tests` region below does.
// Only needed for the in-crate tests, so it's cfg-gated to avoid an "unused extern crate" warning
// in ordinary (non-test) builds, where every real consumer already has `dsl` as a true dependency.
// extern crate self removed after merge

use semio_framework_dsl::*;





use semio_framework_dsl_record::*;
use semio_framework_value::{DslValue,FromValue,ToValue,Number,ValueError,NativeDecodeControl,NativeEncodeControl};
#[cfg(test)]
use semio_framework_dsl_record_derive::{DslRecord,DslScalar,DslEnum};
#[cfg(test)]
use semio_framework_ui_viewport::{Viewport2d,Viewport3dOrbit};
pub use dsl_derive::{diff_binary,diff_text,DslArtifact,MutationLeaf,Mutations};




use semio_framework_value::ValueRefusalKind;



//#region 🏷️ProtocolRecord
/// 🏷️ The one source of a mutation vocabulary's op tags: the `record <kind> tag=<n>` lines of its
/// `💾️binary/📡️.protocol.semio`. Every codec derives its tags from here at compile time, so a kind whose
/// record is missing or duplicated fails the build instead of drifting from the wire.
/// See [`crate::os_dsl::grammar::parse_protocol`] for the full dialect this scanner agrees with.
pub mod protocol_record {
    const fn is_space(byte: u8) -> bool {
        byte == b' ' || byte == b'\t'
    }

    const fn is_name(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' || byte == b'.'
    }

    const fn skip_space(bytes: &[u8], mut at: usize) -> usize {
        while at < bytes.len() && is_space(bytes[at]) {
            at += 1;
        }
        at
    }

    const fn line_end(bytes: &[u8], mut at: usize) -> usize {
        while at < bytes.len() && bytes[at] != b'\n' {
            at += 1;
        }
        at
    }

    const fn starts_with(bytes: &[u8], at: usize, prefix: &[u8]) -> bool {
        if at + prefix.len() > bytes.len() {
            return false;
        }
        let mut index = 0;
        while index < prefix.len() {
            if bytes[at + index] != prefix[index] {
                return false;
            }
            index += 1;
        }
        true
    }

    /// 🔎️ Parses the record header at `line`: `(name start, name end, tag)`, or `None` for any other line.
    const fn record_at(bytes: &[u8], line: usize) -> Option<(usize, usize, u64)> {
        let at = skip_space(bytes, line);
        if !starts_with(bytes, at, b"record") || at + 6 >= bytes.len() || !is_space(bytes[at + 6]) {
            return None;
        }
        let name_start = skip_space(bytes, at + 6);
        let mut name_end = name_start;
        while name_end < bytes.len() && is_name(bytes[name_end]) {
            name_end += 1;
        }
        let at = skip_space(bytes, name_end);
        if name_end == name_start || !starts_with(bytes, at, b"tag=") {
            return None;
        }
        let mut at = at + 4;
        let digits = at;
        let mut tag: u64 = 0;
        while at < bytes.len() && bytes[at].is_ascii_digit() {
            tag = tag * 10 + (bytes[at] - b'0') as u64;
            at += 1;
        }
        if at == digits {
            return None;
        }
        Some((name_start, name_end, tag))
    }

    const fn name_equals(bytes: &[u8], start: usize, end: usize, kind: &[u8]) -> bool {
        end - start == kind.len() && starts_with(bytes, start, kind)
    }

    /// 🔢️ `(tag, occurrences)` of `kind` across every record line.
    const fn scan(protocol: &str, kind: &str) -> (u64, usize) {
        let bytes = protocol.as_bytes();
        let kind = kind.as_bytes();
        let mut line = 0;
        let mut found = 0;
        let mut tag = 0;
        while line < bytes.len() {
            if let Some((start, end, value)) = record_at(bytes, line) {
                if name_equals(bytes, start, end, kind) {
                    found += 1;
                    tag = value;
                }
            }
            line = line_end(bytes, line) + 1;
        }
        (tag, found)
    }

    /// 🏷️ The tag `kind`'s record declares; a missing or duplicated record is a const-evaluation error.
    pub const fn tag(protocol: &str, kind: &str) -> u64 {
        match scan(protocol, kind) {
            (tag, 1) => tag,
            (_, 0) => panic!("📡️.protocol.semio declares no `record <kind> tag=<n>` for this mutation kind"),
            _ => panic!("📡️.protocol.semio declares this mutation kind more than once"),
        }
    }

    /// 🏷️ [`tag`] for a codec whose wire tag is one byte; a tag above 255 is a const-evaluation error.
    pub const fn tag_u8(protocol: &str, kind: &str) -> u8 {
        let tag = tag(protocol, kind);
        assert!(tag <= u8::MAX as u64, "📡️.protocol.semio record tag does not fit the codec's u8 tag field");
        tag as u8
    }

    /// 🏷️ [`tag`] for a codec whose wire tag is a `u32`; a tag above `u32::MAX` is a const-evaluation error.
    pub const fn tag_u32(protocol: &str, kind: &str) -> u32 {
        let tag = tag(protocol, kind);
        assert!(tag <= u32::MAX as u64, "📡️.protocol.semio record tag does not fit the codec's u32 tag field");
        tag as u32
    }

    /// 📇️ Every `(kind, tag)` record, in file order.
    pub fn records(protocol: &str) -> impl Iterator<Item = (&str, u64)> {
        let bytes = protocol.as_bytes();
        let mut line = 0;
        std::iter::from_fn(move || {
            while line < bytes.len() {
                let current = line;
                line = line_end(bytes, line) + 1;
                if let Some((start, end, tag)) = record_at(bytes, current) {
                    return Some((&protocol[start..end], tag));
                }
            }
            None
        })
    }

    /// 🔁️ The kind whose record declares `tag`.
    pub fn kind(protocol: &str, tag: u64) -> Option<&str> {
        records(protocol).find(|(_, value)| *value == tag).map(|(kind, _)| kind)
    }
}
//#endregion 🏷️ProtocolRecord

//#region 🔖️OpRt
/// 🎯️ Handcrafted OpBinary helper (P6): layout `format u8 (=1) | tag varint | record body`.
/// `encode_tagged_op`/`decode_tagged_op` take the tag from the vocabulary's `📡️.protocol.semio` record
/// ([`super::protocol_record`]); `encode_op`/`decode_op` serve the ephemeral layers that carry no wire
/// protocol facet, whose tag is the variant ordinal.
/// Called explicitly from handcrafted `protocol::OpBinary` impls — never re-emitted by derive.
pub mod variants_binary {
    use super::{protocol_record, DslVariants};
    use crate::os_pack::{decode_record_body_exact, encode_record_body, write_varint_u64, ByteReader, DecodeOptions, EncodeOptions};
    use crate::os_spr::ProtocolError;

    pub const OP_BINARY_FORMAT: u8 = 1;

    fn encode_with<T: DslVariants>(op: &T, tag_of: impl Fn(&str, usize) -> Result<u64, ProtocolError>) -> Result<Vec<u8>, ProtocolError> {
        let (keyword, record) = op.to_named_record();
        let variants = T::variants();
        let ordinal = variants.iter().position(|(k, _)| k == &keyword).ok_or(ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword '{keyword}' missing from variants()") })?;
        let tag = tag_of(&keyword, ordinal)?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = encode_record_body(&spec, &record, &EncodeOptions::default()).map_err(ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        write_varint_u64(&mut out, tag);
        out.extend_from_slice(&body);
        Ok(out)
    }

    fn encode_with_into<T:DslVariants>(op:&T,tag_of:impl Fn(&str,usize)->Result<u64,ProtocolError>,options:&EncodeOptions,output:&mut dyn protocol::io::binary::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ProtocolError>{
        protocol::io::binary::operation_bytes::with_operation_encode_policy(options,control,|control|{
        let mut limited=protocol::io::binary::operation_bytes::OperationByteLimitedOutput::new(output,options.limits.max_file_len);
        let output:&mut dyn protocol::io::binary::operation_bytes::OperationByteOutput=&mut limited;
        control.checkpoint().map_err(crate::os_pack::PackRefusal::from)?;
        let(keyword,ordinal,producer)=op.projected_variant_identity();
        let source=semio_framework_dsl_record::native_encoding::VariantProjection::new(op);
        let tag=tag_of(&keyword,ordinal)?;
        let spec=producer.encode(control).map_err(crate::os_pack::PackRefusal::from)?;
        output.write_bytes(&[OP_BINARY_FORMAT],control)?;
        let mut remaining=tag;
        let mut bytes=[0;10];
        let mut length=0;
        loop{bytes[length]=(remaining as u8)&127;remaining>>=7;if remaining!=0{bytes[length]|=128;}length+=1;if remaining==0{break;}}
        output.write_bytes(&bytes[..length],control)?;
        crate::os_pack::record::encode_projected_record_body_into(&spec,&source,options,output,control)?;
        Ok(())
        })
    }

    /// 🏷️ Appends a declared tagged operation into the same admitted source prefix.
    pub fn encode_tagged_op_into<T:DslVariants>(protocol:&str,op:&T,options:&EncodeOptions,output:&mut dyn protocol::io::binary::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ProtocolError>{
        encode_with_into(op,|keyword,_|protocol_record::records(protocol).find(|(kind,_)|*kind==keyword).map(|(_,tag)|tag).ok_or_else(||ProtocolError::Malformed{what:"op tag",offset:1,detail:format!("📡️.protocol.semio declares no record for '{keyword}'")}),options,output,control)
    }

    /// 🎞️ Appends the exact ordinal protocol header and direct canonical Record body.
    pub fn encode_op_into<T:DslVariants>(op:&T,options:&EncodeOptions,output:&mut dyn protocol::io::binary::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ProtocolError>{
        encode_with_into(op,|_,ordinal|u64::try_from(ordinal).map_err(|_|ProtocolError::Malformed{what:"op variant",offset:1,detail:format!("ordinal {ordinal} exceeds the u64 wire range")}),options,output,control)
    }
    fn decode_with_span<T:DslVariants>(source:crate::os_pack::ByteSpan<'_>,index_of:impl Fn(u64,&[(String,super::RecordSpecProducer)])->Result<usize,ProtocolError>,options:&DecodeOptions,canonical_options:&EncodeOptions,decoding:&mut semio_framework_value::NativeDecodeControl<'_>,encoding:&mut semio_framework_value::NativeEncodeControl<'_>,reencode:impl FnOnce(&T,&EncodeOptions,&mut dyn protocol::io::binary::operation_bytes::OperationByteOutput,&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ProtocolError>)->Result<T,ProtocolError>{
        decoding.checkpoint().map_err(crate::os_pack::PackRefusal::from)?;
        if source.len()as u64>options.limits.max_file_len{return Err(crate::os_pack::PackRefusal::LimitExceeded{kind:semio_framework_value::ValueRefusalKind::OwnershipLimit,limit:"operation source byte length"}.into());}
        let mut reader=ByteReader::from_span(source);
        let format=reader.read_u8()?;
        if format!=OP_BINARY_FORMAT{return Err(ProtocolError::Malformed{what:"op format",offset:0,detail:format!("unsupported op format {format}")});}
        let tag=reader.read_varint_u64()?;
        let variants=T::variants_controlled(decoding).map_err(crate::os_pack::PackRefusal::from)?;
        let index=index_of(tag,&variants)?;
        let(keyword,producer)=&variants[index];
        let spec=producer.decode(decoding).map_err(crate::os_pack::PackRefusal::from)?;
        let body=reader.read_span(reader.remaining())?;
        let record=crate::os_pack::record::decode_record_body_span_exact_controlled(body,&spec,options,decoding)?;
        let decoded=T::from_named_record_controlled(keyword,&record,decoding).map_err(crate::os_pack::PackRefusal::from)?;
        let mut comparison=protocol::io::binary::operation_bytes::OperationByteComparison::new(source);
        reencode(&decoded,canonical_options,&mut comparison,encoding)?;
        comparison.finish()?;
        Ok(decoded)
    }

    /// 🧾️ Decodes the same admitted source and compares its complete canonical tagged wire without a second byte owner.
    pub fn decode_tagged_op_span<T:DslVariants>(protocol:&str,source:crate::os_pack::ByteSpan<'_>,options:&DecodeOptions,canonical_options:&EncodeOptions,decoding:&mut semio_framework_value::NativeDecodeControl<'_>,encoding:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<T,ProtocolError>{
        decode_with_span(source,|tag,variants|{
            let kind=protocol_record::kind(protocol,tag).ok_or_else(||ProtocolError::Malformed{what:"op tag",offset:1,detail:format!("📡️.protocol.semio declares no record with tag {tag}")})?;
            variants.iter().position(|(keyword,_)|keyword==kind).ok_or_else(||ProtocolError::Malformed{what:"op tag",offset:1,detail:format!("record '{kind}' names no variant")})
        },options,canonical_options,decoding,encoding,|decoded,options,output,control|encode_tagged_op_into(protocol,decoded,options,output,control))
    }

    /// 🎞️ Decodes a borrowed operation and checks exact ordinal canonical bytes with caller controls.
    pub fn decode_op_span<T:DslVariants>(source:crate::os_pack::ByteSpan<'_>,options:&DecodeOptions,canonical_options:&EncodeOptions,decoding:&mut semio_framework_value::NativeDecodeControl<'_>,encoding:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<T,ProtocolError>{
        decode_with_span(source,|ordinal,variants|{
            let index=usize::try_from(ordinal).map_err(|_|ProtocolError::Malformed{what:"op variant",offset:1,detail:format!("ordinal {ordinal} exceeds the native index range")})?;
            if index<variants.len(){Ok(index)}else{Err(ProtocolError::Malformed{what:"op variant",offset:1,detail:format!("ordinal {ordinal} out of range for {} declared variants",variants.len())})}
        },options,canonical_options,decoding,encoding,encode_op_into)
    }

    fn decode_with<T: DslVariants>(bytes: &[u8], index_of: impl Fn(u64, &[(String, super::RecordSpecProducer)]) -> Result<usize, ProtocolError>, reencode: impl Fn(&T) -> Result<Vec<u8>, ProtocolError>) -> Result<T, ProtocolError> {
        let mut reader = ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let tag = reader.read_varint_u64()?;
        let variants = T::variants();
        let index = index_of(tag, &variants)?;
        let (keyword, spec_fn) = &variants[index];
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let record = decode_record_body_exact(body, &spec, &DecodeOptions::default()).map_err(ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        let decoded = T::from_named_record(keyword, &record).map_err(|error| ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })?;
        if reencode(&decoded)?.as_slice() != bytes {
            return Err(ProtocolError::Malformed { what: "op encoding", offset: 0, detail: "operation bytes are not canonical".into() });
        }
        Ok(decoded)
    }

    /// 🏷️ Encodes `op` with the tag its kind's record declares in `protocol`.
    pub fn encode_tagged_op<T: DslVariants>(protocol: &str, op: &T) -> Result<Vec<u8>, ProtocolError> {
        encode_with(op, |keyword, _| protocol_record::records(protocol).find(|(kind, _)| *kind == keyword).map(|(_, tag)| tag).ok_or(ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("📡️.protocol.semio declares no record for '{keyword}'") }))
    }

    /// 🏷️ Decodes an op whose tag names its kind's record in `protocol`.
    pub fn decode_tagged_op<T: DslVariants>(protocol: &str, bytes: &[u8]) -> Result<T, ProtocolError> {
        decode_with(
            bytes,
            |tag, variants| {
                let kind = protocol_record::kind(protocol, tag).ok_or(ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("📡️.protocol.semio declares no record with tag {tag}") })?;
                variants.iter().position(|(keyword, _)| keyword == kind).ok_or(ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("record '{kind}' names no variant") })
            },
            |decoded| encode_tagged_op(protocol, decoded),
        )
    }

    pub fn encode_op<T: DslVariants>(op: &T) -> Result<Vec<u8>, ProtocolError> {
        encode_with(op, |_, ordinal| u64::try_from(ordinal).map_err(|_| ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} exceeds the u64 wire range") }))
    }

    pub fn decode_op<T: DslVariants>(bytes: &[u8]) -> Result<T, ProtocolError> {
        decode_with(
            bytes,
            |ordinal, variants| {
                let index = usize::try_from(ordinal).map_err(|_| ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} exceeds the native index range") })?;
                if index < variants.len() { Ok(index) } else { Err(ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) }) }
            },
            |decoded| encode_op(decoded),
        )
    }
}
//#endregion 🔖️OpRt

//#region 🏷️TaggedValueRt
/// 🏷️ Op frame for a mutation aggregate whose payload is its `ToValue` tree: `format u8 (=1) | tag varint
/// | wire value (`pack_rt::encode_wire_value`) of the variant's value with its variant name removed`. The tag is
/// the variant kind's `record <kind> tag=<n>` in the vocabulary's `📡️.protocol.semio` ([`super::protocol_record`]),
/// so the wire never spells the variant name and the protocol file is the only source of tags.
pub mod tagged_value_binary {
    use super::protocol_record;
    use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
    use crate::os_pack::{write_varint_u64, ByteReader};
    use crate::os_spr::ProtocolError;
    use crate::os_store::pack_rt::{decode_wire_value, encode_wire_value};

    pub const OP_BINARY_FORMAT: u8 = 1;

    /// 🧭️ Where the aggregate's `ToValue` tree names its variant.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VariantTag {
        /// `#[value(tag = "…")]`, camelCase variant names, with or without `content`.
        Field(&'static str),
        /// Externally tagged, PascalCase variant names: `{"Variant": payload}`.
        Key,
    }

    fn malformed(what: &'static str, offset: u64, detail: String) -> ProtocolError {
        ProtocolError::Malformed { what, offset, detail }
    }

    fn kebab(name: &str) -> String {
        let mut out = String::with_capacity(name.len() + 4);
        for (index, ch) in name.char_indices() {
            if ch.is_ascii_uppercase() {
                if index > 0 {
                    out.push('-');
                }
                out.push(ch.to_ascii_lowercase());
            } else {
                out.push(ch);
            }
        }
        out
    }

    fn cased(kind: &str, pascal: bool) -> String {
        let mut out = String::with_capacity(kind.len());
        let mut upper = pascal;
        for ch in kind.chars() {
            if ch == '-' {
                upper = true;
            } else if upper {
                out.push(ch.to_ascii_uppercase());
                upper = false;
            } else {
                out.push(ch);
            }
        }
        out
    }

    /// 🏷️ Encodes `op` under its kind's record tag.
    pub fn encode_op<T: ToValue>(protocol: &str, tagging: VariantTag, op: &T) -> Result<Vec<u8>, ProtocolError> {
        let semio_framework_value::DslValue::Object(mut entries) = op.to_value() else { return Err(malformed("op value", 0, "a mutation aggregate's value must be an object".into())) };
        let (variant, payload) = match tagging {
            VariantTag::Field(key) => {
                let position = entries.iter().position(|(name, _)| name == key).ok_or_else(|| malformed("op value", 0, format!("value carries no `{key}` variant field")))?;
                let (_, variant) = entries.remove(position);
                let semio_framework_value::DslValue::String(variant) = variant else { return Err(malformed("op value", 0, format!("`{key}` is not a string"))) };
                (variant, semio_framework_value::DslValue::Object(entries))
            }
            VariantTag::Key => {
                let mut entries = entries.into_iter();
                let (Some((variant, payload)), None) = (entries.next(), entries.next()) else { return Err(malformed("op value", 0, "an externally tagged value must hold exactly one variant".into())) };
                (variant, payload)
            }
        };
        let kind = kebab(&variant);
        let tag = protocol_record::records(protocol).find(|(record, _)| *record == kind).map(|(_, tag)| tag).ok_or_else(|| malformed("op tag", 1, format!("📡️.protocol.semio declares no record for '{kind}'")))?;
        let body = encode_wire_value(&payload);
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        write_varint_u64(&mut out, tag);
        out.extend_from_slice(&body);
        Ok(out)
    }

    /// 🏷️ Decodes an op whose tag names its kind's record.
    pub fn decode_op<T: FromValue>(protocol: &str, tagging: VariantTag, bytes: &[u8]) -> Result<T, ProtocolError> {
        let mut reader = ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(malformed("op format", 0, format!("unsupported op format {format}")));
        }
        let tag = reader.read_varint_u64()?;
        let kind = protocol_record::kind(protocol, tag).ok_or_else(|| malformed("op tag", 1, format!("📡️.protocol.semio declares no record with tag {tag}")))?;
        let offset = reader.position();
        let payload = decode_wire_value(&bytes[offset..]).map_err(|error| malformed("op payload", offset as u64, error.to_string()))?;
        let value = match tagging {
            VariantTag::Field(key) => {
                let semio_framework_value::DslValue::Object(mut entries) = payload else { return Err(malformed("op payload", offset as u64, "payload must be an object".into())) };
                entries.insert(0, (key.to_string(), semio_framework_value::DslValue::String(cased(kind, false))));
                semio_framework_value::DslValue::Object(entries)
            }
            VariantTag::Key => semio_framework_value::DslValue::Object(vec![(cased(kind, true), payload)]),
        };
        T::from_value(value).map_err(|error| malformed("op value", offset as u64, error.to_string()))
    }
}
//#endregion 🏷️TaggedValueRt

//#region 🏷️TaggedTextRt
/// 🏷️ Op frame for a mutation aggregate whose canonical payload is its own `OpText` line `<kind> <args>`:
/// `format u8 (=1) | tag varint | args utf-8`. The tag is the kind's `record <kind> tag=<n>`, so the keyword never
/// travels and the protocol file stays the only source of tags. Used where the `ToValue` tree is lossy.
pub mod tagged_text_binary {
    use super::protocol_record;
    use crate::os_pack::{write_varint_u64, ByteReader};
    use crate::os_spr::ProtocolError;

    pub const OP_BINARY_FORMAT: u8 = 1;

    /// 🏷️ Encodes one printed op line under its keyword's record tag.
    pub fn encode_line(protocol: &str, line: &str) -> Result<Vec<u8>, ProtocolError> {
        let (keyword, args) = line.split_once(' ').unwrap_or((line, ""));
        let tag = protocol_record::records(protocol).find(|(kind, _)| *kind == keyword).map(|(_, tag)| tag).ok_or_else(|| ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("📡️.protocol.semio declares no record for '{keyword}'") })?;
        let mut out = Vec::with_capacity(args.len() + 3);
        out.push(OP_BINARY_FORMAT);
        write_varint_u64(&mut out, tag);
        out.extend_from_slice(args.as_bytes());
        Ok(out)
    }

    /// 🏷️ Restores the op line whose keyword the tag's record names.
    pub fn decode_line(protocol: &str, bytes: &[u8]) -> Result<String, ProtocolError> {
        let mut reader = ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let tag = reader.read_varint_u64()?;
        let kind = protocol_record::kind(protocol, tag).ok_or_else(|| ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("📡️.protocol.semio declares no record with tag {tag}") })?;
        let offset = reader.position();
        let args = std::str::from_utf8(&bytes[offset..]).map_err(|error| ProtocolError::Malformed { what: "op args", offset: offset as u64, detail: error.to_string() })?;
        Ok(if args.is_empty() { kind.to_string() } else { format!("{kind} {args}") })
    }
}
//#endregion 🏷️TaggedTextRt

/// 🔍️ Resolves a registered language from `.semio` file bytes (content-derived envelope).
/// Text components (`dsl`/`op`) prefer grammar registrations; binary components (`pack`/`spr`)
/// prefer protocol registrations.
// 🚫️async: E1 pure accessor — see `language_registry` above
pub fn language_for_semio_content(bytes: &[u8]) -> Option<LanguageSpec> {
    let envelope = semio_format::sniff(bytes).ok()?;
    let base = envelope.envelope_id();
    let plugin = envelope.plugin.as_str();
    let artifact = envelope.artifact.as_str();
    match envelope.component {
        semio_format::Component::Dsl => language(&base).or_else(|| language_for_extension(artifact)).or_else(|| language_for_extension(plugin)),
        semio_format::Component::Op => language_for_suffix_candidates(&base, plugin, artifact, "op").or_else(|| {
            language_for_role_extension(LanguageRole::Ops, artifact)
        }),
        semio_format::Component::Pack => language_for_suffix_candidates(&base, plugin, artifact, "pack").or_else(|| language(&base).filter(|s| s.protocol.is_some())),
        semio_format::Component::Spr => language_for_suffix_candidates(&base, plugin, artifact, "spr"),
        _ => None,
    }
}

// 🚫️async: E1 pure accessor — see `language_registry` above
fn language_for_suffix_candidates(base: &str, plugin: &str, artifact: &str, suffix: &str) -> Option<LanguageSpec> {
    language(&format!("{base}.{suffix}")).or_else(|| language(&format!("{plugin}.{suffix}"))).or_else(|| language(&format!("{artifact}.{suffix}"))).or_else(|| language(&format!("{plugin}.{artifact}.{suffix}")))
}
//#endregion 🔖️Idiom



//#region 🧪️Tests
#[cfg(test)]
#[path="🔢️ieee754/🧪️tests/🦀️.rs"]
mod ieee_payload_tests;

#[cfg(test)]
#[path = "🧪️tests/📦️boxed-fields/🦀️.rs"]
mod boxed_field_tests;

#[cfg(test)]
#[path = "🧪️tests/🔢️checked-integers/🦀️.rs"]
mod checked_integer_tests;

#[cfg(test)]
#[path = "🧪️tests/🏷️protocol-record/🦀️.rs"]
mod protocol_record_tests;

#[cfg(test)]
#[path = "🧪️tests/🧪️hygienic-bindings/🦀️.rs"]
mod hygienic_binding_tests;


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests







#[cfg(test)]
#[path = "🧬️schema/🧪️tests/🧬️intrinsic-bytes/🦀️.rs"]
mod canonical_record_intrinsic_consumers;

#[cfg(test)]
#[path = "🧬️schema/🧪️tests/🧾️record-list/🦀️.rs"]
mod canonical_record_list_consumers;

#[cfg(test)]
#[path = "🧬️schema/🏭️producer/🧪️tests/🦀️.rs"]
mod canonical_record_producer_consumers;

#[cfg(test)]
#[path = "🪟️viewport/🧪️tests/🪟️poses/🦀️.rs"]
mod canonical_viewport_pack_consumers;
