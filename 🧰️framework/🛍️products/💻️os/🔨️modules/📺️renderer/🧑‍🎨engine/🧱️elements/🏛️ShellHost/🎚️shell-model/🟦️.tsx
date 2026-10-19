/** 🎚️ Shell contexts, catalog probe and service view types, transient-notice constants, the mutation-code label key, the window fault status component and the app keybinding context hooks.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import React, { createContext } from "react";
import { AppRouter, type Severity } from "@semio-tech/framework";
import { dockSeedActiveWindowIdV1, modeLayoutStacksV1, type WindowScopeLayoutNodeV1 } from "../⌨️window-scope/🟦️.ts";
import { type IconName, type UiTranslationKey, uiDataLabel, type WindowLayoutNode } from "@semio-tech/ui-react";
import { HISTORY_REFUSAL_LABEL_KEYS, historyRefusalCodeV1 } from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { shellLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { type WindowFault, type WindowFaultClass } from "../🩺️fault/🟦️.ts";


//#region FrameworkOsShell
/** 🏷️ Lets a per-window host rewrite its Mode window title (e.g. live projection label). */
export const SetWindowTitleContext = createContext<((windowId: string, title: string) => void) | null>(null);

/** 🖼️ Lets a per-window host rewrite its Mode window icon (e.g. live projection glyph). */
export const SetWindowIconContext = createContext<((windowId: string, iconId: IconName) => void) | null>(null);

/** 🔬️ Dev-only `window.__semioOsCatalogProbe` payload — see `#region 🔖️CatalogSmokeProbe`; schema
 * report fixture `🧑‍💻dev/🧫️fixtures/🔬️catalog-smoke.json`. */
export type ShellCatalogProbe = {
  readonly shellPluginId: string;
  readonly ready: boolean;
  /** 🧯️ `routerFault` carries the `surface.*` fault that excluded this plugin from `AppRouter`
   * (ticket 26/09/05/S-END-TO-END lane H) — the plugin installed fine, so `status` alone would read
   * healthy while none of its surfaces route. */
  readonly plugins: readonly { readonly pluginId: string; readonly status: string; readonly routerFault?: { readonly code: string; readonly message: string } }[];
  readonly programs: readonly { readonly pluginId: string; readonly appId: string; readonly label: string }[];
  readonly spawned: readonly { readonly id: string; readonly pluginId: string; readonly appId: string }[];
};

/** 🔬️ One probe row per installed registry entry: its install status, plus the `surface.*` fault
 * that kept it out of {@link AppRouter} when there is one. Pure and total — a plugin excluded from
 * routing is reported here, never dropped from the list, so a catalog smoke sees the difference
 * between "not installed", "installed and routing" and "installed but unroutable". */
export function shellCatalogProbePlugins(
  registry: readonly { readonly pluginId: string }[],
  pluginStatusById: Readonly<Record<string, string | undefined>>,
  router: AppRouter,
): ShellCatalogProbe["plugins"] {
  return registry.map((entry) => {
    const routerFault = router.faultFor(entry.pluginId);
    return { pluginId: entry.pluginId, status: pluginStatusById[entry.pluginId] ?? "available", ...(routerFault ? { routerFault: { code: routerFault.code, message: routerFault.message } } : {}) };
  });
}

/** ⚖️ `TransientNotice.kind` tone per `Severity` (contract freeze `26/08/16/MUTATION-OUTCOMES-
 * MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C1) — `info`/`warning` share the neutral popover
 * chrome (matches the pre-existing "not error" branch), `error`/`fatal` both render destructive,
 * `fatal` additionally bolded so a rejected dispatch's worst level reads as more severe than a
 * plain `error`. */
export const TRANSIENT_NOTICE_TONE_CLASS: Record<Severity, string> = {
  info: "border-border bg-popover text-popover-foreground",
  warning: "border-warning bg-warning/10 text-warning",
  error: "border-destructive bg-destructive text-destructive-foreground",
  fatal: "border-destructive bg-destructive text-destructive-foreground font-semibold",
};

/** ⏰️ Browser timer duration shared with the native Shell chrome deadline contract. */
export const SHELL_TRANSIENT_NOTICE_AUTO_DISMISS_MS = 4000;

/** ⏰️ Arms the actual ShellHost notice dismissal timer. */
export function scheduleShellTransientNoticeDismissV1(dismiss: () => void): ReturnType<typeof setTimeout> {
  return setTimeout(dismiss, SHELL_TRANSIENT_NOTICE_AUTO_DISMISS_MS);
}

/** ⚖️ `Fault.code` for a rejected local dispatch (contract freeze §C8/§C9) — never invented locally,
 * mirrors the Rust guest's `Fault("app.command.rejected")`. */
export const COMMAND_REJECTED_FAULT_CODE = "app.command.rejected";

