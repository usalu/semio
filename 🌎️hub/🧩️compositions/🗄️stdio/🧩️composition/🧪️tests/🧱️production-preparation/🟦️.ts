import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync, existsSync, rmSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import * as toml from "@iarna/toml";
import Ajv from "ajv";
import cases from "../🧫️fixtures/🧱️production-preparation/🔣️.json";
import mounted from "../🧫️fixtures/🧱️production-preparation/mounted-apps.json";
import ownerSchema from "../../🧬️schema/🔣️.json";
import { BRIDGE_MANIFEST, CATALOG_MANIFEST, COMPONENT_MANIFEST, prepareStdioComposition, readStdioCompositionInputs } from "../../🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../..");
const owner = join(root, "🌎️hub/🧩️compositions/🗄️stdio");
type PackagePartition = { root: string; enum: string; variants: string[] };
type CompositionOwner = { id: string; root: string; enum: string; main: boolean; shards: PackagePartition[]; appLibraries?: PackagePartition[] };

function compositionOwners(): CompositionOwner[] {
  return JSON.parse(readFileSync(join(owner, "🧩️composition/🔣️.json"), "utf8")).owners;
}

test("the catalog owner routes every mounted surface through a library and preserves registration order", () => {
  const authority = JSON.parse(readFileSync(join(owner, "🧩️composition/🔣️.json"), "utf8"));
  expect(new Ajv({ strict: false }).compile(ownerSchema)(authority)).toBe(true);
  const catalog = (authority.owners as CompositionOwner[]).find((entry) => entry.id === mounted.owner)!;
  expect(catalog.appLibraries).toHaveLength(mounted.variants.length / 2);
  const oracle = new Database(":memory:");
  const variants = oracle.query<{ variant: string }, [string]>(
    "SELECT variant.value AS variant FROM json_each(?1) AS lib, json_each(lib.value, '$.variants') AS variant",
  ).all(JSON.stringify(catalog.appLibraries)).map((row) => row.variant);
  oracle.close();
  expect(variants).toEqual(mounted.variants);
  const apps = readStdioCompositionInputs(root, owner).flatMap((input) => input.contribution.apps)
    .filter((app) => app.owner === mounted.owner).sort((a, b) => a.order - b.order);
  expect(apps.map((app) => app.variant)).toEqual(mounted.variants);
  const router = readFileSync(join(owner, catalog.root), "utf8");
  for (const application of catalog.appLibraries!) {
    const directory = join(owner, dirname(application.root));
    const manifest = readFileSync(join(directory, "📦️packages/🦀️rust/Cargo.toml"), "utf8");
    const packageInfo = Bun.TOML.parse(manifest) as any;
    expect(packageInfo).toEqual(toml.parse(manifest));
    expect(packageInfo.lib["crate-type"]).toBeUndefined();
    expect(existsSync(join(directory, COMPONENT_MANIFEST))).toBe(false);
    const source = readFileSync(join(owner, application.root), "utf8");
    expect(source).toContain(`pub enum ${application.enum}: PluginApp`);
    for (const variant of application.variants) expect(source).toContain(`${variant}(VcsArtifactApp<`);
    expect(router).toContain(`${packageInfo.package.name.replaceAll("-", "_")}::${application.enum}`);
  }
  expect([...router.matchAll(/builder = builder\.(?:editor|viewer)::<([^>]+)>/gu)].map((match) => match[1]))
    .toEqual(apps.map((app) => app.type));
  expect(router).not.toContain("builder.component(");
});

