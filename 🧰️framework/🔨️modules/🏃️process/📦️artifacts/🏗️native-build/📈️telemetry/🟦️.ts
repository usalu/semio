import {open} from "node:fs/promises";
import {constants} from "node:fs";
import {withArtifactDescriptorV1} from "../../../../../📦️packages/🟦️typescript/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../🔒️leases/🟦️.ts";
import {requireIssuedResourceControlV1,withRequestedIssuedResourceControlV1} from "../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {captureOwnedCommandResultV1} from "../../../🎛️owned-execution/📥️capture/🟦️.ts";
import {startTelemetrySamplerV1} from "./⏱️sampling/🟦️.ts";
export {startTelemetrySamplerV1} from "./⏱️sampling/🟦️.ts";
import { appendFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { cpus, freemem, totalmem } from "node:os";
import { dirname, join } from "node:path";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type { UnitRecordV1 } from "../🎟️admission/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 📈️ Sampling cadence, idle-window thresholds and report sizes. */
export const TELEMETRY_POLICY_V1 = Object.freeze({...schema.definitions.Policy.const});

/** 🕒️ One timeline row (`TimelineRowV1`). */
export type TimelineRowV1 = Readonly<{ v: 1; atMs: number; cpuPercent: number; cores: number; memoryTotalBytes: number; memoryAvailableBytes: number | null; swapUsedBytes: number | null; swapOutBytesPerSecond: number | null; rustcRunning: number; admissionWaiting: number; rustcRssBytes: number; frozen: boolean; tokensHeld: number; stagesRunning: number; stagesWaiting: number }>;

/** 🏗️ One executed build script, timed by Cargo's `build-script-executed` message (`BuildScriptRunV1`). */
export type BuildScriptRunV1 = Readonly<{ pass: string; package: string; outDir: string; endMs: number }>;

/** 🎬️ One post-link stage of one component (`StageRecordV1`). */
export type StageRecordV1 = Readonly<{ component: string; stage: string; readyMs: number; startMs: number; endMs: number; ok: boolean }>;

/** 🧗️ One segment of the critical path (`CriticalSegmentV1`): the unit, what unblocked it, when it was ready and when it ran. */
export type CriticalSegmentV1 = Readonly<{ id: string; label: string; edge: "root" | "rmeta" | "full" | "run"; readyMs: number; startMs: number; endMs: number }>;

/** 🧗️ A build's critical path and every unit's remaining path to the end of the build. */
export type CriticalPathV1 = Readonly<{ segments: readonly CriticalSegmentV1[]; remainingMs: ReadonlyMap<string, number> }>;

/** 🛂️ Validates one value against a named telemetry definition. */
export function admitTelemetryV1<T>(name: keyof typeof schema.definitions, value: unknown): T {
  const errors = validateJsonSchemaSubset(schema.definitions[name], value, schema);
  if (errors.length) throw Error(`Invalid ${name}: ${errors.join("; ")}`);
  return value as T;
}

/** ⏲️ Cumulative idle and total CPU time of the machine. */
export function cpuTimesV1(): Readonly<{ idle: number; total: number }> {
  let idle = 0, total = 0;
  for (const core of cpus()) { idle += core.times.idle; total += core.times.idle + core.times.user + core.times.nice + core.times.sys + core.times.irq; }
  return { idle, total };
}

/** 🔥️ Machine-wide CPU utilisation between two samples, in percent of all cores. */
export function cpuPercentV1(previous: Readonly<{ idle: number; total: number }>, next: Readonly<{ idle: number; total: number }>): number {
  const total = next.total - previous.total;
  return total <= 0 ? 0 : Math.min(100, Math.max(0, 100 * (1 - (next.idle - previous.idle) / total)));
}

/** 🧠️ Parses fixed platform memory counter DATA without executing platform commands. */
export function parseMemoryCountersV1(platform:"darwin"|"linux",first:string,second:string):Readonly<{availableBytes:number|null;swapUsedBytes:number|null;swappedOutBytes:number|null}>{
 if(Buffer.byteLength(first)>TELEMETRY_POLICY_V1.maximumOutputBytes||Buffer.byteLength(second)>TELEMETRY_POLICY_V1.maximumOutputBytes)throw Error("Bounded telemetry counter bytes required");
 if(platform==="darwin"){
  const pageSize=Number(/page size of (\d+) bytes/u.exec(first)?.[1]??4096),pages=(name:string)=>Number(new RegExp(`^${name}:\\s+(\\d+)\\.`,"mu").exec(first)?.[1]??0),used=/used = ([\d.]+)([KMG])/u.exec(second),unit=used?.[2],scale=unit==="K"?1024:unit==="M"?1024**2:unit==="G"?1024**3:0;
  return Object.freeze({availableBytes:(pages("Pages free")+pages("Pages inactive")+pages("Pages speculative"))*pageSize,swapUsedBytes:used?Math.round(Number(used[1])*scale):null,swappedOutBytes:pages("Swapouts")*pageSize});
 }
 const kib=(name:string):number|null=>{const match=new RegExp(`^${name}:\\s+(\\d+) kB`,"mu").exec(first);return match?Number(match[1])*1024:null;},total=kib("SwapTotal"),free=kib("SwapFree"),out=/^pswpout (\d+)$/mu.exec(second);
 return Object.freeze({availableBytes:kib("MemAvailable"),swapUsedBytes:total===null||free===null?null:total-free,swappedOutBytes:out?Number(out[1])*4096:null});
}
/** 🧠️ Reads finite platform counters beneath the actual carried sample phase. */
export function memoryCountersV1(control:ResourceLeaseControl,platform:NodeJS.Platform=process.platform):Promise<Readonly<{availableBytes:number|null;swapUsedBytes:number|null;swappedOutBytes:number|null}>>{
 requireIssuedResourceControlV1(control);
 return withRequestedIssuedResourceControlV1(control,TELEMETRY_POLICY_V1.maximumSampleMilliseconds,async actual=>{
  const output={maximumBytes:TELEMETRY_POLICY_V1.maximumOutputBytes,maximumLines:TELEMETRY_POLICY_V1.maximumOutputLines};
  const command=async(program:string,args:readonly string[])=>{const result=await captureOwnedCommandResultV1(program,args,process.cwd(),{control:actual,output});if(result.signal!==null)throw Error("Telemetry counter process terminated by signal");return result.status===0?result.stdout:null;};
  if(platform==="darwin"){const stat=await command("vm_stat",[]),swap=await command("sysctl",["-n","vm.swapusage"]);return stat===null||swap===null?Object.freeze({availableBytes:null,swapUsedBytes:null,swappedOutBytes:null}):parseMemoryCountersV1(platform,stat,swap);}
  if(platform==="linux"){
   const read=(path:string)=>withArtifactDescriptorV1(()=>open(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)),async handle=>{const bytes=Buffer.alloc(TELEMETRY_POLICY_V1.maximumOutputBytes+1);let used=0;while(used<bytes.length){checkResourceLeaseControl(actual);const row=await handle.read(bytes,used,bytes.length-used,used);if(row.bytesRead===0)break;used+=row.bytesRead;}checkResourceLeaseControl(actual);if(used>TELEMETRY_POLICY_V1.maximumOutputBytes)throw Error("Bounded telemetry counter file required");return bytes.subarray(0,used).toString("utf8");});
   return parseMemoryCountersV1(platform,await read("/proc/meminfo"),await read("/proc/vmstat"));
  }
  return Object.freeze({availableBytes:freemem(),swapUsedBytes:null,swappedOutBytes:null});
 });
}

