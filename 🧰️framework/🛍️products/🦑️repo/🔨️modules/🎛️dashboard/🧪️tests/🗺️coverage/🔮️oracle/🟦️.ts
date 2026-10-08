/**
 * 🔮️ Independent TypeScript resolver of the registry contract (fleet-plan §2.2): reads the Nx project graph, the
 * project manifests and the playground catalog on its own and states, for every command id, what `semio run <id>
 * --dry-run` must resolve to. It shares no code with the Rust registry; Ajv validates the declarations it reads.
 *
 * @see ../../../🧬️schema/🎮️registry/🔣️.json
 * @see ../../../🧫️fixtures/🗺️coverage/🔣️.json
 */
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

export type Json = Record<string, any>;
export type Selection = { id: string; parameters: Record<string, string | boolean>; extraArgs: string[]; env?: Record<string, string> };
export type Ready = { port: number; path: string; printed?: boolean };
export type Resolved = { cmd: string; args: string[]; env: Record<string, string>; cwd: string; ready?: Ready; requires: Selection[]; members?: { selection: Selection; resolved: Resolved }[]; stop?: string; longRunning: boolean };

export const VERBS = ["setup", "start", "dev", "serve", "watch", "activate", "prepare", "run", "build", "package", "test", "smoke", "check", "typecheck", "verify", "gate", "lint", "format", "generate", "publish", "deploy", "preview", "bench", "clean"];
const SKIPPED = new Set(["node_modules", "target", "dist", "build", "generated", "cache", "fixtures"]);
const MANIFESTS = ["📋️project.json", "project.json"];
const PLAYGROUNDS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🚀️playgrounds.json";
const unique = <T>(words: T[]): T[] => [...new Set(words)];
const substitute = (word: string, values: Record<string, string>): string => word.replace(/\{([a-z0-9._-]+)\}/g, (token, id) => (id in values ? values[id]! : token));
const keyOf = (name: string): string => name.replace(/^[^\p{L}\p{N}]+/u, "").toLowerCase();

type Project = { name: string; root: string; targets: Map<string, { continuous: boolean; configurations: string[] }>; manifest: Json | null };

/** 🧮️ One workspace as the oracle reads it. */
export class Oracle {
  readonly projects = new Map<string, Project>();
  readonly playgrounds: Json[];
  readonly axes: Json[];
  readonly problems: string[] = [];
  readonly tickets = new Map<string, Json>();

  constructor(readonly root: string) {
    const graphPath = join(root, ".nx/workspace-data/project-graph.json");
    const graph = existsSync(graphPath) ? JSON.parse(readFileSync(graphPath, "utf8")) : { nodes: {} };
    const skipped = (folder: string): boolean => folder.split("/").some((part) => part !== "." && (part.startsWith(".") || SKIPPED.has(keyOf(part))));
    for (const [name, node] of Object.entries<Json>(graph.nodes)) {
      const data = node.data;
      if (skipped(data.root)) continue;
      const targets = new Map<string, { continuous: boolean; configurations: string[] }>(Object.entries<Json>(data.targets ?? {}).map(([target, value]) => [target, { continuous: value.continuous === true, configurations: Object.keys(value.configurations ?? {}) }]));
      this.projects.set(name, { name, root: data.root, targets, manifest: this.manifestOf(data.root) });
    }
    for (const path of this.walk(root)) {
      const manifest = JSON.parse(readFileSync(join(root, path), "utf8")) as Json;
      if (!manifest.name || this.projects.has(manifest.name)) continue;
      this.projects.set(manifest.name, { name: manifest.name, root: path.slice(0, path.lastIndexOf("/")) || ".", targets: new Map(Object.entries<Json>(manifest.targets ?? {}).map(([target, value]) => [target, { continuous: value.continuous === true, configurations: Object.keys(value.configurations ?? {}) }])), manifest });
    }
    for (const project of this.projects.values()) if (project.manifest) for (const target of Object.keys(project.manifest.targets ?? {})) if (!project.targets.has(target)) project.targets.set(target, { continuous: project.manifest.targets[target].continuous === true, configurations: Object.keys(project.manifest.targets[target].configurations ?? {}) });
    this.playgrounds = existsSync(join(root, PLAYGROUNDS)) ? JSON.parse(readFileSync(join(root, PLAYGROUNDS), "utf8")) : [];
    this.readTickets();
    this.axes = (this.project("workspace")?.manifest?.metadata?.semio?.dashboard?.parameters ?? []) as Json[];
  }

