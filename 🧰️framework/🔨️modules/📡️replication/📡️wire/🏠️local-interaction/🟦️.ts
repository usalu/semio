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
export const LOCAL_INTERACTION_CAPTURE_MAX_BYTES = 1_048_576;
//#endregion 🧬️Contract

//#region 📥️CaptureDecode
function exactRecord(value: unknown, keys: readonly string[], label: string): Readonly<Record<string, unknown>> {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error(`local-interaction.${label}`);
  const record = value as Readonly<Record<string, unknown>>;
  const actual = Object.keys(record).sort();
  if (actual.length !== keys.length || ![...keys].sort().every((key, index) => key === actual[index])) throw new Error(`local-interaction.${label}`);
  return record;
}

function text(value: unknown, label: string): string {
  if (typeof value !== "string" || value.length === 0) throw new Error(`local-interaction.${label}`);
  return value;
}

function revision(value: unknown, label: string): string {
  const result = text(value, label);
  if (!/^[0-9a-f]{64}$/u.test(result)) throw new Error(`local-interaction.${label}`);
  return result;
}

function decimal(value: unknown, label: string): string {
  const result = text(value, label);
  if (!/^(0|[1-9][0-9]{0,19})$/u.test(result) || BigInt(result) > 18_446_744_073_709_551_615n) throw new Error(`local-interaction.${label}`);
  return result;
}

function decodeSelection(value: unknown, label: string): DomainSelection {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error(`local-interaction.${label}`);
  const record = exactRecord(value, Object.hasOwn(value, "anchorId") ? ["granularity", "ids", "anchorId"] : ["granularity", "ids"], label);
  if (!Array.isArray(record.ids)) throw new Error(`local-interaction.${label}.ids`);
  const ids = record.ids.map((id, index) => text(id, `${label}.ids.${index}`));
  if (new Set(ids).size !== ids.length) throw new Error(`local-interaction.${label}.ids`);
  const anchorId = record.anchorId === undefined ? undefined : text(record.anchorId, `${label}.anchorId`);
  return { granularity: text(record.granularity, `${label}.granularity`), ids, ...(anchorId === undefined ? {} : { anchorId }) };
}

function decodeMap<T>(value: unknown, label: string, decode: (entry: unknown, key: string) => T): Readonly<Record<string, T>> {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error(`local-interaction.${label}`);
  const result: Record<string, T> = {};
  for (const [key, entry] of Object.entries(value)) Object.defineProperty(result, text(key, `${label}.domain`), { value: decode(entry, key), enumerable: true, writable: true, configurable: true });
  return result;
}

/** 📥️ Decodes one bounded canonical JSON capture into owned framework types. */
export function decodeLocalInteractionCaptureJson(bytes: Uint8Array): LocalInteractionCapture {
  if (bytes.length === 0 || bytes.length > LOCAL_INTERACTION_CAPTURE_MAX_BYTES) throw new Error("local-interaction.capture-length");
  const root = exactRecord(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes)), ["identity", "state"], "capture");
  const identity = exactRecord(root.identity, ["appInstanceId", "generation", "revision", "documentRevision", "topologyRevision"], "identity");
  if (!Number.isInteger(identity.appInstanceId) || (identity.appInstanceId as number) < 0 || (identity.appInstanceId as number) > 4_294_967_295) throw new Error("local-interaction.identity.appInstanceId");
  const state = exactRecord(root.state, ["selection", "activeMode", "activeGranularity"], "state");
  return {
    identity: {
      appInstanceId: identity.appInstanceId as number,
      generation: decimal(identity.generation, "identity.generation"),
      revision: revision(identity.revision, "identity.revision"),
      documentRevision: revision(identity.documentRevision, "identity.documentRevision"),
      topologyRevision: revision(identity.topologyRevision, "identity.topologyRevision"),
    },
    state: {
      selection: decodeMap(state.selection, "state.selection", (entry, key) => decodeSelection(entry, `state.selection.${key}`)),
      activeMode: decodeMap(state.activeMode, "state.activeMode", (entry, key) => {
        if (entry !== "single" && entry !== "multiple") throw new Error(`local-interaction.state.activeMode.${key}`);
        return entry;
      }),
      activeGranularity: decodeMap(state.activeGranularity, "state.activeGranularity", (entry, key) => text(entry, `state.activeGranularity.${key}`)),
    },
  };
}
//#endregion 📥️CaptureDecode

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