/** 🎛️ The driver-side counters a timeline row records beside the machine sample. */
export type TimelineCountersV1 = Omit<TimelineRowV1, "v" | "atMs" | "cpuPercent" | "cores" | "memoryTotalBytes" | "memoryAvailableBytes" | "swapUsedBytes" | "swapOutBytesPerSecond">;

/** 🕒️ A running timeline sampler. */
export interface TimelineSamplerV1 {
  rows(): readonly TimelineRowV1[];
  stop(): Promise<readonly TimelineRowV1[]>;
}

/** 🕒️ Samples machine CPU, memory and swap plus the driver's counters at start, every `intervalMs` and at stop, appends each row to `path` and hands it to `onRow`. */
export function startTimelineV1(options:Readonly<{control:ResourceLeaseControl;path:string;intervalMs?:number;counters:()=>TimelineCountersV1;onRow?:(row:TimelineRowV1,control:ResourceLeaseControl)=>Promise<void>;now?:()=>number}>):TimelineSamplerV1{
 requireIssuedResourceControlV1(options.control);const {control,path,intervalMs,counters,onRow}=options,now=options.now??Date.now,rows:TimelineRowV1[]=[];
 mkdirSync(dirname(path),{recursive:true});let previous=cpuTimesV1(),swapped:{atMs:number;bytes:number}|null=null;
 const sampler=startTelemetrySamplerV1(control,async actual=>{
  const memory=await memoryCountersV1(actual),atMs=now(),times=cpuTimesV1(),swapOutBytesPerSecond=memory.swappedOutBytes===null||swapped===null||atMs<=swapped.atMs?null:Math.max(0,(memory.swappedOutBytes-swapped.bytes)*1000/(atMs-swapped.atMs));
  if(memory.swappedOutBytes!==null)swapped={atMs,bytes:memory.swappedOutBytes};
  const row:TimelineRowV1=Object.freeze({v:1,atMs,cpuPercent:Math.round(cpuPercentV1(previous,times)*10)/10,cores:cpus().length,memoryTotalBytes:totalmem(),memoryAvailableBytes:memory.availableBytes,swapUsedBytes:memory.swapUsedBytes,swapOutBytesPerSecond:swapOutBytesPerSecond===null?null:Math.round(swapOutBytesPerSecond),...counters()});
  const line=JSON.stringify(row)+"\n";if(Buffer.byteLength(line)>TELEMETRY_POLICY_V1.maximumRowBytes)throw Error("Bounded telemetry row required");previous=times;rows.push(row);appendFileSync(path,line);await onRow?.(row,actual);
 },intervalMs);
 return Object.freeze({rows:()=>Object.freeze([...rows]),stop:async()=>{await sampler.stop();return Object.freeze([...rows]);}});
}

