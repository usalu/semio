import { describe, expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { basename, dirname, resolve } from "node:path";
import Ajv from "ajv";
import * as TOML from "@iarna/toml";

type Move = Readonly<{ source: string; owner: string; kind: string; sourceDisposition: "removed" | "package-glue"; anchor: string; consumers: readonly string[] }>;
type Fixture = Readonly<{
  moves: readonly Move[];
  packageBoundary: Readonly<{ cargoRoot: string; nodeRoot: string; nodeManifest: string; nxManifest: string; nodeEntry: string; name: string }>;
}>;

const fixtureRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(fixtureRoot, "🧫️fixtures/🖥️os-source-topology/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(fixtureRoot, "🧬️schema/🖥️os-source-topology/🔣️.json"), "utf8"));
const expectedBasename: Readonly<Record<string, string>> = {
  javascript: "🟨️.js",
  powershell: "🔵️.ps1",
  rust: "🦀️.rs",
  typescript: "🟦️.ts",
  "typescript-declaration": "🟦️.d.ts",
  "typescript-jsx": "🟦️.tsx",
};

describe("OS source topology", () => {
  test("validates the portable source, owner and consumer projection", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
  });

  test("preserves authored anchors under anonymous implementation leaves", () => {
    for (const row of fixture.moves) {
      const source = resolve(repoRoot, row.source);
      expect(existsSync(source), row.source).toBe(row.sourceDisposition === "package-glue");
      if (row.sourceDisposition === "package-glue") expect(readFileSync(source, "utf8"), row.source).not.toContain(row.anchor);
      const owner = resolve(repoRoot, row.owner);
      expect(existsSync(owner), row.owner).toBe(true);
      expect(basename(owner), row.owner).toBe(expectedBasename[row.kind]);
      expect(readFileSync(owner, "utf8"), row.anchor).toContain(row.anchor);
      for (const consumer of row.consumers) expect(existsSync(resolve(repoRoot, consumer)), consumer).toBe(true);
    }
  });

  test("separates WGPU Node and Cargo packages while retaining their identities", () => {
    const row = fixture.packageBoundary;
    expect(row.nodeRoot).not.toBe(row.cargoRoot);
    const cargo = TOML.parse(readFileSync(resolve(repoRoot, row.cargoRoot, "Cargo.toml"), "utf8")) as { readonly package?: { readonly name?: string } };
    expect(cargo.package?.name).toBe("semio-framework-os-renderer-wgpu");
    const node = JSON.parse(readFileSync(resolve(repoRoot, row.nodeRoot, row.nodeManifest), "utf8")) as { readonly name?: string; readonly exports?: Readonly<Record<string, string>> };
    const nx = JSON.parse(readFileSync(resolve(repoRoot, row.nodeRoot, row.nxManifest), "utf8")) as { readonly name?: string };
    expect(node.name).toBe(row.name);
    expect(nx.name).toBe(row.name);
    expect(node.exports?.["."]).toBe(`./${row.nodeEntry}`);
    expect(existsSync(resolve(repoRoot, row.nodeRoot, row.nodeEntry))).toBe(true);
    expect(existsSync(resolve(repoRoot, row.cargoRoot, row.nodeManifest))).toBe(false);
  });

  test("passes every moved TypeScript source through Bun's compiler", async () => {
    for (const row of fixture.moves.filter((entry) => entry.kind.startsWith("typescript"))) {
      const build = await Bun.build({ entrypoints: [resolve(repoRoot, row.owner)], target: "bun", format: "esm", external: ["*"], throw: false });
      expect(build.success, `${row.owner}\n${build.logs.join("\n")}`).toBe(true);
    }
  }, 30_000);

  test("keeps configured Cargo targets attached to existing source leaves", () => {
    const manifests = new Set(fixture.moves.filter((row) => row.kind === "rust").flatMap((row) => row.consumers).filter((path) => basename(path) === "Cargo.toml"));
    for (const path of manifests) {
      const manifest = TOML.parse(readFileSync(resolve(repoRoot, path), "utf8")) as { readonly bin?: readonly { readonly path?: string }[] };
      for (const target of manifest.bin ?? []) if (target.path) expect(existsSync(resolve(repoRoot, dirname(path), target.path)), `${path}:${target.path}`).toBe(true);
    }
  });
});
