import { pathEmojiStatuteFindings } from "@semio-tech/framework/identity/path";
import { leadingEmojiIdentity } from "@semio-tech/framework/identity/grapheme";
import { createHash } from "node:crypto";
import { basename, dirname } from "node:path";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { pathBudgetFitsWithReserve } from "../📏️path-budget/🟦️.ts";
import { canonicalSemanticDirectoryName } from "../📂️semantic-directory-kinds/🟦️.ts";
import { fixedContractFilename } from "../🎯️taxonomy-path-matching/🟦️.ts";
import { canonicalFilenameForKind, loadTaxonomy, semanticDirectoryKindId } from "../🔐️taxonomy-authority/🟦️.ts";
import { mutationCatalogSourceOwner, mutationOwnerRelativePath } from "../🪪️mutation-owner-identity/🟦️.ts";

/** 📌️ Filesystem-independent admitted facts for one owner-governed semantic leaf. */
export interface SemanticOwnedFileProjectionFacts {
  readonly ownerPath: string;
  readonly ownerFixedDirectoryContractIds: readonly string[];
  readonly manifestPath: string;
  readonly manifestFixedFilenameContractIds: readonly string[];
  readonly manifestContent: string;
  readonly sourcePath: string;
  readonly sourceFileKindId: string;
  readonly sourceByteLength: number;
}

/** 📌️ Exact authority decision for one owner-governed semantic leaf. */
export interface SemanticOwnedFileProjectionAuthority {
  readonly contractId: string;
  readonly ownerPath: string;
  readonly manifestPath: string;
  readonly manifestContentHash: string;
  readonly sourcePath: string;
  readonly destinationPath?: string;
  readonly status?: "closed" | "open";
  readonly contentState: "nonzero" | "zero-byte";
  readonly disposition: "problem" | "project" | "remove" | "unclaimed";
  readonly problems: readonly string[];
}

/** 🧭️ Resolves one semantic leaf only through its exact owner and sibling-manifest contract. */
export function semanticOwnedFileProjectionAuthority(facts: SemanticOwnedFileProjectionFacts, taxonomy: Taxonomy = loadTaxonomy()): SemanticOwnedFileProjectionAuthority {
  if (!Number.isSafeInteger(facts.sourceByteLength) || facts.sourceByteLength < 0) throw new Error("sourceByteLength must be a non-negative safe integer.");
  const contractId = "ticket-important-markdown-v1";
  const contract = taxonomy.semanticOwnedFileProjectionContracts[contractId];
  if (!contract || contract.contractKind !== "owner-sibling-manifest-file") throw new Error("The active ticket important projection contract is required.");
  const contentState = facts.sourceByteLength === 0 ? "zero-byte" : "nonzero";
  const core = { contractId, ownerPath: facts.ownerPath, manifestPath: facts.manifestPath, manifestContentHash: createHash("sha256").update(facts.manifestContent).digest("hex"), sourcePath: facts.sourcePath, contentState } as const;
  const manifestContract = taxonomy.fixedFilenameContracts[contract.requiredSiblingFixedFilenameContractId];
  const claimed = facts.ownerFixedDirectoryContractIds.includes(contract.ownerFixedDirectoryContractId)
    && facts.manifestFixedFilenameContractIds.includes(contract.requiredSiblingFixedFilenameContractId)
    && dirname(facts.sourcePath) === facts.ownerPath
    && basename(facts.sourcePath) === contract.sourceFilename
    && facts.sourceFileKindId === contract.sourceFileKindId
    && dirname(facts.manifestPath) === facts.ownerPath
    && manifestContract !== undefined
    && basename(facts.manifestPath) === fixedContractFilename(manifestContract);
  if (!claimed) return { ...core, disposition: "unclaimed", problems: [] };
  let status: "closed" | "open" | undefined;
  try {
    const manifest = JSON.parse(facts.manifestContent) as unknown;
    if (typeof manifest === "object" && manifest !== null && !Array.isArray(manifest) && Object.hasOwn(manifest, contract.manifestStatusLocation)) {
      const candidate = (manifest as Record<string, unknown>)[contract.manifestStatusLocation];
      if (candidate === "closed" || candidate === "open") status = candidate;
    }
  } catch {}
  if (!status) return { ...core, disposition: "problem", problems: ["Ticket manifest must own an explicit status equal to closed or open."] };
  const destinationPath = `${facts.ownerPath}/${contract.destinationDirectoryName}/${contract.destinationFilename}`;
  if (status === "open") return { ...core, destinationPath, status, disposition: "project", problems: [] };
  if (contentState === "zero-byte") return { ...core, status, disposition: "remove", problems: [] };
  return { ...core, destinationPath, status, disposition: "problem", problems: ["Closed ticket important document must be exactly zero bytes."] };
}

