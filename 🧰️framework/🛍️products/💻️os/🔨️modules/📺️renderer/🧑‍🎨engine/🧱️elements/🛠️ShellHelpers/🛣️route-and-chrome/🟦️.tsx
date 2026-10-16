/** 🛣️ Shell routes, app breadcrumbs, framework layout seeds, window engagement, plugin install bands, panel-tab nodes and history/command refusal notices.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHelpers}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { TIME_TRAVEL_CODE_LABELS } from "@semio-tech/framework/time-travel";
// #region 🔌️Adapters
import {
  type PluginModuleAcquisitionProgress,
  type ActionDescriptor,
  type AppDefinition,
  type AppPanelTabDefinition,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
  FRAMEWORK_PANEL_TAB_HISTORY_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
  type PanelTabKind,
  panelTabKindId,
  type PluginAppLabelsOverlay,
  type PluginCatalog,
  type PluginUiRefreshRequest,
  type UiRefreshSectionKey,
  UI_REFRESH_SECTIONS,
  resolvePluginHostConfig,
  type Severity,
  SHELL_LOCALES,
  SHELL_TERMINOLOGIES,
  isShellTerminology,
  type UiPatch,
  type WindowEngagement,
  type WindowEngagementControl,
  type WindowLayout,
  type WindowLayoutAxisNode,
  type WindowLayoutStackNode,
  type WindowLayoutWindowNode,
  type WindowMeasure,
  type WindowStackCorner
} from "@semio-tech/framework";
import { type CommandRejectionCodeV1, type CommandRejectionV1 } from "@semio-tech/framework/wire-contracts";
import { decodeWorldProjectionTemplateId } from "@semio-tech/infinite-world-r3f";
import {
  type Anchor,
  createEvenWindowLayout,
  type EngagementControl,
  type EngagementSpec,
  Icon,
  type IconName,
  type PanelTabNode,
  type SearchSpec,
  singleTreeLeaf,
  staticTreePanelDefinition,
  UI_TERMINOLOGY_NATIVE,
  uiDataLabel,
  type UiTranslationKey,
  type WindowLayoutNode
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import React, { type ReactNode } from "react";
import { faultNotice, historyNotice } from "@semio-tech/framework";
import { registerPendingWorldProjection } from "../../🌐️World3dHost/🟦️.tsx";
import { type InputDraftDispositionV1 } from "../../🏛️ShellHost/🎯️input-ledger/🟦️.ts";
import { type ExtraWindowInstance, type LoadedProgramState } from "../../🐚️Shell/🟦️.tsx";
import { UiDocumentStore } from "../../📃️UiDocumentStore/🟦️.tsx";
import type { BrowserActorUiPatchVerdictV1 } from "@semio-tech/framework/browser-bundle/patch-handoff";
import {
  loadPluginModule,
  pluginLoadProgressAt,
  pluginLoadRemainingMs,
  PLUGIN_LOAD_IDLE_TIMEOUT_MS,
  retainedSectionValueV1,
  type PluginWasmHandle
} from "../../🔌️PluginRuntime/🟦️.tsx";
import { wireLabel } from "../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import {
  currentShellLocaleV1,
  type FrozenLabel,
  frozenLabelText,
  resolveAppLabel,
  resolveManifestLabel,
  resolvePanelTabLabel,
  shellLabel,
  shellTabIcon
} from "../🏷️labels/🟦️.tsx";
import { APP_BREADCRUMB_SEPARATOR } from "../🔩️plumbing/🟦️.ts";
import {
  type BrowserActorPanelHostV1,
  cachedActorTreePanelConfigV1,
  cachedBodyStoreTreePanelConfigV1,
  type PanelTreeConfigCacheV1,
  type TreeWindowHostV1
} from "../🌳️tree-windows/🟦️.tsx";
import { windowMeasuresChrome } from "../🪟️window-panes/🟦️.tsx";

//#region ShellHelpers
function isStudioMode(catalog: PluginCatalog, pluginFilter?: string): boolean {
  return pluginFilter !== undefined && resolvePluginHostConfig(catalog, pluginFilter) !== undefined;
}

export type ShellRoute = { readonly kind: "landing" } | { readonly kind: "space"; readonly spaceId: string; readonly instanceId?: string } | { readonly kind: "notFound"; readonly path: string };

/** 🧭️ Classifies shell history paths into landing, studio space, or unknown routes. */
export function parseShellRoute(path: string): ShellRoute {
  const normalized = (path.split("?")[0] ?? "/").trim() || "/";
  if (normalized === "/") return { kind: "landing" };
  const match = /^\/spaces\/([^/]+)(?:\/instances\/([^/]+))?$/.exec(normalized);
  if (match) return { kind: "space", spaceId: match[1]!, instanceId: match[2] };
  return { kind: "notFound", path: normalized };
}

/**
 * 🗺️ Joins a breadcrumb for display.
 *
 * ⚠️ Accepts `undefined`: `AppDefinition.breadcrumb` is DECLARED OPTIONAL in the manifest, so every
 * unguarded `breadcrumb.join(…)` was a latent crash — and not a cosmetic one. This runs inside
 * `FrameworkOsShellInner`'s render, so a single app whose manifest omits the field took down the
 * WHOLE shell, and in a multi-pane host (the demonstrator) every pane died with it. An app with no
 * breadcrumb should render a nameless title, never destroy its host.
 */
export function appBreadcrumb(breadcrumb: readonly string[] | undefined): string {
  return (breadcrumb ?? []).join(APP_BREADCRUMB_SEPARATOR);
}

/** 🗺️ Resolves the breadcrumb effective under the active terminology; unknown/native ids fall back to `app.breadcrumb`, and an app that declares none at all to the empty breadcrumb (the field is optional — see {@link appBreadcrumb}). */
export function resolveAppBreadcrumb(app: Pick<AppDefinition, "breadcrumb" | "terminologyBreadcrumbs">, terminology: string): readonly string[] {
  return app.terminologyBreadcrumbs?.[terminology] ?? app.breadcrumb ?? [];
}

/** 🗺️ Resolves the breadcrumb for a non-active app (studio spawn palette/spawned entries) by looking up its `AppDefinition` across loaded plugins; falls back to the raw breadcrumb when the app can't be found. */
export function resolveArtifactByAppId(loadedPlugins: readonly LoadedProgramState[], appId: string, breadcrumb: readonly string[], terminology: string): readonly string[] {
  for (const program of loadedPlugins) {
    const app = program.manifest.apps.find((candidate) => candidate.id === appId);
    if (app) return resolveAppBreadcrumb(app, terminology);
  }
  return breadcrumb;
}

export function appWindowLabel(app: Pick<AppDefinition, "label" | "breadcrumb" | "terminologyBreadcrumbs">, terminology: string, windowLabel: string, locale: string = SHELL_LOCALES[0]): string {
  const trimmed = windowLabel.trim();
  if (trimmed) return trimmed;
  const override = app.terminologyBreadcrumbs?.[terminology];
  return override?.[override.length - 1]?.trim() || resolveManifestLabel(app.label, terminology, locale).trim();
}

/** 🧭️ Default anchor a plugin-declared panel-tab `group` docks into — groups only ever map to the four corners; the four edge-middle anchors start empty and are user-populated via drag-and-drop or a dock skeleton override. */
export function panelAnchorForGroup(group: string): Anchor {
  if (group === "workbench" || group === "document") return "top-left";
  if (group === "details") return "top-right";
  if (group === "display") return "bottom-left";
  if (group === "settings") return "bottom-right";
  return "top-right";
}

const FRAMEWORK_SETTINGS_BRANCH_ID = "framework.settings";

/** ⚙️ Nests every app-declared Settings-group panel tab under the framework Settings branch so the
 * bottom-right anchor exposes one Settings toggle instead of a duplicate beside it. */
