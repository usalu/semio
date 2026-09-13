import { dialectCoordinate, type ArtifactDialect } from "../🚪️io/🧬️schema/🟦️.ts";
import type { AppRole, AppRef } from "./🧬️schema/🟦️.ts";
export { surfaceAppId, parseSurfaceAppId, type AppRole, type AppRef } from "./🧬️schema/🟦️.ts";
// #region 🛂️Manifest
/// <reference types="vitest/importMeta" />
/** @emoji 🛂️ `@semio-tech/framework` — AppDefinition, PluginManifest, contributions, and declarative UI contract. */
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
  CommandDefinition as GeneratedCommandDefinition,
  CommandOwnerAddress as GeneratedCommandOwnerAddress,
  CommandAddress as GeneratedCommandAddress,
  CommandInvocation as GeneratedCommandInvocation,
  OsDefinition as GeneratedOsDefinition,
  Platform as GeneratedPlatform,
  PlatformKeybinding as GeneratedPlatformKeybinding,
  WindowMeasure as GeneratedWindowMeasure,
  MeasureProgressStep as GeneratedMeasureProgressStep,
  MeasureProgressStepKind as GeneratedMeasureProgressStepKind,
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
  // 🎬️ `//#region 🔖️Tutorial` (`🛂️manifest/🦀️.rs`) — the timeline sibling of
  // `IntroductionDefinition`, typegen-mirrored here exactly like its `Introduction*` neighbors above.
  TutorialArtifactEvent as GeneratedTutorialArtifactEvent,
  TutorialArtifactEventKind as GeneratedTutorialArtifactEventKind,
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
  ContainerProps as GeneratedContainerProps,
  TextProps as GeneratedTextProps,
  ButtonProps as GeneratedButtonProps,
  SeparatorProps as GeneratedSeparatorProps,
  InputProps as GeneratedInputProps,
  SelectProps as GeneratedSelectProps,
  ToggleProps as GeneratedToggleProps,
  KeyValueListProps as GeneratedKeyValueListProps,
  SliderProps as GeneratedSliderProps,
  NumberStepperProps as GeneratedNumberStepperProps,
  RingProps as GeneratedRingProps,
  IconSelectProps as GeneratedIconSelectProps,
  TreeProps as GeneratedTreeProps,
  TreeSectionProps as GeneratedTreeSectionProps,
  TreeItemProps as GeneratedTreeItemProps,
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
export type ContainerProps = GeneratedContainerProps;
export type TextProps = GeneratedTextProps;
export type ButtonProps = GeneratedButtonProps;
export type SeparatorProps = GeneratedSeparatorProps;
export type InputProps = GeneratedInputProps;
export type SelectProps = GeneratedSelectProps;
export type ToggleProps = GeneratedToggleProps;
export type KeyValueListProps = GeneratedKeyValueListProps;
export type SliderProps = GeneratedSliderProps;
export type NumberStepperProps = GeneratedNumberStepperProps;
export type RingProps = GeneratedRingProps;
export type IconSelectProps = GeneratedIconSelectProps;
export type TreeProps = GeneratedTreeProps;
export type TreeSectionProps = GeneratedTreeSectionProps;
export type TreeItemProps = GeneratedTreeItemProps;
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
export const CANVAS_HOVER_SOURCE_ARTIFACT = "document";

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
    policy: { scopes: mutation || kind === "history" ? ["documents.write"] : observes ? ["documents.read", "shell.observe"] : kind === "clipboard" ? ["shell.clipboard"] : ["shell.navigate"], approval: mutation ? "whenDestructive" : "never" },
    execution: { preview: mutation ? "diff" : "none", undo: { kind: mutation ? "inverse" : "none" }, idempotency: "none", expectedRevision: mutation, cancellable: false, class: "interactive", interactiveJob: "unclassified" },
    useWhen: [], examples: [],
  };
}
//#endregion 🎯️ActionSemanticsDefaults

