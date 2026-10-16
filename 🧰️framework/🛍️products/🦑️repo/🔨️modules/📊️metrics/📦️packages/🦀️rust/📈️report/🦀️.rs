//! 📈️ Report: the `LocReport` aggregation model.

use super::{make_numstat_lang_set, CommitDelta, LineMetrics, AGG_CODE, AGG_DATA, AGG_MARKUP, AGG_TOTAL, DEFAULT_BRANCH, DEFAULT_CODE_LANGUAGES};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

//#region 📊️Report

/// 🔢️ Serializes a float the way Go's `encoding/json` does: a value with no fractional part
/// is written as an integer, so `0` and not `0.0` reaches a report a reader diffs against the
/// reference.
pub fn go_float<S: serde::Serializer>(value: &f64, serializer: S) -> Result<S::Ok, S::Error> {
    if value.is_finite() && value.fract() == 0.0 && value.abs() < 9.007_199_254_740_992e15 {
        return serializer.serialize_i64(*value as i64);
    }
    serializer.serialize_f64(*value)
}

/// 🔢️ [`go_float`] for an optional float; `None` never reaches this, the field skips it.
pub fn go_float_option<S: serde::Serializer>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error> {
    match value {
        Some(inner) => go_float(inner, serializer),
        None => serializer.serialize_none(),
    }
}

/// 💿️ One row of a `loc` table: tree LOC, its share, branch churn share and the git delta sums.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocLangStats {
    #[serde(rename = "loc")]
    pub loc: i64,
    #[serde(rename = "percent", serialize_with = "go_float")]
    pub percent: f64,
    #[serde(rename = "since_prev_loc_percent", skip_serializing_if = "Option::is_none", serialize_with = "go_float_option")]
    pub since_prev_loc_percent: Option<f64>,
    #[serde(rename = "wip_percent", serialize_with = "go_float")]
    pub wip_percent: f64,
    #[serde(rename = "edited")]
    pub edited: i64,
    #[serde(rename = "added")]
    pub added: i64,
    #[serde(rename = "removed")]
    pub removed: i64,
}

/// 💿️ One commit step of the history series; `languages` and `by_contributors` are exclusive.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocHistoryEntry {
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "author", skip_serializing_if = "String::is_empty", default)]
    pub author: String,
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<BTreeMap<String, LocLangStats>>,
    #[serde(rename = "byContributors", skip_serializing_if = "Option::is_none")]
    pub by_contributors: Option<BTreeMap<String, BTreeMap<String, LocLangStats>>>,
}

/// 💿️ The whole `loc` payload: the snapshot table plus the optional breakdowns.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocReport {
    #[serde(rename = "snapshot")]
    pub snapshot: BTreeMap<String, LocLangStats>,
    #[serde(rename = "byContributors", skip_serializing_if = "Option::is_none")]
    pub by_contributors: Option<BTreeMap<String, BTreeMap<String, LocLangStats>>>,
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<LocHistoryEntry>>,
    #[serde(rename = "branch", skip_serializing_if = "String::is_empty", default)]
    pub branch: String,
}

/// 👤️ The default contributor alias: `Name <mail>`, or `unknown` when git recorded neither.
pub fn default_contributor_alias(name: &str, email: &str) -> String {
    let author = name.trim();
    let mail = email.trim();
    let combined = if mail.is_empty() { author.to_string() } else { format!("{author} <{mail}>") };
    if combined.trim().is_empty() {
        "unknown".to_string()
    } else {
        combined
    }
}

/// 🧮️ Running added/removed sums keyed by bucket.
pub type Cumulative = BTreeMap<String, LineMetrics>;

/// 🧮️ Running sums keyed by contributor alias and then by bucket.
pub type CumulativeByContributor = BTreeMap<String, Cumulative>;

/// 🧮️ Folds a commit stream into cumulative sums, optionally split by contributor and filtered.
pub fn cumulative_from_raw(commits: &[CommitDelta], languages: &[String], by_contributor: bool, contributor_filter: &str, alias: &dyn Fn(&str, &str) -> String) -> (Cumulative, Option<CumulativeByContributor>) {
    let weights = make_numstat_lang_set(languages);
    let mut cumulative: Cumulative = weights.iter().map(|bucket| (bucket.clone(), LineMetrics::default())).collect();
    let mut by_contributors: Option<CumulativeByContributor> = if by_contributor { Some(BTreeMap::new()) } else { None };
    let filter = contributor_filter.trim();
    for commit in commits {
        let who = alias(&commit.author, &commit.author_mail);
        if !filter.is_empty() && !filter.eq_ignore_ascii_case(&who) {
            continue;
        }
        if let Some(map) = by_contributors.as_mut() {
            map.entry(who.clone()).or_insert_with(|| weights.iter().map(|bucket| (bucket.clone(), LineMetrics::default())).collect());
        }
        for (bucket, pair) in &commit.delta {
            if !weights.contains(bucket) {
                continue;
            }
            let total = cumulative.entry(bucket.clone()).or_default();
            total.added += pair.added;
            total.removed += pair.removed;
            if let Some(map) = by_contributors.as_mut() {
                let row = map.entry(who.clone()).or_default().entry(bucket.clone()).or_default();
                row.added += pair.added;
                row.removed += pair.removed;
            }
        }
    }
    (cumulative, by_contributors)
}

