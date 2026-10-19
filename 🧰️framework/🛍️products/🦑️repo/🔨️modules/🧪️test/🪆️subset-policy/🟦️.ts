/** 🪆️ Subset policy
 *
 * Declared subsets, their policies and the surface coordinates of owners and targets.
 */

//#region 🔌️Adapters
import { type SubsetTarget } from "@semio-tech/framework/test/adapter";
import { existsSync, readFileSync } from "node:fs";
import { basename, join } from "node:path";
import { leadingEmojiIdentity } from "@semio-tech/framework/identity/grapheme";
import { isExcludedTestPath, testFilenameForKind, testTaxonomy } from "../🗂️taxonomy/🟦️.ts";
import { walkDirectories } from "../🧭️case-discovery/🟦️.ts";
import { profileDirectoryIsCanonical } from "../🧮️mutation-catalog/🟦️.ts";
import { rawTaxonomy } from "../📐️schema-contracts/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🪆️Subset
/**
 * 🪆️ The SMALLEST semantic subset one mutation owns, plus the selector addressing the entity inside
 * it. There is deliberately no wildcard: an operation that spans subsets declares an explicit typed
 * `compound`, and falling back to the whole artifact is a contract failure rather than a default.
 */


/** 🚫️ Subset ids that name "everything" rather than a semantic scope. */
export const WILDCARD_SUBSET_IDS: readonly string[] = ["*", "any", "all", "unconstrained", ""];

/** 🚫️ Whether a subset id SPELLS a wildcard. Whether it IS one depends on what else the artifact declares — see `isWildcardSubsetFor`. */
export function isWildcardSubset(subset: string): boolean {
  return WILDCARD_SUBSET_IDS.includes(subset.trim().toLowerCase());
}

/**
 * 🪆️ What subsets one artifact/standard actually declares, read from its own
 * `🪆️subsets/<taxonomy filename>` component. Cached, because the gate asks per mutation.
 */
const declaredSubsetsCache = new Map<string, ReadonlyMap<string, readonly string[]>>();

export function declaredSubsets(repoRoot: string): ReadonlyMap<string, readonly string[]> {
  const cached = declaredSubsetsCache.get(repoRoot);
  if (cached !== undefined) return cached;
  const taxonomy = testTaxonomy(repoRoot);
  const filename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  const found = new Map<string, readonly string[]>();
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    if (basename(abs) !== "🪆️subsets") return "enter";
    for (const candidate of [filename, "🔣️.json"]) {
      const manifest = join(abs, candidate);
      if (!existsSync(manifest)) continue;
      try {
        const parsed = JSON.parse(readFileSync(manifest, "utf8")) as { artifact?: string; standard?: string; subsets?: Record<string, unknown> };
        if (typeof parsed.artifact === "string") found.set(`${parsed.artifact}@${parsed.standard ?? ""}`, Object.keys(parsed.subsets ?? {}));
      } catch {
        /* 🧭️A malformed subsets component is the taxonomy phase's finding, not this one's. */
      }
      break;
    }
    return "skip";
  });
  declaredSubsetsCache.set(repoRoot, found);
  return found;
}

/** 🪆️ Whether an artifact has RECORDED that it genuinely has one scope, rather than merely not having been split. */
export function subsetPolicyIsSingle(repoRoot: string, artifact: string, standard: string): boolean {
  return declaredSubsetPolicies(repoRoot).get(`${artifact}@${standard}`) === "single";
}

const declaredSubsetPolicyCache = new Map<string, ReadonlyMap<string, string>>();

/** 🪆️ The `subsetPolicy` each artifact records beside its subsets, if any. */
export function declaredSubsetPolicies(repoRoot: string): ReadonlyMap<string, string> {
  const cached = declaredSubsetPolicyCache.get(repoRoot);
  if (cached !== undefined) return cached;
  const taxonomy = testTaxonomy(repoRoot);
  const filename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  const found = new Map<string, string>();
  walkDirectories(repoRoot, (abs, rel) => {
    if (isExcludedTestPath(repoRoot, rel)) return "skip";
    if (basename(abs) !== "🪆️subsets") return "enter";
    for (const candidate of [filename, "🔣️.json"]) {
      const manifestPath = join(abs, candidate);
      if (!existsSync(manifestPath)) continue;
      try {
        const parsed = JSON.parse(readFileSync(manifestPath, "utf8")) as { artifact?: string; standard?: string; subsetPolicy?: string };
        if (typeof parsed.artifact === "string" && typeof parsed.subsetPolicy === "string") found.set(`${parsed.artifact}@${parsed.standard ?? ""}`, parsed.subsetPolicy);
      } catch {
        /* 🧭️Reported by the taxonomy phase. */
      }
      break;
    }
    return "skip";
  });
  declaredSubsetPolicyCache.set(repoRoot, found);
  return found;
}