/** 📓️ Filesystem-independent admitted facts for one historical ticket note. */
export interface SemanticOwnedFileHistoryProjectionFacts {
  readonly ownerPath: string;
  readonly ownerFixedDirectoryContractIds: readonly string[];
  readonly manifestPath?: string;
  readonly manifestFixedFilenameContractIds: readonly string[];
  readonly manifestContent?: string;
  readonly sourcePath: string;
  readonly sourceFileKindId: string;
  readonly sourceByteLength: number;
}

/** 📓️ Exact authority decision for one historical ticket note. */
export interface SemanticOwnedFileHistoryProjectionAuthority {
  readonly contractId: "ticket-important-history-markdown-v1";
  readonly ownerPath: string;
  readonly manifestPath?: string;
  readonly manifestContentHash?: string;
  readonly manifestState: "closed" | "invalid" | "missing" | "open";
  readonly sourcePath: string;
  readonly destinationPath?: string;
  readonly contentState: "nonzero" | "zero-byte";
  readonly disposition: "project" | "unclaimed";
  readonly problems: readonly string[];
}

/** 🧭️ Resolves one historical note only through its exact ticket owner and residual lifecycle state. */
export function semanticOwnedFileHistoryProjectionAuthority(facts: SemanticOwnedFileHistoryProjectionFacts, taxonomy: Taxonomy = loadTaxonomy()): SemanticOwnedFileHistoryProjectionAuthority {
  if (!Number.isSafeInteger(facts.sourceByteLength) || facts.sourceByteLength < 0) throw new Error("sourceByteLength must be a non-negative safe integer.");
  const contractId = "ticket-important-history-markdown-v1" as const;
  const contract = taxonomy.semanticOwnedFileProjectionContracts[contractId];
  if (!contract || contract.contractKind !== "owner-optional-sibling-manifest-file") throw new Error("The ticket important history projection contract is required.");
  const contentState = facts.sourceByteLength === 0 ? "zero-byte" : "nonzero";
  const manifestContract = taxonomy.fixedFilenameContracts[contract.optionalSiblingFixedFilenameContractId];
  const manifestClaimed = facts.manifestPath !== undefined
    && facts.manifestContent !== undefined
    && facts.manifestFixedFilenameContractIds.includes(contract.optionalSiblingFixedFilenameContractId)
    && dirname(facts.manifestPath) === facts.ownerPath
    && manifestContract !== undefined
    && basename(facts.manifestPath) === fixedContractFilename(manifestContract);
  let manifestState: SemanticOwnedFileHistoryProjectionAuthority["manifestState"] = "missing";
  if (facts.manifestPath !== undefined || facts.manifestContent !== undefined || facts.manifestFixedFilenameContractIds.length > 0) manifestState = "invalid";
  if (manifestClaimed) {
    try {
      const manifest = JSON.parse(facts.manifestContent!) as unknown;
      const candidate = typeof manifest === "object" && manifest !== null && !Array.isArray(manifest) ? (manifest as Record<string, unknown>)[contract.manifestStatusLocation] : undefined;
      manifestState = candidate === "closed" || candidate === "open" ? candidate : "invalid";
    } catch { manifestState = "invalid"; }
  }
  const core = {
    contractId,
    ownerPath: facts.ownerPath,
    ...(facts.manifestPath === undefined ? {} : { manifestPath: facts.manifestPath }),
    ...(facts.manifestContent === undefined ? {} : { manifestContentHash: createHash("sha256").update(facts.manifestContent).digest("hex") }),
    manifestState,
    sourcePath: facts.sourcePath,
    contentState,
  } as const;
  const claimed = facts.ownerFixedDirectoryContractIds.includes(contract.ownerFixedDirectoryContractId)
    && dirname(facts.sourcePath) === facts.ownerPath
    && basename(facts.sourcePath) === contract.sourceFilename
    && facts.sourceFileKindId === contract.sourceFileKindId;
  const admitted = manifestState === "missing" || manifestState === "invalid" || manifestState === "closed" && contentState === "nonzero";
  if (!claimed || !admitted) return { ...core, disposition: "unclaimed", problems: [] };
  return { ...core, destinationPath: `${facts.ownerPath}/${contract.destinationDirectoryName}/${contract.destinationFilename}`, disposition: "project", problems: [] };
}

