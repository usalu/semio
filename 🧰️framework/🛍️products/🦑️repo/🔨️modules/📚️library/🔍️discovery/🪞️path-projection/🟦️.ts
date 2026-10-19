import { basename, dirname } from "node:path";
import { pathUtf16Units } from "../📏️path-budget/🟦️.ts";
import type { SemanticPathProjectionReferenceConsumerContract, SemanticPathProjectionReferenceConsumerForm } from "../📜️taxonomy-contracts/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { canonicalSemanticDirectoryName } from "../📂️semantic-directory-kinds/🟦️.ts";
import { fixedContractFilename } from "../🎯️taxonomy-path-matching/🟦️.ts";
import { type SemanticPathProjectionAuthority, type SemanticPathProjectionAuthorityOptions, type SemanticPathProjectionMapping, type SemanticProjectionAuthorityNode, type SemanticProjectionProfileIdentity, artifactProjectionPathProblems, projectionByteCompare, projectionDirectories, projectionFileMappingDigest, projectionJsonManifest } from "../🧱️path-projection-contracts/🟦️.ts";
import { canonicalFilenameForKind, loadTaxonomy, renderArtifactPathProjectionRoot, renderSemanticProjectionProfile, semanticDescendantNodeRelativePath, semanticDirectoryKindId } from "../🔐️taxonomy-authority/🟦️.ts";
import type { SemanticProjectionVectorRegistration } from "../🗃️owned-file-projection/🟦️.ts";

/** 🧭️ Full source-owner identity required before resolving a projected mutation member. */
export interface SemanticProjectedMemberContext {
  readonly projectionContractId: string;
  readonly artifactId: string;
  readonly artifactDirectoryName: string;
  readonly standardVersion: string;
  readonly standardDirectoryName: string;
  readonly subsetId: string;
  readonly subsetDirectoryName: string;
  readonly mutationId: string;
  readonly mutationDirectoryName: string;
  readonly scenarioId: string;
  readonly scenarioDirectoryName: string;
  readonly vectors: readonly SemanticProjectionVectorRegistration[];
}

function exactSemanticKindName(name: string, kindId: string, parentKindId: string, taxonomy: Taxonomy): boolean {
  return semanticDirectoryKindId(name, taxonomy, { parentKindId }) === kindId;
}

/** 🪞️ Resolves one canonical mutation member only from a complete artifact/subset/vector identity. */
export function semanticProjectedMemberKindId(name: string, context: SemanticProjectedMemberContext, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const pair = context.projectionContractId === taxonomy.mutationCatalogProjection.contractId ? taxonomy.mutationCatalogProjection : null;
  if (!pair || !context.artifactId || !context.mutationId || !context.scenarioId) return null;
  const sourceOwner = taxonomy.semanticDirectoryMemberKinds[pair.sourceOwnerKindId];
  if (!sourceOwner?.memberNames.includes(canonicalSemanticDirectoryName(context.artifactDirectoryName, taxonomy))) return null;
  if (context.standardDirectoryName !== `🔖️${context.standardVersion}` || !exactSemanticKindName(context.standardDirectoryName, "standard", "standards", taxonomy)) return null;
  if (context.subsetDirectoryName !== `✳️${context.subsetId}` || !exactSemanticKindName(context.subsetDirectoryName, "subset", "subsets", taxonomy)) return null;
  if (context.scenarioDirectoryName !== `🧪️${context.scenarioId}` || semanticDirectoryKindId(context.scenarioDirectoryName, taxonomy, { parentKindId: "mutation-test-subject" }) !== "test-case") return null;
  const memberName = canonicalSemanticDirectoryName(name, taxonomy);
  if (memberName !== canonicalSemanticDirectoryName(context.mutationDirectoryName, taxonomy)) return null;
  const vectorMatches = context.vectors.filter((vector) => vector.mutationId === context.mutationId && canonicalSemanticDirectoryName(vector.mutationDirectoryName, taxonomy) === memberName);
  if (vectorMatches.length !== 1) return null;
  const scenarioMatches = vectorMatches[0]!.scenarios.filter((scenario) => scenario.id === context.scenarioId && scenario.directoryName === context.scenarioDirectoryName);
  if (scenarioMatches.length !== 1) return null;
  const matches = Object.entries(taxonomy.semanticProjectedMemberKinds).filter(([id, spec]) => {
    if (spec.projectionContractId !== context.projectionContractId || !pair.fixtureSegments.some((segment) => "projectedMemberKindId" in segment && segment.projectedMemberKindId === id)) return false;
    const source = taxonomy.semanticDirectoryMemberKinds[spec.sourceMemberKindId];
    return spec.identityField === "mutationDirectoryName" && source?.memberNames.includes(memberName);
  });
  return matches.length === 1 ? matches[0]![0] : null;
}

