
use super::*;

//#region 🔖️SyntheticArtifact
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.wave3.synthetic")]
struct SyntheticArtifact {
    #[state(artifact)]
    schema: String,
    #[state(artifact)]
    label: String,
    #[state(presence)]
    active_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.wave3.synthetic")]
struct SyntheticSnapshot {
    #[state(artifact)]
    schema: String,
    #[state(artifact)]
    label: String,
}

const SYNTHETIC_SNAPSHOT_JSON_SCHEMA: &str = r#"{
  "$id": "https://semio.tech/schema/s/wave3/synthetic/snapshot.json",
  "title": "SyntheticSnapshot",
  "type": "object",
  "additionalProperties": false,
  "required": ["schema", "label"],
  "properties": {
    "schema": { "type": "string", "x-semio-state": "artifact" },
    "label": { "type": "string", "x-semio-state": "artifact" }
  }
}"#;

async fn synthetic_descriptor() -> ArtifactSchemaDescriptor {
    let empty = FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" };
    ArtifactSchemaDescriptor { id: "s.wave3.synthetic", artifact: empty, snapshot: FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: SYNTHETIC_SNAPSHOT_JSON_SCHEMA, proto: "" }, diff: empty, mutations: empty }
}

async fn expected_snapshot_title(id: &str) -> String {
    let key = id.rsplit('.').next().unwrap_or(id);
    let mut chars = key.chars();
    let titled = match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    };
    format!("{titled}Snapshot")
}
//#endregion 🔖️SyntheticArtifact

//#region 🔖️ArtifactCompositionFixture
/// 🧪️ Local stand-ins for `semio-framework-os-kernel`'s store `ArtifactChild<T>` / `ArtifactLink`
/// — legitimate here since `#[derive(ArtifactSchema)]`'s composition support matches field types
/// SYNTACTICALLY (last path segment), never resolving the real types.
struct ArtifactChild<T> {
    _marker: std::marker::PhantomData<T>,
}
struct ArtifactLink;

impl<T> ChildFieldRefs for ArtifactChild<T> {
    const MANY: bool = false;
    fn visit_child_field<'a, V: ChildRefVisitor<'a>>(&'a self, slot: &'static str, visitor: &mut V) -> Result<(), V::Error> {
        visitor.step()?;
        visitor.child(slot, ChildRefFields { child_id: "child", artifact_id: "child", artifact_kind: "s.stdio.mesh", standard: "v1", subset: "*" })
    }
}

type AliasedChild = ArtifactChild<()>;

#[derive(ArtifactSchema)]
#[artifact_schema(id = "s.test.aliased-composition")]
struct AliasedComposition {
    #[state(artifact)]
    #[child(kind = "s.stdio.mesh")]
    optional_child: Option<Option<AliasedChild>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.mesh")]
    children: Vec<Option<AliasedChild>>,
}

#[expect(dead_code, reason = "the derive test inspects the field declarations without constructing this schema-only fixture")]
#[derive(ArtifactSchema)]
#[artifact_schema(id = "s.wave3.composite")]
struct CompositeArtifact {
    #[state(artifact)]
    #[child(kind = "s.stdio.mesh")]
    primary_mesh: ArtifactChild<()>,
    #[state(artifact)]
    #[child(kind = "s.stdio.image")]
    textures: Vec<ArtifactChild<()>>,
    #[state(artifact)]
    #[link_slot(roles("base", "material"))]
    base_material: ArtifactLink,
    #[state(artifact)]
    label: String,
}
//#endregion 🔖️ArtifactCompositionFixture

#[semio_framework_async_macros::async_test]
async fn artifact_composition_fields_derive_emits_expected_slot_tables() {
    let children = CompositeArtifact::child_slots();
    assert_eq!(children.len(), 2, "single child + Vec child must both be captured, plain field must not");
    assert_eq!(children[0], ChildSlotSpec { name: "primaryMesh", kind: "s.stdio.mesh", many: false });
    assert_eq!(children[1], ChildSlotSpec { name: "textures", kind: "s.stdio.image", many: true });

    let links = CompositeArtifact::link_slots();
    assert_eq!(links.len(), 1, "only the ArtifactLink field must be captured");
    assert_eq!(links[0], LinkSlotSpec { name: "baseMaterial", roles: &["base", "material"], many: false });
}

#[semio_framework_async_macros::async_test]
async fn artifact_composition_fields_default_to_empty_for_leaf_artifacts() {
    assert!(SyntheticSnapshot::child_slots().is_empty());
    assert!(SyntheticSnapshot::link_slots().is_empty());
    assert_eq!(SyntheticSnapshot::artifact_schema_id().await, "s.wave3.synthetic");
}

