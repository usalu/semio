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
  UNWATCHED_COMPONENT_SOURCE_DIRECTORIES,
  newestComponentSourceMtime,
  pluginModulesRoot,
  pluginModulesRootIn,
  stagedModuleMtime,
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
    readonly walk: { readonly outputDirectories: readonly string[] };
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
      expect(stagedModuleVerdict({ pluginId: "procedural", role: "plugin", activationTracked: false, stagedAtMs: stagedModuleMtime(stagedDirectory), newestSourceMs: newestComponentSourceMtime(sourceRoot)?.mtimeMs }).kind).toBe("source-newer");
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