/** ⚖️ Renders profile tuples and rejects distinct tuples colliding within one artifact owner. */
export function renderSemanticProjectionProfiles(contractId: string, identities: readonly SemanticProjectionProfileIdentity[], taxonomy: Taxonomy = loadTaxonomy()): readonly Readonly<SemanticProjectionProfileIdentity & { directoryName: string }>[] {
  const seen = new Map<string, string>();
  return identities.map((identity) => {
    const directoryName = renderSemanticProjectionProfile(contractId, identity, taxonomy);
    const key = `${identity.artifactId}\0${directoryName}`;
    const tuple = `${identity.standardVersion}\0${identity.subsetId}`;
    const prior = seen.get(key);
    if (prior && prior !== tuple) throw new Error(`Projection profile collision for artifact ${JSON.stringify(identity.artifactId)} at ${JSON.stringify(directoryName)}.`);
    seen.set(key, tuple);
    return { ...identity, directoryName };
  });
}

/** 🔭️ Resolves exact schema-owned external projection consumers without owner inference. */
export function semanticPathProjectionReferenceConsumers(
  projectionContractId: string,
  sourcePath: string,
  adapter: SemanticPathProjectionReferenceConsumerContract["adapters"][number],
  form: SemanticPathProjectionReferenceConsumerForm,
  taxonomy: Taxonomy = loadTaxonomy(),
): readonly Readonly<{ id: string; contract: SemanticPathProjectionReferenceConsumerContract }>[] {
  return Object.entries(taxonomy.semanticPathProjectionReferenceConsumerContracts)
    .filter(([, contract]) => contract.projectionContractId === projectionContractId && contract.sourcePathIdentities.includes(sourcePath) && contract.adapters.includes(adapter) && contract.supportedForms.includes(form) && new RegExp(contract.sourcePathPattern, "u").test(sourcePath))
    .map(([id, contract]) => ({ id, contract }))
    .sort((left, right) => projectionByteCompare(left.id, right.id));
}

