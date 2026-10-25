/** 🔢️ Content digests
 *
 * SHA-256 digests of bytes, files and sets that every identity in the test platform is built from.
 */

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
//#endregion 🔌️Adapters

//#region #⃣Digest
/** #⃣ Stable content digest used for feature/fixture/source/output identity. Truncated sha256, hex. */
export function digest(input: string | Uint8Array): string {
  return createHash("sha256").update(input).digest("hex").slice(0, 32);
}

/** 📄️ Digest of a file's bytes; the empty digest for a missing file so a plan stays serializable. */
export function fileDigest(absPath: string): string {
  if (!existsSync(absPath)) return digest("");
  return digest(readFileSync(absPath));
}

/** 🥢️ Order-independent digest of a set of `(name, digest)` pairs. */
export function setDigest(pairs: readonly (readonly [string, string])[]): string {
  return digest(
    [...pairs]
      .map(([a, b]) => `${a} ${b}`)
      .sort()
      .join(""),
  );
}
//#endregion #⃣Digest
