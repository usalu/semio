import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import ts from "typescript";

/** 📸️ Captures exact current owner bytes independently of compiler declaration selection. */
export function normalizationSourceFiles(normalizationPath: string, read: (path: string) => Buffer = readFileSync): ReadonlyMap<string, Buffer> {
  const root = dirname(normalizationPath);
  const owners = ["🟦️.ts", "🛣️path/🟦️.ts", "📁️input/🟦️.ts", "🏃️operation/🟦️.ts", "🔣️taxonomy/🟦️.ts", "🚪️source-admission/🟦️.ts", "🚪️source-admission/📁️io/🟦️.ts", "../🧾️serialization/🔣️json/🟦️.ts", "../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts"];
  return new Map(owners.map((owner) => { const path = join(root, owner); return [path, read(path)]; }));
}

/** 🔬️ Reads current declarations from their physical owners for isolated compiler oracles. */
export function normalizationSourceDeclarations(normalizationPath: string, read: (path: string) => Buffer = readFileSync): string {
  return [...normalizationSourceFiles(normalizationPath, read)].flatMap(([path, bytes]) => {
    const file = ts.createSourceFile(path, bytes.toString("utf8"), ts.ScriptTarget.Latest, true);
    return file.statements.filter((node) => !ts.isImportDeclaration(node) && !ts.isExportDeclaration(node) && !(path.includes("/🧾️serialization/") && ts.isTypeAliasDeclaration(node))).map((node) => node.getText(file));
  }).join("\n");
}
