/** 📋️ The window engagement and panel tab types, the app definition types, the program hot swap event, the UI refresh request and response and the context menu request and response. */
import type { TutorialDefinition } from "../../🎓️onboarding/🟦️.ts";
import type { WindowLayout } from "../🗺️window-layout/🟦️.ts";
import type { IconName } from "@semio-tech/assets";
import type { ContextMenuItemSpec } from "../../🖱️ui/🎬️scene/📋️context-menu/🟦️.ts";
import type { Effect } from "@semio-tech/framework-kernel/effect";
import type { ActionDefinition as GeneratedActionDefinition, UtilityDefinition as GeneratedUtilityDefinition, ToolDefinition as GeneratedToolDefinition, CommandDefinition as GeneratedCommandDefinition, WindowMeasure as GeneratedWindowMeasure, WindowEngagementOption as GeneratedWindowEngagementOption, WindowEngagementInput as GeneratedWindowEngagementInput, WindowEngagementStatus as GeneratedWindowEngagementStatus, WindowEngagementPossible as GeneratedWindowEngagementPossible, WindowEngagementRingOption as GeneratedWindowEngagementRingOption, WindowEngagementToggleGroupOption as GeneratedWindowEngagementToggleGroupOption, WindowEngagementSelectItem as GeneratedWindowEngagementSelectItem, WindowEngagementControl as GeneratedWindowEngagementControl, WindowEngagement as GeneratedWindowEngagement, WindowEngagementSlot as GeneratedWindowEngagementSlot, WindowOptions as GeneratedWindowOptions, ActionRef as GeneratedActionRef, PanelGroup as GeneratedPanelGroup, PanelTabKind as GeneratedPanelTabKind, PanelTabDefinition as GeneratedPanelTabDefinition, ModeDefinition as GeneratedModeDefinition, WindowKindDefinition as GeneratedWindowKindDefinition, AppDefinition as GeneratedAppDefinition } from "../🤖️generated/🪪️manifest/🟦️.ts";
import type { MenuRef, NamedLayout } from "../🏷️ui-contract-and-canvas-types/🟦️.ts";
import type { PluginViewState } from "../👁️view-state-and-public-invocation/🟦️.ts";
import type { FaultNoticeDefinition } from "../🔁️host-effects-choices-and-fault-notices/🟦️.ts";

// #region 🛂️Manifest
//#region 🔌️PluginAndAppContract
//#region PluginRuntime
//#region AppManifestProtocol
/** 🧬️ Generated from Rust `WindowMeasure`/`WindowEngagement*` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type WindowMeasure = GeneratedWindowMeasure;
export type WindowEngagementOption = GeneratedWindowEngagementOption;
export type WindowEngagementInput = GeneratedWindowEngagementInput;
export type WindowEngagementStatus = GeneratedWindowEngagementStatus;
export type WindowEngagementPossible = GeneratedWindowEngagementPossible;
export type WindowEngagementRingOption = GeneratedWindowEngagementRingOption;
export type WindowEngagementToggleGroupOption = GeneratedWindowEngagementToggleGroupOption;
export type WindowEngagementSelectItem = GeneratedWindowEngagementSelectItem;
export type WindowEngagementControl = GeneratedWindowEngagementControl;
export type WindowEngagement = GeneratedWindowEngagement;

/** 🌳️ Mirrors Rust `PanelTabKind` — closes the informal `FRAMEWORK_CATEGORY_*`/`*_TAB_ID` string-constant convention: every panel tab is either a framework-predefined kind (exhaustively switchable) or an app-declared custom tab (`{ kind: "app", id }`). */
export type PanelTabKind = GeneratedPanelTabKind;
/** 🔤️ Flat string key for a `PanelTabKind` — mirrors Rust `PanelTabKind::id_str()`. Use for React `key=` props and legacy string-id matching. */
export function panelTabKindId(kind: PanelTabKind): string {
  switch (kind.kind) {
    case "workbenchCategory":
      return "framework.category.workbench";
    case "displayCategory":
      return "framework.category.display";
    case "detailsCategory":
      return "framework.category.details";
    case "settingsCategory":
      return "framework.category.settings";
    case "displayWindows":
      return "framework.display.windows";
    case "displayLayout":
      return "framework.display.layout";
    case "settingsGeneral":
      return "framework.settings.general";
    case "settingsTheme":
      return "framework.settings.theme";
    case "settingsDefaultApps":
      return "framework.settings.defaultApps";
    case "app":
      return kind.id;
  }
}