#[semio_framework_async_macros::async_test]
async fn artifact_composition_projection_walks_aliases_nested_options_and_cancels() {
    struct Visitor {
        steps: usize,
        maximum_steps: usize,
        rows: Vec<(&'static str, &'static str)>,
    }
    impl<'a> ChildRefVisitor<'a> for Visitor {
        type Error = ();
        fn step(&mut self) -> Result<(), ()> {
            if self.steps == self.maximum_steps {
                return Err(());
            }
            self.steps += 1;
            Ok(())
        }
        fn child(&mut self, slot: &'static str, fields: ChildRefFields<'a>) -> Result<(), ()> {
            assert_eq!(fields.child_id, fields.artifact_id);
            self.rows.push((slot, "child"));
            Ok(())
        }
    }
    let slots = AliasedComposition::child_slots();
    assert_eq!(slots, &[ChildSlotSpec { name: "optionalChild", kind: "s.stdio.mesh", many: false }, ChildSlotSpec { name: "children", kind: "s.stdio.mesh", many: true }]);
    let snapshot = AliasedComposition { optional_child: Some(Some(ArtifactChild { _marker: std::marker::PhantomData })), children: vec![None, Some(ArtifactChild { _marker: std::marker::PhantomData })] };
    let mut visitor = Visitor { steps: 0, maximum_steps: 16, rows: Vec::new() };
    snapshot.visit_child_refs(&mut visitor).unwrap();
    assert_eq!(visitor.rows, [("optionalChild", "child"), ("children", "child")]);
    assert_eq!(visitor.steps, 7);
    let mut visitor = Visitor { steps: 0, maximum_steps: 4, rows: Vec::new() };
    assert!(snapshot.visit_child_refs(&mut visitor).is_err());
    assert_eq!(visitor.steps, 4);
    assert_eq!(visitor.rows, [("optionalChild", "child")]);
    eprintln!("[DEBUG] schema child visitor: alias, nested option, collection and bounded early-stop assertions");
}

#[semio_framework_async_macros::async_test]
async fn artifact_composition_projection_real_child_alias_has_fixed_admission_bounds() {
    use semio_framework_os_kernel::{ArtifactChild, ChildRestoreProjection, ChildRestoreProjectionError};
    type ChildAlias = ArtifactChild<()>;
    #[derive(semio_framework_schema::ArtifactSchema)]
    #[artifact_schema(id = "s.test.parent")]
    struct DerivedParent {
        #[state(artifact)]
        #[child(kind = "s.test.member")]
        many: Vec<Option<ChildAlias>>,
    }
    let child = |id: String| {
        Some(ArtifactChild::new(
            id.clone(),
            semio_framework_os_kernel::os_io::ArtifactRef { artifact_id: id, dialect: semio_framework_os_kernel::os_io::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "v1".into(), subset: "first".into() } },
        ))
    };
    let mut parent = DerivedParent { many: (0..64).map(|index| child(index.to_string())).collect() };
    assert_eq!(ChildRestoreProjection::from_snapshot(&parent).unwrap().len(), 64);
    parent.many.push(child("overflow".into()));
    assert!(matches!(ChildRestoreProjection::from_snapshot(&parent), Err(ChildRestoreProjectionError::ReferenceLimit)));
    parent.many = (0..257).map(|_| None).collect();
    assert!(matches!(ChildRestoreProjection::from_snapshot(&parent), Err(ChildRestoreProjectionError::TraversalLimit)));
    parent.many = vec![child("ä".repeat(128))];
    assert!(ChildRestoreProjection::from_snapshot(&parent).is_ok());
    parent.many = vec![child(format!("{}x", "ä".repeat(128)))];
    assert!(matches!(ChildRestoreProjection::from_snapshot(&parent), Err(ChildRestoreProjectionError::InvalidReference)));
    eprintln!("[DEBUG] real derived child projection: 64/65 references, sparse traversal and 256/257 UTF-8 byte boundaries");
}

#[semio_framework_async_macros::async_test]
async fn registry_descriptors_carry_valid_snapshot_state_and_match_field_states() {
    let mut registry = ArtifactSchemaRegistry::new();
    registry.register(synthetic_descriptor().await);

    let mut walked = 0usize;
    for descriptor in registry.iter() {
        walked += 1;
        let schema = parse_json(descriptor.snapshot.json_schema).unwrap_or_else(|error| panic!("{}: snapshot json_schema parse: {error}", descriptor.id));
        let title = schema.get("title").and_then(Value::as_str).unwrap_or("");
        assert_eq!(title, expected_snapshot_title(descriptor.id).await, "{}: snapshot title must be XSnapshot for id", descriptor.id);

        let properties = schema.get("properties").and_then(Value::as_object).unwrap_or_else(|| panic!("{}: snapshot properties object required", descriptor.id));

        let mut json_states = Vec::new();
        for (name, prop) in properties {
            let raw = prop.get("x-semio-state").and_then(Value::as_str).unwrap_or_else(|| panic!("{}: property `{name}` missing x-semio-state", descriptor.id));
            let class = parse_state_class_kebab(raw).unwrap_or_else(|| panic!("{}: property `{name}` has invalid x-semio-state `{raw}`", descriptor.id));
            json_states.push((name.to_string(), class));
        }
        json_states.sort_by(|a, b| a.0.cmp(&b.0));

        let mut derived: Vec<(String, StateClass)> = SyntheticSnapshot::field_states().await.iter().map(|(name, class)| ((*name).to_string(), *class)).collect();
        derived.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(derived, json_states, "{}: field_states() must agree with snapshot JSON x-semio-state", descriptor.id);
        assert_eq!(SyntheticSnapshot::artifact_schema_id().await, descriptor.id);
        assert_eq!(SyntheticArtifact::artifact_schema_id().await, descriptor.id);
    }
    assert_eq!(walked, 1, "registry must be walked for the synthetic descriptor");
    assert!(registry.get("s.wave3.synthetic").is_some());
}

