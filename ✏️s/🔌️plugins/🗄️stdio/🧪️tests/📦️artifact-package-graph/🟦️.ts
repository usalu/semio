export function createStdioArtifactPackageTests(dependencies: Record<string, any>, source: { directory: string; url: string }) {
  const { assert, assertActualCargoDag, assertCargoMetadata, assertSchemaOracle, assertSourceContract, buildTypeScriptPackage, canonicalNames, CARGO_CONTRACT_NAME, cargoMetadata, checkTypeScriptPackageDeclaration, COMPOSITION_RUST_NAME, COMPOSITION_TYPESCRIPT_NAME, dirname, join, NX_CONTRACT_NAME, readJson, runCaptured, slash, stdioArtifactPackageContract } = dependencies;
  type JsonMap = any;
  type PackageRecord = any;
  /** 🔍️ Validates schema fixtures, declarations, workspaces and Cargo's own metadata projection. */
  async function testStdioArtifactPackageContract(repoRoot: string): Promise<void> {
    const contract = stdioArtifactPackageContract(repoRoot);
    await assertSchemaOracle(contract);
    await assertSourceContract(repoRoot, contract);
    await assertCargoMetadata(repoRoot, contract);
    console.log(`[stdio-package-contract] source packages=${contract.packages.length} dag=valid`);
  }
  
  /** 🕸️ Compares declared artifact dependencies with the Nx project graph. */
  async function testStdioArtifactPackageGraph(repoRoot: string): Promise<void> {
    const contract = stdioArtifactPackageContract(repoRoot);
    const metadata = await cargoMetadata(repoRoot);
    const cargoPackages = new Map((metadata.packages as JsonMap[]).map((entry) => [String(entry.name), entry]));
    assertActualCargoDag(metadata, new Set(contract.packages.map((entry) => entry.rust.cargoName)));
    const output = await runCaptured(process.execPath, ["run", "nx", "graph", "--print"], repoRoot, 180_000);
    const start = output.indexOf("{");
    assert(start >= 0, "Nx graph emitted no JSON");
    const graph = JSON.parse(output.slice(start)) as JsonMap;
    const dependencies = graph.graph?.dependencies ?? graph.dependencies;
    const nodes = graph.graph?.nodes ?? graph.nodes;
    for (const entry of contract.packages) {
      assert(nodes?.[entry.rust.nxName], `Nx graph misses ${entry.rust.nxName}`);
      assert(nodes?.[entry.typescript.nxName], `Nx graph misses ${entry.typescript.nxName}`);
      const rustEdges = new Set((dependencies?.[entry.rust.nxName] ?? []).map((edge: JsonMap) => edge.target));
      const typescriptEdges = new Set((dependencies?.[entry.typescript.nxName] ?? []).map((edge: JsonMap) => edge.target));
      assert(!rustEdges.has(COMPOSITION_RUST_NAME), `${entry.rust.nxName} has a composition back-edge`);
      assert(rustEdges.has(NX_CONTRACT_NAME), `${entry.rust.nxName} misses Nx edge ${NX_CONTRACT_NAME}`);
      const cargo = cargoPackages.get(entry.rust.cargoName);
      assert(cargo, `Cargo metadata misses ${entry.rust.cargoName}`);
      const expectedRustEdges = (cargo.dependencies as JsonMap[]).map((dependency) => String(dependency.name))
        .filter((name) => name.startsWith("semio-s-artifact-stdio-") && name !== CARGO_CONTRACT_NAME)
        .map((name) => canonicalNames(name.slice("semio-s-artifact-stdio-".length)).rustNx);
      for (const target of expectedRustEdges) assert(rustEdges.has(target), `${entry.rust.nxName} misses Nx edge ${target}`);
      const typescript = readJson(join(repoRoot, entry.typescript.manifest));
      for (const target of Object.keys(typescript.dependencies ?? {})) assert(typescriptEdges.has(target), `${entry.typescript.nxName} misses Nx edge ${target}`);
      const ownerRoot = slash(dirname(dirname(entry.source)));
      const otherOwnerRoots = contract.packages.filter((candidate) => candidate.identity !== entry.identity).map((candidate) => slash(dirname(dirname(candidate.source))));
      for (const node of [nodes[entry.rust.nxName], nodes[entry.typescript.nxName]]) {
        const defaultInputs = node.data?.namedInputs?.default ?? [];
        assert(defaultInputs.some((input: unknown) => typeof input === "string" && input.startsWith(`{workspaceRoot}/${ownerRoot}/`)), `${node.name} does not hash ${ownerRoot}`);
        for (const input of defaultInputs) if (typeof input === "string" && !input.startsWith("!")) for (const other of otherOwnerRoots) {
          if (input.startsWith(`{workspaceRoot}/${other}/`)) assert(!input.includes("*"), `${node.name} broadly hashes another artifact owner ${other}`);
        }
      }
    }
    assert(nodes?.[NX_CONTRACT_NAME], `Nx graph misses ${NX_CONTRACT_NAME}`);
    assert(nodes?.[COMPOSITION_TYPESCRIPT_NAME], `Nx graph misses ${COMPOSITION_TYPESCRIPT_NAME}`);
    console.log(`[stdio-package-contract] Nx graph rust=${contract.packages.length} typescript=${contract.packages.length}`);
  }
  
  async function testTypeScriptPackage(root: string, entry: PackageRecord): Promise<void> {
    await buildTypeScriptPackage(root, entry);
    checkTypeScriptPackageDeclaration(root, entry.typescript.name, "definition, type ArtifactDefinition");
    const module = await import(entry.typescript.name);
    assert.equal(module.definition?.id, entry.identity);
    assert.equal(module.definition?.artifact, entry.artifact);
    console.log(`[stdio-package] tested ${entry.typescript.name} identity=${entry.identity}`);
  }
  return { testStdioArtifactPackageContract, testStdioArtifactPackageGraph, testTypeScriptPackage };
}
