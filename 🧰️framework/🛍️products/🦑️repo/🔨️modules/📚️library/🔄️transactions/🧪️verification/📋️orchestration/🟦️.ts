import { copyFileSync, mkdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { transactionV2BundleRoot } from "../📁️run-allocation/🟦️.ts";
import { retainTransactionV2Record, transactionV2Identities, transactionV2IdentityPaths } from "../🧾️provenance/🟦️.ts";
import { runTransactionV2Shards, TRANSACTION_V2_DEFAULT_FILTER_WAVES } from "../🏃️shard-execution/🟦️.ts";

/** 🔄️ Runs the bounded transaction verification aggregate and retains exact source/result identities. */
export async function runTransactionV2(repoRoot: string, segments: string[]): Promise<void> {
  const bun = (
    globalThis as unknown as {
      Bun: { build(options: { entrypoints: string[]; outdir: string; naming: string; target: "bun"; packages: "external" }): Promise<{ success: boolean; logs: unknown[] }> };
    }
  ).Bun;
  const invocationStartedAt = performance.now();
  const startedAt = new Date().toISOString();
  const runId = `${process.pid}-${crypto.randomUUID()}`;
  const bundleRoot = transactionV2BundleRoot(repoRoot, runId);
  const runRoot = dirname(bundleRoot);
  console.error(`[DEBUG] Transaction v2 run owner ${runRoot}`);
  const bundle = join(bundleRoot, "🟦️.test.js");
  const identityPaths = transactionV2IdentityPaths(repoRoot);
  const beforeIdentities = transactionV2Identities(identityPaths);
  retainTransactionV2Record(runRoot, "📷️before", { schemaVersion: 1, runId, runRoot, startedAt, inputs: beforeIdentities });
  const normalizationBundleRoot = join(bundleRoot, "🧹️normalization");
  const normalizationBundle = join(normalizationBundleRoot, "🟦️.js");
  const schemaSnapshot = join(bundleRoot, "🔣️.json");
  copyFileSync(identityPaths.taxonomy!, schemaSnapshot);
  const built = await bun.build({ entrypoints: [identityPaths.suite!], outdir: bundleRoot, naming: "🟦️.test.js", target: "bun", packages: "external" });
  if (!built.success) throw new AggregateError(built.logs, "Transaction v2 aggregate bundle failed");
  mkdirSync(normalizationBundleRoot, { recursive: true });
  const normalizationBuilt = await bun.build({ entrypoints: [identityPaths.normalization!], outdir: normalizationBundleRoot, naming: "🟦️.js", target: "bun", packages: "external" });
  if (!normalizationBuilt.success) throw new AggregateError(normalizationBuilt.logs, "Transaction v2 normalization child bundle failed");
  const defaultFilters = TRANSACTION_V2_DEFAULT_FILTER_WAVES.flat();
  const filterWaves = segments.length > 1 ? [[segments.slice(1).join(" ")]] : TRANSACTION_V2_DEFAULT_FILTER_WAVES;
  const staticTitles = [...readFileSync(identityPaths.suite!, "utf8").matchAll(/\btest(?:\.concurrent)?\("([^"]+)"/gu)].map((match) => match[1]!);
  if (staticTitles.length !== 14 || staticTitles.length + 48 !== 62) throw new Error(`Transaction v2 aggregate manifest count changed: ${staticTitles.length + 48}`);
  for (const title of staticTitles) {
    const selections = defaultFilters.filter((filter) => new RegExp(filter, "u").test(title));
    if (selections.length !== 1) throw new Error(`Transaction v2 static case must be selected exactly once (${selections.length}): ${title}`);
  }
  const result = await runTransactionV2Shards({
    repoRoot,
    bundle,
    bundleRoot,
    normalizationBundle,
    schemaSnapshot,
    runId,
    runRoot,
    filterWaves,
  });
  const golden = JSON.parse(readFileSync(identityPaths.ledgerBoundaries!, "utf8")) as { boundaries: Record<string, unknown> };
  const expected = Object.keys(golden.boundaries).sort();
  let failure = result.failure;
  if (!failure && segments.length === 1 && JSON.stringify(result.boundaries) !== JSON.stringify(expected)) failure = new Error(`Transaction v2 boundary coverage is not exact: ${result.boundaries.length}/${expected.length}`);
  const afterIdentities = transactionV2Identities(identityPaths);
  retainTransactionV2Record(runRoot, "📷️after", { schemaVersion: 1, runId, inputs: afterIdentities });
  retainTransactionV2Record(runRoot, "📊️outcome", {
    schemaVersion: 1,
    runId,
    runRoot,
    startedAt,
    finishedAt: new Date().toISOString(),
    milliseconds: performance.now() - invocationStartedAt,
    unfiltered: segments.length === 1,
    unchangedInputs: JSON.stringify(beforeIdentities) === JSON.stringify(afterIdentities),
    failure: failure?.message ?? null,
    shards: result.outcomes,
    expectedBoundaryCount: expected.length,
    actualBoundaryCount: result.boundaries.length,
    missingBoundaries: expected.filter((key) => !result.boundaries.includes(key)),
    extraBoundaries: result.boundaries.filter((key) => !expected.includes(key)),
    duplicateBoundaries: result.boundaries.filter((key, index) => index > 0 && result.boundaries[index - 1] === key),
  });
  if (failure) throw failure;
}
