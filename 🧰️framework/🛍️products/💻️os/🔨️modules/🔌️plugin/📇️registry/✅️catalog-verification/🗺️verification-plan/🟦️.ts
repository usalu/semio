/** 🗺️ Verification plan of the catalog verification: the bounded dependency order of catalog nodes and the cancellable, progress reporting execution of the plan. */
import { CATALOG_ID } from "../../🔎️discovery/🟦️.ts";
import { CATALOG_DEPENDENCY_MAX, CATALOG_NODE_MAX, type CatalogVerificationControl, type CatalogVerificationExecutor, type CatalogVerificationNode, type CatalogVerificationResult, type CatalogVerificationStatus } from "../🟦️.ts";
import { boundedCatalogDiagnostic } from "../🛡️guards/🟦️.ts";

//#region 🔖️CatalogCompleteness
/** 🧱️ Produces one stable dependency-first ordering, with ready plugins before ready extensions. */
export function orderCatalogNodes(nodes: readonly CatalogVerificationNode[]): CatalogVerificationNode[] {
  if (nodes.length === 0 || nodes.length > CATALOG_NODE_MAX) throw new Error(`catalog plan must contain 1..${CATALOG_NODE_MAX} nodes, got ${nodes.length}`);
  const byId = new Map<string, CatalogVerificationNode>();
  for (const node of nodes) {
    if (!CATALOG_ID.test(node.pluginId)) throw new Error(`catalog node has invalid plugin id ${JSON.stringify(node.pluginId)}`);
    if (node.role !== "plugin" && node.role !== "extension") throw new Error(`${node.pluginId}: invalid role ${JSON.stringify(node.role)}`);
    if (node.dependsOn.length > CATALOG_DEPENDENCY_MAX) throw new Error(`${node.pluginId}: dependencies exceed ${CATALOG_DEPENDENCY_MAX}`);
    if (byId.has(node.pluginId)) throw new Error(`catalog plan has duplicate plugin id ${JSON.stringify(node.pluginId)}`);
    if (new Set(node.dependsOn).size !== node.dependsOn.length) throw new Error(`${node.pluginId}: duplicate dependency identity`);
    byId.set(node.pluginId, node);
  }
  for (const node of nodes) {
    for (const dependency of node.dependsOn) {
      if (!CATALOG_ID.test(dependency)) throw new Error(`${node.pluginId}: invalid dependency id ${JSON.stringify(dependency)}`);
      if (!byId.has(dependency)) throw new Error(`${node.pluginId} depends on absent catalog node ${JSON.stringify(dependency)}`);
      if (dependency === node.pluginId) throw new Error(`${node.pluginId}: self dependency is a cycle`);
    }
  }
  const indegree = new Map(nodes.map((node) => [node.pluginId, node.dependsOn.length]));
  const dependents = new Map<string, string[]>();
  for (const node of nodes) for (const dependency of node.dependsOn) dependents.set(dependency, [...(dependents.get(dependency) ?? []), node.pluginId]);
  const compare = (left: CatalogVerificationNode, right: CatalogVerificationNode): number => (left.role === right.role ? left.pluginId.localeCompare(right.pluginId) : left.role === "plugin" ? -1 : 1);
  const ready = nodes.filter((node) => indegree.get(node.pluginId) === 0).sort(compare);
  const ordered: CatalogVerificationNode[] = [];
  while (ready.length > 0) {
    const node = ready.shift()!;
    ordered.push(node);
    for (const dependentId of dependents.get(node.pluginId) ?? []) {
      const next = indegree.get(dependentId)! - 1;
      indegree.set(dependentId, next);
      if (next === 0) {
        ready.push(byId.get(dependentId)!);
        ready.sort(compare);
      }
    }
  }
  if (ordered.length !== nodes.length) throw new Error(`catalog dependency cycle includes ${nodes.filter((node) => !ordered.includes(node)).map(({ pluginId }) => pluginId).sort().join(", ")}`);
  return ordered;
}

/** 🧾️ Verifies a bounded graph and exposes exactly one all-row publication call after full success. */
export async function executeCatalogVerificationPlan<Receipt>(nodes: readonly CatalogVerificationNode[], executor: CatalogVerificationExecutor<Receipt>, control: CatalogVerificationControl): Promise<CatalogVerificationResult> {
  const ordered = orderCatalogNodes(nodes);
  const results: { pluginId: string; status: CatalogVerificationStatus; diagnostic?: string }[] = [];
  const resultById = new Map<string, CatalogVerificationStatus>();
  const verified: { node: CatalogVerificationNode; receipt: Receipt }[] = [];
  for (const node of ordered) {
    let status: CatalogVerificationStatus;
    let diagnostic: string | undefined;
    if (control.cancelled()) {
      status = "cancelled";
      diagnostic = "catalog verification cancelled";
    } else {
      const unavailable = node.dependsOn.find((dependency) => resultById.get(dependency) !== "verified");
      if (unavailable) {
        status = "blocked";
        diagnostic = `dependency ${unavailable} was not verified`;
      } else {
        try {
          verified.push({ node, receipt: await executor.verify(node) });
          status = "verified";
        } catch (error) {
          status = "failed";
          diagnostic = boundedCatalogDiagnostic(error);
        }
      }
    }
    resultById.set(node.pluginId, status);
    results.push({ pluginId: node.pluginId, status, ...(diagnostic ? { diagnostic } : {}) });
    control.progress?.({ schemaVersion: 1, completed: results.length, total: ordered.length, pluginId: node.pluginId, status });
  }
  if (results.some(({ status }) => status !== "verified") || control.cancelled()) return { schemaVersion: 1, order: ordered.map(({ pluginId }) => pluginId), results, publication: "withheld" };
  if (!executor.publish) return { schemaVersion: 1, order: ordered.map(({ pluginId }) => pluginId), results, publication: "not-requested" };
  try {
    await executor.publish(verified);
    return { schemaVersion: 1, order: ordered.map(({ pluginId }) => pluginId), results, publication: "committed" };
  } catch (error) {
    return { schemaVersion: 1, order: ordered.map(({ pluginId }) => pluginId), results, publication: "failed", publicationDiagnostic: boundedCatalogDiagnostic(error) };
  }
}
//#endregion 🔖️CatalogCompleteness
