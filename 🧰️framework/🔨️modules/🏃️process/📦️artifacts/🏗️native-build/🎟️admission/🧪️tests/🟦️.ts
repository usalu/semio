import { createIssuedResourceControlV1 } from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { withBuildTestInvocationV1 } from "../../../../🧪️testing/📥️invocation/🟦️.ts";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import {parse as parseToml} from "smol-toml";
import { quantile, rollup } from "d3-array";
import { Database } from "bun:sqlite";
import ts from "typescript";
import * as admission from "../🟦️.ts";
import { localCompilerInputKeyV1, normalizeRustcArgumentsV1, unitKeyV1 as rustcCacheKey } from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";
import { appendFileSync, mkdtempSync, readFileSync, truncateSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import localHistory from "./🧫️fixtures/local-history.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { stretchedBudgetMillisecondsV1 } from "../../../../📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildThroughBrokerV1, tinyCargoWorkspaceV1 } from "../../🧪️tests/🦀️cargo/🟦️.ts";
import { appendUnitHistoryV1, compactUnitHistoryV1, admissionFrozenV1, componentFinalCratePeaksV1, componentFinalPeaksV1, admissionOrderV1, admitWaitersV1, expectedWallMsV1, externArtifactsV1, predictPeakBytesV1, quantileR7V1, readUnitHistoryV1, recordUnitHistoryV1, unitArtifactV1, unitKeyV1, unitOutputBytesV1, type UnitObservationV1, type UnitRecordV1 } from "../🟦️.ts";

const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);
const ajv = new Ajv({ strict: true, allowUnionTypes: true }).addSchema(schema);
const definition = (name: string) => ajv.getSchema(`${schema.$id}#/definitions/${name}`)!;

test("the wire protocol and observations are schema-first and agree with Ajv", () => {
  for (const [name, rows] of [["AdmitRequestV1", fixture.requests], ["DoneReportV1", fixture.done], ["ObservationV1", fixture.observations]] as const) {
    for (const row of rows.valid) expect(definition(name)(row), `${name} ${JSON.stringify(definition(name).errors)}`).toBe(true);
    for (const row of rows.invalid) expect(definition(name)(row), `${name} ${JSON.stringify(row)}`).toBe(false);
  }
  expect(definition("VerdictV1")({ v: 1, op: "verdict", admitted: false })).toBe(true);
  expect(definition("RssReportV1")({ v: 1, op: "rss", bytes: 1, chargedBytes: 2 })).toBe(true);
});

test("unit keys, artifact identities and extern edges follow the vectors", () => {
  for (const row of fixture.keys) expect(unitKeyV1(row.request, row.profile)).toBe(row.key);
  for (const row of fixture.artifacts) expect(unitArtifactV1(row)).toBe(resolve(row.artifact));
  for (const row of fixture.externs) expect(externArtifactsV1(row.path)).toEqual({ candidates: row.candidates.map(path => resolve(path)), edge: row.edge as "rmeta" | "full" });
});

test("p95 is R-7 and agrees with d3-array; predictions follow headroom, floor, window and the cold-start model", () => {
  for (const row of fixture.quantiles) { expect(quantileR7V1(row.values, row.q)).toBe(row.value); expect(quantileR7V1(row.values, row.q)).toBeCloseTo(quantile(row.values, row.q)!, 9); }
  for (const row of fixture.predictions) expect(predictPeakBytesV1(row.peaks, { kloc: row.kloc, component: row.component })).toBe(row.bytes);
  for (const row of fixture.predictions.filter((entry) => entry.component && entry.component.length >= 3 && !entry.peaks.length)) expect(predictPeakBytesV1([], { kloc: 0, component: row.component })).toBe(Math.max(Math.ceil(1.15 * quantile(row.component!, 0.5)!), 268435456));
  for (const row of fixture.componentPools) expect(componentFinalPeaksV1(new Map(Object.entries(row.history).map(([key, peaks]) => [key, peaks.map((peakRssBytes, atMs) => ({ v: 1 as const, key, peakRssBytes, outputBytes: 1, wallMs: 1, cpuMs: 1, remainingMs: 0, atMs }))])), row.profile)).toEqual(row.peaks);
  for (const row of fixture.walls) expect(expectedWallMsV1(row.walls.map((wallMs, atMs) => ({ v: 1, key: "p@k|host|dev|lib", peakRssBytes: 1, outputBytes: 1, wallMs, cpuMs: 1, remainingMs: 0, atMs })))).toBe(row.expected);
});

