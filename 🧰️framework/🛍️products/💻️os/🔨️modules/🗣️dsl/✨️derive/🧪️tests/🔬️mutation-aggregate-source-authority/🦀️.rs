
use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🏛️mutation-aggregate-source-authority/🔣️.json")).unwrap()
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

fn materialize(name: &str) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let leaf_fixture = mutation_source_authority_tests::fixture();
    let (workspace, _, leaf) = mutation_source_authority_tests::materialize("valid", &leaf_fixture);
    let mutation_root = leaf.parent().unwrap().parent().unwrap().to_path_buf();
    let aggregate = mutation_root.join("🦀️.rs");
    fs::write(&aggregate, "pub enum Probe {}").unwrap();
    match name {
        "compiler-relative" => {
            fs::create_dir_all(workspace.join("consumer")).unwrap();
            return (workspace.clone(), workspace.clone(), PathBuf::from("consumer/../domain/🧬️mutations/🦀️.rs"), leaf);
        }
        "parent-mounted" => {
            let cwd = workspace.parent().unwrap().to_path_buf();
            return (workspace.clone(), cwd, PathBuf::from(workspace.file_name().unwrap()).join("domain/🧬️mutations/🦀️.rs"), leaf);
        }
        "leaf-primary" => return (workspace.clone(), workspace, leaf.clone(), leaf),
        "historical-primary" => {
            const HISTORICAL_PRIMARY_FILENAME: &str = "component.rs";
            let historical = mutation_root.join(HISTORICAL_PRIMARY_FILENAME);
            fs::rename(&aggregate, &historical).unwrap();
            return (workspace.clone(), workspace, historical, leaf);
        }
        "outside-root" => {
            let outside_root = workspace.join("outside");
            let outside = outside_root.join("🦀️.rs");
            fs::create_dir_all(&outside_root).unwrap();
            fs::write(&outside, "pub enum Probe {}").unwrap();
            return (workspace, outside_root, outside, leaf);
        }
        "virtual-compose" => {
            let source = workspace.join("compose/🧬️mutations/🦀️.rs");
            return (workspace.clone(), workspace, source, leaf);
        }
        "nested-nx-anchor" => {
            let nested = workspace.join("nested");
            fs::create_dir_all(&nested).unwrap();
            fs::write(nested.join("nx.json"), "{}").unwrap();
            fs::rename(workspace.join("domain"), nested.join("domain")).unwrap();
            return (workspace.clone(), workspace, nested.join("domain/🧬️mutations/🦀️.rs"), leaf);
        }
        "taxonomy-filename-change" => {
            let changed = mutation_root.join("🐹️.go");
            fs::rename(&aggregate, &changed).unwrap();
            fs::write(workspace.join("authority/🔣️taxonomy.json"), r#"{"fileKinds":{"go":{"emoji":"🐹️","extensionChains":[".go"]},"json":{"emoji":"🔣️","extensionChains":[".json"]}},"mutationComponentFileKindId":"go","mutationDescriptorFileKindId":"json","mutationBehaviorFacetDirs":["🦠️mutation","🔺️diff","↩️inverse"],"semanticCollections":{"🧬️mutations":{"kind":"mutation"}}}"#).unwrap();
            return (workspace.clone(), workspace, changed, leaf);
        }
        "symlink-source" => {
            let actual = mutation_root.join("🦀️actual.rs");
            fs::rename(&aggregate, &actual).unwrap();
            link_file(&actual, &aggregate);
        }
        "symlink-root" => {
            let actual = workspace.join("domain/actual-mutations");
            fs::rename(&mutation_root, &actual).unwrap();
            link_dir(&actual, &mutation_root);
        }
        _ => {}
    }
    (workspace.clone(), workspace, aggregate, leaf)
}

#[test]
fn validates_schema_first_aggregate_authority_fixture() {
    let fixture = fixture();
    assert_eq!(fixture["schemaVersion"], 1);
    for vector in fixture["cases"].as_array().unwrap() {
        let name = vector["name"].as_str().unwrap();
        let (workspace, cwd, source, _) = materialize(name);
        let result = mutation_aggregate_source_authority(&source, &cwd);
        assert_eq!(result.is_ok(), vector["accepted"].as_bool().unwrap(), "{name}: {result:?}");
        if let Some(diagnostic) = vector.get("diagnostic").and_then(serde_json::Value::as_str) {
            assert!(result.as_ref().unwrap_err().contains(diagnostic), "{name}: {result:?}");
        }
        if let Ok(facts) = result {
            assert_eq!(facts.workspace_root, workspace);
            assert!(facts.mutation_root.ends_with("domain/🧬️mutations"));
            assert!(facts.source_path.ends_with(facts.source_filename.as_str()));
            assert!(!facts.descriptor_filename.is_empty());
            assert!(facts.taxonomy_path.ends_with("authority/🔣️taxonomy.json"));
        }
    }
}

