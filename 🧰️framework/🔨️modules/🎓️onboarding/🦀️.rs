//! 🎓️ App onboarding: the step-gated first-run introduction, the recorded tutorial timeline with its playback engine and the reserved actions that start or record them.

use replication::DomainSelection;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use semio_framework_ui_declaration::layout::WindowLayout;
use semio_framework_manifest_actions::{ActionDefinition, ActionKind};
use semio_framework_manifest_address::{ActionRef, ToolRef, UtilityRef};
use semio_framework_manifest_arg_schema::{ActionArgDef, ActionArgOption};

#[path = "🎬️introduction-definition/🦀️.rs"]
mod introduction_definition;
pub use introduction_definition::*;

#[path = "🖱️introduction-interaction/🦀️.rs"]
mod introduction_interaction;
pub use introduction_interaction::*;

#[path = "📚️tutorial-definition/🦀️.rs"]
mod tutorial_definition;
pub use tutorial_definition::*;

#[path = "🎞️tutorial-events-and-ui/🦀️.rs"]
mod tutorial_events_and_ui;
pub use tutorial_events_and_ui::*;

#[path = "🎥️tutorial-engine/🦀️.rs"]
mod tutorial_engine;
pub use tutorial_engine::*;

#[cfg(test)]
use semio_framework_manifest_address::window_element_id;
#[cfg(test)]
use semio_framework_manifest_arg_schema::ActionArgControl;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