test("the peaks that size a final for the fleet are the largest recorded peak of every crate with a non-host cdylib, in megabytes", () => {
  const MB = 1048576, rows = (key: string, ...megabytes: number[]) => [key, megabytes.map((peak, atMs) => ({ v: 1 as const, key, peakRssBytes: peak * MB, outputBytes: 1, wallMs: 1, cpuMs: 1, remainingMs: 0, atMs }))] as const;
  const history = new Map([rows("semio-hub-cad-component@semio_hub_cad_component|wasm32-wasip2|release|cdylib", 6550, 6100), rows("semio-os-flow-component@semio_os_flow_component|wasm32-unknown-unknown|release|cdylib", 1857, 1700), rows("semio-hub-cad-component@semio_hub_cad_component|host|release|cdylib", 9999), rows("serde@serde|wasm32-unknown-unknown|release|lib", 900), rows("p@p_bin|host|release|bin", 80)]);
  expect([...componentFinalCratePeaksV1(history)].sort()).toEqual([["semio_hub_cad_component", 6550], ["semio_os_flow_component", 1857]]);
  expect(componentFinalCratePeaksV1(new Map()).size).toBe(0);
});

test("the forecast history keeps only the rows recorded for a source of comparable size, as an independent size-ratio oracle sees it", () => {
  const tolerance = schema.definitions.Policy.const.forecastSizeToleranceBasisPoints / 10000;
  for (const row of fixture.forecasts) {
    const kept = admission.forecastHistoryV1(row.history as never, row.lines);
    expect(kept.map((entry) => row.history.indexOf(entry as never)), row.name).toEqual(row.kept);
    const oracle = row.history.filter((entry) => "lines" in entry && (row.lines === 0 ? entry.lines === 0 : Math.abs(Math.log((entry.lines as number) / row.lines)) <= Math.log(1 + tolerance) + 1e-12));
    expect(kept, row.name).toEqual(oracle as never);
  }
});

test("waiters are ordered by remaining path, then predicted peak, then arrival", () => {
  for (const row of fixture.order) expect(admissionOrderV1(row.waiters).map((waiter) => waiter.id)).toEqual(row.expected);
});

test("admission decisions follow the language-neutral vectors (progress, freeze, EASY backfilling)", () => {
  for (const row of fixture.decisions) expect(admitWaitersV1(row.state, row.waiters), row.name).toEqual(row.admitted);
  for (const row of fixture.freeze) expect(admissionFrozenV1(row.wasFrozen, row)).toBe(row.frozen);
});

test("the store keeps the newest window per key, survives a torn trailing line and rewrites atomically under its lease", () => withBuildTestInvocationV1(async (original) => {
  const directory = mkdtempSync(join(artifacts, "h-")), path = join(directory, "units.jsonl");
  await recordUnitHistoryV1(path, fixture.store.rows as UnitObservationV1[], { leaseDirectory: artifacts, control: createIssuedResourceControlV1(original, "history:store", 10_000, AbortSignal.timeout(10_000)) });
  appendFileSync(path, "{\"v\":1,\"key\":\"torn");
  const history = await readUnitHistoryV1(path);
  expect(Object.fromEntries([...history].map(([key, rows]) => [key, rows.map((row) => row.atMs)]))).toEqual(fixture.store.kept);
  await recordUnitHistoryV1(path, [], { leaseDirectory: artifacts, control: createIssuedResourceControlV1(original, "history:store", 10_000, AbortSignal.timeout(10_000)) });
  expect([...(await readUnitHistoryV1(path)).keys()].sort()).toEqual(Object.keys(fixture.store.kept).sort());
}));

