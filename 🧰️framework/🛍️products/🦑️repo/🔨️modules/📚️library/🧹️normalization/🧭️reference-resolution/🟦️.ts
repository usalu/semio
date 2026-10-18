import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { normalizeRelative } from "../🛣️path/🟦️.ts";
import { basename, dirname, posix } from "node:path";
import type { TaxonomyInventoryEntry } from "../📜️plan-contracts/🟦️.ts";
import { isExcluded } from "../🏷️file-kind-matching/🟦️.ts";
import type { ReferenceToken } from "../🔤️reference-tokenizers/🟦️.ts";
import { type ReferenceReachSignature, type ReferenceResolutionReach, addUniqueIndex, opaqueLeafNames, referenceLeaf } from "../✂️reference-edits/🟦️.ts";
import { projectedPath } from "../➰️symlink-targets/🟦️.ts";
import { ancestorReferenceCoordinateRoot } from "../🔗️reference-inventory/🟦️.ts";
import { type ReferencePathIndex, splitTokenSuffix } from "../🦀️rust-reference-graph/🟦️.ts";

export function referencePathIndex(paths: Iterable<string>, repoRoot?: string, coordinateRoots: readonly string[] = [], contextPaths?: Iterable<string>, cancelFile?: string, affectedPaths?: ReadonlySet<string>): ReferencePathIndex {
  const exact = new Set<string>();
  const nfc = new Map<string, string | null>();
  const extensionless = new Map<string, string | null>();
  const pythonModule = new Map<string, string | null>();
  for (const path of paths) {
    exact.add(path);
    const normalized = path.normalize("NFC");
    addUniqueIndex(nfc, normalized, path);
    addUniqueIndex(extensionless, normalized.replace(/\.[^/.]+(?:\.[^/.]+)*$/u, ""), path);
    if (!normalized.endsWith(".py")) continue;
    const moduleSegments = (normalized.endsWith("/__init__.py") ? dirname(normalized) : normalized.slice(0, -3)).split("/").filter(Boolean);
    for (let index = 0; index < moduleSegments.length; index++) addUniqueIndex(pythonModule, moduleSegments.slice(index).join("."), path);
  }
  const contexts = [...(contextPaths ?? exact)];
  return { exact, nfc, extensionless, pythonModule, repoRoot, coordinateRoots, coordinateRootSet: new Set(coordinateRoots), coordinateRootByReference: new Map(), contextPaths: contexts, contextPathSet: new Set(contexts), affectedPaths: affectedPaths ?? exact, cancelFile };
}

/** 🧭️ Resolves a reference token to a repository-relative path. A bare single-segment token (no
 * `/` anywhere) tries its same-directory sibling before its root/coordinate-root reading — a
 * `cwd`-relative filename argument reads as "beside me" far more often than "at the repository
 * root", and this order only changes the outcome when both an unrelated root/coordinate-root file
 * and a same-named sibling exist. Every other token shape (containing a `/`, or an explicit
 * absolute/`./`/`../` form) keeps the root-or-coordinate-root-first order. */
const referenceLeafNameSets = new WeakMap<ReferencePathIndex, ReadonlySet<string>>();

/** 🍂️ The final segment of every key {@link resolveReferencePath} can return through its exact, NFC or extensionless lookups, built once per index. */
function referenceLeafNames(index: ReferencePathIndex): ReadonlySet<string> {
  let names = referenceLeafNameSets.get(index);
  if (!names) {
    const leaves = new Set<string>(), add = (key: string): void => { leaves.add(key.slice(key.lastIndexOf("/") + 1)); };
    for (const key of index.exact) add(key);
    for (const key of index.nfc.keys()) add(key);
    for (const key of index.extensionless.keys()) add(key);
    referenceLeafNameSets.set(index, names = leaves);
  }
  return names;
}

