#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { sourceClosureDigest } from "./🟨️.mjs";
export { sourceClosureDigest } from "./🟨️.mjs";
/** 📥️ The owned wire contract contains only exact repository-relative files. */
export interface SourceClosure {
  readonly version: 1;
  readonly files: readonly string[];
}


if (import.meta.main) {
  const [command, path, ...extra] = process.argv.slice(2);
  if (!path || !["digest", "source"].includes(command) || command === "digest" && extra.length) throw Error("Expected digest <closure manifest> or source <repository files...>");
  const contract = command === "source" ? { version: 1 as const, files: [path, ...extra] } : JSON.parse(readFileSync(path, "utf8"));
  process.stdout.write(sourceClosureDigest(process.cwd(), contract));
}