const journalOptions = (original: Parameters<Parameters<typeof withBuildTestInvocationV1>[0]>[0]) => ({ leaseDirectory: artifacts, control: createIssuedResourceControlV1(original, "history:journal", 10_000, AbortSignal.timeout(10_000)) });
const journalView = (history: Map<string, admission.UnitObservationV1[]>) => Object.fromEntries([...history].map(([key, rows]) => [key, rows.map((row) => ({ atMs: row.atMs, peakRssBytes: row.peakRssBytes, remainingMs: row.remainingMs }))]));

test("local compiler forecasts record completed exact-key samples and isolate other hosts", () => withBuildTestInvocationV1(async (original) => {
  const root = mkdtempSync(join(artifacts, "lh-"));
  const darwin = admission.localUnitHistoryPathV1(root, "darwin", "arm64");
  const linux = admission.localUnitHistoryPathV1(root, "linux", "arm64");
  const intel = admission.localUnitHistoryPathV1(root, "darwin", "x64");
  const { owner, lines, peakRssBytes, outputBytes } = localHistory;
  const key = unitKeyV1(owner, owner.profile);
  const compilerKey = (args = ["--cfg", 'feature="std"', "-C", "opt-level=0"], toolchain = "rustc-aarch64") =>
    rustcCacheKey({ toolchain, normalized: normalizeRustcArgumentsV1(args, new Map(), undefined), sourceSha256: "same", externs: [], environment: [] });
  const localInputKey = localCompilerInputKeyV1(compilerKey(), owner.package, owner.version, false)!;
  const unit = {
    ...owner, key, localInputKey, lines, peakRssBytes, pass: "local", primary: false, artifact: "libc-1",
    dependencies: [], buildScriptOutDir: null, predictedBytes: 1, requestMs: 0, admittedMs: 1,
    startMs: 2, endMs: 25, rmetaMs: null, userMs: 6, sysMs: 4, exit: 0,
  } satisfies UnitRecordV1;
  let reads = 0;
  const measure = (artifact: string) => { reads++; expect(artifact).toBe(unit.artifact); return outputBytes; };
  for (const { name, ...details } of localHistory.ignored) {
    expect(admission.localUnitObservationV1({ ...unit, ...details }, measure), name).toBeNull();
  }
  expect(admission.localUnitObservationV1({ ...unit, lines: undefined }, measure)).toBeNull();
  expect(admission.localUnitObservationV1({ ...unit, localInputKey: undefined }, measure)).toBeNull();
  expect(reads).toBe(0);
  const row = admission.localUnitObservationV1({ ...unit, exit: localHistory.accepted[0]!.exit }, measure)!;
  expect(definition("ObservationV1")(row), JSON.stringify(definition("ObservationV1").errors)).toBe(true);
  await recordUnitHistoryV1(darwin, [row], journalOptions(original));
  const disk = await readUnitHistoryV1(darwin);
  expect(await readUnitHistoryV1(linux)).toEqual(new Map());
  expect(await readUnitHistoryV1(intel)).toEqual(new Map());
  expect(await readUnitHistoryV1(admission.unitHistoryPathV1(root))).toEqual(new Map());
  const sample = (localKey?: string, unit = key, history = new Map<string, UnitObservationV1[]>()) =>
    admission.admissionForecastHistoryV1(unit, lines, history, disk, localKey);
  expect(sample(localInputKey, unitKeyV1({ ...owner, target: "wasm32-wasip2" }, owner.profile))).toEqual([]);
  expect(sample(localInputKey, unitKeyV1(owner, "release"))).toEqual([]);
  for (const other of [
    localCompilerInputKeyV1(compilerKey(["--cfg", 'feature="extra_traits"', "-C", "opt-level=0"]), owner.package, owner.version, false),
    localCompilerInputKeyV1(compilerKey(["--cfg", 'feature="std"', "-C", "opt-level=3"]), owner.package, owner.version, false),
    localCompilerInputKeyV1(compilerKey(undefined, "different-compiler"), owner.package, owner.version, false),
    localCompilerInputKeyV1(compilerKey(), owner.package, "0.2.187", false),
    localCompilerInputKeyV1(compilerKey(), owner.package, owner.version, false, ["changed-artifact-bytes"]),
    localCompilerInputKeyV1(compilerKey(), "tokio", owner.version, false),
    localCompilerInputKeyV1(compilerKey(), owner.package, owner.version, true),
    undefined,
  ]) expect(sample(other)).toEqual([]);
  const established = { ...row, key, peakRssBytes: 1024 ** 3 };
  expect(sample(localInputKey, key, new Map([[key, [established]]]))).toEqual([established]);
  const peaks = sample(localInputKey).map(row => row.peakRssBytes);
  expect(peaks).toEqual([peakRssBytes]);
  expect(predictPeakBytesV1(peaks, { kloc: lines / 1000, component: null })).toBe(admission.ADMISSION_POLICY_V1.floorBytes);
  expect(reads).toBe(1);
}));

