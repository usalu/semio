import type { ScriptInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readFileSync, readdirSync } from "node:fs";
import { join as systemJoin } from "node:path";
import { devToolingEnv } from "../🏃️process/🌿️environment/🟦️.ts";
import { runCmd } from "../🏃️process/🟦️.ts";
import { defineLint, type BundleLinter } from "../🚨️policy/🧩️linters/🟦️.ts";
import { getWorkspaceRoot } from "../🗂️workspaces/🟦️.ts";
import { dependencyBoundaryBreachesForBundleDir } from "../🚨️policy/🛡️dependency-boundary/🟦️.ts";
import { runtimeExecutableV1 } from "../../../../../🔨️modules/⚙️runtime/🏷️identity/🟦️.ts";

const join = (...parts: string[]): string => systemJoin(...parts).replaceAll("\\", "/");
export const policy = defineLint("@semio-tech/repo-ui", (_l: BundleLinter) => dependencyBoundaryBreachesForBundleDir(getWorkspaceRoot(), "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript"));
const storybookEnv = (extra: Record<string, string | undefined> = {}) =>
  devToolingEnv({
    WATCHPACK_POLLING: process.env.WATCHPACK_POLLING ?? "true",
    CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "true",
    ...extra,
  });
export const UI_PRIMITIVES_ALLOWLIST: readonly string[] = [
  "♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx",
  "♻️mit-bestand/🧺️demonstrator/🟦️.tsx",
  "🏢️semio-tech/🎡️play/⚛️play-card.tsx",
  "🏢️semio-tech/🎡️play/🟦️.tsx",
] as const;
const UI_PRIMITIVES_SKIP_DIRS = new Set(["node_modules", "dist", "target", ".🧬semio", ".🧬semio", "storybook-static", ".claude", ".git"]);
const UI_PRIMITIVES_FRAMEWORK_PREFIX = "🧰️framework/";
const UI_PRIMITIVES_STORYBOOK_PREFIX = ".storybook";
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
export const CHROME_I18N_ALLOWLIST: readonly string[] = [
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/👥️presence-scope/🌐️browser/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/📥️intake/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🪢️handle-scope/🟦️.ts",
];
const CHROME_I18N_SCANNED_ROOTS = [
  "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements",
  "♻️mit-bestand/🧺️demonstrator",
  "🏢️semio-tech/🎡️play",
] as const;
const CHROME_I18N_SKIP_DIRS = new Set(["node_modules", "dist", "dist-staging", "target", ".🧬semio", ".🧬semio", "storybook-static", ".claude", ".git", "🧪️tests", "📖️stories"]);
const CHROME_I18N_FALLBACK_RE = /(\?\?|\|\|)\s*"[A-Z][^"]*"/;
const CHROME_I18N_JSX_TEXT_RE = />[A-Z][a-zA-Z]+(?: [a-zA-Z…]+)*<\//;
const CHROME_I18N_VITEST_START_RE = /if\s*\(\s*import\.meta\.vitest\s*\)/;
const CHROME_I18N_LOCALE_INVARIANT_VALUES = new Set(["Leibniz Universität Hannover", "Universität der Künste Berlin"]);
const CHROME_I18N_IGNORE_MARKER = "chrome-i18n-allow";
const CHROME_I18N_BOUNDARY_FN_RE = /\b(export\s+)?(function|const)\s+(uiDataLabel|useLabel|useIdLabel|resolveManifestLabel|wireLabel)\b/;
const CHROME_I18N_UI_DATA_LABEL_LITERAL_RE = /\buiDataLabel\(\s*"[A-Z][^"]*"/;
const CHROME_I18N_AS_UI_LABEL_RE = /\bas\s+UiLabel\b/;
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

/** 🏃️ Runs the `DevScript` command under the caller's roots and invocation. */
export async function runDevScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  runCmd(runtimeExecutableV1(), ["./📜️script.ts", "dev", "storybook", "ui", ...segments], { cwd: context.repoRoot, env: storybookEnv() });
}

/** 🏃️ Runs the `BuildScript` command under the caller's roots and invocation. */
export async function runBuildScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  runCmd(runtimeExecutableV1(), ["./📜️script.ts", "build", "storybook", ...segments], { cwd: context.repoRoot, env: storybookEnv({ STORYBOOK_SCOPE: "ui" }) });
}

/** 🏃️ Runs the `CheckUiPrimitivesScript` command under the caller's roots and invocation. */
export function runCheckUiPrimitivesScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>): void {
  const hitsByFile = collectUiPrimitivesHits(context.repoRoot);
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

/** 🏃️ Runs the `CheckChromeI18nScript` command under the caller's roots and invocation. */
export function runCheckChromeI18nScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>): void {
  const hitsByFile = collectChromeI18nHits(context.repoRoot);
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
