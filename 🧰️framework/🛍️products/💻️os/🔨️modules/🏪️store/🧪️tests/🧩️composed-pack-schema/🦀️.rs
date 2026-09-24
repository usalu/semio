//! 🧩️ Derived pack-schema identity for snapshots that compose children and links: the hash is pinned,
//! structure-sensitive down to the nested `ArtifactLink`/`LinkPin` records, cross-checked with `blake3`
//! over the canonical graph bytes, and the derived codec round-trips byte-exactly.
use super::*;

#[derive(Clone, Debug, PartialEq, crate::os_dsl::DslRecord)]
#[dsl(extension = "composed")]
struct ComposedSnapshot {
    schema: String,
    document: ArtifactChild<()>,
    parts: Vec<ArtifactChild<()>>,
    cover: Option<ArtifactLink>,
    links: Vec<ArtifactLink>,
}

#[derive(Clone, Debug, PartialEq, crate::os_dsl::DslRecord)]
#[dsl(extension = "composed")]
struct ComposedWithoutLinks {
    schema: String,
    document: ArtifactChild<()>,
    parts: Vec<ArtifactChild<()>>,
    cover: Option<ArtifactLink>,
}

#[derive(Clone, Debug, PartialEq, crate::os_dsl::DslRecord)]
#[dsl(extension = "composed")]
struct ComposedRenamedChild {
    schema: String,
    model: ArtifactChild<()>,
    parts: Vec<ArtifactChild<()>>,
    cover: Option<ArtifactLink>,
    links: Vec<ArtifactLink>,
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧩️composed-pack-schema/🔣️.json")).expect("composed pack schema fixture")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn child(row: &serde_json::Value) -> ArtifactChild<()> {
    ArtifactChild::new(row["childId"].as_str().unwrap().into(), crate::os_io::ArtifactRef::parse_uri(row["target"].as_str().unwrap()).unwrap())
}

fn link(row: &serde_json::Value) -> ArtifactLink {
    let pin = match row["pin"]["kind"].as_str().unwrap() {
        "head" => LinkPin::Head,
        "checkpoint" => LinkPin::Checkpoint { id: row["pin"]["id"].as_str().unwrap().into() },
        "snapshot" => LinkPin::Snapshot { blob: BlobRef { hash: row["pin"]["hash"].as_str().unwrap().into(), size: row["pin"]["size"].as_u64().unwrap(), media_type: row["pin"]["mediaType"].as_str().unwrap().into() } },
        other => panic!("unknown pin kind {other}"),
    };
    ArtifactLink { target: crate::os_io::ArtifactRef::parse_uri(row["target"].as_str().unwrap()).unwrap(), pin, role: row["role"].as_str().unwrap().into() }
}

fn snapshot(row: &serde_json::Value) -> ComposedSnapshot {
    ComposedSnapshot {
        schema: row["schema"].as_str().unwrap().into(),
        document: child(&row["document"]),
        parts: row["parts"].as_array().unwrap().iter().map(child).collect(),
        cover: (!row["cover"].is_null()).then(|| link(&row["cover"])),
        links: row["links"].as_array().unwrap().iter().map(link).collect(),
    }
}

fn graph_json(spec: &crate::os_dsl::RecordSpec) -> serde_json::Value {
    serde_json::from_str(&crate::os_pack::json::to_string(&crate::os_pack::PackSchemaGraph::of(spec).to_json())).expect("graph json")
}

fn renamed_role_link_spec() -> crate::os_dsl::RecordSpec {
    let mut spec = artifact_link_spec();
    spec.fields.iter_mut().find(|field| field.key == "role").expect("link role field").key = "purpose".into();
    spec
}

fn with_link_spec(link: fn() -> crate::os_dsl::RecordSpec) -> crate::os_dsl::RecordSpec {
    let mut spec = ComposedSnapshot::__dsl_spec();
    for field in &mut spec.fields {
        field.shape = match (field.key.as_str(), &field.shape) {
            ("cover", crate::os_dsl::Shape::Record(_)) => crate::os_dsl::Shape::Record(link),
            ("links", crate::os_dsl::Shape::List(_)) => crate::os_dsl::Shape::List(Box::new(crate::os_dsl::Shape::Record(link))),
            _ => continue,
        };
    }
    spec
}

