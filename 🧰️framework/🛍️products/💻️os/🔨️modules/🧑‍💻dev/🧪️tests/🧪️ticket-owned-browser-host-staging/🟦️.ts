type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { ACTIVATION_RECEIPT_FILE, ACTOR_COMPONENT_EXPORTS, DISTRIBUTION_LAYOUT, EXTENSION_WATCH_MARKER, EventEmitter, MODULE_EXTENSION_ROUTE, MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, PLAYWRIGHT_MODULE_SPECIFIER, PLUGIN_HOST_SHIM_FILE, PLUGIN_MODULES_ROOT, PLUGIN_SOURCE_WATCH_PATH, TEST_BROWSER_ACTIVATION_ROOT_ENV, TEST_BROWSER_HOST_RECEIPT_ENV, TEST_BROWSER_MODULE_ROOT_ENV, assertActorComponentExports, assertExtensionOutputsFresh, assertNoStalePublicPluginOutputs, assertPluginCatalogComplete, assertPluginOutputChildren, atTestLevel, awaitChildExit, awaitHttpOk, awaitTcpReady, backboneDbHandleFor, basename, buildEngineWasm, buildPluginCatalog, cargoProfileDir, catalogSmokeExitCode, catalogSmokeMarkdown, checkDistributionBundle, checkScaleFixtureArtifacts, closeTestBrowserHostStagingV1, compareOwnedParityPixels, cpSync, createConcurrencyLimiter, createHash, createReadStream, cropOwnedParityRgba, decodePackValue, decodeParityScreenshot, descriptorRouteDecision, dirname, distributionFileWitness, distributionPathOrder, distributionStaticSourcePaths, encodePackValue, encodeParityDiff, ensureParityPlaywrightBrowsersPath, exactSpaceCreateArtifactArgs, existsSync, fileURLToPath, finalizePluginDescriptor, hostShimSource, isAbsolute, join, linkedSessionEngines, mkdirSync, mkdtempSync, moduleIdForDirectoryName, moduleRoutePath, packValueToExactJson, parseDistributionManifest, parseDistributionStaticInputs, parseTestBrowserGisMaterializationReceiptV1, parseTestBrowserHostStagingReceiptV1, pathToFileURL, pluginCargoArgs, pluginComponentBridgeSource, pluginOutRoot, pluginWasmProfile, prepareTestBrowserHostRootsV1, publishDistributionBundle, readActivationReceipt, readFileSync, readdirSync, relative, renderScaleFixtureArtifacts, repoRoot, resolve, resolveTestBrowserHostRootsV1, rewriteJcoAsyncResultLifting, rewriteJcoComponentAssetUrls, rewritePreview2ShimImportSource, rmSync, scaleFixtureGeneratedDir, scanBuiltPluginModules, shardWorkerSource, stagePluginDescriptor, statSync, stateProbeCandidates, stateProbeChangedPaths, stateProbeSnapshot, summarizeCatalogSmoke, tmpdir, unlinkSync, watch, writeFileSync, writeTestBrowserGisMaterializationReceiptV1 } = dependencies;
  type OwnedParityImage = any;
  type PackValue = any;
  type ParityDump = any;
  type ParityNode = any;
  type PluginRegistryEntry = any;
  type PluginSourceEvent = any;
  type SpawnDaemonHandle = any;

  const { describe, expect, it, beforeEach, afterEach } = vitest;
  type ContractValidator = { (value: unknown): boolean; errors?: unknown };
  const contractModuleExport = async (modulePath: string, exportId: string): Promise<ContractValidator> => {
    const { default: AjvDraft7 } = await import("ajv");
    const document = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), modulePath), "utf8"));
    const ajv = new AjvDraft7({ strict: true, allErrors: true });
    ajv.addSchema(document);
    return ajv.getSchema(`${document.$id}#/$defs/${exportId}`) as ContractValidator;
  };
  const devContractModule = (exportId: string): Promise<ContractValidator> => contractModuleExport("../../🧬️schema/🔣️.json", exportId);
  const devDistributionContractModule = (exportId: string): Promise<ContractValidator> => contractModuleExport("../../🚚️distribution/🧬️schema/🔣️.json", exportId);
  /** ⏱️Cases that drive a real Rollup/Vite build, a strict-TypeScript program, a detached `node`/`bun`
   * child, or a Wasm instantiation — each measured well past a second on 2026-09-05, together far past the
   * 30 s `quick` wall-clock budget. They stay in the suite and run from `test long` upwards; `test quick`
   * keeps every pure-helper case. */
  const itLong = atTestLevel(it, "long");

  describe("ticket-owned browser host staging", () => {
    it("matches the neutral schema and closes only the exact Space, GIS and support module set", async () => {
      const contractRoot = join(dirname(fileURLToPath(source.url)), "../../♻️activation/🌐️browser-host");
      const document = JSON.parse(readFileSync(join(contractRoot, "🧬️schema/🔣️.json"), "utf8"));
      const fixture = JSON.parse(readFileSync(join(contractRoot, "🔣️.json"), "utf8"));
      const { default: AjvDraft7 } = await import("ajv");
      const stagingAjv = new AjvDraft7({ strict: true, allErrors: true });
      stagingAjv.addSchema(document);
      const validate = stagingAjv.getSchema(`${document.$id}#/$defs/TestBrowserHostStagingV1`)!;
      const validateMaterialization = stagingAjv.getSchema(`${document.$id}#/$defs/TestBrowserGisMaterializationV1`)!;
      const validateProvenanceFixture = stagingAjv.getSchema(`${document.$id}#/$defs/TestBrowserGisProvenanceFixtureV1`)!;
      const provenanceFixture = JSON.parse(readFileSync(join(contractRoot, "🧪️fixtures/🧬️selected-gis-byte-provenance-v1/🔣️.json"), "utf8"));
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      expect(validateProvenanceFixture(provenanceFixture), JSON.stringify(validateProvenanceFixture.errors)).toBe(true);
      expect(parseTestBrowserHostStagingReceiptV1(fixture)).toEqual(fixture);
      for (const specimen of provenanceFixture.specimens) {
        const bytes = new TextEncoder().encode(specimen.utf8);
        expect(createHash("sha256").update(bytes).digest("hex")).toBe(specimen.sha256);
        expect(Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex")).toBe(specimen.sha256);
      }
      expect(exactSpaceCreateArtifactArgs({ dialogs: [{ id: "createArtifact", args: [{ id: "name" }, { id: "kindChoice" }] }] })).toBe(true);
      expect(exactSpaceCreateArtifactArgs({ dialogs: [{ id: "createArtifact", args: [{ id: "name" }, { id: "kindId" }] }] })).toBe(false);
      const scriptSource = readFileSync(fileURLToPath(source.url), "utf8");
      expect(scriptSource).toContain('buildPluginCargo(space, join(artifactRoot, "browser-host-wasi-target"))');
      expect(scriptSource).toContain('CARGO_TARGET_DIR: cargoTargetRoot, CARGO_BUILD_JOBS: "1", CARGO_INCREMENTAL: "0", RUSTC_WRAPPER: "", RUSTC_WORKSPACE_WRAPPER: ""');
      const materializerStart = scriptSource.indexOf("async function materializeTestBrowserPluginV1(");
      const materializerEnd = scriptSource.indexOf("export async function stageTestBrowserHostV1", materializerStart);
      const materializerOwner = scriptSource.slice(materializerStart, materializerEnd);
      expect(materializerOwner.indexOf("writeFileSync(join(outDir, MODULE_BRIDGE_FILE)")).toBeLessThan(materializerOwner.indexOf("writeTestBrowserGisMaterializationReceiptV1(outDir, input.selectedSource)"));
      expect(scriptSource).toContain("selectedSource: { componentSha256: input.selectedGis.componentSha256, descriptorSha256: input.selectedGis.descriptorSha256 }");
      expect(scriptSource.indexOf("await materializeTestBrowserPluginV1({ target: gis")).toBeLessThan(scriptSource.indexOf("Selected GIS bytes changed during browser staging"));

      const owner = mkdtempSync(join(tmpdir(), "semio-browser-host-law-"));
      const artifactRoot = join(owner, "🗑️generated");
      mkdirSync(artifactRoot, { recursive: true });
      try {
        const roots = prepareTestBrowserHostRootsV1(artifactRoot);
        const hostComponent = Buffer.from("space-cargo-component");
        const hostComponentSha256 = createHash("sha256").update(hostComponent).digest("hex");
        const hostCoreSha256 = createHash("sha256").update("space-component").digest("hex");
        const specimens = Object.fromEntries(provenanceFixture.specimens.map((row: any) => [row.id, row]));
        const files = new Map([
          ["🪞️vendor/.nx-artifact.json", "vendor"],
          ["🧵️shard/🟨️shard-worker.js", "shard"],
          ["🪐️space/semio_s_plugin_space_component.core.wasm", "space-component"],
          ["🪐️space/🛂️.descriptor.semio", "space-descriptor"],
          ["🪐️space/🔣️.json", JSON.stringify({ hashes: { wasmSha256: hostComponentSha256, coreWasmSha256: hostCoreSha256 } })],
          ["🪐️space/🌉️bridge.js", "space-bridge"],
          ["🌍️gis/semio_s_plugin_gis_component.core.wasm", specimens["staged-core"].utf8],
          ["🌍️gis/🛂️.descriptor.semio", specimens["selected-descriptor"].utf8],
          ["🌍️gis/🌉️bridge.js", specimens["staged-bridge"].utf8],
        ]);
        for (const [name, body] of files) {
          const path = join(roots.moduleRoot, name);
          mkdirSync(dirname(path), { recursive: true });
          writeFileSync(path, body);
        }
        const materialization = writeTestBrowserGisMaterializationReceiptV1(join(roots.moduleRoot, "🌍️gis"), { componentSha256: specimens["selected-component"].sha256, descriptorSha256: specimens["selected-descriptor"].sha256 });
        expect(validateMaterialization(materialization), JSON.stringify(validateMaterialization.errors)).toBe(true);
        expect(parseTestBrowserGisMaterializationReceiptV1(materialization)).toEqual(materialization);
        const current = { generationId: "a".repeat(64), currentSha256: "b".repeat(64), componentSha256: specimens["selected-component"].sha256, descriptorSha256: specimens["selected-descriptor"].sha256 };
        expect(() => closeTestBrowserHostStagingV1(artifactRoot, current, { byteLength: hostComponent.byteLength, sha256: "c".repeat(64) })).toThrow("descriptor component identity differs");
        const closed = closeTestBrowserHostStagingV1(artifactRoot, current, { byteLength: hostComponent.byteLength, sha256: hostComponentSha256 });
        expect(validate(closed.receipt), JSON.stringify(validate.errors)).toBe(true);
        expect(closed.receipt.selectedGis).toMatchObject(current);
        expect(closed.receipt.selectedGis).toMatchObject({ stagedDescriptorSha256: specimens["selected-descriptor"].sha256, bridgeSha256: specimens["staged-bridge"].sha256, generatedCores: [{ relativePath: "semio_s_plugin_gis_component.core.wasm", byteLength: specimens["staged-core"].utf8.length, sha256: specimens["staged-core"].sha256 }] });
        expect(closed.receipt.host).toMatchObject({ pluginId: "space", componentByteLength: hostComponent.byteLength, componentSha256: hostComponentSha256, coreSha256: hostCoreSha256 });
        expect(readActivationReceipt(closed.activationRoot).plugins.map((row) => row.pluginId)).toEqual(["gis", "space"]);
        const environment = {
          SEMIO_TEST_ARTIFACT_DIR: artifactRoot,
          [TEST_BROWSER_MODULE_ROOT_ENV]: closed.moduleRoot,
          [TEST_BROWSER_ACTIVATION_ROOT_ENV]: closed.activationRoot,
          [TEST_BROWSER_HOST_RECEIPT_ENV]: closed.receiptPath,
        };
        expect(resolveTestBrowserHostRootsV1(environment)?.receipt).toEqual(closed.receipt);
        expect(resolveTestBrowserHostRootsV1({})).toBeNull();
        expect(() => resolveTestBrowserHostRootsV1({ SEMIO_TEST_ARTIFACT_DIR: artifactRoot, [TEST_BROWSER_MODULE_ROOT_ENV]: closed.moduleRoot })).toThrow("complete ticket authority");

        const descriptor = join(closed.moduleRoot, "🪐️space/🛂️.descriptor.semio");
        writeFileSync(descriptor, "substituted-space-descriptor");
        expect(() => resolveTestBrowserHostRootsV1(environment)).toThrow("module set changed");
        writeFileSync(descriptor, "space-descriptor");
        expect(resolveTestBrowserHostRootsV1(environment)?.receipt).toEqual(closed.receipt);

        const bridge = join(closed.moduleRoot, "🪐️space/🌉️bridge.js");
        writeFileSync(bridge, "substituted-space-module");
        expect(() => resolveTestBrowserHostRootsV1(environment)).toThrow("module set changed");
        writeFileSync(bridge, "space-bridge");
        expect(resolveTestBrowserHostRootsV1(environment)?.receipt).toEqual(closed.receipt);

        const receiptBytes = readFileSync(closed.receiptPath);
        const moduleSetSha256 = (): string => {
          const paths = [...new Bun.Glob("**/*").scanSync({ cwd: closed.moduleRoot, onlyFiles: true, dot: true })].sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
          const hash = createHash("sha256");
          for (const path of paths) {
            const bytes = readFileSync(join(closed.moduleRoot, path));
            hash.update(`${JSON.stringify([path.replaceAll("\\", "/"), bytes.byteLength, createHash("sha256").update(bytes).digest("hex")])}\n`);
          }
          return hash.digest("hex");
        };
        for (const hostile of provenanceFixture.hostiles) {
          if (hostile.target === "host-receipt") {
            const changed: any = JSON.parse(JSON.stringify(closed.receipt));
            changed.selectedGis.componentSha256 = hostile.replacement;
            writeFileSync(closed.receiptPath, `${JSON.stringify(changed)}\n`);
          } else {
            const target = join(closed.moduleRoot, hostile.target === "staged-descriptor" ? "🌍️gis/🛂️.descriptor.semio" : "🌍️gis/🌉️bridge.js");
            const original = readFileSync(target);
            writeFileSync(target, hostile.replacement);
            const changed: any = JSON.parse(JSON.stringify(closed.receipt));
            changed.moduleSetSha256 = moduleSetSha256();
            writeFileSync(closed.receiptPath, `${JSON.stringify(changed)}\n`);
            expect(() => resolveTestBrowserHostRootsV1(environment), hostile.id).toThrow("GIS materialization differs");
            writeFileSync(target, original);
            writeFileSync(closed.receiptPath, receiptBytes);
            continue;
          }
          expect(() => resolveTestBrowserHostRootsV1(environment), hostile.id).toThrow("GIS identity changed");
          writeFileSync(closed.receiptPath, receiptBytes);
        }

        const activationPath = join(closed.activationRoot, ACTIVATION_RECEIPT_FILE);
        const activation = readFileSync(activationPath);
        writeFileSync(activationPath, `${activation.toString("utf8").trim()} `);
        expect(() => resolveTestBrowserHostRootsV1(environment)).toThrow("activation changed");
        writeFileSync(activationPath, activation);
        expect(resolveTestBrowserHostRootsV1(environment)?.receipt).toEqual(closed.receipt);
      } finally {
        rmSync(owner, { recursive: true, force: true });
      }
    });
  });

  //#region 🌉️LinkedSessionEnginesTests
  describe("scaleFixtureArtifacts", () => {
    itLong("compares both committed semantic outputs without rewriting them", async () => {
      const { execFileSync } = await import("node:child_process");
      const directory = scaleFixtureGeneratedDir(repoRoot);
      const registryJson = readFileSync(join(directory, "📇️registry/🔣️.json"), "utf8");
      const catalogJson = readFileSync(join(directory, "🗂️catalog/🔣️.json"), "utf8");
      const registry = JSON.parse(registryJson);
      const rendered = renderScaleFixtureArtifacts(registry.pluginCount, registry.extensionsPerPlugin, registry.seed);
      expect(rendered.registryJson).toBe(registryJson);
      expect(rendered.catalogJson).toBe(catalogJson);
      expect(checkScaleFixtureArtifacts(repoRoot)).toBe(true);
      const oracle = JSON.parse(execFileSync("node", ["--input-type=module", "--eval", 'import { readFileSync } from "node:fs"; const { registry, catalog } = JSON.parse(readFileSync(0, "utf8")); console.log(JSON.stringify({ count: registry.records.length, plugins: registry.records.filter(row => row.kind === "plugin").map(row => row.id), catalogPlugins: catalog.plugins.map(row => row.pluginId) }));'], { input: JSON.stringify({ registry, catalog: JSON.parse(catalogJson) }), encoding: "utf8" }));
      expect(oracle.count).toBe(2550);
      expect(oracle.plugins).toEqual(oracle.catalogPlugins);
      expect(oracle.plugins).toHaveLength(50);
    });
  });

  describe("linkedSessionEngines", () => {
    itLong("matches strict schema validation and deduplicates only exact owner engine paths", async () => {
      const { default: Ajv } = await import("ajv");
      const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "../../🧫️fixtures/🔗️linked-session-engines.json"), "utf8"));
      const validate = await devContractModule("LinkedSessionEnginesV1");
      for (const vector of fixture.valid) {
        expect(validate(vector.declarations)).toBe(true);
        expect(linkedSessionEngines(vector.declarations)).toEqual(vector.engines);
      }
      for (const declarations of fixture.invalid) {
        expect(validate(declarations)).toBe(false);
        expect(() => linkedSessionEngines(declarations)).toThrow();
      }
    });

    itLong("uses actual linked factory module and owner crate declarations for every product composition", async () => {
      const { default: Ajv } = await import("ajv");
      const ts = await import("typescript");
      const validate = await devContractModule("LinkedSessionEnginesV1");
      const devRoot = dirname(fileURLToPath(source.url));
      const compositions = [
        { manifest: join(devRoot, "package.json"), entries: [join(devRoot, "../../🟦️.ts"), join(devRoot, "../../🧪️tests/🧪️multi-shell-harness/🟦️.tsx")] },
        { manifest: join(repoRoot, "♻️mit-bestand/🧺️demonstrator/package.json"), entries: [join(repoRoot, "♻️mit-bestand/🧺️demonstrator/🟦️.tsx")] },
      ];
      for (const composition of compositions) {
        const manifest = JSON.parse(readFileSync(composition.manifest, "utf8"));
        const declarations = manifest.semio.browserSessionFactories;
        expect(validate(declarations)).toBe(true);
        expect(declarations.length).toBeGreaterThan(0);
        expect(linkedSessionEngines(declarations).length).toBeGreaterThan(0);
        for (const engine of linkedSessionEngines(declarations)) expect(existsSync(join(repoRoot, engine, "📜️script.ts"))).toBe(true);
        for (const entry of composition.entries) {
          const ast = ts.createSourceFile(entry, readFileSync(entry, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
          const imports = ast.statements.flatMap((node) => ts.isImportDeclaration(node) && !node.importClause?.isTypeOnly && ts.isStringLiteral(node.moduleSpecifier) ? [node.moduleSpecifier.text] : []);
          for (const declaration of declarations) expect(imports).toContain(declaration.module);
        }
      }
    });
  });
  //#endregion 🌉️LinkedSessionEnginesTests

  //#region 🧱️EnginePublicationTests
  describe("buildEngineWasm", () => {
    const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "../../🧫️fixtures/📢️engine-publication.json"), "utf8"));
    it("revalidates every required engine on each build even when outputs already exist", async () => {
      for (let run = 0; run < fixture.repetitions; run++) {
        const calls: string[] = [];
        await buildEngineWasm(fixture.variant, "react", join(repoRoot, fixture.composition), (script) => { calls.push(script); return 0; }, () => false);
        expect(calls).toEqual(fixture.engines.map((engine: string) => join(repoRoot, engine, "📜️script.ts")));
      }
    });
    it("fails publication when the required Flow engine fails", async () => {
      const flow = join(repoRoot, fixture.engines[2], "📜️script.ts");
      await expect(buildEngineWasm(fixture.variant, "react", join(repoRoot, fixture.composition), (script) => script === flow ? 1 : 0, () => false)).rejects.toThrow("flow-core wasm build failed");
    });
  });
  //#endregion 🧱️EnginePublicationTests

  //#region 🔖️PixelCompare-tests
  describe("compareOwnedParityPixels", () => {
    const options = { ignoreAntialiasing: true, threshold: 0.1 } as const;
    const edge = (middle: number): Uint8Array => {
      const pixels = new Uint8Array(3 * 3 * 4);
      for (let y = 0; y < 3; y++) {
        for (let x = 0; x < 3; x++) {
          const value = x === 0 ? 0 : x === 1 ? middle : 255;
          const offset = (y * 3 + x) * 4;
          pixels[offset] = value;
          pixels[offset + 1] = value;
          pixels[offset + 2] = value;
          pixels[offset + 3] = 255;
        }
      }
      return pixels;
    };

    it("writes a muted row-major identity diff", () => {
      const reference = Uint8Array.from([0, 0, 0, 255, 64, 64, 64, 255, 255, 255, 255, 255]);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, reference.slice(), diff, 3, 1, options)).toBe(0);
      expect([...diff]).toEqual([191, 191, 191, 255, 207, 207, 207, 255, 255, 255, 255, 255]);
    });

    it("keeps a grayscale delta of 25 below threshold and counts 26 above it", () => {
      const reference = Uint8Array.from([0, 0, 0, 255, 0, 0, 0, 255]);
      const candidate = Uint8Array.from([25, 25, 25, 255, 26, 26, 26, 255]);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, candidate, diff, 2, 1, options)).toBe(1);
      expect([...diff]).toEqual([191, 191, 191, 255, 255, 32, 64, 255]);
    });

    it("composites alpha over white and ignores invisible RGB", () => {
      const reference = Uint8Array.from([255, 0, 0, 0, 0, 0, 0, 128]);
      const candidate = Uint8Array.from([0, 255, 0, 0, 0, 0, 0, 0]);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, candidate, diff, 2, 1, options)).toBe(1);
      expect([...diff]).toEqual([255, 255, 255, 255, 255, 32, 64, 255]);
    });

    it("marks shared-edge coverage differences as antialiasing", () => {
      const reference = edge(96);
      const candidate = edge(128);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, candidate, diff, 3, 3, options)).toBe(2);
      expect([...diff.slice(16, 20)]).toEqual([255, 192, 0, 255]);
      expect(compareOwnedParityPixels(reference, candidate, new Uint8Array(reference.length), 3, 3, { ...options, ignoreAntialiasing: false })).toBe(3);
    });

    it("counts a real high-contrast edge displacement", () => {
      const reference = edge(0);
      const candidate = edge(255);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, candidate, diff, 3, 3, options)).toBe(3);
      expect([...diff.slice(16, 20)]).toEqual([255, 32, 64, 255]);
    });

    it("rejects writable typed-array overlap without corrupting either read input", () => {
      const reference = edge(96);
      const candidate = edge(128);
      const byteLength = reference.byteLength;
      const exactAliasBefore = reference.slice();
      expect(() => compareOwnedParityPixels(reference, candidate, reference, 3, 3, options)).toThrow("diff buffer must not overlap");
      expect(reference).toEqual(exactAliasBefore);

      const forwardStorage = new Uint8Array(byteLength + 4);
      const forwardReference = forwardStorage.subarray(0, byteLength);
      const forwardDiff = forwardStorage.subarray(4, byteLength + 4);
      forwardReference.set(reference);
      const forwardBefore = forwardStorage.slice();
      expect(() => compareOwnedParityPixels(forwardReference, candidate, forwardDiff, 3, 3, options)).toThrow("diff buffer must not overlap");
      expect(forwardStorage).toEqual(forwardBefore);

      const backwardStorage = new Uint8Array(byteLength + 4);
      const backwardDiff = backwardStorage.subarray(0, byteLength);
      const backwardCandidate = backwardStorage.subarray(4, byteLength + 4);
      backwardCandidate.set(candidate);
      const backwardBefore = backwardStorage.slice();
      expect(() => compareOwnedParityPixels(reference, backwardCandidate, backwardDiff, 3, 3, options)).toThrow("diff buffer must not overlap");
      expect(backwardStorage).toEqual(backwardBefore);

      const disjointStorage = new Uint8Array(byteLength * 2);
      const disjointReference = disjointStorage.subarray(0, byteLength);
      const disjointDiff = disjointStorage.subarray(byteLength);
      disjointReference.set(reference);
      expect(compareOwnedParityPixels(disjointReference, candidate, disjointDiff, 3, 3, options)).toBe(2);
      expect([...disjointDiff.slice(16, 20)]).toEqual([255, 192, 0, 255]);

      const readOnlyAliasDiff = new Uint8Array(byteLength);
      expect(compareOwnedParityPixels(reference, reference, readOnlyAliasDiff, 3, 3, options)).toBe(0);
      const empty = new Uint8Array(0);
      expect(compareOwnedParityPixels(empty, empty, empty, 0, 3, options)).toBe(0);

      const retainedDiff = new Uint8Array(byteLength);
      expect(compareOwnedParityPixels(reference, candidate, retainedDiff, 3, 3, options)).toBe(2);
      expect([...retainedDiff.slice(16, 20)]).toEqual([255, 192, 0, 255]);
    });

    it("rejects malformed lengths, dimensions, and thresholds exactly", () => {
      const pixel = new Uint8Array(4);
      expect(() => compareOwnedParityPixels(new Uint8Array(3), pixel, pixel, 1, 1, options)).toThrow("exactly 4 RGBA bytes");
      expect(() => compareOwnedParityPixels(pixel, new Uint8Array(5), pixel, 1, 1, options)).toThrow("exactly 4 RGBA bytes");
      expect(() => compareOwnedParityPixels(pixel, pixel, new Uint8Array(0), 1, 1, options)).toThrow("exactly 4 RGBA bytes");
      expect(() => compareOwnedParityPixels(pixel, pixel, pixel, -1, 1, options)).toThrow("non-negative safe integers");
      expect(() => compareOwnedParityPixels(pixel, pixel, pixel, Number.MAX_SAFE_INTEGER, 2, options)).toThrow("safe byte range");
      expect(() => compareOwnedParityPixels(pixel, pixel, pixel, 1, 1, { ...options, threshold: Number.NaN })).toThrow("finite and between zero and one");
    });

    it("compares a large identical crop within a bounded tooling smoke budget", () => {
      const width = 640;
      const height = 360;
      const reference = new Uint8Array(width * height * 4);
      const candidate = new Uint8Array(reference);
      const diff = new Uint8Array(reference.length);
      const started = performance.now();
      expect(compareOwnedParityPixels(reference, candidate, diff, width, height, options)).toBe(0);
      expect(performance.now() - started).toBeLessThan(2_000);
      expect(diff[0]).toBe(255);
      expect(diff[diff.length - 1]).toBe(255);
    });

    it("retains representative opaque, transparent, text-edge, and scene-gradient counts", () => {
      const opaqueReference = Uint8Array.from([0, 0, 0, 255, 40, 40, 40, 255, 120, 90, 60, 255, 255, 255, 255, 255]);
      const opaqueCandidate = Uint8Array.from([0, 0, 0, 255, 60, 60, 60, 255, 170, 90, 60, 255, 230, 230, 230, 255]);
      const transparentReference = Uint8Array.from([255, 0, 0, 0, 0, 0, 0, 128, 20, 80, 160, 255]);
      const transparentCandidate = Uint8Array.from([0, 255, 0, 0, 0, 0, 0, 0, 20, 80, 160, 192]);
      const edgeReference = edge(96);
      const edgeCandidate = edge(128);
      const gradientReference = new Uint8Array(16 * 4 * 4);
      const gradientCandidate = new Uint8Array(gradientReference.length);
      for (let index = 0; index < 64; index++) {
        const referenceValue = Math.round((index % 16) * 17);
        const candidateValue = Math.min(255, referenceValue + 12);
        const offset = index * 4;
        gradientReference.set([referenceValue, Math.max(0, referenceValue - 20), 255 - referenceValue, 255], offset);
        gradientCandidate.set([candidateValue, Math.max(0, candidateValue - 20), 255 - candidateValue, 255], offset);
      }
      const fixtures = [
        { antialiasMarkers: 0, candidate: opaqueCandidate, height: 1, mismatches: 1, name: "opaque", reference: opaqueReference, width: 4 },
        { antialiasMarkers: 0, candidate: transparentCandidate, height: 1, mismatches: 1, name: "transparent", reference: transparentReference, width: 3 },
        { antialiasMarkers: 1, candidate: edgeCandidate, height: 3, mismatches: 2, name: "text-edge", reference: edgeReference, width: 3 },
        { antialiasMarkers: 0, candidate: gradientCandidate, height: 4, mismatches: 0, name: "scene-gradient", reference: gradientReference, width: 16 },
      ];
      for (const fixture of fixtures) {
        const ownedDiff = new Uint8Array(fixture.reference.length);
        const ownedCount = compareOwnedParityPixels(fixture.reference, fixture.candidate, ownedDiff, fixture.width, fixture.height, options);
        let ownedAntialiasMarkers = 0;
        let ownedMismatchMarkers = 0;
        for (let offset = 0; offset < ownedDiff.length; offset += 4) {
          if (ownedDiff[offset] === 255 && ownedDiff[offset + 1] === 192 && ownedDiff[offset + 2] === 0) ownedAntialiasMarkers += 1;
          if (ownedDiff[offset] === 255 && ownedDiff[offset + 1] === 32 && ownedDiff[offset + 2] === 64) ownedMismatchMarkers += 1;
        }
        expect({ fixture: fixture.name, ownedAntialiasMarkers, ownedCount, ownedMismatchMarkers }).toEqual({
          fixture: fixture.name,
          ownedAntialiasMarkers: fixture.antialiasMarkers,
          ownedCount: fixture.mismatches,
          ownedMismatchMarkers: fixture.mismatches,
        });
      }
    });

    it("crops complete RGBA rows with strict bounds and independent storage", () => {
      const image: OwnedParityImage = { width: 4, height: 3, data: Uint8Array.from({ length: 48 }, (_, index) => index) };
      const before = image.data.slice();
      const crop = cropOwnedParityRgba(image, 1, 1, 2, 2);
      expect([...crop]).toEqual([...image.data.slice(20, 28), ...image.data.slice(36, 44)]);
      crop.fill(255);
      expect(image.data).toEqual(before);
      expect([...cropOwnedParityRgba(image, 4, 3, 0, 0)]).toEqual([]);
      expect(() => cropOwnedParityRgba(image, -1, 0, 1, 1)).toThrow("non-negative safe integers");
      expect(() => cropOwnedParityRgba(image, 3, 2, 2, 1)).toThrow("within image bounds");
      expect(() => cropOwnedParityRgba({ ...image, data: image.data.subarray(0, 47) }, 0, 0, 1, 1)).toThrow("exactly 48 RGBA bytes");
    });

    itLong("preserves fixed CSS color, alpha, and diagnostic marker pixels through Canvas PNGs", async () => {
      ensureParityPlaywrightBrowsersPath();
      const { chromium } = await import(PLAYWRIGHT_MODULE_SPECIFIER);
      const browser = await chromium.launch({ headless: true });
      try {
        const page = await browser.newPage({ viewport: { width: 4, height: 3 } });
        await page.setContent(
          '<style>*{margin:0}body{background:transparent}#opaque{position:absolute;width:1px;height:1px;background:rgb(12 34 56)}#alpha{position:absolute;left:1px;width:1px;height:1px;background:rgb(20 40 60 / 50%)}</style><div id="opaque"></div><div id="alpha"></div>',
        );
        const screenshot = await page.screenshot({ omitBackground: true });
        const decoded = await decodeParityScreenshot(page, screenshot);
        expect({ width: decoded.width, height: decoded.height, firstRow: [...decoded.data.slice(0, 16)] }).toEqual({
          width: 4,
          height: 3,
          firstRow: [12, 34, 56, 255, 20, 40, 60, 128, 0, 0, 0, 0, 0, 0, 0, 0],
        });
        expect([...cropOwnedParityRgba(decoded, 0, 0, 2, 1)]).toEqual([12, 34, 56, 255, 20, 40, 60, 128]);
        const diagnostic = Uint8Array.from([12, 34, 56, 255, 20, 40, 60, 128, 255, 32, 64, 255, 255, 192, 0, 255]);
        const roundTrip = await decodeParityScreenshot(page, await encodeParityDiff(page, diagnostic, 2, 2));
        expect({ width: roundTrip.width, height: roundTrip.height, data: [...roundTrip.data] }).toEqual({ width: 2, height: 2, data: [...diagnostic] });
      } finally {
        await browser.close();
      }
    }, 30_000);
  });
  //#endregion 🔖️PixelCompare-tests

  describe("descriptorRouteDecision", () => {
    it("admits an exact declared descriptor and rejects missing or undeclared descriptor paths before SPA fallback", () => {
      const root = mkdtempSync(join(tmpdir(), "semio-descriptor-route-"));
      try {
        mkdirSync(join(root, "🗒️note"), { recursive: true });
        writeFileSync(join(root, "🗒️note", "🔣️.json"), '{"manifest":{"pluginId":"note"}}\n');
        const specs = [{ route: "/🔌️plugin-modules", root, directoryNames: new Set(["🗒️note", "🪐️space"]) }];
        expect(descriptorRouteDecision("/🔌️plugin-modules/🗒️note/🔣️.json", specs)).toEqual({ kind: "pass" });
        expect(descriptorRouteDecision("/🔌️plugin-modules/🪐️space/🔣️.json?epoch=1", specs)).toEqual({ kind: "missing", moduleDirectory: "🪐️space" });
        expect(descriptorRouteDecision("/🔌️plugin-modules/unknown/🔣️.json", specs)).toEqual({ kind: "missing", moduleDirectory: "unknown" });
        expect(descriptorRouteDecision("/other/🪐️space/🔣️.json", specs)).toEqual({ kind: "pass" });
      } finally {
        rmSync(root, { recursive: true, force: true });
      }
    });
  });

  describe("scanBuiltPluginModules (plugin hot-swap SSE snapshot)", () => {
    let root: string;

    beforeEach(() => {
      root = mkdtempSync(join(tmpdir(), "semio-plugin-hot-swap-"));
    });

    afterEach(() => {
      rmSync(root, { recursive: true, force: true });
    });

    it("returns nothing for a missing root", () => {
      expect(scanBuiltPluginModules(join(root, "does-not-exist"))).toEqual([]);
    });

    it("skips a plugin dir with no core wasm output yet", () => {
      mkdirSync(join(root, "🗒️note"), { recursive: true });
      writeFileSync(join(root, "🗒️note", "🟨️.js"), "");
      expect(scanBuiltPluginModules(root)).toEqual([]);
    });

    it("skips the shared 🪞️vendor dir", () => {
      mkdirSync(join(root, "🪞️vendor"), { recursive: true });
      writeFileSync(join(root, "🪞️vendor", "shim.core.wasm"), "");
      expect(scanBuiltPluginModules(root)).toEqual([]);
    });

    it("reports a built plugin's newest core wasm mtime", () => {
      mkdirSync(join(root, "🗒️note"), { recursive: true });
      writeFileSync(join(root, "🗒️note", "note_plugin_component.core.wasm"), "");
      const rows = scanBuiltPluginModules(root);
      expect(rows).toHaveLength(1);
      expect(rows[0]!.pluginId).toBe("note");
      expect(rows[0]!.rebuiltAt).toBeGreaterThan(0);
    });

    it("reports one row per plugin dir, largest mtime among multiple core wasm chunks", () => {
      mkdirSync(join(root, "🗒️note"), { recursive: true });
      writeFileSync(join(root, "🗒️note", "note_plugin_component.core.wasm"), "");
      writeFileSync(join(root, "🗒️note", "note_plugin_component.core2.wasm"), "");
      mkdirSync(join(root, "🪐️space"), { recursive: true });
      writeFileSync(join(root, "🪐️space", "semio_s_plugin_space_component.core.wasm"), "");
      const rows = scanBuiltPluginModules(root);
      expect(rows.map((row) => row.pluginId).sort()).toEqual(["note", "space"]);
    });

    it("does not infer a public identity from undeclared raw or decorated basenames", () => {
      for (const directoryName of ["note", "🗒️other", "🧩️unknown"]) {
        mkdirSync(join(root, directoryName), { recursive: true });
        writeFileSync(join(root, directoryName, "module.core.wasm"), "");
      }
      expect(scanBuiltPluginModules(root)).toEqual([]);
    });
  });

  describe("PluginHotSwapMarker JSON round-trip (SSE `built` event payload)", () => {
    it("parses the exact shape buildPlugin writes to ♻️hot-swap.json", () => {
      const marker = JSON.parse(`${JSON.stringify({ pluginId: "note", rebuiltAt: 1785789943669 })}\n`) as PluginHotSwapMarker;
      expect(marker).toEqual({ pluginId: "note", rebuiltAt: 1785789943669 });
      const event: PluginSourceEvent = { kind: "built", pluginId: marker.pluginId, rebuiltAt: marker.rebuiltAt };
      expect(event).toEqual({ kind: "built", pluginId: "note", rebuiltAt: 1785789943669 });
    });
  });

  describe("catalog state transition probe", () => {
    const node = (path: string, kind: string, selected = false): ParityNode => ({
      path,
      kind,
      rect: [0, 0, 20, 20],
      text: null,
      color: null,
      bg: null,
      fontSize: null,
      fontWeight: null,
      visible: true,
      state: { hovered: false, disabled: false, selected },
    });
    const dump = (nodes: readonly ParityNode[]): ParityDump => ({ viewport: { w: 100, h: 100, dpr: 1 }, focusPath: null, nodes });

    it("selects only common explicitly-id'd app controls in semantic priority order", () => {
      const react = dump([node("button[0]#run", "button"), node("toggle[1]#enabled", "toggle"), node("stack[2]", "stack")]);
      const wgpu = dump([node("button[0]#run", "button"), node("toggle[1]#enabled", "toggle"), node("select[3]#mode", "select")]);
      expect(stateProbeCandidates(react, wgpu)).toEqual([
        { path: "toggle[1]#enabled", kind: "toggle" },
        { path: "button[0]#run", kind: "button" },
      ]);
    });

    it("records selected-state and topology changes but ignores focus-only movement", () => {
      const before = dump([node("toggle[0]#enabled", "toggle")]);
      const after = { ...dump([node("toggle[0]#enabled", "toggle", true), node("text[1]#status", "text")]), focusPath: "toggle[0]#enabled" };
      expect(stateProbeChangedPaths(before, after)).toEqual(["text[1]#status", "toggle[0]#enabled"]);
      expect(stateProbeSnapshot(before).digest).not.toBe(stateProbeSnapshot(after).digest);
    });
  });

  //#region 🔖️T-P8-tests
  describe("stagePluginDescriptor", () => {
    it("stages descriptor siblings for migrated plugins and leaves unmigrated plugins absent", () => {
      const root = mkdtempSync(join(tmpdir(), "semio-plugin-descriptor-stage-"));
      try {
        const target = {
          pluginId: "demo",
          cratePath: "owner/demo/📦️packages/🦀️rust",
          packageName: "demo",
          wasmOut: "demo.wasm",
          role: "plugin",
          capabilities: [],
          contributes: [],
          consumes: [],
          dependsOn: [],
          activationEvents: [],
          extensionPoints: [],
        } satisfies PluginRegistryEntry;
        const ownerRoot = join(root, "owner/demo");
        const outDir = join(root, "out");
        mkdirSync(ownerRoot, { recursive: true });
        mkdirSync(outDir, { recursive: true });
        writeFileSync(join(ownerRoot, "🔣️.json"), '{"manifest":{"pluginId":"demo"}}\n');
        writeFileSync(join(ownerRoot, "🛂️.descriptor.semio"), "descriptor-pack");
        expect(stagePluginDescriptor(target, outDir, root)).toBe(true);
        expect(readFileSync(join(outDir, "🔣️.json"), "utf8")).toContain('"pluginId":"demo"');
        expect(readFileSync(join(outDir, "🛂️.descriptor.semio"), "utf8")).toBe("descriptor-pack");
        rmSync(join(ownerRoot, "🔣️.json"));
        expect(stagePluginDescriptor(target, outDir, root)).toBe(false);
        expect(existsSync(join(outDir, "🔣️.json"))).toBe(false);
        expect(existsSync(join(outDir, "🛂️.descriptor.semio"))).toBe(false);
      } finally {
        rmSync(root, { recursive: true, force: true });
      }
    });
  });

  describe("pluginComponentBridgeSource", () => {
    itLong("forwards canonical nested byte pages unchanged into the generated component poll", async () => {
      const { execFileSync } = await import("node:child_process");
      const { createShardCommandIngressPages } = await import("../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts");
      const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🎭️actor/📃️page/🧫️fixture/🔣️.json"), "utf8"));
      const vectors = fixture.vectors.filter((row: { length: number }) => row.length !== 0);
      const inputs = vectors.map((row: { length: number }, index: number) => {
        const command = Uint8Array.from({ length: row.length }, (_, byte) => (byte * fixture.bytePattern.multiplier + fixture.bytePattern.addend) % fixture.bytePattern.modulus);
        return { hex: Buffer.from(command).toString("hex"), page: createShardCommandIngressPages({ owner: 7n, generation: 11n, commandIndex: index, commandCount: vectors.length, instance: 13, seq: BigInt(index + 17), command })[0] };
      });
      const output = execFileSync("node", ["--experimental-vm-modules", "--input-type=module", "--eval", `
        import { SourceTextModule, createContext } from "node:vm";
        import { readFileSync } from "node:fs";
        const input = JSON.parse(readFileSync(0, "utf8"));
        let received;
        const context = createContext({ URL, Uint8Array, record: page => { received = page; } });
        const component = new SourceTextModule("export const reactor = { poll: async (_events, page) => { record(page); return { uiPatches: [], status: { tag: 'idle' }, commandIngress: { kind: 0 } }; } }; export const jobs = {}; export const checkpoint = {}; export const describe = {};", { context });
        const host = new SourceTextModule("export const __resolveEffect = () => {}; export const __rejectEffect = () => {};", { context });
        for (const module of [component, host]) { await module.link(() => { throw new Error("unexpected import"); }); await module.evaluate(); }
        const bridge = new SourceTextModule(input.source, { context, identifier: "https://fixture.invalid/bridge.js", initializeImportMeta: meta => { meta.url = "https://fixture.invalid/bridge.js"; }, importModuleDynamically: async specifier => specifier.includes("host-shim") ? host : component });
        await bridge.link(() => { throw new Error("unexpected static import"); }); await bridge.evaluate();
        const api = await bridge.namespace.createActorApi("paged", 1n);
        const rows = [];
        for (const entry of input.inputs) {
          const page = entry.page;
          for (const key of ["owner", "generation", "seq"]) page.cursor[key] = BigInt(page.cursor[key]);
          for (let block = 0; block < 64; block++) for (let word = 0; word < 8; word++) {
            const value = page.page["block" + block.toString().padStart(2, "0")];
            value["word" + word] = BigInt(value["word" + word]);
          }
          await api.poll([], page, { fuel: 1, wallMs: 4, maxEffects: 1, maxPatchBytes: 4096 });
          const bytes = Buffer.alloc(4096);
          for (let block = 0; block < 64; block++) for (let word = 0; word < 8; word++) bytes.writeBigUInt64LE(received.page["block" + block.toString().padStart(2, "0")]["word" + word], block * 64 + word * 8);
          rows.push({ same: received === page, keys: Object.keys(received), length: received.page.length, hex: bytes.subarray(0, received.page.length).toString("hex"), zeroTail: bytes.subarray(received.page.length).every(byte => byte === 0) });
        }
        console.log(JSON.stringify(rows));
      `], { input: JSON.stringify({ source: pluginComponentBridgeSource("component", "component.core.wasm"), inputs }, (_key, value) => typeof value === "bigint" ? value.toString() : value), encoding: "utf8", timeout: 10_000 });
      expect(JSON.parse(output)).toEqual(inputs.map((entry: { hex: string }, index: number) => ({ same: true, keys: ["cursor", "page"], length: vectors[index].length, hex: entry.hex, zeroTail: true })));
    });

    itLong("maps issued UI patch receipts and exact ACK or rejection through the generated bridge", async () => {
      const { execFileSync } = await import("node:child_process");
      const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🧫️fixture/🔣️.json"), "utf8"));
      const output = execFileSync("node", ["--experimental-vm-modules", "--input-type=module", "--eval", `
        import { SourceTextModule, createContext } from "node:vm";
        import { readFileSync } from "node:fs";
        const input = JSON.parse(readFileSync(0, "utf8"));
        const observed = [];
        const context = createContext({ URL, Uint8Array, nextResult: null, record: events => observed.push(events) });
        const component = new SourceTextModule("export const reactor = { poll: async events => { record(events); return nextResult; } }; export const jobs = {}; export const checkpoint = {}; export const describe = {};", { context });
        const host = new SourceTextModule("export const __resolveEffect = () => {}; export const __rejectEffect = () => {};", { context });
        for (const module of [component, host]) { await module.link(() => { throw new Error("unexpected import"); }); await module.evaluate(); }
        const bridge = new SourceTextModule(input.source, { context, identifier: "https://fixture.invalid/bridge.js", initializeImportMeta: meta => { meta.url = "https://fixture.invalid/bridge.js"; }, importModuleDynamically: async specifier => specifier.includes("host-shim") ? host : component });
        await bridge.link(() => { throw new Error("unexpected static import"); }); await bridge.evaluate();
        const budget = { fuel: 1, wallMs: 4, maxEffects: 1, maxPatchBytes: 4096 };
        const result = (receipt, count) => ({ uiPatches: Array.from({ length: count }, () => ({})), uiPatchReceipt: receipt, status: { tag: "idle" }, commandIngress: { kind: 0 } });
        const convert = value => ({ lifetime: { activationGeneration: BigInt(value.lifetime.activationGeneration), instanceId: value.lifetime.instanceId, guestLifetime: BigInt(value.lifetime.guestLifetime) }, patchSequence: BigInt(value.patchSequence) });
        const rows = [];
        for (const vector of input.fixture.vectors) {
          const receipt = convert(vector.value);
          const api = await bridge.namespace.createActorApi("same", receipt.lifetime.activationGeneration);
          context.nextResult = result(receipt, 1);
          const actual = await api.poll([], undefined, budget);
          const row = { hex: actual.uiPatchReceipt instanceof Uint8Array ? Buffer.from(actual.uiPatchReceipt).toString("hex") : null, feedback: [] };
          context.nextResult = result(undefined, 0);
          for (const kind of input.fixture.feedback.kinds) {
            const payload = { receipt, surface: { instance: receipt.lifetime.instanceId, surface: input.fixture.feedback.surface }, revision: BigInt(input.fixture.feedback.revision), ...(kind === "rejected" ? { reason: "fixture rejection" } : {}) };
            await api.poll([{ kind: "patch-" + kind, payload }], undefined, budget);
            row.feedback.push(observed.at(-1)[0]);
          }
          rows.push(row);
        }
        const receipt = convert(input.fixture.vectors[1].value);
        const api = await bridge.namespace.createActorApi("same", receipt.lifetime.activationGeneration);
        const pairings = [];
        for (const row of input.fixture.pairing) {
          context.nextResult = result(row.hasReceipt ? receipt : undefined, row.patchCount);
          try { await api.poll([], undefined, budget); pairings.push(true); } catch { pairings.push(false); }
        }
        const malformedResults = [result({ ...receipt, patchSequence: 0n }, 1), result({ ...receipt, lifetime: { ...receipt.lifetime, activationGeneration: 42n } }, 1), { ...result(undefined, 0), uiPatches: undefined }];
        const refusedResults = [];
        for (const raw of malformedResults) { context.nextResult = raw; try { await api.poll([], undefined, budget); refusedResults.push(false); } catch { refusedResults.push(true); } }
        context.nextResult = result(undefined, 0);
        const refusedEvents = [];
        for (const kind of input.fixture.feedback.kinds) {
          for (const wrong of [undefined, { ...receipt, patchSequence: 0n }, { ...receipt, lifetime: { ...receipt.lifetime, activationGeneration: 42n } }]) {
            const before = observed.length;
            try { await api.poll([{ kind: "patch-" + kind, payload: { receipt: wrong, surface: { instance: receipt.lifetime.instanceId, surface: input.fixture.feedback.surface }, revision: 1n, reason: "fixture" } }], undefined, budget); refusedEvents.push(false); } catch { refusedEvents.push(observed.length === before); }
          }
        }
        console.log(JSON.stringify({ rows, pairings, refusedResults, refusedEvents }, (_key, value) => typeof value === "bigint" ? value.toString() : value));
      `], { input: JSON.stringify({ source: pluginComponentBridgeSource("component", "component.core.wasm"), fixture }), encoding: "utf8", timeout: 10_000 });
      const result = JSON.parse(output);
      expect(result.rows.map((row: { hex: string }) => row.hex)).toEqual(fixture.vectors.map((row: { hex: string }) => row.hex));
      expect(result.pairings).toEqual(fixture.pairing.map((row: { accepted: boolean }) => row.accepted));
      expect(result.refusedResults).toEqual([true, true, true]);
      expect(result.refusedEvents).toEqual([true, true, true, true, true, true]);
      for (let index = 0; index < fixture.vectors.length; index += 1) {
        for (let kindIndex = 0; kindIndex < fixture.feedback.kinds.length; kindIndex += 1) {
          const kind = fixture.feedback.kinds[kindIndex];
          expect(result.rows[index].feedback[kindIndex]).toEqual({ tag: "patch-" + kind, val: { receipt: fixture.vectors[index].value, surface: { instance: fixture.vectors[index].value.lifetime.instanceId, surface: fixture.feedback.surface }, revision: fixture.feedback.revision, ...(kind === "rejected" ? { reason: "fixture rejection" } : {}) } });
        }
      }
    });

    itLong("maps canonical lifecycle requests and receipts through the real generated bridge", async () => {
      const { execFileSync } = await import("node:child_process");
      const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture/🔣️.json"), "utf8"));
      const output = execFileSync("node", ["--experimental-vm-modules", "--input-type=module", "--eval", `
        import { SourceTextModule, createContext } from "node:vm";
        import { readFileSync } from "node:fs";
        const input = JSON.parse(readFileSync(0, "utf8"));
        const observed = [];
        const context = createContext({ URL, Uint8Array, nextReceipt: undefined, record: events => observed.push(events) });
        const component = new SourceTextModule("export const reactor = { poll: async events => { record(events); return { uiPatches: [], lifecycleReceipt: nextReceipt, status: { tag: 'idle' }, commandIngress: { kind: 0 } }; } }; export const jobs = {}; export const checkpoint = {}; export const describe = {};", { context });
        const host = new SourceTextModule("export const __resolveEffect = () => {}; export const __rejectEffect = () => {};", { context });
        for (const module of [component, host]) { await module.link(() => { throw new Error("unexpected import"); }); await module.evaluate(); }
        const bridge = new SourceTextModule(input.source, { context, identifier: "https://fixture.invalid/bridge.js", initializeImportMeta: meta => { meta.url = "https://fixture.invalid/bridge.js"; }, importModuleDynamically: async specifier => specifier.includes("host-shim") ? host : component });
        await bridge.link(() => { throw new Error("unexpected static import"); }); await bridge.evaluate();
        const budget = { fuel: 1, wallMs: 4, maxEffects: 1, maxPatchBytes: 4096 };
        const rows = [];
        const body = receipt => ({ lifetime: receipt.lifetime, requestSequence: BigInt(receipt.requestSequence), ...(receipt.kind === "captured" ? {} : { closeGeneration: receipt.closeGeneration }) });
        for (const row of input.vectors) {
          const value = JSON.parse(JSON.stringify(row.value), (key, value) => ["activationGeneration", "guestLifetime", "closeGeneration"].includes(key) ? BigInt(value) : value);
          const receipt = value.kind === "ack" ? value.receipt : value;
          const generation = value.kind === "open" ? value.activationGeneration : receipt.lifetime.activationGeneration;
          const api = await bridge.namespace.createActorApi("same", generation);
          context.nextReceipt = undefined;
          if (["captured", "accepted", "retired"].includes(value.kind)) {
            context.nextReceipt = { tag: value.kind, val: body(value) };
            const result = await api.poll([], undefined, budget);
            rows.push({ kind: value.kind, hex: Buffer.from(result.lifecycleReceipt).toString("hex") });
          } else {
            const kind = value.kind === "open" ? "instance-open" : value.kind === "close" ? "instance-close" : "instance-lifecycle-ack";
            const payload = value.kind === "open" ? { instance: value.instanceId, activationGeneration: value.activationGeneration, requestSequence: value.requestSequence, appId: "fixture", actor: "tester", config: [], assets: [], capabilities: [], quotas: [] } : value;
            await api.poll([{ kind, payload }], undefined, budget);
            rows.push({ kind: value.kind, event: observed.at(-1)[0] });
          }
        }
        const api = await bridge.namespace.createActorApi("same", 1n);
        const errors = [];
        for (const receipt of [{ tag: "open", val: {} }, { tag: "captured", val: { lifetime: { activationGeneration: 1n, instanceId: 7, guestLifetime: 13n }, requestSequence: 9007199254740992n } }]) {
          context.nextReceipt = receipt;
          try { await api.poll([], undefined, budget); errors.push(false); } catch { errors.push(true); }
        }
        console.log(JSON.stringify({ rows, errors }, (_key, value) => typeof value === "bigint" ? value.toString() : value));
      `], { input: JSON.stringify({ source: pluginComponentBridgeSource("component", "component.core.wasm"), vectors: fixture.vectors }), encoding: "utf8", timeout: 10_000 });
      const result = JSON.parse(output);
      expect(result.errors).toEqual([true, true]);
      for (let index = 0; index < fixture.vectors.length; index += 1) {
        const value = fixture.vectors[index].value;
        const actual = result.rows[index];
        if (["captured", "accepted", "retired"].includes(value.kind)) expect(actual.hex).toBe(fixture.vectors[index].hex);
        else if (value.kind === "open") expect(actual.event).toMatchObject({ tag: "instance-open", val: { instance: value.instanceId, activationGeneration: value.activationGeneration, requestSequence: String(value.requestSequence) } });
        else if (value.kind === "close") expect(actual.event).toEqual({ tag: "instance-close", val: { lifetime: value.lifetime, requestSequence: String(value.requestSequence) } });
        else expect(actual.event).toEqual({ tag: "instance-lifecycle-ack", val: { tag: value.receipt.kind, val: { lifetime: value.receipt.lifetime, requestSequence: String(value.receipt.requestSequence), ...(value.receipt.kind === "captured" ? {} : { closeGeneration: value.receipt.closeGeneration }) } } });
      }
    });

    it("requires every actor export from plugin and extension component namespaces", () => {
      const actor = Object.fromEntries(Object.entries(ACTOR_COMPONENT_EXPORTS).map(([name, methods]) => [name, Object.fromEntries(methods.map((method) => [method, () => undefined]))]));
      expect(() => assertActorComponentExports(actor, ACTOR_COMPONENT_EXPORTS)).not.toThrow();
      expect(() => assertActorComponentExports({ _util: {} }, ACTOR_COMPONENT_EXPORTS)).toThrow("Missing actor export reactor.poll");
      for (const [name, methods] of Object.entries(ACTOR_COMPONENT_EXPORTS)) {
        for (const method of methods) {
          const incomplete = { ...actor, [name]: { ...actor[name], [method]: undefined } };
          expect(() => assertActorComponentExports(incomplete, ACTOR_COMPONENT_EXPORTS)).toThrow(`${name}.${method}`);
        }
      }
    });

    itLong("finalizes genuine descriptor bytes identically to the native descriptor oracle", async () => {
      const { clonePackValue, decodePackValue, encodePackValue, packValueToExactJson } = await import("@semio-tech/framework-os");
      const { createHash } = await import("node:crypto");
      const bytes = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🎪️demonstrator/🛂️.descriptor.semio"));
      const decoded = decodePackValue(bytes);
      const descriptor = decoded as unknown as { manifest: { pluginId: string }; hashes: { wasmSha256: string; coreWasmSha256: string; descriptorSha256: string } };
      const finalized = finalizePluginDescriptor(bytes, descriptor.manifest.pluginId, descriptor.hashes.wasmSha256, descriptor.hashes.coreWasmSha256);
      expect(Buffer.from(finalized.pack)).toEqual(bytes);
      expect(JSON.parse(finalized.json)).toEqual(packValueToExactJson(decoded));
      const prehash = clonePackValue(decoded) as unknown as { hashes: { descriptorSha256: string } };
      prehash.hashes.descriptorSha256 = "";
      expect(createHash("sha256").update(encodePackValue(prehash as unknown as PackValue)).digest("hex")).toBe(descriptor.hashes.descriptorSha256);
      expect(() => finalizePluginDescriptor(bytes, "wrong-plugin", descriptor.hashes.wasmSha256, descriptor.hashes.coreWasmSha256)).toThrow("identity");
      descriptor.manifest.pluginId = "assembly-failed";
      expect(() => finalizePluginDescriptor(encodePackValue(decoded), "assembly-failed", descriptor.hashes.wasmSha256, descriptor.hashes.coreWasmSha256)).toThrow("assembly");
    });

    it("adapts the shard envelope into the canonical jco variant representation", () => {
      const source = pluginComponentBridgeSource("plugin", "plugin.core.wasm");
      expect(source).toContain('kind === "wake" ? ({ tag: kind }) : ({ tag: kind, val: payload })');
    });

    it("adapts the actor grant into the reactor budget vocabulary", () => {
      const source = pluginComponentBridgeSource("plugin", "plugin.core.wasm");
      expect(source).toContain("fuel: BigInt(budget.fuel), deadlineMs: budget.wallMs");
      expect(source).toContain("maxFrames: 8");
    });

    it("normalizes the scalar command-ingress wire record without relying on async variant discriminants", () => {
      const source = pluginComponentBridgeSource("plugin", "plugin.core.wasm");
      expect(source).toContain("function normalizeCommandIngress(status)");
      expect(source).toContain('[0, "idle"], [1, "page-accepted"], [2, "backpressure"], [3, "command-pending"], [4, "command-complete"], [5, "fault"]');
      expect(source).toContain("commandIngress: normalizeCommandIngress(result.commandIngress)");
    });

    it("gives every actor a distinct component module while carrying the rebuild version", () => {
      const source = pluginComponentBridgeSource("plugin_component", "plugin_component.core.wasm");
      expect(source).toContain('componentUrl.searchParams.set("actor", actorId)');
      expect(source).toContain('componentUrl.searchParams.set("v", rebuildVersion)');
      expect(source).toContain("await import(componentUrl.href)");
      expect(source).not.toContain('await import("./🟦️")');
    });
  });

  describe("rewriteJcoComponentAssetUrls", () => {
    itLong("keeps generated host imports and replies isolated across same-package activations", async () => {
      const { execFileSync } = await import("node:child_process");
      const { default: Ajv } = await import("ajv");
      const fixtureRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🧫️fixtures");
      const fixture = JSON.parse(readFileSync(join(fixtureRoot, "⚡️host-activation.json"), "utf8")) as { activations: Array<{ actorId: string; generation: string; value: string }> };
      const oracle = new Ajv();
      expect(oracle.validate(JSON.parse(readFileSync(join(fixtureRoot, "🛡️host-activation.schema.json"), "utf8")), fixture)).toBe(true);
      const component = rewriteJcoComponentAssetUrls(`import { storageRead, emit } from "./${PLUGIN_HOST_SHIM_FILE}";
export const reactor = { poll: async (events) => { emit(events[0].val); return { value: await storageRead(events[0].val), uiPatches: [], commandIngress: { kind: 0 } }; } };
export const jobs = {};
export const checkpoint = {};
export const describe = {};`);
      const output = execFileSync("node", ["--experimental-vm-modules", "--input-type=module", "--eval", `
        import { SourceTextModule, createContext } from "node:vm";
        import { readFileSync } from "node:fs";
        const input = JSON.parse(readFileSync(0, "utf8"));
        const sent = [];
        const context = createContext({ URL, console, self: { postMessage: (message) => sent.push(message) } });
        const cache = new Map();
        async function load(url) {
          let module = cache.get(url);
          if (module) return module;
          const source = input.sources[decodeURIComponent(new URL(url).pathname).split("/").at(-1)];
          if (typeof source !== "string") throw new Error("unknown fixture module " + url);
          module = new SourceTextModule(source, { context, identifier: url, initializeImportMeta: (meta) => { meta.url = url; }, importModuleDynamically: async (specifier, parent) => {
            const child = await load(new URL(specifier, parent.identifier).href);
            if (child.status === "linked") await child.evaluate();
            return child;
          } });
          cache.set(url, module);
          await module.link((specifier, parent) => load(new URL(specifier, parent.identifier).href));
          return module;
        }
        const bridge = await load("file:///fixture/bridge.js?v=source-checkpoint");
        await bridge.evaluate();
        const apis = [];
        for (const row of input.activations) apis.push(await bridge.namespace.createActorApi(row.actorId, BigInt(row.generation)));
        const pending = apis.map((api, index) => api.poll([{ kind: "request", payload: { input: index } }], undefined, { fuel: 1, wallMs: 4, maxEffects: 8, maxPatchBytes: 4096 }));
        const effects = sent.filter((message) => message.frame.envelope.payload.kind === "effect-request");
        const actual = effects.map((message, index) => ({ actorId: message.actorId, generation: String(message.activationGeneration), value: input.activations[index].value }));
        for (let index = 0; index < effects.length; index += 1) apis[index].resolveEffect(effects[index].frame.envelope.payload.payload.requestId, input.activations[index].value);
        const values = (await Promise.all(pending)).map((value) => value.value);
        const emissions = sent.filter((message) => message.frame.envelope.payload.kind === "effect-emit").map((message) => ({ actorId: message.actorId, generation: String(message.activationGeneration) }));
        console.log(JSON.stringify({ actual, values, emissions, requestIds: effects.map((message) => message.frame.envelope.payload.payload.requestId) }));
      `], { input: JSON.stringify({ activations: fixture.activations, sources: { "bridge.js": pluginComponentBridgeSource("component", "component.core.wasm"), "component.js": component, "🟨️.js": hostShimSource() } }), encoding: "utf8", timeout: 10_000 });
      const result = JSON.parse(output) as { actual: typeof fixture.activations; values: string[]; emissions: Array<{ actorId: string; generation: string }>; requestIds: string[] };
      expect(result.actual).toEqual(fixture.activations);
      expect(oracle.validate({ const: fixture.activations }, result.actual)).toBe(true);
      expect(result.values).toEqual(fixture.activations.map((row) => row.value));
      expect(result.emissions).toEqual(fixture.activations.map(({ actorId, generation }) => ({ actorId, generation })));
      expect(new Set(result.requestIds).size).toBe(fixture.activations.length);
    });

    itLong("rejects a stale effect reply in the actual generated worker dispatcher", async () => {
      const { execFileSync } = await import("node:child_process");
      const output = execFileSync("node", ["--experimental-vm-modules", "--input-type=module", "--eval", `
        import { SourceTextModule, createContext } from "node:vm";
        import { readFileSync } from "node:fs";
        const input = JSON.parse(readFileSync(0, "utf8"));
        const replies = [];
        let listener;
        const context = createContext({ console, WebAssembly: { Suspending() {}, promising() {} }, self: { postMessage() {}, addEventListener: (_kind, callback) => { listener = callback; } }, record: (generation, request, value) => replies.push({ generation: String(generation), request, value }) });
        const bridge = new SourceTextModule("export async function createActorApi(actorId, generation) { return { resolveEffect: (request, value) => record(generation, request, value) }; }", { context });
        await bridge.link(() => { throw new Error("unexpected import"); });
        await bridge.evaluate();
        const worker = new SourceTextModule(input.worker, { context, importModuleDynamically: async () => bridge });
        await worker.link(() => { throw new Error("unexpected import"); });
        await worker.evaluate();
        await listener({ data: { kind: "activate", requestId: "a1", actorId: "same", activationGeneration: 1n, moduleUrl: "fixture" } });
        await listener({ data: { kind: "dispose", actorId: "same", activationGeneration: 1n } });
        await listener({ data: { kind: "activate", requestId: "a2", actorId: "same", activationGeneration: 2n, moduleUrl: "fixture" } });
        for (const [generation, from, to, value] of [[1n, "kernel", "same", "old"], [undefined, "kernel", "same", "missing"], [2n, "actor", "same", "foreign-origin"], [2n, "kernel", "other", "foreign-target"], [2n, "kernel", "same", "fresh"]]) {
          await listener({ data: { kind: "frame", actorId: "same", activationGeneration: generation, frame: { kind: "Envelope", envelope: { to, from: { kind: from }, payload: { kind: "effect-complete", payload: { requestId: "reused", value } } } } } });
        }
        console.log(JSON.stringify(replies));
      `], { input: JSON.stringify({ worker: shardWorkerSource() }), encoding: "utf8", timeout: 10_000 });
      expect(JSON.parse(output)).toEqual([{ generation: "2", request: "reused", value: "fresh" }]);
    });

    itLong("forwards lifecycle through the captured scheduled turn and rejects the removed side message", async () => {
      const { execFileSync } = await import("node:child_process");
      const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture/🔣️.json"), "utf8")) as { vectors: Array<{ value: { kind: string }; hex: string }> };
      const request = fixture.vectors.find((row) => row.value.kind === "close")!;
      const output = execFileSync("node", ["--experimental-vm-modules", "--input-type=module", "--eval", `
        import { SourceTextModule, createContext } from "node:vm";
        import { readFileSync } from "node:fs";
        const input = JSON.parse(readFileSync(0, "utf8"));
        const messages = [];
        const calls = [];
        let listener;
        const context = createContext({ console, Uint8Array, WebAssembly: { Suspending() {}, promising() {} }, self: { postMessage: (message) => messages.push(message), addEventListener: (_kind, callback) => { listener = callback; } }, record: (name) => calls.push(name) });
        const bridge = new SourceTextModule("export async function createActorApi() { record('captured'); return { poll: async (events) => { record(events); return { status: { tag: 'idle' } }; } }; }", { context });
        await bridge.link(() => { throw new Error("unexpected import"); });
        await bridge.evaluate();
        const worker = new SourceTextModule(input.worker, { context, importModuleDynamically: async () => bridge });
        await worker.link(() => { throw new Error("unexpected import"); });
        await worker.evaluate();
        await listener({ data: { kind: "activate", requestId: "a1", actorId: "same", activationGeneration: 1n, moduleUrl: "fixture" } });
        const wire = Uint8Array.from(Buffer.from(input.request, "hex"));
        const payload = JSON.parse(JSON.stringify(input.value), (key, value) => ["activationGeneration", "guestLifetime", "closeGeneration"].includes(key) ? BigInt(value) : value);
        const events = [{ kind: "instance-close", payload }];
        await listener({ data: { kind: "turn", requestId: "close", actorId: "same", activationGeneration: 1n, events, budget: {} } });
        await listener({ data: { kind: "turn", requestId: "old", actorId: "same", activationGeneration: 2n, events, budget: {} } });
        await listener({ data: { kind: "closeInstance", requestId: "side", actorId: "same", activationGeneration: 1n, wire } });
        await listener({ data: { kind: "activate", requestId: "reuse", actorId: "same", activationGeneration: 2n, moduleUrl: "fixture" } });
        console.log(JSON.stringify({ results: messages.filter((message) => message.kind === "result").map(({ requestId, ok, error }) => ({ requestId, ok, ...(error ? { error } : {}) })), receipts: messages.filter((message) => message.kind === "instanceCloseReceipt").length, calls }, (_key, value) => typeof value === "bigint" ? value.toString() : value));
      `], { input: JSON.stringify({ worker: shardWorkerSource(), request: request.hex, value: request.value }), encoding: "utf8", timeout: 10_000 });
      const result = JSON.parse(output);
      expect(result.results).toEqual([
        { requestId: "a1", ok: true },
        { requestId: "close", ok: true },
        { requestId: "old", ok: false, error: "actor-lifecycle.activation-mismatch" },
        { requestId: "side", ok: false, error: "unknown shard worker message kind: closeInstance" },
        { requestId: "reuse", ok: false, error: "actor-close.activation-already-owned" },
      ]);
      expect(result.receipts).toBe(0);
      expect(result.calls).toEqual(["captured", [{ kind: "instance-close", payload: request.value }]]);
    });

    it("propagates a component module rebuild version to every extracted core wasm fetch", () => {
      const generated = `const module0 = fetchCompile(new URL('./plugin_component.core.wasm', source.url));
const module1 = fetchCompile(new URL('./plugin_component.core2.wasm', source.url));`;
      const rewritten = rewriteJcoComponentAssetUrls(generated);
      expect(rewritten).toContain("function __semioVersionedComponentAssetUrl(path)");
      expect(rewritten).toContain("const rebuildVersion = new URL(source.url).searchParams.get(\"v\")");
      expect(rewritten).toContain("__semioVersionedComponentAssetUrl('./plugin_component.core.wasm')");
      expect(rewritten).toContain("__semioVersionedComponentAssetUrl('./plugin_component.core2.wasm')");
      expect(rewriteJcoComponentAssetUrls(rewritten)).toBe(rewritten);
    });
  });

  describe("PluginComponentInstantiation", () => {
    itLong("executes independent Wasm memories from one cached explicit factory module", async () => {
      const { execFileSync } = await import("node:child_process");
      const { default: Ajv } = await import("ajv");
      const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🧫️fixtures");
      const fixture = JSON.parse(readFileSync(join(root, "🏗️component-instantiation.json"), "utf8"));
      const oracle = new Ajv({ strict: true });
      expect(oracle.validate(JSON.parse(readFileSync(join(root, "📐️component-instantiation.schema.json"), "utf8")), fixture)).toBe(true);
      const output = execFileSync("node", ["--experimental-vm-modules", "--input-type=module", "--eval", `
        import { parse, transpile } from "@bytecodealliance/jco";
        import { SourceTextModule, createContext } from "node:vm";
        import { readFileSync } from "node:fs";
        import ts from "typescript";
        const input = JSON.parse(readFileSync(0, "utf8"));
        const bytes = await parse(input.component);
        const options = { name: "counter", noTypescript: true, base64Cutoff: 0, nodejsCompat: false, wasiShim: false };
        const explicit = await transpile(bytes, { ...options, instantiation: "async" });
        const automatic = await transpile(bytes, options);
        const source = new TextDecoder().decode(explicit.files["counter.js"]);
        const auto = new TextDecoder().decode(automatic.files["counter.js"]);
        const moduleInstanceRoots = source => {
          const parsed = ts.createSourceFile("counter.js", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
          return parsed.statements.some(statement => ts.isVariableStatement(statement) && statement.declarationList.declarations.some(declaration => ts.isIdentifier(declaration.name) && /^(exports|memory)[0-9]+$/.test(declaration.name.text)));
        };
        const context = createContext({ WebAssembly, Promise, console, TextEncoder, TextDecoder });
        const module = new SourceTextModule(source, { context, identifier: "https://fixture.invalid/counter.js" });
        await module.link(() => { throw new Error("Unexpected factory import"); });
        await module.evaluate();
        const captured = [];
        const moduleImportInstances = captured.length;
        const load = path => WebAssembly.compile(explicit.files[path]);
        const instantiate = (module, imports) => { const instance = new WebAssembly.Instance(module, imports); captured.push(instance); return instance; };
        const owners = [await module.namespace.instantiate(load, {}, instantiate), await module.namespace.instantiate(load, {}, instantiate)];
        const results = input.calls.map(call => owners[call.instance].advance(call.delta));
        const core = await load("counter.core.wasm");
        const direct = [new WebAssembly.Instance(core), new WebAssembly.Instance(core)];
        const independent = input.calls.map(call => direct[call.instance].exports.advance(call.delta));
        const actual = { moduleImportInstances, instantiated: captured.length, results, memory: captured.map(instance => new DataView(instance.exports.memory.buffer).getUint32(0, true)), distinctMemories: captured[0].exports.memory !== captured[1].exports.memory, factoryHasNoModuleInstanceRoots: !moduleInstanceRoots(source), automaticHasModuleInstanceRoots: moduleInstanceRoots(auto) };
        console.log(JSON.stringify({ actual, independent }));
      `], { cwd: repoRoot, input: JSON.stringify(fixture), encoding: "utf8", timeout: 20_000, maxBuffer: 2 * 1024 * 1024 });
      const { actual, independent } = JSON.parse(output);
      expect(actual).toEqual(fixture.expected);
      expect(independent).toEqual(fixture.expected.results);
      expect(oracle.validate({ const: fixture.expected }, actual)).toBe(true);
    }, 30_000);
  });

  describe("rewriteJcoAsyncResultLifting", () => {
    it("checks the resolved callback memory", () => {
      const jcoGenerated = `function taskReturn(ctx) {
  const memory = ctx.getMemoryFn();
  if (!ctx.memory) {
      _debugLog('missing memory despite indirect param usage', { ctx });
  }
}`;
      const rewritten = rewriteJcoAsyncResultLifting(jcoGenerated);
      expect(rewritten).toContain("if (!memory) {");
      expect(rewriteJcoAsyncResultLifting(rewritten)).toBe(rewritten);
    });

    itLong("preserves direct descriptor and job results and lifts large turn results indirectly", async () => {
      const { execFileSync } = await import("node:child_process");
      const fixturePath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🧫️fixtures/⏳️async-results.json");
      const generated = execFileSync("node", ["--input-type=module", "--eval", `
        import { parse, transpile } from "@bytecodealliance/jco";
        import { readFileSync } from "node:fs";
        const fixture = JSON.parse(readFileSync(process.argv[1], "utf8"));
        const results = [];
        for (const test of fixture.cases) {
          const wat = fixture.componentTemplate.replace("{{type}}", test.type).replace("{{params}}", test.params.join(" "));
          const { files } = await transpile(await parse(wat), { name: test.id, noTypescript: true });
          results.push({ ...test, source: new TextDecoder().decode(files[test.id + ".js"]) });
        }
        console.log(JSON.stringify(results));
      `, fixturePath], { cwd: repoRoot, encoding: "utf8", timeout: 20_000, maxBuffer: 8 * 1024 * 1024 });
      for (const test of JSON.parse(generated)) {
        const rewritten = rewriteJcoAsyncResultLifting(test.source);
        expect(rewritten.match(/taskReturn\.bind\([\s\S]*?useDirectParams: (true|false)/)?.[1], test.id).toBe(String(test.direct));
        expect(rewriteJcoAsyncResultLifting(rewritten), test.id).toBe(rewritten);
      }
    }, 30_000);
  });

  describe("rewritePreview2ShimImportSource", () => {
    itLong("rebases exact declared vendor URLs while preserving the public route", async () => {
      const ts = await import("typescript"), fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🪞️vendor.json"), "utf8"));
      const printer = ts.createPrinter();
      for (const row of fixture.cases) {
        const actual = rewritePreview2ShimImportSource(row.input, fixture.prefix);
        expect(actual).toBe(row.output);
        const source = ts.createSourceFile("fixture.ts", row.input, ts.ScriptTarget.Latest, true);
        const transformed = ts.transform(source, [(context) => (node) => ts.visitEachChild(node, (child) => ts.isImportDeclaration(child) && ts.isStringLiteral(child.moduleSpecifier) && child.moduleSpecifier.text === row.inputModule ? ts.factory.updateImportDeclaration(child, child.modifiers, child.importClause, ts.factory.createStringLiteral(row.outputModule), child.attributes) : child, context)]);
        const actualTree = ts.createSourceFile("fixture.ts", actual, ts.ScriptTarget.Latest, true);
        expect(printer.printFile(transformed.transformed[0])).toBe(printer.printFile(ts.factory.updateSourceFile(actualTree, actualTree.statements.map((node) => ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) ? ts.factory.updateImportDeclaration(node, node.modifiers, node.importClause, ts.factory.createStringLiteral(node.moduleSpecifier.text), node.attributes) : node))));
        transformed.dispose();
        if (row.outputModule.startsWith("../")) expect(decodeURIComponent(new URL(row.outputModule, `https://example.invalid${MODULE_EXTENSION_ROUTE}/🧮️flow-extension-math/component.js`).pathname)).toBe(MODULE_PLUGIN_ROUTE + "/" + fixture.vendorPath + "/" + basename(row.outputModule));
      }
    });
  });

  describe("deployed vendor transport", () => {
    itLong("closes static compiler source imports through explicit neutral entry points", async () => {
      const ts = await import("typescript");
      const authority = resolve(dirname(fileURLToPath(source.url)), "../../🚚️distribution");
      const fixture = JSON.parse(readFileSync(join(authority, "🕸️input-graph.json"), "utf8"));
      const sandbox = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "distribution-input-graph-"));
      for (const [path, source] of Object.entries(fixture.files)) writeFileSync(join(sandbox, path), source as string);
      expect(await distributionStaticSourcePaths(sandbox, [fixture.entry])).toEqual(fixture.expected);
      const oracle = new Set<string>();
      const visit = (path: string) => {
        if (oracle.has(path)) return;
        oracle.add(path);
        if (!path.endsWith(".ts")) return;
        for (const item of ts.preProcessFile(readFileSync(join(sandbox, path), "utf8")).importedFiles) if (!item.fileName.startsWith("node:")) {
          const resolved = ts.resolveModuleName(item.fileName, join(sandbox, path), { moduleResolution: ts.ModuleResolutionKind.Bundler, resolveJsonModule: true }, ts.sys).resolvedModule;
          if (!resolved) throw new Error(`TypeScript oracle did not resolve ${item.fileName}`);
          visit(relative(sandbox, resolved.resolvedFileName).replaceAll("\\", "/"));
        }
      };
      visit(fixture.entry);
      expect([...oracle].sort(distributionPathOrder)).toEqual(fixture.expected);
      for (const entry of fixture.rejectedEntries) await expect(distributionStaticSourcePaths(sandbox, [entry])).rejects.toThrow();
      const input = JSON.parse(readFileSync(join(authority, "🔗️inputs.json"), "utf8"));
      const validate = await devDistributionContractModule("DistributionStaticInputs");
      expect(validate(input), JSON.stringify(validate.errors)).toBe(true);
      expect(parseDistributionStaticInputs(input)).toEqual(input);
      for (const bad of [{ ...input, extra: true }, { ...input, paths: [] }, { ...input, moduleEntries: ["../escape"] }, { ...input, moduleEntries: [input.moduleEntries[0], input.moduleEntries[0]] }]) {
        expect(validate(bad)).toBe(false);
        expect(() => parseDistributionStaticInputs(bad)).toThrow();
      }
    });

    it.runIf(Boolean(process.env.SEMIO_DISTRIBUTION_AUDIT_PATH))("checks actual compiled distribution bytes and preserves the entire old primary distribution", async () => {
      const { pathEmojiStatuteFindings, loadTaxonomy } = await import("../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts");
      const compiled = resolve(process.env.SEMIO_DISTRIBUTION_AUDIT_PATH!), artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);
      expect(relative(artifacts, compiled)).toMatch(/^distribution-compiled-[A-Za-z0-9]+$/u);
      const manifest = parseDistributionManifest(JSON.parse(readFileSync(join(compiled, DISTRIBUTION_LAYOUT.manifest), "utf8")), DISTRIBUTION_LAYOUT);
      const validate = await devDistributionContractModule("DistributionManifest");
      expect(validate(manifest), JSON.stringify(validate.errors)).toBe(true);
      const entries = new Map<string, "file" | "directory">();
      for (const row of manifest.outputs) {
        const bytes = readFileSync(join(compiled, row.path));
        expect(bytes.length).toBe(row.bytes);
        expect(createHash("sha256").update(bytes).digest("hex")).toBe(row.sha256);
        expect(Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex")).toBe(row.sha256);
        entries.set(row.path, "file");
        for (let parent = dirname(row.path); parent !== "."; parent = dirname(parent)) entries.set(parent, "directory");
      }
      entries.set(DISTRIBUTION_LAYOUT.manifest, "file");
      expect(pathEmojiStatuteFindings([...entries].map(([path, nodeKind]) => ({ path, nodeKind })), loadTaxonomy().pathEmojiPolicy.genericEmojiIdentities)).toEqual([]);
      const playDirectory = relative(repoRoot, resolve(dirname(fileURLToPath(source.url)), "../..")).replaceAll("\\", "/"), primary = `${playDirectory}/dist`;
      const baseline = JSON.parse(readFileSync(join(artifacts, "distribution-before-plan.json"), "utf8")).filter((row: { path: string }) => row.path.startsWith(primary + "/"));
      for (const row of baseline) {
        const actual = await distributionFileWitness(join(repoRoot, row.path), row.path);
        expect(actual).toEqual(row);
      }
      const { default: glob } = await import("fast-glob");
      expect((await glob("**/*", { cwd: join(repoRoot, primary), onlyFiles: true, dot: true })).sort()).toEqual(baseline.map((row: { path: string }) => row.path.slice(primary.length + 1)).sort());
      const canonical = join(repoRoot, playDirectory, DISTRIBUTION_LAYOUT.directory);
      expect(await checkDistributionBundle({ manifest, files: new Map(manifest.outputs.map(row => [row.path, readFileSync(join(compiled, row.path))])) }, DISTRIBUTION_LAYOUT, canonical)).toEqual([]);
      console.log(`[DEBUG] independently checked ${manifest.outputs.length} outputs and preserved ${baseline.length} primary files`);
    });

    itLong("completes detached compiler configuration imports in Bun and Node", async () => {
      const { execFileSync, spawnSync } = await import("node:child_process"), { pathToFileURL } = await import("node:url");
      const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/♻️entry-cycle.json"), "utf8"));
      const sandbox = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "distribution-entry-cycle-"));
      const entry = join(sandbox, "📜️entry.mjs"), config = join(sandbox, "⚙️config.mjs");
      writeFileSync(entry, fixture.entry);
      writeFileSync(config, fixture.config);
      for (const executable of ["bun", "node"]) {
        const direct = spawnSync(executable, [entry], { timeout: fixture.timeoutMs, encoding: "utf8", stdio: "pipe" });
        console.log(`[DEBUG] ${executable} direct-cycle status=${direct.status} completion=${direct.stdout.trim() === fixture.expected}`);
        expect(execFileSync(executable, ["--input-type=module", "--eval", fixture.detached, pathToFileURL(config).href], { timeout: fixture.timeoutMs, encoding: "utf8", stdio: "pipe" }).trim()).toBe(fixture.expected);
        expect(() => execFileSync(executable, ["--input-type=module", "--eval", fixture.detached, pathToFileURL(join(sandbox, "🚫️missing.mjs")).href], { timeout: fixture.timeoutMs, stdio: "pipe" })).toThrow();
      }
    });

    itLong("publishes only staged manifest-owned bytes and preserves hostile or unrelated children", async () => {
      const authority = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution");
      const fixture = JSON.parse(readFileSync(join(authority, "🔬️manifest-cases.json"), "utf8"));
      const { layout } = JSON.parse(readFileSync(join(authority, "🧪️cases.json"), "utf8"));
      const sandbox = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "distribution-publication-"));
      const destination = join(sandbox, layout.directory), recovery = join(sandbox, "recovery");
      mkdirSync(destination, { recursive: true });
      mkdirSync(recovery);
      writeFileSync(join(destination, "📌️retained.bin"), fixture.text);
      const plan = { manifest: fixture.manifest, files: new Map(fixture.manifest.outputs.map((row: { path: string }) => [row.path, Buffer.from(fixture.text)])) };
      const before = () => Object.fromEntries([...new Bun.Glob("**/*").scanSync({ cwd: destination, onlyFiles: true, dot: true })].sort().map(path => [path, createHash("sha256").update(readFileSync(join(destination, path))).digest("hex")]));
      await publishDistributionBundle(plan, layout, destination, recovery);
      expect(await checkDistributionBundle(plan, layout, destination)).toEqual([]);
      const retained = before();
      const unknown = join(destination, layout.bundles, "❓️unowned.bin");
      writeFileSync(unknown, fixture.text);
      const withUnknown = before();
      await expect(publishDistributionBundle(plan, layout, destination, recovery)).rejects.toThrow("Unknown distribution child");
      expect(before()).toEqual(withUnknown);
      unlinkSync(unknown);
      const current = plan.manifest.outputs[1]!, oldPath = current.path, newPath = oldPath.replace("AbCd1234", "EfGh5678");
      const next = { manifest: { ...plan.manifest, outputs: [plan.manifest.outputs[0], { ...current, path: newPath }] }, files: new Map([[layout.entry.output, Buffer.from(fixture.text)], [newPath, Buffer.from(fixture.text)]]) };
      writeFileSync(join(destination, oldPath), "external edit");
      const divergent = before();
      await expect(publishDistributionBundle(next, layout, destination, recovery)).rejects.toThrow("Divergent distribution bytes");
      expect(before()).toEqual(divergent);
      writeFileSync(join(destination, oldPath), fixture.text);
      expect(before()).toEqual(retained);
      const result = await publishDistributionBundle(next, layout, destination, recovery);
      expect(result.retired).toEqual([oldPath]);
      expect(readFileSync(join(result.recovery, "📥️previous", oldPath), "utf8")).toBe(fixture.text);
      expect(existsSync(join(destination, oldPath))).toBe(false);
      expect(readFileSync(join(destination, "📌️retained.bin"), "utf8")).toBe(fixture.text);
      expect(await checkDistributionBundle(next, layout, destination)).toEqual([]);
      const { default: glob } = await import("fast-glob");
      expect((await glob("**/*", { cwd: destination, onlyFiles: true, dot: true })).sort()).toEqual([...new Bun.Glob("**/*").scanSync({ cwd: destination, onlyFiles: true, dot: true })].sort());
    });

    itLong("validates exact distribution owner manifests against a neutral digest oracle", async () => {
      const authority = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution");
      const fixture = JSON.parse(readFileSync(join(authority, "🔬️manifest-cases.json"), "utf8"));
      const { layout } = JSON.parse(readFileSync(join(authority, "🧪️cases.json"), "utf8"));
      const validate = await devDistributionContractModule("DistributionManifest");
      expect(validate(fixture.manifest), JSON.stringify(validate.errors)).toBe(true);
      expect(createHash("sha256").update(fixture.text).digest("hex")).toBe(fixture.sha256);
      expect(Buffer.from(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(fixture.text))).toString("hex")).toBe(fixture.sha256);
      const { parseDistributionManifest } = await import("../../🚚️distribution/🟦️.ts");
      expect(parseDistributionManifest(fixture.manifest, layout)).toEqual(fixture.manifest);
      for (const path of fixture.rejectedPaths) expect(() => parseDistributionManifest({ ...fixture.manifest, outputs: [fixture.manifest.outputs[0], { ...fixture.manifest.outputs[1], path }] }, layout)).toThrow();
      for (const path of fixture.rejectedInputs) expect(() => parseDistributionManifest({ ...fixture.manifest, inputs: [{ ...fixture.manifest.inputs[0], path }] }, layout)).toThrow();
      for (const outputs of [[...fixture.manifest.outputs, fixture.manifest.outputs[1]], [fixture.manifest.outputs[0], { ...fixture.manifest.outputs[1], ownerId: "unknown" }], [fixture.manifest.outputs[0], { ...fixture.manifest.outputs[1], bytes: -1 }], [fixture.manifest.outputs[0], { ...fixture.manifest.outputs[1], sha256: "bad" }], [fixture.manifest.outputs[0], { ...fixture.manifest.outputs[1], extra: true }]]) expect(() => parseDistributionManifest({ ...fixture.manifest, outputs }, layout)).toThrow();
    });

    itLong("admits only hand-authored collision-free distribution output owners", async () => {
      const { default: emojiRegex } = await import("emoji-regex");
      const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution");
      const fixture = JSON.parse(readFileSync(join(root, "🧪️cases.json"), "utf8")), catalog = JSON.parse(readFileSync(join(root, "📇️layout.json"), "utf8"));
      const validate = await devDistributionContractModule("DistributionLayout");
      expect(validate(fixture.layout)).toBe(true);
      expect(validate(catalog)).toBe(true);
      const { parseDistributionLayout, distributionChunkName, distributionAssetName } = await import("../../🚚️distribution/🟦️.ts");
      const layout = parseDistributionLayout(fixture.layout);
      for (const row of fixture.cases) expect(row.type === "chunk" ? distributionChunkName(layout, row.facts) : distributionAssetName(layout, row.facts)).toBe(row.output);
      for (const row of fixture.rejectedFacts) expect(() => row.type === "chunk" ? distributionChunkName(layout, row.facts) : distributionAssetName(layout, row.facts)).toThrow();
      for (const output of fixture.rejectedOutputs) expect(() => parseDistributionLayout({ ...fixture.layout, chunks: [{ ...fixture.layout.chunks[0], output }] })).toThrow();
      expect(() => parseDistributionLayout({ ...fixture.layout, extra: true })).toThrow();
      expect(() => parseDistributionLayout({ ...fixture.layout, chunks: [...fixture.layout.chunks, fixture.layout.chunks[0]] })).toThrow();
      expect(() => parseDistributionLayout({ ...fixture.layout, assets: [...fixture.layout.assets, { kind: "name", source: "collision.css", output: "🚀️collision-[hash].css" }] })).toThrow();
      const actual = parseDistributionLayout(catalog), names = new Map<string, string>();
      for (const row of [...actual.chunks, ...actual.assets]) {
        if (row.kind !== "name") expect(existsSync(join(repoRoot, row.source)), row.source).toBe(true);
        const parts = row.output.split("/");
        for (let index = 0; index < parts.length; index++) {
          const name = parts[index]!, emojis = [...name.matchAll(emojiRegex())];
          expect(emojis, name).toHaveLength(1);
          expect(emojis[0]!.index).toBe(0);
          const key = `${parts.slice(0, index).join("/")}\0${emojis[0]![0].replaceAll("\uFE0F", "")}`;
          expect(names.get(key) ?? name).toBe(name);
          names.set(key, name);
        }
      }
      const fonts = actual.assets.filter((row) => row.source.startsWith("node_modules/katex/dist/fonts/"));
      expect(fonts.map((row) => basename(row.source)).sort()).toEqual(readdirSync(join(repoRoot, "node_modules/katex/dist/fonts")).sort());
      console.log(`[DEBUG] validated ${actual.chunks.length} chunk owners and ${actual.assets.length} asset owners, including all ${fonts.length} installed KaTeX fonts`);
    });

    itLong("emits runtime URL assets but no dead in-source-test assets in production", async () => {
      const ts = await import("typescript"), { execFileSync } = await import("node:child_process");
      const fixturePath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🧹️production-tests.json");
      const pluginSource = ts.createSourceFile("vite-plugins.ts", readFileSync(join(dirname(fileURLToPath(source.url)), "🔌️vite-plugins.ts"), "utf8"), ts.ScriptTarget.Latest, true);
      const node = pluginSource.statements.find((item) => ts.isFunctionDeclaration(item) && item.name?.text === "semioProductionTestBoundaryVitePlugin");
      const emitted = node ? ts.transpileModule(node.getText(pluginSource).replace("export function", "function"), { compilerOptions: { module: ts.ModuleKind.ESNext, target: ts.ScriptTarget.ES2022 } }).outputText : "";
      const config = readFileSync(join(dirname(fileURLToPath(source.url)), "⚙️vite.config.ts"), "utf8");
      const active = config.includes("semioProductionTestBoundaryVitePlugin(),");
      const result = JSON.parse(execFileSync("node", ["--input-type=module", "--eval", `
        import { build } from 'vite';
        import { readFileSync } from 'node:fs';
        import { dirname, join, basename } from 'node:path';
        const fixture = JSON.parse(readFileSync(process.argv[1], 'utf8')), rows = [];
        ${emitted}
        for (const row of fixture.assetCases) {
          const id = join(dirname(process.argv[1]), 'neutral-entry.js');
          const result = await build({ configFile: false, root: dirname(process.argv[1]), publicDir: false, logLevel: 'silent', define: { 'vitest': row.mode === 'production' ? 'undefined' : 'globalThis.__semioBuildTestMode' }, plugins: [${node && active ? "...(row.mode === 'production' ? [semioProductionTestBoundaryVitePlugin()] : [])," : ""}{ name: 'neutral-asset-entry', resolveId(value) { if (value === id) return id; }, load(value) { if (value === id) return fixture.assetSource; } }], build: { write: false, emptyOutDir: false, target: 'esnext', minify: false, assetsInlineLimit: 0, rollupOptions: { input: id, preserveEntrySignatures: 'strict', output: { format: 'es' } } } });
          const chunk = result.output.find(item => item.type === 'chunk');
          globalThis.__semioBuildTestMode = row.mode === 'test';
          const code = chunk.code.replaceAll('source.url', JSON.stringify('https://example.invalid/app.js'));
          const runtime = await import('data:text/javascript;base64,' + Buffer.from(code).toString('base64') + '#' + row.mode);
          const assets = result.output.filter(item => item.type === 'asset');
          if (!assets.some(item => decodeURIComponent(new URL(runtime.runtimeUrl).pathname) === '/' + item.fileName)) throw new Error('runtime asset URL missing');
          if (runtime.testUrl && !assets.some(item => decodeURIComponent(new URL(runtime.testUrl).pathname) === '/' + item.fileName)) throw new Error('test asset URL missing');
          rows.push({ mode: row.mode, assets: assets.flatMap(item => item.originalFileNames.map(name => basename(name))).sort(), testUrl: runtime.testUrl !== null });
        }
        console.log(JSON.stringify(rows));
      `, fixturePath], { cwd: repoRoot, encoding: "utf8", timeout: 30_000, maxBuffer: 1024 * 1024 }));
      expect(result).toEqual(JSON.parse(readFileSync(fixturePath, "utf8")).assetCases);
      console.log("[DEBUG] production retains the executable runtime asset URL without emitting test assets; test mode retains both");
    }, 40_000);

    itLong("validates neutral read-only build inspection records against an independent digest oracle", async () => {
      const { default: Ajv } = await import("ajv");
      const fixtureRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures");
      const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔎️build-inspection.json"), "utf8"));
      const validate = await devContractModule("BuildInspectionV1");
      expect(validate(fixture.expected)).toBe(true);
      for (const rejected of fixture.rejected) expect(validate(rejected)).toBe(false);
      expect(fixture.settings).toEqual({ write: false, emptyOutDir: false, publicDir: false, maximumDurationMs: 180_000 });
      expect(Buffer.byteLength(fixture.specimen)).toBe(fixture.expected[0].bytes);
      expect(createHash("sha256").update(fixture.specimen).digest("hex")).toBe(fixture.expected[0].sha256);
      expect(Buffer.from(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(fixture.specimen))).toString("hex")).toBe(fixture.expected[0].sha256);
    });

    it.runIf(Boolean(process.env.SEMIO_BUILD_INSPECTION_OUTPUT))("inspects the actual production Rollup output without writing or replacing distribution bytes", async () => {
      const { execFile } = await import("node:child_process"), { default: Ajv } = await import("ajv"), { writeFileSync } = await import("node:fs");
      const fixtureRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures");
      const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔎️build-inspection.json"), "utf8"));
      const outputPath = resolve(process.env.SEMIO_BUILD_INSPECTION_OUTPUT!);
      const ticketRoot = join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets");
      expect(relative(ticketRoot, outputPath).replace(/\\/g, "/")).toMatch(/^[^/]+\/[^/]+\/[^/]+\/[^/]+\/🗑️generated\/[^/]+\/[^/]+\.json$/u);
      expect(existsSync(outputPath)).toBe(false);
      const configPath = join(dirname(fileURLToPath(source.url)), "⚙️vite.config.ts");
      const primaryDist = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/dist");
      const snapshot = () => {
        const result: Record<string, string> = {};
        const visit = (directory: string) => {
          if (!existsSync(directory)) return;
          for (const entry of readdirSync(directory, { withFileTypes: true })) {
            const path = join(directory, entry.name);
            if (entry.isDirectory()) visit(path);
            else if (entry.isFile()) result[relative(primaryDist, path)] = createHash("sha256").update(readFileSync(path)).digest("hex");
          }
        };
        visit(primaryDist);
        return result;
      };
      const before = snapshot();
      let output: string;
      try {
        output = await new Promise<string>((resolveOutput, reject) => {
          const child = execFile("bun", ["--eval", `
            import { build } from 'vite';
            import { createHash } from 'node:crypto';
            import { pathToFileURL } from 'node:url';
            const settings = JSON.parse(process.argv[2]);
            console.error('[DEBUG] importing actual production configuration');
            const { default: config } = await import(pathToFileURL(process.argv[1]).href);
            console.error('[DEBUG] production configuration loaded; starting no-write Rollup build');
            const result = await build({ ...config, configFile: false, publicDir: settings.publicDir, cacheDir: process.argv[3], build: { ...config.build, write: settings.write, emptyOutDir: settings.emptyOutDir, outDir: process.argv[4] } });
            const bundles = Array.isArray(result) ? result : [result], rows = [];
            for (const bundle of bundles) for (const item of bundle.output) {
              const bytes = Buffer.from(item.type === 'chunk' ? item.code : item.source);
              const sha256 = createHash('sha256').update(bytes).digest('hex');
              if (sha256 !== Buffer.from(await crypto.subtle.digest('SHA-256', bytes)).toString('hex')) throw new Error('digest oracle mismatch');
              rows.push({ type: item.type, fileName: item.fileName, bytes: bytes.length, sha256, facadeModuleId: item.type === 'chunk' ? item.facadeModuleId : null, imports: item.type === 'chunk' ? item.imports : [], dynamicImports: item.type === 'chunk' ? item.dynamicImports : [], modules: item.type === 'chunk' ? Object.keys(item.modules).sort() : [], originalFileNames: item.type === 'asset' ? item.originalFileNames : [], names: item.type === 'asset' ? item.names : [] });
            }
            console.log('[BUILD_INSPECTION]' + JSON.stringify(rows));
          `, configPath, JSON.stringify(fixture.settings), join(dirname(outputPath), "vite-inspection-cache"), join(dirname(outputPath), "vite-inspection-unwritten")], { cwd: repoRoot, env: { ...process.env, SEMIO_PLUGIN: "s", SEMIO_RENDERER: "react", SEMIO_BRAND: "" }, timeout: fixture.settings.maximumDurationMs, killSignal: "SIGKILL", maxBuffer: 32 * 1024 * 1024 }, (error, stdout, stderr) => error ? reject(new Error(`production inspection failed: ${error.message}\n${stderr}`)) : resolveOutput(stdout));
          child.stderr?.on("data", (chunk) => { if (String(chunk).includes("[DEBUG]")) console.log(String(chunk).trim()); });
        });
      } finally {
        expect(snapshot()).toEqual(before);
      }
      const record = output.split("\n").find((line) => line.startsWith("[BUILD_INSPECTION]"));
      expect(record).toBeDefined();
      const rows = JSON.parse(record!.slice("[BUILD_INSPECTION]".length));
      const validate = await devContractModule("BuildInspectionV1");
      expect(validate(rows), JSON.stringify(validate.errors)).toBe(true);
      expect(new Set(rows.map((row: { fileName: string }) => row.fileName)).size).toBe(rows.length);
      expect(existsSync(join(dirname(outputPath), "vite-inspection-unwritten"))).toBe(false);
      writeFileSync(outputPath, JSON.stringify(rows, null, 2) + "\n", { flag: "wx" });
      console.log(`[DEBUG] inspected ${rows.length} actual Rollup outputs; preserved ${Object.keys(before).length} distribution files`);
    }, 210_000);

    itLong("excludes test-only Node imports from production output while preserving runtime and test-mode branches", async () => {
      const ts = await import("typescript"), { execFileSync } = await import("node:child_process");
      const fixturePath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🧹️production-tests.json");
      const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
      const config = ts.createSourceFile("vite.config.ts", readFileSync(join(dirname(fileURLToPath(source.url)), "⚙️vite.config.ts"), "utf8"), ts.ScriptTarget.Latest, true);
      let definition: string | undefined;
      const visit = (node: import("typescript").Node) => {
        if (ts.isPropertyAssignment(node) && ts.isStringLiteral(node.name) && node.name.text === "vitest" && ts.isStringLiteral(node.initializer)) definition = node.initializer.text;
        ts.forEachChild(node, visit);
      };
      visit(config);
      const actual = JSON.parse(execFileSync("node", ["--input-type=module", "--eval", `
        import { build } from 'vite';
        import { readFileSync } from 'node:fs';
        const fixture = JSON.parse(readFileSync(process.argv[1], 'utf8'));
        const definition = process.argv[2], results = [];
        for (const row of fixture.cases) {
          let nodeImports = 0;
          const selected = row.mode === 'production' ? definition : 'globalThis.__semioBuildTestMode';
          const result = await build({ configFile: false, publicDir: false, logLevel: 'silent', define: selected ? { 'vitest': selected } : {}, plugins: [{ name: 'neutral-test-boundary', enforce: 'pre', resolveId(id) { if (id === 'fixture-entry') return '\\0fixture-entry'; if (id === 'node:fs') { nodeImports++; return '\\0fixture-test-module'; } }, load(id) { if (id === '\\0fixture-entry') return fixture.source; if (id === '\\0fixture-test-module') return fixture.testModule; } }], build: { write: false, emptyOutDir: false, target: 'esnext', minify: false, rollupOptions: { input: 'fixture-entry', preserveEntrySignatures: 'strict', output: { format: 'es', inlineDynamicImports: true } } } });
          const code = result.output.find(item => item.type === 'chunk').code;
          globalThis.__semioBuildTestMode = row.mode === 'test';
          const module = await import('data:text/javascript;base64,' + Buffer.from(code).toString('base64') + '#' + row.mode);
          results.push({ mode: row.mode, nodeImports, runtimeWitness: module.runtimeWitness, testWitness: module.testWitness });
        }
        console.log(JSON.stringify(results));
      `, fixturePath, definition ?? ""], { cwd: repoRoot, encoding: "utf8", timeout: 30_000, maxBuffer: 1024 * 1024 }));
      expect(actual).toEqual(fixture.cases);
      expect(definition).toBe(fixture.productionDefinition);
      console.log("[DEBUG] production excludes Node test imports; runtime and test-mode witnesses remain executable");
    }, 40_000);

    itLong("routes encoded OS watcher and installation requests through the actual adapter handlers", async () => {
      const ts = await import("typescript"), { EventEmitter } = await import("node:events");
      const { URL: OracleURL } = await import("whatwg-url");
      const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🧪️cases.json"), "utf8"));
      const handlers = new Map<string, Function>(), watched: string[] = [];
      const environment = {
        mkdirSync() {}, watch(path: string) { watched.push(path); }, join, moduleRoutePath,
        PLUGIN_MODULES_ROOT, MODULE_HOT_SWAP_FILE, PLUGIN_SOURCE_WATCH_PATH,
        startSseKeepalive: () => () => {}, scanBuiltPluginModules: () => [{ pluginId: "puzzle", rebuiltAt: 1 }],
        EXTENSION_WATCH_MARKER: "👀️extension-watch.json", EXTENSION_WATCH_PATH: `${MODULE_EXTENSION_ROUTE}/watch`, EXTENSION_INSTALL_PATH: `${MODULE_EXTENSION_ROUTE}/install`,
        createExtensionStore: () => ({ installRoot: "fixture-install", listInstalled: async () => [], installFromBytes: async () => ({ installed: true }) }),
        readRequestBody: async () => Buffer.from([1]),
      };
      const specs = [
        { owner: "plugin", path: join(dirname(fileURLToPath(source.url)), "🔌️vite-plugins.ts"), name: "semioPluginHotSwapVitePlugin" },
        { owner: "extension", path: join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️store.ts"), name: "semioExtensionStoreVitePlugin" },
      ];
      for (const spec of specs) {
        const source = ts.createSourceFile(spec.path, readFileSync(spec.path, "utf8"), ts.ScriptTarget.Latest, true);
        const node = source.statements.find((entry) => ts.isFunctionDeclaration(entry) && entry.name?.text === spec.name);
        if (!node) throw new Error(`missing production function ${spec.name}`);
        const emitted = ts.transpileModule(node.getText(source), { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 } }).outputText;
        const create = new Function("exports", ...Object.keys(environment), emitted + `\nreturn exports.${spec.name};`)({}, ...Object.values(environment));
        create({ installRoot: "fixture-install", repoRoot, materializer: async () => ({}) }).configureServer({ middlewares: { use(handler: Function) { handlers.set(spec.owner, handler); } } });
      }
      expect(watched).toEqual([pluginOutRoot, "fixture-install"]);
      for (const row of fixture.httpCases) {
        const request = Object.assign(new EventEmitter(), { url: row.url, method: row.method }), chunks: string[] = [];
        let next = false;
        const response = { statusCode: 0, setHeader() {}, write(value: string) { chunks.push(value); }, end(value: string) { chunks.push(value); } };
        await handlers.get(row.owner)!(request, response, () => { next = true; });
        expect(next, row.url).toBe(row.expected === "next");
        if (row.expected !== "next") {
          expect(response.statusCode).toBe(200);
          expect(chunks.join("")).toContain(row.expected === "snapshot" ? '"kind":"snapshot"' : '"installed":true');
          expect(decodeURIComponent(new OracleURL(row.url, "https://example.invalid").pathname)).toBe(moduleRoutePath(row.url));
        }
        request.emit("close");
      }
      console.log(`[DEBUG] verified ${fixture.httpCases.length} encoded OS adapter request cases without filesystem mutation`);
    });

    itLong("serves every declared component import and tool-fixed vendor file byte-identically", async () => {
      const ts = await import("typescript"), { PassThrough } = await import("node:stream");
      const { isAbsolute } = await import("node:path");
      const styling = ts.createSourceFile("styling.ts", readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
      const sourceFunctions = ["contentTypeForStaticDirAsset", "createStaticDirMiddleware", "staticDirVitePlugin"].map((name) => {
        const node = styling.statements.find((entry) => ts.isFunctionDeclaration(entry) && entry.name?.text === name);
        if (!node) throw new Error(`missing production function ${name}`);
        return node.getText(styling);
      });
      const emitted = ts.transpileModule(sourceFunctions.join("\n"), { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 } }).outputText;
      const staticDirVitePlugin = new Function("exports", "resolve", "relative", "isAbsolute", "existsSync", "statSync", "createReadStream", "mkdirSync", "cpSync", "process", emitted + "\nreturn exports.staticDirVitePlugin;")({}, resolve, relative, isAbsolute, existsSync, statSync, createReadStream, mkdirSync, cpSync, process);
      const { loadCatalogTaxonomy, fixedFilenameContractIdsForPath } = await import("../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts");
      const taxonomy = loadCatalogTaxonomy(), fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🪞️vendor.json"), "utf8"));
      const vendorRoot = join(pluginOutRoot, fixture.vendorPath), urls = new Map<string, string>();
      expect(readdirSync(vendorRoot).sort()).toEqual(fixture.vendorFiles);
      for (const filename of fixture.vendorFiles) {
        const file = join(vendorRoot, filename), relativeFile = relative(repoRoot, file).replace(/\\/g, "/");
        const installed = readFileSync(join(repoRoot, "node_modules/@bytecodealliance/preview2-shim/dist/browser", filename));
        expect(readFileSync(file).equals(installed), filename).toBe(true);
        expect(fixedFilenameContractIdsForPath(relativeFile, taxonomy)).toEqual([`dev-vendor-${filename.slice(0, -3)}`]);
        for (const hostile of [relativeFile + ".bak", relativeFile.replace("🪞️vendor", "🪞️other"), relativeFile.replace("🪟️preview2-shim", "nested/🪟️preview2-shim")]) expect(fixedFilenameContractIdsForPath(hostile, taxonomy)).toEqual([]);
        urls.set(`${MODULE_PLUGIN_ROUTE}/${fixture.vendorPath}/${filename}`, file);
      }
      for (const spec of fixture.moduleRoots) {
        const root = join(dirname(pluginOutRoot), spec.directory);
        let components = 0, imports = 0;
        for (const directory of readdirSync(root, { withFileTypes: true }).filter((entry) => entry.isDirectory() && moduleIdForDirectoryName(entry.name))) {
          const files = readdirSync(join(root, directory.name)).filter((name) => name.endsWith("_component.js"));
          for (const filename of files) {
            components++;
            const source = ts.createSourceFile(filename, readFileSync(join(root, directory.name, filename), "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
            for (const node of source.statements) {
              if (!ts.isImportDeclaration(node) || !ts.isStringLiteral(node.moduleSpecifier) || !node.moduleSpecifier.text.includes("preview2-shim")) continue;
              imports++;
              const pathname = decodeURIComponent(new URL(node.moduleSpecifier.text, `https://example.invalid${spec.route}/${directory.name}/${filename}`).pathname);
              expect(pathname).toBe(`${MODULE_PLUGIN_ROUTE}/${fixture.vendorPath}/${basename(node.moduleSpecifier.text)}`);
              expect(urls.has(pathname)).toBe(true);
            }
          }
        }
        expect(components).toBe(spec.components);
        expect(imports).toBe(spec.imports);
      }
      urls.set(`${MODULE_PLUGIN_ROUTE}/${fixture.fontPath}`, join(pluginOutRoot, fixture.fontPath));
      urls.set(`${MODULE_PLUGIN_ROUTE}/${fixture.sharedWorker}`, join(pluginOutRoot, fixture.sharedWorker));
      let middleware: any;
      const serve = staticDirVitePlugin(repoRoot, { kind: "static-dir", route: MODULE_PLUGIN_ROUTE, root: relative(repoRoot, pluginOutRoot) })[0]!;
      (serve.configureServer as Function)({ middlewares: { use(value: unknown) { middleware = value; } } });
      for (const [url, file] of urls) {
        const response = new PassThrough(), chunks: Buffer[] = [];
        Object.assign(response, { setHeader() {} });
        const body = new Promise<Buffer>((resolveBody, reject) => {
          response.on("data", (chunk) => chunks.push(Buffer.from(chunk)));
          response.on("end", () => resolveBody(Buffer.concat(chunks)));
          response.on("error", reject);
          middleware({ url: new URL(url, "https://example.invalid").pathname }, response, () => reject(new Error(`unserved ${url}`)));
        });
        const actual = await body, expected = readFileSync(file);
        expect(actual.equals(expected), url).toBe(true);
        expect(Buffer.from(await crypto.subtle.digest("SHA-256", actual)).toString("hex")).toBe(createHash("sha256").update(expected).digest("hex"));
      }
      console.log(`[DEBUG] verified ${urls.size} live static assets and 345 component vendor imports`);
    }, 20_000);
  });

  describe("WASI codegen profile policy", () => {
    itLong("matches the strict neutral routing fixture and an independent selector", async () => {
      const { default: Ajv } = await import("ajv");
      const fixtureRoot = join(dirname(fileURLToPath(source.url)), "../../🧫️fixtures/🦀️wasm-profile-policy/🧬️v1");
      const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
      expect((await devContractModule("WasiProfilePolicyV1"))(fixture)).toBe(true);
      for (const vector of fixture.cases) {
        const independent = vector.override === null ? vector.mode === "dev" ? "wasm-dev" : "wasm-release" : fixture.runtimeDirectories.includes(vector.override) ? vector.override : null;
        expect(independent).toBe(vector.expectedProfile);
        if (vector.expectedProfile === null) {
          expect(() => pluginWasmProfile(vector.mode, vector.override)).toThrow();
          expect(() => pluginCargoArgs("semio-s-plugin-vcs", vector.override)).toThrow();
        }
        else {
          const profile = pluginWasmProfile(vector.mode, vector.override);
          expect(profile).toBe(vector.expectedProfile);
          expect(cargoProfileDir(profile)).toBe(vector.expectedDirectory);
          expect(pluginCargoArgs("semio-s-plugin-vcs", profile)).toEqual(["rustc", "-p", "semio-s-plugin-vcs", "--target", "wasm32-wasip2", "--profile", profile, "--", "-C", "link-arg=-zstack-size=8388608"]);
        }
      }
    });

    itLong("keeps single-CGU mitigation WASI-only and preserves release publication settings", async () => {
      const { default: toml } = await import("@iarna/toml");
      const manifest = toml.parse(readFileSync(join(repoRoot, "Cargo.toml"), "utf8")) as any;
      expect(manifest.profile.dev["codegen-units"]).toBeUndefined();
      for (const override of Object.values(manifest.profile.dev.package ?? {})) expect((override as any)["codegen-units"]).toBeUndefined();
      expect(manifest.profile["wasm-dev"]).toEqual({ inherits: "dev", "codegen-units": 1 });
      expect(manifest.profile["wasm-release"]).toMatchObject({ inherits: "release", "opt-level": "s", lto: "thin", "codegen-units": 1, strip: "symbols", incremental: false, "trim-paths": "object" });
    });
  });

  describe("pluginCargoArgs", () => {
    it("links every actor component with bounded headroom for descriptor and app assembly", () => {
      expect(pluginCargoArgs("semio-s-plugin-procedural", "wasm-release")).toEqual([
        "rustc",
        "-p",
        "semio-s-plugin-procedural",
        "--target",
        "wasm32-wasip2",
        "--profile",
        "wasm-release",
        "--",
        "-C",
        "link-arg=-zstack-size=8388608",
      ]);
    });

    it("can retain actor symbols for a reproducible browser trap diagnosis", () => {
      process.env.SEMIO_PLUGIN_SYMBOLS = "1";
      try {
        expect(pluginCargoArgs("semio-s-plugin-procedural", "wasm-release").slice(-2)).toEqual(["-C", "strip=none"]);
      } finally {
        delete process.env.SEMIO_PLUGIN_SYMBOLS;
      }
    });
  });

  describe("createConcurrencyLimiter (T-P8 bounded-parallel materialize primitive)", () => {
    it("never runs more than `limit` callbacks concurrently, and still runs every one to completion", async () => {
      const limiter = createConcurrencyLimiter(2);
      let active = 0;
      let maxActive = 0;
      const tasks = Array.from({ length: 6 }, (_, i) =>
        limiter.run(async () => {
          active++;
          maxActive = Math.max(maxActive, active);
          await new Promise((r) => setTimeout(r, 5));
          active--;
          return i;
        }),
      );
      const results = await Promise.all(tasks);
      expect(results.slice().sort((a, b) => a - b)).toEqual([0, 1, 2, 3, 4, 5]);
      expect(maxActive).toBeLessThanOrEqual(2);
      expect(maxActive).toBe(2); // proves it actually overlaps, not just an accidental serialization
    });
  });

  describe("buildPluginCatalog (T-P8: cargo stage serial, materialize stage bounded-parallel)", () => {
    it("rejects incomplete explicit builds instead of accepting stale plugin artifacts", () => {
      expect(() => assertPluginCatalogComplete([])).not.toThrow();
      expect(() => assertPluginCatalogComplete(["cargo-fails", "materialize-fails"])).toThrow("plugin catalog build failed: cargo-fails, materialize-fails");
    });

    const fakeTarget = (pluginId: string): PluginRegistryEntry => ({
      pluginId,
      cratePath: "",
      packageName: pluginId,
      wasmOut: `${pluginId}.wasm`,
      role: "plugin",
      capabilities: [],
      contributes: [],
      consumes: [],
      dependsOn: [],
      activationEvents: [],
      extensionPoints: [],
    });

    it("never overlaps two cargo calls, overlaps materialize up to the cap, and emits every plugin", async () => {
      const targets = Array.from({ length: 6 }, (_, i) => fakeTarget(`p${i}`));
      let cargoActive = 0;
      let maxCargoActive = 0;
      let materializeActive = 0;
      let maxMaterializeActive = 0;
      const materialized: string[] = [];
      const cargoFn = async (target: PluginRegistryEntry) => {
        cargoActive++;
        maxCargoActive = Math.max(maxCargoActive, cargoActive);
        await new Promise((r) => setTimeout(r, 5));
        cargoActive--;
        return { artifact: `${target.pluginId}.wasm` };
      };
      const materializeFn = async (target: PluginRegistryEntry) => {
        materializeActive++;
        maxMaterializeActive = Math.max(maxMaterializeActive, materializeActive);
        await new Promise((r) => setTimeout(r, 15));
        materializeActive--;
        materialized.push(target.pluginId);
      };
      let shardWorkerPublishCount = 0;
      const { failedPluginIds } = await buildPluginCatalog(targets, cargoFn, materializeFn, 3, () => {
        shardWorkerPublishCount++;
      });
      expect(failedPluginIds).toEqual([]);
      expect(materialized.slice().sort()).toEqual(targets.map((t) => t.pluginId).sort());
      expect(maxCargoActive).toBe(1); // cargo NEVER overlaps itself
      expect(maxMaterializeActive).toBeGreaterThan(1); // materialize DOES overlap
      expect(maxMaterializeActive).toBeLessThanOrEqual(3); // never past the cap
      expect(shardWorkerPublishCount).toBe(1); // once per catalog build, not once per plugin
    });

    it("continues past both a cargo failure and a materialize failure, reporting each pluginId exactly once", async () => {
      const targets = [fakeTarget("ok"), fakeTarget("cargo-fails"), fakeTarget("materialize-fails")];
      const cargoFn = async (target: PluginRegistryEntry) => {
        if (target.pluginId === "cargo-fails") throw new Error("boom");
        return { artifact: `${target.pluginId}.wasm` };
      };
      const materialized: string[] = [];
      const materializeFn = async (target: PluginRegistryEntry) => {
        if (target.pluginId === "materialize-fails") throw new Error("boom");
        materialized.push(target.pluginId);
      };
      const { failedPluginIds } = await buildPluginCatalog(targets, cargoFn, materializeFn, 4, () => {});
      expect(new Set(failedPluginIds)).toEqual(new Set(["cargo-fails", "materialize-fails"]));
      expect(materialized).toEqual(["ok"]);
    });
  });
  //#endregion 🔖️T-P8-tests

  //#region 🔖️T-P8-sqlite-handle-cache-tests
  describe("backboneDbHandleFor (T-P8 per-path sqlite handle cache)", () => {
    let root: string;

    beforeEach(() => {
      root = mkdtempSync(join(tmpdir(), "semio-backbone-db-cache-"));
    });

    afterEach(() => {
      rmSync(root, { recursive: true, force: true });
    });

    it("returns the SAME handle for the same path across repeated calls", async () => {
      const dbPath = join(root, "a", "documents.db");
      mkdirSync(dirname(dbPath), { recursive: true });
      const first = await backboneDbHandleFor(dbPath);
      const second = await backboneDbHandleFor(dbPath);
      expect(second).toBe(first);
    });

    it("returns DISTINCT handles for distinct paths", async () => {
      const dbPathA = join(root, "a", "documents.db");
      const dbPathB = join(root, "b", "documents.db");
      mkdirSync(dirname(dbPathA), { recursive: true });
      mkdirSync(dirname(dbPathB), { recursive: true });
      const a = await backboneDbHandleFor(dbPathA);
      const b = await backboneDbHandleFor(dbPathB);
      expect(a).not.toBe(b);
    });
  });
  //#endregion 🔖️T-P8-sqlite-handle-cache-tests

  //#region 🔖️T-P8-extension-sweep-tests
  describe("deployment output preservation", () => {
    let root: string;

    beforeEach(() => {
      root = mkdtempSync(join(tmpdir(), "semio-extension-sweep-"));
    });

    afterEach(() => {
      rmSync(root, { recursive: true, force: true });
    });

    it("reports a retained worker without deleting it", () => {
      const dir = join(root, "📝️flow-extension-text");
      mkdirSync(dir, { recursive: true });
      const staleWorker = join(dir, "🧵️plugin-worker.js");
      writeFileSync(staleWorker, "/** pre-H2 leftover */");
      expect(() => assertExtensionOutputsFresh(root)).toThrow(/retained worker/);
      expect(readFileSync(staleWorker, "utf8")).toBe("/** pre-H2 leftover */");
    });

    it("reports a stale host shim without deleting it", () => {
      const dir = join(root, "📝️flow-extension-text");
      mkdirSync(dir, { recursive: true });
      const staleShim = join(dir, PLUGIN_HOST_SHIM_FILE);
      writeFileSync(staleShim, "/** @generated semio plugin host shim */\nexport function readDocument(handle) { throw `unsupported: ${handle}`; }\n");
      expect(() => assertExtensionOutputsFresh(root)).toThrow(/stale host shim/);
      expect(readFileSync(staleShim, "utf8")).toContain("export function readDocument");
    });

    it("keeps a 🟨️.js whose content already matches the current hostShimSource()", () => {
      const dir = join(root, "🗒️note");
      mkdirSync(dir, { recursive: true });
      const freshShim = join(dir, PLUGIN_HOST_SHIM_FILE);
      writeFileSync(freshShim, hostShimSource());
      assertExtensionOutputsFresh(root);
      expect(existsSync(freshShim)).toBe(true);
      expect(readFileSync(freshShim, "utf8")).toBe(hostShimSource());
    });

    it("is a no-op against a missing root", () => {
      expect(() => assertExtensionOutputsFresh(join(root, "does-not-exist"))).not.toThrow();
    });

    it("preserves an unexpected public output tree and accepts its absence", async () => {
      const { webcrypto } = await import("node:crypto");
      const specimen = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🛡️deployment-preservation.json"), "utf8")).stalePublicOutput;
      const directory = join(root, specimen.directory), file = join(directory, specimen.filename);
      expect(() => assertNoStalePublicPluginOutputs(directory)).not.toThrow();
      mkdirSync(directory, { recursive: true });
      writeFileSync(file, specimen.content);
      expect(() => assertNoStalePublicPluginOutputs(directory)).toThrow(/preserved/);
      expect(createHash("sha256").update(readFileSync(file)).digest("hex")).toBe(Buffer.from(await webcrypto.subtle.digest("SHA-256", new TextEncoder().encode(specimen.content))).toString("hex"));
    });

    it("preserves every neutral hostile specimen byte for byte", async () => {
      const { webcrypto } = await import("node:crypto");
      const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🛡️deployment-preservation.json"), "utf8"));
      for (const row of fixture.cases) {
        const parent = join(root, row.name), dir = join(parent, "📝️flow-extension-text");
        mkdirSync(dir, { recursive: true });
        const path = join(dir, row.filename);
        writeFileSync(path, row.content);
        const expected = Buffer.from(await webcrypto.subtle.digest("SHA-256", new TextEncoder().encode(row.content))).toString("hex");
        if (row.filename !== PLUGIN_HOST_SHIM_FILE) expect(() => assertPluginOutputChildren(dir, "fixture_component")).toThrow(/preserved/);
        if (row.extensionRejected) expect(() => assertExtensionOutputsFresh(parent)).toThrow(/preserved/);
        else expect(() => assertExtensionOutputsFresh(parent)).not.toThrow();
        expect(createHash("sha256").update(readFileSync(path)).digest("hex")).toBe(expected);
      }
    });
  });
  //#endregion 🔖️T-P8-extension-sweep-tests

  //#region 🔖️PollHelpers-tests
  describe("awaitTcpReady (W6 poll-census helper)", () => {
    it("honours its deadline and reports timeout — no real sleeps: fake clock + fake probe", async () => {
      let fakeNow = 0;
      const now = () => fakeNow;
      const sleep = async (ms: number) => {
        fakeNow += ms;
      };
      const outcome = await awaitTcpReady("127.0.0.1", 9999, {
        deadlineMs: 1000,
        intervalMs: 250,
        probe: () => false,
        now,
        sleep,
      });
      expect(outcome).toBe("timeout");
      expect(fakeNow).toBeGreaterThanOrEqual(1000);
    });

    it("resolves ready as soon as the injected probe reports the port open", async () => {
      let calls = 0;
      const outcome = await awaitTcpReady("127.0.0.1", 9999, {
        deadlineMs: 10_000,
        intervalMs: 250,
        probe: () => {
          calls++;
          return calls >= 3;
        },
        now: () => 0,
        sleep: async () => {},
      });
      expect(outcome).toBe("ready");
      expect(calls).toBe(3);
    });

    it("resolves closed-mode ready once the injected probe reports the port free", async () => {
      let calls = 0;
      const outcome = await awaitTcpReady("127.0.0.1", 9999, {
        deadlineMs: 10_000,
        intervalMs: 250,
        mode: "closed",
        probe: () => {
          calls++;
          return calls < 2; // "in use" for the first call, "free" from the second on
        },
        now: () => 0,
        sleep: async () => {},
      });
      expect(outcome).toBe("ready");
      expect(calls).toBe(2);
    });

    it("resolves dead as soon as isDead() reports true, before the deadline", async () => {
      let fakeNow = 0;
      const outcome = await awaitTcpReady("127.0.0.1", 9999, {
        deadlineMs: 10_000,
        intervalMs: 250,
        probe: () => false,
        isDead: () => true,
        now: () => fakeNow,
        sleep: async (ms) => {
          fakeNow += ms;
        },
      });
      expect(outcome).toBe("dead");
      expect(fakeNow).toBe(0); // died on the very first check, before any sleep
    });
  });

  describe("awaitHttpOk (W6 poll-census helper)", () => {
    it("honours its deadline and reports timeout — no real sleeps: fake clock + always-throwing fetch", async () => {
      let fakeNow = 0;
      const outcome = await awaitHttpOk("http://127.0.0.1:9999/admin/api/overview", {
        deadlineMs: 1000,
        intervalMs: 500,
        fetchImpl: (async () => {
          throw new Error("ECONNREFUSED");
        }) as unknown as typeof fetch,
        now: () => fakeNow,
        sleep: async (ms) => {
          fakeNow += ms;
        },
      });
      expect(outcome).toBe("timeout");
    });

    it("resolves ready once the injected fetch stops throwing", async () => {
      let calls = 0;
      const outcome = await awaitHttpOk("http://127.0.0.1:9999/admin/api/overview", {
        deadlineMs: 10_000,
        intervalMs: 500,
        fetchImpl: (async () => {
          calls++;
          if (calls < 2) throw new Error("ECONNREFUSED");
          return {} as unknown as Response; // value is irrelevant — awaitHttpOk only cares that fetch resolved
        }) as unknown as typeof fetch,
        now: () => 0,
        sleep: async () => {},
      });
      expect(outcome).toBe("ready");
      expect(calls).toBe(2);
    });

    it("resolves dead as soon as isDead() reports true, before attempting to fetch", async () => {
      let fetchCalled = false;
      const outcome = await awaitHttpOk("http://127.0.0.1:9999/admin/api/overview", {
        deadlineMs: 10_000,
        intervalMs: 500,
        isDead: () => true,
        fetchImpl: (async () => {
          fetchCalled = true;
          return {} as unknown as Response; // value is irrelevant — awaitHttpOk only cares that fetch resolved
        }) as unknown as typeof fetch,
        now: () => 0,
        sleep: async () => {},
      });
      expect(outcome).toBe("dead");
      expect(fetchCalled).toBe(false);
    });
  });

  describe("awaitChildExit (W6 event-driven fix — replaces polling child.exitCode)", () => {
    it("resolves as soon as the child's own 'exit' event fires, without polling", async () => {
      const fakeChild = new EventEmitter() as unknown as SpawnDaemonHandle["child"];
      Object.assign(fakeChild, { exitCode: null });
      // 🧵️ `timeoutAfter` deliberately never resolves — if `awaitChildExit` secretly depended on a
      // timer/poll to notice the exit (instead of the 'exit' event alone), this promise would never
      // settle and the `await` below would hang until vitest's own test timeout fails the case.
      const resultPromise = awaitChildExit(fakeChild, 30_000, { timeoutAfter: () => new Promise<"timeout">(() => {}) });
      // 🧵️ Simulate Node setting exitCode then emitting 'exit', exactly as a real ChildProcess does.
      (fakeChild as unknown as { exitCode: number | null }).exitCode = 0;
      (fakeChild as unknown as EventEmitter).emit("exit", 0, null);
      const result = await resultPromise;
      expect(result).toBe("exited");
    });

    it("resolves immediately for a child that had already exited before the call", async () => {
      const fakeChild = new EventEmitter() as unknown as SpawnDaemonHandle["child"];
      Object.assign(fakeChild, { exitCode: 0 });
      // Same never-resolving `timeoutAfter` as above: only the already-set `exitCode` can win this race.
      const result = await awaitChildExit(fakeChild, 30_000, { timeoutAfter: () => new Promise<"timeout">(() => {}) });
      expect(result).toBe("exited");
    });

    it("still times out for a hung child that never emits 'exit' — fake deadline, no real sleep", async () => {
      const fakeChild = new EventEmitter() as unknown as SpawnDaemonHandle["child"];
      Object.assign(fakeChild, { exitCode: null });
      const result = await awaitChildExit(fakeChild, 30_000, {
        timeoutAfter: async () => "timeout" as const, // resolves instantly, standing in for "deadline reached"
      });
      expect(result).toBe("timeout");
    });
  });
  //#endregion 🔖️PollHelpers-tests

  //#region 🔖️CatalogSmoke-tests
  describe("catalog smoke aggregation", () => {
    const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🔬️catalog-smoke.json"), "utf8"));

    it("folds program outcomes and install statuses into the committed language-agnostic report", () => {
      const report = summarizeCatalogSmoke(fixture.input);
      expect(report).toEqual(fixture.report);
      expect(catalogSmokeExitCode(report)).toBe(fixture.exitCode);
      expect(catalogSmokeMarkdown(report)).toBe(fixture.markdown);
    });

    it("matches the committed report schema under an independent validator", async () => {
      const { default: Ajv } = await import("ajv");
      const validate = await devContractModule("CatalogSmokeReportV1");
      expect(validate(fixture.report), JSON.stringify(validate.errors)).toBe(true);
      expect(validate({ ...fixture.report, totals: { pass: 1, fail: 1 } })).toBe(false);
      expect(validate({ ...fixture.report, programs: [{ ...fixture.report.programs[0], status: "unknown" }] })).toBe(false);
    });

    it("exits non-zero for a failed program, a failed plugin, or an empty catalog, and zero only for an all-pass run", () => {
      const green = summarizeCatalogSmoke({ ...fixture.input, programs: [fixture.input.programs[0]], plugins: [{ pluginId: "draw", status: "loaded" }] });
      expect(catalogSmokeExitCode(green)).toBe(0);
      expect(catalogSmokeExitCode(summarizeCatalogSmoke({ ...fixture.input, programs: [fixture.input.programs[1]], plugins: [] }))).toBe(1);
      expect(catalogSmokeExitCode(summarizeCatalogSmoke({ ...fixture.input, programs: [fixture.input.programs[0]], plugins: [{ pluginId: "stdio", status: "crashed" }] }))).toBe(1);
      expect(catalogSmokeExitCode(summarizeCatalogSmoke({ ...fixture.input, programs: [], plugins: [] }))).toBe(1);
    });

    it("fails and reports the shell's own boot diagnostics when no program could ever be spawned", () => {
      const boot = { beacon: "error:s", failure: "shell never reached a ready beacon (error:s)", phaseMs: { commit: 210, firstModule: 260, beacon: 31000 }, bodyExcerpt: "no plugins loaded", consoleErrors: ["[os-shell] plugin install/reload failed", "boom"] };
      const report = summarizeCatalogSmoke({ ...fixture.input, boot, programs: [], plugins: [] });
      expect(report.totals).toEqual({ pass: 0, fail: 0, skipped: 0 });
      expect(catalogSmokeExitCode(report)).toBe(1);
      const markdown = catalogSmokeMarkdown(report);
      expect(markdown).toContain("FAILED: shell never reached a ready beacon (error:s)");
      expect(markdown).toContain("commit 210 ms, first module 260 ms, beacon 31000 ms");
      expect(markdown).toContain("no plugins loaded");
      expect(markdown).toContain("- [os-shell] plugin install/reload failed");
    });

    it("bounds the boot diagnostics it carries instead of embedding a whole session log", () => {
      const boot = { beacon: null, failure: "timeout", phaseMs: { commit: 190, firstModule: null, beacon: null }, bodyExcerpt: "x".repeat(5000), consoleErrors: Array.from({ length: 100 }, (_, index) => `error ${index}`) };
      const report = summarizeCatalogSmoke({ ...fixture.input, boot, programs: [], plugins: [] });
      expect(report.boot.bodyExcerpt).toHaveLength(1000);
      expect(report.boot.consoleErrors).toHaveLength(20);
    });

    it("escapes table-breaking cell content instead of corrupting the markdown row", () => {
      const report = summarizeCatalogSmoke({ ...fixture.input, programs: [{ ...fixture.input.programs[0], firstError: "boom | on\nline two" }], plugins: [] });
      const row = catalogSmokeMarkdown(report).split("\n").find((line) => line.startsWith("| draw "))!;
      expect(row.replaceAll("\\|", "").split("|")).toHaveLength(9);
      expect(row).toContain("boom \\| on line two");
    });
  });
  //#endregion 🔖️CatalogSmoke-tests

}
