//! 🎟️ Test host of `child_emit_preparation`: runs `🧪️tests` in the facade, where the names it uses are in scope.

use crate::{store,protocol};
use crate::app_emission::ChildEmit;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_diagnostic::{Fault,FaultFrom};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::{mem::ManuallyDrop, sync::Arc};
use store::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement};
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use crate::child_emit_preparation::*;
#[path = "../../../../🧩️composition/📨️emission/📦️preparation/🎟️receipt/🧪️tests/🦀️.rs"]
pub(crate) mod original_tests;
