import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, resolve } from "node:path";

/** 🐳️ Checks the environment-only Docker context with YAML and Docker's documented double-star exclusion semantics. */
export function testDevcontainerContext(workspace: string): void {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🐳️devcontainer-context/🔣️.json"), "utf8"));
  const compose = require("yaml").parse(readFileSync(join(workspace, fixture.compose), "utf8")), build = compose.services[fixture.service].build;
  assert.equal(build.context, fixture.context); assert.equal(build.dockerfile, fixture.dockerfile);
  const context = resolve(workspace, dirname(fixture.compose), build.context);
  assert.equal(context, join(workspace, ".devcontainer"));
  const dockerfile = readFileSync(join(context, build.dockerfile), "utf8");
  assert.equal(/^(?:COPY|ADD)\s|--mount\b/im.test(dockerfile), false, "Image context or mount changes require an explicit reviewed contract");
  const patterns = readFileSync(join(context, ".dockerignore"), "utf8").trim().split("\n");
  assert.deepEqual(patterns, fixture.patterns);
  for (const path of fixture.ignored) assert.equal(require("minimatch").minimatch(path, patterns[0], { dot: true }), true, path);
  console.log("[DEBUG] Devcontainer build uses only its environment directory and excludes all ordinary context files; YAML/minimatch oracle PASS");
}
