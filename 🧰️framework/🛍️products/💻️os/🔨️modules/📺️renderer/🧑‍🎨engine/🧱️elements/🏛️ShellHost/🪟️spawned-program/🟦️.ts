// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellHost/spawned-program/component.ts
/** @emoji 🪟️ A spawned program as a FIRST-CLASS member of the shell session — as an OWNED unit: no
 * React, no shell imports, so a law can drive the whole projection without building the shell's
 * element graph (the separation `⌨️window-scope` and `📌️panel` already keep).
 *
 * 🏁️ What this exists for: the React shell derived every window instance it shows from
 * `sessionWindowInstances(session.app, …)` alone, while `spawnProgram` put the spawned app in the
 * host panel and never made it the session. So a foreign program opened from the palette was
 * instantiated, focused and breadcrumbed — with NO windows: the canvas pruned the host app's leaves
 * against a window list that no longer contained them and painted "Drag windows from Display in the
 * navbar", and the Display menu offered the LANDING app's window kinds.
 * Measured 2026-09-20 on the real `s` host (ticket 26/09/18, slice S4 §5.2 → S5).
 *
 * 🧭️ The model is the wgpu shell's: one program owns the canvas at a time, it owns EVERY window kind
 * it declares, and the shell's window-scoped rules (active window, per-window utilities, the Actions
 * rail, chords) address that program's instances. The difference is only that React keeps the host
 * session mounted underneath instead of retiring it, so the ids must be namespaced per spawned
 * instance — two spawned instances of the same app declare the same window kinds. */
// #endregion 🧲️Header

//#region 🪟️SpawnedWindowIdentity
/** 🪟️ Separates a spawned instance's own id from the window KIND id it owns. `::` is refused by
 * `isSpacePanelState`'s identifier rule for neither half, so the split is done against the known
 * spawned ids ({@link spawnedIdOfWindowInstanceV1}) rather than by scanning for the separator. */
const SPAWNED_WINDOW_SEPARATOR = "::";

/** 🪟️ A window kind, reduced to what a window instance needs from it. */
export type SpawnedWindowKindV1 = { readonly id: string; readonly bodyKey: string };

/** 🪟️ One live window instance of a spawned program — the same triple
 * `sessionWindowInstances` yields for the session's own app, so every consumer reads one shape. */
export type SpawnedWindowInstanceV1 = { readonly id: string; readonly bodyKey: string; readonly windowKindId: string };

/** 🪟️ The shell-wide window instance id of one window kind of one spawned instance. */
export function spawnedWindowInstanceIdV1(spawnedId: string, windowKindId: string): string {
  return `${spawnedId}${SPAWNED_WINDOW_SEPARATOR}${windowKindId}`;
}

/** 🪟️ Every window instance a spawned program contributes to the session, in declaration order —
 * one per window kind, exactly as a session app contributes one per `AppDefinition.windowKinds`
 * entry. A spawned program has no split/extra instances yet; when it gains them they append here,
 * which is why the return shape already carries `windowKindId`. */
export function spawnedProgramWindowInstancesV1(spawnedId: string, windowKinds: readonly SpawnedWindowKindV1[]): readonly SpawnedWindowInstanceV1[] {
  return windowKinds.map((kind) => ({ id: spawnedWindowInstanceIdV1(spawnedId, kind.id), bodyKey: kind.bodyKey, windowKindId: kind.id }));
}

/** 🪟️ The spawned instance a shell window id belongs to, or `null` for a session window. Resolved
 * against the live spawned ids so a spawned id containing the separator cannot mis-split, and the
 * LONGEST matching id wins for the same reason. */
export function spawnedIdOfWindowInstanceV1(windowId: string, spawnedIds: readonly string[]): string | null {
  let best: string | null = null;
  for (const spawnedId of spawnedIds) {
    if (!windowId.startsWith(`${spawnedId}${SPAWNED_WINDOW_SEPARATOR}`)) continue;
    if (best === null || spawnedId.length > best.length) best = spawnedId;
  }
  return best;
}

/** 🪟️ The window KIND a shell window id names within its spawned instance, or `null` when the id
 * belongs to no spawned instance. */
