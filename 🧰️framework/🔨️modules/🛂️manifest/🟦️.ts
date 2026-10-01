import { dialectCoordinate, parseDialectCoordinate, type ArtifactDialect } from "../🚪️io/🧬️schema/🟦️.ts";
import type { AppRole, AppRef } from "./🧬️schema/🟦️.ts";
export { surfaceAppId, parseSurfaceAppId, type AppRole, type AppRef } from "./🧬️schema/🟦️.ts";
// #region 🛂️Manifest
/// <reference types="vitest/importMeta" />
/** 🛂️ `@semio-tech/framework` — AppDefinition, PluginManifest, contributions, and declarative UI contract. */
import type { IconName } from "@semio-tech/assets";
export type { IconName };
import { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology, type ShellLocale, type ShellTerminology, type LocalizedLabel } from "./🤖️generated/🎚️ui-axes/🟦️.ts";
export { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology };
export type { ShellLocale, ShellTerminology, LocalizedLabel };
// 🧭️ `ContextMenuItemSpec`/`Effect` are hand-written types owned by sibling modules aggregated
// alongside this one into `@semio-tech/framework` (see `🟦️.ts`) — that aggregation only helps
// EXTERNAL consumers of the package; this file's own internal references (`PluginContextMenuResponse`,
// `PluginUiRefreshResponse`) still need a real import, type-only so the cycle back through
// `🖱️ui/🎬️scene/🟦️.ts`'s own `ActionDescriptor` import from this file erases cleanly.
import type { ContextMenuItemSpec } from "../🖱️ui/🎬️scene/🟦️.ts";
import inputLabelGlossaryDocument from "./🔣️input-labels.json" with { type: "json" };
import type { Effect } from "../🎠️kernel/🟦️.ts";

// #region 🧬️GeneratedMirror
/** 🧬️ Types generated from `framework/core/rs/lib.rs` via the owned schema exporter (`bun nx run @semio-tech/framework:generate`); re-exported below alongside their hand-written neighbors so this stays the one import surface. */
import type {
  ActionDescriptor as GeneratedActionDescriptor,
  ActionKind as GeneratedActionKind,
  ActionDefinition as GeneratedActionDefinition,
  ActionAddress as GeneratedActionAddress,
  ActionInvocation as GeneratedActionInvocation,
  ActionArgDef as GeneratedActionArgDef,
  ActionArgControl as GeneratedActionArgControl,
  ActionArgOption as GeneratedActionArgOption,
  // 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema, D6:
  // `ArgSchema`/`ArgFormat`/`ArgPresentation` are the new stored-truth vocabulary behind
  // `ActionArgDef.schema`/`.presentation` — `argControl()` below mirrors Rust `ActionArgDef::control()`.
  ArgSchema as GeneratedArgSchema,
  ArgFormat as GeneratedArgFormat,
  ArgPresentation as GeneratedArgPresentation,
  SnapSource as GeneratedSnapSource,
  NumberScale as GeneratedNumberScale,
  ReferenceIdType as GeneratedReferenceIdType,
  // 🎯️ §3.1 `🔖️ActionSemantics` — effects/policy/execution + natural-language framing.
  ResourceSelector as GeneratedResourceSelector,
  CapabilityEffects as GeneratedCapabilityEffects,
  ApprovalMode as GeneratedApprovalMode,
  CapabilityPolicy as GeneratedCapabilityPolicy,
  PreviewMode as GeneratedPreviewMode,
  UndoMode as GeneratedUndoMode,
  IdempotencyMode as GeneratedIdempotencyMode,
  ExecutionClass as GeneratedExecutionClass,
  CapabilityExecution as GeneratedCapabilityExecution,
  ActionSemantics as GeneratedActionSemantics,
  UtilityDefinition as GeneratedUtilityDefinition,
  UtilityRef as GeneratedUtilityRef,
  // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W1: the wave-0 interaction
  // definition family (see `🕹️interaction/🦀️.rs`), typegen-mirrored here exactly like
  // its `Action*`/`Utility*` neighbors above.
  InteractionDefinition as GeneratedInteractionDefinition,
  GranularityDefinition as GeneratedGranularityDefinition,
  HierarchyProvider as GeneratedHierarchyProvider,
  HoverSpec as GeneratedHoverSpec,
  SelectionSpec as GeneratedSelectionSpec,
  SelectionMode as GeneratedSelectionMode,
  SelectionMethod as GeneratedSelectionMethod,
  MergeMode as GeneratedMergeMode,
  InteractionRef as GeneratedInteractionRef,
  // 🕹️ W3a: `TutorialUiSnapshot.interactionSelection` carries this directly (see
  // `TutorialUiChange` below) — the manifest-typegen twin of the hand-written runtime
  // `DomainSelection` in `🕹️interaction/🟦️.ts`, same duplication shape as
  // `HierarchyProvider`/`HoverSpec`/`SelectionSpec`/`MergeMode` above.
  DomainSelection as GeneratedDomainSelection,
  ToolDefinition as GeneratedToolDefinition,
  ToolRef as GeneratedToolRef,
  JobKindId as GeneratedJobKindId,
  ToolRunCounterDefinition as GeneratedToolRunCounterDefinition,
  ToolRunDefinition as GeneratedToolRunDefinition,
  ToolRunReasonDefinition as GeneratedToolRunReasonDefinition,
  ToolRunRebasePolicy as GeneratedToolRunRebasePolicy,
  ToolRunReconfigurePolicy as GeneratedToolRunReconfigurePolicy,
  ToolRunSettingsReads as GeneratedToolRunSettingsReads,
  ToolRunStageDefinition as GeneratedToolRunStageDefinition,
  ToolRunTraceKind as GeneratedToolRunTraceKind,
  ToolRunVerdict as GeneratedToolRunVerdict,
  CommandDefinition as GeneratedCommandDefinition,
  CommandOwnerAddress as GeneratedCommandOwnerAddress,
  CommandAddress as GeneratedCommandAddress,
  CommandInvocation as GeneratedCommandInvocation,
  OsDefinition as GeneratedOsDefinition,
  Platform as GeneratedPlatform,
  PlatformKeybinding as GeneratedPlatformKeybinding,
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
  DialogChoice as GeneratedDialogChoice,
  DialogDefinition as GeneratedDialogDefinition,
  // 🎬️ `//#region 🔖️Tutorial` (`🛂️manifest/🦀️.rs`) — the timeline sibling of
  // `IntroductionDefinition`, typegen-mirrored here exactly like its `Introduction*` neighbors above.
  TutorialDocumentEvent as GeneratedTutorialDocumentEvent,
  TutorialDocumentEventKind as GeneratedTutorialDocumentEventKind,
  TutorialAssetSrc as GeneratedTutorialAssetSrc,
  TutorialBase as GeneratedTutorialBase,
  TutorialCameraKeyframe as GeneratedTutorialCameraKeyframe,
  TutorialCameraState as GeneratedTutorialCameraState,
  TutorialCaption as GeneratedTutorialCaption,
  TutorialChapter as GeneratedTutorialChapter,
  TutorialDefinition as GeneratedTutorialDefinition,
  TutorialEasing as GeneratedTutorialEasing,
  TutorialEvent as GeneratedTutorialEvent,
  TutorialEventKind as GeneratedTutorialEventKind,
  TutorialGestureCue as GeneratedTutorialGestureCue,
  TutorialNarrationCue as GeneratedTutorialNarrationCue,
  TutorialOverlayRect as GeneratedTutorialOverlayRect,
  TutorialTracks as GeneratedTutorialTracks,
  TutorialUiChange as GeneratedTutorialUiChange,
  TutorialUiKeyframe as GeneratedTutorialUiKeyframe,
  TutorialUiSample as GeneratedTutorialUiSample,
  TutorialUiSnapshot as GeneratedTutorialUiSnapshot,
  TutorialVideoCue as GeneratedTutorialVideoCue,
  UiPresence as GeneratedUiPresence,
  UiState as GeneratedUiState,
  UiStatus as GeneratedUiStatus,
  // 🎫️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION §C8.1: the
  // `🔖️HostResolvedArgs` region below (`ArtifactKindChoice`/`SurfaceAppChoice`/`artifactKindChoices`)
  // names all three by hand, unlike `ArgFormat`'s inline `roles: Array<AppRole>` above.
} from "./🤖️generated/🪪️manifest/🟦️.ts";
// #endregion 🧬️GeneratedMirror

// #region 🧬️GeneratedUiContract
/** 🧬️ The semantic UI contract (`🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️.rs`) — flat,
 * id-keyed replacement for the hand-written `UiNode` recursive-union mirror this file used to
 * carry, generated via the owned schema exporter (`bun nx run @semio-tech/ui-contract-rs:generate`). Five names
 * collide with an unrelated existing export from a different module aggregated into this same
 * barrel (artifact-editor `SurfaceKind`; OS-shell `WindowLayout`/`WindowStackCorner`; the state
 * machine module's own `ActionId`/`Trigger`) and are re-exported `Ui`-prefixed instead of
 * shadowing them; everything else keeps its Rust name verbatim. */
import type {
  Liveness as GeneratedLiveness,
  AccessibilitySpec as GeneratedAccessibilitySpec,
  ActionId as GeneratedActionId,
  Trigger as GeneratedTrigger,
  ActionBinding as GeneratedActionBinding,
  MenuRef as GeneratedMenuRef,
  UiIntent as GeneratedUiIntent,
  UiValue as GeneratedUiValue,
  BuiltNode as GeneratedBuiltNode,
  Label as GeneratedLabel,
  ContainerRole as GeneratedContainerRole,
  InputKind as GeneratedInputKind,
  RowActionPlacement as GeneratedRowActionPlacement,
  DropOverlaySpec as GeneratedDropOverlaySpec,
  SelectItem as GeneratedSelectItem,
  KeyValueEntry as GeneratedKeyValueEntry,
  RowAction as GeneratedRowAction,
  RowTarget as GeneratedRowTarget,
  ContainerProps as GeneratedContainerProps,
  TextProps as GeneratedTextProps,
  ButtonProps as GeneratedButtonProps,
  SeparatorProps as GeneratedSeparatorProps,
  InputProps as GeneratedInputProps,
  SelectProps as GeneratedSelectProps,
  ToggleProps as GeneratedToggleProps,
  ToggleAppearance as GeneratedToggleAppearance,
  KeyValueListProps as GeneratedKeyValueListProps,
  SliderProps as GeneratedSliderProps,
  NumberStepperProps as GeneratedNumberStepperProps,
  RingProps as GeneratedRingProps,
  IconSelectProps as GeneratedIconSelectProps,
  ProgressProps as GeneratedProgressProps,
  TreeProps as GeneratedTreeProps,
  TreePresentation as GeneratedTreePresentation,
  TreeWindowRowExtent as GeneratedTreeWindowRowExtent,
  TreeWindow as GeneratedTreeWindow,
  TreeSectionProps as GeneratedTreeSectionProps,
  TreeItemProps as GeneratedTreeItemProps,
  TableProps as GeneratedTableProps,
  TableRowProps as GeneratedTableRowProps,
  ImageProps as GeneratedImageProps,
  ExtensionProps as GeneratedExtensionProps,
  Component as GeneratedComponent,
  SurfaceId as GeneratedSurfaceId,
  UiNodeId as GeneratedUiNodeId,
  UiRevision as GeneratedUiRevision,
  TransitionHint as GeneratedTransitionHint,
  UiNodeRecord as GeneratedUiNodeRecord,
  UiSnapshot as GeneratedUiSnapshot,
  UiPatchOp as GeneratedUiPatchOp,
  UiPatch as GeneratedUiPatch,
  SpaceToken as GeneratedSpaceToken,
  Sizing as GeneratedSizing,
  Axis as GeneratedAxis,
  Align as GeneratedAlign,
  Justify as GeneratedJustify,
  GridTrack as GeneratedGridTrack,
  ScrollAxes as GeneratedScrollAxes,
  Anchor as GeneratedAnchor,
  EdgeSpace as GeneratedEdgeSpace,
  StackLayout as GeneratedStackLayout,
  GridLayout as GeneratedGridLayout,
  OverlayLayout as GeneratedOverlayLayout,
  ScrollLayout as GeneratedScrollLayout,
  AbsoluteLayout as GeneratedAbsoluteLayout,
  LeafLayout as GeneratedLeafLayout,
  LayoutSpec as GeneratedLayoutSpec,
  WindowStackCorner as GeneratedWindowStackCorner,
  WindowLayoutNode as GeneratedWindowLayoutNode,
  WindowLayout as GeneratedWindowLayout,
  UiDocumentLimits as GeneratedUiDocumentLimits,
  UiNumberBound as GeneratedUiNumberBound,
  UiNumberLimits as GeneratedUiNumberLimits,
  UiNumberScale as GeneratedUiNumberScale,
  UiContractViolation as GeneratedUiContractViolation,
  PatchRejection as GeneratedPatchRejection,
  QuotaKind as GeneratedQuotaKind,
  Activity as GeneratedActivity,
  PeerMark as GeneratedPeerMark,
  OwnPresence as GeneratedOwnPresence,
  PresenceUpdate as GeneratedPresenceUpdate,
  Variant as GeneratedVariant,
  SizeToken as GeneratedSizeToken,
  Density as GeneratedDensity,
  Tone as GeneratedTone,
  Emphasis as GeneratedEmphasis,
  StyleSpec as GeneratedStyleSpec,
  SurfaceKind as GeneratedSurfaceKind,
  SurfaceDoc as GeneratedSurfaceDoc,
  SurfaceProps as GeneratedSurfaceProps,
} from "./🤖️generated/📜️ui-contract/🟦️.ts";

