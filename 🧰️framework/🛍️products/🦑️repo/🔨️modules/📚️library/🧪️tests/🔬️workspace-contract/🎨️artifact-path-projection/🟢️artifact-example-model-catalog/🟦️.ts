/** 🟢️ The artifact example model catalog tests of the artifact path projection authority suite. */
import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { getWorkspaceRoot } from "../../../../🗂️workspaces/🟦️.ts";
import { loadTaxonomy, validateTaxonomy } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { semanticPathProjectionAuthority } from "../../../../🔍️discovery/🪞️path-projection/🟦️.ts";
import { type SemanticProjectionAuthorityNode } from "../../../../🔍️discovery/🧱️path-projection-contracts/🟦️.ts";
import { type Taxonomy } from "../../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { ownedFilePaths } from "../../../🔍️filesystem/🟦️.ts";
import { ARTIFACT_PROJECTION_GOLDEN, CAD_LIVE_BINDINGS, DRAW_SOURCE_SCENARIO, cadProjectionLiveSources, drawScenarioTaxonomy, projectionAuthorityNodes, projectionByteSort, projectionGolden } from "../../🏗️shared-fixtures/🟦️.ts";
import { projectionAuthority } from "../🟣️shared/🟦️.ts";

/** 🔵 Registers the artifact example model catalog tests of the artifact path projection authority suite. */
export function registerArtifactExampleModelCatalogTests(): void {
  test("artifact-example-model-catalog-projection is schema-owned and agrees with owned filesystem discovery", () => {
    const projection = projectionGolden("artifact-example-model-catalog-v1");
    const result = projectionAuthority(projection);
    const platformBoundary = ownedFilePaths(join(getWorkspaceRoot(), projection.sourceRoot)).filter((path) => path.endsWith(".json"))
      .map((path) => `${projection.sourceRoot}/${path}`)
      .sort(projectionByteSort);
    expect(platformBoundary).toEqual([...cadProjectionLiveSources(projection).values()].sort(projectionByteSort));
    expect(result.problems).toEqual([]);
    expect(result.mappings).toEqual(projection.mappings);
    expect(result.destinationRoot).toBe(projection.destinationRoot);
    expect(result.mappingDigest).toBe("b47425503a8ebc2a60120e91a5fd6a7e8a54411bdfc122b87a2ef7d5a7cf2d8f");
    expect(result.destinationDirectoryCount).toBe(266);
    expect(result.destinationNodeCount).toBe(495);
    expect(result.maxPathUtf16Units).toBe(189);
    expect(projection.modelCatalog?.models).toHaveLength(9);
    expect(projection.modelCatalog?.categoryRules.map(({ count }) => count).reduce((sum, count) => sum + count, 0)).toBe(220);
  });
  test("CAD plain live binding examples retain exact mapping and reject escaping carriers", () => {
    const projection = projectionGolden("artifact-example-model-catalog-v1");
    expect(CAD_LIVE_BINDINGS.bindings.map(({ mapping }) => mapping)).toEqual(projection.mappings.map((_, index) => index));
    expect(ARTIFACT_PROJECTION_GOLDEN.projections.every((entry) => !("liveBindings" in entry))).toBe(true);
    const tampered = { ...CAD_LIVE_BINDINGS, bindings: [...CAD_LIVE_BINDINGS.bindings.slice(1), { ...CAD_LIVE_BINDINGS.bindings[0]!, live: `../${CAD_LIVE_BINDINGS.bindings[0]!.live}` }] };
    expect(() => cadProjectionLiveSources(projection, undefined, tampered)).toThrow("outside its exact root");
  });
  test("CAD frozen scenario bindings read exact current bytes and reject missing, duplicate, or extra identities", () => {
    const projection = projectionGolden("artifact-example-model-catalog-v1");
    const paths = ownedFilePaths(join(getWorkspaceRoot(), projection.sourceRoot)).filter((path) => path.endsWith(".json"));
    const bindings = cadProjectionLiveSources(projection, paths), reads: string[] = [];
    const nodes = projectionAuthorityNodes(projection, "live", (path) => {
      reads.push(path);
      return readFileSync(join(getWorkspaceRoot(), path), "utf8");
    }).filter(({ nodeKind }) => nodeKind === "file");
    expect(nodes).toHaveLength(229);
    expect(paths).toHaveLength(229);
    expect(reads.sort(projectionByteSort)).toEqual(paths.map((path) => `${projection.sourceRoot}/${path}`).sort(projectionByteSort));
    for (const node of nodes) expect(node.content).toBe(readFileSync(join(getWorkspaceRoot(), bindings.get(node.path)!), "utf8"));
    expect(() => cadProjectionLiveSources(projection, paths.slice(1))).toThrow("Missing");
    expect(() => cadProjectionLiveSources(projection, [...paths, paths[0]!])).toThrow("Duplicate");
    expect(() => cadProjectionLiveSources(projection, [...paths, "🧪️unknown/🔣️modelDefinition.json"])).toThrow("unbound");
    expect(createHash("sha256").update(projection.mappings.map(({ sourcePath, destinationPath }) => `${sourcePath}\0${destinationPath}`).join("\n")).digest("hex")).toBe(projection.mappingDigest);
  });
  test("artifact-example-model-catalog-projection fails closed for invalid authority and path states", () => {
    const projection = projectionGolden("artifact-example-model-catalog-v1");
    const nodes = projectionAuthorityNodes(projection);
    const concreteManifest = nodes.find(({ path }) => path.endsWith("/🧱️aec.building.concrete/🔣️modelDefinition.json"))!;
    expect(projectionAuthority(projection, nodes.filter((node) => node !== concreteManifest)).problems.some((problem) => problem.toLocaleLowerCase("und").includes("model manifest"))).toBe(true);

    const action = nodes.find((node) => node.nodeKind === "file" && node.path.includes("/🎬️actions/"))!;
    const unknownSchema = nodes.map((node) => node === action ? { ...node, content: JSON.stringify({ id: "invalid", schema: "spatial.unknown", version: "1.0.0" }) } : node);
    expect(projectionAuthority(projection, unknownSchema).problems.some((problem) => problem.includes("manifest schema"))).toBe(true);

    const shapeManifest = nodes.find(({ path }) => path.endsWith("/📐️spatial.shape/🔣️modelDefinition.json"))!;
    const duplicateModel = nodes.map((node) => node === concreteManifest ? { ...node, content: shapeManifest.content } : node);
    expect(projectionAuthority(projection, duplicateModel).problems.some((problem) => problem.includes("duplicated"))).toBe(true);

    const unknownCategoryRoot = `${projection.sourceRoot}/📐️spatial.shape/🧪️unknown`;
    const unknownCategory: SemanticProjectionAuthorityNode[] = [...nodes, { path: unknownCategoryRoot, nodeKind: "directory" }, { path: `${unknownCategoryRoot}/🔣️member.json`, nodeKind: "file", content: JSON.stringify({ id: "unknown", schema: "spatial.unknown", version: "1.0.0" }) }];
    expect(projectionAuthority(projection, unknownCategory).problems.some((problem) => problem.includes("Unknown CAD catalog category"))).toBe(true);

    const symlink = [...nodes, { path: `${projection.sourceRoot}/🧪️symlink`, nodeKind: "symlink" as const }];
    expect(projectionAuthority(projection, symlink).problems.some((problem) => problem.includes("symlink"))).toBe(true);
    const vs15 = [...nodes, { path: `${projection.sourceRoot}/🧪︎vs15`, nodeKind: "symlink" as const }];
    expect(projectionAuthority(projection, vs15).problems.some((problem) => problem.includes("VS15"))).toBe(true);

    const reverse = semanticPathProjectionAuthority({ artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")), contractId: projection.contractId, sourceRoot: projection.destinationRoot, nodes });
    expect(reverse.problems.some((problem) => problem.includes("exact projection grammar"))).toBe(true);

    const modelRoot = `${projection.sourceRoot}/📐️spatial.shape`;
    const collisionStem = "CaseCollision";
    const collisionNodes: SemanticProjectionAuthorityNode[] = [
      ...nodes,
      { path: `${modelRoot}/🎬️actions/🔣️${collisionStem}.json`, nodeKind: "file", content: JSON.stringify({ id: "case-a", schema: "spatial.action", version: "1.0.0" }) },
      { path: `${modelRoot}/🎬️actions/🔣️${collisionStem.toLocaleLowerCase("und")}.json`, nodeKind: "file", content: JSON.stringify({ id: "case-b", schema: "spatial.action", version: "1.0.0" }) },
    ];
    expect(projectionAuthority(projection, collisionNodes).problems.some((problem) => problem.includes("case-fold"))).toBe(true);

    const longStem = "path".repeat(40);
    const longNodes: SemanticProjectionAuthorityNode[] = [...nodes, { path: `${modelRoot}/🎬️actions/🔣️${longStem}.json`, nodeKind: "file", content: JSON.stringify({ id: "long", schema: "spatial.action", version: "1.0.0" }) }];
    expect(projectionAuthority(projection, longNodes).problems.some((problem) => problem.includes("path-too-long"))).toBe(true);
    expect(projectionAuthority(projection, nodes, [projection.mappings[0]!.destinationPath]).problems.some((problem) => problem.includes("occupied"))).toBe(true);
  });
  test("artifact-editor-command-projection preserves the strict fixed-file union, the sealed scenario package bundle, and live command parity", () => {
    const projection = projectionGolden("artifact-editor-command-bundle-v1"), live = loadTaxonomy(), scenario = drawScenarioTaxonomy(live);
    const options = { artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")), contractId: projection.contractId, sourceRoot: projection.sourceRoot };
    const nodes = projectionAuthorityNodes(projection, "authored-draw", () => { throw new Error("Live Draw source must not be read by the strict fixed-file union"); });
    const result = projectionAuthority(projection, nodes);
    expect(validateTaxonomy(scenario)).toEqual([]);
    expect(result.problems).toEqual([]);
    expect(result.mappings).toEqual(projection.mappings);
    expect(result.mappingDigest).toBe(projection.mappingDigest);
    expect(result.destinationDirectoryCount).toBe(projection.destinationDirectoryCount);
    expect(result.destinationNodeCount).toBe(projection.destinationNodeCount);
    expect(result.maxPathUtf16Units).toBe(projection.maxPathUtf16Units);
    expect(result.mappings.filter(({ destinationPath }) => destinationPath.endsWith("/📦️packages/🦀️rust/Cargo.toml"))).toHaveLength(DRAW_SOURCE_SCENARIO.oracle.workspaceMemberCount);
    expect(semanticPathProjectionAuthority({ ...options, nodes }, live).problems.some((problem) => problem.includes("exact command bundle"))).toBe(true);

    const destinationRoot = join(getWorkspaceRoot(), projection.destinationRoot), liveFiles = ownedFilePaths(destinationRoot).map((path) => `${projection.destinationRoot}/${path}`).sort(projectionByteSort);
    const liveDirectories = new Set<string>([projection.destinationRoot]);
    for (const path of liveFiles) for (let parent = dirname(path); parent !== projection.destinationRoot; parent = dirname(parent)) liveDirectories.add(parent);
    const liveNodes: SemanticProjectionAuthorityNode[] = [...[...liveDirectories].map((path) => ({ path, nodeKind: "directory" as const })), ...liveFiles.map((path) => ({ path, nodeKind: "file" as const, content: readFileSync(join(getWorkspaceRoot(), path), "utf8") }))];
    const current = semanticPathProjectionAuthority({ ...options, nodes: liveNodes, layout: "destination" }, live);
    expect(current.problems).toEqual([]);
    expect(current.mappings.map(({ destinationPath }) => destinationPath).sort(projectionByteSort)).toEqual(liveFiles);
    expect(semanticPathProjectionAuthority({ ...options, nodes: liveNodes, layout: "destination" }, scenario).problems.some((problem) => problem.includes("exact command bundle"))).toBe(true);

    const partial = nodes.filter(({ path }) => !path.endsWith("/🔄️fsm/✨️macros/📦️packages/🦀️rust/📋️project.json"));
    expect(projectionAuthority(projection, partial).problems.some((problem) => problem.includes("exact command bundle"))).toBe(true);
    const extra = [...nodes, { path: `${projection.sourceRoot}/🔣️extra.json`, nodeKind: "file" as const, content: "{}" }];
    expect(projectionAuthority(projection, extra).problems.some((problem) => problem.includes("exact command bundle"))).toBe(true);

    const invalid = structuredClone(live) as Taxonomy;
    const contract = invalid.semanticDescendantContracts["draw-editor-command-bundle-v1"] as unknown as { requiredNodes: Record<string, unknown>[] };
    const sourceNode = contract.requiredNodes.find((node) => node.kindId === "rust-source" && !(node.pathSegments as unknown[]).length)!;
    sourceNode.kindId = "json";
    expect(validateTaxonomy(invalid).some((problem) => problem.includes("draw-editor-command-bundle-v1"))).toBe(true);
    const undeclared = structuredClone(scenario) as Taxonomy;
    const bundle = undeclared.semanticDescendantContracts["draw-editor-command-bundle-v1"] as unknown as { requiredNodes: Record<string, unknown>[]; realizedNodeCount: number };
    bundle.requiredNodes = bundle.requiredNodes.filter((node) => node.fixedFilenameContractId !== "root-script" || (node.pathSegments as unknown[]).length !== 3);
    bundle.realizedNodeCount -= 1;
    expect(validateTaxonomy(undeclared).some((problem) => problem.includes("declaration-only nested Rust packages"))).toBe(true);
  });
}
