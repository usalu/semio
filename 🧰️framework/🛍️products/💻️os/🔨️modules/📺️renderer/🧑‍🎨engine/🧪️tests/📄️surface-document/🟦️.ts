/** 📄️ Full archive handoff traces preserve the predecessor until its successor is ready. */
import { expect,it } from "vitest";
import { applyPatch } from "fast-json-patch";
import fixture from "../../🧱️elements/🏛️ShellHost/🔀️surface-switch/📄️document/🧪️tests/🧫️fixtures/🔣️.json";
import { runSessionAppSwitchV1 } from "../../🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts";
import { prepareDocumentSurfaceV1, restoreDocumentSurfaceV1 } from "../../🧱️elements/🏛️ShellHost/🔀️surface-switch/📄️document/🟦️.ts";
for (const law of fixture.cases) it(law.name,async () => {
  const events: string[] = [];
  let current = !("initiallyCurrent" in law) || law.initiallyCurrent === true;
  let target: unknown = null;
  let attached = false;
  let released = false;
  const source = JSON.stringify(fixture.archive);
  const step = async (name: string) => {
    events.push(name);
    if ("failure" in law && law.failure === name) throw new Error(name);
    if ("staleAfter" in law && law.staleAfter === name) current = false;
  };
  const result = prepareDocumentSurfaceV1({
    current: () => current,
    capture: async () => { await step("capture"); return fixture.archive; },
    create: async () => { await step("create"); return 17; },
    restore: async (id,archive) => { expect(id).toBe(17); await step("restore"); target = JSON.parse(JSON.stringify(archive)); },
    attach: async (id) => { expect(id).toBe(17); await step("attach"); attached = true; },
    release: async (id) => { expect(id).toBe(17); events.push("release"); released = true; attached = false; target = null; },
  });
  if (law.ready) {
    expect(await result).toBe(17);
    const oracle = applyPatch({},[{ op: "add",path: "/document",value: fixture.archive }],true,false).newDocument;
    expect({ document: target }).toEqual(oracle);
    expect(attached).toBe(true);
    expect(released).toBe(false);
  } else {
    await expect(result).rejects.toBeInstanceOf(Error);
    expect(attached).toBe(false);
    expect(target).toBeNull();
  }
  expect(events).toEqual(law.events);
  expect(JSON.stringify(fixture.archive)).toBe(source);
});

it("does not return a successor while restoration is pending",async () => {
  let enter!: () => void,finish!: () => void;
  const entered = new Promise<void>(resolve => { enter = resolve; });
  const pending = new Promise<void>(resolve => { finish = resolve; });
  let attached = false,ready = false;
  const preparation = prepareDocumentSurfaceV1({
    current: () => true,
    capture: async () => fixture.archive,
    create: async () => 17,
    restore: async () => { enter(); await pending; },
    attach: async () => { attached = true; },
    release: async () => { throw new Error("unexpected release"); },
  }).then(value => { ready = true; return value; });
  await entered;
  expect(ready).toBe(false);
  expect(attached).toBe(false);
  finish();
  expect(await preparation).toBe(17);
  expect(attached).toBe(true);
});
it("preserves preparation and cleanup failures together",async () => {
  const preparationError = new Error("restore refused");
  const releaseError = new Error("release refused");
  const preparation = prepareDocumentSurfaceV1({
    current: () => true,
    capture: async () => fixture.archive,
    create: async () => 17,
    restore: async () => { throw preparationError; },
    attach: async () => { throw new Error("unexpected attach"); },
    release: async () => { throw releaseError; },
  });
  await expect(preparation).rejects.toMatchObject({ errors: [preparationError,releaseError] });
});

for (const refused of [false,true]) it(`prepares the archive inside the real switch gate (${refused ? "restore refused" : "success"})`,async () => {
  const source = { pluginId: "draw",instanceId: 1,app: { id: "editor" },viewState: {} };
  let current = source;
  let sealed = false;
  let retired = false;
  const documents = new Map<number,unknown>([[1,fixture.archive]]);
  const events: string[] = [];
  const result = runSessionAppSwitchV1({
    session: source,
    resolveApp: () => ({ id: "viewer" }),
    appId: app => app.id,
    quiesce: async () => ({ settled: true,pending: 0 }),
    seal: () => { sealed = true; },
    unseal: () => { sealed = false; },
    createInstance: () => prepareDocumentSurfaceV1({
      current: () => current === source,
      capture: async () => { events.push("capture"); return documents.get(1); },
      create: async () => { events.push("create"); documents.set(2,null); return 2; },
      restore: async (id,archive) => { events.push("restore"); if (refused) throw new Error("load refused"); documents.set(id,archive); },
      attach: async () => { events.push("attach"); },
      release: async id => { events.push("release"); documents.delete(id); },
    }),
    retire: async previous => { expect(documents.get(2)).toEqual(fixture.archive); events.push("retire"); retired = true; documents.delete(previous.instanceId); },
    defaultViewState: () => ({}),
    publish: next => { expect(documents.get(next.instanceId)).toEqual(fixture.archive); events.push("publish"); current = next; },
    seedLayout: () => {},
    refresh: async () => {},
  },{ pluginId: "draw",appId: "viewer" });
  if (refused) {
    await expect(result).rejects.toThrow("load refused");
    expect(current).toBe(source);
    expect(retired).toBe(false);
    expect(sealed).toBe(false);
    expect(documents.get(1)).toEqual(fixture.archive);
    expect(documents.has(2)).toBe(false);
    expect(events).toEqual(["capture","create","restore","release"]);
  } else {
    expect((await result).status).toBe("switched");
    expect(current.instanceId).toBe(2);
    expect(documents.get(2)).toEqual(fixture.archive);
    expect(events).toEqual(["capture","create","restore","attach","retire","publish"]);
  }
});

it("restores through a task that reports the load's progress, ends its row and rolls the unpublished successor back when cancelled", async () => {
  const progress: [number, number | null][] = [];
  const control = new AbortController();
  let finished = 0;
  const released: number[] = [];
  let attached = false;
  const task = { signal: control.signal, progress: (completed: number, total: number | null) => { progress.push([completed, total]); }, finish: () => { finished += 1; } };
  const preparation = prepareDocumentSurfaceV1({
    current: () => true,
    capture: async () => fixture.archive,
    create: async () => 17,
    restore: (_id, archive) => restoreDocumentSurfaceV1(task, async (captured, signal, report) => {
      expect(captured).toBe(archive);
      report({ completed: 1, total: 0 });
      report({ completed: 5, total: 9 });
      await new Promise<void>((_resolve, reject) => { signal.addEventListener("abort", () => reject(signal.reason), { once: true }); });
    }, archive),
    attach: async () => { attached = true; },
    release: async (id) => { released.push(id); },
  });
  const settled = preparation.then(() => null, (error: unknown) => error);
  for (let turn = 0; turn < 50 && progress.length < 2; turn += 1) await new Promise(resolve => setTimeout(resolve, 0));
  expect(progress).toEqual([[1, null], [5, 9]]);
  expect(finished).toBe(0);
  control.abort();
  expect(await settled).toMatchObject({ name: "AbortError" });
  expect(finished).toBe(1);
  expect(released).toEqual([17]);
  expect(attached).toBe(false);
});

it("ends the task row when the load completes", async () => {
  let finished = 0;
  const task = { signal: new AbortController().signal, progress: () => {}, finish: () => { finished += 1; } };
  await restoreDocumentSurfaceV1(task, async () => {}, fixture.archive);
  expect(finished).toBe(1);
});