/** 📄️ Filesystem-independent facts for a primary owner leaf. */
export interface SemanticOwnedPrimaryFileProjectionFacts {
  readonly ownerPath: string;
  readonly ownerFixedDirectoryContractIds: readonly string[];
  readonly sourcePath: string;
  readonly sourceFileKindId: string;
}

/** 📑️ A direct physical-leaf projection that does not depend on ticket lifecycle. */
export interface SemanticOwnedPrimaryFileProjectionAuthority {
  readonly contractId: "ticket-document-primary-markdown-v1";
  readonly ownerPath: string;
  readonly sourcePath: string;
  readonly destinationPath?: string;
  readonly disposition: "project" | "unclaimed";
  readonly problems: readonly string[];
}

/** 🧭️ Drops the exact ticket document convention only under its registered semantic owner. */
export function semanticOwnedPrimaryFileProjectionAuthority(facts: SemanticOwnedPrimaryFileProjectionFacts, taxonomy: Taxonomy = loadTaxonomy()): SemanticOwnedPrimaryFileProjectionAuthority {
  const contractId = "ticket-document-primary-markdown-v1" as const;
  const contract = taxonomy.semanticOwnedFileProjectionContracts[contractId];
  if (!contract || contract.contractKind !== "owner-primary-file") throw new Error("The ticket document primary leaf projection contract is required.");
  const core = { contractId, ownerPath: facts.ownerPath, sourcePath: facts.sourcePath };
  const claimed = facts.ownerFixedDirectoryContractIds.includes(contract.ownerFixedDirectoryContractId)
    && dirname(facts.sourcePath) === facts.ownerPath
    && basename(facts.sourcePath) === contract.sourceFilename
    && facts.sourceFileKindId === contract.sourceFileKindId;
  if (!claimed) return { ...core, disposition: "unclaimed", problems: [] };
  return { ...core, destinationPath: `${facts.ownerPath}/${contract.destinationFilename}`, disposition: "project", problems: [] };
}

/** 🧭️ Resolves an empty-facet marker only through a complete registered semantic owner chain. */
export function semanticArtifactEmptyFacetProjectionAuthority(facts: Readonly<{ sourcePath: string; sourceFileKindId: string }>, taxonomy: Taxonomy = loadTaxonomy()): Readonly<{ contractId: "artifact-empty-facet-primary-markdown-v1"; sourcePath: string; ownerForm?: string; destinationPath?: string; disposition: "project" | "unclaimed" }> {
  const contractId = "artifact-empty-facet-primary-markdown-v1" as const;
  const contract = taxonomy.semanticOwnedFileProjectionContracts[contractId];
  if (!contract || contract.contractKind !== "semantic-facet-primary-file") throw new Error("The artifact empty-facet primary-leaf contract is required.");
  const core = { contractId, sourcePath: facts.sourcePath };
  const kindId = taxonomy[contract.fileKindAuthority];
  const segments = facts.sourcePath.split("/");
  if (segments.some((segment) => !segment || segment === "." || segment === ".." || /[\\\0]/u.test(segment)) || !facts.sourcePath.startsWith(`${contract.sourceRoot}/`) || basename(facts.sourcePath) !== contract.sourceFilename || facts.sourceFileKindId !== kindId) return { ...core, disposition: "unclaimed" };
  const owners = dirname(facts.sourcePath).slice(contract.sourceRoot.length + 1).split("/");
  const matches = Object.entries(contract.ownerPathPatterns).filter(([, pattern]) => {
    const expected = pattern.split("/");
    if (expected.length !== owners.length) return false;
    let parentKindId = "plugins";
    return expected.every((segment, index) => {
      const name = owners[index]!;
      const kind = semanticDirectoryKindId(name, taxonomy, { parentKindId });
      if (!kind) return false;
      parentKindId = kind;
      const capture = /^\{([a-zA-Z]+)\}$/u.exec(segment)?.[1];
      if (!capture) return canonicalSemanticDirectoryName(name, taxonomy) === segment;
      const rule = contract.directoryCaptures[capture];
      return rule !== undefined && rule.kindIds.includes(kind) && (!rule.names || rule.names.includes(canonicalSemanticDirectoryName(name, taxonomy)));
    });
  });
  if (matches.length !== 1) return { ...core, disposition: "unclaimed" };
  return { ...core, ownerForm: matches[0]![0], destinationPath: `${dirname(facts.sourcePath)}/${canonicalFilenameForKind(kindId, taxonomy)}`, disposition: "project" };
}