export function spawnedWindowKindOfInstanceV1(windowId: string, spawnedIds: readonly string[]): string | null {
  const spawnedId = spawnedIdOfWindowInstanceV1(windowId, spawnedIds);
  return spawnedId === null ? null : windowId.slice(spawnedId.length + SPAWNED_WINDOW_SEPARATOR.length);
}
/** 🪟️ The window id the GUEST knows, for a host window id. A spawned instance is a plugin instance
 * with exactly one set of windows, so its own window instance ids ARE its window kind ids — the same
 * identity a session app's windows have. The `${spawnedId}::` namespace is a HOST concern (two
 * spawned instances of one app must not collide in the layout, the utility map or the DOM), so it is
 * stripped at every boundary that speaks to the guest: the refresh request, the dispatched view
 * state, and the action invocation's `windowInstanceId`.
 *
 * 🧯️ Without this strip the guest is asked about a window it never declared, and
 * `undeclaredActionDiagnostic` drops the action with "no window kind declares it" — the exact shape
 * that made every foreign-kind verb unreachable. */
export function guestWindowIdV1(windowId: string | null | undefined, spawnedIds: readonly string[]): string | undefined {
  if (windowId === null || windowId === undefined) return undefined;
  return spawnedWindowKindOfInstanceV1(windowId, spawnedIds) ?? windowId;
}

/** 🧰️ The per-window active-utility map as ONE program sees it: for a spawned instance only its own
 * windows, keyed by the window kind ids it declared; for the session's own app only the windows that
 * belong to no spawned instance. Sending the whole shell-wide map would hand every guest the other
 * programs' window ids, and "active utility is per-window" then resolves against a foreign key. */
export function guestActiveUtilityByWindowIdV1(
  byWindowId: Readonly<Record<string, string | null>>,
  spawnedId: string | null,
  spawnedIds: readonly string[],
): Record<string, string> {
  const entries: [string, string][] = [];
  for (const [windowId, utilityId] of Object.entries(byWindowId)) {
    if (!utilityId) continue;
    const owner = spawnedIdOfWindowInstanceV1(windowId, spawnedIds);
    if (owner !== spawnedId) continue;
    entries.push([owner === null ? windowId : windowId.slice(owner.length + SPAWNED_WINDOW_SEPARATOR.length), utilityId]);
  }
  return Object.fromEntries(entries);
}
//#endregion 🪟️SpawnedWindowIdentity

//#region 🪟️SpawnedLayout
/** 🪟️ The layout shapes {@link renameLayoutWindowIdsV1} walks — structurally the renderer's
 * `WindowLayoutNode` (`🖱️ui/🎯️targets/⚛️react`), restated here so this unit imports no UI target. */
export type SpawnedLayoutNodeV1 =
  | { readonly kind: "window"; readonly id: string }
  | { readonly kind: "stack"; readonly activeId?: string; readonly children: readonly { readonly kind: "window"; readonly id: string }[] }
  | { readonly kind: "row" | "column"; readonly children: readonly SpawnedLayoutNodeV1[] };

/** 🪟️ Rewrites every window id in a layout tree through `rename`, preserving structure, sizes,
 * titles, corners and the per-stack `activeId`. This is what turns the spawned app's OWN declared
 * default layout — whose leaves are bare window-kind ids — into a layout over that instance's
 * namespaced window ids, so a spawned program opens in the arrangement its author declared instead
 * of a synthesized stack. A leaf `rename` does not answer for is left untouched. */
export function renameLayoutWindowIdsV1<T extends SpawnedLayoutNodeV1>(node: T, rename: (windowId: string) => string | undefined): T {
  if (node.kind === "window") {
    const next = rename(node.id);
    return next === undefined ? node : ({ ...node, id: next } as T);
  }
  if (node.kind === "stack") {
    const children = node.children.map((child) => ({ ...child, id: rename(child.id) ?? child.id }));
    const activeId = node.activeId === undefined ? undefined : (rename(node.activeId) ?? node.activeId);
    return { ...node, children, ...(node.activeId === undefined ? {} : { activeId }) } as T;
  }
  return { ...node, children: node.children.map((child) => renameLayoutWindowIdsV1(child, rename)) } as T;
}

/** 🪟️ The rename a spawned instance's layout seed needs: every one of its own window KIND ids to the
 * namespaced instance id, and nothing else. Built as a map so an app whose default layout names a
 * window kind it does not declare leaves that leaf alone (and the canvas then prunes it), instead of
 * minting a window instance the guest was never asked to render. */
export function spawnedLayoutRenameV1(spawnedId: string, windowKinds: readonly SpawnedWindowKindV1[]): (windowId: string) => string | undefined {
  const byKindId = new Map(windowKinds.map((kind) => [kind.id, spawnedWindowInstanceIdV1(spawnedId, kind.id)] as const));
  return (windowId: string) => byKindId.get(windowId);
}
//#endregion 🪟️SpawnedLayout