export function integrateAppSettingsPanelTabsIntoFrameworkBranch(frameworkSettingsTab: PanelTabNode, appSettingsTabs: readonly PanelTabNode[]): PanelTabNode {
  if (appSettingsTabs.length === 0) return frameworkSettingsTab;
  if (frameworkSettingsTab.kind !== "branch" || frameworkSettingsTab.id !== FRAMEWORK_SETTINGS_BRANCH_ID) return frameworkSettingsTab;
  const appChildren = appSettingsTabs.map((tab, index) => ({ ...tab, order: tab.order ?? index }));
  const frameworkChildren = frameworkSettingsTab.children.map((tab, index) => ({ ...tab, order: (tab.order ?? index) + appSettingsTabs.length }));
  return { ...frameworkSettingsTab, children: [...appChildren, ...frameworkChildren] };
}

/**
 * 🕰️ Splits the framework History tab off a list of app panel tabs. `framework.panel.history` is injected into EVERY
 * app in the Settings group (`🔌️plugin/🦀️.rs` `AppBuilder::build_definition`) and its body is the guest's own
 * `framework.body.history`, rendered through the interpreter like every other panel body; the dock keeps it a
 * bottom-right leaf beside Settings, Marketplace and the Task manager instead of nesting it inside the Settings branch,
 * exactly as the wgpu shell's dock assembly partitions it.
 */
export function partitionFrameworkHistoryPanelTab<Tab extends { readonly kind: PanelTabKind }>(tabs: readonly Tab[]): { readonly history: Tab | undefined; readonly rest: readonly Tab[] } {
  return { history: tabs.find((tab) => panelTabKindId(tab.kind) === FRAMEWORK_PANEL_TAB_HISTORY_ID), rest: tabs.filter((tab) => panelTabKindId(tab.kind) !== FRAMEWORK_PANEL_TAB_HISTORY_ID) };
}

/** 🪟️ One leaf in a framework layout tree, with optional instance/template binding for multi-pane world views. */
type FrameworkLayoutWindowSeed = {
  readonly windowId: string;
  readonly windowKindId: string;
  readonly title?: string;
  readonly templateId?: string;
  readonly size: number;
  readonly corner?: WindowStackCorner;
};

/** 🪟️ Walks a framework layout and collects every window leaf, preferring `instanceId` as the live pane id. */
function collectFrameworkLayoutWindowSeeds(node: WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode, parentSize = 100): FrameworkLayoutWindowSeed[] {
  if (node.kind === "window") {
    return [
      {
        windowId: node.instanceId ?? node.windowKindId,
        windowKindId: node.windowKindId,
        title: node.title,
        templateId: node.templateId,
        size: parentSize,
        corner: node.corner,
      },
    ];
  }
  if (node.kind === "stack") {
    const size = node.size ?? parentSize;
    return node.children.map((child) => ({
      windowId: child.instanceId ?? child.windowKindId,
      windowKindId: child.windowKindId,
      title: child.title,
      templateId: child.templateId,
      size,
      corner: child.corner,
    }));
  }
  const childSizes = node.children.map((child) => ("size" in child ? child.size : undefined));
  const explicitTotal = childSizes.reduce<number>((sum, size) => sum + (size ?? 0), 0);
  const unsetCount = childSizes.filter((size) => size === undefined).length;
  const defaultEach = unsetCount > 0 ? Math.max(0, 100 - explicitTotal) / unsetCount : 0;
  return node.children.flatMap((child, index) => {
    const fraction = childSizes[index] ?? defaultEach;
    return collectFrameworkLayoutWindowSeeds(child, parentSize * (fraction / 100));
  });
}

/** 🗣️ For a single, non-instanced window (`instanceId` unset — the common case, one window per kind),
 * the `windowKind`'s own label is the single source of truth: a manifest-baked `WindowLayoutWindowNode.title`
 * (from a plugin's `create_default_layout(..., titles)` call) is a plain, locale-invariant string that
 * predates locale/terminology resolution entirely, so it must never win over a real `windowKinds` lookup —
 * it only survives as a last-resort fallback for a window kind id that isn't declared in the manifest
 * (mirrors {@link retitleWindowLayoutNode}'s already-correct precedence). For a multi-instance window
 * (`instanceId` set — several views sharing one `windowKind`, e.g. "Top"/"Perspective" both backed by a
 * single 3D-viewport kind), the shared kind label can't distinguish instances, so the baked per-instance
 * title is the real title and must win instead. */
function resolveFrameworkWindowTitle(
  windowKindId: string,
  instanceId: string | undefined,
  bakedTitle: string | undefined,
  windowKinds: readonly { readonly id: string; readonly label: unknown }[],
  terminology: string,
  locale: string,
): string {
  if (instanceId) return bakedTitle ?? windowKindId;
  const kind = windowKinds.find((entry) => entry.id === windowKindId);
  return kind ? resolveManifestLabel(kind.label, terminology, locale) : (bakedTitle ?? windowKindId);
}

function convertFrameworkLayoutNodeToModeLayout(
  node: WindowLayoutAxisNode | WindowLayoutStackNode,
  appLabelsOverlay: PluginAppLabelsOverlay,
  windowKinds: readonly { readonly id: string; readonly label: unknown }[],
  terminology: string,
  locale: string,
): Exclude<WindowLayoutNode, { kind: "window" }> {
  if (node.kind === "stack") {
    return {
      kind: "stack",
      size: node.size,
      children: node.children.map((child) => {
        const id = child.instanceId ?? child.windowKindId;
        const title = resolveFrameworkWindowTitle(child.windowKindId, child.instanceId, child.title, windowKinds, terminology, locale);
        return {
          kind: "window" as const,
          id,
          title: wireLabel(resolveAppLabel(appLabelsOverlay, "windowKind", id, title)),
          corner: child.corner,
        };
      }),
    };
  }
  return {
    kind: node.kind,
    size: node.size,
    children: node.children.map((child) => convertFrameworkLayoutNodeToModeLayout(child, appLabelsOverlay, windowKinds, terminology, locale)),
  };
}

/** 🏷️ Localizes singleton window titles while retaining authored instance names and the live layout. */
export function retitleWindowLayoutNode(
  node: WindowLayoutNode,
  windowKinds: readonly { readonly id: string; readonly label: unknown }[],
  extraInstances: readonly ExtraWindowInstance[],
  terminology: string,
  locale: string,
  titleOverrides: Readonly<Record<string, string>>,
): WindowLayoutNode {
  if (node.kind === "window") {
    const extra = extraInstances.find((entry) => entry.id === node.id);
    const kind = windowKinds.find((entry) => entry.id === node.id);
    const override=titleOverrides[node.id];
    const title = override !== undefined ? wireLabel(override) : extra ? wireLabel(extra.title) : kind ? wireLabel(resolveManifestLabel(kind.label, terminology, locale)) : (node.title ?? uiDataLabel(node.id));
    return { ...node, title };
  }
  return {
    ...node,
    children: node.children.map((child) => retitleWindowLayoutNode(child, windowKinds, extraInstances, terminology, locale, titleOverrides)),
  } as WindowLayoutNode;
}

/** 🪟️ The window instances a framework layout DECLARES beyond one per kind — every leaf whose instance id differs
 * from its window kind id, for a kind the app declares — as `{ id, windowKindId }`, in layout order, labels untouched.
 * The identity half of {@link resolveFrameworkLayoutSeed}'s `extraInstances`, for a program whose windows the shell
 * namespaces itself (a spawned program inside `s`). */
export function frameworkLayoutDeclaredInstances(layout: WindowLayout | undefined, windowKinds: readonly { readonly id: string }[]): readonly { readonly id: string; readonly windowKindId: string }[] {
  if (!layout?.root) return [];
  const kindIds = new Set(windowKinds.map((kind) => kind.id));
  return collectFrameworkLayoutWindowSeeds(layout.root)
    .filter((seed) => seed.windowId !== seed.windowKindId && kindIds.has(seed.windowKindId))
    .map((seed) => ({ id: seed.windowId, windowKindId: seed.windowKindId }));
}

