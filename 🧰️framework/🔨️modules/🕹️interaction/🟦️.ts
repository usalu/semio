// #region 🕹️Interaction
/** @emoji 🕹️ Handcrafted TS parity of `🦀️.rs`'s `InteractionDefinition` family and pure
 * hover/selection state machine (`nextSelection`/`nextHover`/`validateState`) — the exact machine
 * Tree/React consume in wave 2, replacing Tree's private `getTreeNextSelectionState`. Plain readonly
 * types/functions only, no external runtime libraries; same semantics, same field names (camelCase
 * mirrors Rust's `#[serde(rename_all = "camelCase")]`), same function names in camelCase. */
import type { IconName } from "@semio-tech/assets";
import type { LocalizedLabel } from "../🛂️manifest/🤖️generated/🎚️ui-axes.ts";
export type { IconName, LocalizedLabel };

// #region 🔖️Definition
/** 🕹️ One interaction domain an app declares (e.g. "graph", "mesh", "ast", "world"). */
export type InteractionDefinition = {
  readonly id: string;
  readonly label: LocalizedLabel;
  /** 🪜️ Non-empty; the first entry is the domain's default granularity. */
  readonly granularities: readonly GranularityDefinition[];
  readonly hierarchy: HierarchyProvider;
  readonly hover: HoverSpec;
  readonly selection: SelectionSpec;
};

/** 🔬️ One selectable/hoverable level of detail within a domain. */
export type GranularityDefinition = {
  readonly id: string;
  readonly label: LocalizedLabel;
  readonly iconId: IconName;
};

/** 🌳️ Where a domain's target ids come from — mirrors Rust's internally tagged (`kind`) enum. */
export type HierarchyProvider =
  | { readonly kind: "flat" }
  | { readonly kind: "topology" }
  | { readonly kind: "uiTree" }
  | { readonly kind: "pathDelimited"; readonly delimiter: string };

/** 🐁️ One domain's hover behavior. */
export type HoverSpec = {
  readonly enabled: boolean;
  readonly transitive: boolean;
  readonly channels: readonly string[];
  readonly broadcast: boolean;
};

/** 🐁️ `HoverSpec` with the same defaults as Rust's `impl Default for HoverSpec`. */
export const DEFAULT_HOVER_SPEC: HoverSpec = { enabled: true, transitive: false, channels: ["pointer"], broadcast: true };

/** 🖱️ One domain's selection behavior. */
export type SelectionSpec = {
  /** 🪜️ Non-empty; the first entry is the domain's default mode. */
  readonly modes: readonly SelectionMode[];
  readonly methods: readonly SelectionMethod[];
  readonly merges: readonly MergeMode[];
  readonly transitive: boolean;
  readonly broadcast: boolean;
};

/** 🔢️ How many targets may be selected at once within a domain. */
export type SelectionMode = "single" | "multiple";

/** 🎯️ How a surface gathers targets for one `interactionSelect` dispatch. */
export type SelectionMethod = "pick" | "rectangle" | "lasso";

/** 🧮️ Set algebra applied when merging new targets into the current selection — see {@link nextSelection}. */
export type MergeMode = "replace" | "additive" | "subtractive" | "invertive" | "range";

/** 📇️ A validated reference into an app's `AppDefinition.interactions` registry — mirrors `ActionRef`/`UtilityRef`. */
export type InteractionRef = string;
// #endregion 🔖️Definition

// #region 🔖️Runtime
/** 🎯️ One addressed target: a granularity id plus the target's own id. */
export type InteractionTarget = {
  readonly granularity: string;
  readonly id: string;
};

/** 🖱️ One domain's current selection. */
export type DomainSelection = {
  readonly granularity: string;
  readonly ids: readonly string[];
  readonly anchorId?: string;
};

/** 🐁️ One domain's current hover on one channel. */
export type DomainHover = {
  readonly channel: string;
  readonly ids: readonly string[];
};

/** 🗺️ Own persisted-local selection + ephemeral-local hover, keyed by domain id. */
export type InteractionState = {
  readonly selection: Readonly<Record<string, DomainSelection>>;
  readonly hover: Readonly<Record<string, DomainHover>>;
  readonly activeMode: Readonly<Record<string, SelectionMode>>;
  readonly activeGranularity: Readonly<Record<string, string>>;
};

/** 🗺️ An empty `InteractionState` — the TS twin of Rust's `#[derive(Default)]`. */
export const EMPTY_INTERACTION_STATE: InteractionState = { selection: {}, hover: {}, activeMode: {}, activeGranularity: {} };
// #endregion 🔖️Runtime

// #region 🔖️Topology
/** 🌳️ One node of a domain's topology: its own granularity and its parent id (absent = a root). */
export type TopologyNode = {
  readonly id: string;
  readonly granularity: string;
  readonly parent?: string;
};

