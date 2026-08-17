#!/usr/bin/env bun
/** 🧭️ Elements react UI router: `bun ./📜️script.ts <dev|build|lint|test|policy|check-ui-primitives|check-chrome-i18n> [args…]`. */

// 🏃️ `bun-types` isn't installed in this workspace; `import.meta.dir` (Bun's own runtime global) needs an ambient
// declaration rather than an `as any` cast at each call site.
declare global {
  interface ImportMeta {
    readonly dir: string;
  }
}
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import type { BundleLinter } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { dependencyBoundaryBreachesForBundleDir } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { BundleScript, ScriptRouter, devToolingEnv, resolveTestLevel, runBundleScriptMain, runBunx, runCmd, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { defineLint } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

export const policy = defineLint("@semio-tech/ui-react-bundle", (l: BundleLinter) => {
  const repoRoot = getWorkspaceRoot();
  return dependencyBoundaryBreachesForBundleDir(repoRoot, l.root());
});

const storybookEnv = (extra: Record<string, string | undefined> = {}) =>
  devToolingEnv({
    WATCHPACK_POLLING: process.env.WATCHPACK_POLLING ?? "true",
    CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "true",
    ...extra,
  });

/** This bundle has no local `.storybook` config — its stories live in the root Storybook's `ui` scope, so `dev`/`build` delegate there instead of running a broken standalone instance. */
class DevScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("bun", ["./📜️script.ts", "dev", "storybook", "ui", ...segments], { cwd: this.repoRoot, env: storybookEnv() });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("bun", ["./📜️script.ts", "build", "storybook", ...segments], { cwd: this.repoRoot, env: storybookEnv({ STORYBOOK_SCOPE: "ui" }) });
  }
}

class LintScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["eslint", "--max-warnings", "0", "--config", "🟦️eslint.config.ts", ".", ...segments], this.root, storybookEnv());
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root, storybookEnv());
  }
}

//#region 🔖️ui-primitives-lint
/** 🧱️ Apps compose framework/ui-provided elements only — raw DOM primitives and Storybook `component:` escape hatches
 * are confined to `ui/` and `framework/`. Seeded from a live repo scan on 2026-07-19; a listed file with zero hits
 * is a stale entry (fails), a hit outside this list fails, a hit inside it is allowed. */
export const UI_PRIMITIVES_ALLOWLIST: readonly string[] = [
  "✏️s/🔌️plugins/📐️cad/🔨️modules/📺️renderer/⚡️implementations/🟦️typescript/📦️index.tsx",
  "♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx",
  "♻️mit-bestand/🧺️demonstrator/📦️index.tsx",
] as const;

const UI_PRIMITIVES_SKIP_DIRS = new Set(["node_modules", "dist", "target", ".🧬semio", ".🧬semio", "storybook-static", ".claude", ".git"]);

const UI_PRIMITIVES_FRAMEWORK_PREFIX = "🧰️framework/";
const UI_PRIMITIVES_STORYBOOK_PREFIX = ".storybook/";
const UI_PRIMITIVES_VITEST_START_RE = /if\s*\(\s*import\.meta\.vitest\s*\)/;

const RAW_DOM_PRIMITIVE_RE = /<(button|input|select|form|textarea|dialog|progress|table)\b/;
const RAW_SVG_ICON_RE = /<svg\b/;
const COMPONENT_ESCAPE_HATCH_RE = /component:\s*[A-Z]/;

interface UiPrimitiveHit {
  file: string;
  line: number;
  kind: string;
  text: string;
}

/** 🔍️ Walks the repo tree for `.tsx`/`.jsx` raw-DOM-primitive and `component:` escape-hatch hits outside
 * framework implementation and Storybook/test-fixture boundaries, keyed by file. */
