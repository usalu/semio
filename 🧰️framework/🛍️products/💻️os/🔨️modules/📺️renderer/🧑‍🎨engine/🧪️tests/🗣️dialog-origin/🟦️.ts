/** 🗣 Shell dialog origin: seek, tutorial drive and admitted-instance ownership. */
import { createElement } from "react";
import { applyPatch } from "fast-json-patch";
import { createTutorialClock } from "@semio-tech/ui-react";
import { makeEffectDispatchOne } from "../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🚀️effect-dispatch/🟦️.ts";
import { parseResolvedPluginViewState } from "../../../../../../../🔨️modules/🛂️manifest/🟦️.ts";
import type { LoadedProgramState } from "../../🧱️elements/🐚️Shell/📦️program-sessions/🟦️.ts";
import deepEqual from "fast-deep-equal";
import dialogOriginFixture from "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🗨️dialog-origin/🔣️.json";
import { createAdmittedShellInstanceV1, shellDialogOriginIsCurrentV1, shellDialogOriginV1, shellEffectOwnerIsCurrentV1, shellEffectSourceIsCurrentV1, type ShellDialogOriginV1 } from "../../🧱️elements/🏛️ShellHost/🗨️dialog-origin/🟦️.ts";
import admittedInstanceFixture from "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🗨️dialog-origin/🛂️admission/🔣️.json";
import { OwnedShellDialog, type OwnedShellDialogProps } from "../../🧱️elements/🏛️ShellHost/🗨️dialog-origin/🌐️browser/🟦️.tsx";
import tutorialRunFixture from "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🗨️dialog-origin/🎥️tutorial/🔣️.json";
import { OwnedTutorialRunV1, TutorialDriveV1, runPausedTutorialSeekV1 } from "../../🧱️elements/🏛️ShellHost/🗨️dialog-origin/🎥️tutorial/🟦️.ts";
import tutorialSeekFixture from "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🗨️dialog-origin/🎥️tutorial/⏩️seek/🔣️.json";
import tutorialSerialFixture from "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🗨️dialog-origin/🎥️tutorial/🧵️serial/🔣️.json";
import { fireEvent, render } from "@semio-tech/ui-react/test";
import { describe, expect, it, vi } from "vitest";
import { rendererExport } from "../🧰️engine-contract-helpers/🟦️.ts";

/** 🗣️ The resolver every `makeEffectDispatchOne` owner passes — the shell's `resolvedTargetViewState`, reduced to what a test session needs: both host preferences always stamped. */
const resolvedViewStateFixture = (session: { readonly viewState?: unknown }) => parseResolvedPluginViewState({ ...(session.viewState as Record<string, unknown> | undefined), locale: "en", terminology: "native" }) as never;

