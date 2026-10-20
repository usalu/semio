/** 🔶️ The catalog phase of useDirectoryAndRoute. */
import { directorySessionAuthorityIsCurrentV1 } from "@semio-tech/framework/directory/session-refresh";
import { useCallback, useEffect, useRef } from "react";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { localCatalogActorConfig } from "@semio-tech/framework/backbone/config-lane";
import { LOCAL_CATALOG_READ_DEADLINE_MS_V1, LOCAL_CATALOG_WRITE_DEADLINE_MS_V1, foldLocalCatalogEnvelopesV1, localCatalogBindingV1, localCatalogMutationEnvelopeV1, sameLocalDocumentArchiveV1, type LocalCatalogAdmissionV1 } from "../../🗂️local-catalog/🟦️.ts";
import { LOCAL_CATALOG_CONFIG_SCHEMA, type LocalCatalog, type LocalCatalogConfigMutation, type LocalDocument } from "@semio-tech/framework/config/mutations";
import { type SpaceAdministrationIntentV1 } from "../../../🛂️SpaceAdministration/🟦️.tsx";
import type { InferencePortUiAction } from "@semio-tech/framework/inference/service";
import { applyDirectoryEventPageBootstrapV1 } from "../../📇️directory-bootstrap/🟦️.tsx";
import { mintDirectoryCommandRequestId } from "../../🧾️space-artifact-creation/🟦️.ts";
import { shellSpaceAdministrationRequest } from "../../🏠️space-administration/🟦️.ts";
import type { ShellCoreAfterBackboneWorker } from "../../🧵️backbone-worker/🟦️.ts";