//#region 🪟️FocusedProgram
/** 🪟️ Which program owns the canvas: the focused spawned instance when there is one, otherwise the
 * session's own app. Deliberately a value, not a branch repeated at every call site — the defect
 * this unit exists for was exactly that branch being absent at seven of them. */
export type FocusedProgramV1 = {
  readonly pluginId: string;
  readonly instanceId: number;
  readonly appId: string;
  /** 🪟️ `null` when the session's own app is focused. */
  readonly spawnedId: string | null;
};

/** 🪟️ Resolves the focused program from the session and the host panel's focus. */
export function focusedProgramV1(
  session: { readonly pluginId: string; readonly instanceId: number; readonly appId: string } | null,
  spawned: { readonly id: string; readonly pluginId: string; readonly instanceId: number; readonly appId: string } | null | undefined,
): FocusedProgramV1 | null {
  if (spawned) return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, appId: spawned.appId, spawnedId: spawned.id };
  return session ? { pluginId: session.pluginId, instanceId: session.instanceId, appId: session.appId, spawnedId: null } : null;
}

/** 🪟️ The identity a layout/active-window re-seed is keyed on. A re-seed must fire when the focused
 * PROGRAM changes — including from a spawned instance back to the session — and must NOT fire on an
 * ordinary refresh of the same program, which would throw away the user's live arrangement. */
export function focusedProgramKeyV1(focused: FocusedProgramV1 | null): string {
  return focused === null ? "" : `${focused.spawnedId ?? ""}|${focused.pluginId}:${focused.appId}:${focused.instanceId}`;
}
/** 🪟️ The view state a spawned program's own dispatch and refresh must carry. A spawned session was
 * built as `{ …, viewState: session.viewState }` — the HOST app's view state — so every invocation
 * addressed the host's active MODE. Measured verbatim on the live `s` host:
 * `addLayer refused: dispatch-failed (window=draw-4::drawing-composite) — unknown action mode owner
 * explore`, where `explore` is `s.space.home`'s mode and `draw` declares none by that name. The mode
 * belongs to the app the action is addressed to, so it is restated here rather than inherited.
 *
 * 🪶️ `windowId`/`activeWindowKindId` are dropped for the same reason: they name the host's windows
 * and each dispatch stamps its own. */
export function spawnedProgramViewStateV1<T extends { activeModeId?: string; windowId?: string; activeWindowKindId?: string }>(
  viewState: T,
  app: { readonly id: string; readonly defaultModeId?: string; readonly modes?: readonly { readonly id: string }[] },
): T {
  const { windowId: _windowId, activeWindowKindId: _activeWindowKindId, ...rest } = viewState;
  return { ...(rest as T), activeModeId: app.defaultModeId ?? app.modes?.[0]?.id ?? app.id };
}
//#endregion 🪟️FocusedProgram

//#region 🧾️ProgramHistory
/** 🧾️ One program's identity as the history lane keys it — the `(pluginId, instanceId)` pair, which
 * is exactly the pair an `AppFrame::OperationCompleted` carries its `historyPatch` for. */
export function programHistoryKeyV1(program: { readonly pluginId: string; readonly instanceId: number } | null | undefined): string {
  return program ? `${program.pluginId}#${program.instanceId}` : "";
}

/** 🧾️ Every open program's history projection, keyed by {@link programHistoryKeyV1}. */
export type ProgramHistoryProjectionsV1<Projection> = Readonly<Record<string, Projection>>;

/** 🧾️ One program's projection, or the empty one for a program nothing has been read for yet. */
export function programHistoryProjectionV1<Projection>(projections: ProgramHistoryProjectionsV1<Projection>, key: string, empty: Projection): Projection {
  return projections[key] ?? empty;
}

/** 🧾️ Admits one `HistoryPatch` into the projection of the program that PRODUCED it.
 *
 * 🧯️ What this exists for, measured on the live `s` host (ticket 26/09/18 slice S7,
 * `🗑️generated/s7-history-hook-draw.txt`): the shell kept ONE history projection for the whole
 * window, and a patch was admitted by comparing its cursor against that single cursor. With a
 * spawned program on the canvas the two cursors belong to DIFFERENT documents, so the host's cursor
 * acts as a floor under the spawned document's own:
 *
 * ```
 * spawned-completion draw#4 operation=64  patchCursor=1 upserts=1
 * patch              patchCursor=1 currentCursor=3 applied=false   ← the studio's cursor, not draw's
 * spawned-completion draw#4 operation=128 patchCursor=2 upserts=1
 * patch              patchCursor=2 currentCursor=3 applied=false
 * ```
 *
 * The completion frames arrive on time — the drain and the per-instance subscription are not the
 * lag — and every one of them is DROPPED until the spawned document's own cursor climbs past the
 * host's. That is the "the host's reading of a spawned program is two dispatches behind" residual
 * (S5 §4.3, S6 §2): a cursor race between two documents sharing one projection, which is why it was
 * never latency and why no amount of host-side refreshing moved it.
 *
 * `admits` is the shell's own cursor rule (`historyPatchShouldApplyV1`), passed in so this unit
 * imports no shell code; `apply` produces the program's next projection. What this owns is the one
 * decision that was missing: WHOSE cursor the rule is taken against. */
