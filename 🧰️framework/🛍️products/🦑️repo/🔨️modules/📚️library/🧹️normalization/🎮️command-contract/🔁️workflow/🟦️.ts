import { readFileSync } from "node:fs";
import { dirname, relative } from "node:path";
import { applyTaxonomyPlan, inventoryTaxonomy, parseTaxonomyPlan, planTaxonomy, taxonomyPlanDigest, verifyTaxonomy } from "../../🟦️.ts";
import { publishTaxonomyInventoryArtifactShards } from "../../📇️inventory/📦️publication/🟦️.ts";
import { runMutationTaxonomyCli } from "../../🧬️mutation/🔁️workflow/🟦️.ts";
import { taxonomyCliArtifactPath, taxonomyCliGuardedPath, taxonomyCliInventoryOptions, taxonomyCliOptions, taxonomyCliPlanOperationConsoleFields, taxonomyCliPlanOperationCounts, taxonomyCliPlanOperationSummaryRows, taxonomyCliPrintJson, taxonomyCliProgress, taxonomyCliRequireCommittedApply, taxonomyCliTicket, taxonomyCliValidateOperationOptions, taxonomyCliWriteJson, taxonomyCliWriteSummary } from "../🟦️.ts";

export function runTaxonomyCliWorkflow(root: string, args: string[]): void {
    const operation = args[0];
    if (operation !== "inventory" && operation !== "plan" && operation !== "apply" && operation !== "verify") {
      throw new Error(`[clean taxonomy] expected inventory, plan, apply or verify, got ${JSON.stringify(operation)}.`);
    }
    const options = taxonomyCliOptions(args.slice(1));
    taxonomyCliValidateOperationOptions(operation, options);
    const planArgumentPath = operation === "plan" || operation === "apply" ? taxonomyCliGuardedPath(root, options.plan, "--plan") : undefined;
    const resumeArgumentPath = operation === "apply" ? taxonomyCliGuardedPath(root, options.resume, "--resume") : undefined;
    const cancelArgumentPath = taxonomyCliGuardedPath(root, options.cancelFile, "--cancel-file");
    const ticketDir = taxonomyCliTicket(root, options.ticket);
    if (options.kind === "mutation") {
      runMutationTaxonomyCli(root, operation, options, ticketDir, cancelArgumentPath);
      return;
    }
    const inventoryOptions = taxonomyCliInventoryOptions(root, options, cancelArgumentPath);
    if (operation === "inventory") {
      const inventory = inventoryTaxonomy(inventoryOptions);
      if (ticketDir) publishTaxonomyInventoryArtifactShards(dirname(taxonomyCliArtifactPath(ticketDir, "inventory", "json")), inventory, (event) => {
        if (event.current === 0 || event.current === event.total || event.current % 100 === 0) console.error(`[clean taxonomy progress] inventory ${event.phase} ${event.current}/${event.total}${event.path ? ` ${event.path}` : ""}`);
      });
      taxonomyCliWriteSummary(ticketDir, "inventory", "Taxonomy Inventory", [
        `- Source tree digest: \`${inventory.sourceTreeDigest}\``,
        `- Entries: ${inventory.entries.length}`,
        `- Violations: ${inventory.entries.reduce((count, entry) => count + entry.violations.length, 0)}`,
      ]);
      if (options.format === "json") taxonomyCliPrintJson(inventory);
      else console.log(`[clean taxonomy inventory] entries=${inventory.entries.length} source=${inventory.sourceTreeDigest}${ticketDir ? ` -> ${ticketDir}` : ""}`);
      return;
    }
    if (operation === "plan") {
      if (!options.baseline) throw new Error("[clean taxonomy plan] --baseline <commit> is required.");
      const inventory = inventoryTaxonomy(inventoryOptions);
      const plan = planTaxonomy(inventory, {
        baselineCommit: options.baseline,
        excludedTreeDigests: [],
        cancelFile: cancelArgumentPath,
        progress: taxonomyCliProgress,
      });
      const counts = taxonomyCliPlanOperationCounts(plan);
      const planPath = planArgumentPath ?? (ticketDir ? taxonomyCliArtifactPath(ticketDir, "plan", "json") : undefined);
      if (planPath) taxonomyCliWriteJson(planPath, plan);
      taxonomyCliWriteSummary(ticketDir, "plan", "Taxonomy Plan", [
        `- Baseline: \`${plan.baselineCommit}\``,
        `- Plan digest: \`${plan.planDigest}\``,
        ...taxonomyCliPlanOperationSummaryRows(counts),
        `- Unresolved: ${plan.unresolved.length}`,
      ]);
      if (options.format === "json" || !planPath) taxonomyCliPrintJson(plan);
      else console.log(`[clean taxonomy plan] ${taxonomyCliPlanOperationConsoleFields(counts)} unresolved=${plan.unresolved.length} digest=${plan.planDigest} -> ${planPath}`);
      if (plan.unresolved.length > 0) throw new Error(`[clean taxonomy plan] blocked by ${plan.unresolved.length} unresolved decision(s).`);
      return;
    }
    if (operation === "apply") {
      if (!ticketDir) throw new Error("[clean taxonomy apply] --ticket <ticket-id> is required.");
      if (!options.baseline) throw new Error("[clean taxonomy apply] --baseline <commit> is required.");
      const planPath = planArgumentPath;
      if (!planPath) throw new Error("[clean taxonomy apply] --plan <path> is required.");
      const plan = parseTaxonomyPlan(JSON.parse(readFileSync(planPath, "utf8")) as unknown);
      if (plan.excludedTreeDigests.length > 0) throw new Error("[clean taxonomy apply] opaque tree digest opt-in is forbidden from the root CLI.");
      const digest = taxonomyPlanDigest(plan);
      if (plan.planDigest !== digest) throw new Error(`[clean taxonomy apply] stored plan digest ${plan.planDigest} does not match ${digest}.`);
      if (options.digest && options.digest !== digest) throw new Error(`[clean taxonomy apply] --digest ${options.digest} does not match ${digest}.`);
      if (plan.unresolved.length > 0) throw new Error(`[clean taxonomy apply] plan contains ${plan.unresolved.length} unresolved decision(s).`);
      const result = applyTaxonomyPlan(plan, {
        repoRoot: root,
        ticketDir,
        expectedBaselineCommit: options.baseline,
        planArtifactPath: planPath,
        expectedPlanDigest: options.digest ?? digest,
        ...(cancelArgumentPath ? { cancelFile: cancelArgumentPath } : {}),
        ...(resumeArgumentPath ? { resumeJournal: resumeArgumentPath } : {}),
        progress: taxonomyCliProgress,
      });
      taxonomyCliWriteJson(taxonomyCliArtifactPath(ticketDir, "apply", "json"), result);
      taxonomyCliWriteSummary(ticketDir, "apply", "Taxonomy Apply", [
        `- Plan digest: \`${result.planDigest}\``,
        `- State: ${result.state}`,
        `- Applied moves: ${result.appliedMoves}`,
        `- Applied embedded ticket-root relocations: ${result.appliedEmbeddedTicketRootRelocations}`,
        `- Applied symlink target edits: ${result.appliedSymlinkTargetEdits}`,
        `- Applied evidence removals: ${result.appliedEvidenceRemovals}`,
        `- Applied edits: ${result.appliedEdits}`,
        `- Applied regenerations: ${result.appliedRegenerations}`,
        `- Journal: \`${relative(ticketDir, result.journalPath)}\``,
      ]);
      if (options.format === "json") taxonomyCliPrintJson(result);
      else console.log(`[clean taxonomy apply] state=${result.state} moves=${result.appliedMoves} relocations=${result.appliedEmbeddedTicketRootRelocations} symlinks=${result.appliedSymlinkTargetEdits} removals=${result.appliedEvidenceRemovals} edits=${result.appliedEdits} regenerations=${result.appliedRegenerations} journal=${result.journalPath}`);
      taxonomyCliRequireCommittedApply(result.state);
      return;
    }
    const verification = verifyTaxonomy(inventoryOptions);
    if (ticketDir) taxonomyCliWriteJson(taxonomyCliArtifactPath(ticketDir, "verify", "json"), verification);
    const errors = verification.violations.filter((violation) => violation.severity === "error").length;
    const warnings = verification.violations.length - errors;
    taxonomyCliWriteSummary(ticketDir, "verify", "Taxonomy Verification", [
      `- Clean: ${verification.clean}`,
      `- Errors: ${errors}`,
      `- Warnings: ${warnings}`,
    ]);
    if (options.format === "json") taxonomyCliPrintJson(verification);
    else console.log(`[clean taxonomy verify] clean=${verification.clean} errors=${errors} warnings=${warnings}${ticketDir ? ` -> ${ticketDir}` : ""}`);
    if (!verification.clean || errors > 0 || (options.failOnWarning && warnings > 0)) throw new Error(`[clean taxonomy verify] errors=${errors} warnings=${warnings}.`);
}
