
use super::*;

/// 🔐️ "" -> e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 (well-known empty-input SHA-256)
#[semio_framework_async_macros::async_test]
async fn sha256_hex_matches_known_vector() {
    assert_eq!(semio_framework_hash::sha256_hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
}

#[semio_framework_async_macros::async_test]
async fn describe_artifact_bound_admits_the_local_iterate_cap() {
    assert_eq!(DESCRIBE_ARTIFACT_MAX_BYTES, 512 * 1024 * 1024);
}

#[semio_framework_async_macros::async_test]
async fn full_catalog_describe_retains_finite_wall_and_fuel_bounds() {
    assert_eq!(DESCRIBE_DEADLINE_MS, 1_800_000);
    assert_eq!(DESCRIBE_FUEL_BUDGET, 32_000_000_000);
}

/// 🪪️ The static kind-identity law over the language-neutral fixture (`🧫️fixtures/🪪️kind-identity`).
#[semio_framework_async_macros::async_test]
async fn kind_identity_law_matches_the_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪪️kind-identity/🔣️.json")).expect("kind-identity fixture");
    for case in fixture["cases"].as_array().expect("cases") {
        let apps: Vec<KindIdentityApp<'_>> = case["apps"]
            .as_array()
            .expect("apps")
            .iter()
            .map(|app| KindIdentityApp {
                app_id: app["appId"].as_str().expect("appId"),
                presented_kind: app["presentedKind"].as_str().expect("presentedKind"),
                presented_schema: app["presentedSchema"].as_str().expect("presentedSchema"),
                declared: app["declared"].as_array().expect("declared").iter().map(|pair| (pair[0].as_str().expect("kind"), pair[1].as_str().expect("schema"))).collect(),
            })
            .collect();
        assert_eq!(kind_identity_faults(&apps).len() as u64, case["faults"].as_u64().expect("faults"), "{}", case["name"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn run_with_no_args_returns_usage_exit_code() {
    assert_eq!(run(Vec::new()).await, 2);
}

#[semio_framework_async_macros::async_test]
async fn run_with_unknown_command_returns_usage_exit_code() {
    assert_eq!(run(vec!["not-describe".to_string()]).await, 2);
}

#[semio_framework_async_macros::async_test]
async fn run_describe_without_out_flag_returns_usage_exit_code() {
    assert_eq!(run(vec!["describe".to_string(), "component.wasm".to_string()]).await, 2);
}

#[semio_framework_async_macros::async_test]
async fn run_describe_refuses_the_retired_core_flag() {
    assert_eq!(run(vec!["describe".to_string(), "component.wasm".to_string(), "--core".to_string(), "core.wasm".to_string(), "--out".to_string(), "/tmp/does-not-matter".to_string()]).await, 2);
}

#[semio_framework_async_macros::async_test]
async fn run_describe_on_missing_file_returns_failure_exit_code() {
    let code = run(vec!["describe".to_string(), "/nonexistent/component.wasm".to_string(), "--out".to_string(), "/tmp/does-not-matter".to_string()]).await;
    assert_eq!(code, 1);
}

mod long {
    use semio_framework_test_fixtures::fixture_path;

    use super::*;

    /// 📦️ The shipped component of every hub composition and extension crate (`📦️packages/🦀️rust/dist/components/*.wasm`).
    fn shipped_components(repository: &Path) -> Vec<PathBuf> {
        let children = |directory: &Path| fs::read_dir(directory).map(|entries| entries.filter_map(Result::ok).map(|entry| entry.path()).collect::<Vec<_>>()).unwrap_or_default();
        let compositions = children(&repository.join("🌎️hub/🧩️compositions"));
        let owners = compositions.iter().cloned().chain(compositions.iter().flat_map(|composition| children(&composition.join("🧩️extensions")))).collect::<Vec<_>>();
        owners.iter().flat_map(|owner| children(&owner.join("📦️packages/🦀️rust/dist/components"))).filter(|path| path.extension().is_some_and(|extension| extension == "wasm")).collect()
    }

    /// 🔬️ The differential suite: the components `SEMIO_OWNED_DIFFERENTIAL_FIXTURES` names, otherwise the two smallest shipped components.
    fn differential_components() -> Vec<PathBuf> {
        if let Some(paths) = std::env::var_os("SEMIO_OWNED_DIFFERENTIAL_FIXTURES") {
            return std::env::split_paths(&paths).collect();
        }
        let repository = fixture_path!("../../../../../../../..");
        let mut shipped = shipped_components(&repository).into_iter().filter_map(|path| fs::metadata(&path).ok().map(|metadata| (metadata.len(), path))).collect::<Vec<_>>();
        shipped.sort();
        shipped.into_iter().take(2).map(|(_, path)| path).collect()
    }

    /// ⚖️ The production owned describe (`OwnedRuntime::compile` + `describe_observed`, the path `describe_component` runs) answers
    /// the exact descriptor bytes the independent Wasmtime component host answers, on real shipped components.
    #[semio_framework_async_macros::async_test]
    async fn shipped_components_owned_descriptors_match_wasmtime() {
        let components = differential_components();
        assert!(components.len() >= 2, "the owned describe differential needs at least two shipped components (a release build ships them under `📦️packages/🦀️rust/dist/components`), found {components:?}");
        for path in components {
            let bytes = fs::read(&path).unwrap_or_else(|error| panic!("read differential component {}: {error}", path.display()));
            let oracle = execute_describe_wasmtime(&bytes, &path).await.unwrap_or_else(|error| panic!("execute Wasmtime descriptor oracle for {}: {error}", path.display()));
            let (owned, _, _) = execute_describe_owned(&bytes, &path).await.unwrap_or_else(|error| panic!("execute owned describe for {}: {error}", path.display()));
            assert!(!oracle.is_empty(), "Wasmtime answered an empty descriptor for {}", path.display());
            assert_eq!(owned, oracle, "owned descriptor differs from Wasmtime for {}", path.display());
            eprintln!("owned describe matches Wasmtime for {} ({} descriptor bytes)", path.display(), owned.len());
        }
    }
}

//#region 🔖️OwnedCoreExportLaw

/// 🧬️ The plugin SDK's own source — `plugin_exports!`, `__semio_plugin_actor_exports!` and
/// `extension_exports!` all live in this one file, so the law below reads their real bodies rather
/// than a restatement of them.
const PLUGIN_SDK_SOURCE: &str = include_str!("../../../⚛️reactor/🏃️runtime/🦀️.rs");

/// 🧱 The text of one `macro_rules!` body, brace-matched from its opening `{`.
fn macro_rules_body<'a>(source: &'a str, name: &str) -> &'a str {
    let header = format!("macro_rules! {name} {{");
    let start = source.find(&header).unwrap_or_else(|| panic!("`macro_rules! {name}` is not declared in the plugin SDK source")) + header.len();
    let mut depth = 1_usize;
    for (offset, character) in source[start..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &source[start..start + offset];
                }
            }
            _ => {}
        }
    }
    panic!("`macro_rules! {name}` is unbalanced in the plugin SDK source")
}

