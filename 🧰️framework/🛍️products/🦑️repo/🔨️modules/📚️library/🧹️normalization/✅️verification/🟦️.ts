import { loadTaxonomy } from "../🔣️taxonomy/🟦️.ts";
import { sourceAdmissionCapture } from "../🚪️source-admission/📁️io/🟦️.ts";
import { existsSync } from "node:fs";
import { join } from "node:path";
import type { TaxonomyInventory, TaxonomyInventoryOptions, TaxonomyScopeVerification, TaxonomyScopesOptions, TaxonomyVerification } from "../📜️plan-contracts/🟦️.ts";
import type { TaxonomyRepositoryCapture } from "../🔗️reference-inventory/🟦️.ts";
import { repositoryHead, taxonomyClosedScope, taxonomyPlanVerdict } from "../🌳️tree-digests/🟦️.ts";
import { referenceTokenStore } from "../🧮️reference-digests/🟦️.ts";
import { incomingReferenceScan } from "../📥️incoming-references/🟦️.ts";
import { inventoryTaxonomy, inventoryTaxonomyWithSourceParentPruning } from "../🔍️inventory-planning/🟦️.ts";
import { planTaxonomy } from "../🧠️planning/🟦️.ts";

/** 🔍️ Verifies that an admitted scope has no unresolved taxonomy problem, move, or structured reference edit. */
export function verifyTaxonomy(options: TaxonomyInventoryOptions): TaxonomyVerification {
  const inventory = inventoryTaxonomy(options);
  const plan = planTaxonomy(inventory, {
    baselineCommit: options.baselineCommit ?? repositoryHead(inventory.repoRoot),
    excludedTreeDigests: options.excludedTreeDigests ?? [],
    cancelFile: options.cancelFile,
    progress: options.progress,
  });
  if (!inventory.scope) referenceTokenStore(inventory.repoRoot)?.retainAccessed();
  return { inventory, plan, ...taxonomyPlanVerdict(plan, options.progress) };
}

/**
 * 🗺️ Verifies many admitted scopes against ONE read of the repository: the index enumeration, repository fences and taxonomy are
 * captured once, every scope inventories only its own index rows, and — when any scope plans a move — the repository-wide
 * incoming-reference candidates and the registry catalog input view are read once for all of them. Yields each scope's verdict as soon
 * as it is planned — first every scope that plans no move, then (after the one incoming-reference scan) every scope that does, each in
 * the listed order; each verdict equals {@link verifyTaxonomy} for that scope.
 */
export function* verifyTaxonomyScopes(options: TaxonomyScopesOptions): Generator<TaxonomyScopeVerification> {
  const { scopes, ...shared } = options;
  const source = sourceAdmissionCapture(shared);
  const capture: TaxonomyRepositoryCapture = { source, taxonomy: loadTaxonomy({ repoRoot: source.prepared.repoRoot, taxonomyPath: source.prepared.taxonomyPath }) };
  const inventories = scopes.map((scope) => inventoryTaxonomyWithSourceParentPruning({ ...shared, scope }, new Set(), undefined, capture));
  const moving = (inventory: TaxonomyInventory): boolean => inventory.entries.some((entry) => entry.sourcePath !== entry.normalizedPath);
  const baselineCommit = shared.baselineCommit ?? repositoryHead(source.prepared.repoRoot);
  const verdict = (index: number): TaxonomyScopeVerification => ({ scope: scopes[index]!, ...taxonomyPlanVerdict(planTaxonomy(inventories[index]!, { baselineCommit, excludedTreeDigests: shared.excludedTreeDigests ?? [], cancelFile: shared.cancelFile, progress: shared.progress }), shared.progress) });
  for (const [index, inventory] of inventories.entries()) if (!moving(inventory)) yield verdict(index);
  capture.incoming = incomingReferenceScan(inventories, capture.taxonomy, shared);
  for (const [index, inventory] of inventories.entries()) if (moving(inventory)) yield verdict(index);
}

/** 🧵️ {@link verifyTaxonomyScopes} under closure: every scope is verified over its {@link taxonomyClosedScope} — one verification per
 * distinct closed scope — and answers only the violations at or below itself, clean when none of those is an error; a widened
 * verdict names its `closure`. A scoped inventory that held only one side of a mutation case pairing reported the other side as
 * missing; under closure such a finding is a finding of the tree. In the listed order. */
export function verifyTaxonomyScopesClosed(options: TaxonomyScopesOptions): TaxonomyScopeVerification[] {
  const { scopes, ...shared } = options;
  const own = scopes.map((scope) => scope.replace(/\/+$/u, ""));
  const taxonomy = loadTaxonomy(shared).discoverySchema;
  const closed = own.map((scope) => taxonomyClosedScope(scope, (path) => existsSync(join(shared.repoRoot, path, taxonomy.schemaScopeOwnerLevels.mutationsFacetDirName)), taxonomy));
  const verdicts = new Map<string, TaxonomyScopeVerification>();
  for (const verification of verifyTaxonomyScopes({ ...shared, scopes: [...new Set(closed)] })) verdicts.set(verification.scope, verification);
  return scopes.map((scope, index) => {
    const whole = verdicts.get(closed[index]!)!;
    if (closed[index] === own[index]) return { ...whole, scope };
    const violations = whole.violations.filter((violation) => violation.path === own[index] || violation.path.startsWith(`${own[index]}/`));
    return { scope, violations, clean: !violations.some((violation) => violation.severity === "error"), closure: closed[index]! };
  });
}
