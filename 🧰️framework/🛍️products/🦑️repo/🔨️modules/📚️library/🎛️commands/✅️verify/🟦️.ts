import { dispatchOwnedScriptRoute } from "../../🏃️process/🧭️routing/🧩️contributions/🟦️.ts";
import { orchestratorBudgetOpts, runCmd } from "../../🏃️process/🟦️.ts";
import { semioBuildParallelFlag } from "../../🏃️process/🌿️environment/🟦️.ts";
import { policyArtifactOwnershipFieldParity } from "../../🧬️schema/🗿️artifact/⚖️laws/🪪️ownership-field-parity/🟦️.ts";
import { abstractionOwnershipChecks } from "../../📏️ownership/🏛️abstraction/✅️verification/🟦️.ts";
import { policyAbstractionOwnershipBreaches } from "../../📏️ownership/🏛️abstraction/⚖️law/🟦️.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { runTypescriptVerification } from "./🔷️typescript-project/🟦️.ts";
import { runAgnosticGate, runDebugTagCensusGate, runDocstringCensusGate, runDocstringEmojiGate, runInterfaceImportGate, runMutationCapGate, runSourceCensusGate } from "./🚦️gates/🟦️.ts";
import { VerifyGateLanes } from "./🏁️gate-lanes/🟦️.ts";
import { runtimeParseTomlV1 } from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";
import {sayVerifyV1,refuseVerifyV1} from "./📖️texts/🟦️.ts";

//#region 🔖️VerifyScript