for (const row of cases.cases) test(`production preparation isolates examples: ${row.id}`, () => {
  const outputs = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!outputs) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned test outputs");
  mkdirSync(outputs, { recursive: true });
  const scratch = mkdtempSync(join(outputs, "stdio-production-"));
  try {
    const owners = compositionOwners();
    const packageManifest = "📦️packages/🦀️rust/Cargo.toml";
    const paths = ["🧩️composition/🔣️.json", "📇️publication/📜️native-catalog.json", "📇️publication/🔣️.json", CATALOG_MANIFEST, BRIDGE_MANIFEST];
    for (const entry of owners) {
      const base = entry.main ? "." : dirname(entry.root);
      paths.push(join(base, packageManifest), join(base, COMPONENT_MANIFEST));
      paths.push(...(entry.appLibraries ?? entry.shards).map((partition) => join(dirname(partition.root), packageManifest)));
    }
    for (const path of paths) {
      const target = join(scratch, path);
      mkdirSync(dirname(target), { recursive: true });
      writeFileSync(target, readFileSync(join(owner, path)));
    }
    const manifestPath = join(scratch, COMPONENT_MANIFEST);
    const manifest = toml.parse(readFileSync(manifestPath, "utf8")) as any;
    const artifacts = manifest.package.metadata.semio.sources.artifacts.map((path: string) => resolve(owner, dirname(COMPONENT_MANIFEST), path));
    const original = readFileSync(manifestPath, "utf8");
    const declaration = original.match(/^artifacts\s*=.*$/mu);
    if (!declaration) throw Error("Composition artifact input authority missing");
    writeFileSync(manifestPath, original.replace(declaration[0], "artifacts = " + JSON.stringify(artifacts)));
    const examples = new Map<string, string>();
    if (row.examplesPresent) for (const path of cases.examplePaths) {
      const bytes = readFileSync(join(owner, path), "utf8");
      mkdirSync(dirname(join(scratch, path)), { recursive: true });
      writeFileSync(join(scratch, path), bytes);
      examples.set(path, bytes);
    }
    const result = prepareStdioComposition(root, scratch);
    expect(result.contributions).toBeGreaterThan(0);
    expect(result.apps).toBeGreaterThan(0);
    expect(result.receipts).toBeGreaterThan(0);
    const prepared = readFileSync(manifestPath, "utf8");
    expect(Bun.TOML.parse(prepared)).toEqual(toml.parse(prepared));
    const oracle = new Database(":memory:");
    try {
      const constructor = oracle.query<{ call: string }, [string, string, string, string, string]>("select printf('%s::<%s, %s>::%s(\"%s\")', ?1, ?2, ?3, ?4, ?5) as call");
      for (const component of owners) {
        const expected = constructor.get(cases.builder.type, cases.builder.labelState, component.enum, cases.builder.method, component.id);
        expect(expected).not.toBeNull();
        expect(readFileSync(join(scratch, component.root), "utf8")).toContain(expected!.call);
      }
    } finally {
      oracle.close();
    }
    let writes = 0;
    for (const path of cases.examplePaths) {
      if (row.examplesPresent) writes += Number(readFileSync(join(scratch, path), "utf8") !== examples.get(path));
      else writes += Number(existsSync(join(scratch, path)));
    }
    expect(writes).toBe(cases.expectedExampleWrites);
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
});

test("every runtime shard of an owner is one deployable component: the hub registers it by its crate name in shard order and its component crate builds its own cdylib", () => {
  const owners = compositionOwners();
  const sharded = owners.filter(entry => entry.shards.length > 0);
  expect(sharded.map(entry => entry.id)).toContain("stdio-pdf");
  for (const entry of sharded) {
    const hub = readFileSync(join(owner, entry.root), "utf8");
    const registered = [...hub.matchAll(/builder = builder\.component\("([^"]+)"\);/gu)].map(match => match[1]!);
    const crates = entry.shards.map(shard => (toml.parse(readFileSync(join(owner, dirname(shard.root), "📦️packages/🦀️rust/Cargo.toml"), "utf8")) as any).package.name as string);
    expect(registered, entry.id).toEqual(crates);
    for (const [index, shard] of entry.shards.entries()) {
      const component = toml.parse(readFileSync(join(owner, dirname(shard.root), COMPONENT_MANIFEST), "utf8")) as any;
      expect(component.package.name, entry.id).toBe(`${crates[index]}-component`);
      expect(component.lib["crate-type"], entry.id).toEqual(["cdylib"]);
      const entrypoint = readFileSync(join(owner, dirname(shard.root), "🧩️component/🦀️.rs"), "utf8");
      expect(entrypoint, entry.id).toContain(`${crates[index]!.replaceAll("-", "_")}::plugin`);
      const shardRoot = readFileSync(join(owner, shard.root), "utf8");
      expect(shardRoot, entry.id).toContain(`builder.component(${JSON.stringify(crates[index])})`);
      for (const variant of shard.variants) expect(shardRoot, `${entry.id} ${variant}`).toContain(`${variant}(`);
    }
    expect(new Set(registered).size, entry.id).toBe(registered.length);
  }
});
