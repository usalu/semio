import { existsSync, readdirSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";

/** 🎯️ Resolves the actor TypeScript mirror from the repository root. */
export function actorTypegenTarget(repoRoot: string): string {
  return join(repoRoot, "🧰️framework/🔨️modules/🎭️actor/🤖️generated/🎭️actor/🟦️.ts");
}

/** 📋️ Plans the actor mirror and exact stale siblings from one exported byte sequence. */
export function actorTypegenPlan(repoRoot: string, content: Uint8Array) {
  const target = actorTypegenTarget(repoRoot);
  const root = dirname(target);
  const rootPath = relative(repoRoot, root).replaceAll("\\", "/").normalize("NFC");
  const nodes = [
    { bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: rootPath },
    { bytesBase64: Buffer.from(content).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: `${rootPath}/${basename(target).normalize("NFC")}` },
  ].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
  const staleRemovals = (existsSync(root) ? readdirSync(root) : []).filter((name) => name !== basename(target)).map((name) => `${rootPath}/${name.normalize("NFC")}`).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
  return { contractId: "actor-typegen", nodes, schemaVersion: 1, staleRemovals };
}
