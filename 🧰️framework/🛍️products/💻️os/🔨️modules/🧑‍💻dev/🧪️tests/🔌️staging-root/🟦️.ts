/**
 * 🔌️ One staging root, one freshness rule. Fixture-driven (`🧫️fixtures/🔌️staging-root.json`), so the
 * contract is readable without TypeScript and portable to a second implementation. Pinned to the node
 * environment: at `long` the suite default is jsdom and every assertion here is filesystem/path work.
 *
 * @vitest-environment node
 */
import { mkdirSync, mkdtempSync, readFileSync, rmSync, utimesSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import picomatch from "picomatch";
import { URL as OracleURL } from "whatwg-url";
import { describe, expect, it } from "vitest";
import {
  COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES,
  GENERATED_COMPONENT_OWNER_FILES,
  UNWATCHED_COMPONENT_SOURCE_DIRECTORIES,
  healthyPreparedComponents,
  componentSourceContentHash,
  newestComponentSourceMtime,
  preparedComponentReportLines,
  preparedComponentVerdict,
  pluginModulesRoot,
  pluginModulesRootIn,
  readStagedSourceContentHash,
  readStagedSourceStatIndex,
  resolveBootSourceContentHashes,
  stagedModuleMtime,
  writeStagedSourceContentHash,
  writeStagedSourceFreshness,
  stagedModuleReportLines,
  stagedModuleVerdict,
  type StagedModuleFacts,
  type StagedModuleVerdict,
} from "../../♻️activation/🟦️.ts";

const suiteDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteDir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🔌️staging-root.json"), "utf8")) as {
  readonly root: {
    readonly packageRelativePath: string;
    readonly modulesDirectoryName: string;
    readonly profiles: readonly ("dev" | "release")[];
    readonly rejectedProfiles: readonly string[];
    readonly consumers: readonly { readonly id: string; readonly renderer: "react" | "wgpu"; readonly profile: "dev" | "release" }[];
    readonly retiredRoots: readonly string[];
    readonly retiredSymbols: readonly string[];
    readonly sourceFiles: readonly string[];
  };
  readonly freshness: {
    readonly command: string;
    readonly cases: readonly { readonly name: string; readonly facts: StagedModuleFacts; readonly verdict: StagedModuleVerdict; readonly line: string | null }[];
    readonly walk: { readonly outputDirectories: readonly string[]; readonly generatedOwnerFiles: readonly string[] };
  };
};

/** @emoji 🔮️ Independent oracle: `whatwg-url` is a third-party, spec-complete URL implementation whose
 * relative-reference resolution is a second implementation of the `..`/segment arithmetic behind the one
 * staging root — the expected path is rebuilt from the repository root through it, never through the
 * `node:path` call the production function itself uses. */
function oracleStagingRoot(profile: string): string {
  const base = pathToFileURL(`${repoRoot}/`).href;
  const relativeReference = `${fixture.root.packageRelativePath}/${profile}/${fixture.root.modulesDirectoryName}`.split("/").map(encodeURIComponent).join("/");
  return fileURLToPath(new OracleURL(relativeReference, base).href);
}

