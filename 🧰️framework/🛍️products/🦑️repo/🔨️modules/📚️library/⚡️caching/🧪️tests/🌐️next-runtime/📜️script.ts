#!/usr/bin/env node
import Module, { createRequire, isBuiltin } from "node:module";
import { realpathSync } from "node:fs";
import { isAbsolute, join, relative } from "node:path";

const [command, directory] = process.argv.slice(2);
if (command !== "guard" || !directory) throw Error("Expected guard <standalone-directory>");
const artifact = realpathSync(directory), require = createRequire(import.meta.url), observed = new Set<string>(), original = (Module as any)._resolveFilename;
(Module as any)._resolveFilename = function (request: string, parent: unknown, ...rest: unknown[]) {
  const resolved = original.call(this, request, parent, ...rest);
  if (!isBuiltin(resolved)) {
    const physical = realpathSync(resolved), part = relative(artifact, physical);
    if (isAbsolute(part) || part.split(/[\\/]/u)[0] === "..") throw Error("Standalone resolution escaped artifact: " + physical);
    observed.add(part);
  }
  return resolved;
};
process.on("SIGTERM", () => {  process.exit(0); });
process.on("SIGINT", () => {  process.exit(0); });
require(join(artifact, "server.js"));

setImmediate(() => {});
