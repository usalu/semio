import { report } from "../🏃️operation/🟦️.ts";
import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { type RecordStore } from "../../⚡️caching/🗃️records/🟦️.ts";
import type { TaxonomyInventoryOptions, TaxonomyReferenceAdapter } from "../📜️plan-contracts/🟦️.ts";
import { checkCancellation, isExcluded, violation } from "../🏷️file-kind-matching/🟦️.ts";
import type { ReferenceToken } from "../🔤️reference-tokenizers/🟦️.ts";
import { opaqueLeafNames } from "../✂️reference-edits/🟦️.ts";
import { leafContent, taxonomyFixturePayloadPath } from "../🧱️inventory-leaf-stamps/🟦️.ts";
import type { MutableInventoryEntry } from "../📸️repository-capture/🟦️.ts";
import { rustManifestReferenceTokens } from "../🦀️rust-reference-graph/🟦️.ts";
import { lexicalOpaqueReferenceTarget, referencePathIndex, resolveReferenceTokenPath } from "../🧭️reference-resolution/🟦️.ts";
import { type ReferenceDigest, precomputeReferenceDigests, referenceQueryToken, textualPath } from "../🧮️reference-digests/🟦️.ts";
import { storedReferenceAnalysis } from "../🔬️reference-analysis/🟦️.ts";

export function referenceGraph(repoRoot: string, entries: ReadonlyMap<string, MutableInventoryEntry>, taxonomy: LoadedTaxonomy, progress?: TaxonomyInventoryOptions["progress"], cancelFile?: string, store: RecordStore | null = null, workers = 1): void {
  const known = referencePathIndex(entries.keys()), opaqueLeaves = opaqueLeafNames(taxonomy);
  const files = [...entries.values()].filter((entry) => entry.nodeKind === "file" && textualPath(entry.sourcePath) && !taxonomyFixturePayloadPath(entry.sourcePath, taxonomy.discoverySchema) && (entry.size ?? 0) <= 16 * 1024 * 1024);
  if (store) precomputeReferenceDigests(store, repoRoot, files.filter((entry) => !isExcluded(entry.sourcePath, taxonomy)).map((entry) => [entry.sourcePath, entry.contentHash, entry.size ?? 0] as const), opaqueLeaves, workers, progress, cancelFile);
  report(progress, "inventory", "references", 0, files.length);
  for (const [index, entry] of files.entries()) {
    checkCancellation(repoRoot, cancelFile);

    if (isExcluded(entry.sourcePath, taxonomy)) {
      report(progress, "inventory", "references", index + 1, files.length, entry.sourcePath);
      continue;
    }
    let digest: ReferenceDigest, manifest: readonly ReferenceToken[];
    try {
      const content = leafContent(repoRoot, entry.sourcePath, entry.contentHash);
      digest = storedReferenceAnalysis(store, repoRoot, entry.sourcePath, entry.contentHash, opaqueLeaves, content).digest;
      manifest = digest.rustManifest ? rustManifestReferenceTokens(entry.sourcePath, content().text, known, digest.rustInspection) : [];
    } catch {
      report(progress, "inventory", "references", index + 1, files.length, entry.sourcePath);
      continue;
    }
    const resolved = digest.queries.map((query) => resolveReferenceTokenPath(entry.sourcePath, referenceQueryToken(query), known));
    for (const target of resolved) {
      if (!target || !entries.has(target)) continue;
      entry.referencesOut.push(target);
      entries.get(target)?.referencesIn.push(entry.sourcePath);
    }
    for (const [query, adapter, location, values] of digest.opaque) {
      const target = resolved[query];
      if (target && entries.has(target)) continue;
      const opaque = lexicalOpaqueReferenceTarget(entry.sourcePath, { adapter: adapter as TaxonomyReferenceAdapter, structuredLocation: location, start: 0, end: 0, value: values[0]!, targetValues: values }, taxonomy);
      if (opaque) entry.violations.push(violation("opaque-reference-target", entry.sourcePath, `${adapter} ${location} lexically targets excluded ${opaque}`, "warning"));
    }
    for (const token of manifest) {
      if (token.unsupportedReason) continue;
      const target = resolveReferenceTokenPath(entry.sourcePath, token, known);
      if (!target || !entries.has(target)) {
        const opaque = lexicalOpaqueReferenceTarget(entry.sourcePath, token, taxonomy);
        if (opaque) entry.violations.push(violation("opaque-reference-target", entry.sourcePath, `${token.adapter} ${token.structuredLocation} lexically targets excluded ${opaque}`, "warning"));
        continue;
      }
      entry.referencesOut.push(target);
      entries.get(target)?.referencesIn.push(entry.sourcePath);
    }
    report(progress, "inventory", "references", index + 1, files.length, entry.sourcePath);
  }
  checkCancellation(repoRoot, cancelFile);
  store?.flush();
  for (const entry of entries.values()) {
    entry.referencesIn = [...new Set(entry.referencesIn)].sort();
    entry.referencesOut = [...new Set(entry.referencesOut)].sort();
  }
}
