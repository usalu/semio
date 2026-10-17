/// 🧱️ DSL field and derived document laws: the primitive and wire field impls, the toy idiom with the language registry, the derived scalar and record document with its handcrafted codecs and the derived operation enum with its text and binary laws.

#[semio_framework_async_macros::async_test]
async fn primitive_dsl_field_impls_round_trip() {
    assert_eq!(<i32 as DslField>::from_value(&DslField::to_value(&42i32)), Ok(42));
    assert_eq!(<u64 as DslField>::from_value(&DslField::to_value(&7u64)), Ok(7));
    assert_eq!(<bool as DslField>::from_value(&DslField::to_value(&true)), Ok(true));
    assert_eq!(<f64 as DslField>::from_value(&DslField::to_value(&1.5f64)), Ok(1.5));
    assert_eq!(<String as DslField>::from_value(&DslField::to_value(&"hi".to_string())), Ok("hi".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn wire_field_dsl_field_impl_round_trips() {
    let literal = parse_wire_text("a:Kind@out->b:Kind2@in").expect("parse_wire_text");
    assert!(matches!(Wire::shape(), Shape::Wire));
    let wire = Wire(literal.clone());
    let value = wire.to_value();
    assert_eq!(value, FieldValue::Wire(literal));
    let restored = Wire::from_value(&value).expect("from_value");
    assert_eq!(restored, wire);
}

// --- DslIdiom: a toy "hello <name>" language exercising the whole trait + registry seam ---
#[derive(Clone, Debug, PartialEq)]
struct GreetAst {
    name: String,
}

struct GreetIdiom;

impl DslIdiom for GreetIdiom {
    const LANG: &'static str = "greet";
    type Ast = GreetAst;

    // 🚫️async: E4 fn-pointer slot — DslIdiom::parse must stay sync, see the trait's own tag.
    fn parse(text: &str) -> Result<Self::Ast, TextError> {
        text.strip_prefix("hello ").map(|name| GreetAst { name: name.trim().to_string() }).ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected 'hello <name>'", TextSpan::at(1, 1)))
    }

    // 🚫️async: E4 fn-pointer slot — see parse above
    fn print(ast: &Self::Ast) -> String {
        format!("hello {}", ast.name)
    }

    // 🚫️async: E4 fn-pointer slot — see parse above
    fn classify(_text: &str) -> Vec<(TokenClass, TextSpan)> {
        Vec::new()
    }
}





#[semio_framework_async_macros::async_test]
async fn language_registry_resolves_by_id_and_semio_content() {
    register_language(LanguageSpec { id: "greet.doc", extension: Some("greet"), role: LanguageRole::Document, grammar: None, grammar_path: None, protocol: None, protocol_path: None, hooks: hooks_for::<GreetIdiom>() });
    let by_id = language("greet.doc").expect("registered language must be found by its id");
    assert_eq!(by_id.extension, Some("greet"));
    assert_eq!(by_id.role, LanguageRole::Document);
    let bytes = b"semio greet.doc.dsl v1\nhello world\n";
    let by_content = language_for_semio_content(bytes).expect("registered language must be found by sniffed envelope");
    assert_eq!(by_content.id, "greet.doc");
    assert!(language("never-registered-id").is_none());
    assert!(language_for_semio_content(b"semio missing.dsl v1\n").is_none());
}

#[semio_framework_async_macros::async_test]
async fn dsl_value_dsl_field_round_trips_through_record_value() {
    let value = DslValue::object([("a".into(), DslValue::float(1.0)), ("b".into(), DslValue::Array(vec![DslValue::Bool(true), DslValue::Null, DslValue::String("x".into())]))]);
    assert_eq!(<DslValue as DslField>::from_value(&DslField::to_value(&value)), Ok(value));

    let map = DslValue::object([("curves".into(), DslValue::Array(vec![DslValue::Array(vec![DslValue::float(0.0), DslValue::float(0.0)]), DslValue::Array(vec![DslValue::float(1.0), DslValue::float(1.0)])]))]);
    assert_eq!(<DslValue as DslField>::from_value(&DslField::to_value(&map)), Ok(map));
}

// --- end-to-end derive tests: mirrors the norm-family "flat scalar document" worked example ---

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslScalar, serde::Serialize, serde::Deserialize)]
enum ClimateZone {
    Cold,
    Temperate,
    Warm,
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, DslArtifact, serde::Serialize, serde::Deserialize)]
#[artifact(id = "derived.doc", extension = "derivedoc")]
struct DerivedDocument {
    category: String,
    climate: ClimateZone,
    airtightness_n50: f64,
    occupants: u32,
    note: Option<String>,
}

