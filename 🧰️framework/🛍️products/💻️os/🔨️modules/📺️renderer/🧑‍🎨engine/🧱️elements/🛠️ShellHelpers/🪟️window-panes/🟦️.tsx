/** 🪟️ Window measure controls, selection utility options and the window action pane with its staged-argument controls.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHelpers}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  type ActionArgControl,
  type ActionDefinition,
  type ActionDescriptor,
  type AppDefinition,
  type AppWindowKindDefinition,
  argControl,
  actionArgNumberFacets,
  type ActionArgNumberFacets,
  referenceIdText,
  referenceIdValue,
  type ReferenceIdType,
  type ArtifactKindChoice,
  effectiveActionArgs,
  type MergeMode,
  partitionWindowMeasures,
  type PluginAppLabelsOverlay,
  resolveWindowActions,
  SET_ACTIVE_UTILITY_ACTION_ID,
  SHELL_LOCALES,
  unresolvedActionArgs,
  type UtilityNode,
  type WindowMeasure
} from "@semio-tech/framework";
import {
  childElementId,
  Icon,
  type IconName,
  IconSelector,
  Input,
  RibbonDivider,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Slider,
  Stepper,
  Textarea,
  Toggle,
  ToggleGroup,
  Tree,
  type TreeDataItem,
  type TreeDataSection,
  UI_TERMINOLOGY_NATIVE,
  uiDataLabel,
  type UIDialogFieldBinding,
  useLabel,
  useShellScope,
  WindowMeasuresTree,
  WindowMeasureTreeGroup,
  WindowMeasureTreeLeaf
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { type ReactElement, type ReactNode, useMemo, useState } from "react";
import {
  parseUiColorHex,
  uiColorHex,
  uiNumberCrossedBound,
  uiNumberDisplay,
  uiNumberDisplayText,
  uiNumberFieldKey,
  uiNumberKeyValue,
  uiNumberTypedValue
} from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧩️component/🟦️.ts";
import type { InteractionState } from "@semio-tech/framework/interaction";
import { groupUtilityNodesByCategory, UTILITY_CATEGORIES, UtilityTree } from "../../🎛️UtilityTree/🟦️.tsx";
import { type ActionPaneState, actionStageKey, type ShellAction } from "../../🐚️Shell/🟦️.tsx";
import { WindowMeasureNumber, WindowMeasureSelect, WindowMeasureToggle } from "../🎚️measure-controls/🟦️.tsx";
import {
  actionArgStringOptions,
  currentShellLocaleV1,
  EMPTY_APP_LABELS_OVERLAY,
  resolveActionArgDef,
  resolveAppLabel,
  type ResolvedActionArgDef,
  type ResolvedActionDefinition,
  resolveManifestLabel,
  ribbonParentLabel,
  shellLabel
} from "../🏷️labels/🟦️.tsx";
import { FRAMEWORK_RESERVED_ACTION_IDS } from "../🔩️plumbing/🟦️.ts";
import { createDirectionalAsyncDispatcher } from "../🛑️extension-io/🟦️.ts";

//#region ShellHelpers
/** 🎚️ Whether any measure (including nested group children) declares `id`. */
export function windowMeasureTreeContainsId(measures: readonly WindowMeasure[], id: string): boolean {
  for (const measure of measures) {
    if (measure.id === id) return true;
    if (measure.kind === "group" && windowMeasureTreeContainsId(measure.children, id)) return true;
  }
  return false;
}

/** 📊️ Probability weights (0–1 simplex sliders) read out as whole-percent labels, not raw fractions. */
function windowMeasureUsesProbabilityReadout(measure: Extract<WindowMeasure, { kind: "slider" }>): boolean {
  const step = measure.step ?? 1;
  return measure.min === 0 && measure.max <= 1 && step < 1;
}

function windowMeasureProbabilityReadout(value: number): string {
  return `${Math.round(value * 100)}%`;
}

/** 🎚️ Keeps a measure slider live without accumulating stale document actions behind the pointer. */
function WindowMeasureSlider({ measure, onAction }: { readonly measure: Extract<WindowMeasure, { kind: "slider" }>; readonly onAction: (action: ActionDescriptor) => unknown }) {
  const dispatchValue = useMemo(
    () => createDirectionalAsyncDispatcher((value) => onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value } })),
    [measure.onChange, onAction],
  );
  const formatDisplayValue = windowMeasureUsesProbabilityReadout(measure) ? windowMeasureProbabilityReadout : undefined;
  const disabled = measure.disabled === true;

  return (
    <Slider
      id={measure.id}
      data-published-value={String(measure.value)}
      value={[measure.value]}
      min={measure.min}
      max={measure.max}
      ready={measure.ready}
      loading={measure.loading === true}
      waiting={measure.waiting === true}
      step={measure.step}
      disabled={disabled}
      formatDisplayValue={formatDisplayValue}
      onValueChange={(values) => {
        if (disabled) return;
        dispatchValue(values[0] ?? measure.value);
      }}
    />
  );
}

function windowMeasureGroupHeaderSlider(measure: Extract<WindowMeasure, { kind: "group" }>, onAction: (action: ActionDescriptor) => unknown): ReactNode | undefined {
  if (measure.value === undefined || measure.onChange === undefined) return undefined;
  const sliderMeasure: Extract<WindowMeasure, { kind: "slider" }> = {
    kind: "slider",
    id: `${measure.id}.header-slider`,
    label: undefined,
    value: measure.value,
    min: measure.min ?? 0,
    max: measure.max ?? 1,
    step: measure.step,
    ready: measure.ready,
    loading: measure.loading,
    waiting: measure.waiting,
    onChange: measure.onChange,
  };
  return <WindowMeasureSlider measure={sliderMeasure} onAction={onAction} />;
}

function windowMeasureSelectControl(measure: Extract<WindowMeasure, { kind: "select" }>, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  return <WindowMeasureSelect measure={measure} onAction={onAction} />;
}

function windowMeasureToggleControl(measure: Extract<WindowMeasure, { kind: "toggle" }>, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  return <WindowMeasureToggle measure={measure} onAction={onAction} />;
}

function windowMeasureNumberControl(measure: Extract<WindowMeasure, { kind: "number" }>, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  return <WindowMeasureNumber measure={measure} onAction={onAction} />;
}

function windowMeasureToggleIcon(measure: Extract<WindowMeasure, { kind: "toggle" }>): ReactNode {
  return <Icon icon={measure.iconId as IconName} size={12} />;
}

/**
 * 🌲️ Maps window measures to native panel-tree rows — same chrome as left-corner trees (label left, control right, guide lines).
 * Pre-reverses top-level measures so bottom-anchored `direction="up"` panels read Count at the bottom, Distribution above.
 */
export function windowMeasuresToTreeItems(measures: readonly WindowMeasure[], onAction: (action: ActionDescriptor) => unknown, reverseForUpPanel = true): TreeDataItem[] {
  const ordered = reverseForUpPanel ? [...measures].reverse() : [...measures];
  const mapMeasure = (measure: WindowMeasure): TreeDataItem => {
    if (measure.kind === "group") {
      return {
        id: measure.id,
        label: measure.label,
        defaultOpen: measure.defaultOpen,
        control: windowMeasureGroupHeaderSlider(measure, onAction),
        items: measure.children.length > 0 ? windowMeasuresToTreeItems(measure.children, onAction, false) : undefined,
      };
    }
    if (measure.kind === "slider") {
      return {
        id: measure.id,
        label: measure.label ?? "",
        control: <WindowMeasureSlider measure={measure} onAction={onAction} />,
        loading: measure.loading,
        waiting: measure.waiting,
      };
    }
    if (measure.kind === "select") {
      return {
        id: measure.id,
        label: measure.label ?? "",
        control: windowMeasureSelectControl(measure, onAction),
      };
    }
    if (measure.kind === "number") {
      return {
        id: measure.id,
        label: measure.label ?? "",
        control: windowMeasureNumberControl(measure, onAction),
        loading: measure.loading,
        waiting: measure.waiting,
      };
    }
    return {
      id: measure.id,
      label: measure.label ?? measure.text ?? "",
      icon: windowMeasureToggleIcon(measure),
      control: windowMeasureToggleControl(measure, onAction),
    };
  };
  return ordered.map(mapMeasure);
}

