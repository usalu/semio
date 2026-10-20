/** 🪞️ The path projection contracts, owned file projections, reference consumers and the mutation catalog projection. */
import { requireExactKeys, requireRecord, requireString, requireStringArray, type UnknownRecord } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import type { FileKindSpec, FixedFilenameContract, SemanticDirectoryKindSpec, SemanticDirectoryMemberKindSpec, SemanticProjectedMemberKindSpec } from "../📐️kind-specs/🟦️.ts";
import { normalizeRelative } from "../../🛣️path/🟦️.ts";
import type { MutationCatalogProjectionContractIds, SemanticPathProjectionCatalogContract, SemanticPathProjectionContract, SemanticPathProjectionProfileRenderer, SemanticPathProjectionReferenceConsumerContract, SemanticProjectionCaptureField, SemanticProjectionDestinationSegment, SemanticProjectionSourceSegment } from "../🧭️path-projection/🟦️.ts";
import type { SemanticDescendantContract, SemanticDescendantKindNode } from "../🌳️descendants/🟦️.ts";
import type { SemanticOwnedFileProjectionContract } from "../🗃️owned-file-projection/🟦️.ts";
import { parseCurrentSourceApprovals, parseSemanticOwnedDocumentCorrections } from "../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts";
import type { MutationFixtureCoverageContract, SemanticPathProjectionReferenceConsumerForm } from "../../../🔍️discovery/📜️taxonomy-contracts/🟦️.ts";