export type Liveness = GeneratedLiveness;
export type AccessibilitySpec = GeneratedAccessibilitySpec;
export type UiActionId = GeneratedActionId;
export type UiTrigger = GeneratedTrigger;
export type ActionBinding = GeneratedActionBinding;
export type MenuRef = GeneratedMenuRef;
export type UiIntent = GeneratedUiIntent;
export type UiValue = GeneratedUiValue;
export type BuiltNode = GeneratedBuiltNode;
export type Label = GeneratedLabel;
export type ContainerRole = GeneratedContainerRole;
export type InputKind = GeneratedInputKind;
export type RowActionPlacement = GeneratedRowActionPlacement;
export type DropOverlaySpec = GeneratedDropOverlaySpec;
export type SelectItem = GeneratedSelectItem;
export type KeyValueEntry = GeneratedKeyValueEntry;
export type RowAction = GeneratedRowAction;
export type RowTarget = GeneratedRowTarget;
export type ContainerProps = GeneratedContainerProps;
export type TextProps = GeneratedTextProps;
export type ButtonProps = GeneratedButtonProps;
export type SeparatorProps = GeneratedSeparatorProps;
export type InputProps = GeneratedInputProps;
export type SelectProps = GeneratedSelectProps;
export type ToggleProps = GeneratedToggleProps;
export type ToggleAppearance = GeneratedToggleAppearance;
export type KeyValueListProps = GeneratedKeyValueListProps;
export type SliderProps = GeneratedSliderProps;
export type NumberStepperProps = GeneratedNumberStepperProps;
export type RingProps = GeneratedRingProps;
export type IconSelectProps = GeneratedIconSelectProps;
export type ProgressProps = GeneratedProgressProps;
export type TreeProps = GeneratedTreeProps;
export type TreePresentation = GeneratedTreePresentation;
export type TreeWindowRowExtent = GeneratedTreeWindowRowExtent;
export type TreeWindow = GeneratedTreeWindow;
export type TreeSectionProps = GeneratedTreeSectionProps;
export type TreeItemProps = GeneratedTreeItemProps;
export type TableProps = GeneratedTableProps;
export type TableRowProps = GeneratedTableRowProps;
export type ImageProps = GeneratedImageProps;
export type ExtensionProps = GeneratedExtensionProps;
export type Component = GeneratedComponent;
export type SurfaceId = GeneratedSurfaceId;
export type UiNodeId = GeneratedUiNodeId;
export type UiRevision = GeneratedUiRevision;
export type TransitionHint = GeneratedTransitionHint;
export type UiNodeRecord = GeneratedUiNodeRecord;
export type UiSnapshot = GeneratedUiSnapshot;
export type UiPatchOp = GeneratedUiPatchOp;
export type UiPatch = GeneratedUiPatch;
export type SpaceToken = GeneratedSpaceToken;
export type Sizing = GeneratedSizing;
export type Axis = GeneratedAxis;
export type Align = GeneratedAlign;
export type Justify = GeneratedJustify;
export type GridTrack = GeneratedGridTrack;
export type ScrollAxes = GeneratedScrollAxes;
export type Anchor = GeneratedAnchor;
export type EdgeSpace = GeneratedEdgeSpace;
export type StackLayout = GeneratedStackLayout;
export type GridLayout = GeneratedGridLayout;
export type OverlayLayout = GeneratedOverlayLayout;
export type ScrollLayout = GeneratedScrollLayout;
export type AbsoluteLayout = GeneratedAbsoluteLayout;
export type LeafLayout = GeneratedLeafLayout;
export type LayoutSpec = GeneratedLayoutSpec;
export type UiWindowStackCorner = GeneratedWindowStackCorner;
export type WindowLayoutNode = GeneratedWindowLayoutNode;
export type UiWindowLayout = GeneratedWindowLayout;
export type UiDocumentLimits = GeneratedUiDocumentLimits;
export type UiNumberBound = GeneratedUiNumberBound;
export type UiNumberLimits = GeneratedUiNumberLimits;
export type UiNumberScale = GeneratedUiNumberScale;
export type UiContractViolation = GeneratedUiContractViolation;
export type PatchRejection = GeneratedPatchRejection;
export type QuotaKind = GeneratedQuotaKind;
export type Activity = GeneratedActivity;
export type PeerMark = GeneratedPeerMark;
export type OwnPresence = GeneratedOwnPresence;
export type PresenceUpdate = GeneratedPresenceUpdate;
export type Variant = GeneratedVariant;
export type SizeToken = GeneratedSizeToken;
export type Density = GeneratedDensity;
export type Tone = GeneratedTone;
export type Emphasis = GeneratedEmphasis;
export type StyleSpec = GeneratedStyleSpec;
export type UiSurfaceKind = GeneratedSurfaceKind;
export type SurfaceDoc = GeneratedSurfaceDoc;
export type SurfaceProps = GeneratedSurfaceProps;
// #endregion 🧬️GeneratedUiContract

export const CANVAS_HOVER_SOURCE_CANVAS = "canvas";
export const CANVAS_HOVER_SOURCE_PICK_MENU = "pick-menu";
export const CANVAS_HOVER_SOURCE_CATALOG = "catalog";
export const CANVAS_HOVER_SOURCE_ARTIFACT = "artifact";

export const FRAMEWORK_PANEL_TAB_ARTIFACT_ID = "framework.panel.artifact";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_ID = "framework.panel.catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_ID = "framework.panel.inspection";
export const FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL = "Artifact";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL = "Catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_LABEL = "Inspection";
export const FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID = "framework.panel.artifact";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID = "framework.panel.catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID = "framework.panel.inspection";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_ID = "framework.panel.parameters";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL = "Parameters";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID = "framework.panel.parameters";
/** 🕰️ Mirrors Rust `FRAMEWORK_PANEL_TAB_HISTORY_ID` — auto-injected into every app's `panelTabs`. */
export const FRAMEWORK_PANEL_TAB_HISTORY_ID = "framework.panel.history";
export const FRAMEWORK_PANEL_TAB_HISTORY_LABEL = "History";
export const FRAMEWORK_PANEL_TAB_HISTORY_ICON_ID = "framework.panel.history";
/** 🕰️ Mirrors Rust `FRAMEWORK_HISTORY_BODY_KEY` — the reserved panel body every renderer fetches for the
 * command-history list, and the one surface a completion's history patch dirties on its own. */
export const FRAMEWORK_HISTORY_BODY_KEY = "framework.body.history";
/** ⏯️ Mirrors Rust `FRAMEWORK_PANEL_TAB_TOOL_RUN_ID` — auto-injected into the `panelTabs` of every app that declares a tool run. */
export const FRAMEWORK_PANEL_TAB_TOOL_RUN_ID = "framework.panel.toolRun";
export const FRAMEWORK_PANEL_TAB_TOOL_RUN_LABEL = "Tool runs";
export const FRAMEWORK_PANEL_TAB_TOOL_RUN_ICON_ID = "framework.panel.toolRun";
/** ⏯️ Mirrors Rust `FRAMEWORK_TOOL_RUN_BODY_KEY` — the reserved body of the framework ToolRun panel. */
export const FRAMEWORK_TOOL_RUN_BODY_KEY = "framework.body.toolRun";

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



export type WindowStackCorner = "topLeft" | "topRight" | "bottomLeft" | "bottomRight";

