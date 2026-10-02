#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🛂️ `teaching-proctor` task router: `bun ./📜️script.ts <build [args…]|test [fundamental|quick|long|exhaustive] [args…]|dev|check [catalog]|rebuild|health|backup [directory/|file|-]|restore <file|->|erase (--handle <handle>|--tag <tag>|--learner <id>) [--dry-run]|prune --older-than <age> [--dry-run]|capacity [--learners n] [--compression n] [--report file] [--executable proctor] [--hall-only]>`.
 *
 * Every verb but `build`, `test` and `capacity` builds the binary and runs a private copy of it
 * (`../../🏗️bootstrap/🟦️.ts`), so a running dev proctor never locks the executable any other build writes. `dev` serves
 * on `PROCTOR_PORT` (8791) over the git-ignored `.🧬semio/🎓️teaching/proctor-dev/` and the architecture catalog unless
 * the launcher set `PROCTOR_*` itself, and moves that folder aside (saying so) when it holds data of a storage format
 * this proctor does not read; `check` validates a catalog (default: the dev catalog); `rebuild` refolds every
 * read model of the dev data directory from its event log (Ctrl+C stops between batches). The operator verbs act on the
 * same dev environment: `health` asks the dev proctor whether it serves; `backup` copies its database while it serves
 * (default: a timestamped file under the git-ignored `.🧬semio/🎓️teaching/proctor-backups/`) and prints the path;
 * `restore` puts a backup in place of the database of the stopped dev proctor; `erase` removes one learner from the
 * stopped dev proctor's database, or with `--dry-run` only says what would go; `prune` removes from it every registration
 * nobody played under that is older than the stated age, or with `--dry-run` only counts them. `capacity` is the load
 * gate: the release build on a throw-away port and data directory with the default limits and caps, a lecture hall of
 * 300 learners from one address alone and then beside an abusive script that also tries to fill the roster
 * (`../../🧪️tests/🏋️capacity/🟦️.ts`); it exits non-zero unless the hall saw no error and no refusal inside its latency
 * budget and the script was served no more than its allowances, capped and cut off — and also when the machine was too
 * busy for the load generator to measure anything, which it then says.
 *
 * @see ../../README.md — the operator guide and every environment variable
 */
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCmd } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { runRepositoryCargoTests } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts";

import { buildRepositoryCargoArtifacts } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { join } from "node:path";
import { PROCTOR_PACKAGE, TEACHING_WORKSPACE, proctorBackupTarget, proctorCatalog, proctorDevelopmentEnvironment, proctorSelection, runDevelopmentProctor, runProctor } from "../../🏗️bootstrap/🟦️.ts";

/** 🏗️ Builds the crate's cargo artifacts (`--release` for a deployment). */
class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildRepositoryCargoArtifacts(`${this.root}/Cargo.toml`, segments, this.repoRoot);
  }
}

/** 🧪️ Runs the unit, conformance and end-to-end suites at the requested level. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests([PROCTOR_PACKAGE], join(this.repoRoot, TEACHING_WORKSPACE), rest);
  }
}

/** 🛠️ Serves the proctor for the dev site (which proxies the gateway routes to it) with the dev defaults; development
 * data of a storage format this proctor does not read is set aside first, and the launcher says so. */
class DevScript extends BundleScript {
  async run(): Promise<void> {
    await runDevelopmentProctor(this.repoRoot);
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

/** 🩺️ Asks the dev proctor whether it serves; exits non-zero when it does not. */
class HealthScript extends BundleScript {
  async run(): Promise<void> {
    await runProctor(this.repoRoot, ["health"], proctorDevelopmentEnvironment(this.repoRoot));
  }
}

/** 💾️ Copies the dev database while the dev proctor serves it: into a directory, to a file or to stdout. */
class BackupScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runProctor(this.repoRoot, ["backup", proctorBackupTarget(this.repoRoot, segments[0])], proctorDevelopmentEnvironment(this.repoRoot));
  }
}

/** ♻️ Puts a backup in place of the database of the stopped dev proctor. */
class RestoreScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 1) throw new Error("restore takes the backup file to restore (or - for stdin)");
    await runProctor(this.repoRoot, ["restore", segments[0]!], proctorDevelopmentEnvironment(this.repoRoot));
  }
}

/** 🧨️ Removes one learner from the stopped dev proctor's database, or with `--dry-run` reports what would go. */
class EraseScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runProctor(this.repoRoot, ["erase", ...proctorSelection(segments)], proctorDevelopmentEnvironment(this.repoRoot));
  }
}

/** 🧹️ Removes from the stopped dev proctor's database the registrations nobody played under that are older than
 * `--older-than <age>` (`90m`, `36h`, `7d`, `2w`), or with `--dry-run` counts them. */
class PruneScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runProctor(this.repoRoot, ["prune", ...proctorSelection(segments)], proctorDevelopmentEnvironment(this.repoRoot));
  }
}

/** 🏋️ Runs the capacity gate in a process of its own, whose HTTP client may hold a lecture hall of requests in flight. */
class CapacityScript extends BundleScript {
  run(segments: string[]): void {
    runCmd(process.execPath, [`${this.root}/../../🧪️tests/🏋️capacity/🟦️.ts`, ...segments], { cwd: this.repoRoot, env: { ...process.env, BUN_CONFIG_MAX_HTTP_REQUESTS: "4096" }, budgetMs: 0 });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("dev", DevScript).register("check", CheckScript).register("rebuild", RebuildScript).register("health", HealthScript).register("backup", BackupScript).register("restore", RestoreScript).register("erase", EraseScript).register("prune", PruneScript).register("capacity", CapacityScript);

await runScriptMain(router, { defaultCommand: "test" });
