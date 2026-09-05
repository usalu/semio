import { minimatch } from "minimatch";
import type { FileKindSpec, Taxonomy } from "../../../../🔍️discovery/🟦️.ts";

//#region 🧬️SourceFileFacts
export type SourceFileFactRole = FileKindSpec["role"];
export type SourceFileFactExpected = { readonly sourcePath: string; readonly fileKindId: string | null; readonly fileRole: SourceFileFactRole | null; readonly extensionChain: string | null };
export type SourceFileFactCase = { readonly id: string; readonly catalog: "current" | "synthetic-generated" | "synthetic-tie"; readonly admissionStatus: "complete" | "rejected"; readonly sourcePath: string; readonly observedKind: "file" | "directory" | "symlink" | "absent" | "other"; readonly mode: "040000" | "100644" | "100755" | "120000" | "160000" | null; readonly repositoryBoundary: "gitlink" | null; readonly expected: SourceFileFactExpected | null };

/** 🧪️ Adds test-only catalog facts without modifying the production taxonomy. */
export function sourceFileFactCatalog(kind: SourceFileFactCase["catalog"], taxonomy: Taxonomy): Taxonomy {
  if (kind === "current") return taxonomy;
  const additions: Record<string, FileKindSpec> = kind === "synthetic-generated"
    ? { "generated-fixture": { emoji: "⚙️", extensionChains: [".generated"], role: "generated" } }
    : { "tie-left": { emoji: "1️⃣", extensionChains: [".tie"], role: "source" }, "tie-right": { emoji: "2️⃣", extensionChains: [".tie"], role: "source" } };
  return { ...taxonomy, fileKinds: { ...taxonomy.fileKinds, ...additions } };
}

/** 🧪️ Compares raw physical path spellings by UTF-8 bytes. */
export function sourceFileFactByteCompare(left: string, right: string): number {
  const leftBytes = new TextEncoder().encode(left), rightBytes = new TextEncoder().encode(right), length = Math.min(leftBytes.length, rightBytes.length);
  for (let index = 0; index < length; index += 1) if (leftBytes[index] !== rightBytes[index]) return leftBytes[index]! - rightBytes[index]!;
  return leftBytes.length - rightBytes.length;
}

/** 🧪️ Selects one unique longest suffix through test-only minimatch. */
export function sourceFileFactOracleKind(path: string, catalog: Taxonomy): { readonly fileKindId: string | null; readonly extensionChain: string | null } {
  const filename = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(catalog.unicodeNormalization.form).split("/").at(-1)!.toLocaleLowerCase("en-US");
  const candidates = Object.entries(catalog.fileKinds).flatMap(([fileKindId, spec]) => spec.extensionChains.filter((extensionChain) => minimatch(filename, `*${extensionChain}`, { nocase: true, dot: true })).map((extensionChain) => ({ fileKindId, extensionChain })));
  const longest = Math.max(0, ...candidates.map((candidate) => candidate.extensionChain.length));
  const unique = [...new Map(candidates.filter((candidate) => candidate.extensionChain.length === longest).map((candidate) => [candidate.fileKindId, candidate])).values()];
  return unique.length === 1 ? unique[0]! : { fileKindId: null, extensionChain: null };
}

/** 🧪️ Projects only complete supplied regular observations with the independent suffix oracle. */
export function sourceFileFactReference(rows: readonly SourceFileFactCase[], catalog: Taxonomy): readonly SourceFileFactExpected[] {
  if (rows.some((row) => row.admissionStatus !== "complete")) throw new Error("source admission is rejected");
  return rows.filter((row) => row.observedKind === "file" && (row.mode === "100644" || row.mode === "100755") && row.repositoryBoundary !== "gitlink").map((row) => {
    const resolved = sourceFileFactOracleKind(row.sourcePath, catalog), role = resolved.fileKindId === null ? null : catalog.fileKinds[resolved.fileKindId]!.role;
    return { sourcePath: row.sourcePath, fileKindId: resolved.fileKindId, fileRole: role, extensionChain: resolved.extensionChain };
  }).sort((left, right) => sourceFileFactByteCompare(left.sourcePath, right.sourcePath));
}
//#endregion 🧬️SourceFileFacts
