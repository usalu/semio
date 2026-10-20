import { leadingEmojiIdentity } from "../../../../../../📦️packages/🟦️typescript/🪪️identity/🧩️grapheme/🟦️.ts";
import type { SemanticDescendantNode } from "../📜️taxonomy-contracts/🟦️.ts";
import { pathUtf16Units } from "../📏️path-budget/🟦️.ts";
import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 🛤️ Audits the semantic directory member kinds, the projected member kinds, the profile renderers and the descendant contracts. */
export function auditMemberAndDescendantContracts(audit: TaxonomyAudit): void {
  const { taxonomy, problems, canonicalEmoji, record, ids, fullPattern, exactKeys, kebabId, semanticDirectoryKindId, fileKindIdForSourcePath, semanticDescendantNodeRelativePath } = audit;
  if (record(taxonomy.semanticDirectoryMemberKinds, "semanticDirectoryMemberKinds")) {
    const allDirectoryKinds = { ...taxonomy.semanticDirectoryKinds, ...taxonomy.semanticDirectoryMemberKinds, ...taxonomy.semanticProjectedMemberKinds };
    const ownerMembers = new Set<string>();
    const ownerPatterns = new Set<string>();
    for (const [id, spec] of Object.entries(taxonomy.semanticDirectoryMemberKinds)) {
      ids(spec.ownerKindIds, allDirectoryKinds, `semanticDirectoryMemberKinds[${JSON.stringify(id)}].ownerKindIds`);
      exactKeys(spec, spec.memberSlugPattern === undefined ? ["ownerKindIds", "memberNames", "source"] : ["ownerKindIds", "memberNames", "memberSlugPattern", "source"], `semanticDirectoryMemberKinds[${JSON.stringify(id)}]`);
      if (!Array.isArray(spec.memberNames) || (spec.memberNames.length === 0 && spec.memberSlugPattern === undefined)) problems.push(`semanticDirectoryMemberKinds[${JSON.stringify(id)}].memberNames must be non-empty without a memberSlugPattern.`);
      if (spec.memberSlugPattern !== undefined) {
        const key = `semanticDirectoryMemberKinds[${JSON.stringify(id)}].memberSlugPattern`;
        fullPattern(spec.memberSlugPattern, key);
        if (typeof spec.memberSlugPattern === "string") {
          try {
            if (new RegExp(spec.memberSlugPattern, "u").test("")) problems.push(`${key} must not match an empty slug.`);
          } catch {}
          for (const owner of spec.ownerKindIds ?? []) {
            const collision = `${owner}\0${spec.memberSlugPattern}`;
            if (ownerPatterns.has(collision)) problems.push(`semanticDirectoryMemberKinds pattern rows collide for owner ${JSON.stringify(owner)} and pattern ${JSON.stringify(spec.memberSlugPattern)}.`);
            ownerPatterns.add(collision);
          }
        }
      }
      for (const name of spec.memberNames ?? []) {
        if (typeof name !== "string" || name !== name.normalize("NFC") || /[\\/]/u.test(name) || !canonicalEmoji(leadingEmojiIdentity(name).first)) problems.push(`semanticDirectoryMemberKinds[${JSON.stringify(id)}] has invalid exact member ${JSON.stringify(name)}.`);
        for (const owner of spec.ownerKindIds ?? []) {
          const key = `${owner}\0${name}`;
          if (ownerMembers.has(key)) problems.push(`semanticDirectoryMemberKinds collide for owner ${JSON.stringify(owner)} and member ${JSON.stringify(name)}.`);
          ownerMembers.add(key);
        }
      }
      if (spec.source !== "registry") problems.push(`semanticDirectoryMemberKinds[${JSON.stringify(id)}].source must be registry.`);
    }
  }

  const projectionDirectoryKinds = { ...taxonomy.semanticDirectoryKinds, ...taxonomy.semanticDirectoryMemberKinds, ...taxonomy.semanticProjectedMemberKinds };
  if (record(taxonomy.semanticProjectedMemberKinds, "semanticProjectedMemberKinds")) {
    for (const [id, spec] of Object.entries(taxonomy.semanticProjectedMemberKinds)) {
      kebabId(id, `semanticProjectedMemberKinds id ${JSON.stringify(id)}`);
      exactKeys(spec, ["ownerKindIds", "projectionContractId", "sourceMemberKindId", "identityField"], `semanticProjectedMemberKinds[${JSON.stringify(id)}]`);
      ids(spec.ownerKindIds, projectionDirectoryKinds, `semanticProjectedMemberKinds[${JSON.stringify(id)}].ownerKindIds`);
      if (spec.ownerKindIds.length === 0) problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].ownerKindIds must be non-empty.`);
      if (!taxonomy.semanticPathProjectionContracts[spec.projectionContractId] && spec.projectionContractId !== taxonomy.mutationCatalogProjection.contractId) problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].projectionContractId is missing.`);
      if (!taxonomy.semanticDirectoryMemberKinds[spec.sourceMemberKindId]) problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].sourceMemberKindId is missing.`);
      const expectedIdentityField = spec.projectionContractId === taxonomy.mutationCatalogProjection.contractId ? "mutationDirectoryName" : spec.projectionContractId === "artifact-editor-command-bundle-v1" ? "commandDirectoryName" : null;
      if (spec.identityField !== expectedIdentityField) problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].identityField does not match its projection contract.`);
    }
    const visiting = new Set<string>();
    const visited = new Set<string>();
    const visit = (id: string): void => {
      if (visiting.has(id)) {
        problems.push(`semanticProjectedMemberKinds contains an owner cycle at ${JSON.stringify(id)}.`);
        return;
      }
      if (visited.has(id)) return;
      visiting.add(id);
      for (const owner of taxonomy.semanticProjectedMemberKinds[id]?.ownerKindIds ?? []) if (taxonomy.semanticProjectedMemberKinds[owner]) visit(owner);
      visiting.delete(id);
      visited.add(id);
    };
    for (const id of Object.keys(taxonomy.semanticProjectedMemberKinds)) visit(id);
  }

  if (record(taxonomy.semanticPathProjectionProfileRenderers, "semanticPathProjectionProfileRenderers")) for (const [id, renderer] of Object.entries(taxonomy.semanticPathProjectionProfileRenderers)) {
    kebabId(id, `semanticPathProjectionProfileRenderers id ${JSON.stringify(id)}`);
    exactKeys(renderer, ["direction", "captureFields", "directoryKindId", "template", "tupleCollisionFields"], `semanticPathProjectionProfileRenderers[${JSON.stringify(id)}]`);
    if (renderer.direction !== "forward-only") problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].direction must be forward-only.`);
    if (renderer.captureFields.join("\0") !== "standardVersion\0subsetId") problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].captureFields must be exactly standardVersion, subsetId.`);
    if (!taxonomy.semanticDirectoryKinds[renderer.directoryKindId]) problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].directoryKindId is missing.`);
    if (renderer.template !== "🪆️{standardVersion}-{subsetId}") problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].template must be the exact forward profile renderer.`);
    if (renderer.tupleCollisionFields.join("\0") !== "artifactId\0standardVersion\0subsetId") problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].tupleCollisionFields must be exactly artifactId, standardVersion, subsetId.`);
  }

  const validateDescendantNode = (node: SemanticDescendantNode, key: string, rootKindId: string): string | null => {
    const authorityKey = node.nodeType === "file" && !("kindId" in node) ? "fixedFilenameContractId" : "kindId";
    exactKeys(node, ["pathSegments", "nodeType", authorityKey, ...("sourceFilename" in node ? ["sourceFilename"] : [])], key);
    const validateSegments = (segments: readonly Readonly<{ kindId: string; literal: string }>[] | unknown, field: string): void => {
      if (!Array.isArray(segments)) {
        problems.push(`${key}.${field} must be an array.`);
        return;
      }
      let parentKindId = rootKindId;
      for (const [index, segment] of segments.entries()) {
        exactKeys(segment, ["kindId", "literal"], `${key}.${field}[${index}]`);
        if (!taxonomy.semanticDirectoryKinds[segment.kindId]) problems.push(`${key}.${field}[${index}].kindId is missing.`);
        else if (semanticDirectoryKindId(segment.literal, taxonomy, { parentKindId }) !== segment.kindId) problems.push(`${key}.${field}[${index}].literal does not resolve uniquely to its kind.`);
        parentKindId = segment.kindId;
      }
    };
    validateSegments(node.pathSegments, "pathSegments");
    if (node.nodeType === "directory") {
      if (!projectionDirectoryKinds[node.kindId]) problems.push(`${key}.kindId is not a directory kind.`);
      const expected = node.pathSegments.at(-1)?.kindId ?? rootKindId;
      if (node.kindId !== expected) problems.push(`${key}.kindId must equal its realized directory kind ${JSON.stringify(expected)}.`);
    } else if (node.nodeType === "file") {
      if ("kindId" in node && !taxonomy.fileKinds[node.kindId]) problems.push(`${key}.kindId is not a file kind.`);
      else if ("kindId" in node && node.sourceFilename !== undefined && (node.sourceFilename !== node.sourceFilename.normalize("NFC") || /[\\/]/u.test(node.sourceFilename) || !node.sourceFilename.startsWith(taxonomy.fileKinds[node.kindId]!.emoji) || fileKindIdForSourcePath(node.sourceFilename, taxonomy) !== node.kindId)) problems.push(`${key}.sourceFilename must be one NFC basename resolving to kindId.`);
      else if (!("kindId" in node) && !taxonomy.fixedFilenameContracts[node.fixedFilenameContractId]) problems.push(`${key}.fixedFilenameContractId is missing.`);
    } else problems.push(`${key}.nodeType must be directory or file.`);
    try {
      return `${node.nodeType}:${semanticDescendantNodeRelativePath(node, taxonomy)}`;
    } catch {
      return null;
    }
  };

  if (record(taxonomy.semanticDescendantContracts, "semanticDescendantContracts")) for (const [id, contract] of Object.entries(taxonomy.semanticDescendantContracts)) {
    kebabId(id, `semanticDescendantContracts id ${JSON.stringify(id)}`);
    if ("contractKind" in contract) {
      exactKeys(contract, ["contractKind", "rootDirectoryKindId", "catalogContractId", "leafFileKindId", "rendering", "pathBudgetReserve"], `semanticDescendantContracts[${JSON.stringify(id)}]`);
      if (contract.contractKind !== "catalog") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].contractKind must be catalog.`);
      if (!taxonomy.semanticDirectoryKinds[contract.rootDirectoryKindId]) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].rootDirectoryKindId is missing.`);
      const catalog = taxonomy.semanticPathProjectionCatalogContracts[contract.catalogContractId];
      if (!catalog || !("contractKind" in catalog) || catalog.contractKind !== "distributed-json-manifest-catalog") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].catalogContractId must reference one distributed JSON manifest catalog.`);
      if (!taxonomy.fileKinds[contract.leafFileKindId]) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].leafFileKindId is missing.`);
      if (contract.rendering !== "semantic-member-directory-and-physical-kind-leaf") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].rendering is invalid.`);
      exactKeys(contract.pathBudgetReserve, ["derivation", "utf16Units"], `semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve`);
      if (contract.pathBudgetReserve.derivation !== "longest-rendered-catalog-descendant-suffix") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.derivation is invalid.`);
      if (!Number.isSafeInteger(contract.pathBudgetReserve.utf16Units) || contract.pathBudgetReserve.utf16Units <= 0 || contract.pathBudgetReserve.utf16Units >= taxonomy.collisionPolicy.maxPathUtf16Units) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.utf16Units must be a positive safe integer below maxPathUtf16Units.`);
      continue;
    }
    exactKeys(contract, ["rootDirectoryKindId", "requiredNodes", "exclusiveAlternatives", "realizedNodeCount", "pathBudgetReserve"], `semanticDescendantContracts[${JSON.stringify(id)}]`);
    if (!projectionDirectoryKinds[contract.rootDirectoryKindId]) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].rootDirectoryKindId is missing.`);
    if (!Array.isArray(contract.requiredNodes) || contract.requiredNodes.length === 0) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].requiredNodes must be non-empty.`);
    const requiredKeys = (contract.requiredNodes ?? []).map((node, index) => validateDescendantNode(node, `semanticDescendantContracts[${JSON.stringify(id)}].requiredNodes[${index}]`, contract.rootDirectoryKindId)).filter((key): key is string => key !== null);
    if (new Set(requiredKeys).size !== requiredKeys.length) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].requiredNodes must be unique.`);
    if (!Array.isArray(contract.exclusiveAlternatives)) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives must be an array.`);
    const alternativeIds = new Set<string>();
    const alternativeKeys: string[] = [];
    for (const [groupIndex, alternative] of (contract.exclusiveAlternatives ?? []).entries()) {
      exactKeys(alternative, ["id", "mode", "nodes"], `semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}]`);
      kebabId(alternative.id, `semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].id`);
      if (alternativeIds.has(alternative.id)) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}] has duplicate alternative id ${JSON.stringify(alternative.id)}.`);
      alternativeIds.add(alternative.id);
      if (alternative.mode !== "exactly-one") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].mode must be exactly-one.`);
      if (!Array.isArray(alternative.nodes) || alternative.nodes.length < 2) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].nodes must contain at least two nodes.`);
      for (const [nodeIndex, node] of (alternative.nodes ?? []).entries()) {
        const nodeKey = validateDescendantNode(node, `semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].nodes[${nodeIndex}]`, contract.rootDirectoryKindId);
        if (nodeKey) alternativeKeys.push(nodeKey);
      }
    }
    const allKeys = [...requiredKeys, ...alternativeKeys];
    if (new Set(allKeys).size !== allKeys.length) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}] descendant nodes must not overlap.`);
    const requiredRealizedKeys = new Set(requiredKeys);
    for (const key of requiredKeys) if (key.startsWith("file:")) {
      const segments = key.slice(5).split("/");
      segments.pop();
      let directory = "";
      for (const segment of segments) {
        directory = directory ? `${directory}/${segment}` : segment;
        requiredRealizedKeys.add(`directory:${directory}`);
      }
    }
    if (contract.realizedNodeCount !== requiredRealizedKeys.size + (contract.exclusiveAlternatives?.length ?? 0)) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].realizedNodeCount must equal realized required destination nodes plus one node per exclusive group.`);
    exactKeys(contract.pathBudgetReserve, ["derivation", "utf16Units"], `semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve`);
    if (contract.pathBudgetReserve.derivation !== "longest-canonical-descendant-suffix") problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.derivation is invalid.`);
    const derivedUnits = Math.max(0, ...allKeys.map((key) => pathUtf16Units(`/${key.slice(key.indexOf(":") + 1)}`)));
    if (contract.pathBudgetReserve.utf16Units !== derivedUnits) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.utf16Units must equal derived longest suffix ${derivedUnits}.`);
    if (contract.pathBudgetReserve.utf16Units >= taxonomy.collisionPolicy.maxPathUtf16Units) problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve must be below maxPathUtf16Units.`);
    if (id === "mutation-implementation-case-v1") {
      const expectedRequired = ["directory:", "file:🦀️.rs"].sort();
      if ([...requiredKeys].sort().join("\0") !== expectedRequired.join("\0") || alternativeKeys.length !== 0 || contract.realizedNodeCount !== 2 || contract.pathBudgetReserve.utf16Units !== 7) problems.push("semanticDescendantContracts.mutation-implementation-case-v1 must encode the exact two-node Rust implementation case and 7-unit reserve.");
    }
    if (id === "mutation-fixture-bundle-v1") {
      const expectedRequired = ["directory:", "directory:🦠️mutation", "file:🦠️mutation/🔣️.json", "directory:📸️snapshot", "directory:📸️snapshot/⬅️before", "file:📸️snapshot/⬅️before/🔣️.json", "directory:📸️snapshot/➡️after", "file:📸️snapshot/➡️after/🔣️.json", "directory:🔺️diff", "directory:🎯️outcome", "file:🎯️outcome/🔣️.json"].sort();
      const expectedAlternatives = ["file:🔺️diff/🔣️.json", "file:🔺️diff/🚫️.absent"].sort();
      if ([...requiredKeys].sort().join("\0") !== expectedRequired.join("\0") || [...alternativeKeys].sort().join("\0") !== expectedAlternatives.join("\0") || contract.realizedNodeCount !== 12) problems.push("semanticDescendantContracts.mutation-fixture-bundle-v1 must encode the exact 12-node data bundle and exclusive diff alternatives.");
    }
    if (id === "draw-editor-command-bundle-v1") {
      const manifestSuffix = "/📦️packages/🦀️rust/Cargo.toml";
      const actual = new Map((contract.requiredNodes ?? []).flatMap((node): (readonly [string, string])[] => {
        try {
          return [[`${node.nodeType}:${semanticDescendantNodeRelativePath(node, taxonomy)}`, node.nodeType === "directory" ? "directory" : "kindId" in node ? `${node.kindId}:${node.sourceFilename ?? ""}` : `fixed:${node.fixedFilenameContractId}`]];
        } catch {
          return [];
        }
      }));
      const expected = new Map<string, string>([["directory:", "directory"], ["file:🦀️.rs", "rust-source:🦀️.rs"]]);
      for (const owner of [...actual.keys()].filter((key) => key.startsWith("file:") && key.endsWith(manifestSuffix)).map((key) => key.slice("file:".length, -manifestSuffix.length))) {
        for (const [name, authority] of [["🦀️.rs", "rust-source:🦀️.rs"], ["📦️packages/🦀️rust/Cargo.toml", "fixed:cargo-manifest"], ["📦️packages/🦀️rust/📋️project.json", "fixed:build-project-manifest"], ["📦️packages/🦀️rust/📜️script.ts", "fixed:root-script"]] as const) expected.set(`file:${owner}/${name}`, authority);
        const segments = `${owner}/📦️packages/🦀️rust`.split("/");
        for (let index = 1; index <= segments.length; index++) expected.set(`directory:${segments.slice(0, index).join("/")}`, "directory");
      }
      if (alternativeKeys.length !== 0 || actual.size !== expected.size || [...expected].some(([key, authority]) => actual.get(key) !== authority)) problems.push("semanticDescendantContracts.draw-editor-command-bundle-v1 must encode one root Rust command declaration and only declaration-only nested Rust packages.");
    }
  }
}
