import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import Ajv from "ajv";
import picomatch from "picomatch";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import { createLiveFilesystemJournalV1 } from "../🟦️.ts";

const control = (signal = new AbortController().signal, budget: number | null = null) => ({ signal, remainingMilliseconds: () => budget, publish: async () => {}, yieldContinuation: async () => {} });
const globalIdentity = (path: string) => fixture.globalIdentity.includes(path.split("/").at(-1)!);
const excluded = (path: string) => path === "dist" || path.startsWith("dist/");
const hash = (paths: readonly string[]) => createHash("sha256").update(paths.map(path => path + "\0").join("")).digest("hex");

test("journal path schema refuses every neutral invalid source selection", () => {
 const ajv = new Ajv({ strict: true }); ajv.addSchema(schema);
 const validate = ajv.compile({ $ref: schema.$id + "#/$defs/Path" });
 for (const path of fixture.invalidPaths) expect(validate(path), JSON.stringify(path)).toBe(false);
});

test("warm selection observes only changed membership instead of all source identities", async () => {
 const files = Array.from({length: fixture.scaling.owners}, (_, owner) => ["owners/" + owner + "/package.json", ...Array.from({length: fixture.scaling.leaves}, (_, leaf) => "owners/" + owner + "/source" + leaf + ".ts")]).flat(), events: any[] = [];
 let identityReads = 0;
 const isGlobalIdentity = (path: string): boolean => { identityReads++; return globalIdentity(path); };
 const source = {watch() {return async () => {};}, async census() {return files;}, flush() {return events.splice(0);}, async reconcile(event: any) {return [event.path];}, async admitScope() {}};
 const journal = await createLiveFilesystemJournalV1({source, isGlobalIdentity, excluded, maximumPaths: files.length + 1, maximumEvents: 100}, control());
 const scope = {roots: ["owners/0"], files: []}, before = await journal.capture(scope, control()); identityReads = 0;
 events.push({path: "owners/99/source1.ts", type: "update"});
 const after = await journal.capture(scope, control());
 const matches = picomatch("owners/0/**");
 expect(after.selectedMembership.paths).toEqual(files.filter(path => matches(path)).sort());
 expect(after.globalIdentity.sha256).toBe(before.globalIdentity.sha256);
 expect(after.sequence).toBe(before.sequence);
 expect(identityReads).toBeLessThan(fixture.scaling.maximumWarmIdentityReads);
 await journal.close();
});

test("live journal neutral mutation cases agree with independent Ajv and Crypto", async () => {
 const validate = new Ajv({ strict: true }).compile(schema);
 for (const row of fixture.cases) {
  const files = new Set(fixture.initial), events: any[] = [], order: string[] = []; let receive: any;
  const source = { watch(callback: any) { order.push("watch"); receive = callback; return async () => { order.push("stop"); }; }, async census() { order.push("census"); return [...files]; }, flush() { return events.splice(0); }, async reconcile(event: any) { if(event.type === "delete") { for(const path of files) if(path === event.path || path.startsWith(event.path + "/")) files.delete(path); } else files.add(event.path); return [...files].filter(path => path === event.path || path.startsWith(event.path + "/")); }, async admitScope() {} };
  const journal = await createLiveFilesystemJournalV1({ source, isGlobalIdentity: globalIdentity, excluded, maximumPaths: 100, maximumEvents: 100 }, control());
  expect(order.slice(0, 2)).toEqual(["watch", "census"]);
  const before = await journal.capture(fixture.scope, control());
  expect(validate(before), JSON.stringify(validate.errors)).toBe(true);
  expect(before.globalIdentity.sha256).toBe(hash(before.globalIdentity.paths));
  expect(before.selectedMembership.sha256).toBe(hash(before.selectedMembership.paths));
  events.push(...row.events);
  expect(await journal.verify(before, fixture.scope, control()), row.id).toBe(row.accept);
  await journal.close(); expect(order.at(-1)).toBe("stop");
 }
});

