// #region 🛂️Manifest
/// <reference types="vitest/importMeta" />
/** @emoji 🛂️ `@semio-tech/framework` — AppDefinition, PluginManifest, contributions, and declarative UI contract. */
import { PLAYGROUND_BUILD_TARGETS, type PlaygroundBuildTarget } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🟦️playgrounds.ts";
import { PLUGIN_BUILD_TARGETS, PLUGIN_HOST_CONFIGS, EXTENSION_TARGETS, pluginModuleUrl, extensionModuleUrl } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🟦️plugins.ts";
import type { IconName } from "@semio-tech/assets";
export type { IconName };
import { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology, type ShellLocale, type ShellTerminology, type LocalizedLabel } from "./🤖️generated/🟦️ui-axes.ts";
export { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology };
export type { ShellLocale, ShellTerminology, LocalizedLabel };

// #region 🧬️GeneratedMirror
/** 🧬️ Types generated from `framework/core/rs/lib.rs` via ts-rs (`bun nx run @semio-tech/framework:generate`); re-exported below alongside their hand-written neighbors so this stays the one import surface. */
import type {
  ActionDescriptor as GeneratedActionDescriptor,
  ActionKind as GeneratedActionKind,
  ActionDefinition as GeneratedActionDefinition,
  ActionArgDef as GeneratedActionArgDef,
  ActionArgControl as GeneratedActionArgControl,
  ActionArgOption as GeneratedActionArgOption,
  UtilityDefinition as GeneratedUtilityDefinition,
  UtilityRef as GeneratedUtilityRef,
  ToolDefinition as GeneratedToolDefinition,
  ToolRef as GeneratedToolRef,
  CommandScope as GeneratedCommandScope,
  CommandDefinition as GeneratedCommandDefinition,
  CommandRef as GeneratedCommandRef,
  WindowMeasure as GeneratedWindowMeasure,
  WindowEngagementOption as GeneratedWindowEngagementOption,
  WindowEngagementInput as GeneratedWindowEngagementInput,
  WindowEngagementStatus as GeneratedWindowEngagementStatus,
  WindowEngagementPossible as GeneratedWindowEngagementPossible,
  WindowEngagementRingOption as GeneratedWindowEngagementRingOption,
  WindowEngagementToggleGroupOption as GeneratedWindowEngagementToggleGroupOption,
  WindowEngagementSelectItem as GeneratedWindowEngagementSelectItem,
  WindowEngagementControl as GeneratedWindowEngagementControl,
  WindowEngagement as GeneratedWindowEngagement,
  WindowEngagementSlot as GeneratedWindowEngagementSlot,
  WindowOptions as GeneratedWindowOptions,
  ActionRef as GeneratedActionRef,
  PanelGroup as GeneratedPanelGroup,
  PanelTabKind as GeneratedPanelTabKind,
  PanelTabDefinition as GeneratedPanelTabDefinition,
  ModeDefinition as GeneratedModeDefinition,
  WindowKindDefinition as GeneratedWindowKindDefinition,
  AppDefinition as GeneratedAppDefinition,
  IntroductionDefinition as GeneratedIntroductionDefinition,
  IntroductionStepDefinition as GeneratedIntroductionStepDefinition,
  IntroductionPlacement as GeneratedIntroductionPlacement,
  IntroductionInteraction as GeneratedIntroductionInteraction,
  IntroductionInteractionKind as GeneratedIntroductionInteractionKind,
  IntroductionLogo as GeneratedIntroductionLogo,
  IntroductionPoint as GeneratedIntroductionPoint,
  IntroductionGesture as GeneratedIntroductionGesture,
  IntroductionKeyModifier as GeneratedIntroductionKeyModifier,
  IntroductionPointerButton as GeneratedIntroductionPointerButton,
  IntroductionCursor as GeneratedIntroductionCursor,
  IntroductionDemonstration as GeneratedIntroductionDemonstration,
  DialogDefinition as GeneratedDialogDefinition,
  UiPresence as GeneratedUiPresence,
  UiState as GeneratedUiState,
  UiStatus as GeneratedUiStatus,
} from "./🤖️generated/🟦️manifest.ts";
// #endregion 🧬️GeneratedMirror

export const CANVAS_HOVER_SOURCE_CANVAS = "canvas";
export const CANVAS_HOVER_SOURCE_PICK_MENU = "pick-menu";
export const CANVAS_HOVER_SOURCE_CATALOG = "catalog";
export const CANVAS_HOVER_SOURCE_DOCUMENT = "document";

export const FRAMEWORK_PANEL_TAB_DOCUMENT_ID = "framework.panel.document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_ID = "framework.panel.catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_ID = "framework.panel.inspection";
export const FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL = "Document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL = "Catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_LABEL = "Inspection";
export const FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID = "framework.panel.document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID = "framework.panel.catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID = "framework.panel.inspection";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_ID = "framework.panel.parameters";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL = "Parameters";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID = "framework.panel.parameters";
/** 🕰️ Mirrors Rust `FRAMEWORK_PANEL_TAB_HISTORY_ID` — auto-injected into every app's `panelTabs`. */
export const FRAMEWORK_PANEL_TAB_HISTORY_ID = "framework.panel.history";
export const FRAMEWORK_PANEL_TAB_HISTORY_LABEL = "History";
export const FRAMEWORK_PANEL_TAB_HISTORY_ICON_ID = "framework.panel.history";

