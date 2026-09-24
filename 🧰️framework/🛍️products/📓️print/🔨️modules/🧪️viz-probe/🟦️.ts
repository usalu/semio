import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import { stagePrintSources, printCompilerName } from "../📥️source-staging/🟦️.ts";
import { compilePrintTexOnce } from "../🖨️tectonic-template-compilation/🟦️.ts";
import { prepareTectonic } from "../🖨️tectonic-template-compilation/🔧️toolchain/📜️script.ts";

//#region 🧪️ProbeProtocol
/** 🧪️ One line of `<job>.probe.jsonl` — the `ProbeRecord` of `🧬️schema/🔣️.json`. */
export type ProbeRecord = Readonly<{ case: string; scenario: string; key: string; values: readonly (number | string)[] }>;

/** 🧪️ A stable, comparable view of a probe run: every key mapped onto its values in emission order. */
export type ProbeProjection = Readonly<Record<string, readonly (number | string)[]>>;

/** 🧪️ How one probe document is compiled. `workDir` must be cache-local, never inside the repository tree. */
export type VizProbeOptions = Readonly<{
  workDir: string;
  caseName?: string;
  scenario?: string;
  jobName?: string;
  extraSources?: readonly string[];
  keepWorkDir?: boolean;
}>;

/** 📝️ One statement of a programmatically rendered probe document. */
export type VizProbeStatement =
  | Readonly<{ command: string; arguments: readonly string[]; optional?: string }>
  | Readonly<{ values: Readonly<{ key: string; clist: string }> }>
  | Readonly<{ evaluate: Readonly<{ key: string; expressions: readonly string[] }> }>
  | Readonly<{ precision: number }>
  | Readonly<{ text: Readonly<{ key: string; value: string }> }>
  | Readonly<{ points: Readonly<{ key: string; coordinates: readonly (readonly [number | string, number | string])[] }> }>
  | Readonly<{ raw: string }>;

/** 📝️ A probe document built from Gherkin data-table vectors instead of a committed fixture. */
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
const productRoot = join(import.meta.dir, "..", "..");
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

/** 🔢️ Rounds every number of an already-built projection, for an ORACLE side that never sees records. */
export function roundProjectionNumbers(projection: ProbeProjection, decimals: number): ProbeProjection {
  const single = roundProbeNumbers(Object.entries(projection).map(([key, values]) => ({ case: "", scenario: "", key, values })), decimals);
  return Object.fromEntries(single.map((record) => [record.key, record.values]));
}

function normalizeZero(value: number): number {
  return value === 0 ? 0 : value;
}
//#endregion 🧮️Projection

//#region 📝️DocumentRendering
/** 📝️ Renders a probe `.tex` from a spec, so vectors can stay in the Gherkin data table. */
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

/** 📝️ Writes a rendered probe document into `workDir` and returns its path, ready for `compileVizProbe`. */
export function writeVizProbeDocument(spec: VizProbeDocumentSpec, workDir: string, fileName = `${spec.scenario}.tex`): string {
  mkdirSync(workDir, { recursive: true });
  const path = join(workDir, fileName);
  writeFileSync(path, renderVizProbeDocument(spec), "utf8");
  return path;
}
//#endregion 📝️DocumentRendering

//#region 🖨️Compilation
/** 🗃️ Records of every probe already compiled in this process, so the scenarios of one adapter share a run. */
const COMPILED_PROBES = new Map<string, Promise<ProbeRecord[]>>();

/** 🖨️ Absolute path of the repo tectonic — downloaded and unpacked on first use, never the system one silently. */
export async function vizProbeTectonic(): Promise<string> {
  return await prepareTectonic();
}

/**
 * 🧪️ Stages one probe `.tex` into a cache-local work directory, compiles it with the repo tectonic
 * and returns the validated records of `<job>.probe.jsonl`.
 *
 * @see 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-probe.sty
 */
export async function compileVizProbe(texPath: string, options: VizProbeOptions): Promise<ProbeRecord[]> {
  if (!existsSync(texPath)) throw new Error(`probe document does not exist: ${texPath}`);
  const key = probeCompilationKey(texPath, options);
  const pending = COMPILED_PROBES.get(key) ?? compileProbeStream(texPath, options);
  COMPILED_PROBES.set(key, pending);
  return selectProbeRecords(await pending, options);
}

