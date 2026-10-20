/** 🔸️ The open artifact app phase of useArtifactOpening. */
import { createAdmittedShellInstanceV1, shellDialogSessionIsCurrentV1 } from "../../🗨️dialog-origin/🟦️.ts";
import { type DocumentOpeningReceiptV1 } from "../../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { runArtifactCreationReadyOpeningV1 } from "../../🌱️artifact-creation/🚪️ready-opening/🟦️.ts";
import { useCallback, useMemo, useRef } from "react";
import { type AppRef, type AppRole, type ArtifactDialect, type Conflict } from "@semio-tech/framework";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { selectOpenConflicts } from "../../../🐚️Shell/🕹️shell-store/🟦️.ts";
import { shellLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { DEFAULT_APP_NONE_VALUE, type ConflictsHostApi, type DefaultAppRow, type DefaultAppsHostApi, encodeDefaultAppValue } from "../../../📌️ChromePanels/📱️settings-defaults-and-conflicts/🟦️.tsx";
import { reduceArtifactCreationProgressUiV1 } from "../../🌱️artifact-creation/🟦️.tsx";
import { mutationCodeLabelKey } from "../../🎚️shell-model/🟦️.tsx";
import { SPACE_INDEX_DOCUMENT_ID, type SpaceArtifactCreationOwnerV1, spaceArtifactCreationReadyOpening } from "../../🧾️space-artifact-creation/🟦️.ts";
import type { PreparedArtifactOpeningTarget } from "../../🔒️identity-document-ports/🧰️identity-port-types/🟦️.ts";
import type { useCatalogState } from "../🔶️catalog/🟦️.ts";

/** 🔹 Builds openArtifactWithAppRef, openReadySpaceArtifactCreation, defaultAppsRows, defaultAppsHost of useArtifactOpening. */
export function useOpenArtifactAppState({ shellStateRef, captureEffectOwner, captureDialogOrigin, isCurrentEffectOwner, loadedPluginsRef, installPlugin, publishPreparedArtifactOpening, loadedPlugins, openArtifactWithAppRefRef, spaceArtifactCreationOwnersRef, openDocumentSessionsRef, setSpaceArtifactCreationUi, resolveArtifactOpeningWithActivationRef, openDocument, closeDocument, retireDocumentAttachment, openReadySpaceArtifactCreationRef, appRouter, knownDialects, pinnedAppFor, pluginLabelById, uiLocale, dispatchSetDefaultApp, dispatchClearDefaultApp, shellState, selectedConflictId, dispatch, dispatchResolveConflict }: Pick<ReturnType<typeof useCatalogState>, "shellStateRef" | "captureEffectOwner" | "captureDialogOrigin" | "isCurrentEffectOwner" | "loadedPluginsRef" | "installPlugin" | "publishPreparedArtifactOpening" | "loadedPlugins" | "openArtifactWithAppRefRef" | "spaceArtifactCreationOwnersRef" | "openDocumentSessionsRef" | "setSpaceArtifactCreationUi" | "resolveArtifactOpeningWithActivationRef" | "openDocument" | "closeDocument" | "retireDocumentAttachment" | "openReadySpaceArtifactCreationRef" | "appRouter" | "knownDialects" | "pinnedAppFor" | "pluginLabelById" | "uiLocale" | "dispatchSetDefaultApp" | "dispatchClearDefaultApp" | "shellState" | "selectedConflictId" | "dispatch" | "dispatchResolveConflict">) {
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
      currentDocumentText: "",
    }),
    [openConflicts, uiLocale, selectedConflictId, conflictKindLabel, conflictMessageText, dispatchResolveConflict],
  );
  const conflictsHostRef = useRef(conflictsHost);
  conflictsHostRef.current = conflictsHost;
  return { openArtifactWithAppRef, openReadySpaceArtifactCreation, defaultAppsHost, defaultAppsHostRef, conflictsHost, conflictsHostRef };
}
