import { registerIsolatedBuildTestV1 } from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import { expect } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { lawStoragePolicyV1 } from "../../../🧪️tests/🦀️cargo/🟦️.ts";
import { NATIVE_BUILD_DRIVER_POLICY_V1 } from "../../🟦️.ts";
import { ConcurrentEditErrorV1, churnProtocolV1, installRestartableFailureHandlerV1 } from "../../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import { artifacts, header, stubCargo, stubWorkspace, runStub } from "../🎭️stub/🟦️.ts";

registerIsolatedBuildTestV1(import.meta.path, "a failed cargo exit with no failed unit surfaces cargo's own stderr in the error and the report", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "g-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release");
  let report = "";
  await expect(runStub(root, { version: 1, passes: [pass] }, cargo, {}, "stderr-exit", ({ held }) => { const runs = join(held.root, "cache", NATIVE_BUILD_DRIVER_POLICY_V1.warmRoot); for (const run of readdirSync(runs)) { const file = join(runs, run, NATIVE_BUILD_DRIVER_POLICY_V1.stores.report, "report.md"); if (existsSync(file)) report = readFileSync(file, "utf8"); } })).rejects.toThrow(/cargo exit 101[\s\S]*build broker unreachable: All pipe instances are busy\./u);
  expect(report).toContain("## Cargo diagnostics");
  expect(report).toContain("build broker unreachable: All pipe instances are busy.");
  checkpoint();
}, /cargo exit 101/u);

registerIsolatedBuildTestV1(import.meta.path, "an unlinked component fails the native build without deleting unowned checkout bytes and still writes the report", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "f-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const stale = join(pass.packages[1]!.manifest,"..","unowned-stale");
  mkdirSync(stale, { recursive: true });
  writeFileSync(join(stale, "comp_b.wasm"), header);
  let reported = false;
  await expect(runStub(root, { version: 1, passes: [pass] }, cargo, {}, "omit-last", ({ held }) => { const runs = join(held.root, "cache", NATIVE_BUILD_DRIVER_POLICY_V1.warmRoot); reported = readdirSync(runs).some((run) => existsSync(join(runs, run, NATIVE_BUILD_DRIVER_POLICY_V1.stores.report, "report.md"))); })).rejects.toThrow(/Native build failed: comp-b/u);
  expect(readFileSync(join(stale,"comp_b.wasm"))).toEqual(header);
  expect(reported).toBe(true);
  checkpoint();
}, /Native build failed: comp-b/u);

registerIsolatedBuildTestV1(import.meta.path, "a failed cargo pass whose failing unit's inputs did not change stays a hard failure", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "e-")), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const failure = await runStub(root, { version: 1, passes: [pass] }, stubCargo(root), {}, "error-last").then(() => undefined, (error: unknown) => error);
  expect(failure).not.toBeInstanceOf(ConcurrentEditErrorV1);
  expect(String((failure as Error).message)).toMatch(/Native build failed: comp-b/u);
  checkpoint();
}, /Native build failed: comp-b/u);

registerIsolatedBuildTestV1(import.meta.path, "a failed cargo pass whose failing unit's inputs changed after Cargo started is a restartable concurrent edit naming the file", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  installRestartableFailureHandlerV1(() => undefined);
  const root = mkdtempSync(join(artifacts, "e-")), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const failure = await runStub(root, { version: 1, passes: [pass] }, stubCargo(root), {}, "error-touch").then(() => undefined, (error: unknown) => error) as { code?: string; reason?: string; path?: string } | undefined;
  expect(failure).toBeInstanceOf(ConcurrentEditErrorV1);
  expect(failure).toMatchObject({ code: churnProtocolV1().concurrentEditCode, reason: "bytes", path: join(dirname(pass.manifest), "comp-b", "lib.rs") });
  checkpoint();
}, /modified after Cargo started/u);

registerIsolatedBuildTestV1(import.meta.path, "a batch the free storage cannot cover above the hard reserve is refused before Cargo starts", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "h-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release");
  const lines: string[] = [];
  await expect(runStub(root, { version: 1, passes: [pass] }, cargo, { storageReserveBytes: lawStoragePolicyV1.stubImageBytes * 2, log: line => lines.push(line) })).rejects.toThrow(/Native build storage refused: .*reserve/u);
  expect(lines.findLast(line => line.includes("failure(s) in "))).toContain("1 failure(s) in ");
  expect(existsSync(cargo.calls)).toBe(false);
  checkpoint();
}, /Native build storage refused/u);

registerIsolatedBuildTestV1(import.meta.path, "cancellation stops every Cargo tree and leaves the Original admitting a successor", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "x-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release"), controller = new AbortController();
  const cancelled = runStub(root, { version: 1, passes: [pass] }, cargo, { cancel: { signal: controller.signal, milliseconds: 30000 } }, "hang");
  const cancel=setInterval(()=>{if(existsSync(cargo.calls)){clearInterval(cancel);controller.abort(Error("cancelled by law"));}},fixture.stub.cancelAfterMs);
  try{await expect(cancelled).rejects.toThrow("cancelled by law");}finally{clearInterval(cancel);}
  const pid = JSON.parse(readFileSync(cargo.calls, "utf8").trim().split("\n")[0]!).pid;
  await Bun.sleep(200);
  expect(() => process.kill(pid, 0)).toThrow();
  expect((await runStub(root, { version: 1, passes: [pass] }, cargo)).failed).toEqual([]);
  checkpoint();
});

registerIsolatedBuildTestV1(import.meta.path, "the stall watchdog stops a pass in which nothing starts or finishes", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const stalledRoot = mkdtempSync(join(artifacts, "y-")), stalledCargo = stubCargo(stalledRoot);
  await expect(runStub(stalledRoot, { version: 1, passes: [stubWorkspace(stalledRoot, "hub", ["comp-a"], "release")] }, stalledCargo, { stallMs: fixture.stub.stallMs }, "hang")).rejects.toThrow(/stalled/u);
  checkpoint();
}, /stalled/u);
