import {fileURLToPath} from "node:url";
import {observeCargoPreparationSourceV1,observeCargoPreparationInputV1,observeCargoPreparationOutputV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🟦️.ts";
import { lstatSync, readFileSync, writeFileSync } from "node:fs";
import { isAbsolute, relative, resolve, sep } from "node:path";
import { admitCargoCapabilityLinksV1, prepareCargoCapabilityLinksV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧩️capabilities/🟦️.ts";

type ExportV1 = { package: string; identity: string };

/** 🧬️ Selects exact definition dependencies from admitted conversion-owner exports. */
export function selectSemioConversionDependenciesV1(packages: readonly string[], exports: readonly ExportV1[]): string[] {
  const declared = new Set(packages), seen = new Set<string>(), identities = new Set<string>();
  for (const value of exports) {
    if (!value || Object.keys(value).length !== 2 || typeof value.package !== "string" || typeof value.identity !== "string" || !declared.has(value.package) || seen.has(value.package) || identities.has(value.identity) || !/^[a-z][a-z0-9-]*(?:\.[a-z][a-z0-9-]*)+$/.test(value.identity)) throw new Error("Invalid Semio conversion definition export");
    seen.add(value.package); identities.add(value.identity);
  }
  return [...identities].sort();
}

function physical(root: string, path: string, optional = false): boolean {
  const local = relative(root, path);
  if (isAbsolute(local) || local === ".." || local.startsWith(".." + sep)) throw new Error("Semio conversion authority escapes its repository");
  let current = root;
  for (const part of ["", ...local.split(sep).filter(Boolean)]) {
    current = resolve(current, part);
    let entry;
    try { entry = lstatSync(current); }
    catch (error) { if (optional && (error as NodeJS.ErrnoException).code === "ENOENT") {observeCargoPreparationInputV1(path,"presence");return false;} throw error; }
    if (entry.isSymbolicLink()) throw new Error("Semio conversion authority follows a symlink");
  }
  observeCargoPreparationInputV1(path,"presence");return true;
}

function read(root: string, path: string): string {
  physical(root, path);
  const entry = lstatSync(path);
  if (!entry.isFile() || entry.size > 1024 * 1024) throw new Error("Semio conversion authority is not a bounded regular file");
  const bytes=readFileSync(path);observeCargoPreparationInputV1(path,"file",bytes);return bytes.toString("utf8");
}

/** 📦️ Publishes definition dependencies and Cargo edges from the same lower source inventory. */
export function prepareSemioConversionDefinitionV1(repoRoot: string, ownerRoot: string): number {
  observeCargoPreparationSourceV1(fileURLToPath(import.meta.url));
  const authority = resolve(ownerRoot, "🧩️composition/🔗️conversions/🔣️.json"), definition = resolve(ownerRoot, "📜️artifact-definition.json");
  const source = read(repoRoot, authority), previous = read(repoRoot, definition), links = admitCargoCapabilityLinksV1(JSON.parse(source));
  const exports = new Map<string, ExportV1>(), observed = new Map<string, string>([[authority, source], [definition, previous]]), presence = new Map<string, boolean>();
  for (const target of links.groups.flatMap(group => group.targets)) {
    const owner = resolve(ownerRoot, target.owner), available = physical(repoRoot, owner, true);
    presence.set(owner, available);
    if (!available) continue;
    if (!lstatSync(owner).isDirectory()) throw new Error("Semio conversion export owner is not a directory");
    const manifest = resolve(ownerRoot, target.manifest), manifestSource = read(repoRoot, manifest), native = Bun.TOML.parse(manifestSource) as any;
    const path = resolve(owner, "📜️artifact-definition.json"), bytes = read(repoRoot, path), document = JSON.parse(bytes);
    if (native.package?.name !== target.package || document.definition_version !== 1 || typeof document.id !== "string" || typeof document.artifact !== "string" || document.id !== `s.stdio.${document.artifact}`) throw new Error("Semio conversion definition differs from its lower source owner");
    const exported = { package: target.package, identity: document.id };
    if (exports.has(target.package) && exports.get(target.package)!.identity !== exported.identity) throw new Error("Semio conversion exports disagree");
    exports.set(target.package, exported); observed.set(manifest, manifestSource); observed.set(path, bytes);
  }
  const dependencies = selectSemioConversionDependenciesV1(links.groups.flatMap(group => group.targets.map(target => target.package)), [...exports.values()]);
  const document = JSON.parse(previous);
  if (document.definition_version !== 1 || document.artifact !== "semio" || !Array.isArray(document.dependencies)) throw new Error("Invalid Semio definition authority");
  document.dependencies = dependencies;
  const next = JSON.stringify(document, null, 2) + "\n";
  const count = prepareCargoCapabilityLinksV1(repoRoot, ownerRoot, "📦️packages/🦀️rust/Cargo.toml", "🧩️composition/🔗️conversions/🔣️.json");
  if ([...observed].some(([path, bytes]) => read(repoRoot, path) !== bytes) || [...presence].some(([path, available]) => physical(repoRoot, path, true) !== available)) throw new Error("Semio conversion authority changed during publication");
  if (next !== previous) writeFileSync(definition, next);
  observeCargoPreparationOutputV1(definition,next);
  return count;
}
