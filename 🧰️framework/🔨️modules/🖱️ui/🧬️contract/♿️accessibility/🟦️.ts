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
};

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
      return "switch";
    case "keyValueList":
      return "list";
    case "slider":
    case "ring":
      return "slider";
    case "numberStepper":
      return "spinbutton";
    case "iconSelect":
      return "radiogroup";
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
  }
}

/** ⌨️ Whether a component takes keyboard focus of its own — the same closed set the Rust
 * `accessibility_is_focusable` names, so a projection and a Tab traversal can never disagree. */
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
      return true;
    case "container":
      return activatable;
    default:
      return false;
  }
}

/** ♿️ Projects ONE published node record. Pure over the record: a renderer's own walk supplies
 * `depth` and afterwards stamps whatever live state only it knows (`focused`, and its laid-out
 * rect). A hidden node is KEPT, carrying `hidden` — dropping it would disagree with the DOM
 * renderer, which renders the element and marks it `aria-hidden`. */
export function uiAccessibilityProjectionNodeV1(record: UiNodeRecord, depth: number): UiAccessibilityProjectionNodeV1 {
  const bindings = record.bindings ?? [];
  const activatable = bindings.some((binding) => binding.trigger === "activate");
  const accessibility = (record.accessibility ?? {}) as Partial<AccessibilitySpec>;
  return {
    nodeId: record.id,
    key: record.key,
    role: uiAccessibilityRoleV1(record.component, activatable),
    depth,
    label: accessibility.label ?? null,
    description: accessibility.description ?? null,
    live: accessibility.live ?? "off",
    shortcut: accessibility.shortcut ?? null,
    hidden: accessibility.hidden ?? false,
    disabled: record.disabled ?? false,
    focusable: uiAccessibilityIsFocusableV1(record.component, activatable),
    actionable: activatable || bindings.length > 0,
    focused: false,
  };
}

/** 🏷️ The subset a reader can reach BY NAME: not hidden, focusable or actionable, and carrying a
 * label of its own. The mirror still exposes the whole projection — a live region is announced
 * without ever being reachable — so this is the reach law, not the exposure law. */
export function uiAccessibilityAnnouncedV1(projection: readonly UiAccessibilityProjectionNodeV1[]): readonly UiAccessibilityProjectionNodeV1[] {
  return projection.filter((node) => !node.hidden && (node.focusable || node.actionable) && node.label !== null);
}

//#endregion ♿️Projection
