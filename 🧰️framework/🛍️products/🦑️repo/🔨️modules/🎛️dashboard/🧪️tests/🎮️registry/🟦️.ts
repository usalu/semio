/**
 * 🎮️ Independent oracle of the dashboard command registry. A TypeScript resolver written from fleet-plan §2.2
 * (not from the Rust code) derives, for the frozen fixture workspace, the commands the registry must list and the
 * launch each selection must resolve to; Ajv validates declarations and outputs against the registry schema; Bun's
 * own XXH3 checks the hash the graph-reuse decision relies on. The Rust registry is exercised through the `semio`
 * binary only: `commands --json`, `commands --check` and `run --dry-run`.
 *
 * @see ../../🧫️fixtures/🎮️registry/🏗️workspace.json
 * @see ../../🧬️schema/🎮️registry/🔣️.json
 */
import { afterAll, beforeAll, expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { installedDashboard } from "../../📦️installation/🟦️.ts";

type Json = Record<string, any>;
const here = import.meta.dir;
const dashboard = resolve(here, "../..");
const repo = resolve(dashboard, "../../../../..");
const schema = JSON.parse(readFileSync(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"), "utf8")) as Json;
const fixture = JSON.parse(readFileSync(join(dashboard, "🧫️fixtures/🎮️registry/🏗️workspace.json"), "utf8")) as { files: Json[] };
const ajv = new (Ajv2020 as any)({ allErrors: true, strict: false });
ajv.addSchema(schema);
const validator = (name: string) => ajv.getSchema(`${schema.$id}#/$defs/${name}`) as ((value: unknown) => boolean) & { errors?: Json[] };

// #region 🔖️Binary
function binary(): string {
  return process.env.SEMIO_DASHBOARD_BIN ?? installedDashboard(repo);
}

let root = "";
const files = new Map<string, string>();
beforeAll(() => {
  root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "semio-registry-oracle-"));
  for (const file of fixture.files) {
    const text = file.json !== undefined ? JSON.stringify(file.json, null, 2) : (file.text as string);
    files.set(file.path, text);
    const target = join(root, ...(file.path as string).split("/"));
    mkdirSync(dirname(target), { recursive: true });
    writeFileSync(target, text);
  }
});
afterAll(() => rmSync(root, { recursive: true, force: true }));

function semio(args: string[], env: Record<string, string> = {}): { code: number; out: string; err: string } {
  const run = Bun.spawnSync([binary(), ...args], { cwd: root, env: { ...process.env, ...env, SEMIO_DASHBOARD_GRAPH: "fresh" }, stdout: "pipe", stderr: "pipe" });
  return { code: run.exitCode, out: run.stdout.toString(), err: run.stderr.toString() };
}
const json = (path: string): Json => JSON.parse(files.get(path)!) as Json;
// #endregion 🔖️Binary

// #region 🔖️Declarations
const manifestPaths = () => [...files.keys()].filter((path) => path.endsWith("project.json"));
const ticketDocuments = () => [...files.keys()].filter((path) => path.endsWith("🎮️commands.json"));

test("Ajv accepts every declaration of the fixture workspace and refuses the forms the registry no longer has", () => {
  const manifest = validator("ProjectManifest");
  for (const path of manifestPaths()) expect(manifest(json(path)), `${path}: ${JSON.stringify(manifest.errors)}`).toBe(true);
  const tickets = validator("TicketCommands");
  for (const path of ticketDocuments()) expect(tickets(json(path)), `${path}: ${JSON.stringify(tickets.errors)}`).toBe(true);
  const project = (dashboardBlock: Json) => ({ metadata: { semio: { dashboard: dashboardBlock } } });
  const refused: Json[] = [
    project({ groups: [{ id: "g", target: "build", projects: ["p"] }] }),
    project({ groups: [{ id: "g", targets: [], projects: ["p"] }] }),
    project({ tools: [{ id: "t", command: ["x"], ready: { port: 1, printed: true, path: "/x" } }] }),
    project({ tools: [{ id: "t", command: ["x"], env: { NX_DAEMON: "true" } }] }),
    project({ tools: [{ id: "t", command: ["x"], requires: [{ parameters: {} }] }] }),
    project({ compounds: [{ id: "c", members: [{ run: "compound:w/c" }] }] }),
    project({ compounds: [{ id: "c", members: [{ run: "a:b", surprise: 1 }] }] }),
  ];
  for (const value of refused) expect(manifest(value), JSON.stringify(value)).toBe(false);
  const accepted: Json[] = [
    project({ groups: [{ id: "g", targets: ["build", "test"], projects: ["*", "!p"] }] }),
    project({ tools: [{ id: "t", command: ["x"], ready: { port: 1, printed: true }, requires: ["a:b", { run: "a:b", parameters: { x: true }, env: { A: "{workspace}" } }] }] }),
  ];
  for (const value of accepted) expect(manifest(value), JSON.stringify(manifest.errors)).toBe(true);
});

test("the registry check reports no problem for the fixture workspace and names the file of a broken declaration", () => {
  const clean = semio(["commands", "--check", "--root", root]);
  expect({ code: clean.code, out: clean.out.trim() }).toEqual({ code: 0, out: expect.stringMatching(/ commands, 0 problems$/) });
  const broken = join(root, "🧰️tools", "broken", "📋️project.json");
  mkdirSync(dirname(broken), { recursive: true });
  writeFileSync(broken, JSON.stringify({ name: "broken", metadata: { semio: { dashboard: { groups: [{ id: "g", target: "build", projects: ["broken"] }] } } } }));
  const report = semio(["commands", "--check", "--root", root]);
  rmSync(dirname(broken), { recursive: true, force: true });
  expect(report.code).toBe(1);
  expect(report.err).toContain("🧰️tools/broken/📋️project.json");
  expect(report.err).toContain("unknown field `target`");
});
// #endregion 🔖️Declarations

// #region 🔖️Listing
const FIXED_REPO = ["goals.list", "goals.tree", "tree.monorepo", "tree.goal", "tree.statute", "tree.territory", "statutes.catalog", ...["all", "rust", "typescript", "go", "python", "markdown"].map((scope) => `analyze:${scope}`)];

/** 🧭️ The command ids the fixture workspace must offer, derived without the registry. */
function expectedIds(): string[] {
  const ids = new Set<string>();
  const graph = json(".nx/workspace-data/project-graph.json").nodes as Json;
  const projects = new Map<string, { root: string; targets: Map<string, Json> }>();
  for (const [name, node] of Object.entries(graph)) if (existsSync(join(root, ...(node.data.root as string).split("/")))) projects.set(name, { root: node.data.root, targets: new Map(Object.entries(node.data.targets ?? {})) });
  for (const path of manifestPaths()) {
    const manifest = json(path);
    const project = projects.get(manifest.name) ?? { root: dirname(path) === "." ? "." : dirname(path), targets: new Map() };
    for (const target of Object.keys(manifest.targets ?? {})) project.targets.set(target, manifest.targets[target]);
    projects.set(manifest.name, project);
    const block = manifest.metadata?.semio?.dashboard ?? {};
    for (const tool of block.tools ?? []) ids.add(`tool:${manifest.name}/${tool.id}`);
    for (const compound of block.compounds ?? []) ids.add(`compound:${manifest.name}/${compound.id}`);
    for (const group of block.groups ?? []) ids.add(`group:${manifest.name}/${group.id}`);
  }
  for (const [name, project] of projects) for (const target of project.targets.keys()) ids.add(`${name}:${target}`);
  for (const row of json("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🚀️playgrounds.json") as unknown as Json[]) ids.add(`playground:${row.variant}`);
  for (const path of files.keys()) {
    const ticket = /\.🧬semio\/🦑️repo\/🎫️tickets\/🎆️(\d\d)\/🌙️(\d\d)\/☀️(\d\d)\/([^/]+)\/🎫️ticket\.json$/u.exec(path);
    if (!ticket) continue;
    const id = `${ticket[1]}/${ticket[2]}/${ticket[3]}/${ticket[4]}`;
    const open = json(path).status === "open";
    for (const action of ["show", "files", open ? "close" : "reopen"]) ids.add(`repo:ticket.${action}:${id}`);
    const commands = path.replace("🎫️ticket.json", "🎮️commands.json");
    if (open && files.has(commands)) for (const entry of [...(json(commands).tools ?? []), ...(json(commands).compounds ?? [])]) ids.add(`ticket:${id}/${entry.id}`);
  }
  for (const action of FIXED_REPO) ids.add(`repo:${action}`);
  return [...ids].sort();
}

test("`semio commands --json --all` lists exactly the commands an independent walk of the fixture derives, each a valid registry entry", () => {
  const listed = semio(["commands", "--json", "--all", "--root", root]);
  expect(listed.code, listed.err).toBe(0);
  const entries = JSON.parse(listed.out) as Json[];
  const entry = validator("RegistryEntry");
  for (const row of entries) expect(entry(row), `${row.id}: ${JSON.stringify(entry.errors)}`).toBe(true);
  expect(entries.map((row) => row.id).sort()).toEqual(expectedIds());
  expect(entries.some((row) => String(row.id).startsWith("script:")), "root scripts are no command source").toBe(false);
  const byId = new Map(entries.map((row) => [row.id as string, row]));
  expect(byId.get("hub:dev")?.ready).toEqual({ port: 8787, path: "/admin" });
  expect(byId.get("hub:dev")?.longRunning).toBe(true);
  expect(byId.get("workspace:build")?.longRunning).toBe(false);
  expect(byId.get("tool:workspace/printer")?.ready).toEqual({ port: 6280, path: "", printed: true });
  expect(byId.get("playground:shell")?.parameters.map((parameter: Json) => parameter.id)).toEqual(expect.arrayContaining(["renderer", "example", "user-slot", "hub", "data", "local-only"]));
});

test("a search narrows by every word and the default listing leaves closed tickets out", () => {
  const search = (...words: string[]) => (JSON.parse(semio(["commands", "--json", ...words, "--root", root]).out) as Json[]).map((row) => row.id as string);
  expect(search("dev", "shell")).toContain("playground:shell");
  expect(search("closed-probe")).toEqual([]);
  expect(search("closed-probe", "--all")).toContain("repo:ticket.reopen:26/08/01/CLOSED-PROBE");
  expect(search("--all", "nonsense-word-xyz")).toEqual([]);
});
// #endregion 🔖️Listing

// #region 🔖️Resolver
const RUNNER_ENV: Record<string, string> = { NX_NATIVE_COMMAND_RUNNER: "false", NX_TUI: "false" };
const VERBS = ["setup", "start", "dev", "serve", "watch", "activate", "prepare", "run", "build", "package", "test", "smoke", "check", "typecheck", "verify", "gate", "lint", "format", "generate", "publish", "deploy", "preview", "bench", "clean"];
const verbOf = (name: string, declared?: string): string => declared ?? (VERBS.includes(name.split(/[^a-z0-9]/u)[0]!) ? name.split(/[^a-z0-9]/u)[0]! : "task");
const rootBlock = (): Json => json("📋️project.json").metadata.semio.dashboard;
const manifestOf = (project: string): Json => json(manifestPaths().find((path) => json(path).name === project)!);
const decl = (project: string, target: string): Json => manifestOf(project).targets?.[target]?.metadata?.semio?.dashboard ?? {};
const projectBlock = (project: string): Json => manifestOf(project).metadata?.semio?.dashboard ?? {};
const tickets = (id: string): Json => json(`.🧬semio/🦑️repo/🎫️tickets/🎆️${id.slice(0, 2)}/🌙️${id.slice(3, 5)}/☀️${id.slice(6, 8)}/${id.slice(9)}/🎮️commands.json`);

type Pins = Record<string, string | boolean | number>;
type Member = { run: string; parameters?: Pins; env?: Record<string, string> };
type Resolved = { commandId: string; cmd: string; args: string[]; cwd: string; env: Record<string, string>; ready?: Json; requires: Resolved[][]; verb: string };

function axesFor(verb: string, owned: Json[], nx: boolean): Json[] {
  const effectBeyondNx = (axis: Json): boolean => Boolean(axis.env || axis.args || axis.valueEnv || axis.valueFlag || axis.valuePositional || (axis.values ?? []).some((value: Json) => value.env || value.args));
  return (rootBlock().parameters as Json[]).filter((axis) => !owned.some((own) => own.id === axis.id) && !axis.appliesTo?.playground && (nx ? !axis.appliesTo?.verbs || axis.appliesTo.verbs.includes(verb) : axis.appliesTo?.verbs?.includes(verb) && effectBeyondNx(axis)));
}

/** 🎛️ Applies the parameters in order (fleet-plan §2.2 "effect semantics") and returns what they contribute. */
function apply(parameters: Json[], chosen: Record<string, string | boolean | number>, values: Record<string, string>, base: Record<string, string>) {
  const out = { env: {} as Record<string, string>, nxFlags: [] as string[], args: [] as string[] };
  for (const parameter of parameters) {
    const typed = chosen[parameter.id];
    let value: string | undefined = typed === undefined ? undefined : String(typed);
    if (value === undefined && parameter.default !== undefined) value = parameter.kind === "text" ? String(parameter.default).replace(/\{([a-z0-9._-]+)\}/g, (token, name) => (name === "workspace" ? root : values[name] ?? token)) : String(parameter.default);
    if (value === undefined) {
      if (parameter.required) throw new Error(`parameter ${parameter.id} is required`);
      continue;
    }
    values[parameter.id] = value;
    if (parameter.kind === "choice") {
      const entry = (parameter.values as Json[]).find((candidate) => candidate.id === value);
      if (!entry) throw new Error(`no value ${value}`);
      Object.assign(out.env, entry.env ?? {});
      out.nxFlags.push(...(entry.nxFlags ?? []));
      out.args.push(...(entry.args ?? []));
    } else if (parameter.kind === "flag") {
      if (value !== "true") continue;
      Object.assign(out.env, parameter.env ?? {});
      out.nxFlags.push(...(parameter.nxFlags ?? []));
      out.args.push(...(parameter.args ?? []));
    } else if (parameter.valueEnv) out.env[parameter.valueEnv] = value;
    else if (parameter.valueFlag) out.args.push(...(parameter.valueFlag.endsWith("=") ? [`${parameter.valueFlag}${value}`] : [parameter.valueFlag, value]));
    else if (parameter.valuePositional) out.args.push(value);
  }
  void base;
  return out;
}

/** 🌱️ Extra environment that takes part in the final environment before the ready port is read, and the part that needs the port. */
const earlyEnv = (extra: Record<string, string>, values: Record<string, string>): Record<string, string> => Object.fromEntries(Object.entries(extra).filter(([, text]) => !text.includes("{port}")).map(([key, text]) => [key, expand(text, values)]));
const lateEnv = (extra: Record<string, string>, values: Record<string, string>, port?: number): Record<string, string> => Object.fromEntries(Object.entries(extra).filter(([, text]) => text.includes("{port}")).map(([key, text]) => [key, expand(text, values, port)]));
const expand = (text: string, values: Record<string, string>, port?: number): string => text.replace(/\{([a-z0-9._-]+)\}/g, (token, name) => (name === "workspace" ? root : name === "port" && port !== undefined ? String(port) : values[name] ?? token));

function resolveReady(declared: Json | undefined, env: Record<string, string>, values: Record<string, string>, parameters: Json[]): { ready?: Json; port?: number } {
  if (!declared) return {};
  let port: number | undefined = declared.port;
  const named = declared.portEnv as string | undefined;
  if (named) {
    const variable = parameters.find((parameter) => parameter.id === named)?.valueEnv as string | undefined;
    const final = env[named] ?? (variable ? env[variable] : undefined);
    const stated = final !== undefined ? expand(final, values) : values[named];
    if (stated !== undefined) port = Number(stated);
    else if (declared.port !== undefined && /^[A-Za-z_][A-Za-z0-9_]*$/u.test(named)) env[named] = String(declared.port);
  }
  return { ready: { port, path: declared.path ?? "", ...(declared.printed ? { printed: true } : {}) }, port };
}

function resolveRequires(requires: (string | Member)[] | undefined): Resolved[][] {
  return (requires ?? []).map((entry) => {
    const member: Member = typeof entry === "string" ? { run: entry } : entry;
    const launch = resolveLaunch(member.run, member.parameters ?? {}, [], {});
    for (const process of launch) for (const [key, value] of Object.entries(member.env ?? {})) process.env[key] = expand(value, {});
    return launch;
  });
}

function resolveLaunch(id: string, chosen: Pins, extra: string[], extraEnv: Record<string, string>): Resolved[] {
  const colon = id.indexOf(":");
  const kind = id.slice(0, colon);
  const rest = id.slice(colon + 1);
  if (kind === "playground") return [resolvePlayground(id, rest, chosen, extra, extraEnv)];
  if (kind === "compound" || (kind === "ticket" && ticketCompound(rest))) {
    const [owner, name] = splitOwner(kind, rest);
    const compound = (kind === "compound" ? projectBlock(owner) : tickets(owner)).compounds.find((entry: Json) => entry.id === name) as Json;
    return (compound.members as Member[]).flatMap((member) => {
      const own = resolveLaunch(member.run, { ...(member.parameters ?? {}), ...Object.fromEntries(Object.entries(chosen).filter(([key]) => !(key in (member.parameters ?? {})))) }, [], extraEnv);
      for (const process of own) for (const [key, value] of Object.entries(member.env ?? {})) process.env[key] = expand(value, {});
      return own;
    });
  }
  const env: Record<string, string> = {};
  let values: Record<string, string> = {};
  if (kind === "tool" || kind === "ticket") {
    const [owner, name] = splitOwner(kind, rest);
    const tool = (kind === "tool" ? projectBlock(owner) : tickets(owner)).tools.find((entry: Json) => entry.id === name) as Json;
    const verb = verbOf(tool.id, tool.verb);
    const parameters = [...axesFor(verb, tool.parameters ?? [], false), ...(tool.parameters ?? [])];
    const effects = apply(parameters, chosen, values, env);
    Object.assign(env, tool.env ?? {}, effects.env, earlyEnv(extraEnv, values));
    const ready = resolveReady(tool.ready, env, values, parameters);
    const command = (tool.command as string[]).map((word) => expand(word, values, ready.port));
    for (const key of Object.keys(env)) env[key] = expand(env[key]!, values, ready.port);
    Object.assign(env, lateEnv(extraEnv, values, ready.port));
    return [{ commandId: id, cmd: command[0]!, args: [...command.slice(1), ...effects.args.map((word) => expand(word, values, ready.port)), ...extra], cwd: tool.cwd ?? "", env: { ...RUNNER_ENV, ...env }, ready: ready.ready, requires: resolveRequires(tool.requires), verb }];
  }
  if (kind === "group") {
    const [owner, name] = splitOwner(kind, rest);
    const group = projectBlock(owner).groups.find((entry: Json) => entry.id === name) as Json;
    const verb = verbOf(group.targets[0]);
    const effects = apply(axesFor(verb, [], true), chosen, values, env);
    const flags = [...new Set(effects.nxFlags)];
    return [{ commandId: id, cmd: "bun", args: ["nx", "run-many", "-t", group.targets.join(","), "-p", group.projects.join(","), ...flags, ...(effects.args.length + extra.length ? ["--", ...effects.args, ...extra] : [])], cwd: "", env: { ...RUNNER_ENV, ...effects.env, ...extraEnv }, requires: [], verb }];
  }
  const configuration = id.split(":").length === 3 ? id.split(":")[2] : undefined;
  const project = configuration ? id.slice(0, id.lastIndexOf(":", id.lastIndexOf(":") - 1)) : id.slice(0, id.lastIndexOf(":"));
  const target = configuration ? id.split(":")[1]! : id.slice(id.lastIndexOf(":") + 1);
  const declared = decl(project, target);
  const verb = verbOf(target, declared.verb);
  const parameters = [...axesFor(verb, declared.parameters ?? [], true), ...(declared.parameters ?? [])];
  const effects = apply(parameters, chosen, values, env);
  Object.assign(env, effects.env, earlyEnv(extraEnv, values));
  const ready = resolveReady(declared.ready, env, values, parameters);
  for (const key of Object.keys(env)) env[key] = expand(env[key]!, values, ready.port);
  const args = [...effects.args.map((word) => expand(word, values, ready.port)), ...extra];
  const flags = [...new Set(effects.nxFlags)];
  return [{ commandId: id, cmd: "bun", args: ["nx", "run", id, ...flags, ...(args.length ? ["--", ...args] : [])], cwd: "", env: { ...RUNNER_ENV, ...env, ...lateEnv(extraEnv, values, ready.port) }, ready: ready.ready, requires: resolveRequires(declared.requires), verb }];
}

function ticketCompound(rest: string): boolean {
  const [owner, name] = splitOwner("ticket", rest);
  return Boolean(tickets(owner).compounds?.some((entry: Json) => entry.id === name));
}
function splitOwner(kind: string, rest: string): [string, string] {
  const at = rest.lastIndexOf("/");
  return [rest.slice(0, at), rest.slice(at + 1)];
}

/** 🛝️ The playground facts the registry derives from the catalog row (fleet-plan §2.2.1 amendment 8). */
function resolvePlayground(id: string, variant: string, chosen: Pins, extra: string[], extraEnv: Record<string, string>): Resolved {
  const row = (json("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🚀️playgrounds.json") as unknown as Json[]).find((entry) => entry.variant === variant)!;
  const renderer = String(chosen.renderer ?? "react");
  const slot = chosen["user-slot"] === undefined ? undefined : Number(chosen["user-slot"]);
  const key = renderer === "react" ? "react" : "wgpu";
  const port = slot === undefined ? row.ports[key] : row.userPorts[key][slot - 1];
  const env: Record<string, string> = { SEMIO_PLUGIN: variant, SEMIO_RENDERER: key, S_OS_PORT: String(extraEnv.S_OS_PORT ?? port), ...(row.app ? { SEMIO_APP_ID: row.app, VITE_SEMIO_APP_ID: row.app } : {}) };
  const target = renderer === "react" ? `dev-${variant}-react-dev` : renderer === "wgpu-native" ? `run-${variant}-native-dev` : `dev-${variant}-wgpu-dev`;
  const args = ["nx", "run", `@semio-tech/framework-os-dev:${target}`];
  if (chosen.example !== undefined) {
    const slug = (row.examples as string[]).find((candidate) => candidate === chosen.example || candidate.split("️").pop() === chosen.example)!;
    env.PLAYGROUND_LOCKED_EXAMPLE_ID = slug;
    if (renderer === "wgpu-native") args.push("--", "--example", slug);
  }
  if (chosen["app-role"] !== undefined) env.SEMIO_APP_ROLE = String(chosen["app-role"]);
  const flag = (name: string, fallback: boolean): boolean => (chosen[name] === undefined ? fallback : String(chosen[name]) === "true");
  let ready: Json | undefined;
  if (renderer !== "wgpu-native") {
    if (row.hub && flag("hub", false)) env.S_HUB_URL = row.hub;
    if (row.localOnly && flag("local-only", false)) env.S_LOCAL_ONLY = "1";
    const directory = slot === undefined ? row.dataDir : row.userDataDir?.replace("{N}", String(slot));
    if ((row.dataDir || row.userDataDir) && flag("data", true) && directory) env.S_DATA_DIR = `${root}/${directory}`;
    ready = { port: Number(env.S_OS_PORT), path: chosen["app-role"] === "viewer" && row.viewerPath ? row.viewerPath : "" };
  }
  return { commandId: id, cmd: "bun", args: [...args, ...extra], cwd: "", env: { ...RUNNER_ENV, ...env, ...extraEnv }, ready, requires: [], verb: "dev" };
}

/** 🧾️ The comparable form of a launch: process list with workspace-relative cwd, environment as a sorted map. */
const PLAYGROUND_ENV = ["SEMIO_APP_ID", "VITE_SEMIO_APP_ID", "S_OS_PORT", "SEMIO_PLUGIN", "SEMIO_RENDERER", "S_DATA_DIR", "S_HUB_URL", "S_LOCAL_ONLY", "PLAYGROUND_LOCKED_EXAMPLE_ID", "SEMIO_APP_ROLE", ...Object.keys(RUNNER_ENV)];
function comparable(process: Json, playground: boolean): Json {
  const env = Object.fromEntries((process.env as [string, string][]).filter(([key]) => !playground || PLAYGROUND_ENV.includes(key)).sort(([a], [b]) => a.localeCompare(b)));
  return { commandId: process.commandId, cmd: process.cmd, args: process.args, cwd: process.cwd, env, ...(process.ready ? { ready: process.ready } : {}) };
}
const expected = (resolved: Resolved): Json => comparable({ ...resolved, env: Object.entries(resolved.env) }, resolved.commandId.startsWith("playground:"));

/** 🔗️ A launch with its nested requirements; a requirement that repeats or is itself a member is listed once or not at all. */
function shapeOfResolved(own: Resolved[]): Json {
  const members = new Set(own.map((process) => process.commandId));
  const seen = new Set<string>();
  const requires: Json[] = [];
  for (const process of own) for (const need of process.requires) {
    const shape = shapeOfResolved(need);
    const key = JSON.stringify(shape);
    if (seen.has(key) || need.some((candidate) => members.has(candidate.commandId))) continue;
    seen.add(key);
    requires.push(shape);
  }
  return { processes: own.map(expected), requires };
}
const shapeOfLaunch = (launch: Json): Json => ({ processes: launch.processes.map((process: Json) => comparable(process, process.commandId.startsWith("playground:"))), requires: launch.requires.map(shapeOfLaunch) });

type Case = { id: string; parameters?: Pins; args?: string[]; env?: Record<string, string> };
const CASES: Case[] = [
  { id: "workspace:test", parameters: { cache: "skip-local", "test-level": "quick", dependencies: true, "cargo-jobs": "2" }, args: ["--filter", "x"], env: { EXTRA: "1" } },
  { id: "workspace:build", parameters: { "build-mode": "ship" } },
  { id: "workspace:dev" },
  { id: "workspace:verify", parameters: { rule: "literal-external" } },
  { id: "hub:dev" },
  { id: "hub:dev", parameters: { backend: "postgres" } },
  { id: "hub:dev", env: { OS_HUB_PORT: "9001" } },
  { id: "hub:dev", parameters: { backend: "postgres" }, env: { OS_HUB_PORT: "9002", NOTE: "{workspace}/n-{backend}-{port}" } },
  { id: "@fixture/quiz:dev", env: { QUIZ_PORT: "6099" } },
  { id: "hub:build:production" },
  { id: "hub:erase", parameters: { handle: "abc", "dry-run": true } },
  { id: "@fixture/quiz:dev", parameters: { listen: "6070" } },
  { id: "@fixture/quiz:test-e2e", parameters: { "test-level": "long" } },
  { id: "tool:workspace/inspector" },
  { id: "tool:workspace/bun-test", parameters: { file: "a.test.ts", bail: true, "build-mode": "ship", "test-level": "quick" } },
  { id: "tool:workspace/printer" },
  { id: "tool:workspace/viewer" },
  { id: "group:workspace/artifact-tests", parameters: { "test-level": "long", cache: "skip-all" } },
  { id: "group:workspace/all-builds" },
  { id: "compound:workspace/dev-shell-with-hub" },
  { id: "compound:workspace/users" },
  { id: "compound:workspace/quiz-with-tools" },
  { id: "compound:workspace/release-checks" },
  { id: "ticket:26/09/23/OPEN-PROBE/probe", parameters: { mode: "deep" } },
  { id: "ticket:26/09/23/OPEN-PROBE/probe-with-hub" },
  { id: "playground:shell" },
  { id: "playground:shell", parameters: { "user-slot": "2", hub: true, "local-only": true, renderer: "wgpu-wasm" } },
  { id: "playground:shell", parameters: { data: false, example: "tower" } },
  { id: "playground:shell", parameters: { example: "demo", renderer: "wgpu-native" } },
  { id: "playground:cad", parameters: { "app-role": "viewer" } },
  { id: "playground:cad", env: { S_OS_PORT: "6555" } },
];

test("`semio run --dry-run` resolves every selection to the launch an independent resolver derives, and the launch validates", () => {
  const launchSchema = validator("Launch");
  for (const entry of CASES) {
    const parameters = Object.entries(entry.parameters ?? {}).flatMap(([key, value]) => ["--param", `${key}=${value}`]);
    const environment = Object.entries(entry.env ?? {}).flatMap(([key, value]) => ["--env", `${key}=${value}`]);
    const run = semio(["run", entry.id, ...parameters, ...environment, "--dry-run", "--root", root, ...(entry.args?.length ? ["--", ...entry.args] : [])]);
    expect(run.code, `${entry.id}: ${run.err}`).toBe(0);
    const actual = JSON.parse(run.out) as Json;
    expect(launchSchema(actual), `${entry.id}: ${JSON.stringify(launchSchema.errors)}`).toBe(true);
    const own = resolveLaunch(entry.id, entry.parameters ?? {}, entry.args ?? [], entry.env ?? {});
    expect(own.length).toBeGreaterThan(0);
    expect(shapeOfLaunch(actual), entry.id + " " + JSON.stringify(entry.parameters ?? {})).toEqual(shapeOfResolved(own));
    expect(actual.stop).toBe(entry.id === "compound:workspace/dev-shell-with-hub" ? "together" : "independent");
    expect(actual.group ?? null).toBe(entry.id.startsWith("compound:") || entry.id.startsWith("ticket:26/09/23/OPEN-PROBE/probe-with-hub") ? entry.id : null);
  }
}, 60_000);

test("a selection the declarations do not allow is refused with the parameter named and nothing is started", () => {
  const cases: [string[], RegExp][] = [
    [["workspace:verify"], /"rule" is required/],
    [["workspace:verify", "--param", "rule=nope"], /choose dependencies, literal-external/],
    [["workspace:build", "--param", "ghost=1"], /unknown parameter "ghost"/],
    [["workspace:build", "--param", "cache"], /a choice parameter needs a value, write `--param cache=<value>`/],
    [["workspace:build", "--env", "NX_DAEMON=true"], /NX_DAEMON belongs to the dashboard/],
    [["workspace:build", "--env", "NOEQUALS"], /not `key=value`/],
    [["compound:workspace/dev-shell-with-hub", "--", "extra"], /takes no extra arguments/],
    [["no-such:command"], /unknown command/],
    [["playground:shell", "--param", "user-slot=9"], /choose 1, 2/],
  ];
  for (const [args, message] of cases) {
    const run = semio(["run", "--dry-run", "--root", root, ...args]);
    expect({ args, code: run.code }).toEqual({ args, code: 2 });
    expect(run.err).toMatch(message);
    expect(run.out).toBe("");
  }
});

test("semio has only the dashboard verbs: nothing is forwarded to the root script and --help prints the usage", () => {
  for (const verb of ["frobnicate", "verify", "test", "build", "commit", "dev", "setup"]) {
    const unknown = semio([verb]);
    expect({ verb, code: unknown.code }).toEqual({ verb, code: 2 });
    expect(unknown.err).toContain(`unknown verb "${verb}"`);
    expect(unknown.err).toContain("semio run <id>");
    expect(unknown.out).toBe("");
  }
  for (const flag of ["--help", "-h", "help"]) {
    const help = semio([flag]);
    expect({ flag, code: help.code }).toEqual({ flag, code: 0 });
    expect(help.out).toContain("Semio has no other verbs");
    for (const verb of ["commands", "run", "tasks", "logs", "stop|restart|kill", "open", "daemon", "dashboard"]) expect(help.out).toContain(`semio ${verb}`);
  }
  for (const verb of ["run", "tasks", "logs", "stop", "restart", "kill", "open"]) {
    const refused = semio([verb, "--root", root]);
    expect(refused.err, verb).not.toContain("unknown verb");
  }
});
// #endregion 🔖️Resolver

// #region 🔖️Hash
test("Bun's own XXH3 gives the vectors the registry pins for the Nx graph-reuse decision", () => {
  const rust = readFileSync(join(dashboard, "🎮️registry/🧪️tests/🔬️unit/🦀️.rs"), "utf8");
  for (const input of ["", "a", "abc"]) {
    const hash = (Bun.hash as any).xxHash3(input) as bigint;
    const literal = `0x${hash.toString(16).padStart(16, "0").match(/.{4}/gu)!.join("_")}`;
    expect(rust, `vector of ${JSON.stringify(input)}`).toContain(literal);
  }
});
// #endregion 🔖️Hash
