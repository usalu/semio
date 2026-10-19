/** ⚡️ Test output cache
 *
 * Cache roots, per-test directories and the output markers that make a directory safe to clean.
 */

//#region 🔌️Adapters
import { existsSync, mkdirSync, readFileSync, realpathSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { repositoryTestArtifactRootV1, repositoryTestArtifactPathV1 } from "../💾️storage/🟦️.ts";
import { testFilenameForKind, testTaxonomy } from "../🗂️taxonomy/🟦️.ts";
import { digest } from "../🔢️digests/🟦️.ts";
//#endregion 🔌️Adapters

//#region ⚡️Cache
/** ⚡️ Root of every generated test artifact. Nothing outside this tree is ever written or deleted. */
export function testCacheRoot(repoRoot: string): string {
  return repositoryTestArtifactRootV1(repoRoot);
}

/** ⚡️ One of the six generated output roots (`work`, `hosts`, `oracles`, `results`, `diffs`, `reports`). */
export function testCacheDir(repoRoot: string, child: string): string {
  const taxonomy = testTaxonomy(repoRoot);
  if (!taxonomy.testOutputChildDirs.includes(child)) throw new Error(`unknown test cache child dir ${JSON.stringify(child)}`);
  const scope = process.env.SEMIO_TEST_OUTPUT_SCOPE;
  if (scope !== undefined) {
    const segments = scope.split("/");
    if (segments.length !== 2 || segments.some((segment) => segment === "" || segment === "." || segment === ".." || segment.includes("\0") || segment.includes("\\") || segment.includes(":"))) throw new Error(`invalid test output scope ${JSON.stringify(scope)}`);
    return repositoryTestArtifactPathV1(repoRoot,child);
  }
  return repositoryTestArtifactPathV1(repoRoot,child);
}

/** 🏷️ Agent-scoped scratch root for non-Cargo per-agent state (e.g. Python's pycache); Cargo output lives in the one shared cargo cache instead. */
export function agentCacheRoot(repoRoot: string, agentId = process.env.SEMIO_AGENT_ID ?? "local"): string {
  return repositoryTestArtifactPathV1(repoRoot,"agents",digest(agentId).slice(0,24));
}

/** 🧾️ Ownership marker written into every generated output root; only marked trees are deletable. */
export type OutputMarker = Readonly<{ kind: string; testId: string; cacheKey: string }>;

/** 🧾️ Creates a generated directory and stamps it with its ownership marker. */
export function markOutputDir(repoRoot: string, absDir: string, marker: Omit<OutputMarker, "kind">): void {
  const taxonomy = testTaxonomy(repoRoot);
  const root = resolve(testCacheRoot(repoRoot));
  if (!resolve(absDir).startsWith(root + sep) && resolve(absDir) !== root) throw new Error(`refusing to mark ${absDir}: outside the test cache root`);
  repositoryTestArtifactPathV1(repoRoot,relative(root,absDir));
  let rootAncestor = root;
  while (!existsSync(rootAncestor)) rootAncestor = dirname(rootAncestor);
  let targetAncestor = resolve(absDir);
  while (!existsSync(targetAncestor)) targetAncestor = dirname(targetAncestor);
  const physicalRoot = resolve(realpathSync(rootAncestor), relative(rootAncestor, root));
  const physicalTarget = resolve(realpathSync(targetAncestor), relative(targetAncestor, resolve(absDir)));
  if (physicalTarget !== physicalRoot && !physicalTarget.startsWith(physicalRoot + sep)) throw new Error(`refusing to mark ${absDir}: outside the physical test artifact root`);
  const markerPath = join(absDir, testFilenameForKind(taxonomy, taxonomy.testOutputMarkerFileKindId));
  if (existsSync(markerPath)) {
    const previous = readOutputMarker(repoRoot, absDir);
    if (previous === null || previous.testId !== marker.testId) throw new Error(`refusing to mark ${absDir}: different output ownership`);
  }
  mkdirSync(absDir, { recursive: true });
  writeFileSync(markerPath, `${JSON.stringify({ kind: taxonomy.testOutputMarkerKind, ...marker }, null, 2)}\n`);
}

/** 🧾️ Reads a directory's ownership marker, or `null` when it carries none. */
export function readOutputMarker(repoRoot: string, absDir: string): OutputMarker | null {
  const taxonomy = testTaxonomy(repoRoot);
  const path = join(absDir, testFilenameForKind(taxonomy, taxonomy.testOutputMarkerFileKindId));
  if (!existsSync(path)) return null;
  try {
    const parsed = JSON.parse(readFileSync(path, "utf8")) as OutputMarker;
    return parsed.kind === taxonomy.testOutputMarkerKind ? parsed : null;
  } catch {
    return null;
  }
}
//#endregion ⚡️Cache