/** 🪟️ Resolves a framework layout into the live mode tree, extra instances, and pending projection templates without inferring window focus (no side effects). */
export function resolveFrameworkLayoutSeed(
  layout: WindowLayout | undefined,
  windowKinds: readonly { readonly id: string; readonly label: unknown }[],
  appLabelsOverlay: PluginAppLabelsOverlay,
  terminology: string,
  locale: string,
): {
  readonly modeLayout: WindowLayoutNode;
  readonly extraInstances: readonly ExtraWindowInstance[];
  readonly pendingProjections: readonly { readonly windowId: string; readonly templateId: string }[];
} {
  const windowIds = windowKinds.map((kind) => kind.id);
  if (!layout?.root) {
    return {
      modeLayout: createEvenWindowLayout(windowIds.length ? windowIds : ["main"]),
      extraInstances: [],
      pendingProjections: [],
    };
  }
  const seeds = collectFrameworkLayoutWindowSeeds(layout.root);
  const kindById = new Map(windowKinds.map((kind) => [kind.id, kind] as const));
  const extraInstances: ExtraWindowInstance[] = [];
  const pendingProjections: { readonly windowId: string; readonly templateId: string }[] = [];
  for (const seed of seeds) {
    const kind = kindById.get(seed.windowKindId);
    if (!kind) continue;
    if (seed.windowId !== seed.windowKindId) {
      extraInstances.push({
        id: seed.windowId,
        windowKindId: seed.windowKindId,
        title: resolveAppLabel(appLabelsOverlay, "windowKind", seed.windowId, seed.title ?? resolveManifestLabel(kind.label, terminology, locale)),
      });
    }
    if (seed.templateId) pendingProjections.push({ windowId: seed.windowId, templateId: seed.templateId });
  }
  return {
    modeLayout: convertFrameworkLayoutNodeToModeLayout(layout.root, appLabelsOverlay, windowKinds, terminology, locale),
    extraInstances,
    pendingProjections,
  };
}

/** 🪟️ Applies a resolved framework layout seed: registers one-shot world projections, then returns the live layout payload. */
export function applyFrameworkLayoutSeed(
  layout: WindowLayout | undefined,
  windowKinds: readonly { readonly id: string; readonly label: unknown }[],
  appLabelsOverlay: PluginAppLabelsOverlay,
  terminology: string,
  locale: string,
): {
  readonly modeLayout: WindowLayoutNode;
  readonly extraInstances: readonly ExtraWindowInstance[];
} {
  const seed = resolveFrameworkLayoutSeed(layout, windowKinds, appLabelsOverlay, terminology, locale);
  for (const pending of seed.pendingProjections) {
    const projectionSpec = decodeWorldProjectionTemplateId(pending.templateId);
    if (projectionSpec) registerPendingWorldProjection(pending.windowId, projectionSpec);
  }
  return { modeLayout: seed.modeLayout, extraInstances: seed.extraInstances };
}

function modeLayoutNodeToFramework(node: WindowLayoutNode, kindByInstanceId: ReadonlyMap<string, string>): WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode {
  if (node.kind === "window") {
    const windowKindId = kindByInstanceId.get(node.id) ?? node.id;
    const instanceId = kindByInstanceId.has(node.id) ? node.id : undefined;
    return {
      kind: "window",
      windowKindId,
      ...(node.title ? { title: node.title } : {}),
      ...(instanceId ? { instanceId } : {}),
    };
  }
  if (node.kind === "stack") {
    return {
      kind: "stack",
      ...(node.size !== undefined ? { size: node.size } : {}),
      children: node.children.map((child) => {
        const windowKindId = kindByInstanceId.get(child.id) ?? child.id;
        const instanceId = kindByInstanceId.has(child.id) ? child.id : undefined;
        return {
          kind: "window" as const,
          windowKindId,
          ...(child.title ? { title: child.title } : {}),
          ...(instanceId ? { instanceId } : {}),
        };
      }),
    };
  }
  return {
    kind: node.kind,
    ...(node.size !== undefined ? { size: node.size } : {}),
    children: node.children.map((child) => modeLayoutNodeToFramework(child, kindByInstanceId) as WindowLayoutStackNode | WindowLayoutAxisNode),
  };
}

export function captureCurrentFrameworkLayout(shellLayout: WindowLayoutNode | null, extraWindowInstances: readonly ExtraWindowInstance[], fallback?: WindowLayout): WindowLayout | undefined {
  if (!shellLayout) return fallback;
  const kindByInstanceId = new Map(extraWindowInstances.map((entry) => [entry.id, entry.windowKindId] as const));
  const root = modeLayoutNodeToFramework(shellLayout, kindByInstanceId);
  if (root.kind === "window") return { root: { kind: "stack", children: [root] } };
  return { root };
}

//#region WindowLayoutChangeClassify
/** 🪟️ Trailing settle delay for `Mode.onLayoutChange` (fires continuously during a live drag/resize) before
 * noting one `shell.windowResize`/`shell.windowMove` command for the whole gesture — matches Board2dHost's
 * own camera-sync settle debounce (`beginCameraInteraction`), the only precedent for this kind of
 * drag-settle pattern already in this file. */
export const LAYOUT_CHANGE_SETTLE_MS = 350;

/** 🪟️ Recursive skeleton of a {@link WindowLayoutNode} — kind/id/nesting only, stripping `size` (resize) and
 * a stack's `activeId` (mere focus echo) — so two trees compare equal here iff neither differs. */
type WindowLayoutSkeletonNode = { readonly kind: string; readonly id?: string; readonly corner?: string; readonly children?: readonly WindowLayoutSkeletonNode[] };
function windowLayoutSkeleton(node: WindowLayoutNode): WindowLayoutSkeletonNode {
  if (node.kind === "window") return { kind: node.kind, id: node.id, corner: node.corner };
  return { kind: node.kind, children: node.children.map((child) => windowLayoutSkeleton(child as WindowLayoutNode)) };
}

/** 🪟️ Like {@link windowLayoutSkeleton} but keeps each node's `size` (still ignores a stack's `activeId`) —
 * comparing two of these (after their plain skeletons already matched) is how {@link classifyWindowLayoutChange}
 * tells a pure resize apart from no change at all. */
type WindowLayoutSizedSkeletonNode = { readonly kind: string; readonly id?: string; readonly size?: number; readonly children?: readonly WindowLayoutSizedSkeletonNode[] };
function windowLayoutSizedSkeleton(node: WindowLayoutNode): WindowLayoutSizedSkeletonNode {
  if (node.kind === "window") return { kind: node.kind, id: node.id, size: node.size };
  return { kind: node.kind, size: node.size, children: node.children.map((child) => windowLayoutSizedSkeleton(child as WindowLayoutNode)) };
}

/** 🪟️ Classifies a `Mode.onLayoutChange` delta by comparing the previous and next layout tree — `"rearrange"`
 * when window ids/nesting structure differ (drag-to-new-position, split, close), `"resize"` when only pane
 * sizes differ, `null` when neither differs (a pure active-window-flag echo, handled by the dedicated
 * active-window seam instead — never worth its own shell command). */
export function classifyWindowLayoutChange(previous: WindowLayoutNode | null, next: WindowLayoutNode | null): "resize" | "rearrange" | null {
  if (previous === next) return null;
  if (!previous || !next) return "rearrange";
  if (JSON.stringify(windowLayoutSkeleton(previous)) !== JSON.stringify(windowLayoutSkeleton(next))) return "rearrange";
  if (JSON.stringify(windowLayoutSizedSkeleton(previous)) !== JSON.stringify(windowLayoutSizedSkeleton(next))) return "resize";
  return null;
}
//#endregion WindowLayoutChangeClassify

