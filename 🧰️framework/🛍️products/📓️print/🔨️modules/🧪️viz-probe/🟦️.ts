import{inspectPrintCompilerRefusalV1,type PrintCompilerRefusalResultV1}from"../🖨️tectonic-template-compilation/📥️refusal/🟦️.ts";
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import { stagePrintSources, printCompilerName } from "../📥️source-staging/🟦️.ts";
import{withCompiledPrintProbeV1,type CompiledPrintProbeV1}from"../🖨️tectonic-template-compilation/🟦️.ts";
import type{PrintProbeResourceV1}from"../🖨️tectonic-template-compilation/📋️intent/🟦️.ts";
import{withPrintExecutionV1,withPrintStorageTaskV1,type PrintExecutionV1}from"../../📦️artifacts/🎛️operation/🟦️.ts";
import type{ScriptInvocation}from"../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type{ResourceLeaseControl}from"@semio-tech/process/leases";


//#region 🧪️ProbeProtocol
/** 🧪️ One line of `<job>.probe.jsonl` — the `ProbeRecord` of `🧬️schema/🔣️.json`. */
export type ProbeRecord = Readonly<{ case: string; scenario: string; key: string; values: readonly (number | string)[] }>;

/** 👁️ A stable, comparable view of a probe run: every key mapped onto its values in emission order. */
export type ProbeProjection = Readonly<Record<string, readonly (number | string)[]>>;

/** ⚙️ How one probe document is compiled. `workDir` must be cache-local, never inside the repository tree. */
export type VizProbeOptions=Readonly<{execution:PrintExecutionV1;caseName?:string;scenario?:string;sources?:readonly string[];resources?:readonly PrintProbeResourceV1[]}>;

/** 📝️ One statement of a programmatically rendered probe document. */
export type VizProbeStatement =
  | Readonly<{ command: string; arguments: readonly string[]; optional?: string }>
  | Readonly<{ values: Readonly<{ key: string; clist: string }> }>
  | Readonly<{ evaluate: Readonly<{ key: string; expressions: readonly string[] }> }>
  | Readonly<{ precision: number }>
  | Readonly<{ text: Readonly<{ key: string; value: string }> }>
  | Readonly<{ points: Readonly<{ key: string; coordinates: readonly (readonly [number | string, number | string])[] }> }>
  | Readonly<{ raw: string }>;

/** 🏓️ A probe document built from Gherkin data-table vectors instead of a committed fixture. */
export type VizProbeDocumentSpec = Readonly<{
  case: string;
  scenario: string;
  packages?: readonly string[];
  documentClass?: string;
  documentClassOptions?: string;
  preamble?: readonly string[];
  geometry?: boolean;
  body: readonly VizProbeStatement[];
}>;

// 📍 Resolved from THIS file rather than from the process cwd: an adapter is spawned by the test
// platform in a work directory that has no relation to the repository tree.
const productRoot = join(import.meta.dirname, "..", "..");
const workspaceRoot = join(productRoot, "..", "..", "..");
const schemaPath = join(productRoot, "🧬️schema", "🔣️.json");
//#endregion 🧪️ProbeProtocol

//#region ✅️Validation
type ProbeRecordSchema = Readonly<{ required: readonly string[]; properties: Readonly<Record<string, unknown>>; additionalProperties: boolean }>;

let schemaCache: ProbeRecordSchema | undefined;

function probeRecordSchema(): ProbeRecordSchema {
  schemaCache ??= (JSON.parse(readFileSync(schemaPath, "utf8")) as { $defs: Record<string, ProbeRecordSchema> }).$defs.ProbeRecord;
  if (schemaCache === undefined) throw new Error(`🧬️schema/🔣️.json declares no ProbeRecord definition`);
  return schemaCache;
}

/** ✅️ Validates parsed lines against the schema's `ProbeRecord`, naming the first offending record. */
export function validateProbeRecords(value: unknown): ProbeRecord[] {
  const schema = probeRecordSchema();
  const allowed = new Set(Object.keys(schema.properties));
  if (!Array.isArray(value)) throw new Error("probe records must be an array");
  return value.map((entry, index) => {
    const where = `probe record ${index}`;
    if (entry === null || typeof entry !== "object" || Array.isArray(entry)) throw new Error(`${where} is not an object`);
    const record = entry as Record<string, unknown>;
    for (const key of schema.required) if (typeof record[key] === "undefined") throw new Error(`${where} is missing "${key}"`);
    if (schema.additionalProperties === false) for (const key of Object.keys(record)) if (!allowed.has(key)) throw new Error(`${where} carries unknown property "${key}"`);
    for (const key of ["case", "scenario", "key"]) if (typeof record[key] !== "string") throw new Error(`${where} property "${key}" is not a string`);
    if (!Array.isArray(record.values)) throw new Error(`${where} property "values" is not an array`);
    for (const item of record.values as unknown[]) if (typeof item !== "number" && typeof item !== "string") throw new Error(`${where} carries a value that is neither number nor string`);
    return { case: record.case as string, scenario: record.scenario as string, key: record.key as string, values: record.values as readonly (number | string)[] };
  });
}

