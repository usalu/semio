// #region 🧬️Schema
/** 🧬️ Schema leaf: canonical TS mirror of `🔣️.json` for the 🕹️interaction module.
 * The declaration types live at the module root (`../🟦️.ts`) and are published here under their
 * own export names, one per `$defs` entry, because execution contract §A defines a TypeScript
 * export as an exported `interface`/`type` AND a `parse<Export>()` entry point: a type alone is
 * erased at runtime, so a consumer handed a `schema://framework.interaction/<Export>` payload would
 * have no way into the contract. `PresenceInteraction`/`PresenceDomain` — the broadcast payload
 * shape — are newly defined here rather than re-published.
 * @see 🧬️schema/🔣️.json for the normative draft-07 definitions these parsers enforce. */
import type * as root from "../🟦️.ts";

export type InteractionDefinition = root.InteractionDefinition;
export type GranularityDefinition = root.GranularityDefinition;
export type HierarchyProvider = root.HierarchyProvider;
export type HoverSpec = root.HoverSpec;
export type SelectionSpec = root.SelectionSpec;
export type SelectionMode = root.SelectionMode;
export type SelectionMethod = root.SelectionMethod;
export type MergeMode = root.MergeMode;
export type InteractionTarget = root.InteractionTarget;
export type DomainSelection = root.DomainSelection;
export type DomainHover = root.DomainHover;
export type InteractionState = root.InteractionState;
export type TopologyNode = root.TopologyNode;
export type DomainTopology = root.DomainTopology;
export type { InteractionRef, InteractionTopology } from "../🟦️.ts";

/** 📡️ One domain's broadcast slice of `PresenceInteraction` — the peer-facing mirror of a domain's
 * `DomainSelection`/`DomainHover`, flattened to raw explicit ids (no transitive expansion on the wire). */
export type PresenceDomain = {
  readonly domain: string;
  readonly granularity: string;
  readonly selected: readonly string[];
  readonly hovered: readonly string[];
};

/** 📡️ One peer's interaction roster for one app instance, mirrored onto `PresencePeer.interaction`
 * (bit 7) on the heartbeat. Only explicit ids broadcast; receivers expand transitive closures via
 * their own topology. */
export type PresenceInteraction = {
  readonly appId: string;
  readonly domains: readonly PresenceDomain[];
};
// #endregion 🧬️Schema

// #region 🚪️Parsers
/** ❌️ Every refusal carries the export it refused and the member it refused on, so a caller can act
 * on the failure without re-deriving where in the payload it happened. */
class InteractionSchemaError extends Error {
  constructor(exported: string, member: string, reason: string) {
    super(`framework.interaction/${exported}: ${member} ${reason}`);
    this.name = "InteractionSchemaError";
  }
}

type Members = Readonly<Record<string, unknown>>;

function members(value: unknown, exported: string, at: string, required: readonly string[], optional: readonly string[] = []): Members {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new InteractionSchemaError(exported, at, "is not an object");
  const entry = value as Members;
  for (const name of required) if (!Object.hasOwn(entry, name)) throw new InteractionSchemaError(exported, `${at}.${name}`, "is required and absent");
  for (const name of Object.keys(entry)) if (!required.includes(name) && !optional.includes(name)) throw new InteractionSchemaError(exported, `${at}.${name}`, "is not a declared member");
  return entry;
}

function text(value: unknown, exported: string, at: string): string {
  if (typeof value !== "string") throw new InteractionSchemaError(exported, at, "is not a string");
  return value;
}

function flag(value: unknown, exported: string, at: string): boolean {
  if (typeof value !== "boolean") throw new InteractionSchemaError(exported, at, "is not a boolean");
  return value;
}

function opaque(value: unknown, exported: string, at: string): Members {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new InteractionSchemaError(exported, at, "is not an object");
  return value as Members;
}

function list<T>(value: unknown, exported: string, at: string, minimum: number, item: (entry: unknown, at: string) => T): readonly T[] {
  if (!Array.isArray(value)) throw new InteractionSchemaError(exported, at, "is not an array");
  if (value.length < minimum) throw new InteractionSchemaError(exported, at, `has fewer than ${minimum} item(s)`);
  return value.map((entry, index) => item(entry, `${at}[${index}]`));
}

function keyed<T>(value: unknown, exported: string, at: string, item: (entry: unknown, at: string) => T): Readonly<Record<string, T>> {
  const entry = opaque(value, exported, at);
  return Object.fromEntries(Object.entries(entry).map(([key, member]) => [key, item(member, `${at}.${key}`)]));
}

function member<T extends string>(value: unknown, exported: string, at: string, allowed: readonly T[]): T {
  const declared = text(value, exported, at);
  if (!(allowed as readonly string[]).includes(declared)) throw new InteractionSchemaError(exported, at, `is not one of ${allowed.join(", ")}`);
  return declared as T;
}