describe("Shell dialog origin", () => {
  it("serializes a seek behind the admitted director write and drains before retirement completes", async () => {

    for (const row of tutorialSerialFixture.cases) {
      const drive = new TutorialDriveV1();
      let owner = true;
      let cursor = 0;
      const writes: string[] = [];
      let resolve!: () => void;
      const gate = new Promise<void>((done) => {
        resolve = done;
      });
      const director = drive.enqueue(
        () => owner,
        async (token) => {
          writes.push("M");
          await gate;
          if (owner && drive.accepts(token)) cursor = 100;
        },
      );
      await Promise.resolve();
      expect(writes).toEqual(["M"]);
      const reconcile = async (token: number) => {
        expect(cursor).toBe(100);
        if (owner && drive.accepts(token)) {
          writes.push("inverse-M");
          cursor = 0;
        }
      };
      const stale = vi.fn(async () => {});
      const seek = drive.enqueue(() => owner, row.replace ? stale : reconcile);
      const latest = row.replace ? drive.enqueue(() => owner, reconcile) : Promise.resolve();
      if (row.close) {
        owner = false;
        drive.retire();
      }
      let drained = false;
      const drain = drive.drain().then(() => {
        drained = true;
      });
      await Promise.resolve();
      expect(drained).toBe(false);
      expect(writes).toEqual(["M"]);
      expect(cursor).toBe(0);
      resolve();
      await drain;
      expect(deepEqual({ writes, cursor }, { writes: row.writes, cursor: row.cursor })).toBe(true);
      expect(drive.busy).toBe(false);
      await Promise.all([director, seek, latest]);
      expect(stale).not.toHaveBeenCalled();
      expect(deepEqual({ writes, cursor }, { writes: row.writes, cursor: row.cursor })).toBe(true);
      expect(drive.busy).toBe(false);
    }
  });

  it("restores the original tutorial snapshot only after the admitted write drains", async () => {
    const drive = new TutorialDriveV1();
    let documentValue = 0;
    let resolve!: () => void;
    const gate = new Promise<void>((done) => {
      resolve = done;
    });
    const restore = vi.fn(async () => {
      documentValue = 0;
    });
    const run = new OwnedTutorialRunV1("tour", dialogOriginFixture.owner, () => true, {
      read: async () => ({ pack: new Uint8Array([0]), spr: new Uint8Array([0]) }),
      drain: () => drive.drain(),
      restore,
    });
    await run.start();
    const write = drive.enqueue(
      () => run.ready,
      async () => {
        await gate;
        documentValue = 1;
      },
    );
    await Promise.resolve();
    drive.retire();
    const stopping = run.stop();
    expect(restore).not.toHaveBeenCalled();
    resolve();
    await Promise.all([write, stopping]);
    expect(restore).toHaveBeenCalledTimes(1);
    expect(documentValue).toBe(0);
    expect(run.ready).toBe(false);
  });

  it("drains a promoted delayed successor and rejects direct token replacement of admitted writes", async () => {
    const drive = new TutorialDriveV1();
    let finishFirst!: () => void;
    let finishSecond!: () => void;
    let startSecond!: () => void;
    const firstGate = new Promise<void>((resolve) => {
      finishFirst = resolve;
    });
    const secondGate = new Promise<void>((resolve) => {
      finishSecond = resolve;
    });
    const secondStarted = new Promise<void>((resolve) => {
      startSecond = resolve;
    });
    const first = drive.enqueue(
      () => true,
      () => firstGate,
    );
    await Promise.resolve();
    expect(() => drive.claim()).toThrow("Cannot replace an admitted tutorial document drive");
    const second = drive.enqueue(
      () => true,
      async () => {
        startSecond();
        await secondGate;
      },
    );
    let drained = false;
    const drain = drive.drain().then(() => {
      drained = true;
    });
    finishFirst();
    await secondStarted;
    expect(drained).toBe(false);
    finishSecond();
    await drain;
    expect(drive.busy).toBe(false);
    await Promise.all([first, second]);
  });

  it("quarantines a failed physical drive until explicit retirement and never runs queued reconciliation", async () => {
    const drive = new TutorialDriveV1();
    let reject!: (error: Error) => void;
    const gate = new Promise<void>((_resolve, fail) => {
      reject = fail;
    });
    const first = drive
      .enqueue(
        () => true,
        () => gate,
      )
      .catch((error) => error);
    await Promise.resolve();
    const reconcile = vi.fn(async () => {});
    const second = drive.enqueue(() => true, reconcile).catch((error) => error);
    const failure = new Error("uncertain physical write");
    reject(failure);
    expect(await first).toBe(failure);
    expect(await second).toBe(failure);
    await expect(drive.enqueue(() => true, reconcile)).rejects.toBe(failure);
    expect(reconcile).not.toHaveBeenCalled();
    drive.retire();
    await drive.enqueue(() => true, reconcile);
    expect(reconcile).toHaveBeenCalledTimes(1);
  });

  it("pauses the actual playback clock across a delayed owned seek without replaying its mutation", async () => {

    for (const row of tutorialSeekFixture.cases) {
      let frame: FrameRequestCallback | undefined;
      vi.stubGlobal("requestAnimationFrame", (next: FrameRequestCallback) => {
        frame = next;
        return 1;
      });
      vi.stubGlobal("cancelAnimationFrame", () => {});
      const clock = createTutorialClock(1000);
      try {
        if (row.playing) clock.play();
        const drive = new TutorialDriveV1();
        let owner = true;
        let wantsPlaying = row.requested;
        let mutations = 0;
        let playhead = 0;
        let resolve!: () => void;
        const delayed = new Promise<void>((done) => {
          resolve = done;
        });
        clock.subscribe(() => {
          if (clock.isPlaying() && playhead !== clock.getTimeMs()) mutations++;
        });
        const seek = runPausedTutorialSeekV1(
          clock,
          drive,
          () => owner,
          () => wantsPlaying,
          async (token) => {
            mutations++;
            await delayed;
            if (!owner || !drive.accepts(token)) return;
            playhead = 200;
            clock.seek(playhead);
          },
        );
        expect(clock.isPlaying()).toBe(false);
        await Promise.resolve();
        frame?.(100);
        frame?.(200);
        expect(mutations).toBe(1);
        expect(playhead).toBe(0);
        if (row.interrupt === "pause") wantsPlaying = false;
        if (row.interrupt === "play") wantsPlaying = true;
        if (row.interrupt === "close") {
          owner = false;
          drive.retire();
        }
        resolve();
        await seek;
        const observed = { mutations, playhead, resumed: clock.isPlaying() };
        expect(deepEqual(observed, { mutations: row.mutations, playhead: row.playhead, resumed: row.resumed })).toBe(true);
        expect(drive.active).toBe(false);
      } finally {
        clock.dispose();
        vi.unstubAllGlobals();
      }
    }
  });

  it("carries current Play intent through coalesced seeks without executing their superseded target", async () => {
    vi.stubGlobal("requestAnimationFrame", () => 1);
    vi.stubGlobal("cancelAnimationFrame", () => {});
    const clock = createTutorialClock(1000);
    const drive = new TutorialDriveV1();
    const writes: number[] = [];
    let resolve!: () => void;
    const gate = new Promise<void>((done) => {
      resolve = done;
    });
    try {
      clock.play();
      const first = runPausedTutorialSeekV1(
        clock,
        drive,
        () => true,
        () => true,
        async () => {
          writes.push(200);
          await gate;
          clock.seek(200);
        },
      );
      await Promise.resolve();
      const obsolete = runPausedTutorialSeekV1(
        clock,
        drive,
        () => true,
        () => true,
        async () => {
          writes.push(300);
          clock.seek(300);
        },
      );
      const latest = runPausedTutorialSeekV1(
        clock,
        drive,
        () => true,
        () => true,
        async () => {
          writes.push(500);
          clock.seek(500);
        },
      );
      expect(clock.isPlaying()).toBe(false);
      resolve();
      await Promise.all([first, obsolete, latest]);
      expect(writes).toEqual([200, 500]);
      expect(clock.getTimeMs()).toBe(500);
      expect(clock.isPlaying()).toBe(true);
      expect(drive.busy).toBe(false);
    } finally {
      clock.dispose();
      vi.unstubAllGlobals();
    }
  });

  it("releases cancelled tutorial drive ownership without clearing a newer seek or tween", () => {

    for (const row of tutorialRunFixture.drives) {
      const drive = new TutorialDriveV1();
      const tokens = new Map<string, number>();
      let oracle: string | null = null;
      const states: boolean[] = [];
      for (const event of row.events) {
        if (event === "retire") {
          drive.retire();
          oracle = null;
        } else if (event.startsWith("claim")) {
          const key = event.slice(-1);
          tokens.set(key, drive.claim());
          oracle = key;
        } else {
          const key = event.slice(-1);
          drive.release(tokens.get(key)!);
          if (oracle === key) oracle = null;
        }
        expect(drive.active, row.id).toBe(!deepEqual(oracle, null));
        states.push(drive.active);
      }
      expect(states, row.id).toEqual(row.active);
      expect(drive.active, row.id).toBe(false);
    }
  });

  it("admits an explicit target handoff once and retires a target created after its source expired", async () => {

    for (const row of admittedInstanceFixture.cases) {
      let admitted = row.before;
      const target = { instanceId: 9 };
      const create = vi.fn(async () => {
        admitted = row.after;
        return target;
      });
      const retire = vi.fn(async () => {});
      const result = await createAdmittedShellInstanceV1(() => admitted, create, retire);
      expect(deepEqual(result, target), row.id).toBe(row.before && row.after);
      expect(result !== null, row.id).toBe(row.accepted);
      expect(create, row.id).toHaveBeenCalledTimes(row.creates);
      expect(retire, row.id).toHaveBeenCalledTimes(row.retires);
      if (row.retires === 1) expect(retire).toHaveBeenCalledWith(target);
    }
  });

  it("admits a spawned program's own progress and completion passes only when the primary session presents them", () => {
    const program = { pluginId: dialogOriginFixture.owner.pluginId, instanceId: dialogOriginFixture.owner.sessionInstanceId, app: { id: dialogOriginFixture.owner.appId, controllerId: dialogOriginFixture.owner.controllerId } };
    const primary = { pluginId: "host", instanceId: 1, app: { id: "host", controllerId: "host" } };
    const spawned = [{ pluginId: program.pluginId, appId: program.app.id, instanceId: program.instanceId }];
    const current = { presentation: shellDialogOriginV1(primary, []), mounted: shellDialogOriginV1(program, []), primary, spawned };
    expect(shellEffectOwnerIsCurrentV1({ presentation: shellDialogOriginV1(primary, []), source: shellDialogOriginV1(program, []) }, current), "presented by the primary session, sourced by the live program").toBe(true);
    expect(shellEffectOwnerIsCurrentV1({ presentation: shellDialogOriginV1(program, []), source: shellDialogOriginV1(program, []) }, current), "the defect: a program presenting its own pass is never current").toBe(false);
    expect(shellEffectOwnerIsCurrentV1({ presentation: shellDialogOriginV1(primary, []), source: shellDialogOriginV1(program, []) }, { ...current, spawned: [] }), "a retired program's pass is dropped").toBe(false);
    expect(shellEffectOwnerIsCurrentV1({ presentation: shellDialogOriginV1(primary, []), source: shellDialogOriginV1(primary, []) }, { ...current, mounted: shellDialogOriginV1(primary, []) }), "the primary session's own lane").toBe(true);
    expect(
      shellEffectOwnerIsCurrentV1({ presentation: shellDialogOriginV1(primary, []), source: shellDialogOriginV1(program, []) }, { ...current, presentation: shellDialogOriginV1({ ...primary, instanceId: 2 }, []) }),
      "a replaced primary session drops every pass it presented",
    ).toBe(false);
  });

  it("retires a spawned source independently of its still-visible primary presentation owner", async () => {
    const source = dialogOriginFixture.owner;
    const primary = { pluginId: "host", instanceId: 1, app: { id: "host", controllerId: "host" } };
    const spawned = [{ pluginId: source.pluginId, appId: source.appId, instanceId: source.sessionInstanceId }];
    for (const row of dialogOriginFixture.cases) {
      const mounted = applyPatch(structuredClone(source), row.patch as Parameters<typeof applyPatch>[1]).newDocument;
      expect(shellEffectSourceIsCurrentV1(source, mounted, primary, spawned), row.id).toBe(row.accepted);
    }
    expect(shellEffectSourceIsCurrentV1(source, source, primary, [])).toBe(false);
    expect(shellEffectSourceIsCurrentV1(source, source, primary, [{ ...spawned[0]!, appId: "replacement" }])).toBe(false);
    expect(shellEffectSourceIsCurrentV1(source, source, primary, [{ ...spawned[0]!, pluginId: "replacement" }])).toBe(false);
    const applyEffects = vi.fn(async () => {});
    let members = spawned;
    let complete!: (response: { requestedEffects: readonly unknown[] }) => void;
    const plugin = {
      handle: {
        handleAction: () =>
          new Promise((done) => {
            complete = done;
          }),
      },
    } as unknown as LoadedProgramState;
    const session = { pluginId: source.pluginId, instanceId: source.sessionInstanceId, app: { id: source.appId, controllerId: source.controllerId, modes: [], windowKinds: [], commands: [] }, viewState: {} } as unknown as Parameters<
      typeof makeEffectDispatchOne
    >[1];
    const pending = makeEffectDispatchOne(plugin, session, applyEffects, () => shellEffectSourceIsCurrentV1(source, source, primary, members), resolvedViewStateFixture)("lateAction");
    members = [];
    complete({ requestedEffects: [{ navigate: { uri: "/wrong" } }, { setPanel: { panelJson: "{}" } }] });
    await pending;
    expect(applyEffects).not.toHaveBeenCalled();
  });

  it("keeps tutorial snapshots on their exact run and never restores into a replacement session", async () => {

    for (const row of tutorialRunFixture.cases) {
      let current = structuredClone(dialogOriginFixture.owner);
      let epoch = 1;
      const restore = vi.fn(async () => {});
      let resolve!: (snapshot: { pack: Uint8Array; spr: Uint8Array }) => void;
      const run = new OwnedTutorialRunV1("tour", dialogOriginFixture.owner, () => epoch === 1 && shellDialogOriginIsCurrentV1(dialogOriginFixture.owner, current), {
        read: () =>
          new Promise<{ pack: Uint8Array; spr: Uint8Array }>((done) => {
            resolve = done;
          }),
        drain: async () => {},
        restore,
      });
      const started = run.start();
      let oracleCurrent = true;
      let oracleClosed = false;
      let oracleSnapshot = false;
      let oracleStarted = false;
      let oracleRestores = 0;
      for (const event of row.events) {
        if (event === "switch") {
          current = applyPatch(current, [{ op: "replace", path: "/document/clientInstanceId", value: "client-b" }]).newDocument;
          oracleCurrent = deepEqual(dialogOriginFixture.owner, current);
        } else if (event === "replace") {
          epoch = 2;
          oracleCurrent = false;
        } else if (event === "resolve") {
          resolve({ pack: Uint8Array.of(1, 2), spr: Uint8Array.of(3) });
          oracleStarted = oracleCurrent && !oracleClosed;
          oracleSnapshot = oracleStarted;
          expect(await started, row.id).toBe(oracleStarted);
        } else {
          if (!oracleClosed && oracleCurrent && oracleSnapshot) oracleRestores += 1;
          oracleClosed = true;
          await run.stop();
        }
      }
      expect(await started, row.id).toBe(row.started);
      expect(oracleRestores, row.id).toBe(row.restores);
      expect(restore, row.id).toHaveBeenCalledTimes(row.restores);
      await run.stop();
      expect(restore, row.id).toHaveBeenCalledTimes(row.restores);
      expect(run.isCurrent()).toBe(false);
    }
  });

  it("remounts staged fields and rejects old callbacks before they can dispatch into or close a replacement", () => {
    const a = { openingId: 1, dialogId: "createArtifact", origin: dialogOriginFixture.owner, seedArgs: { name: "A seed" } };
    const b = { openingId: 2, dialogId: "createArtifact", origin: { ...dialogOriginFixture.owner, document: { ...dialogOriginFixture.owner.document, clientInstanceId: "client-b" } }, seedArgs: { name: "B seed" } };
    let current: ShellDialogOriginV1 | null = a.origin;
    let live: number | null = a.openingId;
    const dispatch = vi.fn();
    const props = (owner: typeof a): OwnedShellDialogProps => ({
      owner,
      dialog: {
        id: "createArtifact",
        title: { native: { en: "Create Artifact", de: "Artefakt erstellen" } },
        args: [{ id: "name", label: { native: { en: "Name", de: "Name" } }, required: true, schema: { kind: "string", options: [] } }],
        submitAction: "createArtifact",
        submitLabel: { native: { en: "Create", de: "Erstellen" } },
        cancelAction: "cancelArtifact",
      },
      renderField: (_def, value, onChange, field) =>
        createElement("input", { id: field.id, "aria-labelledby": field.labelledBy, required: field.required, value: String(value ?? ""), onChange: (event: { target: { value: string } }) => onChange(event.target.value) }),
      isCurrent: (origin) => shellDialogOriginIsCurrentV1(origin, current),
      close: (openingId) => {
        if (live !== openingId) return false;
        live = null;
        return true;
      },
      dispatch,
    });
    const firstCallbacks = OwnedShellDialog(props(a))!.props as { onSubmit: (args: Record<string, unknown>) => void; onCancel: () => void };
    const view = render(createElement(OwnedShellDialog, props(a)));
    fireEvent.change(view.getByRole("textbox", { name: "Name" }), { target: { value: "A staged" } });
    expect((view.getByRole("textbox", { name: "Name" }) as HTMLInputElement).value).toBe("A staged");
    current = b.origin;
    live = b.openingId;
    view.rerender(createElement(OwnedShellDialog, props(b)));
    expect((view.getByRole("textbox", { name: "Name" }) as HTMLInputElement).value).toBe("B seed");
    firstCallbacks.onSubmit({ name: "A staged" });
    firstCallbacks.onCancel();
    expect(dispatch).not.toHaveBeenCalled();
    expect(live).toBe(b.openingId);
    const secondCallbacks = OwnedShellDialog(props(b))!.props as { onSubmit: (args: Record<string, unknown>) => void };
    current = null;
    secondCallbacks.onSubmit({ name: "B stale" });
    expect(dispatch).not.toHaveBeenCalled();
    expect(live).toBeNull();
    current = b.origin;
    live = b.openingId;
    secondCallbacks.onSubmit({ name: "B accepted" });
    secondCallbacks.onSubmit({ name: "B duplicate" });
    expect(dispatch).toHaveBeenCalledExactlyOnceWith("createArtifact", b.origin, { name: "B accepted" });
    view.unmount();
  });

  it("drops delayed effect replies and scheduled invocations after the owning mount retires", async () => {
    let current = true;
    let resolve!: (value: { requestedEffects: readonly unknown[] }) => void;
    const handleAction = vi.fn(
      () =>
        new Promise<{ requestedEffects: readonly unknown[] }>((done) => {
          resolve = done;
        }),
    );
    const plugin = { handle: { handleAction } } as unknown as LoadedProgramState;
    const session = { pluginId: "space", instanceId: 7, app: { id: "space.editor", controllerId: "space.editor", windowKinds: [], commands: [], modes: [] }, viewState: {} } as unknown as Parameters<typeof makeEffectDispatchOne>[1];
    const applyEffects = vi.fn(async () => {});
    const invoke = makeEffectDispatchOne(plugin, session, applyEffects, () => current, resolvedViewStateFixture);
    const pending = invoke("openCreateArtifact", {});
    expect(handleAction).toHaveBeenCalledTimes(1);
    current = false;
    resolve({ requestedEffects: [{ openDialog: { dialogId: "createArtifact" } }] });
    await pending;
    await invoke("openCreateArtifact", {});
    expect(applyEffects).not.toHaveBeenCalled();
    expect(handleAction).toHaveBeenCalledTimes(1);
  });

  it("rejects every changed origin with the same result as the independent JSON Patch and equality oracle", () => {
    expect(rendererExport("ShellDialogOriginV1")(dialogOriginFixture.owner)).toBe(true);
    for (const row of dialogOriginFixture.cases) {
      const current = applyPatch(structuredClone(dialogOriginFixture.owner), row.patch as Parameters<typeof applyPatch>[1]).newDocument;
      expect(deepEqual(dialogOriginFixture.owner, current), row.id).toBe(row.accepted);
      expect(shellDialogOriginIsCurrentV1(dialogOriginFixture.owner, current), row.id).toBe(row.accepted);
    }
    expect(shellDialogOriginIsCurrentV1(dialogOriginFixture.owner, null)).toBe(false);
    expect(shellDialogOriginIsCurrentV1(null, dialogOriginFixture.owner)).toBe(false);
  });

  it("requires a unique exact mounted session and supports documentless dialogs without lending document authority", () => {
    const source = dialogOriginFixture.owner;
    const session = { pluginId: source.pluginId, instanceId: source.sessionInstanceId, app: { id: source.appId, controllerId: source.controllerId } };
    const mount = { session, ...source.document };
    expect(shellDialogOriginV1(session, [mount])).toEqual(source);
    expect(shellDialogOriginV1(session, [mount, mount])).toBeNull();
    expect(shellDialogOriginV1(null, [mount])).toBeNull();
    const unmounted = shellDialogOriginV1(session, []);
    expect(unmounted).toEqual({ ...source, document: null });
    expect(shellDialogOriginIsCurrentV1(unmounted, structuredClone(unmounted))).toBe(true);
    expect(shellDialogOriginIsCurrentV1(source, unmounted)).toBe(false);
  });
});
