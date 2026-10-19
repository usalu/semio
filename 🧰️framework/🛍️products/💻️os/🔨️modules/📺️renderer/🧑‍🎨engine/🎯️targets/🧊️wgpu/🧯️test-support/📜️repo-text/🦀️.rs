//! 📜️ Repository text readers of the renderer source-text laws: run-time reads replace the `include_str!` embeds of production sources and the `*_SOURCE` consts.
//!
//! 🧭️ Every path is relative to the repository root (`repo_text`, `repo_texts`) or to the renderer engine folder (the source sets); the root is found by walking up from this crate's manifest, so a reader works from any test crate of the renderer. Texts are read once and kept for the process.
//! @see <https://doc.rust-lang.org/std/macro.include_str.html>

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use crate::module_tree::read_module_tree_source;

const ENGINE: &str = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine";

/// 🧾️ The files the renderer root, its hubs and the crates they mount held as one text (the former `RENDERER_SOURCE`), relative to the engine folder.
pub const RENDERER_SOURCE_FILES: &[&str] = &[
    "🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📥️mounts/🌱️wp1-foundation/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📥️mounts/🎞️wp2-scenes/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📥️mounts/🧵️wp3-frame-core/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📥️mounts/🐚️wp4-shell-state/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📥️mounts/🎛️wp5-shell-views/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🗂️asset-authority/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🔤️json-lexer-and-schema-types/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🗃️schema-output-and-cursor-state/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🧮️schema-token-application/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🏁️schema-finishing-and-plan-types/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/📐️instance-plan-math-and-outline/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🧵️materialize-cursor-step/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/📖️materialize-readers-and-component-helpers/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs",
    "🎯️targets/🧊️wgpu/🏛️host-base/🦀️.rs",
    "🎯️targets/🧊️wgpu/🏛️host-base/📥️lifted/🔁️asset-decode-jobs/🦀️.rs",
    "🎯️targets/🧊️wgpu/🏛️host-base/📥️lifted/🏊️kernel-pool/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧠️kernel-runtime/🦀️.rs",
    "🎯️targets/🧊️wgpu/📡️kernel-protocol/🦀️.rs",
    "🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🦀️.rs",
    "🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🔁️spawn-recovery/🦀️.rs",
    "🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🚫️request-refusal/🦀️.rs",
    "🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🛂️admission-claim/🦀️.rs",
    "🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/📄️typed-result/🦀️.rs",
    "🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/🏊️kernel-pool/🦀️.rs",
    "🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📦️kernel-runtime/🦀️.rs",
    "🎯️targets/🧊️wgpu/🏊️kernel-pool/📥️lifted/🗃️kernel-retained-state/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/📬️kernel-replay-client/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🗃️kernel-retained-state/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🏗️pool-construction-and-product-replay/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/📼️job-replay-and-command-maintenance/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🧩️app-lifecycle-and-realm-close/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🔄️exchange-and-turn-settling/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🚀️turn-dispatch-and-pool-runner/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📊️scale-bench/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🔁️runtime-apply/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🎮️interaction-state/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🎨️presentation-raster/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🎨️presentation-raster/🌐️native-asset-stream/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🏗️mailbox-construction-and-authorities/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/📥️renderer-asset-steps/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🖼️asset-decode-pump-and-reference-images/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🌐️native-asset-pumps/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/⏳️pending-work-deadlines-and-spawns/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🧾️frame-build/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🧾️frame-build/🧮️frame-transaction/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🧾️frame-build/📋️frame-preparation/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🎞️present-state-and-engine-closers/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🧯️retirement-and-present-steps/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🎬️present-step-inner/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🧠️app-runtime/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧊️renderer/🔬️native-socket-probe/🦀️.rs",
];

/// 🧾️ The files of the Scenes facade held as one text (the former `SCENES_SOURCE`).
pub const SCENES_SOURCE_FILES: &[&str] = &[
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/📐️viewport-drag-and-accessibility-controls/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🧯️surface-state-and-retirement/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🖱️canvas-gestures-input-kit-and-chrome/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/✍️editor-ink-gumball-and-canvas-layer-wire/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🪣️pending-raster/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/📷️scene-camera-dispatch/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/⌨️scene-input/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🎨️scene-render-entry/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧮️table-and-paint2d/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧱️block-list-and-diff/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/📰️event-feed-and-timeline/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖌️canvas2d/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖌️canvas2d/🪣️packet-image-and-blend/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖌️canvas2d/🔷️shape-fills-and-render/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖌️canvas2d/🖼️scene-nodes-and-image-upload/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🧾️ink-document-model-and-edit-helpers/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🔎️ink-interaction-document-and-block-scanning/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/📤️ink-event-pages-and-edit-routing/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/📋️ink-clipboard-camera-and-wheel/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🔁️ink-interaction-job-scan/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🏁️ink-interaction-job-publish-and-close/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖋️ink-canvas-render/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧭️node-graph-and-tiled-map/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖼️icon-render/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧩️board2d-and-vfs/🦀️.rs",
    "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✍️text-editor/🦀️.rs",
];

