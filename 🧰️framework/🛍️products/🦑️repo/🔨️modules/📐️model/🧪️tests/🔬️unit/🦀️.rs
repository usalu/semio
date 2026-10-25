//! 🔬️ Cargo tests of the repo model. The slug vocabulary, the kind derivations and the section and
//! definition ids are judged against `🧫️fixtures/🔬️unit-vectors.json`, which an independent Python model
//! computed; the wire shapes against the real documents of `🔣️json-encoding-conformance/🔣️goldens.json`
//! with `serde_json` as the third-party reader; the allowed tables against the schema file itself.

use super::*;
use semio_framework_test_fixtures::fixture_path;
use serde_json::{json, Value};

//#region 📥️Fixtures
fn fixture(relative: &str) -> Value {
    let mut path = fixture_path!("..").join("..").join("🧪️tests").join("🧫️fixtures");
    for segment in relative.split('/') {
        path = path.join(segment);
    }
    let bytes = std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_slice(&bytes).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn answers() -> Value {
    fixture("🔬️unit-vectors.json")
}

fn rows<'a>(value: &'a Value, key: &str) -> &'a Vec<Value> {
    value[key].as_array().unwrap_or_else(|| panic!("{key} is not an array"))
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_else(|| panic!("member {key} is not a string in {value}"))
}

fn strings(value: &Value, key: &str) -> Vec<String> {
    rows(value, key).iter().map(|item| item.as_str().unwrap().to_string()).collect()
}
//#endregion 📥️Fixtures

//#region 🔁️WireRoundTrip
#[test]
fn every_golden_document_decodes_and_re_encodes_to_the_same_bytes() {
    let goldens = fixture("🔣️json-encoding-conformance/🔣️goldens.json");
    let names = wire_type_names();
    for golden in rows(&goldens, "goldens") {
        let (kind, source) = (text(golden, "type"), text(golden, "json"));
        assert!(names.contains(&kind), "{kind} is not a wire type");
        assert_eq!(round_trip_json(kind, source).unwrap(), source, "{kind}");
        let original: Value = serde_json::from_str(source).unwrap();
        let again: Value = serde_json::from_str(&round_trip_json(kind, source).unwrap()).unwrap();
        assert_eq!(again, original, "{kind} differs structurally for serde_json");
    }
}

#[test]
fn every_wire_type_has_a_golden_document() {
    let goldens = fixture("🔣️json-encoding-conformance/🔣️goldens.json");
    let covered: Vec<&str> = rows(&goldens, "goldens").iter().map(|golden| text(golden, "type")).collect();
    let missing: Vec<&&str> = wire_type_names().iter().filter(|name| !covered.contains(name)).collect();
    assert!(missing.is_empty(), "wire types without a golden: {missing:?}");
}

#[test]
fn an_unknown_type_and_a_malformed_document_are_refused() {
    assert_eq!(round_trip_json("NoSuchType", "{}").unwrap_err(), "unknown domain type NoSuchType");
    assert!(round_trip_json("Range", "{\"start\":1}").is_err());
    assert!(round_trip_json("Range", "not json").is_err());
    assert!(round_trip_json("Range", "{\"start\":\"a\",\"end\":2}").is_err());
}

#[test]
fn omitempty_members_vanish_and_nil_collections_encode_as_null() {
    assert_eq!(round_trip_json("GoalDates", "{}").unwrap(), "{}");
    assert_eq!(round_trip_json("GoalDates", "{\"due\":\"\"}").unwrap(), "{}");
    assert_eq!(round_trip_json("GoalManagementData", "{\"milestone\":\"m\"}").unwrap(), "{\"milestone\":\"m\"}");
    assert_eq!(round_trip_json("DiffLines", "{\"Added\":null,\"Removed\":[3,4]}").unwrap(), "{\"Added\":null,\"Removed\":[3,4]}");
}

#[test]
fn map_keys_encode_in_sorted_order_like_go() {
    let node = json!({"Kind": "file", "ID": "i", "Label": "l", "URI": "u", "SubKind": "", "Description": "", "Summary": "", "Year": 0, "Month": 0, "Day": 0, "Status": "", "Contributor": "", "Data": {"b": 1, "a": 2, "c": [3]}, "Children": null});
    let encoded = round_trip_json("TreeNode", &node.to_string()).unwrap();
    assert!(encoded.contains("\"Data\":{\"a\":2,\"b\":1,\"c\":[3]}"), "{encoded}");
}
//#endregion 🔁️WireRoundTrip

