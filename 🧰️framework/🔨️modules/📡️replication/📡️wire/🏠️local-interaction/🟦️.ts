/** 🏠️ Exact local-only interaction capture and restore contracts; schema lives in `🧬️schema`. */
import type { DomainSelection, SelectionMode } from "../../../🕹️interaction/🟦️.ts";

//#region 🧬️Contract
export type LocalInteractionState = {
  readonly selection: Readonly<Record<string, DomainSelection>>;
  readonly activeMode: Readonly<Record<string, SelectionMode>>;
  readonly activeGranularity: Readonly<Record<string, string>>;
};

/** 🔐️ Full current publication authority; decimal generation is lossless beyond JavaScript integers. */
export type LocalInteractionIdentity = {
  readonly appInstanceId: number;
  readonly generation: string;
  readonly revision: string;
  readonly documentRevision: string;
  readonly topologyRevision: string;
};

export type LocalInteractionDomainPatch = {
  readonly selection: DomainSelection | null;
  readonly activeMode: SelectionMode | null;
  readonly activeGranularity: string | null;
};

export type LocalInteractionCapture = { readonly identity: LocalInteractionIdentity; readonly state: LocalInteractionState };
export type LocalInteractionRestore =
  | { readonly kind: "full"; readonly base: LocalInteractionIdentity; readonly state: LocalInteractionState }
  | { readonly kind: "domains"; readonly base: LocalInteractionIdentity; readonly domains: Readonly<Record<string, LocalInteractionDomainPatch>> };

/** 📃️ One ordered query response page, never a filtered presence heartbeat. */
export type LocalInteractionPage = {
  readonly requestId: string;
  readonly queryGeneration: string;
  readonly identity: LocalInteractionIdentity;
  readonly ordinal: string;
  readonly terminal: boolean;
  readonly bytes: readonly number[];
};

/** 🔐️ Runtime-lifetime query authority, separate from historical tutorial state. */
export type LocalInteractionQueryToken = Pick<LocalInteractionPage, "requestId" | "queryGeneration" | "identity" | "ordinal">;
//#endregion 🧬️Contract

//#region 🧮️ColdComposition
/** 🧊️ Synchronous authority comparison for cold composition and fixtures, not a retained publication gate. */
export function localInteractionIdentityEquals(left: LocalInteractionIdentity, right: LocalInteractionIdentity): boolean {
  return left.appInstanceId === right.appInstanceId && left.generation === right.generation && left.revision === right.revision && left.documentRevision === right.documentRevision && left.topologyRevision === right.topologyRevision;
}

/** 🧊️ Composes an already validated tutorial state; does not perform live topology validation or publication. */
export function applyLocalInteractionRestoreCold(before: LocalInteractionState, current: LocalInteractionIdentity, restore: LocalInteractionRestore): LocalInteractionState {
  if (!localInteractionIdentityEquals(current, restore.base)) throw new Error("stale-authority");
  if (restore.kind === "full") return restore.state;
  const selection = { ...before.selection }, activeMode = { ...before.activeMode }, activeGranularity = { ...before.activeGranularity };
  for (const [domain, patch] of Object.entries(restore.domains)) {
    applyDomainField(selection, domain, patch.selection);
    applyDomainField(activeMode, domain, patch.activeMode);
    applyDomainField(activeGranularity, domain, patch.activeGranularity);
  }
  return { selection, activeMode, activeGranularity };
}

function applyDomainField<T>(map: Record<string, T>, domain: string, value: T | null): void {
  if (value === null) delete map[domain];
  else Object.defineProperty(map, domain, { value, enumerable: true, writable: true, configurable: true });
}
//#endregion 🧮️ColdComposition
