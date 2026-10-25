//! 🔬️ Cargo tests of the metrics crate. The recorded outputs of `🧫️fixtures/📤️*.json` (UTC buckets
//! produced by JavaScript's `Date`, numstat rows, the LOC table, the benchmark table) are replayed
//! through the pipeline; the UTC arithmetic, git path quoting, rename resolution and the extension
//! tables are judged against `🧫️fixtures/🔬️unit-vectors.json` (Python's datetime, `re`); the git source
//! and the recorded transcript against a REAL git repository rebuilt from the committed recipe.

use super::*;
use semio_framework_test_fixtures::fixture_path;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

//#region 📥️Fixtures
fn fixture(name: &str) -> Value {
    let path = fixture_path!("..").join("..").join("🧪️tests").join("🧫️fixtures").join(name);
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

fn transcript() -> GitTranscript {
    GitTranscript::from_json(&serde_json::to_string(&fixture("🎞️git-transcript.json")).unwrap()).unwrap()
}

fn weights() -> BTreeSet<String> {
    make_numstat_lang_set(&DEFAULT_CODE_LANGUAGES.iter().map(|value| value.to_string()).collect::<Vec<_>>())
}

fn commits(log_ref: &str) -> Vec<CommitDelta> {
    parse_numstat_log(&transcript().logs[log_ref], &weights(), None)
}

fn as_value<T: serde::Serialize>(value: &T) -> Value {
    serde_json::from_str(&to_json_string(value).unwrap()).unwrap()
}

const BUCKETS: [(&str, TimeBucket); 6] = [("commit", TimeBucket::Commit), ("hour", TimeBucket::Hour), ("day", TimeBucket::Day), ("week", TimeBucket::Week), ("month", TimeBucket::Month), ("year", TimeBucket::Year)];
//#endregion 📥️Fixtures

//#region 🕰️Time
#[test]
fn every_instant_renders_and_buckets_like_pythons_datetime() {
    let answers = answers();
    assert!(rows(&answers, "instants").len() > 250);
    for row in rows(&answers, "instants") {
        let unix = row["unix"].as_i64().unwrap();
        for (name, bucket) in BUCKETS {
            assert_eq!(bucket_key(unix, bucket), text(row, name), "{name} key of {unix}");
            assert_eq!(bucket_start(unix, bucket), row["starts"][name].as_i64().unwrap(), "{name} start of {unix}");
        }
        assert_eq!(format_rfc3339_utc(unix), text(row, "commit"));
        assert_eq!(iso_weekday(unix), row["weekday"].as_i64().unwrap(), "weekday of {unix}");
        let civil = row["civil"].as_array().unwrap();
        assert_eq!(civil_from_unix(unix), (civil[0].as_i64().unwrap(), civil[1].as_u64().unwrap() as u32, civil[2].as_u64().unwrap() as u32), "civil date of {unix}");
        assert_eq!(unix_days_from_civil(civil[0].as_i64().unwrap(), civil[1].as_u64().unwrap() as u32, civil[2].as_u64().unwrap() as u32), row["days"].as_i64().unwrap(), "days of {unix}");
    }
}

#[test]
fn converting_a_day_to_a_civil_date_and_back_is_the_identity_over_a_wide_range() {
    let mut day = -800_000_i64;
    while day <= 800_000 {
        let (year, month, of_month) = civil_from_unix(day * 86_400);
        assert_eq!(unix_days_from_civil(year, month, of_month), day, "day {day}");
        day += 61;
    }
}

#[test]
fn the_recorded_commit_stream_groups_into_the_javascript_date_buckets() {
    let recorded = fixture("📤️time-buckets.json");
    let stream = commits("");
    for (name, bucket) in BUCKETS {
        assert_eq!(as_value(&bucket_commits(&stream, bucket)), recorded[name], "granularity {name}");
    }
}

#[test]
fn a_granularity_serialises_lowercase_and_an_empty_stream_has_no_buckets() {
    for (name, bucket) in BUCKETS {
        assert_eq!(serde_json::to_value(bucket).unwrap(), Value::String(name.to_string()));
        assert!(bucket_commits(&[], bucket).is_empty());
    }
}

#[test]
fn commits_of_one_bucket_sum_their_deltas_and_keep_the_stream_order() {
    let commit = |sha: &str, when: i64, added: i64| CommitDelta { sha: sha.into(), when_unix: when, delta: BTreeMap::from([("Go".to_string(), LineMetrics { added, removed: 1 })]), ..CommitDelta::default() };
    let groups = bucket_commits(&[commit("a", 100, 2), commit("b", 200, 3), commit("c", 90_000, 4)], TimeBucket::Day);
    assert_eq!(groups.len(), 2);
    assert_eq!((groups[0].key.as_str(), groups[0].commits.clone(), groups[0].delta["Go"].clone()), ("1970-01-01", vec!["a".to_string(), "b".to_string()], LineMetrics { added: 5, removed: 2 }));
    assert_eq!((groups[1].key.as_str(), groups[1].start_unix), ("1970-01-02", 86_400));
}
//#endregion 🕰️Time

//#region 🧩️Numstat
#[test]
fn the_recorded_log_parses_into_the_recorded_numstat_rows() {
    let recorded = fixture("📤️numstat-deltas.json");
    assert_eq!(as_value(&commits("")), recorded["noMerges"]);
    let merged: Vec<Value> = commits("🔀️with-merges")
        .iter()
        .map(|commit| json!({"sha": commit.sha, "when_unix": commit.when_unix, "fileCount": commit.files.len(), "delta": as_value(&commit.delta)}))
        .collect();
    assert_eq!(Value::Array(merged), recorded["withMerges"]);
}

#[test]
fn every_quoted_path_decodes_like_the_independent_model() {
    let answers = answers();
    for row in rows(&answers, "unquote") {
        assert_eq!(unquote_git_path(text(row, "raw")), text(row, "path"), "{:?}", row["raw"]);
    }
}

#[test]
fn every_rename_field_resolves_like_the_independent_model() {
    let answers = answers();
    for row in rows(&answers, "renames") {
        let (path, from) = resolve_numstat_path(text(row, "raw"));
        assert_eq!((path.as_str(), from.as_deref()), (text(row, "path"), row["from"].as_str()), "{:?}", row["raw"]);
    }
}

#[test]
fn a_malformed_stream_is_skipped_never_fatal() {
    let stream = "orphan\t1\t2\tfile.ts\nCOMMIT\tonly\tthree\tfields\n\nCOMMIT\tsha1\tAda\tada@x\t100\n1\t2\ta.ts\nnot a numstat line\nx\ty\tb.ts\n-\t-\timg.png\n5\t0\tc.go\r\n\nCOMMIT\tsha2\tBob\tbob@x\tnotanumber\n";
    let parsed = parse_numstat_log(stream, &weights(), None);
    assert_eq!(parsed.iter().map(|commit| commit.sha.as_str()).collect::<Vec<_>>(), vec!["sha1", "sha2"]);
    assert_eq!(parsed[0].files.iter().map(|file| file.path.as_str()).collect::<Vec<_>>(), vec!["a.ts", "img.png", "c.go"]);
    assert_eq!(parsed[0].files[1].binary, true);
    assert_eq!((parsed[0].files[1].added, parsed[0].files[1].bucket.clone()), (0, None));
    assert_eq!(parsed[0].delta["TypeScript"], LineMetrics { added: 1, removed: 2 });
    assert_eq!(parsed[0].delta["Go"], LineMetrics { added: 5, removed: 0 });
    assert_eq!((parsed[1].when_unix, parsed[1].files.len()), (0, 0));
    assert!(parse_numstat_log("", &weights(), None).is_empty());
}

#[test]
fn hidden_meta_ignored_and_unweighted_paths_are_listed_but_not_counted() {
    let stream = "COMMIT\ts\tA\ta@x\t1\n4\t0\t.hidden/x.ts\n4\t0\t.🧬semio/note.md\n4\t0\tignored/y.ts\n4\t0\tkept.ts\n4\t0\tkept.txt\n4\t0\tkept.md\n";
    let ignore = GitIgnore::compile_lines(["ignored/"]);
    let only_code = make_numstat_lang_set(&["TypeScript".to_string()]);
    let parsed = parse_numstat_log(stream, &only_code, Some(&ignore));
    assert_eq!(parsed[0].files.len(), 6);
    assert_eq!(parsed[0].files.iter().filter(|file| file.bucket.is_some()).map(|file| file.path.as_str()).collect::<Vec<_>>(), vec!["kept.ts", "kept.md"]);
    assert_eq!(parsed[0].delta.keys().cloned().collect::<Vec<_>>(), vec!["Markup", "TypeScript"]);
    let narrowed = parse_numstat_log(stream, &make_numstat_lang_set(&["Go".to_string()]), Some(&ignore));
    assert_eq!(narrowed[0].delta.keys().cloned().collect::<Vec<_>>(), vec!["Markup"]);
}
//#endregion 🧩️Numstat

//#region 🏷️Vocabulary
#[test]
fn every_path_classifies_like_the_independent_extension_tables() {
    let answers = answers();
    for row in rows(&answers, "classify") {
        let path = text(row, "path");
        assert_eq!(extension_of(path), text(row, "extension"), "extension of {path:?}");
        assert_eq!(classify_loc_bucket(path), row["bucket"].as_str(), "bucket of {path:?}");
        assert_eq!(classify_language_fine(path), row["language"].as_str(), "language of {path:?}");
    }
}

#[test]
fn the_weight_set_decides_which_bucket_a_path_counts_for() {
    let only_go = make_numstat_lang_set(&[" Go ".to_string()]);
    assert_eq!(only_go.iter().cloned().collect::<Vec<_>>(), vec!["Data", "Go", "Markup"]);
    assert_eq!(classify_for_numstat("a.go", &only_go).as_deref(), Some("Go"));
    assert_eq!(classify_for_numstat("a.ts", &only_go), None);
    assert_eq!(classify_for_numstat("a.md", &only_go).as_deref(), Some("Markup"));
    assert_eq!(classify_for_numstat("a.unknown", &only_go), None);
    assert_eq!(make_lang_set(&["  A".to_string(), "B  ".to_string()]).into_iter().collect::<Vec<_>>(), vec!["A", "B"]);
}

#[test]
fn hidden_and_meta_paths_are_skipped_from_the_count() {
    for (path, hidden) in [("a/b.ts", false), (".x/b.ts", true), ("a/.b/c.ts", true), ("./a/b.ts", false), ("a/./b.ts", false), ("a/../b.ts", false), (".hidden", true), ("a/.", false), ("", false)] {
        assert_eq!(path_has_hidden_segment(path), hidden, "{path:?}");
    }
    assert!(path_is_repo_meta(".🧬semio") && path_is_repo_meta(".🧬semio/🦑️repo/x.md") && !path_is_repo_meta(".🧬semio-other/x") && !path_is_repo_meta("a/.🧬semio/x"));
    let ignore = GitIgnore::compile_lines(["*.log"]);
    assert!(path_skipped_for_loc("", None) && path_skipped_for_loc(".🧬semio/a.md", None) && path_skipped_for_loc(".dot/a.ts", None));
    assert!(path_skipped_for_loc("a/b.log", Some(&ignore)) && !path_skipped_for_loc("a/b.ts", Some(&ignore)));
    assert_eq!(normalize_repo_path(".\\a\\b.ts"), "a/b.ts");
    assert_eq!(normalize_repo_path("./a/b.ts"), "a/b.ts");
}

#[test]
fn the_numstat_log_arguments_ask_git_for_a_first_parent_reverse_walk() {
    let args = numstat_log_args("");
    assert_eq!(args, vec!["log", "--no-merges", "--numstat", "--first-parent", "--reverse", "--pretty=format:COMMIT%x09%H%x09%aN%x09%aE%x09%at"]);
    assert_eq!(numstat_log_args("  main  ").last().unwrap(), "main");
    assert_eq!(numstat_log_range_args("a..b").last().unwrap(), "a..b");
    assert_eq!(&numstat_log_range_args("a..b")[..6], &args[..]);
}

#[test]
fn a_chain_splits_into_ranges_that_cover_it_once_oldest_first() {
    let chain: Vec<String> = (0..10).rev().map(|index| format!("c{index}")).collect();
    for chunks in [1, 2, 3, 4, 10, 25] {
        let ranges = numstat_log_ranges(&chain, chunks);
        assert!(ranges.len() <= chunks.min(chain.len()) && !ranges.is_empty());
        assert_eq!(ranges[0].contains(".."), false, "the oldest range reaches the root as one commit: {ranges:?}");
        let mut covered: Vec<String> = Vec::new();
        for range in &ranges {
            let (base, tip) = match range.split_once("..") {
                Some((base, tip)) => (Some(base), tip),
                None => (None, range.as_str()),
            };
            let tip_index = chain.iter().position(|commit| commit == tip).unwrap();
            let base_index = base.map_or(chain.len(), |base| chain.iter().position(|commit| commit == base).unwrap());
            covered.extend(chain[tip_index..base_index].iter().cloned());
        }
        covered.sort();
        let mut expected = chain.clone();
        expected.sort();
        assert_eq!(covered, expected, "{chunks} chunks: {ranges:?}");
    }
    assert!(numstat_log_ranges(&[], 4).is_empty());
}

#[test]
fn chunk_outputs_join_with_exactly_one_newline_at_every_seam() {
    assert_eq!(join_numstat_chunks(&["a\n1\t0\tx".to_string(), String::new(), "b\n2\t0\ty".to_string()]), "a\n1\t0\tx\nb\n2\t0\ty");
    assert_eq!(join_numstat_chunks(&[]), "");
    assert_eq!(join_numstat_chunks(&[String::new()]), "");
}
//#endregion 🏷️Vocabulary

//#region 📏️Counting
#[test]
fn lines_are_counted_wc_style_and_json_by_its_object_keys() {
    for (data, lines) in [("", 0), ("a", 1), ("a\n", 2), ("a\nb", 2), ("\n\n", 3)] {
        assert_eq!(physical_line_count(data.as_bytes()), lines, "{data:?}");
    }
    assert_eq!(json_key_count(b"{\"a\":1,\"b\":{\"c\":2,\"d\":[{\"e\":3}]}}"), 5);
    assert_eq!(json_key_count(b"[1,2,{\"a\":1}]"), 1);
    assert_eq!(json_key_count(b"  "), 0);
    assert_eq!(json_key_count(b"not json"), 0);
    assert_eq!(json_key_count(&[0xff, 0xfe]), 0);
    assert_eq!(count_json_keys(&json!({"a": [], "b": {}})), 2);
}

#[test]
fn unified_loc_is_object_keys_for_json_and_physical_lines_for_everything_else() {
    assert_eq!(count_unified_loc_for_file("a/config.json", b"{\n  \"a\": 1,\n  \"b\": { \"c\": 2 }\n}\n"), 3);
    assert_eq!(count_unified_loc_for_file("tsconfig.JSONC", b"{\"a\":1}"), 1);
    assert_eq!(count_unified_loc_for_file("empty.json", b"  \n"), 0);
    assert_eq!(count_unified_loc_for_file("keyless.json", b"[1,\n2]"), 2);
    assert_eq!(count_unified_loc_for_file("broken.json", b"{ broken\n"), 2);
    assert_eq!(count_unified_loc_for_file("a.ts", b"one\ntwo\nthree\n"), 4);
    assert_eq!(count_unified_loc_for_file("noext", b""), 0);
}
//#endregion 📏️Counting

//#region 📈️Report
fn stat(loc: i64, edited: i64) -> LocLangStats {
    LocLangStats { loc, edited, added: edited, ..LocLangStats::default() }
}

#[test]
fn the_recorded_loc_table_is_composed_from_the_recorded_scan_and_the_recorded_cumulative() {
    let recorded = fixture("📤️loc-report.json");
    let languages: Vec<String> = DEFAULT_CODE_LANGUAGES.iter().map(|value| value.to_string()).collect();
    let scan = snapshot_loc_counts(&transcript(), "", &languages, None).unwrap();
    assert_eq!(as_value(&scan), recorded["scan"]);
    let (cumulative, by_contributor) = cumulative_from_raw(&commits(""), &languages, false, "", &default_contributor_alias);
    assert!(by_contributor.is_none());
    assert_eq!(as_value(&cumulative), recorded["cumulative"]);
    let denominator = sum_edited_pairs(&cumulative, &make_numstat_lang_set(&languages));
    assert_eq!(denominator, recorded["wipDenominator"].as_i64().unwrap());
    assert_eq!(as_value(&compose_loc_report_snapshot(&cumulative, Some(&scan), &languages, denominator)), recorded["snapshot"]);
    let report = build_loc_report(&transcript(), &LocOptions::default(), None, &default_contributor_alias).unwrap();
    assert_eq!(as_value(&report.snapshot), recorded["snapshot"]);
    assert_eq!(report.branch, recorded["branch"].as_str().unwrap());
    assert!(report.history.is_none() && report.by_contributors.is_none());
}

#[test]
fn the_recorded_benchmark_stdout_summarises_into_the_recorded_table() {
    let outputs = fixture("⏱️benchmark-output.json");
    let recorded = fixture("📤️benchmark-summary.json");
    let results: Vec<BenchmarkResult> = BENCHMARK_LANGUAGES.iter().flat_map(|language| parse_benchmark_output(language, text(&outputs, language))).collect();
    assert_eq!(as_value(&results), recorded["results"]);
    assert_eq!(as_value(&summarize_benchmarks(&results)), recorded["summary"]);
}

#[test]
fn benchmark_noise_is_dropped_and_the_first_timing_of_a_pair_wins() {
    let parsed = parse_benchmark_output("Go", "warning: x,1s\nC:/path,2s\nplain,0.5s\n  spaced , 1s \nthree,1,2\nlone\n\nrel\\path,3s\n");
    assert_eq!(parsed.iter().map(|result| (result.test.as_str(), result.time.as_str())).collect::<Vec<_>>(), vec![("plain", "0.5s"), ("spaced ", " 1s")]);
    let summary = summarize_benchmarks(&[
        BenchmarkResult { test: "t".into(), lang: "Go".into(), time: "5ms".into() },
        BenchmarkResult { test: "t".into(), lang: "Go".into(), time: "1ms".into() },
        BenchmarkResult { test: "t".into(), lang: "Rust".into(), time: "4000us".into() },
        BenchmarkResult { test: "t".into(), lang: "Python".into(), time: "oops".into() },
    ]);
    assert_eq!(summary.rows[0].timings["Go"], "5ms");
    assert_eq!(summary.rows[0].fastest.as_deref(), Some("Rust"));
    let tie = summarize_benchmarks(&[BenchmarkResult { test: "t".into(), lang: "Rust".into(), time: "1s".into() }, BenchmarkResult { test: "t".into(), lang: "Go".into(), time: "1000ms".into() }]);
    assert_eq!(tie.rows[0].fastest.as_deref(), Some("Go"), "a tie goes to the name that sorts first");
    assert_eq!(benchmark_csv(&[BenchmarkResult { test: "t".into(), lang: "Go".into(), time: "5ms".into() }]), "Test,Typescript,Python,Go,C#,Rust\nt,,,5ms,,\n");
    assert!(summarize_benchmarks(&[]).rows.is_empty());
}

#[test]
fn every_duration_parses_like_pythons_float() {
    let answers = answers();
    for row in rows(&answers, "durations") {
        let parsed = parse_duration_seconds(text(row, "raw"));
        match row["seconds"].as_f64() {
            Some(expected) => assert_eq!(parsed, Some(expected), "{:?}", row["raw"]),
            None => assert_eq!(parsed, None, "{:?}", row["raw"]),
        }
    }
}

#[test]
fn percentages_round_to_two_places_and_a_zero_denominator_gives_zero() {
    let mut table = BTreeMap::from([("A".to_string(), stat(1, 1)), ("B".to_string(), stat(2, 0)), (AGG_TOTAL.to_string(), stat(3, 1))]);
    apply_percents(&mut table);
    assert_eq!((table["A"].percent, table["B"].percent, table[AGG_TOTAL].percent), (33.33, 66.67, 100.0));
    apply_wip_percents(&mut table, 3);
    assert_eq!((table["A"].wip_percent, table["B"].wip_percent), (33.33, 0.0));
    apply_wip_percents(&mut table, 0);
    assert_eq!(table["A"].wip_percent, 100.0, "a zero global denominator falls back to the Total row");
    let mut empty = BTreeMap::from([("A".to_string(), stat(0, 0)), (AGG_TOTAL.to_string(), stat(0, 0))]);
    apply_percents(&mut empty);
    apply_wip_percents(&mut empty, 0);
    assert!(empty.values().all(|row| row.percent == 0.0 && row.wip_percent == 0.0));
    assert_eq!((pct_loc_since_prev(0, 0), pct_loc_since_prev(0, 5), pct_loc_since_prev(200, 250), pct_loc_since_prev(3, 2)), (0.0, 100.0, 25.0, -33.33));
}

#[test]
fn rows_sort_by_size_then_name_with_the_total_pinned_last() {
    let table = BTreeMap::from([("B".to_string(), stat(5, 1)), ("A".to_string(), stat(5, 9)), ("C".to_string(), stat(7, 0)), (AGG_TOTAL.to_string(), stat(17, 10))]);
    assert_eq!(sorted_row_keys(&table), vec!["C", "A", "B", "Total"]);
    assert_eq!(sorted_row_keys_churn(&table), vec!["A", "B", "C", "Total"]);
    assert!(use_full_tree_table(&table) && !use_full_tree_table(&BTreeMap::from([(AGG_TOTAL.to_string(), stat(0, 4))])) && !use_full_tree_table(&BTreeMap::new()));
}

#[test]
fn the_default_alias_is_the_name_and_mail_or_unknown() {
    assert_eq!(default_contributor_alias(" Ada ", " a@x "), "Ada <a@x>");
    assert_eq!(default_contributor_alias("Ada", ""), "Ada");
    assert_eq!(default_contributor_alias("", "a@x"), " <a@x>");
    assert_eq!(default_contributor_alias("  ", "  "), "unknown");
}

#[test]
fn history_labels_map_onto_the_repository_entity_ids() {
    assert_eq!(display_history_branch("refs/heads/wip"), DEFAULT_BRANCH);
    assert_eq!(display_history_branch("WIP"), DEFAULT_BRANCH);
    assert_eq!(display_history_branch(DEFAULT_BRANCH), DEFAULT_BRANCH);
    assert_eq!(display_history_branch(" main "), "main");
    assert_eq!(display_history_branch(""), "");
    assert_eq!(history_checkpoint_label(" 0123456789abcdef "), "\u{1F500}\u{FE0F}0123456");
    assert_eq!(contributor_emoji_id(" Ada Lovelace "), "\u{1F9D1}\u{FE0F}\u{200D}\u{1F4BB}\u{FE0F}adalovelace");
}

#[test]
fn history_stamps_the_change_against_the_previous_row_and_leaves_the_first_empty() {
    let entry = |loc: i64| LocHistoryEntry { sha: loc.to_string(), languages: Some(BTreeMap::from([("Go".to_string(), stat(loc, 0))])), ..LocHistoryEntry::default() };
    let mut history = vec![entry(0), entry(4), entry(2), LocHistoryEntry::default(), entry(2)];
    apply_history_loc_since_prev(&mut history);
    let steps: Vec<Option<f64>> = history.iter().map(|row| row.languages.as_ref().and_then(|rows| rows["Go"].since_prev_loc_percent)).collect();
    assert_eq!(steps, vec![None, Some(100.0), Some(-50.0), None, Some(0.0)]);
    assert!(history_entry_stats(&history[3]).is_none() && history_entry_stats(&history[1]).is_some());
}
//#endregion 📈️Report

//#region 📤️Render
fn small_report() -> LocReport {
    let languages = vec!["Go".to_string(), "Rust".to_string()];
    let cumulative: Cumulative = BTreeMap::from([("Go".to_string(), LineMetrics { added: 10, removed: 2 }), ("Rust".to_string(), LineMetrics { added: 6, removed: 0 })]);
    let scan = BTreeMap::from([("Go".to_string(), 30), ("Rust".to_string(), 10)]);
    LocReport { snapshot: compose_loc_report_snapshot(&cumulative, Some(&scan), &languages, 18), branch: "wip".to_string(), ..LocReport::default() }
}

#[test]
fn the_markdown_snapshot_table_has_the_documented_columns_and_row_order() {
    let report = small_report();
    let markdown = render_markdown(&report, false, false);
    assert_eq!(
        markdown,
        "## LOC\n### Snapshot\n\n| Category | loc | % | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n| Code | 40 | 100.00% | 100.00% | 18 | 16 | 2 |\n| Go | 30 | 75.00% | 66.67% | 12 | 10 | 2 |\n| Rust | 10 | 25.00% | 33.33% | 6 | 6 | 0 |\n| Data | 0 | 0.00% | 0.00% | 0 | 0 | 0 |\n| Markup | 0 | 0.00% | 0.00% | 0 | 0 | 0 |\n| Total | 40 | 100.00% | 100.00% | 18 | 16 | 2 |\n\n"
    );
}

#[test]
fn the_text_snapshot_table_is_fixed_width_and_the_churn_layout_drops_the_tree_columns() {
    let report = small_report();
    let text = render_text(&report, false, false);
    assert!(text.starts_with(&format!("Snapshot\n{}\n", "Category           loc      %    wip  edited   added removed")), "{text}");
    assert!(text.contains(&format!("{}\n", "Go                  30  75.0%  66.7%      12      10       2")), "{text}");
    let churn = BTreeMap::from([("Go".to_string(), LocLangStats { edited: 4, added: 3, removed: 1, wip_percent: 50.0, ..LocLangStats::default() }), (AGG_TOTAL.to_string(), LocLangStats { edited: 8, added: 6, removed: 2, wip_percent: 100.0, ..LocLangStats::default() })]);
    assert_eq!(markdown_table("Churn", &churn, false, false), "### Churn\n\n| Category | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: |\n| Go | 50.00% | 4 | 3 | 1 |\n| Total | 100.00% | 8 | 6 | 2 |\n");
    assert_eq!(text_table("", &BTreeMap::new(), true, false), "");
}

#[test]
fn history_and_contributor_sections_render_when_asked_for() {
    let options = LocOptions { history: true, by_contributors: true, branch: DEFAULT_BRANCH.to_string(), ..LocOptions::default() };
    let report = build_loc_report(&transcript(), &options, None, &default_contributor_alias).unwrap();
    let markdown = render_markdown(&report, true, true);
    assert!(markdown.contains("\n## By contributor\n") && markdown.contains(&format!("\n## History ( {DEFAULT_BRANCH} )\n")), "{markdown}");
    assert!(markdown.contains(&format!("### {}", contributor_emoji_id("Ada Lovelace <ada@semio-tech.com>"))), "{markdown}");
    assert!(markdown.contains("| Category | loc | Δ% | % | wip% | edited | added | removed |"));
    let plain = render_markdown(&report, false, false);
    assert!(!plain.contains("History") && !plain.contains("By contributor"));
    let text = render_text(&report, true, false);
    assert!(text.contains(&format!("\nHistory: {DEFAULT_BRANCH}\n")) && !text.contains("Contributor:"), "{text}");
}
//#endregion 📤️Render

//#region 🧮️Pipeline
#[test]
fn the_history_walk_scans_the_tree_at_every_first_parent_commit_and_ends_where_the_snapshot_does() {
    let recorded = transcript();
    let options = LocOptions { history: true, branch: DEFAULT_BRANCH.to_string(), ..LocOptions::default() };
    let report = build_loc_report(&recorded, &options, None, &default_contributor_alias).unwrap();
    let history = report.history.as_ref().unwrap();
    assert_eq!(history.len(), commits(DEFAULT_BRANCH).len());
    assert_eq!(history.iter().map(|row| row.sha.clone()).collect::<Vec<_>>(), commits(DEFAULT_BRANCH).iter().map(|commit| commit.sha.clone()).collect::<Vec<_>>());
    let first = history[0].languages.as_ref().unwrap();
    assert!(first.values().all(|row| row.since_prev_loc_percent.is_none()));
    let last = history.last().unwrap().languages.as_ref().unwrap();
    assert_eq!(report.snapshot["Total"].loc - last["Total"].loc, 2, "the snapshot also holds side.rs, which only the merge commit adds");
    assert_eq!(report.branch, DEFAULT_BRANCH);
    assert_eq!(history[0].date, "2026-01-01T00:00:00Z");
}

#[test]
fn restricting_to_one_contributor_keeps_only_their_deltas_case_insensitively() {
    let options = LocOptions { contributor: "GRACE HOPPER <GRACE@SEMIO-TECH.COM>".to_string(), by_contributors: true, ..LocOptions::default() };
    let report = build_loc_report(&transcript(), &options, None, &default_contributor_alias).unwrap();
    let rows = report.by_contributors.as_ref().unwrap();
    assert_eq!(rows.keys().cloned().collect::<Vec<_>>(), vec!["Grace Hopper <grace@semio-tech.com>"]);
    let everyone = build_loc_report(&transcript(), &LocOptions::default(), None, &default_contributor_alias).unwrap();
    assert!(report.snapshot["Total"].edited < everyone.snapshot["Total"].edited);
    assert_eq!(report.snapshot["Total"].loc, everyone.snapshot["Total"].loc, "the tree scan does not depend on the contributor");
}

#[test]
fn a_transcript_that_lacks_a_ref_a_path_or_a_blob_refuses_instead_of_guessing() {
    let recorded = transcript();
    assert!(recorded.numstat_log("no-such-ref").unwrap_err().contains("no log for ref"));
    assert!(recorded.tracked_paths("no-such-ref").unwrap_err().contains("no tracked path list"));
    assert!(recorded.tracked_bytes("", "no/such/path").unwrap_err().contains("no blob"));
    assert_eq!(GitTranscript::blob_key("r", "p"), "r\u{1}p");
    assert!(GitTranscript::from_json("{").is_err());
    assert!(build_loc_report(&GitTranscript::default(), &LocOptions::default(), None, &default_contributor_alias).is_err());
}
//#endregion 🧮️Pipeline

//#region 🔌️RealGit
struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Scratch {
        let path = std::env::temp_dir().join(format!("semio-repo-metrics-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        let _ = std::fs::remove_dir_all(home_of(&path));
        std::fs::create_dir_all(&path).unwrap();
        Scratch(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
        let _ = std::fs::remove_dir_all(home_of(&self.0));
    }
}

fn home_of(directory: &Path) -> PathBuf {
    let mut name = directory.file_name().unwrap().to_os_string();
    name.push(".home");
    directory.with_file_name(name)
}

fn pairs(object: &Value) -> Vec<(String, String)> {
    object.as_object().unwrap().iter().map(|(key, value)| (key.clone(), value.as_str().unwrap().to_string())).collect()
}

/// 🔒️ The environment a git process of the recipe runs in: every inherited `GIT_*` variable dropped (but the exec path), a private empty home and global config, the recipe's pinned environment; the `ambient` pairs play the part of whatever the machine layers on top of the process environment.
fn hermetic_environment(directory: &Path, ambient: &[(String, String)]) -> Vec<(String, String)> {
    let home = home_of(directory);
    std::fs::create_dir_all(&home).unwrap();
    std::fs::write(home.join("gitconfig"), "").unwrap();
    let mut environment: BTreeMap<String, String> = std::env::vars().chain(ambient.iter().cloned()).filter(|(key, _)| !key.starts_with("GIT_") || key == "GIT_EXEC_PATH").collect();
    for key in ["HOME", "USERPROFILE", "XDG_CONFIG_HOME"] {
        environment.insert(key.to_string(), home.display().to_string());
    }
    environment.insert("GIT_CONFIG_GLOBAL".to_string(), home.join("gitconfig").display().to_string());
    environment.extend(pairs(&fixture("🌱️repository-recipe.json")["git"]["environment"]));
    environment.into_iter().collect()
}

fn hermetic_command(directory: &Path, ambient: &[(String, String)], arguments: &[&str]) -> std::process::Command {
    let mut command = std::process::Command::new("git");
    for (key, value) in pairs(&fixture("🌱️repository-recipe.json")["git"]["config"]) {
        command.args(["-c", &format!("{key}={value}")]);
    }
    command.args(arguments).current_dir(directory).env_clear().envs(hermetic_environment(directory, ambient));
    command
}

fn git(directory: &Path, arguments: &[&str], identity: Option<(&str, &str, i64)>) -> String {
    git_under(directory, &[], arguments, identity)
}

fn git_under(directory: &Path, ambient: &[(String, String)], arguments: &[&str], identity: Option<(&str, &str, i64)>) -> String {
    let mut command = hermetic_command(directory, ambient, arguments);
    if let Some((name, email, when)) = identity {
        command
            .env("GIT_AUTHOR_NAME", name)
            .env("GIT_AUTHOR_EMAIL", email)
            .env("GIT_COMMITTER_NAME", name)
            .env("GIT_COMMITTER_EMAIL", email)
            .env("GIT_AUTHOR_DATE", format!("{when} +0000"))
            .env("GIT_COMMITTER_DATE", format!("{when} +0000"));
    }
    let outcome = command.output().unwrap_or_else(|error| panic!("git is required to run this suite: {error}"));
    assert!(outcome.status.success(), "git {arguments:?} failed: {}", String::from_utf8_lossy(&outcome.stderr));
    String::from_utf8_lossy(&outcome.stdout).to_string()
}

fn person(value: &Value) -> (String, String) {
    (text(value, "name").to_string(), text(value, "email").to_string())
}

fn replay_recipe(directory: &Path) {
    replay_recipe_under(directory, &[]);
}

/// 🏗️ Builds the committed recipe with real git under `ambient` and answers the commit ids, the five of the main line first and the merge last.
fn replay_recipe_under(directory: &Path, ambient: &[(String, String)]) -> Vec<String> {
    let git = |arguments: &[&str], identity: Option<(&str, &str, i64)>| git_under(directory, ambient, arguments, identity);
    let recipe = fixture("🌱️repository-recipe.json");
    let branch = text(&recipe, "branch");
    git(&["init", "--quiet"], None);
    git(&["symbolic-ref", "HEAD", &format!("refs/heads/{branch}")], None);
    let first = person(&recipe["commits"][0]["author"]);
    git(&["config", "user.name", &first.0], None);
    git(&["config", "user.email", &first.1], None);
    git(&["config", "core.quotepath", "true"], None);
    git(&["config", "diff.renames", "true"], None);
    let mut shas: Vec<String> = Vec::new();
    for commit in rows(&recipe, "commits") {
        for change in rows(commit, "changes") {
            let path = text(change, "path");
            if change["delete"].as_bool() == Some(true) {
                git(&["rm", "--quiet", "--", path], None);
                continue;
            }
            let target = directory.join(path);
            std::fs::create_dir_all(target.parent().unwrap()).unwrap();
            if let Some(from) = change["rename"].as_str() {
                git(&["mv", from, path], None);
            }
            match change["bytes"].as_array() {
                Some(bytes) => std::fs::write(&target, bytes.iter().map(|byte| byte.as_u64().unwrap() as u8).collect::<Vec<u8>>()).unwrap(),
                None => std::fs::write(&target, change["content"].as_str().unwrap_or("")).unwrap(),
            }
            git(&["add", "--", path], None);
        }
        let author = person(&commit["author"]);
        git(&["commit", "--quiet", "-m", text(commit, "message")], Some((&author.0, &author.1, commit["when"].as_i64().unwrap())));
        shas.push(git(&["rev-parse", "HEAD"], None).trim().to_string());
    }
    let side = &recipe["side"];
    git(&["checkout", "--quiet", "-b", text(side, "branch"), &shas[side["from"].as_u64().unwrap() as usize]], None);
    std::fs::write(directory.join(text(side, "path")), text(side, "content")).unwrap();
    git(&["add", "--", text(side, "path")], None);
    let author = person(&side["author"]);
    git(&["commit", "--quiet", "-m", text(side, "message")], Some((&author.0, &author.1, side["when"].as_i64().unwrap())));
    git(&["checkout", "--quiet", branch], None);
    let merger = person(&side["mergeAuthor"]);
    git(&["merge", "--quiet", "--no-ff", "--no-edit", "-m", text(side, "mergeMessage"), text(side, "branch")], Some((&merger.0, &merger.1, side["mergeWhen"].as_i64().unwrap())));
    shas.push(git(&["rev-parse", "HEAD"], None).trim().to_string());
    shas
}

fn without_shas(value: Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(map.into_iter().filter(|(key, _)| key != "sha").map(|(key, inner)| (key, without_shas(inner))).collect()),
        Value::Array(items) => Value::Array(items.into_iter().map(without_shas).collect()),
        other => other,
    }
}

#[test]
fn the_recorded_transcript_is_what_real_git_prints_for_the_committed_recipe() {
    let scratch = Scratch::new("recipe");
    replay_recipe(&scratch.0);
    let source = SystemGit::new(&scratch.0);
    let recorded = transcript();
    assert_eq!(source.numstat_log("").unwrap(), recorded.logs[""], "numstat of HEAD, commit ids included");
    assert_eq!(source.numstat_log(DEFAULT_BRANCH).unwrap(), recorded.logs[DEFAULT_BRANCH], "numstat of the development branch, commit ids included");
    assert_eq!(git(&scratch.0, &["log", "--numstat", "--first-parent", "--reverse", "--pretty=format:COMMIT%x09%H%x09%aN%x09%aE%x09%at", DEFAULT_BRANCH], None), recorded.logs["🔀️with-merges"], "the log with its merge commit");
    assert_eq!(source.tracked_paths("").unwrap(), recorded.tracked[""], "tracked paths of the working tree");
    for path in &recorded.tracked[""] {
        if recorded.blobs.contains_key(&GitTranscript::blob_key("", path)) {
            assert_eq!(source.tracked_bytes("", path).unwrap(), recorded.tracked_bytes("", path).unwrap(), "{path}");
        }
    }
    let head = git(&scratch.0, &["rev-parse", "HEAD"], None).trim().to_string();
    assert_eq!(source.tracked_paths(&head).unwrap().len(), recorded.tracked[""].len());
    assert_eq!(source.tracked_bytes(&head, "pkg/main.go").unwrap(), recorded.tracked_bytes("", "pkg/main.go").unwrap());
    assert!(source.tracked_bytes("", "no/such/file").is_err());
    assert!(SystemGit::new(scratch.0.join("missing")).numstat_log("").is_err());
}

/// 😈️ Writes the recipe's hostile global configuration into `scratch` and answers the environment that points git at it.
fn hostile_ambient(scratch: &Scratch) -> Vec<(String, String)> {
    let recipe = fixture("🌱️repository-recipe.json");
    std::fs::write(scratch.0.join(".gitconfig"), text(&recipe["git"]["hostile"], "config")).unwrap();
    std::fs::write(scratch.0.join("global"), text(&recipe["git"]["hostile"], "config")).unwrap();
    let home = scratch.0.display().to_string();
    let mut ambient: Vec<(String, String)> = ["HOME", "USERPROFILE", "XDG_CONFIG_HOME"].iter().map(|key| (key.to_string(), home.clone())).collect();
    ambient.push(("GIT_CONFIG_GLOBAL".to_string(), scratch.0.join("global").display().to_string()));
    ambient.extend(pairs(&recipe["git"]["hostile"]["environment"]));
    ambient
}

#[test]
fn the_recipe_yields_the_recorded_commit_ids_under_any_ambient_git_configuration() {
    let recorded = transcript();
    let ids: Vec<String> = recorded.logs["🔀️with-merges"].lines().filter_map(|line| line.strip_prefix("COMMIT\t")).map(|rest| rest.split('\t').next().unwrap().to_string()).collect();
    assert_eq!(ids.len(), 6, "five main-line commits and the merge");
    let quiet = Scratch::new("ambient-quiet");
    assert_eq!(replay_recipe_under(&quiet.0, &[]), ids, "quiet ambient");
    let hostile_home = Scratch::new("ambient-hostile-home");
    let ambient = hostile_ambient(&hostile_home);
    let loud = Scratch::new("ambient-hostile");
    assert_eq!(replay_recipe_under(&loud.0, &ambient), ids, "hostile ambient");
    for id in &ids {
        assert_eq!(git_under(&loud.0, &ambient, &["ls-tree", "-r", "--name-only", "-z", id], None).split('\0').filter(|name| !name.is_empty()).map(str::to_string).collect::<Vec<_>>(), recorded.tracked[id], "tree of {id}");
    }
    assert!(!git_under(&loud.0, &ambient, &["cat-file", "-p", &ids[0]], None).contains("encoding"), "no encoding header, no signature header");
    assert_eq!(git_under(&loud.0, &ambient, &["config", "--get", "i18n.commitencoding"], None).trim(), "utf-8");
    let global = std::process::Command::new("git").args(["config", "--global", "--get", "i18n.commitencoding"]).current_dir(&loud.0).envs(ambient.iter().filter(|(key, _)| key != "GIT_DIR" && key != "GIT_INDEX_FILE").cloned()).output().unwrap();
    assert_eq!(String::from_utf8_lossy(&global.stdout).trim(), "latin1", "control: the ambient really is hostile to a git that is not shielded");
}

#[test]
fn the_report_over_real_git_equals_the_report_over_the_recorded_transcript() {
    let scratch = Scratch::new("report");
    replay_recipe(&scratch.0);
    let real = SystemGit::new(&scratch.0);
    for options in [LocOptions::default(), LocOptions { history: true, by_contributors: true, branch: DEFAULT_BRANCH.to_string(), ..LocOptions::default() }] {
        let live = build_loc_report(&real, &options, None, &default_contributor_alias).unwrap();
        let recorded = build_loc_report(&transcript(), &options, None, &default_contributor_alias).unwrap();
        assert_eq!(without_shas(as_value(&live)), without_shas(as_value(&recorded)), "history {}", options.history);
    }
    let recorded = fixture("📤️loc-report.json");
    assert_eq!(as_value(&build_loc_report(&real, &LocOptions::default(), None, &default_contributor_alias).unwrap().snapshot), recorded["snapshot"]);
}

#[test]
fn a_long_history_is_walked_in_parallel_ranges_that_read_exactly_like_one_git_log() {
    let scratch = Scratch::new("chunks");
    git(&scratch.0, &["init", "--quiet"], None);
    for index in 0..70_i64 {
        let name = format!("file{}.ts", index % 4);
        let body = (0..=(index % 7)).map(|line| format!("line {index} {line}\n")).collect::<String>();
        std::fs::write(scratch.0.join(&name), body).unwrap();
        git(&scratch.0, &["add", "--", &name], None);
        git(&scratch.0, &["commit", "--quiet", "-m", &format!("commit {index}")], Some(("Ada Lovelace", "ada@x.org", 1_700_000_000 + index * 60)));
    }
    let direct = git(&scratch.0, &["log", "--no-merges", "--numstat", "--first-parent", "--reverse", "--pretty=format:COMMIT%x09%H%x09%aN%x09%aE%x09%at"], None);
    let chunked = SystemGit::new(&scratch.0).numstat_log("").unwrap();
    assert_eq!(chunked, direct);
    let parsed = parse_numstat_log(&chunked, &weights(), None);
    assert_eq!(parsed.len(), 70);
    assert!(parsed.windows(2).all(|pair| pair[0].when_unix < pair[1].when_unix), "oldest first");
    assert_eq!(parsed[0].delta["TypeScript"], LineMetrics { added: 1, removed: 0 });
}
//#endregion 🔌️RealGit
