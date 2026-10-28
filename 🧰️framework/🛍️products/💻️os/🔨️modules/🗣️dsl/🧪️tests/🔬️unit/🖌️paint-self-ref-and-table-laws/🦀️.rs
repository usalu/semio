/// 🖌️ Paint, self-reference and table laws: the optional statements and block record fields, the paint operations, the boxed required field, the self-referential record struct and the compact structure-of-arrays table field.

// --- end-to-end derive test: `#[dsl(statements, block)] Option<T>` (the `draw` pilot's
// `attributes.fill: Option<FillStyle>` shape) — a sum-type scalar field, not a collection ---

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, serde::Serialize, serde::Deserialize)]
enum PaintStyle {
    #[dsl(key = "solid")]
    Solid { color: [f64; 4] },
    #[dsl(key = "gradient")]
    Gradient { stops: Vec<f64> },
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, serde::Serialize, serde::Deserialize)]
struct PaintAttributes {
    #[dsl(statements, block)]
    fill: Option<PaintStyle>,
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, DslArtifact, serde::Serialize, serde::Deserialize)]
#[artifact(id = "paint.doc", extension = "paintdoc")]
struct PaintDocument {
    #[dsl(block)]
    attributes: PaintAttributes,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).
impl pack::snapshot::ArtifactDsl for PaintDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match semio_framework_os_kernel::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = parse(body, &Self::__dsl_spec(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let record = self.__dsl_to_record();
        let body = print(&record, &Self::__dsl_spec(), JoinMode::Document);
        let envelope = semio_framework_os_kernel::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        semio_framework_os_kernel::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl pack::snapshot::ArtifactPack for PaintDocument {
    fn encode_pack_with(&self, options: &semio_framework_os_kernel::os_store::PackEncodeOptions) -> Result<Vec<u8>, semio_framework_os_kernel::os_store::PackError> {
        let inner = semio_framework_os_kernel::os_pack::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            semio_framework_os_kernel::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::semio_format::Component::Pack, 1).map_err(|error| semio_framework_os_kernel::os_store::PackError::from(error.into_value_error()))?;
        Ok(semio_framework_os_kernel::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &semio_framework_os_kernel::os_store::PackDecodeOptions) -> Result<Self, semio_framework_os_kernel::os_store::PackError> {
        let (envelope, inner) = semio_framework_os_kernel::os_store::semio_format::unwrap_binary(bytes).map_err(|error| semio_framework_os_kernel::os_store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::semio_format::Component::Pack, 1) {
            return Err(semio_framework_os_kernel::os_store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = semio_framework_os_kernel::os_pack::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(semio_framework_os_kernel::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for PaintDocument {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}


}
//#endregion 🔖️ArtifactCodec

#[semio_framework_async_macros::async_test]
async fn derived_option_statements_field_round_trips_present_and_absent() {
    let with_fill = PaintDocument { attributes: PaintAttributes { fill: Some(PaintStyle::Solid { color: [1.0, 0.0, 0.0, 1.0] }) } };
    let printed = <PaintDocument as pack::snapshot::ArtifactDsl>::print_dsl(&with_fill);
    let parsed = <PaintDocument as pack::snapshot::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, with_fill, "Some(..) round trip diverged;\nprinted:\n{printed}");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&with_fill);

    let no_fill = PaintDocument { attributes: PaintAttributes { fill: None } };
    let printed_none = <PaintDocument as pack::snapshot::ArtifactDsl>::print_dsl(&no_fill);
    let parsed_none = <PaintDocument as pack::snapshot::ArtifactDsl>::parse_dsl(&printed_none).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed_none}"));
    assert_eq!(parsed_none, no_fill, "None round trip diverged;\nprinted:\n{printed_none}");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&no_fill);
}

// --- regression: `#[dsl(block)] Option<PlainRecord>` (the `draw` pilot's `attributes.stroke:
// Option<StrokeStyle>` shape) — `None` must OMIT the field, not print empty `{ }` braces, since
// reparsing empty braces would otherwise try to build a record whose required fields are absent ---

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, serde::Serialize, serde::Deserialize)]
struct BrushStyle {
    color: [f64; 4],
    width: f64,
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, DslArtifact, serde::Serialize, serde::Deserialize)]
#[artifact(id = "art.doc", extension = "brushdoc")]
struct BrushDocument {
    #[dsl(block)]
    brush: Option<BrushStyle>,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).
