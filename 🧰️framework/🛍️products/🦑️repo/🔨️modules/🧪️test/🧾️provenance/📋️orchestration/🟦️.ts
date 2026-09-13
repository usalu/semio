import { matchesFixture, readSelectors } from "../../🔍️discovery/🎛️selection/🟦️.ts";
import { contentDigestOf, fixtureManifestProblems, installFixtureFile, loadOracleRegistry, publishFixtureManifest, subsetCoordinate, testCacheDir, verifyFixture } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script, runProbe, testLevelBudgetMs } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, mkdirSync, rmSync } from "node:fs";
import { basename, join } from "node:path";

/** 🧫️ Fixture generation, reproduction, verification and audit — the four halves of provenance. */
export class FixtureScript extends Script {
  run(segments: string[]): void {
    const [subcommand = "verify"] = segments;
    const registry = loadOracleRegistry(this.repoRoot);
    const selectors = readSelectors(segments);
    const fixtures = registry.contributions.flatMap((contribution) => contribution.fixtureManifests).filter((fixture) => matchesFixture(fixture, selectors));
    switch (subcommand) {
      case "verify": {
        let bad = 0;
        for (const fixture of fixtures) {
          for (const verification of verifyFixture(this.repoRoot, fixture)) {
            if (verification.ok) continue;
            bad += 1;
            console.error(`[fixture verify] ${fixture.id}/${verification.role}: ${verification.missing ? "missing" : `${verification.actual} ≠ ${verification.expected}`} (${verification.path})`);
          }
        }
        console.log(`[fixture verify] ${fixtures.length} fixture(s), ${bad} file problem(s)`);
        if (bad > 0) process.exit(1);
        return;
      }
      case "audit": {
        const rows = fixtures.map((fixture) => ({
          id: fixture.id,
          class: fixture.class,
          target: subsetCoordinate(fixture.target),
          mutation: fixture.mutation ?? "",
          outcome: fixture.outcome ?? "",
          license: fixture.provenance.license,
          reproducible: fixture.reproducible,
          generator: fixture.generator?.oracle ?? "",
          engine: fixture.generator?.engineFamily ?? "",
          // 🪆️Resolved, not spelled: `fixtureManifestProblems` judges `✳️any` by what sits BESIDE it
          // when it is handed the repository, exactly as the contract phase and the coverage gate
          // already do (both other call sites pass it). Omitting it here made `fixture audit` the
          // one command that read the bare spelling, so every single-subset owner's fixture — gif,
          // las, obj — audited as a wildcard breach while the release gate it feeds reported it clean.
          problems: fixtureManifestProblems(fixture, this.repoRoot),
        }));
        if (segments.includes("--json")) {
          console.log(JSON.stringify(rows, null, 2));
          return;
        }
        for (const row of rows)
          console.log(
            `[fixture audit] ${row.class.padEnd(24)} ${row.target} ${row.mutation}/${row.outcome} licence=${row.license} reproducible=${row.reproducible} generator=${row.generator}(${row.engine})${row.problems.length > 0 ? ` PROBLEMS: ${row.problems.join("; ")}` : ""}`,
          );
        const bad = rows.filter((row) => row.problems.length > 0).length;
        console.log(`[fixture audit] ${rows.length} fixture(s), ${bad} with contract problems`);
        if (bad > 0) process.exit(1);
        return;
      }
      case "reproduce": {
        // 🏭️Reproduction re-runs the RECORDED generator command and compares the bytes it produces with
        // the committed ones. It never writes into the committed fixture: a "reproduce" that overwrote
        // its own expectation would pass unconditionally, which is the whole failure mode it guards.
        let failed = 0;
        for (const fixture of fixtures.filter((entry) => entry.class === "third-party-generated")) {
          if (fixture.generator === undefined) {
            console.error(`[fixture reproduce] ${fixture.id}: no generator record`);
            failed += 1;
            continue;
          }
          const outDir = join(testCacheDir(this.repoRoot, "work"), "🧫️reproduce", fixture.id);
          rmSync(outDir, { recursive: true, force: true });
          mkdirSync(outDir, { recursive: true });
          const [command = "", ...args] = fixture.generator.command.split(/\s+/);
          const probe = runProbe(command, args, { cwd: this.repoRoot, budgetMs: testLevelBudgetMs("long"), env: { ...process.env, SEMIO_FIXTURE_OUT: outDir, SEMIO_FIXTURE_SEED: String(fixture.generator.seed ?? "") } });
          if ((probe.status ?? 1) !== 0) {
            console.error(`[fixture reproduce] ${fixture.id}: generator exited ${probe.status}`);
            failed += 1;
            continue;
          }
          for (const file of fixture.files) {
            const produced = join(outDir, fixture.id, basename(file.path));
            if (!existsSync(produced)) {
              console.error(`[fixture reproduce] ${fixture.id}/${file.role}: generator produced no ${basename(file.path)}`);
              failed += 1;
              continue;
            }
            const actual = contentDigestOf(produced);
            if (actual !== file.sha256) {
              console.error(`[fixture reproduce] ${fixture.id}/${file.role}: ${actual} ≠ ${file.sha256}`);
              failed += 1;
            }
          }
        }
        console.log(`[fixture reproduce] ${fixtures.filter((entry) => entry.class === "third-party-generated").length} generated fixture(s), ${failed} problem(s)`);
        if (failed > 0) process.exit(1);
        return;
      }
      case "generate": {
        // 🏭️Generation and execution are separate operations on purpose: a normal test run must never
        // be able to rewrite the expectation it is being measured against.
        let generated = 0;
        for (const fixture of fixtures.filter((entry) => entry.class === "third-party-generated" && entry.generator !== undefined)) {
          const outDir = join(testCacheDir(this.repoRoot, "work"), "🧫️generate", fixture.id);
          mkdirSync(outDir, { recursive: true });
          const [command = "", ...args] = fixture.generator!.command.split(/\s+/);
          const probe = runProbe(command, args, { cwd: this.repoRoot, budgetMs: testLevelBudgetMs("long"), env: { ...process.env, SEMIO_FIXTURE_OUT: outDir, SEMIO_FIXTURE_SEED: String(fixture.generator!.seed ?? "") } });
          if ((probe.status ?? 1) !== 0) {
            console.error(`[fixture generate] ${fixture.id}: generator exited ${probe.status} — ${probe.stderr.trim().split("\n").slice(-3).join(" | ")}`);
            continue;
          }
          for (const file of fixture.files) {
            const produced = join(outDir, fixture.id, basename(file.path));
            if (!existsSync(produced)) continue;
            installFixtureFile(this.repoRoot, produced);
          }
          publishFixtureManifest(this.repoRoot, fixture);
          generated += 1;
          console.log(`[fixture generate] ${fixture.id}: ${fixture.files.length} file(s) into the content-addressed store`);
        }
        console.log(`[fixture generate] ${generated} fixture bundle(s) generated — commit review is a separate, human step`);
        return;
      }
      default:
        console.error(`[fixture] unknown subcommand ${JSON.stringify(subcommand)} — expected generate | reproduce | verify | audit`);
        process.exit(1);
    }
  }
}