export type WindowLayoutWindowNode = {
  readonly kind: "window";
  readonly windowKindId: string;
  readonly title?: string;
  readonly instanceId?: string;
  readonly templateId?: string;
  readonly corner?: WindowStackCorner;
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

//#region 🔌️PluginAndAppContract
//#region PluginRuntime
/** 🧬️ Generated from Rust `ActionKind`/`ActionDefinition` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type ActionKind = GeneratedActionKind;
export type ActionDefinition = GeneratedActionDefinition;
export type ActionAddress = GeneratedActionAddress;
export type ActionInvocation = GeneratedActionInvocation;
export type ActionArgDef = GeneratedActionArgDef;
export type ActionArgControl = GeneratedActionArgControl;
export type ActionArgOption = GeneratedActionArgOption;

/** 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema, D6: the
 * stored, engine-neutral shape of one `ActionArgDef`'s value (see Rust `🔖️ArgSchema`) — the sole
 * persisted truth; `ActionArgControl` above (unchanged) is now DERIVED from it by {@link argControl}. */
export type ArgSchema = GeneratedArgSchema;
export type ArgFormat = GeneratedArgFormat;
export type ArgPresentation = GeneratedArgPresentation;
export type SnapSource = GeneratedSnapSource;
export type NumberScale = GeneratedNumberScale;
export type ReferenceIdType = GeneratedReferenceIdType;

/** 🔢️ The largest integer id magnitude a reference admits — the Rust `REFERENCE_ID_INTEGER_MAX` twin. */
export const REFERENCE_ID_INTEGER_MAX = Number.MAX_SAFE_INTEGER;

/** 🎯️ The payload value of one selected id: the text itself, or the integer it spells — `undefined` for empty text or, for an
 * integer reference, text that is not a decimal integer within ±(2^53 − 1). The Rust `ReferenceIdType::id_value` twin. */
export function referenceIdValue(idType: ReferenceIdType | undefined, id: string): string | number | undefined {
  if (id === "") return undefined;
  if (idType !== "integer") return id;
  if (!/^-?[0-9]+$/u.test(id)) return undefined;
  const value = Number(id);
  return Number.isSafeInteger(value) ? value : undefined;
}

/** 🏷️ The text of one reference id value — a non-empty string id as it is, an integer id in decimal (the spelling a selection
 * carries); `undefined` for anything else. The Rust `reference_id_text` twin. */
export function referenceIdText(value: unknown): string | undefined {
  if (typeof value === "string") return value === "" ? undefined : value;
  return typeof value === "number" && Number.isSafeInteger(value) ? String(value) : undefined;
}

/** 🎯️ Generated from Rust `🔖️ActionSemantics` (`🛂️manifest/🦀️.rs`) — effects/policy/
 * execution + natural-language framing carried on every `ActionDefinition`/`CommandDefinition`. */
export type ResourceSelector = GeneratedResourceSelector;
export type CapabilityEffects = GeneratedCapabilityEffects;
export type ApprovalMode = GeneratedApprovalMode;
export type CapabilityPolicy = GeneratedCapabilityPolicy;
export type PreviewMode = GeneratedPreviewMode;
export type UndoMode = GeneratedUndoMode;
export type IdempotencyMode = GeneratedIdempotencyMode;
export type ExecutionClass = GeneratedExecutionClass;
export type CapabilityExecution = GeneratedCapabilityExecution;
export type ActionSemantics = GeneratedActionSemantics;

//#region 🎯️ActionSemanticsDefaults
/** 🏭️ Mirrors native `ActionSemantics::for_kind`; defaults never constitute an interactive-job migration proof. */
export function actionSemanticsForKind(kind: ActionKind): ActionSemantics {
  const mutation = kind === "mutation";
  const observes = kind === "view" || kind === "interaction";
  return {
    effects: { reads: observes ? ["config:{self}"] : [], writes: mutation ? ["artifact:{self}"] : [], external: false, destructive: false, reversible: mutation },
    policy: { scopes: mutation || kind === "history" ? ["artifacts.write"] : observes ? ["artifacts.read", "shell.observe"] : kind === "clipboard" ? ["shell.clipboard"] : ["shell.navigate"], approval: mutation ? "whenDestructive" : "never" },
    execution: { preview: mutation ? "diff" : "none", undo: { kind: mutation ? "inverse" : "none" }, idempotency: "none", expectedRevision: mutation, cancellable: false, class: "interactive", interactiveJob: "unclassified" },
    useWhen: [], examples: [],
  };
}
//#endregion 🎯️ActionSemanticsDefaults

/** 🎛️ Mirrors Rust `ActionArgDef::control()` exactly (D6): derives the renderer-facing
 * `ActionArgControl` from `def.schema`/`def.presentation` — the ONLY place a TS reader should reach
 * for an argument's widget kind; never reconstructs `ActionArgControl` from `schema` by hand.
 * Priority matches Rust: non-empty `options` always wins Select (or Segmented); a number's presentation
 * wins, else an integer steps, else a fully bounded number slides, else it is a plain number field. */
export function argControl(def: ActionArgDef): ActionArgControl {
  const schema = def.schema;
  switch (schema.kind) {
    case "string": {
      const options = schema.options ?? [];
      if (options.length > 0) return def.presentation?.kind === "segmented" ? { kind: "segmented", options } : { kind: "select", options };
      const format = schema.format;
      if (format?.kind === "iconId") return { kind: "iconSelect", classifierKind: "icon" };
      if (format?.kind === "artifactKind") return { kind: "artifactKind", roles: format.roles };
      if (format?.kind === "surfaceApp") return { kind: "surfaceApp", roles: format.roles, dialectArg: format.dialectArg };
      return { kind: "text" };
    }
    case "number": {
      const facets = { step: schema.step, unit: schema.unit, precision: schema.precision, displayUnit: schema.displayUnit, displayFactor: schema.displayFactor };
      const snapping = { snaps: schema.snaps, snapSource: schema.snapSource };
      const travel = { min: schema.softMin ?? schema.min ?? 0, max: schema.softMax ?? schema.max ?? 0 };
      const presentation = def.presentation?.kind;
      if (presentation === "slider" || (presentation !== "dial" && presentation !== "stepper" && !schema.integer && schema.min !== undefined && schema.max !== undefined)) return { kind: "slider", ...travel, ...facets, ...snapping, scale: schema.scale };
      if (presentation === "dial") return { kind: "dial", ...travel, ...facets, ...snapping };
      if (presentation === "stepper" || schema.integer) return { kind: "stepper", min: schema.min, max: schema.max, ...facets, ...snapping };
      return { kind: "number", min: schema.min, max: schema.max, ...facets };
    }
    case "boolean":
      return { kind: "toggle" };
    case "vector":
      if (def.presentation?.kind === "color") return { kind: "color", alpha: schema.dims === 4 };
      return { kind: "vector", dims: schema.dims, min: schema.min, max: schema.max, unit: schema.unit, step: schema.step, snaps: schema.snaps, snapSource: schema.snapSource, precision: schema.precision, displayUnit: schema.displayUnit, displayFactor: schema.displayFactor };
    case "reference":
      return { kind: "reference", kinds: schema.kinds, domain: schema.domain, granularity: schema.granularity, many: schema.many, minItems: schema.minItems, maxItems: schema.maxItems, ...(schema.idType === "integer" ? { idType: schema.idType } : {}) };
    case "array":
    case "object":
    case "any":
      return { kind: "text" };
  }
}

//#region 🔖️MutationInputs
/** 🧭️ Resolves a cross-document `$ref`: the parsed schema document whose `$id` is `id` (no fragment), or `undefined`. */
export type InputSchemaResolver = (id: string) => unknown;

/** 🚫️ The class of an {@link InputSchemaError} — the Rust `InputSchemaErrorCode` twin. */
export type InputSchemaErrorCode = "malformed" | "refUnresolved" | "uiInvalid" | "widgetIncompatible" | "labelMissing" | "optionLabelMissing" | "localeMissing";

/** 🚫️ Why a mutation payload schema yields no input descriptors: `code` and the RFC 6901 `pointer` of the input in the payload. */
export class InputSchemaError extends Error {
  constructor(
    readonly code: InputSchemaErrorCode,
    readonly pointer: string,
    detail: string,
  ) {
    super(`${code} at ${JSON.stringify(pointer)}: ${detail}`);
  }
}

type Json = null | boolean | number | string | Json[] | { [key: string]: Json };
type JsonObject = { [key: string]: Json };
type ResolvedInput = { readonly document: string | null; readonly node: JsonObject; readonly ui: ReadonlyMap<string, Json>; readonly refs: readonly string[]; readonly nullable: boolean };

const INPUT_UI_KEYS = new Set(["widget", "role", "label", "description", "step", "precision", "softMin", "softMax", "snaps", "snapSource", "unit", "displayUnit", "displayFactor", "scale", "group", "order", "options", "ref"]);
const INPUT_UI_NUMBER_KEYS = new Set(["step", "precision", "softMin", "softMax", "snaps", "snapSource", "displayUnit", "displayFactor", "scale"]);
/** 🧭️ The number facets a vector shares across its components — the Rust `INPUT_UI_VECTOR_KEYS` twin. */
const INPUT_UI_VECTOR_KEYS = new Set(["step", "precision", "snaps", "snapSource", "displayUnit", "displayFactor"]);
const INPUT_WIDGETS = new Set(["slider", "stepper", "dial", "toggle", "select", "segmented", "text", "multiline", "vector", "color", "reference", "hidden"]);
const INPUT_TYPES = new Set(["string", "integer", "number", "boolean", "object", "array"]);
const INPUT_REF_DEPTH = 32;

const isObject = (value: unknown): value is JsonObject => value !== null && typeof value === "object" && !Array.isArray(value);
const isInteger = (value: unknown): value is number => typeof value === "number" && Number.isInteger(value);
const inputPointer = (parent: string, key: string): string => `${parent}/${key.replaceAll("~", "~0").replaceAll("/", "~1")}`;

/** 🌐️ A `{<locale>: text}` map naming every locale, or a `{<terminology>: {<locale>: text}}` matrix naming every cell — the Rust `input_localized_text` twin. */
function inputLocalizedText(value: Json | undefined): LocalizedLabel | InputSchemaErrorCode {
  if (!isObject(value)) return "uiInvalid";
  const cell = (map: Json | undefined, locale: ShellLocale): string | undefined => (isObject(map) && typeof map[locale] === "string" && map[locale] !== "" ? (map[locale] as string) : undefined);
  const keys = Object.keys(value);
  const matrix = (resolve: (terminology: ShellTerminology, locale: ShellLocale) => string): LocalizedLabel =>
    Object.fromEntries(SHELL_TERMINOLOGIES.map((terminology) => [terminology, Object.fromEntries(SHELL_LOCALES.map((locale) => [locale, resolve(terminology, locale)]))])) as LocalizedLabel;
  if (keys.every(isShellLocale)) {
    if (SHELL_LOCALES.some((locale) => cell(value, locale) === undefined)) return "localeMissing";
    return matrix((_, locale) => cell(value, locale)!);
  }
  if (keys.every((key) => isShellTerminology(key) && isObject(value[key]) && Object.keys(value[key] as JsonObject).every(isShellLocale))) {
    if (!SHELL_TERMINOLOGIES.every((terminology) => SHELL_LOCALES.every((locale) => cell(value[terminology], locale) !== undefined))) return "localeMissing";
    return matrix((terminology, locale) => cell(value[terminology], locale)!);
  }
  return "uiInvalid";
}

let glossary: ReadonlyMap<string, LocalizedLabel> | undefined;

/** 📚️ The framework input-label glossary (`🔣️input-labels.json`), field name → label — the Rust `input_label_glossary` twin. */
export function inputLabelGlossary(): ReadonlyMap<string, LocalizedLabel> {
  glossary ??= new Map(
    Object.entries(inputLabelGlossaryDocument.labels).map(([name, label]) => {
      const localized = inputLocalizedText(label as Json);
      if (typeof localized === "string") throw new Error(`🛂️ glossary label ${name} is ${localized}`);
      return [name, localized];
    }),
  );
  return glossary;
}

/** 🔤️ The single non-null JSON type a schema node declares — the Rust `input_type` twin. */
function inputType(node: JsonObject): string | undefined {
  const type = node.type;
  const named = typeof type === "string" ? [type] : Array.isArray(type) ? type.filter((name): name is string => typeof name === "string" && name !== "null") : node.properties !== undefined ? ["object"] : node.items !== undefined ? ["array"] : Array.isArray(node.enum) && node.enum.length > 0 && node.enum.every((value) => typeof value === "string") ? ["string"] : [];
  return named.length === 1 && INPUT_TYPES.has(named[0]!) ? named[0] : undefined;
}

/** 🎯️ `<kind>Id`/`<kind>_id` (string) or `<kind>Ids`/`<kind>_ids` (array) names a reference to `kind` — the Rust `input_inferred_reference_kind` twin. */
function inputInferredReferenceKind(key: string, many: boolean): string | undefined {
  const suffixes = many ? ["Ids", "_ids"] : ["Id", "_id"];
  const suffix = suffixes.find((candidate) => key.endsWith(candidate));
  if (suffix === undefined) return undefined;
  let stem = key.slice(0, key.length - suffix.length);
  if (stem.startsWith("new")) {
    const rest = stem.slice(3);
    if (rest === "") return undefined;
    if (/^[A-Z]/u.test(rest)) stem = rest;
  }
  if (!/^[A-Za-z][A-Za-z0-9_]*$/u.test(stem)) return undefined;
  return stem[0]!.toLowerCase() + stem.slice(1);
}

/** 🧬️ The TypeScript twin of Rust `mutation_input_defs` (`🛂️manifest/🦀️.rs`): one {@link ActionArgDef} per input of a mutation
 * leaf payload JSON Schema, byte-equal in canonical JSON; throws {@link InputSchemaError}. Fixture: `🧫️fixtures/🧫️mutation-inputs`. */
export function mutationInputDefs(schema: string | unknown, resolver: InputSchemaResolver): ActionArgDef[] {
  return inputSchemaReader(schema, resolver, false).inputs();
}

/** 🧺️ Every finding of a mutation leaf payload schema at once — the collecting twin of the fail-fast {@link mutationInputDefs}
 * (Rust `InputSchemaAudit`): every refused input at every nested pointer is recorded and reading goes on past it, so
 * `findings` names every pointer an author must fix (first occurrence of each distinct finding, in reading order; its first
 * entry is the error the fail-fast reader throws) and `inputs` holds what reads despite them. */
export type InputSchemaAudit = { readonly inputs: ActionArgDef[]; readonly findings: InputSchemaError[] };

/** 🧺️ Reads `schema` in collecting mode — the Rust `mutation_input_audit` twin; see {@link InputSchemaAudit}. */
export function mutationInputAudit(schema: string | unknown, resolver: InputSchemaResolver): InputSchemaAudit {
  let reader: ReturnType<typeof inputSchemaReader>;
  try {
    reader = inputSchemaReader(schema, resolver, true);
  } catch (error) {
    if (!(error instanceof InputSchemaError)) throw error;
    return { inputs: [], findings: [error] };
  }
  return reader.audit();
}

/** 🧾️ The TypeScript twin of Rust `mutation_input_instance`: `payload` (a leaf payload without its aggregate tag) with every root
 * `const` property of the leaf schema spliced in where absent, ready for a JSON Schema validator; throws {@link InputSchemaError}. */
export function mutationInputInstance(schema: string | unknown, resolver: InputSchemaResolver, payload: unknown): Record<string, unknown> {
  return inputSchemaReader(schema, resolver, false).instance(payload);
}

/** 🏷️ The label a collecting read keeps for an input whose own label is refused. */
const INPUT_LABEL_PLACEHOLDER = { native: { en: "", de: "" }, reuse: { en: "", de: "" } } as LocalizedLabel;

function inputSchemaReader(schema: string | unknown, resolver: InputSchemaResolver, collect: boolean): { inputs(): ActionArgDef[]; audit(): InputSchemaAudit; instance(payload: unknown): Record<string, unknown> } {
  let root: Json;
  try {
    root = (typeof schema === "string" ? JSON.parse(schema) : schema) as Json;
  } catch (error) {
    throw new InputSchemaError("malformed", "", String(error));
  }
  const documents = new Map<string, Json>();
  const active: string[] = ["#"];
  const fail = (code: InputSchemaErrorCode, pointer: string, detail: string): never => {
    throw new InputSchemaError(code, pointer, detail);
  };
  const findings: InputSchemaError[] | undefined = collect ? [] : undefined;
  /** 🧺️ The Rust `recover` twin: in collecting mode records a refusal and goes on with `fallback`; fail-fast rethrows. */
  const recover = <T>(attempt: () => T, fallback: () => T): T => {
    if (findings === undefined) return attempt();
    try {
      return attempt();
    } catch (error) {
      if (!(error instanceof InputSchemaError)) throw error;
      findings.push(error);
      return fallback();
    }
  };

  const target = (document: string | null, reference: string, pointer: string): [string | null, Json] => {
    const hash = reference.indexOf("#");
    const id = hash < 0 ? reference : reference.slice(0, hash);
    const fragment = hash < 0 ? "" : reference.slice(hash + 1);
    const owner = id === "" ? document : id;
    if (owner !== null && !documents.has(owner)) {
      const fetched = resolver(owner);
      if (fetched === undefined || fetched === null) fail("refUnresolved", pointer, `no schema document has $id ${owner}`);
      documents.set(owner, fetched as Json);
    }
    let current: Json | undefined = owner === null ? root : documents.get(owner);
    for (const segment of fragment.split("/").slice(1).map((part) => part.replaceAll("~1", "/").replaceAll("~0", "~"))) {
      current = isObject(current) ? current[segment] : Array.isArray(current) && /^\d+$/u.test(segment) ? current[Number(segment)] : undefined;
    }
    return current === undefined ? fail("refUnresolved", pointer, `${reference} lands on nothing`) : [owner, current];
  };

  const resolve = (document: string | null, start: Json, pointer: string): ResolvedInput => {
    let node = start;
    const ui = new Map<string, Json>();
    const refs: string[] = [];
    let nullable = false;
    for (let hop = 0; hop < INPUT_REF_DEPTH; hop += 1) {
      if (!isObject(node)) return { document, node: {}, ui, refs, nullable };
      const annotation = node["x-semio-ui"];
      if (annotation !== undefined) {
        const entries = recover((): [string, Json][] => (isObject(annotation) ? Object.entries(annotation as JsonObject) : fail("uiInvalid", pointer, "x-semio-ui is an object")), () => []);
        for (const [key, value] of entries) {
          if (!INPUT_UI_KEYS.has(key)) {
            recover(() => fail("uiInvalid", pointer, `x-semio-ui carries the undeclared key ${key}`), () => undefined);
            continue;
          }
          if (!ui.has(key)) ui.set(key, value);
        }
      }
      if (typeof node.$ref === "string") {
        const reference = node.$ref;
        [document, node] = target(document, reference, pointer);
        refs.push(`${document ?? ""}#${reference.includes("#") ? reference.slice(reference.indexOf("#") + 1) : ""}`);
        continue;
      }
      const union = Array.isArray(node.oneOf) ? node.oneOf : Array.isArray(node.anyOf) ? node.anyOf : undefined;
      if (union !== undefined) {
        const concrete = union.filter((branch) => !(isObject(branch) && branch.type === "null" && Object.keys(branch).length === 1));
        if (concrete.length === 1 && concrete.length < union.length) {
          node = concrete[0]!;
          nullable = true;
          continue;
        }
      }
      return { document, node, ui, refs, nullable: nullable || (Array.isArray(node.type) && node.type.includes("null")) };
    }
    return fail("malformed", pointer, "a $ref chain exceeds 32 hops");
  };

  const uiString = (input: ResolvedInput, key: string, pointer: string): string | undefined => {
    const value = input.ui.get(key);
    if (value === undefined) return undefined;
    return typeof value === "string" && value !== "" ? value : fail("uiInvalid", pointer, `${key} is a non-empty string`);
  };
  const uiFinite = (input: ResolvedInput, key: string, pointer: string): number | undefined => {
    const value = input.ui.get(key);
    if (value === undefined) return undefined;
    return typeof value === "number" && Number.isFinite(value) ? value : fail("uiInvalid", pointer, `${key} is a finite number`);
  };
  const uiPositive = (input: ResolvedInput, key: string, pointer: string): number | undefined => {
    const value = uiFinite(input, key, pointer);
    return value !== undefined && value <= 0 ? fail("uiInvalid", pointer, `${key} is positive`) : value;
  };
  const count = (node: JsonObject, key: string): number | undefined => (isInteger(node[key]) && (node[key] as number) >= 0 ? (node[key] as number) : undefined);

  const options = (input: ResolvedInput, pointer: string): ActionArgOption[] => {
    const values = input.node.enum;
    if (!Array.isArray(values)) return [];
    const labels = input.ui.get("options");
    if (labels !== undefined && (!isObject(labels) || Object.keys(labels).some((value) => !values.includes(value)))) fail("uiInvalid", pointer, "options labels only declared enum values");
    return values.map((value) => {
      if (typeof value !== "string") return fail("malformed", pointer, "a labelled enum lists strings");
      const declared = isObject(labels) ? labels[value] : undefined;
      if (declared !== undefined) {
        const label = inputLocalizedText(declared);
        return typeof label === "string" ? fail(label, pointer, `option ${value} names every locale`) : { value, label };
      }
      const label = inputLabelGlossary().get(value);
      return label === undefined ? fail("optionLabelMissing", pointer, `option ${value} has no label`) : { value, label };
    });
  };

  const number = (input: ResolvedInput, integer: boolean, pointer: string): ArgSchema => {
    const keyword = (name: string): number | undefined => (typeof input.node[name] === "number" ? (input.node[name] as number) : undefined);
    const bound = (inclusive: number | undefined, exclusive: number | undefined, lower: boolean): [number | undefined, boolean] => {
      if (exclusive === undefined) return [inclusive, false];
      if (inclusive !== undefined && ((lower && inclusive > exclusive) || (!lower && inclusive < exclusive))) return [inclusive, false];
      if (integer) return [lower ? Math.floor(exclusive) + 1 : Math.ceil(exclusive) - 1, false];
      return [exclusive, true];
    };
    const [min, minExclusive] = bound(keyword("minimum"), keyword("exclusiveMinimum"), true);
    const [max, maxExclusive] = bound(keyword("maximum"), keyword("exclusiveMaximum"), false);
    const inside = (value: number): boolean => (min === undefined || (minExclusive ? value > min : value >= min)) && (max === undefined || (maxExclusive ? value < max : value <= max));
    const rawSnaps = input.ui.get("snaps");
    if (rawSnaps !== undefined && !Array.isArray(rawSnaps)) fail("uiInvalid", pointer, "snaps is an array of numbers");
    const snaps = (rawSnaps ?? []) as Json[];
    if (!snaps.every((snap) => typeof snap === "number" && Number.isFinite(snap))) fail("uiInvalid", pointer, "snaps are finite numbers");
    const outside = (snaps as number[]).find((snap) => !inside(snap));
    if (outside !== undefined) fail("uiInvalid", pointer, `snap ${outside} lies outside the hard bounds`);
    const source = input.ui.get("snapSource");
    let snapSource: SnapSource | undefined;
    if (source !== undefined) {
      const entries = isObject(source) ? Object.entries(source) : [];
      const [name, value] = entries.length === 1 ? entries[0]! : ["", null];
      if (name === "step" && value === true) snapSource = { kind: "step" };
      else if (name === "config" && typeof value === "string" && value !== "") snapSource = { kind: "config", key: value };
      else if (name === "snapshot" && typeof value === "string" && (value === "" || value.startsWith("/"))) snapSource = { kind: "snapshot", pointer: value };
      else fail("uiInvalid", pointer, "snapSource is {step: true}, {config: key} or {snapshot: pointer}");
    }
    const softMin = uiFinite(input, "softMin", pointer);
    const softMax = uiFinite(input, "softMax", pointer);
    if ((softMin !== undefined && !inside(softMin)) || (softMax !== undefined && !inside(softMax)) || (softMin !== undefined && softMax !== undefined && softMin >= softMax)) fail("uiInvalid", pointer, "softMin < softMax lie inside the hard bounds");
    const rawScale = input.ui.get("scale");
    if (rawScale !== undefined && rawScale !== "linear" && rawScale !== "log") fail("uiInvalid", pointer, "scale is linear or log");
    const scale = rawScale as NumberScale | undefined;
    const low = softMin ?? min;
    if (scale === "log" && !(low !== undefined && low > 0)) fail("uiInvalid", pointer, "a log scale starts at a positive softMin or minimum");
    const rawPrecision = input.ui.get("precision");
    if (rawPrecision !== undefined && !(isInteger(rawPrecision) && rawPrecision >= 0 && rawPrecision <= 15)) fail("uiInvalid", pointer, "precision is an integer 0..=15");
    const displayFactor = uiFinite(input, "displayFactor", pointer);
    if (displayFactor === 0) fail("uiInvalid", pointer, "displayFactor is non-zero");
    const step = uiPositive(input, "step", pointer) ?? (integer ? 1 : undefined);
    const unit = uiString(input, "unit", pointer);
    const displayUnit = uiString(input, "displayUnit", pointer);
    return {
      kind: "number",
      ...(min === undefined ? {} : { min }),
      ...(minExclusive ? { minExclusive } : {}),
      ...(max === undefined ? {} : { max }),
      ...(maxExclusive ? { maxExclusive } : {}),
      ...(step === undefined ? {} : { step }),
      integer,
      ...(unit === undefined ? {} : { unit }),
      ...(snaps.length === 0 ? {} : { snaps: snaps as number[] }),
      ...(snapSource === undefined ? {} : { snapSource }),
      ...(softMin === undefined ? {} : { softMin }),
      ...(softMax === undefined ? {} : { softMax }),
      ...(rawPrecision === undefined ? {} : { precision: rawPrecision as number }),
      ...(displayUnit === undefined ? {} : { displayUnit }),
      ...(displayFactor === undefined ? {} : { displayFactor }),
      ...(scale === undefined ? {} : { scale }),
    };
  };

  const reference = (key: string, input: ResolvedInput, many: boolean, inferredKind: string | undefined, idType: ReferenceIdType, pointer: string): ArgSchema => {
    const declared = input.ui.get("ref");
    if (declared !== undefined && (!isObject(declared) || Object.keys(declared).some((name) => !["kind", "domain", "granularity"].includes(name)))) fail("uiInvalid", pointer, "ref is {kind, domain?, granularity?}");
    const kind = isObject(declared) ? declared.kind : undefined;
    let kinds: string[];
    if (typeof kind === "string" && kind !== "") kinds = [kind];
    else if (Array.isArray(kind) && kind.length > 0 && kind.every((entry) => typeof entry === "string" && entry !== "")) kinds = kind as string[];
    else if (kind !== undefined) return fail("uiInvalid", pointer, "ref.kind is a non-empty string or a non-empty array of them");
    else {
      const inferred = inferredKind ?? inputInferredReferenceKind(key, many);
      kinds = inferred === undefined ? fail("uiInvalid", pointer, "a target names its ref.kind") : [inferred];
    }
    const text = (name: string): string | undefined => {
      const value = isObject(declared) ? declared[name] : undefined;
      if (value === undefined) return undefined;
      return typeof value === "string" && value !== "" ? value : fail("uiInvalid", pointer, "ref.domain and ref.granularity are non-empty strings");
    };
    const domain = text("domain");
    const granularity = text("granularity");
    const minItems = many ? count(input.node, "minItems") : undefined;
    const maxItems = many ? count(input.node, "maxItems") : undefined;
    return { kind: "reference", kinds, ...(domain === undefined ? {} : { domain }), ...(granularity === undefined ? {} : { granularity }), ...(many ? { many } : {}), ...(minItems === undefined ? {} : { minItems }), ...(maxItems === undefined ? {} : { maxItems }), ...(idType === "string" ? {} : { idType }) };
  };

  /** ♾️ The Rust `guarded` twin: a `$ref` already being expanded on this path yields a structured value, never an endless tree. */
  const guarded = (node: ResolvedInput, expand: () => ArgSchema): ArgSchema => {
    if (node.refs.some((key) => active.includes(key))) return { kind: "any" };
    const mark = active.length;
    active.push(...node.refs);
    try {
      return expand();
    } finally {
      active.length = mark;
    }
  };

  const itemSchema = (items: ResolvedInput, pointer: string): ArgSchema => guarded(items, () => itemSchemaExpanded(items, pointer));

  const itemSchemaExpanded = (items: ResolvedInput, pointer: string): ArgSchema => {
    if (items.node.const !== undefined) return { kind: "any" };
    switch (inputType(items.node)) {
      case "string": {
        const choices = options(items, pointer);
        return (choices.length === 0 ? { kind: "string" } : { kind: "string", options: choices }) as ArgSchema;
      }
      case "integer":
        return number(items, true, pointer);
      case "number":
        return number(items, false, pointer);
      case "boolean":
        return { kind: "boolean" };
      case "object":
        return items.node.properties !== undefined || items.node.allOf !== undefined ? { kind: "object", fields: fields(items, pointer) } : { kind: "any" };
      case "array": {
        const inner = items.node.items !== undefined ? itemSchema(resolve(items.document, items.node.items, `${pointer}/-`), `${pointer}/-`) : ({ kind: "any" } as ArgSchema);
        const minItems = count(items.node, "minItems");
        const maxItems = count(items.node, "maxItems");
        return { kind: "array", items: inner, ...(minItems === undefined ? {} : { minItems }), ...(maxItems === undefined ? {} : { maxItems }) };
      }
      default:
        return { kind: "any" };
    }
  };

  const valueSchema = (key: string, input: ResolvedInput, role: string | undefined, widget: string | undefined, pointer: string): ArgSchema => {
    const kind = inputType(input.node);
    if (kind !== "integer" && kind !== "number") {
      const misplaced = [...input.ui.keys()].find((name) => INPUT_UI_NUMBER_KEYS.has(name) && !(kind === "array" && INPUT_UI_VECTOR_KEYS.has(name)));
      if (misplaced !== undefined) recover(() => fail("uiInvalid", pointer, `${misplaced} only applies to a number`), () => undefined);
    }
    if (input.ui.has("unit") && kind !== "integer" && kind !== "number" && kind !== "array") recover(() => fail("uiInvalid", pointer, "unit only applies to a number or a vector"), () => undefined);
    if (widget === "hidden" && (kind === "object" || kind === "array")) return { kind: "any" };
    const many = kind === "array";
    const items = many && input.node.items !== undefined ? recover((): ResolvedInput | undefined => resolve(input.document, input.node.items!, `${pointer}/-`), () => undefined) : undefined;
    const itemKind = items === undefined ? undefined : inputType(items.node);
    const idKind = many ? itemKind : kind;
    const referenceShaped = idKind === "string";
    const explicitReference = input.ui.has("ref") || widget === "reference" || role === "target";
    if (explicitReference && idKind !== "string" && idKind !== "integer") fail("widgetIncompatible", pointer, "a reference is a string or integer id or an array of them");
    const inferredKind = role === undefined && widget === undefined && referenceShaped && input.node.enum === undefined ? inputInferredReferenceKind(key, many) : undefined;
    if (explicitReference || inferredKind !== undefined) {
      const numeric = [...input.ui.keys()].find((name) => name === "unit" || INPUT_UI_NUMBER_KEYS.has(name));
      if (numeric !== undefined) recover(() => fail("uiInvalid", pointer, `${numeric} does not apply to a reference`), () => undefined);
      return reference(key, input, many, inferredKind, idKind === "integer" ? "integer" : "string", pointer);
    }
    if (input.ui.has("options") && input.node.enum === undefined) recover(() => fail("uiInvalid", pointer, "options only label enum values"), () => undefined);
    switch (kind) {
      case "string": {
        const minLen = count(input.node, "minLength");
        const maxLen = count(input.node, "maxLength");
        const pattern = typeof input.node.pattern === "string" ? input.node.pattern : undefined;
        const choices = options(input, pointer);
        return { kind: "string", ...(choices.length === 0 ? {} : { options: choices }), ...(minLen === undefined ? {} : { minLen }), ...(maxLen === undefined ? {} : { maxLen }), ...(pattern === undefined ? {} : { pattern }), ...(input.node.format === "uri" ? { format: { kind: "uri" } } : {}) } as ArgSchema;
      }
      case "integer":
        return number(input, true, pointer);
      case "number":
        return number(input, false, pointer);
      case "boolean":
        return { kind: "boolean" };
      case "object":
        return input.node.properties !== undefined || input.node.allOf !== undefined ? { kind: "object", fields: fields(input, pointer) } : { kind: "any" };
      case "array": {
        const minItems = count(input.node, "minItems");
        const maxItems = count(input.node, "maxItems");
        const numeric = itemKind === "integer" || itemKind === "number";
        const fixed = minItems !== undefined && minItems === maxItems && minItems >= 2 && minItems <= 4 ? minItems : undefined;
        if (widget === "vector" || widget === "color" || (numeric && fixed !== undefined)) {
          if (!numeric || fixed === undefined || items === undefined) return fail("widgetIncompatible", pointer, "a vector is an array of 2 to 4 numbers of fixed length");
          const component = number({ document: items.document, node: items.node, ui: input.ui, refs: [], nullable: items.nullable }, itemKind === "integer", pointer) as Extract<ArgSchema, { kind: "number" }>;
          const unit = uiString(input, "unit", pointer);
          const step = uiPositive(input, "step", pointer);
          const min = component.minExclusive ? undefined : component.min;
          const max = component.maxExclusive ? undefined : component.max;
          return {
            kind: "vector",
            dims: fixed,
            ...(min === undefined ? {} : { min }),
            ...(max === undefined ? {} : { max }),
            ...(unit === undefined ? {} : { unit }),
            ...(step === undefined ? {} : { step }),
            ...(component.snaps === undefined ? {} : { snaps: component.snaps }),
            ...(component.snapSource === undefined ? {} : { snapSource: component.snapSource }),
            ...(component.precision === undefined ? {} : { precision: component.precision }),
            ...(component.displayUnit === undefined ? {} : { displayUnit: component.displayUnit }),
            ...(component.displayFactor === undefined ? {} : { displayFactor: component.displayFactor }),
          };
        }
        const vectorOnly = numeric ? undefined : [...input.ui.keys()].find((name) => INPUT_UI_VECTOR_KEYS.has(name));
        if (vectorOnly !== undefined) recover(() => fail("uiInvalid", pointer, `${vectorOnly} only applies to a number or a vector`), () => undefined);
        const inherited = numeric && items !== undefined ? [...input.ui.entries()].filter(([name]) => (name === "unit" || INPUT_UI_VECTOR_KEYS.has(name)) && !items.ui.has(name)) : [];
        const itemSchemaValue = items === undefined ? ({ kind: "any" } as ArgSchema) : itemSchema(inherited.length === 0 ? items : { ...items, ui: new Map([...items.ui, ...inherited]) }, `${pointer}/-`);
        return { kind: "array", items: itemSchemaValue, ...(minItems === undefined ? {} : { minItems }), ...(maxItems === undefined ? {} : { maxItems }) };
      }
      default:
        return { kind: "any" };
    }
  };

  const input = (key: string, resolved: ResolvedInput, required: boolean, pointer: string): ActionArgDef | undefined => {
    const rawRole = resolved.ui.get("role");
    const role = rawRole !== undefined && !(rawRole === "value" || rawRole === "target" || rawRole === "discriminator") ? recover(() => fail("uiInvalid", pointer, "role is value, target or discriminator"), () => undefined) : (rawRole as string | undefined);
    if (role === "discriminator" || resolved.node.const !== undefined) return undefined;
    const rawWidget = resolved.ui.get("widget");
    const widget = rawWidget !== undefined && !(typeof rawWidget === "string" && INPUT_WIDGETS.has(rawWidget)) ? recover(() => fail("uiInvalid", pointer, "widget is not a declared widget"), () => undefined) : (rawWidget as string | undefined);
    const label = recover((): LocalizedLabel => {
      if (resolved.ui.has("label")) {
        const localized = inputLocalizedText(resolved.ui.get("label"));
        return typeof localized === "string" ? fail(localized, pointer, "label names every locale") : localized;
      }
      const glossaryLabel = inputLabelGlossary().get(key);
      return glossaryLabel === undefined ? fail("labelMissing", pointer, `${key} has no x-semio-ui.label and no glossary label`) : glossaryLabel;
    }, () => INPUT_LABEL_PLACEHOLDER);
    const description = recover((): LocalizedLabel | undefined => {
      if (!resolved.ui.has("description")) return undefined;
      const localized = inputLocalizedText(resolved.ui.get("description"));
      return typeof localized === "string" ? fail(localized, pointer, "description names every locale") : localized;
    }, () => undefined);
    const rawGroup = resolved.ui.get("group");
    const group = rawGroup !== undefined && !(typeof rawGroup === "string" && rawGroup !== "") ? recover(() => fail("uiInvalid", pointer, "group is a non-empty string"), () => undefined) : rawGroup;
    const rawOrder = resolved.ui.get("order");
    const order = rawOrder !== undefined && !isInteger(rawOrder) ? recover(() => fail("uiInvalid", pointer, "order is an integer"), () => undefined) : rawOrder;
    const schema = recover((): ArgSchema | undefined => guarded(resolved, () => valueSchema(key, resolved, role, widget, pointer)), () => undefined);
    if (schema === undefined) {
      return {
        id: inputPointer("", key),
        label,
        schema: { kind: "any" },
        required,
        ...(resolved.nullable ? { nullable: true } : {}),
        ...(description === undefined ? {} : { description }),
        ...(group === undefined ? {} : { group: group as string }),
        ...(order === undefined ? {} : { order: order as number }),
      };
    }
    const compatible =
      widget === undefined ||
      widget === "hidden" ||
      (["slider", "stepper", "dial"].includes(widget) && schema.kind === "number") ||
      (widget === "toggle" && schema.kind === "boolean") ||
      (widget === "vector" && schema.kind === "vector") ||
      (widget === "color" && schema.kind === "vector" && (schema.dims === 3 || schema.dims === 4) && schema.min === 0 && schema.max === 1) ||
      (widget === "reference" && schema.kind === "reference") ||
      ((widget === "select" || widget === "segmented") && schema.kind === "string" && (schema.options ?? []).length > 0) ||
      ((widget === "text" || widget === "multiline") && schema.kind === "string" && (schema.options ?? []).length === 0);
    if (!compatible) recover(() => fail("widgetIncompatible", pointer, `widget ${widget} cannot edit this value`), () => undefined);
    const presentation = widget === "slider" || widget === "stepper" || widget === "dial" || widget === "segmented" || widget === "multiline" || widget === "hidden" || widget === "color" ? ({ kind: widget } as ArgPresentation) : undefined;
    const fallback = resolved.node.default;
    return {
      id: inputPointer("", key),
      label,
      schema,
      ...(presentation === undefined ? {} : { presentation }),
      required,
      ...(resolved.nullable ? { nullable: true } : {}),
      ...(fallback === undefined ? {} : { default: fallback }),
      ...(description === undefined ? {} : { description }),
      ...(group === undefined ? {} : { group: group as string }),
      ...(order === undefined ? {} : { order: order as number }),
    };
  };

  const members = (object: ResolvedInput, pointer: string, depth: number, found: ResolvedInput[]): ResolvedInput[] => {
    if (depth === INPUT_REF_DEPTH) fail("malformed", pointer, "an allOf composition exceeds 32 levels");
    found.push(object);
    for (const member of Array.isArray(object.node.allOf) ? object.node.allOf : []) members(resolve(object.document, member, pointer), pointer, depth + 1, found);
    return found;
  };

  const fields = (object: ResolvedInput, pointer: string): ActionArgDef[] => {
    const composed = members(object, pointer, 0, []);
    const required = composed.flatMap((member) => (Array.isArray(member.node.required) ? member.node.required.filter((name): name is string => typeof name === "string") : []));
    const seen = new Set<string>();
    return composed.flatMap((member) =>
      Object.entries(isObject(member.node.properties) ? member.node.properties : {}).flatMap(([key, node]) => {
        if (seen.has(key)) return [];
        seen.add(key);
        const childPointer = inputPointer(pointer, key);
        const resolved = recover((): ResolvedInput | undefined => resolve(member.document, node, childPointer), () => undefined);
        if (resolved === undefined) return [];
        const field = recover(() => input(key, resolved, required.includes(key), childPointer), () => undefined);
        return field === undefined ? [] : [field];
      }),
    );
  };

  const union = (node: JsonObject): Json[] | undefined => (Array.isArray(node.oneOf) ? node.oneOf : Array.isArray(node.anyOf) ? node.anyOf : undefined);

  type Variant = { readonly value: string; readonly member: ResolvedInput; readonly discriminator: ResolvedInput };
  const variants = (object: ResolvedInput, pointer: string): [string, Variant[]] => {
    const candidates = (union(object.node) ?? []).map((branch) => {
      const member = resolve(object.document, branch, pointer);
      const pins: [string, string | undefined, ResolvedInput][] = [];
      for (const composed of members(member, pointer, 0, [])) {
        for (const [key, property] of Object.entries(isObject(composed.node.properties) ? composed.node.properties : {})) {
          if (pins.some(([name]) => name === key)) continue;
          const resolved = resolve(composed.document, property, inputPointer(pointer, key));
          pins.push([key, typeof resolved.node.const === "string" ? resolved.node.const : undefined, resolved]);
        }
      }
      return { member, pins };
    });
    const pinned = (key: string): string[] | undefined => {
      const values = candidates.map(({ pins }) => pins.find(([name]) => name === key)?.[1]);
      if (values.some((value) => value === undefined)) return undefined;
      return values.every((value, index) => !values.slice(0, index).includes(value)) ? (values as string[]) : undefined;
    };
    const key = candidates[0]?.pins.map(([name]) => name).find((name) => pinned(name) !== undefined);
    if (key === undefined) return fail("malformed", pointer, "a payload union names no property every variant pins to a distinct string const");
    return [key, candidates.map(({ member, pins }) => { const [, value, discriminator] = pins.find(([name]) => name === key)!; return { value: value!, member, discriminator }; })];
  };

  const variantInputs = (object: ResolvedInput, pointer: string): ActionArgDef[] => {
    const [key, found] = variants(object, pointer);
    const selector = inputPointer(pointer, key);
    const discriminator = found[0]!.discriminator;
    const label = recover((): LocalizedLabel => {
      if (discriminator.ui.has("label")) {
        const localized = inputLocalizedText(discriminator.ui.get("label"));
        return typeof localized === "string" ? fail(localized, selector, "label names every locale") : localized;
      }
      const glossaryLabel = inputLabelGlossary().get(key);
      return glossaryLabel === undefined ? fail("labelMissing", selector, `${key} has no x-semio-ui.label and no glossary label`) : glossaryLabel;
    }, () => INPUT_LABEL_PLACEHOLDER);
    const description = recover((): LocalizedLabel | undefined => {
      if (!discriminator.ui.has("description")) return undefined;
      const localized = inputLocalizedText(discriminator.ui.get("description"));
      return typeof localized === "string" ? fail(localized, selector, "description names every locale") : localized;
    }, () => undefined);
    const options: ActionArgOption[] = found.map(({ value, member }) => ({
      value,
      label: recover((): LocalizedLabel => {
        if (member.ui.has("label")) {
          const localized = inputLocalizedText(member.ui.get("label"));
          return typeof localized === "string" ? fail(localized, selector, `variant ${value} names every locale`) : localized;
        }
        const glossaryLabel = inputLabelGlossary().get(value);
        return glossaryLabel === undefined ? fail("optionLabelMissing", selector, `variant ${value} has no x-semio-ui.label and no glossary label`) : glossaryLabel;
      }, () => INPUT_LABEL_PLACEHOLDER),
    }));
    const selectorInput: ActionArgDef = {
      id: inputPointer("", key),
      label,
      schema: { kind: "string", options },
      ...(options.length <= 4 ? { presentation: { kind: "segmented" } as ArgPresentation } : {}),
      required: true,
      ...(description === undefined ? {} : { description }),
    };
    return [selectorInput, ...found.flatMap(({ value, member }) => fields(member, pointer).map((field) => ({ ...field, group: value })))];
  };

  const inputs = (): ActionArgDef[] => {
    const payload = resolve(null, root, "");
    if (payload.node.properties === undefined && payload.node.allOf === undefined) {
      if (union(payload.node) !== undefined) return variantInputs(payload, "");
      const type = inputType(payload.node);
      if (type === "object" || type === undefined) return [];
      return fail("malformed", "", "a mutation payload schema describes an object");
    }
    return fields(payload, "");
  };

  return {
    inputs,
    audit(): InputSchemaAudit {
      let read: ActionArgDef[] = [];
      try {
        read = inputs();
      } catch (error) {
        if (!(error instanceof InputSchemaError)) throw error;
        findings!.push(error);
      }
      const distinct: InputSchemaError[] = [];
      for (const finding of findings!) if (!distinct.some((kept) => kept.code === finding.code && kept.pointer === finding.pointer && kept.message === finding.message)) distinct.push(finding);
      return { inputs: read, findings: distinct };
    },
    instance(payload: unknown): Record<string, unknown> {
      if (!isObject(payload as Json)) return fail("malformed", "", "a mutation payload is an object");
      const entries: Record<string, unknown> = { ...(payload as JsonObject) };
      let target = resolve(null, root, "");
      if (target.node.properties === undefined && target.node.allOf === undefined && union(target.node) !== undefined) {
        const [key, found] = variants(target, "");
        const chosen = entries[key];
        if (typeof chosen !== "string") return fail("malformed", "", `a union payload names its variant in ${key}`);
        target = found.find((variant) => variant.value === chosen)?.member ?? fail("malformed", "", `${chosen} is no variant of this payload union`);
      }
      for (const member of members(target, "", 0, [])) {
        for (const [key, node] of Object.entries(isObject(member.node.properties) ? member.node.properties : {})) {
          if (key in entries) continue;
          const constant = resolve(member.document, node, inputPointer("", key)).node.const;
          if (constant !== undefined) entries[key] = constant;
        }
      }
      return entries;
    },
  };
}
//#endregion 🔖️MutationInputs
export type UtilityDefinition = GeneratedUtilityDefinition;
export type UtilityRef = GeneratedUtilityRef;

/** 🕹️ Generated from Rust `InteractionDefinition` family (`🕹️interaction/🦀️.rs`) — see
 * `js/generated/manifest.ts`. Mirrors `ActionDefinition`/`ActionRef`'s import shape above. */
export type InteractionDefinition = GeneratedInteractionDefinition;
export type GranularityDefinition = GeneratedGranularityDefinition;
export type HierarchyProvider = GeneratedHierarchyProvider;
export type HoverSpec = GeneratedHoverSpec;
export type SelectionSpec = GeneratedSelectionSpec;
export type SelectionMode = GeneratedSelectionMode;
export type SelectionMethod = GeneratedSelectionMethod;
export type MergeMode = GeneratedMergeMode;
export type InteractionRef = GeneratedInteractionRef;
export type DomainSelection = GeneratedDomainSelection;

/** 🛠️ Generated from Rust `ToolDefinition`/`ToolRef` (`framework/core/rs/lib.rs`) — a mode-level,
 * activatable capability (e.g. puzzle3d fill), distinct from a per-window `UtilityDefinition`. See
 * `js/generated/manifest.ts`. */
export type ToolDefinition = GeneratedToolDefinition;
export type ToolRef = GeneratedToolRef;

/** ⏯️ Generated from Rust `ToolRunDefinition` (`⏯️tool-run/🦀️.rs`, schema `⏯️tool-run/🧬️schema/🔣️.json`) — the static
 * run declaration a `ToolDefinition`/`UtilityDefinition` carries as `run`. */
export type ToolRunDefinition = GeneratedToolRunDefinition;
export type ToolRunStageDefinition = GeneratedToolRunStageDefinition;
export type ToolRunCounterDefinition = GeneratedToolRunCounterDefinition;
export type ToolRunReasonDefinition = GeneratedToolRunReasonDefinition;
export type ToolRunRebasePolicy = GeneratedToolRunRebasePolicy;
export type ToolRunReconfigurePolicy = GeneratedToolRunReconfigurePolicy;
export type ToolRunSettingsReads = GeneratedToolRunSettingsReads;
export type ToolRunTraceKind = GeneratedToolRunTraceKind;
export type ToolRunVerdict = GeneratedToolRunVerdict;
export type JobKindId = GeneratedJobKindId;
export {
  TOOL_RUN_ABORT_ACTION_ID,
  TOOL_RUN_ABORT_CHORD,
  TOOL_RUN_ACTION_IDS,
  TOOL_RUN_ACTIONS,
  TOOL_RUN_DISMISS_ACTION_ID,
  TOOL_RUN_DISMISS_CHORD,
  TOOL_RUN_FINALIZE_ACTION_ID,
  TOOL_RUN_FINALIZE_CHORD,
  TOOL_RUN_PAUSE_ACTION_ID,
  TOOL_RUN_PAUSE_RESUME_CHORD,
  TOOL_RUN_RESUME_ACTION_ID,
  TOOL_RUN_START_ACTION_ID,
  TOOL_RUN_START_CHORD,
  TOOL_RUN_STEP_ACTION_ID,
  TOOL_RUN_STEP_CHORD,
  type ToolRunActionId,
} from "../⏯️tool-run/🟦️.ts";

/** ⏯️ Mirrors Rust `app_declares_tool_run`: the seven reserved tool run actions are injected exactly when a
 * tool or utility declares `run`. */
export function appDeclaresToolRun(app: { readonly tools: readonly { readonly run?: ToolRunDefinition }[]; readonly utilities: readonly { readonly run?: ToolRunDefinition }[] }): boolean {
  return app.tools.some((tool) => tool.run !== undefined) || app.utilities.some((utility) => utility.run !== undefined);
}

/** 🎛️ Generated command ownership, invocation, and platform-aware keybinding contracts. */
export type CommandDefinition = GeneratedCommandDefinition;
export type CommandOwnerAddress = GeneratedCommandOwnerAddress;
export type CommandAddress = GeneratedCommandAddress;
export type CommandInvocation = GeneratedCommandInvocation;
export type OsDefinition = GeneratedOsDefinition;
export type Platform = GeneratedPlatform;
export type PlatformKeybinding = GeneratedPlatformKeybinding;

/** 🧰️ The framework-owned action id apps dispatch to activate a utility — mirrors `SET_ACTIVE_UTILITY_ACTION_ID`. */
export const SET_ACTIVE_UTILITY_ACTION_ID = "setActiveUtility";

/** 🛠️ The framework-owned action id apps dispatch to activate a mode-level tool — mirrors Rust `SET_ACTIVE_TOOL_ACTION_ID`. */
export const SET_ACTIVE_TOOL_ACTION_ID = "setActiveTool";

/** 🕹️ The six framework-owned Interaction action ids (`interaction_action_definitions`), auto-injected
 * into any app that declares at least one `InteractionDefinition` — mirrors `HISTORY_ACTION_IDS`.
 * `interactionSelect`/`interactionHover` are raw dispatch verbs renderers translate clicks/marquee/
 * hover into (never in the palette); the rest are user-facing and drive the per-domain Select controls. */
export const INTERACTION_SELECT_ACTION_ID = "interactionSelect";
export const INTERACTION_HOVER_ACTION_ID = "interactionHover";
export const CLEAR_SELECTION_ACTION_ID = "clearSelection";
export const SELECT_ALL_ACTION_ID = "selectAll";
export const SET_SELECTION_MODE_ACTION_ID = "setSelectionMode";
export const SET_INTERACTION_GRANULARITY_ACTION_ID = "setInteractionGranularity";

/** 🎓️ The framework-owned action id apps dispatch (or the shell auto-injects into the command palette)
 * to (re)start an app's introduction — mirrors Rust `START_INTRODUCTION_ACTION_ID`. */
export const START_INTRODUCTION_ACTION_ID = "startIntroduction";

/** 📤️ The framework-owned Export Document action id, shell-intercepted in every app — mirrors Rust
 * `EXPORT_ARTIFACT_DOCUMENT_ACTION_ID`. */
export const EXPORT_ARTIFACT_DOCUMENT_ACTION_ID = "exportArtifactDocument";

/** 📥️ The framework-owned Import Document action id, shell-intercepted in every app — mirrors Rust
 * `IMPORT_ARTIFACT_DOCUMENT_ACTION_ID`. */
export const IMPORT_ARTIFACT_DOCUMENT_ACTION_ID = "importArtifactDocument";

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

/** 🔀️ Generated from Rust `DialogChoice` — one decision button of a {@link DialogDefinition}. */
export type DialogChoice = GeneratedDialogChoice;

/** 🔀️ The arg key a {@link DialogChoice} dispatch carries its own id under — mirrors Rust `DIALOG_CHOICE_ARG`. */
export const DIALOG_CHOICE_ARG = "choice";

/** 📤️ The dispatch args of `choice`: the seed context of `effective` (keys no arg of `defs` declares), the args the choice requires, and {@link DIALOG_CHOICE_ARG} naming it — the twin of Rust `DialogChoice::dispatch_args`. */
export function dialogChoiceArgs(choice: DialogChoice, defs: readonly ActionArgDef[], effective: Readonly<Record<string, unknown>>): Record<string, unknown> {
  const requires = choice.requires ?? [];
  const kept = Object.entries(effective).filter(([key]) => key !== DIALOG_CHOICE_ARG && (requires.includes(key) || !defs.some((def) => def.id === key)));
  return { ...Object.fromEntries(kept), [DIALOG_CHOICE_ARG]: choice.id };
}

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

// 🔢️ `at`/`durationMs`/`sourceOffsetMs`/`size` are Rust `u64`, which the owned schema exporter
// projects as `bigint` (see `AssetDeclaration.sizeBytes`/`QuotaSchema.*` above) — every consumer in
// `ui/js/react`'s tutorial player/recorder (`createTutorialClock`, `tutorialCameraAt`, `composeTutorialUi`,
// the `TutorialBar`/`TutorialChapterMarker` chrome) treats timeline offsets as plain `number`
// millisecond values, so every such field is narrowed back with `Omit<Generated*, ...> & {...}` here —
// the identical pattern `AppDefinition` already uses to narrow `iconId`/`defaultLayout`/`namedLayouts`
// below. `title`/`body`/`description`/`text` (Rust `LocalizedLabel`) are NOT narrowed: they stay the
// generated `unknown`, exactly like `AppDefinition.label` — every consumer already resolves them via
// `resolveManifestLabel(label: unknown, …)`.
export type TutorialChapter = Omit<GeneratedTutorialChapter, "at"> & { readonly at: number };

/** 📦️ `Blob.size` is the sole `u64` field — narrowed to `number` like every other timeline offset above. */
export type TutorialAssetSrc = Exclude<GeneratedTutorialAssetSrc, { readonly kind: "blob" }> | (Omit<Extract<GeneratedTutorialAssetSrc, { readonly kind: "blob" }>, "size"> & { readonly size: number });

export type TutorialCaption = Omit<GeneratedTutorialCaption, "at" | "durationMs"> & { readonly at: number; readonly durationMs: number };

export type TutorialNarrationCue = Omit<GeneratedTutorialNarrationCue, "at" | "durationMs" | "audio" | "captions"> & {
  readonly at: number;
  readonly durationMs: number;
  readonly audio?: TutorialAssetSrc;
  readonly captions: readonly TutorialCaption[];
};

export type TutorialOverlayRect = GeneratedTutorialOverlayRect;

export type TutorialVideoCue = Omit<GeneratedTutorialVideoCue, "at" | "durationMs" | "sourceOffsetMs" | "src"> & {
  readonly at: number;
  readonly durationMs: number;
  readonly sourceOffsetMs: number;
  readonly src: TutorialAssetSrc;
};

export type TutorialEventKind = GeneratedTutorialEventKind;

export type TutorialEvent = Omit<GeneratedTutorialEvent, "at"> & { readonly at: number };

/** 🧮️ Renderer-neutral restore point for chrome/UI state — see the Rust doc comment on
 * `TutorialUiSnapshot` for why this is deliberately NOT a serialization of `ShellState`. `layout` is
 * narrowed to this file's own hand-refined `WindowLayout` — same reason and same override
 * (`applyFrameworkLayoutSeed` below requires the narrower `"row" | "column" | "stack" | "window"`
 * literal `kind`) as `AppDefinition.defaultLayout`. */
export type TutorialUiSnapshot = Omit<GeneratedTutorialUiSnapshot, "layout"> & { readonly layout?: WindowLayout };

export type TutorialUiChange = Exclude<GeneratedTutorialUiChange, { readonly kind: "layout" }> | { readonly kind: "layout"; readonly layout: WindowLayout };

export type TutorialUiSample = { readonly kind: "snapshot"; readonly state: TutorialUiSnapshot } | { readonly kind: "delta"; readonly changes: readonly TutorialUiChange[] };

export type TutorialUiKeyframe = Omit<GeneratedTutorialUiKeyframe, "at" | "sample"> & { readonly at: number; readonly sample: TutorialUiSample };

/** 🖋️ Mirrors `store::ArtifactCommand` with `Mutation = unknown` (opaque per-app mutation JSON) — the
 * SOLE source of document mutation during playback; `TutorialEvent`s are annotational only. */
export type TutorialDocumentEventKind = GeneratedTutorialDocumentEventKind;

export type TutorialDocumentEvent = Omit<GeneratedTutorialDocumentEvent, "at"> & { readonly at: number };

export type TutorialCameraState = GeneratedTutorialCameraState;

export type TutorialEasing = GeneratedTutorialEasing;

export type TutorialCameraKeyframe = Omit<GeneratedTutorialCameraKeyframe, "at"> & { readonly at: number };

/** 👻️ Reuses the introduction demonstration vocabulary verbatim — see `IntroductionGesture`/`IntroductionPoint`. */
export type TutorialGestureCue = Omit<GeneratedTutorialGestureCue, "at" | "durationMs"> & { readonly at: number; readonly durationMs: number };

export type TutorialTracks = {
  readonly narration: readonly TutorialNarrationCue[];
  readonly video: readonly TutorialVideoCue[];
  readonly events: readonly TutorialEvent[];
  readonly ui: readonly TutorialUiKeyframe[];
  readonly document: readonly TutorialDocumentEvent[];
  readonly camera: readonly TutorialCameraKeyframe[];
  readonly gestures: readonly TutorialGestureCue[];
};

export type TutorialBase = Omit<GeneratedTutorialBase, "ui" | "cameras"> & {
  readonly ui: TutorialUiSnapshot;
  readonly cameras: readonly TutorialCameraKeyframe[];
};

/** 🎬️ A recorded, timed, replayable walkthrough — the timeline sibling of `IntroductionDefinition`. A
 * *recording* IS a `TutorialDefinition`; the recorder simply produces a densely-sampled one. */
export type TutorialDefinition = Omit<GeneratedTutorialDefinition, "durationMs" | "chapters" | "base" | "tracks"> & {
  readonly durationMs: number;
  readonly chapters: readonly TutorialChapter[];
  readonly base: TutorialBase;
  readonly tracks: TutorialTracks;
};
//#endregion 🎬️Tutorial

//#region 🏷️ShellBrand
// 🌐️ ShellLocale/ShellTerminology are generated from 🖱️ui/🎚️axes/🔣️.json (the same source of
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

/** 🏛️ Localized partner credits contributed by a shell owner. */
export type ShellFooterItem = {
  readonly id: string;
  readonly placement: "leading" | "trailing";
  readonly caption: Readonly<Record<ShellLocale, string>>;
  readonly separator?: Readonly<Record<ShellLocale, string>>;
  readonly logos: readonly { readonly href: string; readonly src: string; readonly darkSrc?: string; readonly alt: string }[];
};

/** 🏷️ Boot-time branding for a standalone shell artifact — identity (window title, logo mark, favicon), locked and defaulted shell preferences, and an optional brand-owned {@link IntroductionDefinition} replacing the app's own (already localized, rendered verbatim). */
export type ShellBrand = {
  readonly id: string;
  readonly windowTitle: string;
  readonly footerItems?: readonly ShellFooterItem[];
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
  /** 🌐️ Custom domain this brand's static build deploys to (e.g. GitHub Pages) — written verbatim into a `CNAME` file at the build root. */
  readonly cnameHost?: string;
};
//#endregion 🏷️ShellBrand

/** 🕹️ Mirrors `semio_framework_core::history_action_definitions` — the six framework-owned
 * History actions every editor receives, used by the shell to render the same set without a wasm round trip. A viewer
 * receives none of them: `switchAlternative` and `checkoutCheckpoint` commit a shared `Checkout` transition, and its
 * manifest never declares a verb its guard rejects. */
export const HISTORY_ACTION_IDS = ["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint"] as const;
/** 🪪️ `switchAlternative`'s alternative id argument — mirrors Rust `SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID`. */
export const SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID = "alternativeId";

/** 📨️ The host-forwarded window fact `hostEvent{windowId, kind}` — mirrors Rust `HOST_EVENT_ACTION_ID`: a pane's focus loss,
 * lost pointer capture or closing reaches its program, which ends an open gesture there without a trace. */
export const HOST_EVENT_ACTION_ID = "hostEvent";
/** 🏷️ The kinds a host forwards — mirror Rust `HOST_EVENT_KIND_BLUR`, `HOST_EVENT_KIND_CAPTURE_LOST`, `HOST_EVENT_KIND_RETIRING`. */
export const HOST_EVENT_KINDS = ["blur", "captureLost", "retiring"] as const;
export type HostEventKind = (typeof HOST_EVENT_KINDS)[number];

/** ✏️ Mirrors Rust `HISTORY_EDIT_ACTION_IDS` — the twelve reserved history-edit verbs, host-driven on the instance's
 * one time-travel session. Every verb but `historyEditBegin`/`historyEditExit` may carry `generation`
 * ({@link HISTORY_EDIT_ARG_GENERATION}); without it the verb addresses the live session. */
export const HISTORY_EDIT_ACTION_IDS = [
  "historyEditBegin",
  "historyEditInput",
  "historyEditUseSelection",
  "historyEditWithdraw",
  "historyEditAccept",
  "historyEditDiscard",
  "historyEditFinalize",
  "historyEditCommit",
  "historyEditBack",
  "historyEditExit",
  "historyEditCancelReplay",
  "historyEditRerun",
] as const;
export type HistoryEditActionId = (typeof HISTORY_EDIT_ACTION_IDS)[number];
/** 🪪️ `historyEditBegin`'s mutation id argument — mirrors Rust `HISTORY_EDIT_ARG_MUTATION_ID`. */
export const HISTORY_EDIT_ARG_MUTATION_ID = "mutationId";
/** 🧩️ `historyEditBegin`'s member store argument (`<slot>/<childId>`), absent for the document's own store — mirrors Rust `HISTORY_EDIT_ARG_STORE`. */
export const HISTORY_EDIT_ARG_STORE = "store";
/** 🧭️ The input pointer argument — mirrors Rust `HISTORY_EDIT_ARG_PATH`. */
export const HISTORY_EDIT_ARG_PATH = "path";
/** 🎚️ `historyEditInput`'s value argument — mirrors Rust `HISTORY_EDIT_ARG_VALUE`. */
export const HISTORY_EDIT_ARG_VALUE = "value";
/** 🧿️ The session generation argument — mirrors Rust `HISTORY_EDIT_ARG_GENERATION`. */
export const HISTORY_EDIT_ARG_GENERATION = "generation";
/** 🏷️ `historyEditCommit`'s alternative name argument — mirrors Rust `HISTORY_EDIT_ARG_NAME`. */
export const HISTORY_EDIT_ARG_NAME = "name";
/** ✍️ The overwrite choice of `historyEditCommit` — mirrors Rust `HISTORY_EDIT_CHOICE_OVERWRITE`. */
export const HISTORY_EDIT_CHOICE_OVERWRITE = "overwrite";
/** 🗳️ The framework-injected finalize dialog id — mirrors Rust `HISTORY_EDIT_FINALIZE_DIALOG_ID`. */
export const HISTORY_EDIT_FINALIZE_DIALOG_ID = "finalizeHistoryEdit";

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
  /** 📌️ Host-owned panel state, opaque to the guest. Registry
   * contributions are not a view-state field: they are installed into the guest by the paged
   * `setContributions` command run (`🛠️ShellHelpers/🧩️contributions/🟦️.ts`), which the guest folds
   * into its own registry, so a refresh crosses a reference-free, bounded context. */
  readonly panelJson?: string;
  /** 🧩️ Parent-owned input for one embedded surface; never persisted in the contributor document. */
  readonly extensionInputJson?: string;
  /** 🪪️ Current authenticated OS session identity for this call. Plugins consume this
   * ephemeral value directly and never persist an app/document copy. */
  readonly sessionIdentity?: Readonly<{ readonly userId: string; readonly displayName: string }>;
  readonly locale?: string;
  readonly terminology?: string;
  /** 🪟️ The window instance a render/action call targets — programs key per-window option state off this, never off `activeWindowKindId`. */
  readonly windowId?: string;
  /** 🎯️ The window instance the user is LOOKING at — the shell's own last-focused pane, sent on every
   * call and deliberately surviving {@link panelViewContext} while it names a live instance. It is never the render target
   * (`windowId` is); it is what lets an app-level panel that authors per-window settings address the
   * pane the user last touched instead of the roster's first entry. */
  readonly focusedWindowId?: string;
  /** 🪟️ The live set of open window instances (base + spawned/split), so `windowMeasures`/`windowEngagements` can return one entry per instance. */
  readonly windowInstances?: readonly { readonly id: string; readonly windowKindId: string }[];
  /** ⏯️ The tool run trace cursor each window instance's renderer echoes, keyed by window instance id: the
   * next trace page it expects of `run` at `generation`. The guest answers the pages after it inside the
   * scene's `toolRunTrace` lane (`📋️tool-run-contract.md` §3.2). */
  readonly toolRunTraceCursorByWindowId?: Readonly<Record<string, ViewToolRunTraceCursor>>;
  /** 🪟️ Every tree container the host holds state for, flattened over all panel bodies — the ONE
   * source of truth for which containers are open and which rows are on screen. A guest materialises
   * exactly the named windows and keeps no expansion state of its own, which is what lets an open
   * container survive a refresh. */
  readonly treeWindows?: readonly ViewTreeWindowRequest[];
  /** 🪟️ Rows the tallest visible panel body fits — the shared first-paint budget a guest spends in
   * document order over containers the host has not yet seen. */
  readonly treeViewportRows?: number;
};

