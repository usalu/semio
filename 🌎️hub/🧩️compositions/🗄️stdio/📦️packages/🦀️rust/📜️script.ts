#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🗄️ `@semio-tech/stdio-plugin` router: `repo ./📜️script.ts test`. */
import assert from "node:assert/strict";
import { existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ARTIFACT_OWNER } from "./🧮️component-constants/🟦️.ts";
import { FlowRetainedDecodeScript } from "./🔬️catalog-contract-and-flow-decode/🟦️.ts";
import { HomeIoSurfaceScript, stdioWalkText } from "./🗺️stdio-text-walk-and-surfaces/🟦️.ts";
import { BenchScript, CompositionScript, EditorComponentCheckScript, MutationInventoryScript, TestScript } from "./📚️editor-catalog-and-command-scripts/🟦️.ts";
import { CatalogRootScript } from "./🏗️catalog-root-script/🟦️.ts";
import { NativeCodecProjectionScript, SnapshotSqliteTestScript } from "./🪢️native-codec-and-sqlite-scripts/🟦️.ts";

type ArtifactDirectoryRow = { directory: string; id: string; kind: string; responsibility: string };

function artifactDirectorySemanticKey(segment: string): string {
  return segment.replace(/^[^A-Za-z0-9_.]+/u, "");
}

function resolveTaxonomyDirectoryReference(sourcePath: string, token: string, stdioRoot: string, repoRoot: string, accepts: (pieces: string[]) => boolean): string {
  const pieces = token.split("/");
  if (!accepts(pieces)) return token;
  let current: string;
  if (token.startsWith("✏️s/")) current = repoRoot;
  else if (token.startsWith("🗿️artifacts/")) current = stdioRoot;
  else if (token.startsWith("/../../🗿️artifacts/")) current = import.meta.dir;
  else if (token.startsWith(".") || token.startsWith("..")) current = dirname(sourcePath);
  else return token;
  const rendered: string[] = [];
  for (const piece of pieces) {
    if (!piece) { rendered.push(piece); continue; }
    if (piece === ".") { rendered.push(piece); continue; }
    if (piece === "..") { current = dirname(current); rendered.push(piece); continue; }
    const exact = join(current, piece);
    if (existsSync(exact)) { current = exact; rendered.push(piece); continue; }
    const key = artifactDirectorySemanticKey(piece);
    const matches = existsSync(current)
      ? readdirSync(current, { withFileTypes: true }).filter((entry) => artifactDirectorySemanticKey(entry.name) === key)
      : [];
    if (matches.length !== 1) return token;
    current = join(current, matches[0]!.name);
    rendered.push(matches[0]!.name);
  }
  return rendered.join("/");
}

function resolveArtifactDirectoryReference(sourcePath: string, token: string, stdioRoot: string, repoRoot: string): string {
  return resolveTaxonomyDirectoryReference(sourcePath, token, stdioRoot, repoRoot, (pieces) => {
    const artifactIndex = pieces.findIndex((piece) => artifactDirectorySemanticKey(piece) === "artifacts");
    return artifactIndex >= 0 && pieces.slice(artifactIndex + 1).some((piece) => artifactDirectorySemanticKey(piece) === "jpg");
  });
}

