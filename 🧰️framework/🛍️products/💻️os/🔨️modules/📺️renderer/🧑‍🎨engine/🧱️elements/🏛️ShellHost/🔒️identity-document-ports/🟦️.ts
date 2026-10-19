/** 🔒️ Ui device, theme and driver reads, the backbone worker refs, the agent bridge, identity and hub session state, directory home refs, the document opening ports, archive pairing, effect owners, browser actor ui and the space directory.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useIdentityAndDocumentPorts` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import {
  shellDialogOriginIsCurrentV1,
  shellDialogOriginV1,
  shellEffectOwnerIsCurrentV1,
  type ShellDialogOriginV1,
  type ShellDialogV1
} from "../🗨️dialog-origin/🟦️.ts";
import {
  BackgroundDocumentSessionsV1,
  DocumentAttachmentLaneV1,
  FolderArchivePersistenceV1,
  FolderReadBackRouteV1,
  LatestDocumentReplacementV1,
  replaceAttachedDocumentV1,
  type DocumentOpeningReceiptV1
} from "../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { type ArtifactCreationCatalogMountV1 } from "../🌱️artifact-creation/🚪️ready-opening/🟦️.ts";
import { type DirectorySessionRefreshV1 } from "@semio-tech/framework/directory/session-refresh";
import { AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1, AgentCredentialInstallUnavailableV1, parseAgentCredentialInstallReceiptV1 } from "@semio-tech/framework/directory/delegations";
import { HubSessionPortClientV1 } from "@semio-tech/framework/directory/session-refresh/session-port";
import { LOCAL_HUB_SESSION_ENDPOINT_V1, parseLocalHubSessionAnswerV1 } from "@semio-tech/framework/directory/local-session";
import { readHubSessionCapabilityV1, writeHubSessionCapabilityV1, type HubSessionCapabilityV1 } from "@semio-tech/framework/directory/sign-in";
import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState
} from "react";
// #region 🔌️Adapters
import {
  type AppRef,
  type AppRole,
  type ArtifactDialect,
  type AppCatalogue,
  type WindowEngagement,
  type WindowMeasure,
  type Conflict,
  type MergeReport,
  type Severity
} from "@semio-tech/framework";
import {
  type DocumentScope
} from "@semio-tech/framework/document/scope";
import {
  BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES,
  DOCUMENT_ARCHIVE_MAXIMUM_BYTES,
  encodeDocumentArchiveBytes,
  type DocumentArchivePack,
  type DocumentArchiveLoadStatus
} from "@semio-tech/framework/wire-contracts";
import {
  type BackboneWorkerResponse,
  type BrowserActorUiMountedV1,
  decodeBackboneMessage,
  encodeBackboneWorkerRequest,
  type PersistenceBinding,
  type ResolvedArtifactOpeningRelay
} from "@semio-tech/framework/backbone";
import {
  type DirectorySessionAuthorityV1
} from "@semio-tech/framework/directory/schema/session-authority-v1";
import {
  type DirectoryEvent
} from "@semio-tech/framework/directory/schema";
import {
  parseDirectorySpaceListEntryV1
} from "@semio-tech/framework/directory-client";
import {
  parseDirectoryCommandReceiptV1,
  sealDirectoryCommandRequestV1,
  directoryCommandRequestJson
} from "@semio-tech/framework/directory/io/text";
import {
  hubDeviceInstanceIdV1
} from "@semio-tech/framework/directory/sign-in";
import {
  type InstalledServiceHistoryStatusV1
} from "@semio-tech/framework/inference/service";
// #endregion 🔌️Adapters
import { DOCUMENT_BACKBONE_RETENTION_LIMITS } from "@semio-tech/framework/replication";
import { SpaceDirectoryHistoryV1 } from "../📇️space-directory/🟦️.ts";
import { shellIdentityResolutionV1 } from "../🔀️surface-switch/🟦️.ts";
import { programHistoryKeyV1, withProgramEntriesV1 } from "../🪟️spawned-program/🟦️.ts";
import { EMPTY_LOCAL_CATALOG_V1 } from "../🗂️local-catalog/🟦️.ts";
import { type Identity, type LocalCatalog } from "@semio-tech/framework/config/mutations";
import {
  builtinUiThemes,
  type ElementsSurfaceDevice,
  resolveUiDriver,
  semioTheme,
  type UiDriver,
  type UiTheme
} from "@semio-tech/ui-react";
import { UiDocumentStore } from "../../📃️UiDocumentStore/🟦️.tsx";
import { type HubConnectionPortV1 } from "../../🔗️HubConnection/🔌️connection-contract/🟦️.ts";
import { createHubConnectionFetchPortV1 } from "../../🔗️HubConnection/🛰️fetch-port/🟦️.ts";
import { sharedDocumentOpeningRoleV1, type DocumentOpeningReference, type DocumentOpeningTarget } from "../🧭️opening/🟦️.ts";
import { type ActiveSession } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { EMPTY_APP_CATALOGUE } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { downloadMediaExport } from "../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { mergeRecordPreservingIdentity, preserveJsonIdentity, type UiRefreshCache } from "../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { browserActorSectionValuesV1 } from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { programHistoryProjectionsWithLoadV1 } from "../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { parsePanelState } from "../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { type PluginWasmHandle } from "../../🔌️PluginRuntime/🟦️.tsx";
import { documentBackboneEffectV1, type ActorDocumentMessagePortV1 } from "@semio-tech/framework/backbone/binding";
import { BrowserActorActionMailboxV1 } from "@semio-tech/framework/browser-bundle/action-handoff/requests";
import { InferencePortOpeningMailboxV1 } from "@semio-tech/framework/inference/opening";
import { type InferencePortOwnerV1 } from "../🪪️host-bootstrap/🟦️.tsx";
import { reduceArtifactCreationProgressUiV1, type ArtifactCreationProgressUiStateV1 } from "../🌱️artifact-creation/🟦️.tsx";
import { closeDirectoryHomeOwnerV1, type DirectoryBootstrapUiState, type DirectoryHomeOwnerV1 } from "../📇️directory-bootstrap/🟦️.tsx";
import { agentBridgeOfferScopeFromDelegationsV1, useAgentBridge, type AgentAppCommandHandler, type AgentBridgeOfferScopeV1 } from "../../🔗️AgentBridge/🟦️.tsx";
import { shellAppFault } from "@semio-tech/framework/mcp/channel";
import type { BridgeInstanceRef } from "@semio-tech/framework/mcp/bridge";
import type { ReduceResult, ShellCommand } from "@semio-tech/framework/shell";
import { LOCAL_SESSION_RECLAIM_INTERVAL_MS } from "../🎚️shell-model/🟦️.tsx";
import { hubBootstrapOriginV1, hubSessionStorageV1, readViteSEnv } from "../🔐️identity-binding/🟦️.ts";
import {
  captureSpaceArtifactCreationCatalogAuthorityV1,
  type DirectoryCommandResultSlotV1,
  mintDirectoryCommandRequestId,
  type SpaceArtifactCreationOwnerV1
} from "../🧾️space-artifact-creation/🟦️.ts";
import { type ShellSpaceAdministrationStateV1 } from "../🏠️space-administration/🟦️.ts";
import { logUnlessRetiredV1 } from "../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterShellCoreState } from "../🧩️shell-core-state/🟦️.ts";

//#region FrameworkOsShell
/** 📇️ `applyHostEffects`'s new `replayShellCommand` branch (below) needs `openDocument`/
 * `openArtifactWithAppRef`, both declared LATER in this same component (after `applyHostEffects`
 * itself) — a direct reference in `applyHostEffects`'s own dependency array would be a `const`
 * temporal-dead-zone violation at the point `useCallback` evaluates that array. Same ref-forwarding
 * idiom `onActionRef`/`dispatchDirectoryEventsRef` already use: assigned as a plain statement right
 * after each real declaration, read only from inside a later callback body, never from a deps array. */
