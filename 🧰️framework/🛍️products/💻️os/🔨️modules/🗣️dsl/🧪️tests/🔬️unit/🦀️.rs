use semio_framework_dsl::DslIdiom;
use semio_framework_dsl::hooks_for;
use semio_framework_dsl::LanguageRole;
use semio_framework_dsl::LanguageSpec;
use semio_framework_dsl::register_language;
use semio_framework_dsl::language;
use semio_framework_diagnostic::TextError;
use semio_framework_diagnostic::TextSpan;
use semio_framework_diagnostic::Limits;
use semio_framework_dsl::TokenClass;
use super::*;

include!("🧱️dsl-field-and-derived-document-laws/🦀️.rs");

include!("🌳️scene-and-shape-laws/🦀️.rs");

include!("🖌️paint-self-ref-and-table-laws/🦀️.rs");

#[path = "../🪆️refusal/🦀️.rs"]
mod controlled_refusal;

#[path = "../🔢️number-refusal/🦀️.rs"]
mod controlled_number_refusal;

include!("📦️paged-operation-laws/🦀️.rs");