  private readTickets(): void {
    const base = join(this.root, ".🧬semio/🦑️repo/🎫️tickets");
    const folders = (directory: string): string[] => (existsSync(directory) ? readdirSync(directory, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => entry.name) : []);
    const plain = (name: string): string => name.replace(/^[^\p{L}\p{N}]+/u, "");
    for (const year of folders(base)) for (const month of folders(join(base, year))) for (const day of folders(join(base, year, month))) for (const slug of folders(join(base, year, month, day))) {
      const folder = join(base, year, month, day, slug);
      const document = join(folder, "🎮️commands.json");
      const ticket = join(folder, "🎫️ticket.json");
      if (!existsSync(document) || !existsSync(ticket) || JSON.parse(readFileSync(ticket, "utf8")).status !== "open") continue;
      this.tickets.set(`${plain(year)}/${plain(month)}/${plain(day)}/${slug}`, JSON.parse(readFileSync(document, "utf8")));
    }
  }

  private manifestOf(folder: string): Json | null {
    for (const name of MANIFESTS) {
      const path = join(this.root, folder, name);
      if (existsSync(path)) return JSON.parse(readFileSync(path, "utf8"));
    }
    return null;
  }

  private walk(directory: string, prefix = ""): string[] {
    const found: string[] = [];
    for (const entry of readdirSync(join(this.root, prefix), { withFileTypes: true })) {
      if (entry.isDirectory()) {
        if (!entry.name.startsWith(".") && !SKIPPED.has(keyOf(entry.name))) found.push(...this.walk(directory, prefix ? `${prefix}/${entry.name}` : entry.name));
      } else if (entry.isFile() && MANIFESTS.includes(entry.name)) found.push(prefix ? `${prefix}/${entry.name}` : entry.name);
    }
    return found;
  }

  project(name: string): Project | undefined { return this.projects.get(name); }
  private dashboard(project: string): Json { return this.project(project)?.manifest?.metadata?.semio?.dashboard ?? {}; }
  private declaration(project: string, target: string): Json { return this.project(project)?.manifest?.targets?.[target]?.metadata?.semio?.dashboard ?? {}; }

  /** 🔢️ Every id the registry must list, with the kind the oracle derives it from. */
  ids(): Map<string, string> {
    const ids = new Map<string, string>();
    for (const project of this.projects.values()) {
      for (const target of project.targets.keys()) ids.set(`${project.name}:${target}`, "target");
      const dashboard = this.dashboard(project.name);
      for (const tool of dashboard.tools ?? []) ids.set(`tool:${project.name}/${tool.id}`, "tool");
      for (const compound of dashboard.compounds ?? []) ids.set(`compound:${project.name}/${compound.id}`, "compound");
      for (const group of dashboard.groups ?? []) ids.set(`group:${project.name}/${group.id}`, "group");
    }
    for (const playground of this.playgrounds) ids.set(`playground:${playground.variant}`, "playground");
    for (const [ticket, document] of this.tickets) for (const entry of [...(document.tools ?? []), ...(document.compounds ?? [])]) ids.set(`ticket:${ticket}/${entry.id}`, "ticket");
    return ids;
  }

  /** 🧩️ Every `project:target:configuration` id: not a registry entry of its own, but a command `run` resolves. */
  configurationIds(): string[] {
    return [...this.projects.values()].flatMap((project) => [...project.targets].flatMap(([target, facts]) => facts.configurations.map((configuration) => `${project.name}:${target}:${configuration}`)));
  }

