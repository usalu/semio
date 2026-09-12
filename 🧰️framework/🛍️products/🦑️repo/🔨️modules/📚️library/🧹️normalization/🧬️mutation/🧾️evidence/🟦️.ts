import { dirname, join, posix, relative, resolve } from "node:path";
import { canonicalPrimaryFilenameForKind, loadTaxonomy, type BreachRecord } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { inspectRustModuleGraph, inspectRustModuleGraphFacts, inspectRustStructure } from "../../../🔍️discovery/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME, policyArtifactRootOfMutationsDir, policyStripEmoji } from "../🪪️identity/🟦️.ts";
import { mutationTaxonomyCancelled, mutationTaxonomyCompare, mutationTaxonomyStructuralView, type MutationTaxonomyAssignmentRow, type MutationTaxonomyInventoryOptions, type MutationTaxonomySourceRecord } from "../📸️captured-source/🟦️.ts";
import { mutationTaxonomySourceIndex, mutationTaxonomySourceSnapshot, type MutationTaxonomySourceIndex } from "../📇️index/🟦️.ts";
import { policyKebabToPascal, policyMutationEnumVariantNames, policyMutationSemanticIdentity, policyMutationStructuralBreachesView, policyStructuralMutationDirs, policyStructuralSource } from "../📐️structural-reachability/🟦️.ts";

//#region 🧬️MutationTaxonomyWorkflow
export interface MutationTaxonomyRecord {
  readonly ownerPath: string;
  readonly artifact: string | null;
  readonly standard: string | null;
  readonly subset: string | null;
  readonly mutationRootPath: string;
  readonly currentCentralComponent: string | null;
  readonly aggregateVariant: string | null;
  readonly payloadLocations: readonly string[];
  readonly applyLocations: readonly string[];
  readonly diffLocations: readonly string[];
  readonly inverseLocations: readonly string[];
  readonly outcomeLocations: readonly string[];
  readonly textCodecLocations: readonly string[];
  readonly binaryCodecLocations: readonly string[];
  readonly schemaAndLanguageSurfaces: readonly string[];
  readonly catalogAndRegistryConsumers: readonly string[];
  readonly commandsEditorsViewers: readonly string[];
  readonly testsFixturesExamplesOracles: readonly string[];
  readonly sharedHelpers: readonly string[];
  readonly crossOwnerDependencies: readonly string[];
  readonly violationClasses: readonly string[];
  readonly targetMutationDirectoryName: string;
  readonly assignedLunaAuditor: string | null;
  readonly assignedTerraExecutor: string | null;
  readonly assignedRootIntegrator: string | null;
  readonly state: "legacy" | "direct" | "central-only";
  readonly structuralState: "legacy" | "direct" | "central-only";
  readonly executionState: "unassigned" | "assigned" | "conflicting" | "invalid";
  readonly consumerEdges: readonly MutationTaxonomyConsumerEdge[];
  readonly assignmentEvidence: MutationTaxonomyAssignmentEvidence;
  readonly evidence: MutationTaxonomyEvidence;
}


export type MutationTaxonomyConsumerKind = "catalog" | "command" | "cross-owner" | "editor" | "leaf" | "oracle" | "registry" | "sibling-operation" | "test" | "viewer";

export interface MutationTaxonomyConsumerEdge { readonly sourcePath: string; readonly targetPath: string; readonly kind: MutationTaxonomyConsumerKind; readonly relation: "import" | "mount" | "reexport" }

export interface MutationTaxonomyAssignmentEvidence { readonly status: "resolved" | "missing" | "conflicting" | "invalid"; readonly ledgerPath: string | null; readonly rows: readonly MutationTaxonomyAssignmentRow[]; readonly reason: string | null }

export interface MutationTaxonomyEvidence { readonly sourceFiles: readonly string[]; readonly resolvedMounts: readonly MutationTaxonomyConsumerEdge[]; readonly unresolvedEdges: readonly { readonly sourcePath: string; readonly specifier: string; readonly reason: string }[] }


export interface MutationTaxonomyInventory {
  readonly schemaVersion: 2;
  readonly kind: "mutation";
  readonly sourceTreeDigest: string;
  readonly roots: readonly string[];
  readonly sourceRoster: readonly MutationTaxonomySourceRecord[];
  readonly records: readonly MutationTaxonomyRecord[];
  readonly unresolved: readonly { readonly path: string; readonly reason: string }[];
  readonly violations: readonly BreachRecord[];
}