/** 🌳️ Mirrors Rust `PanelTabDefinition` — a leaf carries `bodyKey`, a branch carries `children`; `group` is only meaningful on root entries. */
export type AppPanelTabDefinition = GeneratedPanelTabDefinition;

/** 📦️ Mirrors Rust `AppDefinition` — generated 1:1 from `framework/core/rs/lib.rs` via the owned schema exporter, except
 * `defaultLayout`/`namedLayouts` which keep this file's narrower hand-refined `WindowLayout` (owned schema exporter
 * widens `WindowLayoutAxisNode.kind`/`WindowLayoutStackNode.kind` to plain `string` since the Rust
 * field is a runtime `String`, not an enum — the narrower `"row" | "column" | "stack" | "window"`
 * literal unions here are domain knowledge worth keeping for exhaustive switches). */
export type AppActionDefinition = Omit<GeneratedActionDefinition, "iconId"> & { readonly iconId?: IconName };
export type AppUtilityDefinition = Omit<GeneratedUtilityDefinition, "iconId"> & { readonly iconId: IconName };
export type AppToolDefinition = Omit<GeneratedToolDefinition, "iconId"> & { readonly iconId: IconName };
export type AppCommandDefinition = Omit<GeneratedCommandDefinition, "iconId"> & { readonly iconId?: IconName };
export type AppWindowKindDefinition = Omit<GeneratedWindowKindDefinition, "iconId"> & { readonly iconId: IconName };
export type AppDefinition = Omit<GeneratedAppDefinition, "defaultLayout" | "namedLayouts" | "iconId" | "tutorials" | "faultNotices"> & {
  /** 📢️ The app's published fault notices, each label the full terminology × locale matrix (design §20.12). */
  readonly faultNotices: readonly FaultNoticeDefinition[];
  readonly defaultLayout?: WindowLayout;
  readonly namedLayouts: readonly NamedLayout[];
  readonly iconId?: IconName;
  /** 🎬️ Brand-owned tutorials shown ALONGSIDE the app's own declared ones (never replacing them,
   * unlike `introduction`) — narrowed the same way `defaultLayout` is (see class doc above). */
  readonly tutorials: readonly TutorialDefinition[];
};
export type AppModeDefinition = GeneratedModeDefinition;
export type AppWindowOptions = GeneratedWindowOptions;
export type AppWindowEngagementSlot = GeneratedWindowEngagementSlot;
export type AppActionRef = GeneratedActionRef;
export type AppPanelGroup = GeneratedPanelGroup;

export type ProgramHotSwapEvent = {
  readonly pluginId: string;
  readonly version: string;
  readonly addedApps: readonly string[];
  readonly removedApps: readonly string[];
};

//#endregion AppManifestProtocol
//#region UiRefresh
/** 🐢️ One requested window/panel section — `bodyKey` only applies to windows/panels; `hash` is the host's known fnv1a-64 hex of that section's last payload, or absent on first fetch. */
export type PluginUiRefreshSectionRequest = { readonly key: string; readonly bodyKey?: string; readonly hash?: string };

/** 🐢️ One batched, hash-conditional refresh request — one round trip for the window/panel/engagements/measures/labels sections. Utility bars are no longer a plugin section: the renderer derives them from the utility registry via {@link deriveUtilityNodes}. */
export type PluginUiRefreshRequest = {
  readonly viewState: PluginViewState;
  readonly windows?: readonly PluginUiRefreshSectionRequest[];
  readonly panels?: readonly PluginUiRefreshSectionRequest[];
  readonly engagements?: { readonly hash?: string };
  readonly measures?: { readonly hash?: string };
  /** 🛠️ Mode-level tool measures, keyed by tool id — see `DocumentApp::tool_measures`. */
  readonly tools?: { readonly hash?: string };
  /** 🛍️ App-static operator/palette catalogue — see `ArtifactApp::app_catalogue`. Fetched once per app
   * instance; every later refresh sends the cached hash and gets no payload back. */
  readonly catalogue?: { readonly hash?: string };
  readonly labels?: { readonly hash?: string };
};