/** 🧭 Parses the semantic path projection contracts and the owned file projection contracts. */
export function parsePathProjections({ allDirectoryContextIds, semanticDirectoryMemberKinds, semanticProjectedMemberKinds, captureFields, projectionRows, semanticPathProjectionProfileRenderers, semanticDescendantContracts, semanticPathProjectionCatalogContracts, ownedFileProjectionRows, generatorRows, fileKinds, semanticDirectoryKinds, fixedDirectoryRows, fixedFilenameContracts }: {
  readonly allDirectoryContextIds: Set<string>;
  readonly semanticDirectoryMemberKinds: Record<string, SemanticDirectoryMemberKindSpec>;
  readonly semanticProjectedMemberKinds: Record<string, SemanticProjectedMemberKindSpec>;
  readonly captureFields: Set<SemanticProjectionCaptureField>;
  readonly projectionRows: UnknownRecord;
  readonly semanticPathProjectionProfileRenderers: Record<string, SemanticPathProjectionProfileRenderer>;
  readonly semanticDescendantContracts: Record<string, SemanticDescendantContract>;
  readonly semanticPathProjectionCatalogContracts: Record<string, SemanticPathProjectionCatalogContract>;
  readonly ownedFileProjectionRows: UnknownRecord;
  readonly generatorRows: UnknownRecord;
  readonly fileKinds: Record<string, FileKindSpec>;
  readonly semanticDirectoryKinds: Record<string, SemanticDirectoryKindSpec>;
  readonly fixedDirectoryRows: UnknownRecord;
  readonly fixedFilenameContracts: Record<string, FixedFilenameContract>;
}) {
  const parseProjectionSegment = (value: unknown, name: string, destination: boolean): SemanticProjectionSourceSegment | SemanticProjectionDestinationSegment => {
    const spec = requireRecord(value, name);
    const kindId = typeof spec.kindId === "string" ? spec.kindId : undefined;
    const memberKindId = typeof spec.memberKindId === "string" ? spec.memberKindId : undefined;
    const projectedMemberKindId = typeof spec.projectedMemberKindId === "string" ? spec.projectedMemberKindId : undefined;
    if ((kindId ? 1 : 0) + (memberKindId ? 1 : 0) + (projectedMemberKindId ? 1 : 0) !== 1) throw new Error(`Taxonomy v7 ${name} must identify exactly one kind`);
    if (kindId && !allDirectoryContextIds.has(kindId)) throw new Error(`Taxonomy v7 ${name} references unknown directory kind ${kindId}`);
    if (memberKindId && !semanticDirectoryMemberKinds[memberKindId]) throw new Error(`Taxonomy v7 ${name} references unknown semantic member kind ${memberKindId}`);
    if (projectedMemberKindId && !semanticProjectedMemberKinds[projectedMemberKindId]) throw new Error(`Taxonomy v7 ${name} references unknown projected member kind ${projectedMemberKindId}`);
    if (destination) {
      if (memberKindId) throw new Error(`Taxonomy v7 ${name} cannot render a source member kind`);
      if (spec.literal !== undefined && kindId) return { kindId, literal: requireString(spec.literal, `${name}.literal`) };
      if (spec.render === "profile" && kindId) return { kindId, render: "profile" };
      if (typeof spec.copy === "string" && captureFields.has(spec.copy as SemanticProjectionCaptureField)) return projectedMemberKindId ? { projectedMemberKindId, copy: spec.copy as SemanticProjectionCaptureField } : { kindId: kindId!, copy: spec.copy as SemanticProjectionCaptureField };
    } else {
      if (spec.literal !== undefined && kindId) return { kindId, literal: requireString(spec.literal, `${name}.literal`) };
      if (spec.literal !== undefined && memberKindId) {
        const literal = requireString(spec.literal, `${name}.literal`).normalize("NFC");
        if (semanticDirectoryMemberKinds[memberKindId]?.memberNames.includes(literal)!==true) throw new Error(`Taxonomy v7 ${name}.literal is not registered by ${memberKindId}`);
        return { memberKindId, literal };
      }
      if (typeof spec.capture === "string" && captureFields.has(spec.capture as SemanticProjectionCaptureField)) return projectedMemberKindId ? { projectedMemberKindId, capture: spec.capture as SemanticProjectionCaptureField } : { kindId: kindId!, capture: spec.capture as SemanticProjectionCaptureField };
    }
    throw new Error(`Taxonomy v7 ${name} has an invalid ${destination ? "destination" : "source"} operation`);
  };
  const semanticPathProjectionContracts: Record<string, SemanticPathProjectionContract> = {};
  for (const [id, value] of Object.entries(projectionRows)) {
    const spec = requireRecord(value, `semanticPathProjectionContracts.${id}`);
    if (!Array.isArray(spec.sourceSegments) || !Array.isArray(spec.destinationSegments) || !["artifact-example-model-catalog-projection-v1", "artifact-editor-command-projection-v1"].includes(String(spec.rationaleRule))) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} is invalid`);
    const sourceOwnerKindId = requireString(spec.sourceOwnerKindId, `semanticPathProjectionContracts.${id}.sourceOwnerKindId`);
    const destinationOwnerKindId = requireString(spec.destinationOwnerKindId, `semanticPathProjectionContracts.${id}.destinationOwnerKindId`);
    if (!semanticDirectoryMemberKinds[sourceOwnerKindId] || !semanticDirectoryMemberKinds[destinationOwnerKindId]) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} owner kind is invalid`);
    const profileRendererId = requireString(spec.profileRendererId, `semanticPathProjectionContracts.${id}.profileRendererId`);
    const descendantContractId = requireString(spec.descendantContractId, `semanticPathProjectionContracts.${id}.descendantContractId`);
    const catalogContractId = requireString(spec.catalogContractId, `semanticPathProjectionContracts.${id}.catalogContractId`);
    if (!semanticPathProjectionProfileRenderers[profileRendererId] || !semanticDescendantContracts[descendantContractId] || !semanticPathProjectionCatalogContracts[catalogContractId]) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} references an unknown registry`);
    const rationaleRule = spec.rationaleRule as SemanticPathProjectionContract["rationaleRule"];
    const sourceArtifactMemberName = spec.sourceArtifactMemberName === undefined ? undefined : requireString(spec.sourceArtifactMemberName, `semanticPathProjectionContracts.${id}.sourceArtifactMemberName`).normalize("NFC");
    const expectedArtifact = rationaleRule === "artifact-example-model-catalog-projection-v1" ? "📐️cad" : rationaleRule === "artifact-editor-command-projection-v1" ? "🖍️drawing" : undefined;
    if (sourceArtifactMemberName !== expectedArtifact) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id}.sourceArtifactMemberName does not match its rationale`);
    const sourceSegments = spec.sourceSegments.map((segment, index) => parseProjectionSegment(segment, `semanticPathProjectionContracts.${id}.sourceSegments[${index}]`, false) as SemanticProjectionSourceSegment);
    const destinationSegments = spec.destinationSegments.map((segment, index) => parseProjectionSegment(segment, `semanticPathProjectionContracts.${id}.destinationSegments[${index}]`, true) as SemanticProjectionDestinationSegment);
    const captures = sourceSegments.flatMap((segment) => "capture" in segment ? [segment.capture] : []);
    const expectedCaptures: readonly SemanticProjectionCaptureField[] = rationaleRule === "artifact-editor-command-projection-v1" ? ["standardVersion", "subsetId", "commandDirectoryName"] : ["standardVersion", "subsetId"];
    if (canonicalJson(captures) !== canonicalJson(expectedCaptures)) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} has invalid captures for ${rationaleRule}`);
    const descendant = semanticDescendantContracts[descendantContractId];
    const catalog = semanticPathProjectionCatalogContracts[catalogContractId];
    if (rationaleRule === "artifact-example-model-catalog-projection-v1" ? !("contractKind" in descendant && descendant.contractKind === "catalog" && "contractKind" in catalog && catalog.contractKind === "distributed-json-manifest-catalog") : "contractKind" in descendant || !("contractKind" in catalog && catalog.contractKind === "exact-owner-vectors")) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} references incompatible descendant/catalog authorities`);
    const descendantNodes = "contractKind" in descendant ? [] : [...descendant.requiredNodes, ...descendant.exclusiveAlternatives.flatMap((alternative) => alternative.nodes)];
    const sourceNamedNodes = descendantNodes.filter((node): node is SemanticDescendantKindNode & { readonly sourceFilename: string } => "kindId" in node && node.sourceFilename !== undefined);
    if (rationaleRule === "artifact-editor-command-projection-v1" ? sourceNamedNodes.length !== descendantNodes.filter((node) => "kindId" in node && node.nodeType === "file" && node.kindId === "rust-source").length || sourceNamedNodes.filter((node) => node.pathSegments.length === 0).length !== 1 : sourceNamedNodes.length !== 0) throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} has invalid source-filename descendant authority`);
    semanticPathProjectionContracts[id] = { sourceOwnerKindId, ...(sourceArtifactMemberName ? { sourceArtifactMemberName } : {}), sourceSegments, profileRendererId, destinationOwnerKindId, destinationSegments, descendantContractId, catalogContractId, rationaleRule };
  }
  if (Object.keys(semanticPathProjectionContracts).length === 0) throw new Error("Taxonomy v7 semanticPathProjectionContracts must not be empty");
  for (const [id, spec] of Object.entries(semanticProjectedMemberKinds)) if (!semanticPathProjectionContracts[spec.projectionContractId] && spec.projectionContractId !== "canonical-mutation-case-pair-v1") throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id} references unknown projection contract ${spec.projectionContractId}`);
  const semanticOwnedFileProjectionContracts: Record<string, SemanticOwnedFileProjectionContract> = {};
  for (const [id, value] of Object.entries(ownedFileProjectionRows)) {
    const name = `semanticOwnedFileProjectionContracts.${id}`;
    const spec = requireRecord(value, name);
    if (spec.contractKind === "exact-owner-path-catalog") {
      requireExactKeys(spec, ["contractKind", "authorityCatalogPath", "authorityCatalogSha256", "sourceFileKindId", "sourceBasenames", "destinationDirectoryKinds", "allowedDispositions", "ownerEvidenceKinds", "referenceOwnerIds", "generatorOwnerIds", "authoredDocumentCorrections", "rationaleRule", ...(Object.hasOwn(spec, "currentSourceRevisions") ? ["currentSourceRevisions"] : [])], name);
      const authorityCatalogPath = normalizeRelative(requireString(spec.authorityCatalogPath, name + ".authorityCatalogPath"));
      const authorityCatalogSha256 = requireString(spec.authorityCatalogSha256, name + ".authorityCatalogSha256");
      const sourceBasenames = requireStringArray(spec.sourceBasenames, name + ".sourceBasenames");
      const allowedDispositions = requireStringArray(spec.allowedDispositions, name + ".allowedDispositions");
      const ownerEvidenceKinds = requireStringArray(spec.ownerEvidenceKinds, name + ".ownerEvidenceKinds");
      const referenceOwnerIds = requireStringArray(spec.referenceOwnerIds, name + ".referenceOwnerIds");
      const generatorOwnerIds = requireStringArray(spec.generatorOwnerIds, name + ".generatorOwnerIds");
      const destinations = requireRecord(spec.destinationDirectoryKinds, name + ".destinationDirectoryKinds");
      requireExactKeys(destinations, ["license", "readme"], name + ".destinationDirectoryKinds");
      const parseDestination = (kind: "license" | "readme"): { readonly directoryKindId: "owner-license" | "owner-readme"; readonly directoryName: "⚖️license" | "📃️readme"; readonly filename: "📝️.md" } => {
        const destination = requireRecord(destinations[kind], name + ".destinationDirectoryKinds." + kind);
        requireExactKeys(destination, ["directoryKindId", "directoryName", "filename"], name + ".destinationDirectoryKinds." + kind);
        return { directoryKindId: requireString(destination.directoryKindId, name + ".destinationDirectoryKinds." + kind + ".directoryKindId") as "owner-license" | "owner-readme", directoryName: requireString(destination.directoryName, name + ".destinationDirectoryKinds." + kind + ".directoryName") as "⚖️license" | "📃️readme", filename: requireString(destination.filename, name + ".destinationDirectoryKinds." + kind + ".filename") as "📝️.md" };
      };
      const destinationDirectoryKinds = { license: parseDestination("license"), readme: parseDestination("readme") };
      if (id !== "readme-license-owner-leaves-v1"
        || !/^[a-f0-9]{64}$/u.test(authorityCatalogSha256)
        || spec.sourceFileKindId !== "markdown"
        || canonicalJson(sourceBasenames) !== canonicalJson(["LICENSE.md", "README.md"])
        || canonicalJson(destinationDirectoryKinds) !== canonicalJson({ license: { directoryKindId: "owner-license", directoryName: "⚖️license", filename: "📝️.md" }, readme: { directoryKindId: "owner-readme", directoryName: "📃️readme", filename: "📝️.md" } })
        || canonicalJson(allowedDispositions) !== canonicalJson(["attribution-relocate", "configurable-owner-license-relocate", "fixed", "generated-evidence-relocate", "owner-documentation-relocate"])
        || canonicalJson(ownerEvidenceKinds) !== canonicalJson(["configurable-owner-license", "ordinary-owner-doc", "package-publication", "third-party-attribution", "ticket-evidence", "ticket-scratch"])
        || new Set(referenceOwnerIds).size !== referenceOwnerIds.length || referenceOwnerIds.some(id => !/^[a-z][a-z0-9-]*$/u.test(id))
        || new Set(generatorOwnerIds).size !== generatorOwnerIds.length || generatorOwnerIds.some(id => !/^[a-z][a-z0-9-]*$/u.test(id) || !generatorRows[id])
        || spec.rationaleRule !== "readme-license-owner-projection-v1"
        || !fileKinds.markdown
        || semanticDirectoryKinds["owner-license"]?.projectionOnly !== true
        || semanticDirectoryKinds["owner-readme"]?.projectionOnly !== true
) throw new Error("Taxonomy v7 " + name + " does not use the exact README/LICENSE owner catalog grammar");
      semanticOwnedFileProjectionContracts[id] = {
        contractKind: "exact-owner-path-catalog",
        authorityCatalogPath,
        authorityCatalogSha256,
        sourceFileKindId: "markdown",
        sourceBasenames: ["LICENSE.md", "README.md"],
        destinationDirectoryKinds: { license: { directoryKindId: "owner-license", directoryName: "⚖️license", filename: "📝️.md" }, readme: { directoryKindId: "owner-readme", directoryName: "📃️readme", filename: "📝️.md" } },
        allowedDispositions: ["attribution-relocate", "configurable-owner-license-relocate", "fixed", "generated-evidence-relocate", "owner-documentation-relocate"],
        ownerEvidenceKinds: ["configurable-owner-license", "ordinary-owner-doc", "package-publication", "third-party-attribution", "ticket-evidence", "ticket-scratch"],
        referenceOwnerIds,
        generatorOwnerIds,
        authoredDocumentCorrections: parseSemanticOwnedDocumentCorrections(spec.authoredDocumentCorrections),
        ...(Object.hasOwn(spec, "currentSourceRevisions") ? { currentSourceRevisions: parseCurrentSourceApprovals(spec.currentSourceRevisions) } : {}),
        rationaleRule: "readme-license-owner-projection-v1",
      };
      continue;
    }
    if (spec.contractKind === "semantic-facet-primary-file") {
      requireExactKeys(spec, ["contractKind", "sourceRoot", "sourceFilename", "fileKindAuthority", "sourceDisposition", "directoryCaptures", "ownerPathPatterns", "authoringCommand", "referenceConsumer", "rationaleRule"], name);
      const directoryCaptures: Record<string, { kindIds: readonly string[]; names?: readonly string[] }> = {};
      for (const [capture, row] of Object.entries(requireRecord(spec.directoryCaptures, `${name}.directoryCaptures`))) {
        const captureSpec = requireRecord(row, `${name}.directoryCaptures.${capture}`);
        directoryCaptures[capture] = { kindIds: requireStringArray(captureSpec.kindIds, `${name}.directoryCaptures.${capture}.kindIds`), ...(captureSpec.names ? { names: requireStringArray(captureSpec.names, `${name}.directoryCaptures.${capture}.names`) } : {}) };
      }
      const ownerPathPatterns = Object.fromEntries(Object.entries(requireRecord(spec.ownerPathPatterns, `${name}.ownerPathPatterns`)).map(([form, pattern]) => [form, requireString(pattern, `${name}.ownerPathPatterns.${form}`)]));
      const authoring = requireRecord(spec.authoringCommand, `${name}.authoringCommand`), consumer = requireRecord(spec.referenceConsumer, `${name}.referenceConsumer`);
      semanticOwnedFileProjectionContracts[id] = { contractKind: "semantic-facet-primary-file", sourceRoot: requireString(spec.sourceRoot, `${name}.sourceRoot`), sourceFilename: requireString(spec.sourceFilename, `${name}.sourceFilename`), fileKindAuthority: "windowEmptyFacetFileKindId", sourceDisposition: "authored", directoryCaptures, ownerPathPatterns, authoringCommand: { scriptPath: requireString(authoring.scriptPath, `${name}.authoringCommand.scriptPath`), command: ["new", "surface"], writeDisposition: "create-if-absent" }, referenceConsumer: { path: requireString(consumer.path, `${name}.referenceConsumer.path`), ownerRoot: requireString(consumer.ownerRoot, `${name}.referenceConsumer.ownerRoot`), adapter: "rust", region: "✏️👁️Surfaces", lineTemplate: requireString(consumer.lineTemplate, `${name}.referenceConsumer.lineTemplate`) }, rationaleRule: "artifact-empty-facet-primary-markdown-v1" };
      continue;
    }
    if (spec.contractKind === "owner-primary-file") {
      requireExactKeys(spec, ["contractKind", "ownerFixedDirectoryContractId", "sourceFileKindId", "sourceFilename", "destinationFilename", "rationaleRule"], name);
      const contract = { contractKind: "owner-primary-file" as const, ownerFixedDirectoryContractId: requireString(spec.ownerFixedDirectoryContractId, `${name}.ownerFixedDirectoryContractId`), sourceFileKindId: requireString(spec.sourceFileKindId, `${name}.sourceFileKindId`), sourceFilename: requireString(spec.sourceFilename, `${name}.sourceFilename`), destinationFilename: requireString(spec.destinationFilename, `${name}.destinationFilename`), rationaleRule: "ticket-document-primary-markdown-v1" as const };
      if (id !== contract.rationaleRule || spec.rationaleRule !== contract.rationaleRule || contract.ownerFixedDirectoryContractId !== "ticket-slug" || !fixedDirectoryRows[contract.ownerFixedDirectoryContractId] || contract.sourceFileKindId !== "markdown" || !fileKinds.markdown || fileKinds.markdown.extensionChains.length !== 1 || contract.sourceFilename !== "ticket.md" || contract.destinationFilename !== `${fileKinds.markdown.emoji}${fileKinds.markdown.extensionChains[0]}`) throw new Error(`Taxonomy v7 ${name} does not use the exact ticket document primary-leaf grammar`);
      semanticOwnedFileProjectionContracts[id] = contract;
      continue;
    }
    const ownerFixedDirectoryContractId = requireString(spec.ownerFixedDirectoryContractId, `${name}.ownerFixedDirectoryContractId`);
    const sourceFileKindId = requireString(spec.sourceFileKindId, `${name}.sourceFileKindId`);
    const destinationDirectoryKindId = requireString(spec.destinationDirectoryKindId, `${name}.destinationDirectoryKindId`);
    if (!fixedDirectoryRows[ownerFixedDirectoryContractId] || !fileKinds[sourceFileKindId] || semanticDirectoryKinds[destinationDirectoryKindId]?.projectionOnly !== true) throw new Error(`Taxonomy v7 ${name} references unknown or non-projection authority`);
    const common = { ownerFixedDirectoryContractId, sourceFileKindId, sourceFilename: requireString(spec.sourceFilename, `${name}.sourceFilename`), destinationDirectoryKindId, destinationDirectoryName: requireString(spec.destinationDirectoryName, `${name}.destinationDirectoryName`), destinationFilename: requireString(spec.destinationFilename, `${name}.destinationFilename`) };
    if (spec.contractKind === "owner-sibling-manifest-file") {
      requireExactKeys(spec, ["contractKind", "ownerFixedDirectoryContractId", "requiredSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "allowedStatuses", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "emptyContentRule", "statusDispositions", "rationaleRule"], name);
      const allowedStatuses = requireStringArray(spec.allowedStatuses, `${name}.allowedStatuses`);
      const statusDispositions = requireRecord(spec.statusDispositions, `${name}.statusDispositions`);
      requireExactKeys(statusDispositions, ["open", "closed-empty", "closed-nonempty", "invalid"], `${name}.statusDispositions`);
      const requiredSiblingFixedFilenameContractId = requireString(spec.requiredSiblingFixedFilenameContractId, `${name}.requiredSiblingFixedFilenameContractId`);
      if (spec.manifestAdapter !== "json" || spec.manifestStatusLocation !== "status" || canonicalJson(allowedStatuses) !== canonicalJson(["closed", "open"]) || spec.emptyContentRule !== "zero-byte" || spec.rationaleRule !== "ticket-important-markdown-projection-v1" || canonicalJson(statusDispositions) !== canonicalJson({ open: "project", "closed-empty": "remove", "closed-nonempty": "problem", invalid: "problem" }) || !fixedFilenameContracts[requiredSiblingFixedFilenameContractId]) throw new Error(`Taxonomy v7 ${name} does not use the exact active owner-file projection grammar`);
      semanticOwnedFileProjectionContracts[id] = { contractKind: "owner-sibling-manifest-file", ...common, requiredSiblingFixedFilenameContractId, manifestAdapter: "json", manifestStatusLocation: "status", allowedStatuses: ["closed", "open"], emptyContentRule: "zero-byte", statusDispositions: { open: "project", "closed-empty": "remove", "closed-nonempty": "problem", invalid: "problem" }, rationaleRule: "ticket-important-markdown-projection-v1" };
    } else if (spec.contractKind === "owner-optional-sibling-manifest-file") {
      requireExactKeys(spec, ["contractKind", "ownerFixedDirectoryContractId", "optionalSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "admittedDispositions", "rationaleRule"], name);
      const optionalSiblingFixedFilenameContractId = requireString(spec.optionalSiblingFixedFilenameContractId, `${name}.optionalSiblingFixedFilenameContractId`);
      const admittedDispositions = requireStringArray(spec.admittedDispositions, `${name}.admittedDispositions`);
      if (spec.manifestAdapter !== "json" || spec.manifestStatusLocation !== "status" || spec.rationaleRule !== "ticket-important-history-markdown-v1" || canonicalJson(admittedDispositions) !== canonicalJson(["closed-nonzero", "invalid-manifest", "missing-manifest"]) || !fixedFilenameContracts[optionalSiblingFixedFilenameContractId]) throw new Error(`Taxonomy v7 ${name} does not use the exact historical owner-file projection grammar`);
      semanticOwnedFileProjectionContracts[id] = { contractKind: "owner-optional-sibling-manifest-file", ...common, optionalSiblingFixedFilenameContractId, manifestAdapter: "json", manifestStatusLocation: "status", admittedDispositions: ["closed-nonzero", "invalid-manifest", "missing-manifest"], rationaleRule: "ticket-important-history-markdown-v1" };
    } else throw new Error(`Taxonomy v7 ${name}.contractKind is invalid`);
  }
  if (canonicalJson(Object.keys(semanticOwnedFileProjectionContracts)) !== canonicalJson(["artifact-empty-facet-primary-markdown-v1", "readme-license-owner-leaves-v1", "ticket-document-primary-markdown-v1", "ticket-important-history-markdown-v1", "ticket-important-markdown-v1"])) throw new Error("Taxonomy v7 semanticOwnedFileProjectionContracts must contain the exact artifact-facet, README/LICENSE, ticket-document, active, and history contracts");
  return { semanticPathProjectionContracts, parseProjectionSegment, semanticOwnedFileProjectionContracts };
}

/** 📚 Parses the projection reference consumer contracts and the mutation catalog projection. */
function parseMutationFixtureCoverage(value: unknown): MutationFixtureCoverageContract {
  const row = requireRecord(value, "mutationCatalogProjection.fixtureCoverage");
  requireExactKeys(row, ["implementationCase", "exhaustiveCase", "fixtureRunner", "leafCaseTable"], "mutationCatalogProjection.fixtureCoverage");
  const exhaustive = requireRecord(row.exhaustiveCase, "mutationCatalogProjection.fixtureCoverage.exhaustiveCase");
  requireExactKeys(exhaustive, ["featureFileKindId", "stepUriPrefix", "bundleFacetDirectoryNames"], "mutationCatalogProjection.fixtureCoverage.exhaustiveCase");
  const runner = requireRecord(row.fixtureRunner, "mutationCatalogProjection.fixtureCoverage.fixtureRunner");
  requireExactKeys(runner, ["relativePath"], "mutationCatalogProjection.fixtureCoverage.fixtureRunner");
  const table = requireRecord(row.leafCaseTable, "mutationCatalogProjection.fixtureCoverage.leafCaseTable");
  requireExactKeys(table, ["fileKindId", "casesField"], "mutationCatalogProjection.fixtureCoverage.leafCaseTable");
  const facets = requireStringArray(exhaustive.bundleFacetDirectoryNames, "mutationCatalogProjection.fixtureCoverage.exhaustiveCase.bundleFacetDirectoryNames");
  if (facets.length === 0) throw new Error("Taxonomy v7 mutationCatalogProjection.fixtureCoverage.exhaustiveCase.bundleFacetDirectoryNames must not be empty");
  return {
    implementationCase: requireString(row.implementationCase, "mutationCatalogProjection.fixtureCoverage.implementationCase"),
    exhaustiveCase: { featureFileKindId: requireString(exhaustive.featureFileKindId, "mutationCatalogProjection.fixtureCoverage.exhaustiveCase.featureFileKindId"), stepUriPrefix: requireString(exhaustive.stepUriPrefix, "mutationCatalogProjection.fixtureCoverage.exhaustiveCase.stepUriPrefix"), bundleFacetDirectoryNames: [...facets] },
    fixtureRunner: { relativePath: requireString(runner.relativePath, "mutationCatalogProjection.fixtureCoverage.fixtureRunner.relativePath") },
    leafCaseTable: { fileKindId: requireString(table.fileKindId, "mutationCatalogProjection.fixtureCoverage.leafCaseTable.fileKindId"), casesField: requireString(table.casesField, "mutationCatalogProjection.fixtureCoverage.leafCaseTable.casesField") },
  };
}

export function parseConsumersAndMutationCatalog({ projectionConsumerRows, semanticPathProjectionContracts, mutationCatalogProjectionRow, parseProjectionSegment, semanticDirectoryMemberKinds, semanticProjectedMemberKinds, semanticDescendantContracts, semanticPathProjectionCatalogContracts }: {
  readonly projectionConsumerRows: UnknownRecord;
  readonly semanticPathProjectionContracts: Record<string, SemanticPathProjectionContract>;
  readonly mutationCatalogProjectionRow: UnknownRecord;
  readonly parseProjectionSegment: (value: unknown, name: string, destination: boolean) => SemanticProjectionSourceSegment | SemanticProjectionDestinationSegment;
  readonly semanticDirectoryMemberKinds: Record<string, SemanticDirectoryMemberKindSpec>;
  readonly semanticProjectedMemberKinds: Record<string, SemanticProjectedMemberKindSpec>;
  readonly semanticDescendantContracts: Record<string, SemanticDescendantContract>;
  readonly semanticPathProjectionCatalogContracts: Record<string, SemanticPathProjectionCatalogContract>;
}) {
  const semanticPathProjectionReferenceConsumerContracts: Record<string, SemanticPathProjectionReferenceConsumerContract> = {};
  const referenceConsumerForms = new Set<SemanticPathProjectionReferenceConsumerForm>(["path-reference", "artifact-catalog-glob", "artifact-catalog-prose:root-marker", "artifact-catalog-prose:relative-root", "artifact-catalog-prose:category-glob", "artifact-catalog-prose:catalog-grammar"]);
  const referenceConsumerAdapters = new Set<SemanticPathProjectionReferenceConsumerContract["adapters"][number]>(["rust", "typescript", "json", "toml"]);
  const referenceConsumerIdentities = new Set<string>();
  for (const [id, value] of Object.entries(projectionConsumerRows)) {
    const spec = requireRecord(value, `semanticPathProjectionReferenceConsumerContracts.${id}`);
    requireExactKeys(spec, ["projectionContractId", "consumerIdentity", "ownership", "sourcePathPattern", "sourcePathIdentities", "adapters", "supportedForms", "staleMarkers"], `semanticPathProjectionReferenceConsumerContracts.${id}`);
    const projectionContractId = requireString(spec.projectionContractId, `semanticPathProjectionReferenceConsumerContracts.${id}.projectionContractId`);
    if (!semanticPathProjectionContracts[projectionContractId]) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id} references an unknown projection contract`);
    if (spec.ownership !== "external") throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.ownership must be external`);
    const consumerIdentity = requireString(spec.consumerIdentity, `semanticPathProjectionReferenceConsumerContracts.${id}.consumerIdentity`);
    if (referenceConsumerIdentities.has(consumerIdentity)) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts repeats consumer identity ${consumerIdentity}`);
    referenceConsumerIdentities.add(consumerIdentity);
    const sourcePathPattern = requireString(spec.sourcePathPattern, `semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathPattern`);
    if (!sourcePathPattern.startsWith("^") || !sourcePathPattern.endsWith("$")) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathPattern must be a full-match expression`);
    const sourcePathRegex = new RegExp(sourcePathPattern, "u");
    const sourcePathIdentities = requireStringArray(spec.sourcePathIdentities, `semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathIdentities`);
    const adapters = requireStringArray(spec.adapters, `semanticPathProjectionReferenceConsumerContracts.${id}.adapters`) as readonly SemanticPathProjectionReferenceConsumerContract["adapters"][number][];
    const supportedForms = requireStringArray(spec.supportedForms, `semanticPathProjectionReferenceConsumerContracts.${id}.supportedForms`) as readonly SemanticPathProjectionReferenceConsumerForm[];
    const staleMarkers = requireStringArray(spec.staleMarkers, `semanticPathProjectionReferenceConsumerContracts.${id}.staleMarkers`);
    if (sourcePathIdentities.length === 0 || adapters.length === 0 || supportedForms.length === 0 || staleMarkers.length === 0) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id} must be nonempty`);
    if (new Set(sourcePathIdentities).size !== sourcePathIdentities.length || sourcePathIdentities.some((path) => path !== normalizeRelative(path) || !sourcePathRegex.test(path))) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathIdentities are invalid`);
    if (new Set(adapters).size !== adapters.length || adapters.some((adapter) => !referenceConsumerAdapters.has(adapter))) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.adapters are invalid`);
    if (new Set(supportedForms).size !== supportedForms.length || supportedForms.some((form) => !referenceConsumerForms.has(form))) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.supportedForms are invalid`);
    if (new Set(staleMarkers).size !== staleMarkers.length || staleMarkers.some((marker) => !marker || marker !== marker.normalize("NFC"))) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.staleMarkers are invalid`);
    semanticPathProjectionReferenceConsumerContracts[id] = { projectionContractId, consumerIdentity, ownership: "external", sourcePathPattern, sourcePathIdentities: [...sourcePathIdentities], adapters: [...adapters], supportedForms: [...supportedForms], staleMarkers: [...staleMarkers] };
  }
  if (Object.keys(semanticPathProjectionReferenceConsumerContracts).length === 0) throw new Error("Taxonomy v7 semanticPathProjectionReferenceConsumerContracts must not be empty");
  if (mutationCatalogProjectionRow.contractKind !== "canonical-mutation-case-pair" || mutationCatalogProjectionRow.contractId !== "canonical-mutation-case-pair-v1" || mutationCatalogProjectionRow.coverage !== "every-catalog-vector-has-one-fixture-bundle-with-declared-coverage" || !Array.isArray(mutationCatalogProjectionRow.implementationSegments) || !Array.isArray(mutationCatalogProjectionRow.fixtureSegments)) throw new Error("Taxonomy v7 mutationCatalogProjection must be the canonical mutation case pair contract");
  const mutationCatalogProjection: MutationCatalogProjectionContractIds = {
    contractKind: "canonical-mutation-case-pair",
    contractId: "canonical-mutation-case-pair-v1",
    sourceOwnerKindId: requireString(mutationCatalogProjectionRow.sourceOwnerKindId, "mutationCatalogProjection.sourceOwnerKindId"),
    projectedMemberKindId: requireString(mutationCatalogProjectionRow.projectedMemberKindId, "mutationCatalogProjection.projectedMemberKindId"),
    implementationSegments: mutationCatalogProjectionRow.implementationSegments.map((value, index) => parseProjectionSegment(value, `mutationCatalogProjection.implementationSegments[${index}]`, false) as SemanticProjectionSourceSegment),
    fixtureSegments: mutationCatalogProjectionRow.fixtureSegments.map((value, index) => parseProjectionSegment(value, `mutationCatalogProjection.fixtureSegments[${index}]`, false) as SemanticProjectionSourceSegment),
    implementationDescendantContractId: requireString(mutationCatalogProjectionRow.implementationDescendantContractId, "mutationCatalogProjection.implementationDescendantContractId"),
    fixtureDescendantContractId: requireString(mutationCatalogProjectionRow.fixtureDescendantContractId, "mutationCatalogProjection.fixtureDescendantContractId"),
    catalogContractId: requireString(mutationCatalogProjectionRow.catalogContractId, "mutationCatalogProjection.catalogContractId"),
    coverage: "every-catalog-vector-has-one-fixture-bundle-with-declared-coverage",
    fixtureCoverage: parseMutationFixtureCoverage(mutationCatalogProjectionRow.fixtureCoverage),
  };
  if (!semanticDirectoryMemberKinds[mutationCatalogProjection.sourceOwnerKindId] || !semanticProjectedMemberKinds[mutationCatalogProjection.projectedMemberKindId] || !semanticDescendantContracts[mutationCatalogProjection.implementationDescendantContractId] || !semanticDescendantContracts[mutationCatalogProjection.fixtureDescendantContractId] || !semanticPathProjectionCatalogContracts[mutationCatalogProjection.catalogContractId]) throw new Error("Taxonomy v7 mutationCatalogProjection references unknown canonical pair registries");
  return { semanticPathProjectionReferenceConsumerContracts, mutationCatalogProjection };
}