function collectUiPrimitivesHits(repoRootPath: string): Map<string, UiPrimitiveHit[]> {
  const hitsByFile = new Map<string, UiPrimitiveHit[]>();
  const record = (file: string, hit: UiPrimitiveHit): void => {
    const bucket = hitsByFile.get(file) ?? [];
    bucket.push(hit);
    hitsByFile.set(file, bucket);
  };

  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isDirectory()) {
        if (UI_PRIMITIVES_SKIP_DIRS.has(entry.name)) {
          continue;
        }
        walk(full);
        continue;
      }
      if (!/\.(tsx|jsx)$/.test(entry.name)) {
        continue;
      }
      const rel = full.slice(repoRootPath.length + 1);
      if (rel.startsWith(UI_PRIMITIVES_STORYBOOK_PREFIX)) {
        continue;
      }
      const lines = readFileSync(full, "utf8").split("\n");
      // 🧪️ In-file Vitest blocks are terminal test fixtures; their deliberately raw markup must not
      // affect a production UI-boundary check.
      const vitestStart = lines.findIndex((line) => UI_PRIMITIVES_VITEST_START_RE.test(line));
      const scannable = vitestStart === -1 ? lines : lines.slice(0, vitestStart);
      const isFrameworkImplementation = rel.startsWith(UI_PRIMITIVES_FRAMEWORK_PREFIX);
      for (let i = 0; i < scannable.length; i++) {
        const line = scannable[i]!;
        if (!isFrameworkImplementation && RAW_DOM_PRIMITIVE_RE.test(line)) {
          record(rel, { file: rel, line: i + 1, kind: "raw-dom-primitive", text: line.trim() });
        }
        if (!isFrameworkImplementation && RAW_SVG_ICON_RE.test(line)) {
          record(rel, { file: rel, line: i + 1, kind: "raw-svg-icon", text: line.trim() });
        }
        if (!isFrameworkImplementation && COMPONENT_ESCAPE_HATCH_RE.test(line)) {
          record(rel, { file: rel, line: i + 1, kind: "component-escape-hatch", text: line.trim() });
        }
      }
    }
  };

  walk(repoRootPath);
  return hitsByFile;
}

/** 🚫️ Fails when raw DOM primitives (`<button>`, `<svg>`, …) or `component:` escape hatches leak outside `ui/`/`framework/`,
 * or when the allowlist carries a stale entry for a file that no longer has any hits. */
class CheckUiPrimitivesScript extends BundleScript {
  run(): void {
    const hitsByFile = collectUiPrimitivesHits(this.repoRoot);
    const allowlist = new Set(UI_PRIMITIVES_ALLOWLIST);
    const failures: string[] = [];

    for (const [file, hits] of hitsByFile) {
      if (allowlist.has(file)) {
        continue;
      }
      for (const hit of hits) {
        failures.push(`${hit.file}:${hit.line} [${hit.kind}] ${hit.text}`);
      }
    }
    for (const file of allowlist) {
      if (!hitsByFile.has(file)) {
        failures.push(`${file} [stale-allowlist-entry] listed in UI_PRIMITIVES_ALLOWLIST but has zero hits — remove it`);
      }
    }

    if (failures.length === 0) {
      console.log(`framework/ui/js/react: no UI primitive violations (${allowlist.size} allowlisted files)`);
      return;
    }
    console.error(`framework/ui/js/react: found ${failures.length} UI primitive violation(s):`);
    for (const failure of failures.slice(0, 120)) {
      console.error(`  ${failure}`);
    }
    if (failures.length > 120) {
      console.error(`  … and ${failures.length - 120} more`);
    }
    process.exit(1);
  }
}
//#endregion 🔖️ui-primitives-lint

//#region 🌐️chrome-i18n-lint
/** 🌐️ Locale-locked brand shells must never show an untranslated English chrome literal — the strict
 * `useLabel` overloads (non-optional for known keys) and the `UiLabel` branded prop types are the
 * primary guarantee; this lint is the line-based backstop for the fallback-literal/bare-JSX-text
 * positions the type system can't see through (e.g. `foo ?? "Clear"` or bare JSX text compiles fine —
 * only this scan catches it). A listed file with zero hits is a stale entry (fails). */
export const CHROME_I18N_ALLOWLIST: readonly string[] = [];

/** 🌳️ Every chrome-bearing surface: this bundle itself, the OS renderer engine, the demonstrator brand
 * shell — walked recursively (mirrors {@link collectUiPrimitivesHits}'s
 * walker) rather than a fixed file list, so a file rename or new chrome file can't silently drop out of
 * the scan the way the old hardcoded two-file list did. */
const CHROME_I18N_SCANNED_ROOTS = [
  "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react",
  "♻️mit-bestand/🧺️demonstrator",
] as const;

const CHROME_I18N_SKIP_DIRS = new Set(["node_modules", "dist", "dist-staging", "target", ".🧬semio", ".🧬semio", "storybook-static", ".claude", ".git"]);

