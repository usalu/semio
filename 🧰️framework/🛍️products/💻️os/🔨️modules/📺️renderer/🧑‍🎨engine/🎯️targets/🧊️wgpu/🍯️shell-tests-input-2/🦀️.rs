//! 🍯️ Companion test bin of the wgpu renderer: input and pointer (2).
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

extern crate infinite_canvas as infinite_world;
#[cfg(any(test, not(target_arch = "wasm32")))]
extern crate semio_framework_os_kernel as store;
use semio_framework::kernel::UiDirtyScope;
use semio_framework::{AppDefinition, PanelGroup, PanelTabDefinition, ViewModel};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_actor::{ActorId as DirectoryActorId, PackageId as DirectoryPackageId};
use semio_framework_async::{CancelToken, OperationContext, TraceId};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::{Lane, ScopeOwner};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::DirectoryCommandOutcomeV1;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::client::{
    CanonicalDirectoryEventPageV1, DirectoryBootstrapTransition, DirectoryEventPageAckV1, DirectoryEventPageBootstrapV1, DirectoryStreamTurn, DirectoryTransport, DirectoryWsConnection, TransportError,
};
use semio_framework_os_kernel::os_directory::client::{DirectoryClient, DirectoryClientError};
use semio_framework_os_kernel::os_directory::io::text::directory_command_sha256;
use semio_framework_os_kernel::os_directory::{
    DirectoryCommand, DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryCommandResultV1, DirectorySpaceAdministrationCapabilitiesV1, DirectorySpaceAdministrationInviteRowV1,
    DirectorySpaceAdministrationMemberRowV1, DirectorySpaceAdministrationPageV1, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, mint_directory_command_request_id,
};
use semio_framework_os_renderer_wgpu_canvas2d_host::publish;
use semio_framework_os_renderer_wgpu_dock::{DockNode, DockStackTab};
use semio_framework_os_renderer_wgpu_doors::directory_door::ShellDirectoryTransport;
use semio_framework_os_renderer_wgpu_fixtures::shell::{host_test_shell, test_app, tree_pointer_record};
use semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions;
#[cfg(not(target_os = "wasi"))]
use semio_framework_os_renderer_wgpu_host_base::action_args_json;
use semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool;
use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::framework_widget_context;
use semio_framework_os_renderer_wgpu_scene_state::{SceneChromeLabels, SceneEngineHosts};
use semio_framework_os_renderer_wgpu_shell_actions::ActionsCtx;
use semio_framework_os_renderer_wgpu_shell_chrome_state::SHELL_SHORTCUT_ROWS;
use semio_framework_os_renderer_wgpu_shell_commands::{FUZZY_SEARCH_LIMIT, FUZZY_SEARCH_THRESHOLD, fuzzy_token_score, normalize_fuzzy_text, rank_fuzzy_items};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_directory::{DirectoryHomeProjection, ShellDirectoryHomePublicationOutcome, ShellDirectoryHomePublicationResult, ShellDirectoryPageResult, ShellDirectoryRunner};
use semio_framework_os_renderer_wgpu_shell_directory::{
    ShellSpaceAdministrationOperationV1, ShellSpaceAdministrationPhaseV1, ShellSpaceAdministrationTurnV1, shell_space_administration_controls, shell_space_administration_invite_revocable, shell_space_administration_member_removable,
    shell_space_administration_status,
};
use semio_framework_os_renderer_wgpu_shell_doc_open::document_socket_surface_from_descriptor;
use semio_framework_os_renderer_wgpu_shell_dock::{PanelAnchorState, ShellDock};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_docs::terminal_directory_home_ack;
use semio_framework_os_renderer_wgpu_shell_docs::{SearchPaletteItem, command_categories, command_category_label, note_shell_command_action, resolve_commands, shell_command_for_control};
use semio_framework_os_renderer_wgpu_shell_input::{OverlayState, PresentedInputCandidateWitness, ShellPaletteKind};
use semio_framework_os_renderer_wgpu_shell_pointer::{PointerCtx, ShellEditVerb, is_reserved_shell_chord, scope_context_menu_items, shell_context_menu_item_from_spec, shell_edit_verb_for, shell_shortcut_for, shell_shortcut_for_control_id};
use semio_framework_os_renderer_wgpu_shell_prefs::{ActiveSession, DockTabNode, ResolvedCommand, active_theme_id, command_address_stable_key};
use semio_framework_os_renderer_wgpu_shell_session::{SessionCtx, ShellPalettePlanEntry, ShellShortcut};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_surfaces::ShellPoolFuture;
use semio_framework_os_renderer_wgpu_shell_view_chrome::ChromeViewCtx;
use semio_framework_os_renderer_wgpu_shell_view_paint::SHELL_WINDOW_PAINT_OPPORTUNITIES;
use semio_framework_os_renderer_wgpu_shell_view_panels::{PanelsViewCtx, ShellChromeFrameCursor};
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_os_renderer_wgpu_shell_windows::PanelAnchor;
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::{UiDocumentFrameCursor, render_ui_document_step};
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger;
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_services::{ComputePool, TokioHostRuntime};
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
use semio_framework_value::DslValue;
use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use ui_contract::UiDocumentLease;
use ui_wgpu::wgpu::{ActionDescriptor, DrawList, FontAtlas, HitKind, HitTarget, IconAtlas, InputState, KeyAction, PointerModifiers, Rect, Theme, UiNode, WindowStackCorner};

#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-command-registry/🦀️.rs"]
mod command_registry_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs"]
mod shell_shortcuts_palette_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🎯️presented-input-authority/🦀️.rs"]
mod presented_input_authority_tests;
