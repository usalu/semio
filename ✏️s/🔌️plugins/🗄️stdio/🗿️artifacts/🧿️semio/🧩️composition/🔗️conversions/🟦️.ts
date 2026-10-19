import {fileURLToPath} from "node:url";
import { observeCargoPreparationSourceV1, observeCargoPreparationInputV1, observeCargoPreparationOutputV1 } from "@semio-tech/framework/process/native-build";
import { lstatSync, readFileSync, writeFileSync } from "node:fs";
import { isAbsolute, relative, resolve, sep } from "node:path";

type ExportV1 = { package: string; identity: string };
export type SemioConversionV1 = { subset: string; direction: "import" | "export"; package: string; manifest: string; targets: string[] };
export type SemioConversionsV1 = { schema: "semio.stdio.semio.conversions/v1"; catalog: { package: string; manifest: string; source: string }; owners: { package: string; owner: string; symbolPackages: string[] }[]; conversions: SemioConversionV1[] };

const atom = /^[a-z][a-z0-9-]*$/;
const local = /^[^/\\][^\\]*$/;

function closed(value: unknown, fields: readonly string[], what: string): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value) || Object.keys(value).length !== fields.length || fields.some((field) => !Object.hasOwn(value, field))) throw new Error(`Invalid Semio conversion ${what}`);
  return value as Record<string, unknown>;
}

function text(value: unknown, pattern: RegExp, what: string): string {
  if (typeof value !== "string" || !pattern.test(value)) throw new Error(`Invalid Semio conversion ${what}`);
  return value;
}

/** 🔗️ Admits the conversion authority: every conversion crate once per subset and direction, every target an owned format. */
export function admitSemioConversionsV1(value: unknown): SemioConversionsV1 {
  const doc = closed(value, ["schema", "catalog", "owners", "conversions"], "authority");
  if (doc.schema !== "semio.stdio.semio.conversions/v1") throw new Error("Unsupported Semio conversion authority schema");
  const catalog = closed(doc.catalog, ["package", "manifest", "source"], "catalog");
  if (!Array.isArray(doc.owners) || !Array.isArray(doc.conversions)) throw new Error("Invalid Semio conversion authority lists");
  const owners = doc.owners.map((row) => { const owner = closed(row, ["package", "owner", "symbolPackages"], "owner"); if (!Array.isArray(owner.symbolPackages)) throw new Error("Invalid Semio conversion owner symbol packages"); return { package: text(owner.package, atom, "owner package"), owner: text(owner.owner, local, "owner path"), symbolPackages: owner.symbolPackages.map((name) => text(name, atom, "owner symbol package")) }; });
  const crates = owners.flatMap((row) => [row.package, ...row.symbolPackages]);
  const owned = new Set(crates);
  if (owned.size !== crates.length) throw new Error("Semio conversion owners repeat a package");
  const conversions = doc.conversions.map((row): SemioConversionV1 => {
    const conversion = closed(row, ["subset", "direction", "package", "manifest", "targets"], "conversion");
    if (conversion.direction !== "import" && conversion.direction !== "export") throw new Error("Invalid Semio conversion direction");
    if (!Array.isArray(conversion.targets)) throw new Error("Invalid Semio conversion targets");
    const targets = conversion.targets.map((target) => text(target, atom, "target"));
    if (new Set(targets).size !== targets.length || targets.some((target) => !owned.has(target))) throw new Error("Semio conversion targets repeat or name an unowned format");
    return { subset: text(conversion.subset, atom, "subset"), direction: conversion.direction, package: text(conversion.package, atom, "package"), manifest: text(conversion.manifest, local, "manifest"), targets };
  });
  const packages = [text(catalog.package, atom, "catalog package"), ...conversions.map((row) => row.package)];
  if (new Set(packages).size !== packages.length) throw new Error("Semio conversion packages repeat");
  if (new Set(conversions.map((row) => `${row.subset}:${row.direction}`)).size !== conversions.length) throw new Error("Semio conversions repeat a subset direction");
  return { schema: doc.schema, catalog: { package: catalog.package as string, manifest: text(catalog.manifest, local, "catalog manifest"), source: text(catalog.source, local, "catalog source") }, owners, conversions };
}

/** 🧾️ The `# 🧩️ Capability Dependencies` lines of one conversion crate: its format crates, non-optional. */
export function renderConversionDependenciesV1(conversion: SemioConversionV1): string {
  return conversion.targets.map((target) => `${target} = { workspace = true }`).join("\n");
}

/** 📇️ The catalog's `ALL` list and its `# 🔗️ Conversion Dependencies` lines, in authority order. */
export function renderCatalogV1(authority: SemioConversionsV1): { source: string; dependencies: string } {
  const crate = (name: string) => name.replaceAll("-", "_");
  return {
    source: `pub const ALL: &[semio_s_artifact_stdio_semio_base::SemioConversion] = &[\n${authority.conversions.map((row) => `    ${crate(row.package)}::CONVERSION,`).join("\n")}\n];`,
    dependencies: authority.conversions.map((row) => `${row.package} = { workspace = true }`).join("\n"),
  };
}

