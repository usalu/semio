/** 🔶️ The safegitenv replaces unusable configured tests of the micro-commit suite. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

/** 🔷 Registers the safegitenv replaces unusable configured tests of the micro-commit suite. */
export function registerSafegitenvReplacesUnusableConfiguredTests(): void {
  test("safeGitEnv replaces an unusable configured global Git config", async () => {
    const { safeGitEnv } = await import("../../../../🏃️process/🌿️environment/🌳️git/🟦️.ts");
    const { devNull } = await import("node:os");
    const env = safeGitEnv({ GIT_CONFIG_GLOBAL: join(process.cwd(), ".missing-global-gitconfig") });
    expect(env.GIT_CONFIG_GLOBAL).toBe(devNull);
  });
  test("branchValidationError distinguishes lookup failure, detached HEAD, and a wrong branch", async () => {
    const { branchValidationError } = await import("../../../../🎛️commands/📝️commit/👤️contributor/🟦️.ts");
    expect(branchValidationError("micro-commit", { ok: false, error: "permission denied" })).toBe("micro-commit: cannot read current branch: permission denied");
    expect(branchValidationError("micro-commit", { ok: true, name: "" })).toContain("detached HEAD");
    expect(branchValidationError("micro-commit", { ok: true, name: "🐙️ueli/feature" })).toContain('current branch "🐙️ueli/feature"');
    expect(branchValidationError("micro-commit", { ok: true, name: "🐙️ueli/⛳️wip" })).toBeNull();
    expect(branchValidationError("micro-commit", { ok: true, name: "🐙️ueli/⛳️wip" })).toBeNull();
  });
  test("extractCounterFromSubject reads formatted subject lines", async () => {
    const { extractCounterFromSubject } = await import("../../../../🎛️commands/📝️commit/🔢️counter/🟦️.ts");
    expect(extractCounterFromSubject("🧑️ueli🎆️26🌙️06☀️02🚩️009")).toEqual({ nnn: 9, line1Base: "🧑️ueli🎆️26🌙️06☀️02" });
    expect(extractCounterFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️397")).toEqual({ nnn: 397, line1Base: "🐙️ueli🎆️26🌙️06☀️04" });
    expect(extractCounterFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️396")).toEqual({ nnn: 396, line1Base: "🐙️ueli🎆️26🌙️06☀️04" });
    expect(extractCounterFromSubject("33")).toBeNull();
    expect(extractCounterFromSubject("Merge branch foo")).toBeNull();
  });
  test("extractNumericCounterFromSubject reads GitKraken numeric subjects", async () => {
    const { extractNumericCounterFromSubject } = await import("../../../../🎛️commands/📝️commit/🔢️counter/🟦️.ts");
    expect(extractNumericCounterFromSubject("299")).toBe(299);
    expect(extractNumericCounterFromSubject("001")).toBe(1);
    expect(extractNumericCounterFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️151")).toBeNull();
  });
  test("line1BaseFromBundleTag reads WIP epoch from squash tag", async () => {
    const { line1BaseFromBundleTag } = await import("../../../../🎛️commands/📝️commit/🔢️counter/🟦️.ts");
    expect(line1BaseFromBundleTag("🐙️ueli🎆️26🌙️06☀️04🚩️")).toBe("🐙️ueli🎆️26🌙️06☀️04");
    expect(line1BaseFromBundleTag("🐙️ueli🎆️26🌙️06☀️04🚩️")).toBe("🐙️ueli🎆️26🌙️06☀️04");
    expect(line1BaseFromBundleTag("🐙️ueli🎆️26🌙️06☀️04🚩️151")).toBeNull();
  });
  test("bumpCounterFromHistory uses max across formatted commits", async () => {
    const { bumpCounterFromHistory } = await import("../../../../🎛️commands/📝️commit/🔢️counter/🟦️.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli", email: "u@example.com" };
    const subjects = ["🐙️ueli🎆️26🌙️06☀️02🚩️033", "🐙️ueli🎆️26🌙️06☀️02🚩️032", "unrelated"];
    const bumped = bumpCounterFromHistory(subjects, contributor, new Date("2026-06-02T12:00:00"));
    expect(bumped.line1Base).toBe("🐙️ueli🎆️26🌙️06☀️02");
    expect(bumped.nnn).toBe("034");
    const fresh = bumpCounterFromHistory(["unrelated"], contributor, new Date("2026-06-02T12:00:00"));
    expect(fresh.nnn).toBe("001");
  });
  test("bumpCounterFromHistory preserves selector-free history with canonical output", async () => {
    const { bumpCounterFromHistory } = await import("../../../../🎛️commands/📝️commit/🔢️counter/🟦️.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli", email: "u@example.com" };
    const bumped = bumpCounterFromHistory(["🦢️other🎆️26🌙️07☀️31🚩️999", "🐙️ueli🎆️26🌙️06☀️04🚩️397", "🐙️ueli🎆️26🌙️06☀️04🚩️396"], contributor, new Date("2026-07-31T12:00:00"));
    expect(bumped).toEqual({ line1Base: "🐙️ueli🎆️26🌙️06☀️04", nnn: "398" });
    expect(() => bumpCounterFromHistory(["🐙️ueli🎆️26🌙️6☀️04🚩️397"], contributor, new Date("2026-07-31T12:00:00"))).toThrow("refusing to reset counter to 001");
  });
  test("bumpCounterFromHistory continues numeric GitKraken subjects with WIP epoch", async () => {
    const { bumpCounterFromHistory } = await import("../../../../🎛️commands/📝️commit/🔢️counter/🟦️.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli", email: "u@example.com" };
    const subjects = ["299", "298", "297"];
    const bumped = bumpCounterFromHistory(subjects, contributor, new Date("2026-07-17T12:00:00"), "🐙️ueli🎆️26🌙️06☀️04");
    expect(bumped.line1Base).toBe("🐙️ueli🎆️26🌙️06☀️04");
    expect(bumped.nnn).toBe("300");
  });
  test("normalizeBulletLines strips uloc block lines", async () => {
    const { normalizeBulletLines } = await import("../../../../🎛️commands/📝️commit/🔸️bullets/🟦️.ts");
    const bullets = normalizeBulletLines("🎆️Summary\n📊️metric📃uloc💯️65k➕️1✏️1🟰️2\n📊️metric🟦️typescript📃uloc💯️65k➕️1✏️1🟰️2\n🐛️Fix bug");
    expect(bullets).toEqual(["🎆️Summary", "🐛️Fix bug"]);
  });
  test("bulletEmojiValidationError rejects fireworks emoji on bullets", async () => {
    const { bulletLeadEmoji, bulletEmojiValidationError } = await import("../../../../🎛️commands/📝️commit/🔸️bullets/🟦️.ts");
    expect(bulletLeadEmoji("🎆️Drop stacked intro")).toBe("🎆️");
    expect(bulletEmojiValidationError(["🎆️All bullets wrongly use fireworks"])).toContain("🎆️");
    expect(bulletEmojiValidationError(["🐛️Fix real bug"])).toBeNull();
    expect(bulletEmojiValidationError(["🧬️Tune WASM flush timing"])).toBeNull();
    expect(bulletEmojiValidationError(["📊️uloc block"])).toContain("📊️");
  });
  test("normalizeBulletLines enforces compact {emoji}{description} format", async () => {
    const { normalizeBulletLines, formatMicroCommitBulletLine } = await import("../../../../🎛️commands/📝️commit/🔸️bullets/🟦️.ts");
    expect(formatMicroCommitBulletLine("- 🐛️ Fix PDF")).toBe("🐛️Fix PDF");
    expect(normalizeBulletLines("🐛️ Fix PDF\n- 🖼️ Tweak UI")).toEqual(["🐛️Fix PDF", "🖼️Tweak UI"]);
    expect(normalizeBulletLines(Array.from({ length: 10 }, (_, i) => `🎆️item ${i}`).join("\n"))).toHaveLength(8);
  });
  test("buildMicroCommitMessage separates GitKraken summary and description", async () => {
    const { buildMicroCommitMessage } = await import("../../../../🎛️commands/📝️commit/📄️prepared-message/🟦️.ts");
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
      const { MICRO_COMMIT_ULOC_HEADER: ulocHeader } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🧮️summary/🟦️.ts");
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
    const { buildMicroCommitMessage } = await import("../../../../🎛️commands/📝️commit/📄️prepared-message/🟦️.ts");
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
    const { formatMicroCommitMetricLine, formatMicroCommitMetricsLines } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🧮️summary/🟦️.ts"), { formatMetricLocCount, formatMetricSizeCount, formatMetricRatio, COMMIT_METRIC_HEADER } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🎨️format/🟦️.ts");
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
    const { formatMicroCommitMetricsLines } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🧮️summary/🟦️.ts");
    const lines = formatMicroCommitMetricsLines([
      { lang: "TypeScript", emoji: "🟦️", code: 3000, edited: 10, added: 8, removed: 0 },
      { lang: "Markdown", emoji: "📝️", code: 44, edited: 0, added: 0, removed: 0 },
    ]);
    expect(lines[0]).toBe("📊️metric📃uloc💯️3.04k📈️8➗️0.264➕️8✏️10🟰️18");
    expect(lines[1]).toBe("📊️metric🟦️typescript📃uloc💯️3k📈️8➗️0.267➕️8✏️10🟰️18");
    expect(lines[2]).toBe("📊️metric📝️markdown📃uloc💯️44");
  });
}
