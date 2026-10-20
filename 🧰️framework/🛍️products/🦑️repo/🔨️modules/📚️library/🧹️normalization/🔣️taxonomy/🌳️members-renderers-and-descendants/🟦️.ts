/** 🌳️ The directory member kinds, projection renderers, descendant contracts and catalog projection contracts. */
import { posix } from "node:path";
import { requireRecord, requireString, requireStringArray, type UnknownRecord } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { canonicalPrimaryFilenameForKind } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import type { Taxonomy as DiscoveryTaxonomy } from "../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import type { FileKindSpec, FixedDirectoryContract, FixedFilenameContract, SemanticDirectoryKindSpec, SemanticDirectoryMemberKindSpec, SemanticProjectedMemberKindSpec } from "../📐️kind-specs/🟦️.ts";
import { emojiFold, splitLeadingEmoji } from "../../🛣️path/🟦️.ts";
import type { SemanticMutationPathProjectionCatalogContract, SemanticPathProjectionCatalogContract, SemanticPathProjectionProfileRenderer, SemanticProjectionCaptureField } from "../🧭️path-projection/🟦️.ts";
import type { SemanticDescendantContract, SemanticDescendantNode } from "../🌳️descendants/🟦️.ts";
import { pathUtf16Units } from "../../../🔍️discovery/📏️path-budget/🟦️.ts";

