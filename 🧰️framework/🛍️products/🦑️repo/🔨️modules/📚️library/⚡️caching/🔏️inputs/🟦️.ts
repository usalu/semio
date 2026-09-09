import { lstatSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { randomUUID } from "node:crypto";

/** 🔏️ Publishes canonical fingerprint bytes after validation, preserving the prior receipt until replacement. */
export function publishGeneratorInputReceipt(workspace: string, output: string, kind: string, digest: string): { path: string; changed: boolean } {
  if (!/^[a-f0-9]{64}$/.test(digest)) throw new Error("Invalid generator input digest");
  if (!/^[a-z]+(?:-[a-z]+)*$/.test(kind)) throw new Error("Invalid generator input kind");
  if (!output || /[\\:*?{}\[\]\0]/.test(output) || output.split("/").some(part => !part || part === "." || part === "..")) throw new Error("Generator receipt must be a canonical workspace-relative path");
  let directory = resolve(workspace);
  for (const part of output.split("/").slice(0, -1)) {
    directory = join(directory, part);
    try { mkdirSync(directory); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error; }
    const stat = lstatSync(directory);
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Generator receipt directory has another owner: ${directory}`);
  }
  const path = resolve(workspace, output), content = JSON.stringify({ digest, kind, version: 1 }) + "\n";
  const stat = lstatSync(path, { throwIfNoEntry: false });
  if (stat) {
    if (!stat.isFile() || stat.isSymbolicLink()) throw new Error(`Generator receipt has another owner: ${path}`);
    if (readFileSync(path, "utf8") === content) return { path, changed: false };
  }
  const temporary = join(dirname(path), `.receipt-${randomUUID()}.tmp`);
  try { writeFileSync(temporary, content, { flag: "wx" }); renameSync(temporary, path); }
  finally { rmSync(temporary, { force: true }); }
  return { path, changed: true };
}
