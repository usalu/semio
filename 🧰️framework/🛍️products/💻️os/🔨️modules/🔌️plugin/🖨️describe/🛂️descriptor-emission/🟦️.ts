import { createHash } from "node:crypto";
import { closeSync, lstatSync, mkdtempSync, openSync, readFileSync, readSync, realpathSync, renameSync, rmSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { BundleScript, buildBudgetMs, devToolingEnv, runCmdStatus } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { verifyDescriptorPairBytesV1 } from "../../📇️registry/✅️catalog-verification/🟦️.ts";
import { DESCRIPTOR_JSON_FILENAME, DESCRIPTOR_PACK_FILENAME, FRESH_COMPONENT_MAX_BYTES, FRESH_IO_CHUNK_BYTES, cargoTargetRoot, ensureBuiltBin } from "../🏗️component-build/🟦️.ts";
import { freshPathIsWithin } from "../🧾️source-epoch/🟦️.ts";
export type DescriptorEmissionRequestV1 = Readonly<{
  rawComponentPath: string;
  extractedCorePath: string;
  ownerRoot: string;
  artifactRoot?: string;
}>;

export type DescriptorEmissionReceiptV1 = Readonly<{
  pluginId: string;
  packageId: string;
  role: "plugin" | "extension";
  version: string;
  ownerRoot: string;
  jsonPath: string;
  packPath: string;
  rawSha256: string;
  coreSha256: string;
  descriptorSha256: string;
  jsonByteLength: number;
  packByteLength: number;
}>;

export type DescriptorEmissionControlV1 = Readonly<{
  cancelled?: () => boolean;
  deadlineMs?: number;
  checkpoint?: (stage: string) => void;
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

/** @emoji 🛂️ The owner descriptor receipt contract: emits `🛂️.descriptor.semio` + `🔣️.json` for
 * `ownerRoot` from ONE decoded descriptor, out of two independently supplied artifacts — the raw
 * `wasm32-wasip2` component and the separately extracted core module. Both are hashed here, in this
 * process, from the exact bytes on disk; the emitter blanks exactly `hashes.descriptorSha256` for its
 * two-pass self hash; both forms are strict-decoded, compared semantically, re-encoded canonically and
 * checked against those two hashes BEFORE a single owner byte moves. Inputs must be regular
 * non-symlink files inside `artifactRoot` (cargo's target root by default, never the source tree) and
 * `ownerRoot` a regular directory inside the repository; every stage is cancellable and deadline
 * bounded, and a failure at any stage leaves the previous owner pair exactly as it was. */
export function emitOwnerDescriptorPairV1(repoRoot: string, request: DescriptorEmissionRequestV1, control: DescriptorEmissionControlV1 = {}): DescriptorEmissionReceiptV1 {
  const startedAt = Date.now();
  const budgetMs = control.deadlineMs ?? buildBudgetMs();
  emissionGuard(control, startedAt, budgetMs, "validate");
  const artifactRoot = emissionDirectory(repoRoot, request.artifactRoot ?? cargoTargetRoot(repoRoot), "artifact root");
  const ownerRoot = emissionDirectory(repoRoot, request.ownerRoot, "owner root");
  const raw = emissionArtifact(artifactRoot, request.rawComponentPath, "raw component", FRESH_COMPONENT_MAX_BYTES);
  const core = emissionArtifact(artifactRoot, request.extractedCorePath, "extracted core module", FRESH_COMPONENT_MAX_BYTES);
  if (raw.path === core.path) throw new Error("raw component and extracted core module are the same file");
  if (raw.sha256 === core.sha256) throw new Error("raw component and extracted core module have the same SHA-256");
  emissionGuard(control, startedAt, budgetMs, "emit");
  const staging = mkdtempSync(join(ownerRoot, ".🛂️descriptor-staging-"));
  try {
    const emitter = ensureBuiltBin(repoRoot, remainingDescriptorEmissionBudgetMs(budgetMs, Date.now() - startedAt));
    emissionGuard(control, startedAt, budgetMs, "describe");
    const status = runCmdStatus(emitter, ["describe", raw.path, "--core", core.path, "--out", staging], { cwd: repoRoot, env: devToolingEnv(), budgetMs: remainingDescriptorEmissionBudgetMs(budgetMs, Date.now() - startedAt) });
    if (status !== 0) throw new Error(`descriptor emitter exited with ${status}`);
    emissionGuard(control, startedAt, budgetMs, "verify");
    const packPath = join(staging, DESCRIPTOR_PACK_FILENAME);
    const jsonPath = join(staging, DESCRIPTOR_JSON_FILENAME);
    const packBytes = readFileSync(packPath);
    const jsonBytes = readFileSync(jsonPath);
    const pair = verifyDescriptorPairBytesV1(jsonBytes, packBytes, { wasmSha256: raw.sha256, coreWasmSha256: core.sha256 });
    emissionGuard(control, startedAt, budgetMs, "publish");
    renameSync(packPath, join(ownerRoot, DESCRIPTOR_PACK_FILENAME));
    renameSync(jsonPath, join(ownerRoot, DESCRIPTOR_JSON_FILENAME));
    return {
      pluginId: pair.pluginId,
      packageId: pair.packageId,
      role: pair.role,
      version: pair.version,
      ownerRoot,
      jsonPath: join(ownerRoot, DESCRIPTOR_JSON_FILENAME),
      packPath: join(ownerRoot, DESCRIPTOR_PACK_FILENAME),
      rawSha256: raw.sha256,
      coreSha256: core.sha256,
      descriptorSha256: pair.hashes.descriptorSha256,
      jsonByteLength: jsonBytes.byteLength,
      packByteLength: packBytes.byteLength,
    };
  } finally {
    rmSync(staging, { recursive: true, force: true });
  }
}