function windowEngagementControlToSpec(control: WindowEngagementControl | undefined, onAction: (action: ActionDescriptor) => void): EngagementControl | undefined {
  if (!control) return undefined;
  if (control.kind === "ring" || control.kind === "toggleGroup") {
    return {
      kind: control.kind,
      id: control.id,
      label: control.label === undefined ? undefined : uiDataLabel(control.label),
      value: control.value,
      disabled: control.disabled,
      options: control.options.map((row) => ({ id: row.id, label: row.label, disabled: row.disabled })),
      onSelect: control.onSelect ? (id: string) => onAction({ ...control.onSelect!, args: { ...(control.onSelect!.args as object | undefined), id } }) : undefined,
    };
  }
  if (control.kind === "select") {
    return {
      kind: "select",
      id: control.id,
      label: control.label === undefined ? undefined : uiDataLabel(control.label),
      value: control.value,
      placeholder: control.placeholder === undefined ? undefined : uiDataLabel(control.placeholder),
      disabled: control.disabled,
      items: control.items.map((row) => ({ id: row.id, value: row.value, label: row.label })),
      onChange: control.onChange ? (value: string) => onAction({ ...control.onChange!, args: { ...(control.onChange!.args as object | undefined), value } }) : undefined,
    };
  }
  const dispatchNumeric = (action: ActionDescriptor | undefined, value: number) => {
    if (!action) return;
    onAction({ ...action, args: { ...(action.args as object | undefined), value } });
  };
  const numeric = {
    id: control.id,
    label: control.label === undefined ? undefined : uiDataLabel(control.label),
    value: control.value,
    min: control.min,
    max: control.max,
    step: control.step,
    unit: control.unit,
    disabled: control.disabled,
    onChange: control.onChange ? (value: number) => dispatchNumeric(control.onChange, value) : undefined,
    onCommit: control.onCommit ? (value: number) => dispatchNumeric(control.onCommit, value) : undefined,
  };
  if (control.kind === "slider") {
    if (control.min === undefined || control.max === undefined) throw new Error("engagement slider requires explicit minimum and maximum");
    return { ...numeric, kind: "slider", min: control.min, max: control.max };
  }
  return { ...numeric, kind: "stepper" };
}

/** 🔌️ Result of {@link installPlugin} — the boot effect must not infer success from
 * `loadedPluginsRef`, which only updates after the next React commit. */
export type PluginInstallOutcome = "loaded" | "already-loaded" | "in-flight" | "missing-registry" | "failed";

/** 🔐️ Keeps app-session ownership deterministic while dependency plugins stream concurrently. */
export function pluginShouldEstablishSession(pluginId: string, primaryPluginId: string | undefined, hasSession: boolean): boolean {
  return !hasSession && pluginId === primaryPluginId;
}

/** 🔁️ What one `PluginSource` availability event is worth: a first `install`, a `hot-swap` of the
 * artifact already loaded, or a `drop` because the event names nothing this shell does not already run. */
export type PluginAvailabilityRouteV1 = "install" | "hot-swap" | "drop";

/** 🔁️ The routing rule the availability pump obeys — the ONE place that decides whether an event may
 * destroy a live instance.
 *
 * A `PluginSource` streams AVAILABILITY, never commands: `subscribe` opens a fresh stream and the dev
 * source's SSE endpoint answers every connect with a full `snapshot` of what is already built, so the
 * same `rebuiltAt` arrives again on every reconnect. Routing a replay to `hot-swap` destroys the
 * session-owning plugin's live instance and its document (`actor-activation.revoked`, then `no channel
 * for instance N`) — measured in ticket 26/09/02 wave B38 §1.7 and diagnosed in wave B40, where a
 * language switch was re-running the subscription effect and thereby replaying the whole snapshot.
 *
 * `loadedRebuiltAt === undefined` is the first load's unbusted artifact (see `PluginSource.moduleUrl`):
 * it names no build, so any event that DOES name one is newer. An event carrying no `rebuiltAt` names no
 * newer artifact than whatever is loaded, so it can only ever be a first `install`. */
export function pluginAvailabilityRouteV1(alreadyLoaded: boolean, loadedRebuiltAt: number | undefined, eventRebuiltAt: number | undefined): PluginAvailabilityRouteV1 {
  if (!alreadyLoaded) return "install";
  if (eventRebuiltAt === undefined) return "drop";
  return loadedRebuiltAt === undefined || eventRebuiltAt > loadedRebuiltAt ? "hot-swap" : "drop";
}

/** 🧭️ Studio configures its catalogue; a focused shell configures only its active aggregate. */
export function pluginShouldReceiveContributions(pluginId: string, sessionPluginId: string, hostMode: boolean): boolean {
  return hostMode || pluginId === sessionPluginId;
}

/** 🔌️ Loads one plugin module under the idle-progress deadline, answering `null` for every refusal
 * rather than throwing — the shell's own per-plugin isolation.
 *
 * 🛑️ `signal` is the shell's install-band cancel. It is raced explicitly rather than only handed to
 * {@link loadPluginModule}, because the point of a cancel control is that the AWAIT settles now; the
 * underlying fetch finishes into the loader's own cache, so a re-request finds it warm. A cancelled
 * load is not a fault and is logged as such. */
export async function loadPluginModuleResilient(pluginId: string, moduleUrl: string, signal?: AbortSignal, manifestPluginId: string = pluginId): Promise<PluginWasmHandle | null> {
  const startedAtMs = Date.now();
  let timer = 0;
  try {
    if (signal?.aborted === true) throw new Error(`plugin-install.cancelled: ${pluginId}`);
    return await Promise.race([
      loadPluginModule(pluginId, moduleUrl, signal, manifestPluginId),
      new Promise<never>((_, reject) => {
        signal?.addEventListener("abort", () => reject(new Error(`plugin-install.cancelled: ${pluginId}`)), { once: true });
      }),
      new Promise<never>((_, reject) => {
        // ⏱️ Re-arms itself for whatever the idle rule still allows instead of firing once at a fixed
        // wall-clock offset, so an attempt that keeps reporting progress keeps its deadline moving.
        const arm = () => {
          const remaining = pluginLoadRemainingMs(startedAtMs, pluginLoadProgressAt(pluginId), Date.now());
          if (remaining > 0) {
            timer = window.setTimeout(arm, remaining);
            return;
          }
          reject(new Error(`timeout loading ${pluginId} after ${Date.now() - startedAtMs} ms with no progress for ${PLUGIN_LOAD_IDLE_TIMEOUT_MS} ms`));
        };
        arm();
      }),
    ]);
  } catch (error) {
    if (signal?.aborted === true) console.debug(`plugin install cancelled ${pluginId}`);
    else console.error("program load failed", pluginId, error);
    return null;
  } finally {
    if (timer) window.clearTimeout(timer);
  }
}

/** 🎬️ The install band's own copy — the React twin of the wgpu shell's `plugin.install.*` chrome
 * strings. No `ui.common.*` chrome key covers "loading a plugin", so this follows the same
 * local-resolution idiom as the rest of this region. */
const PLUGIN_INSTALL_BAND_LABEL: FrozenLabel = { en: "Loading plugin", de: "Plugin wird geladen" };
const PLUGIN_INSTALL_CANCEL_LABEL: FrozenLabel = { en: "Cancel", de: "Abbrechen" };
const PLUGIN_INSTALL_PROGRESS_LABEL: FrozenLabel = { en: "{completed} of {total} MB verified", de: "{completed} von {total} MB geprüft" };
const PLUGIN_INSTALL_RETRY_LABEL: FrozenLabel = { en: "the hub did not answer, trying again ({attempt} of {of})", de: "der Hub hat nicht geantwortet, neuer Versuch ({attempt} von {of})" };