/** 🧗️ The critical path recomputed from the observed unit graph: every unit's dependencies are the artifacts its `--extern`s name (metadata edges pipeline on rmeta,
 * library edges wait for the full unit) plus the build-script run whose `OUT_DIR` it compiles against. A unit's unblocker is the dependency available last;
 * the path walks unblockers back from the unit that finished last. `remainingMs` is each unit's longest duration path to a sink, the admission priority.
 * Build-script runs are timed by Cargo's message; their start is the end of their compiled script. */
export function criticalPathV1(units: readonly UnitRecordV1[], runs: readonly BuildScriptRunV1[]): CriticalPathV1 {
  type Node = { id: string; label: string; startMs: number; endMs: number; rmetaMs: number | null; predecessors: { node: Node; edge: "rmeta" | "full" | "run" }[]; successors: Node[] };
  const nodes = new Map<string, Node>();
  for (const unit of units) if (unit.exit === 0) nodes.set(unit.artifact, { id: unit.artifact, label: `${unit.package}@${unit.crate} (${unit.crateTypes.join(",")}, ${unit.target}, ${unit.profile})`, startMs: unit.startMs, endMs: unit.endMs, rmetaMs: unit.rmetaMs, predecessors: [], successors: [] });
  const runNodes = new Map<string, Node>();
  for (const run of runs) {
    const script = units.filter((unit) => unit.exit === 0 && unit.pass === run.pass && unit.package === run.package && unit.crate === "build_script_build" && unit.endMs <= run.endMs).sort((left, right) => right.endMs - left.endMs)[0];
    const node: Node = { id: `run:${run.outDir}`, label: `${run.package} build script (run)`, startMs: Math.min(script?.endMs ?? run.endMs, run.endMs), endMs: run.endMs, rmetaMs: null, predecessors: [], successors: [] };
    const scriptNode = script ? nodes.get(script.artifact) : undefined;
    if (scriptNode) { node.predecessors.push({ node: scriptNode, edge: "full" }); scriptNode.successors.push(node); }
    runNodes.set(run.outDir, node);
    nodes.set(node.id, node);
  }
  for (const unit of units) {
    const node = nodes.get(unit.artifact);
    if (!node || unit.exit !== 0) continue;
    for (const dependency of unit.dependencies) {
      const producer = dependency.artifacts.map((artifact) => nodes.get(artifact)).find(Boolean);
      if (producer) { node.predecessors.push({ node: producer, edge: dependency.edge }); producer.successors.push(node); }
    }
    const run = unit.buildScriptOutDir === null ? undefined : runNodes.get(unit.buildScriptOutDir);
    if (run) { node.predecessors.push({ node: run, edge: "run" }); run.successors.push(node); }
  }
  const available = (predecessor: { node: Node; edge: "rmeta" | "full" | "run" }): number => predecessor.edge === "rmeta" ? predecessor.node.rmetaMs ?? predecessor.node.endMs : predecessor.node.endMs;
  const unblocker = (node: Node): { node: Node; edge: "rmeta" | "full" | "run" } | undefined => node.predecessors.reduce<{ node: Node; edge: "rmeta" | "full" | "run" } | undefined>((best, candidate) => !best || available(candidate) > available(best) ? candidate : best, undefined);
  const segments: CriticalSegmentV1[] = [];
  for (let current: Node | undefined = [...nodes.values()].sort((left, right) => right.endMs - left.endMs)[0]; current; ) {
    const cause = unblocker(current);
    segments.unshift({ id: current.id, label: current.label, edge: cause?.edge ?? "root", readyMs: cause ? available(cause) : current.startMs, startMs: current.startMs, endMs: current.endMs });
    current = cause?.node;
  }
  const remainingMs = new Map<string, number>(), visiting = new Set<Node>();
  const remaining = (node: Node): number => {
    const known = remainingMs.get(node.id);
    if (known !== undefined) return known;
    if (visiting.has(node)) return 0;
    visiting.add(node);
    const value = Math.max(0, node.endMs - node.startMs) + Math.max(0, ...node.successors.map(remaining));
    visiting.delete(node);
    remainingMs.set(node.id, value);
    return value;
  };
  for (const node of nodes.values()) remaining(node);
  return { segments, remainingMs };
}

