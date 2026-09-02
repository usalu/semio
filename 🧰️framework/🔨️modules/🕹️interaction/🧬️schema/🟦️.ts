// #region 🧬️Schema
/** 🧬️ Schema leaf: canonical TS mirror of `🔣️.json` for the 🕹️interaction module.
 * `InteractionDefinition`/`InteractionState`/friends are re-exported from the module root
 * (`../🟦️.ts`) rather than redefined here — this leaf only newly defines
 * `PresenceInteraction`/`PresenceDomain`, the broadcast payload shape. */
export type {
  InteractionDefinition,
  GranularityDefinition,
  HierarchyProvider,
  HoverSpec,
  SelectionSpec,
  SelectionMode,
  SelectionMethod,
  MergeMode,
  InteractionRef,
  InteractionTarget,
  DomainSelection,
  DomainHover,
  InteractionState,
  TopologyNode,
  DomainTopology,
  InteractionTopology,
} from "../🟦️.ts";

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