/** 📈️ Every byte the in-flight installs have verified, out of every byte they must verify — `null`
 * while no install is downloading (a locally staged module has nothing to download). */
export function pluginInstallProgressTotalV1(progressById: Readonly<Record<string, PluginModuleAcquisitionProgress>>, pluginIds: readonly string[]): PluginModuleAcquisitionProgress | null {
  const rows = pluginIds.flatMap((pluginId) => (progressById[pluginId] ? [progressById[pluginId]!] : []));
  if (rows.length === 0) return null;
  const retry = rows.find((row) => row.retry !== undefined)?.retry;
  return { completedBytes: rows.reduce((sum, row) => sum + row.completedBytes, 0), totalBytes: rows.reduce((sum, row) => sum + row.totalBytes, 0), ...(retry === undefined ? {} : { retry }) };
}

/** 🎬️ "Loading plugin cad, beta · 12.3 of 81.2 MB verified" — one band for however many installs are
 * in flight, because they share one progress story and one cancel. */
export function pluginInstallBandTextV1(pluginIds: readonly string[], locale: string, progress: PluginModuleAcquisitionProgress | null = null): string {
  const band = `${frozenLabelText(PLUGIN_INSTALL_BAND_LABEL, locale)} ${[...pluginIds].sort().join(", ")}`;
  if (!progress || progress.totalBytes <= 0) return band;
  const megabytes = new Intl.NumberFormat(locale, { minimumFractionDigits: 1, maximumFractionDigits: 1 });
  const verified = frozenLabelText(PLUGIN_INSTALL_PROGRESS_LABEL, locale).replace("{completed}", megabytes.format(progress.completedBytes / 1_000_000)).replace("{total}", megabytes.format(progress.totalBytes / 1_000_000));
  if (progress.retry === undefined) return `${band} · ${verified}`;
  const retrying = frozenLabelText(PLUGIN_INSTALL_RETRY_LABEL, locale).replace("{attempt}", String(progress.retry.attempt)).replace("{of}", String(progress.retry.of));
  return `${band} · ${verified} · ${retrying}`;
}

/** 🔔️ The plugin module store's notices, in the person's language: a stored module that lost files is reinstalled; a full
 * store refuses an install. */
const PLUGIN_MODULE_STORE_NOTICE_LABEL: Readonly<Record<"reinstalling" | "quota-exceeded", FrozenLabel>> = {
  reinstalling: { en: "Plugin {plugin} was incomplete on this device and is being reinstalled.", de: "Plugin {plugin} war auf diesem Gerät unvollständig und wird neu installiert." },
  "quota-exceeded": { en: "Plugin {plugin} could not be installed: this device's storage is full.", de: "Plugin {plugin} konnte nicht installiert werden: Der Speicher dieses Geräts ist voll." },
};

export function pluginModuleStoreNoticeTextV1(notice: Readonly<{ kind: "reinstalling" | "quota-exceeded"; pluginId: string }>, locale: string): string {
  return frozenLabelText(PLUGIN_MODULE_STORE_NOTICE_LABEL[notice.kind], locale).replace("{plugin}", notice.pluginId);
}

export function pluginInstallCancelTextV1(locale: string): string {
  return frozenLabelText(PLUGIN_INSTALL_CANCEL_LABEL, locale);
}

function isViewportSurface(surfaceKind: string | undefined): boolean {
  return surfaceKind === "world-3d" || surfaceKind === "node-graph" || surfaceKind === "canvas-2d";
}

function defaultViewportEngagement(): WindowEngagement {
  return {
    sessionActive: true,
    status: [{ id: "framework.viewport.status", text: shellLabel("ui.engagement.viewport") }],
  };
}

export function resolveWindowEngagement(kind: Pick<AppDefinition["windowKinds"][number], "options">, windowId: string, byWindowId: Readonly<Record<string, WindowEngagement>>): WindowEngagement | undefined {
  const surfaceKind = (kind as { surfaceKind?: string }).surfaceKind;
  const declaredEngagement = kind.options.engagement.kind === "some" ? kind.options.engagement.value : undefined;
  return byWindowId[windowId] ?? declaredEngagement ?? (isViewportSurface(surfaceKind) ? defaultViewportEngagement() : undefined);
}

export function windowEngagementToSpec(engagement: WindowEngagement | undefined, onAction: (action: ActionDescriptor) => void): EngagementSpec | undefined {
  if (!engagement) return undefined;
  const options = engagement.options?.map((option) => ({
    id: option.id,
    label: option.label === undefined ? undefined : uiDataLabel(option.label),
    icon: option.iconId ? <Icon icon={option.iconId} size="small" /> : { kind: "text" as const, text: option.label ?? option.id },
    pressed: option.pressed,
    disabled: option.disabled,
    onPress: option.action ? () => onAction(option.action!) : undefined,
  }));
  const status = engagement.status?.map((row) => ({ id: row.id, content: row.text }));
  const control = windowEngagementControlToSpec(engagement.control, onAction);
  const controls = engagement.controls?.map((row) => windowEngagementControlToSpec(row, onAction)).filter((row): row is EngagementControl => row !== undefined);
  const hasContent = (options?.length ?? 0) > 0 || Boolean(control) || (controls?.length ?? 0) > 0 || (status?.length ?? 0) > 0;
  if (!hasContent) return undefined;
  return { sessionActive: engagement.sessionActive, options, control, controls, status };
}

/** 🔎️ Builds the top-middle window {@link SearchSpec} from the same Rust engagement payload: typed action input and autocomplete possibles. */
export function windowEngagementToSearchSpec(engagement: WindowEngagement | undefined, onAction: (action: ActionDescriptor) => void): SearchSpec | undefined {
  if (!engagement) return undefined;
  const input = engagement.input
    ? {
        id: engagement.input.id,
        value: engagement.input.value,
        placeholder: engagement.input.placeholder === undefined ? undefined : uiDataLabel(engagement.input.placeholder),
        disabled: engagement.input.disabled,
        onChange: engagement.input.onChange ? (value: string) => onAction({ ...engagement.input!.onChange!, args: { ...(engagement.input!.onChange!.args as object | undefined), value } }) : undefined,
        onSubmit: engagement.input.onSubmit ? (value: string) => onAction({ ...engagement.input!.onSubmit!, args: { ...(engagement.input!.onSubmit!.args as object | undefined), value } }) : undefined,
        onRepeatLast: engagement.input.onRepeatLast ? () => onAction(engagement.input!.onRepeatLast!) : undefined,
        onAbort: engagement.input.onAbort ? () => onAction(engagement.input!.onAbort!) : undefined,
      }
    : undefined;
  const possibles = engagement.possibleEngagements?.map((row) => ({
    id: row.id,
    label: row.label,
    detail: row.detail,
    onSelect: row.action ? () => onAction(row.action!) : undefined,
  }));
  const hasContent = Boolean(input) || (possibles?.length ?? 0) > 0;
  if (!hasContent) return undefined;
  return { sessionActive: engagement.sessionActive, input, possibles };
}

function panelTabIcon(tabId: string, group: string): React.FC<{ size?: number }> {
  // 🌱️ `group === "workbench"` already covers every host-app catalogue tab (each such app declares its
  // catalogue tab under `PanelGroup::Workbench` — see `s/plugin/rs`'s `App::builder(...).panel_tab(...)`)
  // so no separate app-specific tab-id literal is needed here.
  if (group === "workbench") return shellTabIcon(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID);
  if (tabId.includes("parameters")) return shellTabIcon(FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID);
  if (tabId.includes("inspector")) return shellTabIcon(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID);
  if (tabId === FRAMEWORK_PANEL_TAB_HISTORY_ID) return shellTabIcon("undo");
  return shellTabIcon(tabId);
}

