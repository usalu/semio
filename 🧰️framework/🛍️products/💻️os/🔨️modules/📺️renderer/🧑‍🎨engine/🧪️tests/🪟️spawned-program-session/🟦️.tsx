/** 🪟️ A spawned program is a member of the shell session — the laws behind
 * `📓️s5-spawned-app-windows-in-s.md`.
 *
 * 🏁️ The defect these pin: the React shell derived every window instance it showed from
 * `sessionWindowInstances(session.app, …)`, while `spawnProgram` put the spawned app in the host
 * panel and never gave it windows. Measured on the real `s` host: after spawning `draw` the
 * breadcrumb read `semio · drawing`, `[data-window-id]` was `[]`, and the canvas painted
 * "Drag windows from Display in the navbar, or restore a saved layout" (ticket 26/09/18, S4 §5.2).
 *
 * The canvas half is driven against the REAL `Mode` renderer, not an oracle of it: the pruning that
 * emptied the shell is `resolveModeLayout` → `reconcileWindows` inside that component. */
import { cleanup, render } from "@semio-tech/ui-react/test";
import { Mode, type ModeWindowDescriptor, type WindowLayoutNode } from "@semio-tech/ui-react";
import { afterEach, describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import {
  focusedProgramKeyV1,
  focusedProgramV1,
  guestActiveUtilityByWindowIdV1,
  guestWindowIdV1,
  programHistoryKeyV1,
  programHistoryProjectionV1,
  programHistoryProjectionsAfterPatchV1,
  programHistoryProjectionsRetainedV1,
  renameLayoutWindowIdsV1,
  spawnedBridgeCensusV1,
  spawnedGuestWindowInstancesV1,
  spawnedIdOfWindowInstanceV1,
  spawnedLayoutRenameV1,
  spawnedProgramWindowInstancesV1,
  spawnedWindowInstanceIdV1,
  spawnedWindowKindOfInstanceV1,
  spawnProgramRefusalCodeV1,
  spawnProgramRefusalNoticeTextV1,
  SPAWN_PROGRAM_REFUSAL_LABELS_V1,
  type SpawnedLayoutNodeV1,
} from "../../🧱️elements/🏛️ShellHost/🪟️spawned-program/🟦️.ts";
import { applyUiRefreshResponseToCache, buildUiRefreshRequest, EMPTY_APP_LABELS_OVERLAY, frameworkLayoutDeclaredInstances, historyPatchShouldApplyV1, resolveFrameworkLayoutSeed, type UiRefreshCache } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import declaredInstancesFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🪟️declared-instances.json";
import { resolveAppSurfaceSessionFactory, type AppSurfaceSessionFactory } from "../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";

afterEach(cleanup);

const DRAW_KINDS = [
  { id: "draw-main", bodyKey: "draw.canvas" },
  { id: "draw-layers", bodyKey: "draw.layers" },
];
const SPAWNED_ID = "draw-7";

const shellHostSource = readFileSync(join(import.meta.dirname, "..", "..", "🧱️elements", "🏛️ShellHost", "🟦️.tsx"), "utf8");

const windowDescriptor = (id: string, label: string): ModeWindowDescriptor =>
  ({ id, title: label as unknown as ModeWindowDescriptor["title"], iconId: "app-window", fill: true, children: <div data-testid={`body-${id}`}>{label}</div> }) as unknown as ModeWindowDescriptor;

describe("🪟️ spawned window identity", () => {
  it("gives every window kind of a spawned instance its own shell window id", () => {
    expect(spawnedProgramWindowInstancesV1(SPAWNED_ID, DRAW_KINDS)).toEqual([
      { id: "draw-7::draw-main", bodyKey: "draw.canvas", windowKindId: "draw-main" },
      { id: "draw-7::draw-layers", bodyKey: "draw.layers", windowKindId: "draw-layers" },
    ]);
  });

  it("keeps two instances of the same app apart", () => {
    const first = spawnedProgramWindowInstancesV1("draw-1", DRAW_KINDS).map((instance) => instance.id);
    const second = spawnedProgramWindowInstancesV1("draw-2", DRAW_KINDS).map((instance) => instance.id);
    expect(first.some((id) => second.includes(id))).toBe(false);
  });

  it("resolves a window id back to its owning instance, and a session window to none", () => {
    const ids = ["draw-1", "draw-1-extra", "note-3"];
    expect(spawnedIdOfWindowInstanceV1("draw-1::draw-main", ids)).toBe("draw-1");
    expect(spawnedIdOfWindowInstanceV1("draw-1-extra::draw-main", ids)).toBe("draw-1-extra");
    expect(spawnedIdOfWindowInstanceV1("s-home-main", ids)).toBe(null);
  });

  it("hands the guest its OWN window id — the strip that keeps a verb from being dropped", () => {
    expect(spawnedWindowKindOfInstanceV1("draw-1::draw-main", ["draw-1"])).toBe("draw-main");
    expect(guestWindowIdV1("draw-1::draw-main", ["draw-1"])).toBe("draw-main");
    expect(guestWindowIdV1("s-home-main", ["draw-1"])).toBe("s-home-main");
    expect(guestWindowIdV1(null, ["draw-1"])).toBe(undefined);
  });

  it("scopes the per-window active-utility map to ONE program", () => {
    const shellWide = { "s-home-main": "select", "draw-1::draw-main": "pencil", "note-2::note-main": "bold" };
    expect(guestActiveUtilityByWindowIdV1(shellWide, "draw-1", ["draw-1", "note-2"])).toEqual({ "draw-main": "pencil" });
    expect(guestActiveUtilityByWindowIdV1(shellWide, null, ["draw-1", "note-2"])).toEqual({ "s-home-main": "select" });
  });
});

describe("🪟️ focused program", () => {
  const session = { pluginId: "space", instanceId: 1, appId: "s.space.home@1/*#editor" };
  const spawned = { id: SPAWNED_ID, pluginId: "draw", instanceId: 4, appId: "s.draw.drawing@1/*#editor" };

  it("is the spawned instance when one is focused, and the session otherwise", () => {
    expect(focusedProgramV1(session, spawned)).toEqual({ pluginId: "draw", instanceId: 4, appId: spawned.appId, spawnedId: SPAWNED_ID });
    expect(focusedProgramV1(session, null)).toEqual({ pluginId: "space", instanceId: 1, appId: session.appId, spawnedId: null });
    expect(focusedProgramV1(null, null)).toBe(null);
  });

  it("re-seeds on a program change and NOT on a refresh of the same program", () => {
    const focused = focusedProgramV1(session, spawned);
    expect(focusedProgramKeyV1(focused)).toBe(focusedProgramKeyV1(focusedProgramV1(session, { ...spawned })));
    expect(focusedProgramKeyV1(focused)).not.toBe(focusedProgramKeyV1(focusedProgramV1(session, null)));
    expect(focusedProgramKeyV1(focused)).not.toBe(focusedProgramKeyV1(focusedProgramV1(session, { ...spawned, instanceId: 5 })));
  });
});

describe("🪟️ spawned layout seed", () => {
  const declared: SpawnedLayoutNodeV1 = {
    kind: "row",
    children: [
      { kind: "stack", activeId: "draw-main", children: [{ kind: "window", id: "draw-main" }] },
      { kind: "stack", children: [{ kind: "window", id: "draw-layers" }] },
    ],
  };

  it("renames the app's own declared layout into the instance's window-id namespace", () => {
    const renamed = renameLayoutWindowIdsV1(declared, spawnedLayoutRenameV1(SPAWNED_ID, DRAW_KINDS));
    expect(JSON.stringify(renamed)).toContain("draw-7::draw-main");
    expect(JSON.stringify(renamed)).toContain("draw-7::draw-layers");
    expect((renamed as Extract<SpawnedLayoutNodeV1, { kind: "row" | "column" }>).children[0]).toMatchObject({ activeId: "draw-7::draw-main" });
  });

  it("leaves a leaf the app does not declare alone rather than minting a window", () => {
    const stray: SpawnedLayoutNodeV1 = { kind: "stack", children: [{ kind: "window", id: "not-a-kind" }] };
    expect(renameLayoutWindowIdsV1(stray, spawnedLayoutRenameV1(SPAWNED_ID, DRAW_KINDS))).toEqual(stray);
  });
});

/** 🪟️ Declared instances, replayed from `🏛️ShellHost/🧫️fixtures/🪟️declared-instances.json`: a program owns the named
 * instances its OWN default layout declares, not only one window per kind (ticket 26/09/23 S15). */
describe("🪟️ spawned declared instances", () => {
  const fixture = declaredInstancesFixture as unknown as {
    readonly spawnedId: string;
    readonly windowKinds: readonly { readonly id: string; readonly bodyKey: string }[];
    readonly layout: Parameters<typeof frameworkLayoutDeclaredInstances>[0];
    readonly declaredInstances: readonly { readonly id: string; readonly windowKindId: string }[];
    readonly guestInstances: readonly { readonly id: string; readonly bodyKey: string; readonly windowKindId: string }[];
    readonly renamedLayoutWindowIds: readonly string[];
  };
  const labelled = fixture.windowKinds.map((kind) => ({ ...kind, label: kind.id }));
  const layoutWindowIds = (node: WindowLayoutNode): string[] => (node.kind === "window" ? [node.id] : node.kind === "stack" ? node.children.map((child) => child.id) : node.children.flatMap(layoutWindowIds));

  it("reads the declared instances off the program's own default layout, skipping undeclared kinds", () => {
    expect(frameworkLayoutDeclaredInstances(fixture.layout, fixture.windowKinds)).toEqual(fixture.declaredInstances);
    expect(frameworkLayoutDeclaredInstances(undefined, fixture.windowKinds)).toEqual([]);
  });

  it("makes every declared instance a window of the program, under the guest's ids and the shell's namespace", () => {
    expect(spawnedGuestWindowInstancesV1(fixture.windowKinds, fixture.declaredInstances)).toEqual(fixture.guestInstances);
    expect(spawnedProgramWindowInstancesV1(fixture.spawnedId, fixture.windowKinds, fixture.declaredInstances)).toEqual(fixture.guestInstances.map((instance) => ({ ...instance, id: spawnedWindowInstanceIdV1(fixture.spawnedId, instance.id) })));
    expect(guestWindowIdV1(spawnedWindowInstanceIdV1(fixture.spawnedId, "puzzle3d-main-top"), [fixture.spawnedId])).toBe("puzzle3d-main-top");
  });

  it("never mints a declared instance that collides with a kind id or names an undeclared kind", () => {
    expect(spawnedGuestWindowInstancesV1(fixture.windowKinds, [{ id: "puzzle3d-catalogue", windowKindId: "puzzle3d-main" }, { id: "x", windowKindId: "nope" }]).map((instance) => instance.id)).toEqual(["puzzle3d-main", "puzzle3d-catalogue"]);
  });

  it("renames the seeded default layout onto the declared instances, leaving a stray leaf alone", () => {
    const seed = resolveFrameworkLayoutSeed(fixture.layout, labelled, EMPTY_APP_LABELS_OVERLAY, "native", "en");
    const renamed = renameLayoutWindowIdsV1(seed.modeLayout as SpawnedLayoutNodeV1, spawnedLayoutRenameV1(fixture.spawnedId, fixture.windowKinds, fixture.declaredInstances)) as unknown as WindowLayoutNode;
    expect(layoutWindowIds(renamed)).toEqual(fixture.renamedLayoutWindowIds);
  });

  it("REGRESSION: without the declared instances the Mode renderer prunes every view and the shell is empty", () => {
    const seed = resolveFrameworkLayoutSeed({ root: { kind: "stack", children: [{ kind: "window", windowKindId: "puzzle3d-main", instanceId: "puzzle3d-main-top" }] } } as Parameters<typeof frameworkLayoutDeclaredInstances>[0], labelled, EMPTY_APP_LABELS_OVERLAY, "native", "en");
    const kindOnly = spawnedProgramWindowInstancesV1(fixture.spawnedId, fixture.windowKinds).map((instance) => windowDescriptor(instance.id, instance.windowKindId));
    const legacyLayout = renameLayoutWindowIdsV1(seed.modeLayout as SpawnedLayoutNodeV1, spawnedLayoutRenameV1(fixture.spawnedId, fixture.windowKinds)) as unknown as WindowLayoutNode;
    render(<Mode windows={kindOnly} layout={legacyLayout} activeWindowId={layoutWindowIds(legacyLayout)[0]!} />);
    expect(document.querySelector('[data-slot="mode-empty"]')).not.toBe(null);
  });

  it("with the declared instances the Mode renderer mounts every declared view", () => {
    const views = { root: { kind: "stack", children: fixture.declaredInstances.map((instance) => ({ kind: "window", windowKindId: instance.windowKindId, instanceId: instance.id })) } } as Parameters<typeof frameworkLayoutDeclaredInstances>[0];
    const seed = resolveFrameworkLayoutSeed(views, labelled, EMPTY_APP_LABELS_OVERLAY, "native", "en");
    const windows = spawnedProgramWindowInstancesV1(fixture.spawnedId, fixture.windowKinds, fixture.declaredInstances).map((instance) => windowDescriptor(instance.id, instance.windowKindId));
    const layout = renameLayoutWindowIdsV1(seed.modeLayout as SpawnedLayoutNodeV1, spawnedLayoutRenameV1(fixture.spawnedId, fixture.windowKinds, fixture.declaredInstances)) as unknown as WindowLayoutNode;
    render(<Mode windows={windows} layout={layout} activeWindowId={spawnedWindowInstanceIdV1(fixture.spawnedId, "puzzle3d-main-perspective")} />);
    expect(document.querySelector('[data-slot="mode-empty"]')).toBe(null);
    const mounted = [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id"));
    expect(mounted).toEqual(fixture.declaredInstances.map((instance) => spawnedWindowInstanceIdV1(fixture.spawnedId, instance.id)));
  });
});

describe("🪟️ the canvas, driven against the real Mode renderer", () => {
  const spawnedWindows = spawnedProgramWindowInstancesV1(SPAWNED_ID, DRAW_KINDS).map((instance) => windowDescriptor(instance.id, instance.windowKindId));
  const hostLayout: WindowLayoutNode = { kind: "stack", activeId: "s-home-main", children: [{ kind: "window", id: "s-home-main" }] };
  const mountedWindowIds = () => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id"));

  it("REGRESSION: the host app's layout prunes every spawned window away and the shell is empty", () => {
    render(<Mode windows={spawnedWindows} layout={hostLayout} activeWindowId="s-home-main" />);
    expect(mountedWindowIds()).toEqual([]);
    expect(document.querySelector(`[data-testid="body-${spawnedWindows[0]!.id}"]`)).toBe(null);
    // 🧯️ The exact live symptom: `[data-window-id] = []` and the empty-shell notice.
    expect(document.querySelector('[data-slot="mode-empty"]')).not.toBe(null);
  });

  it("the spawned program's own seeded layout puts every one of its windows on the canvas", () => {
    const seeded = renameLayoutWindowIdsV1(
      { kind: "stack", activeId: DRAW_KINDS[0]!.id, children: DRAW_KINDS.map((kind) => ({ kind: "window" as const, id: kind.id })) },
      spawnedLayoutRenameV1(SPAWNED_ID, DRAW_KINDS),
    ) as unknown as WindowLayoutNode;
    render(<Mode windows={spawnedWindows} layout={seeded} activeWindowId={spawnedWindows[0]!.id} />);
    expect(document.querySelector('[data-slot="mode-empty"]')).toBe(null);
    expect(mountedWindowIds()).toEqual(spawnedWindows.map((instance) => instance.id));
    expect(document.querySelector(`[data-testid="body-${spawnedWindows[0]!.id}"]`)).not.toBe(null);
  });
});

describe("🪟️ the ShellHost decision sites route through the focused program", () => {
  it("seeds the canvas layout on a focused-program change", () => {
    expect(shellHostSource).toContain("focusedProgramKeyV1(focusedProgram)");
    expect(shellHostSource).toContain("spawnedLayoutRenameV1(focusedSpawnedId, focusedApp.windowKinds, frameworkLayoutDeclaredInstances(focusedApp.defaultLayout, focusedApp.windowKinds))");
  });

  it("offers the FOCUSED program's window kinds in Display ▸ Windows", () => {
    const display = shellHostSource.slice(shellHostSource.indexOf("const displayHost = useNamedLayoutHost({"));
    expect(display.slice(0, 600)).toContain("windowKinds: focusedApp?.windowKinds");
    expect(display.slice(0, 600)).not.toContain("windowKinds: session?.app.windowKinds");
  });

  it("strips the shell's window namespace before every guest dispatch", () => {
    expect(shellHostSource).toContain("const dispatchWindowId = guestWindowIdV1(hostWindowId, spawnedIdsRef.current);");
    expect(shellHostSource).toContain("activeUtilityByWindowId: guestActiveUtilityByWindowIdV1(activeUtilityByWindowIdRef.current, targetSpawnedId, spawnedIdsRef.current)");
  });

  it("routes a window's utility activation to the program that owns that window", () => {
    expect(shellHostSource).toContain("const utilitySpawnedId = spawnedIdOfWindowInstanceV1(windowId, spawnedIdsRef.current);");
    expect(shellHostSource).toContain("handleAction(utilitySession.instanceId");
  });

  it("retires the spawned instance when its LAST window closes", () => {
    expect(shellHostSource).toContain("const closedSpawnedId = hostMode && panel ? spawnedIdOfWindowInstanceV1(windowId, panel.spawnedApps.map((entry) => entry.id)) : null;");
    expect(shellHostSource).toContain("if (remaining.length === 0) {");
  });

  it("subscribes operation completions per spawned instance, not only for the session", () => {
    expect(shellHostSource).toContain("entry.handle.subscribeOperationCompletions(spawned.instanceId, (completion) => {");
    expect(shellHostSource).toContain("spawnedRosterKey");
  });

  it("gives a spawned dispatch its OWN app's mode", () => {
    expect(shellHostSource).toContain("spawnedProgramViewStateV1(session.viewState, app)");
    expect(shellHostSource).toContain("spawnedProgramViewStateV1(session.viewState, ownerApp)");
  });

  it("resolves the board session factory for the FOCUSED program, so a spawned board app gets its own", () => {
    expect(shellHostSource).toContain("resolveAppSurfaceSessionFactory(surfaceSessionFactories ?? [], focusedProgram)");
    const registrations: AppSurfaceSessionFactory[] = [{ kind: "board-2d", pluginId: "puzzle", appId: "s.puzzle.puzzle2d@1/*#editor", create: () => Promise.reject(new Error("not created in this law")) }];
    const home = { pluginId: "space", instanceId: 1, appId: "s.space.home@1/*#editor" };
    const puzzle = { id: "puzzle-3", pluginId: "puzzle", instanceId: 9, appId: "s.puzzle.puzzle2d@1/*#editor" };
    expect(resolveAppSurfaceSessionFactory(registrations, { pluginId: home.pluginId, appId: home.appId, instanceId: home.instanceId })).toBe(null);
    expect(resolveAppSurfaceSessionFactory(registrations, focusedProgramV1(home, puzzle))).toMatchObject({ pluginId: "puzzle", appId: puzzle.appId, instanceId: 9 });
  });

  it("hands keyboard focus to an opened program's active window only while nothing else holds it", () => {
    const effect = shellHostSource.slice(shellHostSource.indexOf("A program the user just opened takes keyboard focus"));
    expect(effect.slice(0, 1600)).toContain("if (holder instanceof HTMLElement && holder !== document.body) return;");
    expect(effect.slice(0, 1600)).toContain(`closest<HTMLElement>('[role="tabpanel"]')`);
  });

  it("routes a window-scoped dispatch to the program that owns the window", () => {
    expect(shellHostSource).toContain("const windowOwnerId = spawnedIdOfWindowInstanceV1(hostWindowId, spawnedIdsRef.current);");
  });

  it("resolves chords against the focused program, not the landing app", () => {
    const keydown = shellHostSource.slice(shellHostSource.indexOf("const handleAppKeydown = useCallback("), shellHostSource.indexOf("useShellKeydown(scope.rootRef"));
    expect(keydown).toContain("const keyApp = focusedApp ?? session.app;");
    expect(keydown).not.toContain("session.app.controllerId");
    expect(keydown).not.toContain("session.app.keybindings");
  });

  it("subscribes operation PROGRESS per spawned instance too, not only for the session", () => {
    expect(shellHostSource).toContain("entry.handle.subscribeOperationProgress(spawned.instanceId,");
    expect(shellHostSource).toContain("entry.handle.subscribeSpawnedJobProgress?.(spawned.instanceId,");
  });

  it("applies every history patch to the program that produced it", () => {
    expect(shellHostSource).toContain("applyHistoryPatch(completion.historyPatch, false, { pluginId: spawned.pluginId, instanceId: spawned.instanceId });");
    expect(shellHostSource).toContain("applyHistoryPatch(response.historyPatch, false, { pluginId: targetSession.pluginId, instanceId: targetSession.instanceId });");
    expect(shellHostSource).toContain("const key = programHistoryKeyV1(owner ?? sessionRef.current);");
  });

  it("reads a history SNAPSHOT through the owner's own plugin handle", () => {
    const snapshot = shellHostSource.slice(shellHostSource.indexOf("const refreshHistorySnapshot = useCallback("));
    expect(snapshot.slice(0, 700)).toContain("entry.handle.pluginId === owner.pluginId");
    expect(snapshot.slice(0, 700)).not.toContain("sessionRef.current?.pluginId");
  });

  it("shows the FOCUSED program's ledger", () => {
    expect(shellHostSource).toContain("const historyProjection = programHistoryProjectionV1(historyProjectionByProgram, programHistoryKeyV1(focusedProgram), EMPTY_SHELL_HISTORY_PROJECTION_V1);");
  });
});

/** 🧾️ The two-dispatch lag, as a law. Measured live on `:6071` (S7 §2): the shell kept ONE history
 * projection for the whole window, so a spawned program's patches were admitted against the HOST
 * document's cursor and silently dropped until the spawned document's own cursor climbed past it. */
describe("🧾️ a spawned program's ledger is its own", () => {
  type Projection = { readonly cursor: number; readonly entries: Readonly<Record<number, { readonly seq: number }>> };
  const EMPTY: Projection = { cursor: 0, entries: {} };
  const HOST = programHistoryKeyV1({ pluginId: "space", instanceId: 2 });
  const SPAWNED = programHistoryKeyV1({ pluginId: "draw", instanceId: 4 });
  const patchOf = (cursor: number) => ({ cursor, upserts: [{ seq: cursor }] });
  const apply = (patch: { cursor: number; upserts: readonly { seq: number }[] }) => (current: Projection): Projection => ({
    cursor: patch.cursor,
    entries: { ...current.entries, ...Object.fromEntries(patch.upserts.map((entry) => [entry.seq, entry])) },
  });
  /** 🧾️ The host's ledger as the live shell had it when a foreign editor was spawned into the studio. */
  const withHostAtCursor3 = (): Record<string, Projection> => ({ [HOST]: { cursor: 3, entries: { 1: { seq: 1 }, 2: { seq: 2 }, 3: { seq: 3 } } } });

  it("N dispatches on a spawned instance leave N rows in ITS ledger, after each one", () => {
    let projections: Record<string, Projection> = withHostAtCursor3();
    const rowsAfterEachDispatch: number[] = [];
    for (let dispatch = 1; dispatch <= 4; dispatch += 1) {
      const patch = patchOf(dispatch);
      const step = programHistoryProjectionsAfterPatchV1(projections, SPAWNED, EMPTY, (cursor) => historyPatchShouldApplyV1(cursor, patch, false), apply(patch));
      expect(step.applied).toBe(true);
      projections = step.projections as Record<string, Projection>;
      rowsAfterEachDispatch.push(Object.keys(programHistoryProjectionV1(projections, SPAWNED, EMPTY).entries).length);
    }
    expect(rowsAfterEachDispatch).toEqual([1, 2, 3, 4]);
  });

  it("REGRESSION: one shared projection drops every one of those patches — the live two-dispatch lag", () => {
    let shared: Projection = { cursor: 3, entries: { 1: { seq: 1 }, 2: { seq: 2 }, 3: { seq: 3 } } };
    const admitted: boolean[] = [];
    for (let dispatch = 1; dispatch <= 3; dispatch += 1) {
      const patch = patchOf(dispatch);
      const ok = historyPatchShouldApplyV1(shared.cursor, patch, false);
      admitted.push(ok);
      if (ok) shared = apply(patch)(shared);
    }
    expect(admitted).toEqual([false, false, true]);
  });

  it("the host's own ledger is untouched while a spawned program is edited", () => {
    const patch = patchOf(1);
    const step = programHistoryProjectionsAfterPatchV1(withHostAtCursor3(), SPAWNED, EMPTY, (cursor) => historyPatchShouldApplyV1(cursor, patch, false), apply(patch));
    expect(programHistoryProjectionV1(step.projections, HOST, EMPTY).cursor).toBe(3);
    expect(programHistoryProjectionV1(step.projections, SPAWNED, EMPTY).cursor).toBe(1);
  });

  it("a program nothing has been read for projects nothing, not another program's ledger", () => {
    expect(programHistoryProjectionV1(withHostAtCursor3(), SPAWNED, EMPTY)).toBe(EMPTY);
  });

  it("drops the projections of programs that are no longer open", () => {
    const projections = { ...withHostAtCursor3(), [SPAWNED]: { cursor: 2, entries: {} } };
    expect(Object.keys(programHistoryProjectionsRetainedV1(projections, [HOST]))).toEqual([HOST]);
    expect(programHistoryProjectionsRetainedV1(projections, [HOST, SPAWNED])).toBe(projections);
  });
});

/** 📇️ The agent census — S6 §5.2's fix as an owned value, plus the focus tie-breaker S6 §5.4 named. */
describe("📇️ the agent census", () => {
  const SESSION = { pluginId: "space", appId: "s.space.home", instanceId: 2, windowIds: ["s-home-main"] };
  const SPAWNED = [
    { id: "draw-4", pluginId: "draw", appId: "s.draw.drawing", instanceId: 4, windowIds: ["draw-4::drawing-composite"] },
    { id: "note-5", pluginId: "note", appId: "s.note.note", instanceId: 5, windowIds: ["note-5::note-composite"] },
  ];

  it("is the session PLUS every spawned program", () => {
    const census = spawnedBridgeCensusV1(SESSION, SPAWNED, focusedProgramV1({ pluginId: "space", instanceId: 2, appId: "s.space.home" }, null));
    expect(census.map((entry) => entry.artifactRef)).toEqual(["space:s.space.home:2", "draw:s.draw.drawing:4", "note:s.note.note:5"]);
    expect(census.map((entry) => entry.windowIds)).toEqual([["s-home-main"], ["draw-4::drawing-composite"], ["note-5::note-composite"]]);
  });

  it("marks exactly ONE row focused, and it is the program that owns the canvas", () => {
    const focused = focusedProgramV1({ pluginId: "space", instanceId: 2, appId: "s.space.home" }, { id: "note-5", pluginId: "note", instanceId: 5, appId: "s.note.note" });
    const census = spawnedBridgeCensusV1(SESSION, SPAWNED, focused);
    expect(census.filter((entry) => entry.focused).map((entry) => entry.artifactRef)).toEqual(["note:s.note.note:5"]);
  });

  it("does not confuse two instances of the same plugin", () => {
    const twins = [
      { id: "draw-4", pluginId: "draw", appId: "s.draw.drawing", instanceId: 4, windowIds: [] },
      { id: "draw-9", pluginId: "draw", appId: "s.draw.drawing", instanceId: 9, windowIds: [] },
    ];
    const focused = focusedProgramV1(null, { id: "draw-9", pluginId: "draw", instanceId: 9, appId: "s.draw.drawing" });
    expect(spawnedBridgeCensusV1(null, twins, focused).filter((entry) => entry.focused).map((entry) => entry.instanceId)).toEqual(["9"]);
  });

  it("is published focused-first, which is the identity the DOM carries", () => {
    expect(shellHostSource).toContain("spawnedBridgeCensusV1(");
    expect(shellHostSource).toContain("sort((left, right) => Number(right.focused) - Number(left.focused))");
    expect(shellHostSource).toContain("data-semio-artifact-id={agentBridgeInstances[0]?.artifactRef}");
  });
});

describe("🚫️ a program the shell cannot open is refused on screen, never dropped", () => {
  it("names every reason in English and German, with the program's own label", () => {
    for (const reason of Object.keys(SPAWN_PROGRAM_REFUSAL_LABELS_V1) as (keyof typeof SPAWN_PROGRAM_REFUSAL_LABELS_V1)[]) {
      const en = spawnProgramRefusalNoticeTextV1(reason, "raster", "en");
      const de = spawnProgramRefusalNoticeTextV1(reason, "raster", "de");
      expect(en).toContain("raster");
      expect(de).toContain("raster");
      expect(de).not.toBe(en);
      expect(spawnProgramRefusalCodeV1(reason)).toBe(`shell.spawnProgram.${reason}`);
    }
    expect(spawnProgramRefusalNoticeTextV1("open-failed", "raster", "fr")).toBe(spawnProgramRefusalNoticeTextV1("open-failed", "raster", "en"));
  });

  it("REGRESSION: spawnProgram catches a failed guest instantiation and has no silent return", () => {
    const start = shellHostSource.indexOf("const spawnProgram = useCallback(");
    const body = shellHostSource.slice(start, shellHostSource.indexOf("[loadedPlugins, session, updateSpacePanel, hostApp]", start));
    expect(body).toContain('if (!pluginEntry) return refuse("program-not-installed");');
    expect(body).toContain('if (!session) return refuse("session-not-ready");');
    expect(body).toContain('return refuse("open-failed", openError);');
    expect(body).toContain('showTransientNoticeRef.current(spawnProgramRefusalNoticeTextV1(reason, program.breadcrumb.join(" · "), uiLocaleRef.current)');
    expect(body).not.toMatch(/if \(!pluginEntry \|\| !session\) return;/u);
  });
});

describe("🪟️ a spawned program's window bodies are read under the key they were requested by", () => {
  it("keys a window body by its WINDOW id, never its body key, when the two differ", () => {
    const cache: UiRefreshCache = new Map();
    const request = buildUiRefreshRequest({ kind: "full" }, [{ id: "dag-main", bodyKey: "dag.main" }], [], {} as never, cache);
    expect(request?.windows?.map((window) => window.key)).toEqual(["dag-main"]);
    applyUiRefreshResponseToCache(cache, { windows: [{ key: "dag-main", hash: "h1", value: { marker: 1 } }] } as never);
    expect(cache.get("window:dag-main")?.value).toEqual({ marker: 1 });
    expect(cache.get("window:dag.main")).toBeUndefined();
  });

  it("REGRESSION: `refreshSpawnedUi` reads the refresh cache by window id, the key `buildUiRefreshRequest` sends and the guest echoes", () => {
    const start = shellHostSource.indexOf("const refreshSpawnedUi = useCallback(");
    const body = shellHostSource.slice(start, shellHostSource.indexOf("[injectActiveUtility, uiLocale, uiTerminology]", start));
    expect(body).toContain("buildUiRefreshRequest({ kind: \"full\" }, windowKinds, [], fullViewState, cache)");
    expect(body).toContain("(cache.get(`window:${kind.id}`)?.value as BuiltNode | undefined) ?? pendingWindowUiNode()");
    expect(body).not.toContain("cache.get(`window:${kind.bodyKey}`)");
  });
});