/** 📚️ One exact physical scenario registration owned by a MutationCatalog vector. */
export interface SemanticProjectionScenarioRegistration {
  readonly id: string;
  readonly directoryName: string;
}

/** 🦠️ One independent physical vector registration; it is not a runtime-kind declaration. */
export interface SemanticProjectionVectorRegistration {
  readonly mutationId: string;
  readonly sourceMutationDirectoryName: string;
  readonly mutationDirectoryName: string;
  readonly scenarios: readonly SemanticProjectionScenarioRegistration[];
}

/** 📖️ One owner-local catalog whose physical and canonical vector identities are validated together. */
export interface SemanticProjectionCatalogRegistration {
  readonly ownerPath: string;
  readonly catalogId: string;
  readonly vectors: readonly SemanticProjectionVectorRegistration[];
}

/** 🧾️ Validates strict source/canonical identities, ownership, collisions, and path budgets. */
export function semanticProjectionCatalogProblems(registrations: readonly SemanticProjectionCatalogRegistration[], taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const destinationOwners = new Set<string>();
  for (const [catalogIndex, catalog] of registrations.entries()) {
    const scope = `catalogs[${catalogIndex}]`;
    if (!catalog.ownerPath || !catalog.catalogId) problems.push(`${scope} must declare ownerPath and catalogId.`);
    const profile = catalog.ownerPath.match(/^(.*)\/🏅️standards\/🔖️([^/]+)\/🪆️subsets\/✳️([^/]+)$/u);
    if (!profile) problems.push(`${scope}.ownerPath is not an exact artifact standard/subset owner.`);
    const sourceTuples = new Set<string>();
    const canonicalTuples = new Set<string>();
    for (const [vectorIndex, vector] of catalog.vectors.entries()) {
      const vectorScope = `${scope}.vectors[${vectorIndex}]`;
      const keys = Object.keys(vector).sort().join("\0");
      if (keys !== ["mutationDirectoryName", "mutationId", "scenarios", "sourceMutationDirectoryName"].sort().join("\0")) problems.push(`${vectorScope} must contain exactly mutationId, sourceMutationDirectoryName, mutationDirectoryName, and scenarios.`);
      const mutationId = typeof vector.mutationId === "string" ? vector.mutationId : "";
      const sourceMutationDirectoryName = typeof vector.sourceMutationDirectoryName === "string" ? vector.sourceMutationDirectoryName : "";
      const mutationDirectoryName = typeof vector.mutationDirectoryName === "string" ? vector.mutationDirectoryName : "";
      for (const [field, value] of Object.entries({ mutationId, sourceMutationDirectoryName, mutationDirectoryName })) if (!value || value !== value.normalize("NFC") || /[\\/]/u.test(value)) problems.push(`${vectorScope}.${field} must be one non-empty NFC basename.`);
      if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(mutationId)) problems.push(`${vectorScope}.mutationId must be kebab-case.`);
      const sourceOwner = mutationCatalogSourceOwner(catalog.ownerPath, taxonomy);
      if (sourceOwner === null) problems.push(`${vectorScope} has invalid explicit catalog source ownership.`);
      const mutationRoot = `${sourceOwner ?? catalog.ownerPath}/🧬️schema/🧬️mutations`;
      const grouped = Object.hasOwn(taxonomy.mutationDomainOwners, mutationRoot);
      const registeredOwner = grouped ? mutationOwnerRelativePath(mutationRoot, mutationId, taxonomy) : null;
      const canonical = grouped ? registeredOwner ?? mutationDirectoryName : mutationDirectoryName;
      if (grouped) {
        if (!registeredOwner || registeredOwner.split("/").at(-1) !== sourceMutationDirectoryName || registeredOwner.split("/").at(-1) !== mutationDirectoryName) problems.push(`${vectorScope} has no exact registered domain-operation owner for mutationId.`);
      } else {
        if ((sourceMutationDirectoryName.match(/[a-z0-9][a-z0-9-]*$/u)?.[0] ?? "") !== mutationId) problems.push(`${vectorScope}.sourceMutationDirectoryName must render mutationId.`);
        if ((mutationDirectoryName.match(/[a-z0-9][a-z0-9-]*$/u)?.[0] ?? "") !== mutationId) problems.push(`${vectorScope}.mutationDirectoryName must render mutationId.`);
        for (const [field, name] of Object.entries({ sourceMutationDirectoryName, mutationDirectoryName })) if (leadingEmojiIdentity(name).rest === name || pathEmojiStatuteFindings([{ path: name, nodeKind: "directory" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities).length > 0) problems.push(`${vectorScope}.${field} must be one canonical physical mutation identity.`);
      }
      if (!Array.isArray(vector.scenarios) || vector.scenarios.length === 0) problems.push(`${vectorScope}.scenarios must be non-empty.`);
      const scenarioIds = new Set<string>();
      const scenarioDirectories = new Set<string>();
      for (const [scenarioIndex, scenario] of (vector.scenarios ?? []).entries()) {
        const scenarioScope = `${vectorScope}.scenarios[${scenarioIndex}]`;
        if (Object.keys(scenario).sort().join("\0") !== ["directoryName", "id"].join("\0")) problems.push(`${scenarioScope} must contain exactly id and directoryName.`);
        const physicalScenarioId = leadingEmojiIdentity(scenario.directoryName).rest;
        if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(scenario.id)) problems.push(`${scenarioScope}.id must be one kebab-case logical identity.`);
        if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(physicalScenarioId) || physicalScenarioId === scenario.directoryName || scenario.directoryName !== scenario.directoryName.normalize("NFC") || /[\\/]/u.test(scenario.directoryName) || pathEmojiStatuteFindings([{ path: scenario.directoryName, nodeKind: "directory" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities).length > 0) problems.push(`${scenarioScope}.directoryName must be one canonical NFC physical test-case identity.`);
        if (scenarioIds.has(scenario.id)) problems.push(`${scenarioScope}.id duplicates one logical scenario identity.`);
        scenarioIds.add(scenario.id);
        const scenarioDirectoryKey = scenario.directoryName.replaceAll("\uFE0F", "").toLocaleLowerCase("und");
        if (scenarioDirectories.has(scenarioDirectoryKey)) problems.push(`${scenarioScope}.directoryName duplicates one physical scenario identity.`);
        scenarioDirectories.add(scenarioDirectoryKey);
        const sourceTuple = `${mutationId}\0${sourceMutationDirectoryName}\0${scenario.id}`;
        const canonicalTuple = `${mutationId}\0${canonical}\0${scenario.id}`;
        if (sourceTuples.has(sourceTuple)) problems.push(`${scenarioScope} duplicates a source bundle tuple.`);
        if (canonicalTuples.has(canonicalTuple)) problems.push(`${scenarioScope} duplicates a canonical bundle tuple.`);
        sourceTuples.add(sourceTuple);
        canonicalTuples.add(canonicalTuple);
        if (profile) {
          const implementation = `${sourceOwner ?? catalog.ownerPath}/🧬️schema/🧬️mutations/${canonical}/🧪️tests/${scenario.directoryName}`.normalize("NFC");
          const fixture = `${sourceOwner ?? catalog.ownerPath}/🧪️tests/🧫️fixtures/🧬️mutations/${canonical}/${scenario.directoryName}`.normalize("NFC");
          for (const destination of [implementation, fixture]) {
            const destinationKey = destination.replaceAll("\uFE0F", "").toLocaleLowerCase("und");
            if (destinationOwners.has(destinationKey)) problems.push(`${scenarioScope} collides at canonical pair root ${JSON.stringify(destination)}.`);
            destinationOwners.add(destinationKey);
          }
          const implementationReserve = taxonomy.semanticDescendantContracts[taxonomy.mutationCatalogProjection.implementationDescendantContractId]?.pathBudgetReserve.utf16Units ?? taxonomy.collisionPolicy.maxPathUtf16Units;
          const fixtureReserve = taxonomy.semanticDescendantContracts[taxonomy.mutationCatalogProjection.fixtureDescendantContractId]?.pathBudgetReserve.utf16Units ?? taxonomy.collisionPolicy.maxPathUtf16Units;
          if (!pathBudgetFitsWithReserve(implementation, implementationReserve, taxonomy.collisionPolicy) || !pathBudgetFitsWithReserve(fixture, fixtureReserve, taxonomy.collisionPolicy)) problems.push(`${scenarioScope} exceeds the path budget after a canonical pair descendant reserve.`);
        }
      }
    }
  }
  return problems;
}
