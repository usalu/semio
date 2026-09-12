use super::*;

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
        text.strip_prefix("hello ").map(|name| GreetAst { name: name.trim().to_string() }).ok_or_else(|| TextError::new("expected 'hello <name>'", TextSpan::at(1, 1)))
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
async fn dsl_idiom_round_trips_through_its_own_parse_and_print() {
    let ast = GreetIdiom::parse("hello world").expect("parse");
    assert_eq!(ast, GreetAst { name: "world".to_string() });
    assert_eq!(GreetIdiom::print(&ast), "hello world");
    assert_eq!(GreetIdiom::parse(&GreetIdiom::print(&ast)), Ok(ast), "idiom round trip law");
}

#[semio_framework_async_macros::async_test]
async fn dsl_idiom_registry_resolves_by_lang_and_canonicalizes_through_the_hooks() {
    register_idiom(hooks_for::<GreetIdiom>());
    let hooks = idiom("greet").expect("registered idiom must be found by its LANG id");
    assert_eq!(hooks.lang, "greet");
    let canonical = (hooks.canonicalize)("hello   world").expect("canonicalize");
    assert_eq!(canonical, "hello world", "canonicalize normalizes through parse -> print");
    assert!((hooks.canonicalize)("not a greeting").is_err(), "a malformed idiom body must surface the idiom's own parse error");
    assert!(idiom("never-registered-lang").is_none(), "an unregistered lang must resolve to None, never a default/error");
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

#[derive(Clone, Debug, PartialEq, DslScalar, serde::Serialize, serde::Deserialize)]
enum ClimateZone {
    Cold,
    Temperate,
    Warm,
}

#[derive(Clone, Debug, PartialEq, DslArtifact, serde::Serialize, serde::Deserialize)]
#[dsl(id = "derived.doc", extension = "derivedoc")]
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
impl crate::os_store::ArtifactDsl for DerivedDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match crate::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = parse(body, &Self::__dsl_spec(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let record = self.__dsl_to_record();
        let body = print(&record, &Self::__dsl_spec(), JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl crate::os_store::ArtifactPack for DerivedDocument {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        let inner = crate::os_store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1) {
            return Err(crate::os_store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as crate::os_store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = crate::os_store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(crate::os_store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactCodec

#[semio_framework_async_macros::async_test]
async fn derived_document_round_trips_through_vcs_document_dsl() {
    let doc = DerivedDocument { category: "external_wall".to_string(), climate: ClimateZone::Cold, airtightness_n50: 0.6, occupants: 4, note: None };
    let printed = <DerivedDocument as crate::os_store::ArtifactDsl>::print_dsl(&doc);
    assert!(!printed.contains("note"), "absent optional field must be omitted: {printed}");
    let parsed = <DerivedDocument as crate::os_store::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, doc, "derived ArtifactDsl round trip diverged;\nprinted:\n{printed}");
}

#[semio_framework_async_macros::async_test]
async fn derived_document_round_trips_with_optional_field_present() {
    let doc = DerivedDocument { category: "roof".to_string(), climate: ClimateZone::Warm, airtightness_n50: 1.2, occupants: 2, note: Some("re-inspect in 2027".to_string()) };
    let printed = <DerivedDocument as crate::os_store::ArtifactDsl>::print_dsl(&doc);
    let parsed = <DerivedDocument as crate::os_store::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, doc);
}

// --- end-to-end derive test: a Mutation enum via #[derive(DslOps)] ---

#[derive(Clone, Debug, PartialEq, DslOps, serde::Serialize, serde::Deserialize)]
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
impl crate::os_spr::OpText for DerivedMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = parse(line, &spec_fn(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline })?;
                return <Self as DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as DslVariants>::to_named_record(self);
        let variants = <Self as DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        print(&record, &spec_fn(), JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl crate::os_spr::OpBinary for DerivedMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as DslVariants>::to_named_record(self);
        let variants = <Self as DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &crate::os_store::PackEncodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let record_offset = reader.position() as u64;
        let body = &bytes[record_offset as usize..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &crate::os_store::PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        <Self as DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec

#[semio_framework_async_macros::async_test]
async fn derived_op_text_round_trips_every_variant_as_one_line() {
    let ops = vec![DerivedMutation::SetCategory { category: "roof".to_string() }, DerivedMutation::SetAirtightness { n50: 0.9 }, DerivedMutation::Reset];
    for op in ops {
        let printed = <DerivedMutation as crate::os_spr::OpText>::print_op(&op);
        assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
        let parsed = <DerivedMutation as crate::os_spr::OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
        assert_eq!(parsed, op, "OpText round trip diverged for {printed:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn derived_document_dsl_satisfies_vcs_test_support_helpers() {
    let doc = DerivedDocument { category: "floor".to_string(), climate: ClimateZone::Temperate, airtightness_n50: 0.4, occupants: 3, note: None };
    crate::os_store::test_support::assert_dsl_round_trip(&doc);
    crate::os_store::test_support::assert_dsl_pack_equivalence(&doc);
}

#[semio_framework_async_macros::async_test]
async fn derived_op_satisfies_vcs_test_support_helpers() {
    crate::os_store::test_support::assert_op_line_round_trip(&DerivedMutation::SetCategory { category: "wall".to_string() });
}

#[semio_framework_async_macros::async_test]
async fn derived_op_binary_round_trips_every_variant_and_matches_text() {
    let ops = vec![DerivedMutation::SetCategory { category: "roof".to_string() }, DerivedMutation::SetAirtightness { n50: 0.9 }, DerivedMutation::Reset];
    for op in ops {
        crate::os_store::test_support::assert_op_text_binary_equivalence(&op);
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

// --- end-to-end derive test: `#[derive(DslEnum)]` recursive block tree (the `note`/`draw`
// pilots' hard case), `Vec<Vec<T>>`, `[T; N]`, and `BTreeMap<String, V>` fields ---

#[derive(Clone, Debug, PartialEq, DslEnum, serde::Serialize, serde::Deserialize)]
enum SceneNode {
    #[dsl(key = "point")]
    Point { pos: [f64; 3] },
    #[dsl(key = "grid")]
    Grid { rows: Vec<Vec<i32>> },
    #[dsl(key = "group")]
    Group {
        #[dsl(positional)]
        id: String,
        #[dsl(statements, block)]
        children: Vec<SceneNode>,
    },
}

#[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
#[dsl(keyword = "camera")]
struct SceneCamera {
    x: f64,
    y: f64,
}

#[derive(Clone, Debug, PartialEq, DslArtifact, serde::Serialize, serde::Deserialize)]
#[dsl(id = "scene.doc", extension = "scenedoc")]
struct SceneDocument {
    // `#[dsl(block)]` alone (no `statements`) wraps a plain nested-record scalar field so it
    // prints as a bare `camera { x=.. y=.. }` line instead of a `camera=...` attribute.
    #[dsl(block)]
    camera: SceneCamera,
    #[dsl(statements, block)]
    nodes: Vec<SceneNode>,
    tags: std::collections::BTreeMap<String, String>,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).
impl crate::os_store::ArtifactDsl for SceneDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match crate::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = parse(body, &Self::__dsl_spec(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let record = self.__dsl_to_record();
        let body = print(&record, &Self::__dsl_spec(), JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl crate::os_store::ArtifactPack for SceneDocument {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        let inner = crate::os_store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1) {
            return Err(crate::os_store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as crate::os_store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = crate::os_store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(crate::os_store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactCodec

#[semio_framework_async_macros::async_test]
async fn derived_enum_recursive_block_tree_and_map_and_nested_collections_round_trip() {
    let doc = SceneDocument {
        camera: SceneCamera { x: 1.0, y: 2.0 },
        nodes: vec![SceneNode::Point { pos: [1.0, 2.0, 3.0] }, SceneNode::Grid { rows: vec![vec![1, 2], vec![3, 4, 5]] }, SceneNode::Group { id: "g1".to_string(), children: vec![SceneNode::Point { pos: [0.0, 0.0, 0.0] }] }],
        tags: std::collections::BTreeMap::from([("author".to_string(), "semio".to_string()), ("version".to_string(), "1".to_string())]),
    };
    let printed = <SceneDocument as crate::os_store::ArtifactDsl>::print_dsl(&doc);
    let parsed = <SceneDocument as crate::os_store::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, doc, "recursive/nested-collection round trip diverged;\nprinted:\n{printed}");
    crate::os_store::test_support::assert_dsl_pack_equivalence(&doc);
}

// --- end-to-end derive test: single-field tuple ("newtype") variants (the `draw` pilot's
// `LayerNode::Shape(ShapeBody)` shape) delegate entirely to the inner type's own spec/keyword ---

#[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
#[dsl(keyword = "circle")]
struct CircleBody {
    #[dsl(positional)]
    id: String,
    r: f64,
}

#[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
#[dsl(keyword = "square")]
struct SquareBody {
    #[dsl(positional)]
    id: String,
    side: f64,
}

#[derive(Clone, Debug, PartialEq, DslEnum, serde::Serialize, serde::Deserialize)]
enum ShapeNode {
    #[dsl(key = "circle")]
    Circle(CircleBody),
    #[dsl(key = "square")]
    Square(SquareBody),
}

#[derive(Clone, Debug, PartialEq, DslArtifact, serde::Serialize, serde::Deserialize)]
#[dsl(id = "shape.doc", extension = "shapedoc")]
struct ShapeDocument {
    #[dsl(statements, block)]
    shapes: Vec<ShapeNode>,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).
impl crate::os_store::ArtifactDsl for ShapeDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match crate::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = parse(body, &Self::__dsl_spec(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let record = self.__dsl_to_record();
        let body = print(&record, &Self::__dsl_spec(), JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl crate::os_store::ArtifactPack for ShapeDocument {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        let inner = crate::os_store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1) {
            return Err(crate::os_store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as crate::os_store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = crate::os_store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(crate::os_store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactCodec

#[semio_framework_async_macros::async_test]
async fn derived_newtype_tuple_variants_round_trip() {
    let doc = ShapeDocument { shapes: vec![ShapeNode::Circle(CircleBody { id: "c1".to_string(), r: 2.0 }), ShapeNode::Square(SquareBody { id: "s1".to_string(), side: 3.0 })] };
    let printed = <ShapeDocument as crate::os_store::ArtifactDsl>::print_dsl(&doc);
    // `"c1"` is bare-ident-shaped, so the unified "strings bare-preferred" law prints it
    // unquoted (`circle c1 r=2`, not `circle "c1" r=2`) — see `crate::os_dsl::is_bare_ident`.
    assert!(printed.contains("circle c1 r=2"), "newtype variant must print via its own inner keyword/fields: {printed}");
    let parsed = <ShapeDocument as crate::os_store::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, doc, "newtype tuple-variant round trip diverged;\nprinted:\n{printed}");
    crate::os_store::test_support::assert_dsl_pack_equivalence(&doc);
}

// --- end-to-end derive test: `#[dsl(statements, block)] Option<T>` (the `draw` pilot's
// `attributes.fill: Option<FillStyle>` shape) — a sum-type scalar field, not a collection ---

#[derive(Clone, Debug, PartialEq, DslEnum, serde::Serialize, serde::Deserialize)]
enum PaintStyle {
    #[dsl(key = "solid")]
    Solid { color: [f64; 4] },
    #[dsl(key = "gradient")]
    Gradient { stops: Vec<f64> },
}

#[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
struct PaintAttributes {
    #[dsl(statements, block)]
    fill: Option<PaintStyle>,
}

#[derive(Clone, Debug, PartialEq, DslArtifact, serde::Serialize, serde::Deserialize)]
#[dsl(id = "paint.doc", extension = "paintdoc")]
struct PaintDocument {
    #[dsl(block)]
    attributes: PaintAttributes,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).
impl crate::os_store::ArtifactDsl for PaintDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match crate::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = parse(body, &Self::__dsl_spec(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let record = self.__dsl_to_record();
        let body = print(&record, &Self::__dsl_spec(), JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl crate::os_store::ArtifactPack for PaintDocument {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        let inner = crate::os_store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1) {
            return Err(crate::os_store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as crate::os_store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = crate::os_store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(crate::os_store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactCodec

#[semio_framework_async_macros::async_test]
async fn derived_option_statements_field_round_trips_present_and_absent() {
    let with_fill = PaintDocument { attributes: PaintAttributes { fill: Some(PaintStyle::Solid { color: [1.0, 0.0, 0.0, 1.0] }) } };
    let printed = <PaintDocument as crate::os_store::ArtifactDsl>::print_dsl(&with_fill);
    let parsed = <PaintDocument as crate::os_store::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, with_fill, "Some(..) round trip diverged;\nprinted:\n{printed}");
    crate::os_store::test_support::assert_dsl_pack_equivalence(&with_fill);

    let no_fill = PaintDocument { attributes: PaintAttributes { fill: None } };
    let printed_none = <PaintDocument as crate::os_store::ArtifactDsl>::print_dsl(&no_fill);
    let parsed_none = <PaintDocument as crate::os_store::ArtifactDsl>::parse_dsl(&printed_none).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed_none}"));
    assert_eq!(parsed_none, no_fill, "None round trip diverged;\nprinted:\n{printed_none}");
    crate::os_store::test_support::assert_dsl_pack_equivalence(&no_fill);
}

// --- regression: `#[dsl(block)] Option<PlainRecord>` (the `draw` pilot's `attributes.stroke:
// Option<StrokeStyle>` shape) — `None` must OMIT the field, not print empty `{ }` braces, since
// reparsing empty braces would otherwise try to build a record whose required fields are absent ---

#[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
struct BrushStyle {
    color: [f64; 4],
    width: f64,
}

#[derive(Clone, Debug, PartialEq, DslArtifact, serde::Serialize, serde::Deserialize)]
#[dsl(id = "art.doc", extension = "brushdoc")]
struct BrushDocument {
    #[dsl(block)]
    brush: Option<BrushStyle>,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).
impl crate::os_store::ArtifactDsl for BrushDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match crate::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = parse(body, &Self::__dsl_spec(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let record = self.__dsl_to_record();
        let body = print(&record, &Self::__dsl_spec(), JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl crate::os_store::ArtifactPack for BrushDocument {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        let inner = crate::os_store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1) {
            return Err(crate::os_store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as crate::os_store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = crate::os_store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(crate::os_store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactCodec

#[semio_framework_async_macros::async_test]
async fn derived_option_block_record_field_omits_rather_than_printing_empty_braces_when_absent() {
    let with_brush = BrushDocument { brush: Some(BrushStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.5 }) };
    let printed = <BrushDocument as crate::os_store::ArtifactDsl>::print_dsl(&with_brush);
    let parsed = <BrushDocument as crate::os_store::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, with_brush, "Some(..) round trip diverged;\nprinted:\n{printed}");
    crate::os_store::test_support::assert_dsl_pack_equivalence(&with_brush);

    let no_brush = BrushDocument { brush: None };
    let printed_none = <BrushDocument as crate::os_store::ArtifactDsl>::print_dsl(&no_brush);
    assert!(!printed_none.contains("brush"), "an absent block-wrapped Option<Record> must be omitted entirely, not printed as empty braces: {printed_none:?}");
    let parsed_none = <BrushDocument as crate::os_store::ArtifactDsl>::parse_dsl(&printed_none).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed_none}"));
    assert_eq!(parsed_none, no_brush, "None round trip diverged;\nprinted:\n{printed_none:?}");
    crate::os_store::test_support::assert_dsl_pack_equivalence(&no_brush);
}

// --- end-to-end derive test: `#[dsl(statements)] Box<T>` (the `draw` pilot's
// `AddLayer { layer: Box<DrawLayerNode> }` shape) — exactly one required tagged value ---

#[derive(Clone, Debug, PartialEq, DslOps, serde::Serialize, serde::Deserialize)]
enum PaintOp {
    #[dsl(key = "addShape")]
    AddShape {
        #[dsl(statements)]
        shape: Box<ShapeNode>,
    },
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl crate::os_spr::OpText for PaintOp {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = parse(line, &spec_fn(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline })?;
                return <Self as DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as DslVariants>::to_named_record(self);
        let variants = <Self as DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        print(&record, &spec_fn(), JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl crate::os_spr::OpBinary for PaintOp {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as DslVariants>::to_named_record(self);
        let variants = <Self as DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &crate::os_store::PackEncodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        crate::os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(crate::os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(crate::os_spr::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let record_offset = reader.position() as u64;
        let body = &bytes[record_offset as usize..];
        let (record, _report) = crate::os_pack::decode_record_body(body, &spec, &crate::os_store::PackDecodeOptions::default()).map_err(crate::os_spr::ProtocolError::from)?;
        <Self as DslVariants>::from_named_record(keyword, &record).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec

#[semio_framework_async_macros::async_test]
async fn derived_required_statements_boxed_field_round_trips() {
    let op = PaintOp::AddShape { shape: Box::new(ShapeNode::Circle(CircleBody { id: "c1".to_string(), r: 2.0 })) };
    let printed = <PaintOp as crate::os_spr::OpText>::print_op(&op);
    assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
    let parsed = <PaintOp as crate::os_spr::OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
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

#[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
struct SelfRefValue {
    #[dsl(key = "n")]
    number: Option<i64>,
    #[dsl(key = "dict")]
    dictionary: Option<std::collections::BTreeMap<String, SelfRefValue>>,
}

#[derive(Clone, Debug, PartialEq, DslArtifact, serde::Serialize, serde::Deserialize)]
#[dsl(id = "selfref.doc", extension = "selfrefdoc")]
struct SelfRefDocument {
    #[dsl(block)]
    root: SelfRefValue,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).
impl crate::os_store::ArtifactDsl for SelfRefDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match crate::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = parse(body, &Self::__dsl_spec(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let record = self.__dsl_to_record();
        let body = print(&record, &Self::__dsl_spec(), JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl crate::os_store::ArtifactPack for SelfRefDocument {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        let inner = crate::os_store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1) {
            return Err(crate::os_store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as crate::os_store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = crate::os_store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(crate::os_store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<RecordSpec> {
        Some(Self::__dsl_spec())
    }
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
    let printed = <SelfRefDocument as crate::os_store::ArtifactDsl>::print_dsl(&doc);
    let parsed = <SelfRefDocument as crate::os_store::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, doc, "self-referential record round trip diverged;\nprinted:\n{printed}");
    crate::os_store::test_support::assert_dsl_pack_equivalence(&doc);
}

// --- end-to-end derive test: `#[dsl(table)] Vec<T>` (Structure-of-Arrays columnar field) ---

#[derive(Clone, Debug, PartialEq, DslRecord, serde::Serialize, serde::Deserialize)]
struct TableNodeRow {
    id: String,
    x: f64,
    y: f64,
}

#[derive(Clone, Debug, PartialEq, DslArtifact, serde::Serialize, serde::Deserialize)]
#[dsl(id = "table.doc", extension = "tabledoc")]
struct TableDocument {
    #[dsl(table)]
    nodes: Vec<TableNodeRow>,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).
impl crate::os_store::ArtifactDsl for TableDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match crate::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = parse(body, &Self::__dsl_spec(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let record = self.__dsl_to_record();
        let body = print(&record, &Self::__dsl_spec(), JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl crate::os_store::ArtifactPack for TableDocument {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        let inner = crate::os_store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1) {
            return Err(crate::os_store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as crate::os_store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = crate::os_store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(crate::os_store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactCodec

#[semio_framework_async_macros::async_test]
async fn derived_table_field_prints_compact_soa_and_round_trips() {
    let doc = TableDocument { nodes: vec![TableNodeRow { id: "a".to_string(), x: 1.0, y: 2.0 }, TableNodeRow { id: "b".to_string(), x: 3.0, y: 4.0 }] };
    let printed = <TableDocument as crate::os_store::ArtifactDsl>::print_dsl(&doc);
    assert!(printed.contains("nodes [id:TEXT x:NUM y:NUM]"), "#[dsl(table)] field must print compact SoA: {printed}");
    let parsed = <TableDocument as crate::os_store::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, doc, "table round trip diverged;\nprinted:\n{printed}");
    crate::os_store::test_support::assert_dsl_pack_equivalence(&doc);
}
