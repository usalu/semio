//! 🧮️ Action argument schema: formats, schemas, controls and argument definitions.

use semio_framework_pack_json::ArtifactCanonicalJsonText as Text;
use semio_framework_ui_contract::SliderAppearance;
use semio_framework_ui_contract::UiNumberBound;
use semio_framework_ui_contract::UiNumberLimits;
use semio_framework_ui_contract::UiNumberScale;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::Number;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::any::Any;
use std::error::Error;
use semio_framework_kernel_identity::ArtifactKind;

#[path = "📐️arg-format-and-schema/🦀️.rs"]
mod arg_format_and_schema;
pub use arg_format_and_schema::*;

#[path = "🎨️presentation-sources-and-reference-ids/🦀️.rs"]
mod presentation_sources_and_reference_ids;
pub use presentation_sources_and_reference_ids::*;

#[path = "🎛️action-arg-control/🦀️.rs"]
mod action_arg_control;
pub use action_arg_control::*;

#[path = "🧾️action-arg-definition/🦀️.rs"]
mod action_arg_definition;
pub use action_arg_definition::*;

#[path = "📜️format-application-and-json-schema/🦀️.rs"]
mod format_application_and_json_schema;
use format_application_and_json_schema::*;

#[path = "🔢️number-facets/🦀️.rs"]
mod number_facets;
pub use number_facets::*;

#[path = "🪪️app-role/🦀️.rs"]
mod app_role;
pub use app_role::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️ActionArgFacets
#[cfg(test)]
#[path = "../🧪️tests/🧪️number-facets/🦀️.rs"]
mod number_facets_tests;
//#endregion 🔖️ActionArgFacets
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
