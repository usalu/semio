/** 📬️ The app router, activation owner, default-app and merge-policy dispatch, artifact opening with an app, ready space artifact creation, conflict hosts, transient notices and remote merge.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useArtifactOpening` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { createAdmittedShellInstanceV1, shellDialogSessionIsCurrentV1 } from "../🗨️dialog-origin/🟦️.ts";
import { type DocumentOpeningReceiptV1 } from "../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { runArtifactCreationReadyOpeningV1 } from "../🌱️artifact-creation/🚪️ready-opening/🟦️.ts";
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
  AppRouter,
  type AppRouterManifest,
  type ArtifactDialect,
  artifactKindActivationOwner,
  dialectCoordinate,
  parseDialectCoordinate,
  parseSurfaceAppId,
  EMPTY_OPENING_PREFERENCES,
  foldOpeningPreferences,
  type OpeningConfigMutation,
  type OpeningPreferences,
  SemioFaultError,
  SURFACE_FAULT_CODES,
  type Conflict,
  type ConflictResolution,
  type Fault,
  type MergePolicy,
  type MergeReport,
  type Severity
} from "@semio-tech/framework";
import { resolveArtifactOpeningRelay, type ResolvedArtifactOpeningRelay } from "@semio-tech/framework-os";
import {
  type ActiveSession,
  selectOpenConflicts,
  localProgramsV1,
  programPluginIdV1,
  sessionProgramsV1
} from "../../🐚️Shell/🟦️.tsx";
import { EMPTY_APP_LABELS_OVERLAY, applyFrameworkLayoutSeed, groupOpenWithEntries, shellLabel } from "../../🛠️ShellHelpers/🟦️.tsx";
import {
  DEFAULT_APP_NONE_VALUE,
  type ConflictsHostApi,
  type DefaultAppRow,
  type DefaultAppsHostApi,
  encodeDefaultAppValue
} from "../../📌️ChromePanels/🟦️.tsx";
// #endregion 🔌️Adapters
import { isPluginInstanceRetiredV1, type PluginWasmHandle } from "../../🔌️PluginRuntime/🟦️.tsx";
import { liveInstanceWindowFaultV1 } from "../🩺️fault/🟦️.ts";
import { parseHubProgramIdV1 } from "../../../../../🔌️plugin/📇️registry/🌎️hub-source/🔍️resolution/🟦️.ts";
import { reduceArtifactCreationProgressUiV1 } from "../🌱️artifact-creation/🟦️.tsx";
import { COMMAND_REJECTED_FAULT_CODE, mutationCodeLabelKey, scheduleShellTransientNoticeDismissV1, seededActiveWindowId } from "../🎚️shell-model/🟦️.tsx";
import { canonicalSurfaceId } from "../🔐️identity-binding/🟦️.ts";
import { SPACE_INDEX_DOCUMENT_ID, type SpaceArtifactCreationOwnerV1, spaceArtifactCreationReadyOpening } from "../🧾️space-artifact-creation/🟦️.ts";
import { withLocalizedWindowKindLabels } from "../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterDirectoryLaneAndLayout } from "../📐️layout-hotkeys/🟦️.ts";
import type { PreparedArtifactOpeningTarget } from "../🔒️identity-document-ports/🟦️.ts";

//#region FrameworkOsShell
/** 👁️✏️ `PluginRuntime`'s `PluginWasmHandle` wraps the raw `exchange` ABI behind typed methods —
 * `transactionPrepare`/`transactionCommit`/`transactionUndo`/`transactionRedo` and (as of ticket
 * `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` lane K2)
 * `setMergePolicy`/`resolveConflict`/`readConflicts` are wrapped this way (`adaptPluginHandle`,
 * `PluginRuntime/🟦️.tsx`), each internally riding its own `AppChannelClient` — see
 * {@link pluginHandleFor} below. `openArtifact`/`setDefaultApp`/`clearDefaultApp` (contract freeze
 * §3) are NOT wrapped yet — the raw `exchange` method itself isn't re-exposed for them, so this
 * lease cannot construct its own `AppChannelClient` from the handle it's given. Feature-detected
 * here (structurally optional, not a type-lying cast) so this activates the moment `PluginRuntime`
 * adds the same three methods the transaction/merge families already have, with zero changes on
 * this side. See `📓️w1-c-report.md` "NOT done" — until then these three are local-only (the
 * opening-prefs fold below and the session switch in `openArtifactWithAppRef` both still work;
 * only the wire notify is inert). */
