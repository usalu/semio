import type { ArtifactActorConfig, ArtifactEvent, BackboneWorkerResponse } from "../../../../🟦️";
import type { OpeningPreferences } from "@semio-tech/framework";
import type { DocumentServiceWorkerHostV1, DocumentServiceDriverContextV1 } from "@semio-tech/framework/inference/service/worker-host";
import type { InstalledServiceDriverV1 } from "@semio-tech/framework/inference/service";
import { documentRuntimeKeyV1 } from "../../../../🟦️";
import { documentServiceRequestV1, admitDocumentServiceResponseV1 } from "@semio-tech/framework/inference/service/io";
import { type ArtifactState, artifactScope, artifactState, documentServices, post } from "../🧾️document-state/🟦️.ts";
import type { DocumentBrowserActorReservation } from "../🧷️actor-reservation/🟦️.ts";
import type { VerifiedColdDocumentPair } from "../🧊️cold-pair/🟦️.ts";
import { bytesHex, captureBrowserSessionOperationFence, directorySessionEpoch, hubSessionFetch, sameBrowserSessionOperationFence } from "../🔐️hub-session/🟦️.ts";
import { SOCKET_GRANT_REQUEST_TIMEOUT_MS } from "../⚙️constants/🟦️.ts";

//#region 🔖️ConfigLane
/** 🎚️ Canonical `documentId`/`schema` for the OS-wide `os.config.opening` facet (contract freeze
 * §4 of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`) — one
 * singleton instance per install, so the schema id doubles as its document id. */
export const OPENING_PREFERENCES_SCHEMA = "os.config.opening";

/** 🎚️ Builds the {@link ArtifactActorConfig} that opens the opening-preferences facet through
 * this SAME generic actor mechanism every other document uses — `bindings: []` is not a stub, it
 * IS the "persisted local-only" lane (contract freeze §4): {@link folderBinding}/{@link hubBinding}
 * both return `null` on an empty `bindings` array, so `openArtifact` below already skips folder
 * watch and hub websocket setup for this config entirely — no schema-specific branch exists or is
 * needed anywhere in this file for that to hold. */
export function openingPreferencesActorConfig(actor: string, actorIdentityGrant: ArtifactActorConfig["actorIdentityGrant"]): ArtifactActorConfig {
  return { documentId: OPENING_PREFERENCES_SCHEMA, schema: OPENING_PREFERENCES_SCHEMA, bindings: [], actor, actorIdentityGrant };
}

/** 🧮️ Reduces one {@link ArtifactEvent} onto a materialized `OpeningPreferences` — event-sourced,
 * never a mutable map (contract freeze §4). This facet's `Mutation::diff` is whole-record (kernel
 * `🔖️OpeningResolver`'s `decodeOpeningPreferences` docstring), so a `remoteMutations` envelope's
 * already-diffed `diff.payload` IS the next full snapshot — folding is "last envelope wins", not a
 * replay of individual `set`/`clear` operations. Every other event kind (`status`, `presence`, …)
 * passes `base` through unchanged. `decodePayload` stays injected rather than imported from
 * `@semio-tech/framework` at the call site — the caller already has `decodeOpeningPreferences` in
 * scope wherever it decoded `MutationEnvelope.diff.payload` off the wire in the first place. */
export function foldOpeningPreferencesEvent(base: OpeningPreferences, event: ArtifactEvent, decodePayload: (payload: unknown) => OpeningPreferences | undefined): OpeningPreferences {
  if (event.kind !== "remoteMutations") return base;
  let next = base;
  for (const envelope of event.envelopes) {
    const decoded = decodePayload(envelope.diff.payload);
    if (decoded) next = decoded;
  }
  return next;
}

//#endregion 🔖️ConfigLane

/** 🪪 Narrow authority port available to installed owner service contributions. */
export type DocumentServiceWorkerContextV1 = Readonly<{ state: ArtifactState; reservation?: DocumentBrowserActorReservation; pair?: VerifiedColdDocumentPair }>;