export function mutationTaxonomySegmentAfter(path: string, marker: string): string | null {
  const parts = path.split("/");
  const index = parts.indexOf(marker);
  return index >= 0 ? policyStripEmoji(parts[index + 1] ?? "") || null : null;
}


export function mutationTaxonomyLocations(contents: ReadonlyMap<string, string>, files: readonly string[], pattern: RegExp): string[] {
  return files.filter((file) => pattern.test(contents.get(file) ?? "")).sort();
}


export function mutationTaxonomyLeafAlias(path: string): string {
  return policyStripEmoji(path.split("/").at(-2) ?? "").replaceAll("-", "_");
}


export function mutationTaxonomyFileAlias(path: string): string {
  return policyStripEmoji(path.split("/").at(-1) ?? "").split(".")[0]!.replaceAll("-", "_");
}


export function mutationTaxonomyConsumerKind(sourcePath: string, targetOwner: string, sourceOwner: string): MutationTaxonomyConsumerKind {
  const segments = sourcePath.split("/").map((segment) => policyStripEmoji(segment));
  if (targetOwner !== "" && sourceOwner !== targetOwner && sourcePath.includes("/🧬️mutations/")) return "cross-owner";
  if (segments.includes("command")) return "command";
  if (segments.includes("editor")) return "editor";
  if (segments.includes("viewer")) return "viewer";
  if (segments.includes("catalog")) return "catalog";
  if (segments.includes("registry")) return "registry";
  if (segments.includes("oracle")) return "oracle";
  if (segments.includes("tests")) return "test";
  if (segments.includes("operations")) return "sibling-operation";
  return "leaf";
}


export function mutationTaxonomyRustSpecs(source: string): readonly { readonly specifier: string; readonly relation: "import" | "reexport"; readonly modulePath: readonly string[] }[] {
  return inspectRustModuleGraphFacts(source).uses.map(({ specifier, relation, modulePath }) => ({ specifier, relation, modulePath }));
}