export const UI_INSPECTOR_MIXED_PLACEHOLDER = "Mixed";


export type CanvasPickTarget = {
  readonly domain: string;
  readonly id: string;
  readonly generality: number;
  readonly label: string;
  readonly kind?: string;
};

export type CanvasPickRequest = {
  readonly targets: readonly CanvasPickTarget[];
  readonly client: { readonly x: number; readonly y: number };
  readonly modifiers?: Readonly<Record<string, boolean>>;
};

export type CanvasHoverFocus = {
  readonly sourceId: string;
  readonly target: CanvasPickTarget | null;
};

/** 🧬️ Generated from Rust `ActionDescriptor` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type ActionDescriptor = GeneratedActionDescriptor;

export type UiPresence = GeneratedUiPresence;
export type UiState = GeneratedUiState;
export type UiStatus = GeneratedUiStatus;



export type WindowLayoutWindowNode = {
  readonly kind: "window";
  readonly windowKindId: string;
  readonly title?: string;
  readonly instanceId?: string;
  readonly templateId?: string;
};

export type WindowLayoutStackNode = {
  readonly kind: "stack";
  readonly size?: number;
  readonly children: readonly WindowLayoutWindowNode[];
};

export type WindowLayoutAxisNode = {
  readonly kind: "row" | "column";
  readonly size?: number;
  readonly children: readonly (WindowLayoutAxisNode | WindowLayoutStackNode)[];
};

export type WindowLayout = {
  readonly root: WindowLayoutAxisNode | WindowLayoutStackNode;
};

export type NamedLayout = {
  readonly id: string;
  readonly label: string;
  readonly iconId?: IconName;
  readonly layout: WindowLayout;
  readonly origin: "builtin" | "user";
  readonly groupPath?: readonly string[];
};

export type UtilityCategory = "selection" | "utilities" | "history" | "sync";

export type UtilityLeaf =
  | { readonly id: string; readonly kind: "separator"; readonly order?: number; readonly disabled?: boolean }
  | {
      readonly id: string;
      readonly kind: "button";
      readonly iconId: IconName;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly controllerId?: string;
      readonly action?: string;
      readonly args?: unknown;
    }
  | {
      readonly id: string;
      readonly kind: "toggle";
      readonly iconId: IconName;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly pressed?: boolean;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly controllerId?: string;
      readonly action?: string;
      readonly args?: unknown;
    };

export type UtilityNode =
  | UtilityLeaf
  | {
      readonly id: string;
      readonly kind: "collection";
      readonly iconId: IconName;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly children: readonly UtilityNode[];
    }
  | {
      readonly id: string;
      readonly kind: "button";
      readonly iconId: IconName;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly onPress: ActionDescriptor;
    }
  | {
      readonly id: string;
      readonly kind: "toggle";
      readonly iconId: IconName;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly pressed?: boolean;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly onChange: ActionDescriptor;
    };

export type UiSectionNode = {
  readonly type: "section";
  readonly id: string;
  readonly label?: string;
  readonly defaultOpen?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly menu?: UiMenuRef;
  readonly children: readonly UiNode[];
};

/** @emoji 🌳️ One hover-revealed row action on a {@link UiTreeItemNode}; renderer-side addition on top of the base wasm tree-item shape. */
export type UiTreeActionPlacement = "row" | "menu";

export type UiTreeItemAction = {
  readonly iconId: IconName;
  readonly label?: string;
  readonly action: ActionDescriptor;
  readonly placement?: UiTreeActionPlacement;
};

export type UiTreeItemNode = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly icon?: string;
  readonly iconId?: IconName;
  readonly selected?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly defaultOpen?: boolean;
  readonly action?: ActionDescriptor;
  readonly hoverAction?: ActionDescriptor;
  readonly unhoverAction?: ActionDescriptor;
  readonly actions?: readonly UiTreeItemAction[];
  readonly draggable?: boolean;
  readonly dragData?: Readonly<Record<string, string>>;
  readonly items?: readonly UiTreeItemNode[];
  readonly control?: UiControlNode;
  readonly isHidden?: boolean;
  /** 🖱️ Row-level context-menu address — most rows share one `menu.id` across a tree with the row
   * id carried in `args` (e.g. `{ id: row.id }`), rather than minting a unique menu id per row. */
  readonly menu?: UiMenuRef;
};

export type UiTreeSectionNode = {
  readonly id: string;
  readonly label?: string;
  readonly defaultOpen?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly items: readonly UiTreeItemNode[];
};

export type UiTreeNode = {
  readonly type: "tree";
  readonly sections: readonly UiTreeSectionNode[];
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly selectedIds?: readonly string[];
  readonly highlightedIds?: readonly string[];
  readonly selectionChange?: ActionDescriptor;
  readonly dropAction?: ActionDescriptor;
  readonly menu?: UiMenuRef;  readonly presence?: UiPresence;
};

export type UiControlNode = UiInputNode | UiSelectNode | UiToggleNode | UiButtonNode | UiKeyValueNode | UiSliderNode | UiNumberStepperNode | UiRingNode | UiIconSelectNode;

