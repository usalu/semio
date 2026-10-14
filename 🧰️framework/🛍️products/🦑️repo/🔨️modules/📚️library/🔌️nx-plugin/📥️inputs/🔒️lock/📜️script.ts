#!/usr/bin/env bun
import { selectedCargoLockDigest } from "./📥️read/🟨️.mjs";
export { selectedCargoLockDigest } from "./📥️read/🟨️.mjs";

if (import.meta.main) {
  const [command, manifest, ...extra] = process.argv.slice(2);
  if (command !== "digest" || !manifest || extra.length) throw Error("Expected digest <manifest>");
  process.stdout.write(selectedCargoLockDigest(manifest));
}
