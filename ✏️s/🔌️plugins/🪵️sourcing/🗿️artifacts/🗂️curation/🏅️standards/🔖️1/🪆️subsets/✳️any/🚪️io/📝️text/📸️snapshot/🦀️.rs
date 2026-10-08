//! 📜️ Sourcing curation artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::CurationSnapshot;

/// 📄️ The `demo` example, handcrafted in the `.curation` DSL.
pub const DEMO_STOCK_TEXT: &str = crate::examples::demo::PRIMARY_TEXT;

/// 📄️ The empty curation the shell's "no example" loads — empty stock and curated table. `catalog`'s
/// handle is content-addressed from an empty stock (`catalog_child_handle(&[])`, same value
/// `CurationSnapshot::default()` mints).
pub const EMPTY_CURATION_TEXT: &str = r#"semio curation.curation.dsl v1
catalog=child_id=catalog-4f53cda18c2baa0c target=artifact-id=catalog-4f53cda18c2baa0c artifact-kind=s.stdio.semio standard=v1 subset=kit stock-extra=[ ]
curated [object-id:REF count:UINT] {
}
"#;

/// 📖️ Parses `.curation` DSL text into a `CurationSnapshot`.
pub fn parse_dsl(text: &str) -> Result<CurationSnapshot, semio_framework_diagnostic::TextError> {
    <CurationSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `CurationSnapshot` back to `.curation` DSL text.
pub fn print_dsl(document: &CurationSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

impl store::ArtifactDsl for CurationSnapshot {
    const EXTENSION: &'static str = "curation";
    fn envelope_id() -> &'static str { "curation.curation" }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = if text.trim_start().starts_with("semio ") {
            let (envelope, body) = store::semio_format::split_text_preamble(text).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            if !envelope.matches_identity(Self::envelope_id(), store::semio_format::Component::Dsl, 1) { return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Curation text envelope mismatch", semio_framework_diagnostic::TextSpan::at(1, 1))); }
            body
        } else { text };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        let result = Self::__dsl_from_record(&record)?;
        result.validate().map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(result)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(), store::semio_format::Component::Dsl, 1).expect("Curation envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
/// 📥 Parses the document's native text representation.
pub fn parse_curation_dsl(text: &str) -> Result<CurationSnapshot, String> { parse_dsl(text).map_err(|error| format!("{error:?}")) }
/// 📤 Emits the document's native text representation.
pub fn print_curation_dsl(snapshot: &CurationSnapshot) -> String { print_dsl(snapshot) }

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
use crate::{CurationSnapshot, SourcingMutation};
use framework_schema::ArtifactSchema;
use semio_framework::parse_contributions;
use semio_framework_plugin::world3d_mesh_id_from_url;
use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::GeometryRecipe;
use crate::ObjectKind;
use crate::ObjectKindExtra;
use crate::CuratedItem;
use crate::Filters;

/// 🗂️ `topic_contribution.payload` shape for the `"sourcing.module"` topic.
/// See `TopicContribution` in `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`.
#[derive(semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub(crate) struct SourcingModuleTopicPayload {
    app_id: String,
    module_id: String,
    label: String,
    typology_json: String,
    kinds_json: String,
}

pub(crate) fn sourcing_json_envelope_is_bounded(input: &str) -> bool {
    if input.len() > SOURCING_JSON_MAX_BYTES {
        return false;
    }
    let mut depth = 0usize;
    let mut items = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut string_bytes = 0usize;
    let mut in_scalar = false;
    for byte in input.bytes() {
        if in_string {
            if escaped {
                string_bytes = string_bytes.saturating_add(1);
                escaped = false;
            } else if byte == b'\\' {
                string_bytes = string_bytes.saturating_add(1);
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            } else {
                string_bytes = string_bytes.saturating_add(1);
            }
            if string_bytes > SOURCING_JSON_MAX_STRING_BYTES {
                return false;
            }
            continue;
        }
        if in_scalar {
            if byte.is_ascii_whitespace() || matches!(byte, b',' | b']' | b'}') {
                in_scalar = false;
            } else {
                continue;
            }
        }
        match byte {
            b'"' => {
                items = items.saturating_add(1);
                in_string = true;
                string_bytes = 0;
            }
            b'{' | b'[' => {
                items = items.saturating_add(1);
                depth = depth.saturating_add(1);
                if depth > SOURCING_JSON_MAX_DEPTH {
                    return false;
                }
            }
            b'}' | b']' => {
                let Some(next) = depth.checked_sub(1) else {
                    return false;
                };
                depth = next;
            }
            b':' | b',' => {}
            byte if byte.is_ascii_whitespace() => {}
            _ => {
                items = items.saturating_add(1);
                in_scalar = true;
            }
        }
        if items > SOURCING_JSON_MAX_ITEMS {
            return false;
        }
    }
    !in_string && !escaped && depth == 0
}

pub(crate) fn contributed_sourcing_modules(contributions_json: &str) -> Vec<ContributedSourcingModule> {
    if !sourcing_json_envelope_is_bounded(contributions_json) {
        return Vec::new();
    }
    let mut modules = Vec::new();
    for entry in parse_contributions(contributions_json) {
        let Some(payload) = entry.topic_contribution.as_ref().filter(|topic| topic.topic == SOURCING_MODULE_TOPIC).and_then(|topic| topic.decode::<SourcingModuleTopicPayload>().ok()) else {
            continue;
        };
        let (app_id, module_id, label, typology_json, kinds_json) = (payload.app_id, payload.module_id, payload.label, payload.typology_json, payload.kinds_json);
        if app_id != SOURCING_CURATION_APP_ID {
            continue;
        }
        if !sourcing_json_envelope_is_bounded(&typology_json) || !sourcing_json_envelope_is_bounded(&kinds_json) {
            continue;
        }
        let Ok(typology) = semio_framework_pack_json::from_json_str::<TypologyNode>(&typology_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
            continue;
        };
        let Ok(kinds) = semio_framework_pack_json::from_json_str::<Vec<ObjectKind>>(&kinds_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
            continue;
        };
        if kinds.len() > SOURCING_JSON_MAX_ITEMS {
            continue;
        }
        modules.push(ContributedSourcingModule { module_id, label, typology, kinds });
    }
    modules
}

/// 🧩️ Every sourcing module known to this crate, in stable order: the three authored ones first, then
/// each contributed module whose id no module already serves — a `sourcing-module-beams` extension
/// re-contributing the authored `beams` module installs nothing, it does not duplicate it.
pub fn sourcing_modules(contributions_json: &str) -> Vec<SourcingModules> {
    let mut modules: Vec<SourcingModules> = authored_modules();
    for module in contributed_sourcing_modules(contributions_json) {
        if modules.len() >= SOURCING_MAXIMUM_MODULES {
            break;
        }
        if modules.iter().any(|installed| installed.module_id() == module.module_id()) {
            continue;
        }
        modules.push(SourcingModules::from(module));
    }
    modules
}

/// ✂️ The INSTALLABLE share of one host `ProgramContributionEntry[]` pack, re-encoded — the only thing
/// `setContributions` ever retains. A host pack is cut from the whole loaded closure and is unbounded;
/// the app's retained config lane is a fixed envelope, so the app keeps exactly what it can act on:
/// `sourcing.module` entries addressed to this app whose module id no installed module already serves,
/// in host order, while the re-encoded roster still fits `maximum_bytes`. Everything else is dropped
/// rather than retained — a duplicate module installs nothing anyway (see [`sourcing_modules`]), and
/// the three shipped `sourcing-module-{beams,slabs,windows}` extensions re-contribute exactly the
/// three modules this crate already authors, so the demonstrator's pack distills to `[]`.
pub fn installable_contributions(contributions_json: &str, maximum_bytes: usize) -> String {
    let empty = "[]".to_string();
    if !sourcing_json_envelope_is_bounded(contributions_json) {
        return empty;
    }
    let mut installed: Vec<String> = sourcing_modules("[]").iter().map(|module| module.module_id().to_string()).collect();
    let mut kept: Vec<semio_framework::ProgramContributionEntry> = Vec::new();
    for entry in parse_contributions(contributions_json) {
        if installed.len() >= SOURCING_MAXIMUM_MODULES {
            break;
        }
        let Some(module_id) = entry
            .topic_contribution
            .as_ref()
            .filter(|topic| topic.topic == SOURCING_MODULE_TOPIC)
            .and_then(|topic| topic.decode::<SourcingModuleTopicPayload>().ok())
            .filter(|payload| payload.app_id == SOURCING_CURATION_APP_ID)
            .map(|payload| payload.module_id)
        else {
            continue;
        };
        if installed.iter().any(|id| id == &module_id) {
            continue;
        }
        kept.push(entry);
        if semio_framework_pack_json::to_json_string(&kept).len() > maximum_bytes {
            kept.pop();
            continue;
        }
        installed.push(module_id);
    }
    if kept.is_empty() { empty } else { semio_framework_pack_json::to_json_string(&kept) }
}

/// 📄️ The demo-stock example, parsed once from `crate::dsl::DEMO_STOCK_TEXT` — the
/// source of truth for every "demo stock" call site (`setActiveExample`, `initial_snapshot`, tests).
/// The fixture's persisted `catalog` handle is content-addressed from `demo_stock()` (see
/// `crate::catalog_child_handle`) — re-deriving the same stock here and seeding the
/// working-scene cache with it resolves that exact handle, since a composed child is a handle only,
/// never inline content, in the persisted DSL text itself.
pub fn default_document() -> CurationSnapshot {
    crate::validate_catalog_payload(&demo_stock());
    <CurationSnapshot as store::ArtifactDsl>::parse_dsl(crate::standards::v1::subsets::any::io::text::snapshot::DEMO_STOCK_TEXT).expect("authored demo stock must match the curation schema")
}

/// 📄️ The empty-curation example, parsed once from
/// `crate::dsl::EMPTY_CURATION_TEXT` — empty stock, so its `catalog` handle is the
/// same content-addressed empty-catalog handle `CurationSnapshot::default()` mints.
pub fn empty_document() -> CurationSnapshot {
    crate::validate_catalog_payload(&[]);
    <CurationSnapshot as store::ArtifactDsl>::parse_dsl(crate::standards::v1::subsets::any::io::text::snapshot::EMPTY_CURATION_TEXT).expect("authored empty curation must match the curation schema")
}
}
pub use snapshot_codec::*;
