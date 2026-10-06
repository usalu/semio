import { formatUiNumber, roundUiNumber } from "../🔢️number-format/🟦️.ts";
import { UI_NUMBER_PRECISION_MAX, uiNumberDisplay, uiNumberDisplayText } from "../🧩️component/🟦️.ts";
/**
 * ♿️ The TypeScript twin of the contract's own `♿️accessibility/🦀️.rs` projection region.
 *
 * `AccessibilitySpec` deliberately carries no `role`: a `Component::Button` is a button on every
 * renderer, so the role is IMPLIED by the component. React reaches that implication for free — each
 * component view renders an HTML element whose tag already carries the role — while a renderer with
 * no DOM has to say it out loud. This module is where the implication is written down for the
 * TypeScript side, answering the same `🧫️fixtures/♿️accessibility-projection.json` rows the Rust
 * `accessibility_role`/`accessibility_projection_node` laws answer, so neither side can drift into a
 * role vocabulary of its own.
 *
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
 */
import type { AccessibilitySpec, Component, ContainerRole, Liveness, UiNodeRecord } from "../../../🛂️manifest/🟦️.ts";

//#region ♿️Projection

/** ♿️ One node of the accessibility projection a renderer publishes — the TS mirror of the Rust
 * `AccessibilityProjectionNode`. Flat and ordered: `depth` carries the tree shape. */
export type UiAccessibilityProjectionNodeV1 = {
  readonly nodeId: number;
  readonly key: string;
  readonly role: string;
  readonly depth: number;
  readonly label: string | null;
  readonly description: string | null;
  readonly live: Liveness;
  readonly shortcut: string | null;
  readonly hidden: boolean;
  readonly disabled: boolean;
  readonly focusable: boolean;
  readonly tabbable: boolean;
  readonly actionable: boolean;
  readonly focused: boolean;
  readonly checked: boolean | null;
  readonly pressed: boolean | null;
  readonly selected: boolean | null;
  readonly expanded: boolean | null;
  readonly editable: boolean;
  readonly multiline: boolean;
  readonly controls: string | null;
  readonly activeDescendant: string | null;
  readonly level: number | null;
  readonly valueMin: number | null;
  readonly valueMax: number | null;
  readonly valueNow: number | null;
  readonly valueText: string | null;
  readonly busy: boolean;
  /** 🪜️ The step a range control moves by, in the display units of `valueMin`/`valueMax`/`valueNow`. */
  readonly valueStep: number | null;
  /** 🚧️ A control whose typed draft is refused (`aria-invalid`); only a renderer's walk knows a draft. */
  readonly invalid: boolean;
  /** 🧮️ The rows of the logical list a windowed row belongs to (`aria-setsize`); a renderer's walk stamps it. */
  readonly setSize: number | null;
  /** 📍️ A windowed row's one-based place in that list (`aria-posinset`). */
  readonly posInSet: number | null;
  /** 🚦️ The semantic tone a tree row paints ({@link uiAccessibilityToneV1}); never the only cue of a state. */
  readonly tone: string | null;
};

/** 📶️ The range semantics a component announces — the TS mirror of the Rust `AccessibilityValue`. */
export type UiAccessibilityValueV1 = Pick<UiAccessibilityProjectionNodeV1, "valueMin" | "valueMax" | "valueNow" | "valueText" | "busy" | "valueStep">;

/** 🗂️ The authored container role's own ARIA name — `plain`/`group`/`field` are all plain grouping
 * (a field is a label plus its control, which is a group, not a landmark), `section` is a region. */
function containerRoleName(role: ContainerRole | undefined): string {
  switch (role ?? "plain") {
    case "section":
      return "region";
    case "form":
      return "form";
    case "toolbar":
      return "toolbar";
    default:
      return "group";
  }
}

/**
 * ♿️ The ARIA role a component implies. `activatable` is the record's own `activate` binding:
 * a container you can press IS a button, exactly as the React Interpreter spells it
 * (`role={activateBinding ? "button" : role}`) — except where the author already declared a landmark
 * (`form`/`toolbar`), which outranks the press.
 */
export function uiAccessibilityRoleV1(component: Component, activatable: boolean): string {
  switch (component.type) {
    case "container": {
      const named = containerRoleName((component as { role?: ContainerRole }).role);
      if (activatable && named !== "form" && named !== "toolbar") return "button";
      return named;
    }
    case "text":
      return "paragraph";
    case "button":
      return "button";
    case "separator":
      return "separator";
    case "input":
      return component.kind === "number" ? "spinbutton" : "textbox";
    case "select":
      return component.appearance === "segmented" ? "radiogroup" : "combobox";
    case "toggle":
      return component.appearance === "checkbox" ? "checkbox" : "button";
    case "keyValueList":
      return "list";
    case "slider":
    case "ring":
      return "slider";
    case "numberStepper":
      return "spinbutton";
    case "iconSelect":
      return "radiogroup";
    case "progress":
      return "progressbar";
    case "tree":
      return "tree";
    case "treeSection":
      return "group";
    case "treeItem":
      return "treeitem";
    case "image":
      return "img";
    case "surface":
      return "application";
    case "extension":
      return "region";
    case "table":
      return "grid";
    case "tableRow":
      return "row";
  }
}

