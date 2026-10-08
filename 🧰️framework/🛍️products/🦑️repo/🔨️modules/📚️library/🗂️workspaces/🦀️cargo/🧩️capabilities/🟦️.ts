import {fileURLToPath} from "node:url";
import {observeCargoPreparationSourceV1,observeCargoPreparationInputV1,observeCargoPreparationOutputV1} from "../🛠️preparation/🧾️custody/🟦️.ts";
import { lstatSync, readFileSync, writeFileSync } from "node:fs";
import { isAbsolute, relative, resolve, sep } from "node:path";

type TargetV1 = { package: string; owner: string; manifest: string; feature: string };
type GroupV1 = { feature: string; requires: string[]; targets: TargetV1[] };
type LinksV1 = { schema: "semio.repository.cargo-capability-links/v1"; groups: GroupV1[] };
const atom = /^[a-z][a-z0-9-]*$/;

function closed(value: unknown, keys: string[]): Record<string, any> {
  if (!value || typeof value !== "object" || Array.isArray(value) || Object.keys(value).length !== keys.length || keys.some((key) => !Object.hasOwn(value, key))) throw new Error("Invalid closed capability contribution");
  return value as Record<string, any>;
}

/** 🧬️ Admits authored capability links before selecting targets or producing native source. */
export function admitCargoCapabilityLinksV1(value: unknown): LinksV1 {
  const document = closed(value, ["schema", "groups"]);
  if (document.schema !== "semio.repository.cargo-capability-links/v1" || !Array.isArray(document.groups)) throw new Error("Unsupported capability contribution");
  for (const value of document.groups) {
    const group = closed(value, ["feature", "requires", "targets"]);
    if (typeof group.feature !== "string" || !atom.test(group.feature) || !Array.isArray(group.requires) || group.requires.some((value) => typeof value !== "string" || !atom.test(value)) || new Set(group.requires).size !== group.requires.length || !Array.isArray(group.targets)) throw new Error("Invalid capability group");
    for (const value of group.targets) {
      const target = closed(value, ["package", "owner", "manifest", "feature"]);
      if (typeof target.package !== "string" || !atom.test(target.package) || typeof target.feature !== "string" || !atom.test(target.feature) || [target.owner, target.manifest].some(path => typeof path !== "string" || !path.length || path.includes("\\") || /^[A-Za-z]:/.test(path) || isAbsolute(path))) throw new Error("Invalid capability target");
    }
    if (new Set(group.targets.map((target: TargetV1) => target.package)).size !== group.targets.length) throw new Error("Repeated capability target");
  }
  const groups = document.groups as GroupV1[];
  if (new Set(groups.map(group => group.feature)).size !== groups.length || groups.some(group => group.requires.some(feature => !groups.some(owner => owner.feature === feature)))) throw new Error("Invalid capability group authority");
  const features = groups.flatMap(group => [group.feature, ...group.targets.map(target => target.feature)]);
  if (new Set(features).size !== features.length) throw new Error("Repeated capability feature");
  const owners = new Map(groups.map(group => [group.feature, group])), visiting = new Set<string>(), visited = new Set<string>();
  const visit = (feature: string): void => {
    if (visited.has(feature)) return;
    if (visiting.has(feature)) throw new Error("Cyclic capability requirements");
    visiting.add(feature);
    for (const requirement of owners.get(feature)!.requires) visit(requirement);
    visiting.delete(feature); visited.add(feature);
  };
  for (const group of groups) visit(group.feature);
  return document as LinksV1;
}

/** 🗂️ Selects every present target using the admitted owner contribution. */
export function selectCargoCapabilityTargetsV1(value: unknown, present: ReadonlySet<string>): TargetV1[] {
  return admitCargoCapabilityLinksV1(value).groups.flatMap(group => group.targets.filter(target => present.has(target.package)));
}

function within(root: string, path: string): string {
  const local = relative(root, path);
  if (isAbsolute(local) || local === ".." || local.startsWith(".." + sep)) throw new Error("Capability authority escapes its physical owner");
  return local;
}