/** 🧬️ Selects exact definition dependencies from admitted conversion-owner exports. */
export function selectSemioConversionDependenciesV1(packages: readonly string[], exports: readonly ExportV1[]): string[] {
  const declared = new Set(packages), seen = new Set<string>(), identities = new Set<string>();
  for (const value of exports) {
    if (!value || Object.keys(value).length !== 2 || typeof value.package !== "string" || typeof value.identity !== "string" || !declared.has(value.package) || seen.has(value.package) || identities.has(value.identity) || !/^[a-z][a-z0-9-]*(?:\.[a-z][a-z0-9-]*)+$/.test(value.identity)) throw new Error("Invalid Semio conversion definition export");
    seen.add(value.package); identities.add(value.identity);
  }
  return [...identities].sort();
}

function physical(root: string, path: string): void {
  const inside = relative(root, path);
  if (isAbsolute(inside) || inside === ".." || inside.startsWith(".." + sep)) throw new Error("Semio conversion authority escapes its repository");
  let current = root;
  for (const part of ["", ...inside.split(sep).filter(Boolean)]) {
    current = resolve(current, part);
    if (lstatSync(current).isSymbolicLink()) throw new Error("Semio conversion authority follows a symlink");
  }
  observeCargoPreparationInputV1(path, "presence");
}

function read(root: string, path: string): string {
  physical(root, path);
  const entry = lstatSync(path);
  if (!entry.isFile() || entry.size > 1024 * 1024) throw new Error("Semio conversion authority is not a bounded regular file");
  const bytes = readFileSync(path); observeCargoPreparationInputV1(path, "file", bytes); return bytes.toString("utf8");
}

function region(source: string, begin: string, end: string, content: string): string {
  const start = source.indexOf(begin), finish = source.indexOf(end);
  if (start < 0 || finish < start || source.indexOf(begin, start + begin.length) >= 0 || source.indexOf(end, finish + end.length) >= 0) throw new Error(`Missing or repeated Semio conversion region ${begin.trim()}`);
  return source.slice(0, start + begin.length) + content + (content ? "\n" : "") + source.slice(finish);
}

/** 📦️ Publishes the conversion crates' format dependencies, the catalog's conversion list and the definition dependencies from one authority. */
export function prepareSemioConversionDefinitionV1(repoRoot: string, ownerRoot: string): number {
  observeCargoPreparationSourceV1(fileURLToPath(import.meta.url));
  const authorityPath = resolve(ownerRoot, "🧩️composition/🔗️conversions/🔣️.json"), definitionPath = resolve(ownerRoot, "📜️artifact-definition.json");
  const source = read(repoRoot, authorityPath), previous = read(repoRoot, definitionPath), authority = admitSemioConversionsV1(JSON.parse(source));
  const observed = new Map<string, string>([[authorityPath, source], [definitionPath, previous]]);
  const exports: ExportV1[] = authority.owners.map((row) => {
    const path = resolve(ownerRoot, row.owner, "📜️artifact-definition.json"), bytes = read(repoRoot, path), document = JSON.parse(bytes);
    if (document.definition_version !== 1 || typeof document.id !== "string" || typeof document.artifact !== "string" || document.id !== `s.stdio.${document.artifact}`) throw new Error("Semio conversion owner definition is invalid");
    observed.set(path, bytes);
    return { package: row.package, identity: document.id };
  });
  const document = JSON.parse(previous);
  if (document.definition_version !== 1 || document.artifact !== "semio" || !Array.isArray(document.dependencies)) throw new Error("Invalid Semio definition authority");
  document.dependencies = selectSemioConversionDependenciesV1(authority.owners.map((row) => row.package), exports);
  const outputs = new Map<string, string>([[definitionPath, JSON.stringify(document, null, 2) + "\n"]]);
  for (const conversion of authority.conversions) {
    const path = resolve(ownerRoot, conversion.manifest), manifest = read(repoRoot, path), native = Bun.TOML.parse(manifest) as any;
    if (native.package?.name !== conversion.package) throw new Error(`Semio conversion manifest names another package: ${conversion.manifest}`);
    observed.set(path, manifest);
    outputs.set(path, region(manifest, "# 🧩️ Capability Dependencies\n", "# /🧩️ Capability Dependencies", renderConversionDependenciesV1(conversion)));
  }
  const catalog = renderCatalogV1(authority);
  const manifestPath = resolve(ownerRoot, authority.catalog.manifest), rustPath = resolve(ownerRoot, authority.catalog.source);
  const manifest = read(repoRoot, manifestPath), rust = read(repoRoot, rustPath);
  if ((Bun.TOML.parse(manifest) as any).package?.name !== authority.catalog.package) throw new Error("Semio catalog manifest names another package");
  observed.set(manifestPath, manifest); observed.set(rustPath, rust);
  outputs.set(manifestPath, region(manifest, "# 🔗️ Conversion Dependencies\n", "# /🔗️ Conversion Dependencies", catalog.dependencies));
  outputs.set(rustPath, region(rust, "//#region 🔗️Conversions\n", "//#endregion 🔗️Conversions", catalog.source));
  if ([...observed].some(([path, bytes]) => read(repoRoot, path) !== bytes)) throw new Error("Semio conversion authority changed during publication");
  for (const [path, next] of outputs) { if (readFileSync(path, "utf8") !== next) writeFileSync(path, next); observeCargoPreparationOutputV1(path, next); }
  return authority.conversions.length;
}
