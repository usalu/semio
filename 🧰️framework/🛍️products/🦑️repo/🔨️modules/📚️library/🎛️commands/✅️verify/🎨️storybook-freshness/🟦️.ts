/** 🖌️ `verify storybook-freshness` checks: every Storybook scope root and story glob resolves on disk, and every app Tailwind entry stylesheet resolves its relative imports and reaches the shared UI stylesheet.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🟦️.ts
 */
import { existsSync, readFileSync, readdirSync, type Dirent } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import {sayVerifyV1} from "../📖️texts/🟦️.ts";

//#region 🔖️VerifyScript
/** 📖️ Every `StoryScope.sourceRoots`/`storyGlobs` entry across `.storybook/📖️stories/🧭️coordination/🟦️.ts`'s `STORY_SCOPES`
 * (both `HAND_CURATED_SCOPES` and the package-catalog-derived `GENERATED_SCOPES`) must resolve to a real
 * on-disk path — catches exactly the "stale de-emojified sourceRoot" class of bug the W0 finding in
 * `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE/📋️master.md` flagged (a plugin/framework
 * dir moves mid-migration and a hand-written scope entry silently keeps pointing at the old location).
 * `GENERATED_SCOPES` entries can't go stale this way (they're re-derived from disk on every import), but
 * a package's own opt-in `sourceRoots` value CAN still name a subdir that no longer exists — checked the
 * same way. See `26/08/06/GENERATED-STORYBOOK-SCOPES-AND-STORIES-FROM-PACKAGE-CATALOG`. */
export async function checkStorybookFreshness(repoRoot: string): Promise<void> {
  const { STORY_SCOPES } = await import("../../../../../../../../.storybook/📖️stories/🧭️coordination/🟦️.ts");
  const offenders: string[] = [];
  for (const scope of STORY_SCOPES) {
    for (const root of scope.sourceRoots) {
      if (!existsSync(join(repoRoot, root))) offenders.push(`scope ${JSON.stringify(scope.id)}: sourceRoot ${JSON.stringify(root)} does not exist`);
    }
    for (const glob of scope.storyGlobs ?? []) {
      const literalPrefix = glob.split(/[*{]/)[0] ?? glob;
      const literalDir = literalPrefix.endsWith("/") ? literalPrefix.slice(0, -1) : dirname(literalPrefix);
      const resolved = literalDir.startsWith("..") || literalDir.startsWith(".") ? resolve(join(repoRoot, ".storybook"), literalDir) : join(repoRoot, literalDir);
      if (!existsSync(resolved)) offenders.push(`scope ${JSON.stringify(scope.id)}: storyGlob ${JSON.stringify(glob)} has no matching directory (resolved ${JSON.stringify(relative(repoRoot, resolved))})`);
    }
  }
  // 🎨️ App-level Tailwind entry CSS files' `@import`/`@source` at-rules are hand-maintained literal paths
  // (Tailwind v4's content-scanning `@source` needs a real filesystem path, not a package specifier) —
  // the same bug class as a de-sandwiched plugin's old `⚡️implementations/<lang>` segment left dangling
  // (see `26/08/06/GENERATED-STORYBOOK-SCOPES-AND-STORIES-FROM-PACKAGE-CATALOG`). Also assert each entry
  // reaches the shared UI stylesheet through its import chain so a future app cannot silently ship a
  // stylesheet that never scans `Layout`/`Panel`/`ShellHost` class names.
  checkAppTailwindEntries(repoRoot, offenders);
  if (offenders.length) {
    sayVerifyV1("storybook.scope-tailwind-entry-path-are",{offenders:offenders.length},"error");
    for (const o of offenders) console.error(`  ${o}`);
    sayVerifyV1("storybook.run-check-stories-coordination-ts",{},"error");
    process.exit(1);
  }
}

/** 🎨️ Discovers every app-level Tailwind entry (`globals.css` / `🎨️.css`), validates
 * relative `@import`/`@source` literals resolve on disk, and asserts each entry's import chain reaches
 * the shared UI stylesheet that owns the framework class sources. */
function checkAppTailwindEntries(repoRoot: string, offenders: string[]): void {
  const uiGlobalsRel = findSharedUiGlobalsRel(repoRoot);
  if (!uiGlobalsRel) {
    offenders.push("shared UI react 🎨️.css not found under 🧰️framework/🔨️modules/🖱️ui");
    return;
  }
  const uiGlobalsAbs = join(repoRoot, uiGlobalsRel);
  const entries = listAppTailwindEntries(repoRoot);
  for (const entryAbs of entries) {
    const entryRel = relative(repoRoot, entryAbs);
    const entryDir = dirname(entryAbs);
    const body = readFileSync(entryAbs, "utf8");
    for (const rawLine of body.split("\n")) {
      const match = rawLine.match(/^@(?:import|source)\s+"([^"]+)"/);
      if (!match) continue;
      const literal = match[1]!;
      if (!literal.startsWith(".")) continue;
      const resolved = resolve(entryDir, literal);
      if (!existsSync(resolved)) offenders.push(`${entryRel}: ${JSON.stringify(literal)} does not exist`);
    }
    // Fragment stylesheets (e.g. animate presentation's reveal overrides) only declare `@source` / custom
    // rules and are `@import`ed by a real entry — they must not be forced to import the UI chain themselves.
    const importsShared = /@import\s+"(?:\.\.?\/|@semio-tech\/ui-styling)/.test(body);
    if (importsShared && !cssImportChainReaches(entryAbs, uiGlobalsAbs, new Set())) {
      offenders.push(`${entryRel}: import chain does not reach shared UI stylesheet ${JSON.stringify(uiGlobalsRel)}`);
    }
  }
}