/** 🎫️ Minimum spacing between two claims of a development session from `/_semio/dev/local-session`: a lapsed or refused
 * local session is re-claimed from the local hub owner's broker at most this often, so a hub that keeps refusing can
 * never drive a claim loop. */
export const LOCAL_SESSION_RECLAIM_INTERVAL_MS = 30_000;

/** ⚖️ Maps one of the frozen `mutation.*` codes (contract freeze §C2, extended 2026-09-30 by the two state-dependent
 * Error codes `mutation.target-referenced` and `mutation.target-mismatch` — no per-plugin codes, ever) onto its
 * `ui.mutation.code.*` label key, and the event log's own `history.foreign-transition` refusal (kernel-db
 * `FOREIGN_HISTORY_TRANSITION_CODE`) onto `ui.mutation.history.foreignTransition`, every history-edit refusal onto its
 * {@link HISTORY_REFUSAL_LABEL_KEYS} key; an unrecognized code falls back to the
 * generic rejected-title key rather than fabricating a key the schema doesn't have. */
export function mutationCodeLabelKey(code: string): UiTranslationKey {
  switch (code) {
    case "mutation.target-missing":
      return "ui.mutation.code.targetMissing";
    case "mutation.target-referenced":
      return "ui.mutation.code.targetReferenced";
    case "mutation.target-mismatch":
      return "ui.mutation.code.targetMismatch";
    case "mutation.no-op":
      return "ui.mutation.code.noOp";
    case "mutation.partial":
      return "ui.mutation.code.partial";
    case "mutation.clamped":
      return "ui.mutation.code.clamped";
    case "mutation.precondition-drifted":
      return "ui.mutation.code.preconditionDrifted";
    case "mutation.duplicate-id":
      return "ui.mutation.code.duplicateId";
    case "mutation.invariant":
      return "ui.mutation.code.invariant";
    case "mutation.inverse-refused":
      return "ui.mutation.code.inverseRefused";
    case "mutation.cascade":
      return "ui.mutation.code.cascade";
    case "history.foreign-transition":
      return "ui.mutation.history.foreignTransition";
    default: {
      if (code.startsWith("mutation.apply.")) return "ui.mutation.code.apply";
      const refusal = historyRefusalCodeV1(code);
      return refusal === null ? "ui.mutation.rejected.title" : HISTORY_REFUSAL_LABEL_KEYS[refusal];
    }
  }
}

/** 🩺️ `WindowFaultClass` → its `ui.windowFault.*` label key. Total by construction, so a new class
 * cannot silently render as a blank body. */
const WINDOW_FAULT_LABEL_KEYS: Readonly<Record<WindowFaultClass, UiTranslationKey>> = {
  "abi-mismatch": "ui.windowFault.abiMismatch",
  "interactive-ceiling": "ui.windowFault.interactiveCeiling",
  clock: "ui.windowFault.clock",
  "plugin-internal": "ui.windowFault.pluginInternal",
  "install-failed": "ui.windowFault.installFailed",
  unknown: "ui.windowFault.unknown",
};

/** 🩺️ The accessible bilingual status a window body shows instead of staying silently empty when
 * its plugin instance faulted. `data-semio-window-fault` carries the machine-readable class so a
 * catalog smoke can name the cause straight off the DOM; the raw `Fault.code` and the plugin's own
 * message stay visible underneath for a human reading the window. */
export function WindowFaultStatus({ fault }: { readonly fault: WindowFault }): React.ReactElement {
  return (
    <div className="flex h-full min-h-0 w-full flex-col gap-single p-double text-sm" role="status" aria-live="polite" data-semio-window-fault={fault.class} data-semio-window-fault-code={fault.code ?? ""} data-semio-window-fault-origin={fault.origin ?? ""}>
      <p className="font-semibold text-destructive">{shellLabel("ui.windowFault.title")}</p>
      <p className="text-muted-foreground">{shellLabel(WINDOW_FAULT_LABEL_KEYS[fault.class])}</p>
      <p className="font-mono text-xs text-muted-foreground">{uiDataLabel(fault.code ? `${fault.code}: ${fault.message}` : fault.message)}</p>
    </div>
  );
}

/** 🪟️ The window a freshly seeded mode layout opens ACTIVE. Every layout seed below routes through
 * this instead of dispatching `null`: a mode with windows always has exactly one active window, so a
 * window-scoped verb, its chord and its Actions rail have an owner before the user's first click —
 * the wgpu shell has always seeded `active_window_id` this way, the React shell never did
 * (`⌨️window-scope/🟦️.ts`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function seededActiveWindowId(modeLayout: WindowLayoutNode): string | null {
  return dockSeedActiveWindowIdV1(modeLayoutStacksV1(modeLayout as WindowScopeLayoutNodeV1), null);
}
//#endregion FrameworkOsShell
