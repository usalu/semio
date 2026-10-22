/** 🪟️ The panel active paths, the history tab and override tab reveals, the time travel reveal and the tool tab selection effects. */
import { useEffect, useMemo, useRef } from "../../../⚛️react-port/🟦️.ts";
import { FRAMEWORK_PANEL_TAB_HISTORY_ID, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_TOOL_RUN_ID, SET_ACTIVE_TOOL_ACTION_ID } from "@semio-tech/framework";
import { spawnedIdOfWindowInstanceV1 } from "../../🪟️spawned-program/🟦️.ts";
import { type Anchor, ANCHORS, findPanelTabInDock, findPanelTabPath, panelTabChildren, reconcileActivePath } from "@semio-tech/ui-react";
import { leftoverWorldSelectionOverlayV1 } from "../../../🌐️World3dHost/🔗️instance-selection/🟦️.ts";
import { FRAMEWORK_CATEGORY_DISPLAY_ID, FRAMEWORK_CATEGORY_TOOL_ID } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { flattenPanelTabLeaves } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { programArmedToolRevealV1, reconcileToolTabSelection, toolIdFromPanelTabId, type ToolTabSelection } from "../../../🛠️ShellHelpers/🎛️command-registry/🟦️.tsx";
import { toolRunPanelReveal } from "../../../🛠️ShellHelpers/⏯️tool-run-panel/🟦️.ts";
import { useTimeTravelRevealV1 } from "../../../🛠️ShellHelpers/⏪️time-travel/🟦️.tsx";
import { FRAMEWORK_SETTINGS_PANEL_ID } from "../../../📌️ChromePanels/🆔️panel-ids/🟦️.ts";
import { leftoverInspectionRefreshScope, leftoverInspectionPanelHash } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { shellAppFault, type ShellAppFrameV1 } from "@semio-tech/framework/mcp/channel";
import type { ShellCommand } from "@semio-tech/framework/shell";
import type { useDockModel } from "../🧱️dock-model-and-persistence/🟦️.ts";
import type { useOverrideAnchorsAndIntroductionIds } from "../🎓️override-anchors-and-introduction/🟦️.ts";

/**
 * 🗺 Derives the panel active paths and reveals the history tab, studio override and tool run panels.
 *
 * 🤖️ Fills {@link applyAgentShellCommandRef}: the MCP gateway's `ui_reveal`/`ui_focus` reach this
 * host as SSOT `ShellCommand`s, and this is where they stop being a mirror update and become the
 * real chrome moving. `setPanelPath` carries the shell SSOT's four-anchor vocabulary while this
 * dock has six, so the TAB is the address — `findPanelTabInDock` answers where it actually lives,
 * exactly as the tool-run reveal does, and the requested anchor is advisory.
 */