#[semio_framework_async_macros::async_test]
async fn graphql_state_preamble_matches_normative_sdl() {
    assert!(GRAPHQL_STATE_PREAMBLE.contains("enum StateClass { ARTIFACT CONFIG PRESENCE TRANSIENT }"));
    assert!(GRAPHQL_STATE_PREAMBLE.contains("directive @state(class: StateClass!) on FIELD_DEFINITION"));
    assert!(GRAPHQL_STATE_PREAMBLE.contains("directive @derived on FIELD_DEFINITION"));
}

#[semio_framework_async_macros::async_test]
async fn state_class_kebab_round_trips_exactly_the_four_lanes() {
    for class in [StateClass::Artifact, StateClass::Config, StateClass::Presence, StateClass::Transient] {
        let kebab = state_class_kebab(class).await;
        assert_eq!(parse_state_class_kebab(kebab), Some(class));
    }
    assert_eq!(state_class_kebab(StateClass::Artifact).await, "artifact");
    assert_eq!(state_class_kebab(StateClass::Config).await, "config");
    assert_eq!(state_class_kebab(StateClass::Presence).await, "presence");
    assert_eq!(state_class_kebab(StateClass::Transient).await, "transient");
}

#[semio_framework_async_macros::async_test]
async fn retired_state_vocabulary_no_longer_parses() {
    for retired in ["persistent", "shared-ui", "local-ui", "preview", "effect", "inferred", "identity"] {
        assert_eq!(parse_state_class_kebab(retired), None, "`{retired}` must not resolve to a state lane");
    }
}

//#region 🔖️DerivedAxis
/// 💡️ Derivation travels on its own axis: `#[derived]` fields carry no [`StateClass`] and are
/// reported by `derived_fields()`, never by `field_states()`.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.wave3.synthetic.inference")]
struct SyntheticInference {
    #[derived]
    topology: String,
    #[derived]
    depth: u32,
}

#[semio_framework_async_macros::async_test]
async fn derived_fields_leave_the_state_class_axis_entirely() {
    assert!(SyntheticInference::field_states().await.is_empty(), "a #[derived] field is not state");
    assert_eq!(SyntheticInference::derived_fields().await, &["topology", "depth"]);
    assert_eq!(SyntheticInference::artifact_schema_id().await, "s.wave3.synthetic.inference");
    assert!(SyntheticSnapshot::derived_fields().await.is_empty(), "state-only structs derive an empty derived table");
    assert_eq!(JSON_SCHEMA_DERIVED_KEY, "x-semio-derived");
}
//#endregion 🔖️DerivedAxis

