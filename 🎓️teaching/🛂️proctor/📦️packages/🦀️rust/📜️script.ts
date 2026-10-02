#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🛂️ `teaching-proctor` task router: `bun ./📜️script.ts <build [args…]|test [fundamental|quick|long|exhaustive] [args…]|dev|check [catalog]|rebuild>`.
 *
 * `dev`, `check` and `rebuild` build the binary and run a private copy of it (`../../🏗️bootstrap/🟦️.ts`), so a running
 * dev proctor never locks the executable any other build writes. `dev` serves on `PROCTOR_PORT` (8791) over the
 * git-ignored `.🧬semio/🎓️teaching/proctor-dev/` and the architecture catalog unless the launcher set `PROCTOR_*`
 * itself; `check` validates a catalog (default: the dev catalog); `rebuild` refolds every read model of the dev data
 * directory from its event log (Ctrl+C stops between batches).
 *
 * @see ../../README.md — the operator guide and every environment variable
 */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

import { buildCargoArtifacts } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { PROCTOR_PACKAGE, proctorCatalog, proctorDevelopmentEnvironment, runProctor } from "../../🏗️bootstrap/🟦️.ts";

/** 🏗️ Builds the crate's cargo artifacts (`--release` for a deployment). */
class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildCargoArtifacts(`${this.root}/Cargo.toml`, segments, this.repoRoot);
  }
}

/** 🧪️ Runs the unit, conformance and end-to-end suites at the requested level. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted([PROCTOR_PACKAGE], this.repoRoot, rest);
  }
}

/** 🛠️ Serves the proctor for the dev site (which proxies the gateway routes to it) with the dev defaults. */
class DevScript extends BundleScript {
  async run(): Promise<void> {
    await runProctor(this.repoRoot, ["serve"], proctorDevelopmentEnvironment(this.repoRoot));
  }
}

/** ✅️ Validates a catalog (default: the dev catalog) and its quizzes; exits non-zero with the issues. */
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runProctor(this.repoRoot, ["check", proctorCatalog(this.repoRoot, segments[0])]);
  }
}

/** 🔁️ Drops and refolds every read model of the dev data directory. */
class RebuildScript extends BundleScript {
  async run(): Promise<void> {
    await runProctor(this.repoRoot, ["rebuild"], proctorDevelopmentEnvironment(this.repoRoot));
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("dev", DevScript).register("check", CheckScript).register("rebuild", RebuildScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
