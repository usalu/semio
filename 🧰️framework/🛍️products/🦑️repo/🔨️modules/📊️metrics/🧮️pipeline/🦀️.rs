//! 🧮️ Pipeline: the pure functions that turn a git log into a report.

use super::{
    apply_history_loc_since_prev, by_contributors_to_snapshot, classify_for_numstat, compose_loc_report_snapshot, count_unified_loc_for_file, cumulative_from_raw, format_rfc3339_utc, make_numstat_lang_set, parse_numstat_log, path_skipped_for_loc,
    sum_edited_pairs, CommitDelta, Cumulative, CumulativeByContributor, GitIgnore, GitLogSource, LineMetrics, LocHistoryEntry, LocReport, DEFAULT_BRANCH, DEFAULT_CODE_LANGUAGES, MAXIMUM_NUMSTAT_WORKERS,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

//#region 🧮️Pipeline

/// 📸️ Walks the tracked tree at a ref and sums unified LOC per `loc` bucket.
pub fn snapshot_loc_counts(source: &dyn GitLogSource, git_ref: &str, languages: &[String], ignore: Option<&GitIgnore>) -> Result<BTreeMap<String, i64>, String> {
    let weights = make_numstat_lang_set(languages);
    let mut counts: BTreeMap<String, i64> = weights.iter().map(|bucket| (bucket.clone(), 0)).collect();
    let considered: Vec<(String, String)> = source
        .tracked_paths(git_ref)?
        .into_iter()
        .filter(|rel| !path_skipped_for_loc(rel, ignore))
        .filter_map(|rel| classify_for_numstat(&rel, &weights).map(|bucket| (rel, bucket)))
        .collect();
    let next = AtomicUsize::new(0);
    let partials: Vec<Mutex<BTreeMap<String, i64>>> = (0..snapshot_scan_workers(considered.len())).map(|_| Mutex::new(BTreeMap::new())).collect();
    std::thread::scope(|scope| {
        for partial in &partials {
            scope.spawn(|| {
                let mut local: BTreeMap<String, i64> = BTreeMap::new();
                loop {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    let Some((rel, bucket)) = considered.get(index) else { break };
                    let Ok(data) = source.tracked_bytes(git_ref, rel) else { continue };
                    if data.contains(&0) {
                        continue;
                    }
                    *local.entry(bucket.clone()).or_insert(0) += count_unified_loc_for_file(rel, &data) as i64;
                }
                *partial.lock().expect("snapshot partial") = local;
            });
        }
    });
    for partial in partials {
        for (bucket, lines) in partial.into_inner().expect("snapshot partial") {
            *counts.entry(bucket).or_insert(0) += lines;
        }
    }
    Ok(counts)
}

/// 🧵️ How many threads read the tracked tree: the scan is one file read per entry and nothing else,
/// so it is bounded by the disk rather than by the host's cores.
fn snapshot_scan_workers(files: usize) -> usize {
    if files < 64 {
        return 1;
    }
    std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get).clamp(1, MAXIMUM_NUMSTAT_WORKERS)
}

