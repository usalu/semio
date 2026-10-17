import type { PluginCatalogRowsV1 } from "@semio-tech/framework/plugin-catalog";
import { resolveFrameworkOsCatalogConfigurationV1 } from "./🧩️catalog/🟦️.ts";

// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🐚️Shell/component.tsx
/** 🐚️ `🐚️Shell` — the pre-mount boot entry point `bootFrameworkOs` and its option and execution types.
 * The rest of the former core shell module lives in the child folders beside this file: boot locks and
 * defaults (`🔒️boot-and-locks`), program sessions, peer interaction, the `ShellState` slice shapes, the
 * closed `ShellAction` union, the slice reducers and root store (`🕹️shell-store`, the `useReducer` store
 * that replaced ~38 independent `useState` calls). It also keeps the `ShellFaultBoundary` scoped React error
 * boundary: its importers load this module and, through it, the composition root that registers the hosts. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import React, { Component, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import { Button, bootstrapElementsSurfaceChromeDocument, type UiLabel } from "@semio-tech/ui-react";
import { createBrowserStoragePort, createMemoryStoragePort, SemioFaultError, type AppRole, type Fault, type ShellBrand } from "@semio-tech/framework";
// 🧱️core: shellLabel imported directly from ShellHelpers (its real implementation, not via the barrel) —
// this module calls shellLabel(...) at module top level (UI_INSPECTOR_MIXED_PLACEHOLDER), which requires
// a non-circular import; routing through the barrel indirection (cleared) hit the same
// module-top-level circular-import initialization-order bug documented in ui-react's
// 🧱️elements/🫀️core/Ports/🟦️.tsx header comment (see 📋️w0-status.md's "W3 follow-up" section).
import { shellLabel } from "../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { FrameworkOsShell } from "../🏛️ShellHost/🟦️.tsx";
import { readUiPreferences } from "../🎚️UiPreferences/🟦️.ts";
import { clearDurableShellStorage, type FrameworkOsDefaults, type FrameworkOsLocks, isEphemeralShellBrand, mergeShellLockSources, resolveBootAppRole, resolveShellDefaults, resolveShellLocks } from "./🔒️boot-and-locks/🟦️.ts";

// #endregion 🔌️Adapters
//#region 🔖️types
export type { SpacePanelState, SpawnedAppEntry } from "../🛠️ShellHelpers/📌️panel/🟦️.ts";

/** 🎬️ Owns host service and worker construction capabilities for one React mount. */
export type FrameworkOsBootExecution = {
  readonly backboneWorkerFactory?: () => Worker;
  readonly documentServices?: readonly import("../🏛️ShellHost/🪪️host-bootstrap/🟦️.tsx").InstalledServicePresentationV1[];
  readonly hostPorts?: import("../🏛️ShellHost/🪝️host-ports/🟦️.ts").ShellHostPortsV1;
};

export type FrameworkOsBootOptions = {
  readonly catalog: PluginCatalogRowsV1;
  readonly surfaceSessionFactories?: readonly import("../🪪️WasmSessionLoader/🟦️.tsx").AppSurfaceSessionFactory[];
  readonly rootId?: string;
  readonly plugin?: string;
  readonly plugins?: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
  readonly appId?: string;
  /** 👁️✏️ Boot-time surface role preference (contract freeze §5) — defaults to `"editor"` via
   * {@link resolveBootAppRole} when omitted, never inferred from `appId`. */
  readonly appRole?: AppRole;
  readonly locks?: FrameworkOsLocks;
  readonly defaults?: FrameworkOsDefaults;
  readonly brand?: ShellBrand;
};

//#endregion 🔖️types
//#region Boot
export async function bootFrameworkOs(options: FrameworkOsBootOptions, execution: FrameworkOsBootExecution = {}): Promise<{ dispose(): void }> {
  const composition = resolveFrameworkOsCatalogConfigurationV1(execution.hostPorts?.catalogAdmission, options.catalog, options.plugin || undefined, { maxBytes: 2097152, maxRows: 512, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => false, progress: () => {} });
  const root = document.getElementById(options.rootId ?? "root");
  if (!root) throw new Error("missing #root");
  const locks = resolveShellLocks(mergeShellLockSources(options.brand?.locks, options.locks));
  const defaults = resolveShellDefaults(options.brand, options.defaults);
  const ephemeral = isEphemeralShellBrand(options.brand);
  if (ephemeral) clearDurableShellStorage();
  if (options.brand) document.title = options.brand.windowTitle;
  // 🐚️ This pre-paint bootstrap runs before any `ShellScope` exists (React hasn't mounted yet), so it
  // resolves storage the same way `resolveShellScopeStorage` will once the shell below actually mounts.
  const storage = ephemeral ? createMemoryStoragePort() : createBrowserStoragePort();
  bootstrapElementsSurfaceChromeDocument(locks.appearance ?? readUiPreferences(storage).appearance ?? "system");
  // 🐢️ No hardcoded fallback app — an omitted `plugins` list boots the shell with an explicit
  // "no plugins available" state rather than silently picking one app.
  const appRole = resolveBootAppRole(options.appRole);
  const mounted = createRoot(root);
  mounted.render(<FrameworkOsShell catalog={composition.catalog} backboneWorkerFactory={execution.backboneWorkerFactory} documentServices={execution.documentServices} hostPorts={execution.hostPorts} pluginFilter={options.plugin} plugins={options.plugins ?? composition.boot.plugins} surfaceSessionFactories={options.surfaceSessionFactories} appId={options.appId} appRole={appRole} locks={locks} defaults={defaults} brand={options.brand} ownsPage />);
  return { dispose: () => mounted.unmount() };
}
//#endregion Boot
//#region ErrorBoundary
type ShellFaultBoundaryProps = {
  readonly boundaryId: string;
  readonly fallbackLabel: UiLabel;
  readonly children: ReactNode;
  readonly onFault?: (fault: Fault | null, error: Error, boundaryId: string) => void;
  readonly retry?: () => void;
};

type ShellFaultBoundaryState = { readonly hasFault: boolean; readonly message: string; readonly fault: Fault | null };

/** 🧯️ Scoped React fault boundary — logs `[TRACE] shell fault` and renders a localized fallback with optional retry. */
export class ShellFaultBoundary extends Component<ShellFaultBoundaryProps, ShellFaultBoundaryState> {
  constructor(props: ShellFaultBoundaryProps) {
    super(props);
    this.state = { hasFault: false, message: "", fault: null };
  }

  static getDerivedStateFromError(error: Error): ShellFaultBoundaryState {
    const fault = error instanceof SemioFaultError ? error.fault : null;
    return { hasFault: true, message: fault?.message ?? error.message, fault };
  }

  componentDidCatch(error: Error, info: React.ErrorInfo) {
    this.props.onFault?.(error instanceof SemioFaultError ? error.fault : null, error, this.props.boundaryId);
  }

  private readonly retry = () => {
    this.setState({ hasFault: false, message: "", fault: null });
    this.props.retry?.();
  };

  render() {
    if (this.state.hasFault) {
      return (
        <div className="p-double" role="alert" data-shell-fault-boundary={this.props.boundaryId}>
          <p className="text-sm text-destructive">
            {this.props.fallbackLabel}: {this.state.message}
          </p>
          {this.props.retry ? <Button className="mt-single" icon="rotate-ccw" text={shellLabel("ui.common.retry")} onClick={this.retry} /> : null}
        </div>
      );
    }
    return this.props.children;
  }
}
//#endregion ErrorBoundary