  /** 🎫️ A ticket command is a tool or a compound of its ticket's document. */
  private canonical(id: string): string {
    if (!id.startsWith("ticket:")) return id;
    const reference = id.slice("ticket:".length);
    const slash = reference.lastIndexOf("/");
    const document = this.tickets.get(reference.slice(0, slash));
    return `${(document?.compounds as Json[] | undefined)?.some((entry) => entry.id === reference.slice(slash + 1)) ? "compound" : "tool"}:${reference}`;
  }

  verbOf(target: string, declared?: string): string {
    if (declared) return declared;
    const word = target.split("-")[0]!;
    return VERBS.includes(word) ? word : "task";
  }

  private applicableAxes(verb: string, owned: Json[]): Json[] {
    return this.axes.filter((axis) => !owned.some((parameter) => parameter.id === axis.id) && (!axis.appliesTo?.verbs || axis.appliesTo.verbs.includes(verb)));
  }

  /** 🎚️ The parameters a command offers: its own, then the global axes that apply to its verb. */
  parameters(id: string): Json[] {
    id = this.canonical(id);
    const colon = id.indexOf(":");
    const kind = id.slice(0, colon);
    const rest = id.slice(colon + 1);
    if (kind === "tool") return this.tool(rest).parameters ?? [];
    if (kind === "group") return this.applicableAxes(this.verbOf(this.group(rest).targets[0]), []);
    if (kind === "playground" || kind === "compound") return [];
    const { project, target } = this.nx(id);
    const declared = this.declaration(project, target);
    return [...(declared.parameters ?? []), ...this.applicableAxes(this.verbOf(target, declared.verb), declared.parameters ?? [])];
  }

  /** 🪪 Splits `project:target[:configuration]`; the target keeps its configuration. */
  nx(id: string): { project: string; target: string; configuration?: string } {
    const first = id.indexOf(":");
    const project = id.slice(0, first);
    const [target, configuration] = id.slice(first + 1).split(":");
    return { project, target: target!, configuration };
  }

  tool(reference: string): Json { return this.owned("tools", reference); }
  group(reference: string): Json { return this.owned("groups", reference); }
  compound(reference: string): Json { return this.owned("compounds", reference); }

  private owned(kind: string, reference: string): Json {
    const slash = reference.lastIndexOf("/");
    const holder = this.tickets.get(reference.slice(0, slash)) ?? this.dashboard(reference.slice(0, slash));
    const found = (holder[kind] as Json[] | undefined)?.find((candidate) => candidate.id === reference.slice(slash + 1));
    if (!found) throw new Error(`${kind} ${reference} is not declared`);
    return found;
  }

  private effect(parameter: Json, chosen: string | boolean | undefined, out: Effects): void {
    const value = chosen ?? parameter.default;
    if (value === undefined || value === false) return;
    if (parameter.kind === "choice") {
      const entry = (parameter.values as Json[]).find((candidate) => candidate.id === value);
      if (!entry) throw new Error(`parameter ${parameter.id}: ${String(value)} is not one of ${(parameter.values as Json[]).map((candidate) => candidate.id).join("|")}`);
      Object.assign(out.env, entry.env ?? {});
      out.nxFlags.push(...(entry.nxFlags ?? []));
      out.args.push(...(entry.args ?? []));
    } else if (parameter.kind === "flag") {
      Object.assign(out.env, parameter.env ?? {});
      out.nxFlags.push(...(parameter.nxFlags ?? []));
      out.args.push(...(parameter.args ?? []));
    } else {
      const text = String(value);
      if (parameter.valueEnv) out.env[parameter.valueEnv] = text;
      else if (parameter.valueFlag) out.args.push(...(parameter.valueFlag.endsWith("=") ? [`${parameter.valueFlag}${text}`] : [parameter.valueFlag, text]));
      else if (parameter.valuePositional) out.args.push(text);
    }
    out.values[parameter.id] = String(value);
  }

