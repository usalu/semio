import { phaseCompilerAdmissionControlV1 } from "../../../../🏗️build/🎟️capacity/🟦️.ts";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { quantile } from "d3-array";
import { appendFileSync, mkdtempSync } from "node:fs";
import { join, resolve } from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { buildThroughBrokerV1, tinyCargoWorkspaceV1 } from "../../🧪️tests/🦀️cargo/🟦️.ts";
import { admissionFrozenV1, admissionOrderV1, admitWaitersV1, expectedWallMsV1, externArtifactsV1, predictPeakBytesV1, quantileR7V1, readUnitHistoryV1, recordUnitHistoryV1, unitArtifactV1, unitKeyV1, unitOutputBytesV1, type UnitObservationV1, type UnitRecordV1 } from "../🟦️.ts";

const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);
const ajv = new Ajv({ strict: true, allowUnionTypes: true }).addSchema(schema);
const definition = (name: string) => ajv.getSchema(`${schema.$id}#/definitions/${name}`)!;

test("the wire protocol and observations are schema-first and agree with Ajv", () => {
  for (const [name, rows] of [["AdmitRequestV1", fixture.requests], ["DoneReportV1", fixture.done], ["ObservationV1", fixture.observations]] as const) {
    for (const row of rows.valid) expect(definition(name)(row), `${name} ${JSON.stringify(definition(name).errors)}`).toBe(true);
    for (const row of rows.invalid) expect(definition(name)(row), `${name} ${JSON.stringify(row)}`).toBe(false);
  }
  expect(definition("VerdictV1")({ v: 1, op: "verdict", admitted: false })).toBe(true);
  expect(definition("RssReportV1")({ v: 1, op: "rss", bytes: 1 })).toBe(true);
});

test("unit keys, artifact identities and extern edges follow the vectors", () => {
  for (const row of fixture.keys) expect(unitKeyV1(row.request, row.profile)).toBe(row.key);
  for (const row of fixture.artifacts) expect(unitArtifactV1(row)).toBe(row.artifact);
  for (const row of fixture.externs) expect(externArtifactsV1(row.path)).toEqual({ candidates: row.candidates, edge: row.edge as "rmeta" | "full" });
});

test("p95 is R-7 and agrees with d3-array; predictions follow headroom, floor, window and the cold-start model", () => {
  for (const row of fixture.quantiles) { expect(quantileR7V1(row.values, row.q)).toBe(row.value); expect(quantileR7V1(row.values, row.q)).toBeCloseTo(quantile(row.values, row.q)!, 9); }
  for (const row of fixture.predictions) expect(predictPeakBytesV1(row.peaks, { kloc: row.kloc, componentFinal: row.componentFinal })).toBe(row.bytes);
  for (const row of fixture.walls) expect(expectedWallMsV1(row.walls.map((wallMs, atMs) => ({ v: 1, key: "p@k|host|dev|lib", peakRssBytes: 1, wallMs, cpuMs: 1, remainingMs: 0, atMs })))).toBe(row.expected);
});

test("waiters are ordered by remaining path, then predicted peak, then arrival", () => {
  for (const row of fixture.order) expect(admissionOrderV1(row.waiters).map((waiter) => waiter.id)).toEqual(row.expected);
});

test("admission decisions follow the language-neutral vectors (progress, freeze, EASY backfilling)", () => {
  for (const row of fixture.decisions) expect(admitWaitersV1(row.state, row.waiters), row.name).toEqual(row.admitted);
  for (const row of fixture.freeze) expect(admissionFrozenV1(row.wasFrozen, row)).toBe(row.frozen);
});

test("the store keeps the newest window per key, survives a torn trailing line and rewrites atomically under its lease", async () => {
  const directory = mkdtempSync(join(artifacts, "h-")), path = join(directory, "units.jsonl");
  await recordUnitHistoryV1(path, fixture.store.rows as UnitObservationV1[], { leaseDirectory: artifacts, control: phaseCompilerAdmissionControlV1(AbortSignal.timeout(10_000),10_000,()=>undefined,()=>new Promise(done=>setImmediate(done))) });
  appendFileSync(path, "{\"v\":1,\"key\":\"torn");
  const history = await readUnitHistoryV1(path);
  expect(Object.fromEntries([...history].map(([key, rows]) => [key, rows.map((row) => row.atMs)]))).toEqual(fixture.store.kept);
  await recordUnitHistoryV1(path, [], { leaseDirectory: artifacts, control: phaseCompilerAdmissionControlV1(AbortSignal.timeout(10_000),10_000,()=>undefined,()=>new Promise(done=>setImmediate(done))) });
  expect([...(await readUnitHistoryV1(path)).keys()].sort()).toEqual(Object.keys(fixture.store.kept).sort());
});

test("real Cargo through the wrapper: a tight memory budget admits one unit at a time, a roomy one runs as many as the pool holds", async () => {
  const manifest = tinyCargoWorkspaceV1(mkdtempSync(join(artifacts, "w-")), fixture.cargo.crates);
  const tight = await buildThroughBrokerV1({ manifest, tokens: fixture.cargo.tokens, physicalBytes: fixture.cargo.tightPhysicalGiB * 1024 ** 3 });
  expect(tight.exit).toBe(0);
  const libraries = (units: readonly UnitRecordV1[]) => units.filter((unit) => fixture.cargo.crates.includes(unit.crate));
  expect(libraries(tight.units).map((unit) => unit.crate).sort()).toEqual([...fixture.cargo.crates].sort());
  const overlap = (units: readonly UnitRecordV1[]) => Math.max(...units.map((unit) => units.filter((other) => other.admittedMs <= unit.admittedMs && unit.admittedMs < other.endMs).length));
  expect(overlap(tight.units)).toBe(1);
  expect(tight.peakRunning).toBe(1);
  expect(tight.units.some((unit) => unit.admittedMs > unit.requestMs)).toBe(true);
  for (const unit of libraries(tight.units)) { expect(unit.peakRssBytes).toBeGreaterThan(0); expect(unit.rmetaMs).not.toBeNull(); expect(unit.endMs).toBeGreaterThanOrEqual(unit.startMs); }
  const top = tight.units.find((unit) => unit.crate === "top")!, producers = new Set(tight.units.map((unit) => unit.artifact));
  expect(top.dependencies.filter((edge) => edge.artifacts.some((artifact) => producers.has(artifact))).map((edge) => edge.edge)).toEqual(["rmeta", "rmeta"]);
  const roomy = await buildThroughBrokerV1({ manifest, tokens: fixture.cargo.tokens, physicalBytes: fixture.cargo.roomyPhysicalGiB * 1024 ** 3 });
  expect(roomy.exit).toBe(0);
  expect(roomy.peakRunning).toBeLessThanOrEqual(fixture.cargo.tokens);
  expect(roomy.peakRunning).toBe(fixture.cargo.tokens);
}, 600_000);

test("a unit's output footprint counts exactly the files its artifact stem names", () => {
  const vector = fixture.outputs as { artifact: string; entries: string[]; sizes: Record<string, number>; bytes: number };
  expect(unitOutputBytesV1(vector.artifact, vector.entries, (name) => vector.sizes[name]!)).toBe(vector.bytes);
});