export function usePanelPathsAndOverrideReveals({ dock, panels, session, refreshHistorySnapshot, focusedProgram, hostOverrideTabId, studioOverrideAnchor, mobile, mobilePanelPath, mobilePanelTabs, dispatch, toolRunPanelNode, focusedTimeTravel, scope, applyAgentShellCommandRef }: Pick<ReturnType<typeof useDockModel>, "dock" | "panels" | "session" | "refreshHistorySnapshot" | "focusedProgram" | "mobile" | "mobilePanelPath" | "mobilePanelTabs" | "dispatch" | "toolRunPanelNode" | "focusedTimeTravel" | "scope" | "applyAgentShellCommandRef"> & Pick<ReturnType<typeof useOverrideAnchorsAndIntroductionIds>, "hostOverrideTabId" | "studioOverrideAnchor">) {
  /** 🧭️ Progressive reveal means a stored path can legitimately end at a branch (or be empty) — this is now a plain per-anchor truncation-validate, no override reassertion (see the write-through effects below). */
  const panelActivePaths = useMemo((): Record<Anchor, readonly string[]> => {
    const result = {} as Record<Anchor, readonly string[]>;
    for (const anchor of ANCHORS) result[anchor] = reconcileActivePath(dock.anchors[anchor], panels[anchor].path, panelTabChildren);
    return result;
  }, [panels, dock]);

  const historyTabOpenRef = useRef(false);
  useEffect(() => {
    if (!session) {
      historyTabOpenRef.current = false;
      return;
    }
    const historyOpen = Object.values(panelActivePaths).some((path) => path.includes(FRAMEWORK_PANEL_TAB_HISTORY_ID));
    const opened = historyOpen && !historyTabOpenRef.current;
    historyTabOpenRef.current = historyOpen;
    if (!opened) return;
    refreshHistorySnapshot(focusedProgram ?? { pluginId: session.pluginId, instanceId: session.instanceId });
  }, [focusedProgram, panelActivePaths, refreshHistorySnapshot, session]);

  /**
   * 🧭️ Generalizes the old `leftPanelActivePath`/`rightPanelActivePath` studio/plugin "snap to the active panel
   * tab" overrides across all eight anchors. Write-through rather than read-time: each override dispatches
   * `SET_PANEL_PATH` only when its target tab id actually changes, so a user's own collapse/navigation
   * afterward sticks instead of being reasserted on every render (progressive reveal made read-time reassertion
   * fight the user's own collapses). Studio wins over details when both would touch the same anchor.
   **/
  const lastStudioOverrideTabIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    if (!hostOverrideTabId || !studioOverrideAnchor) {
      lastStudioOverrideTabIdRef.current = undefined;
      return;
    }
    if (lastStudioOverrideTabIdRef.current === hostOverrideTabId) return;
    lastStudioOverrideTabIdRef.current = hostOverrideTabId;
    if (mobile) {
      if (mobilePanelPath[0] === FRAMEWORK_CATEGORY_DISPLAY_ID) return;
      const resolved = findPanelTabPath(mobilePanelTabs, hostOverrideTabId);
      if (resolved) dispatch({ type: "SET_MOBILE_PANEL_PATH", value: resolved });
      return;
    }
    if (panels[studioOverrideAnchor].path[0] === FRAMEWORK_CATEGORY_DISPLAY_ID) return;
    const resolved = findPanelTabPath(dock.anchors[studioOverrideAnchor], hostOverrideTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: studioOverrideAnchor, value: resolved });
  }, [hostOverrideTabId, studioOverrideAnchor, dock, panels, mobile, mobilePanelTabs, mobilePanelPath]);

  const toolRunPanelRunsRef = useRef<ReadonlySet<bigint>>(new Set());
  useEffect(() => {
    const { runs, added } = toolRunPanelReveal(toolRunPanelRunsRef.current, toolRunPanelNode);
    toolRunPanelRunsRef.current = runs;
    if (added.length === 0) return;
    const located = findPanelTabInDock(dock, FRAMEWORK_PANEL_TAB_TOOL_RUN_ID);
    if (!located) return;
    const resolved = findPanelTabPath(dock.anchors[located.anchor], FRAMEWORK_PANEL_TAB_TOOL_RUN_ID);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: located.anchor, value: resolved });
    dispatch({ type: "SET_PANEL_VISIBLE", anchor: located.anchor, value: true });
  }, [toolRunPanelNode, dock]);

  useTimeTravelRevealV1(focusedTimeTravel, { mobile, dock, mobilePanelTabs, dispatch, root: scope.rootRef.current });

  useEffect(() => {
    applyAgentShellCommandRef.current = (command: ShellCommand): void => {
      if (command.type === "focusWindow") {
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: command.windowId });
        return;
      }
      if (command.type !== "setPanelPath") return;
      const tabId = command.path[command.path.length - 1];
      if (!tabId) return;
      const located = findPanelTabInDock(dock, tabId);
      if (!located) return;
      const resolved = findPanelTabPath(dock.anchors[located.anchor], tabId);
      if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: located.anchor, value: resolved });
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: located.anchor, value: true });
    };
  }, [dock]);
  return { panelActivePaths };
}

/**
 * 🛠 Selects, activates and reveals the tool tabs and the focused panel and details override tabs.
 *
 * 🗿️ Fills {@link agentArtifactRouteRef}: this is the LIVE artifact route — an MCP client's
 * `action_prepare`/`action_invoke`/`history_undo`/`transaction_*` executing against the plugin
 * instance the human is looking at, instead of against the gateway's own headless interpreter on
 * a different `--folder` workspace (`📓️r2…` §12 finding 1).
 *
 * Three laws make it the human's edit and not a side channel:
 * 1. **One dispatch lane.** A mutation leaves through `onActionRef.current`, the shell's ONE input
 *    funnel — the same call the palette, the keybindings and the app's own buttons make. It mints
 *    an input-ledger entry, obeys the active-window rules and settles with a typed outcome, so an
 *    agent can never bypass a gate the UI respects.
 * 2. **The agent is its own actor.** `provenance.origin = "agent"` rides along, so the edit is
 *    attributable in the human's history rather than looking like something they did.
 * 3. **The commit is where the mutation happens.** The gateway's protocol is
 *    `ReadHistory → PureCommand → TransactionPrepare → TransactionCommit`; the shell has no
 *    "compute the ops but do not apply them" seam, so `pureCommand` returns a DEFERRED plan and
 *    `transactionCommit` is the one leg that dispatches. A rollback therefore really does leave
 *    the document untouched, which is the property the two-phase shape exists to give.
 */