//#region 📋️AllowedAndSlugs
#[test]
fn the_allowed_tables_are_the_schema_file_in_declaration_order() {
    let schema = fixture("../../🧬️schema/🔣️allowed-values.json");
    assert_eq!(allowed_llms(), strings(&schema, "llms").as_slice());
    assert_eq!(allowed_efforts(), strings(&schema, "efforts").as_slice());
    assert_eq!(allowed_clients(), strings(&schema, "clients").as_slice());
    for table in [allowed_llms(), allowed_efforts(), allowed_clients()] {
        assert!(!table.is_empty());
        let mut sorted = table.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), table.len(), "duplicate entries in {table:?}");
        assert!(table.iter().all(|entry| normalize_llm_slug(entry) == *entry), "an allowed entry is not in canonical form");
    }
}

fn normaliser(field: &str) -> fn(&str) -> String {
    match field {
        "llm" => normalize_llm_slug,
        "effort" => normalize_effort_slug,
        _ => normalize_client_slug,
    }
}

fn resolver(field: &str) -> fn(&str) -> Result<String, ModelError> {
    match field {
        "llm" => resolve_allowed_llm,
        "effort" => resolve_allowed_effort,
        _ => resolve_allowed_client,
    }
}

#[test]
fn every_slug_vector_normalises_like_the_independent_model_and_is_idempotent() {
    let answers = answers();
    for field in ["llm", "effort", "client"] {
        for row in rows(&answers["slugs"][field], "normalise") {
            let slug = normaliser(field)(text(row, "input"));
            assert_eq!(slug, text(row, "slug"), "{field} {:?}", row["input"]);
            assert_eq!(normaliser(field)(&slug), slug, "{field} {slug:?} is not idempotent");
        }
    }
}

#[test]
fn every_resolvable_vector_picks_the_independent_longest_match() {
    let answers = answers();
    for field in ["llm", "effort", "client"] {
        for row in rows(&answers["slugs"][field], "resolve") {
            let resolved = resolver(field)(text(row, "input")).unwrap_or_else(|error| panic!("{field} {:?}: {error}", row["input"]));
            assert_eq!(Some(resolved.as_str()), row["resolved"].as_str(), "{field} {:?}", row["input"]);
        }
    }
}

#[test]
fn every_unresolvable_vector_is_refused_with_the_not_allowed_class() {
    let answers = answers();
    for field in ["llm", "effort", "client"] {
        for row in rows(&answers["slugs"][field], "reject") {
            assert!(row["resolved"].is_null(), "the model itself resolved {row}");
            let error = resolver(field)(text(row, "input")).unwrap_err();
            assert_eq!(error.class(), "not-allowed", "{field} {:?}", row["input"]);
        }
    }
}

#[test]
fn a_refusal_names_the_field_the_slug_and_the_whole_vocabulary() {
    let message = resolve_allowed_effort("extreme").unwrap_err().to_string();
    assert_eq!(message, format!("effort 'extreme' is not allowed. Please use one of: {}", allowed_efforts().join(", ")));
    assert!(resolve_allowed_llm("not-a-model").unwrap_err().to_string().starts_with("llm 'not-a-model' is not allowed. Please use one of: opus-4-7, "));
}

#[test]
fn a_blank_effort_resolves_to_blank_but_a_blank_llm_is_refused() {
    assert_eq!(resolve_allowed_effort("   ").unwrap(), "");
    assert!(resolve_allowed_llm("   ").is_err());
    assert!(resolve_allowed_client("").is_err());
}

#[test]
fn slugify_splits_camel_case_and_keeps_only_ascii_words() {
    for (input, expected) in [("XMLHttpRequest", "XML-HTTP-REQUEST"), ("aBcDeF", "A-BC-DE-F"), ("SWE-1.5 Evaluation", "SWE-1-5-EVALUATION"), ("Grüße", "GR-E"), ("---", ""), ("42", "42"), ("", "")] {
        assert_eq!(slugify(input), expected, "{input:?}");
    }
}
//#endregion 📋️AllowedAndSlugs

//#region 🏷️Kinds
#[test]
fn every_keyword_derives_the_independent_definition_kind() {
    let answers = answers();
    assert!(rows(&answers, "definitionKinds").len() >= 40);
    for row in rows(&answers, "definitionKinds") {
        assert_eq!(derive_definition_kind(text(row, "keyword")).as_str(), text(row, "kind"), "{:?}", row["keyword"]);
    }
}

#[test]
fn every_file_name_derives_the_independent_file_kind() {
    let answers = answers();
    for row in rows(&answers, "fileKinds") {
        assert_eq!(derive_file_kind(text(row, "name")).as_str(), text(row, "kind"), "{:?}", row["name"]);
    }
}