/**
 * 🚫️ Whether a subset is a wildcard IN ITS ARTIFACT'S CONTEXT.
 *
 * `✳️any` means two different things depending on what sits beside it. For `s.stdio.step@ap214`, which
 * declares `cc1`…`cc6`, it lumps seven conformance classes into one bucket and hides which of them a
 * mutation was actually exercised against — a genuine wildcard. For the 82 artifacts whose ONLY
 * declared subset is `*`, there is nothing narrower to be scoped to, and calling it a wildcard would
 * demand a split that the format itself does not have. Refusing the spelling unconditionally produced
 * 82 findings nobody could act on, which is how a gate teaches people to ignore it.
 */
export function isWildcardSubsetFor(repoRoot: string, artifact: string, standard: string, subset: string): boolean {
  if (!isWildcardSubset(subset)) return false;
  const siblings = declaredSubsets(repoRoot).get(`${artifact}@${standard}`) ?? [];
  return siblings.filter((candidate) => !isWildcardSubset(candidate)).length > 0;
}

/** 🪆️ Renders a subset target as the stable coordinate every report row and cache key is keyed by. */
export function subsetCoordinate(target: SubsetTarget): string {
  const scope = target.compound !== undefined ? `(${[...target.compound].sort().join("+")})` : target.subset;
  return `${target.artifact}@${target.standard}/${scope}`;
}

/** 🎚️ A subset target plus the state lane it measures; `surface` absent is the document dispatch. */
export type SurfaceTarget = Readonly<Pick<SubsetTarget, "artifact" | "standard" | "subset" | "surface">>;

/** 🎚️ The coordinate of a subset target on one surface: the subset coordinate, then `#<surface>` for a state lane. */
export function surfaceCoordinate(target: SurfaceTarget): string {
  return `${subsetCoordinate(target)}${target.surface === undefined ? "" : `#${target.surface}`}`;
}

/**
 * 🎚️ Why a manifest's `surface` is not a state lane of its own owner, or `null` when it is. The value must be
 * exactly the owner's path below its subset directory, spelled with the taxonomy's own directories:
 * `<subsetSurfaceDir>[/<modesDirName>/<mode>[/<windowsDirName>/<window>]]/<lane>`, where a lane is a directory
 * every surface, mode and window may hold (`surfaceChildDirs` ∩ `modeChildDirs` ∩ `windowChildDirs`).
 */
export function surfaceProblem(repoRoot: string, owner: string, surface: string): string | null {
  const taxonomy = rawTaxonomy(repoRoot) as { subsetSurfaceDirs: string[]; modesDirName: string; windowsDirName: string; surfaceChildDirs: string[]; modeChildDirs: string[]; windowChildDirs: string[] };
  const lanes = taxonomy.surfaceChildDirs.filter((dir) => taxonomy.modeChildDirs.includes(dir) && taxonomy.windowChildDirs.includes(dir));
  const at = owner.lastIndexOf(`/${surface}`);
  const subsetDir = at >= 0 && at + surface.length + 1 === owner.length ? owner.slice(0, at) : null;
  if (subsetDir === null || subsetCoordinatesOfOwner(subsetDir) === null) return `surface ${JSON.stringify(surface)} is not this manifest owner's own path below its subset directory`;
  const segments = surface.split("/");
  const lane = segments.at(-1)!;
  const path = segments.slice(1, -1);
  const shapeOk = taxonomy.subsetSurfaceDirs.includes(segments[0]!) && lanes.includes(lane) && (path.length === 0 || (path.length === 2 && path[0] === taxonomy.modesDirName) || (path.length === 4 && path[0] === taxonomy.modesDirName && path[2] === taxonomy.windowsDirName));
  return shapeOk ? null : `surface ${JSON.stringify(surface)} is not <${taxonomy.subsetSurfaceDirs.join("|")}>[/${taxonomy.modesDirName}/<mode>[/${taxonomy.windowsDirName}/<window>]]/<${lanes.join("|")}>`;
}

/** 🪆️ Reads the standards/subsets coordinates out of an owner path, or `null` when it carries none. */
export function subsetCoordinatesOfOwner(owner: string): { standardDirectoryName: string; subsetDirectoryName: string; standard: string; subset: string } | null {
  const match = owner.match(/\/🏅️standards\/([^/]+)\/🪆️subsets\/([^/]+)$/);
  if (match === null || !profileDirectoryIsCanonical(match[1]!) || !profileDirectoryIsCanonical(match[2]!)) return null;
  return { standardDirectoryName: match[1]!, subsetDirectoryName: match[2]!, standard: leadingEmojiIdentity(match[1]!).rest, subset: leadingEmojiIdentity(match[2]!).rest };
}
//#endregion 🪆️Subset
