// 🌱️ HUB WP1: mount hub of WP1 foundation, kernel and small elements: contracts, host-base, native-platform, media-slots, canvas2d-paint, asset-authority, asset-decode, doors, kernel-*, hub-*, agent-elements, dock, program-bridge, mailbox-core.
// This file is spliced into the crate root by `include!` from the renderer root, so every item below stays a crate-root item;
// its `#[path]` and `include!` literals resolve against this folder (`📥️mounts/<hub>/`), hence the `../../` prefix of the renderer folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-RENDERER-WGPU-CRATES.md, section 10.3).

extern crate semio_framework_os_renderer_wgpu_dock as dock;

//#region 🤖️AgentBridgeElements
// 🌉️ WGPU-RENDERER-REACT-PARITY packet W1j — the four `os.agent.*` elements React's `🏛️ShellHost`
// mounts (`useAgentBridge` + `AgentChatPanel` + `AgentApprovals`) had no wgpu twin at all, so an
// approval request delivered to a wgpu-rendered shell could never reach the user. Declared in
// dependency order for readability only; `agent_bridge` owns the frame codec and the consumer
// state, the other three are its renderers.

// 🌉️ WGPU-RENDERER-REACT-PARITY packet W15e — the socket itself, the one third of React's
// `useAgentBridge` W1j deliberately left out: the browser half rides the duplex page door, the native
// half a `tokio-tungstenite` connection on `Lane::Io`. Both drive the same bounded `SocketLane`.
pub use semio_framework_os_renderer_wgpu_agent_elements::{agent_approvals, agent_bridge, agent_bridge_door, agent_chat_panel, agent_presence};
//#endregion 🤖️AgentBridgeElements

// 🛂️ WGPU-RENDERER-REACT-PARITY packet W15e — the administration sheet's CHROME. Its operation and
// control set have been live on both targets since W1e; nothing painted them until this module.
pub use semio_framework_os_renderer_wgpu_hub_workspace::space_administration;

//#region 🔐️HubElements
// 🔐️ Ticket 26/09/18 slice WG6 (G8 items WG-6 and WG-5): the three os-renderer elements AU2/AU3
// landed for React only — `🔐️HubSignIn`, `🏘️SpaceBrowser`, `🔗️HubConnection` — so a wgpu-rendered
// shell had no hub identity of its own at all (its only identity was an inherited local-bootstrap
// credential or a same-origin cookie). Declared in dependency order for readability only:
// `hub_sign_in` and `space_browser` are pure contract twins with no renderer and no transport,
// `hub_connection` composes them with the shell's own `DirectoryTransport` and builds the retained
// workspace tree.
pub use semio_framework_os_renderer_wgpu_hub_contracts::{hub_sign_in, space_browser};
pub use semio_framework_os_renderer_wgpu_hub_workspace::hub_connection;
//#endregion 🔐️HubElements

pub use semio_framework_os_renderer_wgpu_hub_workspace::canvas_presence;

extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;

extern crate semio_framework_os_renderer_wgpu_media_slots as media_slots;

extern crate semio_framework_os_renderer_wgpu_program_bridge as program_bridge;

//#region 🏠️🧳️PluginHostConfig
// 🐛️ Lives at the crate root, not inside `program_bridge` above (see that module's own `PluginHostConfig`
// region for why) — this is the file's real directory, so the 3-`..` climb to
// `framework/plugin/registry/generated/🖥️hosts/🦀️.rs` actually resolves.
pub use plugin_catalog::{install_plugin_catalog_json_v1, install_plugin_catalog_v1};
//#endregion 🏠️🧳️PluginHostConfig

extern crate semio_framework_os_renderer_wgpu_canvas2d_paint as canvas2d_paint;

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_native_platform::native_temporal;

// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-kernel-loop): the real multi-shard `Kernel`
// loop — `ParallelRuntime` — used by both `kernel_runtime` (below) and `scale_bench`. Native-only,
// same reason `kernel_runtime`/`scale_bench` themselves are: native guest execution and the shared
// native worker pool are not available on wasm32.
#[cfg(not(target_arch = "wasm32"))]
pub use semio_framework_os_renderer_wgpu_kernel_protocol::parallel_runtime;

extern crate semio_framework_os_renderer_wgpu_asset_authority as asset_authority;
pub(crate) use asset_authority::*;

pub(crate) use semio_framework_os_renderer_wgpu_asset_decode::*;

//#region 🔖️ScaleBench
#[cfg(not(target_arch = "wasm32"))]
pub use semio_framework_os_renderer_wgpu_kernel_runtime::scale_bench;
//#endregion 🔖️ScaleBench

//#region 🎠️KernelRuntime
#[cfg(not(target_arch = "wasm32"))]
extern crate semio_framework_os_renderer_wgpu_kernel_runtime as kernel_runtime;
//#endregion 🎠️KernelRuntime

//#region 🧭️BootAxisParityTests
//#endregion 🧭️BootAxisParityTests

//#region 🔖️OsHostDecomposition
extern crate semio_framework_os_renderer_wgpu_mailbox_core as runtime_mailbox_core;
// 🔌️ The DUPLEX door — the second browser door, beside the request/response mailbox. Mounted before
// `directory_door` because that door's `open_ws` is one of its two consumers (the other is the MCP
// agent bridge). See its module doc for why a socket cannot ride the mailbox.
pub use semio_framework_os_renderer_wgpu_doors::socket_door;
// 📇️ The ONE directory transport seam the shell's identity/command/Space-Administration lanes are
// parameterized over — native `ureq` here, the page `fetch` door there. See its module doc.
pub use semio_framework_os_renderer_wgpu_doors::directory_door;
// ♿️ The native shell's platform accessibility bridge: every window's accessibility projection as one AccessKit tree
// (NSAccessibility / UI Automation / AT-SPI), AccessKit kept behind this one module. The browser build's DOM mirror
// (`♿️accessibility-mirror`) is its twin.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use semio_framework_os_renderer_wgpu_native_platform::native_accessibility;
//#endregion 🔖️OsHostDecomposition

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_program_bridge::native_runtime_modules;

//#region 🧭️HostBaseTestHosts
// 🧪️ Test hosts of laws that name the monolith (shell source text, entry point, scenes) and therefore cannot live in the crate that owns the code they test.

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "🧪️test-hosts/🔬️renderer-io/🦀️.rs"]
mod renderer_io_retained_host;

//#endregion 🧭️HostBaseTestHosts