type PendingAppChannelMethods = {
  readonly openArtifact?: (artifactRef: string, role: number, pluginId?: string, appId?: string) => Promise<unknown>;
  readonly setDefaultApp?: (artifactKind: string, standard: string, subset: string, role: number, pluginId: string, appId: string) => Promise<unknown>;
  readonly clearDefaultApp?: (artifactKind: string, standard: string, subset: string, role: number) => Promise<unknown>;
};

export type ShellCoreAfterArtifactOpening = ShellCoreAfterDirectoryLaneAndLayout & ReturnType<typeof useArtifactOpening>;

export function useArtifactOpening(core: ShellCoreAfterDirectoryLaneAndLayout) {
  const {
    catalog, shellState, dispatch, loadedPlugins, session, localPrograms, uiLocale, uiTerminology, selectedConflictId,
    extraWindowCounterRef, extraWindowInstancesRef, applyRemoteMergeRef, showTransientNoticeRef,
    retireDocumentAttachment, openArtifactWithAppRefRef, resolveArtifactOpeningRelayRef,
    resolveArtifactOpeningWithActivationRef, openReadySpaceArtifactCreationRef, spaceArtifactCreationOwnersRef,
    setSpaceArtifactCreationUi, cancelSpaceArtifactCreationsForRuntime, openDocumentSessionsRef, captureDialogOrigin,
    captureEffectOwner, isCurrentEffectOwner, loadedPluginsRef, pluginSupervisorByIdRef, registry, hubPluginSource,
    installPlugin, installHubDocumentProgram, shellStateRef, pendingDocumentOpeningPublicationRef, openDocument,
    closeDocument
  } = core;
  //#region 🔖️SurfaceRoles
  /** 👁️✏️ `(dialect, role) -> AppRef[]`, contract freeze §3 — built fresh from every loaded plugin's
   * manifest. `AppRouter.build` is total: a plugin with a genuine authoring defect
   * (`surface.conflict`/`surface.contribution-not-permitted`) is excluded on its own and reported
   * through {@link AppRouter.pluginFaults}, so one bad manifest costs that plugin's surfaces, never
   * every route in the session (ticket 26/09/05/S-END-TO-END lane H). */
  const appRouter = useMemo(
    (): AppRouter =>
      AppRouter.build(
        localPrograms.map((entry): AppRouterManifest => ({
          pluginId: entry.handle.pluginId,
          apps: entry.manifest.apps as unknown as Record<string, unknown>[],
          artifactKinds: entry.manifest.artifactKinds,
          dependencies: entry.manifest.dependencies,
        })),
      ),
    [localPrograms, registry],
  );
  /** 🧯️ `pluginId -> the surface fault that excluded it from {@link appRouter}` — merged onto the
   * plugin's own status so an excluded plugin is visible instead of silently unroutable. */
  const routerFaultByPluginId = useMemo(() => new Map(appRouter.pluginFaults().map((fault) => [fault.scope.pluginId ?? "", fault] as const)), [appRouter]);
  const pluginLabelById = useMemo(() => new Map(localPrograms.map((entry) => [entry.handle.pluginId, entry.manifest.label || entry.handle.pluginId])), [localPrograms]);
  /** 👁️✏️ Every dialect any loaded app declares — read straight off `AppDefinition.dialect`
   * (contract freeze §1), never inferred from a surface id string. Feeds the Settings
   * `SettingsDefaultApps` table; `appRouter` itself has no public "every registered dialect"
   * accessor (by design — it's addressed one `(dialect, role)` pair at a time). */
  const knownDialects = useMemo((): readonly ArtifactDialect[] => {
    const byCoordinate = new Map<string, ArtifactDialect>();
    for (const entry of localPrograms) for (const app of entry.manifest.apps) if (app.dialect) byCoordinate.set(dialectCoordinate(app.dialect), app.dialect);
    return [...byCoordinate.values()];
  }, [cancelSpaceArtifactCreationsForRuntime, localPrograms]);

  /** 🎚️ Client-side fold of `os.config.opening` (contract freeze §4) — event-sourced the same way
   * the host materializes it (`foldOpeningPreferences`), never a mutated map. There is no host
   * readback call on {@link AppChannelClient} yet (only the two write commands), so this mirror is
   * advanced ONLY by this shell's own `setDefaultApp`/`clearDefaultApp` calls below — a pin made by
   * another session/shell is not reflected here until that gap closes. See `📓️w1-c-report.md`. */
  const [openingPreferences, setOpeningPreferences] = useState<OpeningPreferences>(EMPTY_OPENING_PREFERENCES);
  const pinnedAppFor = useCallback((dialect: ArtifactDialect, role: AppRole): AppRef | undefined => openingPreferences.defaults.find((entry) => dialectCoordinate(entry.dialect) === dialectCoordinate(dialect) && entry.role === role)?.app, [openingPreferences]);
  resolveArtifactOpeningRelayRef.current = (actionId, args) => (appRouter ? resolveArtifactOpeningRelay(actionId, args, appRouter, openingPreferences) : null);

  /**
   * 🎬️ The hub half of `os.open-artifact`: a session that has never loaded the kind's owner has no
   * router entry for it, so the sync relay throws `surface.unknown-dialect` — not because the artifact
   * is unopenable, but because nobody who could open it is resident. The owner comes from the catalog's
   * declared `on-artifact-kind:` rows ({@link artifactKindActivationOwner}), which exist without any
   * manifest; this installs it, rebuilds the router over the freshly loaded set (the `appRouter` memo
   * this closure captured is a render behind `installPlugin`'s dispatch) and resolves once more. One
   * install, one retry: a second miss is a real routing gap and surfaces as the original fault.
   */
  const installActivationOwnerAndResolve = useCallback(async (actionId: string, args: unknown): Promise<ResolvedArtifactOpeningRelay | null> => {
    const record = args && typeof args === "object" && !Array.isArray(args) ? (args as Readonly<Record<string, unknown>>) : undefined;
    const artifactRef = typeof record?.artifactRef === "string" ? record.artifactRef : "";
    const openedArtifactKind = (): string => (artifactRef.includes("#") ? parseSurfaceAppId(artifactRef).dialect : parseDialectCoordinate(artifactRef)).artifactKind;
    if (hubPluginSource !== null && record !== undefined && typeof record.spaceId === "string" && record.spaceId.length > 0) {
      // 🌎️ A hub document runs on the module of the catalog generation that serves it, never on whatever this device
      // staged: its program is installed from the hub catalog and it alone routes the opening.
      let artifactKind: string;
      try {
        artifactKind = openedArtifactKind();
      } catch {
        throw new Error("opening.invalid-artifact-ref");
      }
      const program = await installHubDocumentProgram(artifactKind);
      if (program === null || program.catalogModule === undefined) throw new Error(`opening.hub-program-unavailable: ${artifactKind}`);
      const catalogPluginId = program.catalogModule.pluginId;
      const programId = program.handle.pluginId;
      const routed = record.pluginId === catalogPluginId ? { ...record, pluginId: programId } : record;
      const router = AppRouter.build([{ pluginId: programId, apps: program.manifest.apps as unknown as Record<string, unknown>[], artifactKinds: program.manifest.artifactKinds, dependencies: program.manifest.dependencies }]);
      const preferences: OpeningPreferences = { ...openingPreferences, defaults: openingPreferences.defaults.map((entry) => (entry.app.pluginId === catalogPluginId ? { ...entry, app: { ...entry.app, pluginId: programId } } : entry)) };
      return resolveArtifactOpeningRelay(actionId, routed, router, preferences);
    }
    const resolveNow = resolveArtifactOpeningRelayRef.current;
    try {
      return resolveNow(actionId, args);
    } catch (relayError) {
      let artifactKind = "";
      try {
        artifactKind = openedArtifactKind();
      } catch {
        throw relayError;
      }
      const owner = artifactKindActivationOwner(catalog, artifactKind) ?? (await hubPluginSource?.ownerOfDialect(artifactKind).catch(() => undefined));
      if (owner === undefined || loadedPluginsRef.current.some((entry) => entry.handle.pluginId === owner)) throw relayError;
      const outcome = await installPlugin(owner);
      if (outcome !== "loaded" && outcome !== "already-loaded") throw relayError;
      const installed = AppRouter.build(localProgramsV1(loadedPluginsRef.current).map((entry): AppRouterManifest => ({
        pluginId: entry.handle.pluginId,
        apps: entry.manifest.apps as unknown as Record<string, unknown>[],
        artifactKinds: entry.manifest.artifactKinds,
        dependencies: entry.manifest.dependencies,
      })));
      return resolveArtifactOpeningRelay(actionId, args, installed, openingPreferences);
    }
  }, [catalog, installPlugin, installHubDocumentProgram, openingPreferences, hubPluginSource]);
  resolveArtifactOpeningWithActivationRef.current = installActivationOwnerAndResolve;

  
  const pendingAppChannelFor = useCallback(
    (pluginId: string): PendingAppChannelMethods | undefined => loadedPlugins.find((entry) => entry.handle.pluginId === pluginId)?.handle as PendingAppChannelMethods | undefined,
    [loadedPlugins],
  );

  /** ⚖️ Finds a loaded plugin's real `PluginWasmHandle` by pluginId, for the merge-policy/conflict
   * pass-throughs below — unlike {@link pendingAppChannelFor}'s ad hoc cast, `setMergePolicy`/
   * `resolveConflict`/`readConflicts` are genuine, always-present `PluginWasmHandle` members now
   * (`adaptPluginHandle`, `PluginRuntime/🟦️.tsx`), so no feature-detection is needed. */
  const pluginHandleFor = useCallback((pluginId: string): PluginWasmHandle | undefined => loadedPlugins.find((entry) => entry.handle.pluginId === pluginId)?.handle, [loadedPlugins]);

  const dispatchSetDefaultApp = useCallback(
    (dialect: ArtifactDialect, role: AppRole, program: AppRef) => {
      const app: AppRef = { ...program, pluginId: parseHubProgramIdV1(program.pluginId)?.pluginId ?? program.pluginId };
      const mutation: OpeningConfigMutation = { mutation: "setDefaultApp", dialect, role, app };
      setOpeningPreferences((current) => foldOpeningPreferences([mutation], current));
      const roleNum = role === "editor" ? 1 : 0;
      void pendingAppChannelFor(program.pluginId)
        ?.setDefaultApp?.(dialect.artifactKind, dialect.standard, dialect.subset, roleNum, app.pluginId, app.appId)
        ?.catch((commandError) => undefined);
    },
    [pendingAppChannelFor],
  );
  const dispatchClearDefaultApp = useCallback(
    (dialect: ArtifactDialect, role: AppRole) => {
      const mutation: OpeningConfigMutation = { mutation: "clearDefaultApp", dialect, role };
      setOpeningPreferences((current) => foldOpeningPreferences([mutation], current));
      const roleNum = role === "editor" ? 1 : 0;
      void pendingAppChannelFor(session?.pluginId ?? "")
        ?.clearDefaultApp?.(dialect.artifactKind, dialect.standard, dialect.subset, roleNum)
        ?.catch((commandError) => undefined);
    },
    [pendingAppChannelFor, session?.pluginId],
  );

  /** ⚖️ Persists `os.config.merge-policy` through the `🛡️change-merge-policy` config triad's own
   * event-sourced fold (local, always works), then genuinely forwards `AppCommand::SetMergePolicy`
   * (contract freeze §C8) through `PluginWasmHandle.setMergePolicy` — a real handle member now (see
   * {@link pluginHandleFor}'s doc), so a session-less/plugin-not-loaded guard is the only thing
   * standing between this and the wire call; a failed dispatch surfaces loudly via the `.catch`
   * (never a silent optional-chain no-op). */
  const dispatchSetMergePolicy = useCallback(
    (policy: MergePolicy) => {
      dispatch({ type: "SET_MERGE_POLICY", value: policy });
      if (!session) return;
      const plugin = pluginHandleFor(session.pluginId);
      if (!plugin) return;
      void plugin.setMergePolicy(session.instanceId, policy).catch((commandError) => undefined);
    },
    [pluginHandleFor, session],
  );

  /** ⚖️ `📌️ChromePanels`' Conflicts panel Accept/Discard — forwards `AppCommand::ResolveConflict`
   * (contract freeze §C6/§C8/§C9) through `PluginWasmHandle.resolveConflict`; the local roster is
   * replaced from the `Conflicts` frame the reply itself batches (contract freeze §C6
   * `resolve_conflict`: "Returns the authoritative `MergeReport` + `Conflicts` frames"), never
   * optimistically, since `resolve_conflict` can itself reject (Quarantined+Accept still enforces
   * Fatal — that path returns no `Conflicts` frame, so the roster is left untouched). */
  const dispatchResolveConflict = useCallback(
    (conflictId: string, resolution: ConflictResolution) => {
      if (!session) return;
      const plugin = pluginHandleFor(session.pluginId);
      if (!plugin) return;
      void plugin
        .resolveConflict(session.instanceId, conflictId, resolution)
        .then((result) => {
          if (result.conflicts) dispatch({ type: "SET_CONFLICTS", value: result.conflicts });
        })
        .catch((commandError) => undefined);
    },
    [pluginHandleFor, session],
  );

  const publishPreparedArtifactOpening = useCallback((target: PreparedArtifactOpeningTarget, receipt?: DocumentOpeningReceiptV1): void => {
    const { session: nextSession, dialect, role } = target;
    const seeded = applyFrameworkLayoutSeed(nextSession.app.defaultLayout, withLocalizedWindowKindLabels(nextSession.app.windowKinds), EMPTY_APP_LABELS_OVERLAY, uiTerminology, uiLocale);
    extraWindowInstancesRef.current = seeded.extraInstances;
    extraWindowCounterRef.current = seeded.extraInstances.length;
    pendingDocumentOpeningPublicationRef.current = receipt === undefined ? null : { receipt, plugin: target.plugin, session: nextSession };
    dispatch({ type: "SET_SESSION", value: nextSession });
    dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
    dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
    dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: seededActiveWindowId(seeded.modeLayout) });
    try {
      void (target.plugin as PendingAppChannelMethods).openArtifact?.(canonicalSurfaceId(dialect, role), role === "editor" ? 1 : 0, nextSession.pluginId, nextSession.app.id)
        ?.catch((commandError) => undefined);
    } catch (commandError) {
    }
  }, [uiLocale, uiTerminology]);

  /** 👁️✏️ Re-points the primary session at a different registered `AppRef` for the SAME artifact
   * (contract freeze §3/§5's "Open with…") — installs the target plugin first if it isn't loaded
   * yet, then mirrors `establishPrimarySession`'s non-studio create/seed/dispatch sequence. Also
   * best-effort notifies the host once `openArtifact` is wrapped (see `PendingAppChannelMethods`
   * above) using the canonical role-suffixed surface id. */
  const openArtifactWithAppRef = useCallback(
    async (target: AppRef, dialect: ArtifactDialect, role: AppRole, admit?: () => boolean, publish = true): Promise<PreparedArtifactOpeningTarget | null> => {
      const current = shellStateRef.current.pluginRuntime.session;
      const owner = current === null ? null : captureEffectOwner(current, captureDialogOrigin(current));
      const canOpen = admit ?? (() => owner === null ? shellStateRef.current.pluginRuntime.session === null : isCurrentEffectOwner(owner));
      if (!canOpen()) return null;
      let plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === target.pluginId);
      if (!plugin) {
        const outcome = await installPlugin(target.pluginId);
        if (outcome !== "loaded" && outcome !== "already-loaded") return null;
        plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === target.pluginId);
      }
      if (!canOpen()) return null;
      const app = plugin?.manifest.apps.find((candidate) => candidate.id === target.appId);
      if (!plugin || !app) {
        return null;
      }
      const handle = plugin.handle;
      const instanceId = await createAdmittedShellInstanceV1(canOpen, () => handle.createApp(app.id), (id) => handle.destroyApp(id));
      if (instanceId === null) return null;
      const nextSession: ActiveSession = { pluginId: plugin.handle.pluginId, instanceId, app, viewState: { activeModeId: app.defaultModeId ?? app.modes[0]?.id } };
      const prepared = { session: nextSession, plugin: plugin.handle, dialect, role };
      if (publish) publishPreparedArtifactOpening(prepared);
      return prepared;
    },
    [loadedPlugins, installPlugin, captureDialogOrigin, captureEffectOwner, isCurrentEffectOwner, publishPreparedArtifactOpening],
  );
  openArtifactWithAppRefRef.current = openArtifactWithAppRef;

  const openReadySpaceArtifactCreation = useCallback((requestId: string): void => {
    const owner = spaceArtifactCreationOwnersRef.current.get(requestId);
    if (owner === undefined || owner.ready === null || owner.opening) return;
    const openingArgs = spaceArtifactCreationReadyOpening(owner.ready);
    const origin = openDocumentSessionsRef.current.get(owner.runtimeKey);
    if (openingArgs === null || origin === undefined) return;
    const openingOwner: SpaceArtifactCreationOwnerV1 = { ...owner, opening: true };
    const current = (): boolean => {
      const retained = spaceArtifactCreationOwnersRef.current.get(openingOwner.requestId);
      const entry = openDocumentSessionsRef.current.get(openingOwner.runtimeKey);
      return retained === openingOwner && entry?.clientInstanceId === openingOwner.clientInstanceId
        && entry.session.instanceId === openingOwner.sessionInstanceId
        && shellDialogSessionIsCurrentV1(entry.session, shellStateRef.current.pluginRuntime.session)
        && entry.scope?.spaceId === openingOwner.spaceId && entry.scope.documentId === SPACE_INDEX_DOCUMENT_ID;
    };
    spaceArtifactCreationOwnersRef.current.set(requestId, openingOwner);
    setSpaceArtifactCreationUi((state) => reduceArtifactCreationProgressUiV1(state, { kind: "opening", requestId, spaceId: owner.spaceId }));
    void runArtifactCreationReadyOpeningV1<PreparedArtifactOpeningTarget, DocumentOpeningReceiptV1>({
      current,
      prepare: async () => {
        const opening = await resolveArtifactOpeningWithActivationRef.current("os.open-artifact", openingArgs);
        if (opening === null) return null;
        const target = await openArtifactWithAppRef(opening.app, opening.dialect, opening.role, current, false);
        return target === null ? null : { ...target, expectedCatalogGenerationId: openingOwner.expectedCatalogGenerationId };
      },
      open: (target) => openDocument(
        { documentId: openingArgs.artifactId!, schema: openingArgs.schema!, spaceId: openingOwner.spaceId },
        undefined,
        target,
      ),
      publish: (target, receipt) => publishPreparedArtifactOpening(target, receipt),
      release: async (target, receipt) => {
        try {
          if (receipt !== null) {
            const entry = openDocumentSessionsRef.current.get(receipt.runtimeKey);
            if (entry?.clientInstanceId === receipt.clientInstanceId && entry.plugin === target.plugin && shellDialogSessionIsCurrentV1(entry.session, target.session)) {
              closeDocument(receipt.runtimeKey, receipt.clientInstanceId);
            }
            await retireDocumentAttachment(target.plugin, target.session.instanceId, receipt.clientInstanceId);
          }
        } finally {
          await target.plugin.destroyApp(target.session.instanceId);
        }
      },
      failed: (openingError) => {
        if (spaceArtifactCreationOwnersRef.current.get(requestId) !== openingOwner) return;
        spaceArtifactCreationOwnersRef.current.set(requestId, { ...openingOwner, opening: false });
        setSpaceArtifactCreationUi((state) => reduceArtifactCreationProgressUiV1(state, { kind: "open-failed", requestId, spaceId: openingOwner.spaceId }));
        console.warn("[os-shell] created artifact remains ready after opening failed", openingError);
      },
    }).then((outcome) => {
      if (outcome === "failed" || spaceArtifactCreationOwnersRef.current.get(requestId) !== openingOwner) return;
      spaceArtifactCreationOwnersRef.current.delete(requestId);
      setSpaceArtifactCreationUi((state) => reduceArtifactCreationProgressUiV1(state, { kind: "cleared", requestId }));
    });
  }, [closeDocument, openArtifactWithAppRef, openDocument, publishPreparedArtifactOpening, retireDocumentAttachment]);
  openReadySpaceArtifactCreationRef.current = openReadySpaceArtifactCreation;

  /** 👁️✏️ `DefaultAppsHostApi.rows` — one row per `(dialect, role)` pair among `knownDialects`, both
   * roles even when only one has registered surfaces (an empty `options` list still renders the row,
   * matching the Settings table's "dialect × {viewer, editor}" framing in contract freeze §5). */
  const defaultAppsRows = useMemo((): readonly DefaultAppRow[] => {
    if (!appRouter) return [];
    const rows: DefaultAppRow[] = [];
    for (const dialect of knownDialects) {
      for (const role of ["viewer", "editor"] as const) {
        const entries = appRouter.entriesFor(dialect, role);
        const pinned = pinnedAppFor(dialect, role);
        rows.push({
          dialect,
          role,
          options: entries.map((app) => ({ value: encodeDefaultAppValue(app), label: pluginLabelById.get(app.pluginId) ?? app.pluginId })),
          value: pinned ? encodeDefaultAppValue(pinned) : DEFAULT_APP_NONE_VALUE,
        });
      }
    }
    return rows;
  }, [appRouter, knownDialects, pinnedAppFor, pluginLabelById]);

  const defaultAppsHost: DefaultAppsHostApi = useMemo(
    () => ({ rows: defaultAppsRows, locale: uiLocale, setDefault: dispatchSetDefaultApp, clearDefault: dispatchClearDefaultApp }),
    [defaultAppsRows, uiLocale, dispatchSetDefaultApp, dispatchClearDefaultApp],
  );
  const defaultAppsHostRef = useRef(defaultAppsHost);
  defaultAppsHostRef.current = defaultAppsHost;

  /** ⚖️ `📌️ChromePanels`' Conflicts settings tab — `Shell`'s `selectOpenConflicts` selector feeds the
   * roster, `kindLabel`/`messageText` localize a `Conflict`'s own `ConflictKind`/first `MutationMessage`
   * (never parsed from English prose). */
  const openConflicts = useMemo(() => selectOpenConflicts(shellState), [shellState]);
  const conflictKindLabel = useCallback(
    (conflict: Conflict): string => shellLabel(conflict.kind.kind === "quarantined" ? "ui.conflict.quarantined" : "ui.conflict.degraded"),
    [],
  );
  const conflictMessageText = useCallback((conflict: Conflict): string => {
    const worst = conflict.messages[0];
    if (!worst) return "";
    return `${shellLabel(mutationCodeLabelKey(worst.code))} — ${worst.message}`;
  }, []);
  const conflictsHost: ConflictsHostApi = useMemo(
    () => ({
      conflicts: openConflicts,
      locale: uiLocale,
      selectedConflictId,
      kindLabel: conflictKindLabel,
      messageText: conflictMessageText,
      onSelect: (conflictId) => dispatch({ type: "SET_SELECTED_CONFLICT_ID", value: conflictId }),
      onResolve: dispatchResolveConflict,
      // 🐢️ No synchronous "current document as JSON" accessor is in this lease's reach yet (the
      // decoded artifact lives inside the plugin wasm instance, not mirrored into `ShellState`) — an
      // empty string is an honest "no local snapshot", never a fabricated diff side.
      currentDocumentText: "",
    }),
    [openConflicts, uiLocale, selectedConflictId, conflictKindLabel, conflictMessageText, dispatchResolveConflict],
  );
  const conflictsHostRef = useRef(conflictsHost);
  conflictsHostRef.current = conflictsHost;

  /** 📖️ Seeds the Conflicts panel with this session's authoritative roster on session start/switch
   * (`AppCommand::ReadConflicts`, contract freeze §C8/§C9) — otherwise the panel would only ever
   * show conflicts a later `setMergePolicy`/`resolveConflict` reply happened to carry, staying empty
   * across a reload even when the guest already holds `Open` conflicts. Reads
   * `loadedPluginsRef.current` rather than depending on `loadedPlugins` so a plugin hot-swap that
   * leaves the session's pluginId in place doesn't re-fire this on every unrelated roster change. */
  useEffect(() => {
    if (!session) return;
    const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    let cancelled = false;
    void plugin
      .readConflicts(session.instanceId)
      .then((conflicts) => {
        if (cancelled) return;
        dispatch({ type: "SET_CONFLICTS", value: conflicts });
        dispatch({ type: "SET_INSTANCE_FAULT", value: null });
      })
      .catch((commandError) => {
        const fault = liveInstanceWindowFaultV1(commandError, pluginSupervisorByIdRef.current[session.pluginId], isPluginInstanceRetiredV1);
        if (!cancelled && fault !== null) dispatch({ type: "SET_INSTANCE_FAULT", value: fault });
      });
    return () => {
      cancelled = true;
    };
  }, [session?.instanceId, session?.pluginId]);

  /** 👁️✏️ "Open with…" entries for the CURRENT session's own dialect, grouped by role — the Document
   * panel section, and what the context-menu/palette entries focus. `undefined` with no session or
   * router (nothing to list yet), and for a non-surface app — one bound to no subset, such as the
   * workflow studio — which has no dialect to open anything else against. */
  /** 👁️✏️ A hub document's session routes "Open with…" over its own catalog closure (program ids), pins stay keyed by plugin id. */
  const openWithEntries = useMemo(() => {
    if (!session || !appRouter || !session.app.dialect) return undefined;
    const catalogModule = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.catalogModule;
    if (catalogModule === undefined) return groupOpenWithEntries(appRouter, session.app.dialect, { pluginId: session.pluginId, appId: session.app.id }, (role) => pinnedAppFor(session.app.dialect, role), pluginLabelById);
    const programs = sessionProgramsV1(loadedPlugins, session.pluginId);
    const router = AppRouter.build(programs.map((entry): AppRouterManifest => ({ pluginId: entry.handle.pluginId, apps: entry.manifest.apps as unknown as Record<string, unknown>[], artifactKinds: entry.manifest.artifactKinds, dependencies: entry.manifest.dependencies })));
    const labels = new Map(programs.map((entry) => [entry.handle.pluginId, entry.manifest.label || programPluginIdV1(entry)]));
    const pinned = (role: AppRole): AppRef | undefined => {
      const pin = pinnedAppFor(session.app.dialect, role);
      const peer = pin === undefined ? undefined : catalogModule.peers[pin.pluginId];
      return pin === undefined || peer === undefined ? pin : { ...pin, pluginId: peer };
    };
    return groupOpenWithEntries(router, session.app.dialect, { pluginId: session.pluginId, appId: session.app.id }, pinned, labels);
  }, [session, appRouter, loadedPlugins, pinnedAppFor, pluginLabelById]);
  const hasOpenArtifactSurfaces = (openWithEntries?.viewer.length ?? 0) + (openWithEntries?.editor.length ?? 0) > 0;

  const transientNoticeIdRef = useRef(0);
  const transientNoticeTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  /** 🧯️ Shows a non-blocking, auto-dismissing notice (contract freeze §2.3/§5) — replaces whatever
   * notice is currently showing rather than queuing, since only one can render at a time. */
  const showTransientNotice = useCallback(
    (message: string, kind: Severity = "info", code?: string) => {
      if (transientNoticeTimerRef.current) clearTimeout(transientNoticeTimerRef.current);
      transientNoticeIdRef.current += 1;
      const id = transientNoticeIdRef.current;
      dispatch({ type: "SET_TRANSIENT_NOTICE", value: { id, message, kind, code } });
      transientNoticeTimerRef.current = scheduleShellTransientNoticeDismissV1(() => dispatch({ type: "SET_TRANSIENT_NOTICE", value: null }));
    },
    [dispatch],
  );
  core.showTransientNotice = showTransientNotice;
  showTransientNoticeRef.current = showTransientNotice;
  /** 🧯️ `true` for a `SemioFaultError` carrying `"viewer.read-only"` — the one host-raised fault this
   * lease knows to render as a notice instead of letting it crash into `ShellFaultBoundary`. */
  const isViewerReadOnlyFault = useCallback((error: unknown): boolean => error instanceof SemioFaultError && error.fault.code === SURFACE_FAULT_CODES.ViewerReadOnly, []);
  core.isViewerReadOnlyFault = isViewerReadOnlyFault;
  /** ⚖️ `true` for a `SemioFaultError` carrying `"app.command.rejected"` — one LOCAL dispatch's
   * `store.dispatch` was rejected by this authority's `MergePolicy` (contract freeze `26/08/16/
   * MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C8/§C9: `Fault.code ==
   * "app.command.rejected"`, `Fault.severity` mirrors the rejected `DispatchReport.worst`). */
  const isMutationRejectedFault = useCallback((error: unknown): boolean => error instanceof SemioFaultError && error.fault.code === COMMAND_REJECTED_FAULT_CODE, []);
  core.isMutationRejectedFault = isMutationRejectedFault;
  /** ⚖️ One toast per gesture for a rejected local dispatch — worst level already IS `fault.severity`
   * (that field mirrors `DispatchReport.worst`), body = the first cause's localized `ui.mutation.
   * code.*` label + its English prose, falling back to `ui.mutation.rejected.body` when the fault
   * carries no `causes` yet (the guest-side wiring that populates them is a different lane). */
  const showMutationRejectedNotice = useCallback(
    (fault: Fault) => {
      const cause = fault.causes?.[0];
      const codeLabel = cause?.code ? shellLabel(mutationCodeLabelKey(cause.code)) : shellLabel("ui.mutation.rejected.title");
      const body = cause ? `${codeLabel} — ${cause.message}` : shellLabel("ui.mutation.rejected.body");
      showTransientNotice(`${shellLabel("ui.mutation.rejected.title")}: ${body}`, fault.severity, COMMAND_REJECTED_FAULT_CODE);
    },
    [showTransientNotice],
  );
  core.showMutationRejectedNotice = showMutationRejectedNotice;
  /** ⚖️ Remote-origin merge-outcome bridge (contract freeze §C6/§C9) — fed by `applyRemoteMergeRef`
   * (see its declaration doc) from `ensureBackboneWorker`'s `remoteMutations` handling. A non-null
   * `conflicts` roster replaces the Conflicts panel's roster exactly like `dispatchResolveConflict`'s
   * reply does — `📌️ChromePanels`' panel and `ShellSync`'s quarantine badge both derive from `state.
   * merge.conflicts` (`selectOpenConflicts`/`selectQuarantinedConflicts`), so a REMOTE quarantined
   * conflict lands there with zero further wiring. A `"degraded"` outcome additionally gets THIS
   * authority's only "surfaces without being asked" channel — the same transient-notice convention
   * {@link showMutationRejectedNotice} uses for a LOCAL rejected dispatch — since a degraded merge
   * already applied silently and has no other passive indicator (unlike quarantine's badge). */
  const applyRemoteMerge = useCallback(
    (conflicts: readonly Conflict[] | null, mergeReport: MergeReport | null) => {
      if (conflicts) dispatch({ type: "SET_CONFLICTS", value: conflicts });
      if (!mergeReport?.worst || !mergeReport.conflict) return;
      const flagged = (conflicts ?? []).find((conflict) => conflict.id === mergeReport.conflict);
      if (flagged?.kind.kind !== "degraded") return;
      const worst = flagged.messages[0];
      const kindLabel = shellLabel("ui.conflict.degraded");
      const body = worst ? `${shellLabel(mutationCodeLabelKey(worst.code))} — ${worst.message}` : undefined;
      showTransientNotice(body ? `${kindLabel}: ${body}` : kindLabel, mergeReport.worst);
    },
    [showTransientNotice],
  );
  applyRemoteMergeRef.current = applyRemoteMerge;
  //#endregion 🔖️SurfaceRoles

  return {
    appRouter, routerFaultByPluginId, dispatchSetDefaultApp, dispatchClearDefaultApp, dispatchSetMergePolicy,
    openArtifactWithAppRef, openReadySpaceArtifactCreation, defaultAppsHost, defaultAppsHostRef, conflictsHost,
    conflictsHostRef, openWithEntries, hasOpenArtifactSurfaces
  };
}
//#endregion FrameworkOsShell
