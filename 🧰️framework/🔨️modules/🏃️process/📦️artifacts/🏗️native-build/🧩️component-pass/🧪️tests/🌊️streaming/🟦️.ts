import { expect, test } from "bun:test";
import { existsSync, mkdtempSync, readdirSync, readFileSync } from "node:fs";
import { join, sep } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { nativeBuildComponentStagePathV1, NATIVE_BUILD_DRIVER_POLICY_V1 } from "../../🟦️.ts";
import { type NativeBuildPlanV1 } from "../../📋️contract/🟦️.ts";
import { artifacts, header, stubCargo, stubWorkspace, runStub, stubJob } from "../🎭️stub/🟦️.ts";

test("the driver streams each component the moment Cargo links it, runs profiles in parallel lanes and workspaces of one profile in sequence, under one jobserver", async () => {
  const root = mkdtempSync(join(artifacts, "s-")), cargo = stubCargo(root);
  const passes = [stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release"), stubWorkspace(root, "ext", ["comp-c"], "release"), stubWorkspace(root, "hub2", ["comp-d"], "prototype")];
  await runStub(root, { version: 1, passes }, cargo, {verifyStageWriters:true}, "ok", ({ outcome: run }) => {
  if (!run) return;
  const outcome = run;
  expect([...outcome.staged].sort()).toEqual(["comp-a:release", "comp-b:release", "comp-c:release", "comp-d:prototype"]);
  for (const pass of passes) for (const row of pass.packages) expect([...readFileSync(nativeBuildComponentStagePathV1(outcome.generation,pass,row)).subarray(0, 8)]).toEqual([...header]);
  const calls = readFileSync(cargo.calls, "utf8").trim().split("\n").map((line) => JSON.parse(line));
  const span = (manifest: string) => ({ start: calls.find((row) => row.at && row.manifest === manifest).at, end: calls.find((row) => row.end && row.manifest === manifest).end });
  expect(span(passes[1]!.manifest).start).toBeGreaterThanOrEqual(span(passes[0]!.manifest).end);
  expect(span(passes[2]!.manifest).start).toBeLessThan(span(passes[0]!.manifest).end);
  for (const row of calls.filter((call) => call.at)) { expect(row.jobserver).toMatch(/--jobserver-auth=(?:fifo:|semio-jobserver-)/u); expect(row.wrapper).toBe(process.execPath); expect(row.unification).toBe(NATIVE_BUILD_DRIVER_POLICY_V1.featureUnification); }
  const units = JSON.parse(readFileSync(join(outcome.generation.report, "units.json"), "utf8"));
  const stageA = units.stages.find((stage: any) => stage.component === "comp-a" && stage.stage === "stage");
  expect(stageA.startMs).toBeLessThan(span(passes[0]!.manifest).end);
  expect(units.stages.filter((stage: any) => stage.stage === "provenance").length).toBe(passes.length);
  const provenance = readdirSync(join(outcome.generation.report, NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory));
  expect(provenance.filter((name) => name.startsWith("cargo-unit-provenance-"))).toHaveLength(passes.length);
  expect(provenance).toEqual(expect.arrayContaining([NATIVE_BUILD_DRIVER_POLICY_V1.provenanceLeavesFile, NATIVE_BUILD_DRIVER_POLICY_V1.provenanceReadsFile]));
  const reads = JSON.parse(readFileSync(join(outcome.generation.report, NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory, NATIVE_BUILD_DRIVER_POLICY_V1.provenanceReadsFile), "utf8"));
  expect(reads, JSON.stringify(reads.excess)).toMatchObject({ closed: true, maximumReads: 2 });
  expect(readFileSync(outcome.report, "utf8")).toContain("## Critical path");
  });
}, 120_000);

test("a component planned in two passes stages, runs its stages and is observed per pass, never on the other pass's completion", async () => {
  const law = fixture.repeatedComponents, root = mkdtempSync(join(artifacts, "rc-")), cargo = stubCargo(root), seen: string[] = [];
  const passes = law.profiles.map((profile) => stubWorkspace(root, "hub", law.components, profile));
  let writes = 0;
  const outcome = await runStub(root, { version: 1, passes }, cargo, {
    withStageWriter: async (_control, write) => { if (writes++) await Bun.sleep(law.laterStageWriteDelayMs); await write(); },
    stages: (linked) => [{ name: "check", capacity: "io", after: [], run: async () => { expect(existsSync(linked.staged)).toBe(true); seen.push(`${linked.package.name}:${linked.pass.profile}`); } }],
  });
  const expected = law.profiles.flatMap((profile) => law.components.map((name) => `${name}:${profile}`)).sort();
  expect([...outcome.staged].sort()).toEqual(expected);
  expect(seen.sort()).toEqual(expected);
}, 120_000);

test("fleet-sized cargo jobs: a component pass and a jobserver job (a wasm-pack style pass) get the local grant plus the remote slots as their job count and join the fleet-sized jobserver; without a fleet nothing changes", async () => {
  const run = async (fleet: boolean) => {
    const root = mkdtempSync(join(artifacts, fleet ? "fj1-" : "fj0-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release");
    const plan: NativeBuildPlanV1 = { version: 1, passes: [pass], jobs: [stubJob(root, "wasm", [], 20)] };
    await runStub(root, plan, cargo, fleet ? { fleetJobs: async () => ({ jobs: fixture.stub.tokens + 5, local: fixture.stub.tokens, remote: 5, reason: "fleet-all" as const }) } : {});
    const pass_ = readFileSync(cargo.calls, "utf8").trim().split("\n").map((line) => JSON.parse(line)).find((row) => row.at), job = readFileSync(join(root, "jobs.jsonl"), "utf8").trim().split("\n").map((line) => JSON.parse(line))[0];
    return { pass: pass_, job };
  };
  const fleet = await run(true), local = await run(false), planned = String(fixture.stub.tokens + 5);
  expect(fleet.pass.jobs).toBe(planned);expect(fleet.job.jobs).toBe(planned);
  if (process.platform !== "win32") for (const row of [fleet.pass, fleet.job]) expect(row.jobserver).toContain(`${sep}fleet${sep}`);
  expect(local.pass.jobs).not.toBe(planned);expect(local.job.jobs).not.toBe(planned);
}, 120_000);

test("component jobs: workspaces of one wasm lane overlap up to the planned jobs when the fleet takes their finals, and stay in sequence without a fleet", async () => {
  const spans = async (root: string, jobs: number | undefined) => {
    const cargo = stubCargo(root), passes = [stubWorkspace(root, "w0", ["c-0"], "release"), stubWorkspace(root, "w1", ["c-1"], "release"), stubWorkspace(root, "w2", ["c-2"], "release")];
    const requested: { local: number; crates: readonly string[] }[] = [];
    const fleet = jobs === undefined ? {} : { fleetJobs: async () => ({ jobs: 2 + jobs, local: 2, remote: jobs, reason: "fleet-all" as const }), componentJobs: async (facts: { local: number; crates: readonly string[] }) => { requested.push(facts); return { jobs, local: 1, remote: jobs - 1, reason: "jobs-all" as const }; } };
    await runStub(root, { version: 1, passes }, cargo, fleet);
    const calls = readFileSync(cargo.calls, "utf8").trim().split("\n").map((line) => JSON.parse(line));
    return { requested, spans: passes.map(pass => ({ start: calls.find((row) => row.at && row.manifest === pass.manifest).at as number, end: calls.find((row) => row.end && row.manifest === pass.manifest).end as number })) };
  };
  const sequential = await spans(mkdtempSync(join(artifacts, "cj0-")), undefined);
  expect(sequential.spans[1]!.start).toBeGreaterThanOrEqual(sequential.spans[0]!.end);expect(sequential.spans[2]!.start).toBeGreaterThanOrEqual(sequential.spans[1]!.end);
  const down = await spans(mkdtempSync(join(artifacts, "cj1-")), 1);
  expect(down.spans[1]!.start).toBeGreaterThanOrEqual(down.spans[0]!.end);
  const concurrent = await spans(mkdtempSync(join(artifacts, "cj3-")), 3);
  expect(concurrent.requested[0]!.crates.sort()).toEqual(["c_0", "c_1", "c_2"]);
  expect(concurrent.spans[1]!.start).toBeLessThan(concurrent.spans[0]!.end);expect(concurrent.spans[2]!.start).toBeLessThan(concurrent.spans[0]!.end);
}, 120_000);

test("fleet component jobs: under raised local memory pressure passes start on fleet slots while a local reservation is out of reach, and with no free worker slot they wait for a local one", async () => {
  const run = async (root: string, slots: number | null) => {
    const cargo = stubCargo(root), passes = [stubWorkspace(root, "w0", ["c-0"], "release"), stubWorkspace(root, "w1", ["c-1"], "release"), stubWorkspace(root, "w2", ["c-2"], "release")], lines: string[] = [], asked: number[] = [];
    await runStub(root, { version: 1, passes }, cargo, { log: line => lines.push(line), fleetJobs: async () => ({ jobs: 6, local: 2, remote: 4, reason: "fleet-all" as const }), componentJobs: async () => ({ jobs: 5, local: 1, remote: 4, reason: "jobs-all" as const }), fleetSlots: async () => { asked.push(Date.now()); return slots; }, localJobs: { maximum: 2, floorMegabytes: 4000, peakMegabytes: () => 3000, sample: async () => ({ availableMegabytes: 6000, ownedMegabytes: 0, pressure: "warn" }) } });
    const calls = readFileSync(cargo.calls, "utf8").trim().split("\n").map((line) => JSON.parse(line));
    return { lines, asked, spans: passes.map(pass => ({ start: calls.find((row) => row.at && row.manifest === pass.manifest).at as number, end: calls.find((row) => row.end && row.manifest === pass.manifest).end as number })) };
  };
  const open = await run(mkdtempSync(join(artifacts, "fj3-")), 3);
  expect(open.spans[1]!.start).toBeLessThan(open.spans[0]!.end);expect(open.spans[2]!.start).toBeLessThan(open.spans[0]!.end);
  expect(open.lines.filter(line => line.includes("starts on a fleet slot beside")).length).toBe(2);
  expect(open.lines.some(line => line.includes("component build: 2 local slot(s) and up to") && line.includes("fleet slot(s)"))).toBe(true);
  expect(open.asked.length).toBeGreaterThan(1);
  const none = await run(mkdtempSync(join(artifacts, "fj0-")), null);
  expect(none.spans[1]!.start).toBeGreaterThanOrEqual(none.spans[0]!.end);
  expect(none.lines.some(line => /waits for a local slot: it needs 3000 MB/.test(line) && line.includes("wartet auf einen lokalen Platz"))).toBe(true);
}, 180_000);

test("local component jobs: without any fleet two finals overlap with real reservations while a third waits for memory, and the memory is re-read before each start", async () => {
  const root = mkdtempSync(join(artifacts, "lj-")), cargo = stubCargo(root), passes = [stubWorkspace(root, "w0", ["c-0"], "release"), stubWorkspace(root, "w1", ["c-1"], "release"), stubWorkspace(root, "w2", ["c-2"], "release")];
  const peaks: Record<string, number> = { [passes[0]!.id]: 3300, [passes[1]!.id]: 1200, [passes[2]!.id]: 6500 }, lines: string[] = [], samples: number[] = [];
  await runStub(root, { version: 1, passes }, cargo, { log: line => lines.push(line), localJobs: { floorMegabytes: 4000, peakMegabytes: pass => peaks[pass.id]!, sample: async () => { samples.push(Date.now()); return { availableMegabytes: 9500, ownedMegabytes: 0, pressure: "normal" }; } } });
  const calls = readFileSync(cargo.calls, "utf8").trim().split("\n").map((line) => JSON.parse(line)), span = (index: number) => ({ start: calls.find((row) => row.at && row.manifest === passes[index]!.manifest).at as number, end: calls.find((row) => row.end && row.manifest === passes[index]!.manifest).end as number });
  expect(span(1).start).toBeLessThan(span(0).end);
  expect(span(2).start).toBeGreaterThanOrEqual(Math.max(span(0).end, span(1).end));
  expect(lines.some(line => line.includes("[component-jobs] " + passes[1]!.id + " starts beside 1 running final(s) and reserves 1200 MB"))).toBe(true);
  expect(lines.some(line => line.includes("[component-jobs] " + passes[2]!.id + " waits for memory beside 2 running final(s): it needs 6500 MB"))).toBe(true);
  expect(lines.some(line => line.includes("[component-jobs] component build: 2 component job(s) may hold a local reservation at once"))).toBe(true);
  expect(samples.length).toBeGreaterThan(2);
}, 120_000);
