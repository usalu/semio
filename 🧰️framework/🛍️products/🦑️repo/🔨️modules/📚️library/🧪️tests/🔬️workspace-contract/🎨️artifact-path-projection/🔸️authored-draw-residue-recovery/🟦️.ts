/** 🔸️ The authored draw residue recovery tests of the artifact path projection authority suite. */
import toml from "@iarna/toml";
import Ajv from "ajv";
import { expect, test } from "bun:test";
import { existsSync, lstatSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, join, posix, relative } from "node:path";
import { getWorkspaceRoot } from "../../../../🗂️workspaces/🟦️.ts";
import { loadTaxonomy } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { fixedSourceDispositionDecision } from "../../../../🔍️discovery/🔭️repository-scan/🟦️.ts";
import { ARTIFACT_PROJECTION_RUN_PARENT, DRAW_SOURCE_SCENARIO, DRAW_SOURCE_SCENARIO_ROOT, artifactProjectionFixturePath, artifactProjectionIgnoreResidue, artifactProjectionProducerInput, artifactProjectionProducerInputs, normalizationFixture, projectionGolden, retainArtifactProjectionFixture } from "../../🏗️shared-fixtures/🟦️.ts";
import { contractRoot } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🔹 Registers the authored draw residue recovery tests of the artifact path projection authority suite. */
export function registerAuthoredDrawResidueRecoveryTests(): void {
  test("authored Draw residue recovery checkpoints have closed language-neutral ownership", async () => {
    const root = join(DRAW_SOURCE_SCENARIO_ROOT, "🛟️residue-recovery"), bytes = readFileSync(join(root, "🔣️.json"), "utf8");
    const vector = JSON.parse(bytes);
    expect(vector["schemaVersion"]).toEqual(1);expect(vector["contractId"]).toEqual("draw-residue-durable-recovery-v1");expect(vector["childExitCode"]).toEqual(73);expect(vector["foreignInput"]).toEqual("ignored-source-directory-with-file-empty-directory-and-symlink");expect(vector["generatorStarts"]).toEqual(0);expect(vector["inverseState"]).toEqual("rolled-back");expect(vector["preservation"]).toEqual("all-owned-preimages-and-all-foreign-residue");expect(vector["cases"]).toEqual([{"id":"durable-resume","checkpoint":"durable","phase":"transaction-wal-prepared","canonicalState":"regenerating","candidateState":null,"installed":"all","cancel":false},{"id":"cancelled-resume","checkpoint":"durable","phase":"transaction-wal-prepared","canonicalState":"regenerating","candidateState":null,"installed":"all","cancel":true},{"id":"next-journal-wal","checkpoint":"candidate","phase":"transaction-journal-candidate-written","canonicalState":"editing","candidateState":"regenerating","installed":"all","cancel":false},{"id":"rename-before-journal","checkpoint":"rename","phase":"transaction-journal-write-mkdir","canonicalState":"installing","candidateState":null,"installed":"first-unrecorded","cancel":false}]);
    
    const jsonc = await import("jsonc-parser"), errors: import("jsonc-parser").ParseError[] = [];
    expect(jsonc.parse(bytes, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
    expect(errors).toEqual([]);
    expect(vector.cases.filter((row: { cancel: boolean }) => row.cancel)).toHaveLength(1);
    expect(new Set(vector.cases.map((row: { phase: string }) => row.phase)).size).toBe(3);
  });
  test("authored Draw residue ignore authority outranks inherited negation before injection", async () => {
    const inputRoot = join(DRAW_SOURCE_SCENARIO_ROOT, "🙈️residue-ignore"), bytes = readFileSync(join(inputRoot, "🔣️.json"), "utf8"), vector = JSON.parse(bytes);
    const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(contractRoot, "../../🧬️schema/🖍️draw-source-scenario/🙈️residue-ignore/🔣️.json"), "utf8")));
    expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
    for (const changed of [{ ...vector, authoritativeSource: vector.lowerPrioritySource }, { ...vector, absentIgnored: false }, { ...vector, writePolicy: "replace-policy" }]) expect(validate(changed)).toBe(false);
    const jsonc = await import("jsonc-parser"), errors: import("jsonc-parser").ParseError[] = [];
    expect(jsonc.parse(bytes, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
    expect(errors).toEqual([]);
    const fixture = normalizationFixture("artifact-projection-residue-ignore", {}, ({ repoRoot }) => writeFileSync(join(repoRoot, vector.authoritativeSource), vector.rootIgnore), ARTIFACT_PROJECTION_RUN_PARENT);
    let passed = false;
    try {
      const residue = join(fixture.workspace, ...vector.residueSegments), leaf = join(residue, vector.leaf), relativeLeaf = relative(fixture.root, leaf).replaceAll("\\", "/");
      const rule = `/${relative(fixture.root, residue).replaceAll("\\", "/")}/\n`, ignorePath = join(fixture.root, vector.authoritativeSource), beforeMode = lstatSync(ignorePath).mode;
      mkdirSync(dirname(residue), { recursive: true });
      writeFileSync(join(fixture.root, vector.lowerPrioritySource), rule);
      const ignored = (path: string): boolean => {
        const result = Bun.spawnSync(["git", "check-ignore", "--quiet", "--no-index", "--", path], { cwd: fixture.root, stdout: "pipe", stderr: "pipe" });
        if (result.exitCode !== 0 && result.exitCode !== 1) throw new Error(result.stderr.toString());
        return result.exitCode === 0;
      };
      expect(ignored(relativeLeaf)).toBe(vector.beforeIgnored);
      for (const invalid of [join(fixture.root, "🧪️unexpected"), join(fixture.workspace, "*", "🧪️unexpected"), join(fixture.workspace, "🧪️line\nbreak", "🧪️unexpected")]) expect(() => artifactProjectionIgnoreResidue(fixture, invalid)).toThrow();
      expect(readFileSync(ignorePath, "utf8")).toBe(vector.rootIgnore);
      artifactProjectionIgnoreResidue(fixture, residue);
      expect(existsSync(residue)).toBe(false);
      expect(ignored(relativeLeaf)).toBe(vector.absentIgnored);
      expect(readFileSync(ignorePath, "utf8")).toBe(vector.rootIgnore + rule);
      expect(lstatSync(ignorePath).mode).toBe(beforeMode);
      mkdirSync(residue, { recursive: true });
      writeFileSync(leaf, vector.content);
      expect(ignored(relativeLeaf)).toBe(vector.presentIgnored);
      expect(ignored(relative(fixture.root, join(dirname(residue), "🧪️neighbor", vector.leaf)).replaceAll("\\", "/"))).toBe(vector.siblingIgnored);
      const oracle = Bun.spawnSync(["git", "check-ignore", "--verbose", "--no-index", "-z", "--stdin"], { cwd: fixture.root, stdin: Buffer.from(`${relativeLeaf}\0`), stdout: "pipe", stderr: "pipe" });
      expect(oracle.exitCode, oracle.stderr.toString()).toBe(0);
      expect(oracle.stdout.toString().split("\0").slice(0, 4)).toEqual([vector.authoritativeSource, "3", rule.trim(), relativeLeaf]);
      expect(readFileSync(join(fixture.root, vector.lowerPrioritySource), "utf8")).toBe(rule);
      expect(() => artifactProjectionIgnoreResidue(fixture, residue)).toThrow("Residue must be absent");
      expect(readFileSync(ignorePath, "utf8")).toBe(vector.rootIgnore + rule);
      passed = true;
    } finally {
      retainArtifactProjectionFixture(fixture, passed);
    }
  });
  test("authored Draw source schema and parser oracles retain the exact nine-member contract", async () => {
    
    
    expect(DRAW_SOURCE_SCENARIO["schemaVersion"]).toEqual(1);expect(DRAW_SOURCE_SCENARIO["contractId"]).toEqual("draw-source-scenario-input-v1");expect(DRAW_SOURCE_SCENARIO["producerContext"]).toEqual({"generatorId":"plugin-registry","compilerRoots":["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts"],"runtimeModules":["🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🟨️.mjs","🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🟨️.mjs"],"runtimeData":["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"],"runtimeReceiptCatalogs":[{"manifestPath":"🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📜️native-codec-factories.json","sourceRoot":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts","receiptPathField":"protocol_path"}],"workspaceInputs":["package.json","nx.json","📋️project.json",".gitignore"],"runtimePackages":["nx","@nx/js","@nxlv/python","typescript","@iarna/toml"],"authority":"current-exact-no-follow-static-closure","workspaceBindings":"fixture-local","initialOutputs":"actual-generator-before-baseline","registryNodeCount":16});expect(DRAW_SOURCE_SCENARIO["cadConsumerMount"]).toEqual({"path":"✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts","meaning":"unchanged-live-cad-bytes-in-authored-canonical-fixture-context"});expect(DRAW_SOURCE_SCENARIO["owner"]["artifactId"]).toEqual("🖍️drawing");expect(DRAW_SOURCE_SCENARIO["owner"]["standardVersion"]).toEqual("1");expect(DRAW_SOURCE_SCENARIO["owner"]["subsetId"]).toEqual("any");expect(DRAW_SOURCE_SCENARIO["owner"]["commandDirectoryName"]).toEqual("🖱️canvas-pointer-down");expect(DRAW_SOURCE_SCENARIO["commandBundle"]["directoryKinds"]["fsm"]).toEqual({"emoji":"🔄️","slugPattern":"^fsm$","allowEmojiOnly":false});expect(DRAW_SOURCE_SCENARIO["commandBundle"]["descendantContract"]["rootDirectoryKindId"]).toEqual("projected-editor-command");expect(DRAW_SOURCE_SCENARIO["commandBundle"]["descendantContract"]["exclusiveAlternatives"]).toEqual([]);expect(DRAW_SOURCE_SCENARIO["commandBundle"]["descendantContract"]["realizedNodeCount"]).toEqual(16);
    
    const projection = projectionGolden("artifact-editor-command-bundle-v1");
    expect(DRAW_SOURCE_SCENARIO.members.map(({ path }) => `${projection.sourceRoot}/${path}`)).toEqual(projection.mappings.map(({ sourcePath }) => sourcePath));
    expect(DRAW_SOURCE_SCENARIO.owner).toEqual({ artifactId: projection.artifactId, standardVersion: projection.standardVersion, subsetId: projection.subsetId, commandDirectoryName: basename(projection.sourceRoot) });
    expect(DRAW_SOURCE_SCENARIO.members.reduce((total, row) => total + Buffer.byteLength(row.content), 0)).toBeLessThan(10_000);
    const jsonc = await import("jsonc-parser"), ts = await import("typescript");
    const render = (content: string): string => content.replaceAll("{{sourceRoot}}", projection.sourceRoot).replaceAll("{{artifactSourceRoot}}", artifactProjectionFixturePath(projection.sourceRoot));
    let projectCwds = 0, workspaceGlobs = 0;
    for (const row of [...DRAW_SOURCE_SCENARIO.members, ...DRAW_SOURCE_SCENARIO.consumers]) {
      const content = render(row.content);
      expect(content).not.toContain("{{");
      if (row.format === "json") {
        const errors: import("jsonc-parser").ParseError[] = [];
        const parsed = jsonc.parse(content, errors, { disallowComments: true, allowTrailingComma: false });
        expect(errors).toEqual([]);
        expect(parsed).toEqual(JSON.parse(content));
        if (row.path.endsWith("📋️project.json")) {
          projectCwds += Object.values(parsed.targets as Record<string, { options: { cwd: string; command: string } }>).filter(({ options }) => options.cwd.startsWith(projection.sourceRoot) && options.command.startsWith("bun ./📜️script.ts test")).length;
          workspaceGlobs += parsed.namedInputs.default.filter((value: string) => value.startsWith("{workspaceRoot}/") && value.endsWith("/**/*.rs")).length;
        } else expect(parsed.entries.flatMap((entry: { users: string[] }) => entry.users)).toHaveLength(DRAW_SOURCE_SCENARIO.oracle.dependencyUserCount);
      } else if (row.format === "toml") {
        const parsed = toml.parse(content);
        if (row.path.endsWith("/📦️packages/🦀️rust/Cargo.toml")) expect(DRAW_SOURCE_SCENARIO.members.map(({ path }) => path)).toContain(posix.join(dirname(row.path), (parsed.lib as { path: string }).path));
        if (row.path === "Cargo.toml") expect((parsed.workspace as { members: string[] }).members).toHaveLength(DRAW_SOURCE_SCENARIO.oracle.workspaceMemberCount);
      } else if (row.format === "typescript") {
        const result = ts.transpileModule(content, { fileName: row.path, reportDiagnostics: true, compilerOptions: { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext } });
        expect(result.diagnostics).toEqual([]);
        const source = ts.createSourceFile(row.path, content, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
        const imports = source.statements.filter(ts.isImportDeclaration).map((statement) => (statement.moduleSpecifier as import("typescript").StringLiteral).text);
        expect(imports).toEqual(row.path === "📜️script.ts" ? ["node:fs", "node:path", "@semio-tech/repo-lib"] : ["@semio-tech/repo-lib"]);
        expect(fixedSourceDispositionDecision("root-script", content)?.role).toBe("tool-metadata");
      }
    }
    expect(projectCwds).toBe(DRAW_SOURCE_SCENARIO.oracle.projectCwdCount);
    expect(workspaceGlobs).toBe(DRAW_SOURCE_SCENARIO.oracle.workspaceGlobCount);
    const host = DRAW_SOURCE_SCENARIO.catalogContext.find(({ format }) => format === "toml")!;
    const parsedHost = toml.parse(host.content);
    expect(Bun.TOML.parse(host.content)).toEqual(parsedHost);
    expect(parsedHost.package).toEqual({ name: "draw-fixture-host", version: "0.1.0", edition: "2021", metadata: { component: { package: "semio:draw-fixture-host" }, semio: { role: "hub", "component-kind": "plugin", "deployment-directory": "🧪️draw-fixture-host", host: { landing: "fixture-landing", shell: "fixture-shell" }, playground: [{ variant: "draw-fixture-host", ports: { react: 6197, wgpu: 6198 } }] } } });
    expect(DRAW_SOURCE_SCENARIO.catalogContext.find(({ path }) => path === `${dirname(host.path)}/${(parsedHost.lib as { path: string }).path}`)?.content).toBe("pub fn fixture_host() {}\n");
  });
  test("authored Draw source producer context captures actual implementations with independent import parity", async () => {
    const ts = await import("typescript"), { registryStaticImports } = await import("../../../../🔍️discovery/📇️registry-catalog-inputs/🟦️.ts");
    const taxonomy = loadTaxonomy(), readPaths: string[] = [];
    const context = artifactProjectionProducerInputs(taxonomy, (path: string) => {
      readPaths.push(path);
      expect(path).not.toStartWith("✏️s/🔌️plugins/🖍️draw/");
      return artifactProjectionProducerInput(path);
    });
    const contract = taxonomy.generatorContracts[DRAW_SOURCE_SCENARIO.producerContext.generatorId]!;
    expect(context.files[contract.inputDiscovery!.implementationEntryPaths[0]!]?.content).toBe(readFileSync(join(getWorkspaceRoot(), contract.ownerPath!, "📜️script.ts"), "utf8"));
    expect(context.files[contract.inputDiscovery!.implementationEntryPaths[0]!]?.content).not.toBe("export {};\n");
    for (const path of [...contract.inputPatterns, ...DRAW_SOURCE_SCENARIO.producerContext.workspaceInputs, ...DRAW_SOURCE_SCENARIO.producerContext.runtimeData]) expect(context.files[path]?.content).toBe(readFileSync(join(getWorkspaceRoot(), path), "utf8"));
    for (const path of DRAW_SOURCE_SCENARIO.producerContext.runtimeModules) expect(context.modules.some((row) => row.path === path)).toBe(true);
    const nxPlugins = (JSON.parse(context.files["nx.json"]!.content) as { plugins: { plugin: string }[] }).plugins.filter(({ plugin }) => plugin.startsWith("./")).map(({ plugin }) => plugin.slice(2));
    for (const path of [...DRAW_SOURCE_SCENARIO.producerContext.runtimeModules, ...DRAW_SOURCE_SCENARIO.producerContext.runtimeData]) expect(nxPlugins.some((plugin) => context.files[plugin]!.content.includes(posix.relative(posix.dirname(plugin), path))), path).toBe(true);
    expect(readPaths).toEqual([...new Set(readPaths)]);
    const nx = JSON.parse(context.files["nx.json"]!.content);
    for (const row of nx.plugins.filter((entry: { plugin: string }) => entry.plugin.startsWith("./"))) expect(context.files[row.plugin.slice(2)]).toBeDefined();
    const nxScript = (JSON.parse(context.files["package.json"]!.content) as { scripts: Record<string, string> }).scripts.nx!.match(/^bun \.\/(\S+) nx$/u)?.[1];
    if (nxScript === undefined) throw new Error("package.json names no `bun ./<script> nx` runner");
    expect(DRAW_SOURCE_SCENARIO.producerContext.compilerRoots).toEqual([nxScript]);
    for (const row of context.modules) {
      if (row.kind === "json-data") {
        const jsonc = await import("jsonc-parser"), errors: import("jsonc-parser").ParseError[] = [];
        expect(jsonc.parse(context.files[row.path]!.content, errors, { disallowComments: true, allowTrailingComma: false }), row.path).toEqual(JSON.parse(context.files[row.path]!.content));
        expect(errors, row.path).toEqual([]);
        expect(row.imports, row.path).toEqual([]);
        continue;
      }
      const source = ts.createSourceFile(row.path, context.files[row.path]!.content, ts.ScriptTarget.Latest, true, row.path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
      const oracle = [...new Set(source.statements.flatMap((statement) => {
        if (!ts.isImportDeclaration(statement) && !ts.isExportDeclaration(statement) || ts.isImportDeclaration(statement) && statement.importClause?.isTypeOnly || ts.isExportDeclaration(statement) && statement.isTypeOnly) return [];
        return statement.moduleSpecifier && ts.isStringLiteral(statement.moduleSpecifier) ? [statement.moduleSpecifier.text] : [];
      }))].sort();
      expect(registryStaticImports(context.files[row.path]!.content, row.path), row.path).toEqual(oracle);
      expect(row.imports, row.path).toEqual(oracle);
    }
    for (const [name, binding] of Object.entries(contract.inputDiscovery!.workspaceImports)) {
      expect(JSON.parse(context.files[binding.manifestPath]!.content).name).toBe(name);
      expect(context.files[binding.entryPath]).toBeDefined();
    }
  });
}
