import { readFileSync } from "node:fs";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";

export type PrintDocument = { readonly id: string; readonly texPath: string; readonly collection: "templates" | "visualizations"; readonly sources: readonly string[] };
const catalog: { version: number; sourceDateEpoch: number; documents: PrintDocument[] } = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
const product = "🧰️framework/🛍️products/📓️print";
if (catalog.version !== 1 || !Number.isSafeInteger(catalog.sourceDateEpoch) || catalog.sourceDateEpoch < 0 || !catalog.documents.length) throw new Error("Invalid Print document catalog");
const ids = new Set<string>(), paths = new Set<string>();
for (const document of catalog.documents) {
  if (!/^[a-z]+(?:-[a-z0-9]+)*$/.test(document.id) || ids.has(document.id) || paths.has(document.texPath) || !["templates", "visualizations"].includes(document.collection) || !document.texPath.endsWith(".tex")) throw new Error(`Invalid Print document: ${document.id}`);
  for (const path of [document.texPath, ...document.sources]) if (!path || path.includes("\\") || path.startsWith("/") || path.split("/").some(part => [".", "..", ""].includes(part))) throw new Error(`Invalid Print source: ${path}`);
  ids.add(document.id); paths.add(document.texPath);
}

/** 📇️ Lists the authored document owners used by the compiler and Nx inference. */
export function printDocuments(): readonly PrintDocument[] { return catalog.documents; }

/** 📄️ Resolves one declared document without accepting filesystem paths as identifiers. */
export function printDocument(id: string): PrintDocument {
  const document = catalog.documents.find(document => document.id === id);
  if (!document) throw new Error(`Unknown Print document: ${id}`);
  return document;
}

/** 📦️ Gives each document exclusive ownership of its final PDF directory. */
export function printDocumentOutputDirectory(id: string, workspace = getWorkspaceRoot()): string {
  return join(workspace, product, "📦️packages/🟦️typescript/dist/documents", printDocument(id).id);
}

/** 🕰️ Fixes TeX timestamps and date-dependent content to the authored reproducibility epoch. */
export function printSourceDateEpoch(): string { return String(catalog.sourceDateEpoch); }
