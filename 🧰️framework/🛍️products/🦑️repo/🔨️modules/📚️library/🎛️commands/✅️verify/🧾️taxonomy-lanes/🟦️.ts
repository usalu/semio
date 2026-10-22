
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { SelfTestLanes } from "../../../⚖️laws/🧪️self-test-lanes/🟦️.ts";
import { taxonomyImplementationFilesystemFindings } from "../../../🔍️discovery/🗂️file-kind-resolution/🟦️.ts";
import { loadCatalogTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { taxonomyOption } from "../../../🧹️normalization/🔣️taxonomy/🏷️filenames/🟦️.ts";
import { verifyTaxonomy, verifyTaxonomyScopesClosed } from "../../../🧹️normalization/✅️verification/🟦️.ts";
import { verifyComposedChildRefAssetCount, verifyComposedChildRefRows } from "../🧷️composed-child-refs/🟦️.ts";
import { policyMutationOutcomeMergePolicyBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/📐️merge-policy-law/🟦️.ts";
import { HISTORY_CLOSURE_RULES, historyClosureFindings, historyClosureSources } from "../../../⚖️laws/🔬️interactivity-audit/🪣️tool-run-and-history/🟦️.ts";
import { policyListSemanticVocabularyScanFiles, policyRepositoryOwnedRoots, policySemanticVocabularyBreaches } from "../../../⚖️laws/🧩️plugin-consistency/✍️handcrafted-mutation/🟦️.ts";
import { policyPackageLanguagePurityBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🧼️clean-mechanism/🟦️.ts";
import { reconcileBuildOwnerRegistry } from "../../../🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts";
import { policyBuildOwnerRegistryBreaches } from "../../../⚖️laws/📋️build-owner-registry/🟦️.ts";
import { policyZeroTouchClosureBreaches } from "../../../⚖️laws/🛗️zero-touch-closure/🟦️.ts";
import { censusGeneratedCorruption } from "../🧟️generated-corruption/🟦️.ts";
import { POLICY_BREACH_GATES } from "../🚨️policy-breach-gates/🟦️.ts";
import { rustWarningTargetScope } from "../🦀️rust-warnings/🟦️.ts";
import { runCmd } from "../../../🏃️process/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { Script } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runToolSyncV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import {sayVerifyV1,refuseVerifyV1} from "../📖️texts/🟦️.ts";
import {writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";

/** 👁️ Admits only repository-visible paths (tracked or untracked and not git-ignored), so ignored build output, caches and vendored trees never count as authored implementation. */
function repositoryVisibleAdmission(root: string): (path: string, nodeKind: "directory" | "file") => boolean {
  const listing = runToolSyncV1("git", ["-c", "core.fsmonitor=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard"], { check: true, cwd: root, maxBuffer: 1 << 30 }).stdout.toString("utf8");
  const files = new Set<string>(), directories = new Set<string>();
  for (const path of listing.split("\0")) {
    if (!path) continue;
    files.add(path);
    for (let cut = path.lastIndexOf("/"); cut > 0; cut = path.lastIndexOf("/", cut - 1)) directories.add(path.slice(0, cut));
  }
  return (path, nodeKind) => (nodeKind === "file" ? files : directories).has(path.normalize("NFC")) || (nodeKind === "file" ? files : directories).has(path);
}

/** 🧾️ Taxonomy, composed-child, mutation-outcome, history, vocabulary, purity, corruption, breach and Rust-warning verification lanes. */
export abstract class VerifyTaxonomyLanes extends Script {
  protected abstract readonly lanes: SelfTestLanes;

  /** 🚦️ Reports findings without failing, or enforces the identical structured result. */
  protected async runTaxonomy(args: string[]): Promise<void> {
    if (args[0] === "implementation") {
      const mode = args[1];
      if (mode !== "report" && mode !== "enforce" || args.length !== 2) throw refuseVerifyV1("invalid","lanes.taxonomy-implementation-expected-report-or",{args:String(JSON.stringify(args))});
      const controller = new AbortController();
      const cancel = (): void => controller.abort();
      process.once("SIGINT", cancel);
      let previousPhase = "";
      try {
        const findings = await taxonomyImplementationFilesystemFindings(this.root, loadCatalogTaxonomy(), {
          signal: controller.signal,
          admitted: repositoryVisibleAdmission(this.root),
          onProgress: (progress) => {
            if (progress.phase !== previousPhase || (progress.phase === "walk" ? progress.pathsVisited : progress.pathsClassified) % 4096 === 0) sayVerifyV1("lanes.taxonomy-implementation-progress-phase-visited",{phase:String(progress.phase),pathsVisited:String(progress.pathsVisited),pathsClassified:String(progress.pathsClassified),findings:String(progress.findings),vanishedDirectories:String(progress.vanishedDirectories)},"error");
            previousPhase = progress.phase;
          },
        });
        sayVerifyV1("lanes.taxonomy-implementation-mode-clean-errors",{mode:String(mode),clean:String(findings.length === 0),errors:findings.length});
        for (const finding of findings) sayVerifyV1("lanes.taxonomy-implementation-mode-error",{mode:String(mode),breachId:String(finding.breachId),path:String(finding.path),expected:String(finding.actualBasename === null ? "A target boundary must own its language packages." : `Implementation leaf ${JSON.stringify(finding.actualBasename)} must use ${JSON.stringify(finding.expectedBasename)}.`)});
        if (mode === "enforce" && findings.length > 0) throw refuseVerifyV1("breach","lanes.taxonomy-implementation-enforce-error-finding",{findings:findings.length});
      } finally {
        process.removeListener("SIGINT", cancel);
      }
      return;
    }
    const mode = args[0];
    if (mode !== "report" && mode !== "enforce") throw refuseVerifyV1("invalid","lanes.taxonomy-expected-report-or-enforce",{mode:String(JSON.stringify(mode))});
    const scopesFrom = taxonomyOption(args, "--scopes-from");
    if (scopesFrom !== undefined) {
      const scopes = readFileSync(resolve(this.root, scopesFrom), "utf8").split("\n").map((line) => line.trim()).filter(Boolean);
      let dirty = 0, done = 0;
      for (const verification of verifyTaxonomyScopesClosed({ repoRoot: this.root, scopes })) {
        const errors = verification.violations.filter((violation) => violation.severity === "error").length;
        dirty += Number(!verification.clean);
        done += 1;
        if (args.includes("--json")) writeResultLineV1(JSON.stringify(verification));
        else {
          sayVerifyV1("lanes.taxonomy-mode-clean-errors-warnings",{mode:String(mode),clean:String(verification.clean),errors:String(errors),warnings:String(verification.violations.length - errors),scope:String(verification.scope),closure:String(verification.closure === undefined ? "" : ` closure=${verification.closure}`)});
          for (const violation of verification.violations) sayVerifyV1("lanes.taxonomy-mode",{mode:String(mode),severity:String(violation.severity),code:String(violation.code),path:String(violation.path),message:String(violation.message)});
        }
      }
      sayVerifyV1("lanes.taxonomy-mode-scopes-dirty-from",{mode:String(mode),scopes:String(done),dirty:String(dirty),from:String(scopesFrom)},"error");
      if (mode === "enforce" && dirty > 0) throw refuseVerifyV1("failed","lanes.taxonomy-enforce-of-scope-not",{dirty:String(dirty),scopes:String(done)});
      return;
    }
    const scope = taxonomyOption(args, "--scope");
    let previousPhase = "";
    let previousReportAt = 0;
    const verification = verifyTaxonomy({
      repoRoot: this.root,
      ...(scope ? { scope } : {}),
      progress: (progress) => {
        const now = Date.now();
        if (progress.phase === previousPhase && now - previousReportAt < 5000) return;
        previousPhase = progress.phase;
        previousReportAt = now;
        sayVerifyV1("lanes.taxonomy-progress",{operation:String(progress.operation),phase:String(progress.phase),current:String(progress.current),total:String(progress.total),path:String(progress.path ? ` ${progress.path}` : "")},"error");
      },
    });
    const errors = verification.violations.filter((violation) => violation.severity === "error");
    const warnings = verification.violations.filter((violation) => violation.severity === "warning");
    sayVerifyV1("lanes.taxonomy-mode-clean-errors-warnings-2",{mode:String(mode),clean:String(verification.clean),errors:errors.length,warnings:warnings.length,scope:String(scope ? ` scope=${scope}` : "")});
    for (const violation of verification.violations) sayVerifyV1("lanes.taxonomy-mode",{mode:String(mode),severity:String(violation.severity),code:String(violation.code),path:String(violation.path),message:String(violation.message)});
    if (mode === "enforce" && !verification.clean) throw refuseVerifyV1("breach","lanes.taxonomy-enforce-error-finding-warning",{errors:errors.length,warnings:warnings.length});
  }

  /**
   * ⚖️Standalone entry point for the `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`
   * gate bundle (`policyMutationOutcomeMergePolicyBreaches`'s 16 rules) — the same checks `runGate()`'s
   * "mutation-outcome / merge-policy law" block enforces, runnable in isolation via
   * `repo ./📜️script.ts verify mutation-outcome-law` (the `mutation-outcome-law` build task in
   * `📋️project.json` wires this ahead of `verify-gate`).
   */
  /**
   * 🪆️Repo-wide law for composed-child rows in committed `*.dsl.semio` documents: a child row's
   * `child_id` and its target `ArtifactRef.artifact_id` must be the SAME string. That is not a style
   * rule — `ChildRestoreProjection::child`
   * (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`) refuses any other pair with
   * `InvalidReference`, so a mismatched asset cannot survive a whole-document `Effect::LoadDocument`
   * and every app that gains an example picker fails at its first boot dispatch with
   * `document archive genesis child projection failed`. Found 2026-09-20 by slice B2c on
   * `📖️playbook`'s demo asset (ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`), where the verb
   * was correct and the ASSET was a month-stale bare-id artefact. `--json` prints the census instead
   * of failing.
   */
  protected runComposedChildRefs(segments: string[]): void {
    const rows = verifyComposedChildRefRows(this.root);
    const waived = rows.filter((row) => row.waiver !== undefined);
    const breaches = rows.filter((row) => row.waiver === undefined);
    if (segments.includes("--json")) {
      writeResultLineV1(JSON.stringify({ assets: verifyComposedChildRefAssetCount(this.root), rows, breaches: breaches.length, waived: waived.length }, null, 2));
      return;
    }
    sayVerifyV1("lanes.composed-child-refs-assets-mismatched",{assets:String(verifyComposedChildRefAssetCount(this.root)),mismatched:rows.length,waived:waived.length,breaches:breaches.length});
    for (const row of waived) sayVerifyV1("lanes.composed-child-refs-waived",{path:String(row.path),slot:String(row.slot),waiver:String(row.waiver)});
    for (const row of breaches) sayVerifyV1("lanes.composed-child-refs-slot-child",{path:String(row.path),slot:String(row.slot),childId:String(JSON.stringify(row.childId)),targetArtifactId:String(JSON.stringify(row.targetArtifactId))},"error");
    if (breaches.length > 0) throw refuseVerifyV1("failed","lanes.composed-child-refs-composed-child",{breaches:breaches.length});
    sayVerifyV1("lanes.composed-child-refs-passed");
  }

  protected runMutationOutcomeLaw(): void {
    const breaches = policyMutationOutcomeMergePolicyBreaches(this.root).filter((b) => b.priority === "high");
    if (breaches.length > 0) {
      for (const b of breaches) sayVerifyV1("lanes.mutation-outcome-law",{kind:String(b.kind),summary:String(b.summary)},"error");
      throw refuseVerifyV1("breach","lanes.mutation-outcome-law-breach",{breaches:breaches.length});
    }
    sayVerifyV1("lanes.mutation-outcome-law-passed");
  }

  /**
   * 🚫️ The non-destructive history closure gate (`HISTORY_CLOSURE_RULES`, ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`
   * §20): the planted-violation self-test first, then one scan of the git-visible `✏️s`, `🧰️framework`, `🌎️hub` sources.
   * `--self-test` runs only the planted cases; `--json` prints the per-rule census instead of failing.
   */
  protected runHistoryClosure(segments: string[]): void {
    const cases = this.lanes.historyClosurePolicySelfTests();
    sayVerifyV1("lanes.history-closure-self-test-passed",{cases:String(cases)});
    if (segments.includes("--self-test")) return;
    const findings = historyClosureFindings(historyClosureSources(this.root));
    const census = Object.fromEntries(HISTORY_CLOSURE_RULES.map((rule) => [rule.id, findings.filter((finding) => finding.rule === rule.id).length]));
    if (segments.includes("--json")) {
      writeResultLineV1(JSON.stringify({ census, findings }, null, 2));
      return;
    }
    for (const finding of findings) sayVerifyV1("lanes.history-closure",{file:String(finding.file),line:String(finding.line),text:String(finding.text)},"error");
    sayVerifyV1("lanes.history-closure-2",{census:Object.entries(census).map(([rule, count]) => `${rule}=${count}`).join(" ")});
    if (findings.length > 0) throw refuseVerifyV1("breach","lanes.history-closure-closure-violation",{findings:findings.length});
    sayVerifyV1("lanes.history-closure-passed");
  }

  /**
   * 🚫️Standalone entry point for `policySemanticVocabularyBreaches` — the only consumer of
   * `policyRepositoryOwnedRoots()`, so this is also the runnable proof that the walk roots are the
   * taxonomy's `clean` areas and nothing else. `high` breaches fail; the `Set*` dispatch-variant rule is
   * advisory by its own declaration and is reported without failing.
   */
  protected runSemanticVocabulary(): void {
    const breaches = policySemanticVocabularyBreaches(this.root);
    const files = policyListSemanticVocabularyScanFiles(this.root);
    const blocking = breaches.filter((b) => b.priority === "high");
    sayVerifyV1("lanes.semantic-vocabulary-roots",{roots:policyRepositoryOwnedRoots().join(", ")});
    sayVerifyV1("lanes.semantic-vocabulary-scanned-breaches-high",{scanned:files.length,breaches:breaches.length,high:blocking.length,advisory:breaches.length - blocking.length});
    for (const b of breaches) sayVerifyV1("lanes.semantic-vocabulary",{priority:String(b.priority),kind:String(b.kind),summary:String(b.summary)});
    if (blocking.length > 0) throw refuseVerifyV1("breach","lanes.semantic-vocabulary-banned-vocabulary-breach",{blocking:blocking.length});
    sayVerifyV1("lanes.semantic-vocabulary-passed");
  }

  /** 📦️Standalone entry point for the Shape V2 package-folder purity rule, as `verify-package-purity` runs it. */
  protected runPackagePurity(): void {
    const breaches = policyPackageLanguagePurityBreaches(this.root).filter((b) => b.priority === "high");
    for (const b of breaches) sayVerifyV1("lanes.package-purity",{kind:String(b.kind),summary:String(b.summary)},"error");
    if (breaches.length > 0) throw refuseVerifyV1("breach","lanes.package-purity-package-language-purity",{breaches:breaches.length});
    sayVerifyV1("lanes.package-purity-passed");
  }

  /** 📋️ `verify build-owners` fails on any registry finding; `verify build-owners sync` first repairs every finding the tree decides (stale, moved, renamed and unregistered modules) and then fails on the authored ones that remain. */
  protected runBuildOwners(segments: string[]): void {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "sync")) throw refuseVerifyV1("usage","lanes.expected-build-owners-sync");
    if (segments[0] === "sync") {
      const result = reconcileBuildOwnerRegistry(this.root, { write: true });
      sayVerifyV1("lanes.build-owners-sync-removed-added",{removed:result.removed.length,added:result.added.length,renamed:result.renamed.length,relocated:result.relocated.length,repackaged:result.repackaged.length});
      for (const finding of result.findings) sayVerifyV1("lanes.build-owners",{kind:String(finding.kind),owner:String(finding.owner ?? ""),path:String(finding.path ?? ""),detail:String(finding.detail)},"error");
      if (result.findings.length > 0) throw refuseVerifyV1("breach","lanes.build-owners-finding-remain-after",{findings:result.findings.length});
      sayVerifyV1("lanes.build-owners-passed");
      return;
    }
    const breaches = policyBuildOwnerRegistryBreaches(this.root);
    for (const b of breaches) sayVerifyV1("lanes.build-owners-2",{summary:String(b.summary),solution:String(b.solution)},"error");
    if (breaches.length > 0) throw refuseVerifyV1("breach","lanes.build-owners-build-owner-registry",{breaches:breaches.length});
    sayVerifyV1("lanes.build-owners-passed");
  }

  /** 🪫️ `verify zero-touch` fails when a closure that runs before the dependencies are installed imports a workspace package by name, an external package or a post-install module eagerly. */
  protected runZeroTouch(segments: string[]): void {
    if (segments.length) throw refuseVerifyV1("usage","lanes.expected-zero-touch");
    const breaches = policyZeroTouchClosureBreaches(this.root);
    for (const b of breaches) sayVerifyV1("lanes.zero-touch",{summary:String(b.summary),solution:String(b.solution)},"error");
    if (breaches.length > 0) throw refuseVerifyV1("breach","lanes.zero-touch-zero-touch-closure",{breaches:breaches.length});
    sayVerifyV1("lanes.zero-touch-passed");
  }

  /**
   * 🔬️Standalone entry point for the generated-code corruption census — the permanent gate against the
   * 2026-09-03 rename-plan codemod incident, where an emoji+VS16 was spliced INTO ASCII identifiers of
   * jco/wit-bindgen output (`WasiCliEnvironmen🔬️t029`). Two independent checks run: every jco world
   * binding's alias is re-derived from the WIT id jco prints beside it and must match byte-for-byte,
   * and every parsed TS/JS file must carry no emoji in code position once comments, string/template
   * literals and regex literals are blanked. `--json` prints the machine-readable census instead.
   */
  protected runGeneratedCorruption(segments: string[]): void {
    const census = censusGeneratedCorruption(this.root);
    if (segments.includes("--json")) {
      writeResultLineV1(JSON.stringify(census, null, 2));
    } else {
      sayVerifyV1("lanes.generated-corruption-scanned-jco-bindings",{scanned:String(census.filesScanned),jcoFilesScanned:String(census.jcoFilesScanned),jcoAliasesValidated:String(census.jcoAliasesValidated)});
      for (const finding of census.findings) sayVerifyV1("lanes.generated-corruption",{kind:String(finding.kind),file:String(finding.file),line:String(finding.line),detail:String(finding.detail)},"error");
    }
    if (census.findings.length > 0) {
      throw refuseVerifyV1("failed","lanes.generated-corruption-spliced-emoji-identifier",{findings:census.findings.length,files:new Set(census.findings.map((f) => f.file)).size});
    }
    sayVerifyV1("lanes.generated-corruption-passed");
  }

  /**
   * ⚖️Standalone entry point for one of the 12 [[POLICY_BREACH_GATES]] rules, byte-identical in
   * output to the inline `bun -e` eval it replaces: first line is the raw breach count, then one
   * `kind | scope | summary` line per breach, non-zero exit whenever any breach exists.
   */
  protected runPolicyBreach(rule: string | undefined): void {
    const fn = rule ? POLICY_BREACH_GATES[rule] : undefined;
    if (!fn) throw refuseVerifyV1("breach","lanes.policy-breach-unknown-rule-known",{rule:String(JSON.stringify(rule)),known:Object.keys(POLICY_BREACH_GATES).join(", ")});
    const breaches = fn(this.root);
    console.log(breaches.length);
    for (const b of breaches) console.log(b.kind, "|", b.scope, "|", b.summary);
    if (breaches.length > 0) throw refuseVerifyV1("breach","lanes.policy-breach-rule-breach",{rule:String(rule),breaches:breaches.length});
  }

  /**
   * 🦀️ Zero-warning gate per compilation target (ticket
   * `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet `Z1`; exit checklist item 6) —
   * `verify rust-warnings --target <triple> [-p <crate>…]`. The three targets that must all be
   * clean are native (host), `wasm32-wasip2` (the plugin fleet's real component build) and
   * `wasm32-unknown-unknown` (the pure actor kernel, the one that keeps mobile open). With no
   * explicit `-p`, each target resolves to the crate set that target actually exists to prove.
   *
   * Deny-on-warnings is passed as clippy's trailing `-- -D warnings`, NEVER through `RUSTFLAGS`:
   * `RUSTFLAGS` REPLACES rather than merges with `.cargo/config.toml`'s rustflags (`-Z threads=8`,
   * the wasm32 `getrandom_backend` cfg, mold) and would break every wasm build — the same reason
   * [[runCargoLint]] documents for the identical choice.
   */
  protected async runRustWarnings(args: string[]): Promise<void> {
    const { rustWarningScopeChecks } = await import("../../../../../../../../🧪️tests/🦀️rust-warnings/🟦️.ts");
    rustWarningScopeChecks(this.root, rustWarningTargetScope);
    if (args.includes("--scope-only")) return;
    const targetIndex = args.indexOf("--target");
    const target = targetIndex >= 0 ? args[targetIndex + 1] : undefined;
    if (targetIndex >= 0 && !target) throw refuseVerifyV1("invalid","lanes.rust-warnings-target-needs-a");
    const explicit: string[] = [];
    for (const [index, argument] of args.entries()) {
      const name = args[index + 1];
      if (argument === "-p" && name) explicit.push(name);
    }
    const scope = rustWarningTargetScope(this.root, target);
    const packages = explicit.length > 0 ? explicit : scope.packages;
    if (packages.length === 0) throw refuseVerifyV1("failed","lanes.rust-warnings-no-crates-resolved",{target:String(target ?? "native")});
    sayVerifyV1("lanes.rust-warnings-crate",{target:String(target ?? "native"),packages:packages.length});
    for (const pkg of packages) {
      runCmd("cargo", ["clippy", "-p", pkg, ...scope.scopeArgs, ...scope.targetArgs, ...(scope.packageArgs[pkg] ?? []), "--", "-D", "warnings"], { cwd: this.root, budgetMs: buildBudgetMs() });
    }
    sayVerifyV1("lanes.rust-warnings-clean",{target:String(target ?? "native")});
  }
}