export type OpenDocumentSessionTarget = DocumentOpeningTarget<ActiveSession, PluginWasmHandle> & { readonly background?: boolean; readonly expectedCatalogGenerationId?: string };

export type PreparedArtifactOpeningTarget = OpenDocumentSessionTarget & Readonly<{ dialect: ArtifactDialect; role: AppRole }>;

export type OpenDocumentSession = {
  readonly openingReference: DocumentOpeningReference;
  readonly persistenceBindings: readonly PersistenceBinding[];
  session: ActiveSession;
  plugin: PluginWasmHandle;
  documentId: string;
  clientInstanceId: string;
  scope?: DocumentScope;
  port: ActorDocumentMessagePortV1 | null;
  pending: Uint8Array[];
  pendingBytes: number;
  replacements: LatestDocumentReplacementV1<DocumentArchivePack>;
  persistence: FolderArchivePersistenceV1;
  readBack: FolderReadBackRouteV1<DocumentArchivePack>;
  creationMount: ArtifactCreationCatalogMountV1 | null;
  ready: Promise<void>;
  resolveReady(): void;
  rejectReady(error: Error): void;
};

export type BackgroundSpaceIndexSession = {
  readonly plugin: PluginWasmHandle;
  readonly session: ActiveSession;
  readonly runtimeKey: string;
  readonly clientInstanceId: string;
  readonly hubBaseUrl: string;
  readonly userId: string;
};

type MountedInferenceHistoryV1 = Readonly<{ historyEpoch: number; clientInstanceId: string; scope: DocumentScope; sessionInstanceId: number; status: InstalledServiceHistoryStatusV1; owner: string; serviceId: string; order: number; authority: DirectorySessionAuthorityV1 }>;

export type RetainedBrowserActorUiV1 = { readonly clientInstanceId: string; readonly activationGeneration: string; readonly verifiedSurfaceId: string; readonly scope: DocumentScope; readonly sessionInstanceId: number; readonly windows: Map<string, UiDocumentStore>; readonly panels: Map<string, UiDocumentStore>; readonly sections: Map<string, UiDocumentStore>; readonly identity: BrowserActorUiMountedV1 | null; readonly actions: BrowserActorActionMailboxV1 };

export type ShellCoreAfterIdentityAndDocumentPorts = ShellCoreAfterShellCoreState & ReturnType<typeof useIdentityAndDocumentPorts>;

