import { testLevelBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { DAEMON_BUDGET_MS, ORCHESTRATOR_BUDGET_MS, daemonBudgetMs, orchestratorBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { parseArgs } from "node:util";
import { budgetTimeoutHint, daemonBudgetOpts, orchestratorBudgetOpts, runCmd, runCmdStatus } from "../../../🏃️process/🟦️.ts";
import { capturedTestFailureDiagnostics } from "../../../🧪️testing/🎛️repository-command/🟦️.ts";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { devToolingEnv } from "../../../🏃️process/🌿️environment/🟦️.ts";
import { goLevelTestArgs, vitestLevelArgs } from "../../../🧪️testing/🎚️levels/🟦️.ts";
import { nextestArtifactLocation } from "../../../🏃️process/🦀️cargo/🔣️policy/🟦️.ts";
import { resolveCargoPackageName, resolveCargoPackageNames } from "../../../🏃️process/🦀️cargo/🔍️package-names/🟦️.ts";
import { runProbe } from "../../../🏃️process/🔍️probe/🟦️.ts";
import { partitionNextestExecutionFilters } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";

/** 🧬️Reads an isolated fixture's exact current commit without depending on subprocess pipe capture. */
function fixtureGitHead(root: string): string {
  const head = readFileSync(join(root, ".git", "HEAD"), "utf8").trim();
  return head.startsWith("ref: ") ? readFileSync(join(root, ".git", head.slice(5)), "utf8").trim() : head;
}

describe("commit", () => {
  test("parseCommitBundleBody reads emoji scopes dates and bullets", async () => {
    const { parseCommitBundleBody } = await import("../../../🎛️commands/📝️commit/📜️body/🟦️.ts");
    const bundles = parseCommitBundleBody("🏘️compose✍️sketchpad\n🎆️26🌙️06☀️04\n🗺️Map work\n🎆️26🌙️06☀️03\n🧪️Playground\n\n🖱️ui⚛️react\n🎆️26🌙️06☀️02\n🖥️Shell");
    expect(bundles).toHaveLength(2);
    expect(bundles[0]?.label).toBe("🏘️compose✍️sketchpad");
    expect(bundles[0]?.dates).toHaveLength(2);
    expect(bundles[0]?.dates[0]?.bullets[0]).toBe("🗺️Map work");
  });

  test("parseCommitBundleBody rejects path prefixes and reserved emojis", async () => {
    const { parseCommitBundleBody } = await import("../../../🎛️commands/📝️commit/📜️body/🟦️.ts");
    expect(() => parseCommitBundleBody("compose/foo|🏘️compose\n🎆️26🌙️06☀️04\n🗺️Map work")).toThrow();
    expect(() => parseCommitBundleBody("🏘️compose🔀️📊️metric\n🎆️26🌙️06☀️04\n🗺️Map work")).toThrow();
    expect(() => parseCommitBundleBody("🗺️🧩️🕸️\n🎆️26🌙️06☀️04\n🗺️Map work")).toThrow();
  });

  test("normalizeBundleScopeLabel strips reserved and uloc suffix", async () => {
    const { normalizeBundleScopeLabel } = await import("../../../🎛️commands/📝️commit/🏷️labels/🟦️.ts");
    expect(normalizeBundleScopeLabel("🏘️compose🔀️📊️metric📃uloc➕️1")).toBe("🏘️compose");
  });

  test("isBundleScopeLine accepts area and technology root labels", async () => {
    const { isBundleScopeLine } = await import("../../../🎛️commands/📝️commit/🏷️labels/🟦️.ts");
    expect(isBundleScopeLine("🌐️gis📍️map")).toBe(true);
    expect(isBundleScopeLine("🖱️ui⚛️react")).toBe(true);
    expect(isBundleScopeLine("🥅️framework")).toBe(true);
    expect(isBundleScopeLine("🧪️Playground")).toBe(false);
    expect(isBundleScopeLine("🗺️Single emoji line")).toBe(false);
  });

  test("extractBundleDateLineFromSubject reads calendar day from micro-commit subject", async () => {
    const { extractBundleDateLineFromSubject } = await import("../../../🎛️commands/📝️commit/📅️dates/🟦️.ts");
    expect(extractBundleDateLineFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️012")).toBe("🎆️26🌙️06☀️04");
    expect(extractBundleDateLineFromSubject("unrelated")).toBeNull();
  });

  test("extractBundleDateLineFromCommit prefers body timestamp over subject checkpoint day", async () => {
    const { extractBundleDateLineFromCommit, extractBundleDateLineFromCommitBody } = await import("../../../🎛️commands/📝️commit/📅️dates/🟦️.ts");
    const body = "🎆️26🌙️06☀️04⏰️02⌚️38⏱️38\n🗺️Map work\n";
    expect(extractBundleDateLineFromCommitBody(body)).toBe("🎆️26🌙️06☀️04");
    expect(extractBundleDateLineFromCommit("🐙️ueli🎆️26🌙️06☀️02🚩️084", body)).toBe("🎆️26🌙️06☀️04");
  });

  test("pathsFromNumstatRow expands rename paths", async () => {
    const { pathsFromNumstatRow } = await import("../../../🎛️commands/📝️commit/📊️metrics/🔀️deltas/🟦️.ts");
    expect(pathsFromNumstatRow("old/a.ts\tnew/b.ts")).toEqual(["old/a.ts", "new/b.ts"]);
    expect(pathsFromNumstatRow("dir/{old.ts => new.ts}")).toEqual(["dir/old.ts", "dir/new.ts"]);
  });

  test("pathMatchesBundleIndex does not treat empty prefix set as match-all", async () => {
    const { pathMatchesBundleIndex } = await import("../../../🎛️commands/📝️commit/🧭️path-prefixes/🟦️.ts");
    const bundles = [
      { label: "🏘️compose✍️sketchpad", dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["✍️x"] }] },
      { label: "🏘️compose🗃️fixtures", dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["🗃️y"] }] },
    ];
    const prefixSets = [[], ["compose/fixture"]];
    expect(pathMatchesBundleIndex("compose/fixture/a.json", 0, prefixSets, bundles)).toBe(false);
    expect(pathMatchesBundleIndex("compose/fixture/a.json", 1, prefixSets, bundles)).toBe(true);
  });

  test("formatBundleDateLine appends per-day uloc suffix", async () => {
    const { formatBundleDateLine } = await import("../../../🎛️commands/📝️commit/📅️dates/🟦️.ts");
    expect(formatBundleDateLine("🎆️26🌙️06☀️04", { added: 700, edited: 200, removed: 10 }, { added: 788_000, edited: 0, removed: 0 }, 65_000, 10_400_000_000)).toBe(
      "🎆️26🌙️06☀️04📊️metric📃uloc💯️65k📈️690➗️1.07➕️700✏️200➖️10🟰️910📊️metric💾size💯️10.4GB📈️788KB➗️0.008➕️788KB🟰️788KB",
    );
  });

  test("commitBundleBodyError rejects per-day uloc on stdin", async () => {
    const { commitBundleBodyError } = await import("../../../🎛️commands/📝️commit/📜️body/🟦️.ts");
    expect(commitBundleBodyError("🏘️compose\n🎆️26🌙️06☀️04📊️metric📃uloc➕️1\n🗺️Work")).toMatch(/per-day/);
  });

  test("validateMicroCommitLangMetricsDeltaSum passes when language rows sum to footer", async () => {
    const { validateMicroCommitLangMetricsDeltaSum } = await import("../../../🎛️commands/📝️commit/📊️metrics/🧮️summary/🟦️.ts");
    expect(() =>
      validateMicroCommitLangMetricsDeltaSum([
        { lang: "TypeScript", emoji: "🟦️", code: 100, edited: 5, added: 3, removed: 0 },
        { lang: "Rust", emoji: "🦀️", code: 50, edited: 2, added: 1, removed: 1 },
      ]),
    ).not.toThrow();
  });

  test("validateBundleCommitAttribution requires bundle headers to sum to range total", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { validateBundleCommitAttribution } = await import("../../../🎛️commands/📝️commit/📈️attribution/🟦️.ts"), { parseCommitBundleBody } = await import("../../../🎛️commands/📝️commit/📜️body/🟦️.ts");
    const root = mkdtempSync(join(tmpdir(), "compose-commit-check-sum-"));
    try {
      spawnSync("git", ["init"], { cwd: root });
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root });
      mkdirSync(join(root, "repo", "js"), { recursive: true });
      mkdirSync(join(root, "other"), { recursive: true });
      writeFileSync(join(root, "repo/js/a.ts"), "a\n", "utf8");
      writeFileSync(join(root, "other/b.ts"), "b\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "🐙️ueli🎆️26🌙️06☀️01🔀️"], { cwd: root });
      const wip = fixtureGitHead(root);
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\n", "utf8");
      writeFileSync(join(root, "other/b.ts"), "b\nc\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      const msg = join(root, "mc.txt");
      writeFileSync(msg, "🐙️ueli🎆️26🌙️06☀️01🚩️001\n\n🎆️26🌙️06☀️04⏰️12⌚️00⏱️00\n🔧️Work\n", "utf8");
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", msg], { cwd: root });
      const bundles = parseCommitBundleBody("📚️repo🔧️js\n🎆️26🌙️06☀️04\n🔧️Only repo");
      expect(() => validateBundleCommitAttribution(root, wip, "HEAD", bundles)).toThrow(/commit\.attribution\.changed-path-is-not-attributed|commit\.attribution\.per-day-metric-for-does/);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("validateBundleDayDeltasAttribution rejects unlisted micro-commit day", async () => {
    const { validateBundleDayDeltasAttribution } = await import("../../../🎛️commands/📝️commit/📈️attribution/🟦️.ts");
    const bundles = [{ label: "📚️repo🔧️js", dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["🔧️Net"] }] }];
    const prefixSets = [["repo/js/"]];
    const dateDeltas = new Map([
      [
        0,
        new Map([
          ["🎆️26🌙️06☀️03", { added: 4, removed: 0, edited: 0 }],
          ["🎆️26🌙️06☀️04", { added: 0, removed: 4, edited: 0 }],
        ]),
      ],
    ]);
    const bundleTotals = [{ added: 0, removed: 0, edited: 0 }];
    expect(() => validateBundleDayDeltasAttribution(bundles, prefixSets, dateDeltas, bundleTotals)).toThrow(/commit\.attribution\.has-changes-on-but-that/);
  });

  test("validateBundleDayDeltasAttribution rejects when listed days do not sum to bundle total", async () => {
    const { validateBundleDayDeltasAttribution } = await import("../../../🎛️commands/📝️commit/📈️attribution/🟦️.ts");
    const bundles = [
      {
        label: "📚️repo🔧️js",
        dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["🔧️Only day four listed"] }],
      },
    ];
    const dateDeltas = new Map([
      [
        0,
        new Map([
          ["🎆️26🌙️06☀️04", { added: 2, edited: 0, removed: 0 }],
          ["🎆️26🌙️06☀️03", { added: 5, edited: 0, removed: 0 }],
        ]),
      ],
    ]);
    expect(() => validateBundleDayDeltasAttribution(bundles, [["repo/js"]], dateDeltas, [{ added: 2, edited: 0, removed: 0 }])).toThrow(/commit\.attribution\.has-changes-on-but-that/);
    const dateDeltasOneDay = new Map([[0, new Map([["🎆️26🌙️06☀️04", { added: 2, edited: 0, removed: 0 }]])]]);
    expect(() => validateBundleDayDeltasAttribution(bundles, [["repo/js"]], dateDeltasOneDay, [{ added: 7, edited: 0, removed: 0 }])).toThrow(/commit\.attribution\.per-day-metric-for-does/);
  });

  test("buildCommitMessage appends per-day uloc from micro-commit dates", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { buildCommitMessage } = await import("../../../🎛️commands/📝️commit/🔀️run-commit/🟦️.ts"), { parseCommitBundleBody } = await import("../../../🎛️commands/📝️commit/📜️body/🟦️.ts");
    const root = mkdtempSync(join(tmpdir(), "compose-commit-day-uloc-"));
    try {
      spawnSync("git", ["init"], { cwd: root });
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root });
      mkdirSync(join(root, "repo", "js"), { recursive: true });
      writeFileSync(join(root, "repo/js/a.ts"), "a\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "🐙️ueli🎆️26🌙️06☀️01🔀️"], { cwd: root });
      const wip = fixtureGitHead(root);
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\n", "utf8");
      spawnSync("git", ["add", "repo/js/a.ts"], { cwd: root });
      const msg1 = join(tmpdir(), `compose-mc1-${Date.now()}.txt`);
      writeFileSync(msg1, "🐙️ueli🎆️26🌙️06☀️01🚩️001\n\n🎆️26🌙️06☀️03⏰️12⌚️00⏱️00\n🔧️Day three\n", "utf8");
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", msg1], { cwd: root });
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\nc\nd\n", "utf8");
      spawnSync("git", ["add", "repo/js/a.ts"], { cwd: root });
      const msg2 = join(tmpdir(), `compose-mc2-${Date.now()}.txt`);
      writeFileSync(msg2, "🐙️ueli🎆️26🌙️06☀️01🚩️002\n\n🎆️26🌙️06☀️04⏰️12⌚️01⏱️00\n🔧️Day four\n", "utf8");
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", msg2], { cwd: root });
      const contributor = { alias: "ueli", emoji: "🐙️", name: "U", email: "u@e.com" };
      const bundles = parseCommitBundleBody("📚️repo🔧️js\n🎆️26🌙️06☀️04\n🔧️Day four\n🎆️26🌙️06☀️03\n🔧️Day three");
      const msg = buildCommitMessage(root, contributor, bundles, wip, "HEAD");
      expect(msg).toMatch(/🎆️26🌙️06☀️04📊️metric/);
      expect(msg).toMatch(/🎆️26🌙️06☀️03📊️metric/);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 60_000);

  test("sortCommitBundlesByEditTotal orders bundles by descending gitDeltaLineTotal", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { sortCommitBundlesByEditTotal } = await import("../../../🎛️commands/📝️commit/📈️attribution/🟦️.ts");
    const mk = (label: string) => ({
      label,
      dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["🗺️change"] }],
    });
    const root = mkdtempSync(join(tmpdir(), "compose-commit-sort-"));
    try {
      spawnSync("git", ["init"], { cwd: root });
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root });
      mkdirSync(join(root, "compose"), { recursive: true });
      mkdirSync(join(root, "ui"), { recursive: true });
      mkdirSync(join(root, "framework"), { recursive: true });
      writeFileSync(join(root, "compose/a.ts"), "a\n", "utf8");
      writeFileSync(join(root, "ui/b.ts"), "b\n", "utf8");
      writeFileSync(join(root, "framework/c.ts"), "c\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "init"], { cwd: root });
      const base = fixtureGitHead(root);
      writeFileSync(join(root, "compose/a.ts"), `${"a\n".repeat(11)}`, "utf8");
      writeFileSync(join(root, "ui/b.ts"), `${"b\n".repeat(151)}`, "utf8");
      writeFileSync(join(root, "framework/c.ts"), `${"c\n".repeat(5)}`, "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "delta"], { cwd: root });
      const head = fixtureGitHead(root);
      const bundles = [mk("🏘️compose"), mk("🖱️ui"), mk("🥅️framework")];
      const paths = [["compose/a.ts"], ["ui/b.ts"], ["framework/c.ts"]];
      const sorted = sortCommitBundlesByEditTotal(root, base, head, bundles, paths);
      expect(sorted.bundles.map((b) => b.label)).toEqual(["🖱️ui", "🏘️compose", "🥅️framework"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("buildCommitMessage renders bundle subject and footer", async () => {
    const { buildCommitMessage } = await import("../../../🎛️commands/📝️commit/🔀️run-commit/🟦️.ts"), { parseCommitBundleBody } = await import("../../../🎛️commands/📝️commit/📜️body/🟦️.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    const bundles = parseCommitBundleBody("📚️repo🔧️js\n🎆️26🌙️06☀️04\n🔧️Tooling");
    const root = mkdtempSync(join(tmpdir(), "semio-bundle-message-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      mkdirSync(join(root, "repo", "js"), { recursive: true });
      writeFileSync(join(root, "repo", "js", "a.ts"), "export const a = 1;\n");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "fixture"], { cwd: root });
      const head = fixtureGitHead(root);
      const msg = buildCommitMessage(root, contributor, bundles, head, head, { countRepoUlocByLanguage: () => ({ TypeScript: 1000 }), countRepoSizeByLanguage: () => ({}) });
      const lines = msg.trimEnd().split("\n");
      expect(lines[0]).toMatch(/🔀️$/);
      expect(lines.some((l) => l.includes("📊️metric"))).toBe(true);
      expect(lines.at(-1)).toMatch(/^Signed-off-by: /);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 30_000);

  test("formatBundleTagName and formatBundleSubject use contributor date emojis", async () => {
    const { formatBundleTagName, formatBundleSubject } = await import("../../../🎛️commands/📝️commit/🔖️tags/🟦️.ts");
    const c = { alias: "ueli", emoji: "🐙️", name: "U", email: "u@e.com" };
    const now = new Date("2026-06-04T12:00:00");
    expect(formatBundleTagName(c, now)).toBe("🐙️ueli🎆️26🌙️06☀️04🚩️");
    expect(formatBundleSubject(c, now)).toBe("🐙️ueli🎆️26🌙️06☀️04🔀️");
  });

  test("formatCommitPrepareCommands emits four fenced git blocks", async () => {
    const { formatCommitPrepareCommands } = await import("../../../🎛️commands/📝️commit/📋️prepare-commands/🟦️.ts");
    const out = formatCommitPrepareCommands({
      tagName: "🐙️ueli🎆️26🌙️06☀️04🚩️",
      wipSha: "abc123def456",
      messageFile: ".git/compose-commit-message",
    });
    const blocks = out.trimEnd().split("\n\n");
    expect(blocks).toHaveLength(4);
    expect(blocks[0]).toBe("```\ngit tag -s -m '🐙️ueli🎆️26🌙️06☀️04🚩️' '🐙️ueli🎆️26🌙️06☀️04🚩️' HEAD\n```");
  });

  test("formatCommitPrepareAgentReply ends with tag name and commit message blocks", async () => {
    const { formatCommitPrepareAgentReply } = await import("../../../🎛️commands/📝️commit/📋️prepare-commands/🟦️.ts");
    const commitMessage = "🐙️ueli🎆️26🌙️06☀️04🔀️\n\n🏘️compose✍️sketchpad📊️metric\n🎆️26🌙️06☀️04\n🗺️Work\n\n📊️metric📃uloc➕️1🟰️1\n\nSigned-off-by: U <u@e.com>\n";
    const out = formatCommitPrepareAgentReply({
      tagName: "🐙️ueli🎆️26🌙️06☀️04🚩️",
      wipSha: "abc",
      commitMessage,
    });
    const fenceBodies = [...out.matchAll(/```\n([\s\S]*?)\n```/g)].map((m) => m[1]!);
    expect(fenceBodies).toHaveLength(6);
    expect(fenceBodies[4]).toBe("🐙️ueli🎆️26🌙️06☀️04🚩️");
    expect(fenceBodies[5]).toBe(commitMessage.trimEnd());
  });

  test("parseCommitSteps treats cs as squash without tag", async () => {
    const { parseCommitSteps } = await import("../../../🎛️commands/📝️commit/🪜️steps/🟦️.ts");
    expect(parseCommitSteps(["cs"])).toEqual({ tag: false, squash: true, push: false });
    expect(parseCommitSteps(["ct", "cs", "cp"])).toEqual({ tag: true, squash: true, push: true });
  });

  test("bulletMatchesCommitHistory detects verbatim prior commit lines", async () => {
    const { bulletMatchesCommitHistory } = await import("../../../🎛️commands/📝️commit/📤️emit/🟦️.ts");
    const history = new Set(["🗺️copied line from an old micro-commit"]);
    expect(bulletMatchesCommitHistory("🗺️copied line from an old micro-commit", history)).toBe(true);
    expect(bulletMatchesCommitHistory("🗺️fresh summary written from git diff", history)).toBe(false);
  });
});

describe("command budgets", () => {
  const indexPath = join(contractRoot, "../../🏃️process/🟦️.ts");

  /** ⏱️Runs `runCmd` in a fresh subprocess (its budget-exceeded path calls `process.exit`, which would kill the test runner in-process) against a child that sleeps far longer than its budget. */
  function spawnBudgetedSleep(budgetMs: number, envOverride?: Record<string, string>): ReturnType<typeof runProbe> {
    const script = `const { runCmd } = await import(${JSON.stringify(indexPath)}); runCmd("bun", ["-e", "await Bun.sleep(10000)"], ${budgetMs === -1 ? "{}" : `{ budgetMs: ${budgetMs} }`});`;
    return runProbe("bun", ["-e", script], { budgetMs: 5_000, env: { ...process.env, ...envOverride } });
  }

  test("captured Cargo failures replay structured compiler errors without warning floods", () => {
    const warning = JSON.stringify({ reason: "compiler-message", message: { level: "warning", rendered: "warning: noisy dependency" } });
    const error = JSON.stringify({ reason: "compiler-message", message: { level: "error", rendered: "error[E0609]: exact field failure\n" } });
    expect(capturedTestFailureDiagnostics(`${warning}\n${error}\n`, "warning: stderr noise")).toBe("error[E0609]: exact field failure\n");
  });

  test("captured Cargo failures retain unrendered structured errors and stderr causes", () => {
    const error = JSON.stringify({
      reason: "compiler-message",
      message: {
        level: "error",
        message: "missing schema attribute",
        spans: [{ file_name: "src/schema.rs", line_start: 17, column_start: 4, is_primary: true }],
      },
    });
    expect(capturedTestFailureDiagnostics(error, "warning: noise\nCaused by: fixture failed")).toBe(
      "missing schema attribute\n  --> src/schema.rs:17:4\nCaused by: fixture failed",
    );
  });

  test("captured Cargo failures retain human compiler spans and complete causes", () => {
    const fixture = JSON.parse(readFileSync(join(contractRoot,"../🧫️fixtures/📣️cargo-diagnostics/🔣️.json"),"utf8")) as {humanStderr:string;expected:string;rustSource:string};
    expect(capturedTestFailureDiagnostics("",fixture.humanStderr)).toBe(fixture.expected);
    const output = spawnSync("rustc",["--error-format=human","--emit=metadata","-"],{input:fixture.rustSource,encoding:"utf8",timeout:10_000});
    expect(output.status).not.toBe(0);
    expect(output.stderr).toContain("error[E0609]");
    const captured = capturedTestFailureDiagnostics("",output.stderr);
    expect(captured).toContain("--> <anon>:2:");
    expect(captured).toContain("unknown field");
    expect(captured).toBe(output.stderr.slice(output.stderr.indexOf("error[E0609]")).trimEnd());
  });

  test("kills a command that exceeds its explicit budget", () => {
    const start = Date.now();
    const result = spawnBudgetedSleep(250);
    expect(result.status).not.toBe(0);
    expect(result.stderr).toContain("[budget]");
    expect(Date.now() - start).toBeLessThan(5000);
  });

  test("SEMIO_CMD_BUDGET_MS env override kills a command with no explicit budget", () => {
    const result = spawnBudgetedSleep(-1, { SEMIO_CMD_BUDGET_MS: "250" });
    expect(result.status).not.toBe(0);
    expect(result.stderr).toContain("[budget]");
  });

  test("orchestratorBudgetOpts supplies the opt-in orchestrator budget", () => {
    expect(orchestratorBudgetOpts()).toEqual({ budgetMs: orchestratorBudgetMs(), env: devToolingEnv() });
    expect(() => runCmd(process.execPath, ["-e", "1"], orchestratorBudgetOpts())).not.toThrow();
  });

  test("daemonBudgetOpts returns the daemon budget class", () => {
    expect(daemonBudgetOpts()).toEqual({ budgetMs: daemonBudgetMs(), env: devToolingEnv() });
    expect(daemonBudgetMs()).toBe(DAEMON_BUDGET_MS);
    expect(orchestratorBudgetMs()).toBe(ORCHESTRATOR_BUDGET_MS);
  });

  test("goLevelTestArgs includes -timeout derived from the level budget", () => {
    const args = goLevelTestArgs("fundamental");
    expect(args[0]).toBe("-timeout");
    expect(args[1]).toMatch(/^\d+s$/);
    expect(args[1]).toBe(`${Math.ceil(testLevelBudgetMs("fundamental") / 1000)}s`);
  });

  test("vitestLevelArgs returns per-test timeout flags", () => {
    const ms = String(testLevelBudgetMs("quick"));
    expect(vitestLevelArgs("quick")).toEqual(["--testTimeout", ms, "--hookTimeout", ms, "--teardownTimeout", ms]);
  });

  test("orchestratorBudgetMs defaults to unlimited and honors SEMIO_ORCHESTRATOR_BUDGET_MS", () => {
    expect(orchestratorBudgetMs()).toBe(0);
    const prev = process.env.SEMIO_ORCHESTRATOR_BUDGET_MS;
    process.env.SEMIO_ORCHESTRATOR_BUDGET_MS = "12345";
    try {
      expect(orchestratorBudgetMs()).toBe(12345);
    } finally {
      if (prev === undefined) delete process.env.SEMIO_ORCHESTRATOR_BUDGET_MS;
      else process.env.SEMIO_ORCHESTRATOR_BUDGET_MS = prev;
    }
  });

  test("daemonBudgetMs defaults to unlimited and honors SEMIO_DAEMON_BUDGET_MS", () => {
    expect(daemonBudgetMs()).toBe(0);
    const prev = process.env.SEMIO_DAEMON_BUDGET_MS;
    process.env.SEMIO_DAEMON_BUDGET_MS = "67890";
    try {
      expect(daemonBudgetMs()).toBe(67890);
    } finally {
      if (prev === undefined) delete process.env.SEMIO_DAEMON_BUDGET_MS;
      else process.env.SEMIO_DAEMON_BUDGET_MS = prev;
    }
  });

  test("testLevelBudgetMs maps levels and honors SEMIO_TEST_BUDGET_MS", async () => {
    const { TEST_LEVEL_BUDGET_MS } = await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts");
    expect(testLevelBudgetMs("fundamental")).toBe(TEST_LEVEL_BUDGET_MS.fundamental);
    expect(testLevelBudgetMs("exhaustive")).toBe(TEST_LEVEL_BUDGET_MS.exhaustive);
    const prev = process.env.SEMIO_TEST_BUDGET_MS;
    process.env.SEMIO_TEST_BUDGET_MS = "42000";
    try {
      expect(testLevelBudgetMs("quick")).toBe(42000);
    } finally {
      if (prev === undefined) delete process.env.SEMIO_TEST_BUDGET_MS;
      else process.env.SEMIO_TEST_BUDGET_MS = prev;
    }
  });

  test("runProbe captures stdout under budget", () => {
    const result = runProbe("bun", ["-e", "console.log('probe-ok')"]);
    const { status, stdout } = result;
    expect(status).toBe(0);
    expect(stdout.trim()).toBe("probe-ok");
  });

  test("runCmdStatus returns the exit status instead of throwing", () => {
    expect(runCmdStatus(process.execPath, ["-e", "process.exit(3)"])).toBe(3);
  });

  test("budgetTimeoutHint defaults cargo to the target-dir lock-contention hint", () => {
    expect(budgetTimeoutHint("cargo")).toContain("target-dir lock contention");
  });

  test("budgetTimeoutHint honors an explicit override", () => {
    expect(budgetTimeoutHint("git", "custom hint")).toBe("custom hint");
  });

  test("budgetTimeoutHint gives non-cargo commands a generic hint", () => {
    expect(budgetTimeoutHint("git")).not.toContain("target-dir lock contention");
  });
});

describe("nextest execution filters", () => {
  test("uses explicit task artifacts, refuses blank ones and never retains them", () => {
    const fixture = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🏎️nextest/🗺️artifact-location.json"), "utf8")) as { schema: string; cases: Array<{ value: string; relative: string | null; refused: boolean }> };
    for (const row of fixture.cases) {
      if (row.refused) expect(() => nextestArtifactLocation(contractRoot, { SEMIO_TEST_ARTIFACT_DIR: row.value })).toThrow("Bounded original native test evidence directory required");
      else expect(nextestArtifactLocation(contractRoot, { SEMIO_TEST_ARTIFACT_DIR: row.value })).toEqual({ directory: resolve(contractRoot, row.relative!), retain: false });
    }
  });

  test("preserves language-neutral build and execution vectors with an independent Node parser", () => {
    const fixtureRoot = join(contractRoot, "../🧫️fixtures/🏎️nextest");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🎮️command-vectors.json"), "utf8")) as {
      schema: string;
      cases: Array<{ name: string; input: string[]; build: string[]; execution: string[]; libtest: string[] }>;
      rejected: string[][];
    };
    
    expect(fixture["schema"]).toEqual("nextest-execution-filter-vectors/v1");
    const runtimeKeys = ["filter-expr", "partition", "run-ignored", "ignore-default-filter", "no-fail-fast"];
    for (const row of fixture.cases) {
      const options = {
        lib: { type: "boolean" }, release: { type: "boolean" }, "no-default-features": { type: "boolean" }, package: { type: "string", short: "p", multiple: true }, features: { type: "string", short: "F", multiple: true }, target: { type: "string" }, "target-dir": { type: "string" }, "build-jobs": { type: "string" }, "cargo-profile": { type: "string" }, "cargo-message-format": { type: "string", multiple: true }, timings: { type: row.input.some((arg) => arg.startsWith("--timings=")) ? "string" : "boolean" }, config: { type: "string", multiple: true }, Z: { type: "string", short: "Z", multiple: true }, test: { type: "string", multiple: true }, bin: { type: "string", multiple: true }, bench: { type: "string", multiple: true }, example: { type: "string", multiple: true },
        "filter-expr": { type: "string", short: "E", multiple: true }, partition: { type: "string" }, "run-ignored": { type: "string" }, "ignore-default-filter": { type: "boolean" }, "no-fail-fast": { type: "boolean" },
      } as const;
      const actual = partitionNextestExecutionFilters(row.input);
      expect(actual, row.name).toEqual({ buildArgs: row.build, executionArgs: row.execution, libtestArgs: row.libtest });
      const separator = row.input.indexOf("--");
      const original = parseArgs({ args: separator < 0 ? row.input : row.input.slice(0, separator), options, strict: true, allowPositionals: true });
      const runtime = parseArgs({ args: actual.executionArgs, options, strict: true, allowPositionals: true });
      const build = parseArgs({ args: actual.buildArgs, options, strict: true, allowPositionals: true });
      expect(runtime.values, row.name).toEqual(Object.fromEntries(Object.entries(original.values).filter(([key]) => runtimeKeys.includes(key))));
      expect(build.values, row.name).toEqual(Object.fromEntries(Object.entries(original.values).filter(([key]) => !runtimeKeys.includes(key))));
      expect(runtime.positionals, row.name).toEqual(original.positionals);
      expect(build.positionals, row.name).toEqual([]);
    }
    for (const args of fixture.rejected) expect(() => partitionNextestExecutionFilters(args)).toThrow();
  });
});

describe("resolveCargoPackageName", () => {
  test("resolves short lib names to full package names", () => {
    const root = getWorkspaceRoot();
    expect(resolveCargoPackageName("db", join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust"))).toBe("semio-framework-os-kernel-db");
    expect(resolveCargoPackageName("semio-hub-architect", join(root, "🌎️hub/🧩️compositions/🏛️architect/📦️packages/🦀️rust"))).toBe("semio-hub-architect");
    expect(resolveCargoPackageName("semio-hub-energy", join(root, "🌎️hub/🧩️compositions/🔋️energy/📦️packages/🦀️rust"))).toBe("semio-hub-energy");
  });

  test("resolves empty package list to local Cargo.toml package name", () => {
    const root = getWorkspaceRoot();
    const dbDir = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust");
    expect(resolveCargoPackageNames([], dbDir)).toEqual(["semio-framework-os-kernel-db"]);
  });
});