#[semio_framework_async_macros::async_test]
async fn schema_catalog_still_registers_json() {
    let mut catalog = SchemaCatalog::new();
    catalog.register_json("probe", parse_json(r#"{"type":"object","properties":{"n":{"type":"integer"}}}"#).expect("schema json")).expect("register");
    catalog.validate("probe", &parse_json(r#"{"n":1}"#).expect("probe json")).expect("validate");
    assert!(catalog.validate("probe", &parse_json(r#"{"n":1.5}"#).expect("fractional probe json")).is_err());
}

#[semio_framework_async_macros::async_test]
async fn owned_validator_preserves_supported_keyword_corpus() {
    let schema_text = r#"{
            "type":"object",
            "additionalProperties":false,
            "required":["n"],
            "properties":{
                "n":{"type":"integer"},
                "mode":{"enum":["a","b"]},
                "rank":{"enum":[1,2]},
                "enabled":{"type":"boolean"},
                "nested":{"type":"object","additionalProperties":false,"properties":{"label":{"type":"string"}}}
            }
        }"#;
    let mut owned = SchemaCatalog::new();
    owned.register_json("probe", parse_json(schema_text).expect("owned schema")).expect("owned compile");
    let corpus = [
        (r#"{"n":1}"#, true),
        (r#"{"n":1.0}"#, true),
        (r#"{"n":-2,"mode":"a","enabled":true}"#, true),
        (r#"{"n":7,"nested":{"label":"ok"}}"#, true),
        (r#"{"n":7,"rank":1.0}"#, true),
        (r#"{}"#, false),
        (r#"{"n":1.5}"#, false),
        (r#"{"n":"1"}"#, false),
        (r#"{"n":1,"mode":"c"}"#, false),
        (r#"{"n":1,"rank":1.5}"#, false),
        (r#"{"n":1,"enabled":0}"#, false),
        (r#"{"n":1,"extra":true}"#, false),
        (r#"{"n":1,"nested":{"extra":true}}"#, false),
        (r#"null"#, false),
        (r#"[]"#, false),
    ];
    for (text, expected) in corpus {
        let owned_value = parse_json(text).expect("owned value");
        assert_eq!(owned.validate("probe", &owned_value).is_ok(), expected, "unexpected validator outcome for {text}");
    }
}

#[semio_framework_async_macros::async_test]
async fn owned_validator_preserves_every_exercised_keyword_family() {
    let schemas = [
        r##"{"$defs":{"name":{"type":"string","minLength":2,"maxLength":4}},"type":"object","properties":{"name":{"$ref":"#/$defs/name"}},"required":["name"],"additionalProperties":{"type":"integer"}}"##,
        r#"{"type":"array","items":{"type":["string","null"]},"minItems":1,"maxItems":3,"uniqueItems":true}"#,
        r#"{"allOf":[{"type":"number","minimum":0,"maximum":10},{"multipleOf":0.5}],"not":{"const":3.5}}"#,
        r#"{"anyOf":[{"const":"automatic"},{"type":"integer","exclusiveMinimum":0,"exclusiveMaximum":4}]}"#,
        r#"{"oneOf":[{"const":"left"},{"const":"right"}],"title":"side","description":"side choice","default":"left","examples":["right"],"readOnly":false,"writeOnly":false,"deprecated":false,"format":"semio-side","x-semio-kind":"choice"}"#,
    ];
    let corpora: [&[&str]; 5] = [
        &[r#"{"name":"ab"}"#, r#"{"name":"abcd","rank":2}"#, r#"{"name":"a"}"#, r#"{"name":"abcde"}"#, r#"{"name":"ok","rank":"2"}"#, r#"{}"#],
        &[r#"["a"]"#, r#"[null,"b"]"#, r#"[]"#, r#"["a","b","c","d"]"#, r#"["a","a"]"#, r#"[1]"#],
        &["0", "2.5", "10", "-0.5", "10.5", "2.25", "3.5", r#""2.5""#],
        &[r#""automatic""#, "1", "3", "0", "4", r#""manual""#],
        &[r#""left""#, r#""right""#, r#""center""#, "null"],
    ];
    let outcomes: [&[bool]; 5] = [&[true, true, false, false, false, false], &[true, true, false, false, false, false], &[true, true, true, false, false, false, false, false], &[true, true, true, false, false, false], &[true, true, false, false]];
    for ((schema_text, corpus), expected) in schemas.into_iter().zip(corpora).zip(outcomes) {
        let owned = crate::OwnedJsonSchemaValidator::compile(schema_text).expect("owned compile");
        for (text, expected) in corpus.iter().zip(expected) {
            assert_eq!(owned.is_valid_json(text), *expected, "unexpected validator outcome for schema {schema_text} and value {text}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn owned_validator_diagnostics_progress_and_cancellation_are_deterministic() {
    let validator = crate::OwnedJsonSchemaValidator::compile(r#"{"type":"object","properties":{"n":{"type":"integer"}},"required":["n"],"additionalProperties":false}"#).expect("compile");
    assert_eq!(validator.validate_json(r#"{"n":"wrong"}"#), Err(SchemaError::Validation("$.n: expected integer".to_string())));
    assert_eq!(validator.validate_json("{}"), Err(SchemaError::Validation("$: missing required property `n`".to_string())));
    assert_eq!(validator.validate_json(r#"{"n":1,"z":2}"#), Err(SchemaError::Validation("$: additional property `z` is not allowed".to_string())));

    let progress = validator.validate_json(r#"{"n":2}"#).expect("valid");
    assert_eq!(progress.visited_nodes, 2);
    let limited = crate::ValidationControl::new(1);
    assert_eq!(validator.validate_json_with_control(r#"{"n":2}"#, &limited), Err(SchemaError::LimitExceeded(1)));
    let cancelled = crate::ValidationControl::default();
    cancelled.cancel();
    assert_eq!(validator.validate_json_with_control(r#"{"n":2}"#, &cancelled), Err(SchemaError::Cancelled));
}

#[semio_framework_async_macros::async_test]
async fn schema_versions_ignore_whitespace_and_detect_drift() {
    let compact = schema_version(r#"{"type":"object","properties":{"n":{"type":"integer"}}}"#).expect("compact");
    let spaced = schema_version(r#"{ "type": "object", "properties": { "n": { "type": "integer" } } }"#).expect("spaced");
    let reordered = schema_version(r#"{"properties":{"n":{"type":"integer"}},"type":"object"}"#).expect("reordered");
    let drifted = schema_version(r#"{"type":"object","required":["n"],"properties":{"n":{"type":"integer"}}}"#).expect("drifted");
    assert_eq!(compact, spaced);
    assert_eq!(compact, reordered);
    assert_ne!(compact, drifted);
}

//#region 🔖️ArtifactInferenceDescriptorParity
#[semio_framework_async_macros::async_test]
async fn artifact_inference_registry_registers_independently_of_the_snapshot_diff_mutations_descriptor() {
    let mut registry = ArtifactInferenceRegistry::new();
    let empty = FacetLeaves { rust: "", typescript: "", graphql: "component { id }", json_schema: "", proto: "" };
    registry.register(ArtifactInferenceDescriptor { id: "s.wave3.synthetic.inference", inference: empty });
    assert_eq!(registry.len(), 1);
    assert!(!registry.is_empty());
    assert!(registry.get("s.wave3.synthetic.inference").is_some());

    let mut walked = 0usize;
    for descriptor in registry.iter() {
        walked += 1;
        assert_eq!(descriptor.id, "s.wave3.synthetic.inference");
    }
    assert_eq!(walked, 1);
}

#[semio_framework_async_macros::async_test]
async fn artifact_inference_graphql_sdl_composes_shared_preamble_with_facet_leaf() {
    register_artifact_inference_descriptor(ArtifactInferenceDescriptor {
        id: "s.wave3.synthetic.sdl-probe.inference",
        inference: FacetLeaves { rust: "", typescript: "", graphql: "type SdlProbeInference { flag: Boolean }", json_schema: "", proto: "" },
    });
    assert!(artifact_inference_descriptor_registered("s.wave3.synthetic.sdl-probe.inference"));
    let sdl = artifact_inference_graphql_sdl("s.wave3.synthetic.sdl-probe.inference").await.expect("registered inference sdl");
    assert!(sdl.contains("TRANSIENT"), "composed SDL must carry the shared @state preamble");
    assert!(sdl.contains("type SdlProbeInference"));
    assert!(artifact_inference_graphql_sdl("s.wave3.synthetic.unregistered.inference").await.is_none());
}
//#endregion 🔖️ArtifactInferenceDescriptorParity

//#region 🔖️AppSchemaRegistryParity

async fn empty_app_facet_leaves() -> FacetLeaves {
    FacetLeaves {
        rust: "",
        typescript: "",
        graphql: "",
        json_schema: r#"{
  "$id": "https://semio.tech/schema/app/placeholder/empty/config.json",
  "title": "EmptyConfig",
  "type": "object",
  "additionalProperties": false,
  "properties": {}
}"#,
        proto: "",
    }
}

#[semio_framework_async_macros::async_test]
async fn app_schema_registry_accepts_placeholder_owner_for_wave_structure() {
    let mut registry = AppSchemaRegistry::new();
    let empty = empty_app_facet_leaves().await;
    registry.register(AppSchemaDescriptor {
        id: "s.wave.a3.placeholder",
        config: empty,
        presence: FacetLeaves {
            json_schema: r#"{
  "$id": "https://semio.tech/schema/app/placeholder/empty/presence.json",
  "title": "EmptyPresence",
  "type": "object",
  "additionalProperties": false,
  "properties": {}
}"#,
            ..empty
        },
    });
    assert_eq!(registry.len(), 1);
    validate_registered_app_descriptor(registry.get("s.wave.a3.placeholder").expect("placeholder")).await;
    assert!(GRAPHQL_STATE_PREAMBLE.contains("directive @state"));
}
//#endregion 🔖️AppSchemaRegistryParity

//#region 🔖️SchemaExportResolution
const EXPORT_LEAVES: FacetLeaves =
    FacetLeaves { rust: "pub struct Thing;", typescript: "export type Thing = {};", graphql: "type Thing { id: String! }", json_schema: r#"{"$id":"https://semio.tech/schema/test/thing.json","type":"object"}"#, proto: "" };

#[semio_framework_async_macros::async_test]
async fn artifact_schema_descriptor_registration_mirrors_its_facets_into_the_export_registry() {
    let empty = FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" };
    let descriptor = ArtifactSchemaDescriptor { id: "test.mirror", artifact: EXPORT_LEAVES, snapshot: empty, diff: empty, mutations: empty };
    assert_eq!(descriptor.facet_leaves(), [EXPORT_LEAVES, empty, empty, empty]);
    register_artifact_schema_descriptor(descriptor);
    register_artifact_schema_descriptor(descriptor);

    assert!(artifact_schema_descriptor_registered("test.mirror"));
    assert!(scope_schema_facets_registered("test.mirror"));
    assert_eq!(resolve_schema_export("test.mirror", "artifact", SchemaFormat::Rust), Ok("pub struct Thing;"));
    assert_eq!(resolve_schema_export("test.mirror", "snapshot", SchemaFormat::Rust), Err(SchemaResolveError::FormatAbsent { scope: "test.mirror".to_string(), export: "snapshot".to_string(), format: SchemaFormat::Rust }));
    with_schema_export_registry(|registry| assert_eq!(registry.exports("test.mirror").expect("mirrored scope"), RESERVED_FACET_EXPORT_IDS.to_vec()));
}

#[semio_framework_async_macros::async_test]
async fn framework_schema_exports_match_the_modules_json_schema_defs_and_resolve() {
    register_framework_schema_exports().expect("framework.schema exports");
    register_framework_schema_exports().expect("exact duplicate registration is accepted");
    assert!(scope_schema_exports_registered(FRAMEWORK_SCHEMA_SCOPE));

    let document = parse_json(include_str!("../../🔣️.json")).expect("framework.schema json facet parses");
    let defs = document.get("$defs").and_then(Value::as_object).expect("$defs");
    let mut declared: Vec<&str> = defs.iter().map(|(key, _)| key).collect();
    declared.sort_unstable();
    let mut registered: Vec<&str> = FRAMEWORK_SCHEMA_EXPORTS.iter().map(|export| export.id).collect();
    registered.sort_unstable();
    assert_eq!(registered, declared, "every `$defs` key of 🔣️.json is a registered export and vice versa");

    for export in FRAMEWORK_SCHEMA_EXPORTS {
        for format in [SchemaFormat::Rust, SchemaFormat::Typescript, SchemaFormat::JsonSchema] {
            let leaf = resolve_schema_export(FRAMEWORK_SCHEMA_SCOPE, export.id, format).expect("declared format resolves");
            assert!(!leaf.trim().is_empty());
        }
        for format in [SchemaFormat::Graphql, SchemaFormat::Protobuf] {
            assert_eq!(resolve_schema_export(FRAMEWORK_SCHEMA_SCOPE, export.id, format), Err(SchemaResolveError::FormatAbsent { scope: FRAMEWORK_SCHEMA_SCOPE.to_string(), export: export.id.to_string(), format }));
        }
    }
    assert_eq!(resolve_schema_export(FRAMEWORK_SCHEMA_SCOPE, "SchemaFormat", SchemaFormat::Rust), Ok(include_str!("../../📇️registry/🦀️.rs")));
    assert_eq!(resolve_schema_export(FRAMEWORK_SCHEMA_SCOPE, "ValidationDiagnostic", SchemaFormat::Rust), Ok(include_str!("../../⚛️component.rs")));
}

#[semio_framework_async_macros::async_test]
async fn framework_schema_facet_validates_a_real_runtime_entries_dump() {
    register_framework_schema_exports().expect("framework.schema exports");
    let dump = SchemaExportEntries::from_catalog("cargo test -p semio-framework-schema");
    assert_eq!(dump.contract_id, SchemaExportEntries::CONTRACT_ID);
    assert!(dump.entries.iter().any(|entry| entry.scope == FRAMEWORK_SCHEMA_SCOPE && entry.export == "SchemaExportEntries" && entry.format == SchemaFormat::JsonSchema));

    let validator = structural_validator_for(FRAMEWORK_SCHEMA_SCOPE, "SchemaExportEntries").expect("SchemaExportEntries validator");
    let rendered = dump.to_json();
    if let Err(error) = validator.validate_json(&rendered) {
        panic!("the runtime dump must satisfy its own contract: {error}\n{rendered}");
    }

    assert!(validator.validate_json(r#"{"contractId":"schema-export-registry-entries-v2","generator":"x","entries":[]}"#).is_err());
    assert!(validator.validate_json(r#"{"contractId":"schema-export-registry-entries-v1","generator":"x"}"#).is_err());
    assert!(validator.validate_json(r#"{"contractId":"schema-export-registry-entries-v1","generator":"x","entries":[{"scope":"framework.schema","export":"SchemaFormat","format":"wit"}]}"#).is_err());
}

#[semio_framework_async_macros::async_test]
async fn validation_diagnostics_round_trip_through_the_validation_error() {
    let diagnostic = ValidationDiagnostic { instance_path: "$.entries[0].format".to_string(), reason: "value is not one of the allowed constants".to_string() };
    assert_eq!(diagnostic.message(), "$.entries[0].format: value is not one of the allowed constants");
    assert_eq!(diagnostic.json_pointer(), "/entries/0/format");
    assert_eq!(ValidationDiagnostic::from_error(&SchemaError::Validation(diagnostic.message())), Some(diagnostic));
    assert_eq!(ValidationDiagnostic::from_error(&SchemaError::Validation("no path here".to_string())), None);
}

#[semio_framework_async_macros::async_test]
async fn structural_validator_resolves_cross_scope_refs_by_document_id() {
    const SHARED: [SchemaExport; 1] = [SchemaExport {
        id: "ScopeId",
        leaves: FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: r#"{"$id":"https://semio.tech/schema/test/shared.json","definitions":{"ScopeId":{"type":"string","pattern":"^[a-z][a-z0-9]*(\\.[a-z][a-z0-9-]*)+$"}}}"#, proto: "" },
    }];
    const CONSUMER: [SchemaExport; 1] = [SchemaExport {
        id: "Binding",
        leaves: FacetLeaves {
            rust: "",
            typescript: "",
            graphql: "",
            json_schema: r#"{"$id":"https://semio.tech/schema/test/binding.json","type":"object","required":["scope"],"additionalProperties":false,"properties":{"scope":{"$ref":"https://semio.tech/schema/test/shared.json#/definitions/ScopeId"}}}"#,
            proto: "",
        },
    }];
    let mut registry = SchemaExportRegistry::new();
    registry.register_exports(ScopeSchemaExports { scope: "test.shared", exports: &SHARED }).expect("shared");
    registry.register_exports(ScopeSchemaExports { scope: "test.consumer", exports: &CONSUMER }).expect("consumer");

    let validator = structural_validator_in(&registry, "test.consumer", "Binding").expect("cross-scope validator");
    assert!(validator.is_valid_json(r#"{"scope":"s.trinity.jack"}"#));
    assert_eq!(validator.validate_json(r#"{"scope":"Trinity"}"#), Err(SchemaError::Validation("$.scope: string does not match pattern".to_string())));
    assert_eq!(structural_validator_in(&registry, "test.consumer", "Absent").err(), Some(SchemaBoundaryError::Resolve(SchemaResolveError::UnknownExport { scope: "test.consumer".to_string(), export: "Absent".to_string() })));
}

#[semio_framework_async_macros::async_test]
async fn scope_schema_exports_register_into_the_os_wide_catalog() {
    const GLOBAL: [SchemaExport; 1] = [SchemaExport { id: "Thing", leaves: EXPORT_LEAVES }];
    register_scope_schema_exports(ScopeSchemaExports { scope: "test.global", exports: &GLOBAL }).expect("register");
    register_scope_schema_exports(ScopeSchemaExports { scope: "test.global", exports: &GLOBAL }).expect("exact duplicate");
    assert!(scope_schema_exports_registered("test.global"));
    assert_eq!(resolve_schema_export("test.global", "Thing", SchemaFormat::Typescript), Ok("export type Thing = {};"));
    assert!(schema_export_catalog_entries().contains(&SchemaExportEntry { scope: "test.global", export: "Thing", format: SchemaFormat::Typescript }));
}
//#endregion 🔖️SchemaExportResolution

//#region 🔖️Draft07OracleVectors
const DRAFT07_VECTORS: &str = include_str!("../../🧫️fixtures/✅️draft07-validation-vectors.json");

fn pointer_to_owned_path(pointer: &str) -> String {
    pointer.split('/').skip(1).fold("$".to_string(), |path, segment| if segment.chars().all(|entry| entry.is_ascii_digit()) { format!("{path}[{segment}]") } else { format!("{path}.{segment}") })
}

#[semio_framework_async_macros::async_test]
async fn owned_validator_agrees_with_the_shared_draft07_vectors() {
    let vectors = parse_json(DRAFT07_VECTORS).expect("vectors json");
    let cases = vectors.get("cases").and_then(Value::as_array).expect("cases");
    assert!(cases.len() >= 16, "expected the full vector corpus, found {}", cases.len());
    for case in cases {
        let id = case.get("id").and_then(Value::as_str).expect("case id");
        let documents: Vec<String> = case.get("documents").and_then(Value::as_array).map(|documents| documents.iter().map(json_to_string).collect()).unwrap_or_default();
        let documents: Vec<&str> = documents.iter().map(String::as_str).collect();
        let schema = json_to_string(case.get("schema").expect("case schema"));
        let validator = crate::OwnedJsonSchemaValidator::compile_with_documents(&schema, &documents).unwrap_or_else(|error| panic!("{id}: compile: {error}"));
        for instance in case.get("valid").and_then(Value::as_array).expect("valid instances") {
            let instance = json_to_string(instance);
            assert!(validator.validate_json(&instance).is_ok(), "{id}: expected {instance} to validate, got {:?}", validator.validate_json(&instance));
        }
        for entry in case.get("invalid").and_then(Value::as_array).expect("invalid instances") {
            let instance = json_to_string(entry.get("instance").expect("instance"));
            let expected = pointer_to_owned_path(entry.get("errorPath").and_then(Value::as_str).expect("errorPath"));
            let Err(SchemaError::Validation(message)) = validator.validate_json(&instance) else {
                panic!("{id}: expected {instance} to be rejected");
            };
            assert!(message.starts_with(&format!("{expected}: ")), "{id}: expected {instance} to fail at {expected}, got `{message}`");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn owned_pattern_matcher_covers_the_supported_ecma_subset() {
    let corpus: [(&str, &[(&str, bool)]); 11] = [
        ("^[a-z][a-z0-9]*$", &[("scope", true), ("s9", true), ("Scope", false), ("", false)]),
        ("^v\\d{1,3}$", &[("v1", true), ("v123", true), ("v1234", false), ("v", false)]),
        ("^(rust|typescript|graphql)$", &[("rust", true), ("graphql", true), ("proto", false)]),
        ("^[^,]+$", &[("plain", true), ("a,b", false)]),
        ("a.c", &[("abc", true), ("a\nc", false), ("xxabcxx", true)]),
        ("^\\w+(\\s\\w+)*$", &[("one two three", true), ("one  two", false)]),
        ("^a{2,}b?$", &[("aa", true), ("aaab", true), ("a", false)]),
        ("colou?r", &[("color", true), ("colour", true), ("colr", false)]),
        ("^(?!0{4}$)[0-9a-f]{4}$", &[("00a0", true), ("0000", false), ("ffff", true)]),
        ("^(?!/)(?!.*(?:^|/)\\.\\.(?:/|$)).+$", &[("a/b", true), ("a/..b", true), ("/a", false), ("a/../b", false), ("..", false)]),
        ("(?=.*x)ab", &[("abx", true), ("xab", false), ("ab", false)]),
    ];
    for (pattern, cases) in corpus {
        let matcher = crate::PatternMatcher::compile(pattern).unwrap_or_else(|reason| panic!("{pattern}: {reason}"));
        for (text, expected) in cases {
            assert_eq!(matcher.is_match(text), *expected, "pattern {pattern} against {text}");
        }
    }
    for rejected in ["(?<=a)", "(?<!a)", "a\\1", "\\ba", "[z-a]", "(a", "*a"] {
        assert!(crate::PatternMatcher::compile(rejected).is_err(), "expected {rejected} to be rejected");
    }
}
//#endregion 🔖️Draft07OracleVectors

//#region 🔖️EntityKindCatalog
#[semio_framework_async_macros::async_test]
async fn entity_kind_catalog_data_validates_through_the_owned_validator_and_matches_the_rust_projection() {
    register_framework_schema_exports().expect("framework.schema exports");
    let validator = structural_validator_for(FRAMEWORK_SCHEMA_SCOPE, "EntityKindCatalog").expect("EntityKindCatalog validator");
    if let Err(error) = validator.validate_json(ENTITY_KIND_CATALOG_JSON) {
        panic!("🔣️entity-kinds.json must satisfy framework.schema#/$defs/EntityKindCatalog: {error}");
    }

    let document = parse_json(ENTITY_KIND_CATALOG_JSON).expect("catalog parses");
    let entries = document.as_array().expect("catalog is an array");
    assert_eq!(entries.len(), ENTITY_KINDS.len(), "the generated Rust projection carries every declared entity kind");
    for (entry, kind) in entries.iter().zip(ENTITY_KINDS) {
        assert_eq!(entry.get("id").and_then(Value::as_str), Some(kind.id));
        assert_eq!(entry.get("emoji").and_then(Value::as_str), Some(kind.emoji));
        assert_eq!(entry.get("iconId").and_then(Value::as_str), Some(kind.icon_id));
        assert_eq!(entry.get("label").and_then(Value::as_str), Some(kind.label));
        assert_eq!(entry.get("filterable").and_then(Value::as_bool), Some(kind.filterable));
    }

    let mut ids: Vec<&str> = ENTITY_KINDS.iter().map(|kind| kind.id).collect();
    let declared = ids.len();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), declared, "`id` is unique catalog-wide — the invariant draft-07 cannot express");
}

#[semio_framework_async_macros::async_test]
async fn entity_kind_emoji_index_is_first_wins_in_the_rust_projection() {
    let shadowed: Vec<&str> = ENTITY_KINDS.iter().filter(|kind| entity_kind_by_emoji(kind.emoji).is_some_and(|winner| winner.id != kind.id)).map(|kind| kind.id).collect();
    assert_eq!(shadowed, vec!["todo", "interaction-started"], "exactly two kinds are shadowed by an earlier kind carrying the same emoji");
    assert_eq!(entity_kind_by_emoji("📝️").map(|kind| kind.id), Some("draft"), "first-wins keeps `draft`, last-wins would answer `todo`");
    assert_eq!(entity_kind_by_emoji("🌱️").map(|kind| kind.id), Some("technology-mono"), "first-wins keeps `technology-mono`, last-wins would answer `interaction-started`");
    assert!(entity_kind_by_emoji("🦕️").is_none());
}

#[semio_framework_async_macros::async_test]
async fn entity_kind_catalog_rejects_every_declared_violation() {
    register_framework_schema_exports().expect("framework.schema exports");
    let validator = structural_validator_for(FRAMEWORK_SCHEMA_SCOPE, "EntityKindCatalog").expect("EntityKindCatalog validator");
    let valid = r#"[{"id":"technology-user","emoji":"👤️","iconId":"user","label":"User","filterable":true}]"#;
    assert!(validator.validate_json(valid).is_ok());
    for rejected in [
        "[]",
        r#"[{"id":"technology-user","emoji":"👤️","iconId":"user","label":"User"}]"#,
        r#"[{"id":"technology-user","emoji":"👤️","iconId":"user","label":"User","filterable":true,"color":"red"}]"#,
        r#"[{"id":"Technology-User","emoji":"👤️","iconId":"user","label":"User","filterable":true}]"#,
        r#"[{"id":"technology-user","emoji":"👤️ ","iconId":"user","label":"User","filterable":true}]"#,
        r#"[{"id":"technology-user","emoji":"👤️","iconId":"user","label":" User","filterable":true}]"#,
        r#"[{"id":"technology-user","emoji":"👤️","iconId":"user","label":"User","filterable":"yes"}]"#,
        r#"[{"id":"technology-user","emoji":"👤️","iconId":"user","label":"User","filterable":true},{"id":"technology-user","emoji":"👤️","iconId":"user","label":"User","filterable":true}]"#,
    ] {
        assert!(validator.validate_json(rejected).is_err(), "expected the facet to reject {rejected}");
    }
}
//#endregion 🔖️EntityKindCatalog