/** 📤️ Parses a `.probe.jsonl` text into validated records; an empty stream is an error, never an empty pass. */
export function parseProbeJsonLines(text: string): ProbeRecord[] {
  const lines = text.split(/\r?\n/).map((line) => line.trim()).filter((line) => line.length > 0);
  if (lines.length === 0) throw new Error("probe stream is empty — the document emitted no \\SemioVizProbe… record");
  return validateProbeRecords(lines.map((line, index) => {
    try {
      return JSON.parse(line) as unknown;
    } catch (error) {
      throw new Error(`probe line ${index + 1} is not JSON: ${line} (${(error as Error).message})`);
    }
  }));
}
//#endregion ✅️Validation

//#region 🧮️Projection
/** 🧮️ Groups records into the comparable projection, optionally narrowed to one scenario. */
export function probeProjection(records: readonly ProbeRecord[], scenario?: string): ProbeProjection {
  const selected = scenario === undefined ? records : records.filter((record) => record.scenario === scenario);
  if (selected.length === 0) throw new Error(`probe run carries no record for scenario ${JSON.stringify(scenario ?? "*")}`);
  const projection: Record<string, (number | string)[]> = {};
  for (const record of selected) projection[record.key] = [...(projection[record.key] ?? []), ...record.values];
  return Object.fromEntries(Object.keys(projection).sort().map((key) => [key, projection[key]!]));
}

/** 🔢️ Rounds every numeric value onto a decimal grid so both sides of a comparison share one resolution. */
export function roundProbeNumbers(records: readonly ProbeRecord[], decimals: number): ProbeRecord[] {
  if (!Number.isInteger(decimals) || decimals < 0 || decimals > 15) throw new Error(`probe rounding needs an integer 0…15 decimal count, got ${decimals}`);
  const factor = 10 ** decimals;
  return records.map((record) => ({ ...record, values: record.values.map((value) => (typeof value === "number" ? normalizeZero(Math.round(value * factor) / factor) : value)) }));
}

/** 🔮️ Rounds every number of an already-built projection, for an ORACLE side that never sees records. */
export function roundProjectionNumbers(projection: ProbeProjection, decimals: number): ProbeProjection {
  const single = roundProbeNumbers(Object.entries(projection).map(([key, values]) => ({ case: "", scenario: "", key, values })), decimals);
  return Object.fromEntries(single.map((record) => [record.key, record.values]));
}

function normalizeZero(value: number): number {
  return value === 0 ? 0 : value;
}
//#endregion 🧮️Projection

//#region 📝️DocumentRendering
/** 🖌️ Renders a probe `.tex` from a spec, so vectors can stay in the Gherkin data table. */
export function renderVizProbeDocument(spec: VizProbeDocumentSpec): string {
  const packages = spec.packages ?? ["semio-viz"];
  const lines = [
    `\\documentclass${spec.documentClassOptions === undefined ? "" : `[${spec.documentClassOptions}]`}{${spec.documentClass ?? "article"}}`,
    ...[...new Set(["semio-viz-probe", ...packages])].map((name) => `\\usepackage{${name}}`),
    ...(spec.preamble ?? []),
    "\\begin{document}",
    `\\SemioVizProbeBegin{${spec.case}}{${spec.scenario}}`,
    ...(spec.geometry === true ? ["\\SemioVizProbeOn"] : []),
    "\\SemioVizProbePage",
    ...spec.body.map(renderStatement),
    "\\SemioVizProbeEnd",
    "\\end{document}",
    "",
  ];
  return lines.join("\n");
}

function renderStatement(statement: VizProbeStatement): string {
  if ("raw" in statement) return statement.raw;
  if ("values" in statement) return `\\SemioVizProbeValues{${statement.values.key}}{${statement.values.clist}}`;
  if ("evaluate" in statement) return `\\SemioVizProbeEval{${statement.evaluate.key}}{${statement.evaluate.expressions.join(",")}}`;
  if ("precision" in statement) return `\\SemioVizProbePrecision{${statement.precision}}`;
  if ("text" in statement) return `\\SemioVizProbeString{${statement.text.key}}{${statement.text.value}}`;
  if ("points" in statement) return `\\SemioVizProbePoints{${statement.points.key}}{${statement.points.coordinates.map(([x, y]) => `{${x},${y}}`).join("")}}`;
  const optional = statement.optional === undefined ? "" : `[${statement.optional}]`;
  return `\\${statement.command}${optional}${statement.arguments.map((value) => `{${value}}`).join("")}`;
}