/** 🧩️ The four refresh sections that are NOT authored window/panel bodies. Each is its own
 * retained surface whose reserved body key names the plugin accessor the runtime calls in place of
 * `render` (`window_engagements`/`window_measures`/`tool_measures`/`app_catalogue`), so they publish,
 * re-publish and page through exactly the same `surface-visible` → mount → reconcile → patch law as a
 * window body. `catalogue` is app-STATIC and carries the whole registered operator catalogue once per
 * app instance, instead of riding on every node-graph scene payload.
 *
 * Mirrors the Rust `UiRefreshSection` / `UI_REFRESH_SECTION_KEYS` / `UI_REFRESH_SECTION_BODY_KEYS` in
 * `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`. Both sides are pinned against the one language-neutral
 * declaration in `🛂️manifest/🧪️tests/🔬️ui-refresh-section/🔣️.json`, so neither can drift. */
export type UiRefreshSectionKey = "engagements" | "measures" | "tools" | "catalogue";

export type UiRefreshSection = { readonly key: UiRefreshSectionKey; readonly bodyKey: string };

export const UI_REFRESH_SECTIONS: readonly UiRefreshSection[] = [
  { key: "engagements", bodyKey: "framework.section.engagements" },
  { key: "measures", bodyKey: "framework.section.measures" },
  { key: "tools", bodyKey: "framework.section.tools" },
  { key: "catalogue", bodyKey: "framework.section.catalogue" },
];

/** 🎯️ Projects host-owned context for a section surface — the FULL view state, unnarrowed:
 * `window_measures`/`window_engagements` iterate `windowInstances` and re-project each instance
 * themselves, and `tool_measures` keys off `activeToolId`, so narrowing here would blind all three. */
export function sectionViewContext(view: PluginViewState): PluginViewState {
  return { ...view };
}

/** 🐢️ `value` is present only when `hash` differs from what the request supplied — an unchanged section costs one hash compare instead of a full re-serialize. */
export type PluginUiRefreshSectionResponse = { readonly key: string; readonly hash: string; readonly value?: unknown };

export type PluginUiRefreshResponse = {
  readonly windows?: readonly PluginUiRefreshSectionResponse[];
  readonly panels?: readonly PluginUiRefreshSectionResponse[];
  readonly engagements?: PluginUiRefreshSectionResponse;
  readonly measures?: PluginUiRefreshSectionResponse;
  readonly tools?: PluginUiRefreshSectionResponse;
  readonly catalogue?: PluginUiRefreshSectionResponse;
  readonly labels?: PluginUiRefreshSectionResponse;
  /** ⏱️ See `DocumentApp::pending_effects` — background work (e.g. a `flowEvalTick` chain) the host
   * should dispatch right after this refresh, fed through the same `applyHostEffects` pass as an
   * action's own `requestedEffects`. */
  readonly requestedEffects?: readonly Effect[];
};

//#endregion UiRefresh
//#region 🖱️ContextMenu
/** 🖱️ Scene-target info for an on-demand context-menu request — hit-test results from the
 * surface's own picking (hover/selection), not cached across clicks. */
export type ContextMenuHit = {
  readonly domain: string;
  readonly id: string;
  readonly label?: string;
};

export type ContextMenuSelectionGroup = {
  readonly domain: string;
  readonly ids: readonly string[];
};

export type ContextMenuTextContext = {
  readonly caret: number;
  readonly hasSelection: boolean;
  readonly word?: string;
  readonly canRename: boolean;
  readonly hasCompletions: boolean;
};

export type PluginContextMenuSurfaceTarget = {
  readonly surfaceId: string;
  readonly kind: string;
  readonly hits?: readonly ContextMenuHit[];
  readonly selection?: readonly ContextMenuSelectionGroup[];
  readonly text?: ContextMenuTextContext;
};

export type PluginContextMenuPoint = { readonly x: number; readonly y: number };

/** 🖱️ On-demand context-menu request — never cached, never batched into {@link PluginUiRefreshRequest}.
 * `menu` is the {@link MenuRef} the host resolved from `data-menu-id`/a scene surface convention id
 * (`"world3d"`, `"nodeGraph"`, `"window"`, `"panel:<tabId>"`, ...). */
export type PluginContextMenuRequest = {
  readonly menu: MenuRef;
  readonly surface?: PluginContextMenuSurfaceTarget;
  readonly windowInstanceId?: string;
  readonly point?: PluginContextMenuPoint;
};

export type PluginContextMenuResponse = {
  readonly items: readonly ContextMenuItemSpec[];
};
//#endregion 🖱️ContextMenu
//#endregion PluginRuntime
//#endregion 🔌️PluginAndAppContract
// #endregion 🛂️Manifest