function renderWindowMeasure(measure: WindowMeasure, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  if (measure.kind === "group") {
    const headerSlider = windowMeasureGroupHeaderSlider(measure, onAction);
    return (
      <WindowMeasureTreeGroup key={measure.id} id={measure.id} label={measure.label} defaultOpen={measure.defaultOpen} headerControl={headerSlider}>
        {measure.children.map((child) => renderWindowMeasure(child, onAction))}
      </WindowMeasureTreeGroup>
    );
  }
  if (measure.kind === "select") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label === undefined ? undefined : uiDataLabel(measure.label)}>
        {windowMeasureSelectControl(measure, onAction)}
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "slider") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label === undefined ? undefined : uiDataLabel(measure.label)}>
        <WindowMeasureSlider measure={measure} onAction={onAction} />
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "number") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label === undefined ? undefined : uiDataLabel(measure.label)}>
        {windowMeasureNumberControl(measure, onAction)}
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "toggle") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label ?? measure.text ? uiDataLabel(measure.label ?? measure.text ?? "") : undefined} icon={windowMeasureToggleIcon(measure)}>
        {windowMeasureToggleControl(measure, onAction)}
      </WindowMeasureTreeLeaf>
    );
  }
  return null;
}

function windowMeasuresOverlay(measures: readonly WindowMeasure[] | undefined, onAction: (action: ActionDescriptor) => unknown, direction: "up" | "down" = "down"): ReactNode | undefined {
  if (!measures || measures.length === 0) return undefined;
  return <WindowMeasuresTree direction={direction}>{measures.map((measure) => renderWindowMeasure(measure, onAction))}</WindowMeasuresTree>;
}

/** 🪟️ Public window-options tree for measures rails and tests — icon before label, checkbox for toggles. */
export function renderWindowMeasuresTree(measures: readonly WindowMeasure[], onAction: (action: ActionDescriptor) => unknown, direction: "up" | "down" = "down"): ReactNode | undefined {
  return windowMeasuresOverlay(measures, onAction, direction);
}

/** 🔢️ `generation` is the window's utility register generation this picker rendered — stamped as `expectedGeneration` on its `setActiveUtility` press (`🎯️input-ledger` L3); absent, the press is an unconditional write (a legacy call site). */
export function SelectionUtilityOptions({ activeUtilityId, windowId, onAction, generation }: { readonly activeUtilityId: string | undefined; readonly windowId: string; readonly onAction: (action: ActionDescriptor) => void; readonly generation?: number }) {
  const methodLabel = useLabel("ui.selection.method");
  const modeLabel = useLabel("ui.selection.mode");
  const rectangleLabel = useLabel("ui.selection.rectangle");
  const lassoLabel = useLabel("ui.selection.lasso");
  const selectiveLabel = useLabel("ui.selection.selective");
  const additiveLabel = useLabel("ui.selection.additive");
  const subtractiveLabel = useLabel("ui.selection.subtractive");
  const invertiveLabel = useLabel("ui.selection.invertive");
  const selectionMethod = activeUtilityId === "selectLasso" ? "lasso" : "rectangle";
  // 🐚️ Replaces the old `(globalThis).__selectionMode` + `window` `"semio:selectionOptionsChanged"`
  // broadcast — this shell's own store, so its selection-mode toggle never reconfigures another mounted
  // shell's marquee gestures.
  const selectionStore = useShellScope().selection;

  const [selectionMode, setSelectionMode] = useState<MergeMode>(() => selectionStore.get());

  const handleModeChange = (mode: MergeMode) => {
    selectionStore.set(mode);
    setSelectionMode(mode);
  };

  const handleMethodChange = (method: "rectangle" | "lasso") => {
    onAction({
      controllerId: "window",
      action: SET_ACTIVE_UTILITY_ACTION_ID,
      args: { windowId, utilityId: method === "lasso" ? "selectLasso" : "selectMarquee", ...(generation === undefined ? {} : { expectedGeneration: generation }) },
    });
  };

  return (
    <div className="flex w-full min-w-0 max-w-full flex-wrap items-start gap-double">
      <div className="flex min-w-0 max-w-full flex-wrap items-center gap-single">
        <span className="text-tiny text-muted-foreground uppercase tracking-wider font-semibold">{methodLabel}</span>
        <ToggleGroup
          kind="single"
          semantics="radio"
          aria-label={methodLabel}
          className="h-auto max-w-full flex-wrap"
          value={selectionMethod}
          onValueChange={(val) => {
            if (val === "rectangle" || val === "lasso") {
              handleMethodChange(val);
            }
          }}
          items={[
            { value: "rectangle", icon: <Icon icon="square-dashed" size="small" />, text: rectangleLabel },
            { value: "lasso", icon: <Icon icon="lasso" size="small" />, text: lassoLabel },
          ]}
        />
      </div>
      <RibbonDivider />
      <div className="flex min-w-0 max-w-full flex-wrap items-center gap-single">
        <span className="text-tiny text-muted-foreground uppercase tracking-wider font-semibold">{modeLabel}</span>
        <ToggleGroup
          kind="single"
          semantics="radio"
          aria-label={modeLabel}
          className="h-auto max-w-full flex-wrap"
          value={selectionMode}
          onValueChange={(val) => {
            if (val === "replace" || val === "additive" || val === "subtractive" || val === "invertive") {
              handleModeChange(val);
            }
          }}
          items={[
            { value: "replace", icon: "mouse-pointer", text: selectiveLabel },
            { value: "additive", icon: "plus", text: additiveLabel },
            { value: "subtractive", icon: "minus", text: subtractiveLabel },
            { value: "invertive", icon: { kind: "emoji", emoji: "🔄" }, text: invertiveLabel },
          ]}
        />
      </div>
    </div>
  );
}

/** 🪪️ THE DOM identity of one window-measure control: the WINDOW INSTANCE it is rendered for, then the
 * program-authored measure id. Same rule and same separator as `uiNodeDomId` — a window kind's measure tree is
 * authored ONCE for the kind (`world3d_projection_measures` takes a kind-level `id_prefix`, and puzzle3d passes
 * the literal `"puzzle3d"`) and then rendered once per OPEN INSTANCE of that kind, so the authored id alone puts
 * `puzzle3d-measure-projection-orthographic-view` in the document once per pane the moment more than one
 * measures rail is unfolded. Duplicate ids are invalid HTML, they make the control unaddressable through
 * `<label for>`/`aria-labelledby`/automation, and they also collapse the rail's own fold state
 * (`WindowMeasureTreeGroup` keys `useTreeOpenState` on the group id), so unfolding "Parallel" in one pane
 * unfolded it in the other. */
export function windowMeasureDomId(windowId: string, measureId: string): string {
  return `${windowId}/${measureId}`;
}

/** 🪪️ {@link windowMeasureDomId} applied to a whole authored measure subtree — ids only, every other field
 * (labels, values, `reveal` group, `onChange`) verbatim, so the authored id stays the one integration key the
 * program, `windowMeasureTreeContainsId` and the `activeUtilityId` routing all speak. */
