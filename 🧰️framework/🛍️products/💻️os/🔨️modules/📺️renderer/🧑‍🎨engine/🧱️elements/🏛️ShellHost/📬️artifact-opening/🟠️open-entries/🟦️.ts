/** 🟠️ The open entries phase of useArtifactOpening. */
import { useCallback, useEffect, useMemo, useRef } from "react";
import { type AppRef, type AppRole, AppRouter, type AppRouterManifest, SemioFaultError, SURFACE_FAULT_CODES, type Conflict, type Fault, type MergeReport, type Severity } from "@semio-tech/framework";
import { programPluginIdV1, sessionProgramsV1 } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { shellLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { groupOpenWithEntries } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { isPluginInstanceRetiredV1 } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { liveInstanceWindowFaultV1 } from "../../🩺️fault/🟦️.ts";
import { COMMAND_REJECTED_FAULT_CODE, mutationCodeLabelKey, scheduleShellTransientNoticeDismissV1 } from "../../🎚️shell-model/🟦️.tsx";
import type { ShellCoreAfterDirectoryLaneAndLayout } from "../../📐️layout-hotkeys/🟦️.ts";
import type { useCatalogState } from "../🔶️catalog/🟦️.ts";

/** 🟡 Builds openWithEntries, hasOpenArtifactSurfaces, transientNoticeIdRef, transientNoticeTimerRef of useArtifactOpening. */
export function useOpenEntriesState({ session, loadedPluginsRef, dispatch, pluginSupervisorByIdRef, appRouter, loadedPlugins, pinnedAppFor, pluginLabelById, core, showTransientNoticeRef, applyRemoteMergeRef }: Pick<ReturnType<typeof useCatalogState>, "session" | "loadedPluginsRef" | "dispatch" | "pluginSupervisorByIdRef" | "appRouter" | "loadedPlugins" | "pinnedAppFor" | "pluginLabelById" | "showTransientNoticeRef" | "applyRemoteMergeRef"> & { readonly core: ShellCoreAfterDirectoryLaneAndLayout }) {
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
  return { openWithEntries, hasOpenArtifactSurfaces };
}
