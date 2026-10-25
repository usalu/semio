import { git } from "../🌳️git/🟦️.ts";
import { labelPathTokens } from "../🏷️labels/🟦️.ts";
import type { CommitBundleSection } from "../🧾️bundle-model/🟦️.ts";
import { pathsFromNumstatRow, pathUnderPrefixes } from "../📊️metrics/🔀️deltas/🟦️.ts";

function changedPathsInRange(root: string, base: string, head: string): string[] {
  const r = git(root, ["diff", "--name-only", `${base}..${head}`]);
  if (!r.ok || !r.out) return [];
  return r.out.split("\n").filter(Boolean);
}

function longestCommonPathPrefix(paths: string[]): string {
  if (paths.length === 0) return "";
  const split = paths.map((p) => p.split("/"));
  const first = split[0]!;
  let len = 0;
  for (let i = 0; i < first.length; i++) {
    const seg = first[i]!;
    if (split.every((parts) => parts[i] === seg)) len = i + 1;
    else break;
  }
  if (len === 0) return paths[0]!.split("/")[0] ?? "";
  return first.slice(0, len).join("/");
}

/** 📂️Infers repo path prefixes for a bundle label from the revision range (not shown in messages). */
export function inferPathPrefixesForBundleLabel(root: string, base: string, head: string, label: string, assignedPaths?: string[]): string[] {
  const tokens = labelPathTokens(label);
  if (tokens.length === 0) return [];
  const pool = assignedPaths ?? changedPathsInRange(root, base, head);
  const matched = pool.filter((p) => {
    const pl = p.toLowerCase();
    return tokens.every((t) => pl.includes(t));
  });
  if (matched.length === 0) return [];
  const prefix = longestCommonPathPrefix(matched);
  return prefix ? [prefix] : [];
}

function assignPathToBundleIndex(bundles: CommitBundleSection[], path: string): number {
  const pl = path.toLowerCase();
  let best = -1;
  let bestScore = 0;
  for (let i = 0; i < bundles.length; i++) {
    const tokens = labelPathTokens(bundles[i]!.label);
    if (tokens.length === 0) continue;
    if (!tokens.every((t) => pl.includes(t))) continue;
    const score = tokens.length;
    if (score > bestScore) {
      bestScore = score;
      best = i;
    }
  }
  return best;
}

export function assignChangedPathsToBundles(root: string, base: string, head: string, bundles: CommitBundleSection[]): string[][] {
  const paths = changedPathsInRange(root, base, head);
  const assigned = bundles.map(() => [] as string[]);
  for (const p of paths) {
    const bi = assignPathToBundleIndex(bundles, p);
    if (bi >= 0) assigned[bi]!.push(p);
  }
  return assigned;
}

/** 🔨️Prefix set for a bundle: all range paths, inferred roots, and token-matching parents (renames). */
export function buildBundlePathPrefixSets(root: string, base: string, head: string, bundles: CommitBundleSection[]): string[][] {
  const assignments = assignChangedPathsToBundles(root, base, head, bundles);
  return bundles.map((bundle, i) => {
    const assigned = assignments[i] ?? [];
    const prefixes = new Set<string>();
    const tokens = labelPathTokens(bundle.label);
    for (const p of assigned) {
      prefixes.add(p);
      if (tokens.length === 0) continue;
      const segments = p.split("/");
      let acc = "";
      for (const seg of segments) {
        acc = acc ? `${acc}/${seg}` : seg;
        const pl = acc.toLowerCase();
        if (tokens.every((t) => pl.includes(t))) prefixes.add(acc);
      }
    }
    return [...prefixes];
  });
}

/** 🗂️Whether a path belongs to a bundle (assigned path prefixes, else label token scoring). */
export function pathMatchesBundleIndex(path: string, bundleIndex: number, prefixSets: string[][], bundles: CommitBundleSection[]): boolean {
  const prefixes = prefixSets[bundleIndex] ?? [];
  if (prefixes.length > 0) return pathUnderPrefixes(path, prefixes);
  return assignPathToBundleIndex(bundles, path) === bundleIndex;
}

/** 🔎️Bundle indices that own a numstat row (0 or 1 after validation). */
export function resolveBundleIndicesForNumstatRow(pathField: string, prefixSets: string[][], bundles: CommitBundleSection[]): number[] {
  const matched = new Set<number>();
  for (const path of pathsFromNumstatRow(pathField)) {
    for (let bi = 0; bi < bundles.length; bi++) {
      if (pathMatchesBundleIndex(path, bi, prefixSets, bundles)) matched.add(bi);
    }
  }
  return [...matched];
}