/** 🌳️ Category-row icon: the first child's icon, or `fallback` when the category has no tabs yet. */
export function categoryTabIcon(tabs: readonly PanelTabNode[], fallback: IconName): React.FC<{ size?: number }> {
  const FirstIcon = tabs[0]?.icon;
  return function CategoryTabIcon({ size = 16 }: { size?: number }) {
    return FirstIcon ? <FirstIcon size={size} /> : <Icon icon={fallback} size="small" />;
  };
}

/** 🌳️ Depth-first leaves of a recursive panel-tab tree — the nodes that actually carry a `bodyKey`
 * to render. `PanelTabDefinition` (required `children: T[]`, no leaf variant) satisfies this
 * constraint directly; `PanelTabNode` (`PanelTabLeaf | PanelTabBranch`, `PanelTabLeaf` carrying no
 * `children` key at all) is a TS "weak type" mismatch against a constraint of only-optional
 * properties — callers with that shape use `flattenPanelTabNodes` (`ShellHost`'s own, built on
 * `panelTabChildren`'s union-aware accessor) instead of fighting the weak-type check here. */
export function flattenPanelTabLeaves<T extends { readonly children?: readonly T[] }>(tabs: readonly T[]): T[] {
  return tabs.flatMap((tab) => (tab.children && tab.children.length > 0 ? flattenPanelTabLeaves(tab.children) : [tab]));
}

/** 🌳️ Converts one plugin-declared {@link AppPanelTabDefinition} (recursively) into a {@link PanelTabNode}.
 *
 * 🪟️ `treeWindows`/`cache` are the windowed-tree plumbing: the tab's own `bodyKey` is what the guest
 * and `UiDirtyScope.panelBodies` name, and `cache` keeps a tab whose body did not change on exactly the
 * `TreePanelConfig` (and therefore the `UiDocumentStore` and the mounted `<Tree>`) it already had. */
export function panelTabDefinitionToNode(
  tab: AppPanelTabDefinition,
  group: string,
  panelBodyStoreByKey: Readonly<Record<string, UiDocumentStore>>,
  onAction: (action: ActionDescriptor) => void,
  order: number,
  appLabelsOverlay: PluginAppLabelsOverlay,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
  treeWindows: TreeWindowHostV1 | null = null,
  cache?: PanelTreeConfigCacheV1,
  actorPanels: BrowserActorPanelHostV1 | null = null,
): PanelTabNode {
  const tabId = panelTabKindId(tab.kind);
  const label = resolvePanelTabLabel(appLabelsOverlay, tabId, resolveManifestLabel(tab.label, terminology, locale));
  if (tab.children && tab.children.length > 0) {
    return {
      kind: "branch",
      id: tabId,
      icon: panelTabIcon(tabId, group),
      name: label,
      order,
      children: tab.children.map((child, childOrder) => panelTabDefinitionToNode(child, group, panelBodyStoreByKey, onAction, childOrder, appLabelsOverlay, terminology, locale, treeWindows, cache, actorPanels)),
    };
  }
  return singleTreeLeaf({
    id: tabId,
    icon: panelTabIcon(tabId, group),
    name: label,
    order,
    tree: staticTreePanelDefinition(
      actorPanels === null
        ? cachedBodyStoreTreePanelConfigV1(cache, tabId, panelBodyStoreByKey[tabId], tab.bodyKey ?? tabId, onAction, treeWindows)
        : cachedActorTreePanelConfigV1(cache, tabId, actorPanels, tab.bodyKey ?? tabId, treeWindows),
    ),
  });
}

/** 🧯️ Every history-edit refusal code the shell names, with its label key: the event log's and the hub's `history.*`
 * transition refusals (`🛢️db/🗿️artifact`, `🏪️store/🔄️sync`), a live session's `timeTravel.*` refusals
 * (`⏪️time-travel`), the plugin runtime's own preconditions of a `historyEdit*` verb and the driver faults it records on
 * the session (`🔌️plugin/⏪️time-travel`). Every text is byte-equal with the wgpu shell's (the shared band corpus).
 * @see ../../../../../../../../🔨️modules/🛢️db/🗿️artifact/🦀️.rs
 * @see ../../../../🔌️plugin/⏪️time-travel/🦀️.rs
 * @see ../../../../../../../../../../🔨️modules/⏪️time-travel/🟦️.ts */
type RefusalLabelId<Code extends string> = Code extends `timeTravel.${infer Name}` ? RefusalCamelCase<Name> : never;
type RefusalCamelCase<Name extends string> = Name extends `${infer Head}-${infer Tail}` ? `${Head}${Capitalize<RefusalCamelCase<Tail>>}` : Name;
type TimeTravelCode = typeof TIME_TRAVEL_CODE_LABELS[number][0];

const timeTravelRefusalLabelKeys = Object.fromEntries(TIME_TRAVEL_CODE_LABELS.map(([code]) => [code, `ui.timeTravel.refusal.${code.slice("timeTravel.".length).replace(/-([a-z])/gu, (_, letter: string) => letter.toUpperCase())}`])) as { readonly [Code in TimeTravelCode]: `ui.timeTravel.refusal.${RefusalLabelId<Code>}` };

export const HISTORY_REFUSAL_LABEL_KEYS = {
  "history.malformed-transition": "ui.history.refusal.malformedTransition",
  "history.unknown-target": "ui.history.refusal.unknownTarget",
  "history.transition-refused": "ui.history.refusal.transitionRefused",
  ...timeTravelRefusalLabelKeys,
} as const satisfies Readonly<Record<string, UiTranslationKey>>;

/** 🛟️ One code of {@link HISTORY_REFUSAL_LABEL_KEYS}. */
export type HistoryRefusalCodeV1 = keyof typeof HISTORY_REFUSAL_LABEL_KEYS;

/** 🔎️ `code` when it is a history-edit refusal the shell names, else `null`. */
export function historyRefusalCodeV1(code: unknown): HistoryRefusalCodeV1 | null {
  return typeof code === "string" && Object.hasOwn(HISTORY_REFUSAL_LABEL_KEYS, code) ? (code as HistoryRefusalCodeV1) : null;
}

/** 🔎️ The history-edit refusal a dispatch fault carries, as its own code or its first cause naming one. */
export function historyRefusalOfFaultV1(fault: { readonly code: string; readonly causes?: readonly { readonly code?: string }[] }): HistoryRefusalCodeV1 | null {
  return historyRefusalCodeV1(fault.code) ?? fault.causes?.map((cause) => historyRefusalCodeV1(cause.code)).find((code) => code !== null) ?? null;
}

/** 🧽️ A history admission refusal discards its submitted field draft; other failures keep recovery available. */
export function historyInputDraftDispositionV1(fault: { readonly code: string; readonly causes?: readonly { readonly code?: string }[] }): InputDraftDispositionV1 {
  return historyRefusalOfFaultV1(fault) === null ? "retain" : "discard";
}

/** 🔎️ The history-edit refusal a reserved verb answered with its silent `{rejected: <code>}` result, else `null`. */
export function historyRefusalOfOutputV1(output: unknown): HistoryRefusalCodeV1 | null {
  return typeof output === "object" && output !== null ? historyRefusalCodeV1((output as { readonly rejected?: unknown }).rejected) : null;
}

/** 🤫️ The history-edit refusals no shell tells the human (the band corpus' `refusals[].silent`): an event that names an
 * older session generation — a blur-commit arriving after Discard — is a silent no-op (design §4). Nothing the human did
 * is wrong and nothing is asked of them, so it earns no notice; its code stays in the dispatch trace. */
export const HISTORY_SILENT_REFUSALS_V1: ReadonlySet<HistoryRefusalCodeV1> = new Set<HistoryRefusalCodeV1>(["timeTravel.stale"]);