function renderArtifactDirectoryReferences(sourcePath: string, content: string, stdioRoot: string, repoRoot: string, directory: string): string {
  const canonicalRoot = content.replace(/(🗿️artifacts\/)([^/\s"'`]+)/gu, (match, prefix: string, candidate: string) => artifactDirectorySemanticKey(candidate) === "jpg" ? `${prefix}${directory}` : match);
  return canonicalRoot.replace(/[^\s"'`]+/gu, (token) => resolveArtifactDirectoryReference(sourcePath, token, stdioRoot, repoRoot));
}

class ArtifactDirectoryWiringScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const mode = segments[0];
    const artifact = segments[1];
    if ((mode !== "generate" && mode !== "check") || artifact !== "jpg") throw new Error("artifact-directory-wiring expects generate|check jpg");
    const repoRoot = this.repoRoot;
    const stdioRoot = resolve(repoRoot, ARTIFACT_OWNER);
    const artifactsRoot = join(stdioRoot, "🗿️artifacts");
    const definitions = readdirSync(artifactsRoot, { withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      .map((entry) => ({ directory: entry.name, path: join(artifactsRoot, entry.name, "📜️artifact-definition.json") }))
      .filter((entry) => existsSync(entry.path))
      .map((entry) => ({ directory: entry.directory, definition: JSON.parse(readFileSync(entry.path, "utf8")) as { artifact?: unknown; directory?: unknown; id?: unknown } }));
    if (definitions.length !== 36) throw new Error(`artifact-directory-wiring expected 36 physical definitions, got ${definitions.length}`);
    const selected = definitions.find((entry) => entry.definition.artifact === artifact);
    if (!selected || selected.definition.id !== `s.stdio.${artifact}` || selected.definition.directory !== selected.directory) throw new Error(`artifact-directory-wiring ${artifact} definition does not own its physical directory`);
    const collectionPath = join(artifactsRoot, "🔣️.json");
    const collection = JSON.parse(readFileSync(collectionPath, "utf8")) as { "x-semio"?: { members?: ArtifactDirectoryRow[] } };
    const members = collection["x-semio"]?.members;
    if (!Array.isArray(members) || members.length !== 36) throw new Error("artifact-directory-wiring requires the complete 36-row artifact collection");
    const row = members.find((entry) => entry.id === `s.stdio.${artifact}`);
    if (!row || row.kind !== "artifact") throw new Error(`artifact-directory-wiring missing ${artifact} collection row`);
    row.directory = selected.directory;
    const expected = new Map<string, string>();
    expected.set(collectionPath, `${JSON.stringify(collection, null, 2)}\n`);
    const visit = (directory: string): void => {
      for (const entry of readdirSync(directory, { withFileTypes: true })) {
        if (["target", "node_modules", "🗑️generated"].includes(entry.name)) continue;
        const path = join(directory, entry.name);
        if (entry.isDirectory()) { visit(path); continue; }
        if (path === import.meta.filename) continue;
        const original = readFileSync(path, "utf8");
        const rendered = renderArtifactDirectoryReferences(path, original, stdioRoot, repoRoot, selected.directory);
        if (rendered !== original) expected.set(path, rendered);
      }
    };
    visit(stdioRoot);
    const stale = [...expected].filter(([path, content]) => readFileSync(path, "utf8") !== content);
    if (mode === "check") {
      if (stale.length > 0) throw new Error(`artifact-directory-wiring ${artifact} has ${stale.length} stale files; first: ${relative(repoRoot, stale[0]![0])}`);
    } else {
      for (const [path, content] of stale) writeFileSync(path, content);
    }
    const remaining = stdioWalkText(stdioRoot).filter((path) => [...readFileSync(path, "utf8").matchAll(/🗿️artifacts\/([^/\s"'`]+)/gu)].some((match) => artifactDirectorySemanticKey(match[1]!) === "jpg" && match[1] !== selected.directory));
    if (remaining.length > 0) throw new Error(`artifact-directory-wiring ${artifact} left stale identity in ${relative(repoRoot, remaining[0]!)}`);
    console.log(`[stdio] artifact-directory-wiring ${mode}: ${artifact}=${selected.directory}; ${stale.length} files ${mode === "generate" ? "regenerated" : "stale"}`);
  }
}

type SubsetDirectoryOutcome = "accepted" | "rewritten" | "missing-directory" | "ambiguous-directory";
type SubsetDirectoryCase = { id: string; directories: string[]; reference: string; outcome: SubsetDirectoryOutcome };
type SubsetDirectoryFixture = { version: number; artifact: string; standard: string; subset: string; schema: string; canonicalDirectory: string; cases: SubsetDirectoryCase[] };

function selectSubsetDirectory(directories: string[], subset: string, reference: string): { outcome: SubsetDirectoryOutcome; directory?: string } {
  const matches = directories.filter((directory) => artifactDirectorySemanticKey(directory) === subset);
  if (matches.length === 0) return { outcome: "missing-directory" };
  if (matches.length !== 1) return { outcome: "ambiguous-directory" };
  return { outcome: matches[0] === reference ? "accepted" : "rewritten", directory: matches[0] };
}

function resolveSubsetDirectoryReference(sourcePath: string, token: string, stdioRoot: string, repoRoot: string, artifact: string, standard: string, subset: string): string {
  return resolveTaxonomyDirectoryReference(sourcePath, token, stdioRoot, repoRoot, (pieces) => {
    const keys = pieces.map(artifactDirectorySemanticKey);
    const artifacts = keys.indexOf("artifacts");
    const standards = keys.indexOf("standards", artifacts + 1);
    const subsets = keys.indexOf("subsets", standards + 1);
    return artifacts >= 0 && keys[artifacts + 1] === artifact && standards > artifacts && keys[standards + 1] === standard && subsets > standards && keys[subsets + 1] === subset;
  });
}

class SubsetDirectoryWiringScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const [mode, artifact, subset] = segments;
    if ((mode !== "generate" && mode !== "check") || artifact !== "semio" || subset !== "mesh") throw new Error("subset-directory-wiring expects generate|check semio mesh");
    
    const fixtureRoot = join(resolve(this.repoRoot, ARTIFACT_OWNER), "🧪️tests/🧫️fixtures/🧭️wiring/🗂️subset-directory-wiring");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as SubsetDirectoryFixture;
    assert.equal(fixture.cases.length, 4);
    assert.equal(new Set(fixture.cases.map((row) => row.id)).size, fixture.cases.length);
    for (const row of fixture.cases) {
      const selected = selectSubsetDirectory(row.directories, fixture.subset, row.reference);
      assert.equal(selected.outcome, row.outcome, row.id);
      if (selected.directory) assert.equal(selected.directory, fixture.canonicalDirectory, row.id);
    }
    const repoRoot = this.repoRoot;
    const stdioRoot = resolve(repoRoot, ARTIFACT_OWNER);
    const subsetsRoot = join(stdioRoot, `🗿️artifacts/🧿️${fixture.artifact}/🏅️standards/🔖️${fixture.standard}/🪆️subsets`);
    const selection = selectSubsetDirectory(readdirSync(subsetsRoot, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => entry.name), fixture.subset, fixture.canonicalDirectory);
    if (!selection.directory || selection.outcome !== "accepted") throw new Error(`subset-directory-wiring ${fixture.subset} physical authority is ${selection.outcome}`);
    const manifestPath = join(subsetsRoot, "🔣️.json");
    const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as { artifact?: unknown; standard?: unknown; subsets?: Record<string, { schema?: unknown }> };
    if (manifest.artifact !== `s.stdio.${fixture.artifact}` || manifest.standard !== fixture.standard || manifest.subsets?.[fixture.subset]?.schema !== fixture.schema) throw new Error("subset-directory-wiring logical mesh manifest row is absent or detached from its schema");
    const expected = new Map<string, string>();
    for (const path of stdioWalkText(stdioRoot)) {
      if (path === import.meta.filename) continue;
      const original = readFileSync(path, "utf8");
      const rendered = original.replace(/[^\s"'`]+/gu, (token) => resolveSubsetDirectoryReference(path, token, stdioRoot, repoRoot, fixture.artifact, fixture.standard, fixture.subset));
      if (rendered !== original) expected.set(path, rendered);
    }
    const stale = [...expected].filter(([path, content]) => readFileSync(path, "utf8") !== content);
    if (mode === "check" && stale.length > 0) throw new Error(`subset-directory-wiring ${fixture.subset} has ${stale.length} stale files; first: ${relative(repoRoot, stale[0]![0])}`);
    if (mode === "generate") for (const [path, content] of stale) writeFileSync(path, content);
    const remaining = stdioWalkText(stdioRoot).filter((path) => [...readFileSync(path, "utf8").matchAll(/🗿️artifacts\/([^/\s"'`]+)\/🏅️standards\/([^/\s"'`]+)\/🪆️subsets\/([^/\s"'`]+)/gu)].some((match) => artifactDirectorySemanticKey(match[1]!) === fixture.artifact && artifactDirectorySemanticKey(match[2]!) === fixture.standard && artifactDirectorySemanticKey(match[3]!) === fixture.subset && match[3] !== selection.directory));
    if (remaining.length > 0) throw new Error(`subset-directory-wiring ${fixture.subset} left stale identity in ${relative(repoRoot, remaining[0]!)}`);
    console.log(`Stdio subset directory oracle: ${fixture.cases.length} accepted/stale/missing/ambiguous cases`);
    console.log(`[stdio] subset-directory-wiring ${mode}: ${fixture.artifact}/${fixture.subset}=${selection.directory}; ${stale.length} files ${mode === "generate" ? "regenerated" : "stale"}`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("composition", CompositionScript).register("mutation-inventory", MutationInventoryScript).register("snapshot-sqlite", SnapshotSqliteTestScript).register("editor-component-check", EditorComponentCheckScript).register("native-codec-projection", NativeCodecProjectionScript).register("test", TestScript).register("bench", BenchScript).register("catalog-root", CatalogRootScript).register("flow-retained-decode-check", FlowRetainedDecodeScript).register("artifact-directory-wiring", ArtifactDirectoryWiringScript).register("subset-directory-wiring", SubsetDirectoryWiringScript).register("home-io-surface", HomeIoSurfaceScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