/** 🧾️ Validates and renders either the CAD manifest catalog or Draw exact command bundle without writes. */
export function semanticPathProjectionAuthority(options: SemanticPathProjectionAuthorityOptions, taxonomy: Taxonomy = loadTaxonomy()): SemanticPathProjectionAuthority {
  const root = renderArtifactPathProjectionRoot(options, taxonomy);
  const problems = [...root.problems];
  const contract = taxonomy.semanticPathProjectionContracts[options.contractId];
  const destinationLayout = options.layout === "destination";
  const physicalRoot = destinationLayout ? root.destinationRoot : options.sourceRoot;
  const pathOwners = new Map<string, SemanticProjectionAuthorityNode>();
  for (const node of options.nodes) {
    if (node.path !== node.path.normalize("NFC") || /\uFE0E/u.test(node.path)) problems.push(`Projection node ${JSON.stringify(node.path)} must be NFC and must not contain VS15.`);
    if (!(node.path === physicalRoot || node.path.startsWith(`${physicalRoot}/`))) problems.push(`Projection node ${JSON.stringify(node.path)} is outside its exact ${destinationLayout ? "destination" : "source"} root.`);
    if (pathOwners.has(node.path)) problems.push(`Projection node ${JSON.stringify(node.path)} is duplicated.`);
    pathOwners.set(node.path, node);
    if (node.nodeKind === "symlink") problems.push(`Projection source contains forbidden symlink ${JSON.stringify(node.path)}.`);
  }
  if (pathOwners.get(physicalRoot)?.nodeKind !== "directory") problems.push("Projection physical root must be present as a directory node.");
  const actualFiles = options.nodes.filter((node) => node.nodeKind === "file").map((node) => node.path).sort(projectionByteCompare);
  const expectedSourceDirectories = projectionDirectories(physicalRoot, actualFiles);
  const actualSourceDirectories = options.nodes.filter((node) => node.nodeKind === "directory").map((node) => node.path).sort(projectionByteCompare);
  if (expectedSourceDirectories.join("\0") !== actualSourceDirectories.join("\0")) problems.push("Projection source directories must be exactly those owned by source files.");
  const candidateMappings: SemanticPathProjectionMapping[] = [];
  if (contract && root.destinationRoot) {
    const catalog = taxonomy.semanticPathProjectionCatalogContracts[contract.catalogContractId];
    if (catalog && "contractKind" in catalog && catalog.contractKind === "distributed-json-manifest-catalog") {
      if (catalog.profileVectors.filter((vector) => vector.artifactId === basename(options.artifactRoot) && vector.standardVersion === root.captures.standardVersion && vector.subsetId === root.captures.subsetId).length !== 1) problems.push("CAD catalog must match exactly one forward owner profile vector.");
      const modelIds = new Set<string>();
      const memberIds = new Set<string>();
      const modelDirectories = actualSourceDirectories.filter((path) => dirname(path) === physicalRoot).map((path) => basename(path));
      if (modelDirectories.length === 0) problems.push("CAD catalog must contain at least one manifest-owned model.");
      for (const modelDirectoryName of modelDirectories) {
        const manifestPath = `${physicalRoot}/${modelDirectoryName}/${destinationLayout ? canonicalFilenameForKind("json", taxonomy) : catalog.modelManifestSourceFilename}`;
        const manifestNode = pathOwners.get(manifestPath);
        const manifest = manifestNode ? projectionJsonManifest(manifestNode, `Model manifest ${JSON.stringify(manifestPath)}`, problems) : null;
        if (!manifestNode) problems.push(`Model manifest is missing for ${JSON.stringify(modelDirectoryName)}.`);
        const leading = modelDirectoryName.match(/^(\p{Extended_Pictographic}\uFE0F?(?:\u200D\p{Extended_Pictographic}\uFE0F?)*)/u)?.[1] ?? "";
        const semanticStem = modelDirectoryName.slice(leading.length);
        if (!leading || canonicalSemanticDirectoryName(modelDirectoryName, taxonomy) !== modelDirectoryName || manifest?.schema !== catalog.modelManifestSchema || manifest?.[catalog.memberVersionField] !== catalog.requiredMemberVersion || manifest?.[catalog.modelIdentityField] !== semanticStem) problems.push(`Model manifest ${JSON.stringify(manifestPath)} must declare the canonical directory id, schema, and version.`);
        if (typeof manifest?.[catalog.modelIdentityField] === "string") {
          const id = manifest[catalog.modelIdentityField] as string;
          if (modelIds.has(id)) problems.push(`Model manifest identity ${JSON.stringify(id)} is duplicated.`);
          modelIds.add(id);
        }
      }
      for (const physicalPath of actualFiles) {
        const relativePath = physicalPath.slice(physicalRoot.length + 1);
        let segments = relativePath.split("/");
        if (destinationLayout) {
          if (segments.length === 2 && segments[1] === canonicalFilenameForKind("json", taxonomy)) segments = [segments[0]!, catalog.modelManifestSourceFilename];
          else {
            const rule = catalog.categoryRules.find((candidate) => candidate.sourceDirectoryName === segments[1]);
            if (segments.length !== 4 || segments[3] !== canonicalFilenameForKind("json", taxonomy) || !rule) problems.push(`Canonical CAD category member ${JSON.stringify(physicalPath)} does not match its exact category rule.`);
            else if (rule.sourceShape === "direct-semantic-json" && segments[2]!.startsWith(rule.memberDirectoryEmoji) && segments[2]!.length > rule.memberDirectoryEmoji.length) segments = [segments[0]!, segments[1]!, `🔣️${segments[2]!.slice(rule.memberDirectoryEmoji.length)}.json`];
            else if (rule.sourceShape === "nested-fixed-json") segments = [segments[0]!, segments[1]!, segments[2]!, rule.fixedSourceFilename];
            else problems.push(`Canonical CAD member ${JSON.stringify(physicalPath)} has no exact semantic directory identity.`);
          }
        }
        const sourcePath = `${options.sourceRoot}/${segments.join("/")}`;
        const modelDirectoryName = segments[0] ?? "";
        let destinationRelativePath = "";
        let expectedSchema = catalog.modelManifestSchema;
        if (segments.length === 2 && segments[1] === catalog.modelManifestSourceFilename) destinationRelativePath = `${modelDirectoryName}/${canonicalFilenameForKind("json", taxonomy)}`;
        else {
          const rule = catalog.categoryRules.find((candidate) => candidate.sourceDirectoryName === segments[1]);
          if (!rule) problems.push(`Unknown CAD catalog category in ${JSON.stringify(sourcePath)}.`);
          else if (rule.sourceShape === "direct-semantic-json" && segments.length === 3) {
            const match = segments[2]!.match(/^🔣️(.+)\.json$/u);
            if (!match) problems.push(`CAD direct category file ${JSON.stringify(sourcePath)} does not have the exact semantic JSON shape.`);
            else destinationRelativePath = `${modelDirectoryName}/${rule.sourceDirectoryName}/${rule.memberDirectoryEmoji}${match[1]}/${canonicalFilenameForKind("json", taxonomy)}`;
            expectedSchema = rule.manifestSchema;
          } else if (rule.sourceShape === "nested-fixed-json" && segments.length === 4 && segments[3] === rule.fixedSourceFilename && canonicalSemanticDirectoryName(segments[2]!, taxonomy) === segments[2]) {
            destinationRelativePath = `${modelDirectoryName}/${rule.sourceDirectoryName}/${segments[2]}/${canonicalFilenameForKind("json", taxonomy)}`;
            expectedSchema = rule.manifestSchema;
          } else problems.push(`CAD category member ${JSON.stringify(sourcePath)} does not match its exact category rule.`);
        }
        const node = pathOwners.get(physicalPath)!;
        const manifest = projectionJsonManifest(node, `Catalog manifest ${JSON.stringify(sourcePath)}`, problems);
        if (manifest && (manifest.schema !== expectedSchema || manifest[catalog.memberVersionField] !== catalog.requiredMemberVersion || typeof manifest[catalog.memberIdentityField] !== "string" || manifest[catalog.memberIdentityField] === "")) problems.push(`Catalog manifest ${JSON.stringify(sourcePath)} has an invalid manifest schema, identity, or version.`);
        if (manifest && typeof manifest[catalog.memberIdentityField] === "string") {
          const key = `${modelDirectoryName}\0${segments[1] ?? ""}\0${manifest[catalog.memberIdentityField]}`;
          if (memberIds.has(key)) problems.push(`Distributed catalog member identity ${JSON.stringify(key)} is duplicated.`);
          memberIds.add(key);
        }
        if (destinationRelativePath) {
          const destinationPath = `${root.destinationRoot}/${destinationRelativePath}`;
          if (destinationLayout && destinationPath !== physicalPath) problems.push(`Canonical CAD member ${JSON.stringify(physicalPath)} does not equal its forward-rendered catalog path.`);
          candidateMappings.push({ sourcePath, destinationPath });
        }
      }
    } else if (catalog && "contractKind" in catalog && catalog.contractKind === "exact-owner-vectors") {
      const capture = root.captures.commandDirectoryName;
      const vectors = catalog.vectors.filter((vector) => vector.artifactId === basename(options.artifactRoot) && vector.standardVersion === root.captures.standardVersion && vector.subsetId === root.captures.subsetId && vector.commandDirectoryName === capture);
      if (vectors.length !== 1) problems.push("Draw command source must match exactly one owner vector.");
      const descendant = taxonomy.semanticDescendantContracts[contract.descendantContractId];
      if (!descendant || "contractKind" in descendant) problems.push("Draw projection must reference one exact descendant bundle.");
      else {
        const expectedSourceNodes = new Set<string>();
        for (const node of descendant.requiredNodes) {
          const sourceParent = node.pathSegments.map((segment) => segment.literal);
          const destinationRelativePath = semanticDescendantNodeRelativePath(node, taxonomy);
          let sourceRelativePath = sourceParent.join("/");
          if (node.nodeType === "file") {
            sourceRelativePath = [...sourceParent, "kindId" in node ? node.sourceFilename ?? canonicalFilenameForKind(node.kindId, taxonomy) : fixedContractFilename(taxonomy.fixedFilenameContracts[node.fixedFilenameContractId]!)].join("/");
            candidateMappings.push({ sourcePath: `${options.sourceRoot}/${sourceRelativePath}`, destinationPath: `${root.destinationRoot}/${destinationRelativePath}` });
          }
          expectedSourceNodes.add(destinationLayout ? destinationRelativePath ? `${root.destinationRoot}/${destinationRelativePath}` : root.destinationRoot : sourceRelativePath ? `${options.sourceRoot}/${sourceRelativePath}` : options.sourceRoot);
        }
        if (destinationLayout) for (const directory of projectionDirectories(root.destinationRoot, candidateMappings.map(({ destinationPath }) => destinationPath))) expectedSourceNodes.add(directory);
        const actualSourceNodes = new Set(options.nodes.filter((node) => node.nodeKind !== "symlink").map((node) => node.path));
        if (expectedSourceNodes.size !== actualSourceNodes.size || [...expectedSourceNodes].some((path) => !actualSourceNodes.has(path))) problems.push("Draw source does not contain the exact command bundle.");
      }
    } else problems.push(`Projection catalog ${JSON.stringify(contract.catalogContractId)} has the wrong authority kind.`);
  }
  candidateMappings.sort((left, right) => projectionByteCompare(left.sourcePath, right.sourcePath));
  const destinationDirectories = root.destinationRoot ? projectionDirectories(root.destinationRoot, candidateMappings.map(({ destinationPath }) => destinationPath)) : [];
  const destinationNodes = [...destinationDirectories.map((path) => ({ path, nodeKind: "directory" as const })), ...candidateMappings.map(({ destinationPath: path }) => ({ path, nodeKind: "file" as const }))];
  problems.push(...artifactProjectionPathProblems(destinationNodes, options.occupiedPaths ?? [], taxonomy, options.occupiedExclusionRoot));
  const accepted = problems.length === 0;
  return {
    contractId: options.contractId,
    sourceRoot: options.sourceRoot,
    destinationRoot: root.destinationRoot,
    mappings: accepted ? candidateMappings : [],
    destinationDirectoryCount: accepted ? destinationDirectories.length : 0,
    destinationNodeCount: accepted ? destinationNodes.length : 0,
    mappingDigest: accepted ? projectionFileMappingDigest(candidateMappings) : "",
    maxPathUtf16Units: accepted ? Math.max(0, ...destinationNodes.map(({ path }) => pathUtf16Units(path))) : 0,
    problems,
  };
}