  private ready(declared: Json | undefined, env: Record<string, string>): Ready | undefined {
    if (!declared) return undefined;
    const port = declared.port ?? Number(env[declared.portEnv]);
    if (declared.port !== undefined && declared.portEnv && env[declared.portEnv] === undefined) env[declared.portEnv] = String(declared.port);
    return { port, path: declared.path ?? "", ...(declared.printed ? { printed: true } : {}) };
  }

  private reference(member: string | Json): Selection {
    const entry = typeof member === "string" ? { run: member } : member;
    return { id: entry.run, parameters: Object.fromEntries(Object.entries<any>(entry.parameters ?? {}).map(([key, value]) => [key, typeof value === "boolean" ? value : String(value)])), extraArgs: [], env: entry.env };
  }

  /** 🧭️ What `semio run <selection> --dry-run` must resolve to, before the runner policy adds its own environment. */
  resolve(selection: Selection): Resolved {
    const resolved = this.resolveRaw(selection);
    const expand = (text: string): string => text.replaceAll("{workspace}", this.root);
    return { ...resolved, cmd: expand(resolved.cmd), args: resolved.args.map(expand), cwd: expand(resolved.cwd), env: Object.fromEntries(Object.entries(resolved.env).map(([key, value]) => [key, expand(value)])) };
  }

  private resolveRaw(selection: Selection): Resolved {
    selection = { ...selection, id: this.canonical(selection.id) };
    const colon = selection.id.indexOf(":");
    const kind = selection.id.slice(0, colon);
    const rest = selection.id.slice(colon + 1);
    if (kind === "playground") return this.playground(rest, selection);
    if (kind === "compound") {
      const declared = this.compound(rest);
      return { cmd: "", args: [], env: {}, cwd: "", requires: [], stop: declared.stop ?? "independent", longRunning: true, members: (declared.members as (string | Json)[]).map((member) => { const reference = this.reference(member); return { selection: reference, resolved: this.resolve(reference) }; }) };
    }
    const out: Effects = { env: {}, nxFlags: [], args: [], values: {} };
    const offered = this.parameters(selection.id);
    for (const key of Object.keys(selection.parameters)) if (!offered.some((parameter) => parameter.id === key)) throw new Error(`${selection.id}: parameter ${key} is not offered (offered: ${offered.map((parameter) => parameter.id).join(", ")})`);
    for (const parameter of offered) {
      if (parameter.required && selection.parameters[parameter.id] === undefined && parameter.default === undefined) throw new Error(`${selection.id}: required parameter ${parameter.id} has no value`);
      this.effect(parameter, selection.parameters[parameter.id], out);
    }
    const nxFlags = unique(out.nxFlags);
    const tail = [...out.args, ...selection.extraArgs];
    const extraEnv = selection.env ?? {};
    if (kind === "tool") {
      const declared = this.tool(rest);
      const fill = (text: string): string => substitute(text, out.values);
      const env = Object.fromEntries(Object.entries<string>({ ...(declared.env ?? {}), ...out.env, ...extraEnv }).map(([key, value]) => [key, fill(value)]));
      const command = (declared.command as string[]).map(fill);
      const ready = this.ready(declared.ready, env);
      const port = (text: string): string => (ready ? text.replaceAll("{port}", String(ready.port)) : text);
      return { cmd: port(command[0]!), args: [...command.slice(1), ...tail.map(fill)].map(port), env: Object.fromEntries(Object.entries(env).map(([key, value]) => [key, port(value)])), cwd: fill(declared.cwd ?? ""), ready, requires: (declared.requires ?? []).map((member: string | Json) => this.reference(member)), longRunning: declared.continuous === true || ready !== undefined };
    }
    if (kind === "group") {
      const declared = this.group(rest);
      const targets = declared.targets as string[];
      return { cmd: "bun", args: ["nx", "run-many", "-t", targets.join(","), `--projects=${(declared.projects as string[]).join(",")}`, ...nxFlags, ...(tail.length ? ["--", ...tail] : [])], env: { ...out.env, ...extraEnv }, cwd: "", requires: [], longRunning: false };
    }
    const { project, target } = this.nx(selection.id);
    const declared = this.declaration(project, target);
    const env = { ...out.env, ...extraEnv };
    const ready = this.ready(declared.ready, env);
    const continuous = this.project(project)?.targets.get(target)?.continuous === true;
    return { cmd: "bun", args: ["nx", "run", selection.id, ...nxFlags, ...(tail.length ? ["--", ...tail] : [])], env, cwd: "", ready, requires: (declared.requires ?? []).map((member: string | Json) => this.reference(member)), longRunning: continuous || ready !== undefined };
  }