const CHROME_I18N_FALLBACK_RE = /(\?\?|\|\|)\s*"[A-Z][^"]*"/;
const CHROME_I18N_JSX_TEXT_RE = />[A-Z][a-zA-Z]+(?: [a-zA-Z…]+)*<\//;
const CHROME_I18N_VITEST_START_RE = /if\s*\(\s*import\.meta\.vitest\s*\)/;

/** 🗣️ Proper nouns (organization names) whose value is identical across every locked locale — not a
 * translation gap, so the lint must not flag them even though they start with a capital letter. */
const CHROME_I18N_LOCALE_INVARIANT_VALUES = new Set(["Leibniz Universität Hannover", "Universität der Künste Berlin"]);

/** 🙈️ Explicit, greppable per-line escape hatch (mirrors `eslint-disable-line`) for English literals that
 * are demo/fixture data — never rendered by a real locale-locked app — rather than actual chrome. */
const CHROME_I18N_IGNORE_MARKER = "chrome-i18n-allow";

/** 🕳️ The sanctioned "wire boundary" mint points where a resolved manifest/i18next string is allowed to
 * become a branded `UiLabel` via an `as UiLabel` cast — `uiDataLabel` (the explicit runtime-data escape
 * hatch), `useLabel`/`useIdLabel` (the strict/dynamic translation-key hooks), and the renderer engine's
 * `resolveManifestLabel`/`wireLabel`. Everywhere else, a real translation lookup or `uiDataLabel(...)`
 * must be used instead of casting a raw string. */
const CHROME_I18N_BOUNDARY_FN_RE = /\b(export\s+)?(function|const)\s+(uiDataLabel|useLabel|useIdLabel|resolveManifestLabel|wireLabel)\b/;

const CHROME_I18N_UI_DATA_LABEL_LITERAL_RE = /\buiDataLabel\(\s*"[A-Z][^"]*"/;
const CHROME_I18N_AS_UI_LABEL_RE = /\bas\s+UiLabel\b/;

/** 🧮️ Finds `[startLine, endLine]` (0-indexed, inclusive) ranges for every boundary-function definition
 * in a file by brace-depth tracking from its signature line — covers both `function` bodies and
 * single-line `const x = (...) => ... as UiLabel;` arrow forms (no braces at all). */
function findChromeI18nBoundaryRanges(lines: readonly string[]): Array<[number, number]> {
  const ranges: Array<[number, number]> = [];
  for (let i = 0; i < lines.length; i++) {
    if (!CHROME_I18N_BOUNDARY_FN_RE.test(lines[i]!)) {
      continue;
    }
    let depth = 0;
    let sawBrace = false;
    let end = i;
    for (let j = i; j < lines.length; j++) {
      for (const ch of lines[j]!) {
        if (ch === "{") {
          depth++;
          sawBrace = true;
        } else if (ch === "}") {
          depth--;
        }
      }
      end = j;
      if (sawBrace && depth <= 0) {
        break;
      }
      if (!sawBrace && lines[j]!.trimEnd().endsWith(";")) {
        break;
      }
    }
    ranges.push([i, end]);
  }
  return ranges;
}

interface ChromeI18nHit {
  file: string;
  line: number;
  kind: string;
  text: string;
}

/** 🔍️ Walks every root in {@link CHROME_I18N_SCANNED_ROOTS} for `.ts`/`.tsx` files and scans each for
 * fallback-literal/bare-JSX-text/literal-abuse English hits, skipping `import.meta.vitest` blocks (test
 * fixtures, not real chrome) via a single vitest-start cutoff per file. A root that doesn't resolve to a
 * real directory is a hard failure (misconfiguration), not a silently-skipped scan. */