/// 🧩️ Builds one table row out of the tree scan and the cumulative git pair for a bucket.
pub fn stat_from_pair_and_scan(cumulative: &Cumulative, scanned: Option<&BTreeMap<String, i64>>, key: &str) -> LocLangStats {
    let mut stat = LocLangStats { loc: scanned.and_then(|map| map.get(key).copied()).unwrap_or(0), ..LocLangStats::default() };
    if let Some(pair) = cumulative.get(key) {
        stat.added = pair.added;
        stat.removed = pair.removed;
        stat.edited = pair.added + pair.removed;
    }
    stat
}

/// 🧮️ Stamps `percent` with `Total.loc` as denominator; the `Total` row is 100 % by construction.
pub fn apply_percents(rows: &mut BTreeMap<String, LocLangStats>) {
    let denominator = rows.get(AGG_TOTAL).map_or(0, |row| row.loc);
    if denominator <= 0 {
        for row in rows.values_mut() {
            row.percent = 0.0;
        }
        return;
    }
    for (key, row) in rows.iter_mut() {
        row.percent = if key == AGG_TOTAL { 100.0 } else { (10000.0 * row.loc as f64 / denominator as f64).round() / 100.0 };
    }
}

/// 📊️ Sums added + removed over every weighted bucket: the branch's whole edited-line churn.
pub fn sum_edited_pairs(cumulative: &Cumulative, weights: &BTreeSet<String>) -> i64 {
    weights.iter().filter_map(|bucket| cumulative.get(bucket)).map(|pair| pair.added + pair.removed).sum()
}

/// 🧮️ Stamps `wip_percent` from edited churn; a zero global denominator falls back to `Total`.
pub fn apply_wip_percents(rows: &mut BTreeMap<String, LocLangStats>, global_denominator: i64) {
    let mut denominator = global_denominator;
    if denominator <= 0 {
        denominator = rows.get(AGG_TOTAL).map_or(0, |row| row.edited);
    }
    if denominator <= 0 {
        for row in rows.values_mut() {
            row.wip_percent = 0.0;
        }
        return;
    }
    for row in rows.values_mut() {
        row.wip_percent = (10000.0 * row.edited as f64 / denominator as f64).round() / 100.0;
    }
}

/// 📊️ Merges tree scan and git deltas into the full row set with its `Code`/`Total` aggregates.
pub fn compose_loc_report_snapshot(cumulative: &Cumulative, scanned: Option<&BTreeMap<String, i64>>, languages: &[String], wip_global_denominator: i64) -> BTreeMap<String, LocLangStats> {
    let weights = make_numstat_lang_set(languages);
    let mut rows: BTreeMap<String, LocLangStats> = BTreeMap::new();
    let mut code = LocLangStats::default();
    for language in DEFAULT_CODE_LANGUAGES {
        if !weights.contains(language) {
            continue;
        }
        let stat = stat_from_pair_and_scan(cumulative, scanned, language);
        code.loc += stat.loc;
        code.added += stat.added;
        code.removed += stat.removed;
        code.edited += stat.edited;
        rows.insert(language.to_string(), stat);
    }
    let markup = stat_from_pair_and_scan(cumulative, scanned, AGG_MARKUP);
    let data = stat_from_pair_and_scan(cumulative, scanned, AGG_DATA);
    let total = LocLangStats {
        loc: code.loc + markup.loc + data.loc,
        edited: code.edited + markup.edited + data.edited,
        added: code.added + markup.added + data.added,
        removed: code.removed + markup.removed + data.removed,
        ..LocLangStats::default()
    };
    rows.insert(AGG_MARKUP.to_string(), markup);
    rows.insert(AGG_DATA.to_string(), data);
    rows.insert(AGG_CODE.to_string(), code);
    rows.insert(AGG_TOTAL.to_string(), total);
    apply_percents(&mut rows);
    apply_wip_percents(&mut rows, wip_global_denominator);
    rows
}

/// 📶️ Row names ordered by tree LOC descending, ties broken by name, with `Total` last.
pub fn sorted_row_keys(rows: &BTreeMap<String, LocLangStats>) -> Vec<String> {
    sorted_rows_by(rows, |row| row.loc)
}

