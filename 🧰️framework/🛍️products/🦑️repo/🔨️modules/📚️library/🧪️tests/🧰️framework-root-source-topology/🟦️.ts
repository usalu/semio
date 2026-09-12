import { describe, expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { basename, resolve } from "node:path";
import Ajv from "ajv";
import * as TOML from "@iarna/toml";
import { classifyPackageSource, semanticDirectoryKindId } from "../../🔍️discovery/🟦️.ts";

type Move = Readonly<{ source: string; owner: string; kind: "rust" | "typescript"; anchor: string; consumer: string }>;
type NodePackage = Readonly<{ manifest: string; entry: string; name: string; maximumEntryLines: number; testOwner: string; vitestConfig: string }>;
type Fixture = Readonly<{
  moves: readonly Move[];
  packages: Readonly<{ rust: Readonly<{ manifest: string; entry: string; name: string; feature: string; maximumEntryLines: number }>; typescript: NodePackage }>;
  directoryContexts: readonly Readonly<{ name: string; parentKind: string; kind: string }>[];
}>;

const fixtureRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(fixtureRoot, "🧫️fixtures/🧰️framework-root-source-topology/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(fixtureRoot, "🧬️schema/🧰️framework-root-source-topology/🔣️.json"), "utf8"));
const basenames = { rust: "🦀️.rs", typescript: "🟦️.ts" } as const;

describe("framework root source topology", () => {
  test("validates the portable source projection and every new directory context", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    const taxonomy = JSON.parse(readFileSync(resolve(fixtureRoot, "🔣️taxonomy.json"), "utf8"));
    for (const row of fixture.directoryContexts) expect(semanticDirectoryKindId(row.name, taxonomy, { parentKindId: row.parentKind }), `${row.parentKind}/${row.name}`).toBe(row.kind);
  });

  test("owns behavior in anonymous leaves and leaves both package roots as explicit glue", () => {
    for (const row of fixture.moves) {
      const source = readFileSync(resolve(repoRoot, row.source), "utf8");
      const owner = resolve(repoRoot, row.owner);
      expect(source, row.source).not.toContain(row.anchor);
      expect(existsSync(owner), row.owner).toBe(true);
      expect(basename(owner), row.owner).toBe(basenames[row.kind]);
      expect(readFileSync(owner, "utf8"), row.anchor).toContain(row.anchor);
      expect(source, row.owner).toContain(row.owner.split("/").slice(-3).join("/"));
      expect(existsSync(resolve(repoRoot, row.consumer)), row.consumer).toBe(true);
    }
  });

  test("preserves Cargo and Node package identities and typegen activation", () => {
    const taxonomy = JSON.parse(readFileSync(resolve(fixtureRoot, "🔣️taxonomy.json"), "utf8"));
    const rust = fixture.packages.rust;
    const cargo = TOML.parse(readFileSync(resolve(repoRoot, rust.manifest), "utf8")) as { readonly package?: { readonly name?: string }; readonly lib?: { readonly path?: string }; readonly features?: Readonly<Record<string, unknown>> };
    const rustEntry = readFileSync(resolve(repoRoot, rust.entry), "utf8");
    expect(cargo.package?.name).toBe(rust.name);
    expect(cargo.lib?.path).toBe("🦀️.rs");
    expect(cargo.features?.[rust.feature]).toBeDefined();
    expect(rustEntry.trimEnd().split("\n").length).toBeLessThanOrEqual(rust.maximumEntryLines);
    expect(classifyPackageSource(rustEntry, taxonomy.packageGlueGrammar.rust).role).toBe("declaration");
    for (const node of [fixture.packages.typescript]) {
      const nodeEntry = readFileSync(resolve(repoRoot, node.entry), "utf8");
      const manifest = JSON.parse(readFileSync(resolve(repoRoot, node.manifest), "utf8")) as { readonly name?: string; readonly exports?: Readonly<Record<string, string>> };
      expect(manifest.name).toBe(node.name);
      expect(manifest.exports?.["."]).toBe(`./${basename(node.entry)}`);
      expect(existsSync(resolve(repoRoot, node.entry))).toBe(true);
      expect(nodeEntry.trimEnd().split("\n").length).toBeLessThanOrEqual(node.maximumEntryLines);
      expect(classifyPackageSource(nodeEntry, taxonomy.packageGlueGrammar.typescript).role).toBe("declaration");
      const testOwner = readFileSync(resolve(repoRoot, node.testOwner), "utf8");
      expect(testOwner).toContain("if (import.meta.vitest)");
      expect(testOwner).toContain(`import("${node.name}")`);
      expect(readFileSync(resolve(repoRoot, node.vitestConfig), "utf8")).toContain(node.testOwner.split("/").slice(-3).join("/"));
    }
  });

  test("passes every TypeScript owner through Bun's browser compiler", async () => {
    for (const row of fixture.moves.filter(({ kind }) => kind === "typescript")) {
      const build = await Bun.build({ entrypoints: [resolve(repoRoot, row.owner)], target: "browser", format: "esm", external: ["*"], throw: false });
      expect(build.success, `${row.owner}\n${build.logs.join("\n")}`).toBe(true);
    }
  }, 30_000);

  test("registers the portable gate in both editor launch authorities", () => {
    const name = "🧹clean🧩️taxonomy🧪️framework-root-source-topology";
    const command = "bun nx run @semio-tech/repo-lib:test-framework-root-source-topology";
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const launch = Bun.JSONC.parse(readFileSync(resolve(repoRoot, path), "utf8")) as { readonly configurations: readonly { readonly name?: string; readonly command?: string }[] };
      expect(launch.configurations.filter((row) => row.name === name && row.command === command), path).toHaveLength(1);
    }
  });
});