/// 🔍️ Every arm body of one `macro_rules!` body, brace-matched from each arm's `=> {`, whatever its matcher spells.
fn macro_arms(body: &str) -> Vec<&str> {
    let mut arms = Vec::new();
    let mut rest = body;
    while let Some(arrow) = rest.find("=> {") {
        let start = arrow + "=> {".len();
        let mut depth = 1_usize;
        let end = rest[start..]
            .char_indices()
            .find_map(|(offset, character)| {
                match character {
                    '{' => depth += 1,
                    '}' => depth -= 1,
                    _ => {}
                }
                (depth == 0).then_some(start + offset)
            })
            .unwrap_or_else(|| panic!("a macro arm is unbalanced in the plugin SDK source"));
        arms.push(&rest[start..end]);
        rest = &rest[end + 1..];
    }
    arms
}

/// 🏷️ Every `semio_owned_*_v1` symbol a chunk of source defines as a core export.
fn owned_export_definitions(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = source;
    while let Some(offset) = rest.find("extern \"C\" fn semio_owned_") {
        let tail = &rest[offset + "extern \"C\" fn ".len()..];
        let end = tail.find('(').expect("owned core export declaration has an argument list");
        names.push(tail[..end].to_string());
        rest = &tail[end..];
    }
    names.sort();
    names
}

