/** 🛂️ Local document and catalog refs, directory event and bootstrap handlers, space administration and inference port intents, session authority bootstrap, the shell uri and route, dock stores and the plugin source.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useDirectoryAndRoute` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { directorySessionAuthorityIsCurrentV1, startDirectorySessionRefreshV1 } from "@semio-tech/framework/directory/session-refresh";
import { HubSessionPortClientV1 } from "@semio-tech/framework/directory/session-refresh/session-port";
import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState
} from "react";
// #region 🔌️Adapters
import {
  DockLayoutStore,
  DockUiStateStore,
  expandPluginRegistry,
  resolvePluginRegistryId,
  waitForEvent
} from "@semio-tech/framework";
import { encodePackValue } from "@semio-tech/framework/packs";
import { encodeDocumentArchiveBytes } from "@semio-tech/framework/wire-contracts";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { SHELL_HUB_ROUTE, shellRouteAdmissionV1, shellRouteIsOverlayV1 } from "../🔀️surface-switch/🟦️.ts";
import { IDENTITY_CONFIG_SCHEMA, identityActorConfig, localCatalogActorConfig } from "@semio-tech/framework/backbone/config-lane";
import {
  LOCAL_CATALOG_READ_DEADLINE_MS_V1,
  LOCAL_CATALOG_WRITE_DEADLINE_MS_V1,
  localCatalogArchiveV1,
  localCatalogBindingV1,
  localCatalogMutationEnvelopeV1,
  sameLocalDocumentArchiveV1,
  type LocalCatalogAdmissionV1
} from "../🗂️local-catalog/🟦️.ts";
import {
  type Identity,
  signIn,
  applyLocalCatalogConfigMutation,
  LOCAL_CATALOG_CONFIG_SCHEMA,
  type LocalCatalog,
  type LocalCatalogConfigMutation,
  type LocalDocument
} from "@semio-tech/framework/config/mutations";
// #endregion 🔌️Adapters
import { type SpaceAdministrationIntentV1 } from "../../🛂️SpaceAdministration/🟦️.tsx";
import { parseShellRoute } from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { useUIHistory } from "../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { isPluginInstanceRetiredV1, setPluginRuntimeActor } from "../../🔌️PluginRuntime/🟦️.tsx";
import { type InferencePortUiAction } from "../🪪️host-bootstrap/🟦️.tsx";
import { applyDirectoryEventPageBootstrapV1, openDirectoryHomeOwnerV1, directoryHomeOwnerAppV1 } from "../📇️directory-bootstrap/🟦️.tsx";
import { identityMutationEnvelope, shellActorId } from "../🔐️identity-binding/🟦️.ts";
import { mintDirectoryCommandRequestId } from "../🧾️space-artifact-creation/🟦️.ts";
import { shellSpaceAdministrationRequest } from "../🏠️space-administration/🟦️.ts";
import type { ShellCoreAfterBackboneWorker } from "../🧵️backbone-worker/🟦️.ts";

export type ShellCoreAfterDirectoryAndRoute = ShellCoreAfterBackboneWorker & ReturnType<typeof useDirectoryAndRoute>;

export function useDirectoryAndRoute(core: ShellCoreAfterBackboneWorker) {
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

  /** 🧾️ Records one catalog mutation: the whole-record envelope for cross-tab listeners, then the new catalog archive for the
   * folder lane — the facet's own event-sourced record. Without an open facet nothing is recorded. */
  const recordLocalCatalogRef = useRef((_mutation: LocalCatalogConfigMutation): LocalCatalog | null => null);
  recordLocalCatalogRef.current = (mutation: LocalCatalogConfigMutation): LocalCatalog | null => {
    const clientInstanceId = localCatalogClientInstanceIdRef.current;
    if (clientInstanceId === null) return null;
    const base = localCatalogRef.current;
    const next = applyLocalCatalogConfigMutation(base, mutation);
    const worker = ensureBackboneWorker();
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: LOCAL_CATALOG_CONFIG_SCHEMA, clientInstanceId, message: { kind: "localMutations", envelopes: [localCatalogMutationEnvelopeV1(shellActorIdRef.current, mutation, base)] } }) });
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: LOCAL_CATALOG_CONFIG_SCHEMA, clientInstanceId, message: { kind: "localDocumentArchive", archive: Array.from(localCatalogArchiveV1(next)) } }) });
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

  /** 🧯️ Unmount and identity change both retire the operation: the worker cancels its transport and
   * erases page, receipt, and capability before the pane can be remounted under a new identity. */
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

  /** 🧯️ Retires session-owned presentation immediately while the worker retains uncertain jobs. */
  const retireAuthenticatedInferencePresentation = useCallback(() => {
    inferencePortOpeningRef.current?.mailbox.close("inference-opening: identity retired");
    inferencePortOpeningRef.current = null;
    const worker = backboneWorkerRef.current;
    const operationEpoch = inferencePortEpochRef.current;
    if (worker && inferencePortOwnerRef.current) worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "service-operation", owner: inferencePortOwnerRef.current.owner, serviceId: inferencePortOwnerRef.current.serviceId, action: "close", operationEpoch, payload: {} }) });
    const owner = inferencePortOwnerRef.current;
    inferencePortOwnerRef.current = null;
    inferencePortAuthorityRef.current = null;
    inferencePortEpochRef.current = operationEpoch + 1;
    if (owner !== null) dispatch({ type: "CLEAR_INFERENCE_PORT_FOR_DOCUMENT", runtimeKey: owner.runtimeKey });
    for (const entry of openDocumentSessionsRef.current.values()) if (entry.scope !== undefined) retiredSessionDocumentOwnersRef.current.add(entry);
    inferenceHistoryByRuntimeKeyRef.current = {};
    setInferenceHistoryByRuntimeKey({});
  }, []);
  useEffect(() => retireAuthenticatedInferencePresentation, [retireAuthenticatedInferencePresentation]);

  // 🪪️ §C3 identity bootstrap — pre-identity default actor, set once at mount so `PluginRuntime`'s
  // `AppChannelClient`s created before sign-in resolves (or with no hub env at all) still carry the
  // SAME `client-{sessionId}` id `shellActorIdRef` already defaults to.
  useEffect(() => {
    setPluginRuntimeActor(shellActorIdRef.current);
  }, []);

  const cancelSessionAuthorityBootstrap = useCallback(() => {
    const abort = identityBootstrapAbortRef.current;
    if (!hubEnv || abort === null || abort.signal.aborted) return;
    identityBootstrapAbortRef.current = null;
    abort.abort("directory.session-authority.cancelled");
    sessionRefreshRef.current?.close();
    sessionRefreshRef.current = null;
    hubSessionPortRef.current?.close();
    hubSessionPortRef.current = null;
    retireAuthenticatedInferencePresentation();
    verifiedSessionAuthorityRef.current = null;
    setVerifiedSessionAuthority(null);
    setIdentityOffline(true);
    setHubSessionRefused(true);
    shellActorIdRef.current = shellActorId(shellSessionIdRef.current, null);
    setPluginRuntimeActor(shellActorIdRef.current);
    const clientInstanceId = identityClientInstanceIdRef.current;
    if (clientInstanceId !== null) {
      backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "close", documentId: IDENTITY_CONFIG_SCHEMA, clientInstanceId }) });
      if (identityClientInstanceIdRef.current === clientInstanceId) identityClientInstanceIdRef.current = null;
    }
  }, [hubEnv, retireAuthenticatedInferencePresentation]);

  useEffect(() => {
    // 📇️ §C3 "No hub env ⇒ skip all of it and keep today's local-only behaviour exactly" — the
    // existing `🛠️dev🖥️s⚛️react` launcher (no `S_HUB_URL`) never reaches any code below this guard.
    // 🎫️ A hub env with no session is the ordinary local-first state, not a fault: the shell runs,
    // and the hub badge offers sign-in. Only a signed-in human has hub authority to bootstrap.
    const capability = hubSessionCapability;
    if (!hubEnv || capability === null) return;
    setHubSessionRefused(false);
    let cancelled = false;
    const identityWaitAbort = new AbortController();
    identityBootstrapAbortRef.current = identityWaitAbort;
    (async () => {
      const worker = ensureBackboneWorker();
      const identityConfig = identityActorConfig(shellActorIdRef.current, {maximumItems:1024,maximumCopyBytes:65536,maximumCapacityBytes:65536,maximumReleaseBytes:65536,maximumDepth:64}, hubEnv.dataDir);
      const identityAttempt = { clientInstanceId: crypto.randomUUID() };
      const { clientInstanceId } = identityAttempt;
      identityClientInstanceIdRef.current = clientInstanceId;
      // 📇️ Opens the identity document FIRST (folder poll starts immediately) so the snapshot wait
      // below has something to resolve against; re-opening later with the same `documentId` (once the
      // real actor id is known) is a harmless idempotent re-subscribe (`openArtifact` always closes
      // any prior state for the same id first).
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "open", clientInstanceId, ...identityConfig }) });
      // 📇️ terra-web-shellhost (finding 3) — event-driven wait for a previously-persisted identity's
      // `documentArchiveReplaced`, raced against a 2s `AbortSignal.timeout`: a 404/no-file-yet folder read
      // never emits a `documentArchiveReplaced` at all (see `pollFolderOnce`'s doc in `🏪️store/👷️worker/🟦️.ts`).
      // It runs BESIDE the hub's authority read below, never before it: awaited first, it held every first
      // sign-in's identity — and with it the input funnel's identity gate — 2 s behind the hub's answer
      // (ticket 26/09/23 S18 §14c). The record only decides what is written back: an unchanged identity is
      // not rewritten unless the authority moved, and a returning human keeps the record's `issuedAtMs`.
      // `identityWaitAbort` folds into the raced signal so an unmount tears the subscription down at once.
      const storedIdentityRead = waitForEvent<Identity | null>(
        (handler) => {
          identitySnapshotResolverRef.current = handler;
          return () => {
            if (identitySnapshotResolverRef.current === handler) identitySnapshotResolverRef.current = null;
          };
        },
        { signal: AbortSignal.any([identityWaitAbort.signal, AbortSignal.timeout(2000)]) },
      ).catch(() => null);
      const sameIdentity = (left: Identity | null, right: Identity): boolean => left !== null && left.userId === right.userId && left.email === right.email && left.displayName === right.displayName && left.hubBaseUrl === right.hubBaseUrl;
      let shownIdentity: Identity | null = null;
      let writtenIdentity: Identity | null = null;
      // 🎫️ Hands the credential-owning worker the human's own capability over a private port, then
      // reads the hub's authority back through it. A fresh channel per bootstrap is deliberate: the
      // worker retires every owner bound to a previous capability when a new one is installed, so
      // switching humans can never leave one principal's document open under another's session.
      const sessionChannel = new MessageChannel();
      worker.postMessage({ kind: "semio-hub-session-port", port: sessionChannel.port2 }, [sessionChannel.port2]);
      const sessionPort = new HubSessionPortClientV1(sessionChannel.port1, capability.token);
      hubSessionPortRef.current = sessionPort;
      const ownerIsCurrent = (): boolean => !cancelled && identityClientInstanceIdRef.current === clientInstanceId;
      sessionRefreshRef.current = startDirectorySessionRefreshV1({
        signal: identityWaitAbort.signal,
        read: (signal) => sessionPort.me(signal),
        onAuthority: (authority) => {
          if (!ownerIsCurrent()) return;
          const previous = verifiedSessionAuthorityRef.current;
          const authorityChanged = previous === null || previous.sessionBindingSha256 !== authority.sessionBindingSha256 || previous.authorizationGeneration !== authority.authorizationGeneration;
          if (authorityChanged) retireAuthenticatedInferencePresentation();
          verifiedSessionAuthorityRef.current = authority;
          if (authorityChanged) setVerifiedSessionAuthority(authority);
          setIdentityOffline(false);
          setHubSessionRefused(false);
          const resolved: Identity = { userId: authority.userId, email: authority.email, displayName: authority.displayName, hubBaseUrl: hubEnv.hubBaseUrl, issuedAtMs: shownIdentity?.userId === authority.userId ? shownIdentity.issuedAtMs : Date.now() };
          shellActorIdRef.current = shellActorId(shellSessionIdRef.current, resolved);
          setPluginRuntimeActor(shellActorIdRef.current);
          if (sameIdentity(shownIdentity, resolved) && !authorityChanged) return;
          shownIdentity = resolved;
          setIdentity(resolved);
          void storedIdentityRead.then((stored) => {
            if (!ownerIsCurrent()) return;
            const previous = writtenIdentity ?? stored;
            if (sameIdentity(previous, resolved) && !authorityChanged) return;
            const record: Identity = previous?.userId === resolved.userId ? { ...resolved, issuedAtMs: previous.issuedAtMs } : resolved;
            const envelope = identityMutationEnvelope(shellActorIdRef.current, signIn(record), previous);
            writtenIdentity = record;
            worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: IDENTITY_CONFIG_SCHEMA, clientInstanceId, message: { kind: "localMutations", envelopes: [envelope] } }) });
            const archive = encodeDocumentArchiveBytes({ parent_pack: Array.from(encodePackValue(record)), parent_spr: [], members: [] });
            worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: IDENTITY_CONFIG_SCHEMA, clientInstanceId, message: { kind: "localDocumentArchive", archive: Array.from(archive) } }) });
          });
        },
        onUnavailable: () => {
          if (!ownerIsCurrent()) return;
          retireAuthenticatedInferencePresentation();
          verifiedSessionAuthorityRef.current = null;
          setVerifiedSessionAuthority(null);
          setIdentityOffline(true);
          setHubSessionRefused(true);
          shellActorIdRef.current = shellActorId(shellSessionIdRef.current, null);
          setPluginRuntimeActor(shellActorIdRef.current);
        },
        // 🔌️ A link that stopped answering is not a session that ended: the shell says it is offline
        // and keeps the authority it already verified, so a short connection loss leaves the document,
        // its history and its owners exactly where they were and the loop catches up on its own.
        onDegraded: (offline) => {
          if (!ownerIsCurrent()) return;
          setIdentityOffline(offline);
        },
      });
    })().catch((error) => console.error("[os-shell] identity bootstrap failed unexpectedly", error));
    return () => {
      cancelled = true;
      identityWaitAbort.abort();
      if (identityBootstrapAbortRef.current === identityWaitAbort) identityBootstrapAbortRef.current = null;
      sessionRefreshRef.current?.close();
      sessionRefreshRef.current = null;
      hubSessionPortRef.current?.close();
      hubSessionPortRef.current = null;
      verifiedSessionAuthorityRef.current = null;
      setVerifiedSessionAuthority(null);
      const clientInstanceId = identityClientInstanceIdRef.current;
      if (clientInstanceId !== null) {
        backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "close", documentId: IDENTITY_CONFIG_SCHEMA, clientInstanceId }) });
        if (identityClientInstanceIdRef.current === clientInstanceId) identityClientInstanceIdRef.current = null;
      }
    };
  }, [hubEnv, hubSessionCapability]);

  useEffect(() => {
    if (!identity || !verifiedSessionAuthority || verifiedSessionAuthority.userId !== identity.userId || !hubEnv || !hostPlugin || !landingApp || !session || session.pluginId !== hostPlugin.handle.pluginId) return;
    const ownerApp = directoryHomeOwnerAppV1(landingApp, session.app);
    if (!ownerApp) return;
    const worker = ensureBackboneWorker();
    const bootstrapEpoch = directoryBootstrapEpochRef.current + 1;
    directoryBootstrapEpochRef.current = bootstrapEpoch;
    const openingAbort = new AbortController();
    const visibleSession = session;
    const opening = directoryHomeOpeningRef.current.catch(() => {}).then(async () => {
      if (openingAbort.signal.aborted) return;
      const current = sessionRef.current;
      if (!current || current.pluginId !== visibleSession.pluginId || current.app.id !== visibleSession.app.id || current.instanceId !== visibleSession.instanceId) return;
      const owner = await openDirectoryHomeOwnerV1({
        plugin: hostPlugin.handle,
        app: ownerApp,
        identity: { userId: identity.userId, displayName: identity.displayName },
        instance: { instanceId: visibleSession.instanceId, viewState: visibleSession.viewState },
        baseUrl: identity.hubBaseUrl,
        bootstrapEpoch,
        locale: uiLocaleRef.current,
        terminology: uiTerminologyRef.current,
        signal: openingAbort.signal,
        beforeBootstrap: async (openingOwner) => {
          const active = sessionRef.current;
          if (!active || active.pluginId !== openingOwner.plugin.pluginId || active.app.id !== openingOwner.app.id || active.instanceId !== openingOwner.instanceId) throw new Error("directory-bootstrap.visible-owner-stale");
          await refreshDirectoryHomeRef.current(active);
        },
        post: (request) => worker.postMessage({ wire: encodeBackboneWorkerRequest(request) }),
      });
      if (openingAbort.signal.aborted || directoryBootstrapEpochRef.current !== bootstrapEpoch) {
        await retireDirectoryHomeOwner(owner, worker);
        return;
      }
      directoryHomeOwnerRef.current = owner;
      setDirectoryBootstrapUi({ kind: "idle" });
    });
    directoryHomeOpeningRef.current = opening.catch(() => {});
    void opening.catch((ownerError) => {
      if (!openingAbort.signal.aborted && directoryBootstrapEpochRef.current === bootstrapEpoch) setDirectoryBootstrapUi({ kind: "fault", code: ownerError instanceof Error ? ownerError.message : "directory-bootstrap.owner-open-failed" });
    });
    return () => {
      openingAbort.abort("directory-bootstrap-owner-replaced");
      const owner = directoryHomeOwnerRef.current;
      if (!owner || owner.bootstrapEpoch !== bootstrapEpoch) return;
      directoryHomeOwnerRef.current = null;
      void retireDirectoryHomeOwner(owner, worker);
    };
  }, [ensureBackboneWorker, hostPlugin?.handle, identity?.displayName, identity?.hubBaseUrl, identity?.userId, landingApp?.id, retireDirectoryHomeOwner, session?.app.id, session?.instanceId, session?.pluginId, verifiedSessionAuthority?.authorizationGeneration, verifiedSessionAuthority?.sessionBindingSha256]);

  useEffect(() => {
    if (typeof window === "undefined") return;
    const trackPointer = (event: PointerEvent) => {
      presenceCursorRef.current = { x: event.clientX, y: event.clientY };
    };
    window.addEventListener("pointermove", trackPointer, { passive: true });
    return () => window.removeEventListener("pointermove", trackPointer);
  }, []);

  // 🐚️ Only a page-owning studio shell syncs to the real browser URL bar/history — an embedded shell
  // sharing the page with others must not fight them over `window.history`.
  const { uri: shellUri, canGoBack, canGoForward, canGoUp, goBack, goForward, goUp, navigate: navigateHistory } = useUIHistory("/", hostMode && scope.ownsPage);
  const shellRoute = useMemo(() => parseShellRoute(shellUri.split("?")[0] ?? "/"), [shellUri]);
  /** 🧭️ The path the shell renders as not found — never the hub overlay route, which renders over the underlying session. */
  const notFoundPath = shellRoute.kind === "notFound" && shellRoute.path !== SHELL_HUB_ROUTE ? shellRoute.path : null;
  const shellUriRef = useRef(shellUri);
  shellUriRef.current = shellUri;
  /** 🧭️ A navigation a human or a guest asks for always takes effect: a session-replacing route is invalidated, a new URI
   * goes through history (the route effect applies it), and the URI already shown is applied on the spot — back to a
   * space's index from one of its documents, whose URI is the space's own ({@link createShellRouteLedgerV1}). */
  /** 🪟️ The route the session stands at ({@link shellSessionRouteV1}): the shell URI, or the route an open overlay sits
   * over — kept so a session replaced under `/hub` returns to it. The ref is what the route code reads mid-flight, the
   * state what a render reads. */
  const sessionRouteRef = useRef<string | null>(null);
  const [sessionRoute, setSessionRoute] = useState<string | null>(null);
  /** 🚪️ Hands a session route to the lane unless its admission holds it ({@link shellRouteAdmissionV1}); a held route is
   * applied by the route effect once the identity it waits for resolves. */
  const requestSessionRoute = useCallback((uri: string): void => {
    const admission = shellRouteAdmissionV1(uri, identityResolutionRef.current);
    if (admission === "await-identity" || admission === "await-sign-in") return;
    void shellSessionLaneRef.current.route({ uri }).catch((uriError: unknown) => {
      if (!isPluginInstanceRetiredV1(uriError)) console.warn(`[os-shell] route ${uri} was not applied`, uriError);
    });
  }, []);
  const navigateShellUri = useCallback((uri: string) => {
    const overlay = shellRouteIsOverlayV1(uri);
    if (!overlay) shellRouteLedgerRef.current.invalidate();
    if (uri !== shellUriRef.current) {
      navigateHistory(uri);
      return;
    }
    if (!hostMode || overlay) return;
    requestSessionRoute(uri);
  }, [hostMode, navigateHistory, requestSessionRoute]);

  // 🐚️ `scope.storage` (not a separately-resolved ephemeral/browser port here) — two shells sharing a
  // page must not clobber each other's panel layout/dock state through an unnamespaced localStorage key.
  const shellStorage = scope.storage;
  const dockLayoutStore = useMemo(() => new DockLayoutStore(shellStorage, session?.app.id), [session?.app.id, shellStorage]);
  const dockUiStateStore = useMemo(() => new DockUiStateStore(shellStorage, session?.app.id), [session?.app.id, shellStorage]);

  const registry = useMemo(() => {
    const expanded = expandPluginRegistry(plugins, pluginFilter ? resolvePluginRegistryId(catalog, pluginFilter) : undefined, hostMode);
    if (hostMode) return expanded;
    return pluginFilter ? expanded : plugins;
  }, [catalog, pluginFilter, plugins, hostMode]);

  return {
    writeLocalDocumentRef, readLocalDocumentRef, recordLocalCatalogRef, cancelDirectoryBootstrap,
    dispatchSpaceAdministrationIntent, dispatchInferencePortIntent, cancelSessionAuthorityBootstrap, shellUri, canGoBack,
    canGoForward, canGoUp, goBack, goForward, goUp, navigateHistory, notFoundPath, sessionRouteRef, sessionRoute,
    setSessionRoute, requestSessionRoute, navigateShellUri, dockLayoutStore, dockUiStateStore, registry
  };
  //#endregion FrameworkOsShell
}