function collectChromeI18nHits(repoRootPath: string): Map<string, ChromeI18nHit[]> {
  const hitsByFile = new Map<string, ChromeI18nHit[]>();
  const record = (file: string, hit: ChromeI18nHit): void => {
    const bucket = hitsByFile.get(file) ?? [];
    bucket.push(hit);
    hitsByFile.set(file, bucket);
  };

  const scanFile = (full: string, rel: string): void => {
    const content = readFileSync(full, "utf8");
    const lines = content.split("\n");
    // 🧪️ `import.meta.vitest` blocks are always the last top-level statement in a chrome file (in-file
    // vitest convention) — once seen, everything after is test fixture content, not real chrome.
    const vitestStart = lines.findIndex((line) => CHROME_I18N_VITEST_START_RE.test(line));
    const scannable = vitestStart === -1 ? lines : lines.slice(0, vitestStart);
    const boundaryRanges = findChromeI18nBoundaryRanges(scannable);
    const isInsideBoundary = (index: number): boolean => boundaryRanges.some(([start, end]) => index >= start && index <= end);
    for (let i = 0; i < scannable.length; i++) {
      const line = scannable[i]!;
      if (line.includes(CHROME_I18N_IGNORE_MARKER)) {
        continue;
      }
      if (CHROME_I18N_FALLBACK_RE.test(line)) {
        record(rel, { file: rel, line: i + 1, kind: "fallback-literal", text: line.trim() });
      }
      if (CHROME_I18N_JSX_TEXT_RE.test(line)) {
        record(rel, { file: rel, line: i + 1, kind: "bare-jsx-text", text: line.trim() });
      }
      if (CHROME_I18N_UI_DATA_LABEL_LITERAL_RE.test(line)) {
        record(rel, { file: rel, line: i + 1, kind: "ui-data-label-literal", text: line.trim() });
      }
      if (CHROME_I18N_AS_UI_LABEL_RE.test(line) && !isInsideBoundary(i)) {
        record(rel, { file: rel, line: i + 1, kind: "as-ui-label-outside-boundary", text: line.trim() });
      }
    }
  };

  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (entry.isDirectory()) {
        if (CHROME_I18N_SKIP_DIRS.has(entry.name)) {
          continue;
        }
        walk(join(dir, entry.name));
        continue;
      }
      if (!/\.(ts|tsx)$/.test(entry.name)) {
        continue;
      }
      // 🛠️ Build-tooling scripts (`📜️script.ts`) are never rendered chrome — this repo's CLAUDE.md
      // mandates every bundle keep exactly one, so excluding the fixed basename is precise, not fuzzy.
      if (entry.name === "📜️script.ts") {
        continue;
      }
      // 🧪️ Dedicated test files (as opposed to an in-file `import.meta.vitest` block, already handled
      // above) assert against rendered markup — their string literals are expectations about chrome
      // text, not chrome text itself, so scanning them produces pure false positives.
      if (/\.test\.(ts|tsx)$/.test(entry.name)) {
        continue;
      }
      const full = join(dir, entry.name);
      scanFile(full, full.slice(repoRootPath.length + 1));
    }
  };

  for (const root of CHROME_I18N_SCANNED_ROOTS) {
    // 🚫️ Unlike the pre-rewrite version, a root that fails to resolve throws here (readdirSync inside
    // `walk` raises ENOENT) rather than being swallowed by a bare `catch { continue }` — that silent
    // skip is exactly the bug that let this scan pass vacuously for months.
    walk(join(repoRootPath, root));
  }
  return hitsByFile;
}

/** 🚫️ Fails when a scanned chrome file has an untranslated-English hit outside the allowlist, or when the
 * allowlist carries a stale entry for a file that no longer has any hits. */
class CheckChromeI18nScript extends BundleScript {
  run(): void {
    const hitsByFile = collectChromeI18nHits(this.repoRoot);
    const allowlist = new Set(CHROME_I18N_ALLOWLIST);
    const failures: string[] = [];

    for (const [file, hits] of hitsByFile) {
      if (allowlist.has(file)) {
        continue;
      }
      for (const hit of hits) {
        failures.push(`${hit.file}:${hit.line} [${hit.kind}] ${hit.text}`);
      }
    }
    for (const file of allowlist) {
      if (!hitsByFile.has(file)) {
        failures.push(`${file} [stale-allowlist-entry] listed in CHROME_I18N_ALLOWLIST but has zero hits — remove it`);
      }
    }

    if (failures.length === 0) {
      console.log(`framework/ui/js/react: no chrome i18n violations (${allowlist.size} allowlisted files)`);
      return;
    }
    console.error(`framework/ui/js/react: found ${failures.length} chrome i18n violation(s):`);
    for (const failure of failures.slice(0, 120)) {
      console.error(`  ${failure}`);
    }
    if (failures.length > 120) {
      console.error(`  … and ${failures.length - 120} more`);
    }
    process.exit(1);
  }
}
//#endregion 🌐️chrome-i18n-lint

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("lint", LintScript)
  .register("test", TestScript)
  .register("typecheck", TypecheckScript)
  .register("check-ui-primitives", CheckUiPrimitivesScript)
  .register("check-chrome-i18n", CheckChromeI18nScript);

await runBundleScriptMain(router, import.meta.url);
