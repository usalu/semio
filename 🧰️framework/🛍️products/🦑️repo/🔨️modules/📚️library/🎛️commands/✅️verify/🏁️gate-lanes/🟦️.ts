import { join } from "node:path";
import { runDependencyVerification } from "../../../🕸️dependencies/⚖️truth/🟦️.ts";
import { orchestratorBudgetOpts, runCmd } from "../../../🏃️process/🟦️.ts";
import { semioBuildParallelFlag } from "../../../🏃️process/🌿️environment/🟦️.ts";
import { canonicalArchitectureEnvironment } from "../../../⚡️caching/🦀️cargo/🟦️.ts";
import { repoTaxonomy } from "../../../🧹️normalization/🔣️taxonomy/🏷️filenames/🟦️.ts";
import { checkStorybookFreshness } from "../🎨️storybook-freshness/🟦️.ts";
import { policyIndexedGeneratedOutputBreaches } from "../../../⚖️laws/📇️indexed-generated-output/🟦️.ts";
import { policyDocumentAppShapeBreaches, policyOsStateAuthorityBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🚧️authority-bans/🟦️.ts";
import { policyStandardsCoverageBreaches, policyStandardSubsetVocabularyBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🚪️artifact-io/🧾️standards/🟦️.ts";
import { policyHandcraftedSpecP3Breaches } from "../../../⚖️laws/🧩️plugin-consistency/✍️handcrafted-mutation/🟦️.ts";
import { policyArtifactSchemaBreaches } from "../../../🧬️schema/🗿️artifact/⚖️laws/📋️aggregate/🟦️.ts";
import { policyAppSchemaBreaches } from "../../../🧬️schema/🗺️surface/⚖️laws/📋️aggregate/🟦️.ts";
import { policyPackageLanguagePurityBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🧼️clean-mechanism/🟦️.ts";
import { policyBannedNameStemBreaches, policyEmojiPrefixBreaches, policyPluginBuilderBreaches, policyPluginRootShapeBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🏷️naming/🟦️.ts";
import { policyModeCompletenessBreaches, policyWindowCompletenessBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🟦️.ts";
import { policyDiscoverCrateDirs } from "../../../⚖️laws/🧩️plugin-consistency/🏛️foundation/🟦️.ts";
import { policyApaBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts";
import { policyInferenceFamilyBreaches } from "../../../🧬️schema/💡️inference/⚖️laws/📋️aggregate/🟦️.ts";
import { policyMutationOutcomeMergePolicyBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/📐️merge-policy-law/🟦️.ts";
import { policyHistoryClosureBreaches } from "../../../⚖️laws/🔬️interactivity-audit/🪣️tool-run-and-history/🟦️.ts";
import { VerifyToolJobLane } from "../🎯️tool-job-lane/🟦️.ts";

/** 🏁️ Dependency-freeze, layering, canonical-architecture and whole-gate verification lanes. */
export abstract class VerifyGateLanes extends VerifyToolJobLane {
  /**
   * 🔒️ `verify dependencies` keeps the baseline ratchet and prints the all-ecosystem literal truth;
   * `summary`, `list`, `self-test`, and the red-until-zero `literal-external` mode are source-only.
   */
  protected runDependencyFreeze(args: string[]): void {
    runDependencyVerification(this.root, args, { jsLockParity: this.lanes.dependencyJsLockParitySelfTests, truth: this.lanes.dependencyTruthSelfTests });
  }

  /** 🧱️ Enforces dependency direction through the repository library's taxonomy-owned graph rule. */
  protected runLayering(segments: string[]): void {
    if (segments.length) throw new Error("Expected verify layering");
    runCmd("bun", ["repo", "run-many", "-p", "@semio-tech/repo-lib", "-t", "lint-dependency-direction", "lint-rust-source-direction", "lint-cargo-dependency-direction", "--skip-cache", ...semioBuildParallelFlag()], { cwd: this.root, ...orchestratorBudgetOpts() });
  }

  /** 🏛️ Executes architecture contract checks contributed by their owning build projects. */
  protected runCanonicalArchitecture(): void {
    console.log("[verify canonical-architecture] checking owner-contributed dependency, state and I/O contracts…");
    runCmd("bun", ["repo", "run-many", "-t", "canonical-architecture", "--all", "--exclude", "workspace", "--skip-cache", ...semioBuildParallelFlag()], {
      cwd: this.root,
      ...orchestratorBudgetOpts(canonicalArchitectureEnvironment(this.root, process.env)),
    });
  }

  protected async runGate(): Promise<void> {
    this.runCanonicalArchitecture();
    // Deliberately calls dependency-cruiser directly rather than `LintScript`/`bun repo run-many --targets=lint --all`:
    // several unrelated projects have pre-existing broken eslint configs,
    // and framework-renderer-wgpu:lint has known pending color-literal violations (see spawn_task follow-ups) —
    // this gate must stay a meaningful, currently-green signal for refactor sessions, not inherit that noise.
    console.log("[verify] dependency-cruiser boundaries…");
    runCmd(process.execPath, ["x", "dependency-cruiser", "🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand", "--config", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs", "--output-type", "err"], { cwd: this.root });
    console.log("[verify] generated catalog freshness…");
    runCmd("bun", ["repo", "run", "@semio-tech/plugin-registry:check"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] region/host-contract script lints…");
    runCmd("bun", ["repo", "run", "@semio-tech/framework-renderer-react:lint"], { cwd: this.root, ...orchestratorBudgetOpts() });
    runCmd("bun", ["repo", "run", "@semio-tech/framework-os-dev:plugin", "lint"], { cwd: this.root, ...orchestratorBudgetOpts() });
    runCmd("bun", ["repo", "run", "@semio-tech/repo-lib:lint-styling-pixels"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] framework owned-schema binding freshness…");
    runCmd("bun", ["repo", "run", "@semio-tech/framework-rs:check"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] schema parser projection freshness…");
    runCmd("bun", ["repo", "run", "workspace:schema-parsers-check"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] ui locale/terminology axes freshness…");
    runCmd("bun", ["repo", "run", "@semio-tech/ui-rs:check"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] chrome i18n literal scan…");
    runCmd("bun", ["repo", "run", "@semio-tech/repo-lib:check-chrome-i18n"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("[verify] TypeScript/JavaScript dependency direction…");
    this.runLayering([]);
    console.log("[verify] owner-root test taxonomy and feature contract…");
    {
      const domain = String(repoTaxonomy(this.root).testDomainPath ?? "");
      if (domain !== "") runCmd("bun", [join(this.root, domain, "📜️script.ts"), "contract"], { cwd: join(this.root, domain), ...orchestratorBudgetOpts() });
    }
    console.log("[verify] storybook scope freshness…");
    await checkStorybookFreshness(this.root);
    console.log("[verify] indexed generated output (wasm/jco/wasm-pack)…");
    {
      const generatedBreaches = policyIndexedGeneratedOutputBreaches(this.root);
      if (generatedBreaches.length > 0) {
        for (const b of generatedBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${generatedBreaches.length} indexed generated-output breach(es)`);
      }
    }
    console.log("[verify] OS exclusive state authority policies…");
    {
      const osBreaches = [
        ...policyOsStateAuthorityBreaches(this.root),
        ...policyDocumentAppShapeBreaches(this.root),
      ];
      if (osBreaches.length > 0) {
        for (const b of osBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${osBreaches.length} OS exclusive state authority policy breach(es)`);
      }
    }
    console.log("[verify] standards/subsets vocabulary…");
    {
      const subsetVocabBreaches = [
        ...policyStandardsCoverageBreaches(this.root),
        ...policyStandardSubsetVocabularyBreaches(this.root),
      ].filter((b) => b.priority === "high");
      if (subsetVocabBreaches.length > 0) {
        for (const b of subsetVocabBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${subsetVocabBreaches.length} standards/subsets vocabulary policy breach(es)`);
      }
    }
    console.log("[verify] handcrafted grammar P3/M4 policies…");
    {
      const handcraftedBreaches = policyHandcraftedSpecP3Breaches(this.root);
      if (handcraftedBreaches.length > 0) {
        for (const b of handcraftedBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${handcraftedBreaches.length} handcrafted-grammar P3/M4 policy breach(es)`);
      }
    }
    console.log("[verify] artifact-schema facet policies…");
    {
      const artifactSchemaBreaches = policyArtifactSchemaBreaches(this.root);
      if (artifactSchemaBreaches.length > 0) {
        for (const b of artifactSchemaBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${artifactSchemaBreaches.length} artifact-schema policy breach(es)`);
      }
    }
    console.log("[verify] app-schema facet policies…");
    {
      const appSchemaBreaches = policyAppSchemaBreaches(this.root);
      if (appSchemaBreaches.length > 0) {
        for (const b of appSchemaBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${appSchemaBreaches.length} app-schema policy breach(es)`);
      }
    }
    console.log("[verify] package language purity…");
    {
      const packagePurityBreaches = policyPackageLanguagePurityBreaches(this.root).filter((b) => b.priority === "high");
      if (packagePurityBreaches.length > 0) {
        for (const b of packagePurityBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${packagePurityBreaches.length} package language purity breach(es)`);
      }
    }
    console.log("[verify] dissolve-core / plugin-root policies…");
    {
      const dissolveBreaches = [
        ...policyBannedNameStemBreaches(this.root),
        ...policyEmojiPrefixBreaches(this.root),
        ...policyPluginRootShapeBreaches(this.root),
        ...policyPluginBuilderBreaches(this.root, policyDiscoverCrateDirs(this.root)),
        ...policyApaBreaches(this.root),
        ...policyInferenceFamilyBreaches(this.root),
      ].filter((b) => b.priority === "high");
      if (dissolveBreaches.length > 0) {
        for (const b of dissolveBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${dissolveBreaches.length} dissolve-core / plugin-root policy breach(es)`);
      }
    }
    console.log("[verify] window capability taxonomy…");
    {
      const crateDirs = policyDiscoverCrateDirs(this.root);
      const windowBreaches = [...policyWindowCompletenessBreaches(this.root, crateDirs), ...policyModeCompletenessBreaches(this.root, crateDirs)];
      if (windowBreaches.length > 0) {
        for (const breach of windowBreaches) console.error(`[verify] ${breach.kind}: ${breach.summary}`);
        throw new Error(`[verify] ${windowBreaches.length} window/mode capability taxonomy breach(es)`);
      }
    }
    console.log("[verify] mutation-outcome / merge-policy law…");
    {
      const mutationOutcomeBreaches = policyMutationOutcomeMergePolicyBreaches(this.root).filter((b) => b.priority === "high");
      if (mutationOutcomeBreaches.length > 0) {
        for (const b of mutationOutcomeBreaches) {
          console.error(`[verify] ${b.kind}: ${b.summary}`);
        }
        throw new Error(`[verify] ${mutationOutcomeBreaches.length} mutation-outcome/merge-policy law breach(es)`);
      }
    }
    console.log("[verify] history closure…");
    {
      if (this.lanes.historyClosurePolicySelfTests() === 0) throw new Error("[verify] history-closure self-test executed no cases");
      const historyClosureBreaches = policyHistoryClosureBreaches(this.root);
      if (historyClosureBreaches.length > 0) {
        for (const breach of historyClosureBreaches) console.error(`[verify] ${breach.kind}: ${breach.summary}`);
        throw new Error(`[verify] ${historyClosureBreaches.length} history-closure breach(es)`);
      }
    }
    console.log("[verify] dsl fixture laws…");
    // Quick level here: the full repo-wide sweep (parse→print→reparse fixpoint, canonicalize
    // idempotence over every real 📚️examples fixture — @semio-tech/s-fixture-sweep-rs) runs at
    // `test dsl`/`test dsl exhaustive`; the gate only needs the engine crates' own quick-level unit tests.
    runCmd("bun", ["repo", "run-many", "-t", "test", "-p", "@semio-tech/dsl-rs", "@semio-tech/dsl-schema-rs", "@semio-tech/dsl-derive-rs", ...semioBuildParallelFlag(), "--", "quick"], {
      cwd: this.root,
      ...orchestratorBudgetOpts(),
    });
    // 🌐️ Cross-target compile coverage — the ONLY thing in this repo that compiles
    // `#[cfg(target_arch = "wasm32")]` code. Native `cargo check`/`cargo test` skip those blocks
    // entirely, so a signature change can leave the wasm bindings broken behind a fully green
    // native build AND a green test suite; that is exactly how `🎭️actor`'s `🦀️.rs` sat with an
    // E0308 until MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME's Z1 ran clippy against the real triple.
    // Scoped to the actor kernel because it is small, fast, purity-critical, and the one crate whose
    // wasm glue every renderer depends on — the fleet-wide wasip2/native sweeps stay opt-in via
    // `verify rust-warnings --target <triple>`, which is far too slow for a pre-close gate.
    console.log("[verify] actor kernel wasm32 bindings…");
    await this.runRustWarnings(["--target", "wasm32-unknown-unknown"]);
    console.log("[verify] gate passed.");
  }
}