test("queued journal queries cancel without retiring another caller", async () => {
 let enter!: () => void, release!: () => void, queries = 0;
 const entered = new Promise<void>(resolve => { enter = resolve; }), held = new Promise<void>(resolve => { release = resolve; });
 const source = { watch() { return async () => {}; }, async census() { return [...fixture.initial]; }, flush() { return []; }, async reconcile() { return []; }, async admitScope() { if (++queries === 1) { enter(); await held; } } };
 const journal = await createLiveFilesystemJournalV1({ source, isGlobalIdentity: globalIdentity, excluded, maximumPaths: 100, maximumEvents: 100 }, control());
 const first = journal.capture(fixture.scope, control()); await entered;
 const abort = new AbortController(), queued = journal.capture(fixture.scope, control(abort.signal)); abort.abort(Error("queued caller cancelled"));
 let timeout: ReturnType<typeof setTimeout> | undefined;
 try { await expect(Promise.race([queued, new Promise((_, reject) => { timeout = setTimeout(() => reject(Error("queued cancellation stuck")), 250); })])).rejects.toThrow("queued caller cancelled"); }
 finally { clearTimeout(timeout); release(); await first; await queued.catch(() => {}); await journal.close(); }
});

test("journal lifetime cancellation retires its watcher and refuses fresh callers", async () => {
 const owner = new AbortController(); let stops = 0;
 const source = {watch() {return async () => {stops++;};}, async census() {return [...fixture.initial];}, flush() {return [];}, async reconcile() {return [];}, async admitScope() {}};
 const journal = await createLiveFilesystemJournalV1({source, isGlobalIdentity: globalIdentity, excluded, maximumPaths: 100, maximumEvents: 100}, control(owner.signal));
 await journal.capture(fixture.scope, control()); owner.abort(Error("journal owner cancelled"));
 await expect(journal.capture(fixture.scope, control())).rejects.toThrow("journal owner cancelled");
 await journal.close(); expect(stops).toBe(1);
});

test("journal rejects a restarted epoch, unknown events and altered membership", async () => {
 let receive: any;
 const source = { watch(callback: any) { receive = callback; return async () => {}; }, async census() { return [...fixture.initial]; }, flush() { return []; }, async reconcile(event: any) { return [event.path]; }, async admitScope() {} };
 const options = { source, isGlobalIdentity: globalIdentity, excluded, maximumPaths: 100, maximumEvents: 100 }, first = await createLiveFilesystemJournalV1(options, control()), before = await first.capture(fixture.scope, control());
 expect(await first.verify({ ...before, selectedMembership: { ...before.selectedMembership, paths: ["forged.ts"] } }, fixture.scope, control())).toBe(false);
 receive(null, [{ path: "a/main.ts", type: "unknown" }]); expect(await first.verify(before, fixture.scope, control())).toBe(false); await first.close();
 const restarted = await createLiveFilesystemJournalV1(options, control()); expect(await restarted.verify(before, fixture.scope, control())).toBe(false); await restarted.close();
});

test("live custody refuses missed authority and preserves cancellation ports", async () => {
 let receive: any, stops = 0, failAdmission = false; const events: any[] = [];
 const source = { watch(callback: any) { receive = callback; return async () => { stops++; }; }, async census() { return [...fixture.initial]; }, flush() { return events.splice(0); }, async reconcile(event: any) { return [event.path]; }, async admitScope() { if(failAdmission) throw Error("symlink"); } };
 const journal = await createLiveFilesystemJournalV1({ source, isGlobalIdentity: globalIdentity, excluded, maximumPaths: 100, maximumEvents: 2 }, control());
 const before = await journal.capture(fixture.scope, control()), abort = new AbortController(); abort.abort(Error("original cancelled"));
 await expect(journal.capture(fixture.scope, control(abort.signal))).rejects.toThrow("original cancelled");
 await expect(journal.capture(fixture.scope, control(undefined, 0))).rejects.toThrow("deadline");
 failAdmission = true; await expect(journal.capture(fixture.scope, control())).rejects.toThrow("symlink"); failAdmission = false;
 receive(null, [{ path: "a/one", type: "create" }, { path: "a/two", type: "create" }, { path: "a/three", type: "create" }]);
 expect(await journal.verify(before, fixture.scope, control())).toBe(false);
 await journal.close(); await journal.close(); expect(stops).toBe(1);
 await expect(journal.capture(fixture.scope, control())).rejects.toThrow("closed");
});
