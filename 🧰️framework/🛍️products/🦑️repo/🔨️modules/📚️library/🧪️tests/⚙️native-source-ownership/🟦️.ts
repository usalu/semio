import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import { testBuiltTreeRetirementFixture } from "../../../../../../🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧪️tests/🔬️built-tree-retirement/🟦️.ts";
import { findWorkspaceRoot } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { fixedFilenameContractIdsForPath, inspectRustModuleGraphFacts, loadTaxonomy } from "../../🔍️discovery/🟦️.ts";
import corpus from "../../🧫️fixtures/⚙️native-source-ownership/🔣️.json";


const root = findWorkspaceRoot(import.meta.dir);
const read = (path: string) => readFileSync(join(root, path), "utf8");

test("Repo taxonomy keeps every exact surface companion filename owner and refuses unowned siblings", () => {
  expect(corpus["version"]).toEqual(1);
  const fixture = JSON.parse(read(corpus.surface.fixture)) as { directoryName: string; module: string; types: string; wasm: string; wasmTypes: string; contracts: string[] };
  const names = [fixture.module, fixture.types, fixture.wasm, fixture.wasmTypes];
  const taxonomy = loadTaxonomy();
  for (const [index, name] of names.entries()) {
    const path = corpus.surface.root + "/" + fixture.directoryName + "/" + name;
    expect(fixedFilenameContractIdsForPath(path, taxonomy)).toContain(fixture.contracts[index]!);
    expect(fixedFilenameContractIdsForPath(corpus.surface.root + "/unowned/" + name, taxonomy)).not.toContain(fixture.contracts[index]!);
  }
}, 30_000);

test("Repo Rust discovery binds the exact built retirement native test to its canonical portable fixture", () => {
  const native = read(corpus.built.source);
  const modules = inspectRustModuleGraphFacts(native).modules.filter(module => module.name === "tests" && module.conditional && !module.inline && module.pathTarget !== null);
  expect(modules.length).toBe(1);
  const testUrl = pathToFileURL(join(dirname(join(root, corpus.built.source)), modules[0]!.pathTarget!));
  const fixtureUrl = pathToFileURL(join(root, corpus.built.fixture));
  const includes = [...readFileSync(testUrl, "utf8").matchAll(/include_str!\(\s*"([^"\n]+)"\s*\)/gu)];
  expect(includes.some(match => new URL(match[1]!, testUrl).href === fixtureUrl.href)).toBe(true);
});

test("the unchanged neutral built retirement fixture and typed-depth laws execute beside the Repo source join", () => {
  testBuiltTreeRetirementFixture();
});
