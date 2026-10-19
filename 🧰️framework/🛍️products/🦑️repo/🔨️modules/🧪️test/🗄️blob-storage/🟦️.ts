/** 🗄️ Blob and evidence storage
 *
 * Fixture blob and test evidence roots, installation and publication.
 */

//#region 🔌️Adapters
import { type TestEvidence } from "@semio-tech/framework/test/adapter";
import { constants, cpSync, existsSync, linkSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { testCacheRoot } from "../⚡️output-cache/🟦️.ts";
import { agentId } from "../🔐️run-lease/🟦️.ts";
import { contentDigest, SHA256_RE } from "../🧷️fixture-evidence/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🗄️Storage
/**
 * 🗄️ Content-addressed fixture storage. Every agent and every run would otherwise take its own copy of
 * a megabyte-scale STEP and mesh corpus, so blobs are stored ONCE by digest and referenced into run
 * directories by the cheapest safe mechanism the filesystem offers.
 */
export function fixtureBlobRoot(repoRoot: string): string {
  return join(testCacheRoot(repoRoot), "fixtures", "blobs", "sha256");
}

/** 🗄️ Where fixture manifests are cached by id, for GC's mark phase and for cross-run reuse. */
export function testEvidenceRoot(repoRoot: string): string {
  return join(testCacheRoot(repoRoot), "fixtures", "manifests");
}

/** 🗄️ Absolute path of one blob. Sharded by the first two hex digits so a directory never grows unbounded. */
export function fixtureBlobPath(repoRoot: string, sha256: string): string {
  if (!SHA256_RE.test(sha256)) throw new Error(`not a content digest: ${JSON.stringify(sha256)}`);
  const hex = sha256.slice("sha256:".length);
  return join(fixtureBlobRoot(repoRoot), hex.slice(0, 2), hex);
}

/**
 * 🗄️ Installs bytes under their digest, atomically. Two agents generating the same fixture concurrently
 * is the normal case, not the exception: each writes a private temporary file and renames it into
 * place, so a reader never observes a partial blob and neither writer clobbers the other.
 */
export function installFixtureBlob(repoRoot: string, bytes: Uint8Array): string {
  const sha256 = contentDigest(bytes);
  const target = fixtureBlobPath(repoRoot, sha256);
  if (existsSync(target)) return sha256;
  mkdirSync(dirname(target), { recursive: true });
  const staging = `${target}.${process.pid}.${agentId()}.tmp`;
  writeFileSync(staging, bytes);
  try {
    renameSync(staging, target);
  } catch {
    rmSync(staging, { force: true });
  }
  return sha256;
}

/** 🗄️ Installs a file from disk under its digest without reading it twice into memory when possible. */
export function installEvidenceFile(repoRoot: string, absPath: string): string {
  return installFixtureBlob(repoRoot, readFileSync(absPath));
}

/** 🗄️ How one blob was materialized into a run directory, so the report can show what the cache saved. */
export type MaterializeMode = "reflink" | "hardlink" | "copy";

/**
 * 🗄️ References a blob into a run directory: copy-on-write clone first, hard link second, plain copy
 * last. A hard link is only safe for a file the consumer will not write through — `mutable: true`
 * forces a real copy, because a mutation scenario handed a link would write into shared storage.
 */
export function materializeFixtureBlob(repoRoot: string, sha256: string, targetPath: string, opts: { mutable?: boolean } = {}): MaterializeMode {
  const source = fixtureBlobPath(repoRoot, sha256);
  if (!existsSync(source)) throw new Error(`fixture blob ${sha256} is not in the store`);
  mkdirSync(dirname(targetPath), { recursive: true });
  rmSync(targetPath, { force: true });
  try {
    cpSync(source, targetPath, { mode: constants.COPYFILE_FICLONE_FORCE });
    return "reflink";
  } catch {}
  if (opts.mutable !== true) {
    try {
      linkSync(source, targetPath);
      return "hardlink";
    } catch {}
  }
  cpSync(source, targetPath);
  return "copy";
}

/** 🗄️ Publishes a fixture manifest into the cache so GC can mark its blobs as referenced. */
export function publishTestEvidence(repoRoot: string, manifest: TestEvidence): string {
  const path = join(testEvidenceRoot(repoRoot), `${manifest.id}.json`);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(manifest, null, 2)}\n`);
  return path;
}
//#endregion 🗄️Storage
