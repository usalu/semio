import type { GeneratorPreviewProgressControlV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏭️generator/👁️preview/📈️progress/🟦️.ts";
import { atTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { afterAll, beforeAll, describe, expect, test } from "vitest";

import {
  PLAYGROUND_SESSION_OUTPUT_ROOT_ENV,
  playgroundSessionOutputPath,
  playgroundSessionStagedOutputPath,
  playgroundSessionViteAlias,
} from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { readGeneratedCatalogProjection } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import { PLAYGROUND_SESSION_ARTIFACT_KEY, renderPlaygroundSessionTypeScript, stagePlaygroundSession } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts";

type Fixture = {
  readonly schemaVersion: 1;
  readonly default: { readonly variant: string; readonly sourcePath: string; readonly producerScript: string; readonly producerSource: string };
  readonly isolation: {
    readonly canonicalOutputRoot: string;
    readonly stagedOutputRoot: string;
    readonly canonicalRootEnvironment: string;
    readonly resolverSource: string;
  };
  readonly staging: {
    readonly rootPath: string;
    readonly ownerDirectory: string;
    readonly sourceLeaf: string;
    readonly artifactKey: string;
    readonly viteConfigPath: string;
    readonly viteSpecifier: string;
    readonly variants: readonly { readonly variant: string; readonly pluginId: string }[];
  };
};

type FileSnapshot = { readonly exists: false } | { readonly exists: true; readonly bytes: number; readonly mtimeNs: string; readonly sha256: string };
type TreeSnapshot = { readonly exists: false } | { readonly exists: true; readonly files: Readonly<Record<string, FileSnapshot>> };

const workspace = resolve(import.meta.dirname, "../../../../..");
const owning = JSON.parse(readFileSync(resolve(import.meta.dirname, "../../🎮️session/🧫️fixtures/🔣️.json"), "utf8"));
const fixture = owning.integration as Fixture;
const projection = readGeneratedCatalogProjection(join(workspace, owning.publication.catalogPath));
const scratchBase = process.env.SEMIO_SESSION_OUTPUT_TEST_ROOT ? resolve(workspace, process.env.SEMIO_SESSION_OUTPUT_TEST_ROOT) : join(workspace, ".🧬semio/🦑️repo/⚡️cache/🧪️plugin-registry/playground-session-output");
let scratch = "";
const absolute = (path: string): string => join(workspace, path);
const sha256 = (bytes: string | Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const snapshotFile = (path: string): FileSnapshot => {
  if (!existsSync(path)) return { exists: false };
  const bytes = readFileSync(path);
  return { exists: true, bytes: bytes.length, mtimeNs: statSync(path, { bigint: true }).mtimeNs.toString(), sha256: sha256(bytes) };
};
const snapshotTree = (rootPath: string): TreeSnapshot => {
  if (!existsSync(rootPath)) return { exists: false };
  const files: [string, FileSnapshot][] = [];
  const pending = [rootPath];
  let visited = 0;
  while (pending.length > 0) {
    const directory = pending.pop()!;
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      if (++visited > 10_000) throw new Error(`Session staging inventory exceeded its 10000-entry bound: ${rootPath}`);
      const path = join(directory, entry.name);
      if (entry.isSymbolicLink()) throw new Error(`Session staging inventory refuses symbolic links: ${path}`);
      if (entry.isDirectory()) pending.push(path);
      else if (entry.isFile()) files.push([relative(rootPath, path).replaceAll("\\", "/"), snapshotFile(path)]);
      else throw new Error(`Session staging inventory found an unsupported entry: ${path}`);
    }
  }
  files.sort(([left], [right]) => Buffer.from(left).compare(Buffer.from(right)));
  return { exists: true, files: Object.fromEntries(files) };
};

beforeAll(() => { mkdirSync(scratchBase, { recursive: true }); scratch = mkdtempSync(join(scratchBase, "run-")); });
afterAll(() => { if (scratch) rmSync(scratch, { recursive: true, force: true }); });

describe("playground session output ownership", () => {
  test("the session defaults match the actual source owners", () => {
    expect(fixture.default.variant).toBe(owning.publication.variant);
    expect(fixture.isolation.canonicalRootEnvironment).toBe(PLAYGROUND_SESSION_OUTPUT_ROOT_ENV);
    expect(existsSync(absolute(fixture.isolation.resolverSource))).toBe(true);
    expect(fixture.staging.artifactKey).toBe(PLAYGROUND_SESSION_ARTIFACT_KEY);
    expect(new Set(fixture.staging.variants.map((row) => row.variant)).size).toBe(2);
    const devSource = readFileSync(absolute(fixture.default.producerSource), "utf8");
    const ensureStart = devSource.indexOf("export async function ensurePluginRegistry");
    const ensureEnd = devSource.indexOf("\nexport {", ensureStart);
    expect(ensureStart).toBeGreaterThanOrEqual(0);
    expect(ensureEnd).toBeGreaterThan(ensureStart);
    expect(devSource.slice(ensureStart, ensureEnd)).not.toContain("writePlaygroundSession");
    expect(readFileSync(absolute(fixture.staging.viteConfigPath), "utf8")).toContain("playgroundSessionViteAlias(sessionRoot, plugin)");
  });

  atTestLevel(test, "long")("an empty private root isolates two variants and the default from live outputs", async () => {
    const liveStagingRoot = absolute(fixture.staging.rootPath);
    const liveSnapshot = () => ({ canonical: snapshotFile(absolute(fixture.default.sourcePath)), staged: snapshotTree(liveStagingRoot) });
    const liveBefore = liveSnapshot();
    const canonicalRoot = join(scratch, fixture.isolation.canonicalOutputRoot);
    const stagedRoot = join(scratch, fixture.isolation.stagedOutputRoot);
    expect(existsSync(canonicalRoot)).toBe(false);
    expect(existsSync(stagedRoot)).toBe(false);

    const devScript = absolute(fixture.default.producerScript);
    const isolatedEnv = { ...process.env, [fixture.isolation.canonicalRootEnvironment]: canonicalRoot };
    const generated = spawnSync("bun", [devScript, "playground-session"], { cwd: workspace, env: isolatedEnv, encoding: "utf8", timeout: 20_000 });
    expect(generated.status, generated.stderr).toBe(0);
    const canonical = playgroundSessionOutputPath(canonicalRoot);
    const canonicalBytes = readFileSync(canonical, "utf8");
    const canonicalBefore = snapshotFile(canonical);
    expect(canonicalBytes).toBe(renderPlaygroundSessionTypeScript(owning.publication.variant, projection));

    for (const row of fixture.staging.variants) {
      const result = await stagePlaygroundSession(row.variant, stagedRoot, projection);
      expect(result.path).toBe(playgroundSessionStagedOutputPath(stagedRoot, row.variant));
      expect(result.session.variant).toBe(row.variant);
      expect(result.session.registryPluginId).toBe(row.pluginId);
      expect(existsSync(join(stagedRoot, row.variant, "🟦️session.ts"))).toBe(false);
      expect(JSON.parse(readFileSync(join(stagedRoot, row.variant, ".nx-artifact.json"), "utf8")).files).toEqual(["🎠️playgrounds.json", fixture.staging.artifactKey, "🔌️plugins.json"]);
      const native = await import(pathToFileURL(result.path).href + "?native=" + row.variant);
      expect(native.PLAYGROUND_SESSION.variant).toBe(row.variant);
      expect(native.PLAYGROUND_SESSION.registryPluginId).toBe(row.pluginId);

      const bunPath = join(scratch, "bun-" + row.variant + ".mjs");
      const built = spawnSync("bun", ["build", result.path, "--target=bun", "--format=esm", "--outfile=" + bunPath], { cwd: workspace, env: process.env, encoding: "utf8", timeout: 20_000 });
      expect(built.status, built.stderr).toBe(0);
      const importedBun = await import(pathToFileURL(bunPath).href + "?variant=" + row.variant);
      expect(importedBun.PLAYGROUND_SESSION.variant).toBe(row.variant);
      expect(importedBun.PLAYGROUND_SESSION.registryPluginId).toBe(row.pluginId);

      const esbuild = await import("esbuild");
      const oracle = await esbuild.build({ entryPoints: [result.path], bundle: true, format: "esm", platform: "node", write: false, logLevel: "silent" });
      const oraclePath = join(scratch, row.variant + ".mjs");
      writeFileSync(oraclePath, oracle.outputFiles[0].text);
      const importedOracle = await import(pathToFileURL(oraclePath).href + "?variant=" + row.variant);
      expect(importedOracle.PLAYGROUND_SESSION.variant).toBe(row.variant);
      expect(importedOracle.PLAYGROUND_SESSION.registryPluginId).toBe(row.pluginId);

      const alias = playgroundSessionViteAlias(stagedRoot, row.variant);
      expect(alias.find).toBe(fixture.staging.viteSpecifier);
      expect(alias.replacement).toBe(result.path);
      const selected = await import(pathToFileURL(alias.replacement).href + "?vite=" + row.variant);
      expect(selected.PLAYGROUND_SESSION.variant).toBe(row.variant);
      expect(selected.PLAYGROUND_SESSION.registryPluginId).toBe(row.pluginId);
      expect(snapshotFile(canonical)).toEqual(canonicalBefore);
    }

    const check = spawnSync("bun", [devScript, "playground-session", "check"], { cwd: workspace, env: isolatedEnv, encoding: "utf8", timeout: 20_000 });
    expect(check.status, check.stderr).toBe(0);
    const preview = spawnSync("bun", [devScript, "playground-session", "preview"], { cwd: workspace, env: isolatedEnv, encoding: "utf8", timeout: 20_000 });
    expect(preview.status, preview.stderr).toBe(0);
    const previewProjection = JSON.parse(preview.stdout);
    const file = previewProjection.nodes.find((node: { readonly nodeKind: string }) => node.nodeKind === "file");
    expect(Buffer.from(file.bytesBase64, "base64").toString("utf8")).toBe(canonicalBytes);
    expect(previewProjection.staleRemovals).toEqual([]);
    expect(snapshotFile(canonical)).toEqual(canonicalBefore);
    expect(liveSnapshot()).toEqual(liveBefore);
  }, 120_000);
});

 test("the actual owning session preview passes the strict normalization progress receiver", async () => {
  const lib = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"), ts = await import("typescript"), esbuild = await import("esbuild"), discovery = await import("../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"), progress = await import("../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏭️generator/👁️preview/📈️progress/🟦️.ts"), normalization = await import("../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts");
  const source = readFileSync(join(lib, "🧹️normalization/🟦️.ts"), "utf8"), syntax = ts.createSourceFile("normalization.ts", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), names = ["assertGeneratorPreviewTarget", "invokeGeneratorPreview"], functions = names.map(name => syntax.statements.find(statement => ts.isFunctionDeclaration(statement) && statement.name?.text === name)?.getText(syntax));
  expect(functions.every(Boolean)).toBe(true);
  const code = functions.join("\n") + "\nreturn invokeGeneratorPreview;", contract = discovery.loadCatalogTaxonomy().generatorContracts["s-playground-session"]!, declaration = readFileSync(join(workspace, contract.ownerPath!, "📋️project.json"), "utf8"), target = JSON.parse(declaration).targets[contract.previewTarget!.slice(contract.previewTarget!.lastIndexOf(":") + 1)], controller = new AbortController(), outputs = snapshotFile(absolute(fixture.default.sourcePath));
  for (const javascript of [ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText, (await esbuild.transform(code, { loader: "ts", target: "es2022" })).code]) {
    let received = 0;
    const invoke = new Function("nxTargetRecord", "requireRecord", "generatorPreviewExecution", "generatorPreviewResourceLimits", "receiveGeneratorPreviewProgressV1", "checkCancellation", "absolutePath", "spawnSync", "parseGeneratorPreviewManifest", "generatorPathCompare", "sha256", javascript)(() => target, (value: unknown) => value, discovery.generatorPreviewExecution, discovery.generatorPreviewResourceLimits, (stderr: string, policy: typeof contract.previewProgress, control: GeneratorPreviewProgressControlV1) => { const events = progress.receiveGeneratorPreviewProgressV1(stderr, policy, control); received += events.length; return events; }, () => { if (controller.signal.aborted) throw new Error("owned test cancellation"); }, join, spawnSync, normalization.parseGeneratorPreviewManifest, (left: string, right: string) => Buffer.from(left).compare(Buffer.from(right)), sha256);
    const actual = invoke({ repoRoot: workspace }, "s-playground-session", contract, { exclusions: [] });
    expect(received).toBeGreaterThan(0);
    expect(actual.manifest.contractId).toBe("s-playground-session");
    const file = actual.manifest.nodes.find((node: { nodeKind: string }) => node.nodeKind === "file");
    expect(Buffer.from(file.bytesBase64, "base64").toString("utf8")).toBe(renderPlaygroundSessionTypeScript(owning.publication.variant, projection));
    expect(snapshotFile(absolute(fixture.default.sourcePath))).toEqual(outputs);
  }
  console.log("[DEBUG] actual Specific publisher and two independently compiled normalization receivers admitted explicit bounded progress with unchanged artifact stdout and source outputs");
 }, 120000);