/** 💤️ Idle windows: stretches of at least `idleWindowMs` below `idleCpuPercent`, each with its dominant cause. */
export function idleWindowsV1(rows: readonly TimelineRowV1[]): readonly Readonly<{ startMs: number; endMs: number; meanCpuPercent: number; cause: "memory-blocked" | "stage-blocked" | "dependency-blocked" }>[] {
  const windows: { startMs: number; endMs: number; meanCpuPercent: number; cause: "memory-blocked" | "stage-blocked" | "dependency-blocked" }[] = [];
  let run: TimelineRowV1[] = [];
  const close = (): void => {
    if (run.length >= 2 && run.at(-1)!.atMs - run[0]!.atMs >= TELEMETRY_POLICY_V1.idleWindowMs) {
      const count = (predicate: (row: TimelineRowV1) => boolean): number => run.filter(predicate).length;
      const memory = count((row) => row.admissionWaiting > 0 || row.frozen), stage = count((row) => row.stagesWaiting > 0);
      windows.push({ startMs: run[0]!.atMs, endMs: run.at(-1)!.atMs, meanCpuPercent: Math.round(run.reduce((sum, row) => sum + row.cpuPercent, 0) / run.length), cause: memory * 2 >= run.length ? "memory-blocked" : stage * 2 >= run.length ? "stage-blocked" : "dependency-blocked" });
    }
    run = [];
  };
  for (const row of rows) { if (row.cpuPercent < TELEMETRY_POLICY_V1.idleCpuPercent) run.push(row); else close(); }
  close();
  return windows;
}

/** 🕸️ Relink fan-out (L9): for every library, the planned component final units (primary cdylibs) whose closure contains it and their mean duration. */
export function relinkFanOutV1(units: readonly UnitRecordV1[]): readonly Readonly<{ library: string; components: number; meanComponentMs: number; costMs: number }>[] {
  const byArtifact = new Map(units.filter((unit) => unit.exit === 0).map((unit) => [unit.artifact, unit]));
  const components = [...byArtifact.values()].filter((unit) => unit.primary && unit.crateTypes.includes("cdylib") && unit.target !== "host");
  const fanOut = new Map<string, number[]>();
  for (const component of components) {
    const seen = new Set<string>(), stack = [component];
    while (stack.length) {
      const unit = stack.pop()!;
      for (const dependency of unit.dependencies) {
        const producer = dependency.artifacts.map((artifact) => byArtifact.get(artifact)).find(Boolean);
        if (!producer || seen.has(producer.artifact)) continue;
        seen.add(producer.artifact);
        stack.push(producer);
      }
    }
    for (const artifact of seen) { const unit = byArtifact.get(artifact)!; const name = `${unit.package}@${unit.crate} (${unit.profile})`; fanOut.set(name, [...(fanOut.get(name) ?? []), component.endMs - component.startMs]); }
  }
  return [...fanOut].map(([library, durations]) => { const mean = durations.reduce((sum, value) => sum + value, 0) / durations.length; return { library, components: durations.length, meanComponentMs: Math.round(mean), costMs: Math.round(mean * durations.length) }; }).sort((left, right) => right.costMs - left.costMs || left.library.localeCompare(right.library));
}