/// 🧾️ The files of the EngineCanvas facade held as one text (the former `ENGINE_CANVAS_SOURCE`).
pub const ENGINE_CANVAS_SOURCE_FILES: &[&str] = &[
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/🆔️surface-ids-and-registry/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/🧯️surface-retirement-ladder/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/🗃️sync-caches-and-surface-helpers/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/✍️text-editor-delivery-and-diagnostics/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/📦️prepared-engine-canvas/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🎓️tutorial-surface-geometry/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/📎️registry-and-node-graph/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🗺️graph-scene-and-map-sync/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🎲️board-sync/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🖌️paint-editor-and-composite/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🕸️node-graph/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🕸️node-graph/🎥️scene-camera-and-drag-drop/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🕸️node-graph/🖱️wheel-and-pointer-into/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🕸️node-graph/🔗️screen-pointer-and-commit/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🕸️node-graph/📌️interaction-dispatch/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🕸️node-graph/🏷️labels-and-overlays/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗺️tiled-map/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🎲️board2d/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🖌️paint2d/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/✍️text-editor/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/✍️text-editor/📄️scene-document-and-host-sync/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/✍️text-editor/📤️draft-actions-and-outbox/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/✍️text-editor/🧾️receipts-and-explicit-draft/🦀️.rs",
    "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/✍️text-editor/🖋️editing-and-pointer-api/🦀️.rs",
];

/// 🧾️ The files of the Interpreter facade held as one text (the former `INTERPRETER_SOURCE`).
pub const INTERPRETER_SOURCE_FILES: &[&str] = &[
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧮️render-plan-validator/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/📕️close-scene-pointer-and-dispatch/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/♿️accessibility-dispatch-and-focus/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/📋️receipts-drop-and-clipboard/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🎬️scene-intents-and-clipboard/🦀️.rs",
    "🎯️targets/🧊️wgpu/🤝️contracts/📥️lifted/🧩️retained-ui-engine/🦀️.rs",
    "🎯️targets/🧊️wgpu/🤝️contracts/📥️lifted/🖼️ui-image-loading/🦀️.rs",
    "🎯️targets/🧊️wgpu/🎭️scene-state/📥️lifted/🎯️chrome-ledger/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/✍️text-and-table-focus/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🖋️ink-and-vfs-focus/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🖱️scene-interaction/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/📄️retained-document-consumer/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🖼️ui-image-loading/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🔬️introspection/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🔬️introspection/🧾️dump-types-paths-and-walk/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🔬️introspection/🏗️structure-dump-and-table-accessibility/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🔬️introspection/♿️vfs-feed-world-and-timeline-accessibility/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🔬️introspection/📤️editor-accessibility-frame-stats-and-exports/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧱️mesh-and-board2d-stats/🦀️.rs",
    "🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🎯️chrome-ledger/🦀️.rs",
];

/// 🐚️ The roots of the Shell module tree: the `shell` parent and the library root of every Shell crate (the former `SHELL_SOURCE`), relative to the engine folder.
pub const SHELL_SOURCE_ROOTS: &[&str] = &[
    "🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
    "🎯️targets/🧊️wgpu/📖️shell-vocab/🦀️.rs",
    "🎯️targets/🧊️wgpu/⏳️shell-async/🦀️.rs",
    "🎯️targets/🧊️wgpu/🎚️shell-chrome-state/🦀️.rs",
    "🎯️targets/🧊️wgpu/🫧️shell-surfaces/🦀️.rs",
    "🎯️targets/🧊️wgpu/🧲️shell-plugins/🦀️.rs",
    "🎯️targets/🧊️wgpu/⏪️shell-history/🦀️.rs",
    "🎯️targets/🧊️wgpu/🎛️shell-prefs/🦀️.rs",
    "🎯️targets/🧊️wgpu/📒️shell-directory/🦀️.rs",
    "🎯️targets/🧊️wgpu/🔃️shell-sync/🦀️.rs",
    "🎯️targets/🧊️wgpu/📄️shell-docs/🦀️.rs",
    "🎯️targets/🧊️wgpu/📂️shell-doc-open/🦀️.rs",
    "🎯️targets/🧊️wgpu/🔲️shell-windows/🦀️.rs",
    "🎯️targets/🧊️wgpu/⚓️shell-dock/🦀️.rs",
    "🎯️targets/🧊️wgpu/🕹️shell-input/🦀️.rs",
    "🎯️targets/🧊️wgpu/🎹️shell-commands/🦀️.rs",
    "🎯️targets/🧊️wgpu/🐙️shell-state/🦀️.rs",
    "🎯️targets/🧊️wgpu/🚶️shell-session/🦀️.rs",
    "🎯️targets/🧊️wgpu/🪝️shell-window-actions/🦀️.rs",
    "🎯️targets/🧊️wgpu/🔀️shell-actions/🦀️.rs",
    "🎯️targets/🧊️wgpu/🖐️shell-pointer/🦀️.rs",
    "🎯️targets/🧊️wgpu/🖍️shell-view-paint/🦀️.rs",
    "🎯️targets/🧊️wgpu/🍱️shell-view-chrome/🦀️.rs",
    "🎯️targets/🧊️wgpu/🗨️shell-view-overlays/🦀️.rs",
    "🎯️targets/🧊️wgpu/🪀️shell-view-panels/🦀️.rs",
];

