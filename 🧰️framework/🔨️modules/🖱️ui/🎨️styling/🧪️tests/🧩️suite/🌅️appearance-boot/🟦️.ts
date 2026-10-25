import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, it, expect } from "bun:test";
import { PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT, PLAYGROUND_PLAY_BOOT_THEME_SCRIPT } from "../../../🏗️builder/🌐️vite/🚀️boot-html/🟦️.ts";
import { repoRoot } from "../🧰️suite-build-server/🟦️.ts";

/**
 * 🌓️ The pre-paint appearance bootstrap, answered by EXECUTING the shipped inline script.
 *
 * A head script is not importable code: it is a string that the browser parses before any module
 * exists, so the only way to test what it actually does is to run that exact string. `node:vm` gives
 * it the smallest DOM it touches — `document.documentElement`, `document.body`, `localStorage` and
 * `matchMedia` — and the resolved class/dataset/background are read straight back off that DOM.
 * `🧪️tests/🧫️fixtures/🌓️appearance-boot.json` is the statement; `🎚️config/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`
 * answers the same `events` logs through the OS config projection.
 */
const appearanceBootFixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🌓️appearance-boot.json"), "utf8")) as {
  readonly storageKey: string;
  readonly preferenceKey: string;
  readonly deadKeys: readonly string[];
  readonly cases: readonly { name: string; storage?: unknown; storageRaw?: string; prefersDark: boolean; resolved: "light" | "dark"; bodyBackground: string }[];
  readonly deadKeyCases: readonly { name: string; deadKey: string; deadValue: string; prefersDark: boolean; resolved: "light" | "dark" }[];
};

/** 🚀️ Runs one boot script against a throwaway DOM and reports what the first painted frame would use. */
async function runBootAppearanceScript(script: string, store: Record<string, string>, prefersDark: boolean): Promise<{ dark: boolean; appearance: string; bodyBackground: string }> {
  const vm = await import("node:vm");
  const documentElement = { classList: { values: new Set<string>(), toggle(name: string, on: boolean) { if (on) this.values.add(name); else this.values.delete(name); } }, dataset: {} as Record<string, string>, style: {} as Record<string, string> };
  const body = { style: {} as Record<string, string> };
  const context = vm.createContext({
    document: { documentElement, body },
    window: { matchMedia: (query: string) => ({ matches: query.includes("dark") ? prefersDark : false }) },
    localStorage: { getItem: (key: string) => (key in store ? store[key]! : null) },
    JSON,
    Array,
  });
  vm.runInContext(script, context);
  return { dark: documentElement.classList.values.has("dark"), appearance: documentElement.dataset.uiAppearance ?? "", bodyBackground: body.style.backgroundColor ?? "" };
}

describe("pre-paint appearance bootstrap", () => {
  it("reads the document the OS shell actually writes, and none of the retired keys", () => {
    expect(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT).toContain(appearanceBootFixture.storageKey);
    expect(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT).toContain(appearanceBootFixture.preferenceKey);
    expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).toContain(appearanceBootFixture.storageKey);
    for (const dead of appearanceBootFixture.deadKeys) {
      expect(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT).not.toContain(dead);
      expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).not.toContain(dead);
    }
  });

  for (const testCase of appearanceBootFixture.cases) {
    it(testCase.name, async () => {
      const store: Record<string, string> = {};
      if (testCase.storageRaw !== undefined) store[appearanceBootFixture.storageKey] = testCase.storageRaw;
      else if (testCase.storage !== null && testCase.storage !== undefined) {
        const snapshot = testCase.storage as { version: number; preferences: Record<string, unknown> };
        store[appearanceBootFixture.storageKey] = JSON.stringify({ ...snapshot, preferences: Object.fromEntries(Object.entries(snapshot.preferences).map(([key, value]) => [key, JSON.stringify(value)])) });
      }
      const result = await runBootAppearanceScript(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT, store, testCase.prefersDark);
      expect(result.appearance).toBe(testCase.resolved);
      expect(result.dark).toBe(testCase.resolved === "dark");
      expect(result.bodyBackground).toBe(testCase.bodyBackground);
    });
  }

  for (const testCase of appearanceBootFixture.deadKeyCases) {
    it(testCase.name, async () => {
      const result = await runBootAppearanceScript(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT, { [testCase.deadKey]: testCase.deadValue }, testCase.prefersDark);
      expect(result.appearance).toBe(testCase.resolved);
    });
  }

  it("the hand-authored dev distribution head carries the byte-identical scripts", () => {
    const html = readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🌐️.html"), "utf8");
    expect(html).toContain(`<script>${PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT}</script>`);
    expect(html).toContain(`<script>${PLAYGROUND_PLAY_BOOT_THEME_SCRIPT}</script>`);
  });
});
