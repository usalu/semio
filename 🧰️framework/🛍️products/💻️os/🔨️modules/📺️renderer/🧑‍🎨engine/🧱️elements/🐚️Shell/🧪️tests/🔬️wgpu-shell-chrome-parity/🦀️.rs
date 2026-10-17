//! 🔀️ Laws for the wgpu shell's chrome parity with the React shell: the example picker, the mode and
//! surface-role navbar groups and their chords, and the two per-surface overlay controls (the World3d
//! compute cancel). Framing the flow graph is an app-declared action, not shell overlay chrome.
//!
//! Every law is answered from a SHARED, language-neutral fixture — `📚️example-picker.json` (the
//! dialect-keyed picker, also answered by `manifest::examples_for_app`'s own Rust law and by the
//! TypeScript `examplesForApp` twin), `🔀️surface-switch/🔣️.json` (the React surface-switch lane's
//! own corpus) and this element's `🛑️surface-controls/🔣️.json` (also answered by the TypeScript
//! `world3dComputeStatusV1` twin). Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.

use semio_framework_os_renderer_wgpu_fixtures::shell::{host_test_shell, retained_world_sequence_probe};
use super::*;
use {semio_framework::AppDefinition,semio_framework::AppRole,semio_framework_artifact_reference::ArtifactDialect,semio_framework::ExampleDefinition,semio_framework::ModeDefinition,semio_framework::Modes,semio_framework::WindowKindDefinition,semio_framework::WindowKinds};
use semio_framework_ui_layout::layout::WindowLayout;

include!("📚️fixtures-and-parity-builders/🦀️.rs");

include!("🎛️example-picker-and-role-groups/🦀️.rs");

include!("🛑️chords-controls-and-compute-status/🦀️.rs");

include!("🗣️chrome-i18n-and-switch-order/🦀️.rs");

include!("🪟️window-scope-laws/🦀️.rs");

include!("🛟️safe-area-and-incoming-layout/🦀️.rs");