/** ⌨️ Whether a component takes keyboard focus of its own — the same closed set the Rust
 * `accessibility_is_focusable` names, so a projection and a Tab traversal can never disagree.
 *
 * A `surface` is in the set because it is the app's ONLY door to a scene it paints itself: the
 * node-graph and World3d canvases carry their whole interaction surface inside a texture no
 * assistive technology can walk, so a canvas nobody can Tab into is a canvas nobody can drive
 * without a mouse. Its `application` role is exactly the ARIA promise that the widget handles its
 * own arrow/Enter keys. */
export function uiAccessibilityIsFocusableV1(component: Component, activatable: boolean): boolean {
  switch (component.type) {
    case "button":
    case "input":
    case "select":
    case "toggle":
    case "slider":
    case "numberStepper":
    case "ring":
    case "iconSelect":
    case "treeItem":
    case "tableRow":
    case "surface":
      return true;
    case "container":
      return activatable;
    default:
      return false;
  }
}

/** 🔊️ The number a range control announces for a stored value: unchanged without a display factor, else `stored × factor` cleaned to twelve significant digits; the twin of the Rust `spoken_number`. */
function spokenNumber(stored: number, factor: number | null): number {
  if (factor == null) return stored;
  const shown = uiNumberDisplay(stored, factor);
  const cleaned = Number(formatUiNumber(shown));
  return Number.isFinite(cleaned) ? cleaned : shown;
}

/** 🗣️ The spoken value of a range control: its display text and the unit it shows; the twin of the Rust `spoken_text`. */
function spokenText(stored: number, factor: number | null, precision: number | null | undefined, unit: string | null): string {
  const text = uiNumberDisplayText(stored, factor, precision);
  return unit == null || unit === "" ? text : `${text} ${unit}`;
}

/** 👣️ The step a range control announces, in display units: its declared `step`, else (a non-positive or non-finite one) the `10^-precision` display step the keyboard law walks, else none; the twin of the Rust `spoken_step`. */
function spokenStep(step: number | null | undefined, precision: number | null | undefined, factor: number | null): number | null {
  if (step != null && Number.isFinite(step) && step > 0) return spokenNumber(step, factor);
  return precision == null ? null : roundUiNumber(10 ** -Math.min(precision, UI_NUMBER_PRECISION_MAX), precision);
}

/** 🎨️ The tone a tree row announces — `info`, `success`, `warning` or `danger`; `null` for every other component, the neutral tone and the brand roles; the twin of the Rust `accessibility_tone`. */
export function uiAccessibilityToneV1(record: UiNodeRecord): string | null {
  const tone = record.component.type === "treeItem" ? record.style?.tone : undefined;
  return tone === "info" || tone === "success" || tone === "warning" || tone === "danger" ? tone : null;
}

/** 📶️ `aria-valuemin`/`max`/`now`/`valuetext` for a determinate progress bar, only `aria-busy` while it
 * is indeterminate (a total that is absent, never merely zero), and nothing for every other component. */
