import { strict as assert } from "node:assert";
import { readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { admitSemioConversionsV1, renderCatalogV1, renderConversionDependenciesV1, selectSemioConversionDependenciesV1 } from "../🟦️.ts";

/** 🧪️ Compares conversion authority admission and rendering with an independent schema oracle, and the committed authority with the crates the leaves actually name. */
export function runSemioConversionDefinitionChecks(): number {
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const corpus = read("🧫️fixtures/🔣️.json");
  assert.equal(new Set(corpus.cases.map((row: { id: string }) => row.id)).size, corpus.cases.length);
  for (const vector of corpus.cases) {
    const accepted = vector.exports.every((value: { package: string; identity: string }) => vector.packages.includes(value.package) && /^[a-z][a-z0-9-]*(?:\.[a-z][a-z0-9-]*)+$/.test(value.identity)) && new Set(vector.exports.map((value: { package: string }) => value.package)).size === vector.exports.length && new Set(vector.exports.map((value: { identity: string }) => value.identity)).size === vector.exports.length;
    assert.equal(accepted, vector.accepted, vector.id);
    if (accepted) assert.deepEqual(selectSemioConversionDependenciesV1(vector.packages, vector.exports), [...vector.exports.map((value: { identity: string }) => value.identity)].sort());
    else assert.throws(() => selectSemioConversionDependenciesV1(vector.packages, vector.exports));
  }
  const validShape = new Ajv({ strict: true }).compile(read("../🧬️schema/🔣️.json"));
  for (const vector of corpus.admissionCases) {
    const doc = vector.authority;
    const crates = (doc.owners ?? []).flatMap((row: { package: string; symbolPackages?: string[] }) => [row.package, ...(row.symbolPackages ?? [])]);
    const owned = new Set(crates);
    const conversions = doc.conversions ?? [];
    const packages = [doc.catalog?.package, ...conversions.map((row: { package: string }) => row.package)];
    const oracle = validShape(doc) && owned.size === crates.length && conversions.every((row: { targets: string[] }) => row.targets.every((target) => owned.has(target))) && new Set(packages).size === packages.length && new Set(conversions.map((row: { subset: string; direction: string }) => `${row.subset}:${row.direction}`)).size === conversions.length;
    assert.equal(oracle, vector.accepted, `${vector.id}: independent admission`);
    if (!vector.accepted) { assert.throws(() => admitSemioConversionsV1(doc), vector.id); continue; }
    const authority = admitSemioConversionsV1(doc);
    for (const row of authority.conversions) assert.equal(renderConversionDependenciesV1(row), vector.rendered[`${row.subset}-${row.direction}`], vector.id);
    const catalog = renderCatalogV1(authority);
    assert.equal(catalog.source, vector.rendered.catalogSource, vector.id);
    assert.equal(catalog.dependencies, vector.rendered.catalogDependencies, vector.id);
  }
  const root = resolve(import.meta.dir, "../../..");
  const authority = admitSemioConversionsV1(read("../🔣️.json"));
  const named = (directory: string): Set<string> => {
    const found = new Set<string>();
    const walk = (path: string) => { for (const entry of readdirSync(path, { withFileTypes: true })) { if (entry.name === "📦️packages" || entry.name === "🧪️tests") continue; const next = join(path, entry.name); if (entry.isDirectory()) walk(next); else if (entry.name.endsWith(".rs")) for (const match of readFileSync(next, "utf8").matchAll(/\bsemio_s_artifact_stdio_([a-z0-9_]+)::/g)) if (!match[1]!.startsWith("semio")) found.add(`semio-s-artifact-stdio-${match[1]!.replaceAll("_", "-")}`); } };
    walk(directory);
    return found;
  };
  for (const row of authority.conversions) assert.deepEqual([...row.targets].sort(), [...named(resolve(root, row.manifest, "../../.."))].filter((name) => name !== "semio-s-artifact-stdio-contract").sort(), `${row.package} names exactly its authored format crates`);
  const expected = authority.owners.map((row) => JSON.parse(readFileSync(resolve(root, row.owner, "📜️artifact-definition.json"), "utf8")).id).sort();
  assert.deepEqual(JSON.parse(readFileSync(resolve(root, "📜️artifact-definition.json"), "utf8")).dependencies, [...new Set(expected)]);
  return corpus.cases.length + corpus.admissionCases.length + authority.conversions.length + 1;
}