/** 🗝️ Identity of one compilation: the source bytes, its companions, the job name and the work directory. */
function probeCompilationKey(texPath: string, options: VizProbeOptions): string {
  const source = readFileSync(texPath, "utf8");
  let hash = 5381;
  for (let index = 0; index < source.length; index += 1) hash = ((hash * 33) ^ source.charCodeAt(index)) >>> 0;
  return [texPath, hash.toString(36), (options.extraSources ?? []).join("|"), options.jobName ?? "", options.workDir, options.keepWorkDir === true ? "keep" : "sweep"].join(" ");
}

/** 🖨️ The compilation itself, awaited once per identity so a multi-scenario adapter runs tectonic once. */
async function compileProbeStream(texPath: string, options: VizProbeOptions): Promise<ProbeRecord[]> {
  const stageRoot = join(options.workDir, "🧪️probe-source");
  const outDirectory = join(options.workDir, "🧪️probe-out");
  const sourceRoot = dirname(texPath);
  const entries = [basename(texPath), ...(options.extraSources ?? [])];
  const staged = stagePrintSources(sourceRoot, entries, stageRoot);
  const stagedTex = staged.get(basename(texPath).replaceAll("\\", "/"));
  if (stagedTex === undefined) throw new Error(`staging did not produce ${basename(texPath)} under ${stageRoot}`);
  rmSync(outDirectory, { recursive: true, force: true });
  mkdirSync(outDirectory, { recursive: true });
  await compilePrintTexOnce(stagedTex, outDirectory, stageRoot);
  const jobName = options.jobName ?? basename(printCompilerName(basename(texPath)), ".tex");
  const stream = join(outDirectory, `${jobName}.probe.jsonl`);
  if (!existsSync(stream)) throw new Error(`no probe stream at ${stream} — the run produced ${readdirSync(outDirectory).join(", ") || "nothing"}. A probe document that typesets no page writes no files at all, so it must call \\SemioVizProbePage.`);
  const records = parseProbeJsonLines(readFileSync(stream, "utf8"));
  if (options.keepWorkDir !== true) rmSync(stageRoot, { recursive: true, force: true });
  return records;
}

/** 🧪️ Compiles a spec-rendered probe document — the data-table path, with no committed fixture. */
export async function compileVizProbeDocument(spec: VizProbeDocumentSpec, options: VizProbeOptions): Promise<ProbeRecord[]> {
  const sourceDir = join(options.workDir, "🧪️probe-input");
  const texPath = writeVizProbeDocument(spec, sourceDir);
  return await compileVizProbe(texPath, { caseName: spec.case, scenario: spec.scenario, ...options });
}

function selectProbeRecords(records: readonly ProbeRecord[], options: VizProbeOptions): ProbeRecord[] {
  const selected = records.filter((record) => (options.caseName === undefined || record.case === options.caseName) && (options.scenario === undefined || record.scenario === options.scenario));
  if (selected.length === 0) throw new Error(`probe stream carries no record for case ${JSON.stringify(options.caseName ?? "*")} scenario ${JSON.stringify(options.scenario ?? "*")}; it holds ${[...new Set(records.map((record) => `${record.case}/${record.scenario}`))].join(", ")}`);
  return selected;
}

/** 🧪️ Convenience for an adapter's `subject`: compile, round, project, in one call. */
export async function vizProbeProjection(texPath: string, options: VizProbeOptions & { readonly decimals?: number }): Promise<ProbeProjection> {
  const records = await compileVizProbe(texPath, options);
  return probeProjection(options.decimals === undefined ? records : roundProbeNumbers(records, options.decimals));
}

/** 📁️ The cache-local directory a probe of this case and scenario compiles in. */
export function vizProbeWorkDirectory(caseName: string, scenario: string): string {
  return join(workspaceRoot, ".🧬semio", "🦑️repo", "⚡️cache", "tests", "viz-probe", caseName, scenario);
}

/** 📁️ Repository-relative form of a path, for diagnostics that must stay platform-neutral. */
export function vizProbeRelativePath(path: string): string {
  return relative(workspaceRoot, path).replaceAll("\\", "/");
}
//#endregion 🖨️Compilation
