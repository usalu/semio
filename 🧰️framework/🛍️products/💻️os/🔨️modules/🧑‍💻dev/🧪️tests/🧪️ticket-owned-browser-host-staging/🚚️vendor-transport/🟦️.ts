/** 🚚️ The deployed vendor transport cases of the ticket-owned browser host staging suite. */
import { runtimeCatalogBuildInventoryV1 } from "../../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import { pathEmojiStatuteFindings } from "@semio-tech/framework/identity/path";
import { createTestKit, type ContractValidator, type TestDependencies, type TestSource } from "../🧰️test-kit/🟦️.ts";

export async function registerVendorTransportTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: TestDependencies, source: TestSource): Promise<void> {
  const { DISTRIBUTION_LAYOUT, MODULE_EXTENSION_ROUTE, MODULE_PLUGIN_ROUTE, basename, checkDistributionBundle, cpSync, createHash, createReadStream, dirname, distributionFileWitness, distributionPathOrder, distributionStaticSourcePaths, existsSync, fileURLToPath, join, mkdirSync, mkdtempSync, defaultExtensionInstallRoot, patchPreview2ShimGuestLogClassification, patchPreview2ShimGuestLogLineRelease, moduleIdForDirectoryName, moduleRoutePath, parseDistributionManifest, parseDistributionStaticInputs, pluginOutRoot, publishDistributionBundle, readFileSync, readdirSync, relative, repoRoot, resolve, rmSync, statSync, tmpdir, unlinkSync, writeFileSync } = dependencies;
  const { describe, expect, it } = vitest;
  const { contractModuleExport, devContractModule, itLong } = createTestKit(vitest, dependencies, source);

  const devDistributionContractModule = (exportId: string): Promise<ContractValidator> => contractModuleExport("../../🚚️distribution/🧬️schema/🔣️.json", exportId);
  const ownedRelativeFiles = (root: string): string[] => {
    const files: string[] = [];
    const visit = (directory: string): void => {
      for (const entry of readdirSync(directory, { withFileTypes: true })) {
        const path = join(directory, entry.name);
        if (entry.isDirectory()) visit(path);
        else if (entry.isFile()) files.push(relative(root, path).replaceAll("\\", "/"));
      }
    };
    visit(root);
    return files.sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
  };

  //#region 🔖️T-P8-tests
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
      const { loadTaxonomy } = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts");
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
      const plan = { manifest: fixture.manifest, files: new Map<string, Uint8Array>(fixture.manifest.outputs.map((row: { path: string }): readonly [string, Uint8Array] => [row.path, Buffer.from(fixture.text)])) };
      const before = () => Object.fromEntries(ownedRelativeFiles(destination).map(path => [path, createHash("sha256").update(readFileSync(join(destination, path))).digest("hex")]));
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
      const next = { manifest: { ...plan.manifest, outputs: [plan.manifest.outputs[0], { ...current, path: newPath }] }, files: new Map<string, Uint8Array>([[layout.entry.output, Buffer.from(fixture.text)], [newPath, Buffer.from(fixture.text)]]) };
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
      expect((await glob("**/*", { cwd: destination, onlyFiles: true, dot: true })).sort()).toEqual(ownedRelativeFiles(destination));
    });

    itLong("validates exact distribution owner manifests against a neutral digest oracle", async () => {
      const authority = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution");
      const fixture = JSON.parse(readFileSync(join(authority, "🔬️manifest-cases.json"), "utf8"));
      const { layout } = JSON.parse(readFileSync(join(authority, "🧪️cases.json"), "utf8"));
      const validate = await devDistributionContractModule("DistributionManifest");
      expect(validate(fixture.manifest), JSON.stringify(validate.errors)).toBe(true);
      expect(createHash("sha256").update(fixture.text).digest("hex")).toBe(fixture.sha256);
      expect(Buffer.from(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(fixture.text))).toString("hex")).toBe(fixture.sha256);
      const { parseDistributionManifest } = await import("../../../🚚️distribution/🟦️.ts");
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
      const { parseDistributionLayout, distributionChunkName, distributionAssetName } = await import("../../../🚚️distribution/🟦️.ts");
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
    });

    itLong("emits runtime URL assets but no dead in-source-test assets in production", async () => {
      const ts = await import("typescript"), { execFileSync } = await import("node:child_process");
      const fixturePath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/🧹️production-tests.json");
      const pluginSource = ts.createSourceFile("🟦️.ts", readFileSync(join(dirname(fileURLToPath(source.url)), "../../🔌️vite-plugins/🚧️production-test-boundary/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
      const node = pluginSource.statements.find((item) => ts.isFunctionDeclaration(item) && item.name?.text === "semioProductionTestBoundaryVitePlugin");
      const emitted = node ? ts.transpileModule(node.getText(pluginSource).replace("export function", "function"), { compilerOptions: { module: ts.ModuleKind.ESNext, target: ts.ScriptTarget.ES2022 } }).outputText : "";
      const config = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts"), "utf8");
      const active = config.includes("semioProductionTestBoundaryVitePlugin(),");
      const result = JSON.parse(execFileSync("node", ["--input-type=module", "--eval", `
        import { build } from 'vite';
        import { readFileSync } from 'node:fs';
        import { dirname, join, basename } from 'node:path';
        const fixture = JSON.parse(readFileSync(process.argv[1], 'utf8')), rows = [];
        ${emitted}
        for (const row of fixture.assetCases) {
          const id = join(dirname(process.argv[1]), 'neutral-entry.js');
          const result = await build({ configFile: false, root: dirname(process.argv[1]), publicDir: false, logLevel: 'silent', define: { 'import.meta.vitest': row.mode === 'production' ? 'undefined' : 'globalThis.__semioBuildTestMode' }, plugins: [${node && active ? "...(row.mode === 'production' ? [semioProductionTestBoundaryVitePlugin()] : [])," : ""}{ name: 'neutral-asset-entry', resolveId(value) { if (value === id) return id; }, load(value) { if (value === id) return fixture.assetSource; } }], build: { write: false, emptyOutDir: false, target: 'esnext', minify: false, assetsInlineLimit: 0, rollupOptions: { input: id, preserveEntrySignatures: 'strict', output: { format: 'es' } } } });
          const chunk = result.output.find(item => item.type === 'chunk');
          globalThis.__semioBuildTestMode = row.mode === 'test';
          const code = chunk.code.replaceAll('import.meta.url', JSON.stringify('https://example.invalid/app.js'));
          const runtime = await import('data:text/javascript;base64,' + Buffer.from(code).toString('base64') + '#' + row.mode);
          const assets = result.output.filter(item => item.type === 'asset');
          if (!assets.some(item => decodeURIComponent(new URL(runtime.runtimeUrl).pathname) === '/' + item.fileName)) throw new Error('runtime asset URL missing');
          if (runtime.testUrl && !assets.some(item => decodeURIComponent(new URL(runtime.testUrl).pathname) === '/' + item.fileName)) throw new Error('test asset URL missing');
          rows.push({ mode: row.mode, assets: assets.flatMap(item => item.originalFileNames.map(name => basename(name))).sort(), testUrl: runtime.testUrl !== null });
        }
        console.log(JSON.stringify(rows));
      `, fixturePath], { cwd: repoRoot, encoding: "utf8", timeout: 30_000, maxBuffer: 1024 * 1024 }));
      expect(result).toEqual(JSON.parse(readFileSync(fixturePath, "utf8")).assetCases);
    }, 40_000);

    itLong("validates neutral read-only build inspection records against an independent digest oracle", async () => {
      const { default: Ajv } = await import("ajv");
      const fixtureRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures");
      const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔎️build-inspection.json"), "utf8"));
      const validate = await devContractModule("BuildInspectionV1");
      expect(validate(fixture.expected)).toBe(true);
      for (const rejected of fixture.rejected) expect(validate(rejected)).toBe(false);
      expect(fixture.settings).toEqual({ write: false, emptyOutDir: false, publicDir: false, maximumDurationMs: 240_000 });
      expect(Buffer.byteLength(fixture.specimen)).toBe(fixture.expected[0].bytes);
      expect(createHash("sha256").update(fixture.specimen).digest("hex")).toBe(fixture.expected[0].sha256);
      expect(Buffer.from(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(fixture.specimen))).toString("hex")).toBe(fixture.expected[0].sha256);
    });

    it.runIf(Boolean(process.env.SEMIO_BUILD_INSPECTION_OUTPUT))("inspects the actual production Rollup output without writing or replacing distribution bytes", async () => {
      const { execFile } = await import("node:child_process"), { default: Ajv } = await import("ajv"), { writeFileSync } = await import("node:fs");
      const fixtureRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures");
      const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔎️build-inspection.json"), "utf8"));
      const boundary = JSON.parse(readFileSync(join(fixtureRoot, "🧹️production-tests.json"), "utf8"));
      const outputPath = resolve(process.env.SEMIO_BUILD_INSPECTION_OUTPUT!);
      const ticketRoot = join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets");
      expect(relative(ticketRoot, outputPath).replace(/\\/g, "/")).toMatch(/^[^/]+\/[^/]+\/[^/]+\/[^/]+\/🗑️generated\/[^/]+\/[^/]+\.json$/u);
      expect(existsSync(outputPath)).toBe(false);
      const configPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts");
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
            const { default: configExport } = await import(pathToFileURL(process.argv[1]).href);
            const config = typeof configExport === 'function' ? await configExport({ command: 'build', mode: 'production', isSsrBuild: false, isPreview: false }) : await configExport;
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
          child.stderr?.on("data", (chunk) => { if (String(chunk).includes("[TRACE]")) console.log(String(chunk).trim()); });
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
      const moduleIdentities = rows.flatMap((row: { facadeModuleId: string | null; imports: string[]; dynamicImports: string[]; modules: string[] }) => [row.facadeModuleId, ...row.imports, ...row.dynamicImports, ...row.modules]).filter((value: string | null): value is string => value !== null).map((value: string) => value.replaceAll("\\", "/"));
      expect(moduleIdentities.flatMap((identity: string) => boundary.deniedProductionModuleFragments.filter((fragment: string) => identity.includes(fragment)).map((fragment: string) => ({ identity, fragment })))).toEqual([]);
      expect(existsSync(join(dirname(outputPath), "vite-inspection-unwritten"))).toBe(false);
      writeFileSync(outputPath, JSON.stringify(rows, null, 2) + "\n", { flag: "wx" });
    }, 270_000);

    itLong("excludes test-only Node imports from production output while preserving runtime and test-mode branches", async () => {
      const ts = await import("typescript"), { execFileSync } = await import("node:child_process");
      const fixturePath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/🧹️production-tests.json");
      const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
      const config = ts.createSourceFile("🟦️.ts", readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
      let definition: string | undefined;
      const visit = (node: import("typescript").Node) => {
        if (ts.isPropertyAssignment(node) && ts.isStringLiteral(node.name) && node.name.text === "import.meta.vitest" && ts.isStringLiteral(node.initializer)) definition = node.initializer.text;
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
          const result = await build({ configFile: false, publicDir: false, logLevel: 'silent', define: selected ? { 'import.meta.vitest': selected } : {}, plugins: [{ name: 'neutral-test-boundary', enforce: 'pre', resolveId(id) { if (id === 'fixture-entry') return '\\0fixture-entry'; if (id === 'node:fs') { nodeImports++; return '\\0fixture-test-module'; } }, load(id) { if (id === '\\0fixture-entry') return fixture.source; if (id === '\\0fixture-test-module') return fixture.testModule; } }], build: { write: false, emptyOutDir: false, target: 'esnext', minify: false, rollupOptions: { input: 'fixture-entry', preserveEntrySignatures: 'strict', output: { format: 'es', inlineDynamicImports: true } } } });
          const code = result.output.find(item => item.type === 'chunk').code;
          globalThis.__semioBuildTestMode = row.mode === 'test';
          const module = await import('data:text/javascript;base64,' + Buffer.from(code).toString('base64') + '#' + row.mode);
          results.push({ mode: row.mode, nodeImports, runtimeWitness: module.runtimeWitness, testWitness: module.testWitness });
        }
        console.log(JSON.stringify(results));
      `, fixturePath, definition ?? ""], { cwd: repoRoot, encoding: "utf8", timeout: 30_000, maxBuffer: 1024 * 1024 }));
      expect(actual).toEqual(fixture.cases);
      expect(definition).toBe(fixture.productionDefinition);
    }, 40_000);

    itLong("routes encoded OS installation requests through the actual adapter handler and registers its stream route", async () => {
      const ts = await import("typescript"), { EventEmitter } = await import("node:events");
      const { URL: OracleURL } = await import("whatwg-url");
      const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧫️fixtures/📦️deployment/🧪️cases.json"), "utf8"));
      const handlers = new Map<string, Function>(), watched: string[] = [];
      const routes: string[] = [];
      const environment = {
        mkdirSync() {}, watch(path: string) { watched.push(path); }, join, moduleRoutePath,
        devStreamMuxServer: () => ({ route: (name: string) => { routes.push(name); return () => undefined; }, publish() {} }), DEV_STREAM_ROUTES: { extensionModules: "extension-modules.watch" },
        EXTENSION_WATCH_MARKER: "👀️extension-watch.json", EXTENSION_INSTALL_PATH: `${MODULE_EXTENSION_ROUTE}/install`,
        createExtensionStore: () => ({ installRoot: "fixture-install", listInstalled: async () => [], installFromBytes: async () => ({ installed: true }) }),
        readRequestBody: async () => Buffer.from([1]),
      };
      const specs = [
        { owner: "extension", path: join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts"), name: "semioExtensionStoreVitePlugin" },
      ];
      for (const spec of specs) {
        const source = ts.createSourceFile(spec.path, readFileSync(spec.path, "utf8"), ts.ScriptTarget.Latest, true);
        const node = source.statements.find((entry) => ts.isFunctionDeclaration(entry) && entry.name?.text === spec.name);
        if (!node) throw new Error(`missing production function ${spec.name}`);
        const emitted = ts.transpileModule(node.getText(source), { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 } }).outputText;
        const create = new Function("exports", ...Object.keys(environment), emitted + `\nreturn exports.${spec.name};`)({}, ...Object.values(environment));
        create({ moduleRoot: pluginOutRoot, installRoot: "fixture-install", repoRoot, materializer: async () => ({}) }).configureServer({ middlewares: { use(handler: Function) { handlers.set(spec.owner, handler); } } });
      }
      expect(watched).toEqual(["fixture-install"]);
      expect(routes).toEqual(["extension-modules.watch"]);
      for (const row of fixture.httpCases) {
        const request = Object.assign(new EventEmitter(), { url: row.url, method: row.method }), chunks: string[] = [];
        let next = false;
        const response = { statusCode: 0, setHeader() {}, write(value: string) { chunks.push(value); }, end(value: string) { chunks.push(value); } };
        await handlers.get(row.owner)!(request, response, () => { next = true; });
        expect(next, row.url).toBe(row.expected === "next");
        if (row.expected !== "next") {
          expect(response.statusCode).toBe(200);
          if (row.expected === "listed") expect(JSON.parse(chunks.join(""))).toEqual([]);
          else expect(chunks.join("")).toContain('"installed":true');
          expect(decodeURIComponent(new OracleURL(row.url, "https://example.invalid").pathname)).toBe(moduleRoutePath(row.url));
        }
        request.emit("close");
      }
    });

    itLong("serves every declared component import and tool-fixed vendor file byte-identically", async () => {
      const ts = await import("typescript"), { PassThrough } = await import("node:stream");
      const { isAbsolute } = await import("node:path");
      const styling = ts.createSourceFile("styling.ts", readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
      const sourceFunctions = ["contentTypeForStaticDirAsset", "serveFileWithValidatorsV1", "createStaticDirMiddleware", "staticDirVitePlugin"].map((name) => {
        const node = styling.statements.find((entry) => ts.isFunctionDeclaration(entry) && entry.name?.text === name);
        if (!node) throw new Error(`missing production function ${name}`);
        return node.getText(styling);
      });
      const emitted = ts.transpileModule(sourceFunctions.join("\n"), { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 } }).outputText;
      const staticDirVitePlugin = new Function("exports", "resolve", "relative", "isAbsolute", "existsSync", "statSync", "createReadStream", "mkdirSync", "cpSync", "process", emitted + "\nreturn exports.staticDirVitePlugin;")({}, resolve, relative, isAbsolute, existsSync, statSync, createReadStream, mkdirSync, cpSync, process);
      const { loadCatalogTaxonomy, fixedFilenameContractIdsForPath } = await import("../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts");
      const taxonomy = loadCatalogTaxonomy(), fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/🪞️vendor.json"), "utf8"));
      const vendorRoot = join(pluginOutRoot, fixture.vendorPath), urls = new Map<string, string>();
      expect(readdirSync(vendorRoot).filter((name) => name !== ".semio-artifact.json").sort()).toEqual(fixture.vendorFiles);
      for (const filename of fixture.vendorFiles) {
        const file = join(vendorRoot, filename), relativeFile = relative(repoRoot, file).replace(/\\/g, "/");
        const installed = readFileSync(join(repoRoot, "node_modules/@bytecodealliance/preview2-shim/dist/browser", filename));
        let expectedBytes = installed;
        if (filename === "cli.js") {
          const expectDir = mkdtempSync(join(tmpdir(), "semio-cli-expect-"));
          const expectFile = join(expectDir, "cli.js");
          writeFileSync(expectFile, installed);
          patchPreview2ShimGuestLogClassification(expectFile);
          patchPreview2ShimGuestLogLineRelease(expectFile);
          expectedBytes = readFileSync(expectFile);
          rmSync(expectDir, { recursive: true, force: true });
        }
        expect(readFileSync(file).equals(expectedBytes), filename).toBe(true);
        expect(fixedFilenameContractIdsForPath(relativeFile, taxonomy)).toEqual([`dev-vendor-${filename.slice(0, -3)}`]);
        for (const hostile of [relativeFile + ".bak", relativeFile.replace("🪞️vendor", "🪞️other"), relativeFile.replace("🪟️preview2-shim", "nested/🪟️preview2-shim")]) expect(fixedFilenameContractIdsForPath(hostile, taxonomy)).toEqual([]);
        urls.set(`${MODULE_PLUGIN_ROUTE}/${fixture.vendorPath}/${filename}`, file);
      }
      for (const spec of fixture.moduleRoots) {
        const root = spec.route === MODULE_EXTENSION_ROUTE ? defaultExtensionInstallRoot(repoRoot) : join(dirname(pluginOutRoot), spec.directory);
        let components = 0, imports = 0;
        for (const directory of readdirSync(root, { withFileTypes: true }).filter((entry) => entry.isDirectory() && moduleIdForDirectoryName(entry.name, runtimeCatalogBuildInventoryV1().moduleDirectories))) {
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
          middleware({ url: new URL(url, "https://example.invalid").pathname, method: "GET", headers: {} }, response, () => reject(new Error(`unserved ${url}`)));
        });
        const actual = await body, expected = readFileSync(file);
        expect(actual.equals(expected), url).toBe(true);
        expect(Buffer.from(await crypto.subtle.digest("SHA-256", new Uint8Array(actual))).toString("hex")).toBe(createHash("sha256").update(expected).digest("hex"));
      }
    }, 120_000);
  });
  //#endregion 🔖️T-P8-tests
}
