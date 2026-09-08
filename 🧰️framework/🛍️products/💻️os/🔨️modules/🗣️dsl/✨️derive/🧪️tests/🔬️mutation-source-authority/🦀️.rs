
use super::*;

pub(super) fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🛂️mutation-source-authority/🧫️fixtures/🔣️.json")).unwrap()
}

fn link_file(target: &Path, link: &Path) {
    #[cfg(unix)]
    std::os::unix::fs::symlink(target, link).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(target, link).unwrap();
}

fn link_dir(target: &Path, link: &Path) {
    #[cfg(unix)]
    std::os::unix::fs::symlink(target, link).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(target, link).unwrap();
}

fn fixture_workspace(case: &str) -> PathBuf {
    let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
    fs::create_dir_all(&base).unwrap();
    fs::canonicalize(base).unwrap().join(format!("semio-source-authority-{case}-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
}

pub(super) fn materialize(case: &str, fixture: &serde_json::Value) -> (PathBuf, PathBuf, PathBuf) {
    let workspace = fixture_workspace(case);
    let mutation_root = workspace.join("domain/🧬️mutations");
    let owner = mutation_root.join("🆕️insert-page");
    let source = owner.join("🦀️.rs");
    let descriptor = owner.join("🔣️.json");
    let taxonomy = workspace.join("authority/🔣️taxonomy.json");
    fs::create_dir_all(&owner).unwrap();
    fs::create_dir_all(taxonomy.parent().unwrap()).unwrap();
    fs::write(workspace.join("nx.json"), "{}").unwrap();
    fs::write(&source, "pub struct Probe;").unwrap();
    let owner_text = mutation_authority_relative(&workspace, &owner).unwrap();
    let mut descriptor_value = fixture["descriptor"].clone();
    descriptor_value["owner"] = serde_json::Value::String(owner_text);
    fs::write(&descriptor, serde_json::to_vec(&descriptor_value).unwrap()).unwrap();
    fs::write(&taxonomy, r#"{"fileKinds":{"rust":{"emoji":"🦀️","extensionChains":[".rs"]},"json":{"emoji":"🔣️","extensionChains":[".json"]}},"mutationComponentFileKindId":"rust","mutationDescriptorFileKindId":"json","mutationBehaviorFacetDirs":["🦠️mutation","🔺️diff","↩️inverse"],"semanticCollections":{"🧬️mutations":{"kind":"mutation"}}}"#).unwrap();
    fs::write(workspace.join("📋️project.json"), r#"{"metadata":{"semio":{"taxonomy":"authority/🔣️taxonomy.json"}}}"#).unwrap();
    match case {
        "missing-locator" => fs::write(workspace.join("📋️project.json"), r#"{"metadata":{"semio":{}}}"#).unwrap(),
        "malformed-locator" => fs::write(workspace.join("📋️project.json"), r#"{"metadata":{"semio":{"taxonomy":"../authority/🔣️taxonomy.json"}}}"#).unwrap(),
        "wrong-root-pair" => fs::remove_file(workspace.join("nx.json")).unwrap(),
        "wrong-primary-filename" => {
            const HISTORICAL_PRIMARY_FILENAME: &str = "component.rs";
            let wrong = owner.join(HISTORICAL_PRIMARY_FILENAME);
            fs::rename(&source, &wrong).unwrap();
            return (workspace.clone(), workspace, wrong);
        }
        "owner-mismatch" => fs::write(&descriptor, r#"{"owner":"other/🧬️mutations/🆕️insert-page"}"#).unwrap(),
        "valid-behavior-facet" => {
            let facet = owner.join("🦠️mutation");
            fs::create_dir_all(&facet).unwrap();
            let nested = facet.join("🦀️.rs");
            fs::rename(&source, &nested).unwrap();
            return (workspace.clone(), workspace, nested);
        }
        "wrong-behavior-facet" => {
            let facet = owner.join("🔺️diff");
            fs::create_dir_all(&facet).unwrap();
            let nested = facet.join("🦀️.rs");
            fs::rename(&source, &nested).unwrap();
            return (workspace.clone(), workspace, nested);
        }
        "nested-behavior-facet" => {
            let nested = owner.join("🦠️mutation/nested");
            fs::create_dir_all(&nested).unwrap();
            let nested = nested.join("🦀️.rs");
            fs::rename(&source, &nested).unwrap();
            return (workspace.clone(), workspace, nested);
        }
        "symlink-parent" => {
            let actual = mutation_root.join("actual-owner");
            fs::rename(&owner, &actual).unwrap();
            link_dir(&actual, &owner);
        }
        "symlink-source" => {
            let actual = owner.join("🦀️actual.rs");
            fs::rename(&source, &actual).unwrap();
            link_file(&actual, &source);
        }
        "symlink-descriptor" => {
            let actual = owner.join("🔣️actual.json");
            fs::rename(&descriptor, &actual).unwrap();
            link_file(&actual, &descriptor);
        }
        "symlink-taxonomy" => {
            let actual = workspace.join("authority/🔣️actual.json");
            fs::rename(&taxonomy, &actual).unwrap();
            link_file(&actual, &taxonomy);
        }
        "nested-nx-anchor" => {
            let nested = workspace.join("nested");
            fs::create_dir_all(&nested).unwrap();
            fs::write(nested.join("nx.json"), "{}").unwrap();
            fs::rename(workspace.join("domain"), nested.join("domain")).unwrap();
            return (workspace.clone(), workspace, nested.join("domain/🧬️mutations/🆕️insert-page/🦀️.rs"));
        }
        "symlink-ancestor" => {
            let link = workspace.join("domain-link");
            link_dir(&workspace.join("domain"), &link);
            return (workspace.clone(), workspace, link.join("🧬️mutations/🆕️insert-page/🦀️.rs"));
        }
        "symlink-parent-erasure" => {
            let link = workspace.join("erased");
            link_dir(&workspace.join("domain"), &link);
            return (workspace.clone(), workspace, PathBuf::from("erased/../domain/🧬️mutations/🆕️insert-page/🦀️.rs"));
        }
        "virtual-compose" => return (workspace.clone(), workspace.clone(), workspace.join("compose/🧬️mutations/🆕️insert-page/🦀️.rs")),
        "raw-case-folded-compose-parent" => return (workspace.clone(), workspace, PathBuf::from("consumer/../COMPOSE/../domain/🧬️mutations/🆕️insert-page/🦀️.rs")),
        "file-parent-erasure" => {
            fs::write(workspace.join("not-a-directory"), "file").unwrap();
            return (workspace.clone(), workspace, PathBuf::from("not-a-directory/../domain/🧬️mutations/🆕️insert-page/🦀️.rs"));
        }
        "raw-nonutf8" => {
            #[cfg(unix)]
            {
                use std::os::unix::ffi::OsStringExt;
                return (workspace.clone(), workspace, PathBuf::from(std::ffi::OsString::from_vec(b"consumer/\xff".to_vec())));
            }
            #[cfg(windows)]
            {
                use std::os::windows::ffi::OsStringExt;
                return (workspace.clone(), workspace, PathBuf::from(std::ffi::OsString::from_wide(&[0xD800])));
            }
        }
        "valid-relative-parent" => {
            fs::create_dir_all(workspace.join("consumer")).unwrap();
            return (workspace.clone(), workspace, PathBuf::from("consumer/../domain/🧬️mutations/🆕️insert-page/🦀️.rs"));
        }
        _ => {}
    }
    (workspace.clone(), workspace, source)
}

#[test]
fn validates_mutation_source_authority_fixture() {
    let fixture = fixture();
    assert_eq!(fixture["schemaVersion"], 1);
    let mut descriptor_keys: Vec<&str> = fixture["descriptor"].as_object().unwrap().keys().map(String::as_str).collect();
    descriptor_keys.sort_unstable();
    assert_eq!(
        descriptor_keys,
        ["aggregateVariant", "binaryTag", "composition", "diffParticipation", "displayName", "emoji", "invertibility", "outcomeClasses", "owner", "payloadSchema", "requiredLanguageSurfaces", "schemaVersion", "semanticKind", "textOpcode"]
    );
    for vector in fixture["cases"].as_array().unwrap() {
        let name = vector["name"].as_str().unwrap();
        let (workspace, compiler_cwd, source) = materialize(name, &fixture);
        let result = mutation_source_authority(&source, &compiler_cwd);
        assert_eq!(result.is_ok(), vector["accepted"].as_bool().unwrap(), "{name}: {result:?}");
        #[cfg(any(unix, windows))]
        if name == "raw-nonutf8" {
            assert!(result.as_ref().unwrap_err().contains("not UTF-8"));
        }
        if name == "nested-nx-anchor" {
            assert!(result.as_ref().unwrap_err().contains("lacks paired"));
        }
        if name == "file-parent-erasure" {
            assert_eq!(fs::metadata(compiler_cwd.join(&source)).unwrap_err().kind(), std::io::ErrorKind::NotADirectory);
        }
        if let Ok(facts) = result {
            assert_eq!(facts.workspace_root, workspace);
            assert!(facts.mutation_root.ends_with("domain/🧬️mutations"));
            assert!(facts.owner.ends_with("🆕️insert-page"));
            assert_eq!(facts.source_path.file_name().and_then(|name| name.to_str()), Some("🦀️.rs"));
            assert_eq!(facts.descriptor_path.file_name().and_then(|name| name.to_str()), Some("🔣️.json"));
            assert!(facts.taxonomy_path.ends_with("authority/🔣️taxonomy.json"));
        }
    }
}

#[test]
fn validates_exact_domain_mutation_source_authority_fixture() {
    fn string_literals(tokens: proc_macro2::TokenStream) -> Vec<String> {
        tokens
            .into_iter()
            .flat_map(|token| match token {
                proc_macro2::TokenTree::Group(group) => string_literals(group.stream()),
                proc_macro2::TokenTree::Literal(literal) => syn::parse_str::<syn::LitStr>(&literal.to_string()).map(|value| vec![value.value()]).unwrap_or_default(),
                _ => Vec::new(),
            })
            .collect()
    }
    let domain_fixture: serde_json::Value = serde_json::from_str(include_str!("../🛂️mutation-source-authority/🧫️fixtures/🧭️domains.json")).unwrap();
    for vector in domain_fixture["cases"].as_array().unwrap() {
        let (workspace, _, _) = materialize(vector["name"].as_str().unwrap(), &fixture());
        let mutation_root = workspace.join(domain_fixture["mutationRoot"].as_str().unwrap());
        let owner = mutation_root.join(vector["owner"].as_str().unwrap());
        let source = owner.join(vector["source"].as_str().unwrap());
        fs::create_dir_all(source.parent().unwrap()).unwrap();
        fs::write(&source, "pub struct Probe;").unwrap();
        let mut descriptor = fixture()["descriptor"].clone();
        descriptor["owner"] = mutation_authority_relative(&workspace, &owner).unwrap().into();
        descriptor["semanticKind"] = vector["semanticKind"].clone();
        fs::write(owner.join("🔣️.json"), serde_json::to_vec(&descriptor).unwrap()).unwrap();
        let taxonomy_path = workspace.join("authority/🔣️taxonomy.json");
        let mut taxonomy: serde_json::Value = serde_json::from_slice(&fs::read(&taxonomy_path).unwrap()).unwrap();
        let mut domains = domain_fixture["domains"].clone();
        let mut root = domain_fixture["mutationRoot"].as_str().unwrap();
        match vector["fault"].as_str() {
            Some("duplicate-identity") => domains["🎥️camera"]["🌱️create"] = "reorder-cameras".into(),
            Some("empty-registry") => domains = serde_json::json!({}),
            Some("wrong-registry-root") => root = "foreign/🧬️mutations",
            Some("symlink-domain") => {
                let actual = mutation_root.join("actual-domain");
                fs::rename(mutation_root.join("🎥️camera"), &actual).unwrap();
                link_dir(&actual, &mutation_root.join("🎥️camera"));
            }
            None => (),
            Some(other) => panic!("unknown domain fixture fault {other}"),
        }
        taxonomy["mutationDomainOwners"] = serde_json::json!({ root: domains });
        fs::write(&taxonomy_path, serde_json::to_vec(&taxonomy).unwrap()).unwrap();
        let result = mutation_source_authority(&source, &workspace);
        assert_eq!(result.is_ok(), vector["accepted"].as_bool().unwrap(), "{}: {result:?}", vector["name"]);
        if let Ok(authority) = result {
            assert_eq!(authority.mutation_root, mutation_root);
            assert_eq!(authority.owner, descriptor["owner"].as_str().unwrap());
            assert_eq!(authority.source_path, source);
            let aggregate_source = mutation_root.join("🦀️.rs");
            fs::write(&aggregate_source, "pub enum Operations {}").unwrap();
            let aggregate = mutation_aggregate_source_authority(&aggregate_source, &workspace).unwrap();
            let operations = aggregate.domain_operations.as_ref().expect("explicit domain roster");
            assert_eq!(operations.len(), 4);
            assert!(operations.iter().any(|(owner, identity)| owner == &authority.owner && identity == descriptor["semanticKind"].as_str().unwrap()));
            let input: DeriveInput =
                syn::parse_str("#[mutations(snapshot = Snapshot, diff = Diff, schema = \"probe\")] enum Operations { CreateCamera(Create), ReorderCameras(Reorder), ChangeNodeName(ChangeName), BindNodeCamera(BindCamera) }").unwrap();
            let tokens = expand_mutations(&input, &aggregate).unwrap();
            syn::parse2::<syn::File>(tokens.clone()).expect("independent syntax validates emitted domain scope");
            let emitted = tokens.to_string();
            assert!(emitted.contains("MutationOwnerLayout :: DomainOperations"));
            assert!(!emitted.contains("MutationOwnerLayout :: Flat"));
            let literals = string_literals(tokens);
            for (owner, identity) in operations {
                assert!(literals.contains(owner), "missing owner {owner}");
                assert!(literals.contains(identity), "missing identity {identity}");
            }
        }
    }
}
