import { foldPathEmojiIdentity, pathEmojiStatuteFindings } from "@semio-tech/framework/identity/path";
import { leadingEmojiIdentity } from "@semio-tech/framework/identity/grapheme";
import { posix } from "node:path";
import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 📙️ Audits the semantic manifest filename overrides, the subset directory overrides, the spec file kinds and the root contract id lists. */
export function auditManifestAndRootContracts(audit: TaxonomyAudit): void {
  const { taxonomy, problems, canonicalEmoji, record, ids, fileKindIdForSourcePath } = audit;
  if (taxonomy.semanticManifestFilenameOverrides !== undefined && record(taxonomy.semanticManifestFilenameOverrides, "semanticManifestFilenameOverrides")) {
    for (const [owner, name] of Object.entries(taxonomy.semanticManifestFilenameOverrides)) {
      if (!owner || owner.startsWith("/") || owner !== owner.normalize("NFC") || /[\\*?{}\0]/u.test(owner) || owner.split("/").some((segment) => !segment || segment === "." || segment === "..")) problems.push(`semanticManifestFilenameOverrides[${JSON.stringify(owner)}] must name one exact repository-relative collection owner.`);
      const identity = leadingEmojiIdentity(name);
      const statuteProblems = pathEmojiStatuteFindings([{ path: name, nodeKind: "file" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities);
      if (!canonicalEmoji(identity.first) || identity.rest !== "manifest.json" || statuteProblems.some((problem) => problem.kind !== "duplicate") || fileKindIdForSourcePath(name, taxonomy) !== taxonomy.semanticManifestFileKindId) problems.push(`semanticManifestFilenameOverrides[${JSON.stringify(owner)}] must name one canonical semantic emoji followed by "manifest.json" in the semantic manifest file kind.`);
    }
  }
  if (taxonomy.subsetDirectoryOverrides !== undefined && record(taxonomy.subsetDirectoryOverrides, "subsetDirectoryOverrides")) {
    const subsetSlug = new RegExp(taxonomy.subsetSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$", "u");
    const anyId = taxonomy.subsetAnyId ?? "*";
    for (const [owner, directories] of Object.entries(taxonomy.subsetDirectoryOverrides)) {
      if (!owner || owner.startsWith("/") || owner !== owner.normalize("NFC") || /[\\*?{}\0]/u.test(owner) || owner.split("/").some((segment) => !segment || segment === "." || segment === "..") || posix.basename(owner) !== taxonomy.subsetsDirName) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}] must name one exact repository-relative ${JSON.stringify(taxonomy.subsetsDirName)} owner.`);
      if (!record(directories, `subsetDirectoryOverrides[${JSON.stringify(owner)}]`)) continue;
      const physicalNames = new Set<string>();
      for (const [id, name] of Object.entries(directories)) {
        if (id !== anyId && !subsetSlug.test(id)) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}][${JSON.stringify(id)}] must use the canonical logical subset id grammar.`);
        const expectedRest = id === anyId ? "any" : id;
        const identity = leadingEmojiIdentity(name);
        const statuteProblems = pathEmojiStatuteFindings([{ path: name, nodeKind: "directory" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities);
        if (!canonicalEmoji(identity.first) || identity.rest !== expectedRest || statuteProblems.some((problem) => problem.kind !== "duplicate")) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}][${JSON.stringify(id)}] must name one canonical semantic emoji followed by ${JSON.stringify(expectedRest)}.`);
        if (physicalNames.has(foldPathEmojiIdentity(name))) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}] has duplicate physical directory ${JSON.stringify(name)}.`);
        physicalNames.add(foldPathEmojiIdentity(name));
      }
      if (!Object.hasOwn(directories, anyId)) problems.push(`subsetDirectoryOverrides[${JSON.stringify(owner)}] must declare the unconstrained subset id ${JSON.stringify(anyId)}.`);
    }
  }
  ids(taxonomy.textSpecFileKinds, taxonomy.fileKinds, "textSpecFileKinds");
  ids(taxonomy.binarySpecFileKinds, taxonomy.fileKinds, "binarySpecFileKinds");
  ids(taxonomy.rootDataContractIds, taxonomy.fixedFilenameContracts, "rootDataContractIds");
  ids(taxonomy.rootDocumentContractIds, taxonomy.fixedFilenameContracts, "rootDocumentContractIds");
  ids(taxonomy.repoWideContractIds, taxonomy.fixedFilenameContracts, "repoWideContractIds");
}