test("a refined observation replaces the incrementally recorded one of the same key and time (vectors, cross-checked with d3-array)", () => {
  const rows = fixture.journal.rows as UnitObservationV1[];
  expect(journalView(compactUnitHistoryV1(rows))).toEqual(fixture.journal.refined);
  const last = rollup(rows, (group) => group[group.length - 1]!.remainingMs, (row) => row.key, (row) => row.atMs);
  for (const [key, observed] of compactUnitHistoryV1(rows)) for (const row of observed) expect(row.remainingMs).toBe(last.get(key)!.get(row.atMs)!);
});

test("appended rows are durable and concurrent writers never interleave", () => withBuildTestInvocationV1(async (original) => {
  const path = join(mkdtempSync(join(artifacts, "j-")), "units.jsonl"), rows = fixture.journal.rows as UnitObservationV1[];
  await Promise.all(rows.map((row) => appendUnitHistoryV1(path, [row], journalOptions(original))));
  const text = readFileSync(path, "utf8");
  expect(text.endsWith("\n")).toBe(true);
  expect(text.split("\n").filter(Boolean).map((line) => JSON.parse(line).atMs).sort()).toEqual(rows.map((row) => row.atMs).sort());
  expect(await readUnitHistoryV1(path)).toEqual(compactUnitHistoryV1(rows));
}));

test("a write killed at any byte of a record leaves every complete row readable and the next append repairs the tail", () => withBuildTestInvocationV1(async (original) => {
  const path = join(mkdtempSync(join(artifacts, "k-")), "units.jsonl"), rows = fixture.journal.rows as UnitObservationV1[], complete = rows.slice(0, 2);
  await appendUnitHistoryV1(path, complete, journalOptions(original));
  const base = readFileSync(path, "utf8"), record = `${JSON.stringify(rows[3])}\n`;
  for (let cut = 0; cut < record.length; cut++) {
    writeFileSync(path, base + record.slice(0, cut));
    expect(await readUnitHistoryV1(path), `cut ${cut}`).toEqual(compactUnitHistoryV1(complete));
    await appendUnitHistoryV1(path, [rows[2]!], journalOptions(original));
    expect(readFileSync(path, "utf8"), `cut ${cut}`).toBe(base + `${JSON.stringify(rows[2])}\n`);
    truncateSync(path, base.length);
  }
  writeFileSync(path, base + record);
  expect(await readUnitHistoryV1(path)).toEqual(compactUnitHistoryV1([...complete, rows[3]!]));
  for (const damage of [fixture.journal.invalidTerminated, "\0".repeat(4096), `${"\0".repeat(64)}${JSON.stringify(rows[3])}`]) {
    writeFileSync(path, `${damage}\n${base}`);
    expect(await readUnitHistoryV1(path), JSON.stringify(damage.slice(0, 8))).toEqual(compactUnitHistoryV1(complete));
    await recordUnitHistoryV1(path, [rows[3]!], journalOptions(original));
    expect(readFileSync(path, "utf8")).not.toContain("\0");
    expect(await readUnitHistoryV1(path)).toEqual(compactUnitHistoryV1([...complete, rows[3]!]));
  }
  writeFileSync(path, `{"v":1}\n${base}`);
  await expect(readUnitHistoryV1(path)).rejects.toThrow();
}));