/** 📋️ App / storybook / product Tailwind entry CSS files that must inherit the shared UI sources. */
function listAppTailwindEntries(repoRoot: string): string[] {
  const found: string[] = [];
  const visit = (dir: string, depth: number): void => {
    if (depth > 10) return;
    let entries: Dirent[];
    try {
      entries = readdirSync(dir, { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      if (entry.name === "node_modules" || entry.name === "dist" || entry.name === "target" || entry.name === ".git") continue;
      const full = join(dir, entry.name);
      if (entry.isDirectory()) {
        visit(full, depth + 1);
        continue;
      }
      if (entry.name === "globals.css" || entry.name === "🎨️.css") found.push(full);
    }
  };
  for (const top of [".storybook", "♻️mit-bestand", "✏️s", "🧰️framework"]) {
    const abs = join(repoRoot, top);
    if (existsSync(abs)) visit(abs, 0);
  }
  return found.filter((abs) => {
    const rel = relative(repoRoot, abs).replace(/\\/g, "/");
    // Shared module stylesheets are sources in the chain, not app entries that must import themselves.
    if (rel === "🧰️framework/🔨️modules/🖱️ui/🧵️styles/🎨️.css") return false;
    if (rel.includes("/🎨️styling/")) return false;
    return true;
  });
}

/** 🔎️ Relative path of the shared UI `🎨️.css` from the workspace root. */
function findSharedUiGlobalsRel(repoRoot: string): string | null {
  const candidates = [
    "🧰️framework/🔨️modules/🖱️ui/🧵️styles/🎨️.css",
  ];
  for (const rel of candidates) {
    if (existsSync(join(repoRoot, rel))) return rel;
  }
  return null;
}

/** 🔗️ Walks relative `@import "..."` edges (skipping package specifiers) until `targetAbs` is reached. */
function cssImportChainReaches(fromAbs: string, targetAbs: string, seen: Set<string>): boolean {
  const normalizedFrom = resolve(fromAbs);
  const normalizedTarget = resolve(targetAbs);
  if (normalizedFrom === normalizedTarget) return true;
  if (seen.has(normalizedFrom)) return false;
  seen.add(normalizedFrom);
  if (!existsSync(normalizedFrom)) return false;
  const body = readFileSync(normalizedFrom, "utf8");
  for (const rawLine of body.split("\n")) {
    const match = rawLine.match(/^@import\s+"([^"]+)"/);
    if (!match) continue;
    const literal = match[1]!;
    let nextAbs: string | null = null;
    if (literal.startsWith(".")) {
      nextAbs = resolve(dirname(normalizedFrom), literal);
    } else if (literal.startsWith("@semio-tech/ui-styling")) {
      // Package import of the styling base — the shared UI globals imports this; reaching UI globals is enough.
      continue;
    } else {
      continue;
    }
    if (cssImportChainReaches(nextAbs, normalizedTarget, seen)) return true;
  }
  return false;
}
//#endregion 🔖️VerifyScript