/** 🧭️ View-context form of the tool run `ToolRunTraceCursor` — `run` bounded to exact JavaScript integers. */
export type ViewToolRunTraceCursor = { readonly run: number; readonly generation: number; readonly page: number };

/** 🪟️ One tree container's host-known state: whether the user opened or closed it (absent = the
 * author's own default still stands) and the row window on screen, overscan included. The exact twin
 * of `TreeWindowRequest` (`🛂️manifest/🦀️.rs`). */
export type ViewTreeWindowRequest = { readonly bodyKey: string; readonly nodeKey: string; readonly open?: boolean; readonly offset: number; readonly rows: number };

export type ResolvedPluginViewState = PluginViewState & { readonly locale: "en" | "de"; readonly terminology: "native" | "reuse" };

//#region 📏️PublicInvocationCapacity
/** 📏️ Largest UTF-8 body one structurally addressed action/command JSON invocation may occupy
 * on the way into a plugin process — `🎛️public-invocation/🧬️schema/🔣️.json`'s `maxBodyBytes`, the
 * Rust mirror being `PUBLIC_INVOCATION_BODY_BYTES` (`🛂️manifest/🦀️.rs`). */
export const PUBLIC_INVOCATION_BODY_BYTES = 262_144;

/** 📏️ Largest single JSON string one invocation may carry — `maxStringBytes`, counted as
 * escaped bytes minus their leading backslashes.
 *
 * This is the bound that actually sizes a host→guest push, and NO tool execution contract can widen
 * it: `validate_public_json_envelope` runs before the addressed tool's own `max_raw_wire_bytes` is
 * ever consulted. Any payload larger than this must be paged by its producer — see
 * {@link publicInvocationStringPages}. */