#[test]
fn derived_graph_matches_the_language_agnostic_fixture() {
    let fixture = fixture();
    assert_eq!(graph_json(&ComposedSnapshot::__dsl_spec()), fixture["graph"]);
    let child_keys: Vec<String> = artifact_child_spec().fields.iter().map(|field| field.key.to_string()).collect();
    assert_eq!(serde_json::json!(child_keys), fixture["childFields"]);
    let link_keys: Vec<String> = artifact_link_spec().fields.iter().map(|field| field.key.to_string()).collect();
    assert_eq!(serde_json::json!(link_keys), fixture["linkFields"]);
}

#[test]
fn pack_schema_hash_is_pinned_and_matches_an_independent_blake3() {
    let fixture = fixture();
    let spec = ComposedSnapshot::__dsl_spec();
    let canonical = crate::os_pack::PackSchemaGraph::of(&spec).canonical_bytes();
    assert_eq!(hex(&canonical), fixture["canonicalHex"].as_str().unwrap());
    let expected = fixture["schemaHash"].as_str().unwrap();
    assert_eq!(hex(&crate::os_pack::schema_hash(&spec)), expected);
    assert_eq!(blake3::hash(&canonical).to_hex().to_string(), expected);
}

#[test]
fn pack_schema_hash_differs_when_the_structure_differs() {
    let fixture = fixture();
    let composed = crate::os_pack::schema_hash(&ComposedSnapshot::__dsl_spec());
    let without_links = crate::os_pack::schema_hash(&ComposedWithoutLinks::__dsl_spec());
    let renamed_child = crate::os_pack::schema_hash(&ComposedRenamedChild::__dsl_spec());
    let renamed_link_role = crate::os_pack::schema_hash(&with_link_spec(renamed_role_link_spec));
    assert_eq!(crate::os_pack::schema_hash(&with_link_spec(artifact_link_spec)), composed);
    assert_ne!(composed, without_links);
    assert_ne!(composed, renamed_child);
    assert_ne!(composed, renamed_link_role);
    assert_ne!(without_links, renamed_child);
    assert_eq!(hex(&without_links), fixture["withoutLinksSchemaHash"].as_str().unwrap());
    assert_eq!(hex(&renamed_child), fixture["renamedChildSchemaHash"].as_str().unwrap());
    assert_eq!(hex(&renamed_link_role), fixture["renamedLinkRoleSchemaHash"].as_str().unwrap());
}

#[test]
fn derived_codec_round_trips_composed_children_and_links_byte_exactly() {
    let fixture = fixture();
    for row in fixture["documents"].as_array().unwrap() {
        let document = snapshot(&row["snapshot"]);
        let spec = ComposedSnapshot::__dsl_spec();
        let bytes = crate::os_pack::encode_document(&spec, &document.__dsl_to_record(), &PackEncodeOptions::default()).expect("encode composed snapshot");
        assert_eq!(hex(&bytes), row["packHex"].as_str().unwrap(), "{}", row["name"]);
        let (record, report) = crate::os_pack::decode_document(&bytes, &spec, &PackDecodeOptions::default()).expect("decode composed snapshot");
        assert!(report.unknown_field_ids.is_empty() && !report.schema_drift);
        let decoded = ComposedSnapshot::__dsl_from_record(&record).expect("record to snapshot");
        assert_eq!(decoded, document);
        assert_eq!(crate::os_pack::encode_document(&spec, &decoded.__dsl_to_record(), &PackEncodeOptions::default()).expect("re-encode"), bytes);
        assert!(crate::os_pack::decode_document(&bytes, &ComposedWithoutLinks::__dsl_spec(), &PackDecodeOptions::default()).expect("narrow decode").1.schema_drift);
    }
}
