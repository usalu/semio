/** 📥️ Artifact io of the catalog verification: the contained catalog file read, the chunked and cancellable artifact sha256, the verified artifact read and the artifact receipt. */
import { createHash } from "node:crypto";
import { closeSync, fstatSync, lstatSync, openSync, readFileSync, readSync, realpathSync, statSync } from "node:fs";
import { CATALOG_ARTIFACT_MAX_BYTES, CATALOG_IO_CHUNK_BYTES, type CatalogArtifactControl, type CatalogArtifactProgress, type CatalogFileReceipt } from "../🟦️.ts";
import { pathIsWithin } from "../🛡️guards/🟦️.ts";

//#region 🔖️CatalogCompleteness
/** 📄️ Reads one bounded regular non-symlink file and rejects a path that resolves outside `root`. */
export function readCatalogFile(path: string, root: string, limit: number): Uint8Array {
  const info = lstatSync(path);
  if (info.isSymbolicLink() || !info.isFile()) throw new Error(`${path} must be a regular non-symlink file`);
  if (info.size > limit) throw new Error(`${path} exceeds ${limit} bytes`);
  const realRoot = realpathSync(root);
  const realPath = realpathSync(path);
  if (!pathIsWithin(realRoot, realPath)) throw new Error(`${path} resolves outside ${root}`);
  return readFileSync(path);
}

/** #️⃣ Hashes one bounded build artifact in chunks with containment, progress and cancellation checks. */
export function sha256CatalogArtifact(path: string, containmentRoot: string, pluginId = "catalog", artifact: CatalogArtifactProgress["artifact"] = "raw", control: CatalogArtifactControl = {}): string {
  const info = lstatSync(path);
  if (info.isSymbolicLink() || !info.isFile()) throw new Error(`${artifact} artifact must be a regular non-symlink file`);
  if (info.size > CATALOG_ARTIFACT_MAX_BYTES) throw new Error(`${artifact} artifact exceeds ${CATALOG_ARTIFACT_MAX_BYTES} bytes`);
  const realRoot = realpathSync(containmentRoot);
  const realPath = realpathSync(path);
  if (!pathIsWithin(realRoot, realPath)) throw new Error(`${artifact} artifact escapes the fresh build root`);
  const hash = createHash("sha256");
  const buffer = Buffer.allocUnsafe(CATALOG_IO_CHUNK_BYTES);
  const descriptor = openSync(path, "r");
  let bytesRead = 0;
  try {
    while (bytesRead < info.size) {
      if (control.cancelled?.()) throw new Error("catalog artifact verification cancelled");
      const length = readSync(descriptor, buffer, 0, Math.min(buffer.length, info.size - bytesRead), bytesRead);
      if (length === 0) throw new Error(`${artifact} artifact changed while hashing`);
      hash.update(buffer.subarray(0, length));
      bytesRead += length;
      control.progress?.({ pluginId, artifact, bytesRead, totalBytes: info.size });
    }
  } finally {
    closeSync(descriptor);
  }
  if (statSync(path).size !== info.size) throw new Error(`${artifact} artifact changed while hashing`);
  return hash.digest("hex");
}

export function readVerifiedCatalogArtifact(path: string, containmentRoot: string, limit: number, pluginId: string, artifact: CatalogArtifactProgress["artifact"], control: CatalogArtifactControl): { readonly bytes: Uint8Array; readonly sha256: string } {
  const pathInfo = lstatSync(path);
  if (pathInfo.isSymbolicLink() || !pathInfo.isFile()) throw new Error(`${artifact} artifact must be a regular non-symlink file`);
  const realRoot = realpathSync(containmentRoot);
  const realPath = realpathSync(path);
  if (!pathIsWithin(realRoot, realPath)) throw new Error(`${artifact} artifact escapes the fresh build root`);
  const descriptor = openSync(path, "r");
  try {
    const before = fstatSync(descriptor);
    if (!before.isFile() || before.size > limit) throw new Error(`${artifact} artifact exceeds ${limit} bytes`);
    const bytes = Buffer.allocUnsafe(before.size);
    const hash = createHash("sha256");
    let bytesRead = 0;
    while (bytesRead < before.size) {
      if (control.cancelled?.()) throw new Error("catalog artifact verification cancelled");
      const length = readSync(descriptor, bytes, bytesRead, Math.min(CATALOG_IO_CHUNK_BYTES, before.size - bytesRead), bytesRead);
      if (length === 0) throw new Error(`${artifact} artifact changed while reading verified bytes`);
      hash.update(bytes.subarray(bytesRead, bytesRead + length));
      bytesRead += length;
      control.progress?.({ pluginId, artifact, bytesRead, totalBytes: before.size });
    }
    const after = fstatSync(descriptor);
    if (after.size !== before.size) throw new Error(`${artifact} artifact changed while reading verified bytes`);
    control.afterArtifact?.(artifact);
    return { bytes, sha256: hash.digest("hex") };
  } finally {
    closeSync(descriptor);
  }
}

export function catalogArtifactReceipt(path: string, containmentRoot: string, relativePath: string, pluginId: string, artifact: CatalogArtifactProgress["artifact"], control: CatalogArtifactControl): CatalogFileReceipt {
  const sha256 = sha256CatalogArtifact(path, containmentRoot, pluginId, artifact, control);
  return { path: relativePath, bytes: lstatSync(path).size, sha256 };
}
//#endregion 🔖️CatalogCompleteness