/** 🔷 Builds catalog, pluginFilter, plugins, scope of useDirectoryAndRoute. */
export function useCatalogState({ core }: { readonly core: ShellCoreAfterBackboneWorker }) {
  const {
    catalog, pluginFilter, plugins, scope, hostMode, dispatch, session, hostPlugin, landingApp, sessionRef,
    shellRouteLedgerRef, shellSessionLaneRef, backboneWorkerRef, shellSessionIdRef, shellActorIdRef, identity,
    setIdentity, setIdentityOffline, setHubSessionRefused, verifiedSessionAuthority, setVerifiedSessionAuthority,
    verifiedSessionAuthorityRef, sessionRefreshRef, hubSessionPortRef, hubEnv, hubSessionCapability,
    identityResolutionRef, directoryHomeOwnerRef, directoryHomeOpeningRef, directoryBootstrapEpochRef,
    setDirectoryBootstrapUi, refreshDirectoryHomeRef, handleDirectoryEventPageRef, handleDirectoryBootstrapFailureRef,
    uiLocaleRef, uiTerminologyRef, retireDirectoryHomeOwner, spaceAdministration, setSpaceAdministration,
    spaceAdministrationEpochRef, inferencePortEpochRef, inferencePortOwnerRef, inferencePortAuthorityRef,
    retiredSessionDocumentOwnersRef, inferencePortOpeningRef, setInferenceHistoryByRuntimeKey,
    inferenceHistoryByRuntimeKeyRef, identitySnapshotResolverRef, identityClientInstanceIdRef, identityBootstrapAbortRef,
    localDataDir, setLocalCatalog, localCatalogRef, localCatalogClientInstanceIdRef, localDocumentOwnersRef,
    presenceCursorRef, openDocumentSessionsRef, ensureBackboneWorker
  } = core;
  //#region FrameworkOsShell
  //#region 🗂️LocalCatalog
  useEffect(() => {
    const worker = ensureBackboneWorker();
    const clientInstanceId = crypto.randomUUID();
    localCatalogClientInstanceIdRef.current = clientInstanceId;
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "open", clientInstanceId, ...localCatalogActorConfig(shellActorIdRef.current, {maximumItems:1024,maximumCopyBytes:65536,maximumCapacityBytes:65536,maximumReleaseBytes:65536,maximumDepth:64}, localDataDir) }) });
    return () => {
      if (localCatalogClientInstanceIdRef.current === clientInstanceId) localCatalogClientInstanceIdRef.current = null;
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "close", documentId: LOCAL_CATALOG_CONFIG_SCHEMA, clientInstanceId }) });
    };
  }, [ensureBackboneWorker, localDataDir]);

  /** 💾️ Writes one kept document into its folder lane and VERIFIES it: the worker actor stays open until a folder read-back
   * returns exactly the archive that was written (each `persisted` status asks for a fresh read), so a stale folder or a
   * failed PUT never counts as kept. `false` once the deadline passes. */
  const writeLocalDocumentRef = useRef(async (_admission: LocalCatalogAdmissionV1): Promise<boolean> => false);
  writeLocalDocumentRef.current = async (admission: LocalCatalogAdmissionV1): Promise<boolean> => {
    const worker = ensureBackboneWorker();
    const clientInstanceId = crypto.randomUUID();
    const { documentId, schema } = admission.document;
    const verified = new Promise<boolean>((resolve) => {
      const settle = (written: boolean): void => {
        clearTimeout(deadline);
        localDocumentOwnersRef.current.delete(clientInstanceId);
        resolve(written);
      };
      const deadline = setTimeout(() => settle(false), LOCAL_CATALOG_WRITE_DEADLINE_MS_V1);
      localDocumentOwnersRef.current.set(clientInstanceId, {
        status: (persisted) => {
          if (persisted) worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId, clientInstanceId, message: { kind: "externalChanged" } }) });
        },
        archive: (archive) => {
          if (sameLocalDocumentArchiveV1(archive, admission.archive)) settle(true);
        },
      });
    });
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "open", actorIdentityGrant: {maximumItems:1024,maximumCopyBytes:65536,maximumCapacityBytes:65536,maximumReleaseBytes:65536,maximumDepth:64}, clientInstanceId, documentId, schema, bindings: [admission.binding], actor: shellActorIdRef.current }) });
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId, clientInstanceId, message: { kind: "localDocumentArchive", archive: Array.from(admission.archive) } }) });
    const written = await verified;
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "close", documentId, clientInstanceId }) });
    return written;
  };

  /** 📖️ Reads one kept document's archive back from its lane, or `null` when the lane holds none before the deadline. */
  const readLocalDocumentRef = useRef(async (_document: LocalDocument): Promise<Uint8Array | null> => null);
  readLocalDocumentRef.current = async (document: LocalDocument): Promise<Uint8Array | null> => {
    const worker = ensureBackboneWorker();
    const clientInstanceId = crypto.randomUUID();
    const read = new Promise<Uint8Array | null>((resolve) => {
      const settle = (archive: Uint8Array | null): void => {
        clearTimeout(deadline);
        localDocumentOwnersRef.current.delete(clientInstanceId);
        resolve(archive);
      };
      const deadline = setTimeout(() => settle(null), LOCAL_CATALOG_READ_DEADLINE_MS_V1);
      localDocumentOwnersRef.current.set(clientInstanceId, { archive: settle });
    });
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "open", actorIdentityGrant: {maximumItems:1024,maximumCopyBytes:65536,maximumCapacityBytes:65536,maximumReleaseBytes:65536,maximumDepth:64}, clientInstanceId, documentId: document.documentId, schema: document.schema, bindings: [localCatalogBindingV1(document)], actor: shellActorIdRef.current }) });
    const archive = await read;
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "close", documentId: document.documentId, clientInstanceId }) });
    return archive;
  };

  /** 🧾️ Records one catalog mutation as ONE envelope to the store worker, which folds its sparse diff through the twin of the
   * central applier, persists the catalog to the folder lane and answers with the resulting archive — that archive is the
   * projection the shell renders. Until it arrives the shell shows the optimistic projection: the same diff through the same
   * twin applier ({@link foldLocalCatalogEnvelopesV1}), replaced by the store's result. Without an open facet nothing is recorded. */
  const recordLocalCatalogRef = useRef((_mutation: LocalCatalogConfigMutation): LocalCatalog | null => null);
  recordLocalCatalogRef.current = (mutation: LocalCatalogConfigMutation): LocalCatalog | null => {
    const clientInstanceId = localCatalogClientInstanceIdRef.current;
    if (clientInstanceId === null) return null;
    const base = localCatalogRef.current;
    const envelope = localCatalogMutationEnvelopeV1(shellActorIdRef.current, mutation, base);
    const next = foldLocalCatalogEnvelopesV1(base, [envelope]);
    const worker = ensureBackboneWorker();
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: LOCAL_CATALOG_CONFIG_SCHEMA, clientInstanceId, message: { kind: "localMutations", envelopes: [envelope] } }) });
    localCatalogRef.current = next;
    setLocalCatalog(next);
    return next;
  };
  //#endregion 🗂️LocalCatalog

  handleDirectoryEventPageRef.current = (message) => {
    const owner = directoryHomeOwnerRef.current;
    const worker = backboneWorkerRef.current;
    if (!owner || !worker || message.bootstrapEpoch !== owner.bootstrapEpoch) return;
    setDirectoryBootstrapUi({ kind: "pending", throughSeqInclusive: message.throughSeqInclusive, cancellable: true });
    void applyDirectoryEventPageBootstrapV1(
      owner,
      message,
      (request) => worker.postMessage({ wire: encodeBackboneWorkerRequest(request) }),
      async (appliedOwner) => {
        const current = sessionRef.current;
        if (!current || current.pluginId !== appliedOwner.plugin.pluginId || current.app.id !== appliedOwner.app.id || current.instanceId !== appliedOwner.instanceId) throw new Error("directory-bootstrap.visible-owner-stale");
        await refreshDirectoryHomeRef.current(current);
      },
    ).then((result) => {
      if (directoryHomeOwnerRef.current !== owner) return;
      if (owner.abort.signal.aborted) directoryHomeOwnerRef.current = null;
      setDirectoryBootstrapUi(result.state);
    });
  };

  handleDirectoryBootstrapFailureRef.current = (message) => {
    const owner = directoryHomeOwnerRef.current;
    const worker = backboneWorkerRef.current;
    if (!owner || !worker || message.bootstrapEpoch !== owner.bootstrapEpoch) return;
    if (message.retryable) {
      setDirectoryBootstrapUi({ kind: "retrying", throughSeqInclusive: owner.pending?.throughSeqInclusive ?? 0 });
      return;
    }
    directoryHomeOwnerRef.current = null;
    void retireDirectoryHomeOwner(owner, worker);
    setDirectoryBootstrapUi({ kind: "fault", code: `directory-bootstrap.${message.code}` });
  };

  const cancelDirectoryBootstrap = useCallback(() => {
    const owner = directoryHomeOwnerRef.current;
    const worker = backboneWorkerRef.current;
    if (!owner || !worker) return;
    directoryHomeOwnerRef.current = null;
    void retireDirectoryHomeOwner(owner, worker).finally(() => {
      if (!directoryHomeOwnerRef.current) setDirectoryBootstrapUi({ kind: "idle" });
    });
  }, [retireDirectoryHomeOwner]);

  /** 🎬️ The pane's ONLY path to the wire. An intent the current canonical page does not authorize
   * produces no request at all, so an unauthorized control can never reach the hub even if the DOM
   * is tampered with; `close` additionally retires the epoch so a late worker message is dropped. */
  const dispatchSpaceAdministrationIntent = useCallback((intent: SpaceAdministrationIntentV1) => {
    const state = spaceAdministration;
    const worker = backboneWorkerRef.current;
    if (state === null || worker === null || state.operationEpoch !== spaceAdministrationEpochRef.current) return;
    const request = shellSpaceAdministrationRequest(state, intent, mintDirectoryCommandRequestId());
    if (request === null) return;
    if (intent.kind === "close") {
      spaceAdministrationEpochRef.current += 1;
      setSpaceAdministration(null);
    }
    worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
  }, [spaceAdministration]);

  useEffect(() => () => {
    const worker = backboneWorkerRef.current;
    const operationEpoch = spaceAdministrationEpochRef.current;
    if (worker) worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-administration-close", operationEpoch }) });
    spaceAdministrationEpochRef.current = operationEpoch + 1;
    setSpaceAdministration(null);
  }, [identity, verifiedSessionAuthority?.authorizationGeneration, verifiedSessionAuthority?.sessionBindingSha256]);

  /** 💡️ Relays one operator intent to the worker-owned port. Nothing is applied locally: `cancel`
   * and `approve` only ask, and the phase moves solely on the worker's own next status. `close`
   * waits for the worker's exact server-confirmed retirement. */
  const dispatchInferencePortIntent = useCallback((runtimeKey: string, action: InferencePortUiAction) => {
    const worker = backboneWorkerRef.current;
    const operationEpoch = inferencePortEpochRef.current;
    const owner = inferencePortOwnerRef.current;
    if (!worker || owner === null || owner.runtimeKey !== runtimeKey || owner.operationEpoch !== operationEpoch) return;
    if (action.kind !== "close" && !directorySessionAuthorityIsCurrentV1(inferencePortAuthorityRef.current, verifiedSessionAuthorityRef.current)) return;
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "service-operation", owner: owner.owner, serviceId: owner.serviceId, action: action.kind, operationEpoch, payload: action.payload ?? {} }) });
  }, []);
  return { inferencePortOpeningRef, backboneWorkerRef, inferencePortEpochRef, inferencePortOwnerRef, inferencePortAuthorityRef, dispatch, openDocumentSessionsRef, retiredSessionDocumentOwnersRef, inferenceHistoryByRuntimeKeyRef, setInferenceHistoryByRuntimeKey, shellActorIdRef, identityBootstrapAbortRef, hubEnv, sessionRefreshRef, hubSessionPortRef, verifiedSessionAuthorityRef, setVerifiedSessionAuthority, setIdentityOffline, setHubSessionRefused, shellSessionIdRef, identityClientInstanceIdRef, hubSessionCapability, ensureBackboneWorker, identitySnapshotResolverRef, setIdentity, identity, verifiedSessionAuthority, hostPlugin, landingApp, session, directoryBootstrapEpochRef, directoryHomeOpeningRef, sessionRef, uiLocaleRef, uiTerminologyRef, refreshDirectoryHomeRef, retireDirectoryHomeOwner, directoryHomeOwnerRef, setDirectoryBootstrapUi, presenceCursorRef, hostMode, scope, identityResolutionRef, shellSessionLaneRef, shellRouteLedgerRef, plugins, pluginFilter, catalog, writeLocalDocumentRef, readLocalDocumentRef, recordLocalCatalogRef, cancelDirectoryBootstrap, dispatchSpaceAdministrationIntent, dispatchInferencePortIntent };
}