/// 📶️ Row names ordered by edited churn descending, ties broken by name, with `Total` last.
pub fn sorted_row_keys_churn(rows: &BTreeMap<String, LocLangStats>) -> Vec<String> {
    sorted_rows_by(rows, |row| row.edited)
}

/// 📶️ Shared ordering: one numeric key descending, name ascending, `Total` pinned last.
fn sorted_rows_by(rows: &BTreeMap<String, LocLangStats>, key: impl Fn(&LocLangStats) -> i64) -> Vec<String> {
    let mut names: Vec<String> = rows.keys().filter(|name| *name != AGG_TOTAL).cloned().collect();
    names.sort_by(|left, right| {
        let value_left = rows.get(left).map_or(0, &key);
        let value_right = rows.get(right).map_or(0, &key);
        value_right.cmp(&value_left).then_with(|| left.cmp(right))
    });
    if rows.contains_key(AGG_TOTAL) {
        names.push(AGG_TOTAL.to_string());
    }
    names
}

/// 🧾️ Whether a table has real tree LOC, which is what decides between the wide and churn layouts.
pub fn use_full_tree_table(rows: &BTreeMap<String, LocLangStats>) -> bool {
    rows.get(AGG_TOTAL).is_some_and(|row| row.loc > 0)
}

/// 🔧️ Turns per-contributor cumulative pairs into per-contributor tables with zero tree LOC.
pub fn by_contributors_to_snapshot(source: &CumulativeByContributor, languages: &[String], branch_wip_denominator: i64) -> BTreeMap<String, BTreeMap<String, LocLangStats>> {
    let zero: BTreeMap<String, i64> = make_numstat_lang_set(languages).into_iter().map(|bucket| (bucket, 0)).collect();
    source.iter().map(|(alias, pairs)| (alias.clone(), compose_loc_report_snapshot(pairs, Some(&zero), languages, branch_wip_denominator))).collect()
}

/// 📉️ Percent change in tree LOC against the previous history row; `0 -> n` is defined as 100 %.
pub fn pct_loc_since_prev(previous: i64, current: i64) -> f64 {
    let value = match (previous, current) {
        (0, 0) => 0.0,
        (0, _) => 100.0,
        _ => 100.0 * (current - previous) as f64 / previous as f64,
    };
    (100.0 * value).round() / 100.0
}

/// 📉️ The language rows of one history step: the merged tree, else the single contributor row.
pub fn history_entry_stats(entry: &LocHistoryEntry) -> Option<&BTreeMap<String, LocLangStats>> {
    if let Some(languages) = entry.languages.as_ref() {
        if !languages.is_empty() {
            return Some(languages);
        }
    }
    entry.by_contributors.as_ref().and_then(|rows| rows.values().next())
}

/// 📉️ Stamps every history row's `since_prev_loc_percent`; the first row keeps `None`.
pub fn apply_history_loc_since_prev(history: &mut [LocHistoryEntry]) {
    let mut previous: Option<BTreeMap<String, i64>> = None;
    for entry in history.iter_mut() {
        let rows = match (entry.languages.as_mut(), entry.by_contributors.as_mut()) {
            (Some(languages), _) if !languages.is_empty() => languages,
            (_, Some(by_contributor)) => match by_contributor.values_mut().next() {
                Some(rows) => rows,
                None => continue,
            },
            _ => continue,
        };
        if let Some(prior) = previous.as_ref() {
            for (name, row) in rows.iter_mut() {
                row.since_prev_loc_percent = Some(pct_loc_since_prev(prior.get(name).copied().unwrap_or(0), row.loc));
            }
        } else {
            for row in rows.values_mut() {
                row.since_prev_loc_percent = None;
            }
        }
        previous = Some(rows.iter().map(|(name, row)| (name.clone(), row.loc)).collect());
    }
}

/// 📉️ Maps the common spellings of the development branch onto its emoji id.
pub fn display_history_branch(git_ref: &str) -> String {
    let trimmed = git_ref.trim();
    if trimmed.is_empty() {
        return trimmed.to_string();
    }
    let short = trimmed.strip_prefix("refs/heads/").unwrap_or(trimmed);
    if short.eq_ignore_ascii_case("wip") || short == DEFAULT_BRANCH {
        return DEFAULT_BRANCH.to_string();
    }
    trimmed.to_string()
}

/// 📉️ The short checkpoint id of a commit, matching the repository tree's checkpoint nodes.
pub fn history_checkpoint_label(sha: &str) -> String {
    let trimmed = sha.trim();
    let short: String = trimmed.chars().take(7).collect();
    format!("🔀️{short}")
}

/// 📉️ The contributor entity id of an alias, matching the repository tree's contributor nodes.
pub fn contributor_emoji_id(alias: &str) -> String {
    format!("🧑️‍💻️{}", alias.trim().to_lowercase().replace(' ', ""))
}

//#endregion 📊️Report