/** 🌲️ One domain's topology, pre-order: `ordered`'s sequence IS the range-selection order. */
export type DomainTopology = {
  readonly ordered: readonly TopologyNode[];
};

/** 🔎️ The pre-order index of `id` in `topo`, or `undefined` when absent. */
export const domainTopologyIndexOf = (topo: DomainTopology, id: string): number | undefined => {
  const index = topo.ordered.findIndex((node) => node.id === id);
  return index === -1 ? undefined : index;
};

/** ✅️ Whether `id` is a known node in `topo`. */
export const domainTopologyContains = (topo: DomainTopology, id: string): boolean => domainTopologyIndexOf(topo, id) !== undefined;

const childrenByParent = (topo: DomainTopology): Map<string, string[]> => {
  const children = new Map<string, string[]>();
  for (const node of topo.ordered) {
    if (node.parent === undefined) continue;
    const siblings = children.get(node.parent);
    if (siblings) siblings.push(node.id);
    else children.set(node.parent, [node.id]);
  }
  return children;
};

const visitDescendants = (id: string, children: Map<string, string[]>, out: string[]): void => {
  out.push(id);
  for (const kid of children.get(id) ?? []) visitDescendants(kid, children, out);
};

/** 🌳️ `rootId` plus every descendant, pre-order (root first) — empty when `rootId` is absent from `topo`. */
export const domainTopologyDescendantClosure = (topo: DomainTopology, rootId: string): string[] => {
  if (!domainTopologyContains(topo, rootId)) return [];
  const out: string[] = [];
  visitDescendants(rootId, childrenByParent(topo), out);
  return out;
};

/** 🪜️ `id`'s ancestor chain, nearest parent first, root last. */
export const domainTopologyAncestors = (topo: DomainTopology, id: string): string[] => {
  const out: string[] = [];
  let current = topo.ordered.find((node) => node.id === id)?.parent;
  while (current !== undefined) {
    const parentId: string = current;
    current = topo.ordered.find((node) => node.id === parentId)?.parent;
    out.push(parentId);
  }
  return out;
};

/** 🗺️ Every domain's topology for one app instance, keyed by domain id. */
export type InteractionTopology = {
  readonly domains: Readonly<Record<string, DomainTopology>>;
};
// #endregion 🔖️Topology

// #region 🔖️SelectionMachine
/** 🖱️ One `nextSelection` call's input: the batch of targets, the merge mode, and the active selection mode. */
export type SelectionInput = {
  readonly targets: readonly InteractionTarget[];
  readonly merge: MergeMode;
  readonly mode: SelectionMode;
};

const dedupPreservingOrder = (ids: readonly string[]): string[] => {
  const out: string[] = [];
  for (const id of ids) if (!out.includes(id)) out.push(id);
  return out;
};

const expandTarget = (spec: Pick<SelectionSpec, "transitive">, topo: DomainTopology, id: string): string[] => {
  if (!spec.transitive) return [id];
  const closure = domainTopologyDescendantClosure(topo, id);
  return closure.length === 0 ? [id] : closure;
};

/**
 * 🖱️ Computes the next `DomainSelection` for one domain — the TS twin of Rust's `next_selection`,
 * itself the generalization of Tree's `getTreeNextSelectionState`
 * (`🖱️ui/🧱️elements/🪵️Tree/🟦️.tsx:946-968`).
 *
 * - `"single"` mode ignores `merge` entirely and clamps to the batch's last target.
 * - `"range"` replaces the selection with the topology-order slice between the anchor (falling back to
 *   `current.anchorId`, then `current.ids`'s last id, then the target itself) and the batch's last
 *   target, ascending index order; the anchor does not move.
 * - `"replace"`/`"additive"`/`"subtractive"`/`"invertive"` apply ordinary set algebra over the batch's
 *   targets (each expanded to its descendant closure first when `spec.transitive`), and update the
 *   anchor to the batch's last target.
 *
 * Empty `input.targets` is a no-op (returns `current` unchanged).
 */