/** 📖️ Carries parsed records while the actual compiled input readers and scratch remain held. */
export interface CompiledVizProbeV1{readonly records:readonly ProbeRecord[];readonly probe:CompiledPrintProbeV1}
/** 🔨️ Compiles generated DATA and keeps every expensive consumer inside the retained callback. */
export function withCompiledVizProbeDocumentV1<T>(spec:VizProbeDocumentSpec,options:VizProbeOptions,body:(compiled:CompiledVizProbeV1)=>Promise<T>):Promise<T>{const source=renderVizProbeDocument(spec),execution=Object.freeze({...options.execution}),selection=Object.freeze({caseName:options.caseName??spec.case,scenario:Object.hasOwn(options,"scenario")?options.scenario:spec.scenario}),resources=options.resources;return withCompiledPrintProbeV1(execution,{version:1,kind:"probe",name:"probe.tex",source,...(resources?{resources}:{})},async probe=>{const text=new TextDecoder("utf-8",{fatal:true}).decode(await probe.read("probe.probe.jsonl",1048576)),records=selectProbeRecords(parseProbeJsonLines(text),{execution,...selection});return body(Object.freeze({records:Object.freeze(records),probe}));});}
/** 📹️ Returns finite validated records as DATA, with no escaped compiler path or memoized pending work. */
export function compileVizProbeDocument(spec:VizProbeDocumentSpec,options:VizProbeOptions):Promise<ProbeRecord[]>{return withCompiledVizProbeDocumentV1(spec,options,async compiled=>[...compiled.records]);}
/** 🧭️ The part of a test adapter context that owns a probe run: the workspace, the original invocation and its phase control. */
export type VizProbeContextV1=Readonly<{repoRoot:string;invocation:ScriptInvocation;phaseControl:ResourceLeaseControl}>;
/** 🆕️ Compiles one probe document inside a fresh probe execution of the adapter context that already owns the invocation and its phase control. */
export function compileVizProbeDocumentInContext(context:VizProbeContextV1,spec:VizProbeDocumentSpec,options:Omit<VizProbeOptions,"execution">={}):Promise<ProbeRecord[]>{return withPrintExecutionV1({workspace:context.repoRoot,invocation:context.invocation,phaseControl:context.phaseControl,kind:"probe"},execution=>compileVizProbeDocument(spec,{...options,execution}));}
/** 📚️ Compiles an explicit workspace Source probe and retains all consumers through its callback. */
export function withCompiledVizProbeV1<T>(texPath:string,options:VizProbeOptions,body:(compiled:CompiledVizProbeV1)=>Promise<T>):Promise<T>{const path=texPath,execution=Object.freeze({...options.execution}),selection=Object.freeze({caseName:options.caseName,scenario:options.scenario}),sources=Object.freeze([...(options.sources??[])]);return withPrintStorageTaskV1(execution.operation,execution.phaseControl,async(operation,context)=>{const sourceRoot=relative(context.workspace,dirname(path)).replaceAll("\\","/"),name=basename(path);return withCompiledPrintProbeV1({operation,phaseControl:context.phaseControl},{version:1,kind:"document",namespace:"probe",id:"source-probe",sourceRoot,texPath:name,sources:[name,...sources],dark:false},async probe=>{const stream=basename(printCompilerName(name),".tex")+".probe.jsonl",records=selectProbeRecords(parseProbeJsonLines(new TextDecoder("utf-8",{fatal:true}).decode(await probe.read(stream,1048576))),{execution,...selection});return body(Object.freeze({records:Object.freeze(records),probe}));});});}
/** 🎙️ Projects a consumed Source compilation into finite record DATA. */
export function compileVizProbe(texPath:string,options:VizProbeOptions):Promise<ProbeRecord[]>{return withCompiledVizProbeV1(texPath,options,async compiled=>[...compiled.records]);}

function selectProbeRecords(records: readonly ProbeRecord[], options: VizProbeOptions): ProbeRecord[] {
  const selected = records.filter((record) => (options.caseName === undefined || record.case === options.caseName) && (options.scenario === undefined || record.scenario === options.scenario));
  if (selected.length === 0) throw new Error(`probe stream carries no record for case ${JSON.stringify(options.caseName ?? "*")} scenario ${JSON.stringify(options.scenario ?? "*")}; it holds ${[...new Set(records.map((record) => `${record.case}/${record.scenario}`))].join(", ")}`);
  return selected;
}

/** 🔌️ Convenience for an adapter's `subject`: compile, round, project, in one call. */
export async function vizProbeProjection(texPath: string, options: VizProbeOptions & { readonly decimals?: number }): Promise<ProbeProjection> {
  const records = await compileVizProbe(texPath, options);
  return probeProjection(options.decimals === undefined ? records : roundProbeNumbers(records, options.decimals));
}

/** 📁️ Repository-relative form of a path, for diagnostics that must stay platform-neutral. */
export function vizProbeRelativePath(path: string): string {
  return relative(workspaceRoot, path).replaceAll("\\", "/");
}
//#endregion 🖨️Compilation

/** 🛑️ Observes an actual failed child compiler as finite diagnostics while the parent retains its paid execution. */
export function inspectVizProbeDocumentRefusalV1(spec:VizProbeDocumentSpec,options:VizProbeOptions):Promise<PrintCompilerRefusalResultV1>{return inspectPrintCompilerRefusalV1(options.execution,{version:1,kind:"probe",name:"probe.tex",source:renderVizProbeDocument(spec),...(options.resources?{resources:options.resources}:{})});}
