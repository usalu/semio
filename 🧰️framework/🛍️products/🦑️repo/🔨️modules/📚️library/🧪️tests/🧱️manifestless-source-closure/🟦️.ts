import { describe, expect, test } from "bun:test";
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { tmpdir } from "node:os";
import { basename, dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import ts from "typescript";
import { semanticDirectoryKindId } from "../../🔍️discovery/🟦️.ts";

type Owner = Readonly<{ path: string; language: "python" | "typescript"; anchors: readonly string[] }>;
type Source = Readonly<{ legacy: string; legacyDisposition: "absent" | "thin-command"; owners: readonly Owner[]; consumers: readonly string[] }>;
type Alias = Readonly<{ config: string; specifier: string; target: string; consumer: string }>;
type Fixture = Readonly<{
  sources: readonly Source[];
  supportMoves: readonly Readonly<{ legacy: string; owner: string; language: "dotnet"; consumer: string }>[];
  aliases: readonly Alias[];
  command: Readonly<{ path: string; maximumLines: number; forbiddenExports: readonly string[] }>;
  cacheAuthority: readonly Readonly<{ path: string; fragments: readonly string[] }>[];
  descriptorProbe: Readonly<{ bytes: readonly number[]; base64: string }>;
  directoryContexts: readonly Readonly<{ name: string; parentKind: string; kind: string }>[];
}>;

const fixtureRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(fixtureRoot, "🧫️fixtures/🧱️manifestless-source-closure/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(fixtureRoot, "🧬️schema/🧱️manifestless-source-closure/🔣️.json"), "utf8"));
const leaf = { python: "🐍️.py", typescript: "🟦️.ts" } as const;

describe("manifestless source closure", () => {
  test("validates the portable seven-source projection and every owned context", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    const taxonomy = JSON.parse(readFileSync(resolve(fixtureRoot, "🔣️taxonomy.json"), "utf8"));
    for (const row of fixture.directoryContexts) expect(semanticDirectoryKindId(row.name, taxonomy, { parentKindId: row.parentKind }), `${row.parentKind}/${row.name}`).toBe(row.kind);
  });

  test("moves all seven behavior sources to admitted anonymous semantic leaves", () => {
    expect(fixture.sources).toHaveLength(7);
    for (const source of fixture.sources) {
      if (source.legacyDisposition === "absent") expect(existsSync(resolve(repoRoot, source.legacy)), source.legacy).toBe(false);
      for (const owner of source.owners) {
        const path = resolve(repoRoot, owner.path);
        expect(existsSync(path), owner.path).toBe(true);
        expect(basename(path), owner.path).toBe(leaf[owner.language]);
        const ignored = spawnSync("git", ["check-ignore", "--no-index", "--quiet", "--", owner.path], { cwd: repoRoot, encoding: "utf8" });
        expect(ignored.status, `${owner.path}: ${ignored.stderr}`).toBe(1);
        const content = readFileSync(path, "utf8");
        for (const anchor of owner.anchors) expect(content, `${owner.path}: ${anchor}`).toContain(anchor);
      }
      for (const consumer of source.consumers) expect(existsSync(resolve(repoRoot, consumer)), consumer).toBe(true);
    }
  });

  test("co-locates the adjacent .NET native host while preserving its package mount", () => {
    expect(fixture.supportMoves).toHaveLength(1);
    for (const row of fixture.supportMoves) {
      expect(existsSync(resolve(repoRoot, row.legacy)), row.legacy).toBe(false);
      expect(basename(resolve(repoRoot, row.owner)), row.owner).toBe("🔷️.cs");
      expect(existsSync(resolve(repoRoot, row.owner)), row.owner).toBe(true);
      expect(readFileSync(resolve(repoRoot, row.consumer), "utf8"), row.consumer).toContain("../../🖥️host/🔷️.cs");
    }
  });

  test("resolves each internal coordinator alias to its exact native source", () => {
    for (const row of fixture.aliases) {
      const configPath = resolve(repoRoot, row.config);
      const loaded = ts.readConfigFile(configPath, ts.sys.readFile);
      expect(loaded.error, row.config).toBeUndefined();
      const parsed = ts.parseJsonConfigFileContent(loaded.config, ts.sys, dirname(configPath), undefined, configPath);
      const resolved = ts.resolveModuleName(row.specifier, resolve(repoRoot, row.consumer), parsed.options, ts.sys).resolvedModule?.resolvedFileName;
      expect(resolve(resolved ?? ""), row.specifier).toBe(resolve(dirname(configPath), row.target));
      expect(readFileSync(resolve(repoRoot, row.consumer), "utf8"), row.consumer).toContain(`"${row.specifier}"`);
    }
  });

  test("keeps the plugin command leaf as routing only", () => {
    const source = readFileSync(resolve(repoRoot, fixture.command.path), "utf8");
    expect(source.trimEnd().split("\n").length).toBeLessThanOrEqual(fixture.command.maximumLines);
    for (const name of fixture.command.forbiddenExports) expect(source).not.toContain(`export ${name}`);
  });

  test("hashes semantic browser sources in support and component materialization targets", () => {
    for (const row of fixture.cacheAuthority) {
      const source = readFileSync(resolve(repoRoot, row.path), "utf8");
      for (const fragment of row.fragments) expect(source, `${row.path}: ${fragment}`).toContain(fragment);
    }
  });

  test("registers the closure gate in both editor launch authorities", () => {
    const name = "🧹clean🧩️taxonomy🧪️manifestless-source-closure";
    const command = "bun nx run @semio-tech/repo-lib:test-manifestless-source-closure";
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const launch = Bun.JSONC.parse(readFileSync(resolve(repoRoot, path), "utf8")) as { readonly configurations: readonly { readonly name?: string; readonly command?: string }[] };
      expect(launch.configurations.filter((row) => row.name === name && row.command === command), path).toHaveLength(1);
    }
  });

  test("executes the descriptor probe in native Node and rejects an incomplete actor API", async () => {
    const descriptor = await import(pathToFileURL(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts")).href);
    const root = mkdtempSync(join(tmpdir(), "semio-manifestless-descriptor-"));
    try {
      const complete = join(root, "complete.mjs"), incomplete = join(root, "incomplete.mjs");
      const methods = (names: readonly string[]) => `{${names.map((name) => `${JSON.stringify(name)}(){}`).join(",")}}`;
      const exports = Object.entries(descriptor.ACTOR_COMPONENT_EXPORTS as Record<string, readonly string[]>).map(([name, names]) => `export const ${name}=${name === "describe" ? `{async describe(){return Uint8Array.from(${JSON.stringify(fixture.descriptorProbe.bytes)})}}` : methods(names)};`);
      writeFileSync(complete, exports.join("\n"));
      writeFileSync(incomplete, exports.filter((line) => !line.startsWith("export const reactor=")).join("\n"));
      const run = (path: string) => spawnSync("node", ["--input-type=module", "--eval", descriptor.PLUGIN_DESCRIPTOR_PROBE_SOURCE, path], { encoding: "utf8" });
      const accepted = run(complete);
      expect(accepted.status, accepted.stderr).toBe(0);
      expect(accepted.stdout).toBe(fixture.descriptorProbe.base64);
      const rejected = run(incomplete);
      expect(rejected.status).not.toBe(0);
      expect(rejected.stderr).toContain("Missing actor export reactor.poll");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("passes every moved source through an installed language parser", () => {
    for (const owner of fixture.sources.flatMap(({ owners }) => owners)) {
      const source = readFileSync(resolve(repoRoot, owner.path), "utf8");
      if (owner.language === "typescript") {
        const parsed = ts.createSourceFile(owner.path, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
        expect(parsed.parseDiagnostics, owner.path).toHaveLength(0);
      } else {
        const parsed = spawnSync("python3", ["-c", "import ast,sys; ast.parse(sys.stdin.read())"], { input: source, encoding: "utf8" });
        expect(parsed.status, parsed.stderr).toBe(0);
      }
    }
  });
});