export function mutationTaxonomyTsSpecs(source: string): readonly { readonly specifier: string; readonly relation: "import" | "reexport"; readonly modulePath: readonly string[] }[] {
  return [...source.replace(/\/\*[\s\S]*?\*\//gu, "").replace(/(^|\s)\/\/.*$/gmu, "$1").matchAll(/(?:^|\n)\s*(import|export)\b[^\n]*?\bfrom\s*["']([^"']+)["']/gu)].map((match) => ({ specifier: match[2]!, relation: match[1] === "export" ? "reexport" as const : "import" as const, modulePath: [] }));
}


export function mutationTaxonomyAssignment(rows: readonly MutationTaxonomyAssignmentRow[], ledger: MutationTaxonomySourceIndex["ledger"], mutationRootPath: string, targetMutationDirectoryName: string): MutationTaxonomyAssignmentEvidence {
  if (ledger.invalidReason) return { status: "invalid", ledgerPath: ledger.path, rows: [], reason: ledger.invalidReason };
  const matches = rows.filter((row) => row.mutationRootPath === mutationRootPath && row.targetMutationDirectoryName === targetMutationDirectoryName);
  if (matches.length === 0) return { status: "missing", ledgerPath: ledger.path, rows: [], reason: "No explicit assignment ledger row exists for this exact mutation root and leaf." };
  if ((["lunaAuditor", "terraExecutor", "rootIntegrator"] as const).some((field) => new Set(matches.map((row) => row[field])).size !== 1)) return { status: "conflicting", ledgerPath: ledger.path, rows: matches, reason: "Conflicting explicit assignment ledger rows exist for this exact mutation root and leaf." };
  return { status: "resolved", ledgerPath: ledger.path, rows: matches, reason: null };
}


export type MutationTaxonomyRustModuleContext = { readonly crateRoot: string; readonly modulePath: readonly string[]; readonly sourceScope: readonly string[]; readonly moduleBase: string };

export type MutationTaxonomyRustModuleGraph = { readonly targets: ReadonlyMap<string, string>; readonly contexts: ReadonlyMap<string, readonly MutationTaxonomyRustModuleContext[]>; readonly namedCrates: ReadonlyMap<string, readonly string[]>; readonly dependencies: ReadonlyMap<string, readonly string[]> };


export function mutationTaxonomyRustModuleKey(crateRoot: string, modulePath: readonly string[]): string {
  return `${crateRoot}\0${modulePath.join("::")}`;
}


/** 🦀️ Builds only crate/module edges demonstrated by a mounted Rust source graph. */
export function mutationTaxonomyRustModuleGraph(files: readonly string[], contents: ReadonlyMap<string, string>): MutationTaxonomyRustModuleGraph {
  return inspectRustModuleGraph(files, (path) => contents.get(path), { conventionalRoots: true });
}


export function mutationTaxonomyRustUsePath(specifier: string): string[] | null {
  const match = /^\s*((?:::)?(?:[A-Za-z_][A-Za-z0-9_]*)(?:::[A-Za-z_][A-Za-z0-9_]*)*)/u.exec(specifier);
  return match ? match[1]!.replace(/^::/u, "").split("::") : null;
}


export function mutationTaxonomyResolveRustGraphTargets(sourcePath: string, specifier: string, modulePathInSource: readonly string[], graph: MutationTaxonomyRustModuleGraph): string[] {
  const parts = mutationTaxonomyRustUsePath(specifier);
  const sourceContexts = (graph.contexts.get(sourcePath) ?? []).filter((context) => context.sourceScope.join("::") === modulePathInSource.join("::"));
  if (!parts || sourceContexts.length !== 1) return [];
  const source = sourceContexts[0]!;
  let crateRoots: readonly string[] = [source.crateRoot];
  let modulePath: string[];
  if (parts[0] === "crate") modulePath = parts.slice(1);
  else if (parts[0] === "self") modulePath = [...source.modulePath, ...parts.slice(1)];
  else if (parts[0] === "super") {
    let offset = 0;
    while (parts[offset] === "super") offset += 1;
    if (offset > source.modulePath.length) return [];
    modulePath = [...source.modulePath.slice(0, source.modulePath.length - offset), ...parts.slice(offset)];
  } else {
    crateRoots = (graph.dependencies.get(source.crateRoot) ?? []).includes(parts[0]!) ? graph.namedCrates.get(parts[0]!) ?? [] : [];
    modulePath = parts.slice(1);
  }
  if (crateRoots.length !== 1) return [];
  for (let length = modulePath.length; length > 0; length -= 1) {
    const target = graph.targets.get(mutationTaxonomyRustModuleKey(crateRoots[0]!, modulePath.slice(0, length)));
    if (target) return [target];
  }
  return [];
}


export function mutationTaxonomyResolveTargets(repoRoot: string, taxonomy: ReturnType<typeof loadTaxonomy>, sourcePath: string, source: string, specifier: string, files: readonly string[], rustGraph: MutationTaxonomyRustModuleGraph, modulePathInSource: readonly string[] = []): string[] {
  if (specifier.startsWith(".")) {
    const base = posix.normalize(posix.join(posix.dirname(sourcePath), specifier));
    const componentFiles = Object.values(taxonomy.componentFileKinds).map((kind) => canonicalPrimaryFilenameForKind(kind, taxonomy));
    return [base, `${base}.ts`, `${base}.tsx`, `${base}.rs`, `${base}.json`, ...componentFiles.map((file) => `${base}/${file}`)].filter((candidate) => files.includes(candidate));
  }
  if (!sourcePath.endsWith(".rs")) return [];
  const graphTargets = mutationTaxonomyResolveRustGraphTargets(sourcePath, specifier, modulePathInSource, rustGraph);
  if (graphTargets.length > 0) return graphTargets;
  if (/^(?:self|super|crate)::/u.test(specifier.trim())) return [];
  const first = specifier.split("::")[0]?.trim();
  if (!first) return [];
  const declaration = inspectRustModuleGraphFacts(source).modules.find((module) => module.modulePath.length === modulePathInSource.length + 1 && module.modulePath.slice(0, -1).join("::") === modulePathInSource.join("::") && module.name === first && !module.inline && module.pathTarget !== null);
  if (!declaration?.pathTarget) return [];
  const target = relative(repoRoot, resolve(dirname(join(repoRoot, sourcePath)), declaration.pathTarget)).replaceAll("\\", "/");
  if (files.includes(target)) return [target];
  return [];
}


/** 📊️ Builds the deterministic direct-leaf mutation inventory without traversing opaque roots. */
export function inventoryMutationTaxonomy(repoRoot: string, options: MutationTaxonomyInventoryOptions = {}): MutationTaxonomyInventory {
  for (let attempt = 0; attempt < 2; attempt += 1) {
    const before = mutationTaxonomySourceSnapshot(repoRoot, options);
    const roots = before.roots;
    const view = mutationTaxonomyStructuralView(before);
    const violations = policyMutationStructuralBreachesView(view, roots);
    const records: MutationTaxonomyRecord[] = [];
    const leaves: { rootRel: string; leafRel: string; leafFiles: string[]; identity: string; folder: string | undefined; variants: string[] }[] = [];
    for (let rootIndex = 0; rootIndex < roots.length; rootIndex += 1) {
      mutationTaxonomyCancelled(repoRoot, options);
      const rootRel = roots[rootIndex]!;
      const files = before.files.filter((file) => file.startsWith(`${rootRel}/`));
      const rootComponent = `${rootRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
      const rootSource = policyStructuralSource(view, rootComponent) ?? "";
      const variants = policyMutationEnumVariantNames(rootSource);
      const folders = policyStructuralMutationDirs(view, rootRel);
      const ownerTaxonomy = JSON.parse(view.taxonomySchema.bytes.toString("utf8")) as ReturnType<typeof loadTaxonomy>;
      const folderByVariant = new Map(folders.map((folder) => [policyKebabToPascal(policyMutationSemanticIdentity(rootRel, folder, ownerTaxonomy)), folder]));
      const identities = [...new Set([...folders.map((folder) => policyKebabToPascal(policyMutationSemanticIdentity(rootRel, folder, ownerTaxonomy))), ...variants])].sort();
      for (const identity of identities) {
        const folder = folderByVariant.get(identity);
        const semantic = folder ? policyMutationSemanticIdentity(rootRel, folder, ownerTaxonomy) : identity.replace(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase();
        const leafRel = folder ? `${rootRel}/${folder}` : rootRel;
        const leafFiles = files.filter((file) => file === leafRel || file.startsWith(`${leafRel}/`));
        leaves.push({ rootRel, leafRel, leafFiles, identity, folder, variants });
      }
    }
    const edgesByTarget = new Map<string, MutationTaxonomyConsumerEdge[]>();
    const helpersByLeaf = new Map<string, string[]>();
    const unresolvedByLeaf = new Map<string, { sourcePath: string; specifier: string; reason: string }[]>();
    const unresolvedBySource = new Map<string, { sourcePath: string; specifier: string; reason: string }[]>();
    const rustGraph = mutationTaxonomyRustModuleGraph(before.files, before.contents), taxonomy = JSON.parse(view.taxonomySchema.bytes.toString("utf8")) as ReturnType<typeof loadTaxonomy>;
    for (const [index, sourcePath] of before.files.entries()) {
      mutationTaxonomyCancelled(repoRoot, options);
      const source = before.contents.get(sourcePath) ?? "";
      const specs = sourcePath.endsWith(".rs") ? mutationTaxonomyRustSpecs(source) : sourcePath.endsWith(".ts") || sourcePath.endsWith(".tsx") ? mutationTaxonomyTsSpecs(source) : [];
      for (const { specifier, relation, modulePath = [] } of specs) {
        const sourceLeaf = leaves.find((leaf) => sourcePath.startsWith(`${leaf.leafRel}/`));
        const targets = mutationTaxonomyResolveTargets(repoRoot, taxonomy, sourcePath, source, specifier, before.files, rustGraph, modulePath);
        if (targets.length > 0) {
          for (const targetPath of targets) {
            const target = leaves.find((leaf) => leaf.leafFiles.includes(targetPath));
            if (!target && sourceLeaf) helpersByLeaf.set(sourceLeaf.leafRel, [...(helpersByLeaf.get(sourceLeaf.leafRel) ?? []), targetPath]);
            const edge: MutationTaxonomyConsumerEdge = { sourcePath, targetPath, kind: mutationTaxonomyConsumerKind(sourcePath, target?.rootRel ?? "", sourceLeaf?.rootRel ?? target?.rootRel ?? ""), relation };
            edgesByTarget.set(targetPath, [...(edgesByTarget.get(targetPath) ?? []), edge]);
          }
        } else {
          const unresolvedEdge = { sourcePath, specifier, reason: "The import could not be resolved to one unambiguous mounted source path." };
          const mutationLike = /mutation/iu.test(specifier);
          if (sourceLeaf || mutationLike) unresolvedBySource.set(sourcePath, [...(unresolvedBySource.get(sourcePath) ?? []), unresolvedEdge]);
          if (sourceLeaf && mutationLike) unresolvedByLeaf.set(sourceLeaf.leafRel, [...(unresolvedByLeaf.get(sourceLeaf.leafRel) ?? []), unresolvedEdge]);
        }
      }
      options.progress?.({ operation: "inventory", phase: "consumer-graph", current: index + 1, total: before.files.length, path: sourcePath });
    }
    for (const leaf of leaves) {
      const rootComponent = `${leaf.rootRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
      const targetPath = leaf.leafFiles.find((path) => path.endsWith(`/${POLICY_RS_COMPONENT_LEAF_NAME}`));
      const rootFacts = inspectRustStructure(before.contents.get(rootComponent) ?? "");
      const rootSource = before.contents.get(rootComponent) ?? "";
      const alias = leaf.folder ? policyMutationSemanticIdentity(leaf.rootRel, leaf.folder, taxonomy).replaceAll("-", "_") : targetPath ? mutationTaxonomyLeafAlias(targetPath) : "";
      const mounted = rootFacts.modules.some((module) => module.name === alias);
      const mountedModule = rootFacts.modules.find((entry) => entry.name === alias && !entry.inline && entry.pathTarget !== null);
      const resolvedPath = mountedModule?.pathTarget ? posix.normalize(posix.join(leaf.rootRel, mountedModule.pathTarget)) : null;
      const explicitPath = new RegExp(`#\\[path\\s*=\\s*"[^"]+"\\]\\s*(?:pub\\s+)?mod\\s+${alias}\\s*;`, "u").test(rootSource);
      if (targetPath && mounted && (!explicitPath || resolvedPath === targetPath)) edgesByTarget.set(targetPath, [...(edgesByTarget.get(targetPath) ?? []), { sourcePath: rootComponent, targetPath, kind: "leaf", relation: "mount" }]);
    }
    for (const leaf of leaves) {
      const semantic = leaf.folder ? policyMutationSemanticIdentity(leaf.rootRel, leaf.folder, taxonomy) : leaf.identity.replace(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase();
      const direct = leaf.folder ? policyStructuralSource(view, `${leaf.leafRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`) !== null : false;
      const nested = leaf.folder ? policyStructuralSource(view, `${leaf.leafRel}/🦠️mutation/${POLICY_RS_COMPONENT_LEAF_NAME}`) !== null : false;
      const structuralState = direct ? "direct" as const : nested ? "legacy" as const : "central-only" as const;
      const assignmentEvidence = mutationTaxonomyAssignment(before.ledger.rows, before.ledger, leaf.rootRel, leaf.folder ?? `🧬️${semantic}`);
      const assigned = assignmentEvidence.rows[0] ?? null;
      const executionState = assignmentEvidence.status === "resolved" ? "assigned" as const : assignmentEvidence.status === "conflicting" ? "conflicting" as const : assignmentEvidence.status === "invalid" ? "invalid" as const : "unassigned" as const;
      const componentTargets = leaf.leafFiles.filter((file) => /\.(?:rs|ts|tsx|json|graphql|proto)$/u.test(file));
      const outgoingCrossOwnerEdges = [...edgesByTarget.values()].flat().filter((edge) => edge.kind === "cross-owner" && edge.sourcePath.startsWith(`${leaf.leafRel}/`));
      const consumerEdges = [...new Map([...componentTargets.flatMap((path) => edgesByTarget.get(path) ?? []), ...outgoingCrossOwnerEdges].map((edge) => [`${edge.sourcePath}\0${edge.targetPath}\0${edge.kind}\0${edge.relation}`, edge])).values()].sort((left, right) => mutationTaxonomyCompare(`${left.sourcePath}\0${left.targetPath}`, `${right.sourcePath}\0${right.targetPath}`));
      records.push({
        ownerPath: policyArtifactRootOfMutationsDir(leaf.rootRel), artifact: mutationTaxonomySegmentAfter(leaf.rootRel, "🗿️artifacts"), standard: mutationTaxonomySegmentAfter(leaf.rootRel, "🏅️standards"), subset: mutationTaxonomySegmentAfter(leaf.rootRel, "🪆️subsets"), mutationRootPath: leaf.rootRel, currentCentralComponent: policyStructuralSource(view, `${leaf.rootRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`) !== null ? `${leaf.rootRel}/${POLICY_RS_COMPONENT_LEAF_NAME}` : null, aggregateVariant: leaf.variants.includes(leaf.identity) ? leaf.identity : null,
        payloadLocations: mutationTaxonomyLocations(before.contents, leaf.leafFiles, /\b(?:struct|enum|type)\s+\w+/u), applyLocations: mutationTaxonomyLocations(before.contents, leaf.leafFiles, /\b(?:apply|transform)\b/u), diffLocations: mutationTaxonomyLocations(before.contents, leaf.leafFiles, /\bdiff\b/u), inverseLocations: mutationTaxonomyLocations(before.contents, leaf.leafFiles, /\binverse\b/u), outcomeLocations: mutationTaxonomyLocations(before.contents, leaf.leafFiles, /\bMutationOutcome\b/u), textCodecLocations: leaf.leafFiles.filter((file) => file.includes("/📝️text/") || file.endsWith(".grammar.semio")), binaryCodecLocations: leaf.leafFiles.filter((file) => file.includes("/💾️binary/") || file.endsWith(".protocol.semio")), schemaAndLanguageSurfaces: leaf.leafFiles.filter((file) => /\.(?:ts|tsx|graphql|proto|json)$/u.test(file) && !file.includes("/🧪️")), catalogAndRegistryConsumers: consumerEdges.filter((edge) => edge.kind === "catalog" || edge.kind === "registry").map((edge) => edge.sourcePath), commandsEditorsViewers: consumerEdges.filter((edge) => edge.kind === "command" || edge.kind === "editor" || edge.kind === "viewer").map((edge) => edge.sourcePath), testsFixturesExamplesOracles: [...leaf.leafFiles.filter((file) => /\/(?:🧪️tests|🧫️fixtures|📚️examples|🔮️oracles)\//u.test(file)), ...consumerEdges.filter((edge) => edge.kind === "test" || edge.kind === "oracle").map((edge) => edge.sourcePath)].sort(mutationTaxonomyCompare), sharedHelpers: [...new Set(helpersByLeaf.get(leaf.leafRel) ?? [])].sort(mutationTaxonomyCompare), crossOwnerDependencies: [...new Set(consumerEdges.filter((edge) => edge.kind === "cross-owner" && edge.sourcePath.startsWith(`${leaf.leafRel}/`)).map((edge) => edge.targetPath))].sort(mutationTaxonomyCompare), violationClasses: [...new Set(violations.filter((violation) => violation.scope === leaf.leafRel || violation.scope.startsWith(`${leaf.leafRel}/`) || (leaf.folder === undefined && violation.scope.startsWith(leaf.rootRel))).map((violation) => violation.kind))].sort(), targetMutationDirectoryName: leaf.folder ?? `🧬️${semantic}`, assignedLunaAuditor: assigned?.lunaAuditor ?? null, assignedTerraExecutor: assigned?.terraExecutor ?? null, assignedRootIntegrator: assigned?.rootIntegrator ?? null, state: structuralState, structuralState, executionState, consumerEdges, assignmentEvidence, evidence: { sourceFiles: leaf.leafFiles, resolvedMounts: consumerEdges.filter((edge) => edge.relation === "mount"), unresolvedEdges: (unresolvedByLeaf.get(leaf.leafRel) ?? []).sort((left, right) => mutationTaxonomyCompare(`${left.sourcePath}\0${left.specifier}`, `${right.sourcePath}\0${right.specifier}`)) },
      });
    }
    options.afterFactsCollected?.();
    records.sort((left, right) => left.mutationRootPath.localeCompare(right.mutationRootPath) || left.targetMutationDirectoryName.localeCompare(right.targetMutationDirectoryName));
    const unresolved = records.flatMap((record) => record.assignmentEvidence.status === "resolved" ? [] : [{ path: `${record.mutationRootPath}/${record.targetMutationDirectoryName}`, reason: record.assignmentEvidence.reason ?? "Assignment evidence is unresolved." }]);
    for (const record of records) for (const edge of record.evidence.unresolvedEdges) unresolved.push({ path: edge.sourcePath, reason: edge.reason });
    for (const [sourcePath, edges] of unresolvedBySource) for (const edge of edges) unresolved.push({ path: sourcePath, reason: `${edge.reason} ${edge.specifier}` });
    const after = mutationTaxonomySourceSnapshot(repoRoot, options);
    if (before.sourceTreeDigest === after.sourceTreeDigest) return { schemaVersion: 2, kind: "mutation", sourceTreeDigest: before.sourceTreeDigest, roots, sourceRoster: before.sourceRoster, records, unresolved: unresolved.sort((left, right) => mutationTaxonomyCompare(`${left.path}\0${left.reason}`, `${right.path}\0${right.reason}`)), violations };
  }
  throw new Error("[clean taxonomy --kind mutation] source changed while inventory facts were collected.");
}