export const PUBLIC_INVOCATION_STRING_BYTES = 4_096;

/** 📏️ Deepest object/array nesting one invocation may reach — `maxDepth`. */
export const PUBLIC_INVOCATION_DEPTH = 64;

/** 📐️ What ONE character of a raw string costs against {@link PUBLIC_INVOCATION_STRING_BYTES}
 * once the JSON encoder has written it — the exact accounting the guest performs, which skips a
 * leading `\` and counts every byte after it. Non-ASCII is charged at its `\uXXXX` escape (five per
 * UTF-16 unit), never at its shorter raw UTF-8 form, so a page cut with this function is admitted
 * whether or not the encoder escapes above U+007F. */
export function publicInvocationCharCost(character: string): number {
  const code = character.codePointAt(0) ?? 0;
  if (character === '"' || character === "\\" || character === "\n" || character === "\r" || character === "\t" || code === 0x08 || code === 0x0c) return 1;
  if (code < 0x20) return 5;
  if (code < 0x80) return 1;
  return (code > 0xffff ? 2 : 1) * 5;
}

/** 📄️ Cuts one oversized string into the page run a public invocation can actually carry —
 * each page filled to, and never past, {@link PUBLIC_INVOCATION_STRING_BYTES} as
 * {@link publicInvocationCharCost} measures it, split only on code-point boundaries.
 *
 * The twin of `public_invocation_string_pages` (`🛂️manifest/🦀️.rs`); the two must cut the same
 * payload identically. An empty input yields one empty page, so a producer always sends at least one
 * addressed page. */
