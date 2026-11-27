/** 🪦️ The journey's route: a window of a spawned program (`stdio-office-3::s.stdio.window.snapshot-details`) stays the active window while its program is replaced, and shell chrome (a window note, an automatic check-in) keeps addressing it. */
import { describe, expect, it } from "vitest";
import { buildNoteShellCommandAction, shellLabelTextV1 } from "../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import { liveChromeTargetV1 } from "../../🧱️elements/🏛️ShellHost/🖱️shell-actions/🧭️input-funnel/🎯️dispatch-target-and-view-state/🟦️.ts";
import { createSealedInstanceLedgerV1 } from "../../🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts";
import { keptOpenRouteV1 } from "../../🧱️elements/🏛️ShellHost/🗂️local-catalog/🟦️.ts";
import { settleProgramRetirementsV1 } from "../../🧱️elements/🏛️ShellHost/🚪️program-retirement/🟦️.ts";
import { windowProgramGoneV1 } from "../../🧱️elements/🏛️ShellHost/🪟️spawned-program/🟦️.ts";

describe("shell chrome never addresses a sealed instance", () => {
  const windowId = "stdio-office-3::s.stdio.window.snapshot-details";
  const handle = { pluginId: "stdio-office" };
  const hostApp = { id: "s", controllerId: "s", windowKinds: [], modes: [], panelTabs: [] };
  const officeApp = { id: "s.stdio.xlsx", controllerId: "office", windowKinds: [], modes: [], panelTabs: [] };
  const host = { pluginId: "s", instanceId: 1, app: hostApp, viewState: {} } as never;
  const loadedPlugins = [{ handle: { pluginId: "s" }, manifest: { apps: [hostApp] } }, { handle, manifest: { apps: [officeApp] } }] as never;
  const panel = { spawnedApps: [{ id: "stdio-office-3", pluginId: "stdio-office", appId: "s.stdio.xlsx", instanceId: 3, label: "Q" }] } as never;
  const note = buildNoteShellCommandAction("s", "shell.windowActivate", shellLabelTextV1("ui.shellCommand.windowActivate"), { windowId });
  const target = (sealed: ReturnType<typeof createSealedInstanceLedgerV1>, spawnedIds: readonly string[], action = note, active = windowId) =>
    liveChromeTargetV1({ hostMode: true, panel, loadedPlugins, activeWindowIdRef: { current: active }, spawnedIdsRef: { current: spawnedIds }, contributorInstancesRef: { current: new Map() }, session: host, action, sealed } as never);

  it("addresses the window's own live program instance while it is unsealed", () => {
    expect(target(createSealedInstanceLedgerV1(), ["stdio-office-3"])?.instanceId).toBe(3);
  });

  it("drops the note once the owning instance is sealed by a replacement, repeated notes included", () => {
    const sealed = createSealedInstanceLedgerV1();
    sealed.seal(handle, "stdio-office", 3);
    for (let attempt = 0; attempt < 6; attempt++) expect(target(sealed, ["stdio-office-3"])).toBeNull();
  });

  it("drops the note when the program has left the roster and only its window id remains", () => {
    expect(windowProgramGoneV1(windowId, [])).toBe(true);
    expect(windowProgramGoneV1(windowId, ["stdio-office-3"])).toBe(false);
    expect(windowProgramGoneV1("plain-window", [])).toBe(false);
    expect(target(createSealedInstanceLedgerV1(), [])).toBeNull();
  });

  it("still notes the live host for a host window", () => {
    const hostNote = buildNoteShellCommandAction("s", "shell.windowActivate", shellLabelTextV1("ui.shellCommand.windowActivate"), { windowId: "plain-window" });
    expect(target(createSealedInstanceLedgerV1(), ["stdio-office-3"], hostNote, "plain-window")?.instanceId).toBe(1);
  });
});

describe("a kept-file open never re-focuses a program whose retirement is in flight", () => {
  it("loads when no session is mounted, focuses a live one and waits for a sealed one", () => {
    expect(keptOpenRouteV1(undefined)).toBe("load");
    expect(keptOpenRouteV1({ sealed: false })).toBe("focus");
    expect(keptOpenRouteV1({ sealed: true })).toBe("await-retirement");
  });

  it("settles to a reload once the retirement removed the session, and to a focus if it was released", () => {
    const sealed = createSealedInstanceLedgerV1();
    const handle = {};
    const sessions = new Map<string, { sealed: boolean }>();
    const route = () => keptOpenRouteV1(sessions.get("doc") === undefined ? undefined : { sealed: sealed.sealed(handle, "stdio-office", 3) });
    sessions.set("doc", { sealed: false });
    sealed.seal(handle, "stdio-office", 3);
    expect(route()).toBe("await-retirement");
    sessions.delete("doc");
    expect(route()).toBe("load");
    sessions.set("doc", { sealed: false });
    sealed.unseal(handle, "stdio-office", 3);
    expect(route()).toBe("focus");
  });
});

describe("a document open waits for program retirements in flight", () => {
  it("resolves only after every running retirement, including ones that start meanwhile, has settled", async () => {
    const inFlight = new Set<Promise<unknown>>();
    const order: string[] = [];
    const retire = (name: string, ms: number, fail = false): Promise<unknown> => {
      const running = new Promise<void>((resolve, reject) => setTimeout(() => { order.push(name); fail ? reject(new Error(name)) : resolve(); }, ms));
      inFlight.add(running);
      void running.then(() => inFlight.delete(running), () => inFlight.delete(running));
      return running.catch(() => undefined);
    };
    void retire("first", 20);
    void retire("failing", 5, true);
    setTimeout(() => void retire("late", 10), 10);
    await settleProgramRetirementsV1(inFlight);
    expect(order).toEqual(["failing", "first", "late"]);
    expect(inFlight.size).toBe(0);
  });

  it("returns at once when nothing is retiring", async () => {
    await expect(settleProgramRetirementsV1(new Set())).resolves.toBeUndefined();
  });
});
