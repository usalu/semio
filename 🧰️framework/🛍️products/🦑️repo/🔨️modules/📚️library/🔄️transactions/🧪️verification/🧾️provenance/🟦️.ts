import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { join } from "node:path";

/** 🧭️ Resolves every source and fixture authority that defines one aggregate transaction run. */
export function transactionV2IdentityPaths(repoRoot: string): Record<string, string> {
  const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
  return {
    allocation: join(repoRoot, library, "🔄️transactions/🧪️verification/📁️run-allocation/🟦️.ts"),
    provenance: join(repoRoot, library, "🔄️transactions/🧪️verification/🧾️provenance/🟦️.ts"),
    shards: join(repoRoot, library, "🔄️transactions/🧪️verification/🏃️shard-execution/🟦️.ts"),
    execution: join(repoRoot, library, "🔄️transactions/🧪️verification/📋️orchestration/🟦️.ts"),
    suite: join(repoRoot, library, "🧪️tests/🔄️transaction-v2/🟦️.ts"),
    normalization: join(repoRoot, library, "🧹️normalization/🟦️.ts"),
    discovery: join(repoRoot, library, "🔍️discovery/🟦️.ts"),
    taxonomy: join(repoRoot, library, "🔣️taxonomy.json"),
    ledgerBoundaries: join(repoRoot, library, "🧫️fixtures/📒️transaction-ledger-boundaries/🔣️.json"),
    harness: join(repoRoot, library, "🧫️fixtures/🪢️transaction-harness-retention/🔣️.json"),
  };
}

/** 🪪️ Captures byte identities for all admitted transaction aggregate inputs. */
export function transactionV2Identities(paths: Readonly<Record<string, string>>): Record<string, { path: string; bytes: number; sha256: string }> {
  return Object.fromEntries(
    Object.entries(paths).map(([key, path]) => {
      const bytes = readFileSync(path);
      return [key, { path, bytes: bytes.length, sha256: createHash("sha256").update(bytes).digest("hex") }];
    }),
  );
}

/** 🧾️ Retains one immutable record under the owning transaction run. */
export function retainTransactionV2Record(runRoot: string, kind: string, value: unknown): void {
  const root = join(runRoot, kind);
  mkdirSync(root);
  writeFileSync(join(root, "🔣️.json"), `${JSON.stringify(value, null, 2)}\n`, { flag: "wx" });
}
