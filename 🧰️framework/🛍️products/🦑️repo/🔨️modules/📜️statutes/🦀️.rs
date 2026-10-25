//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 📜️ Repository statutes: the closed catalog of statutes and policies, the breach analysis that
//! detects violations of them, the ignore directives that suppress a breach, the autofix
//! transformations, and the compressed breach-cache envelope.
//! Behaviour twin of `github.com/usalu/semio/repo/statutes`; the vocabulary comes from
//! `🧬️schema/🔣️statutes.json`, the one table the twin is meant to read too.

//#endregion 🧲️Header

/// 🚪️ The model types the statute surface speaks in, re-exported so a client never depends on
/// `📐️model` directly to read a catalog entry or a breach.
pub use semio_framework_repo_model::{Breach, BreachPriority, Policy, Statute, StatuteMeta, Territory};

#[path = "📚️catalog/🦀️.rs"]
mod catalog;
pub use catalog::{catalog, find_policy, is_autofixable, policies, policy_kinds, statute_info, statutes, territory_kinds, Catalog};
#[path = "🙈️ignore-directives/🦀️.rs"]
mod ignore_directives;
pub use ignore_directives::{extract_file_from_scope, filter_ignored, is_ignored, parse_ignore_directives};
#[path = "🔶️breach/🦀️.rs"]
mod breach;
pub use breach::{build_breach_id, create_breach};
#[path = "📐️specification-text/🦀️.rs"]
mod specification_text;
use specification_text::contributor_line;
pub use specification_text::{has_implementation_syntax, is_spec_text};
#[path = "🗂️sources/🦀️.rs"]
mod sources;
use sources::{is_test_or_benchmark_file, policy_section_end_match, policy_section_start_match, requires_definition_requirements};
pub use sources::{SourceFile, SourceSet};
#[path = "🔍️analyze/🦀️.rs"]
mod analyze;
pub use analyze::{analyze, header_policy, requirements_policy, section_policy};
#[path = "🩹️autofix/🦀️.rs"]
mod autofix;
pub use autofix::{autofix, FixedFile};
#[path = "🔏️digest/🦀️.rs"]
mod digest;
pub use digest::{sha256, sha256_hex};
#[path = "🗜️deflate/🦀️.rs"]
mod deflate;
pub use deflate::{gzip_decode, gzip_encode};
#[path = "🧷️breach-cache/🦀️.rs"]
mod breach_cache;
pub use breach_cache::{breach_cache_digest, breachs_from_cache, decode_breach_cache, encode_breach_cache, encode_breach_cache_json, parse_breach_cache, BreachCacheEnvelope};

//#region 🧪️Tests

#[cfg(test)]
#[path = "./🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