export function publicInvocationStringPages(text: string): readonly string[] {
  const pages: string[] = [];
  let page = "";
  let cost = 0;
  for (const character of text) {
    const next = publicInvocationCharCost(character);
    if (cost + next > PUBLIC_INVOCATION_STRING_BYTES) {
      pages.push(page);
      page = "";
      cost = 0;
    }
    page += character;
    cost += next;
  }
  pages.push(page);
  return pages;
}
//#endregion 📏️PublicInvocationCapacity

//#region 📏️ViewContextCapacity
/** 📏️ `panelJson` capacity in Unicode characters — the TypeScript mirror of the ONE neutral
 * declaration `🪟️view-context/🧬️schema/🔣️.json` (`maxLength`), twinned in Rust by
 * `VIEW_CONTEXT_LONG_STRING_CHARS` (`🛂️manifest/🦀️.rs`) and pinned against the schema by
 * `🧪️tests/🔬️view-context-capacity/🦀️.rs`. */
export const VIEW_CONTEXT_LONG_STRING_CHARS = 65_536;
/** 📏️ Bounded opaque inputs: panel state and one embedded surface's parameters. */
export const VIEW_CONTEXT_LONG_STRING_FIELDS = ["panelJson", "extensionInputJson"] as const;
//#endregion 📏️ViewContextCapacity

