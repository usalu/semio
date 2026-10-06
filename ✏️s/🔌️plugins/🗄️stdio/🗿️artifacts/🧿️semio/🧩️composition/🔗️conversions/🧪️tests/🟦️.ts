import { strict as assert } from "node:assert";
import { existsSync, readFileSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { selectSemioConversionDependenciesV1, prepareSemioConversionDefinitionV1 } from "../🟦️.ts";

/** 🧪️ Compares authored Semio dependencies with actual lower conversion exports. */
export function runSemioConversionDefinitionChecks(): number {
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const corpus = read("../🧫️fixtures/🔣️.json");
  assert.equal(new Set(corpus.cases.map((row: { id: string }) => row.id)).size, corpus.cases.length);
  for (const vector of corpus.cases) {
    const accepted = vector.exports.every((value: { package: string; identity: string }) => vector.packages.includes(value.package) && /^[a-z][a-z0-9-]*(?:\.[a-z][a-z0-9-]*)+$/.test(value.identity)) && new Set(vector.exports.map((value: { package: string }) => value.package)).size === vector.exports.length && new Set(vector.exports.map((value: { identity: string }) => value.identity)).size === vector.exports.length;
    assert.equal(accepted, vector.accepted, vector.id);
    if (accepted) {
      const oracle = JSON.parse(JSON.stringify(vector.exports.map((value: { identity: string }) => value.identity))).sort();
      assert.deepEqual(oracle, vector.dependencies);
      assert.deepEqual(selectSemioConversionDependenciesV1(vector.packages, vector.exports), oracle);
    } else assert.throws(() => selectSemioConversionDependenciesV1(vector.packages, vector.exports));
  }
  const root = resolve(import.meta.dir, "../../..");
  const links = read("../🔣️.json");
  const targets = new Map<string, string>();
  for (const group of links.groups) for (const target of group.targets) {
    const owner = resolve(root, target.owner);
    if (existsSync(owner)) targets.set(target.package, JSON.parse(readFileSync(resolve(owner, "📜️artifact-definition.json"), "utf8")).id);
  }
  const expected = [...new Set(targets.values())].sort();
  assert.deepEqual(JSON.parse(readFileSync(resolve(root, "📜️artifact-definition.json"), "utf8")).dependencies, expected);
  return corpus.cases.length + 2 + physicalChecks();
}

function physicalChecks(): number {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(artifacts, { recursive: true });
  const repo = mkdtempSync(join(artifacts, "semio-conversion-definition-")), owner = join(repo, "semio");
  mkdirSync(join(owner, "🧩️composition/🔗️conversions"), { recursive: true }); mkdirSync(join(owner, "📦️packages/🦀️rust"), { recursive: true });
  const links = { schema: "semio.repository.cargo-capability-links/v1", groups: [{ feature: "conversion", requires: [], targets: ["alpha", "bravo"].map(name => ({ package: `neutral-${name}`, owner: `../${name}`, manifest: `../${name}/Cargo.toml`, feature: `conversion-${name}` })) }] };
  const definition = join(owner, "📜️artifact-definition.json"), manifest = join(owner, "📦️packages/🦀️rust/Cargo.toml");
  writeFileSync(join(owner, "🧩️composition/🔗️conversions/🔣️.json"), JSON.stringify(links));
  writeFileSync(definition, JSON.stringify({ definition_version: 1, artifact: "semio", dependencies: ["s.stdio.stale"] }));
  writeFileSync(manifest, '[package]\nname="neutral-parent"\nversion="0.0.0"\n[features]\n# 🧩️ Capability Features\n# /🧩️ Capability Features\n[dependencies]\n# 🧩️ Capability Dependencies\n# /🧩️ Capability Dependencies\n');
  const source = (name: string) => { mkdirSync(join(repo, name)); writeFileSync(join(repo, name, "Cargo.toml"), `[package]\nname="neutral-${name}"\n`); writeFileSync(join(repo, name, "📜️artifact-definition.json"), JSON.stringify({ definition_version: 1, artifact: name, id: `s.stdio.${name}` })); };
  source("alpha"); source("bravo");
  assert.equal(prepareSemioConversionDefinitionV1(repo, owner), 2);
  assert.deepEqual(JSON.parse(readFileSync(definition, "utf8")).dependencies, ["s.stdio.alpha", "s.stdio.bravo"]);
  rmSync(join(repo, "alpha"), { recursive: true }); assert.equal(prepareSemioConversionDefinitionV1(repo, owner), 1);
  assert.deepEqual(JSON.parse(readFileSync(definition, "utf8")).dependencies, ["s.stdio.bravo"]);
  const retained = readFileSync(definition, "utf8"), native = readFileSync(manifest, "utf8");
  mkdirSync(join(repo, "alpha")); assert.throws(() => prepareSemioConversionDefinitionV1(repo, owner));
  assert.equal(readFileSync(definition, "utf8"), retained); assert.equal(readFileSync(manifest, "utf8"), native);
  rmSync(join(repo, "alpha"), { recursive: true }); rmSync(join(repo, "bravo"), { recursive: true });
  assert.equal(prepareSemioConversionDefinitionV1(repo, owner), 0);
  assert.deepEqual(JSON.parse(readFileSync(definition, "utf8")).dependencies, []);
  rmSync(repo, { recursive: true });
  return 4;
}