/** 👥 Parses the directory and projected member kinds and the projection profile renderers. */
export function parseMemberKindsAndRenderers({ directoryMemberRows, semanticDirectoryKinds, projectedMemberRows, fixedDirectoryContracts, projectionRendererRows }: {
  readonly directoryMemberRows: UnknownRecord;
  readonly semanticDirectoryKinds: Record<string, SemanticDirectoryKindSpec>;
  readonly projectedMemberRows: UnknownRecord;
  readonly fixedDirectoryContracts: Record<string, FixedDirectoryContract>;
  readonly projectionRendererRows: UnknownRecord;
}) {
  const semanticDirectoryMemberKinds: Record<string, SemanticDirectoryMemberKindSpec> = {};
  for (const [id, value] of Object.entries(directoryMemberRows)) {
    const spec = requireRecord(value, `semanticDirectoryMemberKinds.${id}`);
    if (spec.source !== "registry") throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id}.source must be registry`);
    const ownerKindIds = requireStringArray(spec.ownerKindIds, `semanticDirectoryMemberKinds.${id}.ownerKindIds`);
    const memberNames = requireStringArray(spec.memberNames, `semanticDirectoryMemberKinds.${id}.memberNames`);
    const memberSlugPattern = spec.memberSlugPattern === undefined ? undefined : requireString(spec.memberSlugPattern, `semanticDirectoryMemberKinds.${id}.memberSlugPattern`);
    if (ownerKindIds.length === 0 || (memberNames.length === 0 && memberSlugPattern === undefined)) throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} must declare owners and members or a memberSlugPattern`);
    if (memberSlugPattern !== undefined && (!memberSlugPattern.startsWith("^") || !memberSlugPattern.endsWith("$") || new RegExp(memberSlugPattern, "u").test(""))) throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id}.memberSlugPattern must be an anchored pattern that rejects an empty slug`);
    if (memberNames.some((name) => name !== name.normalize("NFC") || !splitLeadingEmoji(name).emoji)) throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} member names must be NFC emoji-leading evidence`);
    semanticDirectoryMemberKinds[id] = { ownerKindIds: [...new Set(ownerKindIds)].sort(), memberNames: [...new Set(memberNames)].sort(), ...(memberSlugPattern === undefined ? {} : { memberSlugPattern }), source: "registry" };
  }
  const directoryContextIds = new Set([...Object.keys(semanticDirectoryKinds), ...Object.keys(semanticDirectoryMemberKinds)]);
  for (const [id, spec] of Object.entries(semanticDirectoryMemberKinds)) for (const ownerId of spec.ownerKindIds) if (!directoryContextIds.has(ownerId)) throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} references unknown owner kind ${ownerId}`);

  const semanticProjectedMemberKinds: Record<string, SemanticProjectedMemberKindSpec> = {};
  for (const [id, value] of Object.entries(projectedMemberRows)) {
    const spec = requireRecord(value, `semanticProjectedMemberKinds.${id}`);
    if (spec.identityField !== "mutationDirectoryName" && spec.identityField !== "commandDirectoryName") throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id}.identityField is invalid`);
    const ownerKindIds = requireStringArray(spec.ownerKindIds, `semanticProjectedMemberKinds.${id}.ownerKindIds`);
    if (ownerKindIds.length === 0) throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id}.ownerKindIds must not be empty`);
    semanticProjectedMemberKinds[id] = { ownerKindIds: [...new Set(ownerKindIds)].sort(), projectionContractId: requireString(spec.projectionContractId, `semanticProjectedMemberKinds.${id}.projectionContractId`), sourceMemberKindId: requireString(spec.sourceMemberKindId, `semanticProjectedMemberKinds.${id}.sourceMemberKindId`), identityField: spec.identityField };
  }
  if (Object.keys(semanticProjectedMemberKinds).length === 0) throw new Error("Taxonomy v7 semanticProjectedMemberKinds must not be empty");
  const allDirectoryContextIds = new Set([...directoryContextIds, ...Object.keys(semanticProjectedMemberKinds), ...Object.keys(fixedDirectoryContracts)]);
  for (const [id, spec] of Object.entries(semanticDirectoryKinds)) for (const parentId of spec.parentKindIds ?? []) if (!allDirectoryContextIds.has(parentId)) throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id} references unknown parent kind ${parentId}`);
  for (const [id, spec] of Object.entries(semanticProjectedMemberKinds)) {
    if (!semanticDirectoryMemberKinds[spec.sourceMemberKindId]) throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id} references unknown source member kind ${spec.sourceMemberKindId}`);
    for (const ownerId of spec.ownerKindIds) if (!allDirectoryContextIds.has(ownerId)) throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id} references unknown owner kind ${ownerId}`);
  }

  const semanticPathProjectionProfileRenderers: Record<string, SemanticPathProjectionProfileRenderer> = {};
  for (const [id, value] of Object.entries(projectionRendererRows)) {
    const spec = requireRecord(value, `semanticPathProjectionProfileRenderers.${id}`);
    if (spec.direction !== "forward-only" || canonicalJson(spec.captureFields) !== canonicalJson(["standardVersion", "subsetId"]) || spec.template !== "🪆️{standardVersion}-{subsetId}" || canonicalJson(spec.tupleCollisionFields) !== canonicalJson(["artifactId", "standardVersion", "subsetId"])) throw new Error(`Taxonomy v7 semanticPathProjectionProfileRenderers.${id} is not the forward-only standard/subset contract`);
    const directoryKindId = requireString(spec.directoryKindId, `semanticPathProjectionProfileRenderers.${id}.directoryKindId`);
    if (!semanticDirectoryKinds[directoryKindId]) throw new Error(`Taxonomy v7 semanticPathProjectionProfileRenderers.${id} references unknown directory kind ${directoryKindId}`);
    semanticPathProjectionProfileRenderers[id] = { direction: "forward-only", captureFields: ["standardVersion", "subsetId"], directoryKindId, template: "🪆️{standardVersion}-{subsetId}", tupleCollisionFields: ["artifactId", "standardVersion", "subsetId"] };
  }
  if (Object.keys(semanticPathProjectionProfileRenderers).length === 0) throw new Error("Taxonomy v7 semanticPathProjectionProfileRenderers must not be empty");
  return { allDirectoryContextIds, semanticDirectoryMemberKinds, semanticProjectedMemberKinds, semanticPathProjectionProfileRenderers };
}