/** 🪟️ Admits an explicit host projection before it crosses a process boundary. */
export function parseResolvedPluginViewState(value: unknown): ResolvedPluginViewState {
  const object = (input: unknown): Record<string, unknown> => {
    if (input === null || typeof input !== "object" || Array.isArray(input)) throw new Error("view context: expected object");
    return input as Record<string, unknown>;
  };
  const identifier = (input: unknown): string => {
    if (typeof input !== "string" || input.length === 0 || Array.from(input).length > 256 || /[\u0000-\u001f\u007f]/u.test(input)) throw new Error("view context: invalid identifier");
    return input;
  };
  // 🕳️ An absent optional field crosses the worker wire as `null`, not as `undefined`: `encodePackValue`
  // has no `undefined` in its vocabulary and the Rust twin's `Option<String>` decodes `null` to `None`
  // (`🛂️manifest/🦀️.rs:4691`). Reading `!== undefined` therefore handed `identifier()` a `null` and
  // refused EVERY view context that had no active utility or tool — the ordinary case — so a hub
  // document's browser actor never received one and the opening stalled after its execution-target
  // fetches with `view context: invalid identifier` (ticket 26/09/18 slice C2).
  const row = Object.fromEntries(Object.entries(object(value)).filter(([, item]) => item !== null));
  const short = ["activeModeId", "activeWindowKindId", "activeUtilityId", "activeToolId", "windowId", "focusedWindowId"];
  const long = VIEW_CONTEXT_LONG_STRING_FIELDS;
  const allowed = new Set([...short, ...long, "locale", "terminology", "sessionIdentity", "activeUtilityByWindowId", "windowInstances", "toolRunTraceCursorByWindowId", "treeWindows", "treeViewportRows"]);
  if (Object.keys(row).some((key) => !allowed.has(key)) || !["en", "de"].includes(row.locale as string) || !["native", "reuse"].includes(row.terminology as string)) throw new Error("view context: explicit supported preferences required");
  for (const key of short) if (row[key] !== undefined) identifier(row[key]);
  for (const key of long) if (row[key] !== undefined && (typeof row[key] !== "string" || Array.from(row[key]).length > VIEW_CONTEXT_LONG_STRING_CHARS)) throw new Error(`view context: invalid panel data at ${key}`);
  if (row.sessionIdentity !== undefined) {
    const identity = object(row.sessionIdentity);
    if (Object.keys(identity).sort().join(",") !== "displayName,userId") throw new Error("view context: invalid session identity fields");
    identifier(identity.userId);
    identifier(identity.displayName);
  }
  if (row.activeUtilityByWindowId !== undefined) {
    const entries = Object.entries(object(row.activeUtilityByWindowId));
    if (entries.length > 64) throw new Error("view context: utility capacity exceeded");
    for (const [key, item] of entries) { identifier(key); identifier(item); }
  }
  if (row.windowInstances !== undefined) {
    if (!Array.isArray(row.windowInstances) || row.windowInstances.length > 64) throw new Error("view context: window capacity exceeded");
    const ids = new Set<string>();
    for (const item of row.windowInstances) {
      const window = object(item);
      if (Object.keys(window).sort().join(",") !== "id,windowKindId") throw new Error("view context: invalid window fields");
      const id = identifier(window.id);
      identifier(window.windowKindId);
      if (ids.has(id)) throw new Error("view context: repeated window instance");
      ids.add(id);
    }
  }
  if (row.toolRunTraceCursorByWindowId !== undefined) {
    const entries = Object.entries(object(row.toolRunTraceCursorByWindowId));
    if (entries.length > 64) throw new Error("view context: trace cursor capacity exceeded");
    const bounded = (input: unknown, maximum: number): boolean => Number.isInteger(input) && (input as number) >= 0 && (input as number) <= maximum;
    for (const [key, item] of entries) {
      identifier(key);
      const cursor = object(item);
      if (Object.keys(cursor).sort().join(",") !== "generation,page,run" || !bounded(cursor.run, Number.MAX_SAFE_INTEGER) || !bounded(cursor.generation, 0xffff_ffff) || !bounded(cursor.page, 0xffff_ffff)) throw new Error("view context: invalid tool run trace cursor");
    }
  }
  const boundedInteger = (input: unknown, maximum: number): boolean => Number.isInteger(input) && (input as number) >= 0 && (input as number) <= maximum;
  if (row.treeWindows !== undefined) {
    if (!Array.isArray(row.treeWindows) || row.treeWindows.length > 128) throw new Error("view context: tree window capacity exceeded");
    for (const item of row.treeWindows) {
      const request = object(item);
      const keys = Object.keys(request).sort().join(",");
      if (keys !== "bodyKey,nodeKey,offset,rows" && keys !== "bodyKey,nodeKey,offset,open,rows") throw new Error("view context: invalid tree window fields");
      identifier(request.bodyKey);
      identifier(request.nodeKey);
      if (request.open !== undefined && typeof request.open !== "boolean") throw new Error("view context: invalid tree window open state");
      if (!boundedInteger(request.offset, 0xffff_ffff) || !boundedInteger(request.rows, 0xffff_ffff)) throw new Error("view context: invalid tree window");
    }
  }
  if (row.treeViewportRows !== undefined && !boundedInteger(row.treeViewportRows, 0xffff_ffff)) throw new Error("view context: invalid tree viewport rows");
  return structuredClone(row) as ResolvedPluginViewState;
}

/** 🔢️ Rewrites every EXACT-INTEGER view-context field through `mint`, so a transport whose own
 * number type is an IEEE double still hands the guest the integer carrier its `u64`/`u32` fields
 * decode from. The schema knows WHICH fields are integers; the crossing knows what an integer
 * carrier looks like on ITS wire — this function is the seam between the two, and the ONE place the
 * integer-typed fields of a view context are written down.
 *
 * `toolRunTraceCursorByWindowId.<window>.{run,generation,page}` is the first such field a view
 * context ever carried: `locale`, `terminology`, `windowInstances`, `activeUtilityByWindowId` and
 * `panelJson` are all strings, so no crossing had ever had to carry an integer and every one of them
 * widened it silently. The guest's `FromValue` refuses a widened float by design
 * (`🌱️value/🔁️codec/🦀️.rs`), so the WHOLE view state failed to decode and took every dispatch with
 * it — `toolRunTraceCursorByWindowId.procedural-preview.run.expected an exact u64 integer, found
 * Float(1.0)`, on both renderer doors (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 *
 * Call it on an ALREADY-admitted context ({@link parseResolvedPluginViewState}); it projects, it
 * does not validate. A context that carries no integer field is returned untouched, and `mint`
 * receives whatever the field actually holds — a crossing whose context is ALREADY in its own wire
 * form (the wgpu bridge decodes one straight out of pack) recognises its carriers and passes them
 * through rather than minting them twice. Pinned by
 * `🪟️view-context/🧫️fixtures/🔢️integer-carriers/🔣️.json` and its Rust/TypeScript twins. */
export function viewContextWithIntegerCarriers(view: PluginViewState, mint: (value: unknown) => unknown): Record<string, unknown> {
  const cursors = view.toolRunTraceCursorByWindowId;
  const windows = view.treeWindows;
  const viewportRows = view.treeViewportRows;
  if (cursors === undefined && windows === undefined && viewportRows === undefined) return view as unknown as Record<string, unknown>;
  const projected: Record<string, unknown> = { ...view };
  if (cursors !== undefined) {
    const carried: Record<string, unknown> = {};
    for (const [windowId, cursor] of Object.entries(cursors as Readonly<Record<string, Readonly<Record<string, unknown>>>>)) {
      carried[windowId] = { run: mint(cursor.run), generation: mint(cursor.generation), page: mint(cursor.page) };
    }
    projected.toolRunTraceCursorByWindowId = carried;
  }
  if (windows !== undefined) {
    projected.treeWindows = windows.map((request) => (request.open === undefined ? { bodyKey: request.bodyKey, nodeKey: request.nodeKey, offset: mint(request.offset), rows: mint(request.rows) } : { bodyKey: request.bodyKey, nodeKey: request.nodeKey, open: request.open, offset: mint(request.offset), rows: mint(request.rows) }));
  }
  if (viewportRows !== undefined) projected.treeViewportRows = mint(viewportRows);
  return projected;
}

/** 🎯️ Projects host-owned context onto one concrete window, preserving its own utility selection. */
export function windowViewContext(view: PluginViewState, windowId: string): PluginViewState | undefined {
  const window = view.windowInstances?.find((window) => window.id === windowId);
  if (!window) return undefined;
  const utility = view.activeUtilityByWindowId;
  return { ...view, windowId: window.id, activeWindowKindId: window.windowKindId, activeUtilityId: utility && Object.hasOwn(utility, windowId) ? utility[windowId] : undefined };
}

/** 📌️ Projects app-level panels without binding their controls to a window. `focusedWindowId`
 * deliberately survives: the panel is not RENDERED FOR a window, but a panel that authors per-window
 * settings still has to know which pane the user is looking at, or its controls silently retune the
 * roster's first entry (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12 §5.1 measured exactly that).
 *
 * 🎯️ It survives only while it names a pane the roster actually carries. A shell publishes the new
 * mode's window roster before it refocuses, and the guest's window-config capture faults on a focused
 * pane the roster does not list — BEFORE the panel's body key is matched — so every app panel
 * published nothing at all (`wgpu-ui.surface-not-published:framework.panel.*` on the wgpu shell in
 * generate mode, ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Exact twin of `ViewModel::for_panel`
 * (`🛂️manifest/🦀️.rs`). */
export function panelViewContext(view: PluginViewState): PluginViewState {
  const focused = view.windowInstances?.some((window) => window.id === view.focusedWindowId) ? view.focusedWindowId : undefined;
  return { ...view, windowId: undefined, activeWindowKindId: undefined, activeUtilityId: undefined, focusedWindowId: focused };
}

/** 🛠️ Overlays the host-owned mode tool, then binds the view to a window or the panel.
 * Windowed `handleAction` must use this — {@link windowViewContext} alone keeps a stale/absent
 * session `activeToolId`, so retained tool jobs (`fillBuildTick`) never see the armed tool. */
export function hostArmedViewContext(view: PluginViewState, hostActiveToolId: string | null | undefined, windowId?: string): PluginViewState | undefined {
  const activeToolId = hostActiveToolId ?? undefined;
  const armed = view.activeToolId === activeToolId ? view : { ...view, activeToolId };
  return windowId ? windowViewContext(armed, windowId) : panelViewContext(armed);
}

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

export type ProgramContributionEntry = {
  readonly pluginId: string;
  readonly topicContribution?: TopicContribution;
};

/** 🗂️ Open plugin contribution shape — see Rust `TopicContribution` (`🦀️.rs`) for the full
 * rationale. `topic` reuses the same dot-namespaced vocabulary as a crate's existing
 * `contributes`/`consumes` metadata (e.g. `"flow.extension"`, `"playbook.blockKind"`,
 * `"cad.computer"`); this type does not enumerate topics, each producer/consumer picks its own. */
export type TopicContribution = {
  readonly topic: string;
  readonly payload: unknown;
};

/** 📚️ One authored example document on the manifest — TS twin of Rust `ExampleDefinition`. It carries
 * the DIALECT it was authored for, never one app id: every surface bound to that dialect (the editor
 * and the viewer alike) opens the same fixtures (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export type ManifestExample = {
  readonly id: string;
  readonly label: string;
  readonly documentJson?: string;
  readonly artifactJson?: string;
  readonly dialect: ArtifactDialect;
};

/** 🧹️ Stamps `dialect` onto a descriptor example row that still carries the pre-dialect `appId` field. */
export function dialectFromLegacyExampleAppId(appId: string): ArtifactDialect {
  const separator = appId.lastIndexOf("#");
  if (separator < 0) throw new Error("surface id is missing '#'");
  const role = appId.slice(separator + 1);
  if (role !== "viewer" && role !== "editor") throw new Error("surface id requires viewer or editor role");
  return parseDialectCoordinate(appId.slice(0, separator));
}

/** 🧹️ One manifest example row after admission — drops legacy `appId` once `dialect` is stamped. */
export function normalizeManifestExampleRow<E extends { readonly id: string; readonly dialect?: ArtifactDialect; readonly appId?: string }>(example: E): E {
  if (example.dialect) return example;
  const appId = example.appId;
  if (!appId) return example;
  const { appId: _legacy, ...rest } = example;
  return { ...rest, dialect: dialectFromLegacyExampleAppId(appId) } as E;
}