export function qualifyWindowMeasureIds(measures: readonly WindowMeasure[], windowId: string): WindowMeasure[] {
  return measures.map((measure) =>
    measure.kind === "group"
      ? { ...measure, id: windowMeasureDomId(windowId, measure.id), children: qualifyWindowMeasureIds(measure.children, windowId) }
      : { ...measure, id: windowMeasureDomId(windowId, measure.id) },
  );
}

export function windowMeasuresChrome(
  measures: readonly WindowMeasure[] | undefined,
  activeUtilityId: string | undefined,
  windowId: string,
  onAction: (action: ActionDescriptor) => unknown,
): { readonly measures: ReactNode | undefined; readonly utilityOptions: ReactNode | undefined } {
  const partitioned = partitionWindowMeasures(measures ?? [], activeUtilityId);
  const general = qualifyWindowMeasureIds(partitioned.general, windowId);
  const utilityOptions = qualifyWindowMeasureIds(partitioned.utilityOptions, windowId);
  // 🪟️ Stamps this chrome's owning `windowId` onto every measure action, mirroring `tagSetActiveUtilityWindow`
  // for the utility bar — the generic `onAction` dispatch path reads it back out to target the plugin call's
  // `view_state.windowId`, so a grid/LOD/selection toggle only ever mutates ITS OWN window's options.
  const taggedOnAction = (action: ActionDescriptor) => onAction({ ...action, args: { ...(action.args as object | undefined), windowId } });
  return {
    measures: windowMeasuresOverlay(general, taggedOnAction),
    utilityOptions: windowMeasuresOverlay(utilityOptions, taggedOnAction, "up"),
  };
}

/** 🎓️ Whether a utility node tree has a node (leaf or group) with the given id anywhere in it — used
 * to decide if this window's utility bar is the one an introduction step's `Utility` anchor targets. */
export function utilityNodeTreeContainsId(nodes: readonly UtilityNode[], targetId: string): boolean {
  return nodes.some((node) => node.id === targetId || (node.kind === "collection" && utilityNodeTreeContainsId(node.children, targetId)));
}

export function utilityBarNode(utilities: readonly UtilityNode[] | undefined, windowId: string, onAction: (action: ActionDescriptor) => void, revealUtilityId?: string | null, utilityOptions?: ReactNode): ReactNode {
  if (!utilities?.length && !utilityOptions) return undefined;
  const categories = groupUtilityNodesByCategory(utilities ?? [], UTILITY_CATEGORIES);
  if (!categories.length && !utilityOptions) return undefined;
  const grouped: UtilityNode[] = [];
  for (const node of categories) {
    if (node.kind === "collection" && (node.category === "utilities" || node.category === "selection")) {
      if (node.id === "group:Select" || node.id === "group:selection" || node.label === "Select" || node.text === "Select") {
        grouped.push(...node.children);
      } else {
        for (const child of node.children) {
          if (child.kind === "collection" && (child.id === "group:Select" || child.id === "group:selection" || child.label === "Select" || child.text === "Select")) {
            grouped.push(...child.children);
          } else {
            grouped.push(child);
          }
        }
      }
    } else {
      grouped.push(node);
    }
  }
  return <UtilityTree id={`ui.utilities.${windowId}`} utilities={grouped} onAction={onAction} direction="up" revealUtilityId={revealUtilityId} utilityOptions={utilityOptions} />;
}

//#region 🧰️WindowActionPane
/** 🎛️ What a staged field reads from the shell beyond its own value: the live interaction selection of a domain (every
 * domain's when the reference names none) a reference input's "use current selection" takes. */
export type StagedArgContextV1 = { readonly selection?: (domain: string | undefined) => readonly string[] };

/** 🕹️ The ids selected in `domain` of an interaction state, or in every domain when `domain` is absent. */
export function interactionSelectionIdsV1(state: InteractionState, domain: string | undefined): readonly string[] {
  return domain === undefined ? Object.values(state.selection).flatMap((entry) => entry.ids) : (state.selection[domain]?.ids ?? []);
}

/** 🪧️ A staged number as the human reads it: `stored × displayFactor` at `precision` (`uiNumberDisplayText`), followed by
 * the display unit (else the stored unit). */
export function stagedNumberDisplayText(value: number, facets: Pick<ActionArgNumberFacets, "precision" | "displayFactor" | "displayUnit" | "unit">): string {
  const text = uiNumberDisplayText(value, facets.displayFactor, facets.precision);
  const unit = facets.displayUnit ?? facets.unit;
  return unit ? `${text} ${unit}` : text;
}

/** 📐️ The UI-contract number facets of a staged number, stepper, slider, dial or vector input — `actionArgNumberFacets`, the
 * twin of the Rust `ActionArgDef::number_facets` the wgpu shell's staged rows read (`🛂️manifest/🧫️fixtures/🧫️number-facets`):
 * key range, step, look, axis, unit symbols, display factor, precision, admitted detents and the hard limits whose
 * refusals read in the shell's current language. */
export function stagedNumberFacetsV1(def: ResolvedActionArgDef): ActionArgNumberFacets {
  const facets = actionArgNumberFacets(def, currentShellLocaleV1());
  if (facets === null) throw new Error(`Staged input ${def.id} has no number facets`);
  return facets;
}

/** 🧷️ The ids a reference input holds, whether it takes many or one, as the text a selection spells them in. */
function stagedReferenceIds(value: unknown): readonly string[] {
  return (Array.isArray(value) ? value : [value]).flatMap((id) => referenceIdText(id) ?? []);
}

/** 🧷️ A staged reference input with the W1-E `reference_list` recipe semantics: one chip per referenced id that removes
 * it on activation (announced as "Remove <id>"), an empty line while there is none, and a "use current selection"
 * button that takes the live selection (capped at `maxItems`, the first id for a single reference). */
function StagedReferenceField({ id, labelledBy, value, many, maxItems, domain, idType, disabled, selection, onChange }: { readonly id: string; readonly labelledBy?: string; readonly value: unknown; readonly many: boolean; readonly maxItems?: number; readonly domain?: string; readonly idType?: ReferenceIdType; readonly disabled?: boolean; readonly selection?: StagedArgContextV1["selection"]; readonly onChange: (value: unknown) => void }): ReactElement {
  const ids = stagedReferenceIds(value);
  const removeLabel = (item: string) => String(shellLabel("ui.referenceList.remove", { item }));
  const commit = (next: readonly string[]) => {
    const values = next.flatMap((text) => referenceIdValue(idType, text) ?? []);
    onChange(many ? values : (values[0] ?? (idType === "integer" ? null : "")));
  };
  return (
    <div id={id} role="group" aria-labelledby={labelledBy} data-staged-reference="" className="flex w-full min-w-0 flex-col gap-tiny">
      {ids.length === 0 ? (
        <span data-staged-reference-empty="" className="text-xs text-muted-foreground">{shellLabel("ui.referenceList.empty")}</span>
      ) : (
        <div role="toolbar" aria-labelledby={labelledBy} className="flex flex-wrap gap-tiny">
          {ids.map((item) => (
            <button key={item} type="button" data-staged-reference-chip={item} aria-label={removeLabel(item)} title={removeLabel(item)} disabled={disabled} className="inline-flex items-center gap-tiny rounded-sm border px-single text-xs" onClick={() => commit(ids.filter((other) => other !== item))}>
              {item}
              <Icon icon="x" size="small" />
            </button>
          ))}
        </div>
      )}
      <button type="button" data-staged-reference-use-selection="" disabled={disabled || selection === undefined} className="self-start text-xs underline" onClick={() => {
        const picked = [...new Set(selection?.(domain) ?? [])];
        commit(many ? picked.slice(0, maxItems ?? picked.length) : picked.slice(0, 1));
      }}>
        {shellLabel("ui.referenceList.useSelection")}
      </button>
    </div>
  );
}

