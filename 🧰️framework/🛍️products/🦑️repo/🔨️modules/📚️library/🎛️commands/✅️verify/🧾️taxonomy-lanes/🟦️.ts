import { execFileSync } from "node:child_process";
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

/** 👁️ Admits only repository-visible paths (tracked or untracked and not git-ignored), so ignored build output, caches and vendored trees never count as authored implementation. */
function repositoryVisibleAdmission(root: string): (path: string, nodeKind: "directory" | "file") => boolean {
  const listing = execFileSync("git", ["-c", "core.fsmonitor=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard"], { cwd: root, maxBuffer: 1 << 30 }).toString("utf8");
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
      if (mode !== "report" && mode !== "enforce" || args.length !== 2) throw new Error(`[verify taxonomy implementation] expected report or enforce, got ${JSON.stringify(args)}.`);
      const controller = new AbortController();
      const cancel = (): void => controller.abort();
      process.once("SIGINT", cancel);
      let previousPhase = "";
      try {
        const findings = await taxonomyImplementationFilesystemFindings(this.root, loadCatalogTaxonomy(), {
          signal: controller.signal,
          admitted: repositoryVisibleAdmission(this.root),
          onProgress: (progress) => {
            if (progress.phase !== previousPhase || (progress.phase === "walk" ? progress.pathsVisited : progress.pathsClassified) % 4096 === 0) console.error(`[verify taxonomy implementation progress] phase=${progress.phase} visited=${progress.pathsVisited} classified=${progress.pathsClassified} findings=${progress.findings} vanished=${progress.vanishedDirectories}`);
            previousPhase = progress.phase;
          },
        });
        console.log(`[verify taxonomy implementation ${mode}] clean=${findings.length === 0} errors=${findings.length}`);
        for (const finding of findings) console.log(`[verify taxonomy implementation ${mode}] error ${finding.breachId} ${finding.path}: ${finding.actualBasename === null ? "A target boundary must own its language packages." : `Implementation leaf ${JSON.stringify(finding.actualBasename)} must use ${JSON.stringify(finding.expectedBasename)}.`}`);
        if (mode === "enforce" && findings.length > 0) throw new Error(`[verify taxonomy implementation enforce] ${findings.length} error finding(s).`);
      } finally {
        process.removeListener("SIGINT", cancel);
      }
      return;
    }
    const mode = args[0];
    if (mode !== "report" && mode !== "enforce") throw new Error(`[verify taxonomy] expected report or enforce, got ${JSON.stringify(mode)}.`);
    const scopesFrom = taxonomyOption(args, "--scopes-from");
    if (scopesFrom !== undefined) {
      const scopes = readFileSync(resolve(this.root, scopesFrom), "utf8").split("\n").map((line) => line.trim()).filter(Boolean);
      let dirty = 0, done = 0;
      for (const verification of verifyTaxonomyScopesClosed({ repoRoot: this.root, scopes })) {
        const errors = verification.violations.filter((violation) => violation.severity === "error").length;
        dirty += Number(!verification.clean);
        done += 1;
        if (args.includes("--json")) console.log(JSON.stringify(verification));
        else {
          console.log(`[verify taxonomy ${mode}] clean=${verification.clean} errors=${errors} warnings=${verification.violations.length - errors} scope=${verification.scope}${verification.closure === undefined ? "" : ` closure=${verification.closure}`}`);
          for (const violation of verification.violations) console.log(`[verify taxonomy ${mode}] ${violation.severity} ${violation.code} ${violation.path}: ${violation.message}`);
        }
      }
      console.error(`[verify taxonomy ${mode}] scopes=${done} dirty=${dirty} from=${scopesFrom}`);
      if (mode === "enforce" && dirty > 0) throw new Error(`[verify taxonomy enforce] ${dirty} of ${done} scope(s) not clean.`);
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
        console.error(`[verify taxonomy progress] ${progress.operation}/${progress.phase} ${progress.current}/${progress.total}${progress.path ? ` ${progress.path}` : ""}`);
      },
    });
    const errors = verification.violations.filter((violation) => violation.severity === "error");
    const warnings = verification.violations.filter((violation) => violation.severity === "warning");
    console.log(`[verify taxonomy ${mode}] clean=${verification.clean} errors=${errors.length} warnings=${warnings.length}${scope ? ` scope=${scope}` : ""}`);
    for (const violation of verification.violations) console.log(`[verify taxonomy ${mode}] ${violation.severity} ${violation.code} ${violation.path}: ${violation.message}`);
    if (mode === "enforce" && !verification.clean) throw new Error(`[verify taxonomy enforce] ${errors.length} error finding(s), ${warnings.length} warning finding(s).`);
  }

  /**
   * ⚖️Standalone entry point for the `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`
   * gate bundle (`policyMutationOutcomeMergePolicyBreaches`'s 16 rules) — the same checks `runGate()`'s
   * "mutation-outcome / merge-policy law" block enforces, runnable in isolation via
   * `bun ./📜️script.ts verify mutation-outcome-law` (the `mutation-outcome-law` build task in
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
      console.log(JSON.stringify({ assets: verifyComposedChildRefAssetCount(this.root), rows, breaches: breaches.length, waived: waived.length }, null, 2));
      return;
    }
    console.log(`[verify composed-child-refs] assets=${verifyComposedChildRefAssetCount(this.root)} mismatched=${rows.length} waived=${waived.length} breaches=${breaches.length}`);
    for (const row of waived) console.log(`[verify composed-child-refs] waived ${row.path} ${row.slot}: ${row.waiver}`);
    for (const row of breaches) console.error(`[verify composed-child-refs] ${row.path} slot ${row.slot}: child_id ${JSON.stringify(row.childId)} but target artifact_id ${JSON.stringify(row.targetArtifactId)}`);
    if (breaches.length > 0) throw new Error(`[verify composed-child-refs] ${breaches.length} composed-child row(s) whose target artifact_id is not their own child_id`);
    console.log("[verify composed-child-refs] passed.");
  }

  protected runMutationOutcomeLaw(): void {
    const breaches = policyMutationOutcomeMergePolicyBreaches(this.root).filter((b) => b.priority === "high");
    if (breaches.length > 0) {
      for (const b of breaches) console.error(`[verify mutation-outcome-law] ${b.kind}: ${b.summary}`);
      throw new Error(`[verify mutation-outcome-law] ${breaches.length} breach(es)`);
    }
    console.log("[verify mutation-outcome-law] passed.");
  }

  /**
   * 🚫️ The non-destructive history closure gate (`HISTORY_CLOSURE_RULES`, ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`
   * §20): the planted-violation self-test first, then one scan of the git-visible `✏️s`, `🧰️framework`, `🌎️hub` sources.
   * `--self-test` runs only the planted cases; `--json` prints the per-rule census instead of failing.
   */
  protected runHistoryClosure(segments: string[]): void {
    const cases = this.lanes.historyClosurePolicySelfTests();
    console.log(`[verify history-closure] self-test passed (${cases} cases).`);
    if (segments.includes("--self-test")) return;
    const findings = historyClosureFindings(historyClosureSources(this.root));
    const census = Object.fromEntries(HISTORY_CLOSURE_RULES.map((rule) => [rule.id, findings.filter((finding) => finding.rule === rule.id).length]));
    if (segments.includes("--json")) {
      console.log(JSON.stringify({ census, findings }, null, 2));
      return;
    }
    for (const finding of findings) console.error(`[verify history-closure] ${finding.file}:${finding.line} ${finding.text}`);
    console.log(`[verify history-closure] ${Object.entries(census).map(([rule, count]) => `${rule}=${count}`).join(" ")}`);
    if (findings.length > 0) throw new Error(`[verify history-closure] ${findings.length} closure violation(s)`);
    console.log("[verify history-closure] passed.");
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
    console.log(`[verify semantic-vocabulary] roots=${policyRepositoryOwnedRoots().join(", ")}`);
    console.log(`[verify semantic-vocabulary] scanned=${files.length} breaches=${breaches.length} high=${blocking.length} advisory=${breaches.length - blocking.length}`);
    for (const b of breaches) console.log(`[verify semantic-vocabulary] ${b.priority} ${b.kind}: ${b.summary}`);
    if (blocking.length > 0) throw new Error(`[verify semantic-vocabulary] ${blocking.length} banned-vocabulary breach(es)`);
    console.log("[verify semantic-vocabulary] passed.");
  }

  /** 📦️Standalone entry point for the Shape V2 package-folder purity rule, as `verify-package-purity` runs it. */
  protected runPackagePurity(): void {
    const breaches = policyPackageLanguagePurityBreaches(this.root).filter((b) => b.priority === "high");
    for (const b of breaches) console.error(`[verify package-purity] ${b.kind}: ${b.summary}`);
    if (breaches.length > 0) throw new Error(`[verify package-purity] ${breaches.length} package language purity breach(es)`);
    console.log("[verify package-purity] passed.");
  }

  /** 📋️ `verify build-owners` fails on any registry finding; `verify build-owners sync` first repairs every finding the tree decides (stale, moved, renamed and unregistered modules) and then fails on the authored ones that remain. */
  protected runBuildOwners(segments: string[]): void {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "sync")) throw new Error("Expected verify build-owners [sync]");
    if (segments[0] === "sync") {
      const result = reconcileBuildOwnerRegistry(this.root, { write: true });
      console.log(`[verify build-owners] sync removed=${result.removed.length} added=${result.added.length} renamed=${result.renamed.length} relocated=${result.relocated.length} repackaged=${result.repackaged.length}`);
      for (const finding of result.findings) console.error(`[verify build-owners] ${finding.kind} ${finding.owner ?? ""} ${finding.path ?? ""} ${finding.detail}`);
      if (result.findings.length > 0) throw new Error(`[verify build-owners] ${result.findings.length} finding(s) remain after the sync (authored, or rows whose paths git still tracks while the files are absent)`);
      console.log("[verify build-owners] passed.");
      return;
    }
    const breaches = policyBuildOwnerRegistryBreaches(this.root);
    for (const b of breaches) console.error(`[verify build-owners] ${b.summary} (${b.solution})`);
    if (breaches.length > 0) throw new Error(`[verify build-owners] ${breaches.length} build owner registry finding(s)`);
    console.log("[verify build-owners] passed.");
  }

  /** 🪫️ `verify zero-touch` fails when a closure that runs before the dependencies are installed imports a workspace package by name, an external package or a post-install module eagerly. */
  protected runZeroTouch(segments: string[]): void {
    if (segments.length) throw new Error("Expected verify zero-touch");
    const breaches = policyZeroTouchClosureBreaches(this.root);
    for (const b of breaches) console.error(`[verify zero-touch] ${b.summary} (${b.solution})`);
    if (breaches.length > 0) throw new Error(`[verify zero-touch] ${breaches.length} zero-touch closure finding(s)`);
    console.log("[verify zero-touch] passed.");
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
      console.log(JSON.stringify(census, null, 2));
    } else {
      console.log(`[verify generated-corruption] scanned=${census.filesScanned} jco-bindings=${census.jcoFilesScanned} jco-aliases-validated=${census.jcoAliasesValidated}`);
      for (const finding of census.findings) console.error(`[verify generated-corruption] ${finding.kind} ${finding.file}:${finding.line} ${finding.detail}`);
    }
    if (census.findings.length > 0) {
      throw new Error(`[verify generated-corruption] ${census.findings.length} spliced-emoji identifier(s) in ${new Set(census.findings.map((f) => f.file)).size} file(s)`);
    }
    console.log("[verify generated-corruption] passed.");
  }

  /**
   * ⚖️Standalone entry point for one of the 12 [[POLICY_BREACH_GATES]] rules, byte-identical in
   * output to the inline `bun -e` eval it replaces: first line is the raw breach count, then one
   * `kind | scope | summary` line per breach, non-zero exit whenever any breach exists.
   */
  protected runPolicyBreach(rule: string | undefined): void {
    const fn = rule ? POLICY_BREACH_GATES[rule] : undefined;
    if (!fn) throw new Error(`[verify policy-breach] unknown rule ${JSON.stringify(rule)}. known: ${Object.keys(POLICY_BREACH_GATES).join(", ")}`);
    const breaches = fn(this.root);
    console.log(breaches.length);
    for (const b of breaches) console.log(b.kind, "|", b.scope, "|", b.summary);
    if (breaches.length > 0) throw new Error(`[verify policy-breach ${rule}] ${breaches.length} breach(es)`);
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
    if (targetIndex >= 0 && !target) throw new Error("[verify rust-warnings] --target needs a triple (native | wasm32-wasip2 | wasm32-unknown-unknown).");
    const explicit: string[] = [];
    for (const [index, argument] of args.entries()) {
      const name = args[index + 1];
      if (argument === "-p" && name) explicit.push(name);
    }
    const scope = rustWarningTargetScope(this.root, target);
    const packages = explicit.length > 0 ? explicit : scope.packages;
    if (packages.length === 0) throw new Error(`[verify rust-warnings] no crates resolved for target ${target ?? "native"}.`);
    console.log(`[verify rust-warnings] ${target ?? "native"}: ${packages.length} crate(s)…`);
    for (const pkg of packages) {
      runCmd("cargo", ["clippy", "-p", pkg, ...scope.scopeArgs, ...scope.targetArgs, ...(scope.packageArgs[pkg] ?? []), "--", "-D", "warnings"], { cwd: this.root, budgetMs: buildBudgetMs() });
    }
    console.log(`[verify rust-warnings] ${target ?? "native"} clean.`);
  }
}
