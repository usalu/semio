import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { canonicalJson } from "../../🟦️.ts";
import type { BreachRecord } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { taxonomyCliArtifactPath, taxonomyCliGuardedPath, taxonomyCliPrintJson, taxonomyCliProgress, taxonomyCliRequireCommittedApply, taxonomyCliWriteJson, type TaxonomyCliOperation, type TaxonomyCliOptions } from "../../🎮️command-contract/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME } from "../🪪️identity/🟦️.ts";
import { inventoryMutationTaxonomy, type MutationTaxonomyInventory } from "../🧾️evidence/🟦️.ts";
import { mutationTaxonomySourceSnapshot } from "../📇️index/🟦️.ts";
import type { MutationTaxonomyInventoryOptions } from "../📸️captured-source/🟦️.ts";

export interface MutationTaxonomyPlanMove { readonly source: string; readonly destination: string }

export interface MutationTaxonomyPlan {
  readonly schemaVersion: 1;
  readonly kind: "mutation";
  readonly baselineCommit: string;
  readonly inventoryDigest: string;
  readonly moves: readonly MutationTaxonomyPlanMove[];
  readonly unresolved: readonly { path: string; reason: string }[];
  readonly planDigest: string;
}


/** 🗺️ Plans only lossless legacy-component moves; semantic/root/glue rewrites remain explicit blockers. */
export function planMutationTaxonomy(inventory: MutationTaxonomyInventory, baselineCommit: string): MutationTaxonomyPlan {
  const moves: MutationTaxonomyPlanMove[] = [];
  const unresolved: { path: string; reason: string }[] = [];
  for (const record of inventory.records) {
    if (record.state === "direct") {
      for (const violation of record.violationClasses) unresolved.push({ path: `${record.mutationRootPath}/${record.targetMutationDirectoryName}`, reason: `Direct-shaped mutation remains structurally unresolved: ${violation}.` });
      continue;
    }
    if (record.state === "central-only") {
      unresolved.push({ path: record.mutationRootPath, reason: `${record.aggregateVariant ?? record.targetMutationDirectoryName} requires semantic extraction from the aggregate.` });
      continue;
    }
    const leafRel = `${record.mutationRootPath}/${record.targetMutationDirectoryName}`;
    const source = `${leafRel}/🦠️mutation/${POLICY_RS_COMPONENT_LEAF_NAME}`;
    const destination = `${leafRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
    if (record.evidence.sourceFiles.includes(source)) moves.push({ source, destination });
    unresolved.push({ path: leafRel, reason: "Direct cutover also requires AST-verified aggregate/glue references and root-purity redistribution." });
  }
  for (const item of inventory.unresolved ?? []) unresolved.push(item);
  const representedViolations = new Set(unresolved.map(({ path }) => path));
  for (const violation of inventory.violations) {
    if (!representedViolations.has(violation.scope)) unresolved.push({ path: violation.scope, reason: `Live structural violation: ${violation.kind}.` });
  }
  moves.sort((left, right) => left.source.localeCompare(right.source));
  unresolved.sort((left, right) => left.path.localeCompare(right.path));
  const unsigned = { schemaVersion: 1 as const, kind: "mutation" as const, baselineCommit, inventoryDigest: inventory.sourceTreeDigest, moves, unresolved };
  return { ...unsigned, planDigest: createHash("sha256").update(canonicalJson(unsigned)).digest("hex") };
}


export function verifyMutationTaxonomy(repoRoot: string, options: MutationTaxonomyInventoryOptions = {}): { readonly clean: boolean; readonly inventoryDigest: string; readonly violations: readonly BreachRecord[] } {
  const inventory = inventoryMutationTaxonomy(repoRoot, options);
  return { clean: inventory.violations.length === 0, inventoryDigest: inventory.sourceTreeDigest, violations: inventory.violations };
}


export function mutationTaxonomyCheckCancellation(root: string, cancelFile: string | undefined): void {
  if (cancelFile && existsSync(taxonomyCliGuardedPath(root, cancelFile, "--cancel-file")!)) throw new Error("[clean taxonomy apply --kind mutation] cancelled before commit.");
}


export function runMutationTaxonomyCli(root: string, operation: TaxonomyCliOperation, options: TaxonomyCliOptions, ticketDir: string | undefined, cancelFile?: string): void {
  const inventoryOptions: MutationTaxonomyInventoryOptions = {
    ...(options.scope ? { scope: options.scope } : {}),
    ...(cancelFile ?? options.cancelFile ? { cancelFile: cancelFile ?? options.cancelFile } : {}),
    ...(ticketDir ? { assignmentLedgerPath: join(ticketDir, "📋️mutation-assignments.json") } : {}),
    progress: taxonomyCliProgress,
  };
  if (operation === "inventory") {
    const inventory = inventoryMutationTaxonomy(root, inventoryOptions);
    if (ticketDir) taxonomyCliWriteJson(taxonomyCliArtifactPath(ticketDir, operation, "json"), inventory);
    if (options.format === "json") taxonomyCliPrintJson(inventory);
    else console.log(`[clean taxonomy inventory --kind mutation] roots=${inventory.roots.length} records=${inventory.records.length} violations=${inventory.violations.length}`);
    return;
  }
  if (operation === "plan") {
    if (!options.baseline) throw new Error("[clean taxonomy plan --kind mutation] --baseline <commit> is required.");
    const plan = planMutationTaxonomy(inventoryMutationTaxonomy(root, inventoryOptions), options.baseline);
    const path = options.plan ? taxonomyCliGuardedPath(root, options.plan, "--plan") : ticketDir ? taxonomyCliArtifactPath(ticketDir, operation, "json") : undefined;
    if (path) taxonomyCliWriteJson(path, plan);
    if (options.format === "json" || !path) taxonomyCliPrintJson(plan);
    else console.log(`[clean taxonomy plan --kind mutation] moves=${plan.moves.length} unresolved=${plan.unresolved.length} digest=${plan.planDigest} -> ${path}`);
    if (plan.unresolved.length > 0) throw new Error(`[clean taxonomy plan --kind mutation] blocked by ${plan.unresolved.length} semantic decision(s).`);
    return;
  }
  if (operation === "apply") {
    if (!ticketDir || !options.baseline || !options.plan) throw new Error("[clean taxonomy apply --kind mutation] --ticket, --baseline, and --plan are required.");
    mutationTaxonomyCheckCancellation(root, cancelFile ?? options.cancelFile);
    const planPath = taxonomyCliGuardedPath(root, options.plan, "--plan")!;
    const plan = JSON.parse(readFileSync(planPath, "utf8")) as MutationTaxonomyPlan;
    const { planDigest: _digest, ...unsigned } = plan;
    const digest = createHash("sha256").update(canonicalJson(unsigned)).digest("hex");
    if (plan.kind !== "mutation" || plan.schemaVersion !== 1 || digest !== plan.planDigest || plan.baselineCommit !== options.baseline) throw new Error("[clean taxonomy apply --kind mutation] plan identity or baseline is invalid.");
    const terminalVerification = verifyMutationTaxonomy(root, inventoryOptions);
    if (terminalVerification.inventoryDigest !== plan.inventoryDigest) throw new Error("[clean taxonomy apply --kind mutation] fresh inventory digest does not match the plan.");
    if (plan.unresolved.length > 0 || plan.moves.length > 0) throw new Error("[clean taxonomy apply --kind mutation] direct cutover plans must be semantically integrated before apply; no partial compatibility move is permitted.");
    if (!terminalVerification.clean) throw new Error(`[clean taxonomy apply --kind mutation] terminal verification has ${terminalVerification.violations.length} violation(s).`);
    if (mutationTaxonomySourceSnapshot(root, inventoryOptions).sourceTreeDigest !== terminalVerification.inventoryDigest) throw new Error("[clean taxonomy apply --kind mutation] source changed after terminal verification.");
    mutationTaxonomyCheckCancellation(root, cancelFile ?? options.cancelFile);
    const result = { schemaVersion: 1 as const, kind: "mutation" as const, state: "committed" as const, planDigest: plan.planDigest, inventoryDigest: terminalVerification.inventoryDigest, terminalVerification, appliedMoves: 0 };
    taxonomyCliWriteJson(taxonomyCliArtifactPath(ticketDir, operation, "json"), result);
    if (options.format === "json") taxonomyCliPrintJson(result);
    else console.log(`[clean taxonomy apply --kind mutation] state=committed moves=0 digest=${plan.planDigest}`);
    taxonomyCliRequireCommittedApply(result.state);
    return;
  }
  const verification = verifyMutationTaxonomy(root, inventoryOptions);
  if (ticketDir) taxonomyCliWriteJson(taxonomyCliArtifactPath(ticketDir, operation, "json"), verification);
  if (options.format === "json") taxonomyCliPrintJson(verification);
  else console.log(`[clean taxonomy verify --kind mutation] clean=${verification.clean} errors=${verification.violations.length}`);
  if (!verification.clean) throw new Error(`[clean taxonomy verify --kind mutation] errors=${verification.violations.length}.`);
}