/// 📅️ Assembles the per-commit history series, scanning the tree at every commit.
pub fn build_history(source: &dyn GitLogSource, languages: &[String], by_contributor: bool, commits: &[CommitDelta], contributor_filter: &str, ignore: Option<&GitIgnore>, alias: &dyn Fn(&str, &str) -> String) -> Result<Vec<LocHistoryEntry>, String> {
    let weights = make_numstat_lang_set(languages);
    let mut running: Cumulative = weights.iter().map(|bucket| (bucket.clone(), LineMetrics::default())).collect();
    let mut running_branch: Cumulative = running.clone();
    let mut running_by_contributor: CumulativeByContributor = BTreeMap::new();
    let mut history: Vec<LocHistoryEntry> = Vec::new();
    let filter = contributor_filter.trim();
    for commit in commits {
        for (bucket, pair) in &commit.delta {
            if !weights.contains(bucket) {
                continue;
            }
            let branch = running_branch.entry(bucket.clone()).or_default();
            branch.added += pair.added;
            branch.removed += pair.removed;
        }
        let who = alias(&commit.author, &commit.author_mail);
        if !filter.is_empty() && !filter.eq_ignore_ascii_case(&who) {
            continue;
        }
        if by_contributor {
            running_by_contributor.entry(who.clone()).or_insert_with(|| weights.iter().map(|bucket| (bucket.clone(), LineMetrics::default())).collect());
        }
        for (bucket, pair) in &commit.delta {
            if !weights.contains(bucket) {
                continue;
            }
            let total = running.entry(bucket.clone()).or_default();
            total.added += pair.added;
            total.removed += pair.removed;
            if by_contributor {
                let row = running_by_contributor.entry(who.clone()).or_default().entry(bucket.clone()).or_default();
                row.added += pair.added;
                row.removed += pair.removed;
            }
        }
        let scan = snapshot_loc_counts(source, &commit.sha, languages, ignore)?;
        let wip_denominator = sum_edited_pairs(&running_branch, &weights);
        let mut entry = LocHistoryEntry { sha: commit.sha.clone(), date: format_rfc3339_utc(commit.when_unix), author: who.clone(), ..LocHistoryEntry::default() };
        if by_contributor {
            let rows = compose_loc_report_snapshot(running_by_contributor.get(&who).unwrap_or(&BTreeMap::new()), Some(&scan), languages, wip_denominator);
            entry.by_contributors = Some(BTreeMap::from([(who.clone(), rows)]));
        } else {
            entry.languages = Some(compose_loc_report_snapshot(&running, Some(&scan), languages, wip_denominator));
        }
        history.push(entry);
    }
    Ok(history)
}

/// 🎛️ Everything a `loc` run decides before any git call happens.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LocOptions {
    /// 🗣️ The enabled code languages; markup and data are always counted when matched.
    pub languages: Vec<String>,
    /// 📜️ Whether to assemble the per-commit history series.
    pub history: bool,
    /// 👥️ Whether to break the cumulative deltas down by first author.
    pub by_contributors: bool,
    /// ⛳️ The ref the history walk logs; empty means `HEAD` and no branch label.
    pub branch: String,
    /// 👤️ Restricts deltas and history rows to one contributor alias, case-insensitively.
    pub contributor: String,
}

impl Default for LocOptions {
    fn default() -> Self {
        LocOptions { languages: DEFAULT_CODE_LANGUAGES.iter().map(|value| (*value).to_string()).collect(), history: false, by_contributors: false, branch: String::new(), contributor: String::new() }
    }
}

/// 📊️ The whole `loc` pipeline: log, fold, scan, compose — with git behind [`GitLogSource`].
pub fn build_loc_report(source: &dyn GitLogSource, options: &LocOptions, ignore: Option<&GitIgnore>, alias: &dyn Fn(&str, &str) -> String) -> Result<LocReport, String> {
    let weights = make_numstat_lang_set(&options.languages);
    let log_ref = if options.history { if options.branch.trim().is_empty() { DEFAULT_BRANCH.to_string() } else { options.branch.trim().to_string() } } else { String::new() };
    let commits = parse_numstat_log(&source.numstat_log(&log_ref)?, &weights, ignore);
    let (cumulative_all, by_contributor) = cumulative_from_raw(&commits, &options.languages, options.by_contributors, "", alias);
    let filter = options.contributor.trim();
    let cumulative_for_snapshot = if filter.is_empty() { cumulative_all.clone() } else { cumulative_from_raw(&commits, &options.languages, false, filter, alias).0 };
    let scan = snapshot_loc_counts(source, "", &options.languages, ignore)?;
    let wip_denominator = sum_edited_pairs(&cumulative_all, &weights);
    let mut report = LocReport { snapshot: compose_loc_report_snapshot(&cumulative_for_snapshot, Some(&scan), &options.languages, wip_denominator), branch: log_ref, ..LocReport::default() };
    if options.by_contributors {
        let selected = by_contributor.map(|rows| if filter.is_empty() { rows } else { rows.into_iter().filter(|(alias, _)| alias.eq_ignore_ascii_case(filter)).collect() });
        report.by_contributors = selected.map(|rows| by_contributors_to_snapshot(&rows, &options.languages, wip_denominator));
    }
    if options.history {
        let mut history = build_history(source, &options.languages, options.by_contributors, &commits, filter, ignore, alias)?;
        apply_history_loc_since_prev(&mut history);
        report.history = Some(history);
    }
    Ok(report)
}

//#endregion 🧮️Pipeline
