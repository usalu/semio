//! 🏁️ Benchmark: the benchmark timing model.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

//#region ⏱️Benchmark

/// 🌐️ The ecosystems a benchmark sweep reports, in report-column order.
pub const BENCHMARK_LANGUAGES: [&str; 5] = ["Typescript", "Python", "Go", "C#", "Rust"];

/// 🔶️ One timing a benchmark run reported: which case, in which ecosystem, how long.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BenchmarkResult {
    #[serde(rename = "test")]
    pub test: String,
    #[serde(rename = "lang")]
    pub lang: String,
    #[serde(rename = "time")]
    pub time: String,
}

/// 🔬️ Extracts `name,time` timing lines from one ecosystem's benchmark stdout.
///
/// A benchmark process also prints warnings and paths, so only two-field lines whose first field
/// carries no `warning`, no `:` and no path separator are timings.
pub fn parse_benchmark_output(lang: &str, output: &str) -> Vec<BenchmarkResult> {
    output
        .split('\n')
        .filter_map(|raw| {
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                return None;
            }
            let parts: Vec<&str> = trimmed.split(',').collect();
            if parts.len() != 2 || parts[0].contains("warning") || parts[0].contains(':') || parts[0].contains('/') || parts[0].contains('\\') {
                return None;
            }
            Some(BenchmarkResult { test: parts[0].to_string(), lang: lang.to_string(), time: parts[1].to_string() })
        })
        .collect()
}

/// ⏱️ Parses a timing into seconds; accepts a bare number or an `s`/`ms`/`us`/`ns` suffix.
pub fn parse_duration_seconds(raw: &str) -> Option<f64> {
    let text = raw.trim();
    if text.is_empty() {
        return None;
    }
    let (number, factor) = if let Some(head) = text.strip_suffix("ms") {
        (head, 1e-3)
    } else if let Some(head) = text.strip_suffix("us").or_else(|| text.strip_suffix("µs")) {
        (head, 1e-6)
    } else if let Some(head) = text.strip_suffix("ns") {
        (head, 1e-9)
    } else if let Some(head) = text.strip_suffix('s') {
        (head, 1.0)
    } else {
        (text, 1.0)
    };
    number.trim().parse::<f64>().ok().map(|value| value * factor)
}

/// 🏆️ One summary row: a case, its timing per ecosystem and the fastest ecosystem for it.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BenchmarkRow {
    pub test: String,
    pub timings: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fastest: Option<String>,
}

/// 📋️ The whole benchmark table: the case names, the ecosystem columns and the rows.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BenchmarkSummary {
    pub tests: Vec<String>,
    pub languages: Vec<String>,
    pub rows: Vec<BenchmarkRow>,
}

/// 📊️ Folds raw timings into the report table; the first timing of a `(test, lang)` pair wins.
pub fn summarize_benchmarks(results: &[BenchmarkResult]) -> BenchmarkSummary {
    let tests: Vec<String> = results.iter().map(|entry| entry.test.clone()).collect::<BTreeSet<String>>().into_iter().collect();
    let languages: Vec<String> = BENCHMARK_LANGUAGES.iter().map(|value| (*value).to_string()).collect();
    let rows = tests
        .iter()
        .map(|test| {
            let mut timings: BTreeMap<String, String> = BTreeMap::new();
            for language in &languages {
                if let Some(found) = results.iter().find(|entry| entry.test == *test && entry.lang == *language) {
                    timings.insert(language.clone(), found.time.clone());
                }
            }
            let fastest = timings
                .iter()
                .filter_map(|(language, time)| parse_duration_seconds(time).map(|seconds| (language.clone(), seconds)))
                .min_by(|left, right| left.1.partial_cmp(&right.1).unwrap_or(std::cmp::Ordering::Equal).then_with(|| left.0.cmp(&right.0)))
                .map(|(language, _)| language);
            BenchmarkRow { test: test.clone(), timings, fastest }
        })
        .collect();
    BenchmarkSummary { tests, languages, rows }
}

/// ✏️ The benchmark report as CSV, one row per case and one column per ecosystem.
pub fn benchmark_csv(results: &[BenchmarkResult]) -> String {
    let summary = summarize_benchmarks(results);
    let mut out = String::new();
    out.push_str("Test,");
    out.push_str(&summary.languages.join(","));
    out.push('\n');
    for row in &summary.rows {
        out.push_str(&row.test);
        for language in &summary.languages {
            out.push(',');
            out.push_str(row.timings.get(language).map_or("", String::as_str));
        }
        out.push('\n');
    }
    out
}

//#endregion ⏱️Benchmark