#[test]
fn every_technology_name_derives_the_independent_kind_and_the_emoji_is_the_wire_value() {
    let answers = answers();
    for row in rows(&answers, "technologyKinds") {
        let kind = derive_technology_kind(text(row, "name"));
        let expected = match text(row, "kind") {
            "user" => TechnologyKind::User,
            "infrastructure" => TechnologyKind::Infrastructure,
            "research" => TechnologyKind::Research,
            other => panic!("model answered {other}"),
        };
        assert_eq!(kind, expected, "{:?}", row["name"]);
        assert_eq!(serde_json::to_value(kind).unwrap(), Value::String(kind.as_str().to_string()));
    }
}

#[test]
fn test_function_names_follow_the_go_conventions() {
    let answers = answers();
    for row in rows(&answers, "testFunctionNames") {
        assert_eq!(is_test_function_name(text(row, "name")), row["test"].as_bool().unwrap(), "{:?}", row["name"]);
    }
}

#[test]
fn every_enumeration_renders_as_its_wire_string() {
    fn check<T: serde::Serialize + fmt::Display + Copy>(value: T, wire: &str) {
        assert_eq!(serde_json::to_value(value).unwrap(), Value::String(wire.to_string()));
        assert_eq!(value.to_string(), wire);
    }
    check(DefinitionKind::Test, "test");
    check(TicketStatus::Closed, "closed");
    check(GoalStatus::Open, "open");
    check(BreachPriority::High, "high");
    check(BundleKind::Assets, "assets");
    check(FolderKind::Required, "required");
    check(FileKind::Template, "template");
    check(SemanticChangeType::Renamed, "renamed");
    check(TreeNodeKind::Statute, "statute");
    check(CbTreeNodeKind::Definition, "definition");
    check(TechnologyKind::Research, "\u{1F52C}\u{FE0F}");
    assert_eq!(DefinitionKind::default(), DefinitionKind::Implementation);
    assert_eq!(BundleKind::default(), BundleKind::Library);
    assert_eq!(BreachPriority::default(), BreachPriority::Low);
}

#[test]
fn kind_emojis_come_from_the_shared_entity_table() {
    let table = fixture("../../../🪪️identity/🧬️schema/🔣️entity-emojis.json");
    let entity = |name: &str| text(&table["entities"], name).to_string();
    let bare = |value: String| value.replace('\u{FE0F}', "");
    assert_eq!(bare(file_kind_emoji(FileKind::Lab)), entity("file-lab"));
    assert_eq!(bare(file_kind_emoji(FileKind::License)), entity("file-license"));
    assert_eq!(bare(folder_kind_emoji(FolderKind::Root)), entity("folder-root"));
    assert_eq!(bare(definition_kind_emoji(DefinitionKind::Constant)), entity("definition-constant"));
    assert_eq!(bare(definition_kind_emoji(DefinitionKind::Implementation)), entity("definition-impl"));
    let mut bundle: Bundle = serde_json::from_value(json!({"name": "b", "root": "r", "technologyName": "t", "kind": "schema", "emoji": ""})).unwrap();
    assert_eq!(bare(bundle_kind_emoji(&bundle)), entity("bundle-schema"));
    bundle.emoji = "🌟".to_string();
    assert_eq!(bundle_kind_emoji(&bundle), "🌟");
    let technology: Technology = serde_json::from_value(json!({"name": "repo", "root": "r", "kind": "🧰️", "emoji": "", "bundles": null})).unwrap();
    assert_eq!(bare(technology_kind_emoji(&technology)), entity("technology-infrastructure"));
}

#[test]
fn every_section_and_definition_id_matches_the_independent_model() {
    let answers = answers();
    for row in rows(&answers, "sectionIds") {
        assert_eq!(build_section_id(text(row, "fileId"), &strings(row, "path")), text(row, "id"), "{row}");
    }
    for row in rows(&answers, "definitionIds") {
        let kind = match text(row, "kind") {
            "interface" => DefinitionKind::Interface,
            "constant" => DefinitionKind::Constant,
            "test" => DefinitionKind::Test,
            _ => DefinitionKind::Implementation,
        };
        assert_eq!(build_definition_id(text(row, "fileId"), &strings(row, "path"), text(row, "name"), kind), text(row, "id"), "{row}");
    }
}
//#endregion 🏷️Kinds

//#region 🧹️TreeFilter
fn kinds(entries: &[(TreeNodeKind, bool)]) -> Option<BTreeMap<TreeNodeKind, bool>> {
    Some(entries.iter().cloned().collect())
}

fn words(items: &[&str]) -> Option<Vec<String>> {
    Some(items.iter().map(|item| item.to_string()).collect())
}

#[test]
fn an_empty_filter_shows_everything() {
    let filter = TreeFilter::default();
    for kind in [TreeNodeKind::Technology, TreeNodeKind::Goal, TreeNodeKind::Category] {
        assert!(filter.is_kind_visible(kind));
    }
    assert!(filter.matches_sub_kind(TreeNodeKind::File, "code") && filter.matches_date(2026, 10, 25) && filter.matches_status("open") && filter.matches_contributor("anyone"));
    assert!(!filter.has_only_kinds());
}