/// 🧩️ The owned core exports an owner entry point reaches, following the one indirection each arm
/// uses — `__semio_owned_core_exports!` directly, or `__semio_plugin_actor_exports!` which invokes it.
fn owned_exports_reached_by(arm: &str) -> Vec<String> {
    let core = macro_rules_body(PLUGIN_SDK_SOURCE, "__semio_owned_core_exports");
    let direct = arm.contains("__semio_owned_core_exports!(");
    let indirect = arm.contains("__semio_plugin_actor_exports!(") && macro_rules_body(PLUGIN_SDK_SOURCE, "__semio_plugin_actor_exports").contains("__semio_owned_core_exports!(");
    assert!(direct || indirect, "owner arm reaches no `__semio_owned_core_exports!` invocation:\n{arm}");
    owned_export_definitions(core)
}

/// 🛡️ Every component owner exports the SAME `semio_owned_*_v1` core symbols, from ONE shared
/// definition, and that set is exactly `OwnedSemioExport::ALL`, the one
/// `OwnedSemioArtifact::from_component` demands. Its arity is owned by the type of `ALL`, never
/// restated here. The codec and SQLite exports are the owned twins of `world actor`'s `codec`
/// interface (ticket 26/09/18 slice TC3b) — a component that lacks them cannot answer a headless
/// hub's creation or validation call, so the ABI admits it as a whole or not at all.
///
/// 🐛️ Regression guard for `📓️a3-descriptor-regeneration.md` §3b: `extension_exports!`'s
/// single-argument arm used to expand `__semio_actor_exports!` directly and emit none of them,
/// so every extension component failed owned-ABI validation and no extension could be described.
/// Copying them into the extension arm would have satisfied a symbol-set assertion while
/// leaving two bodies free to drift, so the law also refuses any second definition of them.
#[semio_framework_async_macros::async_test]
async fn owned_core_exports_are_defined_once_and_invoked_by_both_owners() {
    let mut required: Vec<String> = OwnedSemioExport::ALL.iter().map(|export| export.core_name().to_string()).collect();
    required.sort();
    assert!(required.windows(2).all(|pair| pair[0] != pair[1]), "`OwnedSemioExport::ALL` names one core export twice");

    let defined_in_shared_macro = owned_export_definitions(macro_rules_body(PLUGIN_SDK_SOURCE, "__semio_owned_core_exports"));
    assert_eq!(defined_in_shared_macro, required, "the shared owned-core-export macro does not define exactly `OwnedSemioExport::ALL`");
    assert_eq!(owned_export_definitions(PLUGIN_SDK_SOURCE), required, "a `semio_owned_*_v1` core export is defined outside `__semio_owned_core_exports!` — the arms must share one body, never a copy");

    for owner in ["plugin_exports", "extension_exports"] {
        let arms = macro_arms(macro_rules_body(PLUGIN_SDK_SOURCE, owner));
        assert!(!arms.is_empty(), "`{owner}!` declares no arm");
        for arm in arms {
            assert_eq!(owned_exports_reached_by(arm), required, "an arm of `{owner}!` does not reach exactly the owned core exports:\n{arm}");
        }
    }
}
//#endregion 🔖️OwnedCoreExportLaw