export function useIdentityAndDocumentPorts(core: ShellCoreAfterShellCoreState) {
  const {
    measuredDevice, dispatch, historyStore, uiLayout, uiDriverId, uiCustomDrivers, uiDriverDraft, uiLocale,
    uiTerminology, uiThemeId, uiCustomThemes, uiThemeDraft, refreshHistorySnapshot, dialogOpeningRef, liveDialogRef,
    openSpaceId, sessionRef, hostPorts
  } = core;
  const documentAccess = hostPorts?.documentAccess;
  const uiDevice: ElementsSurfaceDevice = measuredDevice === "desktop" ? uiLayout : measuredDevice;
  const uiTheme: UiTheme = useMemo(() => {
    if (uiThemeDraft) return uiThemeDraft;
    const found = builtinUiThemes().find((t) => t.id === uiThemeId) ?? uiCustomThemes[uiThemeId];
    return found ?? semioTheme();
  }, [uiThemeId, uiCustomThemes, uiThemeDraft]);
  const uiDriver: UiDriver = useMemo(() => uiDriverDraft ?? resolveUiDriver(uiDriverId, uiCustomDrivers), [uiDriverId, uiCustomDrivers, uiDriverDraft]);
  /** 🧵️ Lazily-created worker running `🟦️backbone-🟦️worker.ts` — one per shell instance, reused across `openDocument` calls. */
  const backboneWorkerRef = useRef<Worker | null>(null);
  const backboneWorkerStreamsDetachRef = useRef<(() => void) | null>(null);
  /** 🪪️ Per-tab session id component of the actor (contract §C0 `user:{userId}#{shellSessionId}`) —
   * stable for this tab's whole lifetime, shared by both the pre-identity `client-{id}` actor and the
   * post-sign-in `user:{userId}#{id}` one, so a tab's actor id only ever changes its PREFIX on sign-in,
   * never re-mints the suffix mid-session. */
  const shellSessionIdRef = useRef<string>(Math.random().toString(36).slice(2));
  /** 🖋️ Stable per-tab actor id for hub `Hello`/presence frames and operation-origin filtering. */
  const shellActorIdRef = useRef<string>(`client-${shellSessionIdRef.current}`);
  /** 🎛️ Applies one inbound agent `ShellCommand` to THIS host's real chrome. `useAgentBridge` runs
   * before `dispatch`/`dock` exist, so the seam is a ref the effect below fills — without it the
   * bridge reduces `ui_reveal`/`ui_focus` against its private mirror, answers the gateway `ok: true`
   * and nothing on screen moves, which is exactly what slice M7's live transcript measured. */
  const applyAgentShellCommandRef = useRef<(command: ShellCommand) => void>(() => {});
  /** 🗿️ Executes one agent artifact command against THIS host's live plugin instance. Same ref
   * rationale as {@link applyAgentShellCommandRef} — `useAgentBridge` runs thousands of lines before
   * `onAction`/`historyProjection` exist. Until the effect below fills it, the honest answer is a
   * typed refusal, never silence (a silent route costs the agent its whole wall budget). */
  const agentArtifactRouteRef = useRef<AgentAppCommandHandler>(() => [shellAppFault("plugin.unavailable", "this shell's artifact route is not mounted yet — no plugin session has been established")]);
  /** 🧾️ Prepared-but-uncommitted agent plans, by transaction id. The shell has no "compute the ops
   * without applying them" seam, so `action_prepare`'s ops are a DEFERRED plan parked here and
   * `transaction_commit` is the leg that dispatches — which is what makes a rollback a real no-op. */
  const agentDeferredPlansRef = useRef<Map<string, string>>(new Map());
  /** 📇️🪟️ The agent census, filled by the effect beside the spawned roster far below (the roster,
   * the focused program and the plugin manifests are all declared after this line, and
   * `useAgentBridge` takes a value rather than a ref). */
  const [agentBridgeCensus, setAgentBridgeCensus] = useState<readonly BridgeInstanceRef[]>([]);
  /** 📇️ What an agent may address in this shell. The gateway's `ShellArtifactChannel` resolves an
   * `AppCommand`'s target from exactly this census, so a shell with nothing open is a shell whose
   * artifact verbs answer `plugin.unavailable` by name.
   *
   * 🪟️ It is the live session PLUS every spawned program. Before this it was the session alone, which
   * made a spawned foreign editor invisible to an MCP client even while the human was working in it:
   * measured verbatim on the real `s` host with `note` spawned from the palette and its window on the
   * canvas (ticket 26/09/18, S6, `🗑️generated/s6-agent-gate-s.txt`) —
   *
   * ```
   * PLUGIN_UNAVAILABLE: capability `note.s.note.note@1/*#editor.addBlock` is owned by plugin `note`,
   * which has no open instance in the attached shell — open one there first
   * (the shell reports 1 open instance(s))
   * ```
   *
   * — that ONE instance being `s`'s own Home. It is the same session-only shape S5 found at five other
   * decision sites, on the last lane that still had it: outcome 4's agent could drive only the landing
   * app of a shell whose whole point is hosting every other plugin's artifacts.
   *
   * 🎯️ ORDER carries the focus: entry 0 is the program that owns the canvas right now. A
   * capability-less agent command has no other rule for choosing among several open instances, and
   * the shell's own answer to "which document does the user mean" is this one — which is also the
   * identity published on `data-semio-artifact-id`, so what a browser can be asked about and what
   * the agent route hands out stay the same string once more than one program is open. */
  const agentBridgeInstances = agentBridgeCensus;
  /** 🎯️ Who this shell is for the agent-bridge offer — assigned once the hub connection port exists below. */
  const agentBridgeOfferScopeRef = useRef<(signal: AbortSignal) => Promise<AgentBridgeOfferScopeV1>>(async () => null);
  // 🤖️ Agent bridge: stays `disabled` (never throws, never blocks render) until a gateway URL+token
  // is discovered, so a shell with no agent attached behaves exactly as before.
  const agentBridge = useAgentBridge({
    shellSessionId: shellSessionIdRef.current,
    offerScope: useCallback((signal: AbortSignal) => agentBridgeOfferScopeRef.current(signal), []),
    instances: agentBridgeInstances,
    onCommandApplied: useCallback((command: ShellCommand | null, result: ReduceResult | null) => {
      if (command && result?.ok) applyAgentShellCommandRef.current(command);
    }, []),
    // 🗿️ Mounting this is what makes the gateway select the `shell` channel for a session: the
    // `Hello` frame's `relayAppCommands` is derived from the handler being present.
    onAppCommand: useCallback<AgentAppCommandHandler>((request) => agentArtifactRouteRef.current(request), []),
  });
  /** 🪪️ §C3 identity bootstrap — `null` until `DirectoryClient.me()`/`mintSession` resolves (or forever,
   * with no hub env). Mirrored into {@link shellActorIdRef}/`setPluginRuntimeActor` by the effect below,
   * never read directly for the actor id (that's always `shellActorIdRef.current`). */
  const [identity, setIdentity] = useState<Identity | null>(null);
  const identityRef = useRef<Identity | null>(null);
  identityRef.current = identity;
  /** 🪪️ True once a hub env is configured but the hub could not be reached (§C3 "keep the last
   * persisted identity, show an offline chip... never blocks the UI thread"). */
  const [identityOffline, setIdentityOffline] = useState(false);
  /** 🚫️ The hub pronounced the remembered session over (refused or expired), or the human cancelled its
   * verification: the badge then offers sign-in instead of reading a shortage it would never recover from. */
  const [hubSessionRefused, setHubSessionRefused] = useState(false);
  const [verifiedSessionAuthority, setVerifiedSessionAuthority] = useState<DirectorySessionAuthorityV1 | null>(null);
  const verifiedSessionAuthorityRef = useRef<DirectorySessionAuthorityV1 | null>(null);
  const sessionRefreshRef = useRef<DirectorySessionRefreshV1 | null>(null);
  /** 🪪️ REST-only client for the identity boot handshake (`me`/`mintSession`) — distinct from the
   * directory-lane's persistent `/directory/socket/v1` subscription, which the shell never opens itself (§C6:
   * `🏪️store/👷️worker/🟦️.ts`'s `🔖️Directory` region is the only socket owner). Re-created only if the
   * hub base url or token actually changes. */
  const hubSessionPortRef = useRef<HubSessionPortClientV1 | null>(null);
  const hubEnv = useMemo(() => {
    const hubBaseUrl = readViteSEnv("VITE_S_HUB_URL");
    const dataDir = readViteSEnv("VITE_S_DATA_DIR");
    return hubBaseUrl ? { hubBaseUrl, dataDir } : null;
  }, []);
  /** 🎫️ The one hub session this shell acts with. Seeded from the browsing context's own store, so a
   * reload continues the session the human signed into; replaced by every mint and cleared by a
   * sign-out or by the hub refusing the capability. It is the ONLY identity input below: there is no
   * second, launcher-issued credential path any more. */
  const [hubSessionCapability, setHubSessionCapability] = useState<HubSessionCapabilityV1 | null>(() =>
    hubEnv === null ? null : readHubSessionCapabilityV1(hubSessionStorageV1(), hubBootstrapOriginV1()),
  );
  /** 🎫️ The capability as the hub pane's port reads it on every request — written here, together with the state, so a
   * request sent right after a mint already carries it. */
  const hubSessionCapabilityRef = useRef(hubSessionCapability);
  const rememberHubSessionCapability = useCallback((minted: Readonly<{ token: string; userId: string }> | null) => {
    const origin = hubBootstrapOriginV1();
    const next = minted === null ? null : { origin, token: minted.token, userId: minted.userId };
    writeHubSessionCapabilityV1(hubSessionStorageV1(), next);
    hubSessionCapabilityRef.current = next;
    setHubSessionCapability(next);
  }, []);
  const localSessionClaimedAtRef = useRef(0);
  useEffect(() => {
    if (!hubEnv || (hubSessionCapability !== null && !hubSessionRefused)) return;
    if (Date.now() - localSessionClaimedAtRef.current < LOCAL_SESSION_RECLAIM_INTERVAL_MS) return;
    localSessionClaimedAtRef.current = Date.now();
    let cancelled = false;
    void (async () => {
      try {
        const response = await fetch(LOCAL_HUB_SESSION_ENDPOINT_V1, { method: "GET", credentials: "same-origin", signal: AbortSignal.timeout(5_000) });
        if (!response.ok || cancelled) return;
        const session = parseLocalHubSessionAnswerV1(await response.json());
        if (cancelled || session === null) return;
        rememberHubSessionCapability(session);
      } catch {
        /* 🔌️ Hub down or local-only serve — stay local-first; identity refresh reconnects later. */
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [hubEnv, hubSessionCapability, hubSessionRefused, rememberHubSessionCapability]);

  /** 🪪️ Where the hub identity stands, for the route admission ({@link shellRouteAdmissionV1}): a space route waits for
   * the human it is opened for, and a hub that does not answer opens it offline. */
  const identityResolution = shellIdentityResolutionV1({ hubConfigured: hubEnv !== null, sessionHeld: hubSessionCapability !== null, refused: hubSessionRefused, confirmed: verifiedSessionAuthority !== null && identity?.userId === verifiedSessionAuthority.userId, offline: identityOffline });
  const identityResolutionRef = useRef(identityResolution);
  identityResolutionRef.current = identityResolution;
  /** 📇️ Retained directory owner bound to the same visible Home landing instance. */
  const directoryHomeOwnerRef = useRef<DirectoryHomeOwnerV1 | null>(null);
  const directoryHomeRetirementRef = useRef<{ readonly owner: DirectoryHomeOwnerV1; readonly promise: Promise<void> } | null>(null);
  const directoryHomeOpeningRef = useRef<Promise<void>>(Promise.resolve());
  const directoryBootstrapEpochRef = useRef(0);
  const [directoryBootstrapUi, setDirectoryBootstrapUi] = useState<DirectoryBootstrapUiState>({ kind: "idle" });
  const refreshDirectoryHomeRef = useRef<(session: ActiveSession) => Promise<void>>(async () => {});
  const handleDirectoryEventPageRef = useRef<(message: Extract<BackboneWorkerResponse, { readonly kind: "directory-event-page" }>) => void>(() => {});
  /** 🌐️ The worker's preference-lane answers (a page, a recorded change, a failure), read through a ref because the worker
   * handler is built once. */
  const handlePreferenceLaneMessageRef = useRef<(message: Extract<BackboneWorkerResponse, { readonly kind: "preference-lane-page" | "preference-lane-recorded" | "preference-lane-failed" }>) => void>(() => {});
  const handleDirectoryBootstrapFailureRef = useRef<(message: Extract<BackboneWorkerResponse, { readonly kind: "directory-bootstrap-failed" }>) => void>(() => {});
  const uiLocaleRef = useRef(uiLocale);
  const uiTerminologyRef = useRef(uiTerminology);
  uiLocaleRef.current = uiLocale;
  uiTerminologyRef.current = uiTerminology;
  const retireDirectoryHomeOwner = useCallback((owner: DirectoryHomeOwnerV1, worker: Worker | null): Promise<void> => {
    const current = directoryHomeRetirementRef.current;
    if (current?.owner === owner) return current.promise;
    const promise = closeDirectoryHomeOwnerV1(owner, (request) => worker?.postMessage({ wire: encodeBackboneWorkerRequest(request) }));
    directoryHomeRetirementRef.current = { owner, promise };
    void promise.finally(() => {
      if (directoryHomeRetirementRef.current?.owner === owner) directoryHomeRetirementRef.current = null;
    });
    return promise;
  }, []);
  /** 📇️ Set by the `DirectoryLane` region below once it's defined — `ensureBackboneWorker`'s
   * `onmessage` (created once, `useCallback([])`) reads this indirection rather than the callback
   * itself so a fold that depends on `session`/`hostConfig`/`onActionRef` never goes stale. */
  const dispatchDirectoryEventsRef = useRef<(events: readonly DirectoryEvent[]) => void>(() => {});
  /** ⚖️ Same ref-forwarding idiom as {@link dispatchDirectoryEventsRef} — `ensureBackboneWorker`'s
   * `remoteMutations` handling (below) needs `showTransientNotice`/`shellLabel`, both declared LATER
   * in this component (contract freeze `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
   * CONFLICTS` §C6/§C9: a peer's `MergeReport`/`Conflicts` reply to `ApplyEnvelopes` must reach the
   * Conflicts panel and, for a `"degraded"` outcome, a transient notice). Set once `applyRemoteMerge`
   * itself is defined, read only from inside the worker's `onmessage` closure body. */
  const applyRemoteMergeRef = useRef<(conflicts: readonly Conflict[] | null, mergeReport: MergeReport | null) => void>(() => {});
  /** 🧯️ Same ref-forwarding idiom as {@link applyRemoteMergeRef} — `applyHostEffects`'s `notify`
   * branch (declared ABOVE `showTransientNotice`) is the shell end of `kernel::Effect::Notify`, the
   * one channel a plugin has for telling the user its work was refused (e.g. puzzle3d's
   * `addBrushObject`/`acceptSuggestion` rejecting a colliding placement). Without this branch the
   * effect reached `wireEffectToFriendly` and was dropped, so a refusal looked exactly like nothing
   * happening. Assigned right after `showTransientNotice` itself is defined. */
  const showTransientNoticeRef = useRef<(message: string, kind?: Severity, code?: string) => void>(() => {});
  /** 📌️ Same idiom for the explicit check-in: `onAction` answers the history body's `framework.checkin` `submit` with
   * `dispatchCheckpoint`, which is declared below it in the 🔖️CheckIn region. */
  const dispatchCheckpointRef = useRef<(message: string) => void>(() => {});
  /** 🐚️ Same ref-forwarding idiom — `applyHostEffects`'s `replayShellCommand` branch replays a
   * SHELL-owned command id against shell-owned state (`dispatchOsCommand`, declared far below), never
   * into the guest, which has no window kind for chrome and refuses it `undeclared-action`. Answers
   * whether the id was actually routed; an unroutable one is a warning, never a guest dispatch. */
  const replayShellOwnedCommandRef = useRef<(commandId: string, args?: Record<string, unknown>) => boolean>(() => false);
  
  
  
  
  const backgroundSpaceIndexSessionsRef = useRef(new BackgroundDocumentSessionsV1<BackgroundSpaceIndexSession>());
  const openDocumentRef = useRef<(ref: DocumentOpeningReference, bindings?: readonly PersistenceBinding[], target?: OpenDocumentSessionTarget) => Promise<DocumentOpeningReceiptV1 | null>>(async () => null);
  const closeDocumentRef = useRef<(runtimeKey: string, clientInstanceId?: string) => void>(() => {});
  const documentAttachmentLanesRef = useRef(new WeakMap<PluginWasmHandle, Map<number, DocumentAttachmentLaneV1>>());
  const documentPortsRef = useRef(new WeakMap<PluginWasmHandle, Map<number, ActorDocumentMessagePortV1>>());
  const documentAttachmentLane = useCallback((plugin: PluginWasmHandle, instanceId: number): DocumentAttachmentLaneV1 => {
    let lanes = documentAttachmentLanesRef.current.get(plugin);
    if (lanes === undefined) { lanes = new Map(); documentAttachmentLanesRef.current.set(plugin, lanes); }
    let lane = lanes.get(instanceId);
    if (lane === undefined) {
      lane = new DocumentAttachmentLaneV1(async () => {
        const ports = documentPortsRef.current.get(plugin);
        const port = ports?.get(instanceId);
        await port?.retire().catch(logUnlessRetiredV1("[os-shell] a document port could not be retired"));
        if (port !== undefined && ports?.get(instanceId) === port) ports.delete(instanceId);
      });
      lanes.set(instanceId, lane);
    }
    return lane;
  }, []);
  const retireDocumentAttachment = useCallback(async (plugin: PluginWasmHandle, instanceId: number, clientInstanceId: string): Promise<void> => {
    const lanes = documentAttachmentLanesRef.current.get(plugin);
    const lane = lanes?.get(instanceId);
    if (lane === undefined) return;
    await lane.close(clientInstanceId);
    if (lane.idle && lanes?.get(instanceId) === lane) lanes.delete(instanceId);
  }, []);
  const openArtifactWithAppRefRef = useRef<(target: AppRef, dialect: ArtifactDialect, role: AppRole, admit?: () => boolean, publish?: boolean) => Promise<PreparedArtifactOpeningTarget | null>>(async () => null);
  const resolveArtifactOpeningRelayRef = useRef<(actionId: string, args: unknown) => ResolvedArtifactOpeningRelay | null>(() => null);
  /** 🎬️ The hub's "open an artifact whose plugin has never been loaded" resolution — see
   * {@link installActivationOwnerAndResolve}. Sync resolution stays on
   * {@link resolveArtifactOpeningRelayRef}; only this one may install. */
  const resolveArtifactOpeningWithActivationRef = useRef<(actionId: string, args: unknown) => Promise<ResolvedArtifactOpeningRelay | null>>(async () => null);
  const openReadySpaceArtifactCreationRef = useRef<(requestId: string) => void>(() => {});
  const spaceArtifactCreationOwnersRef = useRef(new Map<string, SpaceArtifactCreationOwnerV1>());
  const [spaceArtifactCreationUi, setSpaceArtifactCreationUi] = useState<ArtifactCreationProgressUiStateV1>({});
  const [spaceArtifactCreationCatalog, setSpaceArtifactCreationCatalog] = useState<Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog" }> | null>(null);
  const [spaceArtifactCreationCatalogUi, setSpaceArtifactCreationCatalogUi] = useState<Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog-status" }> | null>(null);
  const spaceArtifactCreationCatalogRef = useRef(spaceArtifactCreationCatalog);
  const spaceArtifactCreationCatalogUiRef = useRef(spaceArtifactCreationCatalogUi);
  spaceArtifactCreationCatalogRef.current = spaceArtifactCreationCatalog;
  spaceArtifactCreationCatalogUiRef.current = spaceArtifactCreationCatalogUi;
  const cancelSpaceArtifactCreationsForRuntime = useCallback((runtimeKey: string, worker: Worker | null) => {
    const cleared: string[] = [];
    for (const [requestId, owner] of spaceArtifactCreationOwnersRef.current) {
      if (owner.runtimeKey !== runtimeKey) continue;
      spaceArtifactCreationOwnersRef.current.delete(requestId);
      cleared.push(requestId);
      if (owner.ready === null) worker?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-create-cancel", requestId, spaceId: owner.spaceId }) });
    }
    if (cleared.length > 0) setSpaceArtifactCreationUi((current) => cleared.reduce((next, requestId) => reduceArtifactCreationProgressUiV1(next, { kind: "cleared", requestId }), current));
  }, []);
  const cancelSpaceArtifactCreation = useCallback((requestId: string, spaceId: string) => {
    const owner = spaceArtifactCreationOwnersRef.current.get(requestId);
    if (owner === undefined || owner.spaceId !== spaceId || owner.cancelRequested || owner.opening || owner.ready !== null) return;
    spaceArtifactCreationOwnersRef.current.set(requestId, { ...owner, cancelRequested: true });
    setSpaceArtifactCreationUi((current) => reduceArtifactCreationProgressUiV1(current, { kind: "cancel-requested", requestId, spaceId }));
    backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-create-cancel", requestId, spaceId }) });
  }, []);
  /** 📇️ Offline-queue depth surfaced by the worker's `directory-status` — not yet rendered by any
   * chrome this lane owns (2-F/3-A's "row shows 'pending'" territory); kept as local state so a
   * consumer can read it once that chrome lands, with zero further plumbing here. */
  const [, setDirectoryPendingCommands] = useState(0);
  /** 🧾️ Retained, request-id-keyed command result slot. A receipt lands here BEFORE any accepted
   * event is folded, so a completion always has an owner; a one-shot invite capability lives only
   * in this slot until an explicit administration copy handler consumes it, and is never logged,
   * folded into Home, or put into React state telemetry. */
  const directoryCommandResultsRef = useRef(new Map<string, DirectoryCommandResultSlotV1>());
  /** 🏛️ The one Shell-owned administration pane state. It is filled ONLY from the worker's retained
   * `DirectoryAdministrationOperation`: the exact canonical page bytes the hub sealed, the phase,
   * and whether a one-shot invite capability is still held by the worker. No role, session identity,
   * bearer, cursor key, or invite token is ever stored here — a terminal phase arrives with the page
   * already erased, so an unmount, identity change, 401/403 or scoped 4401 clears the pane. */
  const [spaceAdministration, setSpaceAdministration] = useState<ShellSpaceAdministrationStateV1 | null>(null);
  // 🔗️ ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END slice AU2 — the hub connection surface
  // (sign in, browse/switch/create spaces, invite, redeem). Reached at `/hub`; the port is built once
  // per shell and keeps the minted capability inside its own closure, never in React state.
  const [hubWorkspaceOpen, setHubWorkspaceOpen] = useState(false);
  const hubConnectionPort = useMemo<HubConnectionPortV1>(
    () =>
      createHubConnectionFetchPortV1({
        request: (url, init, signal) => fetch(url, { ...init, credentials: "include", signal }),
        storage: typeof globalThis.localStorage === "undefined" ? null : globalThis.localStorage,
        // 🌐️ The hub is its own origin, not the UI's: `VITE_S_HUB_URL` is the dev/production value the
        // build already injects as non-secret collaborative endpoint metadata
        // (`🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`). Falling back to the page origin only matters where hub and
        // UI really are co-served; a sign-in against the UI origin would otherwise post credentials
        // at the SPA's own index route. The capability this mints is the shell's ONLY identity: it is
        // remembered for this browsing context and handed to the credential-owning backbone worker,
        // whose own hub lane (`/_semio/hub/*`, same-origin) carries exactly this human's session.
        bootstrapOrigin: hubBootstrapOriginV1(),
        capability: { read: () => hubSessionCapabilityRef.current, write: rememberHubSessionCapability },
        deviceInstanceId: hubDeviceInstanceIdV1(typeof globalThis.localStorage === "undefined" ? null : globalThis.localStorage),
        clientClass: "browser",
        parseSpaces: (body) => (JSON.parse(body) as unknown[]).map(parseDirectorySpaceListEntryV1),
        sealCommand: (command) => {
          const request = sealDirectoryCommandRequestV1(mintDirectoryCommandRequestId(), command);
          return { body: directoryCommandRequestJson(request), parseReceipt: (text) => parseDirectoryCommandReceiptV1(text, request) };
        },
        writeClipboard: typeof navigator === "undefined" || navigator.clipboard === undefined ? undefined : (text) => navigator.clipboard.writeText(text),
        // 📄️ ticket 26/09/18 slice M6b — the one-time agent credential leaves as a downloaded file,
        // never through the clipboard: it must end up somewhere `semio-os-mcp --credential-file` can
        // read at mode 0600, and a paste buffer is neither a file nor private.
        saveFile: typeof document === "undefined"
          ? undefined
          : async (file) => {
              downloadMediaExport(file.fileName, file.mediaType, file.contents);
            },
        installAgentCredential: async (request, signal) => {
          const response = await fetch(AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(request), signal, cache: "no-store" });
          if (response.status === 404 || response.status === 405) throw new AgentCredentialInstallUnavailableV1();
          if (!response.ok) throw new Error(`directory.delegations.install-refused.${response.status}`);
          return parseAgentCredentialInstallReceiptV1(await response.text());
        },
        uninstallAgentCredential: async (delegationId) => {
          await fetch(`${AGENT_CREDENTIAL_INSTALL_ENDPOINT_V1}/${encodeURIComponent(delegationId)}`, { method: "DELETE", cache: "no-store" });
        },
      }),
    [rememberHubSessionCapability],
  );
  /** 🎯️ The scope this shell asks the agent-bridge supervisor with (ticket 26/09/23, G12 session 14c — scoped offers): its
   * hub, the space it has open and the agents THIS human delegated there, read from the hub, which lists only the asking
   * human's own. A hub-bound gateway's offer is then served to its own human's shell and to no other. */
  agentBridgeOfferScopeRef.current = async (signal) => {
    if (hubEnv === null || openSpaceId === null || hubSessionCapabilityRef.current === null) return null;
    const hubOrigin = hubBootstrapOriginV1();
    return agentBridgeOfferScopeFromDelegationsV1(hubOrigin, openSpaceId, await hubConnectionPort.listAgentDelegations(hubOrigin, openSpaceId, signal), Date.now());
  };
  const spaceAdministrationRef = useRef<ShellSpaceAdministrationStateV1 | null>(null);
  spaceAdministrationRef.current = spaceAdministration;
  const spaceAdministrationEpochRef = useRef(0);
  const spaceAdministrationMessageRef = useRef(0);
  /** 💡️ Monotonic owner of the one worker-side inference port; a stale epoch's status is ignored. */
  const inferencePortEpochRef = useRef(0);
  /** 🗂️ Exact scope and runtime owner for the current inference epoch. */
  const inferencePortOwnerRef = useRef<InferencePortOwnerV1 | null>(null);
  const inferencePortAuthorityRef = useRef<DirectorySessionAuthorityV1 | null>(null);
  const retiredSessionDocumentOwnersRef = useRef(new WeakSet<object>());
  const inferencePortOpeningRef = useRef<{ readonly owner: InferencePortOwnerV1; readonly clientInstanceId: string; readonly sessionInstanceId: number; readonly mailbox: InferencePortOpeningMailboxV1 } | null>(null);
  
  const [inferenceHistoryByRuntimeKey, setInferenceHistoryByRuntimeKey] = useState<Readonly<Record<string, MountedInferenceHistoryV1>>>({});
  const inferenceHistoryByRuntimeKeyRef = useRef<Readonly<Record<string, MountedInferenceHistoryV1>>>({});
  inferenceHistoryByRuntimeKeyRef.current = inferenceHistoryByRuntimeKey;
  /** 🪪️ Settled once by the identity bootstrap effect's very first `documentArchiveReplaced` for
   * `IDENTITY_CONFIG_SCHEMA` (a previously-persisted session) — see that effect for the bounded
   * timeout that resolves it to `null` when no such file exists (never blocks the UI thread). */
  const identitySnapshotResolverRef = useRef<((value: Identity | null) => void) | null>(null);
  const identityClientInstanceIdRef = useRef<string | null>(null);
  const identityBootstrapAbortRef = useRef<AbortController | null>(null);
  /** 🗂️ The host-owned local document catalog (`os.config.local-catalog`, persisted local-only in `${S_DATA_DIR}/os`) —
   * `🗂️local-catalog/🟦️.ts`. Opened at mount, local-first, with or without a hub. `localDocumentOwnersRef` routes one kept
   * document's worker events (a write being verified, a read-back) by the client id it was opened under;
   * `rehydratedLocalDocumentsRef` remembers, per landing-app instance, which kept documents it was already handed. */
  const localDataDir = useMemo(() => readViteSEnv("VITE_S_DATA_DIR"), []);
  const [localCatalog, setLocalCatalog] = useState<LocalCatalog>(EMPTY_LOCAL_CATALOG_V1);
  const localCatalogRef = useRef<LocalCatalog>(EMPTY_LOCAL_CATALOG_V1);
  localCatalogRef.current = localCatalog;
  const localCatalogClientInstanceIdRef = useRef<string | null>(null);
  const localDocumentOwnersRef = useRef(new Map<string, { readonly status?: (persisted: boolean) => void; readonly archive?: (archive: Uint8Array) => void }>());
  const rehydratedLocalDocumentsRef = useRef(new Map<number, Set<string>>());
  const presenceConnectedAtMsRef = useRef(Date.now());
  const presenceCursorRef = useRef<{ readonly x: number; readonly y: number } | undefined>(undefined);
  /** 🐚️ terra-web-shellhost (finding 5) — per-document `latestWins` triggers for the presence-beat
   * effect below: keyed lazily on first beat so each document gets its own single-flight-with-trailing-
   * coalesce wrapper instead of sharing one across every open document, which would serialize them.
   * Cleared on that effect's own cleanup (identity/ephemeral change or unmount). */
  const presenceBeatTriggersRef = useRef<Map<string, () => Promise<void>>>(new Map());
  /** 🌐️ terra-web-shellhost (finding 4) — one `AbortController` for the whole component's lifetime,
   * aborted from the unmount-teardown effect below (never recreated: this file's own `presenceConnectedAtMsRef`
   * a few lines up is the same "construct once, keep for the component's life" idiom). Ties every
   * extension install fetch (`installExtension`/`installExtensionFromFile`/`uninstallExtension`) to
   * the component's own lifetime instead of leaving them free-running past unmount. */
  const extensionFetchAbortRef = useRef(new AbortController());
  /** 🧵 Cancels every active operation-owned download drain when this shell unmounts. */
  const segmentedDownloadAbortRef = useRef(new AbortController());
  /** 🗂️ Which session/plugin owns each exact document runtime, so equal document ids in different
   * spaces cannot share socket, bootstrap, presence or plugin-routing state. */
  const openDocumentSessionsRef = useRef<Map<string, OpenDocumentSession>>(new Map());
  const failDocumentBackbone = useCallback((runtimeKey: string, entry: OpenDocumentSession, failure: unknown) => {
    if (openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
    const error = failure instanceof Error ? failure : new Error(String(failure));
    if (entry.replacements.pending) {
      return;
    }
    entry.creationMount?.close(error);
    entry.rejectReady(error);
    closeDocumentRef.current(runtimeKey, entry.clientInstanceId);
  }, []);
  const receiveDocumentBackbone = useCallback((runtimeKey: string, entry: OpenDocumentSession, message: Uint8Array) => {
    if (openDocumentSessionsRef.current.get(runtimeKey) !== entry) return;
    try {
      if (!(message instanceof Uint8Array) || message.length === 0 || message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || !["mutations", "retract"].includes(decodeBackboneMessage(message).kind)) throw new Error("document-backbone.invalid-message");
      if (entry.port === null) {
        if (entry.pending.length >= DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumMessages || message.length > DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes - entry.pendingBytes) {
          return;
        }
        entry.pending.push(message.slice());
        entry.pendingBytes += message.length;
      } else void entry.persistence.ingested(entry.port.receive({ runtimeKey, clientInstanceId: entry.clientInstanceId, scope: entry.scope ?? null }, message)).catch(error => failDocumentBackbone(runtimeKey, entry, error));
    } catch (error) { failDocumentBackbone(runtimeKey, entry, error); }
  }, [failDocumentBackbone]);
  const bindDocumentBackbone = useCallback(async (runtimeKey: string, entry: OpenDocumentSession, admitted: () => boolean = () => true): Promise<void> => {
    const current = () => admitted() && openDocumentSessionsRef.current.get(runtimeKey) === entry;
    if (!current()) return;
    if (!entry.plugin.bindDocumentPort) throw new Error("document-backbone.binding-unavailable");
    const worker = backboneWorkerRef.current;
    if (worker === null) throw new Error("document-backbone.worker-unavailable");
    const preparation: { port?: ActorDocumentMessagePortV1 } = {};
    try { await entry.plugin.bindDocumentPort(entry.session.instanceId, {
      runtimeKey, clientInstanceId: entry.clientInstanceId, scope: entry.scope ?? null, current,
      send: message => {
        if (!current() || backboneWorkerRef.current !== worker) return;
        if (documentBackboneEffectV1(message) === "remote-ingest-receipt") return;
        worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: entry.documentId, clientInstanceId: entry.clientInstanceId, ...(entry.scope === undefined ? {} : { spaceId: entry.scope.spaceId }), message: { kind: "documentBackbone", message } }) });
        void entry.persistence.published().catch(error => failDocumentBackbone(runtimeKey, entry, error));
      },
      merge: (conflicts, report) => { if (current()) applyRemoteMergeRef.current(conflicts, report); },
      prepared: port => {
        preparation.port = port;
        entry.port = port;
        let ports = documentPortsRef.current.get(entry.plugin);
        if (ports === undefined) { ports = new Map(); documentPortsRef.current.set(entry.plugin, ports); }
        ports.set(entry.session.instanceId, port);
        const pending = entry.pending;
        entry.pending = [];
        entry.pendingBytes = 0;
        for (const bytes of pending) receiveDocumentBackbone(runtimeKey, entry, bytes);
      },
    }); } catch (error) {
      const candidate = preparation.port;
      await candidate?.retire().catch(() => {});
      if (candidate !== undefined) {
        const ports = documentPortsRef.current.get(entry.plugin);
        if (ports?.get(entry.session.instanceId) === candidate) ports.delete(entry.session.instanceId);
        if (entry.port === candidate) {
          entry.port = null;
          entry.pending = [];
          entry.pendingBytes = 0;
        }
      }
      throw error;
    }
    if (current()) entry.resolveReady();
  }, [receiveDocumentBackbone]);
  /** 🗃️ Loads one archive into a program through the stepped, ACK-owned archive load: `task` hears every polled status and
   * cancels it; a cancelled or faulted load leaves the previous document exactly as it was and its backbone bound again. */
  const loadDocumentArchive = useCallback(async (plugin: PluginWasmHandle, instanceId: number, archive: DocumentArchivePack, current: () => boolean, task?: { readonly signal?: AbortSignal; readonly progress?: (status: DocumentArchiveLoadStatus) => void }): Promise<boolean> => {
    const archiveBytes = encodeDocumentArchiveBytes(archive);
    if (archiveBytes.length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES) throw new Error("document-backbone.archive-capacity");
    if (!plugin.loadAppDocumentArchive) throw new Error("document-backbone.archive-loader-unavailable");
    const load = plugin.loadAppDocumentArchive;
    const owned = [...openDocumentSessionsRef.current].find(([, entry]) => entry.plugin === plugin && entry.session.instanceId === instanceId);
    const lane = documentAttachmentLane(plugin, instanceId);
    if (owned === undefined) {
      const owner = `cold:${crypto.randomUUID()}`;
      const unbound = () => current() && ![...openDocumentSessionsRef.current.values()].some(entry => entry.plugin === plugin && entry.session.instanceId === instanceId);
      let loaded = false;
      try { await lane.replace(owner, unbound, async () => { if (unbound()) { await load(instanceId, archive, task?.signal, task?.progress); loaded = unbound(); } }); }
      finally { await lane.close(owner); }
      return loaded;
    }
    const [runtimeKey, entry] = owned;
    const retirement = entry.port?.retire();
    void retirement?.catch(() => {});
    entry.port = null;
    return entry.replacements.replace(archive, (candidate, latest) => {
      const exact = () => latest() && current() && openDocumentSessionsRef.current.get(runtimeKey) === entry;
      return replaceAttachedDocumentV1(lane, entry.clientInstanceId, exact, { retired: retirement, load: () => load(instanceId, candidate, task?.signal, task?.progress), bind: () => bindDocumentBackbone(runtimeKey, entry, latest) });
    });
  }, [bindDocumentBackbone, documentAttachmentLane]);
  const loadDocumentPair = useCallback(async (plugin: PluginWasmHandle, instanceId: number, pack: Uint8Array, spr: Uint8Array, current: () => boolean, task?: { readonly signal?: AbortSignal; readonly progress?: (status: DocumentArchiveLoadStatus) => void }): Promise<boolean> =>
    loadDocumentArchive(plugin, instanceId, { parent_pack: Array.from(pack), parent_spr: Array.from(spr), members: [] }, current, task), [loadDocumentArchive]);
  const captureDialogOrigin = useCallback((target: ActiveSession | null): ShellDialogOriginV1 | null =>
    shellDialogOriginV1(target, [...openDocumentSessionsRef.current].map(([runtimeKey, entry]) => ({ runtimeKey, ...entry }))), []);
  const isCurrentDialogOrigin = useCallback((origin: ShellDialogOriginV1 | null): boolean =>
    shellDialogOriginIsCurrentV1(origin, captureDialogOrigin(core.shellStateRef.current.pluginRuntime.session)), [captureDialogOrigin]);
  /** 🔗️ `inputSeq` is the ledger entry (`🎯️input-ledger`) whose guest answer this owner's effect pass
   * applies — `onAction` passes its own entry, every other owner (a refresh, a tutorial, a route) is a
   * root (`null`) — so a guest follow-up (`replayShellCommand`, `dispatchAction`) can carry `causedBy`. */
  const captureEffectOwner = useCallback((source: ActiveSession, presentation: ShellDialogOriginV1 | null, inputSeq: number | null = null) => ({
    presentation,
    source: captureDialogOrigin(source),
    session: source,
    plugin: core.loadedPluginsRef.current.find((entry) => entry.handle.pluginId === source.pluginId)?.handle ?? null,
    creationCatalog: captureSpaceArtifactCreationCatalogAuthorityV1(spaceArtifactCreationCatalogRef.current, spaceArtifactCreationCatalogUiRef.current, presentation),
    inputSeq,
  }), [captureDialogOrigin]);
  const isCurrentEffectOwner = useCallback((owner: ReturnType<typeof captureEffectOwner>): boolean => {
    const primary = core.shellStateRef.current.pluginRuntime.session;
    return owner.plugin !== null
      && core.loadedPluginsRef.current.find((entry) => entry.handle.pluginId === owner.session.pluginId)?.handle === owner.plugin
      && shellEffectOwnerIsCurrentV1(owner, { presentation: captureDialogOrigin(primary), mounted: captureDialogOrigin(owner.session), primary, spawned: primary === null ? [] : parsePanelState(primary.viewState)?.spawnedApps ?? [] });
  }, [captureDialogOrigin]);
  /** 🎭️ The owner of a pass a PROGRAM runs on its own — its operation progress and completions, its deferred effects:
   * the primary session presents it, the program is its source ({@link shellEffectOwnerIsCurrentV1}). */
  const captureProgramEffectOwner = useCallback((program: ActiveSession) => captureEffectOwner(program, captureDialogOrigin(core.shellStateRef.current.pluginRuntime.session)), [captureDialogOrigin, captureEffectOwner]);
  /** 🚪️ What restoring an archive into a program drives ({@link FolderReadBackRouteV1}): the
   * stepped load through the document's attachment lane — every polled status is the program's `load` status outside the
   * History panel until the load ends, as for a file-open load — the program's history re-read and a full refresh of
   * every surface it renders, through the live session when the program is the primary one. */
  const documentArchivePorts = useCallback((plugin: PluginWasmHandle, program: ActiveSession, current: () => boolean) => {
    const live = (): ActiveSession => {
      const primary = sessionRef.current;
      return primary !== null && primary.pluginId === program.pluginId && primary.instanceId === program.instanceId ? primary : program;
    };
    const showLoad = (load: { readonly completed: number; readonly total: number } | null) => historyStore.update((projections) => programHistoryProjectionsWithLoadV1(projections, programHistoryKeyV1(program), load));
    return {
      load: (value: DocumentArchivePack) => loadDocumentArchive(plugin, program.instanceId, value, current, { progress: showLoad }).finally(() => showLoad(null)),
      history: () => refreshHistorySnapshot(program),
      refresh: () => core.applyHostEffectsRef.current([], live(), { kind: "full" }, captureProgramEffectOwner(live())),
    };
  }, [captureProgramEffectOwner, historyStore, loadDocumentArchive, refreshHistorySnapshot]);
  const closeOwnedDialog = useCallback((openingId: number): boolean => {
    if (liveDialogRef.current?.openingId !== openingId) return false;
    liveDialogRef.current = null;
    dispatch({ type: "CLOSE_DIALOG", openingId });
    return true;
  }, []);
  const makeOwnedDialog = useCallback((dialogId: string, origin: ShellDialogOriginV1 | null, seedArgs?: Readonly<Record<string, unknown>>): ShellDialogV1 | null => {
    const active = core.shellStateRef.current.pluginRuntime.session;
    if (origin === null || !isCurrentDialogOrigin(origin) || !active?.app.dialogs?.some((entry) => entry.id === dialogId)) return null;
    return { openingId: ++dialogOpeningRef.current, dialogId, origin, ...(seedArgs === undefined ? {} : { seedArgs }) };
  }, [isCurrentDialogOrigin]);
  
  const browserActorUiByRuntimeKeyRef = useRef(new Map<string, RetainedBrowserActorUiV1>());
  const [browserActorUiVersion, setBrowserActorUiVersion] = useState(0);
  /** 🎭️ The painted actor serving `session`'s document, if any: its reserved section stores are then the ONLY source of the
   * session's engagements, measures, tool measures and catalogue — the local instance never sees a hub document, so its
   * sections froze at the opening state (C13 P1: draw's "N layers" stayed 1 after committed `addLayer`s). */
  const browserActorForSessionV1 = (session: Pick<ActiveSession, "pluginId" | "instanceId">): RetainedBrowserActorUiV1 | null => {
    for (const [runtimeKey, retained] of browserActorUiByRuntimeKeyRef.current) {
      const entry = openDocumentSessionsRef.current.get(runtimeKey);
      if (entry?.clientInstanceId === retained.clientInstanceId && retained.sessionInstanceId === session.instanceId && entry.session.pluginId === session.pluginId && entry.session.instanceId === session.instanceId) return retained;
    }
    return null;
  };
  /** 🧩️ Dispatches `cache`'s reserved sections — after taking them from `retained`'s section stores when an actor serves the
   * session — exactly as a local refresh does, so window chrome and tool measures follow the live document in every open
   * window. The app-static catalogue is kept by identity, so a scene host subscribing through `AppCatalogueContext` never
   * re-renders on an unchanged one. */
  const publishUiRefreshSectionsV1 = (cache: UiRefreshCache, retained: RetainedBrowserActorUiV1 | null): void => {
    if (retained !== null) for (const [key, { revision, value }] of browserActorSectionValuesV1(retained.sections, `actor ${retained.verifiedSurfaceId}`)) cache.set(key, { hash: `actor:${revision}`, value });
    const engagements = (cache.get("engagements")?.value as Readonly<Record<string, WindowEngagement>> | undefined) ?? {},
      measures = (cache.get("measures")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {},
      toolMeasures = (cache.get("tools")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {},
      catalogue = (cache.get("catalogue")?.value as AppCatalogue | undefined) ?? EMPTY_APP_CATALOGUE;
    dispatch({ type: "SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID", value: (current) => mergeRecordPreservingIdentity(current, Object.entries(engagements)) });
    dispatch({ type: "SET_WINDOW_MEASURES_BY_WINDOW_ID", value: (current) => mergeRecordPreservingIdentity(current, Object.entries(measures)) });
    dispatch({ type: "SET_TOOL_MEASURES_BY_TOOL_ID", value: (current) => mergeRecordPreservingIdentity(current, withProgramEntriesV1(current, null, core.spawnedIdsRef.current, Object.entries(toolMeasures))) });
    dispatch({ type: "SET_APP_CATALOGUE", value: (current) => preserveJsonIdentity(current, catalogue) });
  };
  const retireBrowserActorUi = useCallback((runtimeKey: string, reason: string) => {
    browserActorUiByRuntimeKeyRef.current.get(runtimeKey)?.actions.close(reason);
    if (browserActorUiByRuntimeKeyRef.current.delete(runtimeKey)) setBrowserActorUiVersion((current) => current + 1);
  }, []);
  const directoryScopedOwnersRef = useRef<Map<string, DocumentScope>>(new Map());
  /** 📇️ The directory history of every mounted space index, by its runtime key ({@link SpaceDirectoryHistoryV1}). */
  const spaceDirectoryHistoriesRef = useRef<Map<string, SpaceDirectoryHistoryV1>>(new Map());
  const spaceDirectoryFoldPendingRef = useRef(false);
  const foldSpaceDirectoryRef = useRef<() => void>(() => {});
  /** 📇️ Collects directory events into every mounted space index's history of their space and folds the current
   * index's full history once per turn when it changed. */
  /** 👁️ The surface role the signed-in human may open each shared space's documents with ({@link sharedDocumentOpeningRoleV1}),
   * decided at every opening from the space index and kept current while an index of the space folds directory events. */
  const sharedSpaceAccessRef = useRef<Map<string, AppRole>>(new Map());
  const [sharedSpaceAccessVersion, setSharedSpaceAccessVersion] = useState(0);
  const adoptSharedSpaceAccess = useCallback((spaceId: string, role: AppRole | null): void => {
    if (role === null || sharedSpaceAccessRef.current.get(spaceId) === role) return;
    sharedSpaceAccessRef.current.set(spaceId, role);
    setSharedSpaceAccessVersion((version) => version + 1);
  }, []);
  const collectSpaceDirectoryEvents = useCallback((events: readonly DirectoryEvent[]): void => {
    let changed = false;
    for (const history of spaceDirectoryHistoriesRef.current.values()) {
      if (!history.add(events)) continue;
      changed = true;
      if (identityRef.current) adoptSharedSpaceAccess(history.spaceId, sharedDocumentOpeningRoleV1(history.events(), history.spaceId, identityRef.current.userId, documentAccess));
    }
    if (!changed || spaceDirectoryFoldPendingRef.current) return;
    spaceDirectoryFoldPendingRef.current = true;
    queueMicrotask(() => {
      spaceDirectoryFoldPendingRef.current = false;
      foldSpaceDirectoryRef.current();
    });
  }, [documentAccess]);

  return {
    uiDevice, uiTheme, uiDriver, backboneWorkerRef, backboneWorkerStreamsDetachRef, shellSessionIdRef, shellActorIdRef,
    applyAgentShellCommandRef, agentArtifactRouteRef, agentDeferredPlansRef, setAgentBridgeCensus, agentBridgeInstances,
    agentBridge, identity, setIdentity, identityRef, identityOffline, setIdentityOffline, hubSessionRefused,
    setHubSessionRefused, verifiedSessionAuthority, setVerifiedSessionAuthority, verifiedSessionAuthorityRef,
    sessionRefreshRef, hubSessionPortRef, hubEnv, hubSessionCapability, identityResolution, identityResolutionRef,
    directoryHomeOwnerRef, directoryHomeRetirementRef, directoryHomeOpeningRef, directoryBootstrapEpochRef,
    directoryBootstrapUi, setDirectoryBootstrapUi, refreshDirectoryHomeRef, handleDirectoryEventPageRef,
    handlePreferenceLaneMessageRef, handleDirectoryBootstrapFailureRef, uiLocaleRef, uiTerminologyRef,
    retireDirectoryHomeOwner, dispatchDirectoryEventsRef, applyRemoteMergeRef, showTransientNoticeRef,
    dispatchCheckpointRef, replayShellOwnedCommandRef, backgroundSpaceIndexSessionsRef, openDocumentRef,
    closeDocumentRef, documentAttachmentLanesRef, documentAttachmentLane, retireDocumentAttachment,
    openArtifactWithAppRefRef, resolveArtifactOpeningRelayRef, resolveArtifactOpeningWithActivationRef,
    openReadySpaceArtifactCreationRef, spaceArtifactCreationOwnersRef, spaceArtifactCreationUi,
    setSpaceArtifactCreationUi, spaceArtifactCreationCatalog, setSpaceArtifactCreationCatalog,
    spaceArtifactCreationCatalogUi, setSpaceArtifactCreationCatalogUi, spaceArtifactCreationCatalogRef,
    spaceArtifactCreationCatalogUiRef, cancelSpaceArtifactCreationsForRuntime, cancelSpaceArtifactCreation,
    setDirectoryPendingCommands, directoryCommandResultsRef, spaceAdministration, setSpaceAdministration,
    hubWorkspaceOpen, setHubWorkspaceOpen, hubConnectionPort, spaceAdministrationRef, spaceAdministrationEpochRef,
    spaceAdministrationMessageRef, inferencePortEpochRef, inferencePortOwnerRef, inferencePortAuthorityRef,
    retiredSessionDocumentOwnersRef, inferencePortOpeningRef, setInferenceHistoryByRuntimeKey,
    inferenceHistoryByRuntimeKeyRef, identitySnapshotResolverRef, identityClientInstanceIdRef, identityBootstrapAbortRef,
    localDataDir, localCatalog, setLocalCatalog, localCatalogRef, localCatalogClientInstanceIdRef,
    localDocumentOwnersRef, rehydratedLocalDocumentsRef, presenceConnectedAtMsRef, presenceCursorRef,
    presenceBeatTriggersRef, extensionFetchAbortRef, segmentedDownloadAbortRef, openDocumentSessionsRef,
    failDocumentBackbone, receiveDocumentBackbone, bindDocumentBackbone, loadDocumentPair, captureDialogOrigin,
    isCurrentDialogOrigin, captureEffectOwner, isCurrentEffectOwner, captureProgramEffectOwner, documentArchivePorts,
    closeOwnedDialog, makeOwnedDialog, browserActorUiByRuntimeKeyRef, browserActorUiVersion,
    setBrowserActorUiVersion, browserActorForSessionV1, publishUiRefreshSectionsV1, retireBrowserActorUi,
    directoryScopedOwnersRef, spaceDirectoryHistoriesRef, foldSpaceDirectoryRef, sharedSpaceAccessRef,
    sharedSpaceAccessVersion, adoptSharedSpaceAccess, collectSpaceDirectoryEvents
  };
}
//#endregion FrameworkOsShell