test("real Cargo through the wrapper: a tight memory budget admits one unit at a time, a roomy one runs as many as the pool holds", async () => {
  const manifest = tinyCargoWorkspaceV1(mkdtempSync(join(artifacts, "w-")), fixture.cargo.crates);
  const tight = await buildThroughBrokerV1({ manifest, tokens: fixture.cargo.tokens, physicalBytes: Math.round(fixture.cargo.tightPhysicalGiB * 1024 ** 3) });
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
  const roomy = await buildThroughBrokerV1({ manifest, tokens: fixture.cargo.tokens, physicalBytes: Math.round(fixture.cargo.roomyPhysicalGiB * 1024 ** 3) });
  expect(roomy.exit).toBe(0);
  expect(roomy.peakRunning).toBeLessThanOrEqual(fixture.cargo.tokens);
  expect(roomy.peakRunning).toBe(fixture.cargo.tokens);
}, stretchedBudgetMillisecondsV1(600_000));

test("a unit's output footprint counts exactly the files its artifact stem names", () => {
  const vector = fixture.outputs as { artifact: string; entries: string[]; sizes: Record<string, number>; bytes: number };
  expect(unitOutputBytesV1(vector.artifact, vector.entries, (name) => vector.sizes[name]!)).toBe(vector.bytes);
});

test("memory capacity respects actual process bounds and agrees with SQLite independently",()=>{
 const api=admission as unknown as{admissionMemoryBytesV1?:(physical:number,constraint:number)=>number;admissionMemoryShareV1?:(effective:number,tokens:number,parallelism:number)=>number};expect(typeof api.admissionMemoryBytesV1).toBe("function");expect(typeof api.admissionMemoryShareV1).toBe("function");const db=new Database(":memory:");try{for(const row of fixture.memory){expect(definition("MemoryCapacityV1")({physicalBytes:row.physicalBytes,constrainedBytes:row.constrainedBytes})).toBe(true);const oracle=db.query("select case when ?2=0 then ?1 else min(?1,?2) end bytes").get(row.physicalBytes,row.constrainedBytes)as{bytes:number};expect(api.admissionMemoryBytesV1!(row.physicalBytes,row.constrainedBytes),row.name).toBe(row.effectiveBytes);expect(oracle.bytes,row.name).toBe(row.effectiveBytes);}for(const row of fixture.memoryShares){expect(definition("MemoryShareV1")({effectiveBytes:row.effectiveBytes,tokens:row.tokens,parallelism:row.parallelism})).toBe(true);const oracle=db.query("select min(?1,min(?4,cast(floor(?1*?5/10000.0) as integer))+cast(floor((?1-min(?4,cast(floor(?1*?5/10000.0) as integer)))*1.0*?2/?3) as integer)) bytes").get(row.effectiveBytes,row.tokens,row.parallelism,admission.ADMISSION_POLICY_V1.maximumReserveBytes,admission.ADMISSION_POLICY_V1.reserveBasisPoints)as{bytes:number};expect(api.admissionMemoryShareV1!(row.effectiveBytes,row.tokens,row.parallelism)).toBe(row.bytes);expect(oracle.bytes).toBe(row.bytes);}for(const value of[-1,NaN,Infinity,0.5])expect(()=>api.admissionMemoryBytesV1!(8589934592,value)).toThrow();for(const value of[0,-1,NaN,Infinity,0.5])expect(()=>api.admissionMemoryBytesV1!(value,0)).toThrow();expect(()=>api.admissionMemoryShareV1!(8589934592,2,1)).toThrow();expect(admission.ADMISSION_POLICY_V1.maximumReserveBytes).toBe(6442450944);expect(admission.ADMISSION_POLICY_V1.reserveBasisPoints).toBe(2500);for(const[total,reserve]of[[8*2**30,2*2**30],[16*2**30,4*2**30],[32*2**30,6*2**30],[64*2**30,6*2**30]])expect(admission.admissionReserveBytesV1(total),String(total)).toBe(reserve);}finally{db.close();}
});

