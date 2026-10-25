import { readFileSync } from "node:fs";
import { join } from "node:path";
import ts from "typescript";

/** 🏭️ The physical owners of the normalization engine, in dependency order. */
export const NORMALIZATION_ENGINE_OWNERS: readonly string[] = ["📜️plan-contracts/🟦️.ts", "🏷️file-kind-matching/🟦️.ts", "🧾️plan-parsing/🟦️.ts", "🔤️reference-tokenizers/🟦️.ts", "🧷️typescript-reference-authority/🟦️.ts", "💾️durable-journal/🟦️.ts", "🩹️recovery-authorities/🟦️.ts", "🏭️generator-inputs/🟦️.ts", "✂️reference-edits/🟦️.ts", "➰️symlink-targets/🟦️.ts", "🧱️inventory-leaf-stamps/🟦️.ts", "📸️repository-capture/🟦️.ts", "👥️mutation-projection/🟦️.ts", "📏️fixed-contracts/🟦️.ts", "🗺️artifact-projection-catalogs/🟦️.ts", "🧐️preimage-assertions/🟦️.ts", "📂️owned-file-projection/🟦️.ts", "🏛️repository-authority/🟦️.ts", "🔗️reference-inventory/🟦️.ts", "🌳️tree-digests/🟦️.ts", "⏪️rollback-reconciliation/🟦️.ts", "🔐️transaction-lease/🟦️.ts", "🛟️recovery/🟦️.ts", "🚿️transaction-cleanup/🟦️.ts", "🔄️journal-reconciliation/🟦️.ts", "🔑️transaction-lease-acquisition/🟦️.ts", "🗃️affected-state/🟦️.ts", "📃️plan-document/🟦️.ts", "❄️frozen-evidence/🟦️.ts", "🦀️rust-reference-graph/🟦️.ts", "🧭️reference-resolution/🟦️.ts", "🧮️reference-digests/🟦️.ts", "♻️projection-staleness/🟦️.ts", "↩️rollback-transaction/🟦️.ts", "🗑️removal-references/🟦️.ts", "🔬️reference-analysis/🟦️.ts", "📥️incoming-references/🟦️.ts", "🪞️artifact-reference-projection/🟦️.ts", "🛫️embedded-roots/🟦️.ts", "🕸️reference-graph/🟦️.ts", "🔍️inventory-planning/🟦️.ts", "🪢️symlink-target-edits/🟦️.ts", "🧠️planning/🟦️.ts", "✅️verification/🟦️.ts", "🚀️apply/📜️plan-checks/🟦️.ts", "🚀️apply/🔎️attempt-discovery/🟦️.ts", "🚀️apply/🎯️attempt-selection/🟦️.ts", "🚀️apply/🛫️preflight-and-lock/🟦️.ts", "🚀️apply/🪪️journal-preparation/🟦️.ts", "🚀️apply/🏁️transaction-run/🔧️transaction-steps/🟦️.ts", "🚀️apply/🏁️transaction-run/🟦️.ts", "🚀️apply/🟦️.ts"];

/** 📸️ Captures exact current owner bytes independently of compiler declaration selection. */
export function normalizationSourceFiles(normalizationDirectory: string, read: (path: string) => Buffer = readFileSync): ReadonlyMap<string, Buffer> {
  const root = normalizationDirectory;
  const owners = [...NORMALIZATION_ENGINE_OWNERS, "🛣️path/🟦️.ts", "📁️input/🟦️.ts", "🏃️operation/🟦️.ts", "🔣️taxonomy/📐️kind-specs/🟦️.ts", "🔣️taxonomy/🗃️owned-file-projection/🟦️.ts", "🔣️taxonomy/🧭️path-projection/🟦️.ts", "🔣️taxonomy/🌳️descendants/🟦️.ts", "🔣️taxonomy/🏭️generators-packages/🟦️.ts", "🔣️taxonomy/📚️loaded/🟦️.ts", "🔣️taxonomy/🧰️taxonomy-parse-helpers/🟦️.ts", "🔣️taxonomy/🏷️filenames/🟦️.ts", "🔣️taxonomy/🧱️kinds-and-fixed-contracts/🟦️.ts", "🔣️taxonomy/🌳️members-renderers-and-descendants/🟦️.ts", "🔣️taxonomy/🪞️projections-and-consumers/🟦️.ts", "🔣️taxonomy/🏭️generators-packages-and-exclusions/🟦️.ts", "🔣️taxonomy/🟦️.ts", "🚪️source-admission/🟦️.ts", "🚪️source-admission/📁️io/🟦️.ts", "../🧾️serialization/🔣️json/🟦️.ts", "../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts"];
  return new Map(owners.map((owner) => { const path = join(root, owner); return [path, read(path)]; }));
}

/** 🔬️ Reads current declarations from their physical owners for isolated compiler oracles. */
export function normalizationSourceDeclarations(normalizationDirectory: string, read: (path: string) => Buffer = readFileSync): string {
  const engine = new Set(NORMALIZATION_ENGINE_OWNERS.map((owner) => join(normalizationDirectory, owner)));
  return [...normalizationSourceFiles(normalizationDirectory, read)].flatMap(([path, bytes]) => {
    const file = ts.createSourceFile(path, bytes.toString("utf8"), ts.ScriptTarget.Latest, true);
    return file.statements.filter((node) => !ts.isImportDeclaration(node) && !ts.isExportDeclaration(node) && !(path.includes("/🧾️serialization/") && ts.isTypeAliasDeclaration(node))).map((node) => (engine.has(path) ? node.getText(file).replace(/^export /u, "") : node.getText(file)));
  }).join("\n");
}

/** ⚙️ Reads the declarations of the normalization engine alone, in dependency order and without export modifiers, for isolated compiler oracles. */
export function normalizationEngineDeclarations(normalizationDirectory: string, read: (path: string) => Buffer = readFileSync): string {
  const root = normalizationDirectory;
  return NORMALIZATION_ENGINE_OWNERS.flatMap((owner) => {
    const path = join(root, owner), file = ts.createSourceFile(path, read(path).toString("utf8"), ts.ScriptTarget.Latest, true);
    return file.statements.filter((node) => !ts.isImportDeclaration(node) && !ts.isExportDeclaration(node)).map((node) => node.getText(file).replace(/^export /u, ""));
  }).join("\n");
}