/** 🎛️ Mirrors Rust `ActionArgDef::control()` exactly (D6): derives the renderer-facing
 * `ActionArgControl` from `def.schema`/`def.presentation` — the ONLY place a TS reader should reach
 * for an argument's widget kind; never reconstructs `ActionArgControl` from `schema` by hand.
 * Priority matches Rust: non-empty `options` always wins Select; a `Slider` presentation OR a fully
 * bounded `Number` wins Slider over plain Number; everything else falls through to Text. */
export function argControl(def: ActionArgDef): ActionArgControl {
  // 🛟️ `schema` is the newer stored-truth vocabulary (ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY,
  // packet P3-manifest-schema). Plugin manifests built before that migration — including any stale
  // `plugin-modules/` wasm still on disk — carry an arg def without it. Reading `.kind` off `undefined`
  // there threw inside `resolveActionArgDef` during the shell's first render, which blanked the ENTIRE
  // shell over one un-migrated argument. A missing schema degrades to a plain text control instead.
  const schema = def.schema;
  if (!schema) return { kind: "text", placeholder: undefined };
  switch (schema.kind) {
    case "string": {
      if (schema.options && schema.options.length > 0) {
        return { kind: "select", options: schema.options };
      }
      const format = schema.format;
      if (format?.kind === "iconId") {
        return { kind: "iconSelect", classifierKind: "icon" };
      }
      if (format?.kind === "artifactKind") {
        return { kind: "artifactKind", roles: format.roles };
      }
      if (format?.kind === "surfaceApp") {
        return { kind: "surfaceApp", roles: format.roles, dialectArg: format.dialectArg };
      }
      return { kind: "text", placeholder: undefined };
    }
    case "number": {
      if (def.presentation?.kind === "slider" || (schema.min !== undefined && schema.max !== undefined)) {
        return { kind: "slider", min: schema.min ?? 0, max: schema.max ?? 0, step: schema.step, unit: schema.unit };
      }
      return { kind: "number", min: schema.min, max: schema.max, step: schema.step };
    }
    case "boolean":
      return { kind: "toggle" };
    case "vec3":
      return { kind: "vec3" };
    case "array":
    case "object":
    case "any":
    default:
      return { kind: "text", placeholder: undefined };
  }
}
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
export type TutorialArtifactEventKind = GeneratedTutorialArtifactEventKind;

export type TutorialArtifactEvent = Omit<GeneratedTutorialArtifactEvent, "at"> & { readonly at: number };

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
  readonly document: readonly TutorialArtifactEvent[];
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
  /** 🌐️ Custom domain this brand's static build deploys to (e.g. GitHub Pages) — written verbatim into a `CNAME` file at the build root. */
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
  /** 📌️ Host-owned panel state, opaque to the guest. The ONE long string a view context carries —
   * contributions are NOT a view-state field: they are installed into the guest by the paged
   * `setContributions` command run (`🛠️ShellHelpers/🧩️contributions/🟦️.ts`), which the guest folds
   * into its own registry, so a refresh crosses a reference-free, bounded context. */
  readonly panelJson?: string;
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
};

export type ResolvedPluginViewState = PluginViewState & { readonly locale: "en" | "de"; readonly terminology: "native" | "reuse" };

//#region 📏️PublicInvocationCapacity
/** @emoji 📏️ Largest UTF-8 body one structurally addressed action/command JSON invocation may occupy
 * on the way into a plugin process — `🎛️public-invocation/🧬️schema/🔣️.json`'s `maxBodyBytes`, the
 * Rust mirror being `PUBLIC_INVOCATION_BODY_BYTES` (`🛂️manifest/🦀️.rs`). */
export const PUBLIC_INVOCATION_BODY_BYTES = 262_144;

