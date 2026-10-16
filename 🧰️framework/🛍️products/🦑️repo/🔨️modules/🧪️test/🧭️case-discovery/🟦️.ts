/** 🧭️ Test case discovery
 *
 * Walks the repository for test cases, resolves the collection indexes of a path and finds the repository root.
 */

//#region 🔌️Adapters
import { type Implementation } from "../../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { existsSync, lstatSync, readdirSync } from "node:fs";
import { basename, dirname, join, relative, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { findWorkspaceRoot } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { implementationOfAdapterKey, isExcludedTestPath, testAdapterFilenames, testFeatureFilename, testTaxonomy, type TestTaxonomy } from "../🗂️taxonomy/🟦️.ts";
import { digest } from "../🔢️digests/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔍️Discovery
/** 🔍️ One discovered test case: its owner, slug, feature, adapters and fixtures. */
export type DiscoveredCase = Readonly<{
  owner: string;
  ownerName: string;
  case: string;
  caseDir: string;
  featurePath: string;
  adapters: Readonly<Partial<Record<Implementation, string>>>;
  sharedFixtureDir: string | null;
  projectName: string;
}>;

export const SKIP_DIR_NAMES = new Set(["node_modules", ".git", "target", "dist", "📤️dist", "build", "out", "storybook-static", ".venv", "__pycache__", "obj", "bin", "🗑️generated"]);

export function walkDirectories(root: string, onDir: (absDir: string, relDir: string) => "enter" | "skip"): void {
  const stack: string[] = [root];
  while (stack.length > 0) {
    const dir = stack.pop()!;
    let entries: string[];
    try {
      entries = readdirSync(dir);
    } catch {
      continue;
    }
    for (const name of entries) {
      const abs = join(dir, name);
      let st;
      try {
        st = lstatSync(abs);
      } catch {
        continue;
      }
      if (!st.isDirectory() || st.isSymbolicLink()) continue;
      if (SKIP_DIR_NAMES.has(name)) continue;
      if (onDir(abs, relative(root, abs).split(sep).join("/")) === "enter") stack.push(abs);
    }
  }
}

/** 🏷️ Deterministic identity of one case — its owner's Nx project name (`test-<slug>-<hash>`, see `./🟨️.mjs`) followed by the case slug; stable across
 * platforms, safe for the CLI, and the key of the case's work, result and host directories. */
export function testProjectName(ownerRel: string, caseSlug: string): string {
  const slug = ownerRel
    .split("/")
    .map((segment) => segment.replace(/[^a-zA-Z0-9]+/g, "").toLowerCase())
    .filter(Boolean)
    .join("-");
  return `test-${slug || "root"}-${digest(ownerRel).slice(0, 6)}-${caseSlug}`;
}

/**
 * 🔍️ Walks every non-excluded path for `**​/🧪️tests/<case>/component.feature`. The owner is the
 * directory that holds `🧪️tests`; a case found under `📦️packages/<lang>` is still returned so the
 * taxonomy policy can report it rather than silently dropping the evidence.
 */
export function discoverTestCases(repoRoot: string): DiscoveredCase[] {
  const taxonomy = testTaxonomy(repoRoot);
  const featureFilename = testFeatureFilename(taxonomy);
  const adapterFilenames = testAdapterFilenames(taxonomy);
  const found: DiscoveredCase[] = [];
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel) || testFixtureCollectionIndex(rel, taxonomy) >= 0) return "skip";
    if (testCollectionIndexes(rel, taxonomy.testsDirName, taxonomy).at(-1) !== rel.split("/").length - 1) return "enter";
    const ownerAbs = dirname(abs);
    const ownerRel = relative(repoRoot, ownerAbs).split(sep).join("/") || ".";
    for (const entry of readdirSync(abs)) {
      if (entry === taxonomy.testFixturesDirName) continue;
      const caseDir = join(abs, entry);
      let st;
      try {
        st = lstatSync(caseDir);
      } catch {
        continue;
      }
      if (!st.isDirectory()) continue;
      const featurePath = join(caseDir, featureFilename);
      if (!existsSync(featurePath)) continue;
      const adapters: Partial<Record<Implementation, string>> = {};
      for (const [key, filename] of Object.entries(adapterFilenames)) {
        const impl = implementationOfAdapterKey(repoRoot, key);
        if (!impl) continue;
        const adapterPath = join(caseDir, filename);
        if (existsSync(adapterPath)) adapters[impl] = relative(repoRoot, adapterPath).split(sep).join("/");
      }
      const sharedFixtureDir = join(ownerAbs, taxonomy.testsDirName, taxonomy.testFixturesDirName);
      found.push({
        owner: ownerRel,
        ownerName: basename(ownerAbs),
        case: entry,
        caseDir: relative(repoRoot, caseDir).split(sep).join("/"),
        featurePath: relative(repoRoot, featurePath).split(sep).join("/"),
        adapters,
        sharedFixtureDir: existsSync(sharedFixtureDir) ? relative(repoRoot, sharedFixtureDir).split(sep).join("/") : null,
        projectName: testProjectName(ownerRel, entry),
      });
    }
    return "skip";
  });
  return found.sort((a, b) => a.projectName.localeCompare(b.projectName));
}
//#endregion 🔍️Discovery

//#region 🧾️Contract
/** 🧱️ Names only actual collections below the declared module boundary. */
export function testCollectionIndexes(path: string, name: string, taxonomy: TestTaxonomy): number[] {
  const segments = path.replaceAll("\\", "/").split("/");
  return segments.flatMap((segment, index) => segment === name && segments[index - 1] !== taxonomy.schemaScopeOwnerLevels.moduleMemberDirName ? [index] : []);
}

/** 🧭️ Keeps source-shaped samples within their actual fixture collection. */
export function testFixtureCollectionIndex(path: string, taxonomy: TestTaxonomy): number {
  return testCollectionIndexes(path, taxonomy.testFixturesDirName, taxonomy)[0] ?? -1;
}
//#endregion 🧾️Contract

//#region 🧭️Root
/** 📁️ Repository root, resolved from this package's own location. */
export function repoRootFromHere(): string {
  return findWorkspaceRoot(dirname(fileURLToPath(import.meta.url)));
}
//#endregion 🧭️Root