type Cache = Mutex<HashMap<String, &'static str>>;

fn cache() -> &'static Cache {
    static CACHE: OnceLock<Cache> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn remember(key: String, build: impl FnOnce() -> String) -> &'static str {
    if let Some(text) = cache().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&key) {
        return text;
    }
    let leaked: &'static str = Box::leak(build().into_boxed_str());
    cache().lock().unwrap_or_else(std::sync::PoisonError::into_inner).entry(key).or_insert(leaked)
}

/// 🗺️ The repository root: the first ancestor of this crate's manifest that holds the framework folder and a workspace manifest.
pub fn repo_root() -> &'static Path {
    static ROOT: OnceLock<PathBuf> = OnceLock::new();
    ROOT.get_or_init(|| Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|ancestor| ancestor.join("Cargo.toml").is_file() && ancestor.join("🧰️framework").is_dir()).expect("a repository root above the test-support manifest").to_path_buf())
}

/// 🧑‍🎨 The renderer engine folder (`…/📺️renderer/🧑‍🎨engine`).
pub fn engine_root() -> PathBuf {
    repo_root().join(ENGINE)
}

/// 📖️ One repository file as text, read once; a missing file names its path in the panic.
pub fn repo_text(relative: &str) -> &'static str {
    remember(format!("file:{relative}"), || {
        let path = repo_root().join(relative);
        std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{} is readable: {error}", path.display()))
    })
}

/// 📦️ One repository file as bytes, read once; a missing file names its path in the panic.
pub fn repo_bytes(relative: &str) -> &'static [u8] {
    static BYTES: OnceLock<Mutex<HashMap<String, &'static [u8]>>> = OnceLock::new();
    let table = BYTES.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(bytes) = table.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(relative) {
        return bytes;
    }
    let path = repo_root().join(relative);
    let leaked: &'static [u8] = Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("{} is readable: {error}", path.display())).into_boxed_slice());
    table.lock().unwrap_or_else(std::sync::PoisonError::into_inner).entry(relative.to_owned()).or_insert(leaked)
}

/// 📚️ Several repository files as one text, in order, each newline-terminated.
pub fn repo_texts(relatives: &[&str]) -> &'static str {
    remember(format!("files:{}", relatives.join("\n")), || {
        relatives
            .iter()
            .map(|relative| {
                let text = repo_text(relative);
                if text.ends_with('\n') {
                    text.to_owned()
                } else {
                    format!("{text}\n")
                }
            })
            .collect()
    })
}

/// 🌳️ One repository source file with its whole module tree (the file, then every `#[path]` and `include!` child in declaration order), read once.
pub fn repo_module_tree(relative: &str) -> &'static str {
    remember(format!("tree:{relative}"), || read_module_tree_source(&repo_root().join(relative)).unwrap_or_else(|error| panic!("{relative} module tree is readable: {error}")))
}

/// 🌳️ Several repository source files, each with its whole module tree, as one text in order.
pub fn repo_module_trees(relatives: &[&str]) -> &'static str {
    remember(format!("trees:{}", relatives.join("\n")), || relatives.iter().map(|relative| repo_module_tree(relative)).collect::<Vec<_>>().join("\n"))
}

fn engine_texts(relatives: &[&str]) -> &'static str {
    remember(format!("engine:{}", relatives.join("\n")), || {
        let full: Vec<String> = relatives.iter().map(|relative| format!("{ENGINE}/{relative}")).collect();
        let refs: Vec<&str> = full.iter().map(String::as_str).collect();
        repo_texts(&refs).to_owned()
    })
}

/// 🔎️ The renderer root, its hubs and the crates they mount as one text.
pub fn renderer_source() -> &'static str {
    engine_texts(RENDERER_SOURCE_FILES)
}

/// 🔎️ The Scenes facade as one text.
pub fn scenes_source() -> &'static str {
    engine_texts(SCENES_SOURCE_FILES)
}

/// 🔎️ The EngineCanvas facade as one text.
pub fn engine_canvas_source() -> &'static str {
    engine_texts(ENGINE_CANVAS_SOURCE_FILES)
}

/// 🔎️ The Interpreter facade as one text.
pub fn interpreter_source() -> &'static str {
    engine_texts(INTERPRETER_SOURCE_FILES)
}

/// 🔎️ The whole Shell module tree (the `shell` parent and every Shell crate) as one text, `🧪️tests` mounts excluded.
pub fn shell_source() -> &'static str {
    remember("shell".to_owned(), || SHELL_SOURCE_ROOTS.iter().map(|root| read_module_tree_source(&engine_root().join(root)).expect("the shell source tree is readable")).collect())
}
