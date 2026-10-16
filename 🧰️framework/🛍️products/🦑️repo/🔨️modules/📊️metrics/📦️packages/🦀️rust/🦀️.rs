//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 📊️ Repository metrics: the pure `git log --numstat` parser, the unified-LOC counters, the
//! `LocReport` aggregation, UTC time bucketing and the benchmark timing model.
//!
//! Behaviour twin of `github.com/usalu/semio/repo/metrics`. Everything in this crate is a pure
//! function over data; the only door to the outside world is [`GitLogSource`], which the CLI
//! satisfies with [`SystemGit`] and the tests satisfy with a recorded [`GitTranscript`].
//!
//! Rendering the `loc` CLI verb (flags, engine wiring, colour) is NOT here — it belongs to
//! `⌨️cli`. What this crate exposes instead is the report model plus the two pure table renderers
//! [`render_markdown`] and [`render_text`].
//!
//! See `🧬️schema/🔣️.json` for the wire shapes and `🧪️tests/` for the language-agnostic cases.

//#endregion 🧲️Header

pub use semio_framework_repo_model::LineMetrics;
pub use semio_framework_repo_workspace::{find_repo_root, GitIgnore};

#[path = "🏷️vocabulary/🦀️.rs"]
mod vocabulary;
use vocabulary::extension_of;
pub use vocabulary::{
    classify_for_numstat, classify_language_fine, classify_loc_bucket, join_numstat_chunks, make_lang_set, make_numstat_lang_set, normalize_repo_path, numstat_log_args, numstat_log_range_args, numstat_log_ranges, path_has_hidden_segment,
    path_is_repo_meta, path_skipped_for_loc, to_json_string, AGG_CODE, AGG_DATA, AGG_MARKUP, AGG_TOTAL, DEFAULT_BRANCH, DEFAULT_CODE_LANGUAGES, NUMSTAT_PRETTY,
};
#[path = "📏️counting/🦀️.rs"]
mod counting;
pub use counting::{count_json_keys, count_unified_loc_for_file, json_key_count, physical_line_count};
#[path = "🧩️numstat/🦀️.rs"]
mod numstat;
pub use numstat::{parse_numstat_log, resolve_numstat_path, unquote_git_path, CommitDelta, FileDelta};
#[path = "📈️report/🦀️.rs"]
mod report;
pub use report::{
    apply_history_loc_since_prev, apply_percents, apply_wip_percents, by_contributors_to_snapshot, compose_loc_report_snapshot, contributor_emoji_id, cumulative_from_raw, default_contributor_alias, display_history_branch, go_float, go_float_option,
    history_checkpoint_label, history_entry_stats, pct_loc_since_prev, sorted_row_keys, sorted_row_keys_churn, stat_from_pair_and_scan, sum_edited_pairs, use_full_tree_table, Cumulative, CumulativeByContributor, LocHistoryEntry, LocLangStats,
    LocReport,
};
#[path = "🕰️time/🦀️.rs"]
mod time;
pub use time::{bucket_commits, bucket_key, bucket_start, civil_from_unix, format_rfc3339_utc, iso_weekday, unix_days_from_civil, TimeBucket, TimeBucketGroup};
#[path = "🔌️git-source/🦀️.rs"]
mod git_source;
use git_source::MAXIMUM_NUMSTAT_WORKERS;
pub use git_source::{GitLogSource, GitTranscript, SystemGit};
#[path = "🧮️pipeline/🦀️.rs"]
mod pipeline;
pub use pipeline::{build_history, build_loc_report, snapshot_loc_counts, LocOptions};
#[path = "📤️render/🦀️.rs"]
mod render;
pub use render::{markdown_table, render_markdown, render_text, text_table};
#[path = "⏱️benchmark/🦀️.rs"]
mod benchmark;
pub use benchmark::{benchmark_csv, parse_benchmark_output, parse_duration_seconds, summarize_benchmarks, BenchmarkResult, BenchmarkRow, BenchmarkSummary, BENCHMARK_LANGUAGES};