/** 🌲 Parses the semantic descendant contracts and the path projection catalog contracts. */
export function parseDescendantsAndCatalogs({ semanticDirectoryKinds, allDirectoryContextIds, fileKinds, fixedFilenameContracts, descendantContractRows, root, projectionCatalogRows }: {
  readonly semanticDirectoryKinds: Record<string, SemanticDirectoryKindSpec>;
  readonly allDirectoryContextIds: Set<string>;
  readonly fileKinds: Record<string, FileKindSpec>;
  readonly fixedFilenameContracts: Record<string, FixedFilenameContract>;
  readonly descendantContractRows: UnknownRecord;
  readonly root: UnknownRecord;
  readonly projectionCatalogRows: UnknownRecord;
}) {
  const parseDescendantNode = (value: unknown, name: string): SemanticDescendantNode => {
    const spec = requireRecord(value, name);
    if (spec.nodeType !== "directory" && spec.nodeType !== "file") throw new Error(`Taxonomy v7 ${name}.nodeType is invalid`);
    const parseSegments = (value: unknown, key: string): readonly Readonly<{ kindId: string; literal: string }>[] => {
      if (!Array.isArray(value)) throw new Error(`Taxonomy v7 ${key} must be an array`);
      return value.map((value, index) => {
        const segment = requireRecord(value, `${key}[${index}]`);
        const kindId = requireString(segment.kindId, `${key}[${index}].kindId`);
        const literal = requireString(segment.literal, `${key}[${index}].literal`).normalize("NFC");
        const kind = semanticDirectoryKinds[kindId];
        const leading = splitLeadingEmoji(literal);
        if (!kind || emojiFold(leading.emoji) !== emojiFold(kind.emoji) || !new RegExp(kind.slugPattern, "u").test(leading.rest)) throw new Error(`Taxonomy v7 ${key} has an invalid semantic path segment ${literal}`);
        return { kindId, literal };
      });
    };
    const pathSegments = parseSegments(spec.pathSegments, `${name}.pathSegments`);
    if (spec.nodeType === "directory") {
      const kindId = requireString(spec.kindId, `${name}.kindId`);
      if (!allDirectoryContextIds.has(kindId) || spec.sourceFilename !== undefined || spec.fixedFilenameContractId !== undefined || spec.packageGlue !== undefined) throw new Error(`Taxonomy v7 ${name} references an invalid directory kind ${kindId}`);
      return { pathSegments, nodeType: "directory", kindId };
    }
    const authorities = [spec.kindId !== undefined, spec.fixedFilenameContractId !== undefined].filter(Boolean).length;
    if (authorities !== 1) throw new Error(`Taxonomy v7 ${name} must declare exactly one file authority`);
    if (spec.kindId !== undefined) {
      const kindId = requireString(spec.kindId, `${name}.kindId`);
      if (!fileKinds[kindId]) throw new Error(`Taxonomy v7 ${name} references unknown file kind ${kindId}`);
      const sourceFilename = spec.sourceFilename === undefined ? undefined : requireString(spec.sourceFilename, `${name}.sourceFilename`).normalize("NFC");
      if (sourceFilename !== undefined && (kindId !== "rust-source" || sourceFilename !== "🦀️.rs")) throw new Error(`Taxonomy v7 ${name}.sourceFilename is not the frozen Draw Rust source leaf`);
      return { pathSegments, nodeType: "file", kindId, ...(sourceFilename ? { sourceFilename } : {}) };
    }
    if (spec.fixedFilenameContractId !== undefined) {
      const fixedFilenameContractId = requireString(spec.fixedFilenameContractId, `${name}.fixedFilenameContractId`);
      if (!fixedFilenameContracts[fixedFilenameContractId]) throw new Error(`Taxonomy v7 ${name} references unknown fixed filename contract ${fixedFilenameContractId}`);
      return { pathSegments, nodeType: "file", fixedFilenameContractId };
    }
    throw new Error(`Taxonomy v7 ${name} has no file authority`);
  };
  const semanticDescendantContracts: Record<string, SemanticDescendantContract> = {};
  for (const [id, value] of Object.entries(descendantContractRows)) {
    const spec = requireRecord(value, `semanticDescendantContracts.${id}`);
    const rootDirectoryKindId = requireString(spec.rootDirectoryKindId, `semanticDescendantContracts.${id}.rootDirectoryKindId`);
    if (!allDirectoryContextIds.has(rootDirectoryKindId)) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} references unknown root directory kind ${rootDirectoryKindId}`);
    if (spec.contractKind === "catalog") {
      const catalogContractId = requireString(spec.catalogContractId, `semanticDescendantContracts.${id}.catalogContractId`);
      const leafFileKindId = requireString(spec.leafFileKindId, `semanticDescendantContracts.${id}.leafFileKindId`);
      const reserve = requireRecord(spec.pathBudgetReserve, `semanticDescendantContracts.${id}.pathBudgetReserve`);
      if (!fileKinds[leafFileKindId] || spec.rendering !== "semantic-member-directory-and-physical-kind-leaf" || reserve.derivation !== "longest-rendered-catalog-descendant-suffix" || !Number.isSafeInteger(reserve.utf16Units) || (reserve.utf16Units as number) <= 0) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} is not a valid catalog descendant contract`);
      semanticDescendantContracts[id] = { contractKind: "catalog", rootDirectoryKindId, catalogContractId, leafFileKindId, rendering: "semantic-member-directory-and-physical-kind-leaf", pathBudgetReserve: { derivation: "longest-rendered-catalog-descendant-suffix", utf16Units: reserve.utf16Units as number } };
      continue;
    }
    if (!Array.isArray(spec.requiredNodes) || !Array.isArray(spec.exclusiveAlternatives)) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} node lists must be arrays`);
    const requiredNodes = spec.requiredNodes.map((node, index) => parseDescendantNode(node, `semanticDescendantContracts.${id}.requiredNodes[${index}]`));
    const exclusiveAlternatives = spec.exclusiveAlternatives.map((value, index) => {
      const alternative = requireRecord(value, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}]`);
      if (alternative.mode !== "exactly-one" || !Array.isArray(alternative.nodes) || alternative.nodes.length < 2) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} alternative must contain exactly-one candidates`);
      return { id: requireString(alternative.id, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}].id`), mode: "exactly-one" as const, nodes: alternative.nodes.map((node, nodeIndex) => parseDescendantNode(node, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}].nodes[${nodeIndex}]`)) };
    });
    if (!Number.isSafeInteger(spec.realizedNodeCount) || spec.realizedNodeCount !== requiredNodes.length + exclusiveAlternatives.length) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id}.realizedNodeCount is invalid`);
    const reserve = requireRecord(spec.pathBudgetReserve, `semanticDescendantContracts.${id}.pathBudgetReserve`);
    const suffix = (node: SemanticDescendantNode): string => {
      const segments = node.pathSegments.map((segment) => segment.literal);
      if (node.nodeType === "file") {
        if ("kindId" in node) {
          segments.push(canonicalPrimaryFilenameForKind(node.kindId, root as unknown as DiscoveryTaxonomy));
        } else if ("fixedFilenameContractId" in node){const fixed=fixedFilenameContracts[node.fixedFilenameContractId];if(!fixed)throw new Error("Required descendant fixed filename contract is missing");segments.push(posix.basename(fixed.pathPattern));}
        else throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} file authority is invalid`);
      }
      return segments.length === 0 ? "" : `/${segments.join("/")}`;
    };
    const reserveUnits = Math.max(...[...requiredNodes, ...exclusiveAlternatives.flatMap((alternative) => alternative.nodes)].map((node) => pathUtf16Units(suffix(node))));
    if (reserve.derivation !== "longest-canonical-descendant-suffix" || reserve.utf16Units !== reserveUnits) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id}.pathBudgetReserve is not derived from its longest suffix`);
    semanticDescendantContracts[id] = { rootDirectoryKindId, requiredNodes, exclusiveAlternatives, realizedNodeCount: spec.realizedNodeCount as number, pathBudgetReserve: { derivation: "longest-canonical-descendant-suffix", utf16Units: reserveUnits } };
  }
  if (Object.keys(semanticDescendantContracts).length === 0) throw new Error("Taxonomy v7 semanticDescendantContracts must not be empty");

  const semanticPathProjectionCatalogContracts: Record<string, SemanticPathProjectionCatalogContract> = {};
  const expectedCatalogContract: SemanticMutationPathProjectionCatalogContract = { registryField: "vectors", required: true, allowEmpty: true, runtimeKindsField: "kinds", runtimeKindsRelation: "independent", mutationIdField: "mutationId", sourceMutationDirectoryNameField: "sourceMutationDirectoryName", mutationDirectoryNameField: "mutationDirectoryName", scenariosField: "scenarios", scenarioIdField: "id", scenarioDirectoryNameField: "directoryName", sourceBundleUniquenessFields: ["mutationId", "sourceMutationDirectoryName", "scenarioId"], canonicalBundleUniquenessFields: ["mutationId", "mutationDirectoryName", "scenarioId"], coverage: "every-physical-bundle-exactly-once" };
  for (const [id, value] of Object.entries(projectionCatalogRows)) {
    const spec = requireRecord(value, `semanticPathProjectionCatalogContracts.${id}`);
    if (spec.contractKind === undefined) {
      if (canonicalJson(value) !== canonicalJson(expectedCatalogContract)) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not the independent required vector registry contract`);
      semanticPathProjectionCatalogContracts[id] = expectedCatalogContract;
      continue;
    }
    if (spec.contractKind === "distributed-json-manifest-catalog") {
      if (spec.modelIdentityField !== "id" || spec.memberIdentityField !== "id" || spec.memberVersionField !== "version" || spec.requiredModelManifest !== true || spec.coverage !== "every-source-file-and-destination-node-exactly-once" || spec.unknownCategoryPolicy !== "problem" || spec.unownedModelPolicy !== "problem" || !Array.isArray(spec.categoryRules) || spec.categoryRules.length === 0) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not a strict distributed manifest catalog`);
      if (!Array.isArray(spec.profileVectors) || spec.profileVectors.length === 0) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.profileVectors must be non-empty`);
      const profileVectors = spec.profileVectors.map((value, index) => {
        const vector = requireRecord(value, `semanticPathProjectionCatalogContracts.${id}.profileVectors[${index}]`);
        const profile = { artifactId: requireString(vector.artifactId, "profile artifactId"), standardVersion: requireString(vector.standardVersion, "profile standardVersion"), subsetId: requireString(vector.subsetId, "profile subsetId") };
        if (canonicalJson(Object.keys(vector).sort()) !== canonicalJson(["artifactId", "standardVersion", "subsetId"]) || profile.artifactId !== spec.ownerArtifactMemberName || Object.values(profile).some((field) => field !== field.normalize("NFC") || /[\\/]/u.test(field))) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.profileVectors[${index}] is not an exact NFC owner tuple`);
        return profile;
      });
      if (new Set(profileVectors.map((vector) => canonicalJson(vector))).size !== profileVectors.length) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} repeats a profile vector`);
      const categoryRules = spec.categoryRules.map((value, index) => {
        const rule = requireRecord(value, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}]`);
        const sourceDirectoryName = requireString(rule.sourceDirectoryName, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].sourceDirectoryName`).normalize("NFC");
        const directoryKindId = requireString(rule.directoryKindId, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].directoryKindId`);
        const manifestSchema = requireString(rule.manifestSchema, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].manifestSchema`);
        if (!semanticDirectoryKinds[directoryKindId]) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}] references an unknown directory kind`);
        if (rule.sourceShape === "direct-semantic-json") return { sourceDirectoryName, directoryKindId, sourceShape: "direct-semantic-json" as const, manifestSchema, memberDirectoryEmoji: requireString(rule.memberDirectoryEmoji, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].memberDirectoryEmoji`).normalize("NFC") };
        if (rule.sourceShape === "nested-fixed-json") return { sourceDirectoryName, directoryKindId, sourceShape: "nested-fixed-json" as const, manifestSchema, fixedSourceFilename: requireString(rule.fixedSourceFilename, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].fixedSourceFilename`).normalize("NFC") };
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].sourceShape is invalid`);
      });
      if (new Set(categoryRules.map((rule) => rule.sourceDirectoryName)).size !== categoryRules.length) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} repeats a catalog category`);
      semanticPathProjectionCatalogContracts[id] = { contractKind: "distributed-json-manifest-catalog", ownerArtifactMemberName: requireString(spec.ownerArtifactMemberName, `semanticPathProjectionCatalogContracts.${id}.ownerArtifactMemberName`).normalize("NFC"), profileVectors, modelManifestSchema: requireString(spec.modelManifestSchema, `semanticPathProjectionCatalogContracts.${id}.modelManifestSchema`), modelManifestSourceFilename: requireString(spec.modelManifestSourceFilename, `semanticPathProjectionCatalogContracts.${id}.modelManifestSourceFilename`).normalize("NFC"), modelIdentityField: "id", memberIdentityField: "id", memberVersionField: "version", requiredMemberVersion: requireString(spec.requiredMemberVersion, `semanticPathProjectionCatalogContracts.${id}.requiredMemberVersion`), requiredModelManifest: true, categoryRules, coverage: "every-source-file-and-destination-node-exactly-once", unknownCategoryPolicy: "problem", unownedModelPolicy: "problem" };
      continue;
    }
    if (spec.contractKind === "exact-owner-vectors") {
      if (spec.required !== true || spec.allowEmpty !== false || canonicalJson(spec.identityFields) !== canonicalJson(["artifactId", "standardVersion", "subsetId", "commandDirectoryName"]) || spec.coverage !== "every-physical-command-bundle-exactly-once" || !Array.isArray(spec.vectors) || spec.vectors.length === 0) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not a strict exact-owner vector registry`);
      const vectors = spec.vectors.map((value, index) => {
        const vector = requireRecord(value, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}]`);
        return { artifactId: requireString(vector.artifactId, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].artifactId`).normalize("NFC"), standardVersion: requireString(vector.standardVersion, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].standardVersion`), subsetId: requireString(vector.subsetId, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].subsetId`), commandDirectoryName: requireString(vector.commandDirectoryName, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].commandDirectoryName`).normalize("NFC") };
      });
      if (new Set(vectors.map((vector) => canonicalJson(vector))).size !== vectors.length) throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} repeats an owner vector`);
      semanticPathProjectionCatalogContracts[id] = { contractKind: "exact-owner-vectors", required: true, allowEmpty: false, identityFields: ["artifactId", "standardVersion", "subsetId", "commandDirectoryName"], coverage: "every-physical-command-bundle-exactly-once", vectors };
      continue;
    }
    throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.contractKind is invalid`);
  }
  if (Object.keys(semanticPathProjectionCatalogContracts).length === 0) throw new Error("Taxonomy v7 semanticPathProjectionCatalogContracts must not be empty");
  for (const [id, contract] of Object.entries(semanticDescendantContracts)) if ("contractKind" in contract && !semanticPathProjectionCatalogContracts[contract.catalogContractId]) throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} references an unknown catalog contract`);

  const captureFields = new Set<SemanticProjectionCaptureField>(["standardVersion", "subsetId", "mutationId", "scenarioId", "commandDirectoryName"]);
  return { captureFields, semanticDescendantContracts, semanticPathProjectionCatalogContracts };
}