export function uiAccessibilityValueV1(component: Component): UiAccessibilityValueV1 {
  if (component.type === "input") {
    const numeric = /^[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?$/u.test(component.value) ? Number(component.value) : Number.NaN;
    const factor = component.kind === "number" ? (component.displayFactor ?? null) : null;
    const shown = (value: number | null | undefined): number | null => (value == null ? null : spokenNumber(value, factor));
    const valueText = component.kind === "number" && (component.precision != null || factor != null) && Number.isFinite(numeric) ? uiNumberDisplayText(numeric, factor, component.precision) : component.value;
    return { valueMin: shown(component.min), valueMax: shown(component.max), valueNow: Number.isFinite(numeric) ? shown(numeric) : null, valueText, busy: false, valueStep: component.kind === "number" ? spokenStep(component.step, component.precision, factor) : null };
  }
  if (component.type === "select" || component.type === "iconSelect") return { valueMin: null, valueMax: null, valueNow: null, valueText: component.value, busy: false, valueStep: null };
  if (component.type === "slider") {
    const factor = component.displayFactor ?? null;
    const unit = component.displayUnit ?? component.unit ?? null;
    const described = unit != null || component.precision != null || factor != null;
    return { valueMin: spokenNumber(component.min, factor), valueMax: spokenNumber(component.max, factor), valueNow: spokenNumber(component.value, factor), valueText: described ? spokenText(component.value, factor, component.precision, unit) : null, busy: false, valueStep: spokenStep(component.step, component.precision, factor) ?? spokenNumber(1, factor) };
  }
  if (component.type === "numberStepper") {
    const factor = component.displayFactor ?? null;
    const shown = (value: number | null | undefined): number | null => (value == null ? null : spokenNumber(value, factor));
    return { valueMin: shown(component.min), valueMax: shown(component.max), valueNow: component.uniform ? shown(component.value) : null, valueText: component.uniform ? spokenText(component.value, factor, component.precision, component.displayUnit ?? component.unit ?? null) : null, busy: false, valueStep: spokenStep(component.step, component.precision, factor) ?? spokenNumber(1, factor) };
  }
  if (component.type === "ring") return { valueMin: 0, valueMax: 1, valueNow: component.t, valueText: String(component.t), busy: false, valueStep: null };
  if (component.type !== "progress") return { valueMin: null, valueMax: null, valueNow: null, valueText: null, busy: false, valueStep: null };
  if (component.total == null) return { valueMin: null, valueMax: null, valueNow: null, valueText: null, busy: true, valueStep: null };
  return { valueMin: 0, valueMax: component.total, valueNow: component.completed, valueText: component.valueText, busy: false, valueStep: null };
}

/** 📐️ The filled share in `0..1` of a progress bar, `null` while indeterminate — the twin of the Rust
 * `progress_fraction`: a zero total fills nothing, an overshoot clamps the fill but never the value. */
export function uiProgressFractionV1(completed: number, total: number | null | undefined): number | null {
  if (total == null) return null;
  return total > 0 ? Math.min(1, Math.max(0, completed / total)) : 0;
}

/** ♿️ Projects ONE published node record. Pure over the record: a renderer's own walk supplies
 * `depth` and afterwards stamps whatever live state only it knows (`focused`, and its laid-out
 * rect). A hidden node is KEPT, carrying `hidden` — dropping it would disagree with the DOM
 * renderer, which renders the element and marks it `aria-hidden`. */
export function uiAccessibilityProjectionNodeV1(record: UiNodeRecord, depth: number): UiAccessibilityProjectionNodeV1 {
  const bindings = record.bindings ?? [];
  const activatable = bindings.some((binding) => binding.trigger === "activate");
  const focusable = uiAccessibilityIsFocusableV1(record.component, activatable);
  const accessibility = (record.accessibility ?? {}) as Partial<AccessibilitySpec>;
  const componentLabel =
    record.component.type === "text"
      ? record.component.value
      : record.component.type === "button" || record.component.type === "treeSection" || record.component.type === "treeItem" || record.component.type === "table" || (record.component.type === "container" && (record.component.role === "section" || record.component.role === "group"))
        ? record.component.label
        : null;
  const treeItem = record.component.type === "treeItem" ? record.component : null;
  const treeItemHasOrdinaryChild = treeItem !== null && (record.children ?? []).some((child) => child !== treeItem.inlineToolbar && child !== treeItem.detail);
  const expanded = record.component.type === "select"
    ? (record.component.appearance === "segmented" ? null : false)
    : record.component.type === "treeSection"
      ? record.component.defaultOpen ?? true
      : record.component.type === "treeItem" && (record.component.defaultOpen != null || treeItemHasOrdinaryChild)
        ? record.component.defaultOpen ?? true
        : null;
  return {
    nodeId: record.id,
    key: record.key,
    role: uiAccessibilityRoleV1(record.component, activatable),
    depth,
    label: accessibility.label ?? componentLabel,
    description: accessibility.description ?? null,
    live: accessibility.live ?? "off",
    shortcut: accessibility.shortcut ?? null,
    hidden: accessibility.hidden ?? false,
    disabled: record.disabled ?? false,
    focusable,
    tabbable: focusable,
    actionable: activatable || bindings.length > 0,
    focused: false,
    checked: record.component.type === "toggle" && record.component.appearance === "checkbox" ? record.component.on : null,
    pressed: record.component.type === "toggle" && record.component.appearance !== "checkbox" ? record.component.on : null,
    selected: null,
    expanded,
    editable: false,
    multiline: false,
    controls: null,
    activeDescendant: null,
    level: record.component.type === "treeItem" ? depth + 1 : null,
    ...uiAccessibilityValueV1(record.component),
    invalid: false,
    setSize: null,
    posInSet: null,
    tone: uiAccessibilityToneV1(record),
  };
}

/** 🏷️ The subset a reader can reach BY NAME: not hidden, focusable or actionable, and carrying a
 * label of its own. The mirror still exposes the whole projection — a live region is announced
 * without ever being reachable — so this is the reach law, not the exposure law. */
export function uiAccessibilityAnnouncedV1(projection: readonly UiAccessibilityProjectionNodeV1[]): readonly UiAccessibilityProjectionNodeV1[] {
  return projection.filter((node) => !node.hidden && (node.focusable || node.actionable) && node.label !== null);
}

//#endregion ♿️Projection