#[test]
fn aggregate_and_leaf_authority_share_workspace_taxonomy_names_and_token() {
    let (workspace, cwd, aggregate_source, leaf_source) = materialize("direct-canonical");
    let aggregate = mutation_aggregate_source_authority(&aggregate_source, &cwd).unwrap();
    let leaf = mutation_source_authority(&leaf_source, &cwd).unwrap();
    let leaf_common = mutation_authority_common(&leaf_source, &cwd).unwrap();
    assert_eq!(aggregate.workspace_root, leaf.workspace_root);
    assert_eq!(aggregate.mutation_root, leaf.mutation_root);
    assert_eq!(aggregate.taxonomy_path, leaf.taxonomy_path);
    assert_eq!(aggregate.source_filename, leaf_common.source_filename);
    assert_eq!(aggregate.descriptor_filename, leaf_common.descriptor_filename);
    assert_eq!(mutation_authority_workspace_token(&aggregate.workspace_root, &aggregate.taxonomy_path).unwrap(), mutation_leaf_workspace_token(&leaf).unwrap());
    let (_, other_cwd, other_aggregate_source, _) = materialize("direct-canonical");
    let other = mutation_aggregate_source_authority(&other_aggregate_source, &other_cwd).unwrap();
    assert_ne!(mutation_authority_workspace_token(&aggregate.workspace_root, &aggregate.taxonomy_path).unwrap(), mutation_authority_workspace_token(&other.workspace_root, &other.taxonomy_path).unwrap());
    assert!(workspace.exists());
}

#[test]
fn validates_explicit_aggregate_component_sources() {
    fn literals(tokens: proc_macro2::TokenStream) -> Vec<String> {
        tokens
            .into_iter()
            .flat_map(|token| match token {
                proc_macro2::TokenTree::Group(group) => literals(group.stream()),
                proc_macro2::TokenTree::Literal(literal) => syn::parse_str::<syn::LitStr>(&literal.to_string()).map(|value| vec![value.value()]).unwrap_or_default(),
                _ => Vec::new(),
            })
            .collect()
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏛️mutation-aggregate-source-authority/🧩️sources.json")).unwrap();
    for vector in fixture["cases"].as_array().unwrap() {
        let name = vector["name"].as_str().unwrap();
        let (workspace, cwd, source, _) = materialize("direct-canonical");
        for root in fixture["sources"].as_array().unwrap() {
            fs::create_dir_all(workspace.join(root.as_str().unwrap())).unwrap();
        }
        let mut sources = fixture["sources"].clone();
        let mut aggregate = fixture["aggregate"].as_str().unwrap();
        match name {
            "empty-sources" => sources = serde_json::json!([]),
            "duplicate-source" => sources[1] = sources[0].clone(),
            "unsafe-source" => sources[0] = "graph/../foreign/🧬️mutations".into(),
            "non-mutation-source" => sources[0] = "graph".into(),
            "missing-source" => sources[0] = "missing/🧬️mutations".into(),
            "symlink-source" => {
                fs::rename(workspace.join("graph/🧬️mutations"), workspace.join("graph/actual")).unwrap();
                link_dir(&workspace.join("graph/actual"), &workspace.join("graph/🧬️mutations"));
            }
            "non-array-sources" => sources = serde_json::json!({}),
            "non-string-source" => sources[0] = 1.into(),
            "unsafe-aggregate" => aggregate = "../domain/🧬️mutations",
            "explicit-sources" => (),
            _ => panic!("unknown component source fixture {name}"),
        }
        let taxonomy_path = workspace.join("authority/🔣️taxonomy.json");
        let mut taxonomy: serde_json::Value = serde_json::from_slice(&fs::read(&taxonomy_path).unwrap()).unwrap();
        taxonomy["mutationAggregateSources"] = serde_json::json!({ aggregate: sources });
        fs::write(&taxonomy_path, serde_json::to_vec(&taxonomy).unwrap()).unwrap();
        let result = mutation_aggregate_source_authority(&source, &cwd);
        assert_eq!(result.is_ok(), vector["accepted"].as_bool().unwrap(), "{name}: {result:?}");
        if let Ok(authority) = result {
            let input: DeriveInput = syn::parse_str("#[mutations(snapshot = Snapshot, diff = Diff, schema = \"probe\")] enum Operations { CreateNode(Create), MovePoint(Move) }").unwrap();
            let tokens = expand_mutations(&input, &authority).unwrap();
            syn::parse2::<syn::File>(tokens.clone()).expect("independent Rust parser accepts emitted component scopes");
            let emitted = literals(tokens);
            for root in fixture["sources"].as_array().unwrap() {
                assert!(emitted.iter().any(|value| value == root.as_str().unwrap()), "missing explicit source {root}");
            }
        }
    }
}
