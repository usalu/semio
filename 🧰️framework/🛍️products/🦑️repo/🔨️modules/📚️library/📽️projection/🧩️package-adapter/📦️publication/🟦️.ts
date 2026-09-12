import { existsSync, lstatSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { semanticPackageAdapterPreview } from "../../../🔍️discovery/🟦️.ts";
import { canonicalJson } from "../../../🧹️normalization/🟦️.ts";

/** 🧩️ Emits or verifies exactly the schema-owned JCO Cargo library adapter. */
export function runNestedCargoPackageAdapter(repoRoot: string, mode: string): void {
  if (!["preview", "generate", "check"].includes(mode)) throw new Error("Package adapter mode must be preview, generate, or check");
  const adapters = semanticPackageAdapterPreview(repoRoot, "jcoprobe-guest");
  const nodes = adapters.map((entry) => ({ bytesBase64: Buffer.from(entry.content).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: entry.path }));
  if (mode === "preview") { process.stdout.write(canonicalJson({ contractId: "jco-package-adapter", nodes, schemaVersion: 1, staleRemovals: [] }) + "\n"); return; }
  for (const adapter of adapters) {
    const path = join(repoRoot, adapter.path);
    if (!existsSync(join(dirname(dirname(path)), "Cargo.toml"))) throw new Error("Package adapter generation requires its canonical Cargo manifest");
    if (existsSync(path)) {
      if (readFileSync(path, "utf8") !== adapter.content || (lstatSync(path).mode & 0o7777) !== 0o644) throw new Error("Package adapter bytes or mode drift: " + adapter.path);
      continue;
    }
    if (mode === "check") throw new Error("Package adapter is absent: " + adapter.path);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, adapter.content, { flag: "wx", mode: 0o644 });
  }
}