export function programHistoryProjectionsAfterPatchV1<Projection extends { readonly cursor: number }>(
  projections: ProgramHistoryProjectionsV1<Projection>,
  key: string,
  empty: Projection,
  admits: (currentCursor: number) => boolean,
  apply: (current: Projection) => Projection,
): { readonly projections: ProgramHistoryProjectionsV1<Projection>; readonly applied: boolean } {
  const current = projections[key] ?? empty;
  if (!admits(current.cursor)) return { projections, applied: false };
  return { projections: { ...projections, [key]: apply(current) }, applied: true };
}

/** 🧾️ Drops the projections of programs that are no longer open, so a shell that spawns and closes
 * editors all day keeps one entry per LIVE program rather than a growing ledger of dead ones. The
 * session's own key is always retained, including before its first patch. */
export function programHistoryProjectionsRetainedV1<Projection>(
  projections: ProgramHistoryProjectionsV1<Projection>,
  liveKeys: readonly string[],
): ProgramHistoryProjectionsV1<Projection> {
  const live = new Set(liveKeys.filter((key) => key.length > 0));
  const kept = Object.entries(projections).filter(([key]) => live.has(key));
  return kept.length === Object.keys(projections).length ? projections : Object.fromEntries(kept);
}
//#endregion 🧾️ProgramHistory

//#region 📇️SpawnedBridgeCensus
/** 📇️ One program as the agent bridge addresses it: an artifact handle plus the SHELL's own window
 * ids, which is what `ui_focus` and every window-scoped agent command take. */
export type BridgeProgramRefV1 = {
  readonly pluginId: string;
  readonly appId: string;
  readonly instanceId: string;
  readonly artifactRef: string;
  readonly windowIds: readonly string[];
  /** 🎯️ `true` for the ONE program that owns the canvas right now. A capability-less agent command
   * (`ReadArtifact` with no prepared action) has no other rule for choosing among several open
   * instances, and the shell's own answer to "which one does the user mean" is this one. */
  readonly focused: boolean;
};

/** 📇️ The census an MCP client sees: the live session PLUS every spawned program, with exactly one
 * row marked focused.
 *
 * 🧯️ S6 §5.2 measured the fault this answers — `agentBridgeInstances` was the live session and
 * nothing else, so an agent could drive only the landing app of the shell whose whole purpose is
 * hosting every other plugin's artifacts (`the shell reports 1 open instance(s)`). S6 fixed that
 * inside the component; this is the same decision as an owned value, plus the `focused` flag S6 §5.4
 * named as the missing tie-breaker (`the attached shell has 3 open instances — prepare an action
 * against the artifact first so the route binds`). */
export function spawnedBridgeCensusV1(
  session: { readonly pluginId: string; readonly appId: string; readonly instanceId: number; readonly windowIds: readonly string[] } | null,
  spawned: readonly { readonly id: string; readonly pluginId: string; readonly appId: string; readonly instanceId: number; readonly windowIds: readonly string[] }[],
  focused: FocusedProgramV1 | null,
): readonly BridgeProgramRefV1[] {
  const focusedKey = programHistoryKeyV1(focused);
  const row = (pluginId: string, appId: string, instanceId: number, windowIds: readonly string[]): BridgeProgramRefV1 => ({
    pluginId,
    appId,
    instanceId: String(instanceId),
    artifactRef: `${pluginId}:${appId}:${instanceId}`,
    windowIds,
    focused: programHistoryKeyV1({ pluginId, instanceId }) === focusedKey,
  });
  return [
    ...(session ? [row(session.pluginId, session.appId, session.instanceId, session.windowIds)] : []),
    ...spawned.map((entry) => row(entry.pluginId, entry.appId, entry.instanceId, entry.windowIds)),
  ];
}
//#endregion 📇️SpawnedBridgeCensus

