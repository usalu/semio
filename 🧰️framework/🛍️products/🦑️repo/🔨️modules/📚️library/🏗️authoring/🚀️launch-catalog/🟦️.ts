import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import environmentSchema from "@semio-tech/process/build/environment/schema.json";
import { compileJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import { crateLaunchRouteProblems, crateLaunchRoutes, crateLaunchSubjects, nextLaunchRouteOrders, type CrateLaunchSubject } from "../📦️crate-tree/🚀️launch-routes/🟦️.ts";
import cratesSchema from "../📦️crate-tree/🚀️launch-routes/🧬️schema/🔣️.json" with { type: "json" };
import type { ArtifactCratePlan } from "../📦️crate-tree/🗺️plan/🧾️model/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const protocol = schema.definitions.Protocol.const;
const environment = environmentSchema.definitions.Protocol.const.environment;
const placeholderPattern = /^@generated:([a-z0-9]+(?:-[a-z0-9]+)*):(react|wgpu|users)$/u;
const machinePath = /\/(?:Users|home)\/[^/\s"]+\/(?:Documents\/)?semio\b/u;
const inputReference = /\$\{input:([^}]+)\}/gu;

/** 🚀️ One entry of an editor launch document, in the key order that is written. */
export type LaunchEntry = Record<string, any>;

/** 🎮️ One playground variant of the component crates: the part of its registry row that decides a dev launcher. */
export type PlaygroundRow = Readonly<{ variant: string; app?: string; ports: Readonly<{ react: number; wgpu: number }>; userPorts?: Readonly<{ react: readonly number[]; wgpu: readonly number[] }> }>;

/** 🛠️ The authored part of the dev launchers of one playground variant. */
export type DevLauncher = Readonly<{ namePrefix: string; order: number; wgpuOrder?: number; env?: Readonly<Record<string, string>>; users?: Readonly<{ namePrefixPattern: string; env: Readonly<Record<string, string>> }> }>;

/** 🧩️ The authored launch seed. */
export type LaunchSeed = Readonly<{ version: string; configurations: readonly (LaunchEntry | string)[]; compounds: readonly LaunchEntry[]; inputs: readonly LaunchEntry[]; devLaunchers: Readonly<Record<string, DevLauncher>>; crateRoutes: Readonly<{ subjects: readonly CrateLaunchSubject[] }> }>;

/** 📄️ The expanded launch document the editor reads. */
export type LaunchCatalog = Readonly<{ version: string; configurations: LaunchEntry[]; compounds: readonly LaunchEntry[]; inputs: readonly LaunchEntry[] }>;

const seedValidator = compileJsonSchemaSubset(schema.definitions.Seed, schema, [cratesSchema]);
const environmentValidator = compileJsonSchemaSubset(environmentSchema.definitions.Environment, environmentSchema);

function requirePort(row: any, renderer: "react" | "wgpu"): number {
  if (!Number.isInteger(row.ports?.[renderer])) throw new Error(`launch catalog: playground variant ${row.variant} declares no ${renderer} port`);
  return row.ports[renderer];
}

/** 🔬️ The playground rows one component manifest declares; a manifest that declares none yields none. */
export function parsePlaygroundRows(manifest: string): PlaygroundRow[] {
  const { registry } = protocol;
  if (!manifest.includes(registry.marker)) return [];
  let node: any = Bun.TOML.parse(manifest);
  for (const key of registry.table.slice(0, -1)) node = node?.[key];
  return (node?.[registry.table.at(-1)!] ?? []).map((row: any): PlaygroundRow => ({ variant: row.variant, ...(row.app ? { app: row.app } : {}), ports: { react: requirePort(row, "react"), wgpu: requirePort(row, "wgpu") }, ...(row.user_ports ? { userPorts: { react: row.user_ports.react, wgpu: row.user_ports.wgpu } } : {}) }));
}

/** 🗂️ Reads the playground rows of the tracked component manifests, so a variant needs no second declaration of its ports. */
export function readPlaygroundRegistry(repoRoot: string): PlaygroundRow[] {
  const { registry } = protocol;
  const listed = execFileSync("git", ["ls-files", "-z", "--", `*${registry.manifest}`], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 28 }).split("\0");
  const rows = new Map<string, PlaygroundRow>();
  for (const file of listed) {
    if (!file || file.startsWith(registry.excludedPrefix) || !file.endsWith(registry.manifest) || !existsSync(join(repoRoot, file))) continue;
    for (const row of parsePlaygroundRows(readFileSync(join(repoRoot, file), "utf8"))) {
      if (rows.has(row.variant)) throw new Error(`launch catalog: playground variant ${row.variant} is declared twice (${file})`);
      rows.set(row.variant, row);
    }
  }
  return [...rows.values()].sort((left, right) => left.variant.localeCompare(right.variant));
}

