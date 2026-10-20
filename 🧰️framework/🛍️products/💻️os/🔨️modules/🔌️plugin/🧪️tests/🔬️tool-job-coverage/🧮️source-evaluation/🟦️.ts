/** 🧮️ The tool job source evaluator and the exact bounded first step proof builder shared by the proof catalog phases. */
import { toolJobStaticRows, toolJobDispositions, toolJobProofs, toolJobAppOwnedRows, toolJobProofCatalogFailures } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts";

/** ⚗ Evaluates the tool job rows that remain unmigrated for a map of owner sources. */
export const evaluateToolJobSources = (sources: Record<string, string>) => {
  const files = new Map(Object.entries(sources));
  const rows = toolJobStaticRows(files);
  const dispositions = toolJobDispositions(files);
  const proofs = toolJobProofs(files);
  const appOwned = new Set(toolJobAppOwnedRows(files).map((row) => `${row.ownerFile}\0${row.toolId}`));
  const catalogClean = toolJobProofCatalogFailures(files, rows, dispositions, proofs).length === 0;
  const registered = new Set(rows.map((row) => row.id));
  const remaining = rows.filter((row) => {
    const disposition = dispositions.get(`${row.file}\0${row.id}`);
    const unbounded = /for\s+item\s+in\s+items\b|while\s+true\b/.test(sources[row.file] ?? "");
    const proof = catalogClean ? proofs.find((candidate) => candidate.ownerFile === row.file && candidate.ownerTypeName.length > 0 && candidate.controllerId.length > 0 && candidate.factory.length > 0 && candidate.toolId === row.id && candidate.documentSchema.length > 0) : undefined;
    return !["BatchOnlyPendingRewrite", "ForbiddenFromUi", "Deleted"].includes(disposition ?? "") && !(disposition === "Migrated" && proof && !unbounded && appOwned.has(`${row.file}\0${row.id}`));
  });
  for (const [file, source] of files) {
    for (const match of source.matchAll(/dispatch\(\s*"([^"]+)"/g)) if (!registered.has(match[1]!)) remaining.push({ file, id: match[1]!, source: "literal" });
  }
  return remaining;
};

/** 🪙 Builds the exact bounded first step proof source of one tool for an owner file and controller. */
export const exactBoundedFirstStepProof = (owner: string, controller: string, id: string, raw = 64) => {
  const ownerType = `Owner${owner.replaceAll(/[^A-Za-z0-9]/g, "")}`;
  return `impl ArtifactEditor for ${ownerType} { const DOCUMENT_SCHEMA: &'static str = "fixture.document"; }\nbounded_first_step_tool_proofs! { owner: semio_framework_plugin::EditorApp<${ownerType}>, owner_file: "${owner}", controller: "${controller}", artifact_schema: "fixture.document", factory: "BoundedFirstStepCommandJobFactory", tools: { "${id}" => semio_framework::ToolExecutionContract::bounded_first_step(${raw}, 1, 1, 64, 100), } }`;
};