function resolveReferencePath(referencePath: string, token: string, index: ReferencePathIndex): string | null {
  const split = splitTokenSuffix(token);
  const absoluteRoot = index.repoRoot?.replaceAll("\\", "/").replace(/\/+$/u, "");
  const absoluteLocal = absoluteRoot && split.path.startsWith(`${absoluteRoot}/`) ? split.path.slice(absoluteRoot.length + 1) : null;
  if (!split.path || absoluteLocal === null && /^(?:[a-z][a-z0-9+.-]*:|#|@|\$|\{)/i.test(split.path) || /[*{}]/.test(split.path)) return null;
  const leaf = referenceLeaf(absoluteLocal ?? split.path);
  if (leaf !== null && !referenceLeafNames(index).has(leaf)) return /^[\w.]+$/.test(split.path) ? index.pythonModule.get(split.path.normalize("NFC")) || null : null;
  const candidates: string[] = [];
  if (!index.coordinateRootByReference.has(referencePath)) index.coordinateRootByReference.set(referencePath, ancestorReferenceCoordinateRoot(referencePath, index.coordinateRootSet));
  const coordinateRoot = index.coordinateRootByReference.get(referencePath);
  const pushRootCandidate = (): void => { try { if (!/^\.\.?\//u.test(split.path)) candidates.push(normalizeRelative(absoluteLocal ?? (coordinateRoot ? `${coordinateRoot}/${split.path.replace(/^\//u, "")}` : split.path.replace(/^\//u, "")))); } catch {} };
  const pushSiblingCandidate = (): void => { try { candidates.push(normalizeRelative(posix.join(dirname(referencePath), split.path))); } catch {} };
  if (absoluteLocal === null && !split.path.includes("/")) { pushSiblingCandidate(); pushRootCandidate(); } else { pushRootCandidate(); pushSiblingCandidate(); }
  for (const candidate of candidates) {
    if (index.exact.has(candidate)) return candidate;
    const comparison = candidate.normalize("NFC");
    const nfc = index.nfc.get(comparison);
    if (nfc) return nfc;
    const extensionless = index.extensionless.get(comparison);
    if (extensionless) return extensionless;
  }
  if (/^[\w.]+$/.test(split.path)) {
    const python = index.pythonModule.get(split.path.normalize("NFC"));
    if (python) return python;
  }
  return null;
}

/** ➕️ Visits one token's reach, mirroring {@link resolveReferencePath}'s value admission and leaf derivation exactly: every final segment
 * a candidate value or physical target names (`null` when normalization can replace it) and whether the value could take the Python route;
 * stops as soon as `visit` returns true and reports whether it did. */
export function visitReferenceTokenReach(token: ReferenceToken, repoRoot: string | undefined, visit: (leaf: string | null, python: boolean) => boolean): boolean {
  if (token.physicalTargets !== undefined) return token.physicalTargets.some((value) => visit(referenceLeaf(value), false));
  const absoluteRoot = repoRoot?.replaceAll("\\", "/").replace(/\/+$/u, "");
  for (const value of token.targetValues ?? [token.value]) {
    const split = splitTokenSuffix(value), absoluteLocal = absoluteRoot && split.path.startsWith(`${absoluteRoot}/`) ? split.path.slice(absoluteRoot.length + 1) : null;
    if (!split.path || absoluteLocal === null && /^(?:[a-z][a-z0-9+.-]*:|#|@|\$|\{)/i.test(split.path) || /[*{}]/.test(split.path)) continue;
    if (visit(referenceLeaf(absoluteLocal ?? split.path), /^[\w.]+$/.test(split.path))) return true;
  }
  return false;
}

/** ➕️ Adds one token's reach to a signature. */
export function addReferenceTokenReach(signature: ReferenceReachSignature, token: ReferenceToken, repoRoot: string | undefined): void {
  visitReferenceTokenReach(token, repoRoot, (leaf, python) => {
    if (leaf === null) signature.nullLeaf = true;
    else signature.leaves.add(leaf);
    signature.python ||= python;
    return false;
  });
}

/** 🎯️ False only when {@link resolveReferenceTokenPath} provably cannot return a member of the reach's target set for this token. */
function referenceTokenMayResolveInto(token: ReferenceToken, reach: ReferenceResolutionReach, index: ReferencePathIndex): boolean {
  return visitReferenceTokenReach(token, index.repoRoot, (leaf, python) => leaf === null || reach.leaves.has(leaf) || python && reach.python);
}

export function resolveReferenceTokenPath(referencePath: string, token: ReferenceToken, index: ReferencePathIndex): string | null {
  if (token.physicalTargets !== undefined) {
    const matches = [...new Set(token.physicalTargets.map((value) => index.exact.has(value) ? value : index.nfc.get(value.normalize("NFC"))).filter((value): value is string => typeof value === "string"))];
    return matches.length === 1 ? matches[0]! : null;
  }
  const matches = [...new Set((token.targetValues ?? [token.value]).map((value) => resolveReferencePath(referencePath, value, index)).filter((value): value is string => value !== null))];
  return matches.length === 1 ? matches[0] ?? null : null;
}

/** 🕳️ The opaque exclusion a token lexically targets from a referencing file outside every exclusion, if any. */
/** 🚪️ True when one token value could name an opaque exclusion: a plain path containing one of the exclusions' last segments. */
export function opaqueReferenceCandidate(value: string, opaqueLeaves: readonly string[]): boolean {
  const path = splitTokenSuffix(value).path;
  if (!path || /^(?:[a-z][a-z0-9+.-]*:|#|@|\$|\{)/iu.test(path) || /[*{}]/u.test(path)) return false;
  const comparable = path.normalize("NFC");
  return opaqueLeaves.some((leaf) => comparable.includes(leaf));
}

export function lexicalOpaqueReferenceTarget(referencePath: string, token: ReferenceToken, taxonomy: LoadedTaxonomy): string | null {
  for (const value of token.targetValues ?? [token.value]) {
    if (!opaqueReferenceCandidate(value, opaqueLeafNames(taxonomy))) continue;
    const path = splitTokenSuffix(value).path;
    const candidates = [path.replace(/^\//u, ""), posix.join(dirname(referencePath), path)];
    for (const candidate of candidates) {
      try {
        const normalized = normalizeRelative(candidate);
        if (isExcluded(normalized, taxonomy)) return normalized;
      } catch {}
    }
  }
  return null;
}

export function rewriteReferenceValue(referencePath: string, oldValue: string, oldTarget: string, newTarget: string, sourceReferencePath = referencePath, repoRoot?: string): string {
  const split = splitTokenSuffix(oldValue);
  const absoluteRoot = repoRoot?.replaceAll("\\", "/").replace(/\/+$/u, "");
  if (absoluteRoot && split.path.startsWith(`${absoluteRoot}/`)) return `${absoluteRoot}/${newTarget}${split.suffix}`;
  if (/^[\w.]+$/.test(split.path) && oldTarget.endsWith(".py")) {
    const modulePath = newTarget.replace(/(?:\/__init__)?\.py$/, "").replaceAll("/", ".");
    return `${modulePath}${split.suffix}`;
  }
  const absoluteStyle = split.path.startsWith("/");
  const relativeStyle = split.path.startsWith(".") || split.path.startsWith("..");
  let localBareStyle = false;
  if (!absoluteStyle && !relativeStyle) {
    try {
      localBareStyle = normalizeRelative(posix.join(dirname(sourceReferencePath), split.path)) === oldTarget;
    } catch {}
  }
  const omittedExtension = !posix.extname(split.path);
  let value = absoluteStyle ? `/${newTarget}` : relativeStyle || localBareStyle ? posix.relative(dirname(referencePath), newTarget) : newTarget;
  if (relativeStyle && !value.startsWith(".")) value = `./${value}`;
  if (omittedExtension) {
    const finalName = basename(newTarget);
    const extensionStart = finalName.indexOf(".");
    const extensionChain = extensionStart <= 0 ? "" : finalName.slice(extensionStart);
    if (extensionChain && value.endsWith(extensionChain)) value = value.slice(0, -extensionChain.length);
  }
  if (oldValue.includes("\\")) value = value.replaceAll("/", "\\");
  return `${value}${split.suffix}`;
}

export function rewriteReferenceToken(referencePath: string, sourceReferencePath: string, token: ReferenceToken, oldTarget: string, newTarget: string, repoRoot?: string, entries: readonly TaxonomyInventoryEntry[] = []): string {
  if (token.rewriteKind === "rust-path-join") {
    const base = token.rewriteData?.sourceBase;
    if (base === undefined) throw new Error("Rust join has no exact physical base");
    let value = posix.relative(projectedPath(base, entries), newTarget);
    if (token.value.startsWith(".") && !value.startsWith(".")) value = `./${value}`;
    return value || ".";
  }
  if (token.rewriteKind === "rust-mod") {
    let relativeTarget = posix.relative(dirname(referencePath), newTarget);
    if (!relativeTarget.startsWith(".")) relativeTarget = `./${relativeTarget}`;
    const indentation = token.rewriteData?.indentation ?? "";
    const declaration = token.rewriteData?.declaration ?? token.value;
    return `#[path = ${JSON.stringify(relativeTarget)}]\n${indentation}${declaration}`;
  }
  if (token.rewriteKind === "python-entrypoint") {
    const targetValue = token.targetValues?.[0] ?? token.value;
    return `${rewriteReferenceValue(referencePath, targetValue, oldTarget, newTarget, sourceReferencePath)}${token.rewriteData?.suffix ?? ""}`;
  }
  if (token.rewriteKind === "path-prefix") return `${token.rewriteData?.prefix ?? ""}${newTarget}${token.rewriteData?.suffix ?? ""}`;
  return rewriteReferenceValue(referencePath, token.value, oldTarget, newTarget, sourceReferencePath, repoRoot);
}
