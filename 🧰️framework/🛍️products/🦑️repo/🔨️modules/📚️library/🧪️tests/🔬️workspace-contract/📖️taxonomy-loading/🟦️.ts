import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { loadTaxonomy, validateTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { findWorkspaceRoot } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { artifactFacetPathIsDeclared } from "../../../🔍️discovery/🪪️mutation-owner-identity/🟦️.ts";
import { type Taxonomy } from "../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";
import { registerParsesTaxonomyJsonExpectedTests } from "./🔶️parses-taxonomy-json-expected/🟦️.ts";
import { registerRequiresOwnedGeneratorExposeTests } from "./🔸️requires-owned-generator-expose/🟦️.ts";
import { registerRejectsUnsettledBroadForeignTests } from "./🟠️rejects-unsettled-broad-foreign/🟦️.ts";

describe("loadTaxonomy", () => {
  registerParsesTaxonomyJsonExpectedTests();
  registerRequiresOwnedGeneratorExposeTests();
  registerRejectsUnsettledBroadForeignTests();
});

describe("validateTaxonomy", () => {
  test("the shipped vocabulary is internally consistent", () => {
    expect(validateTaxonomy()).toEqual([]);
  });

  test("each rule group of the decomposed validator reports its corruption exactly as the pre-split validator did", () => {
    const base = JSON.parse(readFileSync(join(findWorkspaceRoot(contractRoot), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")) as Record<string, unknown>;
    const rows: readonly Readonly<{ group: string; path: readonly (string | number)[]; value: unknown; expected: readonly string[] }>[] = [
    { group: "auditSchemaVersionAndEvidence", path: ["schemaVersion"], value: 6, expected: ["schemaVersion must be exactly 7, got 6."] },
    { group: "auditLeafAndReferencePolicies", path: ["physicalLeafRendering"], value: {}, expected: ["physicalLeafRendering must be a non-empty object.", "physicalLeafRendering must declare exact forward-only kind-only rendering, longest source extension, primary authoring extension, and canonical-only runtime lookup."] },
    { group: "auditFileKinds", path: ["fileKinds", "json", "emoji"], value: "x", expected: ["fileKinds[\"json\"].emoji must be one canonical NFC emoji sequence.", "transaction-attempt-preparation must retain its validate-all exact child and duplicate-sibling authority.", "transaction-lease-preparation must admit one complete canonical JSON lease before publication.", "semanticDescendantContracts[\"mutation-fixture-bundle-v1\"].pathBudgetReserve.utf16Units must equal derived longest suffix 28.", "semanticDescendantContracts.mutation-fixture-bundle-v1 must encode the exact 12-node data bundle and exclusive diff alternatives."] },
    { group: "auditScopedAndDirectoryKinds", path: ["scopedFileKinds"], value: [], expected: ["scopedFileKinds must be an object.", "scopedFileKinds.transaction-edit-candidate must remain the exact transaction edit candidate authority.", "scopedFileKinds.transaction-edit-preimage must remain the exact transaction edit displaced-preimage authority.", "scopedFileKinds.transaction-edit-write-candidate must remain the exact unpublished edit-writer authority.", "scopedFileKinds.transaction-backup-write-candidate must remain the exact unpublished backup-writer authority.", "scopedFileKinds.transaction-json-previous must remain the exact displaced canonical JSON authority.", "scopedFileKinds.transaction-backup-candidate must remain the exact transaction restore preimage authority.", "scopedFileKinds.transaction-postimage-candidate must remain the exact transaction restore postimage authority."] },
    { group: "auditTransactionArtifacts", path: ["scopedFileKinds", "transaction-edit-candidate", "pathPattern"], value: "**/x", expected: ["scopedFileKinds.transaction-edit-candidate must remain the exact transaction edit candidate authority."] },
    { group: "auditMemberAndDescendantContracts", path: ["semanticPathProjectionProfileRenderers"], value: 1, expected: ["semanticPathProjectionProfileRenderers must be a non-empty object.", "semanticPathProjectionContracts[\"artifact-example-model-catalog-v1\"].profileRendererId is missing.", "semanticPathProjectionContracts[\"artifact-example-model-catalog-v1\"] has invalid forward profile authority: Invalid forward profile vector for artifact-example-model-catalog-v1: Unknown or incomplete semantic projection \"artifact-example-model-catalog-v1\".", "semanticPathProjectionContracts[\"artifact-editor-command-bundle-v1\"].profileRendererId is missing.", "semanticPathProjectionContracts[\"artifact-editor-command-bundle-v1\"] has invalid forward profile authority: Invalid forward profile vector for artifact-editor-command-bundle-v1: Unknown or incomplete semantic projection \"artifact-editor-command-bundle-v1\"."] },
    { group: "auditProjectionAndPackageContracts", path: ["semanticPolicyStateCoordinateContracts"], value: 1, expected: ["semanticPolicyStateCoordinateContracts must be a non-empty object."] },
    { group: "auditOwnedFileProjection", path: ["semanticOwnedFileProjectionContracts"], value: {}, expected: ["semanticOwnedFileProjectionContracts must be a non-empty object.", "semanticOwnedFileProjectionContracts must contain the exact artifact-facet, README/LICENSE, ticket-document, active ticket-important, and history ticket-important contracts."] },
    { group: "auditReferenceConsumersAndMutationCatalog", path: ["semanticPathProjectionReferenceConsumerContracts"], value: 1, expected: ["semanticPathProjectionReferenceConsumerContracts must be a non-empty object."] },
    { group: "auditFixedContracts", path: ["fixedFilenameRejectionContracts"], value: 1, expected: ["fixedFilenameRejectionContracts must be a non-empty object."] },
    { group: "auditPackageBoundaryContracts", path: ["packageSourceDispositions"], value: 1, expected: ["packageSourceDispositions must be a non-empty object."] },
    { group: "auditGeneratorContracts", path: ["generatorContracts"], value: {}, expected: ["semanticOwnedFileProjectionContracts[\"readme-license-owner-leaves-v1\"] has an unregistered generator owner.", "generatorContracts must be a non-empty object."] },
    { group: "auditPathPoliciesAndAreas", path: ["unicodeNormalization"], value: {}, expected: ["unicodeNormalization must be NFC/lower/und."] },
    { group: "auditDependencyDirections", path: ["areaLayers"], value: {}, expected: ["areaLayers must be a non-empty object."] },
    { group: "auditEcosystemsAndTargets", path: ["targets", "__probe__"], value: {}, expected: ["targets[\"__probe__\"].lang is missing.", "targets[\"__probe__\"].componentFileKindId is missing.", "targets[\"__probe__\"].entryContractIds must be an array."] },
    { group: "auditTestVocabulary", path: ["testExamplesDirName"], value: "x", expected: ["testExamplesDirName must select the canonical \"📚️examples\" collection.", "Semantic directory \"x\" is not uniquely registered."] },
    { group: "auditManifestAndRootContracts", path: ["rootDataContractIds"], value: ["missing"], expected: ["rootDataContractIds references missing id \"missing\"."] },
    { group: "auditSchemaFormatsAndFacets", path: ["schemaJsonDialect"], value: "x", expected: ["schemaJsonDialect must be the single dialect mutationPayloadSchemaAuthority already declares."] },
    { group: "auditMutationVocabulary", path: ["mutationComponentFileKindId"], value: "x", expected: ["mutationComponentFileKindId references a missing file kind.", "mutationComponentFileKindId must select the direct owner \"rust-source\" kind."] },
    { group: "auditRepresentationAndChildDirectories", path: ["nativeRepresentationDirs"], value: [], expected: ["nativeRepresentationDirs must be a non-empty unique subset of representationDirs."] },
    ];
    expect(new Set(rows.map((row) => row.group)).size).toBe(20);
    for (const row of rows) {
      const clone = structuredClone(base) as Record<string, any>;
      const parent = row.path.slice(0, -1).reduce<any>((value, key) => value[key], clone);
      parent[row.path[row.path.length - 1]!] = row.value;
      expect(validateTaxonomy(clone as unknown as Taxonomy), `${row.group}: ${row.path.join(".")}`).toEqual([...row.expected]);
    }
  }, 120_000);

  test("no rule group, audit context or scope vocabulary module reaches the authority module that injects its ports", () => {
    const library = join(findWorkspaceRoot(contractRoot), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"), discovery = join(library, "🔍️discovery"), authority = join(discovery, "🔐️taxonomy-authority/🟦️.ts");
    const relativeImports = (file: string): string[] => [...readFileSync(file, "utf8").matchAll(/^(?:import|export)\s[^;]*?\sfrom\s"(\.[^"]+)";$/gmu)].map((match) => join(file, "..", match[1]!));
    const owners = relativeImports(authority).filter((file) => /\/(?:[^/]*audit[^/]*)\/🟦️\.ts$/u.test(file));
    expect(owners).toHaveLength(21);
    for (const owner of owners) {
      expect(readFileSync(owner, "utf8").split("\n").length, owner).toBeLessThanOrEqual(600);
      const seen = new Set<string>([owner]), queue = [owner];
      while (queue.length > 0) for (const next of relativeImports(queue.pop()!)) if (!seen.has(next)) { seen.add(next); queue.push(next); }
      expect(seen.has(authority), `${owner} reaches the authority module`).toBe(false);
    }
  });

  test("freezes the four canonical testing collections and the obsolete-category stems", () => {
    const taxonomy = JSON.parse(readFileSync(join(findWorkspaceRoot(contractRoot), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")) as Taxonomy;
    expect({ tests: taxonomy.testsDirName, fixtures: taxonomy.testFixturesDirName, examples: taxonomy.testExamplesDirName, oracles: taxonomy.testOraclesDirName }).toEqual({ tests: "🧪️tests", fixtures: "🧫️fixtures", examples: "📚️examples", oracles: "🔮️oracles" });
    expect("testContributionDirName" in taxonomy).toBe(false);
    expect("testContributionDirectoryOverrides" in taxonomy).toBe(false);
    const singularOracle = { ...taxonomy, testOraclesDirName: "🔮️oracle" } as Taxonomy;
    expect(validateTaxonomy(singularOracle).some((problem) => problem.includes('canonical "🔮️oracles" collection'))).toBe(true);
    const singularExample = { ...taxonomy, testExamplesDirName: "📚️example" } as Taxonomy;
    expect(validateTaxonomy(singularExample).some((problem) => problem.includes('canonical "📚️examples" collection'))).toBe(true);
    const duplicateStem = { ...taxonomy, testObsoleteCategoryStems: [...taxonomy.testObsoleteCategoryStems, taxonomy.testObsoleteCategoryStems[0]!] } as Taxonomy;
    expect(validateTaxonomy(duplicateStem).some((problem) => problem.includes("testObsoleteCategoryStems") && problem.includes("unique normalized lowercase stems"))).toBe(true);
    const uppercaseStem = { ...taxonomy, testObsoleteTestEmojiCategoryStems: [...taxonomy.testObsoleteTestEmojiCategoryStems, "Helper"] } as Taxonomy;
    expect(validateTaxonomy(uppercaseStem).some((problem) => problem.includes("testObsoleteTestEmojiCategoryStems") && problem.includes("unique normalized lowercase stems"))).toBe(true);
  });

  test("discriminates catalog descendants from exact bundles and diagnoses missing exact alternatives", () => {
    const taxonomy = structuredClone(loadTaxonomy()) as Taxonomy;
    expect(validateTaxonomy(taxonomy)).toEqual([]);
    const malformed = structuredClone(taxonomy) as Taxonomy;
    delete (malformed.semanticDescendantContracts["draw-editor-command-bundle-v1"] as unknown as { exclusiveAlternatives?: unknown }).exclusiveAlternatives;
    expect(validateTaxonomy(malformed).some((problem) => problem.includes("draw-editor-command-bundle-v1") && problem.includes("exclusiveAlternatives must be an array"))).toBe(true);
    const wrongCatalog = structuredClone(taxonomy) as Taxonomy;
    (wrongCatalog.semanticDescendantContracts["cad-model-catalog-bundle-v1"] as unknown as { catalogContractId: string }).catalogContractId = "draw-editor-command-vectors-v1";
    expect(validateTaxonomy(wrongCatalog).some((problem) => problem.includes("cad-model-catalog-bundle-v1") && problem.includes("distributed JSON manifest catalog"))).toBe(true);
  });

  test("declares schema facet kinds partitioning data vs interface formats", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.schemaFacetKinds?.["🧬️data"]?.normativeFormat).toBe("🔣️jsonschema");
    expect(taxonomy.schemaFacetKinds?.["📜️interface"]?.normativeFormat).toBe("📜️wit");
    expect(taxonomy.schemaFacetKinds?.["📜️interface"]?.formats).toEqual(["📜️wit"]);
    expect(taxonomy.schemaFormats?.["📜️wit"]?.fieldCasing).toBe("kebab");
    expect(taxonomy.packagingDirectoryKindIds).toEqual(["targets", "fixtures", "apps"]);
    expect(taxonomy.ecosystems["🦀️rust"]?.packagingDirectoryKindIds).toEqual(["benchmarks"]);
  });

  test("declares canonical semantic collection and module ownership contracts", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.semanticManifestFileKindId).toBe("json");
    expect(taxonomy.semanticExtensionKey).toBe("x-semio");
    expect(taxonomy.semanticConsumerMinimum).toBe(2);
    expect(taxonomy.semanticCollections["🔨️modules"]?.kind).toBe("module");
    expect(taxonomy.semanticCollections["💡️inferences"]?.kind).toBe("inference");
    expect(taxonomy.semanticCollections["🚪️io/📝️text/🧬️mutations"]).toEqual({ kind: "io", direction: "transport" });
    expect(taxonomy.ioSemanticCollectionDirNames).toEqual(["📸️snapshot", "🔺️diff", "💡️inferences", "🧬️mutations"]);
    expect(artifactFacetPathIsDeclared("🚪️io/📝️text/🧬️mutations", taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🚪️io/💾️binary/🧬️mutations", taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🚪️io/📝️text/🧬️mutations/📝️text", taxonomy)).toBe(false);
    expect(artifactFacetPathIsDeclared("🚪️io/💾️binary/🧬️mutations/💾️binary", taxonomy)).toBe(false);
    expect(taxonomy.ioSemanticCollectionDirNames).toContain("🧬️mutations");
  });

  test("declares the surface vocabulary ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET froze", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.schemaVersion).toBe(7);
    expect(taxonomy.viewerDirName).toBe("👁️viewer");
    expect(taxonomy.editorDirName).toBe("✏️editor");
    expect(taxonomy.surfaceRoles).toEqual(["viewer", "editor"]);
    expect(taxonomy.surfaceDirNames).toEqual({ viewer: "👁️viewer", editor: "✏️editor" });
    expect(taxonomy.subsetChildDirs).toContain("👁️viewer");
    expect(taxonomy.subsetChildDirs).toContain("✏️editor");
    expect(taxonomy.subsetRequiredSurfaceDirs).toEqual(["👁️viewer", "✏️editor"]);
    expect(taxonomy.contributedSubsetChildDirs).toEqual(["👁️viewer", "✏️editor"]);
    expect(taxonomy.surfaceChildDirs).toContain(taxonomy.modesDirName);
    expect(taxonomy.surfaceComponentLangs).toEqual(["🦀️rust", "🟦️typescript"]);
    expect(taxonomy.windowLeafLangs).toEqual(["🦀️rust", "🟦️typescript"]);
  });

  test("surfaceRoles preserves the AppRole declaration and wire-tag order", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.surfaceRoles).toEqual(["viewer", "editor"]);
  });

  test("surface role mappings name their registered v7 directories", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.surfaceDirNames).toEqual({ viewer: taxonomy.viewerDirName, editor: taxonomy.editorDirName });
  });

  test("required surface children belong to the structural set and cover every state lane", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.surfaceRequiredChildDirs.every((dir) => taxonomy.surfaceChildDirs.includes(dir))).toBe(true);
    expect(taxonomy.surfaceRequiredChildDirs).toEqual(expect.arrayContaining(["🎚️config", "👥️presence", "🫧️transient"]));
  });

  test("every surface directory belongs to the subset structural set", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.subsetSurfaceDirs.every((dir) => taxonomy.subsetChildDirs.includes(dir))).toBe(true);
  });

  test("rejects any area outside the required clean state", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, areas: { ...taxonomy.areas, "🧰️framework": "taxonomy" as never } };
    expect(validateTaxonomy(broken).some((problem) => problem.includes('must be "clean"'))).toBe(true);
  });

  test("keeps plugin roots inside the clean area registry", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.pluginAreas.every((area) => taxonomy.areas[area] === "clean")).toBe(true);
    const broken = { ...taxonomy, areas: { ...taxonomy.areas, [taxonomy.pluginAreas[0]!]: "mixed" as never } };
    expect(validateTaxonomy(broken).some((problem) => problem.includes('must be "clean"'))).toBe(true);
  });

  test("reports a completeness dir missing from the structural set", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, artifactChildDirs: taxonomy.artifactChildDirs.filter((dir) => dir !== "🚪️io") };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("🚪️io"))).toBe(true);
  });

  test("keeps derived lifecycle directories out of the subset structural set", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.subsetChildDirs.filter((dir) => ["🏗️builder", "🧐️analyzer", "🎹️composer"].includes(dir))).toEqual([]);
  });

  test("registers every mutation facet (behavior and organizational) as a taxonomy leaf parent", () => {
    const taxonomy = loadTaxonomy();
    expect([...taxonomy.mutationBehaviorFacetDirs, ...taxonomy.mutationOrganizationalFacetDirs].every((dir) => taxonomy.taxonomyLeafParentDirs.includes(dir))).toBe(true);
  });

  test("declares direct mutation ownership plus placement-bound behavior and optional organizational subfacets", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.mutationComponentFileKindId).toBe("rust-source");
    expect(taxonomy.mutationDescriptorFileKindId).toBe("json");
    expect(taxonomy.mutationBehaviorFacetDirs).toEqual(["🦠️mutation", "🔺️diff", "↩️inverse"]);
    expect(taxonomy.mutationOrganizationalFacetDirs).toEqual(["🧩️plan", "📝️text", "💾️binary", "🧬️schema"]);
    expect(new RegExp(taxonomy.mutationDirectoryPattern, "u").test("➕️add-node")).toBe(true);
    expect(new RegExp(taxonomy.mutationDirectoryPattern, "u").test("➕️node")).toBe(false);
    expect("mutationChildDirs" in taxonomy).toBe(false);
    expect("compositeMutationChildDirs" in taxonomy).toBe(false);
    expect(taxonomy.schemaChildDirs).toContain("🧬️mutations");
  });

  test("keeps required window capabilities non-empty and structural", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.windowRequiredChildDirs.length).toBeGreaterThan(0);
    expect(taxonomy.windowRequiredChildDirs.every((dir) => taxonomy.windowChildDirs.includes(dir))).toBe(true);
  });

  test("declares command facets at plugin and OS ownership boundaries", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.pluginRequiredChildDirs).toContain("🎮️commands");
    expect(taxonomy.osRequiredChildDirs).toContain("🎮️commands");
    expect(taxonomy.pluginRequiredChildDirs.every((dir) => taxonomy.pluginChildDirs.includes(dir))).toBe(true);
    expect(taxonomy.osRequiredChildDirs.every((dir) => taxonomy.osChildDirs.includes(dir))).toBe(true);
  });

  test("declares every state lane in each state-owning scope", () => {
    const taxonomy = loadTaxonomy();
    const stateLanes = ["🎚️config", "👥️presence", "🫧️transient"];
    expect(stateLanes.every((dir) => taxonomy.modeChildDirs.includes(dir))).toBe(true);
    expect(stateLanes.every((dir) => taxonomy.windowRequiredChildDirs.includes(dir))).toBe(true);
    expect(taxonomy.modeChildDirs).toContain(taxonomy.windowsDirName);
  });

  test("reports an invalid window component language set", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.windowComponentLangs.every((lang) => Boolean(taxonomy.componentFileKinds[lang]))).toBe(true);
    const missingMarkerKind = { ...taxonomy, windowEmptyFacetFileKindId: "unknown" };
    expect(validateTaxonomy(missingMarkerKind).some((problem) => problem.includes("windowEmptyFacetFileKindId references a missing file kind"))).toBe(true);
  });

  test("declares direct plugin-root facets", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.pluginChildDirs).toEqual(["🎮️commands", "🔨️modules", "🧬️schema"]);
  });

  test("includes commands at every command-owning scope", () => {
    const taxonomy = loadTaxonomy();
    for (const key of ["surfaceChildDirs", "modeChildDirs", "pluginChildDirs", "osChildDirs"] as const) {
      expect(taxonomy[key]).toContain("🎮️commands");
    }
  });

  test("keeps direct plugin-root facet declarations non-empty", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.pluginChildDirs.every((dir) => dir.length > 0)).toBe(true);
  });

  test("rejects plural example component dirs in taxonomyLeafParentDirs", () => {
    const taxonomy = loadTaxonomy();
    const broken = {
      ...taxonomy,
      taxonomyLeafParentDirs: [...taxonomy.taxonomyLeafParentDirs, "🎒️packs"],
    };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("🎒️packs"))).toBe(true);
  });

  test("declares a valid example slug pattern", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.exampleSlugPattern.length).toBeGreaterThan(0);
    expect(() => new RegExp(taxonomy.exampleSlugPattern, "u")).not.toThrow();
  });

  test("declares the schema lifecycle collections", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.schemaChildDirs).toEqual(["📸️snapshot", "🔺️diff", "🧬️mutations", "💡️inferences"]);
  });

  test("declares the schema representation kinds", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.representationDirs).toEqual(["📝️text", "💾️binary", "🪶️sqlite"]);
    expect(taxonomy.nativeRepresentationDirs).toEqual(["📝️text", "💾️binary"]);
  });

  test("keeps bare pack out of artifact structural and completeness sets", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.artifactComponentDirs).not.toContain("🎒️pack");
    expect(taxonomy.artifactChildDirs).not.toContain("🎒️pack");
  });

  test("keeps artifact specification mappings on fully qualified v7 facet paths", () => {
    const taxonomy = loadTaxonomy();
    expect("🎒️pack" in taxonomy.artifactSpecFileKinds).toBe(false);
    expect(taxonomy.artifactSpecFileKinds["🚪️io/💾️binary/📸️snapshot"]).toBe("protocol-semio");
  });

  test("reports a schema format referencing an unknown file kind", () => {
    const taxonomy = loadTaxonomy();
    const broken = {
      ...taxonomy,
      schemaFormats: {
        ...taxonomy.schemaFormats,
        "🦀️rust": { ...taxonomy.schemaFormats["🦀️rust"], fileKindId: "unknown" },
      },
    };
    expect(validateTaxonomy(broken).some((problem) => problem.includes('schemaFormats["🦀️rust"].fileKindId is missing'))).toBe(true);
  });

  test("reports artifact schema mappings referencing an unknown file kind", () => {
    const taxonomy = loadTaxonomy();
    const broken = {
      ...taxonomy,
      artifactSchemaSpecFileKinds: { ...taxonomy.artifactSchemaSpecFileKinds, "🧬️schema": "unknown" },
    };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("artifactSchemaSpecFileKinds") && problem.includes("unknown"))).toBe(true);
  });
});