function devEntry(row: PlaygroundRow, rendererId: string, name: string, port: number, order: number, extra: Readonly<Record<string, string>>): LaunchEntry {
  const { dev } = protocol;
  return {
    name,
    type: dev.type,
    request: dev.request,
    command: dev.command.replace("{variant}", row.variant).replace("{renderer}", rendererId),
    cwd: dev.cwd,
    env: { [dev.environment.port]: String(port), [dev.environment.plugin]: row.variant, [dev.environment.renderer]: rendererId, ...(row.app ? { [dev.environment.app]: row.app } : {}), ...extra, ...environment },
    presentation: { group: dev.group, order },
    serverReadyAction: { action: dev.readyAction, pattern: dev.readyPattern.replace("{port}", String(port)), uriFormat: dev.readyUriFormat },
  };
}

function userEntries(row: PlaygroundRow, launcher: DevLauncher): LaunchEntry[] {
  const { dev } = protocol;
  const { users } = launcher;
  if (!users || !row.userPorts) throw new Error(`launch catalog: ${row.variant} has no user template or no user ports`);
  const slot = (text: string, n: number) => text.replaceAll("{N}", String(n));
  return dev.renderers.filter((renderer) => launcher[renderer.orderKey as "order"] !== undefined).flatMap((renderer) =>
    row.userPorts![renderer.portKey as "react"].map((port, index) => devEntry(row, renderer.id, `${dev.namePrefix}${slot(users.namePrefixPattern, index + 1)}${renderer.icon}`, port, Math.round((launcher[renderer.orderKey as "order"]! + (index + 1) * dev.userOrderStep) * 100) / 100, Object.fromEntries(Object.entries(users.env).map(([key, value]) => [key, slot(value, index + 1)])))),
  );
}

function generatedDevEntries(placeholder: string, rows: ReadonlyMap<string, PlaygroundRow>, launchers: Readonly<Record<string, DevLauncher>>): LaunchEntry[] {
  const match = placeholderPattern.exec(placeholder);
  if (!match) throw new Error(`launch catalog: ${JSON.stringify(placeholder)} is no placeholder`);
  const [, variant, kind] = match as unknown as [string, string, string];
  const row = rows.get(variant);
  const launcher = launchers[variant];
  if (!row || !launcher) throw new Error(`launch catalog: ${placeholder} names a variant that ${row ? "has no devLaunchers row" : "the playground registry does not declare"}`);
  if (kind === "users") return userEntries(row, launcher);
  const renderer = protocol.dev.renderers.find((candidate) => candidate.id === kind)!;
  const order = launcher[renderer.orderKey as "order"];
  if (order === undefined) throw new Error(`launch catalog: ${placeholder} needs ${renderer.orderKey} in devLaunchers`);
  return [devEntry(row, renderer.id, `${protocol.dev.namePrefix}${launcher.namePrefix}${renderer.icon}`, row.ports[renderer.portKey as "react"], order, launcher.env ?? {})];
}

/** 🧬️ Expands a seed into the launch document: each placeholder becomes its generated routes in place, and the crate routes continue the orders of their groups above every other route. */
export function expandLaunchCatalog(seed: LaunchSeed, registry: readonly PlaygroundRow[]): LaunchCatalog {
  const rows = new Map(registry.map((row) => [row.variant, row]));
  const anchors = seed.configurations.filter((item) => item === protocol.crateRoutesPlaceholder).length;
  if (anchors > 1 || (anchors === 0 && seed.crateRoutes.subjects.length)) throw new Error(`launch catalog: ${protocol.crateRoutesPlaceholder} must appear exactly once while crate subjects exist`);
  const slots = seed.configurations.map((item) => (item === protocol.crateRoutesPlaceholder ? undefined : typeof item === "string" ? generatedDevEntries(item, rows, seed.devLaunchers) : [item]));
  const crate = crateLaunchRoutes(seed.crateRoutes.subjects, nextLaunchRouteOrders(slots.flatMap((slot) => slot ?? [])));
  return { version: seed.version, configurations: slots.flatMap((slot) => slot ?? (crate as unknown as LaunchEntry[])), compounds: seed.compounds, inputs: seed.inputs };
}