/** 🔌 Deployment composition installs executable owners; verified document declarations grant routes. */
function activateDocumentServiceWorkerV1(owner: string, serviceId: string, factory: (host: DocumentServiceWorkerHostV1) => InstalledServiceDriverV1<DocumentServiceDriverContextV1>): () => void {
  const retirement = new AbortController();
  const host: DocumentServiceWorkerHostV1 = {
    artifactScope, artifactState, bytesHex, captureBrowserSessionOperationFence, sameBrowserSessionOperationFence,
    post(message) { if (retirement.signal.aborted) return; const outbound = message as BackboneWorkerResponse; post(outbound.kind === "inference-port-opened" || outbound.kind === "inference-port-closed" ? { ...outbound, owner, serviceId } : outbound); },
    documentRuntimeKeyV1, SOCKET_GRANT_REQUEST_TIMEOUT_MS,
    get directorySessionEpoch() { return directorySessionEpoch; },
    bindDocumentService(requestOwner, requestServiceId, scope) {
      if (requestOwner !== owner || requestServiceId !== serviceId || retirement.signal.aborted) throw new Error("installed-service.owner-retired");
      const state = artifactState(scope.documentId, scope.spaceId), lease = state?.executionTargetLease;
      if (!lease?.live || lease.fields().package.pluginId !== owner) throw new Error("installed-service.unavailable");
      const declarations = lease.documentServices().filter((entry) => entry.owner === owner && entry.serviceId === serviceId);
      if (declarations.length !== 1) throw new Error("installed-service.unavailable");
      const declaration = declarations[0]!;
      const fence = captureBrowserSessionOperationFence();
      if (fence === null) throw new Error("installed-service.session-unavailable");
      return Object.freeze({ async call(action, payload, options) {
        if (retirement.signal.aborted || !sameBrowserSessionOperationFence(fence)) throw new Error("installed-service.owner-retired");
        const request = documentServiceRequestV1(declaration, scope, action, payload);
        const signal = options.signal === undefined ? retirement.signal : AbortSignal.any([retirement.signal, options.signal]);
        const response = await hubSessionFetch(request.path, { method: request.method, ...(request.body === undefined ? {} : { headers: { "content-type": "application/json" }, body: request.body }) }, { ...options, signal, admit: () => !retirement.signal.aborted && sameBrowserSessionOperationFence(fence) && (options.admit?.() ?? true), retain: () => !retirement.signal.aborted && sameBrowserSessionOperationFence(fence) && (options.retain?.() ?? true) });
        if (!response.ok) return response;
        const declared = response.headers.get("content-length");
        if (declared !== null && (!Number.isSafeInteger(Number(declared)) || Number(declared) < 1 || Number(declared) > request.responseMaxBytes)) throw new Error("installed-service.bounds");
        const body = await response.text();
        if (retirement.signal.aborted || signal.aborted || !sameBrowserSessionOperationFence(fence)) throw new Error("installed-service.owner-retired");
        const value = admitDocumentServiceResponseV1(declaration,action,body);
        return { ok: response.ok, status: response.status, statusText: response.statusText, headers: response.headers, text: async () => body, json: async () => value };
      } });
    },
  };
  const driver = factory(host);
  if (driver.owner !== owner || driver.serviceId !== serviceId) throw new Error("installed-service.foreign-owner");
  const remove = documentServices.install(driver);
  return () => { if (retirement.signal.aborted) return; remove(); retirement.abort(new Error("installed service contribution retired")); };
}

/** 🧩 Composition holds factories; registry presence creates and retires the actual operation owners. */
const documentServiceFactoriesV1 = new Map<string, { owner: string; serviceId: string; factory: (host: DocumentServiceWorkerHostV1) => InstalledServiceDriverV1<DocumentServiceDriverContextV1>; retire: (() => void) | null }>();
export function installDocumentServiceWorkerV1(owner: string, serviceId: string, factory: (host: DocumentServiceWorkerHostV1) => InstalledServiceDriverV1<DocumentServiceDriverContextV1>): () => void {
  const key = JSON.stringify([owner, serviceId]);
  if (documentServiceFactoriesV1.size >= 64 || documentServiceFactoriesV1.has(key)) throw new Error("installed-service.capacity");
  const contribution = { owner, serviceId, factory, retire: activateDocumentServiceWorkerV1(owner, serviceId, factory) as (() => void) | null };
  documentServiceFactoriesV1.set(key, contribution);
  return () => { if (documentServiceFactoriesV1.get(key) !== contribution) return; documentServiceFactoriesV1.delete(key); contribution.retire?.(); contribution.retire = null; };
}
export function reconcileDocumentServiceContributionsV1(services: readonly Readonly<{owner: string; serviceId: string}>[]): void {
  const wanted = new Set(services.map(({owner, serviceId}) => JSON.stringify([owner, serviceId])));
  for (const [key, contribution] of documentServiceFactoriesV1) {
    if (!wanted.has(key)) { contribution.retire?.(); contribution.retire = null; }
    else if (contribution.retire === null) contribution.retire = activateDocumentServiceWorkerV1(contribution.owner, contribution.serviceId, contribution.factory);
  }
}