impl pack::snapshot::ArtifactDsl for BrushDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match semio_framework_os_kernel::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = parse(body, &Self::__dsl_spec(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let record = self.__dsl_to_record();
        let body = print(&record, &Self::__dsl_spec(), JoinMode::Document);
        let envelope = semio_framework_os_kernel::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        semio_framework_os_kernel::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl pack::snapshot::ArtifactPack for BrushDocument {
    fn encode_pack_with(&self, options: &semio_framework_os_kernel::os_store::PackEncodeOptions) -> Result<Vec<u8>, semio_framework_os_kernel::os_store::PackError> {
        let inner = semio_framework_os_kernel::os_pack::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            semio_framework_os_kernel::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::semio_format::Component::Pack, 1).map_err(|error| semio_framework_os_kernel::os_store::PackError::from(error.into_value_error()))?;
        Ok(semio_framework_os_kernel::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &semio_framework_os_kernel::os_store::PackDecodeOptions) -> Result<Self, semio_framework_os_kernel::os_store::PackError> {
        let (envelope, inner) = semio_framework_os_kernel::os_store::semio_format::unwrap_binary(bytes).map_err(|error| semio_framework_os_kernel::os_store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::semio_format::Component::Pack, 1) {
            return Err(semio_framework_os_kernel::os_store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = semio_framework_os_kernel::os_pack::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(semio_framework_os_kernel::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for BrushDocument {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}


}
//#endregion 🔖️ArtifactCodec

#[semio_framework_async_macros::async_test]
async fn derived_option_block_record_field_omits_rather_than_printing_empty_braces_when_absent() {
    let with_brush = BrushDocument { brush: Some(BrushStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.5 }) };
    let printed = <BrushDocument as pack::snapshot::ArtifactDsl>::print_dsl(&with_brush);
    let parsed = <BrushDocument as pack::snapshot::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, with_brush, "Some(..) round trip diverged;\nprinted:\n{printed}");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&with_brush);

    let no_brush = BrushDocument { brush: None };
    let printed_none = <BrushDocument as pack::snapshot::ArtifactDsl>::print_dsl(&no_brush);
    assert!(!printed_none.contains("brush"), "an absent block-wrapped Option<Record> must be omitted entirely, not printed as empty braces: {printed_none:?}");
    let parsed_none = <BrushDocument as pack::snapshot::ArtifactDsl>::parse_dsl(&printed_none).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed_none}"));
    assert_eq!(parsed_none, no_brush, "None round trip diverged;\nprinted:\n{printed_none:?}");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&no_brush);
}

// --- end-to-end derive test: `#[dsl(statements)] Box<T>` (the `draw` pilot's
// `AddLayer { layer: Box<DrawLayerNode> }` shape) — exactly one required tagged value ---

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, serde::Serialize, serde::Deserialize)]
enum PaintOp {
    #[dsl(key = "addShape")]
    AddShape {
        #[dsl(statements)]
        shape: Box<ShapeNode>,
    },
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl semio_framework_os_kernel::os_spr::OpText for PaintOp {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = parse(line, &(spec_fn.ordinary)(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline })?;
                return <Self as DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown operation line '{line}'"),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as DslVariants>::to_named_record(self);
        let variants = <Self as DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        print(&record, &(spec_fn.ordinary)(), JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl semio_framework_os_kernel::os_spr::OpBinary for PaintOp {
    fn encode_op(&self) -> Result<Vec<u8>, semio_framework_os_kernel::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as DslVariants>::to_named_record(self);
        let variants = <Self as DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(semio_framework_os_kernel::os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = semio_framework_os_kernel::os_pack::record::encode_record_body(&spec, &record, &semio_framework_os_kernel::os_store::PackEncodeOptions::default()).map_err(semio_framework_os_kernel::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        semio_framework_os_kernel::os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, semio_framework_os_kernel::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = semio_framework_os_kernel::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(semio_framework_os_kernel::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(semio_framework_os_kernel::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let record_offset = reader.position() as u64;
        let body = &bytes[record_offset as usize..];
        let (record, _report) = semio_framework_os_kernel::os_pack::record::decode_record_body(body, &spec, &semio_framework_os_kernel::os_store::PackDecodeOptions::default()).map_err(semio_framework_os_kernel::os_spr::ProtocolError::from)?;
        <Self as DslVariants>::from_named_record(keyword, &record).map_err(|error| semio_framework_os_kernel::os_spr::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec

#[semio_framework_async_macros::async_test]
async fn derived_required_statements_boxed_field_round_trips() {
    let op = PaintOp::AddShape { shape: Box::new(ShapeNode::Circle(CircleBody { id: "c1".to_string(), r: 2.0 })) };
    let printed = <PaintOp as semio_framework_os_kernel::os_spr::OpText>::print_op(&op);
    assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
    let parsed = <PaintOp as semio_framework_os_kernel::os_spr::OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
    assert_eq!(parsed, op, "boxed required-statements round trip diverged for {printed:?}");
}

// --- regression: a genuinely self-referential `#[derive(DslRecord)]` STRUCT (not an enum) —
// a field whose type recurses back to the struct itself, e.g. a dynamic-value type with a
// nested-dictionary-of-itself field (the `imperative` pilot's `ValueDsl` shape). Unlike
// `Shape::Statements` (already lazy), `Shape::Record` used to eagerly call `Self::__dsl_spec()`
// to build its own shape, which itself built its "dict" field's shape by calling
// `Self::__dsl_spec()` again — infinite recursion just constructing the spec, stack overflow
// before a single byte of real data was ever touched. Now lazy (a `fn() -> RecordSpec` pointer,
// mirroring `Statements`), so this must round trip a genuinely nested value correctly.

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, serde::Serialize, serde::Deserialize)]
struct SelfRefValue {
    #[dsl(key = "n")]
    number: Option<i64>,
    #[dsl(key = "dict")]
    dictionary: Option<std::collections::BTreeMap<String, SelfRefValue>>,
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, DslArtifact, serde::Serialize, serde::Deserialize)]
#[artifact(id = "selfref.doc", extension = "selfrefdoc")]
struct SelfRefDocument {
    #[dsl(block)]
    root: SelfRefValue,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).
impl pack::snapshot::ArtifactDsl for SelfRefDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match semio_framework_os_kernel::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = parse(body, &Self::__dsl_spec(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let record = self.__dsl_to_record();
        let body = print(&record, &Self::__dsl_spec(), JoinMode::Document);
        let envelope = semio_framework_os_kernel::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        semio_framework_os_kernel::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl pack::snapshot::ArtifactPack for SelfRefDocument {
    fn encode_pack_with(&self, options: &semio_framework_os_kernel::os_store::PackEncodeOptions) -> Result<Vec<u8>, semio_framework_os_kernel::os_store::PackError> {
        let inner = semio_framework_os_kernel::os_pack::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            semio_framework_os_kernel::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::semio_format::Component::Pack, 1).map_err(|error| semio_framework_os_kernel::os_store::PackError::from(error.into_value_error()))?;
        Ok(semio_framework_os_kernel::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &semio_framework_os_kernel::os_store::PackDecodeOptions) -> Result<Self, semio_framework_os_kernel::os_store::PackError> {
        let (envelope, inner) = semio_framework_os_kernel::os_store::semio_format::unwrap_binary(bytes).map_err(|error| semio_framework_os_kernel::os_store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::semio_format::Component::Pack, 1) {
            return Err(semio_framework_os_kernel::os_store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = semio_framework_os_kernel::os_pack::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(semio_framework_os_kernel::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SelfRefDocument {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}


}
//#endregion 🔖️ArtifactCodec

#[semio_framework_async_macros::async_test]
async fn derived_self_referential_record_struct_round_trips_nested_values() {
    let doc = SelfRefDocument {
        root: SelfRefValue {
            number: None,
            dictionary: Some(std::collections::BTreeMap::from([("a".to_string(), SelfRefValue { number: Some(1), dictionary: Some(std::collections::BTreeMap::from([("b".to_string(), SelfRefValue { number: Some(2), dictionary: None })])) })])),
        },
    };
    let printed = <SelfRefDocument as pack::snapshot::ArtifactDsl>::print_dsl(&doc);
    let parsed = <SelfRefDocument as pack::snapshot::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, doc, "self-referential record round trip diverged;\nprinted:\n{printed}");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&doc);
}

// --- end-to-end derive test: `#[dsl(table)] Vec<T>` (Structure-of-Arrays columnar field) ---

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, serde::Serialize, serde::Deserialize)]
struct TableNodeRow {
    id: String,
    x: f64,
    y: f64,
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, DslArtifact, serde::Serialize, serde::Deserialize)]
#[artifact(id = "table.doc", extension = "tabledoc")]
struct TableDocument {
    #[dsl(table)]
    nodes: Vec<TableNodeRow>,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).
impl pack::snapshot::ArtifactDsl for TableDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match semio_framework_os_kernel::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = parse(body, &Self::__dsl_spec(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let record = self.__dsl_to_record();
        let body = print(&record, &Self::__dsl_spec(), JoinMode::Document);
        let envelope = semio_framework_os_kernel::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        semio_framework_os_kernel::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl pack::snapshot::ArtifactPack for TableDocument {
    fn encode_pack_with(&self, options: &semio_framework_os_kernel::os_store::PackEncodeOptions) -> Result<Vec<u8>, semio_framework_os_kernel::os_store::PackError> {
        let inner = semio_framework_os_kernel::os_pack::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            semio_framework_os_kernel::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::semio_format::Component::Pack, 1).map_err(|error| semio_framework_os_kernel::os_store::PackError::from(error.into_value_error()))?;
        Ok(semio_framework_os_kernel::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &semio_framework_os_kernel::os_store::PackDecodeOptions) -> Result<Self, semio_framework_os_kernel::os_store::PackError> {
        let (envelope, inner) = semio_framework_os_kernel::os_store::semio_format::unwrap_binary(bytes).map_err(|error| semio_framework_os_kernel::os_store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::semio_format::Component::Pack, 1) {
            return Err(semio_framework_os_kernel::os_store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = semio_framework_os_kernel::os_pack::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(semio_framework_os_kernel::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for TableDocument {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}


}
//#endregion 🔖️ArtifactCodec

#[semio_framework_async_macros::async_test]
async fn derived_table_field_prints_compact_soa_and_round_trips() {
    let doc = TableDocument { nodes: vec![TableNodeRow { id: "a".to_string(), x: 1.0, y: 2.0 }, TableNodeRow { id: "b".to_string(), x: 3.0, y: 4.0 }] };
    let printed = <TableDocument as pack::snapshot::ArtifactDsl>::print_dsl(&doc);
    assert!(printed.contains("nodes [id:TEXT x:NUM y:NUM]"), "#[dsl(table)] field must print compact SoA: {printed}");
    let parsed = <TableDocument as pack::snapshot::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, doc, "table round trip diverged;\nprinted:\n{printed}");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&doc);
}
