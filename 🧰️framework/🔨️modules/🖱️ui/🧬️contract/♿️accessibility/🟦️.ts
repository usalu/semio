/**
 * @emoji ♿️ The TypeScript twin of the contract's own `♿️accessibility/🦀️.rs` projection region.
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
};

/** 📶️ The range semantics a component announces — the TS mirror of the Rust `AccessibilityValue`. */
export type UiAccessibilityValueV1 = Pick<UiAccessibilityProjectionNodeV1, "valueMin" | "valueMax" | "valueNow" | "valueText" | "busy">;

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
 * @emoji ♿️ The ARIA role a component implies. `activatable` is the record's own `activate` binding:
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
      return "textbox";
    case "select":
      return "combobox";
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

/** 📶️ `aria-valuemin`/`max`/`now`/`valuetext` for a determinate progress bar, only `aria-busy` while it
 * is indeterminate (a total that is absent, never merely zero), and nothing for every other component. */
export function uiAccessibilityValueV1(component: Component): UiAccessibilityValueV1 {
  if (component.type === "input") {
    const numeric = /^[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?$/u.test(component.value) ? Number(component.value) : Number.NaN;
    return { valueMin: component.min ?? null, valueMax: component.max ?? null, valueNow: Number.isFinite(numeric) ? numeric : null, valueText: component.value, busy: false };
  }
  if (component.type === "select" || component.type === "iconSelect") return { valueMin: null, valueMax: null, valueNow: null, valueText: component.value, busy: false };
  if (component.type === "slider") return { valueMin: component.min, valueMax: component.max, valueNow: component.value, valueText: component.unit == null ? null : `${component.value} ${component.unit}`, busy: false };
  if (component.type === "numberStepper") return { valueMin: null, valueMax: null, valueNow: component.value, valueText: String(component.value), busy: false };
  if (component.type === "ring") return { valueMin: 0, valueMax: 1, valueNow: component.t, valueText: String(component.t), busy: false };
  if (component.type !== "progress") return { valueMin: null, valueMax: null, valueNow: null, valueText: null, busy: false };
  if (component.total == null) return { valueMin: null, valueMax: null, valueNow: null, valueText: null, busy: true };
  return { valueMin: 0, valueMax: component.total, valueNow: component.completed, valueText: component.valueText, busy: false };
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
  const accessibility = (record.accessibility ?? {}) as Partial<AccessibilitySpec>;
  const componentLabel = record.component.type === "button" || record.component.type === "treeItem" || record.component.type === "table" ? record.component.label : null;
  const treeItem = record.component.type === "treeItem" ? record.component : null;
  const treeItemHasOrdinaryChild = treeItem !== null && (record.children ?? []).some((child) => child !== treeItem.inlineToolbar && child !== treeItem.detail);
  const expanded = record.component.type === "select"
    ? false
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
    focusable: uiAccessibilityIsFocusableV1(record.component, activatable),
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
  };
}

/** 🏷️ The subset a reader can reach BY NAME: not hidden, focusable or actionable, and carrying a
 * label of its own. The mirror still exposes the whole projection — a live region is announced
 * without ever being reachable — so this is the reach law, not the exposure law. */
export function uiAccessibilityAnnouncedV1(projection: readonly UiAccessibilityProjectionNodeV1[]): readonly UiAccessibilityProjectionNodeV1[] {
  return projection.filter((node) => !node.hidden && (node.focusable || node.actionable) && node.label !== null);
}

//#endregion ♿️Projection