/** 🔇️ The silent history-edit refusal a dispatch fault carries ({@link HISTORY_SILENT_REFUSALS_V1}), else `null`. */
export function historySilentRefusalOfFaultV1(fault: { readonly code: string; readonly causes?: readonly { readonly code?: string }[] }): HistoryRefusalCodeV1 | null {
  const refusal = historyRefusalOfFaultV1(fault);
  return refusal !== null && HISTORY_SILENT_REFUSALS_V1.has(refusal) ? refusal : null;
}

/** 🔈️ The silent history-edit refusal a reserved verb's `{rejected: <code>}` result names, else `null`. */
export function historySilentRefusalOfOutputV1(output: unknown): HistoryRefusalCodeV1 | null {
  const refusal = historyRefusalOfOutputV1(output);
  return refusal !== null && HISTORY_SILENT_REFUSALS_V1.has(refusal) ? refusal : null;
}

/** 📣️ One transient notice: its localized text, severity and the machine code `data-notice-code` publishes. */
export type ShellNoticeV1 = { readonly text: string; readonly kind: Severity; readonly code: string };

/** 🛟️ The notice for a history-edit refusal: a `history.*` transition refusal is an error, a session refusal keeps the
 * severity its fault reported and is a warning when it arrives without one (a verb's silent `{rejected}` result). */
export function historyRefusalNoticeV1(code: HistoryRefusalCodeV1, severity: Severity = "warning"): ShellNoticeV1 {
  return { text: String(shellLabel(HISTORY_REFUSAL_LABEL_KEYS[code])), kind: code.startsWith("history.") ? "error" : severity, code };
}

/** 📢️ The notice of a history-lane refusal a fault carries — its own code or a cause's — in the shell's language: the kernel's
 * own en/de text (`historyNotice`: an open tool transaction, an exhausted or still replaying history, a document still
 * loading), `{n}` the program's `editCount` (`HistoryPatch.editCount`, carried as data — never read from a message). A
 * warning: waiting, finishing or cancelling resolves it. `null` for any other fault. */
export function historyLaneNoticeV1(fault: { readonly code?: string; readonly causes?: readonly { readonly code?: string }[] }, editCount: number): ShellNoticeV1 | null {
  const code = [fault.code, ...(fault.causes ?? []).map((cause) => cause.code)].find((candidate): candidate is string => candidate !== undefined && historyNotice(candidate) !== undefined);
  const notice = code === undefined ? undefined : historyNotice(code);
  return code === undefined || notice === undefined ? null : { text: notice[currentShellLocaleV1()].replace("{n}", String(editCount)), kind: "warning", code };
}

/** 🔕️ The notice a refused dispatch earns: a history-edit refusal ({@link historyRefusalNoticeV1}, keeping the fault's
 * severity) or a history-lane refusal ({@link historyLaneNoticeV1}); `null` for a silent refusal
 * ({@link HISTORY_SILENT_REFUSALS_V1}) and for any other fault — never a raw code. */
export function historyFaultNoticeV1(fault: { readonly code: string; readonly severity?: Severity; readonly causes?: readonly { readonly code?: string }[] }, editCount: number): ShellNoticeV1 | null {
  const refusal = historyRefusalOfFaultV1(fault);
  if (refusal !== null) return HISTORY_SILENT_REFUSALS_V1.has(refusal) ? null : historyRefusalNoticeV1(refusal, fault.severity ?? "warning");
  return historyLaneNoticeV1(fault, editCount);
}

/** 📣️ The notice of a refused dispatch an app declared (design §20.12): the kernel's `faultNotice` over the framework table and the
 * refusing app's published `faultNotices`, in the shell's language and `terminology`, `{name}` filled from `fault.params` only, told
 * with the fault's own severity. `null` for a code no table declares — the shell then shows its generic refusal, never a raw code. */
export function appFaultNoticeV1(fault: { readonly code: string; readonly severity?: Severity; readonly causes?: readonly { readonly code?: string }[]; readonly params?: Readonly<Record<string, string>> }, app: Pick<AppDefinition, "faultNotices"> | undefined, terminology: string): ShellNoticeV1 | null {
  const notice = faultNotice(fault, app?.faultNotices ?? [], isShellTerminology(terminology) ? terminology : SHELL_TERMINOLOGIES[0], currentShellLocaleV1());
  return notice === null ? null : { text: notice.text, kind: fault.severity ?? "error", code: notice.code };
}

/** 🔊️ The notice of a reserved verb's silent `{rejected: <code>}` result: a history-edit or a history-lane refusal, else `null` —
 * and `null` for a silent refusal ({@link HISTORY_SILENT_REFUSALS_V1}). */
export function historyOutputNoticeV1(output: unknown, editCount: number): ShellNoticeV1 | null {
  const refusal = historyRefusalOfOutputV1(output);
  if (refusal !== null) return HISTORY_SILENT_REFUSALS_V1.has(refusal) ? null : historyRefusalNoticeV1(refusal);
  const rejected = typeof output === "object" && output !== null ? (output as { readonly rejected?: unknown }).rejected : undefined;
  return typeof rejected === "string" ? historyLaneNoticeV1({ code: rejected }, editCount) : null;
}

/** 🚫️ The line and severity each local refusal (`CommandRejectionV1`'s `local.*` codes) is told with: a refusal the human
 * can wait out or cannot act on here is a warning, one that means this device produced a change it cannot send is an error. */
export const LOCAL_COMMAND_REJECTION_NOTICES_V1 = {
  "local.read-only": { key: "ui.conflict.local.readOnly", kind: "warning" },
  "local.queue-full": { key: "ui.conflict.local.queueFull", kind: "warning" },
  "local.backbone-capacity": { key: "ui.conflict.local.queueFull", kind: "warning" },
  "local.backbone-duplicate": { key: "ui.conflict.local.duplicate", kind: "warning" },
  "local.backbone-pair-unavailable": { key: "ui.conflict.local.notReady", kind: "warning" },
  "local.backbone-scope-mismatch": { key: "ui.conflict.local.foreignDocument", kind: "error" },
  "local.backbone-malformed": { key: "ui.conflict.local.unreadable", kind: "error" },
  "local.socket-frame-ceiling": { key: "ui.conflict.local.tooLarge", kind: "error" },
} as const satisfies Readonly<Record<Exclude<CommandRejectionCodeV1, `hub.${string}`>, { readonly key: UiTranslationKey; readonly kind: Severity }>>;

/** ⚔️ The notice for one refused command batch, read from the one typed rejection contract — total, whatever the producer.
 * A local refusal is told by its code ({@link LOCAL_COMMAND_REJECTION_NOTICES_V1}) under that code. A hub refusal is read
 * from the `MutationMessage` codes it was graded with: a batch refused for a history transition names that refusal
 * ({@link historyRefusalNoticeV1}); otherwise the hub's outcome step graded a region another human's concurrent command
 * also touched as `mutation.clamped` or a structural constraint that command now holds as `mutation.invariant`
 * (`🛢️db/🗿️artifact` `grade_conflict_record`, frozen code set), named after `ui.conflict.hubRejected`; a refusal naming
 * neither (admission, transport, unreadable diagnostics) says only that the hub refused the change. No producer's English
 * `reason` ever reaches the human (ticket 26/09/23 C10, audit G-P2-3; 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING follow-up 3).
 * @see ../../../../🏪️store/🔄️sync/🧬️schema/🔣️command-rejection/🔣️.json
 * @see ../../../../../../../../🔨️modules/🛢️db/🗿️artifact/🦀️.rs */