/// 🪪️ The emitter classifies a declared-but-unowned kind by the plugin crate's own fault text; the law keeps the two in step.
#[semio_framework_async_macros::async_test]
async fn unowned_codec_schema_fault_is_the_plugin_crates_own_text() {
    assert!(PLUGIN_SDK_SOURCE.contains(&format!("plugin_internal_fault(\"{UNOWNED_ARTIFACT_CODEC_SCHEMA}\")")), "the plugin crate no longer faults with UNOWNED_ARTIFACT_CODEC_SCHEMA");
}

//#region 🔖️CanonicalDescriptorPackLaw
/// 🔁️ `value` with the members of every object in reverse order, at every depth.
fn reversed_members(value: semio_framework_value::DslValue) -> semio_framework_value::DslValue {
    use semio_framework_value::DslValue;
    match value {
        DslValue::Array(items) => DslValue::Array(items.into_iter().map(reversed_members).collect()),
        DslValue::Object(entries) => DslValue::Object(entries.into_iter().rev().map(|(key, entry)| (key, reversed_members(entry))).collect()),
        scalar => scalar,
    }
}

/// 🔤️ LAW (design §22.19): a descriptor packs to the bytes the TypeScript pack encoder sealed in the language-neutral fixture
/// (`🧫️fixtures/🧫️canonical-descriptor-pack`), whatever order its members were authored in — the members of every object
/// in UTF-8 key-byte order at every depth, with either order policy of the value encoder underneath.
#[semio_framework_async_macros::async_test]
async fn a_descriptor_pack_is_canonical_whatever_order_its_members_were_authored_in() {
    let fixture = semio_framework_pack_json::parse(include_str!("../🧫️fixtures/🧫️canonical-descriptor-pack/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("canonical descriptor pack fixture");
    let cases = fixture.get("cases").and_then(|cases| cases.as_array()).expect("cases");
    assert!(!cases.is_empty());
    for case in cases {
        let id = case.get("id").and_then(|id| id.as_str()).expect("id");
        let authored = semio_framework_pack_json::to_dsl_value(case.get("authored").expect("authored"));
        let canonical = semio_framework_pack_json::to_dsl_value(case.get("canonical").expect("canonical"));
        let expected = case.get("expectedPackHex").and_then(|hex| hex.as_str()).expect("expectedPackHex");
        assert_ne!(authored, canonical, "{id}: the fixture authors its members out of canonical order");
        assert_eq!(CanonicalDescriptorValue::new(authored.clone()).value(), &canonical, "{id}: members in key-byte order at every depth");
        assert_eq!(CanonicalDescriptorValue::new(canonical.clone()).value(), &canonical, "{id}: canonical order is a fixed point");
        for (form, value) in [("authored", authored), ("canonical", canonical.clone()), ("reversed", reversed_members(canonical))] {
            let packed: String = descriptor_pack(&CanonicalDescriptorValue::new(value)).iter().map(|byte| format!("{byte:02x}")).collect();
            assert_eq!(packed, expected, "{id}: the {form} member order packs to the bytes the TypeScript encoder sealed");
        }
    }
}

/// 🚧️ Every descriptor byte leaves through [`descriptor_pack`]: the emitter names the wire value encoder exactly once, on the
/// canonical value.
#[semio_framework_async_macros::async_test]
async fn the_emitter_encodes_descriptors_only_through_the_canonical_pack() {
    let source = include_str!("../../🛂️descriptor-emission/🦀️.rs");
    assert_eq!(source.matches("encode_wire_value(").count(), 1, "a descriptor is encoded outside `descriptor_pack`");
    assert!(source.contains("pub fn descriptor_pack(value: &CanonicalDescriptorValue) -> Vec<u8> {\n    store::pack_rt::encode_wire_value(value.value())\n}"), "`descriptor_pack` encodes anything but the canonical value");
}
//#endregion 🔖️CanonicalDescriptorPackLaw