/** 🔎️ Every way a seed, the registry and the expansion depart from the catalog contract; empty when the catalog is sound. */
export function launchCatalogProblems(seed: LaunchSeed, registry: readonly PlaygroundRow[], workspaceRoot?: string): string[] {
  const problems = seedValidator.validate(seed).map((problem) => `seed: ${problem}`);
  if (problems.length) return problems;
  const rows = new Map(registry.map((row) => [row.variant, row]));
  const placeholders = seed.configurations.filter((item): item is string => typeof item === "string");
  const seen = new Set<string>();
  for (const placeholder of placeholders) {
    if (seen.has(placeholder)) problems.push(`seed: placeholder ${placeholder} appears twice`);
    seen.add(placeholder);
    const match = placeholderPattern.exec(placeholder);
    if (!match) continue;
    const [, variant, kind] = match as unknown as [string, string, string];
    const launcher = seed.devLaunchers[variant];
    if (!rows.has(variant)) problems.push(`seed: ${placeholder} names a variant the playground registry does not declare`);
    if (!launcher) problems.push(`seed: ${placeholder} has no devLaunchers row`);
    else if (kind === "wgpu" && launcher.wgpuOrder === undefined) problems.push(`seed: ${placeholder} needs wgpuOrder in devLaunchers`);
    else if (kind === "users" && (!launcher.users || !rows.get(variant)?.userPorts)) problems.push(`seed: ${placeholder} needs a users template and user ports in the registry`);
  }
  for (const row of registry) if (!seed.devLaunchers[row.variant]) problems.push(`seed: playground variant ${row.variant} has no devLaunchers row`);
  for (const [variant, launcher] of Object.entries(seed.devLaunchers)) {
    const required = ["react", ...(launcher.wgpuOrder === undefined ? [] : ["wgpu"]), ...(launcher.users ? ["users"] : [])];
    for (const kind of required) if (!seen.has(`@generated:${variant}:${kind}`)) problems.push(`seed: devLaunchers.${variant} has no @generated:${variant}:${kind} placeholder`);
    if (!rows.has(variant)) problems.push(`seed: devLaunchers.${variant} is no playground variant of the registry`);
  }
  const subjectNames = seed.crateRoutes.subjects.map((subject) => subject.name);
  for (const name of subjectNames.filter((name, index) => subjectNames.indexOf(name) !== index)) problems.push(`seed: crate subject ${name} appears twice`);
  if (problems.length) return problems;
  let catalog: LaunchCatalog;
  try {
    catalog = expandLaunchCatalog(seed, registry);
  } catch (error) {
    return [(error as Error).message];
  }
  const names = new Map<string, number>();
  for (const entry of catalog.configurations) names.set(entry.name, (names.get(entry.name) ?? 0) + 1);
  for (const [name, count] of names) if (count > 1) problems.push(`catalog: configuration name ${name} appears ${count} times`);
  for (const compound of catalog.compounds) for (const member of compound.configurations) if (!names.has(member)) problems.push(`catalog: compound ${compound.name} names the missing configuration ${member}`);
  const inputs = new Set(catalog.inputs.map((input) => input.id));
  for (const entry of catalog.configurations) {
    const text = JSON.stringify(entry);
    for (const match of text.matchAll(inputReference)) if (!inputs.has(match[1])) problems.push(`catalog: ${entry.name} uses the undeclared input ${match[1]}`);
    if (machinePath.test(text) || (workspaceRoot && text.includes(workspaceRoot))) problems.push(`catalog: ${entry.name} holds a machine specific absolute path`);
    for (const problem of environmentValidator.validate(entry.env ?? {})) problems.push(`catalog: ${entry.name}: env ${problem}`);
  }
  problems.push(...crateLaunchRouteProblems(catalog.configurations));
  return problems;
}

