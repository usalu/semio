/** 🌉️ Selects this artifact's authored conversion links from present target owners. */
import { existsSync, lstatSync, readFileSync, writeFileSync } from "node:fs";
import { relative, resolve, sep } from "node:path";

type TargetV1 = { package: string; manifest: string; feature: string };
type GroupV1 = { feature: string; requires: string[]; targets: TargetV1[] };
type LinksV1 = { schema: "semio.stdio.conversion-links/v1"; groups: GroupV1[] };
const atom = /^[a-z][a-z0-9-]*$/;

function closed(value: unknown, keys: string[]): Record<string, any> {
  if (!value || typeof value !== "object" || Array.isArray(value) || Object.keys(value).length !== keys.length || keys.some((key) => !Object.hasOwn(value, key))) throw new Error("Invalid closed conversion contribution");
  return value as Record<string, any>;
}

/** 🧬️ Admits the conversion schema before selecting any target or producing Cargo source. */
export function admitSemioConversionLinksV1(value: unknown): LinksV1 {
  const document = closed(value, ["schema", "groups"]);
  if (document.schema !== "semio.stdio.conversion-links/v1" || !Array.isArray(document.groups)) throw new Error("Unsupported conversion contribution");
  for (const value of document.groups) {
    const group = closed(value, ["feature", "requires", "targets"]);
    if (typeof group.feature !== "string" || !atom.test(group.feature) || !Array.isArray(group.requires) || group.requires.some((value) => typeof value !== "string" || !atom.test(value)) || new Set(group.requires).size !== group.requires.length || !Array.isArray(group.targets)) throw new Error("Invalid conversion group");
    for (const value of group.targets) {
      const target = closed(value, ["package", "manifest", "feature"]);
      if (typeof target.package !== "string" || !atom.test(target.package) || typeof target.feature !== "string" || !atom.test(target.feature) || typeof target.manifest !== "string" || !target.manifest.length) throw new Error("Invalid conversion target");
    }
    if (new Set(group.targets.map((target: TargetV1) => target.package)).size !== group.targets.length) throw new Error("Repeated conversion target");
  }
  const groups = document.groups as GroupV1[];
  if (new Set(groups.map((group) => group.feature)).size !== groups.length || groups.some((group) => group.requires.some((feature) => !groups.some((owner) => owner.feature === feature)))) throw new Error("Invalid conversion group authority");
  const features = groups.flatMap((group) => [group.feature, ...group.targets.map((target) => target.feature)]);
  if (new Set(features).size !== features.length) throw new Error("Repeated conversion feature");
  return document as LinksV1;
}

/** 🗂️ Retains every present target and omits only physically absent owner contributions. */
export function selectSemioConversionTargetsV1(value: unknown, present: ReadonlySet<string>): TargetV1[] {
  return admitSemioConversionLinksV1(value).groups.flatMap((group) => group.targets.filter((target) => present.has(target.package)));
}

function region(source: string, name: string, value: string): string {
  const begin = `# 🧩️ ${name}\n`, end = `# /🧩️ ${name}`, start = source.indexOf(begin), finish = source.indexOf(end);
  if (start < 0 || finish < start || source.indexOf(begin, start + begin.length) >= 0 || source.indexOf(end, finish + end.length) >= 0) throw new Error(`Invalid conversion output region: ${name}`);
  return source.slice(0, start + begin.length) + value + (value ? "\n" : "") + source.slice(finish);
}

/** 📦️ Publishes dependency and feature closure from this owner and its present target manifests. */
export function prepareSemioConversionComposition(ownerRoot: string): number {
  const path = resolve(ownerRoot, "📦️packages/🦀️rust/Cargo.toml"), previous = readFileSync(path, "utf8"), authorityPath = resolve(ownerRoot, "🧩️composition/🔗️conversions/🔣️.json"), authoritySource = readFileSync(authorityPath, "utf8");
  if (lstatSync(path).isSymbolicLink() || lstatSync(authorityPath).isSymbolicLink() || Buffer.byteLength(authoritySource) > 1024 * 1024) throw new Error("Invalid physical conversion owner authority");
  const links = admitSemioConversionLinksV1(JSON.parse(authoritySource)), present = new Set<string>(), observed = new Map<string, string>();
  const collection = resolve(ownerRoot, "..");
  for (const target of links.groups.flatMap((group) => group.targets)) {
    const manifest = resolve(ownerRoot, target.manifest), local = relative(collection, manifest);
    if (local.startsWith(".." + sep) || local === "..") throw new Error("Conversion target escapes its artifact collection");
    if (!existsSync(manifest)) {
      if (existsSync(resolve(manifest, "../../.."))) throw new Error(`Present conversion target has no Cargo manifest: ${target.package}`);
      continue;
    }
    let current = collection;
    for (const part of local.split(sep)) { current = resolve(current, part); if (lstatSync(current).isSymbolicLink()) throw new Error("Conversion target follows a symlink"); }
    const info = lstatSync(manifest);
    if (!info.isFile() || info.size > 1024 * 1024) throw new Error("Conversion target manifest is not bounded and regular");
    const source = readFileSync(manifest, "utf8"), document = Bun.TOML.parse(source) as any;
    if (document.package?.name !== target.package) throw new Error("Conversion target differs from its physical Cargo owner");
    observed.set(manifest, source); present.add(target.package);
  }
  const selected = selectSemioConversionTargetsV1(links, present), features: string[] = [];
  for (const group of links.groups) {
    const targets = group.targets.filter((target) => present.has(target.package));
    features.push(`${group.feature} = ${JSON.stringify([...group.requires, ...targets.map((target) => target.feature)])}`);
    for (const target of targets) features.push(`${target.feature} = ${JSON.stringify([`dep:${target.package}`])}`);
  }
  const dependencies = [...new Set(selected.map((target) => target.package))].sort().map((name) => `${name} = { workspace = true, optional = true }`).join("\n");
  const next = region(region(previous, "Conversion Features", features.join("\n")), "Conversion Dependencies", dependencies);
  if (readFileSync(path, "utf8") !== previous || readFileSync(authorityPath, "utf8") !== authoritySource || [...observed].some(([path, source]) => readFileSync(path, "utf8") !== source)) throw new Error("Conversion authority changed during publication");
  if (next !== previous) writeFileSync(path, next);
  return selected.length;
}