export const nextSelection = (spec: SelectionSpec, current: DomainSelection, topo: DomainTopology, input: SelectionInput): DomainSelection => {
  const lastTarget = input.targets[input.targets.length - 1];
  if (!lastTarget) return current;
  const granularity = lastTarget.granularity;
  const targetIds = input.targets.map((target) => target.id);
  const lastTargetId = lastTarget.id;

  if (input.mode === "single") {
    return { granularity, ids: [lastTargetId], anchorId: lastTargetId };
  }

  if (input.merge === "range") {
    const fallbackAnchor = current.anchorId ?? current.ids[current.ids.length - 1] ?? lastTargetId;
    const anchorIndex = domainTopologyIndexOf(topo, fallbackAnchor);
    const targetIndex = domainTopologyIndexOf(topo, lastTargetId);
    if (anchorIndex !== undefined && targetIndex !== undefined) {
      const start = Math.min(anchorIndex, targetIndex);
      const end = Math.max(anchorIndex, targetIndex);
      return { granularity, ids: topo.ordered.slice(start, end + 1).map((node) => node.id), anchorId: fallbackAnchor };
    }
    return { granularity, ids: [lastTargetId], anchorId: lastTargetId };
  }

  const expanded = targetIds.flatMap((id) => expandTarget(spec, topo, id));

  let ids: string[];
  switch (input.merge) {
    case "replace":
      ids = dedupPreservingOrder(expanded);
      break;
    case "additive": {
      ids = [...current.ids];
      for (const id of expanded) if (!ids.includes(id)) ids.push(id);
      break;
    }
    case "subtractive":
      ids = current.ids.filter((id) => !expanded.includes(id));
      break;
    case "invertive": {
      ids = [...current.ids];
      for (const id of expanded) {
        const index = ids.indexOf(id);
        if (index === -1) ids.push(id);
        else ids.splice(index, 1);
      }
      break;
    }
  }
  return { granularity, ids: dedupPreservingOrder(ids), anchorId: lastTargetId };
};
// #endregion 🔖️SelectionMachine

// #region 🔖️HoverMachine
/** 🐁️ One `nextHover` call's input: the channel and the batch of hovered targets (empty = clear). */
export type HoverInput = {
  readonly channel: string;
  readonly targets: readonly InteractionTarget[];
};

/**
 * 🐁️ Computes the next `DomainHover` for one channel — the TS twin of Rust's `next_hover`: always
 * REPLACES the channel's id list. When `spec.transitive`, each target expands to its descendant
 * closure with the hovered root first; multiple targets concatenate in input order, deduplicated.
 * Disabled specs and empty target batches both clear the channel.
 */
export const nextHover = (spec: HoverSpec, topo: DomainTopology, input: HoverInput): DomainHover => {
  if (!spec.enabled || input.targets.length === 0) {
    return { channel: input.channel, ids: [] };
  }
  const ids: string[] = [];
  for (const target of input.targets) {
    for (const id of expandTarget(spec, topo, target.id)) {
      if (!ids.includes(id)) ids.push(id);
    }
  }
  return { channel: input.channel, ids };
};
// #endregion 🔖️HoverMachine

// #region 🔖️Validation
/**
 * 🧹️ Re-derives a consistent `InteractionState` from declared `defs` + current `topo` — the TS twin of
 * Rust's `validate_state`: drops any domain absent from `defs`, prunes selection/hover ids no longer
 * present in that domain's topology, resets `activeGranularity`/`activeMode` to a declared value when
 * the stored one is no longer declared, and clamps `"single"`-mode selections down to their first id.
 */
export const validateState = (defs: readonly InteractionDefinition[], topo: InteractionTopology, state: InteractionState): InteractionState => {
  const selection: Record<string, DomainSelection> = {};
  const hover: Record<string, DomainHover> = {};
  const activeMode: Record<string, SelectionMode> = {};
  const activeGranularity: Record<string, string> = {};

  for (const def of defs) {
    const domainTopo = topo.domains[def.id];
    const declaredGranularities = def.granularities.map((granularity) => granularity.id);
    const defaultGranularity = def.granularities[0]?.id ?? "";
    const defaultMode: SelectionMode = def.selection.modes[0] ?? "single";

    const storedMode = state.activeMode[def.id];
    const mode = storedMode && def.selection.modes.includes(storedMode) ? storedMode : defaultMode;
    activeMode[def.id] = mode;

    const storedGranularity = state.activeGranularity[def.id];
    activeGranularity[def.id] = storedGranularity && declaredGranularities.includes(storedGranularity) ? storedGranularity : defaultGranularity;

    const domainSelection = state.selection[def.id];
    if (domainSelection) {
      const granularity = declaredGranularities.includes(domainSelection.granularity) ? domainSelection.granularity : defaultGranularity;
      let ids = domainSelection.ids.filter((id) => (domainTopo ? domainTopologyContains(domainTopo, id) : true));
      if (mode === "single" && ids.length > 1) ids = ids.slice(0, 1);
      const anchorId = domainSelection.anchorId && ids.includes(domainSelection.anchorId) ? domainSelection.anchorId : undefined;
      selection[def.id] = { granularity, ids, anchorId };
    }

    const domainHover = state.hover[def.id];
    if (domainHover) {
      const ids = domainHover.ids.filter((id) => (domainTopo ? domainTopologyContains(domainTopo, id) : true));
      hover[def.id] = { channel: domainHover.channel, ids };
    }
  }

  return { selection, hover, activeMode, activeGranularity };
};
// #endregion 🔖️Validation
// #endregion 🕹️Interaction