test("both actual production broker inputs apply memory bounds and preserve explicit driver observations",()=>{
 const files=["../../🧩️component-pass/🟦️.ts", "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts"];
 for(const[index,path]of files.entries()){
  const tree=ts.createSourceFile(path,readFileSync(new URL(path,import.meta.url),"utf8"),ts.ScriptTarget.Latest,true),brokers:ts.ObjectLiteralExpression[]=[],parent:ts.Expression[]=[];
  const visit=(node:ts.Node):void=>{
   if(ts.isCallExpression(node)&&node.expression.getText(tree)==="startAdmissionBrokerV1"&&node.arguments[0]&&ts.isObjectLiteralExpression(node.arguments[0]))brokers.push(node.arguments[0]);
   if(ts.isVariableDeclaration(node)&&node.name.getText(tree)==="parentMemoryAllowanceBytes"&&node.initializer)parent.push(node.initializer);
   ts.forEachChild(node,visit);
  };visit(tree);
  expect(brokers).toHaveLength(1);expect(parent).toHaveLength(1);
  expect(brokers[0]!.getText(tree)).toContain("parentMemoryAllowanceBytes");
  expect(parent[0]!.getText(tree)).toBe(`parentCompilerMemoryAllowanceBytesV1(${index===0?"inputEnvironment":"environment"})`);
  const physical=brokers[0]!.properties.find((property):property is ts.PropertyAssignment=>ts.isPropertyAssignment(property)&&property.name.getText(tree)==="physicalBytes");
  expect(physical).toBeDefined();
  const expression=ts.transpile(`(${physical!.initializer.getText(tree)})`,{target:ts.ScriptTarget.ESNext});
  const run=new Function("options","admissionMemoryBytesV1","admissionMemoryShareV1","tokens","local","physical","availableParallelism",`return ${expression}`);
  for(const row of fixture.memoryShares){
   const observe=(options:object)=>run(options,()=>row.effectiveBytes,admission.admissionMemoryShareV1,row.tokens,row.tokens,row.effectiveBytes,()=>row.parallelism);
   expect(observe({})).toBe(row.bytes);
   if(index===0)expect(observe({physicalBytes:123})).toBe(123);
  }
 }
 const physical=8321515520,constraint=7516192768,effective=admission.admissionMemoryBytesV1(physical,constraint);expect(effective-admission.admissionReserveBytesV1(effective)).toBe(effective-1879048192);expect(physical-effective).toBe(805322752);
});

test("the broker endpoint stays below the Unix socket path limit however long the temporary root is", () => {
  expect(Buffer.byteLength(admission.brokerEndpointV1("linux", "/" + "t".repeat(120)))).toBeLessThan(104);
  expect(admission.brokerEndpointV1("linux", "/tmp/s").startsWith("/tmp/s/semio-broker-")).toBe(true);
  expect(admission.brokerEndpointV1("win32").startsWith("\\\\.\\pipe\\semio-broker-")).toBe(true);
});

 test("scheduler input selection declares the complete locked host profile resolve before offline planning",()=>{
 const select=(admission as unknown as {schedulerCargoSelectionV1:(examples?:boolean,mode?:"prototype"|"release")=>{manifest:string;args:readonly string[]}}).schedulerCargoSelectionV1;
 for(const vector of fixture.schedulerSelections){const selected=select(vector.examples,vector.mode as "prototype"|"release");expect(definition("SchedulerSelectionV1")(selected)).toBe(true);expect(selected.args).toEqual(vector.arguments.map(arg=>arg==="$manifest"?selected.manifest:arg));expect(selected.manifest.endsWith("Cargo.toml")).toBe(true);const manifest=readFileSync(selected.manifest,"utf8");expect(manifest).toContain("semio-build-scheduler");expect(parseToml(manifest)["cargo-features"]).toContain("trim-paths");}
 });
