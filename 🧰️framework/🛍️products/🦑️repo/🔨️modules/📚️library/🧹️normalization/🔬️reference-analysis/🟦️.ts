import { sha256 } from "../📁️input/🟦️.ts";
import { readFileSync } from "node:fs";
import { recordKey, type RecordStore } from "../../⚡️caching/🗃️records/🟦️.ts";
import { absolutePath } from "../🏷️file-kind-matching/🟦️.ts";
import type { ReferenceToken } from "../🔤️reference-tokenizers/🟦️.ts";
import { type ReferenceReachSignature, referenceAdapter, unsupportedReferenceTokens } from "../✂️reference-edits/🟦️.ts";
import { type ReferencePathIndex, type StandaloneReferenceTokens, indexedReferenceTokens, rustReferenceInterpretationCovers, rustStandaloneTokens } from "../🦀️rust-reference-graph/🟦️.ts";
import { addReferenceTokenReach, opaqueReferenceCandidate } from "../🧭️reference-resolution/🟦️.ts";
import { REFERENCE_DIGEST_NAMESPACE, type ReferenceAnalysis, type ReferenceDigest, referenceTokenQuery, referenceTokens } from "../🧮️reference-digests/🟦️.ts";

/** 🧱️ The index-free part of {@link referenceTokens}: a pure function of path and content, so it can be stored by content digest. */
function standaloneReferenceTokens(path: string, content: string): StandaloneReferenceTokens {
  if (path.toLowerCase().endsWith(".rs")) return rustStandaloneTokens(content);
  const tokens = referenceTokens(path, content);
  return { tokens, rustManifestAt: tokens.length, rustManifest: false };
}

/** 🧮️ Computes one file's {@link ReferenceDigest} (a pure function of path, content, repository root and the opaque leaf names) together
 * with the tokens it summarizes. */
function referenceAnalysis(path: string, content: string, repoRoot: string, opaqueLeaves: readonly string[]): Required<ReferenceAnalysis> {
  const standalone = standaloneReferenceTokens(path, content), queries: string[][] = [], queryIndexes = new Map<string, number>(), opaque: [number, string, string, string[]][] = [];
  const unsupportedQueries = new Map<string, string[]>();
  const supported: ReferenceReachSignature = { leaves: new Set(), nullLeaf: false, python: false }, unsupported: ReferenceReachSignature = { leaves: new Set(), nullLeaf: false, python: false };
  for (const token of standalone.tokens) {
    addReferenceTokenReach(supported, token, repoRoot);
    const query = referenceTokenQuery(token), identity = JSON.stringify(query);
    if (token.unsupportedReason) { unsupportedQueries.set(identity, query); continue; }
    let index = queryIndexes.get(identity);
    if (index === undefined) { queryIndexes.set(identity, index = queries.length); queries.push(query); }
    const values = token.targetValues ?? [token.value];
    if (values.some((value) => opaqueReferenceCandidate(value, opaqueLeaves))) opaque.push([index, token.adapter, token.structuredLocation, [...values]]);
  }
  const unsupportedTokens = unsupportedReferenceTokens(content, referenceAdapter(path));
  for (const token of unsupportedTokens) addReferenceTokenReach(unsupported, token, repoRoot);
  const plain = (signature: ReferenceReachSignature) => ({ leaves: [...signature.leaves], nullLeaf: signature.nullLeaf, python: signature.python });
  return { digest: { queries, unsupportedQueries: [...unsupportedQueries.values()], opaque, supported: plain(supported), unsupported: plain(unsupported), rustManifest: standalone.rustManifest, ...(standalone.rustInspection ? { rustInspection: standalone.rustInspection } : {}) }, standalone, unsupported: unsupportedTokens };
}

/** 📦️ {@link referenceAnalysis} through the store, keyed by path, content digest and opaque leaves: a hit carries only the digest. `content`
 * yields the text and whether its bytes still have that digest (a digest computed from other bytes is returned but never stored). */
export function storedReferenceAnalysis(store: RecordStore | null, repoRoot: string, path: string, contentHash: string, opaqueLeaves: readonly string[], content: () => Readonly<{ text: string; matches: boolean }>): ReferenceAnalysis {
  const key = recordKey(REFERENCE_DIGEST_NAMESPACE, repoRoot, path, contentHash, ...opaqueLeaves), hit = store?.get(key);
  if (hit !== undefined) {
    try { return { digest: JSON.parse(Buffer.from(hit).toString("utf8")) as ReferenceDigest }; } catch {}
  }
  const source = content(), analysis = referenceAnalysis(path, source.text, repoRoot, opaqueLeaves);
  if (store && source.matches) store.put(key, Buffer.from(JSON.stringify(analysis.digest), "utf8"));
  return analysis;
}

/** 🧵️ The `key<TAB>digest` record lines of the given leaves whose bytes still have their inventoried digest; the digest worker's entry. */
export function referenceDigestRecordLines(repoRoot: string, rows: readonly (readonly [path: string, contentHash: string])[], opaqueLeaves: readonly string[], advance: () => void): string {
  const lines: string[] = [];
  for (const [path, contentHash] of rows) {
    try {
      const bytes = readFileSync(absolutePath(repoRoot, path));
      if (sha256(bytes) === contentHash) lines.push(`${recordKey(REFERENCE_DIGEST_NAMESPACE, repoRoot, path, contentHash, ...opaqueLeaves)}\t${JSON.stringify(referenceAnalysis(path, bytes.toString("utf8"), repoRoot, opaqueLeaves).digest)}`);
    } catch {}
    advance();
  }
  return lines.join("\n");
}

export function referenceTokensIncludingUnsupported(path: string, content: string, index: ReferencePathIndex, analysis?: ReferenceAnalysis): readonly ReferenceToken[] {
  const supported = analysis?.standalone ? indexedReferenceTokens(path, content, index, analysis.standalone) : referenceTokens(path, content, index);
  return [...supported, ...(analysis?.unsupported ?? unsupportedReferenceTokens(content, referenceAdapter(path))).filter((candidate) => candidate.adapter !== "rust" || !supported.some((token) => rustReferenceInterpretationCovers(token, candidate)))];
}