/** 🔢️ `$defs.SelectionMode` — how many targets may be selected at once within a domain. */
export function parseSelectionMode(value: unknown, at = "SelectionMode"): SelectionMode {
  return member(value, "SelectionMode", at, ["single", "multiple"] as const);
}

/** 🎯️ `$defs.SelectionMethod` — how a surface gathers targets for one dispatch. */
export function parseSelectionMethod(value: unknown, at = "SelectionMethod"): SelectionMethod {
  return member(value, "SelectionMethod", at, ["pick", "rectangle", "lasso"] as const);
}

/** 🧮️ `$defs.MergeMode` — the set algebra a selection merge applies. */
export function parseMergeMode(value: unknown, at = "MergeMode"): MergeMode {
  return member(value, "MergeMode", at, ["replace", "additive", "subtractive", "invertive", "range"] as const);
}

/** 🔬️ `$defs.GranularityDefinition` — one selectable level of detail within a domain. */
export function parseGranularityDefinition(value: unknown, at = "GranularityDefinition"): GranularityDefinition {
  const entry = members(value, "GranularityDefinition", at, ["id", "label", "iconId"]);
  return { id: text(entry.id, "GranularityDefinition", `${at}.id`), label: opaque(entry.label, "GranularityDefinition", `${at}.label`), iconId: text(entry.iconId, "GranularityDefinition", `${at}.iconId`) } as GranularityDefinition;
}

/** 🌳️ `$defs.HierarchyProvider` — the internally tagged (`kind`) source of a domain's target ids. */
export function parseHierarchyProvider(value: unknown, at = "HierarchyProvider"): HierarchyProvider {
  const tag = text(members(value, "HierarchyProvider", at, ["kind"], ["delimiter"]).kind, "HierarchyProvider", `${at}.kind`);
  if (tag === "flat" || tag === "topology" || tag === "uiTree") {
    members(value, "HierarchyProvider", at, ["kind"]);
    return { kind: tag };
  }
  if (tag !== "pathDelimited") throw new InteractionSchemaError("HierarchyProvider", `${at}.kind`, "is not one of flat, topology, uiTree, pathDelimited");
  const entry = members(value, "HierarchyProvider", at, ["kind", "delimiter"]);
  return { kind: "pathDelimited", delimiter: text(entry.delimiter, "HierarchyProvider", `${at}.delimiter`) };
}

/** 🐁️ `$defs.HoverSpec` — one domain's hover behavior. */
export function parseHoverSpec(value: unknown, at = "HoverSpec"): HoverSpec {
  const entry = members(value, "HoverSpec", at, ["enabled", "transitive", "channels", "broadcast"]);
  return {
    enabled: flag(entry.enabled, "HoverSpec", `${at}.enabled`),
    transitive: flag(entry.transitive, "HoverSpec", `${at}.transitive`),
    channels: list(entry.channels, "HoverSpec", `${at}.channels`, 0, (item, itemAt) => text(item, "HoverSpec", itemAt)),
    broadcast: flag(entry.broadcast, "HoverSpec", `${at}.broadcast`),
  };
}

/** 🖱️ `$defs.SelectionSpec` — one domain's selection behavior; `modes` is non-empty and its first entry is the default. */
export function parseSelectionSpec(value: unknown, at = "SelectionSpec"): SelectionSpec {
  const entry = members(value, "SelectionSpec", at, ["modes", "methods", "merges", "transitive", "broadcast"]);
  return {
    modes: list(entry.modes, "SelectionSpec", `${at}.modes`, 1, (item, itemAt) => parseSelectionMode(item, itemAt)),
    methods: list(entry.methods, "SelectionSpec", `${at}.methods`, 0, (item, itemAt) => parseSelectionMethod(item, itemAt)),
    merges: list(entry.merges, "SelectionSpec", `${at}.merges`, 0, (item, itemAt) => parseMergeMode(item, itemAt)),
    transitive: flag(entry.transitive, "SelectionSpec", `${at}.transitive`),
    broadcast: flag(entry.broadcast, "SelectionSpec", `${at}.broadcast`),
  };
}

/** 🕹️ `$defs.InteractionDefinition` — one interaction domain an app declares. */
export function parseInteractionDefinition(value: unknown, at = "InteractionDefinition"): InteractionDefinition {
  const entry = members(value, "InteractionDefinition", at, ["id", "label", "granularities", "hierarchy", "hover", "selection"]);
  return {
    id: text(entry.id, "InteractionDefinition", `${at}.id`),
    label: opaque(entry.label, "InteractionDefinition", `${at}.label`),
    granularities: list(entry.granularities, "InteractionDefinition", `${at}.granularities`, 1, (item, itemAt) => parseGranularityDefinition(item, itemAt)),
    hierarchy: parseHierarchyProvider(entry.hierarchy, `${at}.hierarchy`),
    hover: parseHoverSpec(entry.hover, `${at}.hover`),
    selection: parseSelectionSpec(entry.selection, `${at}.selection`),
  } as InteractionDefinition;
}

