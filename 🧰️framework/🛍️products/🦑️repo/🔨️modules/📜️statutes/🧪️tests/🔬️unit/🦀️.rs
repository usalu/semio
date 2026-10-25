//! 🔬️ Cargo tests of the statutes crate. SHA-256, gzip and the spec-text scanners are judged against
//! `🧫️fixtures/🔬️unit-vectors.json`, which `hashlib`, CPython's `zlib`/`gzip` and `re` models of the Go
//! regular expressions produced; the catalog against the schema file read with `serde_json`; the analysis
//! against the reviewed golden breach list of the committed source tree.

use super::*;
use semio_framework_test_fixtures::fixture_path;
use serde_json::Value;
use std::path::{Path, PathBuf};

//#region 📥️Fixtures
fn fixtures_dir() -> PathBuf {
    fixture_path!("..").join("..").join("🧪️tests").join("🧫️fixtures")
}

fn fixture(relative: &str) -> Value {
    let path = relative.split('/').fold(fixtures_dir(), |path, segment| path.join(segment));
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

fn hex_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len() / 2).map(|index| u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).unwrap()).collect()
}

fn hex_text(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

const WORDS: [&str; 16] = ["alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta", "iota", "kappa", "lambda", "mu", "statute", "policy", "breach", "section"];

struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = (self.0 * 1103515245 + 12345) & 0x7FFF_FFFF;
        self.0 >> 8
    }
}

fn recipe_bytes(recipe: &Value) -> Vec<u8> {
    let count = recipe["count"].as_u64().unwrap_or(0) as usize;
    match text(recipe, "kind") {
        "text" => text(recipe, "text").as_bytes().to_vec(),
        "repeat" => vec![recipe["byte"].as_u64().unwrap() as u8; count],
        "bytes" => {
            let mut generator = Lcg(recipe["seed"].as_u64().unwrap());
            (0..count).map(|_| (generator.next() & 0xFF) as u8).collect()
        }
        "words" => {
            let mut generator = Lcg(recipe["seed"].as_u64().unwrap());
            let mut out = String::new();
            for index in 0..count {
                out.push_str(WORDS[(generator.next() % 16) as usize]);
                out.push(if index % 11 == 10 { '\n' } else { ' ' });
            }
            out.into_bytes()
        }
        other => panic!("unknown recipe kind {other}"),
    }
}
//#endregion 📥️Fixtures

//#region 🔏️Digest
#[test]
fn every_recipe_hashes_to_the_hashlib_digest() {
    let answers = answers();
    assert!(rows(&answers, "sha").len() >= 20);
    for row in rows(&answers, "sha") {
        let data = recipe_bytes(&row["recipe"]);
        assert_eq!(sha256_hex(&data), text(row, "sha256"), "{} bytes of {}", data.len(), row["recipe"]["kind"]);
    }
}

