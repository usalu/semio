/// 🌳️ Scene and shape laws: the recursive block tree, the nested collections, the newtype tuple variants and their handcrafted document codecs.

// --- end-to-end derive test: `#[derive(DslEnum)]` recursive block tree (the `note`/`draw`
// pilots' hard case), `Vec<Vec<T>>`, `[T; N]`, and `BTreeMap<String, V>` fields ---

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, serde::Serialize, serde::Deserialize)]
#[dsl(keyword = "camera")]
struct SceneCamera {
    x: f64,
    y: f64,
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, DslArtifact, serde::Serialize, serde::Deserialize)]
#[artifact(id = "scene.doc", extension = "scenedoc")]
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
impl pack::snapshot::ArtifactDsl for SceneDocument {
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
impl pack::snapshot::ArtifactPack for SceneDocument {
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
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SceneDocument {

}
//#endregion 🔖️ArtifactCodec

#[semio_framework_async_macros::async_test]
async fn derived_enum_recursive_block_tree_and_map_and_nested_collections_round_trip() {
    let doc = SceneDocument {
        camera: SceneCamera { x: 1.0, y: 2.0 },
        nodes: vec![SceneNode::Point { pos: [1.0, 2.0, 3.0] }, SceneNode::Grid { rows: vec![vec![1, 2], vec![3, 4, 5]] }, SceneNode::Group { id: "g1".to_string(), children: vec![SceneNode::Point { pos: [0.0, 0.0, 0.0] }] }],
        tags: std::collections::BTreeMap::from([("author".to_string(), "semio".to_string()), ("version".to_string(), "1".to_string())]),
    };
    let printed = <SceneDocument as pack::snapshot::ArtifactDsl>::print_dsl(&doc);
    let parsed = <SceneDocument as pack::snapshot::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, doc, "recursive/nested-collection round trip diverged;\nprinted:\n{printed}");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&doc);
}

// --- end-to-end derive test: single-field tuple ("newtype") variants (the `draw` pilot's
// `LayerNode::Shape(ShapeBody)` shape) delegate entirely to the inner type's own spec/keyword ---

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, serde::Serialize, serde::Deserialize)]
#[dsl(keyword = "circle")]
struct CircleBody {
    #[dsl(positional)]
    id: String,
    r: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, serde::Serialize, serde::Deserialize)]
#[dsl(keyword = "square")]
struct SquareBody {
    #[dsl(positional)]
    id: String,
    side: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, serde::Serialize, serde::Deserialize)]
enum ShapeNode {
    #[dsl(key = "circle")]
    Circle(CircleBody),
    #[dsl(key = "square")]
    Square(SquareBody),
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, DslArtifact, serde::Serialize, serde::Deserialize)]
#[artifact(id = "shape.doc", extension = "shapedoc")]
struct ShapeDocument {
    #[dsl(statements, block)]
    shapes: Vec<ShapeNode>,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6).
impl pack::snapshot::ArtifactDsl for ShapeDocument {
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
impl pack::snapshot::ArtifactPack for ShapeDocument {
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
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for ShapeDocument {

}
//#endregion 🔖️ArtifactCodec

#[semio_framework_async_macros::async_test]
async fn derived_newtype_tuple_variants_round_trip() {
    let doc = ShapeDocument { shapes: vec![ShapeNode::Circle(CircleBody { id: "c1".to_string(), r: 2.0 }), ShapeNode::Square(SquareBody { id: "s1".to_string(), side: 3.0 })] };
    let printed = <ShapeDocument as pack::snapshot::ArtifactDsl>::print_dsl(&doc);
    // `"c1"` is bare-ident-shaped, so the unified "strings bare-preferred" law prints it
    // unquoted (`circle c1 r=2`, not `circle "c1" r=2`) — see `crate::os_dsl::is_bare_ident`.
    assert!(printed.contains("circle c1 r=2"), "newtype variant must print via its own inner keyword/fields: {printed}");
    let parsed = <ShapeDocument as pack::snapshot::ArtifactDsl>::parse_dsl(&printed).unwrap_or_else(|e| panic!("parse failed: {e}\nprinted:\n{printed}"));
    assert_eq!(parsed, doc, "newtype tuple-variant round trip diverged;\nprinted:\n{printed}");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&doc);
}
