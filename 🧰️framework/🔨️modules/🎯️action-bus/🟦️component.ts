// #region 🎯️ActionBus
/// <reference types="vitest/importMeta" />
/** @emoji 🎯️ `@semio-tech/framework` — action arg resolution and utility/tool derivation helpers. */
import type { IconName } from "@semio-tech/assets";
import {
  type ActionArgDef,
  type ActionDefinition,
  type ToolDefinition,
  type ToolRef,
  type UtilityCategory,
  type UtilityNode,
  type WindowMeasure,
  SET_ACTIVE_TOOL_ACTION_ID,
  SET_ACTIVE_UTILITY_ACTION_ID,
} from "../🛂️manifest/🟦️component.ts";

//#region 🧰️ActionArgsAndUtilities
/** 🧰️ A resolved utility ready for the utility bar — the TS twin of Rust `DerivedUtilitySpec` in `ui_wgpu`. */
export type DerivedUtilitySpec = {
  readonly id: string;
  readonly label: string;
  readonly iconId: IconName;
  readonly group?: string;
  readonly groupLabel?: string;
  readonly category?: UtilityCategory;
};

/**
 * 🧰️ Hand-written twin of Rust `derive_utility_nodes` (`framework/ui/wgpu/rs/lib.rs`): builds the utility bar node tree
 * from resolved utilities + the host-owned active utility id. Each utility becomes a `toggle` whose `pressed`
 * reflects `activeUtilityId === id` and whose `onChange` dispatches `setActiveUtility { utilityId }`; utilities
 * sharing a `group` collapse into one `collection` placed where the group first appears. A group that ends
 * with exactly one child is hoisted to a top-level toggle (no nested Transform/Transform pair).
 */
export function deriveUtilityNodes(controllerId: string, utilities: readonly DerivedUtilitySpec[], activeUtilityId?: string): UtilityNode[] {
  const toggle = (utility: DerivedUtilitySpec): UtilityNode => ({
    id: utility.id,
    kind: "toggle",
    iconId: utility.iconId,
    label: utility.label,
    title: utility.label,
    pressed: activeUtilityId === utility.id,
    category: utility.category,
    onChange: { controllerId, action: SET_ACTIVE_UTILITY_ACTION_ID, args: { utilityId: utility.id } },
  });
  const nodes: UtilityNode[] = [];
  const groupIndex = new Map<string, number>();
  for (const utility of utilities) {
    const node = toggle(utility);
    if (utility.group === undefined) {
      nodes.push(node);
      continue;
    }
    const existing = groupIndex.get(utility.group);
    if (existing !== undefined) {
      const collection = nodes[existing] as Extract<UtilityNode, { kind: "collection" }>;
      (collection.children as UtilityNode[]).push(node);
    } else {
      groupIndex.set(utility.group, nodes.length);
      const groupLabel = utility.groupLabel ?? utility.group;
      nodes.push({ id: `group:${utility.group}`, kind: "collection", iconId: utility.iconId, label: groupLabel, title: groupLabel, category: utility.category, children: [node] });
    }
  }
  return nodes.map((node) => (node.kind === "collection" && node.children.length === 1 ? node.children[0]! : node));
}

/**
 * 🎯️ Hand-written twin of Rust `partition_window_measures` (`framework/ui/wgpu/rs/lib.rs`): splits a window's
 * top-level measures into `general` and `utilityOptions`. A top-level `group` tagged with `activeUtilityId`
 * contributes its **children** to `utilityOptions` only when it equals the window's active utility (the
 * tagged wrapper is routing-only and never rendered), and is dropped from both buckets otherwise. Untagged
 * groups and non-group top-level measures stay in `general`, unchanged.
 */
export function partitionWindowMeasures(measures: readonly WindowMeasure[], activeUtilityId?: string): { readonly general: WindowMeasure[]; readonly utilityOptions: WindowMeasure[] } {
  const general: WindowMeasure[] = [];
  const utilityOptions: WindowMeasure[] = [];
  for (const measure of measures) {
    if (measure.kind === "group" && measure.activeUtilityId !== undefined) {
      if (measure.activeUtilityId === activeUtilityId) utilityOptions.push(...measure.children);
      continue;
    }
    general.push(measure);
  }
  return { general, utilityOptions };
}

/**
 * 🧮️ Hand-written twin of Rust `effective_action_args`: for each declared arg, the staged value if
 * present, else its declared `default`, else omitted.
 */
export function effectiveActionArgs(defs: readonly ActionArgDef[], staged: Readonly<Record<string, unknown>>): Record<string, unknown> {
  const effective: Record<string, unknown> = {};
  for (const def of defs) {
    if (Object.prototype.hasOwnProperty.call(staged, def.id)) {
      effective[def.id] = staged[def.id];
    } else if (def.default !== undefined && def.default !== null) {
      effective[def.id] = def.default;
    }
  }
  return effective;
}

/**
 * ❗️ Hand-written twin of Rust `missing_required_args`: ids of required args still unset in `effective`
 * (absent, null, or an empty string).
 */
export function missingRequiredArgs(defs: readonly ActionArgDef[], effective: Readonly<Record<string, unknown>>): string[] {
  return defs
    .filter((def) => def.required)
    .filter((def) => {
      const value = effective[def.id];
      return value === undefined || value === null || value === "";
    })
    .map((def) => def.id);
}

/**
 * 📇️ Returns the definitions owned by one window kind in declaration order.
 */
export function resolveWindowActions(
  _app: { readonly windowKinds: readonly { readonly actions?: readonly ActionDefinition[] }[] },
  windowKind: { readonly actions?: readonly ActionDefinition[] },
): ActionDefinition[] {
  const resolved: ActionDefinition[] = [];
  const seen = new Set<string>();
  for (const action of windowKind.actions ?? []) {
    if (action && !seen.has(action.id)) {
      seen.add(action.id);
      resolved.push(action);
    }
  }
  return resolved;
}

/**
 * 🛠️ Hand-written twin of Rust `resolve_mode_tools`: resolves the active mode's tools in declared
 * order. Unlike `resolveWindowActions`, unresolvable or unreferenced tools have no orphan fallback —
 * tools are opt-in per mode, never automatically shown everywhere.
 */
export function resolveModeTools(
  app: { readonly tools?: readonly ToolDefinition[]; readonly modes: readonly { readonly id: string; readonly tools?: readonly ToolRef[] }[] } | undefined,
  activeModeId: string | undefined,
): ToolDefinition[] {
  const tools = app?.tools ?? [];
  const mode = app?.modes.find((entry) => entry.id === activeModeId);
  if (!mode) return [];
  const resolved: ToolDefinition[] = [];
  const seen = new Set<string>();
  for (const ref of mode.tools ?? []) {
    const tool = tools.find((entry) => entry.id === ref);
    if (tool && !seen.has(tool.id)) {
      seen.add(tool.id);
      resolved.push(tool);
    }
  }
  return resolved;
}
//#endregion 🧰️ActionArgsAndUtilities
// #endregion 🎯️ActionBus
