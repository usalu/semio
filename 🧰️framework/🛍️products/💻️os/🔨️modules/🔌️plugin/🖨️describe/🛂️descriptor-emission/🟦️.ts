import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
import { closeSync, lstatSync, mkdtempSync, openSync, readFileSync, readSync, realpathSync, rmSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { createHash } from "node:crypto";
import { devToolingEnv, runCmdStatus } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { captureOwnedCommandV1 } from "../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { verifySemanticDescriptorPairBytesV1 } from "../../📇️registry/✅️catalog-verification/🟦️.ts";
import { ITERATE_COMPONENT_MAX_BYTES } from "../🏷️component-receipt/🟦️.ts";
import { CRATE_NAME, DESCRIPTOR_JSON_FILENAME, DESCRIPTOR_PACK_FILENAME, FRESH_IO_CHUNK_BYTES, publishedDescriptorEmitter } from "../🏗️component-build/🟦️.ts";
import { freshPathIsWithin } from "../🧾️source-epoch/🟦️.ts";
export type DescriptorEmissionRequestV1 = Readonly<{
  component: string;
  artifactRoot: string;
}>;

export type SemanticDescriptorEmissionV1 = Readonly<{
  pluginId: string;
  packageId: string;
  role: "plugin" | "extension";
  version: string;
  componentSha256: string;
  jsonBytes: Uint8Array;
  packBytes: Uint8Array;
  semanticSha256: string;
}>;

export type DescriptorEmissionControlV1 = Readonly<{
  cancelled?: () => boolean;
  deadlineMs?: number;
  checkpoint?: (stage: string) => void;
  signal?: AbortSignal;
}>;

export class DescribeScript extends BundleScript {
  run(segments: string[]): void {
    const bin = join(import.meta.dir, "..", "📦️packages", "🦀️rust", "dist", "build", process.platform === "win32" ? `${CRATE_NAME}.exe` : CRATE_NAME);
    process.exit(runCmdStatus(bin, ["describe", ...segments], { cwd: this.repoRoot, env: devToolingEnv() }));
  }
}

/** ⏱️ Carries the descriptor emission budget into each child process. */
export function remainingDescriptorEmissionBudgetMs(budgetMs: number, elapsedMs: number): number {
  return budgetMs === 0 ? 0 : Math.max(1, budgetMs - elapsedMs);
}

function emissionGuard(control: DescriptorEmissionControlV1, startedAt: number, budgetMs: number, stage: string): void {
  if (control.cancelled?.()) throw new Error(`descriptor emission cancelled at ${stage}`);
  if (budgetMs !== 0 && Date.now() - startedAt > budgetMs) throw new Error(`descriptor emission deadline of ${budgetMs}ms exceeded at ${stage}`);
  control.checkpoint?.(stage);
}

function emissionDirectory(root: string, path: string, label: string): string {
  const exact = resolve(path);
  const info = lstatSync(exact);
  if (info.isSymbolicLink() || !info.isDirectory()) throw new Error(`${label} ${exact} must be a regular non-symlink directory`);
  const real = realpathSync(exact);
  if (!freshPathIsWithin(realpathSync(root), real)) throw new Error(`${label} ${exact} resolves outside ${root}`);
  return real;
}

function emissionArtifact(root: string, path: string, label: string, maximum: number): { readonly path: string; readonly sha256: string } {
  const exact = resolve(path);
  const info = lstatSync(exact);
  if (info.isSymbolicLink() || !info.isFile() || info.size === 0 || info.size > maximum) throw new Error(`${label} ${exact} must be a regular non-symlink file of 1..${maximum} bytes`);
  const real = realpathSync(exact);
  if (!freshPathIsWithin(realpathSync(root), real)) throw new Error(`${label} ${exact} resolves outside the declared artifact root ${root}`);
  const handle = openSync(real, "r");
  const chunk = Buffer.allocUnsafe(FRESH_IO_CHUNK_BYTES);
  const hash = createHash("sha256");
  try {
    let read = 0;
    while (read < info.size) {
      const count = readSync(handle, chunk, 0, Math.min(chunk.byteLength, info.size - read), read);
      if (count === 0) throw new Error(`${label} ${exact} shrank while being hashed`);
      hash.update(chunk.subarray(0, count));
      read += count;
    }
  } finally {
    chunk.fill(0);
    closeSync(handle);
  }
  if (statSync(real).size !== info.size) throw new Error(`${label} ${exact} changed while being hashed`);
  return { path: real, sha256: hash.digest("hex") };
}

/** 🛂️ The semantic descriptor emission contract: runs the published emitter once over ONE component inside `artifactRoot`
 * (a crate's deliverable directory, never the source tree) and returns the canonical semantic `🛂️.descriptor.semio` +
 * `🔣️.json` pair in memory, strict-decoded, compared form against form and refused if it carries any byte identity or a
 * placeholder identity. Nothing is published here: the describe stage decides whether the pair is verified against the
 * committed one, written over it, or kept local (ticket 26/09/23 wave A2). Every stage is cancellable and deadline bounded. */
export async function emitSemanticDescriptorV1(repoRoot: string, request: DescriptorEmissionRequestV1, control: DescriptorEmissionControlV1 = {}): Promise<SemanticDescriptorEmissionV1> {
  const startedAt = Date.now();
  const budgetMs = control.deadlineMs ?? buildBudgetMs();
  emissionGuard(control, startedAt, budgetMs, "validate");
  const artifactRoot = emissionDirectory(repoRoot, request.artifactRoot, "artifact root");
  const component = emissionArtifact(artifactRoot, request.component, "component", ITERATE_COMPONENT_MAX_BYTES);
  emissionGuard(control, startedAt, budgetMs, "emit");
  const staging = mkdtempSync(join(artifactRoot, ".🛂️descriptor-staging-"));
  try {
    const emitter = publishedDescriptorEmitter(repoRoot);
    emissionGuard(control, startedAt, budgetMs, "describe");
    const remaining = remainingDescriptorEmissionBudgetMs(budgetMs, Date.now() - startedAt), signals = [control.signal, remaining === 0 ? undefined : AbortSignal.timeout(remaining)].filter((value): value is AbortSignal => value !== undefined);
    await captureOwnedCommandV1(emitter, ["describe", component.path, "--out", staging], repoRoot, { env: devToolingEnv(), ...(signals.length ? { signal: AbortSignal.any(signals) } : {}) }).catch((error: unknown) => { throw new Error(`descriptor emitter failed: ${error instanceof Error ? error.message : String(error)}`, { cause: error }); });
    emissionGuard(control, startedAt, budgetMs, "verify");
    const packBytes = readFileSync(join(staging, DESCRIPTOR_PACK_FILENAME));
    const jsonBytes = readFileSync(join(staging, DESCRIPTOR_JSON_FILENAME));
    const pair = verifySemanticDescriptorPairBytesV1(jsonBytes, packBytes);
    return { pluginId: pair.pluginId, packageId: pair.packageId, role: pair.role, version: pair.version, componentSha256: component.sha256, jsonBytes, packBytes, semanticSha256: pair.semanticSha256 };
  } finally {
    rmSync(staging, { recursive: true, force: true });
  }
}
