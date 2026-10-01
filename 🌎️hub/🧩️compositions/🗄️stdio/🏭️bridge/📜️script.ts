#!/usr/bin/env bun
import { join } from "node:path";
import { runCargo } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

if (import.meta.main) {
  const [command, artifact, standard, subset, ...surface] = process.argv.slice(2);
  if (command !== "list-mutations" || !artifact || !standard || !subset || surface.length > 1) throw new Error("list-mutations <artifact> <standard> <subset> [<surface>]");
  await runCargo(["run", "--quiet", "--offline", "--manifest-path", join(import.meta.dir, "../📦️packages/🦀️rust/Cargo.toml"), "--features", "full-artifact-catalog", "--bin", "semio-hub-stdio-mutation-bridge", "--", command, artifact, standard, subset, ...surface], import.meta.dir);
}
