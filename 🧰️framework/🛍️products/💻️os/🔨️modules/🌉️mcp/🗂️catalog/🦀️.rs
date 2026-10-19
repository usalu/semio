//! 🗂️ `CapabilityDefinition` + the catalog compiler — packet `P2-catalog`, `📓️design-decisions.md`
//! D5 (these types live HERE, not in `🛂️manifest`) and D3 (the `<plugin_id>.<app_id>.<action_id>`
//! id grammar is mandatory: a bare action id is never a capability id, since 14 action ids collide
//! across plugins). `compile()` walks real `semio_framework::manifest` types
//! (`PackageDescriptor`/`PluginManifest`/`AppDefinition`) — this facet is the ONLY place in the
//! gateway crate that depends on `semio-framework`; every other facet stays framework-free
//! (`📓️terra-P1a-report.md` §5, D8) so the peer ticket's mid-flight plugin-host rewrite can never
//! break this crate's own build.

use semio_framework::manifest::kernel;
use semio_framework::manifest;
use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::Terminology;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use std::collections::BTreeMap;
use semio_framework_os_mcp_schema::schema::{artifact_create_template_input_schema, capability_action_input_schema, capability_generic_input_schema, capability_generic_output_schema, compile_validator, ui_dialog_open_input_schema, validate, with_artifact_create_templates};

#[path = "🪪️capability-identity/🦀️.rs"]
mod capability_identity;
pub use capability_identity::*;

#[path = "🧰️presentation-and-definition/🦀️.rs"]
mod presentation_and_definition;
pub use presentation_and_definition::*;

#[path = "♻️framework-dedup/🦀️.rs"]
mod framework_dedup;
use framework_dedup::*;

#[path = "🏭️capability-builders/🦀️.rs"]
mod capability_builders;
pub use capability_builders::*;

#[path = "📚️catalog-compile/🦀️.rs"]
mod catalog_compile;
pub use catalog_compile::*;

#[path = "🔍️catalog-audit/🦀️.rs"]
mod catalog_audit;
pub use catalog_audit::*;

#[path = "💬️description-law/🦀️.rs"]
mod description_law;
pub use description_law::*;

//#region 🧪️Tests
//#endregion 🧪️Tests