function physical(root: string, path: string, optional = false): boolean {
  let current = root;
  if (!lstatSync(root).isDirectory() || lstatSync(root).isSymbolicLink()) throw new Error("Invalid capability repository owner");
  for (const part of within(root, path).split(sep).filter(Boolean)) {
    current = resolve(current, part);
    let info;
    try { info = lstatSync(current); }
    catch (error) { if (optional && (error as NodeJS.ErrnoException).code === "ENOENT") {observeCargoPreparationInputV1(path,"presence");return false;} throw error; }
    if (info.isSymbolicLink()) throw new Error("Capability authority follows a symlink");
  }
  observeCargoPreparationInputV1(path,"presence");return true;
}

function read(root: string, path: string): string {
  physical(root, path);
  const info = lstatSync(path);
  if (!info.isFile() || info.size > 1024 * 1024) throw new Error("Capability authority is not a bounded regular file");
  const bytes=readFileSync(path);observeCargoPreparationInputV1(path,"file",bytes);return bytes.toString("utf8");
}

function region(source: string, name: string, value: string): string {
  const begin = `# 🧩️ ${name}\n`, end = `# /🧩️ ${name}`, start = source.indexOf(begin), finish = source.indexOf(end);
  if (start < 0 || finish < start || source.indexOf(begin, start + begin.length) >= 0 || source.indexOf(end, finish + end.length) >= 0) throw new Error(`Invalid capability output region: ${name}`);
  return source.slice(0, start + begin.length) + value + (value ? "\n" : "") + source.slice(finish);
}

/** 📦️ Publishes native capability edges from physically present, owner-authored target manifests. */
export function prepareCargoCapabilityLinksV1(repoRoot: string, ownerRoot: string, manifestRelative: string, linksRelative: string): number {
  observeCargoPreparationSourceV1(fileURLToPath(import.meta.url));
  within(repoRoot, ownerRoot);
  const path = resolve(ownerRoot, manifestRelative), authorityPath = resolve(ownerRoot, linksRelative);
  within(ownerRoot, path); within(ownerRoot, authorityPath);
  const previous = read(repoRoot, path), authoritySource = read(repoRoot, authorityPath), links = admitCargoCapabilityLinksV1(JSON.parse(authoritySource));
  const present = new Set<string>(), observed = new Map<string, string>([[path, previous], [authorityPath, authoritySource]]), presence = new Map<string, boolean>();
  for (const target of links.groups.flatMap(group => group.targets)) {
    const owner = resolve(ownerRoot, target.owner), manifest = resolve(ownerRoot, target.manifest);
    within(repoRoot, owner); within(owner, manifest);
    const available = physical(repoRoot, owner, true);
    presence.set(owner, available);
    if (!available) continue;
    if (!lstatSync(owner).isDirectory()) throw new Error("Capability target owner is not a directory");
    const source = read(repoRoot, manifest), document = Bun.TOML.parse(source) as any;
    if (document.package?.name !== target.package) throw new Error("Capability target differs from its actual native owner");
    observed.set(manifest, source); present.add(target.package);
  }
  const selected = selectCargoCapabilityTargetsV1(links, present), features: string[] = [];
  for (const group of links.groups) {
    const targets = group.targets.filter(target => present.has(target.package));
    features.push(`${group.feature} = ${JSON.stringify([...group.requires, ...targets.map(target => target.feature)])}`);
    for (const target of targets) features.push(`${target.feature} = ${JSON.stringify([`dep:${target.package}`])}`);
  }
  const dependencies = [...new Set(selected.map(target => target.package))].sort().map(name => `${name} = { workspace = true, optional = true }`).join("\n");
  const next = region(region(previous, "Capability Features", features.join("\n")), "Capability Dependencies", dependencies);
  if ([...observed].some(([path, source]) => read(repoRoot, path) !== source) || [...presence].some(([path, available]) => physical(repoRoot, path, true) !== available)) throw new Error("Capability authority changed during publication");
  if (next !== previous) writeFileSync(path, next);
  observeCargoPreparationOutputV1(path,next);
  return selected.length;
}