describe("one plugin staging root", () => {
  it.each(fixture.root.profiles)("resolves %s to the single fixture-declared directory", (profile) => {
    expect(pluginModulesRoot(profile)).toBe(join(repoRoot, fixture.root.packageRelativePath, profile, fixture.root.modulesDirectoryName));
  });

  it.each(fixture.root.profiles)("agrees with whatwg-url's independent relative-reference resolution for %s", (profile) => {
    expect(pluginModulesRoot(profile)).toBe(oracleStagingRoot(profile));
  });

  it("projects every declared staging-input consumer to one root per profile", () => {
    const byProfile = new Map<string, Set<string>>();
    for (const consumer of fixture.root.consumers) {
      const roots = byProfile.get(consumer.profile) ?? new Set<string>();
      roots.add(pluginModulesRoot(consumer.profile));
      byProfile.set(consumer.profile, roots);
    }
    for (const [profile, roots] of byProfile) expect([...roots], `${profile} must resolve to exactly one root`).toHaveLength(1);
    expect(new Set(fixture.root.consumers.map((consumer) => consumer.renderer))).toEqual(new Set(["react", "wgpu"]));
  });

  it.each(fixture.root.profiles)("derives %s identically with and without an explicit workspace", (profile) => {
    expect(pluginModulesRoot(profile)).toBe(pluginModulesRootIn(repoRoot, profile));
  });

  it("keeps a sandboxed consumer inside its own workspace", () => {
    const sandbox = mkdtempSync(join(tmpdir(), "semio-staging-workspace-"));
    try {
      expect(pluginModulesRootIn(sandbox, "release")).toBe(join(sandbox, fixture.root.packageRelativePath, "release", fixture.root.modulesDirectoryName));
      expect(pluginModulesRootIn(sandbox, "release")).not.toBe(pluginModulesRoot("release"));
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  it.each(fixture.root.rejectedProfiles)("refuses %j for an explicit workspace too", (profile) => {
    expect(() => pluginModulesRootIn(repoRoot, profile as "dev" | "release")).toThrow(/staging profile/u);
  });

  it("keeps the two profiles apart", () => {
    expect(pluginModulesRoot("dev")).not.toBe(pluginModulesRoot("release"));
  });

  it.each(fixture.root.rejectedProfiles)("refuses %j rather than inventing a third root", (profile) => {
    expect(() => pluginModulesRoot(profile as "dev" | "release")).toThrow(/staging profile/u);
  });

  it("leaves no retired root or symbol in any declared source file", () => {
    expect(new Set(fixture.root.sourceFiles).size).toBe(fixture.root.sourceFiles.length);
    for (const file of fixture.root.sourceFiles) {
      const source = readFileSync(join(repoRoot, file), "utf8");
      for (const retired of fixture.root.retiredRoots) expect(source, `${file} still names the retired root ${retired}`).not.toContain(`${retired}"`);
      for (const symbol of fixture.root.retiredSymbols) expect(source, `${file} still names the retired symbol ${symbol}`).not.toContain(symbol);
    }
  });

  it("tracks every declared source through the registered test inputs", () => {
    const project = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json"), "utf8"));
    const expected = [...fixture.root.sourceFiles, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🔌️staging-root.json", relative(repoRoot, fileURLToPath(import.meta.url)).replaceAll("\\", "/")].map((path) => `{workspaceRoot}/${path}`).sort();
    expect([...project.namedInputs.stagingRootSources].sort()).toEqual(expected);
    const targets = Object.values(project.targets) as { options?: { command?: string }; inputs?: readonly string[] }[];
    for (const target of targets.filter((target) => target.options?.command?.startsWith("bun ./📜️script.ts test"))) expect(target.inputs).toContain("stagingRootSources");
  });
});

/** @emoji 🩺️ The healthy-set rule a 60-plugin host stands or falls on: one red crate is a missing
 * component, never an un-bootable product. Every case is stated against the same pure pair
 * (`preparedComponentVerdict` → `healthyPreparedComponents`) the preparation pass and the activation
 * receipt both call, so there is no second, drifting notion of "prepared". */
describe("healthy prepared set", () => {
  const complete = (pluginId: string) => ({ pluginId, directoryPresent: true, descriptorPluginId: pluginId, bridgePresent: true, artifactMarkerPresent: true });

  it.each([
    ["a complete staging directory is prepared", complete("space"), "prepared", undefined],
    ["a crate that never compiled leaves no directory", { pluginId: "block", directoryPresent: false, bridgePresent: false, artifactMarkerPresent: false }, "unstaged", "no staged module directory"],
    ["an unreadable descriptor is a fact, not a throw", { pluginId: "block", directoryPresent: true, bridgePresent: true, artifactMarkerPresent: true }, "incomplete", "no readable 🔣️.json descriptor"],
    ["a descriptor naming another plugin is incomplete", { ...complete("block"), descriptorPluginId: "draw" }, "incomplete", "descriptor names draw"],
    ["a missing bridge is incomplete", { ...complete("block"), bridgePresent: false }, "incomplete", "no module bridge"],
    ["a missing Nx marker is incomplete", { ...complete("block"), artifactMarkerPresent: false }, "incomplete", "no .nx-artifact.json marker"],
  ] as const)("%s", (_name, facts, kind, detail) => {
    expect(preparedComponentVerdict(facts)).toEqual({ pluginId: facts.pluginId, kind, ...(detail === undefined ? {} : { detail }) });
  });

  it("excludes one red component and still boots the host", () => {
    const verdicts = [complete("space"), complete("draw"), { pluginId: "block", directoryPresent: false, bridgePresent: false, artifactMarkerPresent: false }].map(preparedComponentVerdict);
    const healthy = healthyPreparedComponents(verdicts, "space");
    expect(healthy.refusal).toBeUndefined();
    expect(healthy.prepared).toEqual(["space", "draw"]);
    expect(preparedComponentReportLines(healthy.excluded, "CMD")).toEqual(["[excluded] block: unstaged — no staged module directory — run: CMD"]);
  });

  it("refuses when the host's own component is the one that failed", () => {
    const verdicts = [{ pluginId: "space", directoryPresent: false, bridgePresent: false, artifactMarkerPresent: false }, complete("draw")].map(preparedComponentVerdict);
    expect(healthyPreparedComponents(verdicts, "space").refusal).toBe("Host component space is unstaged — no staged module directory");
  });

  it("refuses when nothing prepared at all", () => {
    const verdicts = [{ pluginId: "draw", directoryPresent: false, bridgePresent: false, artifactMarkerPresent: false }].map(preparedComponentVerdict);
    expect(healthyPreparedComponents(verdicts, "space").refusal).toBe("No component of 1 prepared");
  });
});

describe("staged module freshness", () => {
  it.each(fixture.freshness.cases.map((row) => [row.name, row] as const))("%s", (_name, row) => {
    expect(stagedModuleVerdict(row.facts)).toEqual(row.verdict);
    expect(stagedModuleReportLines([stagedModuleVerdict(row.facts)], fixture.freshness.command)).toEqual(row.line === null ? [] : [row.line]);
  });

  it("reports every non-fresh component and only those", () => {
    const verdicts = fixture.freshness.cases.map((row) => stagedModuleVerdict(row.facts));
    const expected = fixture.freshness.cases.map((row) => row.line).filter((line): line is string => line !== null);
    expect(stagedModuleReportLines(verdicts, fixture.freshness.command)).toEqual(expected);
  });

  it("reads staged and source mtimes off a real tree and skips every declared output directory", () => {
    const sandbox = mkdtempSync(join(tmpdir(), "semio-staging-root-"));
    try {
      const stagedDirectory = join(sandbox, "staged"), sourceRoot = join(sandbox, "source");
      mkdirSync(stagedDirectory, { recursive: true });
      writeFileSync(join(stagedDirectory, "component.core.wasm"), "staged");
      writeFileSync(join(stagedDirectory, ".nx-artifact.json"), "{}");
      utimesSync(join(stagedDirectory, "component.core.wasm"), new Date(2_000_000), new Date(2_000_000));
      // 🕰️ The Nx ownership marker is written AFTER every payload file, so counting it would make every
      // staged module look newer than its own bytes and hide exactly the drift this check exists for.
      utimesSync(join(stagedDirectory, ".nx-artifact.json"), new Date(9_000_000), new Date(9_000_000));
      expect(stagedModuleMtime(stagedDirectory)).toBe(2_000_000);
      expect(stagedModuleMtime(join(sandbox, "absent"))).toBeUndefined();

      mkdirSync(join(sourceRoot, "🦀️rust"), { recursive: true });
      writeFileSync(join(sourceRoot, "🦀️rust", "plugin.rs"), "fn main() {}");
      utimesSync(join(sourceRoot, "🦀️rust", "plugin.rs"), new Date(1_000_000), new Date(1_000_000));
      for (const directory of fixture.freshness.walk.outputDirectories) {
        mkdirSync(join(sourceRoot, "🦀️rust", directory), { recursive: true });
        const output = join(sourceRoot, "🦀️rust", directory, "artifact.bin");
        writeFileSync(output, "output");
        utimesSync(output, new Date(8_000_000), new Date(8_000_000));
      }
      const newest = newestComponentSourceMtime(sourceRoot);
      expect(newest?.mtimeMs).toBe(1_000_000);
      expect(relative(sourceRoot, newest!.path).split(/[\\/]/).join("/")).toBe("🦀️rust/plugin.rs");
      expect(newestComponentSourceMtime(join(sandbox, "absent"))).toBeUndefined();

      writeFileSync(join(sourceRoot, "🦀️rust", "later.rs"), "fn later() {}");
      utimesSync(join(sourceRoot, "🦀️rust", "later.rs"), new Date(3_000_000), new Date(3_000_000));
      expect(newestComponentSourceMtime(sourceRoot)?.mtimeMs).toBe(3_000_000);
      const before = componentSourceContentHash(sourceRoot);
      writeStagedSourceContentHash(stagedDirectory, before);
      expect(readStagedSourceContentHash(stagedDirectory)).toBe(before);
      expect(stagedModuleVerdict({ pluginId: "procedural", role: "plugin", activationTracked: false, stagedAtMs: stagedModuleMtime(stagedDirectory), newestSourceMs: newestComponentSourceMtime(sourceRoot)?.mtimeMs, sourceContentSha256: componentSourceContentHash(sourceRoot), stagedSourceContentSha256: before }).kind).toBe("fresh");
      writeFileSync(join(sourceRoot, "changed.rs"), "changed");
      const after = componentSourceContentHash(sourceRoot);
      expect(after).not.toBe(before);
      expect(stagedModuleVerdict({ pluginId: "procedural", role: "plugin", activationTracked: false, stagedAtMs: stagedModuleMtime(stagedDirectory), newestSourceMs: newestComponentSourceMtime(sourceRoot)?.mtimeMs, sourceContentSha256: after, stagedSourceContentSha256: before }).kind).toBe("source-changed");
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  it("boot freshness: unchanged tree reuses every file digest and stays fresh", () => {
    const sandbox = mkdtempSync(join(tmpdir(), "semio-stat-index-unchanged-"));
    try {
      const stagedDirectory = join(sandbox, "staged"), sourceRoot = join(sandbox, "source");
      mkdirSync(join(sourceRoot, "src"), { recursive: true });
      writeFileSync(join(sourceRoot, "src", "a.rs"), "fn a() {}");
      utimesSync(join(sourceRoot, "src", "a.rs"), new Date(1_000_000), new Date(1_000_000));
      const content = writeStagedSourceFreshness(stagedDirectory, sourceRoot);
      const first = resolveBootSourceContentHashes({ sourceRoot, moduleDirectory: stagedDirectory, receiptSourceContentSha256: content });
      expect(first.hashedFileCount).toBe(0);
      expect(first.reusedFileCount).toBe(1);
      expect(first.sourceContentSha256).toBe(content);
      expect(stagedModuleVerdict({ pluginId: "procedural", role: "plugin", activationTracked: false, stagedAtMs: 1, sourceContentSha256: first.sourceContentSha256, stagedSourceContentSha256: content }).kind).toBe("fresh");
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  it("boot freshness: touched-but-identical file rehashes one entry and stays fresh", () => {
    const sandbox = mkdtempSync(join(tmpdir(), "semio-stat-index-touch-"));
    try {
      const stagedDirectory = join(sandbox, "staged"), sourceRoot = join(sandbox, "source");
      mkdirSync(join(sourceRoot, "src"), { recursive: true });
      const file = join(sourceRoot, "src", "a.rs");
      writeFileSync(file, "fn a() {}");
      utimesSync(file, new Date(1_000_000), new Date(1_000_000));
      const content = writeStagedSourceFreshness(stagedDirectory, sourceRoot);
      utimesSync(file, new Date(2_000_000), new Date(2_000_000));
      const second = resolveBootSourceContentHashes({ sourceRoot, moduleDirectory: stagedDirectory, receiptSourceContentSha256: content });
      expect(second.hashedFileCount).toBe(1);
      expect(second.reusedFileCount).toBe(0);
      expect(second.sourceContentSha256).toBe(content);
      expect(stagedModuleVerdict({ pluginId: "procedural", role: "plugin", activationTracked: false, stagedAtMs: 1, sourceContentSha256: second.sourceContentSha256, stagedSourceContentSha256: content }).kind).toBe("fresh");
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  it("boot freshness: edited file is detected as source-changed", () => {
    const sandbox = mkdtempSync(join(tmpdir(), "semio-stat-index-edit-"));
    try {
      const stagedDirectory = join(sandbox, "staged"), sourceRoot = join(sandbox, "source");
      mkdirSync(join(sourceRoot, "src"), { recursive: true });
      const file = join(sourceRoot, "src", "a.rs");
      writeFileSync(file, "fn a() {}");
      utimesSync(file, new Date(1_000_000), new Date(1_000_000));
      const content = writeStagedSourceFreshness(stagedDirectory, sourceRoot);
      writeFileSync(file, "fn a() { /* edited */ }");
      utimesSync(file, new Date(2_000_000), new Date(2_000_000));
      const second = resolveBootSourceContentHashes({ sourceRoot, moduleDirectory: stagedDirectory, receiptSourceContentSha256: content });
      expect(second.hashedFileCount).toBe(1);
      expect(second.sourceContentSha256).not.toBe(content);
      expect(stagedModuleVerdict({ pluginId: "procedural", role: "plugin", activationTracked: false, stagedAtMs: 1, sourceContentSha256: second.sourceContentSha256, stagedSourceContentSha256: content }).kind).toBe("source-changed");
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  it("boot freshness: added or removed file is detected as source-changed", () => {
    const sandbox = mkdtempSync(join(tmpdir(), "semio-stat-index-add-remove-"));
    try {
      const stagedDirectory = join(sandbox, "staged"), sourceRoot = join(sandbox, "source");
      mkdirSync(join(sourceRoot, "src"), { recursive: true });
      writeFileSync(join(sourceRoot, "src", "a.rs"), "fn a() {}");
      utimesSync(join(sourceRoot, "src", "a.rs"), new Date(1_000_000), new Date(1_000_000));
      const content = writeStagedSourceFreshness(stagedDirectory, sourceRoot);
      expect(readStagedSourceStatIndex(stagedDirectory)?.files).toHaveLength(1);
      writeFileSync(join(sourceRoot, "src", "b.rs"), "fn b() {}");
      utimesSync(join(sourceRoot, "src", "b.rs"), new Date(2_000_000), new Date(2_000_000));
      const added = resolveBootSourceContentHashes({ sourceRoot, moduleDirectory: stagedDirectory, receiptSourceContentSha256: content });
      expect(added.sourceContentSha256).not.toBe(content);
      expect(stagedModuleVerdict({ pluginId: "procedural", role: "plugin", activationTracked: false, stagedAtMs: 1, sourceContentSha256: added.sourceContentSha256, stagedSourceContentSha256: content }).kind).toBe("source-changed");
      rmSync(join(sourceRoot, "src", "b.rs"));
      writeStagedSourceFreshness(stagedDirectory, sourceRoot);
      const afterAddPersisted = readStagedSourceContentHash(stagedDirectory)!;
      rmSync(join(sourceRoot, "src", "a.rs"));
      writeFileSync(join(sourceRoot, "src", "only.rs"), "fn only() {}");
      const removed = resolveBootSourceContentHashes({ sourceRoot, moduleDirectory: stagedDirectory, receiptSourceContentSha256: afterAddPersisted });
      expect(removed.sourceContentSha256).not.toBe(afterAddPersisted);
      expect(stagedModuleVerdict({ pluginId: "procedural", role: "plugin", activationTracked: false, stagedAtMs: 1, sourceContentSha256: removed.sourceContentSha256, stagedSourceContentSha256: afterAddPersisted }).kind).toBe("source-changed");
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  /** @emoji 🔮️ Independent oracle: `picomatch` is the glob engine chokidar filters with, so the declared
   * output-directory exclusions are decided a second time by a third-party matcher rather than by a
   * second reading of the exported list. */
  it("agrees with picomatch on which walked paths are build output", () => {
    const isOutput = picomatch(fixture.freshness.walk.outputDirectories.map((directory) => `**/${directory}/**`), { dot: true });
    expect([...UNWATCHED_COMPONENT_SOURCE_DIRECTORIES].sort()).toEqual([...fixture.freshness.walk.outputDirectories].sort());
    for (const directory of fixture.freshness.walk.outputDirectories) expect(isOutput(`crate/${directory}/artifact.bin`), directory).toBe(true);
    for (const kept of ["crate/🦀️.rs", "crate/🧬️schema/🔣️.json", "crate/🧪️tests/🟦️.ts"]) expect(isOutput(kept), kept).toBe(false);
    expect(COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES).toBeGreaterThan(0);
  });

  /** @emoji 🛂️ The describe outputs sit at the owner ROOT, next to the sources, and are tracked in git —
   * so only a name-and-depth rule can tell them from authored files. The walk must skip exactly those two
   * at exactly that level: a `🔣️.json` one directory deeper is a schema or a fixture and decides
   * freshness like any other source. */
  it("treats the owner root's describe outputs as build output and a nested 🔣️.json as source", () => {
    const sandbox = mkdtempSync(join(tmpdir(), "semio-staging-descriptor-"));
    try {
      const sourceRoot = join(sandbox, "🗄️stdio");
      mkdirSync(join(sourceRoot, "🗿️artifacts", "🧬️schema"), { recursive: true });
      const write = (path: string, mtime: number): string => {
        writeFileSync(path, "x");
        utimesSync(path, new Date(mtime), new Date(mtime));
        return path;
      };
      write(join(sourceRoot, "🗿️artifacts", "🦀️.rs"), 1_000_000);
      for (const name of fixture.freshness.walk.generatedOwnerFiles) write(join(sourceRoot, name), 9_000_000);
      expect(newestComponentSourceMtime(sourceRoot)?.mtimeMs).toBe(1_000_000);

      const nested = write(join(sourceRoot, "🗿️artifacts", "🧬️schema", fixture.freshness.walk.generatedOwnerFiles[0]!), 3_000_000);
      const newest = newestComponentSourceMtime(sourceRoot);
      expect(newest?.mtimeMs).toBe(3_000_000);
      expect(newest?.path).toBe(nested);

      // 🧯️ The exact live symptom: staged at 14:21, described at 15:42, no source touched — `fresh`.
      expect(stagedModuleVerdict({ pluginId: "stdio", role: "plugin", activationTracked: false, stagedAtMs: 4_000_000, newestSourceMs: newestComponentSourceMtime(sourceRoot)?.mtimeMs }).kind).toBe("fresh");
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  /** @emoji 🛂️ Pinned to the `describe` target's own file names, read from the module that declares them
   * (`🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts`) as SOURCE TEXT — importing it would drag the cargo
   * tool-chain into a filesystem suite, and this module keeps a node-builtin-only import surface because
   * every dev server's Vite config bundles it. */
  it("names exactly the two files the describe target declares as its outputs", () => {
    expect([...GENERATED_COMPONENT_OWNER_FILES].sort()).toEqual([...fixture.freshness.walk.generatedOwnerFiles].sort());
    const source = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts"), "utf8");
    const declared = [...source.matchAll(/export const DESCRIPTOR_(?:PACK|JSON)_FILENAME = "([^"]+)";/g)].map((match) => match[1]!);
    expect(declared.length, "DESCRIPTOR_PACK_FILENAME / DESCRIPTOR_JSON_FILENAME moved — re-derive GENERATED_COMPONENT_OWNER_FILES").toBe(2);
    expect(declared.sort()).toEqual([...GENERATED_COMPONENT_OWNER_FILES].sort());
  });

  it("hashes plugin-owner sources stably and ignores declared output directories", () => {
    const sandbox = mkdtempSync(join(tmpdir(), "semio-staging-hash-"));
    try {
      const sourceRoot = join(sandbox, "source");
      mkdirSync(join(sourceRoot, "src"), { recursive: true });
      writeFileSync(join(sourceRoot, "src", "main.rs"), "fn main() {}");
      for (const directory of fixture.freshness.walk.outputDirectories) {
        mkdirSync(join(sourceRoot, directory), { recursive: true });
        writeFileSync(join(sourceRoot, directory, "noise.bin"), "noise");
      }
      const first = componentSourceContentHash(sourceRoot);
      expect(first).toMatch(/^[a-f0-9]{64}$/);
      writeFileSync(join(sourceRoot, "dist", "more.bin"), "more");
      expect(componentSourceContentHash(sourceRoot)).toBe(first);
      writeFileSync(join(sourceRoot, "src", "extra.rs"), "fn extra() {}");
      expect(componentSourceContentHash(sourceRoot)).not.toBe(first);
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  it("stops at its declared entry bound instead of walking an unbounded tree", () => {
    const sandbox = mkdtempSync(join(tmpdir(), "semio-staging-bound-"));
    try {
      mkdirSync(sandbox, { recursive: true });
      for (let index = 0; index < 8; index += 1) {
        const file = join(sandbox, `f${index}.rs`);
        writeFileSync(file, "x");
        utimesSync(file, new Date(1_000_000 + index * 1000), new Date(1_000_000 + index * 1000));
      }
      expect(newestComponentSourceMtime(sandbox, 3)!.mtimeMs).toBeLessThan(newestComponentSourceMtime(sandbox)!.mtimeMs);
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });
});
