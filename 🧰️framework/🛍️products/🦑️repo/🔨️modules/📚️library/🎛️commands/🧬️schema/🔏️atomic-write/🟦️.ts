import { randomBytes } from "node:crypto";
import { renameSync, rmSync, writeFileSync } from "node:fs";

/** 🔏️ Writes `text` to `path` through a temporary sibling and a rename, so a failed write leaves the previous file intact and no replica ever sees a torn file.
 * @see https://man7.org/linux/man-pages/man2/rename.2.html
 */
export function writeTextAtomicV1(path: string, text: string, write: (temporary: string, text: string) => void = (temporary, body) => writeFileSync(temporary, body)): void {
  const temporary = `${path}.${process.pid}.${randomBytes(4).toString("hex")}.tmp`;
  try {
    write(temporary, text);
    renameSync(temporary, path);
  } catch (error) {
    try { rmSync(temporary, { force: true }); } catch {}
    throw error;
  }
}