/** 🧹️ Descriptor admission — every example row the navbar picker reads must carry `dialect`. */
export function normalizeManifestExamples<M extends { readonly examples?: readonly { readonly id: string; readonly dialect?: ArtifactDialect; readonly appId?: string }[] }>(manifest: M): M {
  if (!manifest.examples?.length) return manifest;
  return { ...manifest, examples: manifest.examples.map((example) => normalizeManifestExampleRow(example)) };
}

/** 📚️ THE example-picker predicate — every example authored for `dialect`, in manifest order,
 * deduplicated by id. Twinned by Rust `manifest::examples_for_dialect`, both pinned against
 * `🧫️fixtures/📚️example-picker.json` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function examplesForDialect<E extends { readonly id: string; readonly dialect?: ArtifactDialect }>(examples: readonly E[], dialect: ArtifactDialect): E[] {
  const wanted = dialectCoordinate(dialect);
  const seen = new Set<string>();
  const resolved: E[] = [];
  for (const example of examples) {
    if (!example.dialect || dialectCoordinate(example.dialect) !== wanted || seen.has(example.id)) continue;
    seen.add(example.id);
    resolved.push(example);
  }
  return resolved;
}

/** 🏷️ One asset-name component — everything outside `[0-9A-Za-z._-]` folds to `-`. TS twin of Rust
 * `describe::asset_name_segment`. */
function assetNameSegment(raw: string): string {
  return raw.replace(/[^0-9A-Za-z._-]/g, "-");
}

/** 📦️ The directory-and-stem prefix an externalized example body travels under — TS twin of Rust
 * `manifest::example_body_asset_prefix`. The suffix after it names the BYTES declared: `.json` for a
 * body `describe::externalize_oversized_example_bodies` moved out of the manifest, and the authored
 * fixture's own extension for a body the plugin deferred and never materialised
 * (`ExampleSource::deferred`). Match the prefix, never one fixed extension. */
export function exampleBodyAssetPrefix(example: { readonly id: string; readonly dialect: ArtifactDialect }): string {
  return `📚️examples/${assetNameSegment(example.dialect.artifactKind)}.${assetNameSegment(example.dialect.standard)}.${assetNameSegment(example.dialect.subset)}/${assetNameSegment(example.id)}`;
}

/** 📦️ The `AssetDeclaration.name` an externalized INLINE example body travels under — TS twin of
 * Rust `describe::externalize_oversized_example_bodies`. A descriptor never inlines a body over
 * `DESCRIPTOR_INLINE_EXAMPLE_MAX_BYTES` (256 KiB); it keeps the row and declares the body as this
 * asset instead, so the 4 MiB descriptor bound cannot be breached by an authored document. */
export function exampleBodyAssetName(example: { readonly id: string; readonly dialect: ArtifactDialect }): string {
  return `${exampleBodyAssetPrefix(example)}.json`;
}

/** 📚️ The examples one surface may offer — {@link examplesForDialect} against that surface's own
 * dialect. Role plays no part: an editor and its viewer are two surfaces of ONE dialect and offer
 * exactly the same picker. */
export function examplesForApp<E extends { readonly id: string; readonly dialect?: ArtifactDialect }>(examples: readonly E[], app: { readonly dialect?: ArtifactDialect }): E[] {
  return app.dialect ? examplesForDialect(examples, app.dialect) : [];
}

export type PluginManifest = {
  readonly pluginId: string;
  readonly label: string;
  readonly version: string;
  readonly apps: readonly Record<string, unknown>[];
  readonly workflows: readonly {
    readonly workflowStepId: string;
    readonly appId: string;
    readonly label: string;
    readonly breadcrumb?: readonly string[];
    readonly yields: string;
  }[];
  readonly examples: readonly ManifestExample[];
  /** 🗂️ Open plugin contributions — see `TopicContribution`. */
  readonly topicContributions?: readonly TopicContribution[];
  /** 🎛️ Plugin-scope commands this plugin exposes — apply whenever any of its apps is focused. */
  readonly commands?: readonly CommandDefinition[];
};

//#region 🔁️HostEffectInvocation
/** 🔁️ The scope a host effect's re-dispatch is addressed in — the fields both invocation shapes need,
 * read off whatever view state the host holds. */
export type HostEffectDispatchScope = {
  readonly pluginId: string;
  readonly appId: string;
  readonly modeId: string;
  readonly windowKindId: string;
  readonly windowInstanceId: string;
};

/** 🔁️ The ONE rule that decides which channel a guest's `dispatchAction` host effect re-enters, and
 * the invocation it re-enters with.
 *
 * ⚖️ An id the app DECLARES as a command re-enters the typed command channel; everything else — every
 * framework-reserved verb, every window action — re-enters the scoped ACTION channel. Addressing a
 * reserved verb as an app-owned command is refused by the guest's own ownership gate
 * (`dispatch_command`, `💻️os/🔨️modules/🔌️plugin/🦀️.rs`: `command '<id>' is not owned by app <app>`), and
 * a refused re-arm is a dropped continuation: the wgpu bridge addressed EVERY re-arm as an app
 * command, so `toolRunStart` was refused at boot and the preview evaluation's `ToolRunView` never
 * reached the guest's own surface — leaving `cancellable: false` and no Cancel affordance for the
 * whole of a computation React offers one for at t=5.3 s
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-regressions-sweep-2026-09-15.md`).
 *
 * Fixture: `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🔁️host-effect-invocation/🔣️.json`. */
export function hostEffectInvocationV1(
  scope: HostEffectDispatchScope,
  appCommandIds: readonly string[],
  action: string,
  args?: Record<string, unknown>,
): { readonly kind: "command"; readonly invocation: CommandInvocation } | { readonly kind: "action"; readonly invocation: ActionInvocation } {
  if (appCommandIds.includes(action)) {
    return { kind: "command", invocation: { address: { owner: { app: { pluginId: scope.pluginId, appId: scope.appId } }, commandId: action }, arguments: { ...args } } as CommandInvocation };
  }
  const invocation = {
    address: { pluginId: scope.pluginId, appId: scope.appId, modeId: scope.modeId, windowKindId: scope.windowKindId, windowInstanceId: scope.windowInstanceId, actionId: action },
    arguments: { ...args, windowId: scope.windowInstanceId },
  } as ActionInvocation;
  return { kind: "action", invocation };
}

/** 🔁️ Every command id one app of `manifest` declares — the left-hand side of
 * {@link hostEffectInvocationV1}'s rule, read off the manifest a host already holds. */
export function appCommandIdsV1(manifest: PluginManifest, appId: string): readonly string[] {
  const app = manifest.apps.find((entry) => (entry as { readonly id?: unknown }).id === appId) as { readonly commands?: readonly { readonly id?: unknown }[] } | undefined;
  return (app?.commands ?? []).map((command) => String(command.id ?? "")).filter((id) => id.length > 0);
}
//#endregion 🔁️HostEffectInvocation

//#region 🔖️HostResolvedArgs


/** 🗂️ TS twin of Rust `ArtifactKindChoice` — one artifact-kind choice offered by an
 * `ActionArgControl.artifactKind` dialog field, resolved by the host from its live plugin catalogue
 * (`artifactKindChoices`) into a plain `select` control right before the dialog renders. Round-trips
 * through `ActionArgOption.value` as JSON via `encodeArtifactKindChoice`/`decodeArtifactKindChoice` —
 * the frozen wire shape (contract §C8.1): `{"kindId":"s.draw.draw","schema":"draw.document","dialect":
 * {"artifactKind":"s.draw.draw","standard":"1","subset":"*"},"label":{"en":"Draw","de":"Zeichnung"}}`.
 * Rust twin: `ArtifactKindChoice` (`🦀️.rs`) — both codecs must agree byte-for-byte over the
 * pinned fixtures. */
export type ArtifactKindChoice = {
  readonly kindId: string;
  readonly schema: string;
  readonly dialect: ArtifactDialect;
  readonly label: { readonly en: string; readonly de: string };
};

/** 🎭️ TS twin of Rust `SurfaceAppChoice` — one `(pluginId, appId, role)` choice offered by an
 * `ActionArgControl.surfaceApp` dialog field, resolved by the host against the dialect coordinate
 * found in the dialog's seed argument named `dialectArg`. Round-trips through `ActionArgOption.value`
 * as JSON via `encodeSurfaceAppChoice`/`decodeSurfaceAppChoice`. Rust twin: `SurfaceAppChoice`
 * (`🦀️.rs`). */
export type SurfaceAppChoice = {
  readonly app: AppRef;
  readonly role: AppRole;
};

/** 🧵️ Encodes an `ArtifactKindChoice` into the frozen `ActionArgOption.value` JSON shape — key order
 * (kindId, schema, dialect, label.en, label.de) matches Rust `encode_artifact_kind_choice`'s
 * `serde_json::json!` insertion order byte-for-byte. */
export function encodeArtifactKindChoice(choice: ArtifactKindChoice): string {
  return JSON.stringify({
    kindId: choice.kindId,
    schema: choice.schema,
    dialect: choice.dialect,
    label: { en: choice.label.en, de: choice.label.de },
  });
}

/** 🧵️ Inverse of {@link encodeArtifactKindChoice}. Throws with a message naming the missing/malformed
 * field, mirroring Rust `decode_artifact_kind_choice`'s `Result<_, String>` messages. */
export function decodeArtifactKindChoice(value: string): ArtifactKindChoice {
  const json = JSON.parse(value) as Record<string, unknown>;
  if (typeof json.kindId !== "string") throw new Error("artifact kind choice missing string field kindId");
  if (typeof json.schema !== "string") throw new Error("artifact kind choice missing string field schema");
  const dialect = json.dialect as Partial<ArtifactDialect> | undefined;
  if (typeof dialect?.artifactKind !== "string" || typeof dialect.standard !== "string" || typeof dialect.subset !== "string") {
    throw new Error("artifact kind choice missing field dialect");
  }
  const label = json.label as { readonly en?: unknown; readonly de?: unknown } | undefined;
  if (typeof label?.en !== "string") throw new Error("artifact kind choice missing string field label.en");
  if (typeof label.de !== "string") throw new Error("artifact kind choice missing string field label.de");
  return { kindId: json.kindId, schema: json.schema, dialect: { artifactKind: dialect.artifactKind, standard: dialect.standard, subset: dialect.subset }, label: { en: label.en, de: label.de } };
}

/** 🧵️ Encodes a `SurfaceAppChoice` into its frozen `ActionArgOption.value` JSON shape — must agree
 * byte-for-byte with Rust `encode_surface_app_choice`. */
export function encodeSurfaceAppChoice(choice: SurfaceAppChoice): string {
  return JSON.stringify({ pluginId: choice.app.pluginId, appId: choice.app.appId, role: choice.role });
}

/** 🧵️ Inverse of {@link encodeSurfaceAppChoice}. */
export function decodeSurfaceAppChoice(value: string): SurfaceAppChoice {
  const json = JSON.parse(value) as Record<string, unknown>;
  if (typeof json.pluginId !== "string") throw new Error("surface app choice missing string field pluginId");
  if (typeof json.appId !== "string") throw new Error("surface app choice missing string field appId");
  if (json.role !== "editor" && json.role !== "viewer") throw new Error("surface app choice missing string field role");
  return { app: { pluginId: json.pluginId, appId: json.appId }, role: json.role };
}

/** 🗺️ Resolves an app's manifest `label` field's native (terminology-invariant) cell — the wire shape
 * is `{ native: { en, de }, reuse: { en, de } }` (Rust `LocalizedLabel`'s `Serialize`, see
 * `ShellHelpers/🟦️.tsx`'s `resolveManifestLabel` for the full terminology-aware resolver
 * used at render time). `artifactKindChoices` only ever needs the native cell, matching Rust
 * `encode_artifact_kind_choice` resolving under `Terminology::Native`. Takes `unknown` because
 * `AppDefinition.label` is `unknown` on the generated type (no owned schema mirror for `LocalizedLabel` yet). */
function resolveNativeLabel(label: unknown): { readonly en: string; readonly de: string } {
  const native = (label as { readonly native?: { readonly en?: string; readonly de?: string } } | undefined)?.native;
  return { en: native?.en ?? "", de: native?.de ?? "" };
}

/** 🗂️ Every artifact-kind choice for the given `roles` — TS twin of Rust `artifact_kind_choices`.
 * Every app across `manifests` whose `role` is in `roles` and whose `io.artifactSchema` is non-empty
 * and whose package declares an artifact kind of that schema (on any of its apps — a viewer shares its
 * editor's kind — or on the manifest) contributes one choice per dialect coordinate, labelled with that
 * KIND's own label, never its app's label (every editor app is labelled "Editor"). An app whose schema
 * names no declared kind (a per-user Home, a studio) is not a creatable kind. Deduped by dialect
 * coordinate (first manifest/app wins — callers pass owner manifests first so the owner's label wins
 * over a later contributor's), sorted by coordinate for determinism — the pure resolver behind
 * `ActionArgControl.artifactKind`. */
export function artifactKindChoices(manifests: readonly { readonly apps: readonly unknown[]; readonly artifactKinds?: readonly unknown[] }[], roles: readonly AppRole[]): ArtifactKindChoice[] {
  const byCoordinate = new Map<string, ArtifactKindChoice>();
  for (const manifest of manifests) {
    for (const raw of manifest.apps) {
      const app = raw as unknown as { readonly role: AppRole; readonly dialect: ArtifactDialect; readonly label: unknown; readonly io: { readonly artifactSchema: string }; readonly artifactKinds?: readonly unknown[] };
      if (!roles.includes(app.role) || app.io.artifactSchema === "") continue;
      const coordinate = `${app.dialect.artifactKind}@${app.dialect.standard}/${app.dialect.subset}`;
      if (byCoordinate.has(coordinate)) continue;
      const kinds = [...manifest.apps.flatMap((other) => (other as { readonly artifactKinds?: readonly unknown[] }).artifactKinds ?? []), ...(manifest.artifactKinds ?? [])] as readonly { readonly schema?: string; readonly label?: unknown }[];
      const kind = kinds.find((candidate) => candidate.schema === app.io.artifactSchema);
      if (kind === undefined) continue;
      byCoordinate.set(coordinate, { kindId: app.dialect.artifactKind, schema: app.io.artifactSchema, dialect: app.dialect, label: resolveNativeLabel(kind.label) });
    }
  }
  return [...byCoordinate.keys()].sort().map((coordinate) => byCoordinate.get(coordinate)!);
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️hostresolvedargs/🟦️.ts");
  await registerTests1(import.meta.vitest, { artifactKindChoices, decodeArtifactKindChoice, decodeSurfaceAppChoice, encodeArtifactKindChoice, encodeSurfaceAppChoice }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🔖️HostResolvedArgs

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
export type AppDefinition = Omit<GeneratedAppDefinition, "defaultLayout" | "namedLayouts" | "iconId" | "tutorials"> & {
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