/** 🖨️ The text of a launch document: two-space JSON with a final newline, the one form both files are written in. */
export function renderLaunchDocument(document: object): string {
  return `${JSON.stringify(document, null, 2)}\n`;
}

function writeWhole(file: string, text: string): void {
  writeFileSync(`${file}.tmp`, text);
  renameSync(`${file}.tmp`, file);
}

/** 📖️ Reads the authored seed of a checkout. */
export function readLaunchSeed(repoRoot: string): LaunchSeed {
  return Bun.JSONC.parse(readFileSync(join(repoRoot, protocol.seed), "utf8")) as LaunchSeed;
}

/** ✍️ Writes the authored seed of a checkout in its one form. */
export function writeLaunchSeed(repoRoot: string, seed: LaunchSeed): void {
  writeWhole(join(repoRoot, protocol.seed), renderLaunchDocument(seed));
}

/** 💾️ Expands the seed of a checkout and writes `.vscode/launch.json` when it differs; a catalog that has problems is not written. */
export function writeLaunchCatalog(repoRoot: string, registry: readonly PlaygroundRow[] = readPlaygroundRegistry(repoRoot)): { changed: boolean; problems: string[] } {
  const seed = readLaunchSeed(repoRoot);
  const problems = launchCatalogProblems(seed, registry, repoRoot);
  if (problems.length) return { changed: false, problems };
  const text = renderLaunchDocument(expandLaunchCatalog(seed, registry));
  const file = join(repoRoot, protocol.catalog);
  if (existsSync(file) && readFileSync(file, "utf8") === text) return { changed: false, problems };
  writeWhole(file, text);
  return { changed: true, problems };
}

/** 🔁️ Every drift of the checked-in catalog from the expansion of the seed, plus every problem of the seed itself. */
export function checkLaunchCatalog(repoRoot: string, registry: readonly PlaygroundRow[] = readPlaygroundRegistry(repoRoot)): string[] {
  const seed = readLaunchSeed(repoRoot);
  const problems = launchCatalogProblems(seed, registry, repoRoot);
  if (problems.length) return problems;
  const file = join(repoRoot, protocol.catalog);
  return existsSync(file) && readFileSync(file, "utf8") === renderLaunchDocument(expandLaunchCatalog(seed, registry)) ? [] : [`${protocol.catalog} is not the expansion of ${protocol.seed}; run generate launch`];
}

/** ➕️ Adds the subjects whose crate has none yet to a seed; repeating the call adds nothing. */
export function withCrateLaunchSubjects(seed: LaunchSeed, subjects: readonly CrateLaunchSubject[]): { seed: LaunchSeed; added: CrateLaunchSubject[] } {
  const known = new Set(seed.crateRoutes.subjects.map((subject) => subject.name));
  const added = subjects.filter((subject, index) => !known.has(subject.name) && subjects.findIndex((other) => other.name === subject.name) === index);
  crateLaunchRoutes(added, nextLaunchRouteOrders([]));
  return { seed: added.length ? { ...seed, crateRoutes: { subjects: [...seed.crateRoutes.subjects, ...added] } } : seed, added };
}

/** 📦️ Registers the carved crates of a plan as subjects of the seed and rewrites the catalog; the added crate names are listed beside the backup so a restore removes exactly those subjects. */
export function registerCrateLaunchRoutes(repoRoot: string, plan: ArtifactCratePlan, backupDir?: string, registry: readonly PlaygroundRow[] = readPlaygroundRegistry(repoRoot)): string[] {
  const { seed, added } = withCrateLaunchSubjects(readLaunchSeed(repoRoot), crateLaunchSubjects(plan));
  if (!added.length) return [];
  if (backupDir) {
    mkdirSync(dirname(backupDir), { recursive: true });
    if (!existsSync(`${backupDir}-launch-subjects.json`)) writeFileSync(`${backupDir}-launch-subjects.json`, `${JSON.stringify(added.map((subject) => subject.name), null, 2)}\n`);
  }
  writeLaunchSeed(repoRoot, seed);
  const { problems } = writeLaunchCatalog(repoRoot, registry);
  if (problems.length) throw new Error(`launch catalog: ${problems.slice(0, 5).join("; ")}`);
  return crateLaunchRoutes(added, nextLaunchRouteOrders([])).map((route) => route.name);
}
