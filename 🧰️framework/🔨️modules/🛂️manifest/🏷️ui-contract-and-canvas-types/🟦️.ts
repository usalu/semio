/** 🏷️ The semantic UI contract aliases, the framework panel tab ids and labels, the canvas pick and hover types, the named layouts and the utility category, leaf and node trees. */
import type { WindowLayout } from "@semio-tech/framework-onboarding/window-layout";
import type { IconName } from "@semio-tech/assets";
import type { ActionDescriptor as GeneratedActionDescriptor, UiPresence as GeneratedUiPresence, UiState as GeneratedUiState, UiStatus as GeneratedUiStatus } from "../🤖️generated/🪪️manifest/🟦️.ts";
import type { Liveness as GeneratedLiveness, AccessibilitySpec as GeneratedAccessibilitySpec, ActionId as GeneratedActionId, Trigger as GeneratedTrigger, ActionBinding as GeneratedActionBinding, MenuRef as GeneratedMenuRef, UiIntent as GeneratedUiIntent, UiValue as GeneratedUiValue, BuiltNode as GeneratedBuiltNode, Label as GeneratedLabel, ContainerRole as GeneratedContainerRole, InputKind as GeneratedInputKind, RowActionPlacement as GeneratedRowActionPlacement, DropOverlaySpec as GeneratedDropOverlaySpec, SelectItem as GeneratedSelectItem, KeyValueEntry as GeneratedKeyValueEntry, RowAction as GeneratedRowAction, RowTarget as GeneratedRowTarget, ContainerProps as GeneratedContainerProps, TextProps as GeneratedTextProps, ButtonProps as GeneratedButtonProps, SeparatorProps as GeneratedSeparatorProps, InputProps as GeneratedInputProps, SelectProps as GeneratedSelectProps, SelectAppearance as GeneratedSelectAppearance, ToggleProps as GeneratedToggleProps, ToggleAppearance as GeneratedToggleAppearance, KeyValueListProps as GeneratedKeyValueListProps, SliderProps as GeneratedSliderProps, SliderAppearance as GeneratedSliderAppearance, NumberStepperProps as GeneratedNumberStepperProps, RingProps as GeneratedRingProps, IconSelectProps as GeneratedIconSelectProps, ProgressProps as GeneratedProgressProps, TreeProps as GeneratedTreeProps, TreePresentation as GeneratedTreePresentation, TreeWindowRowExtent as GeneratedTreeWindowRowExtent, TreeWindow as GeneratedTreeWindow, TreeSectionProps as GeneratedTreeSectionProps, TreeItemProps as GeneratedTreeItemProps, TableProps as GeneratedTableProps, TableRowProps as GeneratedTableRowProps, ImageProps as GeneratedImageProps, ExtensionProps as GeneratedExtensionProps, Component as GeneratedComponent, SurfaceId as GeneratedSurfaceId, UiNodeId as GeneratedUiNodeId, UiRevision as GeneratedUiRevision, TransitionHint as GeneratedTransitionHint, UiNodeRecord as GeneratedUiNodeRecord, UiSnapshot as GeneratedUiSnapshot, UiPatchOp as GeneratedUiPatchOp, UiPatch as GeneratedUiPatch, SpaceToken as GeneratedSpaceToken, Sizing as GeneratedSizing, Axis as GeneratedAxis, Align as GeneratedAlign, Justify as GeneratedJustify, GridTrack as GeneratedGridTrack, ScrollAxes as GeneratedScrollAxes, Anchor as GeneratedAnchor, EdgeSpace as GeneratedEdgeSpace, StackLayout as GeneratedStackLayout, GridLayout as GeneratedGridLayout, OverlayLayout as GeneratedOverlayLayout, ScrollLayout as GeneratedScrollLayout, AbsoluteLayout as GeneratedAbsoluteLayout, LeafLayout as GeneratedLeafLayout, LayoutSpec as GeneratedLayoutSpec, WindowStackCorner as GeneratedWindowStackCorner, WindowLayoutNode as GeneratedWindowLayoutNode, WindowLayout as GeneratedWindowLayout, UiDocumentLimits as GeneratedUiDocumentLimits, UiNumberBound as GeneratedUiNumberBound, UiNumberLimits as GeneratedUiNumberLimits, UiNumberScale as GeneratedUiNumberScale, UiContractViolation as GeneratedUiContractViolation, PatchRejection as GeneratedPatchRejection, QuotaKind as GeneratedQuotaKind, Activity as GeneratedActivity, PeerMark as GeneratedPeerMark, OwnPresence as GeneratedOwnPresence, PresenceUpdate as GeneratedPresenceUpdate, Variant as GeneratedVariant, SizeToken as GeneratedSizeToken, Density as GeneratedDensity, Tone as GeneratedTone, Emphasis as GeneratedEmphasis, StyleSpec as GeneratedStyleSpec, SurfaceKind as GeneratedSurfaceKind, SurfaceDoc as GeneratedSurfaceDoc, SurfaceProps as GeneratedSurfaceProps } from "../🤖️generated/📜️ui-contract/🟦️.ts";

// #region 🛂️Manifest
// #region 🧬️GeneratedUiContract
/** 🧬️ The semantic UI contract (`🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️.rs`) — flat,
 * id-keyed replacement for the hand-written `UiNode` recursive-union mirror this file used to
 * carry, generated via the owned schema exporter (`repo run @semio-tech/ui-contract-rs:generate`). Five names
 * collide with an unrelated existing export from a different module aggregated into this same
 * barrel (artifact-editor `SurfaceKind`; OS-shell `WindowLayout`/`WindowStackCorner`; the state
 * machine module's own `ActionId`/`Trigger`) and are re-exported `Ui`-prefixed instead of
 * shadowing them; everything else keeps its Rust name verbatim. */
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
export type SelectAppearance = GeneratedSelectAppearance;
export type ToggleProps = GeneratedToggleProps;
export type ToggleAppearance = GeneratedToggleAppearance;
export type KeyValueListProps = GeneratedKeyValueListProps;
export type SliderProps = GeneratedSliderProps;
export type SliderAppearance = GeneratedSliderAppearance;
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
/** ⌨️ Mirrors Rust `FRAMEWORK_HISTORY_BODY_KEY` — the reserved panel body every renderer fetches for the
 * command-history list, and the one surface a completion's history patch dirties on its own. */
export const FRAMEWORK_HISTORY_BODY_KEY = "framework.body.history";
/** ⏯️ Mirrors Rust `FRAMEWORK_PANEL_TAB_TOOL_RUN_ID` — auto-injected into the `panelTabs` of every app that declares a tool run. */
export const FRAMEWORK_PANEL_TAB_TOOL_RUN_ID = "framework.panel.toolRun";
export const FRAMEWORK_PANEL_TAB_TOOL_RUN_LABEL = "Tool runs";
export const FRAMEWORK_PANEL_TAB_TOOL_RUN_ICON_ID = "framework.panel.toolRun";
/** 🔧️ Mirrors Rust `FRAMEWORK_TOOL_RUN_BODY_KEY` — the reserved body of the framework ToolRun panel. */
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

/** 🎬️ Generated from Rust `ActionDescriptor` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type ActionDescriptor = GeneratedActionDescriptor;

export type UiPresence = GeneratedUiPresence;
export type UiState = GeneratedUiState;
export type UiStatus = GeneratedUiStatus;

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
// #endregion 🛂️Manifest
