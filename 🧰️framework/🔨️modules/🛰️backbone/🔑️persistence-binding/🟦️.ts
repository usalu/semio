/** 🔑️ Document runtime key and persistence binding: the collision-free ownership key, data classes of wire lanes and the persistence binding constructors. */

import type { DocumentExecutionTargetLeaseFieldsV1 } from "../../📇️directory/🧬️schema/🎯️execution-target/🟦️.ts";

//#region 🔖️BackboneWorkerProtocol
export type DocumentRuntimeScopeV1 = Readonly<{ kind: "hub", dataClass: "persistedShared"; spaceId: string; documentId: string }> | Readonly<{ kind: "local"; documentId: string }>;

/** 🔑️ Canonical collision-free document runtime ownership key. Hub documents are keyed by their
 * complete authority scope; local documents occupy an explicitly separate namespace. */
export function documentRuntimeKeyV1(scope: DocumentRuntimeScopeV1): string {
  const encodeId = (value: string): Uint8Array => {
    const bytes = new TextEncoder().encode(value);
    if (bytes.length === 0 || bytes.length > 256 || /[\u0000-\u001f\u007f]/u.test(value)) throw new Error("document runtime scope: invalid id");
    return bytes;
  };
  const documentBytes = encodeId(scope.documentId);
  if (scope.kind === "local") return `local:v1:${documentBytes.length}:${scope.documentId}`;
  const spaceBytes = encodeId(scope.spaceId);
  return `v1:${spaceBytes.length}:${documentBytes.length}:${scope.spaceId}${scope.documentId}`;
}

/** 🗃️ Durability × sharing class for persistence bindings and replication lanes.
 * Preview/presence are always `ephemeralShared` and must never be treated as durable WAL state. */
export type PersistenceDataClass = "persistedLocalOnly" | "persistedShared" | "ephemeralLocalOnly" | "ephemeralShared";

export type ClassifiedWireLane =
  | { readonly lane: "command"; readonly dataClass: "persistedLocalOnly" | "persistedShared" }
  | { readonly lane: "preview"; readonly dataClass: "ephemeralShared" }
  | { readonly lane: "presence"; readonly dataClass: "ephemeralShared" };

export function wireLaneDataClass(lane: "command" | "preview" | "presence"): PersistenceDataClass {
  if (lane === "preview" || lane === "presence") return "ephemeralShared";
  return "persistedShared";
}

export function bindingsDataClass(bindings: readonly PersistenceBinding[]): PersistenceDataClass {
  for (const dataClass of ["persistedShared", "persistedLocalOnly", "ephemeralShared"] as const) {
    if (bindings.some(binding => binding.dataClass === dataClass)) return dataClass;
  }
  return "ephemeralLocalOnly";
}

export function folderPersistenceBinding(path: string): Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }> {
  return { kind: "folder", path, dataClass: "persistedLocalOnly" };
}

export function ephemeralLocalPersistenceBinding(): Extract<PersistenceBinding, { kind: "ephemeral"; dataClass: "ephemeralLocalOnly" }> {
  return { kind: "ephemeral", dataClass: "ephemeralLocalOnly" };
}

export function ephemeralSharedPersistenceBinding(lane: "preview" | "presence" = "preview"): Extract<PersistenceBinding, { kind: "ephemeral"; dataClass: "ephemeralShared" }> {
  return { kind: "ephemeral", dataClass: "ephemeralShared", lane };
}

export function hubPersistenceBinding(
  baseUrl: string,
  spaceId: string,
  extras?: Readonly<{ requestedSurfaceId?: string; installedTarget?: DocumentExecutionTargetLeaseFieldsV1 }>,
): Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }> {
  return { kind: "hub", baseUrl, spaceId, dataClass: "persistedShared", ...extras };
}

export type PersistenceBinding =
  | { readonly kind: "folder"; readonly path: string; readonly dataClass: "persistedLocalOnly" }
  /** ☁️ A hub binding states which surface it would like (`requestedSurfaceId`) and, when a previous
   * verified installation is already known, the complete {@link DocumentExecutionTargetLeaseFieldsV1}
   * to compare the next plan against. Neither is byte ownership: a non-`react` renderer target is
   * admitted only through a live private lease minted from server-verified bytes. */
  | { readonly kind: "hub"; readonly baseUrl: string; readonly spaceId: string; readonly dataClass: "persistedShared"; readonly requestedSurfaceId?: string; readonly installedTarget?: DocumentExecutionTargetLeaseFieldsV1 }
  | { readonly kind: "ephemeral"; readonly dataClass: "ephemeralLocalOnly" }
  | { readonly kind: "ephemeral"; readonly dataClass: "ephemeralShared"; readonly lane?: "preview" | "presence" };
//#endregion 🔖️BackboneWorkerProtocol