/** 🗒️ What one build report summarises. */
export type BuildReportInputV1 = Readonly<{ title: string; startedMs: number; endedMs: number; timeline: readonly TimelineRowV1[]; units: readonly UnitRecordV1[]; runs: readonly BuildScriptRunV1[]; stages: readonly StageRecordV1[]; passes: readonly Readonly<{ id: string; manifest: string; profile: string; target: string; startMs: number; endMs: number; exit: number; failed: readonly string[] }>[]; critical: CriticalPathV1; sections?: readonly Readonly<{ title: string; rows: readonly (readonly [string, string | number])[] }>[] }>;

/** 🗒️ Renders `report.md`: wall, CPU-h and utilisation by phase, passes, critical path, top units by CPU and peak, idle windows with cause, relink fan-out and stage overlap. */
export function renderBuildReportV1(input: BuildReportInputV1): string {
  const seconds = (ms: number): string => (ms / 1000).toFixed(1), gib = (bytes: number): string => (bytes / 1024 ** 3).toFixed(2);
  const ok = input.units.filter((unit) => unit.exit === 0), cpuMs = ok.reduce((sum, unit) => sum + unit.userMs + unit.sysMs, 0);
  const firstStart = Math.min(...ok.map((unit) => unit.startMs), input.endedMs), lastEnd = Math.max(...ok.map((unit) => unit.endMs), input.startedMs);
  const mean = (from: number, to: number): string => { const rows = input.timeline.filter((row) => row.atMs >= from && row.atMs <= to); return rows.length ? `${(rows.reduce((sum, row) => sum + row.cpuPercent, 0) / rows.length).toFixed(1)} %` : "n/a"; };
  const offset = (ms: number): string => seconds(ms - input.startedMs);
  const table = (header: readonly string[], rows: readonly (readonly (string | number)[])[]): string => [`| ${header.join(" | ")} |`, `|${header.map(() => "---").join("|")}|`, ...rows.map((row) => `| ${row.join(" | ")} |`)].join("\n");
  const stageMs = input.stages.reduce((sum, stage) => sum + stage.endMs - stage.startMs, 0), streamed = input.stages.filter((stage) => stage.startMs < lastEnd), streamedMs = streamed.reduce((sum, stage) => sum + stage.endMs - stage.startMs, 0);
  const label = (unit: UnitRecordV1): string => `${unit.package}@${unit.crate} (${unit.crateTypes.join(",")}, ${unit.target}, ${unit.profile})`;
  return [
    `# ${input.title}`,
    "",
    "## Summary",
    "",
    table(["Metric", "Value"], [
      ["Wall", `${seconds(input.endedMs - input.startedMs)} s`],
      ["Compiled units", ok.length],
      ["Failed units", input.units.length - ok.length],
      ["Unit CPU", `${(cpuMs / 3_600_000).toFixed(2)} CPU-h`],
      ["Mean CPU (whole build)", mean(input.startedMs, input.endedMs)],
      ["Peak rustc resident (sum)", `${gib(Math.max(0, ...input.timeline.map((row) => row.rustcRssBytes)))} GiB`],
      ["Frozen samples", input.timeline.filter((row) => row.frozen).length],
      ["Peak swap used", `${gib(Math.max(0, ...input.timeline.map((row) => row.swapUsedBytes ?? 0)))} GiB`],
    ]),
    "",
    "## Phases",
    "",
    table(["Phase", "From s", "To s", "Mean CPU"], [["head", offset(input.startedMs), offset(firstStart), mean(input.startedMs, firstStart)], ["compile", offset(firstStart), offset(lastEnd), mean(firstStart, lastEnd)], ["tail", offset(lastEnd), offset(input.endedMs), mean(lastEnd, input.endedMs)]]),
    "",
    "## Passes",
    "",
    table(["Pass", "Profile", "Target", "Start s", "Wall s", "Exit", "Failed packages"], input.passes.map((pass) => [pass.id, pass.profile, pass.target, offset(pass.startMs), seconds(pass.endMs - pass.startMs), pass.exit, pass.failed.join(", ") || "-"])),
    "",
    "## Critical path",
    "",
    table(["Unit", "Unblocked by", "Ready s", "Start s", "Wait s", "Duration s"], input.critical.segments.map((segment) => [segment.label, segment.edge, offset(segment.readyMs), offset(segment.startMs), seconds(Math.max(0, segment.startMs - segment.readyMs)), seconds(segment.endMs - segment.startMs)])),
    "",
    `Critical path length: ${seconds((input.critical.segments.at(-1)?.endMs ?? input.startedMs) - (input.critical.segments[0]?.startMs ?? input.startedMs))} s.`,
    "",
    `## Top ${TELEMETRY_POLICY_V1.topUnits} units by CPU`,
    "",
    table(["Unit", "CPU s", "Wall s", "Peak GiB", "Admission wait s"], [...ok].sort((left, right) => right.userMs + right.sysMs - left.userMs - left.sysMs).slice(0, TELEMETRY_POLICY_V1.topUnits).map((unit) => [label(unit), seconds(unit.userMs + unit.sysMs), seconds(unit.endMs - unit.startMs), gib(unit.peakRssBytes), seconds(unit.admittedMs - unit.requestMs)])),
    "",
    `## Top ${TELEMETRY_POLICY_V1.topUnits} units by peak`,
    "",
    table(["Unit", "Peak GiB", "Predicted GiB", "Wall s"], [...ok].sort((left, right) => right.peakRssBytes - left.peakRssBytes).slice(0, TELEMETRY_POLICY_V1.topUnits).map((unit) => [label(unit), gib(unit.peakRssBytes), gib(unit.predictedBytes), seconds(unit.endMs - unit.startMs)])),
    "",
    "## Idle windows",
    "",
    table(["From s", "To s", "Mean CPU", "Cause"], idleWindowsV1(input.timeline).map((window) => [offset(window.startMs), offset(window.endMs), `${window.meanCpuPercent} %`, window.cause])),
    "",
    "## Relink fan-out (L9)",
    "",
    table(["Library", "Components", "Mean component unit s", "Fan-out cost s"], relinkFanOutV1(input.units).slice(0, TELEMETRY_POLICY_V1.fanOutRows).map((row) => [row.library, row.components, seconds(row.meanComponentMs), seconds(row.costMs)])),
    "",
    "## Stage overlap",
    "",
    `${input.stages.length} stage(s), ${seconds(stageMs)} s of stage time; ${streamed.length} (${seconds(streamedMs)} s) started while units still compiled, ${input.stages.length - streamed.length} (${seconds(stageMs - streamedMs)} s) after the last unit.`,
    "",
    ...(input.sections ?? []).flatMap((section) => [`## ${section.title}`, "", table(["Metric", "Value"], section.rows), ""]),
  ].join("\n");
}

/** 💾️ Writes `units.json` (units, build-script runs, critical path) and `report.md` into the report directory. */
export function writeBuildReportV1(directory: string, input: BuildReportInputV1): string {
  mkdirSync(directory, { recursive: true });
  const report = join(directory, TELEMETRY_POLICY_V1.reportFile);
  writeFileSync(join(directory, TELEMETRY_POLICY_V1.unitsFile), `${JSON.stringify({ units: input.units, runs: input.runs, stages: input.stages, criticalPath: input.critical.segments }, null, 1)}\n`);
  writeFileSync(report, renderBuildReportV1(input));
  return report;
}

/** 📖️ Reads a written timeline back, validating every row. */
export function readTimelineV1(path: string): TimelineRowV1[] {
  return existsSync(path) ? readFileSync(path, "utf8").split("\n").filter(Boolean).map((line) => admitTelemetryV1<TimelineRowV1>("TimelineRowV1", JSON.parse(line))) : [];
}