export function useToolTabSelectionEffects({ agentDeferredPlansRef, agentArtifactRouteRef, sessionRef, spawnedAppsRef, loadedPluginsRef, onActionRef, activeWindowIdRef, leftoverInspectionIdsRef, leftoverInspectionEpoch, dock, panels, dispatch, uiRefreshCacheRef, refreshUi, reportRefreshFault, activeWindowId, spawnedIdsRef, detailsOverrideTabId, detailsOverrideAnchor, studioOverrideAnchor, mobile, mobilePanelPath, mobilePanelTabs, session, panelActivePaths, activeToolId, onActionStable, toolGeneration }: Pick<ReturnType<typeof useDockModel>, "agentDeferredPlansRef" | "agentArtifactRouteRef" | "sessionRef" | "spawnedAppsRef" | "loadedPluginsRef" | "onActionRef" | "activeWindowIdRef" | "leftoverInspectionIdsRef" | "leftoverInspectionEpoch" | "dock" | "panels" | "dispatch" | "uiRefreshCacheRef" | "refreshUi" | "reportRefreshFault" | "activeWindowId" | "spawnedIdsRef" | "mobile" | "mobilePanelPath" | "mobilePanelTabs" | "session" | "activeToolId" | "onActionStable" | "toolGeneration"> & Pick<ReturnType<typeof useOverrideAnchorsAndIntroductionIds>, "detailsOverrideTabId" | "detailsOverrideAnchor" | "studioOverrideAnchor"> & Pick<ReturnType<typeof usePanelPathsAndOverrideReveals>, "panelActivePaths">) {
  useEffect(() => {
    const deferred = agentDeferredPlansRef.current;
    agentArtifactRouteRef.current = async (request): Promise<readonly ShellAppFrameV1[]> => {
      const session = sessionRef.current;
      if (!session) return [shellAppFault("plugin.unavailable", "no plugin session is established in this shell — open an artifact here before driving it from an agent")];
      /** 🗿️🪟️ The program this request addresses: the session, or any SPAWNED program of it.
       *
       * 🧯️ This route bound `sessionRef.current` and refused every other instance by name — the LAST
       * of the session-only decision sites S5 started on (five), S6 continued (the agent census) and
       * §3 finished (progress). With it, an MCP client could reach only the landing app of the shell
       * whose whole purpose is hosting every other plugin's artifacts: measured on the live gate
       * against `s` with a spawned `note` editor as
       * `this shell's live instance is \`1\`, not \`3\` — the agent addressed an instance this shell
       * has closed`, where instance 3 was exactly the editor the human was looking at. */
      const spawnedTarget = spawnedAppsRef.current.find((entry) => String(entry.instanceId) === request.instanceId);
      const spawnedTargetApp = spawnedTarget ? loadedPluginsRef.current.find((entry) => entry.handle.pluginId === spawnedTarget.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawnedTarget.appId) : undefined;
      const live =
        request.instanceId === String(session.instanceId)
          ? { pluginId: session.pluginId, instanceId: session.instanceId, app: session.app }
          : spawnedTarget && spawnedTargetApp
            ? { pluginId: spawnedTarget.pluginId, instanceId: spawnedTarget.instanceId, app: spawnedTargetApp }
            : null;
      if (!live) {
        const open = [`${session.pluginId}:${session.app.id}:${session.instanceId}`, ...spawnedAppsRef.current.map((entry) => `${entry.pluginId}:${entry.appId}:${entry.instanceId}`)].join(", ");
        return [shellAppFault("plugin.unavailable", `this shell has no open instance \`${request.instanceId}\` — the agent addressed an instance this shell has closed (open: ${open})`)];
      }
      const readHistory = async (): Promise<ShellAppFrameV1> => {
        const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === live.pluginId)?.handle;
        if (!plugin?.readHistory) return shellAppFault("plugin.unavailable", `plugin \`${live.pluginId}\` exposes no history port in this shell`);
        const patch = await plugin.readHistory(live.instanceId);
        const head = [...(patch?.upserts ?? [])].sort((left, right) => right.seq - left.seq)[0];
        return { kind: "historySnapshot", artifactId: `${live.pluginId}:${live.app.id}:${live.instanceId}`, headEditId: head?.actionId ?? "", cursor: String(patch?.cursor ?? 0) };
      };
      const dispatchVerb = async (action: string, args: Record<string, unknown>): Promise<ShellAppFrameV1 | null> => {
        const outcome = await onActionRef.current({ controllerId: live.app.controllerId, action, args, provenance: { origin: "agent", windowId: activeWindowIdRef.current ?? null, causedBy: null } });
        if (outcome.kind === "refused") return shellAppFault(outcome.reason === "viewer-read-only" ? "viewer.read-only" : outcome.reason === "mutation-rejected" ? "app.command.rejected" : "channel.not-wired", `the shell refused \`${action}\`: ${outcome.reason}${outcome.detail ? ` (${outcome.detail})` : ""}`);
        return null;
      };
      switch (request.command.kind) {
        case "readHistory":
          return [await readHistory()];
        case "pureCommand": {
          const verb = request.command.capabilityId.split(".").pop() ?? "";
          if (!(live.app.actions ?? []).some((declared) => declared.id === verb)) {
            return [shellAppFault("capability.not-found", `app \`${live.app.id}\` declares no action \`${verb}\` (from capability \`${request.command.capabilityId}\`)`)];
          }
          const plan = new TextEncoder().encode(JSON.stringify({ deferredAction: verb, args: request.command.input ?? {} }));
          return [{ kind: "emit", ops: { document: [plan], config: [], draft: [] }, warnings: [] }];
        }
        case "transactionPrepare": {
          const lane = request.command.ops.document[0];
          if (!lane) return [shellAppFault("app.command.rejected", "the shell route prepares only its own deferred plan; this transaction carries no document-lane ops")];
          deferred.set(request.command.txnId, new TextDecoder().decode(lane));
          return [{ kind: "transactionPrepared", txnId: request.command.txnId }];
        }
        case "transactionRollback":
          deferred.delete(request.command.txnId);
          return [{ kind: "transactionRolledBack", txnId: request.command.txnId }];
        case "transactionCommit": {
          const plan = deferred.get(request.command.txnId);
          deferred.delete(request.command.txnId);
          if (plan === undefined) return [shellAppFault("transaction.generation-mismatch", `this shell holds no prepared plan for transaction \`${request.command.txnId}\``)];
          let parsed: { readonly deferredAction?: string; readonly args?: Record<string, unknown> };
          try {
            parsed = JSON.parse(plan) as { readonly deferredAction?: string; readonly args?: Record<string, unknown> };
          } catch {
            return [shellAppFault("app.command.rejected", "the prepared plan is not this shell route's own")];
          }
          if (!parsed.deferredAction) return [shellAppFault("app.command.rejected", "the prepared plan names no action")];
          const refusal = await dispatchVerb(parsed.deferredAction, parsed.args ?? {});
          if (refusal) return [refusal];
          const after = await readHistory();
          return [{ kind: "transactionCommitted", txnId: request.command.txnId, editId: after.kind === "historySnapshot" ? after.headEditId : "" }];
        }
        case "transactionUndo":
        case "transactionRedo": {
          const verb = request.command.kind === "transactionUndo" ? "undo" : "redo";
          const refusal = await dispatchVerb(verb, {});
          if (refusal) return [refusal];
          return request.command.kind === "transactionUndo" ? [{ kind: "transactionUndone", groupId: request.command.groupId }] : [{ kind: "transactionRedone", groupId: request.command.groupId }];
        }
        case "cancel":
          return [shellAppFault("budget.exceeded", `the gateway abandoned command ${request.command.cancelSeq}`)];
        case "readArtifact": {
          const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === live.pluginId)?.handle;
          if (!plugin?.readAppDocumentPack) return [shellAppFault("plugin.unavailable", `plugin \`${live.pluginId}\` exposes no document port in this shell — an agent needing a genesis read must resolve a headless context (\`--folder\`/\`--hub\`)`)];
          const document = await plugin.readAppDocumentPack(live.instanceId);
          if (!document) return [shellAppFault("plugin.unavailable", `plugin \`${live.pluginId}\` instance ${live.instanceId} holds no document to read`)];
          return [{ kind: "artifact", pack: document.pack, spr: document.spr }];
        }
        case "exportMedia": {
          const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === live.pluginId)?.handle;
          if (!plugin?.exportAppMedia) return [shellAppFault("plugin.unavailable", `plugin \`${live.pluginId}\` exposes no media OUT port in this shell — an agent needing an export must resolve a headless context (\`--folder\`/\`--hub\`)`)];
          const exported = await plugin.exportAppMedia(live.instanceId, request.command.port);
          if (!exported) return [shellAppFault("plugin.unavailable", `plugin \`${live.pluginId}\` produced no media on port \`${request.command.port}\``)];
          return [{ kind: "exported", port: exported.port, descriptor: exported.descriptor, data: exported.data }];
        }
      }
    };
  }, []);

  useEffect(() => {
    const leftoverIds = leftoverInspectionIdsRef.current.length > 0 ? leftoverInspectionIdsRef.current : leftoverWorldSelectionOverlayV1()?.ids ?? [];
    if (leftoverInspectionEpoch === 0 || leftoverIds.length === 0) return;
    const located = findPanelTabInDock(dock, FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    if (located) {
      const resolved = findPanelTabPath(dock.anchors[located.anchor], FRAMEWORK_PANEL_TAB_INSPECTION_ID);
      const current = panels[located.anchor]?.path ?? [];
      if (resolved && current.join("/") !== resolved.join("/")) dispatch({ type: "SET_PANEL_PATH", anchor: located.anchor, value: resolved });
      if (!panels[located.anchor]?.visible) dispatch({ type: "SET_PANEL_VISIBLE", anchor: located.anchor, value: true });
    }
    const inspectionCacheKey = `panel:${FRAMEWORK_PANEL_TAB_INSPECTION_ID}`;
    const cachedInspection = uiRefreshCacheRef.current.get(inspectionCacheKey);
    const inspectionHash = leftoverInspectionPanelHash(leftoverIds, cachedInspection?.hash);
    if (inspectionHash === undefined) {
      uiRefreshCacheRef.current.delete(inspectionCacheKey);
      for (const key of [...uiRefreshCacheRef.current.keys()]) {
        if (key.includes("inspection") || key.includes("inspector")) uiRefreshCacheRef.current.delete(key);
      }
    }
    const scope = leftoverInspectionRefreshScope(leftoverIds);
    const currentSession = sessionRef.current;
    if (scope && currentSession) {
      let cancelled = false;
      const timer = window.setTimeout(() => {
        if (cancelled) return;
        void refreshUi(currentSession, scope, undefined, true).catch((error: unknown) => reportRefreshFault(currentSession.pluginId, error));
      }, 250);
      return () => {
        cancelled = true;
        window.clearTimeout(timer);
      };
    }
  }, [leftoverInspectionEpoch, refreshUi]);

  const lastFocusedPanelWindowIdRef = useRef<string | null>(null);
  useEffect(() => {
    if (lastFocusedPanelWindowIdRef.current === activeWindowId) return;
    lastFocusedPanelWindowIdRef.current = activeWindowId;
    const currentSession = sessionRef.current;
    if (!currentSession) return;
    if (activeWindowId && spawnedIdOfWindowInstanceV1(activeWindowId, spawnedIdsRef.current) !== null) return;
    const panelBodies = flattenPanelTabLeaves(currentSession.app.panelTabs)
      .map((tab) => tab.bodyKey)
      .filter((bodyKey): bodyKey is string => Boolean(bodyKey));
    if (panelBodies.length === 0) return;
    void refreshUi(currentSession, { kind: "partial", panelBodies }).catch((error: unknown) => reportRefreshFault(currentSession.pluginId, error));
  }, [activeWindowId, refreshUi]);

  const lastDetailsOverrideTabIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    if (!detailsOverrideTabId || !detailsOverrideAnchor) {
      lastDetailsOverrideTabIdRef.current = undefined;
      return;
    }
    if (lastDetailsOverrideTabIdRef.current === detailsOverrideTabId) return;
    lastDetailsOverrideTabIdRef.current = detailsOverrideTabId;
    if (detailsOverrideAnchor === studioOverrideAnchor) return;
    if (mobile) {
      if (mobilePanelPath[0] === FRAMEWORK_SETTINGS_PANEL_ID) return;
      const resolved = findPanelTabPath(mobilePanelTabs, detailsOverrideTabId);
      if (resolved) dispatch({ type: "SET_MOBILE_PANEL_PATH", value: resolved });
      return;
    }
    if (panels[detailsOverrideAnchor].path[0] === FRAMEWORK_SETTINGS_PANEL_ID) return;
    const resolved = findPanelTabPath(dock.anchors[detailsOverrideAnchor], detailsOverrideTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: detailsOverrideAnchor, value: resolved });
  }, [detailsOverrideTabId, detailsOverrideAnchor, studioOverrideAnchor, dock, panels, mobile, mobilePanelTabs, mobilePanelPath]);

  /**
   * 🛠️ Single owner of "the selected `tool.<id>` leaf tab IS the active tool" (see
   * {@link reconcileToolTabSelection}). Every route into the Tool category's path lands here — a user
   * press, a `DockUiStateStore` arrangement restored on boot, an introduction step, a program
   * `setActiveTool` effect, a utility claiming the pointer — so the tab and the tool can never disagree,
   * and the very first press on a restored `tool.fill` leaf arms Fill instead of reading as a re-press.
   * Skipped wholesale (leaving `toolTabSelectionRef` untouched) while the Tool category is not the active
   * root, so an armed tool survives browsing the Command palette and is reconciled again on re-entry.
   */
  const toolTabSelectionRef = useRef<ToolTabSelection | null>(null);
  const pendingToolActivateRef = useRef<string | null | undefined>(undefined);
  const revealedToolIdRef = useRef<string | null>(null);
  useEffect(() => {
    if (!session) return;
    const toolAnchor = findPanelTabInDock(dock, FRAMEWORK_CATEGORY_TOOL_ID)?.anchor ?? "bottom-middle";
    const toolCategoryTabs = mobile ? mobilePanelTabs : dock.anchors[toolAnchor];
    const branchPath = findPanelTabPath(toolCategoryTabs, FRAMEWORK_CATEGORY_TOOL_ID);
    if (!branchPath) return;
    const path = mobile ? mobilePanelPath : panelActivePaths[toolAnchor];
    const toolCategoryActive = !branchPath.some((segment, index) => path[index] !== segment);
    if (programArmedToolRevealV1(revealedToolIdRef.current, activeToolId, toolCategoryActive)) {
      revealedToolIdRef.current = activeToolId;
      const value = findPanelTabPath(toolCategoryTabs, `tool.${activeToolId}`) ?? branchPath;
      dispatch(mobile ? { type: "SET_MOBILE_PANEL_PATH", value } : { type: "SET_PANEL_PATH", anchor: toolAnchor, value });
      dispatch(mobile ? { type: "SET_MOBILE_PANEL_VISIBLE", value: true } : { type: "SET_PANEL_VISIBLE", anchor: toolAnchor, value: true });
      return;
    }
    if (!toolCategoryActive) return;
    revealedToolIdRef.current = activeToolId;
    const selectedToolId = toolIdFromPanelTabId(path[path.length - 1]);
    if (pendingToolActivateRef.current !== undefined) {
      const pending = pendingToolActivateRef.current;
      if ((pending ?? "") === (activeToolId ?? "")) {
        pendingToolActivateRef.current = undefined;
      } else if (selectedToolId === pending) {
        return;
      } else {
        pendingToolActivateRef.current = undefined;
      }
    }
    const { next, effect } = reconcileToolTabSelection(toolTabSelectionRef.current, activeToolId, selectedToolId);
    toolTabSelectionRef.current = next;
    if (effect.kind === "activate") {
      pendingToolActivateRef.current = effect.toolId;
      void onActionStable({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: effect.toolId ?? "", expectedGeneration: toolGeneration() } });
      return;
    }
    if (effect.kind === "select") {
      const value = (effect.toolId ? findPanelTabPath(toolCategoryTabs, `tool.${effect.toolId}`) : undefined) ?? branchPath;
      dispatch(mobile ? { type: "SET_MOBILE_PANEL_PATH", value } : { type: "SET_PANEL_PATH", anchor: toolAnchor, value });
    }
  }, [activeToolId, dock, mobile, mobilePanelPath, mobilePanelTabs, onActionStable, panelActivePaths, session, toolGeneration]);
}
