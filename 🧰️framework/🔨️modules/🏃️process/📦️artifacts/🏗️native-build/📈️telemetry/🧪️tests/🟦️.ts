import { expect, test } from "bun:test";
import Ajv from "ajv";
import { mkdtempSync, readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import type { UnitRecordV1 } from "../../🎟️admission/🟦️.ts";
import { buildThroughBrokerV1, tinyCargoWorkspaceV1 } from "../../🧪️tests/🦀️cargo/🟦️.ts";
import { cpuPercentV1, criticalPathV1, idleWindowsV1, memoryCountersV1, readTimelineV1, relinkFanOutV1, renderBuildReportV1, startTimelineV1, type TimelineRowV1 } from "../🟦️.ts";

const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);
const ajv = new Ajv({ strict: true, allowUnionTypes: true }).addSchema(schema);

/** 🔮️ The critical chain Cargo's own `--timings` report implies: every unit's unblocker is the unit whose `unblocked_units` (full) or `unblocked_rmeta_units` lists it. */
function cargoTimingsChain(html: string): { readonly names: readonly string[]; readonly lengthMs: number } {
  const units = JSON.parse(/UNIT_DATA = (\[[\s\S]*?\n\]);/u.exec(html)![1]!) as { i: number; name: string; start: number; duration: number; unblocked_units: number[]; unblocked_rmeta_units: number[]; sections?: [string, { end: number }][] }[];
  const unblocker = new Map<number, { unit: (typeof units)[number]; at: number }>();
  for (const unit of units) {
    const rmeta = unit.start + (unit.sections?.find(([name]) => name === "frontend")?.[1].end ?? unit.duration);
    for (const [targets, at] of [[unit.unblocked_units, unit.start + unit.duration], [unit.unblocked_rmeta_units, rmeta]] as const) for (const index of targets) if (!unblocker.has(index) || unblocker.get(index)!.at < at) unblocker.set(index, { unit, at });
  }
  const names: string[] = [];
  let current: (typeof units)[number] | undefined = [...units].sort((left, right) => right.start + right.duration - left.start - left.duration)[0];
  const last = current!;
  let first = current!;
  for (; current; current = unblocker.get(current.i)?.unit) { names.unshift(current.name); first = current; }
  return { names, lengthMs: Math.round((last.start + last.duration - first.start) * 1000) };
}

test("timeline rows are schema-first and agree with Ajv", () => {
  const validate = ajv.getSchema(`${schema.$id}#/definitions/TimelineRowV1`)!;
  for (const row of fixture.timeline.valid) expect(validate(row), JSON.stringify(validate.errors)).toBe(true);
  for (const row of fixture.timeline.invalid) expect(validate(row)).toBe(false);
});

test("machine CPU utilisation follows the vectors and the platform memory counters are readable", async () => {
  for (const row of fixture.cpu) expect(cpuPercentV1(row.previous, row.next)).toBe(row.percent);
  const memory = await memoryCountersV1();
  if (process.platform === "darwin" || process.platform === "linux") { expect(memory.availableBytes).toBeGreaterThan(0); expect(memory.swappedOutBytes).not.toBeNull(); }
});

test("the critical path walks unblockers through rmeta, full and build-script edges; remaining paths follow the vectors", () => {
  const critical = criticalPathV1(fixture.critical.units as UnitRecordV1[], fixture.critical.runs);
  expect(critical.segments.map(({ id, edge, readyMs, startMs, endMs }) => ({ id, edge, readyMs, startMs, endMs }))).toEqual(fixture.critical.segments as never);
  expect(Object.fromEntries(critical.remainingMs)).toEqual(fixture.critical.remainingMs);
  const validate = ajv.getSchema(`${schema.$id}#/definitions/CriticalSegmentV1`)!;
  for (const segment of critical.segments) expect(validate(segment), JSON.stringify(validate.errors)).toBe(true);
});

test("idle windows carry their dominant cause; relink fan-out counts every component closure", () => {
  expect(idleWindowsV1(fixture.idle.rows as TimelineRowV1[])).toEqual(fixture.idle.windows as never);
  expect(relinkFanOutV1(fixture.critical.units as UnitRecordV1[])).toEqual(fixture.fanOut);
});

test("the report renders every section and the timeline sampler writes schema-valid rows", async () => {
  const critical = criticalPathV1(fixture.critical.units as UnitRecordV1[], fixture.critical.runs);
  const report = renderBuildReportV1({ title: "Release build law", startedMs: 0, endedMs: 600, timeline: fixture.idle.rows as TimelineRowV1[], units: fixture.critical.units as UnitRecordV1[], runs: fixture.critical.runs, stages: [{ component: "k", stage: "stage", readyMs: 500, startMs: 500, endMs: 550, ok: true }], passes: [{ id: "pass-0", manifest: "/w/Cargo.toml", profile: "p", target: "host", startMs: 0, endMs: 500, exit: 0, failed: [] }], critical });
  for (const heading of fixture.report.headings) expect(report).toContain(heading);
  const path = join(mkdtempSync(join(artifacts, "t-")), "timeline.jsonl"), sampler = startTimelineV1({ path, intervalMs: 50, counters: () => ({ rustcRunning: 0, admissionWaiting: 0, rustcRssBytes: 0, frozen: false, tokensHeld: 0, stagesRunning: 0, stagesWaiting: 0 }) });
  await Bun.sleep(400);
  const rows = await sampler.stop();
  expect(rows.length).toBeGreaterThanOrEqual(2);
  expect(readTimelineV1(path)).toEqual(rows as TimelineRowV1[]);
});

test("the critical path recomputed from the wrapper's unit graph matches the chain in Cargo's own --timings report", async () => {
  const manifest = tinyCargoWorkspaceV1(mkdtempSync(join(artifacts, "w-")), fixture.timings.crates);
  const build = await buildThroughBrokerV1({ manifest, tokens: fixture.timings.tokens, physicalBytes: fixture.timings.physicalGiB * 1024 ** 3, extraArgs: ["--timings"] });
  expect(build.exit).toBe(0);
  const reports = join(build.targetDirectory, "cargo-timings"), html = readFileSync(join(reports, readdirSync(reports).find((name) => /^cargo-timing-.+\.html$/u.test(name))!), "utf8");
  const oracle = cargoTimingsChain(html), critical = criticalPathV1(build.units, build.runs), byArtifact = new Map(build.units.map((unit) => [unit.artifact, unit]));
  const ours = critical.segments.map((segment) => byArtifact.get(segment.id)!.crate), lengthMs = critical.segments.at(-1)!.endMs - critical.segments[0]!.startMs;
  expect(ours).toEqual([...oracle.names]);
  expect(Math.abs(lengthMs - oracle.lengthMs)).toBeLessThanOrEqual(Math.max(fixture.timings.toleranceMs, fixture.timings.toleranceFraction * oracle.lengthMs));
  console.log(`[telemetry] critical chain ${ours.join(" → ")}: ours ${lengthMs} ms, cargo --timings ${oracle.lengthMs} ms`);
}, 600_000);
