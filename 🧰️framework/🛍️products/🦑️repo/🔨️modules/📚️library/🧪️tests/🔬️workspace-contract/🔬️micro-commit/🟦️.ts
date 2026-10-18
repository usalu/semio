import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { gitSpawnEnv } from "../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";

describe("micro-commit", () => {
  function gitDirFor(root: string): string {
    const rel = spawnSync("git", ["rev-parse", "--git-dir"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).stdout?.trim() ?? ".git";
    return rel.startsWith("/") ? rel : join(root, rel);
  }

  test("safeGitEnv replaces an unusable configured global Git config", async () => {
    const { safeGitEnv } = await import("../../../🏃️process/🌿️environment/🌳️git/🟦️.ts");
    const { devNull } = await import("node:os");
    const env = safeGitEnv({ GIT_CONFIG_GLOBAL: join(process.cwd(), ".missing-global-gitconfig") });
    expect(env.GIT_CONFIG_GLOBAL).toBe(devNull);
  });

  test("branchValidationError distinguishes lookup failure, detached HEAD, and a wrong branch", async () => {
    const { branchValidationError } = await import("../../../🎛️commands/📝️commit/👤️contributor/🟦️.ts");
    expect(branchValidationError("micro-commit", { ok: false, error: "permission denied" })).toBe("micro-commit: cannot read current branch: permission denied");
    expect(branchValidationError("micro-commit", { ok: true, name: "" })).toContain("detached HEAD");
    expect(branchValidationError("micro-commit", { ok: true, name: "🐙️ueli/feature" })).toContain('current branch "🐙️ueli/feature"');
    expect(branchValidationError("micro-commit", { ok: true, name: "🐙️ueli/⛳️wip" })).toBeNull();
    expect(branchValidationError("micro-commit", { ok: true, name: "🐙️ueli/⛳️wip" })).toBeNull();
  });

  test("extractCounterFromSubject reads formatted subject lines", async () => {
    const { extractCounterFromSubject } = await import("../../../🎛️commands/📝️commit/🔢️counter/🟦️.ts");
    expect(extractCounterFromSubject("🧑️ueli🎆️26🌙️06☀️02🚩️009")).toEqual({ nnn: 9, line1Base: "🧑️ueli🎆️26🌙️06☀️02" });
    expect(extractCounterFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️397")).toEqual({ nnn: 397, line1Base: "🐙️ueli🎆️26🌙️06☀️04" });
    expect(extractCounterFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️396")).toEqual({ nnn: 396, line1Base: "🐙️ueli🎆️26🌙️06☀️04" });
    expect(extractCounterFromSubject("33")).toBeNull();
    expect(extractCounterFromSubject("Merge branch foo")).toBeNull();
  });

  test("extractNumericCounterFromSubject reads GitKraken numeric subjects", async () => {
    const { extractNumericCounterFromSubject } = await import("../../../🎛️commands/📝️commit/🔢️counter/🟦️.ts");
    expect(extractNumericCounterFromSubject("299")).toBe(299);
    expect(extractNumericCounterFromSubject("001")).toBe(1);
    expect(extractNumericCounterFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️151")).toBeNull();
  });

  test("line1BaseFromBundleTag reads WIP epoch from squash tag", async () => {
    const { line1BaseFromBundleTag } = await import("../../../🎛️commands/📝️commit/🔢️counter/🟦️.ts");
    expect(line1BaseFromBundleTag("🐙️ueli🎆️26🌙️06☀️04🚩️")).toBe("🐙️ueli🎆️26🌙️06☀️04");
    expect(line1BaseFromBundleTag("🐙️ueli🎆️26🌙️06☀️04🚩️")).toBe("🐙️ueli🎆️26🌙️06☀️04");
    expect(line1BaseFromBundleTag("🐙️ueli🎆️26🌙️06☀️04🚩️151")).toBeNull();
  });

  test("bumpCounterFromHistory uses max across formatted commits", async () => {
    const { bumpCounterFromHistory } = await import("../../../🎛️commands/📝️commit/🔢️counter/🟦️.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli", email: "u@example.com" };
    const subjects = ["🐙️ueli🎆️26🌙️06☀️02🚩️033", "🐙️ueli🎆️26🌙️06☀️02🚩️032", "unrelated"];
    const bumped = bumpCounterFromHistory(subjects, contributor, new Date("2026-06-02T12:00:00"));
    expect(bumped.line1Base).toBe("🐙️ueli🎆️26🌙️06☀️02");
    expect(bumped.nnn).toBe("034");
    const fresh = bumpCounterFromHistory(["unrelated"], contributor, new Date("2026-06-02T12:00:00"));
    expect(fresh.nnn).toBe("001");
  });

  test("bumpCounterFromHistory preserves selector-free history with canonical output", async () => {
    const { bumpCounterFromHistory } = await import("../../../🎛️commands/📝️commit/🔢️counter/🟦️.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli", email: "u@example.com" };
    const bumped = bumpCounterFromHistory(["🦢️other🎆️26🌙️07☀️31🚩️999", "🐙️ueli🎆️26🌙️06☀️04🚩️397", "🐙️ueli🎆️26🌙️06☀️04🚩️396"], contributor, new Date("2026-07-31T12:00:00"));
    expect(bumped).toEqual({ line1Base: "🐙️ueli🎆️26🌙️06☀️04", nnn: "398" });
    expect(() => bumpCounterFromHistory(["🐙️ueli🎆️26🌙️6☀️04🚩️397"], contributor, new Date("2026-07-31T12:00:00"))).toThrow("refusing to reset counter to 001");
  });

  test("bumpCounterFromHistory continues numeric GitKraken subjects with WIP epoch", async () => {
    const { bumpCounterFromHistory } = await import("../../../🎛️commands/📝️commit/🔢️counter/🟦️.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli", email: "u@example.com" };
    const subjects = ["299", "298", "297"];
    const bumped = bumpCounterFromHistory(subjects, contributor, new Date("2026-07-17T12:00:00"), "🐙️ueli🎆️26🌙️06☀️04");
    expect(bumped.line1Base).toBe("🐙️ueli🎆️26🌙️06☀️04");
    expect(bumped.nnn).toBe("300");
  });

  test("normalizeBulletLines strips uloc block lines", async () => {
    const { normalizeBulletLines } = await import("../../../🎛️commands/📝️commit/🔸️bullets/🟦️.ts");
    const bullets = normalizeBulletLines("🎆️Summary\n📊️metric📃uloc💯️65k➕️1✏️1🟰️2\n📊️metric🟦️typescript📃uloc💯️65k➕️1✏️1🟰️2\n🐛️Fix bug");
    expect(bullets).toEqual(["🎆️Summary", "🐛️Fix bug"]);
  });

  test("bulletEmojiValidationError rejects fireworks emoji on bullets", async () => {
    const { bulletLeadEmoji, bulletEmojiValidationError } = await import("../../../🎛️commands/📝️commit/🔸️bullets/🟦️.ts");
    expect(bulletLeadEmoji("🎆️Drop stacked intro")).toBe("🎆️");
    expect(bulletEmojiValidationError(["🎆️All bullets wrongly use fireworks"])).toContain("🎆️");
    expect(bulletEmojiValidationError(["🐛️Fix real bug"])).toBeNull();
    expect(bulletEmojiValidationError(["🧬️Tune WASM flush timing"])).toBeNull();
    expect(bulletEmojiValidationError(["📊️uloc block"])).toContain("📊️");
  });

  test("normalizeBulletLines enforces compact {emoji}{description} format", async () => {
    const { normalizeBulletLines, formatMicroCommitBulletLine } = await import("../../../🎛️commands/📝️commit/🔸️bullets/🟦️.ts");
    expect(formatMicroCommitBulletLine("- 🐛️ Fix PDF")).toBe("🐛️Fix PDF");
    expect(normalizeBulletLines("🐛️ Fix PDF\n- 🖼️ Tweak UI")).toEqual(["🐛️Fix PDF", "🖼️Tweak UI"]);
    expect(normalizeBulletLines(Array.from({ length: 10 }, (_, i) => `🎆️item ${i}`).join("\n"))).toHaveLength(8);
  });

  test("buildMicroCommitMessage separates GitKraken summary and description", async () => {
    const { buildMicroCommitMessage } = await import("../../../🎛️commands/📝️commit/📄️prepared-message/🟦️.ts");
    const root = mkdtempSync(join(tmpdir(), "semio-micro-commit-message-"));
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      const msg = buildMicroCommitMessage(root, contributor, ["🎆️LLM-authored change summary"], {
        countRepoByLanguage: () => ({ TypeScript: 1000, Rust: 500 }),
      });
      const lines = msg.trimEnd().split("\n");
      expect(lines[0]).toMatch(/🚩️\d{3}$/);
      expect(lines[1]).toMatch(/^🎆️/);
      expect(lines.some((l) => l.includes("LLM-authored"))).toBe(true);
      expect(lines.at(-1)).toMatch(/^Signed-off-by: /);
      expect(lines.at(-2)).toBe("");
      const { MICRO_COMMIT_ULOC_HEADER: ulocHeader } = await import("../../../🎛️commands/📝️commit/📊️metrics/🧮️summary/🟦️.ts");
      const metricsIdx = lines.findIndex((l) => l.startsWith(ulocHeader));
      expect(metricsIdx).toBeGreaterThan(2);
      expect(lines[metricsIdx - 1]).toBe("");
      expect(lines[metricsIdx]?.startsWith(ulocHeader)).toBe(true);
      expect(lines[metricsIdx + 1]).toMatch(/^📊️metric/);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("buildMicroCommitMessage rejects output without the required uloc footer", async () => {
    const { buildMicroCommitMessage } = await import("../../../🎛️commands/📝️commit/📄️prepared-message/🟦️.ts");
    const { mkdtempSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-empty-uloc-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      expect(() =>
        buildMicroCommitMessage(root, contributor, ["🐛️Reject incomplete commit messages"], {
          countRepoByLanguage: () => ({}),
        }),
      ).toThrow("required 📊️metric footer");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("formatMicroCommitMetricsLines uses compact loc and delta counts", async () => {
    const { formatMicroCommitMetricLine, formatMicroCommitMetricsLines } = await import("../../../🎛️commands/📝️commit/📊️metrics/🧮️summary/🟦️.ts"), { formatMetricLocCount, formatMetricSizeCount, formatMetricRatio, COMMIT_METRIC_HEADER } = await import("../../../🎛️commands/📝️commit/📊️metrics/🎨️format/🟦️.ts");
    expect(COMMIT_METRIC_HEADER).toBe("📊️metric");
    expect(formatMetricSizeCount(1024)).toBe("1.02KB");
    expect(formatMetricSizeCount(10_400_000_000)).toBe("10.4GB");
    expect(formatMetricLocCount(200_000)).toBe("200k");
    expect(formatMetricLocCount(1_300_000)).toBe("1.3M");
    expect(formatMetricLocCount(769_000)).toBe("769k");
    expect(formatMetricLocCount(422_377)).toBe("422k");
    expect(formatMetricLocCount(52_759)).toBe("52.8k");
    expect(formatMetricLocCount(500)).toBe("500");
    expect(formatMetricRatio(0.001)).toBe("0.1");
    expect(formatMetricRatio(0.0000854)).toBe("0.009");
    expect(formatMetricRatio(0.0305)).toBe("3.05");
    expect(formatMetricRatio(0.00264)).toBe("0.264");
    expect(formatMetricRatio(0.10061)).toBe("10.1");
    expect(formatMetricRatio(1.21001)).toBe("121");
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚️", code: 2000, edited: 0, added: 0, removed: 0 })).toBe("📊️metric🐚️shell📃uloc💯️2k");
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚️", code: 2000, edited: 2, added: 2, removed: 0 })).toBe("📊️metric🐚️shell📃uloc💯️2k📈️2➗️0.1➕️2✏️2🟰️4");
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚️", code: 2000, edited: 2, added: 0, removed: 2 })).toBe("📊️metric🐚️shell📃uloc💯️2k📉️2➗️0.1✏️2➖️2🟰️4");
    const lines = formatMicroCommitMetricsLines([{ lang: "Rust", emoji: "🦀️", code: 200_000, edited: 2220, added: 2000, removed: 500 }]);
    expect(lines[0]).toBe("📊️metric📃uloc💯️200k📈️1.5k➗️0.756➕️2k✏️2.22k➖️500🟰️4.72k");
    expect(lines[1]).toBe("📊️metric🦀️rust📃uloc💯️200k📈️1.5k➗️0.756➕️2k✏️2.22k➖️500🟰️4.72k");
  });

  test("formatMicroCommitMetricsLines totals all languages on the first row", async () => {
    const { formatMicroCommitMetricsLines } = await import("../../../🎛️commands/📝️commit/📊️metrics/🧮️summary/🟦️.ts");
    const lines = formatMicroCommitMetricsLines([
      { lang: "TypeScript", emoji: "🟦️", code: 3000, edited: 10, added: 8, removed: 0 },
      { lang: "Markdown", emoji: "📝️", code: 44, edited: 0, added: 0, removed: 0 },
    ]);
    expect(lines[0]).toBe("📊️metric📃uloc💯️3.04k📈️8➗️0.264➕️8✏️10🟰️18");
    expect(lines[1]).toBe("📊️metric🟦️typescript📃uloc💯️3k📈️8➗️0.267➕️8✏️10🟰️18");
    expect(lines[2]).toBe("📊️metric📝️markdown📃uloc💯️44");
  });

  test("buildMicroCommitMetrics merges uloc and git numstat by language", async () => {
    const { buildMicroCommitMetrics } = await import("../../../🎛️commands/📝️commit/📊️metrics/🧮️summary/🟦️.ts");
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
    const { isUlocCachePlausible } = await import("../../../🎛️commands/📝️commit/📊️metrics/💾️scan/🟦️.ts"), { gitRepoRoot } = await import("../../../🎛️commands/📝️commit/🌳️git/🟦️.ts");
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
    const { shouldSkipPathForUloc } = await import("../../../🎛️commands/📝️commit/📊️metrics/🛤️path-rules/🟦️.ts");
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
    const { countJsonKeys } = await import("../../../🎛️commands/📝️commit/📊️metrics/🔢️uloc/🟦️.ts");
    expect(countJsonKeys('{"a":1,"b":{"c":2}}')).toBe(3);
  });

  test("JSON ULOC deltas use recursive keys rather than physical Git lines", async () => {
    const { accumulateUlocDeltasFromPaths } = await import("../../../🎛️commands/📝️commit/📊️metrics/🔀️deltas/🟦️.ts"), { countUnifiedLocForFile } = await import("../../../🎛️commands/📝️commit/📊️metrics/🔢️uloc/🟦️.ts");
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
    const { appendGitDeltaSuffix, formatBundleUlocSuffix } = await import("../../../🎛️commands/📝️commit/📊️metrics/🎨️format/🟦️.ts");
    expect(appendGitDeltaSuffix("🟦️65k", { added: 700, edited: 200, removed: 10 })).toBe("🟦️65k➕️700✏️200➖️10🟰️910");
    expect(formatBundleUlocSuffix({ added: 700, edited: 200, removed: 10 }, 65_000)).toBe("📊️metric📃uloc💯️65k📈️690➗️1.07➕️700✏️200➖️10🟰️910");
  });

  test("splitGitNumstatDelta separates replaced lines from net added and removed", async () => {
    const { splitGitNumstatDelta } = await import("../../../🎛️commands/📝️commit/📊️metrics/🔀️deltas/🟦️.ts");
    expect(splitGitNumstatDelta(4, 2)).toEqual({ edited: 2, added: 2, removed: 0 });
    expect(splitGitNumstatDelta(2, 4)).toEqual({ edited: 2, added: 0, removed: 2 });
    expect(splitGitNumstatDelta(5, 5)).toEqual({ edited: 5, added: 0, removed: 0 });
    expect(splitGitNumstatDelta(10, 0)).toEqual({ edited: 0, added: 10, removed: 0 });
    expect(splitGitNumstatDelta(0, 7)).toEqual({ edited: 0, added: 0, removed: 7 });
  });

  test("countUnifiedLocForFile uses physical lines for code and keys for json", async () => {
    const { countUnifiedLocForFile } = await import("../../../🎛️commands/📝️commit/📊️metrics/🔢️uloc/🟦️.ts");
    expect(countUnifiedLocForFile("x.rs", "// c\nfn main() {}\n")).toBe(3);
    expect(countUnifiedLocForFile("x.json", '{"k":1}')).toBe(1);
  });

  test("classifyPathForMetrics maps TeX ecosystem extensions", async () => {
    const { classifyPathForMetrics, langMetricsEmoji } = await import("../../../🎛️commands/📝️commit/📊️metrics/🛤️path-rules/🟦️.ts");
    expect(classifyPathForMetrics("mit-bestand/bericht/zwischenbericht/🖋️zwischenbericht.tex")).toBe("TeX");
    expect(classifyPathForMetrics("print/semio.sty")).toBe("TeX");
    expect(classifyPathForMetrics("print/🖋️semio.cls")).toBe("TeX");
    expect(classifyPathForMetrics("report/📚️references.bib")).toBe("TeX");
    expect(classifyPathForMetrics("doc/sample.ltx")).toBe("TeX");
    expect(langMetricsEmoji("TeX")).toBe("📐️");
  });

  test("uncoveredStagedAreas flags missing cursor-plans and product coverage", async () => {
    const { uncoveredStagedAreas } = await import("../../../🎛️commands/📝️commit/🔸️bullets/🟦️.ts");
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

  test("installMicroCommitGitHooks writes portable hooks and bun pin", async () => {
    const { installMicroCommitGitHooks, renderMicroCommitGitHook } = await import("../../../🎛️commands/📝️commit/🪝️hooks/🟦️.ts");
    const { mkdtempSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-"));
    try {
      const init = spawnSync("git", ["init"], { cwd: root, encoding: "utf8" });
      expect(init.status).toBe(0);
      installMicroCommitGitHooks(root);
      const hook = readFileSync(join(root, ".git/hooks/post-commit"), "utf8");
      expect(hook).toContain("compose_micro_commit_wipe");
      expect(hook).not.toContain("\r");
      expect(existsSync(join(root, ".🧬semio/🦑️repo/compose-micro-commit-bun"))).toBe(true);
      expect(renderMicroCommitGitHook("post-commit")).toContain("#!/usr/bin/env sh");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("prepare-commit-msg lets a manual commit complete when template refresh fails", async () => {
    const { renderMicroCommitGitHook } = await import("../../../🎛️commands/📝️commit/🪝️hooks/🟦️.ts");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-hook-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      expect(spawnSync("git", ["config", "user.email", "hook@example.com"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      expect(spawnSync("git", ["config", "user.name", "Hook"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      expect(spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      const hook = join(root, ".git", "hooks", "prepare-commit-msg");
      const brokenBun = join(root, "broken-bun");
      writeFileSync(hook, renderMicroCommitGitHook("prepare-commit-msg"));
      writeFileSync(brokenBun, "#!/usr/bin/env sh\nexit 1\n");
      chmodSync(hook, 0o755);
      chmodSync(brokenBun, 0o755);
      writeFileSync(join(root, ".git", "compose-micro-commit-active"), "1\n");
      writeFileSync(join(root, "tracked.txt"), "tracked\n");
      expect(spawnSync("git", ["add", "tracked.txt"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      const commit = spawnSync("git", ["commit", "-m", "Manual Commit"], { cwd: root, encoding: "utf8", env: { ...gitSpawnEnv(), COMPOSE_BUN: brokenBun } });
      expect(commit.status, commit.stderr).toBe(0);
      expect(spawnSync("git", ["log", "-1", "--format=%s"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).stdout?.trim()).toBe("Manual Commit");
      expect(existsSync(join(root, ".git", "compose-micro-commit-active"))).toBe(false);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("handlePrepareCommitMsg inactive does not clear commit message file", async () => {
    const { handlePrepareCommitMsg } = await import("../../../🎛️commands/📝️commit/📄️prepared-message/🟦️.ts");
    const { mkdtempSync, writeFileSync, readFileSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      const msgFile = join(root, ".git", "COMMIT_EDITMSG");
      writeFileSync(msgFile, "🐙️ueli manual subject\n", "utf8");
      handlePrepareCommitMsg(root, msgFile, "template");
      expect(readFileSync(msgFile, "utf8")).toContain("manual subject");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("wipeAfterCommit clears all GK templates and prepare state", async () => {
    const { wipeAfterCommit, writeMicroCommitTemplates } = await import("../../../🎛️commands/📝️commit/📄️prepared-message/🟦️.ts");
    const { mkdtempSync, writeFileSync, readFileSync, rmSync, existsSync, readdirSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-wipe-"));
    const prevRoot = process.env.REPO_ROOT;
    const prevGitDir = process.env.GIT_DIR;
    const prevWorkTree = process.env.GIT_WORK_TREE;
    delete process.env.REPO_ROOT;
    delete process.env.GIT_DIR;
    delete process.env.GIT_WORK_TREE;
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      spawnSync("git", ["config", "user.email", "u@example.com"], { cwd: root, env: gitSpawnEnv() });
      spawnSync("git", ["config", "user.name", "U"], { cwd: root, env: gitSpawnEnv() });
      const msg = "🐙️ueli🎆️26🌙️08☀️17🚩️001\n\n🎆️test reset\n";
      const gitDir = gitDirFor(root);
      writeMicroCommitTemplates(root, msg);
      writeFileSync(join(gitDir, "gkcommittemplate-099.txt"), "stale numbered", "utf8");
      wipeAfterCommit(root);
      const gkTemplate = join(gitDir, "gkcommittemplate.txt");
      expect(existsSync(gkTemplate)).toBe(true);
      expect(readFileSync(gkTemplate, "utf8")).toBe("");
      const gkLeft = readdirSync(gitDir).filter((n) => n.startsWith("gkcommittemplate"));
      expect(gkLeft).toEqual(["gkcommittemplate.txt"]);
      expect(existsSync(join(gitDir, "compose-micro-commit-active"))).toBe(false);
      expect(readFileSync(join(gitDir, "COMMIT_EDITMSG"), "utf8")).toBe("");
    } finally {
      if (prevRoot === undefined) delete process.env.REPO_ROOT;
      else process.env.REPO_ROOT = prevRoot;
      if (prevGitDir === undefined) delete process.env.GIT_DIR;
      else process.env.GIT_DIR = prevGitDir;
      if (prevWorkTree === undefined) delete process.env.GIT_WORK_TREE;
      else process.env.GIT_WORK_TREE = prevWorkTree;
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("writeMicroCommitTemplates uses single gkcommittemplate.txt", async () => {
    const { writeMicroCommitTemplates } = await import("../../../🎛️commands/📝️commit/📄️prepared-message/🟦️.ts");
    const { mkdtempSync, readdirSync, rmSync, readFileSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-tpl-"));
    const prevRoot = process.env.REPO_ROOT;
    const prevGitDir = process.env.GIT_DIR;
    const prevWorkTree = process.env.GIT_WORK_TREE;
    delete process.env.REPO_ROOT;
    delete process.env.GIT_DIR;
    delete process.env.GIT_WORK_TREE;
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      const msg = "🐙️ueli🎆️26🌙️08☀️17🚩️001\n\n🎆️bullet\n";
      const gitDir = gitDirFor(root);
      writeMicroCommitTemplates(root, msg);
      const gk = readdirSync(gitDir).filter((n) => n.startsWith("gkcommittemplate"));
      expect(gk).toEqual(["gkcommittemplate.txt"]);
      expect(readFileSync(join(gitDir, "gkcommittemplate.txt"), "utf8")).toContain("🚩️");
    } finally {
      if (prevRoot === undefined) delete process.env.REPO_ROOT;
      else process.env.REPO_ROOT = prevRoot;
      if (prevGitDir === undefined) delete process.env.GIT_DIR;
      else process.env.GIT_DIR = prevGitDir;
      if (prevWorkTree === undefined) delete process.env.GIT_WORK_TREE;
      else process.env.GIT_WORK_TREE = prevWorkTree;
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("shouldRefreshPreparedCommitMessage keeps user edits", async () => {
    const { digestMicroCommitMessage, shouldRefreshPreparedCommitMessage } = await import("../../../🎛️commands/📝️commit/📄️prepared-message/🟦️.ts");
    const prepared = "line1\nline2\n";
    const digest = digestMicroCommitMessage(prepared);
    expect(shouldRefreshPreparedCommitMessage(prepared, digest)).toBe(true);
    expect(shouldRefreshPreparedCommitMessage("", digest)).toBe(true);
    expect(shouldRefreshPreparedCommitMessage(`${prepared}\nmy edit`, digest)).toBe(false);
  });
});