  private playground(variant: string, selection: Selection): Resolved {
    const entry = this.playgrounds.find((candidate) => candidate.variant === variant);
    if (!entry) throw new Error(`playground ${variant} is not in the catalog`);
    const renderer = String(selection.parameters.renderer ?? "react");
    if (renderer === "wgpu-native") return { cmd: "bun", args: ["nx", "run", "@semio-tech/framework-renderer-wgpu:native", "--", variant, ...selection.extraArgs], env: { ...(selection.env ?? {}) }, cwd: "", requires: [], longRunning: true };
    const key = renderer === "react" ? "react" : "wgpu";
    const slot = selection.parameters["user-slot"] === undefined ? undefined : Number(selection.parameters["user-slot"]);
    const port = slot === undefined ? entry.ports[key] : entry.userPorts?.[key]?.[slot - 1];
    if (port === undefined) throw new Error(`playground ${variant}: no port for renderer ${renderer}${slot === undefined ? "" : ` user slot ${slot}`}`);
    const env: Record<string, string> = { S_OS_PORT: String(port), SEMIO_PLUGIN: variant, SEMIO_RENDERER: key, ...(entry.app ? { SEMIO_APP_ID: entry.app } : {}) };
    if (selection.parameters.example !== undefined) env.SEMIO_DEFAULT_EXAMPLE = String(selection.parameters.example);
    if (selection.parameters["app-role"] !== undefined) env.SEMIO_APP_ROLE = String(selection.parameters["app-role"]);
    return { cmd: "bun", args: ["nx", "run", "workspace:dev", "--", variant, ...selection.extraArgs], env: { ...env, ...(selection.env ?? {}) }, cwd: "", ready: { port, path: "" }, requires: [], longRunning: true };
  }
}

type Effects = { env: Record<string, string>; nxFlags: string[]; args: string[]; values: Record<string, string> };

/** 🧾️ Command-line flags that state the same selection to `semio run`. */
export function runArguments(selection: Selection, flags: string[] = []): string[] {
  return [selection.id, ...flags, ...Object.entries(selection.parameters).flatMap(([key, value]) => ["--param", `${key}=${String(value)}`]), ...Object.entries(selection.env ?? {}).flatMap(([key, value]) => ["--env", `${key}=${value}`]), ...(selection.extraArgs.length ? ["--", ...selection.extraArgs] : [])];
}

/** ⚖️ The shape of `semio run --dry-run` output. */
export type Proc = { commandId: string; cmd: string; args: string[]; cwd: string; env: [string, string][]; ready?: { port: number; path: string; printed?: boolean }; longRunning: boolean };
export type Launch = { commandId: string; processes: Proc[]; requires: Launch[]; stop: string; group?: string };

export const policyEnv = new Set(["NX_NATIVE_COMMAND_RUNNER", "NX_TUI", "NX_FORCE_REUSE_CACHED_GRAPH", "NX_TASKS_RUNNER_DYNAMIC_OUTPUT", "VITE_SEMIO_PLUGIN", "VITE_SEMIO_RENDERER"]);