/** @emoji 📏️ Largest single JSON string one invocation may carry — `maxStringBytes`, counted as
 * escaped bytes minus their leading backslashes.
 *
 * This is the bound that actually sizes a host→guest push, and NO tool execution contract can widen
 * it: `validate_public_json_envelope` runs before the addressed tool's own `max_raw_wire_bytes` is
 * ever consulted. Any payload larger than this must be paged by its producer — see
 * {@link publicInvocationStringPages}. */
export const PUBLIC_INVOCATION_STRING_BYTES = 4_096;

/** @emoji 📏️ Deepest object/array nesting one invocation may reach — `maxDepth`. */
export const PUBLIC_INVOCATION_DEPTH = 64;

/** @emoji 📐️ What ONE character of a raw string costs against {@link PUBLIC_INVOCATION_STRING_BYTES}
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

/** @emoji 📄️ Cuts one oversized string into the page run a public invocation can actually carry —
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
/** 📏️ Long-string fields a view context may carry: `panelJson`, and nothing else. Contributions are
 * installed by the paged `setContributions` run (`🛠️ShellHelpers/🧩️contributions/🟦️.ts`), never
 * carried here — the aggregated closure is 248 635 characters
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export const VIEW_CONTEXT_LONG_STRING_FIELDS = ["panelJson"] as const;
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
  const row = object(value);
  const short = ["activeModeId", "activeWindowKindId", "activeUtilityId", "activeToolId", "windowId", "focusedWindowId"];
  const long = VIEW_CONTEXT_LONG_STRING_FIELDS;
  const allowed = new Set([...short, ...long, "locale", "terminology", "sessionIdentity", "activeUtilityByWindowId", "windowInstances"]);
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
  return structuredClone(row) as ResolvedPluginViewState;
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
 * Every app across `manifests` whose `role` is in `roles` and whose `io.documentSchema` is non-empty
 * contributes one choice per dialect coordinate. Deduped by dialect coordinate (first manifest/app
 * wins — callers pass owner manifests first so the owner's label wins over a later contributor's),
 * sorted by coordinate for determinism — the pure resolver behind `ActionArgControl.artifactKind`. */
export function artifactKindChoices(manifests: readonly { readonly apps: readonly unknown[] }[], roles: readonly AppRole[]): ArtifactKindChoice[] {
  const byCoordinate = new Map<string, ArtifactKindChoice>();
  for (const manifest of manifests) {
    for (const raw of manifest.apps) {
      const app = raw as unknown as { readonly role: AppRole; readonly dialect: ArtifactDialect; readonly label: unknown; readonly io: { readonly documentSchema: string } };
      if (!roles.includes(app.role) || app.io.documentSchema === "") continue;
      const coordinate = `${app.dialect.artifactKind}@${app.dialect.standard}/${app.dialect.subset}`;
      if (byCoordinate.has(coordinate)) continue;
      byCoordinate.set(coordinate, { kindId: app.dialect.artifactKind, schema: app.io.documentSchema, dialect: app.dialect, label: resolveNativeLabel(app.label) });
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
export type MeasureProgressStep = GeneratedMeasureProgressStep;
export type MeasureProgressStepKind = GeneratedMeasureProgressStepKind;
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
  /** 🛍️ App-static operator/palette catalogue — see `ArtifactApp::app_catalogue`. Fetched once per app
   * instance; every later refresh sends the cached hash and gets no payload back. */
  readonly catalogue?: { readonly hash?: string };
  readonly labels?: { readonly hash?: string };
};

/** @emoji 🧩️ The four refresh sections that are NOT authored window/panel bodies. Each is its own
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

/** @emoji 🎯️ Projects host-owned context for a section surface — the FULL view state, unnarrowed:
 * `window_measures`/`window_engagements` iterate `windowInstances` and re-project each instance
 * themselves, and `tool_measures` keys off `activeToolId`, so narrowing here would blind all three. */
export function sectionViewContext(view: PluginViewState): PluginViewState {
  return { ...view };
}

/** @emoji 🐢️ `value` is present only when `hash` differs from what the request supplied — an unchanged section costs one hash compare instead of a full re-serialize. */
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