export function commandRejectionNoticeV1(rejection: CommandRejectionV1): ShellNoticeV1 {
  if (rejection.code !== "hub.refused" && rejection.code !== "hub.unreadable") {
    const notice = LOCAL_COMMAND_REJECTION_NOTICES_V1[rejection.code];
    return { text: String(shellLabel(notice.key)), kind: notice.kind, code: rejection.code };
  }
  const codes = rejection.messages.map((message) => message.code);
  const history = codes.map(historyRefusalCodeV1).find((code) => code !== null);
  if (history !== undefined && history !== null) return historyRefusalNoticeV1(history);
  const reason = codes.includes("mutation.invariant") ? "ui.conflict.hubConcurrentInvariant" : codes.includes("mutation.clamped") ? "ui.conflict.hubConcurrentEdit" : null;
  const rejected = String(shellLabel("ui.conflict.hubRejected"));
  return { text: reason === null ? rejected : `${rejected}: ${shellLabel(reason)}`, kind: "warning", code: "sync.command.rejected" };
}

export function browserActorPanelKeysV1(app: Pick<AppDefinition, "panelTabs">): ReadonlySet<string> {
  return new Set(flattenPanelTabLeaves(app.panelTabs).flatMap((tab) => (tab.bodyKey ? [panelTabKindId(tab.kind)] : [])));
}

export type BrowserActorUiStoresV1 = Readonly<{ windows: Map<string, UiDocumentStore>; panels: Map<string, UiDocumentStore>; sections: Map<string, UiDocumentStore> }>;

/** 🧩️ The reserved refresh-section surfaces an actor renders beside its windows and panels, each keyed by its reserved body
 * key (`framework.section.*`) — the local refresh's own section binding (`UI_REFRESH_SECTIONS`). */
export const BROWSER_ACTOR_SECTION_KEYS: ReadonlySet<string> = new Set(UI_REFRESH_SECTIONS.map(({ bodyKey }) => bodyKey));

/** 🩹️ Applies one browser-actor patch offer surface by surface and answers one verdict per patch, in offer order
 * — the guest acknowledges and resends per surface (`patch-ack` / `patch-rejected`), so one stale panel never
 * costs a window its frame. Every window kind of the app, every bodied panel and every reserved refresh section is a
 * surface: an actor-bound document renders all of its windows and their chrome from its one actor. A patch for a
 * surface this app does not render is refused `unknown-surface`; a patch that does not apply resets its store to the
 * empty document (see `UiDocumentStore.reset`), which is what the guest's full resend assumes. The FIRST offer of an
 * opening must paint a window, or nothing is retained and every surface is refused `window-surface-unpainted` so the
 * guest resends them all. `sectionsChanged` says an acknowledged patch moved a section the shell must re-dispatch. */
export function applyBrowserActorUiPatchesV1(
  patches: readonly UiPatch[],
  windowKeys: ReadonlySet<string>,
  panelKeys: ReadonlySet<string>,
  retained: BrowserActorUiStoresV1 | null,
): Readonly<{ verdicts: readonly BrowserActorUiPatchVerdictV1[]; stores: BrowserActorUiStoresV1 | null; surfacesAdded: boolean; sectionsChanged: boolean }> {
  const windows = retained?.windows ?? new Map<string, UiDocumentStore>(),
    panels = retained?.panels ?? new Map<string, UiDocumentStore>(),
    sections = retained?.sections ?? new Map<string, UiDocumentStore>();
  let surfacesAdded = false;
  const verdicts = patches.map((patch): BrowserActorUiPatchVerdictV1 => {
    const owner = windowKeys.has(patch.surface) ? windows : panelKeys.has(patch.surface) ? panels : BROWSER_ACTOR_SECTION_KEYS.has(patch.surface) ? sections : null;
    if (owner === null) return { surface: patch.surface, outcome: "rejected", revision: 0, reason: "unknown-surface" };
    let store = owner.get(patch.surface);
    if (store === undefined) {
      store = new UiDocumentStore(patch.surface);
      owner.set(patch.surface, store);
      surfacesAdded = true;
    }
    const applied = store.applyPatch(patch);
    if (applied.ok) return { surface: patch.surface, outcome: "acknowledged", revision: store.getRevisionSnapshot() };
    store.reset();
    return { surface: patch.surface, outcome: "rejected", revision: 0, reason: applied.rejection.type };
  });
  if (retained !== null || verdicts.some((verdict) => windowKeys.has(verdict.surface) && verdict.outcome === "acknowledged"))
    return { verdicts, stores: { windows, panels, sections }, surfacesAdded, sectionsChanged: verdicts.some((verdict) => BROWSER_ACTOR_SECTION_KEYS.has(verdict.surface) && verdict.outcome === "acknowledged") };
  return {
    verdicts: verdicts.map((verdict): BrowserActorUiPatchVerdictV1 => (verdict.outcome === "acknowledged" ? { surface: verdict.surface, outcome: "rejected", revision: 0, reason: "window-surface-unpainted" } : verdict)),
    stores: null,
    surfacesAdded: false,
    sectionsChanged: false,
  };
}

/** 🧩️ The reserved refresh sections an actor-bound document's section stores hold now, keyed like the local refresh's
 * cache (`engagements`/`measures`/`tools`/`catalogue`), each with the store revision it was read at; a section whose store
 * has no root is absent. Decoded by the local refresh's own `retainedSectionValueV1`, so both paths read one carrier. */
export function browserActorSectionValuesV1(sections: ReadonlyMap<string, UiDocumentStore>, producer: string): ReadonlyMap<UiRefreshSectionKey, Readonly<{ revision: number; value: unknown }>> {
  const values = new Map<UiRefreshSectionKey, Readonly<{ revision: number; value: unknown }>>();
  for (const section of UI_REFRESH_SECTIONS) {
    const store = sections.get(section.bodyKey);
    if (store === undefined) continue;
    const value = retainedSectionValueV1(section.bodyKey, store.getState(), producer);
    if (value !== undefined) values.set(section.key, { revision: store.getRevisionSnapshot(), value });
  }
  return values;
}

/** 🧩️ `request` without its reserved sections — for a session an actor serves, whose engagements, measures, tool measures
 * and catalogue come from the actor's section surfaces (its local instance never sees the hub document); `null` when
 * nothing is left to fetch. */
export function withoutUiRefreshSectionsV1(request: PluginUiRefreshRequest | null): PluginUiRefreshRequest | null {
  if (request === null) return null;
  const stripped: PluginUiRefreshRequest = { ...request, engagements: undefined, measures: undefined, tools: undefined, catalogue: undefined };
  return (stripped.windows ?? []).length === 0 && (stripped.panels ?? []).length === 0 && stripped.labels === undefined ? null : stripped;
}

export function resolveCanvasBodyKey(app: AppDefinition): string {
  const windowKind = app.windowKinds[0];
  if (!windowKind) return "main";
  if (windowKind.bodyKey.includes("composite")) {
    const workflow = app.windowKinds.find((kind) => kind.bodyKey.includes("workflow"));
    return workflow?.bodyKey ?? windowKind.bodyKey;
  }
  return windowKind.bodyKey;
}

/** 💬️ Builds spawned-window engagement, search, measures, and utility-options chrome for one window instance. */
export function spawnedWindowChromeForKind(
  kind: Pick<AppDefinition["windowKinds"][number], "options">,
  windowId: string,
  engagementsByWindowId: Readonly<Record<string, WindowEngagement>>,
  measuresByWindowId: Readonly<Record<string, readonly WindowMeasure[]>>,
  activeUtilityId: string | undefined,
  onAction: (action: ActionDescriptor) => void,
): { readonly engagement?: EngagementSpec; readonly search?: SearchSpec; readonly measures: ReactNode; readonly utilityOptions: ReactNode } {
  const { measures, utilityOptions } = windowMeasuresChrome(measuresByWindowId[windowId] ?? kind.options.measures, activeUtilityId, windowId, onAction);
  const resolvedEngagement = resolveWindowEngagement(kind, windowId, engagementsByWindowId);
  return {
    engagement: windowEngagementToSpec(resolvedEngagement, onAction),
    search: windowEngagementToSearchSpec(resolvedEngagement, onAction),
    measures,
    utilityOptions,
  };
}
//#endregion ShellHelpers