#[semio_framework_async_macros::async_test]
async fn derived_artifact_preserves_explicit_file_extension() {
    assert_eq!(DerivedDocument::__DSL_ENVELOPE_ID, "derived.doc");
    assert_eq!(DerivedDocument::__DSL_EXTENSION, "derivedoc");
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).
impl pack::snapshot::ArtifactDsl for DerivedDocument {
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
impl pack::snapshot::ArtifactPack for DerivedDocument {
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
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for DerivedDocument {

}
//#endregion 🔖️ArtifactCodec

#[semio_framework_async_macros::async_test]
async fn derived_document_round_trips_through_vcs_document_dsl() {
    let doc = DerivedDocument { category: "external_wall".to_string(), climate: ClimateZone::Cold, airtightness_n50: 0.6, occupants: 4, note: None };
    let printed = <DerivedDocument as pack::snapshot::ArtifactDsl>::print_dsl(&doc);
    assert!(!printed.contains("note"), "absent optional field must be omitted: {printed}");
    let parsed = <DerivedDocument as pack::snapshot::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, doc, "derived ArtifactDsl round trip diverged;\nprinted:\n{printed}");
}

#[semio_framework_async_macros::async_test]
async fn derived_document_round_trips_with_optional_field_present() {
    let doc = DerivedDocument { category: "roof".to_string(), climate: ClimateZone::Warm, airtightness_n50: 1.2, occupants: 2, note: Some("re-inspect in 2027".to_string()) };
    let printed = <DerivedDocument as pack::snapshot::ArtifactDsl>::print_dsl(&doc);
    let parsed = <DerivedDocument as pack::snapshot::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, doc);
}

// --- end-to-end derive test: a Mutation enum via #[derive(DslOps)] ---

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, serde::Serialize, serde::Deserialize)]
enum DerivedMutation {
    #[dsl(key = "setCategory")]
    SetCategory { category: String },
    #[dsl(key = "setAirtightness")]
    SetAirtightness { n50: f64 },
    #[dsl(key = "reset")]
    Reset,
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl semio_framework_os_kernel::os_spr::OpText for DerivedMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        variants_text::print_op(self)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl semio_framework_os_kernel::os_spr::OpBinary for DerivedMutation {
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
async fn derived_op_text_round_trips_every_variant_as_one_line() {
    let ops = vec![DerivedMutation::SetCategory { category: "roof".to_string() }, DerivedMutation::SetAirtightness { n50: 0.9 }, DerivedMutation::Reset];
    for op in ops {
        let printed = <DerivedMutation as semio_framework_os_kernel::os_spr::OpText>::print_op(&op);
        assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
        let parsed = <DerivedMutation as semio_framework_os_kernel::os_spr::OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
        assert_eq!(parsed, op, "OpText round trip diverged for {printed:?}");
    }
}

/// 🧾️ LAW: an operation line is ONE terminal record — the text twin of the trailing-byte refusal
/// `derived_op_binary_round_trips_every_variant_and_matches_text` already proves for the binary
/// form. `parse` alone stops at the end of the record it recognises and drops the rest, so a line
/// carrying an unknown trailing field parsed as a well-formed operation and silently lost it.
#[semio_framework_async_macros::async_test]
async fn derived_op_text_refuses_every_token_outside_its_own_record() {
    let ops = vec![DerivedMutation::SetCategory { category: "roof".to_string() }, DerivedMutation::SetAirtightness { n50: 0.9 }, DerivedMutation::Reset];
    for op in ops {
        let printed = <DerivedMutation as semio_framework_os_kernel::os_spr::OpText>::print_op(&op);
        for trailing in ["unknown-field 1", "reset", "\"", "setCategory category=\"wall\""] {
            let line = format!("{printed} {trailing}");
            assert!(<DerivedMutation as semio_framework_os_kernel::os_spr::OpText>::parse_op(&line).is_err(), "an operation line must refuse everything outside its own record: {line:?}");
        }
    }
    assert!(<DerivedMutation as semio_framework_os_kernel::os_spr::OpText>::parse_op("").is_err());
    assert!(<DerivedMutation as semio_framework_os_kernel::os_spr::OpText>::parse_op("unknown-field 1").is_err());
}

#[semio_framework_async_macros::async_test]
async fn derived_document_dsl_satisfies_vcs_test_support_helpers() {
    let doc = DerivedDocument { category: "floor".to_string(), climate: ClimateZone::Temperate, airtightness_n50: 0.4, occupants: 3, note: None };
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&doc);
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&doc);
}

#[semio_framework_async_macros::async_test]
async fn derived_op_satisfies_vcs_test_support_helpers() {
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&DerivedMutation::SetCategory { category: "wall".to_string() });
}

#[semio_framework_async_macros::async_test]
async fn derived_op_binary_round_trips_every_variant_and_matches_text() {
    let ops = vec![DerivedMutation::SetCategory { category: "roof".to_string() }, DerivedMutation::SetAirtightness { n50: 0.9 }, DerivedMutation::Reset];
    for op in ops {
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&op);
        let encoded = variants_binary::encode_op(&op).unwrap();
        let decoded: DerivedMutation = variants_binary::decode_op(&encoded).unwrap();
        assert_eq!(serde_json::to_value(&decoded).unwrap(), serde_json::to_value(&op).unwrap());
        let mut trailing = encoded.clone();
        trailing.push(0);
        assert!(variants_binary::decode_op::<DerivedMutation>(&trailing).is_err());
        let mut nonminimal = encoded;
        nonminimal[1] |= 0x80;
        nonminimal.insert(2, 0);
        assert!(variants_binary::decode_op::<DerivedMutation>(&nonminimal).is_err());
    }
}