export type UiInputNode = {
  readonly type: "input";
  readonly id: string;
  readonly inputKind: string;
  readonly value: string;
  readonly placeholder?: string;
  readonly commit?: string;
  readonly min?: number;
  readonly max?: number;
  readonly step?: number;
  readonly accept?: string;
  readonly onChange: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

export type UiSelectItem = {
  readonly value: string;
  readonly label: string;
};

export type UiSelectNode = {
  readonly type: "select";
  readonly id: string;
  readonly value: string;
  readonly items: readonly UiSelectItem[];
  readonly placeholder?: string;
  readonly onChange: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

export type UiToggleNode = {
  readonly type: "toggle";
  readonly id: string;
  readonly iconId: IconName;
  readonly pressed: boolean;
  readonly text?: string;
  readonly onChange: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

/** @emoji 🌿️ A nestable labeled container of {@link UiNode} children — the declarative-tree mechanism
 * for subtrees like `Origin > X/Y/Z`: {@link uiDeclarativeChildToTreeItem} expands a `Group` into a
 * {@link UiTreeItemNode} whose `items` are its recursively-converted children, so depth composes to
 * any level (`Plane > Origin > X/Y/Z`). Unlike {@link UiSectionNode} (top-level tree sections only,
 * see `assertNoNestedTreeSections`), a `Group` may itself appear as another `Group`'s or
 * {@link UiFieldNode}'s child. */
export type UiGroupNode = {
  readonly type: "group";
  readonly id: string;
  readonly label: string;
  readonly defaultOpen?: boolean;
  readonly menu?: UiMenuRef;
  readonly children: readonly UiNode[];
};

export type UiKeyValueEntry = {
  readonly label: string;
  readonly value: string;
};

export type UiKeyValueNode = {
  readonly type: "keyValue";
  readonly entries: readonly UiKeyValueEntry[];
  readonly menu?: UiMenuRef;
};

export type UiSliderNode = {
  readonly type: "slider";
  readonly id: string;
  readonly value: number;
  readonly min: number;
  readonly max: number;
  readonly step: number;
  readonly unit?: string;
  readonly onChange: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

export type UiNumberStepperNode = {
  readonly type: "numberStepper";
  readonly id: string;
  readonly value: number;
  readonly step: number;
  readonly uniform: boolean;
  readonly onAbsolute: ActionDescriptor;
  readonly onDelta: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

export type UiRingNode = {
  readonly type: "ring";
  readonly id: string;
  readonly orbId: string;
  readonly t: number;
  readonly disabled?: boolean;
  readonly onChange: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

export type UiIconSelectNode = {
  readonly type: "iconSelect";
  readonly id: string;
  readonly value: string;
  readonly uniform: boolean;
  readonly classifierKind: string;
  readonly onChange: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

export type UiFieldNode = {
  readonly type: "field";
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly required?: boolean;
  readonly error?: string;
  readonly child: UiNode;
  readonly menu?: UiMenuRef;
};

/** 🎨️ Renderer-side visual variant/size/density hints on a {@link UiButtonNode} — no wasm/plugin equivalent, purely a display hint. */
export type StyleSpec = {
  readonly variant?: string;
  readonly size?: string;
  readonly density?: string;
};

export type UiButtonNode = {
  readonly type: "button";
  readonly id?: string;
  readonly iconId: IconName;
  readonly label: string;
  readonly action: ActionDescriptor;
  readonly style?: StyleSpec;
  readonly disabled?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly menu?: UiMenuRef;
};

export type UiTextNode = {
  readonly type: "text";
  readonly value: string;
  readonly emphasize?: boolean;
  readonly dataAttributes?: Readonly<Record<string, string>>;
  readonly menu?: UiMenuRef;
};

export type UiStackNode = {
  readonly type: "stack";
  readonly direction: string;
  readonly gap?: string;
  readonly padding?: string;
  readonly id?: string;
  readonly selected?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly activate?: ActionDescriptor;
  readonly dropAction?: ActionDescriptor;
  readonly dropOverlay?: UiDropOverlaySpec;
  readonly menu?: UiMenuRef;
  readonly children: readonly UiNode[];  readonly presence?: UiPresence;
};

/** 📥️ Hover-state copy for a {@link UiStackNode}'s `dropOverlay` — shown while a drag is over the stack, ahead of `dropAction` firing on release. */
export type UiDropOverlaySpec = {
  readonly title: string;
  readonly hint: string;
  readonly accept?: string;
};

export type UiSeparatorNode = { readonly type: "separator"; readonly menu?: UiMenuRef };

export type UiImageNode = {
  readonly type: "image";
  readonly id: string;
  readonly src: string;
  readonly alt?: string;
  readonly menu?: UiMenuRef;
};

export type UiNode =
  | UiStackNode
  | UiTextNode
  | UiButtonNode
  | UiSeparatorNode
  | UiSectionNode
  | UiInputNode
  | UiSelectNode
  | UiToggleNode
  | UiKeyValueNode
  | UiSliderNode
  | UiNumberStepperNode
  | UiRingNode
  | UiIconSelectNode
  | UiFieldNode
  | UiGroupNode
  | UiTreeNode
  | UiImageNode
  | UiComponentSceneNode
  | UiExternalSlotNode;

export type UiInspectorFieldGroup = {
  readonly id: string;
  readonly label: string;
  readonly defaultOpen?: boolean;
  readonly fields: readonly UiNode[];
};


//#region 🔌️PluginAndAppContract
//#region PluginRuntime
/** 🧬️ Generated from Rust `ActionKind`/`ActionDefinition` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type ActionKind = GeneratedActionKind;
export type ActionDefinition = GeneratedActionDefinition;
export type ActionArgDef = GeneratedActionArgDef;
export type ActionArgControl = GeneratedActionArgControl;
export type ActionArgOption = GeneratedActionArgOption;
export type UtilityDefinition = GeneratedUtilityDefinition;
export type UtilityRef = GeneratedUtilityRef;

/** 🛠️ Generated from Rust `ToolDefinition`/`ToolRef` (`framework/core/rs/lib.rs`) — a mode-level,
 * activatable capability (e.g. puzzle3d fill), distinct from a per-window `UtilityDefinition`. See
 * `js/generated/manifest.ts`. */
export type ToolDefinition = GeneratedToolDefinition;
export type ToolRef = GeneratedToolRef;

/** 🎛️ Generated from Rust `CommandScope`/`CommandDefinition`/`CommandRef` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type CommandScope = GeneratedCommandScope;
export type CommandDefinition = GeneratedCommandDefinition;
export type CommandRef = GeneratedCommandRef;

/** 🧰️ The framework-owned action id apps dispatch to activate a utility — mirrors `SET_ACTIVE_UTILITY_ACTION_ID`. */
export const SET_ACTIVE_UTILITY_ACTION_ID = "setActiveUtility";

/** 🛠️ The framework-owned action id apps dispatch to activate a mode-level tool — mirrors Rust `SET_ACTIVE_TOOL_ACTION_ID`. */
export const SET_ACTIVE_TOOL_ACTION_ID = "setActiveTool";

/** 🎓️ The framework-owned action id apps dispatch (or the shell auto-injects into the command palette)
 * to (re)start an app's introduction — mirrors Rust `START_INTRODUCTION_ACTION_ID`. */
export const START_INTRODUCTION_ACTION_ID = "startIntroduction";

/** 🎓️ Generated from Rust `Introduction*` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type IntroductionDefinition = GeneratedIntroductionDefinition;
export type IntroductionStepDefinition = GeneratedIntroductionStepDefinition;
export type IntroductionPlacement = GeneratedIntroductionPlacement;
export type IntroductionInteraction = GeneratedIntroductionInteraction;
export type IntroductionInteractionKind = GeneratedIntroductionInteractionKind;
export type IntroductionLogo = GeneratedIntroductionLogo;
export type IntroductionPoint = GeneratedIntroductionPoint;
export type IntroductionGesture = GeneratedIntroductionGesture;
export type IntroductionKeyModifier = GeneratedIntroductionKeyModifier;
export type IntroductionPointerButton = GeneratedIntroductionPointerButton;
export type IntroductionCursor = GeneratedIntroductionCursor;
export type IntroductionDemonstration = GeneratedIntroductionDemonstration;

/** 🗨️ Generated from Rust `DialogDefinition` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type DialogDefinition = GeneratedDialogDefinition;

//#region 🎬️Tutorial
/** 🎬️ The framework-owned action id apps dispatch (or the shell auto-injects into the command palette,
 * with a `tutorialId` Select arg) to (re)start a tutorial — mirrors Rust `START_TUTORIAL_ACTION_ID`.
 * Distinct from the docs-tooltip `tutorial` link field on `UiLabelLeaf` (`framework/ui/js/react`), a URL into the
 * manual — this is the interactive recorded-walkthrough mechanism. */
export const START_TUTORIAL_ACTION_ID = "startTutorial";

/** ⏺️ The framework-owned action id that opens the tutorial recorder chrome — injected into EVERY app
 * unconditionally (recording needs no app-side declaration). Mirrors Rust `RECORD_TUTORIAL_ACTION_ID`. */
export const RECORD_TUTORIAL_ACTION_ID = "recordTutorial";

/** ⏱️ Real-time (rate-independent) duration of the camera glide the player performs when the user
 * presses Play after deviating from an active tutorial's recorded state. Mirrors Rust `TUTORIAL_CONVERGE_MS`. */
export const TUTORIAL_CONVERGE_MS = 600;

// 🚧️ TODO(core-rs): these seven `Tutorial*` types mirror `framework/core/rs/lib.rs`'s `//#region 🔖️Tutorial`
// field-for-field (see that region's doc comments for the authoritative semantics) and are meant to be
// ts-rs GENERATED like their `Introduction*` neighbors above. Regeneration is blocked right now by an
// unrelated, pre-existing `typegen`-feature compile break in a concurrent session's work (`IconName` is
// missing its `TS` derive in `framework/ui/wgpu/rs/lib.rs`, breaking `cargo test --features typegen` workspace-wide).
// Once that lands, run `bun nx run @semio-tech/framework:generate`, delete this hand-written block,
// and re-add `Tutorial* as GeneratedTutorial*` imports above — names/shapes here were written to match the
// eventual generated output exactly, so every other file importing from this module is unaffected.
export type TutorialChapter = { readonly id: string; readonly at: number; readonly title: LocalizedLabel | string; readonly body?: LocalizedLabel | string };

export type TutorialAssetSrc =
  | { readonly kind: "url"; readonly url: string }
  | { readonly kind: "blob"; readonly hash: string; readonly size: number; readonly mediaType: string }
  | { readonly kind: "dataUrl"; readonly data: string };

export type TutorialCaption = { readonly at: number; readonly durationMs: number; readonly text: string };

export type TutorialNarrationCue = {
  readonly id: string;
  readonly at: number;
  readonly durationMs: number;
  readonly text: string;
  readonly audio?: TutorialAssetSrc;
  readonly voice?: string;
  readonly rate: number;
  readonly captions: readonly TutorialCaption[];
};

export type TutorialOverlayRect = { readonly x: number; readonly y: number; readonly width: number; readonly height: number };

export type TutorialVideoCue = {
  readonly at: number;
  readonly durationMs: number;
  readonly src: TutorialAssetSrc;
  readonly rect: TutorialOverlayRect;
  readonly muted: boolean;
  readonly sourceOffsetMs: number;
};

export type TutorialEventKind =
  | { readonly kind: "action"; readonly action: string; readonly args?: unknown }
  | { readonly kind: "command"; readonly command: string; readonly args?: unknown }
  | { readonly kind: "key"; readonly keys: string };

export type TutorialEvent = { readonly at: number; readonly kind: TutorialEventKind };

/** 🧮️ Renderer-neutral restore point for chrome/UI state — see the Rust doc comment on
 * `TutorialUiSnapshot` for why this is deliberately NOT a serialization of `ShellState`. */
export type TutorialUiSnapshot = {
  readonly activeModeId?: string;
  readonly focusedWindowId?: string;
  readonly activeUtilityByWindowId: Readonly<Record<string, string>>;
  readonly activeToolId?: string;
  readonly layout?: WindowLayout;
  readonly activePanelTabByGroup: Readonly<Record<string, string>>;
  readonly panelJson?: string;
  readonly selectionJson?: string;
  readonly openDialogId?: string;
  readonly expandedTreeIds: readonly string[];
  readonly commandPanelOpen: boolean;
};

export type TutorialUiChange =
  | { readonly kind: "activeMode"; readonly id: string }
  | { readonly kind: "focusedWindow"; readonly id?: string }
  | { readonly kind: "activeUtility"; readonly windowId: string; readonly utilityId?: string }
  | { readonly kind: "activeTool"; readonly id?: string }
  | { readonly kind: "layout"; readonly layout: WindowLayout }
  | { readonly kind: "panelTab"; readonly group: string; readonly tabId?: string }
  | { readonly kind: "panelState"; readonly panelJson: string }
  | { readonly kind: "selection"; readonly selectionJson: string }
  | { readonly kind: "dialog"; readonly id?: string; readonly args?: unknown }
  | { readonly kind: "treeExpansion"; readonly id: string; readonly expanded: boolean }
  | { readonly kind: "commandPanel"; readonly open: boolean };

export type TutorialUiSample =
  | { readonly kind: "snapshot"; readonly state: TutorialUiSnapshot }
  | { readonly kind: "delta"; readonly changes: readonly TutorialUiChange[] };

export type TutorialUiKeyframe = { readonly at: number; readonly sample: TutorialUiSample };

/** 🖋️ Mirrors `store::DocumentCommand` with `Operation = unknown` (opaque per-app operation JSON) — the
 * SOLE source of document mutation during playback; `TutorialEvent`s are annotational only. */
export type TutorialDocumentEventKind =
  | { readonly kind: "edit"; readonly forwards: readonly unknown[]; readonly backwards: readonly unknown[]; readonly description?: string; readonly coalesceKey?: string }
  | { readonly kind: "undo" }
  | { readonly kind: "redo" }
  | { readonly kind: "checkpoint"; readonly message?: string }
  | { readonly kind: "checkoutCheckpoint"; readonly checkpointId: string }
  | { readonly kind: "switchAlternative"; readonly alternativeId: string }
  | { readonly kind: "load"; readonly documentDsl: string; readonly previousDsl: string };

export type TutorialDocumentEvent = { readonly at: number; readonly kind: TutorialDocumentEventKind };

export type TutorialCameraState =
  | { readonly kind: "orbit"; readonly position: readonly [number, number, number]; readonly target: readonly [number, number, number]; readonly up: readonly [number, number, number]; readonly fov?: number }
  | { readonly kind: "canvas"; readonly x: number; readonly y: number; readonly zoom: number };

export type TutorialEasing = "linear" | "easeInOut" | "hold";

export type TutorialCameraKeyframe = { readonly at: number; readonly windowId: string; readonly camera: TutorialCameraState; readonly easing: TutorialEasing };

/** 👻️ Reuses the introduction demonstration vocabulary verbatim — see `IntroductionGesture`/`IntroductionPoint`. */
export type TutorialGestureCue = { readonly at: number; readonly durationMs: number; readonly gesture: IntroductionGesture; readonly cursor?: IntroductionCursor };

export type TutorialTracks = {
  readonly narration: readonly TutorialNarrationCue[];
  readonly video: readonly TutorialVideoCue[];
  readonly events: readonly TutorialEvent[];
  readonly ui: readonly TutorialUiKeyframe[];
  readonly document: readonly TutorialDocumentEvent[];
  readonly camera: readonly TutorialCameraKeyframe[];
  readonly gestures: readonly TutorialGestureCue[];
};

export type TutorialBase = {
  readonly documentDsl?: string;
  readonly exampleId?: string;
  readonly ui: TutorialUiSnapshot;
  readonly cameras: readonly TutorialCameraKeyframe[];
};

/** 🎬️ A recorded, timed, replayable walkthrough — the timeline sibling of `IntroductionDefinition`. A
 * *recording* IS a `TutorialDefinition`; the recorder simply produces a densely-sampled one. */
export type TutorialDefinition = {
  readonly id: string;
  readonly title: LocalizedLabel | string;
  readonly description?: LocalizedLabel | string;
  readonly durationMs: number;
  readonly chapters: readonly TutorialChapter[];
  readonly base: TutorialBase;
  readonly tracks: TutorialTracks;
  readonly recordedAt?: string;
};
//#endregion 🎬️Tutorial

//#region 🏷️ShellBrand
// 🌐️ ShellLocale/ShellTerminology are generated from ui_wgpu's 🔣️ui-axes.json (the same source of
// truth Rust's Locale/Terminology enums derive from), imported/re-exported above — so a locale
// added there and here can never drift. The single source `UiLocale` (`framework/ui/js/react`),
// `ShellBrandLocks.locale`, and `resolveShellLocks` all derive from this.

/** 🔒️ Shell preferences a brand pins at boot: each set axis is fixed and its in-app switcher hidden (validated by the renderer's `resolveShellLocks`). */
export type ShellBrandLocks = {
  readonly exampleId?: string;
  readonly locale?: ShellLocale;
  readonly terminology?: ShellTerminology;
  readonly themeId?: string;
  readonly appearance?: string;
};

/** 🎛️ Shell preferences a brand seeds at boot without pinning them: the value applies on first launch but the in-app switcher stays visible. */
export type ShellBrandDefaults = {
  readonly exampleId?: string;
};

/** 🏷️ Boot-time branding for a standalone shell artifact — identity (window title, logo mark, favicon), locked and defaulted shell preferences, and an optional brand-owned {@link IntroductionDefinition} replacing the app's own (already localized, rendered verbatim). */
export type ShellBrand = {
  readonly id: string;
  readonly windowTitle: string;
  readonly logoSvg?: string;
  readonly faviconIcoPath?: string;
  readonly locks?: ShellBrandLocks;
  readonly defaults?: ShellBrandDefaults;
  readonly introduction?: IntroductionDefinition;
  /** 🎬️ Brand-owned tutorials shown ALONGSIDE the app's own declared ones (never replacing them, unlike `introduction`). */
  readonly tutorials?: readonly TutorialDefinition[];
  /** 🎓️ When true, auto-starts the brand introduction on every window load and never persists a device-local "seen" flag. */
  readonly replayIntroductionOnLoad?: boolean;
  /** 🧊️ When true, the shell never reads or writes device-local shell state (dock, panes, named layouts, chrome prefs, introduction seen) — every refresh boots from brand locks/defaults only. */
  readonly ephemeral?: boolean;
  /** 🗂️ Repo-root-relative directory of this brand's own static assets (logos, etc.) — the dev/build server mounts it as a static route at `/<assetsDir>` alongside the shared `framework/ui/asset` mount. */
  readonly assetsDir?: string;
  /** 📦️ Repo-root-relative directory this brand's build output lands in instead of the shared playground `dist/` — keeps a brand's specialization (including its build artifact) self-contained. */
  readonly distDir?: string;
  /** 🌐️ Custom domain this brand's static build deploys to (e.g. GitHub Pages) — written verbatim into a `🌐️CNAME` file at the build root. */
  readonly cnameHost?: string;
};
//#endregion 🏷️ShellBrand

/** @emoji 🕹️ Mirrors `semio_framework_core::history_action_definitions` — the six framework-owned
 * History actions every app receives, used by the shell to render the same set without a wasm round trip. */
export const HISTORY_ACTION_IDS = ["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint"] as const;

export type PluginViewState = {
  readonly activeModeId?: string;
  readonly activeWindowKindId?: string;
  /** 🧰️ Per-call overlay: host-owned active utility for the window targeted by this render/action (`windowId`). */
  readonly activeUtilityId?: string;
  /** 🧰️ Host-owned active utility per window instance (never a document field, never a VCS operation). */
  readonly activeUtilityByWindowId?: Readonly<Record<string, string>>;
  /** 🛠️ Host-owned active tool of the active mode (never a document field, never a VCS operation) — mutually
   * exclusive with `activeUtilityId`: activating one clears the other. */
  readonly activeToolId?: string;
  readonly selectionJson?: string;
  readonly panelJson?: string;
  readonly contributionsJson?: string;
  readonly locale?: string;
  readonly terminology?: string;
  /** 🪟️ The window instance a render/action call targets — programs key per-window option state off this, never off `activeWindowKindId`. */
  readonly windowId?: string;
  /** 🪟️ The live set of open window instances (base + spawned/split), so `windowMeasures`/`windowEngagements` can return one entry per instance. */
  readonly windowInstances?: readonly { readonly id: string; readonly windowKindId: string }[];
};

export type PluginUiNode = Record<string, unknown> & { readonly type: string };

/** 🗣️ Locale/terminology-aware label patch for an app's window-kind/panel-tab/mode labels, resolved fresh per {@link PluginViewState} — merge over the static {@link PluginManifest} app labels by id. */
export type PluginAppLabelsOverlay = {
  readonly windowKindLabels: Readonly<Record<string, string>>;
  readonly panelTabLabels: Readonly<Record<string, string>>;
  readonly modeLabels: Readonly<Record<string, string>>;
  readonly actionLabels: Readonly<Record<string, string>>;
  readonly utilityLabels: Readonly<Record<string, string>>;
  readonly exampleLabels: Readonly<Record<string, string>>;
  readonly actionArgLabels: Readonly<Record<string, string>>;
  readonly dialogLabels: Readonly<Record<string, string>>;
  readonly introductionLabels: Readonly<Record<string, string>>;
  readonly groupLabels: Readonly<Record<string, string>>;
};

export const EMPTY_APP_LABELS_OVERLAY: PluginAppLabelsOverlay = {
  windowKindLabels: {},
  panelTabLabels: {},
  modeLabels: {},
  actionLabels: {},
  utilityLabels: {},
  exampleLabels: {},
  actionArgLabels: {},
  dialogLabels: {},
  introductionLabels: {},
  groupLabels: {},
};

/** 🗣️ Rust's `skip_serializing_if` omits empty maps entirely, so a parsed overlay may be missing keys — fill them back in. */
export function normalizeAppLabelsOverlay(raw: Partial<PluginAppLabelsOverlay> | null | undefined): PluginAppLabelsOverlay {
  return {
    windowKindLabels: raw?.windowKindLabels ?? {},
    panelTabLabels: raw?.panelTabLabels ?? {},
    modeLabels: raw?.modeLabels ?? {},
    actionLabels: raw?.actionLabels ?? {},
    utilityLabels: raw?.utilityLabels ?? {},
    exampleLabels: raw?.exampleLabels ?? {},
    actionArgLabels: raw?.actionArgLabels ?? {},
    dialogLabels: raw?.dialogLabels ?? {},
    introductionLabels: raw?.introductionLabels ?? {},
    groupLabels: raw?.groupLabels ?? {},
  };
}

export type PluginContribution =
  | {
      readonly kind: "playbookBlockKind";
      readonly appId: string;
      readonly blockKind: string;
      readonly label: string;
      readonly iconId: IconName;
      readonly defaultValueJson?: string;
      readonly paramsBodyKey: string;
      readonly previewBodyKey: string;
    }
  | {
      readonly kind: "sourcingModule";
      readonly appId: string;
      readonly moduleId: string;
      readonly label: string;
      readonly iconId: IconName;
      readonly typologyJson: string;
      readonly kindsJson: string;
    }
  | {
      readonly kind: "processMachines";
      readonly appId: string;
      readonly moduleId: string;
      readonly label: string;
      readonly iconId: IconName;
      readonly machinesJson: string;
    }
  | {
      readonly kind: "flowExtension";
      readonly appId: string;
      readonly extensionId: string;
      readonly label: string;
      readonly iconId: IconName;
      readonly manifestJson: string;
    }
  | {
      readonly kind: "formsQuestionKind";
      readonly appId: string;
      readonly questionKind: string;
      readonly label: string;
      readonly iconId: IconName;
      readonly defaultValueJson?: string;
      readonly paramsBodyKey: string;
      readonly previewBodyKey: string;
    }
  | {
      readonly kind: "cadComputer";
      readonly appId: string;
      readonly moduleId: string;
      readonly label: string;
      readonly iconId: IconName;
      readonly computersJson: string;
    }
  | {
      readonly kind: "imperativeModule";
      readonly appId: string;
      readonly moduleId: string;
      readonly label: string;
      readonly iconId: IconName;
      readonly manifestJson: string;
    };

export type ProgramContributionEntry = {
  readonly pluginId: string;
  readonly contribution: PluginContribution;
};

export type PluginManifest = {
  readonly pluginId: string;
  readonly label: string;
  readonly version: string;
  readonly apps: readonly Record<string, unknown>[];
  readonly workflows: readonly {
    readonly workflowStepId: string;
    readonly appId: string;
    readonly label: string;
    readonly document?: readonly string[];
    readonly yields: string;
  }[];
  readonly examples: readonly { readonly id: string; readonly label: string; readonly documentJson: string; readonly appId: string }[];
  readonly contributions?: readonly PluginContribution[];
  /** 🎛️ Plugin-scope commands this plugin exposes — apply whenever any of its apps is focused. */
  readonly commands?: readonly CommandDefinition[];
};

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
    case "app":
      return kind.id;
  }
}

/** 🌳️ Mirrors Rust `PanelTabDefinition` — a leaf carries `bodyKey`, a branch carries `children`; `group` is only meaningful on root entries. */
export type AppPanelTabDefinition = GeneratedPanelTabDefinition;

/** 📦️ Mirrors Rust `AppDefinition` — generated 1:1 from `framework/core/rs/lib.rs` via ts-rs, except
 * `defaultLayout`/`namedLayouts` which keep this file's narrower hand-refined `WindowLayout` (ts-rs
 * widens `WindowLayoutAxisNode.kind`/`WindowLayoutStackNode.kind` to plain `string` since the Rust
 * field is a runtime `String`, not an enum — the narrower `"row" | "column" | "stack" | "window"`
 * literal unions here are domain knowledge worth keeping for exhaustive switches). */
export type AppActionDefinition = Omit<GeneratedActionDefinition, "iconId"> & { readonly iconId?: IconName };
export type AppUtilityDefinition = Omit<GeneratedUtilityDefinition, "iconId"> & { readonly iconId: IconName };
export type AppToolDefinition = Omit<GeneratedToolDefinition, "iconId"> & { readonly iconId: IconName };
export type AppCommandDefinition = Omit<GeneratedCommandDefinition, "iconId"> & { readonly iconId?: IconName };
export type AppWindowKindDefinition = Omit<GeneratedWindowKindDefinition, "iconId"> & { readonly iconId: IconName };
export type AppDefinition = Omit<GeneratedAppDefinition, "defaultLayout" | "namedLayouts" | "iconId"> & {
  readonly defaultLayout?: WindowLayout;
  readonly namedLayouts: readonly NamedLayout[];
  readonly iconId?: IconName;
  /** 🎬️ TODO(core-rs): fold into `GeneratedAppDefinition.tutorials` once typegen is unblocked (see the
   * `//#region 🎬️Tutorial` TODO above) — same field name/shape. */
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
/** @emoji 🐢️ One requested window/panel section — `bodyKey` only applies to windows/panels; `hash` is the host's known fnv1a-64 hex of that section's last payload, or absent on first fetch. */
export type PluginUiRefreshSectionRequest = { readonly key: string; readonly bodyKey?: string; readonly hash?: string };

/** @emoji 🐢️ One batched, hash-conditional refresh request — one round trip for the window/panel/engagements/measures/labels sections. Utility bars are no longer a plugin section: the renderer derives them from the utility registry via {@link deriveUtilityNodes}. */
export type PluginUiRefreshRequest = {
  readonly viewState: PluginViewState;
  readonly windows?: readonly PluginUiRefreshSectionRequest[];
  readonly panels?: readonly PluginUiRefreshSectionRequest[];
  readonly engagements?: { readonly hash?: string };
  readonly measures?: { readonly hash?: string };
  /** 🛠️ Mode-level tool measures, keyed by tool id — see `DocumentApp::tool_measures`. */
  readonly tools?: { readonly hash?: string };
  readonly labels?: { readonly hash?: string };
};

/** @emoji 🐢️ `value` is present only when `hash` differs from what the request supplied — an unchanged section costs one hash compare instead of a full re-serialize. */
export type PluginUiRefreshSectionResponse = { readonly key: string; readonly hash: string; readonly value?: unknown };

export type PluginUiRefreshResponse = {
  readonly windows?: readonly PluginUiRefreshSectionResponse[];
  readonly panels?: readonly PluginUiRefreshSectionResponse[];
  readonly engagements?: PluginUiRefreshSectionResponse;
  readonly measures?: PluginUiRefreshSectionResponse;
  readonly tools?: PluginUiRefreshSectionResponse;
  readonly labels?: PluginUiRefreshSectionResponse;
  /** ⏱️ See `DocumentApp::pending_effects` — background work (e.g. a `flowEvalTick` chain) the host
   * should dispatch right after this refresh, fed through the same `applyHostEffects` pass as an
   * action's own `requestedEffects`. */
  readonly requestedEffects?: readonly HostEffect[];
};
//#endregion UiRefresh

//#region 🖱️ContextMenu
/** @emoji 🖱️ Scene-target info for an on-demand context-menu request — hit-test results from the
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

/** @emoji 🖱️ On-demand context-menu request — never cached, never batched into {@link PluginUiRefreshRequest}.
 * `menu` is the {@link UiMenuRef} the host resolved from `data-menu-id`/a scene surface convention id
 * (`"world3d"`, `"nodeGraph"`, `"window"`, `"panel:<tabId>"`, ...). */
export type PluginContextMenuRequest = {
  readonly menu: UiMenuRef;
  readonly surface?: PluginContextMenuSurfaceTarget;
  readonly windowInstanceId?: string;
  readonly point?: PluginContextMenuPoint;
};

export type PluginContextMenuResponse = {
  readonly items: readonly ContextMenuItemSpec[];
};
//#endregion 🖱️ContextMenu

/**
 * 📡️ Host-facing shape of one loaded plugin, mirroring the 5-function `semio:framework/plugin` WIT
 * ABI exactly (`world.wit`): `manifest`/`instantiate-app`(as `createApp`)/`exchange` are the whole
 * runtime surface now — every former per-verb call (`handleAction`, `render`, `refreshUi`,
 * `contextMenu`, ...) is a binary `protocol_channel::AppCommand` sent through {@link exchange}
 * instead (see `🔖️AppChannelClient` in the os-product package, which frames these bytes). `dispose`
 * remains host-side only (never part of the WIT ABI) for worker/resource teardown.
 */
//#endregion 🔌️PluginAndAppContract
// #endregion 🛂️Manifest