#[test]
fn only_kinds_is_an_allow_list_and_a_category_is_always_shown() {
    let filter = TreeFilter { only_kinds: kinds(&[(TreeNodeKind::Goal, true), (TreeNodeKind::Ticket, false)]), exclude_kinds: kinds(&[(TreeNodeKind::Goal, true)]), ..Default::default() };
    assert!(filter.has_only_kinds());
    assert!(filter.is_kind_visible(TreeNodeKind::Goal));
    assert!(!filter.is_kind_visible(TreeNodeKind::Ticket));
    assert!(!filter.is_kind_visible(TreeNodeKind::File));
    assert!(filter.is_kind_visible(TreeNodeKind::Category));
}

#[test]
fn excluded_kinds_hide_exactly_those_kinds() {
    let filter = TreeFilter { exclude_kinds: kinds(&[(TreeNodeKind::Todo, true), (TreeNodeKind::Draft, false)]), ..Default::default() };
    assert!(!filter.is_kind_visible(TreeNodeKind::Todo));
    assert!(filter.is_kind_visible(TreeNodeKind::Draft) && filter.is_kind_visible(TreeNodeKind::Goal));
}

#[test]
fn sub_kinds_compare_case_insensitively_and_a_blank_one_always_survives() {
    let only = TreeFilter { only_sub_kinds: Some(BTreeMap::from([(TreeNodeKind::File, vec!["Code".to_string()])])), ..Default::default() };
    assert!(only.matches_sub_kind(TreeNodeKind::File, "CODE") && !only.matches_sub_kind(TreeNodeKind::File, "docs"));
    assert!(only.matches_sub_kind(TreeNodeKind::File, "") && only.matches_sub_kind(TreeNodeKind::Folder, "docs"));
    let exclude = TreeFilter { exclude_sub_kinds: Some(BTreeMap::from([(TreeNodeKind::File, vec!["docs".to_string()])])), ..Default::default() };
    assert!(!exclude.matches_sub_kind(TreeNodeKind::File, "DOCS") && exclude.matches_sub_kind(TreeNodeKind::File, "code"));
}

#[test]
fn dates_test_the_allow_lists_only_for_positive_months_and_days() {
    let filter = TreeFilter { only_years: Some(vec![2026]), only_months: Some(vec![10]), only_days: Some(vec![25]), exclude_days: Some(vec![13]), ..Default::default() };
    assert!(filter.matches_date(2026, 10, 25));
    assert!(!filter.matches_date(2025, 10, 25) && !filter.matches_date(2026, 9, 25) && !filter.matches_date(2026, 10, 24));
    assert!(filter.matches_date(2026, 0, 0), "a zero month and day are not tested against the allow lists");
    let excluded = TreeFilter { exclude_months: Some(vec![0]), exclude_years: Some(vec![1999]), ..Default::default() };
    assert!(!excluded.matches_date(2026, 0, 5), "a zero month is still tested against the exclusion list");
    assert!(!excluded.matches_date(1999, 1, 1));
}

#[test]
fn status_and_contributor_filters_compare_case_insensitively() {
    let filter = TreeFilter { only_status: "Open".to_string(), only_contributors: words(&["Ueli"]), exclude_contributors: words(&["bot"]), ..Default::default() };
    assert!(filter.matches_status("open") && !filter.matches_status("closed"));
    assert!(filter.matches_contributor("UELI") && !filter.matches_contributor("bot") && !filter.matches_contributor("other"));
    let excluding = TreeFilter { exclude_contributors: words(&["BOT"]), ..Default::default() };
    assert!(!excluding.matches_contributor("bot") && excluding.matches_contributor("ueli"));
}

#[test]
fn a_territory_lists_its_statutes_depth_first() {
    let leaf = |name: &str, kinds: &[&str]| Territory { name: name.to_string(), description: String::new(), scopes: Vec::new(), groups: Vec::new(), kinds: kinds.iter().map(|kind| Statute::from(*kind)).collect() };
    let mut parent = leaf("parent", &["a/one", "a/two"]);
    let mut child = leaf("child", &["b/one"]);
    child.groups.push(leaf("grandchild", &["c/one"]));
    parent.groups.push(child);
    parent.groups.push(leaf("sibling", &["d/one"]));
    let all: Vec<String> = parent.all_kinds().iter().map(Statute::to_string).collect();
    assert_eq!(all, vec!["a/one", "a/two", "b/one", "c/one", "d/one"]);
}
//#endregion 🧹️TreeFilter