/** 🎯️ `$defs.InteractionTarget` — one addressed target: a granularity id plus the target's own id. */
export function parseInteractionTarget(value: unknown, at = "InteractionTarget"): InteractionTarget {
  const entry = members(value, "InteractionTarget", at, ["granularity", "id"]);
  return { granularity: text(entry.granularity, "InteractionTarget", `${at}.granularity`), id: text(entry.id, "InteractionTarget", `${at}.id`) };
}

/** 🖱️ `$defs.DomainSelection` — one domain's current selection. */
export function parseDomainSelection(value: unknown, at = "DomainSelection"): DomainSelection {
  const entry = members(value, "DomainSelection", at, ["granularity", "ids"], ["anchorId"]);
  const selection: DomainSelection = { granularity: text(entry.granularity, "DomainSelection", `${at}.granularity`), ids: list(entry.ids, "DomainSelection", `${at}.ids`, 0, (item, itemAt) => text(item, "DomainSelection", itemAt)) };
  return entry.anchorId === undefined ? selection : { ...selection, anchorId: text(entry.anchorId, "DomainSelection", `${at}.anchorId`) };
}

/** 🐁️ `$defs.DomainHover` — one domain's current hover on one channel. */
export function parseDomainHover(value: unknown, at = "DomainHover"): DomainHover {
  const entry = members(value, "DomainHover", at, ["channel", "ids"]);
  return { channel: text(entry.channel, "DomainHover", `${at}.channel`), ids: list(entry.ids, "DomainHover", `${at}.ids`, 0, (item, itemAt) => text(item, "DomainHover", itemAt)) };
}

/** 🗺️ `$defs.InteractionState` — persisted-local selection/mode/granularity plus ephemeral-local hover, keyed by domain id. */
export function parseInteractionState(value: unknown, at = "InteractionState"): InteractionState {
  const entry = members(value, "InteractionState", at, ["selection", "hover", "activeMode", "activeGranularity"]);
  return {
    selection: keyed(entry.selection, "InteractionState", `${at}.selection`, (item, itemAt) => parseDomainSelection(item, itemAt)),
    hover: keyed(entry.hover, "InteractionState", `${at}.hover`, (item, itemAt) => parseDomainHover(item, itemAt)),
    activeMode: keyed(entry.activeMode, "InteractionState", `${at}.activeMode`, (item, itemAt) => parseSelectionMode(item, itemAt)),
    activeGranularity: keyed(entry.activeGranularity, "InteractionState", `${at}.activeGranularity`, (item, itemAt) => text(item, "InteractionState", itemAt)),
  };
}

/** 🌳️ `$defs.TopologyNode` — one node of a domain's topology; an absent `parent` is a root. */
export function parseTopologyNode(value: unknown, at = "TopologyNode"): TopologyNode {
  const entry = members(value, "TopologyNode", at, ["id", "granularity"], ["parent"]);
  const node: TopologyNode = { id: text(entry.id, "TopologyNode", `${at}.id`), granularity: text(entry.granularity, "TopologyNode", `${at}.granularity`) };
  return entry.parent === undefined ? node : { ...node, parent: text(entry.parent, "TopologyNode", `${at}.parent`) };
}

/** 🌲️ `$defs.DomainTopology` — one domain's pre-order topology; `ordered` IS the range-selection order. */
export function parseDomainTopology(value: unknown, at = "DomainTopology"): DomainTopology {
  const entry = members(value, "DomainTopology", at, ["ordered"]);
  return { ordered: list(entry.ordered, "DomainTopology", `${at}.ordered`, 0, (item, itemAt) => parseTopologyNode(item, itemAt)) };
}

/** 📡️ `$defs.PresenceDomain` — one domain's broadcast slice, explicit ids only. */
export function parsePresenceDomain(value: unknown, at = "PresenceDomain"): PresenceDomain {
  const entry = members(value, "PresenceDomain", at, ["domain", "granularity", "selected", "hovered"]);
  return {
    domain: text(entry.domain, "PresenceDomain", `${at}.domain`),
    granularity: text(entry.granularity, "PresenceDomain", `${at}.granularity`),
    selected: list(entry.selected, "PresenceDomain", `${at}.selected`, 0, (item, itemAt) => text(item, "PresenceDomain", itemAt)),
    hovered: list(entry.hovered, "PresenceDomain", `${at}.hovered`, 0, (item, itemAt) => text(item, "PresenceDomain", itemAt)),
  };
}

/** 📡️ `$defs.PresenceInteraction` — one peer's interaction roster for one app instance. */
export function parsePresenceInteraction(value: unknown, at = "PresenceInteraction"): PresenceInteraction {
  const entry = members(value, "PresenceInteraction", at, ["appId", "domains"]);
  return { appId: text(entry.appId, "PresenceInteraction", `${at}.appId`), domains: list(entry.domains, "PresenceInteraction", `${at}.domains`, 0, (item, itemAt) => parsePresenceDomain(item, itemAt)) };
}
// #endregion 🚪️Parsers