export abstract class VerifyScript extends VerifyGateLanes {
  /**
   * 🧭️ Routes one `verify` lane to its checks.
   */
  async run(segments: string[]): Promise<void> {
    if (await dispatchOwnedScriptRoute(this.invocation,this.root, ["verify", ...segments])) return;
    if (segments[0] === "canonical-architecture") {
      if (segments.length !== 1) throw refuseVerifyV1("usage","run.expected-canonical-architecture");
      this.runCanonicalArchitecture();
      return;
    }
    if (segments[0] === "taxonomy") {
      await this.runTaxonomy(segments.slice(1));
      return;
    }
    if (segments[0] === "mutation-outcome-law") {
      this.runMutationOutcomeLaw();
      return;
    }
    if (segments[0] === "history-closure") {
      this.runHistoryClosure(segments.slice(1));
      return;
    }
    if (segments[0] === "composed-child-refs") {
      this.runComposedChildRefs(segments.slice(1));
      return;
    }
    if (segments[0] === "semantic-vocabulary") {
      this.runSemanticVocabulary();
      return;
    }
    if (segments[0] === "package-purity") {
      this.runPackagePurity();
      return;
    }
    if (segments[0] === "build-owners") {
      this.runBuildOwners(segments.slice(1));
      return;
    }
    if (segments[0] === "zero-touch") {
      this.runZeroTouch(segments.slice(1));
      return;
    }
    if (segments[0] === "tool-declarations") {
      const { runDirectToolDeclarationLawV1 } = await import("../../⚖️laws/🧰️direct-tool-declarations/⌨️command/🟦️.ts");
      process.exitCode = runDirectToolDeclarationLawV1(this.root, segments.slice(1));
      return;
    }
    if (segments[0] === "tool-spawns") {
      const { runToolSpawnLawsV1 } = await import("../../⚖️laws/🚷️raw-process-spawn/⌨️command/🟦️.ts");
      process.exitCode = runToolSpawnLawsV1(this.root, segments.slice(1));
      return;
    }
    if (segments[0] === "generated-corruption") {
      this.runGeneratedCorruption(segments.slice(1));
      return;
    }
    if (segments[0] === "build-parity") {
      const { runBuildParityV1 } = await import("../../🏗️build/🪞️parity/⌨️command/🟦️.ts");
      await runBuildParityV1(this.root, segments.slice(1), this.invocation);
      return;
    }
    if (segments[0] === "typescript") {
      await runTypescriptVerification(this.root, segments.slice(1));
      return;
    }
    if (segments[0] === "rust-warnings") {
      await this.runRustWarnings(segments.slice(1));
      return;
    }
    if (segments[0] === "production-placeholders") {
      await runSourceCensusGate(this.root, "placeholders");
      return;
    }
    if (segments[0] === "docstrings") {
      await (segments[1] === "emoji-unique" ? runDocstringEmojiGate(this.root, segments.slice(2)) : runDocstringCensusGate(this.root, segments[1]));
      return;
    }
    if (segments[0] === "agnostic") {
      await runAgnosticGate(this.root, segments.slice(1));
      return;
    }
    if (segments[0] === "mutation-caps") {
      await runMutationCapGate(this.root, segments.slice(1));
      return;
    }
    if (segments[0] === "debug-tags") {
      await runDebugTagCensusGate(this.root);
      return;
    }
    if (segments[0] === "interface-owners") {
      await runInterfaceImportGate(this.root);
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "commands") {
      await runSourceCensusGate(this.root, "commands");
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "tool-jobs") {
      await this.runToolJobCoverage(segments.slice(2));
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "apps") {
      await this.runInteractivityApps(segments.slice(2));
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1q-b1-b6") {
      this.runInteractivityP1qB1B6();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1w") {
      this.runInteractivityP1w();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1x") {
      this.runInteractivityP1x();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1y") {
      this.runInteractivityP1y();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1z") {
      this.runInteractivityP1z();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p5d") {
      this.runInteractivityP5d();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p5e") {
      this.runInteractivityP5e();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p3mn") {
      this.runInteractivityP3mn();
      return;
    }
    if (segments[0] === "interactivity") {
      await this.runInteractivityAudit();
      return;
    }
    if (segments[0] === "dependencies") {
      this.runDependencyFreeze(segments.slice(1));
      return;
    }
    if (segments[0] === "layering") {
      this.runLayering(segments.slice(1));
      return;
    }
    if (segments[0] === "artifact-contract-ownership") {
      abstractionOwnershipChecks(this.root);
      const fixture = JSON.parse(readFileSync(join(this.root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧪️tests/🧫️fixtures/🧪️abstraction-ownership/🔣️.json"), "utf8")) as { artifactSchemas: string[] };
      const packages = [...new Set(fixture.artifactSchemas.map((path) => {
        const [owner] = path.split("/🏅️standards/");
        if (owner === undefined) throw refuseVerifyV1("missing","run.artifact-schema-owner-required");
        const manifest = runtimeParseTomlV1(readFileSync(join(this.root, owner, "📦️packages/🦀️rust/Cargo.toml"), "utf8")) as { package: { name: string } };
        return manifest.package.name;
      }))];
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      const packageArgs = packages.flatMap((name) => ["-p", name]);
      if (segments[1] === "test") {
        await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "--no-fail-fast", ...packageArgs, "committed_diff", "--", "--nocapture"], this.root);
        sayVerifyV1("run.artifact-contract-ownership-committed-diff",{packages:packages.length});
        return;
      }
      await runCargo(["check", "--manifest-path", "Cargo.toml", "--tests", ...packageArgs], this.root);
      sayVerifyV1("run.artifact-contract-ownership-artifact-crates",{packages:packages.length});
      return;
    }
    
    if (segments[0] === "artifact-field-parity") {
      const { testArtifactFieldParityOracle } = await import("../../📏️ownership/🧪️tests/🪪️field-parity/🟦️.ts");
      testArtifactFieldParityOracle();
      if (segments[1] === "test") return;
      const breaches = policyArtifactOwnershipFieldParity(this.root);
      for (const breach of breaches) sayVerifyV1("run.artifact-field-parity-missing-extra",{path:String(breach.path),missing:breach.missing.join(","),extra:breach.extra.join(",")});
      sayVerifyV1("run.artifact-field-parity-schema-representations",{breaches:breaches.length});
      if (breaches.length && segments[1] !== "report") throw refuseVerifyV1("breach","run.artifact-schema-representations-disagree-on");
      return;
    }
    if (segments[0] === "abstraction-ownership") {
      abstractionOwnershipChecks(this.root);
      if (segments[1] === "test") return;
      const breaches = policyAbstractionOwnershipBreaches(this.root);
      for (const breach of breaches) sayVerifyV1("run.abstraction-ownership",{scope:String(breach.scope),summary:String(breach.summary)});
      sayVerifyV1("run.abstraction-ownership-misplaced-declarations",{breaches:breaches.length});
      if (breaches.length && segments[1] !== "report") throw refuseVerifyV1("failed","run.os-owned-preferences-or-host");
      return;
    }
    if (segments[0] === "policy-breach") {
      this.runPolicyBreach(segments[1]);
      return;
    }
    await this.runGate();
    if (segments[0] === "gate") return;
    runCmd("repo", ["run-many", "--targets=test", "--exclude=workspace", ...semioBuildParallelFlag()], { cwd: this.root, ...orchestratorBudgetOpts() });
  }
}
//#endregion 🔖️VerifyScript
