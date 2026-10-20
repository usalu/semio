/** 🔹️ The buildmicrocommitmetrics merges uloc git tests of the micro-commit suite. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../../../🗂️workspaces/🟦️.ts";
import { gitSpawnEnv } from "../../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import { contractRoot } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🟠 Registers the buildmicrocommitmetrics merges uloc git tests of the micro-commit suite. */
export function registerBuildmicrocommitmetricsMergesUlocGitTests(): void {
  test("buildMicroCommitMetrics merges uloc and git numstat by language", async () => {
    const { buildMicroCommitMetrics } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🧮️summary/🟦️.ts");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-metrics-"));
    try {
      for (const args of [["init"], ["config", "user.email", "t@e.com"], ["config", "user.name", "T"], ["config", "commit.gpgsign", "false"]]) spawnSync("git", args, { cwd: root, env: gitSpawnEnv() });
      mkdirSync(join(root, "repo", "js"), { recursive: true });
      writeFileSync(join(root, "repo/js/a.ts"), "a\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root, env: gitSpawnEnv() });
      spawnSync("git", ["commit", "-m", "seed"], { cwd: root, env: gitSpawnEnv() });
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\nc\n", "utf8");
      spawnSync("git", ["add", "repo/js/a.ts"], { cwd: root, env: gitSpawnEnv() });
      const metrics = buildMicroCommitMetrics(root, { countRepoByLanguage: () => ({ Rust: 100, TypeScript: 50, JSON: 20 }) });
      expect(metrics.some((m) => m.lang === "Rust" && m.code === 100)).toBe(true);
      expect(metrics.some((m) => m.lang === "TypeScript" && m.code === 50 && m.added > 0)).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);
  test("isUlocCachePlausible rejects partial caches", async () => {
    const { isUlocCachePlausible } = await import("../../../../🎛️commands/📝️commit/📊️metrics/💾️scan/🟦️.ts"), { gitRepoRoot } = await import("../../../../🎛️commands/📝️commit/🌳️git/🟦️.ts");
    const prevRoot = process.env.REPO_ROOT;
    const prevGitDir = process.env.GIT_DIR;
    const prevWorkTree = process.env.GIT_WORK_TREE;
    delete process.env.REPO_ROOT;
    delete process.env.GIT_DIR;
    delete process.env.GIT_WORK_TREE;
    const root = gitRepoRoot(contractRoot);
    const tracked = spawnSync("git", ["ls-files"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).stdout?.split("\n").filter(Boolean).length ?? 0;
    try {
      expect(isUlocCachePlausible(root, {})).toBe(false);
      if (tracked <= 1000) return;
      expect(isUlocCachePlausible(root, { TypeScript: 3000, JavaScript: 78, Markdown: 44, JSON: 43 })).toBe(false);
      expect(
        isUlocCachePlausible(root, {
          JSON: 400_000,
          TypeScript: 150_000,
          Go: 90_000,
          Rust: 44_000,
          Markdown: 10_000,
          Python: 30_000,
        }),
      ).toBe(true);
    } finally {
      if (prevRoot === undefined) delete process.env.REPO_ROOT;
      else process.env.REPO_ROOT = prevRoot;
      if (prevGitDir === undefined) delete process.env.GIT_DIR;
      else process.env.GIT_DIR = prevGitDir;
      if (prevWorkTree === undefined) delete process.env.GIT_WORK_TREE;
      else process.env.GIT_WORK_TREE = prevWorkTree;
    }
  });
  test("shouldSkipPathForUloc skips dot paths license templates and .🧬semio", async () => {
    const { shouldSkipPathForUloc } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🛤️path-rules/🟦️.ts");
    const root = getWorkspaceRoot();
    expect(shouldSkipPathForUloc(root, ".cursor/plans/foo.plan.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, ".agents/skills/micro-commit/SKILL.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, "compose/client/ui/LICENSE.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, "repo/AGENTS.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, "repo/CHANGELOG.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, ".🧬semio/🦑️repo/⚡️cache/x")).toBe(true);
    expect(shouldSkipPathForUloc(root, "framework/README.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, "puzzle/3d/src/foo.ts")).toBe(false);
  });
  test("countJsonKeys counts nested object keys", async () => {
    const { countJsonKeys } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🔢️uloc/🟦️.ts");
    expect(countJsonKeys('{"a":1,"b":{"c":2}}')).toBe(3);
  });
  test("JSON ULOC deltas use recursive keys rather than physical Git lines", async () => {
    const { accumulateUlocDeltasFromPaths } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🔀️deltas/🟦️.ts"), { countUnifiedLocForFile } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🔢️uloc/🟦️.ts");
    const { parse: parseJsonc } = await import("jsonc-parser");
    const root = mkdtempSync(join(tmpdir(), "semio-json-uloc-"));
    const before = JSON.stringify({ stable: true });
    const after = JSON.stringify({ stable: true, entries: Object.fromEntries(Array.from({ length: 150 }, (_, index) => [`key-${index}`, index])) }, null, 2);
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      expect(spawnSync("git", ["config", "user.email", "metrics@example.com"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      expect(spawnSync("git", ["config", "user.name", "Metrics"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      expect(spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      writeFileSync(join(root, "metrics.json"), before);
      expect(spawnSync("git", ["add", "metrics.json"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      expect(spawnSync("git", ["commit", "-m", "base"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      writeFileSync(join(root, "metrics.json"), after);
      expect(spawnSync("git", ["add", "metrics.json"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      const numstat = spawnSync("git", ["diff", "--cached", "--numstat", "--", "metrics.json"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).stdout?.trim().split("\t") ?? [];
      const added = Number(numstat[0]);
      const total = countUnifiedLocForFile("metrics.json", after);
      expect(parseJsonc(after)).toEqual(JSON.parse(after));
      expect(added).toBeGreaterThan(total);
      const delta = accumulateUlocDeltasFromPaths(root, [{ path: "metrics.json", added, removed: Number(numstat[1]) }], "HEAD", ":0").get("JSON");
      expect(delta).toEqual({ added: 151, edited: 0, removed: 0 });
      expect(delta!.added).toBeLessThanOrEqual(total);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("appendGitDeltaSuffix formats legacy delta-only suffixes", async () => {
    const { appendGitDeltaSuffix, formatBundleUlocSuffix } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🎨️format/🟦️.ts");
    expect(appendGitDeltaSuffix("🟦️65k", { added: 700, edited: 200, removed: 10 })).toBe("🟦️65k➕️700✏️200➖️10🟰️910");
    expect(formatBundleUlocSuffix({ added: 700, edited: 200, removed: 10 }, 65_000)).toBe("📊️metric📃uloc💯️65k📈️690➗️1.07➕️700✏️200➖️10🟰️910");
  });
  test("splitGitNumstatDelta separates replaced lines from net added and removed", async () => {
    const { splitGitNumstatDelta } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🔀️deltas/🟦️.ts");
    expect(splitGitNumstatDelta(4, 2)).toEqual({ edited: 2, added: 2, removed: 0 });
    expect(splitGitNumstatDelta(2, 4)).toEqual({ edited: 2, added: 0, removed: 2 });
    expect(splitGitNumstatDelta(5, 5)).toEqual({ edited: 5, added: 0, removed: 0 });
    expect(splitGitNumstatDelta(10, 0)).toEqual({ edited: 0, added: 10, removed: 0 });
    expect(splitGitNumstatDelta(0, 7)).toEqual({ edited: 0, added: 0, removed: 7 });
  });
  test("countUnifiedLocForFile uses physical lines for code and keys for json", async () => {
    const { countUnifiedLocForFile } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🔢️uloc/🟦️.ts");
    expect(countUnifiedLocForFile("x.rs", "// c\nfn main() {}\n")).toBe(3);
    expect(countUnifiedLocForFile("x.json", '{"k":1}')).toBe(1);
  });
  test("classifyPathForMetrics maps TeX ecosystem extensions", async () => {
    const { classifyPathForMetrics, langMetricsEmoji } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🛤️path-rules/🟦️.ts");
    expect(classifyPathForMetrics("mit-bestand/bericht/zwischenbericht/🖋️zwischenbericht.tex")).toBe("TeX");
    expect(classifyPathForMetrics("print/semio.sty")).toBe("TeX");
    expect(classifyPathForMetrics("print/🖋️semio.cls")).toBe("TeX");
    expect(classifyPathForMetrics("report/📚️references.bib")).toBe("TeX");
    expect(classifyPathForMetrics("doc/sample.ltx")).toBe("TeX");
    expect(langMetricsEmoji("TeX")).toBe("📐️");
  });
  test("uncoveredStagedAreas flags missing cursor-plans and product coverage", async () => {
    const { uncoveredStagedAreas } = await import("../../../../🎛️commands/📝️commit/🔸️bullets/🟦️.ts");
    const staged = [".cursor/plans/brush_fix_cfd8a931.plan.md", "framework/product/playground/renderer/react/index.tsx"];
    expect(uncoveredStagedAreas(["🫡️Only micro-commit skill wording"], staged)).toContain(".cursor/plans");
    expect(uncoveredStagedAreas(["🫡️Only micro-commit skill wording"], staged)).toContain("product");
    const ok = uncoveredStagedAreas(["📋️Plan brush edge resurrection guard and sync", "🖌️Playground renderer restores brush placement after structural deletes"], staged);
    expect(ok).toEqual([]);
  });
  test("validateBulletsAgainstStaged rejects bullets that ignore staged product code", async () => {
    const root = getWorkspaceRoot();
    const stagedHasPresentation = spawnSync("git", ["diff", "--cached", "--name-only"], { cwd: root, encoding: "utf8" }).stdout?.includes("presentation/");
    if (!stagedHasPresentation) return;
    const prev = process.env.REPO_ROOT;
    process.env.REPO_ROOT = root;
    const stdin = ["🫡️Only micro-commit skill docs"].join("\n");
    const r = spawnSync(process.execPath, ["./📜️script.ts", "micro-commit", "prepare"], {
      cwd: root,
      input: stdin,
      encoding: "utf8",
      timeout: 120_000,
    });
    if (prev === undefined) delete process.env.REPO_ROOT;
    else process.env.REPO_ROOT = prev;
    expect(r.status).not.toBe(0);
  }, 120_000);
}