const asObject = (pairs: [string, string][]): Record<string, string> => Object.fromEntries(pairs);
/** 🧽️ Path separators and the two spellings of the Nx project filter do not distinguish launches. */
const normal = (words: string[]): string[] => words.flatMap((word) => (word.startsWith("--projects=") ? ["-p", word.slice("--projects=".length)] : [word])).map((word) => word.replaceAll("\\", "/"));


/** ⚖️ Faults between what the oracle resolved and what the Rust registry launches; `notes` collects accepted equivalences. */
export function compare(id: string, expected: Resolved, launch: Launch, notes: string[]): string[] {
  const faults: string[] = [];
  const actual = launch.processes;
  if (expected.members) {
    if (actual.length !== expected.members.length) return [`${id}: ${actual.length} processes, expected ${expected.members.length} members`];
    if (launch.stop !== expected.stop) faults.push(`${id}: stop ${launch.stop}, expected ${expected.stop}`);
    expected.members.forEach((member, index) => faults.push(...compareProcess(`${id}#${index} ${member.selection.id}`, member.resolved, actual[index]!, notes)));
    return faults;
  }
  if (actual.length < 1) return [`${id}: no process`];
  faults.push(...compareProcess(id, expected, actual[actual.length - 1]!, notes));
  const required = launch.requires.map((required) => required.commandId);
  const wanted = expected.requires.map((selection) => selection.id);
  if (JSON.stringify(required) !== JSON.stringify(wanted)) faults.push(`${id}: requires ${JSON.stringify(required)}, expected ${JSON.stringify(wanted)}`);
  return faults;
}

function compareProcess(label: string, expected: Resolved, actual: Proc, notes: string[]): string[] {
  const faults: string[] = [];
  const playground = label.startsWith("playground:") || actual.commandId.startsWith("playground:");
  if (expected.cmd !== actual.cmd) faults.push(`${label}: cmd ${actual.cmd}, expected ${expected.cmd}`);
  const sameArgs = JSON.stringify(normal(expected.args)) === JSON.stringify(normal(actual.args));
  if (!sameArgs) {
    const entry = /^nx run (@semio-tech\/framework-os-dev:(?:dev|serve)-[^ ]+-(?:react|wgpu)-dev)$/.exec(`nx run ${actual.args.slice(2).join(" ")}`);
    if (playground && actual.args[0] === "nx" && actual.args[1] === "run" && entry && actual.args.length === 3) notes.push("playground-entry-target");
    else if (playground && actual.args[0] === "nx" && actual.args[1] === "run" && actual.args[2] === "@semio-tech/framework-renderer-wgpu:native") notes.push("playground-native-entry");
    else faults.push(`${label}: args ${JSON.stringify(actual.args)}, expected ${JSON.stringify(expected.args)}`);
  }
  if ((expected.cwd ?? "") !== (actual.cwd ?? "")) faults.push(`${label}: cwd ${JSON.stringify(actual.cwd)}, expected ${JSON.stringify(expected.cwd)}`);
  const env = asObject(actual.env);
  for (const [key, value] of Object.entries(expected.env)) if (normal([env[key] ?? ""])[0] !== normal([value])[0]) faults.push(`${label}: env ${key}=${JSON.stringify(env[key])}, expected ${JSON.stringify(value)}`);
  const extra = Object.keys(env).filter((key) => !(key in expected.env) && !policyEnv.has(key));
  if (extra.length) notes.push(`extra-env:${extra.sort().join(",")}`);
  const ready = actual.ready;
  if (expected.ready) {
    if (!ready) faults.push(`${label}: no ready, expected port ${expected.ready.port}`);
    else {
      if (ready.port !== expected.ready.port) faults.push(`${label}: ready port ${ready.port}, expected ${expected.ready.port}`);
      if ((ready.path ?? "") !== (expected.ready.path ?? "")) faults.push(`${label}: ready path ${JSON.stringify(ready.path)}, expected ${JSON.stringify(expected.ready.path)}`);
    }
  } else if (ready) notes.push(`extra-ready:${ready.port}`);
  return faults;
}