/** 🎨️ A staged colour with the W1-E `color_input` recipe semantics: the swatch (the platform's colour picker, named by
 * the field), the hex field — typed hex is staged once it parses (`parseUiColorHex`), on Enter or blur, keeping the
 * alpha when the text carries none — and, for a colour with alpha, the opacity slider over 0..1. sRGB components in
 * 0..1, straight alpha. */
function StagedColorField({ id, labelledBy, value, alpha, disabled, required, onChange }: { readonly id: string; readonly labelledBy?: string; readonly value: unknown; readonly alpha: boolean; readonly disabled?: boolean; readonly required?: boolean; readonly onChange: (value: unknown) => void }): ReactElement {
  const components = Array.isArray(value) ? value : [];
  const rgba = [0, 1, 2, 3].map((index) => {
    const component = Number(components[index] ?? (index === 3 ? 1 : 0));
    return Number.isFinite(component) ? Math.min(1, Math.max(0, component)) : 0;
  });
  const [draft, setDraft] = useState<string | null>(null);
  const commit = (next: readonly number[]) => onChange(alpha ? [...next] : next.slice(0, 3));
  const commitHex = (text: string) => {
    setDraft(null);
    const parsed = parseUiColorHex(text);
    if (parsed) commit(/^#?([0-9a-f]{4}|[0-9a-f]{8})$/i.test(text.trim()) ? parsed : [...parsed.slice(0, 3), rgba[3]!]);
  };
  return (
    <div id={id} role="group" aria-labelledby={labelledBy} data-staged-control="color" className="flex w-full min-w-0 flex-wrap items-center gap-single">
      <input
        id={`${id}.swatch`}
        type="color"
        aria-labelledby={labelledBy}
        className="h-medium w-medium shrink-0 cursor-pointer rounded-sm border"
        value={uiColorHex(rgba, false)}
        disabled={disabled}
        onChange={(event) => {
          const parsed = parseUiColorHex(event.target.value);
          if (parsed) commit([...parsed.slice(0, 3), rgba[3]!]);
        }}
      />
      <Input
        id={`${id}.hex`}
        aria-label={String(shellLabel("ui.colorInput.hex"))}
        required={required}
        type="text"
        spellCheck={false}
        className="h-medium w-[7rem] min-w-0 font-mono"
        value={draft ?? uiColorHex(rgba, alpha)}
        disabled={disabled}
        onChange={(event) => setDraft(event.target.value)}
        onBlur={(event) => commitHex(event.target.value)}
        onKeyDown={(event) => {
          if (event.key !== "Enter") return;
          event.preventDefault();
          commitHex(event.currentTarget.value);
        }}
      />
      {alpha ? <Slider id={`${id}.alpha`} aria-label={String(shellLabel("ui.colorInput.alpha"))} className="min-w-[6rem] flex-1" min={0} max={1} step={0.01} value={[rgba[3]!]} disabled={disabled} onValueChange={(values) => commit([...rgba.slice(0, 3), values[0] ?? rgba[3]!])} /> : null}
    </div>
  );
}

/** 🧲️ A staged vector with the W1-E `vector_input` recipe semantics: one {@link StagedNumberField} per component, named
 * "x (mm)" and so on, every axis sharing the vector's facets. */
function StagedVectorField({ id, labelledBy, value, dims, facets, disabled, required, onChange }: { readonly id: string; readonly labelledBy?: string; readonly value: unknown; readonly dims: number; readonly facets: ActionArgNumberFacets; readonly disabled?: boolean; readonly required?: boolean; readonly onChange: (value: unknown) => void }): ReactElement {
  const tuple = Array.isArray(value) && value.length >= dims ? (value as readonly number[]) : null;
  const axes = Array.from({ length: dims }, (_, index) => ["x", "y", "z", "w"][index] ?? String(index));
  const unit = facets.displayUnit ?? facets.unit;
  const commit = (index: number, next: number) => onChange(axes.map((_, component) => (component === index ? next : (tuple?.[component] ?? 0))));
  return (
    <div id={id} role="group" aria-labelledby={labelledBy} data-staged-control="vector" className="flex w-full min-w-0 flex-wrap gap-single">
      {axes.map((axis, index) => (
        <StagedNumberField key={`${id}.${axis}`} id={`${id}.${axis}`} label={unit ? `${axis} (${unit})` : axis} stored={tuple === null ? null : (tuple[index] ?? 0)} facets={facets} disabled={disabled} required={required} onCommit={(next) => commit(index, next)} />
      ))}
    </div>
  );
}

/** 🗜️ A staged number field — a number input or one vector axis — under the number-control law (design §18,
 * `🖱️ui/🧬️contract/🧫️fixtures/🧫️number-controls`) and its facets: it shows `stored × displayFactor` at the precision; the
 * keys `uiNumberFieldKey` names (Home/End only toward a key bound) move the stored value through `uiNumberKeyValue`; a typed
 * display value reads back through `uiNumberTypedValue` (an exact detent or the current value kept, else rounded half away
 * from zero and divided by the display factor) and is staged at once — unless it is unreadable or crosses a hard limit,
 * which keeps the draft, marks the field invalid and names the limit's refusal; an emptied field clears (`onClear`) where
 * the input may be unset. A stored value changed from outside (Reset, a default) replaces a draft that does not read back
 * to it. A lone field shows its unit beside it; an axis names it in its label. */
function StagedNumberField({ id, label, labelledBy, unit, stored, facets, disabled, required, onCommit, onClear }: { readonly id: string; readonly label?: string; readonly labelledBy?: string; readonly unit?: string; readonly stored: number | null; readonly facets: ActionArgNumberFacets; readonly disabled?: boolean; readonly required?: boolean; readonly onCommit: (next: number) => void; readonly onClear?: () => void }): ReactElement {
  const [draft, setDraft] = useState<string | null>(null);
  const [refusal, setRefusal] = useState<{ readonly message: string | null } | null>(null);
  const factor = facets.displayFactor ?? null;
  const precision = facets.precision ?? null;
  const shown = (value: number) => uiNumberDisplayText(value, factor, precision);
  const step = facets.step ?? (precision === null ? undefined : 10 ** -precision / (factor ?? 1));
  const refusalId = `${id}.refusal`;
  const [seen, setSeen] = useState(stored);
  if (seen !== stored) {
    setSeen(stored);
    const number = Number(draft?.trim() ?? "");
    const readBack = draft === null || draft.trim() === "" || !Number.isFinite(number) ? null : uiNumberTypedValue(number, factor, precision, [stored ?? 0, ...facets.snaps]);
    if (draft !== null && readBack !== stored) {
      setDraft(null);
      setRefusal(null);
    }
  }
  const typed = (text: string) => {
    setDraft(text);
    const number = Number(text.trim());
    if (text.trim() === "" && onClear !== undefined) {
      setRefusal(null);
      onClear();
      return;
    }
    if (text.trim() === "" || !Number.isFinite(number)) {
      setRefusal({ message: null });
      return;
    }
    const next = uiNumberTypedValue(number, factor, precision, [stored ?? 0, ...facets.snaps]);
    const crossed = uiNumberCrossedBound(next, facets.min, facets.max, facets.limits);
    if (crossed !== null) {
      setRefusal({ message: crossed.refusal ?? null });
      return;
    }
    setRefusal(null);
    onCommit(next);
  };
  const input = (
    <Input
      id={id}
      aria-labelledby={labelledBy}
      required={required}
      type="number"
      className="h-medium w-full min-w-0"
      value={draft ?? (stored === null ? "" : shown(stored))}
      min={facets.min === undefined ? undefined : uiNumberDisplay(facets.min, factor)}
      max={facets.max === undefined ? undefined : uiNumberDisplay(facets.max, factor)}
      step={step === undefined ? undefined : uiNumberDisplay(step, factor)}
      disabled={disabled}
      aria-invalid={refusal === null ? undefined : true}
      aria-describedby={refusal?.message ? refusalId : undefined}
      onKeyDown={(event) => {
        const key = uiNumberFieldKey(event.key, event.shiftKey, facets.min, facets.max);
        if (key === null) return;
        event.preventDefault();
        setDraft(null);
        setRefusal(null);
        onCommit(uiNumberKeyValue(stored ?? 0, facets.min ?? null, facets.max ?? null, facets.step ?? 0, precision, factor, facets.snaps, key.key, key.large));
      }}
      onChange={(event) => typed(event.target.value)}
      onBlur={() => {
        if (refusal === null) setDraft(null);
      }}
    />
  );
  return (
    <div className="flex min-w-[4rem] flex-1 flex-col gap-tiny">
      <div className="flex items-center gap-tiny text-xs text-muted-foreground">
        {label === undefined ? null : <label htmlFor={id}>{label}</label>}
        {input}
        {unit === undefined ? null : <span data-slot="staged-unit" className="shrink-0 tabular-nums">{unit}</span>}
      </div>
      {refusal?.message ? <span id={refusalId} role="alert" data-staged-refusal="" className="text-xs text-destructive">{refusal.message}</span> : null}
    </div>
  );
}

/**
 * 🎛️ Renders one {@link ActionArgControl} into a STAGED form field — the crucial difference from
 * `renderUiControl` in `ui-interpreter.tsx` is that this dispatches NOTHING globally; `onChange` only
 * writes to the caller's local staged buffer. `value` is the already-resolved effective value
 * (staged ?? default ?? unset). Stepper, dial (a detented slider read in display units), segmented choice (a radio group),
 * multi-line text (a textarea: Enter is its line break, the form's own submit commits), vector axes and reference chips
 * follow the W1-E contract recipes.
 */
export function renderStagedArgControl(def: ResolvedActionArgDef, value: unknown, onChange: (value: unknown) => void, disabled?: boolean, field?: UIDialogFieldBinding, context: StagedArgContextV1 = {}): ReactElement {
  const control = renderStagedArgValueControl(def, value, onChange, disabled, field, context);
  if (!def.nullable) return control;
  const cleared = value === null;
  return (
    <div className="flex w-full min-w-0 items-center gap-single" data-staged-nullable="">
      <div className="min-w-0 flex-1">{control}</div>
      <button type="button" aria-pressed={cleared} aria-describedby={field?.labelledBy} disabled={disabled} className="shrink-0 text-xs underline" onClick={() => onChange(cleared ? undefined : null)}>
        {shellLabel("ui.nullableInput.clear")}
      </button>
    </div>
  );
}

/** 🎛️ The value control of one staged argument, without the clear toggle a nullable input adds around it. */
function renderStagedArgValueControl(def: ResolvedActionArgDef, value: unknown, onChange: (value: unknown) => void, disabled?: boolean, field?: UIDialogFieldBinding, context: StagedArgContextV1 = {}): ReactElement {
  const control: ActionArgControl = argControl(def);
  const fieldId = field?.id ?? def.id;
  const labelledBy = field?.labelledBy;
  switch (control.kind) {
    case "text":
      return <Input id={fieldId} aria-labelledby={labelledBy} required={field?.required} type="text" className="h-medium w-full min-w-0" value={typeof value === "string" ? value : ""} placeholder={control.placeholder} disabled={disabled} onChange={(event) => onChange(event.target.value)} />;
    case "multiline":
      return <Textarea id={fieldId} aria-labelledby={labelledBy} required={field?.required} data-staged-control="multiline" className="min-h-[4.5rem] w-full min-w-0" value={typeof value === "string" ? value : ""} disabled={disabled} onChange={(event) => onChange(event.target.value)} />;
    case "stepper": {
      const facets = stagedNumberFacetsV1(def);
      const numeric = typeof value === "number" && Number.isFinite(value) ? value : undefined;
      return <Stepper id={fieldId} aria-labelledby={labelledBy} aria-valuetext={numeric === undefined ? undefined : stagedNumberDisplayText(numeric, facets)} value={numeric} defaultValue={facets.min ?? 0} min={facets.min} max={facets.max} step={facets.step ?? 1} precision={facets.precision} snapValues={facets.snaps} displayFactor={facets.displayFactor ?? null} unit={facets.displayUnit ?? facets.unit ?? null} limits={facets.limits} disabled={disabled} onChange={(next) => onChange(next)} />;
    }
    case "slider":
    case "dial": {
      const facets = stagedNumberFacetsV1(def);
      const numeric = typeof value === "number" && Number.isFinite(value) ? value : (facets.min ?? control.min);
      return <Slider id={fieldId} aria-labelledby={labelledBy} aria-valuetext={stagedNumberDisplayText(numeric, facets)} data-staged-control={control.kind} className="w-full min-w-0" min={facets.min ?? control.min} max={facets.max ?? control.max} step={facets.step} snapValues={facets.snaps} scale={facets.scale} appearance={facets.appearance} displayFactor={facets.displayFactor ?? null} precision={facets.precision ?? null} limits={facets.limits} formatDisplayValue={(shown) => stagedNumberDisplayText(shown, facets)} value={[numeric]} disabled={disabled} onValueChange={(values) => onChange(values[0] ?? numeric)} />;
    }
    case "number": {
      const facets = stagedNumberFacetsV1(def);
      return <StagedNumberField id={fieldId} labelledBy={labelledBy} unit={facets.displayUnit ?? facets.unit} stored={typeof value === "number" && Number.isFinite(value) ? value : null} facets={facets} disabled={disabled} required={field?.required} onCommit={(next) => onChange(next)} onClear={() => onChange(undefined)} />;
    }
    case "toggle":
      return <Toggle id={fieldId} aria-labelledby={labelledBy} icon="check" pressed={value === true} text={uiDataLabel(def.label)} disabled={disabled} onPressedChange={(pressed) => onChange(pressed)} />;
    case "segmented": {
      if (def.schema.kind !== "string") throw new Error("Segmented control requires a string argument schema");
      const options = actionArgStringOptions(def.schema);
      return (
        <ToggleGroup
          id={fieldId}
          kind="single"
          semantics="radio"
          aria-labelledby={labelledBy}
          data-staged-control="segmented"
          disabled={disabled || options.length === 0}
          value={typeof value === "string" && options.some((option) => option.value === value) ? value : ""}
          onValueChange={(next) => { if (next !== "") onChange(next); }}
          items={options.map((option) => ({ value: option.value, icon: "circle-dot", text: option.label }))}
        />
      );
    }
    case "artifactKind":
    case "surfaceApp":
    case "select": {
      if (def.schema.kind !== "string") throw new Error("Select control requires a string argument schema");
      return (
        <Select id={def.id} value={typeof value === "string" && actionArgStringOptions(def.schema).some(option => option.value === value) ? value : ""} disabled={disabled || actionArgStringOptions(def.schema).length === 0} onValueChange={(next) => onChange(next)}>
          <SelectTrigger id={fieldId} aria-labelledby={labelledBy} aria-required={field?.required} className="h-medium w-full min-w-0" size="sm">
            <SelectValue placeholder={def.label} />
          </SelectTrigger>
          <SelectContent>
            {actionArgStringOptions(def.schema).map((option, index) => (
              <SelectItem key={`${def.id}:${index}:${option.value}`} value={option.value}>
                {option.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      );
    }
    case "reference":
      return <StagedReferenceField id={fieldId} labelledBy={labelledBy} value={value} many={control.many ?? false} maxItems={control.maxItems} domain={control.domain} idType={control.idType} disabled={disabled} selection={context.selection} onChange={onChange} />;
    case "vector":
      return <StagedVectorField id={fieldId} labelledBy={labelledBy} value={value} dims={control.dims} facets={stagedNumberFacetsV1(def)} disabled={disabled} required={field?.required} onChange={onChange} />;
    case "color":
      return <StagedColorField id={fieldId} labelledBy={labelledBy} value={value} alpha={control.alpha} disabled={disabled} required={field?.required} onChange={onChange} />;
    case "iconSelect":
      return <IconSelector id={fieldId} aria-labelledby={labelledBy} disabled={disabled} classifyIconSelectorMode={undefined} value={typeof value === "string" ? value : ""} uniform onChange={(next) => onChange(next)} />;
  }
}

/** 🧰️ True when an action carries a user-facing argument and therefore stages a form instead of firing immediately
 * (P1–P4). Hidden arguments are never entered by a user — the framework ToolRun verbs resolve their run or active
 * tool when a chord fires them bare — so an action whose every argument is hidden fires. */
export function actionRequiresStagedForm(action: Pick<ActionDefinition, "args">): boolean {
  return (action.args ?? []).some((arg) => arg.presentation?.kind !== "hidden");
}

/** 🧰️ The decision a bound hotkey makes for one action (P4). */
/** ⌨️ True when a keydown's target is a text-editing surface (input/textarea/select/contenteditable) — hotkeys never fire while the user is typing. */
export function isEditableEventTarget(target: EventTarget | null): boolean {
  if (!(target instanceof Element)) return false;
  if (target instanceof HTMLElement && target.isContentEditable) return true;
  return target.closest('input,textarea,select,[contenteditable]:not([contenteditable="false"]),[role="textbox"],[role="searchbox"]') !== null;
}

/** 🎹️ Focused widgets own ordinary keys; text editors also retain modified editing chords. */
export function controlOwnsKeyboardEvent(event: KeyboardEventLike & { readonly target: EventTarget | null; readonly isComposing?: boolean }): boolean {
  if (event.isComposing || isEditableEventTarget(event.target)) return true;
  if (event.ctrlKey || event.metaKey || event.altKey || !(event.target instanceof Element)) return false;
  return event.target.closest('button,a[href],summary,audio[controls],video[controls],[role="button"],[role="link"],[role="checkbox"],[role="radio"],[role="switch"],[role="combobox"],[role="listbox"],[role="option"],[role="menu"],[role="menuitem"],[role="menuitemcheckbox"],[role="menuitemradio"],[role="slider"],[role="spinbutton"],[role="tab"],[role="tree"],[role="treeitem"],[role="gridcell"]') !== null;
}

/** 🩹️ Structural, not `React.KeyboardEvent`/`globalThis.KeyboardEvent` specifically —
 * `keyboardEventMatchesChord` only ever reads the five fields below, which both shapes carry
 * identically, and its real callers legitimately hold either (a native `useShellKeydown` listener vs a
 * React `onKeyDown` handler). */
export type KeyboardEventLike = { readonly key: string; readonly ctrlKey: boolean; readonly metaKey: boolean; readonly shiftKey: boolean; readonly altKey: boolean };

/** ⌨️ Punctuation keys whose DOM `event.key` is a single character — chord declarations MUST use the character, never these names. */
export const CHORD_KEY_NAME_ALIASES: Readonly<Record<string, string>> = {
  period: ".",
  comma: ",",
  slash: "/",
  minus: "-",
  equal: "=",
  semicolon: ";",
  quote: "'",
  backquote: "`",
  bracketleft: "[",
  bracketright: "]",
  backslash: "\\",
};

/** ⌨️ The key token `ShellHost` compares against `event.key` for one chord's last `+` segment. */
export function canonicalChordKeyToken(token: string): string {
  const lower = token.toLowerCase();
  return CHORD_KEY_NAME_ALIASES[lower] ?? token;
}

/** ⌨️ False when a chord's key segment spells punctuation as a name (`period`) instead of the character (`.`). */
export function chordKeyTokenIsCanonical(token: string): boolean {
  const lower = token.toLowerCase();
  if (lower in CHORD_KEY_NAME_ALIASES && token !== CHORD_KEY_NAME_ALIASES[lower]) return false;
  return true;
}

/** ⌨️ True when every non-modifier segment of a chord uses canonical key tokens (no `mod+period` dead strings). */
export function chordUsesCanonicalKeyTokens(chord: string): boolean {
  const modifierTokens = new Set(["mod", "ctrl", "meta", "shift", "alt"]);
  const parts = chord.split("+").map((part) => part.trim()).filter(Boolean);
  const key = parts[parts.length - 1] ?? "";
  if (modifierTokens.has(key.toLowerCase())) return true;
  return chordKeyTokenIsCanonical(key);
}

/** ⌨️ True when a keydown event matches one `+`-joined chord (e.g. `"mod+shift+z"`), where `mod` accepts either ctrl or meta. */
export function keyboardEventMatchesChord(event: KeyboardEventLike, chord: string): boolean {
  const parts = chord.split("+").map((part) => part.trim());
  const key = parts[parts.length - 1] ?? "";
  const needsMod = parts.includes("mod");
  const needsCtrl = parts.includes("ctrl") || parts.includes("control");
  const needsMeta = parts.includes("meta");
  const needsShift = parts.includes("shift");
  const needsAlt = parts.includes("alt");
  if (needsMod && !needsCtrl && !needsMeta) {
    if (event.ctrlKey === event.metaKey) return false;
  } else if (event.ctrlKey !== needsCtrl || event.metaKey !== needsMeta) {
    return false;
  }
  if (needsShift !== event.shiftKey) return false;
  if (needsAlt !== event.altKey) return false;
  return event.key.toLowerCase() === key.toLowerCase();
}

export type KeybindingIntent = { readonly kind: "fire" } | { readonly kind: "open"; readonly actionId: string } | { readonly kind: "execute"; readonly actionId: string; readonly args: Record<string, unknown> };

/**
 * ✍️ Pure P4 decision: an arg-less action fires directly; an arg-carrying action opens its staged form,
 * unless that form is already the expanded one in the active window AND validation passes, in which case
 * the hotkey executes with the merged effective args. An already-open-but-invalid form stays open.
 */
export function resolveKeybindingIntent(definition: Pick<ActionDefinition, "id" | "args"> | undefined, expandedActionId: string | null, stagedArgs: Readonly<Record<string, unknown>>): KeybindingIntent {
  if (!definition || !actionRequiresStagedForm(definition)) return { kind: "fire" };
  if (expandedActionId === definition.id) {
    const effective = effectiveActionArgs(definition.args, stagedArgs);
    if (unresolvedActionArgs(definition.args, effective).length === 0) return { kind: "execute", actionId: definition.id, args: effective };
  }
  return { kind: "open", actionId: definition.id };
}

/** 📋 Fragment carried by a guest `clipboardWrite` host effect, or undefined when the effect is not that variant. */
export function clipboardWriteFragmentFromEffect(effect: unknown): unknown | undefined {
  if (effect === null || typeof effect !== "object" || !("clipboardWrite" in effect)) return undefined;
  const write = (effect as { clipboardWrite?: { fragment?: unknown } }).clipboardWrite;
  return write?.fragment;
}

export type ClipboardActionDescriptor = { readonly action: string; readonly args?: unknown };

/** 📋 Named `fragment` on a paste descriptor, when the args object carries one. */
export function pasteArgsFragment(action: { readonly args?: unknown }): unknown | undefined {
  const args = action.args;
  if (args === undefined || args === null || typeof args !== "object" || Array.isArray(args)) return undefined;
  if (!("fragment" in args)) return undefined;
  return (args as { fragment?: unknown }).fragment;
}

/** 📋 Injects a host-retained copy fragment onto paste when `args.fragment` is missing. */
export function pasteActionWithRetainedFragment<T extends ClipboardActionDescriptor>(action: T, fragment: unknown | undefined): T {
  if (action.action !== "paste" || fragment === undefined) return action;
  if (pasteArgsFragment(action) !== undefined) return action;
  const args = action.args;
  const base = args !== undefined && typeof args === "object" && args !== null && !Array.isArray(args) ? (args as Record<string, unknown>) : {};
  return { ...action, args: { ...base, fragment } };
}

/** 🧰️ Pure P5 activation decision: an empty request, or re-requesting the already-active utility, deactivates (null); otherwise the requested utility becomes active. */
export function resolveUtilityActivation(current: string | null | undefined, requested: string): string | null {
  return requested === "" || (current ?? null) === requested ? null : requested;
}

/** 🗂️ Category id for one action: declared category, else `"history"` for history actions, else `"actions"` (mirrors the command-palette fallback at {@link resolveCommands}'s sibling `searchItems` builder). */
export function actionCategoryId(action: Pick<ActionDefinition, "category" | "kind">): string {
  return action.category ?? (action.kind === "history" ? "history" : "actions");
}

/** 🗂️ Resolves an action category's display label: the app's own group-label overlay first, then the shared `ui.ribbon.parent.*` chrome vocabulary for known category ids ({@link ribbonParentLabel}), else the raw id (mirrors {@link resolveUtilityGroupLabel}). */
function actionCategoryLabel(category: string, appLabelsOverlay: PluginAppLabelsOverlay): string {
  return resolveAppLabel(appLabelsOverlay, "group", category, ribbonParentLabel(category) ?? category);
}

/** 🗂️ Ordered, deduped categories from resolved actions (sibling of {@link commandCategories}). */
export function actionCategories(actions: readonly ActionDefinition[], appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY): { readonly id: string; readonly label: string }[] {
  const seen = new Set<string>();
  const categories: { readonly id: string; readonly label: string }[] = [];
  for (const action of actions) {
    const id = actionCategoryId(action);
    if (seen.has(id)) continue;
    seen.add(id);
    categories.push({ id, label: actionCategoryLabel(id, appLabelsOverlay) });
  }
  return categories;
}

/**
 * 🎛️ Category sections of one window's Actions rail (Tree twin of {@link buildCommandCategoryTree}):
 * one section per category, zero-arg actions fire directly, arg-carrying actions toggle a sibling form
 * section — exactly {@link buildCommandCategoryTree}'s list/form split, localized per category so
 * multiple categories can render side by side. Only one action (across all categories) is expanded at a
 * time, per `expandedActionId`.
 */
export function buildActionCategoryTree(
  windowId: string,
  controllerId: string,
  actions: readonly ResolvedActionDefinition[],
  expandedActionId: string | null,
  stagedArgsByKey: Readonly<Record<string, Readonly<Record<string, unknown>>>>,
  disabled: boolean,
  onExpandedChange: (actionId: string | null) => void,
  onStageArg: (actionId: string, argId: string, value: unknown) => void,
  onResetArgs: (actionId: string) => void,
  onExecute: (descriptor: ActionDescriptor) => void,
  appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
): TreeDataSection[] {
  const categories = actionCategories(actions, appLabelsOverlay);
  const expandedAction = expandedActionId ? actions.find((action) => action.id === expandedActionId) : undefined;
  const sections: TreeDataSection[] = [];
  for (const category of categories) {
    const categoryActions = actions.filter((action) => actionCategoryId(action) === category.id);
    sections.push({
      id: `action.category.${category.id}`,
      label: category.label,
      defaultOpen: true,
      items: categoryActions.map((action): TreeDataItem => {
        const icon = action.iconId ? <Icon icon={action.iconId as IconName} size="small" /> : undefined;
        // 🧯️ The armed-utility gate is about the APP's own verbs — a brush that owns the pointer must not
        // have "Delete Selection" pressed out from under it. It is NOT about the framework's own reserved
        // rows: `mod+z` undoes while the brush is armed (the chord is bound outside this pane), so a pane
        // that refuses `#action.undo` at the same moment is inconsistent with its own keybinding. That
        // inconsistency is the whole defect — `pointer-events-none` swallows the click AND the handler
        // short-circuits, so the row reported as inert with no fault and no console line while
        // `framework.history.undo` (ungated) undid the same document (measured 2026-09-17, ◻️2d battery
        // `undo-action-row-changes-document`; every lane after the first `armUtility` was affected).
        const rowDisabled = disabled && !FRAMEWORK_RESERVED_ACTION_IDS.has(action.id);
        const rowClassName = rowDisabled ? "pointer-events-none opacity-50" : undefined;
        if (!actionRequiresStagedForm(action)) {
          return { id: `action.${action.id}`, label: action.label, icon, className: rowClassName, onClick: () => !rowDisabled && onExecute({ controllerId, action: action.id }) };
        }
        const expanded = expandedActionId === action.id;
        return {
          id: `action.${action.id}`,
          label: `${action.label}…`,
          icon: icon ?? <Icon icon={expanded ? "chevron-down" : "chevron-right"} size="small" />,
          className: rowClassName,
          onClick: () => !rowDisabled && onExpandedChange(expanded ? null : action.id),
        };
      }),
    });
    if (expandedAction && actionCategoryId(expandedAction) === category.id) {
      const staged = stagedArgsByKey[actionStageKey(windowId, expandedAction.id)] ?? {};
      const effective = effectiveActionArgs(expandedAction.args, staged);
      const missing = unresolvedActionArgs(expandedAction.args, effective);
      sections.push({
        id: `action.category.${category.id}.form`,
        // 🏷️ The staged-argument form is a SECTION of the Actions pane and every section a user reads
        // carries a name. It had none, so the export dialog's own argument list announced as an unnamed
        // group to a screen reader and read as an untranslated row to the German pass (measured on
        // generation3d 6018: `actions-pane-de` → `unlabelled: ["action.category.actions.form"]`). The
        // expanded action's OWN label is already resolved against the host's locale, and it is what the
        // form is for — no second string to translate, and it says WHICH action is being staged.
        label: expandedAction.label,
        defaultOpen: true,
        items: expandedAction.args.map(
          (def): TreeDataItem => ({
            id: `action.${expandedAction.id}.arg.${def.id}`,
            label: def.label,
            description: def.description,
            control: renderStagedArgControl(def, effective[def.id], (value) => onStageArg(expandedAction.id, def.id, value), disabled),
          }),
        ),
        actions: [
          {
            id: childElementId("framework.window", windowId, "action", expandedAction.id, "execute"),
            icon: <Icon icon="check" size="small" />,
            text: shellLabel("ui.common.execute"),
            disabled: disabled || missing.length > 0,
            reason: missing.map((id) => expandedAction.args.find((def) => def.id === id)!.label).join(", ") || undefined,
            onClick: () => { if (!disabled && missing.length === 0) onExecute({ controllerId, action: expandedAction.id, args: effective }); },
          },
          {
            id: childElementId("framework.window", windowId, "action", expandedAction.id, "reset"),
            icon: <Icon icon="undo" size="small" />,
            text: shellLabel("ui.common.reset"),
            disabled,
            onClick: () => onResetArgs(expandedAction.id),
          },
        ],
      });
    }
  }
  return sections;
}

/** 🎛️ Props for the per-window Action rail body (P1/P2). */
export type WindowActionPaneProps = {
  readonly windowId: string;
  readonly controllerId: string;
  readonly actions: readonly ResolvedActionDefinition[];
  readonly expandedActionId: string | null;
  readonly stagedArgsByKey: Readonly<Record<string, Readonly<Record<string, unknown>>>>;
  readonly disabled: boolean;
  readonly onExpandedChange: (actionId: string | null) => void;
  readonly onStageArg: (actionId: string, argId: string, value: unknown) => void;
  readonly onResetArgs: (actionId: string) => void;
  readonly onExecute: (descriptor: ActionDescriptor) => void;
  readonly appLabelsOverlay?: PluginAppLabelsOverlay;
};

/**
 * 🎛️ The per-window Actions rail body (P1/P2), grouped into categories like the command panel. Zero-arg
 * actions fire directly; arg-carrying actions expand a locally-buffered staged form (same inline
 * property-row controls as utility measures) — nothing dispatches on edit, effective value is
 * `staged ?? default ?? unset`, Execute is enabled only when every required arg has an effective value,
 * fires exactly ONE `ActionDescriptor` with the merged args, and keeps the staged values afterward.
 * When `disabled` (an active utility with `allowsActionsWhileActive === false`), every row renders disabled.
 */
export function WindowActionPane(props: WindowActionPaneProps): ReactElement {
  const { windowId, controllerId, actions, expandedActionId, stagedArgsByKey, disabled, onExpandedChange, onStageArg, onResetArgs, onExecute, appLabelsOverlay } = props;
  const sections = buildActionCategoryTree(windowId, controllerId, actions, expandedActionId, stagedArgsByKey, disabled, onExpandedChange, onStageArg, onResetArgs, onExecute, appLabelsOverlay);
  // 📜️ The rail is its OWN scroll container, not a block that grows past the pane.
  //
  // `Tree` clips itself (`data-slot="tree"` carries `overflow-hidden`), so a rail taller than the pane
  // hid its own last rows instead of handing them to the pane body's scroller: measured on generation3d
  // at 1600×1000 with 32 rows and the app panels raised, `[data-slot="tree"]` rendered 960 px tall inside
  // a 901 px pane body and the expanded `addWidget` form's `kind` combobox sat at y=1038, below the
  // 1000 px viewport — `click` timed out in the shared bar probe and the verb dispatched with no staged
  // argument (ticket 26/09/18, B3c §4.3's reproduction, re-measured by FL1 in `🗑️generated/fl1-*-rail-*.txt`).
  // `min-h-0 flex-1` lets the pane body shrink this box to the band it actually has, and `overflow-y-auto`
  // makes the rows past that band reachable — by wheel, by keyboard focus, and by a probe's
  // `scrollIntoViewIfNeeded`. `shrink-0` on the tree is the other half and not optional: as a flex item the
  // tree would otherwise be shrunk to the band too and clip the very rows this is trying to reach (measured
  // in between: pane 862, tree offsetHeight 862 with scrollHeight 960, nothing scrollable anywhere).
  // A pane whose body has no definite height is unchanged: `flex-1` has nothing to shrink against and the
  // rail renders exactly as before.
  return (
    <div data-slot="window-action-pane" className="flex min-h-0 min-w-0 flex-1 flex-col overflow-y-auto overscroll-contain">
      <Tree className="shrink-0" sections={sections} showLines={false} sortableSections={false} />
    </div>
  );
}

/** 🧰️ Slice of the {@link ActionPaneState} the {@link windowActionPaneNode} builder reads. */
export type ActionPaneSlice = Pick<ActionPaneState, "expandedByWindowId" | "stagedArgsByKey" | "activeUtilityByWindowId">;

/**
 * 🧰️ Sibling of {@link utilityBarNode}: resolves a window kind's panel-eligible actions and returns a
 * bound {@link WindowActionPane}, or `undefined` when the window has no panel-eligible action (so the rail
 * chip never renders). Rows render disabled while an active utility gates actions
 * (`allowsActionsWhileActive === false`).
 *
 * 🧹️ Panel-eligible means `inPalette` — the SAME curation `resolveCommands` gives the palette and
 * `buildShellContextMenuItems` gives the shell fallback menu: an app declares its raw dispatch verbs
 * (`worldPointerDown`, `registerBrushMesh`, `targetBrushSuggestions`, …) and the framework declares its reserved
 * ones (`interactionSelect`, `noteShellCommand`, `setActiveUtility`, …) as window actions purely so a
 * surface can dispatch them, and a rail that renders them buries the user's own verbs: the puzzle3d
 * perspective rail carried 96 rows and put `Export` at y=1990 inside an 807 px band, which is what
 * `export-only` could not press (`📓️2026-09-13-wave-B53-nakagin-export-full-run.md` §3).
 */
export function windowActionPaneNode(
  app: AppDefinition,
  windowKind: AppWindowKindDefinition,
  windowId: string,
  actionPane: ActionPaneSlice,
  onAction: (action: ActionDescriptor) => void,
  dispatch: (action: ShellAction) => void,
  appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
  terminology: string = UI_TERMINOLOGY_NATIVE,
  locale: string = SHELL_LOCALES[0],
  manifests: readonly { readonly apps: readonly unknown[] }[] = [],
  selectedArtifactKinds?: readonly ArtifactKindChoice[],
): ReactNode {
  const resolvedActions = resolveWindowActions(app, windowKind).filter((action) => action.inPalette);
  if (resolvedActions.length === 0) return undefined;
  const actions = resolvedActions.map((action) => ({
    ...action,
    label: resolveAppLabel(appLabelsOverlay, "action", action.id, resolveManifestLabel(action.label, terminology, locale)),
    args: action.args.map((def) => resolveActionArgDef(def, action.id, appLabelsOverlay, terminology, locale, manifests, selectedArtifactKinds)),
  }));
  const activeUtilityId = actionPane.activeUtilityByWindowId[windowId] ?? null;
  const activeUtility = activeUtilityId ? (app.utilities ?? []).find((utility) => utility.id === activeUtilityId) : undefined;
  const disabled = Boolean(activeUtility && activeUtility.allowsActionsWhileActive === false);
  return (
    <WindowActionPane
      windowId={windowId}
      controllerId={app.controllerId}
      actions={actions}
      expandedActionId={actionPane.expandedByWindowId[windowId] ?? null}
      stagedArgsByKey={actionPane.stagedArgsByKey}
      disabled={disabled}
      onExpandedChange={(actionId) => dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId, value: actionId })}
      onStageArg={(actionId, argId, value) => dispatch({ type: "STAGE_ACTION_ARG", windowId, actionId, argId, value })}
      onResetArgs={(actionId) => dispatch({ type: "RESET_ACTION_ARGS", windowId, actionId })}
      onExecute={onAction}
      appLabelsOverlay={appLabelsOverlay}
    />
  );
}
//#endregion 🧰️WindowActionPane
//#endregion ShellHelpers
