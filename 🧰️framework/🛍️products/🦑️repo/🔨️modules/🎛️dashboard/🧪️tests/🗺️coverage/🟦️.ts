/**
 * 🗺️ Coverage of the command registry on the real monorepo (feature: ./🥒️.feature). The Rust registry
 * answers `semio commands --json --all` and `semio run <id> --dry-run`; the independent TypeScript oracle
 * (./🔮️oracle/🟦️.ts) reads the Nx project graph, the manifests, the playground catalog and the open tickets
 * on its own, and Ajv validates every declaration against the registry schema.
 *
 * Environment: `SEMIO_TEST_CLI` the binary; `SEMIO_COVERAGE_FULL=1` dry-runs every id (hours with a debug
 * build), else every non-trivial id plus `SEMIO_COVERAGE_SAMPLE` (default 400) plain targets.
 */
import { beforeAll, describe, expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { dashboard, pool, repository, semio, type Run } from "../🧭️journeys/🧰️support/🟦️.ts";
import { Oracle, compare, runArguments, type Json, type Launch, type Selection } from "./🔮️oracle/🟦️.ts";

type Entry = { id: string; kind: string; verb: string; label: string; listed: boolean; longRunning: boolean; source: string; parameters: { id: string; kind: string; required: boolean; origin: string; values: string[]; default?: string }[]; ready?: { port: number; path: string; printed?: boolean } };
const sharing = JSON.parse(readFileSync(join(dashboard, "🧫️fixtures/🗺️coverage/🔣️.json"), "utf8")) as { portSharing: Record<string, string> };
const width = Number(process.env.SEMIO_COVERAGE_WIDTH ?? 6);
const cwd = repository;
const json = (run: Run) => JSON.parse(run.stdout);

let oracle: Oracle;
let entries: Entry[];
let byId: Map<string, Entry>;
let check: Run;

beforeAll(async () => {
  oracle = new Oracle(repository);
  const listed = await semio(["commands", "--json", "--all", "--refresh"], { cwd, timeoutMs: 300_000 });
  if (listed.code !== 0) throw new Error(`semio commands --json --all exited ${listed.code}: ${listed.stderr}`);
  entries = JSON.parse(listed.stdout);
  byId = new Map(entries.map((entry) => [entry.id, entry]));
  check = await semio(["commands", "--check"], { cwd, timeoutMs: 600_000 });
}, 900_000);

const dryRun = async (selection: Selection): Promise<{ run: Run; launch?: Launch }> => {
  const run = await semio(["run", ...runArguments(selection, ["--dry-run"])], { cwd, timeoutMs: 300_000 });
  return { run, launch: run.code === 0 ? (json(run) as Launch) : undefined };
};

describe("the registry covers the whole monorepo", () => {
  test("Every Nx target, playground variant, declaration and open ticket command is listed", () => {
    const wanted = oracle.ids();
    const missing = [...wanted.keys()].filter((id) => !byId.has(id));
    expect(missing, `${missing.length} oracle ids are not listed:\n${missing.slice(0, 40).join("\n")}`).toEqual([]);
    const phantom = entries.filter((entry) => entry.kind !== "repo" && !wanted.has(entry.id)).map((entry) => entry.id);
    expect(phantom, `${phantom.length} listed ids exist in no source:\n${phantom.slice(0, 40).join("\n")}`).toEqual([]);
    for (const kind of ["target", "playground", "tool", "compound", "group", "ticket"]) expect(entries.filter((entry) => entry.kind === kind).length, kind).toBe([...wanted.values()].filter((value) => value === kind).length);
    expect(entries.filter((entry) => entry.kind === "playground")).toHaveLength(oracle.playgrounds.length);
    expect(entries.filter((entry) => entry.kind === "repo").length).toBeGreaterThan(0);
  });

  test("The published project graph has no errors and no fixture manifest is a project", () => {
    const graph = JSON.parse(readFileSync(join(repository, ".nx/workspace-data/project-graph.json"), "utf8")) as { errors?: unknown[]; nodes: Record<string, { data: { root: string } }> };
    expect(graph.errors ?? [], `the graph carries ${(graph.errors ?? []).length} errors`).toEqual([]);
    const fixtureRoots = Object.entries(graph.nodes).filter(([, node]) => node.data.root.split("/").some((part) => /^[^\p{L}\p{N}]*fixtures$/u.test(part))).map(([name]) => name);
    expect(fixtureRoots, "fixture manifests became Nx projects (add them to .nxignore, and never name a fixture project like a real one)").toEqual([]);
    const names = new Map<string, number>();
    for (const project of oracle.projects.values()) if (project.manifest?.name) names.set(project.manifest.name, (names.get(project.manifest.name) ?? 0) + 1);
    expect([...names].filter(([, count]) => count > 1)).toEqual([]);
  });

  test("The workspace check reports zero problems", () => {
    const summary = /(\d+) commands, (\d+) problems/.exec(check.stdout);
    expect(summary, `${check.stdout}\n${check.stderr}`).not.toBeNull();
    expect(Number(summary![2]), `semio commands --check:\n${check.stderr}`).toBe(0);
    expect(check.code).toBe(0);
    expect(Number(summary![1])).toBe(entries.length);
  });

  test("Every dashboard declaration of every project manifest and ticket document is valid", () => {
    const schema = JSON.parse(readFileSync(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"), "utf8"));
    const ajv = new Ajv2020({ strict: false, allErrors: true });
    ajv.addSchema(schema);
    const manifest = ajv.getSchema(`${schema.$id}#/$defs/ProjectManifest`)!;
    const tickets = ajv.getSchema(`${schema.$id}#/$defs/TicketCommands`)!;
    let declarations = 0;
    const invalid: string[] = [];
    for (const project of oracle.projects.values()) {
      const document = project.manifest;
      if (!document) continue;
      const found = (document.metadata?.semio?.dashboard ? 1 : 0) + Object.values<Json>(document.targets ?? {}).filter((target) => target.metadata?.semio?.dashboard).length;
      declarations += found;
      if (found && !manifest(document)) invalid.push(`${project.root}: ${ajv.errorsText(manifest.errors)}`);
    }
    for (const [ticket, document] of oracle.tickets) if (!tickets(document)) invalid.push(`ticket ${ticket}: ${ajv.errorsText(tickets.errors)}`);
    expect(declarations).toBeGreaterThan(50);
    expect(invalid, invalid.slice(0, 10).join("\n")).toEqual([]);
  });

  test("Every declared parameter, ready port, requirement, compound member and group names something that exists", () => {
    const wanted = oracle.ids();
    const dangling: string[] = [];
    const known = (reference: string | Json): boolean => { const run = typeof reference === "string" ? reference : reference.run; return wanted.has(run) || oracle.configurationIds().includes(run); };
    for (const project of oracle.projects.values()) {
      const dashboardBlock = project.manifest?.metadata?.semio?.dashboard ?? {};
      for (const [name, target] of Object.entries<Json>(project.manifest?.targets ?? {})) for (const reference of target.metadata?.semio?.dashboard?.requires ?? []) if (!known(reference)) dangling.push(`${project.name}:${name} requires ${JSON.stringify(reference)}`);
      for (const tool of dashboardBlock.tools ?? []) for (const reference of tool.requires ?? []) if (!known(reference)) dangling.push(`tool:${project.name}/${tool.id} requires ${JSON.stringify(reference)}`);
      for (const compound of dashboardBlock.compounds ?? []) for (const member of compound.members) if (!known(member)) dangling.push(`compound:${project.name}/${compound.id} member ${JSON.stringify(member)}`);
      for (const group of dashboardBlock.groups ?? []) {
        for (const target of group.targets) if (![...oracle.projects.values()].some((candidate) => candidate.targets.has(target))) dangling.push(`group:${project.name}/${group.id} target ${target}`);
        for (const pattern of group.projects as string[]) if (!/[*!]/.test(pattern) && !oracle.projects.has(pattern)) dangling.push(`group:${project.name}/${group.id} project ${pattern}`);
      }
      for (const name of Object.keys(project.manifest?.targets ?? {})) if (project.manifest?.targets[name].metadata?.semio?.dashboard && !project.targets.has(name)) dangling.push(`${project.name}:${name} declares a dashboard block but is no Nx target`);
    }
    expect(dangling, dangling.slice(0, 30).join("\n")).toEqual([]);
  });

  test("Every target configuration resolves through its colon id", async () => {
    const ids = oracle.configurationIds();
    const sample = process.env.SEMIO_COVERAGE_FULL ? ids : ids.filter((_, index) => index % Math.max(1, Math.floor(ids.length / 60)) === 0);
    expect(ids.length).toBeGreaterThan(0);
    const failures: string[] = [];
    await pool(sample, width, async (id) => {
      const { run, launch } = await dryRun({ id, parameters: {}, extraArgs: [] });
      if (!launch) failures.push(`${id}: exit ${run.code} ${run.stderr.trim()}`);
      else if (!launch.processes.at(-1)!.args.includes(id)) failures.push(`${id}: launch does not name the configuration ${JSON.stringify(launch.processes.at(-1)!.args)}`);
    });
    expect(failures, failures.slice(0, 20).join("\n")).toEqual([]);
  }, 900_000);

  test("Every command resolves in a dry run with the launch the oracle states", async () => {
    const wanted = new Map([...oracle.ids()].filter(([id]) => byId.has(id)));
    const declared = (id: string): boolean => { const entry = byId.get(id); return !entry || entry.kind !== "target" || entry.ready !== undefined || entry.parameters.some((parameter) => parameter.origin !== "axis") || entry.longRunning; };
    const special = [...wanted.keys()].filter((id) => declared(id));
    const plain = [...wanted.keys()].filter((id) => !declared(id));
    const sampleSize = process.env.SEMIO_COVERAGE_FULL ? plain.length : Number(process.env.SEMIO_COVERAGE_SAMPLE ?? 400);
    const step = Math.max(1, Math.floor(plain.length / Math.max(1, sampleSize)));
    const ids = [...special, ...plain.filter((_, index) => index % step === 0)];
    const failures: string[] = [];
    const notes = new Map<string, number>();
    let completed = 0;
    const started = Date.now();
    const targets = new Set<string>();
    await pool(ids, width, async (id) => {
      const required = byId.get(id)!.parameters.filter((parameter) => parameter.required && parameter.default === undefined);
      const parameters = Object.fromEntries(required.map((parameter) => [parameter.id, parameter.values[0] ?? (parameter.kind === "flag" ? true : "1")]));
      const selection: Selection = { id, parameters, extraArgs: [] };
      const { run, launch } = await dryRun(selection);
      if (++completed % 100 === 0 || completed === ids.length) console.log(`[DEBUG] coverage ${completed}/${ids.length}, ${Date.now() - started} ms, ${failures.length} differences, latest ${id}`);
      if (!launch) { failures.push(`${id}: dry run exited ${run.code}: ${run.stderr.trim().slice(0, 300)}`); return; }
      const local: string[] = [];
      try { failures.push(...compare(id, oracle.resolve(selection), launch, local)); } catch (error) { failures.push(`${id}: oracle refused: ${(error as Error).message}`); }
      for (const note of local) notes.set(note.split(":")[0]!, (notes.get(note.split(":")[0]!) ?? 0) + 1);
      const walk = (item: Launch): void => { item.requires.forEach(walk); for (const process of item.processes) if (process.args[0] === "nx" && process.args[1] === "run") targets.add(process.args[2]!); };
      walk(launch);
    });
    const unknown = [...targets].filter((target) => { const first = target.indexOf(":"); const project = oracle.project(target.slice(0, first)); return !project || !project.targets.has(target.slice(first + 1).split(":")[0]!); });
    const kinds = new Map<string, number>();
    for (const failure of failures) { const kind = failure.replace(/^[^ ]+ /, "").replace(/\[[^\]]*\]/g, "").replace(/=.*$/s, "").slice(0, 70); kinds.set(kind, (kinds.get(kind) ?? 0) + 1); }
    if (kinds.size) console.log(`[coverage] differences by kind ${JSON.stringify(Object.fromEntries([...kinds].sort((a, b) => b[1] - a[1]).slice(0, 15)))}`);
    console.log(`[coverage] dry-ran ${ids.length} of ${wanted.size} ids (${special.length} declared, ${ids.length - special.length} of ${plain.length} plain); accepted equivalences ${JSON.stringify(Object.fromEntries(notes))}`);
    const problems = [
      ...(unknown.length ? [`${unknown.length} launches name an Nx target that is not in the project graph:\n${unknown.slice(0, 20).join("\n")}`] : []),
      ...(failures.length ? [`${failures.length} of ${ids.length} dry runs differ:\n${failures.slice(0, 60).join("\n")}`] : []),
    ];
    expect(problems, problems.join("\n\n")).toEqual([]);
  }, 7_200_000);

  test("Every ready port is unique among commands that can run together or is documented as shared", () => {
    const ports = new Map<number, string[]>();
    for (const entry of entries) if (entry.ready && entry.kind !== "ticket") ports.set(entry.ready.port, [...(ports.get(entry.ready.port) ?? []), entry.id]);
    const shared = [...ports].filter(([, ids]) => ids.length > 1);
    const allowed = new Map<number, string>(Object.entries(sharing.portSharing).map(([port, reason]) => [Number(port), String(reason)]));
    const undocumented = shared.filter(([port]) => !allowed.has(port)).map(([port, ids]) => `port ${port}: ${ids.join(", ")}`);
    expect(ports.size, "no command declares a ready port").toBeGreaterThan(20);
    expect(undocumented, `${undocumented.length} ready ports are claimed by several commands:\n${undocumented.join("\n")}`).toEqual([]);
  });
});

describe("traceability", () => {
  test("every scenario of the feature has a test of the same title in this file", () => {
    const feature = readFileSync(join(import.meta.dir, "🥒️.feature"), "utf8");
    const text = readFileSync(join(import.meta.dir, "🟦️.ts"), "utf8");
    const titles = [...feature.matchAll(/Scenario:\s*(.+?)\s*$/gm)].map((match) => match[1]!);
    expect(titles.length).toBeGreaterThan(5);
    for (const title of titles) expect(text, `no test titled ${JSON.stringify(title)}`).toContain(`test(${JSON.stringify(title)}`);
    expect(existsSync(join(dashboard, "🧪️tests/🗺️coverage/🔮️oracle/🟦️.ts"))).toBe(true);
  });
});