#[test]
fn the_published_fips_vectors_hold() {
    assert_eq!(sha256_hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    assert_eq!(sha256_hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"), "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1");
    assert_eq!(sha256(b"abc").len(), 32);
    assert_eq!(hex_text(&sha256(b"abc")), sha256_hex(b"abc"));
}
//#endregion 🔏️Digest

//#region 🗜️Gzip
#[test]
fn every_member_written_by_zlib_inflates_to_its_payload() {
    let answers = answers();
    let mut members = 0;
    for row in rows(&answers, "gzip") {
        let payload = recipe_bytes(&row["recipe"]);
        assert_eq!(payload.len() as u64, row["length"].as_u64().unwrap());
        for member in rows(row, "members") {
            let decoded = gzip_decode(&hex_bytes(text(member, "hex"))).unwrap_or_else(|error| panic!("{} of {}: {error}", text(member, "how"), row["recipe"]));
            assert!(decoded == payload, "{} of {} inflated to different bytes", text(member, "how"), row["recipe"]);
            members += 1;
        }
    }
    assert!(members >= 60, "only {members} members");
}

#[test]
fn the_encoder_writes_the_stored_member_layout_of_the_independent_model() {
    let answers = answers();
    for row in rows(&answers, "storedMembers") {
        let payload = recipe_bytes(&row["recipe"]);
        assert!(hex_text(&gzip_encode(&payload)) == text(row, "hex"), "stored member of {} differs", row["recipe"]);
    }
}

#[test]
fn the_encoder_trailer_carries_the_zlib_crc_and_the_length() {
    let answers = answers();
    for row in rows(&answers, "gzip") {
        let payload = recipe_bytes(&row["recipe"]);
        let member = gzip_encode(&payload);
        let trailer = &member[member.len() - 8..];
        assert_eq!(u32::from_le_bytes(trailer[..4].try_into().unwrap()) as u64, row["crc32"].as_u64().unwrap(), "{}", row["recipe"]);
        assert_eq!(u32::from_le_bytes(trailer[4..].try_into().unwrap()) as u64, row["length"].as_u64().unwrap());
        assert_eq!(&member[..4], &[0x1f, 0x8b, 0x08, 0x00]);
        assert_eq!(gzip_decode(&member).unwrap(), payload);
    }
}

#[test]
fn a_payload_beyond_one_stored_block_splits_into_blocks_of_at_most_65535_bytes() {
    let payload = vec![7u8; 65535 * 2 + 3];
    let member = gzip_encode(&payload);
    assert_eq!(member.len(), 10 + 3 * 5 + payload.len() + 8);
    assert_eq!(member[10], 0, "first block is not final");
    assert_eq!(member[10 + 5 + 65535], 0, "second block is not final");
    assert_eq!(member[10 + 2 * (5 + 65535)], 1, "third block is final");
    assert_eq!(gzip_decode(&member).unwrap(), payload);
}

#[test]
fn a_member_that_is_not_gzip_is_refused_before_anything_is_inflated() {
    assert_eq!(gzip_decode(b"").unwrap_err(), "not a gzip member");
    assert_eq!(gzip_decode(&[0u8; 40]).unwrap_err(), "not a gzip member");
    let mut member = gzip_encode(b"payload");
    member[2] = 9;
    assert_eq!(gzip_decode(&member).unwrap_err(), "unsupported gzip compression method");
}

#[test]
fn a_damaged_member_is_refused_never_trusted_and_never_a_panic() {
    let member = gzip_encode(b"a payload long enough to damage in several places");
    let end = member.len();
    let mut crc = member.clone();
    crc[end - 8] ^= 0xff;
    assert_eq!(gzip_decode(&crc).unwrap_err(), "gzip crc mismatch");
    let mut length = member.clone();
    length[end - 4] ^= 0x01;
    assert_eq!(gzip_decode(&length).unwrap_err(), "gzip length mismatch");
    let mut complement = member.clone();
    complement[10 + 3] ^= 0xff;
    assert_eq!(gzip_decode(&complement).unwrap_err(), "invalid stored block lengths");
    let mut reserved = member.clone();
    reserved[10] = 0b111;
    assert_eq!(gzip_decode(&reserved).unwrap_err(), "reserved deflate block type");
}

#[test]
fn every_strict_prefix_of_a_compressed_member_is_refused() {
    let answers = answers();
    let row = &rows(&answers, "gzip")[3];
    for member in rows(row, "members") {
        let bytes = hex_bytes(text(member, "hex"));
        for cut in 0..bytes.len() {
            assert!(gzip_decode(&bytes[..cut]).is_err(), "{} cut at {cut} of {} was accepted", text(member, "how"), bytes.len());
        }
    }
}

#[test]
fn every_single_bit_flip_of_a_dynamic_member_is_refused_or_changes_nothing_silently() {
    let answers = answers();
    let row = rows(&answers, "gzip").iter().find(|row| row["recipe"]["kind"] == "words" && row["recipe"]["count"] == 40).expect("words recipe");
    let payload = recipe_bytes(&row["recipe"]);
    let member = rows(row, "members").iter().find(|member| text(member, "how") == "zlib level 9 strategy 0").expect("level 9");
    let bytes = hex_bytes(text(member, "hex"));
    for index in 10..bytes.len() {
        for bit in 0..8 {
            let mut damaged = bytes.clone();
            damaged[index] ^= 1 << bit;
            if let Ok(decoded) = gzip_decode(&damaged) {
                assert_eq!(decoded, payload, "flipping bit {bit} of byte {index} produced a different payload that passed the checks");
            }
        }
    }
}
//#endregion 🗜️Gzip

//#region 🧷️BreachCache
fn envelope_text() -> String {
    text(&fixture("🗜️breach-cache-envelope/🔣️vectors.json"), "envelope").to_string()
}

#[test]
fn the_envelope_keeps_its_canonical_json_and_its_hashlib_digest() {
    let answers = answers();
    let document = envelope_text();
    let envelope = parse_breach_cache(&document).unwrap();
    assert_eq!(envelope.entity_id, serde_json::from_str::<Value>(&document).unwrap()["entityId"].as_str().unwrap());
    assert_eq!(envelope.script, "lint");
    assert_eq!(envelope.breachs.len(), 1);
    assert_eq!(encode_breach_cache_json(&envelope).unwrap(), document);
    assert_eq!(breach_cache_digest(&envelope).unwrap(), text(&answers["envelope"], "sha256"));
}

#[test]
fn the_compressed_envelope_is_the_modelled_member_and_reads_back() {
    let answers = answers();
    let envelope = parse_breach_cache(&envelope_text()).unwrap();
    let member = encode_breach_cache(&envelope).unwrap();
    assert_eq!(hex_text(&member), text(&answers["envelope"], "memberHex"));
    assert_eq!(decode_breach_cache(&member).unwrap(), envelope);
    assert_eq!(u32::from_le_bytes(member[member.len() - 8..member.len() - 4].try_into().unwrap()) as u64, answers["envelope"]["crc32"].as_u64().unwrap());
}

#[test]
fn a_cache_that_is_not_an_envelope_is_refused() {
    assert!(parse_breach_cache("{}").is_err());
    assert!(parse_breach_cache("[]").is_err());
    assert!(decode_breach_cache(b"not gzip at all, not gzip at all").is_err());
    assert!(decode_breach_cache(&gzip_encode(b"{\"entityId\":1}")).is_err());
    assert!(breachs_from_cache(&["{}".to_string()]).is_err());
}

#[test]
fn breaches_of_many_documents_concatenate_in_document_order() {
    let first = BreachCacheEnvelope { entity_id: "a".into(), script: "s".into(), breachs: vec![create_breach("one", Statute::from("code/section/empty"), "a#S", 3, 0, "")] };
    let second = BreachCacheEnvelope { entity_id: "b".into(), script: "s".into(), breachs: vec![create_breach("two", Statute::from("code/file/wrong-license"), "b", 1, 2, "x"), create_breach("three", Statute::from("code/section/empty"), "b#T", 0, 0, "")] };
    let documents: Vec<String> = [&first, &second].iter().map(|envelope| encode_breach_cache_json(envelope).unwrap()).collect();
    let summaries: Vec<String> = breachs_from_cache(&documents).unwrap().into_iter().map(|breach| breach.summary).collect();
    assert_eq!(summaries, vec!["one", "two", "three"]);
    assert!(breachs_from_cache(&[]).unwrap().is_empty());
}
//#endregion 🧷️BreachCache

//#region 🔶️Breach
#[test]
fn a_breach_id_names_the_scope_and_the_most_precise_position() {
    assert_eq!(build_breach_id("a/b.ts", 0, 0), "repo/breach/a/b.ts");
    assert_eq!(build_breach_id("a/b.ts", 7, 0), "repo/breach/a/b.ts#7");
    assert_eq!(build_breach_id("a/b.ts", 7, 3), "repo/breach/a/b.ts#7:3");
    assert_eq!(build_breach_id("a/b.ts", 0, 3), "repo/breach/a/b.ts");
}

#[test]
fn a_created_breach_carries_its_inputs_and_no_derived_members() {
    let breach = create_breach("Empty section", Statute::from("code/section/empty"), "a.ts#Types", 4, 2, "excerpt");
    assert_eq!((breach.id.as_str(), breach.summary.as_str(), breach.kind.0.as_str(), breach.scope.as_str()), ("repo/breach/a.ts#Types#4:2", "Empty section", "code/section/empty", "a.ts#Types"));
    assert_eq!((breach.line, breach.column, breach.excerpt.as_str()), (4, 2, "excerpt"));
    assert!(breach.lint_priority.is_none() && breach.lint_autofixable.is_none() && breach.reason.is_empty() && breach.solution.is_empty());
    let encoded: Value = serde_json::to_value(&breach).unwrap();
    assert!(encoded.get("priority").is_none() && encoded.get("reason").is_none(), "omitempty members must vanish: {encoded}");
}
//#endregion 🔶️Breach

//#region 📚️Catalog
#[test]
fn the_catalog_is_the_schema_document_row_by_row() {
    let schema = fixture("../../🧬️schema/🔣️statutes.json");
    let expected = rows(&schema, "statutes");
    assert_eq!(statutes().len(), expected.len());
    for (meta, row) in statutes().iter().zip(expected) {
        assert_eq!(meta.kind.0, text(row, "kind"));
        assert_eq!(meta.policy_id, text(row, "policyId"));
        assert_eq!(meta.priority.as_str(), text(row, "priority"));
        assert_eq!((meta.reason.as_str(), meta.solution.as_str()), (text(row, "reason"), text(row, "solution")));
        assert_eq!(meta.autofixable, row["autofixable"].as_bool().unwrap());
        assert_eq!(statute_info(&meta.kind), *meta);
        assert_eq!(is_autofixable(&meta.kind), meta.autofixable);
    }
    let ids: Vec<&str> = policies().iter().map(|policy| policy.id.as_str()).collect();
    assert_eq!(ids, rows(&schema, "policies").iter().map(|row| text(row, "id")).collect::<Vec<_>>());
    assert_eq!(catalog().schema_version, 1);
}

#[test]
fn every_statute_is_unique_and_every_policy_kind_is_declared() {
    let mut kinds: Vec<&str> = statutes().iter().map(|meta| meta.kind.0.as_str()).collect();
    kinds.sort();
    let total = kinds.len();
    kinds.dedup();
    assert_eq!(kinds.len(), total, "duplicate statute kinds");
    for policy in policies() {
        assert!(statutes().iter().any(|meta| meta.policy_id == policy.id), "policy {} owns no statute", policy.id);
        for kind in policy_kinds(policy) {
            let meta = statutes().iter().find(|meta| meta.kind == kind).unwrap_or_else(|| panic!("policy {} names the undeclared statute {kind}", policy.id));
            assert_eq!(meta.policy_id, policy.id, "{kind} is declared by another policy");
        }
    }
}

#[test]
fn an_unknown_statute_falls_back_to_the_unknown_breach_row() {
    let unknown = Statute::from("no/such/statute");
    let meta = statute_info(&unknown);
    assert_eq!((meta.reason.as_str(), meta.solution.as_str(), meta.autofixable, meta.priority), ("Unknown breach", "Fix the breach", false, BreachPriority::Low));
    assert!(meta.policy_id.is_empty());
    assert!(!is_autofixable(&unknown));
    assert!(find_policy("no-such-policy").is_none());
    assert_eq!(find_policy("code").unwrap().id, "code");
}

#[test]
fn a_policy_lists_its_statutes_group_by_group_in_declaration_order() {
    let code = find_policy("code").unwrap();
    let listed: Vec<String> = policy_kinds(code).into_iter().map(|statute| statute.0).collect();
    let schema = fixture("../../🧬️schema/🔣️statutes.json");
    let policy = rows(&schema, "policies").iter().find(|row| text(row, "id") == "code").unwrap();
    let mut expected: Vec<String> = Vec::new();
    fn walk(territory: &Value, out: &mut Vec<String>) {
        out.extend(territory["kinds"].as_array().into_iter().flatten().map(|kind| kind.as_str().unwrap().to_string()));
        for child in territory["groups"].as_array().into_iter().flatten() {
            walk(child, out);
        }
    }
    for group in rows(policy, "groups") {
        walk(group, &mut expected);
    }
    assert_eq!(listed, expected);
    assert_eq!(territory_kinds(&code.groups.as_ref().unwrap()[0]), code.groups.as_ref().unwrap()[0].all_kinds());
}
//#endregion 📚️Catalog

//#region 🙈️IgnoreDirectives
#[test]
fn every_directive_vector_parses_and_suppresses_like_the_independent_model() {
    let answers = answers();
    for row in rows(&answers, "ignoreDirectives") {
        let source = fixture("🙈️ignore-directives/🔣️vectors.json");
        let document = rows(&source, "documents").iter().find(|document| document["name"] == row["name"]).expect("vector");
        let parsed = parse_ignore_directives(text(document, "content"));
        let expected: Vec<(i64, Vec<String>)> = row["directives"].as_object().unwrap().iter().map(|(line, patterns)| (line.parse().unwrap(), patterns.as_array().unwrap().iter().map(|p| p.as_str().unwrap().to_string()).collect())).collect();
        assert_eq!(parsed.into_iter().collect::<Vec<_>>(), { let mut sorted = expected.clone(); sorted.sort(); sorted }, "{}", text(row, "name"));
        let directives = parse_ignore_directives(text(document, "content"));
        for probe in rows(row, "probes") {
            let line = probe["line"].as_i64().unwrap();
            assert_eq!(is_ignored(&directives, line, &Statute::from(text(probe, "kind"))), probe["ignored"].as_bool().unwrap(), "{} line {line} {}", text(row, "name"), text(probe, "kind"));
        }
    }
}

#[test]
fn the_window_reaches_one_hundred_lines_forward_and_never_backward_or_onto_itself() {
    let directives = parse_ignore_directives("x\n// compose-ignore-code/a\ny\n");
    let kind = Statute::from("code/a/b");
    assert!(!is_ignored(&directives, 1, &kind) && !is_ignored(&directives, 2, &kind));
    assert!(is_ignored(&directives, 3, &kind) && is_ignored(&directives, 102, &kind));
    assert!(!is_ignored(&directives, 103, &kind));
    assert!(!is_ignored(&directives, 3, &Statute::from("code")), "a prefix is the directive's, not the statute's");
}

#[test]
fn a_scope_loses_its_section_and_definition_suffixes() {
    for (scope, file) in [("a/b.ts", "a/b.ts"), ("a/b.ts#Header#2", "a/b.ts"), ("a/b.ts::CONTENT", "a/b.ts"), ("a/b.ts::X#1", "a/b.ts"), ("a/b.ts#S::X", "a/b.ts"), ("", "")] {
        assert_eq!(extract_file_from_scope(scope), file, "{scope:?}");
    }
}

#[test]
fn filtering_drops_exactly_the_breaches_a_directive_of_their_own_file_covers() {
    let sources = SourceSet::new(vec![
        SourceFile { path: "a.ts".into(), content: "// compose-ignore-code/section\nx\ny\n".into() },
        SourceFile { path: "b.ts".into(), content: "x\ny\nz\n".into() },
    ]);
    let breach = |scope: &str, kind: &str, line: i64| create_breach("m", Statute::from(kind), scope, line, 0, "");
    let kept = filter_ignored(
        vec![breach("a.ts#S", "code/section/empty", 3), breach("a.ts#S", "code/file/missing-license", 3), breach("b.ts#S", "code/section/empty", 3), breach("c.ts", "code/section/empty", 3), breach("a.ts::D", "code/section/empty", 1)],
        &sources,
    );
    let ids: Vec<String> = kept.into_iter().map(|breach| breach.id).collect();
    assert_eq!(ids, vec!["repo/breach/a.ts#S#3", "repo/breach/b.ts#S#3", "repo/breach/c.ts#3", "repo/breach/a.ts::D#1"]);
    assert_eq!(sources.files().len(), 2);
    assert!(sources.directives("a.ts").is_some_and(|map| map.contains_key(&1)) && sources.directives("missing.ts").is_none());
}
//#endregion 🙈️IgnoreDirectives

//#region 📐️SpecificationText
#[test]
fn every_spec_text_vector_matches_the_independent_regular_expression() {
    let answers = answers();
    for row in rows(&answers, "specText") {
        assert_eq!(is_spec_text(text(row, "text")), row["spec"].as_bool().unwrap(), "{:?}", row["text"]);
    }
}

#[test]
fn every_implementation_syntax_vector_matches_the_independent_regular_expressions() {
    let answers = answers();
    for row in rows(&answers, "implementationSyntax") {
        assert_eq!(has_implementation_syntax(text(row, "text")).as_deref(), row["reason"].as_str(), "{:?}", row["text"]);
    }
}

#[test]
fn every_contributor_vector_matches_the_independent_regular_expression() {
    let answers = answers();
    for row in rows(&answers, "contributorLines") {
        assert_eq!(contributor_line(text(row, "text")), row["contributor"].as_bool().unwrap(), "{:?}", row["text"]);
    }
}
//#endregion 📐️SpecificationText

//#region 🗂️Sources
#[test]
fn test_and_benchmark_files_are_recognised_by_name_and_by_folder() {
    for path in ["a/b_test.go", "a/b.test.ts", "a/c.spec.tsx", "x/test_models.py", "x/conftest.py", "pkg/tests/x.rs", "pkg.tests/x.rs", "pkg/test/x.rs", "a/benchmark/x.ts", "a/perf_benchmark.rs", "A\\B\\widget.stories.tsx", "a/x.requirements.ts"] {
        assert!(is_test_or_benchmark_file(path), "{path}");
    }
    for path in ["a/b.go", "a/contest.ts", "a/latest.rs", "a/protests.ts", "src/main.rs"] {
        assert!(!is_test_or_benchmark_file(path), "{path}");
    }
}

#[test]
fn a_definition_head_needs_requirements_per_language() {
    for (line, language, needs) in [
        ("export async function run() {", "typescript", true),
        ("export default class Foo {", "typescript", true),
        ("export const x = 1;", "typescript", false),
        ("function f() {}", "typescript", true),
        ("func Name() {", "go", true),
        ("type Name struct {", "go", false),
        ("def run():", "python", true),
        ("async def run():", "python", true),
        ("x = 1", "python", false),
        ("public class A {", "csharp", true),
        ("public interface IA {", "csharp", false),
        ("pub fn run() {", "rust", true),
        ("pub (crate) struct A;", "rust", true),
        ("const X: u8 = 1;", "rust", false),
        ("anything", "unknown-language", true),
    ] {
        assert_eq!(requires_definition_requirements(line, language), needs, "{language}: {line}");
    }
}
//#endregion 🗂️Sources

//#region 🔍️Analysis
fn collect(root: &Path, relative: &str, out: &mut Vec<SourceFile>) {
    let directory = if relative.is_empty() { root.to_path_buf() } else { root.join(relative) };
    for entry in std::fs::read_dir(&directory).unwrap().flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        let child = if relative.is_empty() { name } else { format!("{relative}/{name}") };
        if entry.path().is_dir() {
            collect(root, &child, out);
        } else {
            out.push(SourceFile { path: child, content: std::fs::read_to_string(entry.path()).unwrap() });
        }
    }
}

fn tree() -> Vec<SourceFile> {
    let root = fixtures_dir();
    let mut files = Vec::new();
    for name in std::fs::read_dir(&root).unwrap().flatten().map(|entry| entry.file_name().to_string_lossy().to_string()).filter(|name| name.starts_with('📁')) {
        collect(&root, &name, &mut files);
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    files
}

fn render(breach: &Breach) -> String {
    format!("{}|{}|{}|{}|{}|{}|{}", breach.id, breach.kind.0, breach.scope, breach.line, breach.column, is_autofixable(&breach.kind), breach.summary)
}

#[test]
fn the_committed_tree_breaches_exactly_the_reviewed_golden_list() {
    let files = tree();
    assert_eq!(files.len(), 14, "the committed tree changed: {:?}", files.iter().map(|file| &file.path).collect::<Vec<_>>());
    let rendered: Vec<String> = analyze(&SourceSet::new(files)).iter().map(render).collect();
    let golden = fixture("🔍️analyze-breaches/🔣️breaches.json");
    let expected: Vec<String> = rows(&golden, "breachs").iter().map(|row| row.as_str().unwrap().to_string()).collect();
    assert_eq!(rendered, expected);
}

#[test]
fn the_repaired_sources_raise_nothing() {
    let clean: Vec<SourceFile> = tree().into_iter().filter(|file| file.path.contains("file-fixed") || file.path.contains("file-fixable-expected")).collect();
    assert_eq!(clean.len(), 5);
    assert!(analyze(&SourceSet::new(clean)).is_empty());
}

#[test]
fn analysis_is_the_header_section_and_requirements_policies_in_that_order() {
    let sources = SourceSet::new(tree());
    let mut expected: Vec<String> = Vec::new();
    for policy in [header_policy, section_policy, requirements_policy] {
        expected.extend(policy(&sources).iter().map(render));
    }
    assert_eq!(analyze(&sources).iter().map(render).collect::<Vec<_>>(), expected);
}

#[test]
fn an_empty_source_set_and_an_empty_file_are_judged_without_a_panic() {
    assert!(analyze(&SourceSet::new(Vec::new())).is_empty());
    let breaches = analyze(&SourceSet::new(vec![SourceFile { path: "a.ts".into(), content: String::new() }]));
    assert!(breaches.iter().all(|breach| breach.scope.starts_with("a.ts")));
}
//#endregion 🔍️Analysis

//#region 🩹️Autofix
fn file<'a>(files: &'a [SourceFile], infix: &str) -> &'a SourceFile {
    files.iter().find(|file| file.path.contains(infix)).unwrap_or_else(|| panic!("no source {infix}"))
}

#[test]
fn the_fixable_source_becomes_the_expected_source_and_stays_fixed() {
    let files = tree();
    let fixable = file(&files, "file-fixable/");
    let expected = file(&files, "file-fixable-expected/");
    let fixed = autofix(fixable);
    assert_eq!(fixed.path, fixable.path);
    assert_eq!(fixed.content, expected.content);
    assert_eq!(fixed.fixed, vec![Statute::from("code/section/wrong-format/newline-after-region")]);
    let again = autofix(&SourceFile { path: fixed.path.clone(), content: fixed.content.clone() });
    assert!(again.fixed.is_empty() && again.content == fixed.content, "autofix is not idempotent");
}

#[test]
fn a_clean_source_and_an_unsupported_path_are_left_untouched() {
    let files = tree();
    let clean = file(&files, "file-fixed/🟦️.tsx");
    let result = autofix(clean);
    assert!(result.fixed.is_empty());
    assert_eq!(result.content, clean.content);
    let unsupported = autofix(&SourceFile { path: "notes.unknown-extension".into(), content: "// #region X\n\n\nbody\n".into() });
    assert!(unsupported.fixed.is_empty());
    assert_eq!(unsupported.content, "// #region X\n\n\nbody\n");
}
//#endregion 🩹️Autofix
