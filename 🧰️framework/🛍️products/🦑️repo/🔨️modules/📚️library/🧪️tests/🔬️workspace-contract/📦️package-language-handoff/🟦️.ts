import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import toml from "@iarna/toml";
import Ajv from "ajv";
import { describe, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join, posix, relative, resolve, sep } from "node:path";
import { readSemioMarker } from "../../../🔍️discovery/🔭️repository-scan/🟦️.ts";
import { validateTaxonomy, canonicalPrimaryFilenameForKind } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { partitionNextestExecutionFilters } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter, readScriptPolicy, type ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { fixedFilenameContractIdsForPath } from "../../../🔍️discovery/⚖️fixed-contract-resolution/🟦️.ts";
import { pathEmojiStatuteFindings } from "@semio-tech/framework/identity/path";
import { normalizationEngineDeclarations } from "../../../🧹️normalization/🧪️support/🏗️source-services/🟦️.ts";
import { readToml } from "../🏗️shared-fixtures/🟦️.ts";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";

/** 🎟️ Supplies each router dispatch the original 15-second control and a silent progress port. */
function routerInvocation(owner: string): ScriptInvocation {
  const controller = new AbortController(), started = performance.now();
  return {
    policy: readScriptPolicy({ version: 1, owner, maximumElapsedMilliseconds: 15000 }),
    control: {
      signal: controller.signal,
      remainingMilliseconds: () => Math.max(1, 15000 - (performance.now() - started)),
      publish: () => undefined,
      yieldContinuation: () => Promise.resolve(),
    },
    capabilities: {},
  };
}

/** 🦀️ The workspace root manifest, read only for the version every member inherits. */
interface CargoWorkspaceManifest {
  readonly workspace: { readonly package: { readonly version: string }; readonly members: string[]; readonly dependencies: Record<string, unknown> };
}

describe("package language semantic handoff", () => {
  const inputRoot = join(contractRoot, "../🧫️fixtures/🤝️package-language-kind-handoff");
  const vector = JSON.parse(readFileSync(join(inputRoot, "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(contractRoot, "../../🔣️taxonomy.json"), "utf8"));
  const fold = (value: string): string => value.normalize("NFC").replaceAll("\uFE0F", "");

  test("schema-first six-language authority preserves the existing package purity boundary", () => {
    
    expect(vector["schemaVersion"]).toEqual(1);expect(vector["contractId"]).toEqual("package-language-semantic-kind-handoff-v1");expect(vector["semantics"]).toEqual({"boundary":"semantic-directory-registry","parentKindId":"packages","variation":"canonical-registered-emoji","ambiguity":"fail-closed","packagePurity":"target-before-package"});expect(vector["oracleCompilation"]).toEqual({"scope":"single-test-invocation","identity":"exact-serialized-json-schema","maxCompilationsPerIdentity":1,"validation":"every-scenario-step-and-compiler"});
    
    
    
    
    expect(vector.languages.map((row: { name: string }) => row.name).sort()).toEqual(Object.keys(schema.packageBoundaryRules).sort());
    for (const language of vector.languages) {
      expect(schema.semanticDirectoryKinds[language.id]).toEqual({ emoji: language.emoji, slugPattern: "^" + language.slug + "$", allowEmojiOnly: false, parentKindIds: ["packages"] });
      expect(schema.packageBoundaryRules[language.name].allowedDirectoryKindIds).toEqual(language.allowedDirectoryKindIds);
    }
  });

  test("the actual directory classifier agrees with independent compilers and JSON Schema semantic matching", async () => {
    const ts = await import("typescript");
    const { parse: parseJsonc } = await import("jsonc-parser");
    const { createTaxonomyPathMatcher } = await import("../../../🔍️discovery/🎯️taxonomy-path-matching/🟦️.ts"), { mutationOwnerIdentity } = await import("../../../🔍️discovery/🪪️mutation-owner-identity/🟦️.ts"), { targetInsidePackageBoundaryFinding } = await import("../../../🔍️discovery/🗂️file-kind-resolution/🟦️.ts");
    expect(parseJsonc(readFileSync(join(inputRoot, "🔣️.json"), "utf8"))).toEqual(vector);
    const source = normalizationEngineDeclarations(join(contractRoot, "../../🧹️normalization"));
    const syntax = ts.createSourceFile("🟦️.ts", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const names = new Set(["emojiFold", "graphemes", "isEmojiGrapheme", "splitLeadingEmoji", "splitLeadingEmojiIdentity", "matchDirectoryKind", "packageLocation", "fixedSpecificity", "compareFixedSpecificity", "equalFixedSpecificity", "fixedScopeMatches", "matchingFixedContracts", "violation", "mutationDomainOwnerLocation", "mutationFixtureMirrorOwnerPath", "canonicalDirectory", "directoryKindIndex", "directoryKindIndexes", "fixedContractCandidates", "fixedContractLeafIndex", "fixedContractLeafIndexes", "fixedContractLiteralLeaf", "fixedContractSpecificity", "fixedContractSpecificities"]);
    const declarations = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) && names.has(node.name?.text ?? ""));
    expect(declarations).toHaveLength(names.size);
    const code = declarations.map((node) => node.getText(syntax)).join("\n") + "\nreturn canonicalDirectory;";
    const compilers = [
      new Bun.Transpiler({ loader: "ts" }).transformSync(code),
      ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText,
    ];
    const operations = compilers.map((javascript) => new Function("basename", "dirname", "SEGMENTER", "pathEmojiStatuteFindings", "targetInsidePackageBoundaryFinding", "mutationOwnerIdentity", javascript)(basename, dirname, new Intl.Segmenter("und", { granularity: "grapheme" }), pathEmojiStatuteFindings, targetInsidePackageBoundaryFinding, mutationOwnerIdentity));
    const cases = [
      ...vector.languages.flatMap((language: { id: string; name: string }) => [language.name, fold(language.name)].map((name) => ({ id: name, steps: [{ name, parentKindId: "packages", kindId: language.id, canonicalName: language.name, violationCodes: [] }] }))),
      ...vector.cases,
    ];
    const ajv = new Ajv({ strict: true });
    const oracleValidators = new Map<string, ReturnType<Ajv["compile"]>>();
    const oracleCompilationCounts = new Map<string, number>();
    let oracleCompileCalls = 0;
    for (const scenario of cases) {
      const registry = structuredClone(schema);
      registry.fixedDirectoryContracts = {};
      if (scenario.removeKind) delete registry.semanticDirectoryKinds[scenario.removeKind];
      if (scenario.duplicateKind) registry.semanticDirectoryKinds["duplicate-kind"] = registry.semanticDirectoryKinds[scenario.duplicateKind];
      if (scenario.duplicateBoundary) registry.packageBoundaryRules[fold(scenario.duplicateBoundary)] = registry.packageBoundaryRules[scenario.duplicateBoundary];
      const directoryKinds = Object.entries(registry.semanticDirectoryKinds).map(([id, entry]) => ({ id, ...(entry as Record<string, unknown>), slugRegex: new RegExp((entry as { slugPattern: string }).slugPattern, "u") }));
      const predicates = Object.entries(registry.semanticDirectoryKinds).map(([id, entry]) => {
        const spec = entry as { emoji: string; slugPattern: string; parentKindIds?: string[] };
        const predicateSchema = { type: "object", required: ["emoji", "slug", "parentKindId"], properties: { emoji: { const: fold(spec.emoji) }, slug: { type: "string", pattern: spec.slugPattern }, parentKindId: spec.parentKindIds ? { enum: spec.parentKindIds } : {} } };
        const identity = JSON.stringify(predicateSchema);
        let validate = oracleValidators.get(identity);
        if (!validate) {
          validate = ajv.compile(JSON.parse(identity));
          oracleValidators.set(identity, validate);
          const count = (oracleCompilationCounts.get(identity) ?? 0) + 1;
          oracleCompilationCounts.set(identity, count);
          oracleCompileCalls++;
          expect(count, JSON.stringify({ pid: process.pid, scenario: scenario.id, kind: id, compileCalls: oracleCompileCalls, uniqueSchemas: oracleCompilationCounts.size })).toBeLessThanOrEqual(vector.oracleCompilation.maxCompilationsPerIdentity);
        }
        return { id, emoji: spec.emoji, validate };
      });
      const observed = operations.map((operation) => {
        let parentCanonical = "📦️packages", previousKind: string | undefined;
        return scenario.steps.map((step: { name: string; parentKindId: string; kindId: string | null; canonicalName: string; violationCodes: string[] }) => {
          const parentKindId = step.parentKindId === "previous" ? previousKind : step.parentKindId;
          const result = operation(parentCanonical + "/" + step.name, parentCanonical, parentKindId, ["packages"], { schema: registry, discoverySchema: registry, directoryKinds, pathMatcher: createTaxonomyPathMatcher() });
          const expected = { path: parentCanonical + "/" + step.canonicalName, kindId: step.kindId, violationCodes: step.violationCodes };
          const actual = { path: result.path, kindId: result.kindId, violationCodes: result.violations.map((row: { code: string }) => row.code) };
          const slug = step.name.replace(/^[^\p{L}]*/u, ""), emoji = step.name.slice(0, step.name.length - slug.length);
          const matches = predicates.filter((candidate) => candidate.validate({ emoji: fold(emoji), slug, parentKindId }));
          const exact = matches.filter((candidate) => candidate.id === slug), selected = exact.length ? exact : matches;
          const duplicateBoundary = parentKindId === "packages" && Object.keys(registry.packageBoundaryRules).filter((name) => fold(name) === fold(step.name)).length > 1;
          const oracle = duplicateBoundary ? { path: parentCanonical + "/" + step.name, kindId: null, violationCodes: ["package-language-ambiguous"] } : selected.length === 1 ? { path: parentCanonical + "/" + selected[0].emoji + slug, kindId: selected[0].id, violationCodes: [] } : { path: parentCanonical + "/" + step.name, kindId: null, violationCodes: [selected.length > 1 ? "directory-kind-ambiguous" : "directory-kind-unresolved"] };
          expect(oracle, scenario.id).toEqual<typeof expected>(expected);
          expect(actual, scenario.id).toEqual(expected);
          parentCanonical = result.path;
          previousKind = result.kindId ?? undefined;
          return actual;
        });
      });
      expect(observed[0], scenario.id).toEqual(observed[1]);
    }
    expect(oracleCompileCalls).toBe(oracleCompilationCounts.size);
  }, 30_000);

  test("resident native metadata binds the exact semantic owner and one Cargo identity", async () => {
    const { parse: parseJsonc } = await import("jsonc-parser");
    const { win32 } = await import("node:path");
    const { semanticDirectoryKindId } = await import("../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts"), { semanticOwnedInputFileSnapshot } = await import("../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts");
    const root = resolve(contractRoot, "../../../../../../.."), fixtureRoot = join(inputRoot, "💾️resident-package");
    const text = readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"), expected = JSON.parse(text);
    expect(parseJsonc(text)).toEqual(expected);
    expect(validateTaxonomy(schema)).toEqual([]);
    expect(canonicalPrimaryFilenameForKind("rust-source", schema)).toBe("🦀️.rs");
    expect(semanticDirectoryKindId(basename(dirname(inputRoot)), schema, { parentKindId: "members-of-modules" })).toBe("fixtures");
    expect(semanticDirectoryKindId("🔨️modules", schema)).toBe("modules");
    expect(semanticDirectoryKindId("🌱️value", schema, { parentKindId: "modules" })).toBe("members-of-modules");
    expect(semanticDirectoryKindId(expected.member.name, schema, { parentKindId: expected.member.ownerKindId })).toBe(expected.member.registry);
    expect(schema.semanticDirectoryMemberKinds[expected.member.registry].memberNames.filter((name: string) => name === expected.member.name)).toHaveLength(1);
    expect(semanticDirectoryKindId("📦️packages", schema, { parentKindId: expected.member.registry })).toBe("packages");
    expect(semanticDirectoryKindId("🦀️rust", schema, { parentKindId: "packages" })).toBe("rust-language");
    expect(semanticDirectoryKindId("🧪️tests", schema, { parentKindId: expected.member.registry })).toBe("tests");
    expect(semanticDirectoryKindId(expected.member.name, schema, { parentKindId: "packages" })).toBeNull();
    const source = semanticOwnedInputFileSnapshot(root, expected.sourcePath), nativeTests = semanticOwnedInputFileSnapshot(root, expected.testPath);
    expect(source?.nodeKind).toBe("file");
    expect(nativeTests?.nodeKind).toBe("file");
    expect(source!.bytes.toString()).toContain(`#[path = "${posix.relative(posix.dirname(expected.sourcePath), expected.testPath)}"]`);
    expect(nativeTests!.bytes.toString()).toContain(`include_str!("${posix.relative(posix.dirname(expected.testPath), expected.fixturePath)}")`);
    expect(semanticOwnedInputFileSnapshot(root, expected.ownerPath + "/🦀️missing.rs")).toBeNull();
    for (const path of [expected.sourcePath, expected.testPath]) expect(basename(path)).toBe(canonicalPrimaryFilenameForKind("rust-source", schema));
    const packageRoot = join(root, expected.packagePath);
    expect(readdirSync(packageRoot).sort()).toEqual([...expected.packageFiles].sort());
    const manifestText = readFileSync(join(packageRoot, "Cargo.toml"), "utf8"), manifest = toml.parse(manifestText);
    expect(Bun.TOML.parse(manifestText)).toEqual(manifest);
    expect(JSON.parse(JSON.stringify(manifest))).toEqual(expected.cargo);
    expect(readSemioMarker(join(packageRoot, "Cargo.toml"), "🦀️rust", schema)).toEqual({ role: "framework", id: "value-resident" });
    const libPath = (manifest.lib as { path: string }).path;
    expect(posix.normalize(posix.join(expected.packagePath, libPath))).toBe(expected.sourcePath);
    expect(win32.normalize(win32.join(expected.packagePath, libPath)).replaceAll("\\", "/")).toBe(expected.sourcePath);
    for (const path of expected.rejectedEntryPaths) {
      expect(posix.normalize(posix.join(expected.packagePath, path))).not.toBe(expected.sourcePath);
      expect(win32.normalize(win32.join(expected.packagePath, path)).replaceAll("\\", "/")).not.toBe(expected.sourcePath);
    }
    const workspaceText = readFileSync(join(root, "Cargo.toml"), "utf8"), workspace = toml.parse(workspaceText).workspace as { members: string[]; dependencies: Record<string, unknown>; package: Record<string, unknown> };
    const parsedWorkspace = Bun.TOML.parse(workspaceText);
    expect("workspace" in parsedWorkspace ? parsedWorkspace.workspace : undefined).toEqual(workspace);
    expect(workspace.members.filter(path => path === expected.packagePath)).toHaveLength(1);
    expect(workspace.dependencies[expected.cargo.package.name]).toEqual({ path: expected.packagePath });
    expect(workspace.package).toMatchObject({ version: "0.1.0", edition: "2021", "rust-version": "1.95" });
    const projectText = readFileSync(join(packageRoot, "📋️project.json"), "utf8"), project = JSON.parse(projectText);
    expect(parseJsonc(projectText)).toEqual(project);
    expect(project).toEqual(expected.project);
    expect(resolve(packageRoot, project.$schema)).toBe(join(root, "node_modules/nx/schemas/project-schema.json"));
    const toolchainText = readFileSync(join(root, "rust-toolchain.toml"), "utf8"), toolchain = toml.parse(toolchainText);
    expect(Bun.TOML.parse(toolchainText)).toEqual(toolchain);
    for (const target of expected.wasmTargets) expect((toolchain.toolchain as { targets: string[] }).targets).toContain(target);
    expect(JSON.parse(readFileSync(join(root, expected.ownerPath, "📋️project.json"), "utf8")).name).toBe(expected.existingTs.projectName);
    const runs = join(realpathSync(tmpdir()), "semio-native-resident-metadata");
    mkdirSync(runs, { recursive: true });
    const fixture = mkdtempSync(join(runs, "🔖️")), target = join(fixture, "🧫️target");
    mkdirSync(target);
    writeFileSync(join(target, "🦀️.rs"), "pub fn neutral_fixture() {}\n");
    symlinkSync(target, join(fixture, "🧫️link"), process.platform === "win32" ? "junction" : "dir");
    expect(() => semanticOwnedInputFileSnapshot(fixture, "🧫️link/🦀️.rs")).toThrow("no-follow");
    expect(semanticOwnedInputFileSnapshot(fixture, "🧫️target/🦀️.rs")?.bytes.toString()).toBe("pub fn neutral_fixture() {}\n");
    writeFileSync(join(fixture, "📝️.md"), "# Resident Metadata No-Follow Fixture\n\nAuthored neutral Rust input and ancestor-link rejection only; no native compilation.\n");
  });

  test("resident native metadata router preserves exact sequential bounded commands without executing Cargo", async () => {
    const ts = await import("typescript"), { spyOn } = await import("bun:test");
    const root = resolve(contractRoot, "../../../../../../.."), expected = JSON.parse(readFileSync(join(inputRoot, "💾️resident-package/🔣️.json"), "utf8"));
    const packageRoot = join(root, expected.packagePath), sourcePath = join(packageRoot, "📜️script.ts");
    expect(existsSync(sourcePath)).toBe(true);
    const source = readFileSync(sourcePath, "utf8"), syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    expect(source.split(/\r?\n/u)[0]).toBe("#!/usr/bin/env bun");
    const imports = syntax.statements.filter(ts.isImportDeclaration);
    expect(imports.map(row => ({ module: (row.moduleSpecifier as import("typescript").StringLiteral).text, names: row.importClause?.namedBindings && ts.isNamedImports(row.importClause.namedBindings) ? row.importClause.namedBindings.elements.map(node => node.name.text).sort() : [] }))).toEqual(expected.moduleImports);
    expect(expected.moduleImports.flatMap((row: { names: string[] }) => row.names).sort()).toEqual(expected.imports);
    expect(syntax.statements.every(node => ts.isImportDeclaration(node) || ts.isClassDeclaration(node) || ts.isVariableStatement(node) || ts.isIfStatement(node))).toBe(true);
    expect(syntax.statements.filter(ts.isClassDeclaration).map(node => node.name?.text)).toEqual(["TestScript", "CheckWasmScript"]);
    const transformed = ts.transform(syntax, [(context) => {
      const visit: import("typescript").Visitor = (node) => {
        if (ts.isImportDeclaration(node)) return undefined;
        if (ts.isPropertyAccessExpression(node) && ts.isMetaProperty(node.expression)) {
          if (node.name.text === "dir") return ts.factory.createStringLiteral(packageRoot);
          if (node.name.text === "url") return ts.factory.createStringLiteral("resident-source-only");
          if (node.name.text === "main") return ts.factory.createTrue();
          throw new Error("Unexpected import.meta dependency");
        }
        return ts.visitEachChild(node, visit, context);
      };
      return node => ts.visitNode(node, visit) as import("typescript").SourceFile;
    }]);
    const code = ts.createPrinter().printFile(transformed.transformed[0]!); transformed.dispose();
    const compiled = [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext } }).outputText];
    const AsyncFunction = Object.getPrototypeOf(async () => {}).constructor;
    for (const javascript of compiled) {
      let router: ScriptRouter | undefined, active = 0, refuse = false;
      const calls: unknown[][] = [], events: string[] = [];
      const dependencies = {
        BundleScript, ScriptRouter, resolve, readCargoTestPolicyV1: (environment:unknown) => {expect(environment).toBe(process.env);return {version:1};}, buildBudgetMs: buildBudgetMs,
        runScriptMain: async (value: ScriptRouter, ...rest: unknown[]) => { expect(rest).toEqual([]); router = value; },
        runCargoTestsV1: async (...args: unknown[]) => { calls.push(args); },
        runBudgetedTestCommand: async (command: string, args: string[], options: { cwd: string; budgetMs: number; env?: NodeJS.ProcessEnv }) => {
          expect(++active).toBe(1); calls.push([command, args, options]); events.push("start:" + args.at(-1));
          await Promise.resolve(); active--; events.push("end:" + args.at(-1));
          if (refuse) throw new Error("command refused");
        },
      };
      const body = javascript.replace(/^#!\/usr\/bin\/env bun\r?\n/u, "");
      expect(body.startsWith("#!")).toBe(false);
      await new AsyncFunction(...Object.keys(dependencies), body)(...Object.values(dependencies));
      expect(router).toBeInstanceOf(ScriptRouter);
      expect(router!.usage()).toBe("bun ./📜️script.ts <test|check-wasm> [args…]");
      await router!.run(["test"], routerInvocation("workspace-contract"));
      expect(calls).toEqual([[{manifestPath:join(packageRoot,"Cargo.toml"),packages:[expected.cargo.package.name],cwd:packageRoot,extraArgs:["--lib"]},{version:1}]]);
      calls.length = 0;
      await router!.run(["check-wasm"], routerInvocation("workspace-contract"));
      expect(calls).toEqual(expected.wasmTargets.map((target: string) => ["cargo", ["check", "--manifest-path", "Cargo.toml", "--lib", "--target", target], { cwd: packageRoot, budgetMs: buildBudgetMs() }]));
      expect(events).toEqual(expected.wasmTargets.flatMap((target: string) => ["start:" + target, "end:" + target]));
      calls.length = 0;
      for (const command of ["test", "check-wasm"]) await expect(router!.run([command, "unexpected"], routerInvocation("workspace-contract"))).rejects.toThrow("accepts no arguments");
      expect(calls).toEqual([]);
      await expect(router!.run(["unknown"], routerInvocation("workspace-contract"))).rejects.toThrow('unknown command "unknown"');
      expect(calls).toEqual([]);
      refuse = true;
      await expect(router!.run(["check-wasm"], routerInvocation("workspace-contract"))).rejects.toThrow("command refused");
      expect(calls).toHaveLength(1);
      expect((calls[0]![1] as string[]).at(-1)).toBe(expected.wasmTargets[0]);
    }
  });

  test("UI-host metadata binds canonical source inputs and exact existing Cargo ownership", async () => {
    const { parse: parseJsonc } = await import("jsonc-parser"), { win32 } = await import("node:path");
    const { semanticDirectoryKindId } = await import("../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts"), { semanticOwnedInputFileSnapshot } = await import("../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts");
    const root = resolve(contractRoot, "../../../../../../.."), fixtureRoot = join(inputRoot, "🖥️ui-host-package"), captured = new Map<string, Uint8Array>();
    const read = (path: string): Buffer => {
      const row = semanticOwnedInputFileSnapshot(root, path);
      expect(row?.nodeKind, path).toBe("file");
      const bytes = row!.bytes;
      captured.set(path, bytes);
      return Buffer.from(bytes);
    };
    const neutralPath = posix.join(relative(root, fixtureRoot).split(sep).join("/"), "🔣️.json"), expected = JSON.parse(read(neutralPath).toString("utf8"));
    const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json").toString("utf8")));
    expect(validate(expected), JSON.stringify(validate.errors)).toBe(true);
    expect(parseJsonc(Buffer.from(captured.get(neutralPath)!).toString("utf8"))).toEqual(expected);
    try {
      const projectPath = expected.packagePath + "/📋️project.json", projectText = read(projectPath).toString("utf8"), project = JSON.parse(projectText);
      expect(parseJsonc(projectText)).toEqual(project);
      expect(project).toEqual(expected.project);
      const packageRoot = join(root, expected.packagePath), manifestText = read(expected.packagePath + "/Cargo.toml").toString("utf8"), manifest = toml.parse(manifestText);
      expect(Bun.TOML.parse(manifestText)).toEqual(manifest);
      expect((manifest.package as { name: string }).name).toBe(expected.cargo.name);
      expect(readSemioMarker(join(packageRoot, "Cargo.toml"), "🦀️rust", schema)).toEqual({ role: expected.cargo.role, id: expected.cargo.id });
      expect((manifest.lib as { path: string }).path).toBe(expected.cargo.libPath);
      for (const row of expected.internalDependencies) {
        let table: unknown = manifest;
        for (const key of row.table) table = (table as Record<string, unknown>)[key];
        expect((table as Record<string, unknown>)[row.name], row.name).toEqual({ path: row.path, ...(row.package ? { package: row.package } : {}) });
      }
      const workspaceInput = semanticOwnedInputFileSnapshot(root, "Cargo.toml");
      expect(workspaceInput?.nodeKind).toBe("file");
      const workspaceBefore = Buffer.from(workspaceInput!.bytes), workspace = readToml<CargoWorkspaceManifest>(workspaceBefore.toString("utf8")).workspace;
      const parsedBefore = Bun.TOML.parse(workspaceBefore.toString("utf8"));
      expect("workspace" in parsedBefore ? parsedBefore.workspace : undefined).toEqual(workspace);
      expect(workspace.members.filter((path) => path === expected.packagePath)).toEqual([expected.packagePath]);
      expect(workspace.dependencies[expected.cargo.name]).toEqual({ path: expected.packagePath });
      expect(semanticOwnedInputFileSnapshot(root, expected.packagePath + "/package.json")).toBeNull();
      expect(fixedFilenameContractIdsForPath(projectPath, schema)).toContain("build-project-manifest");
      expect(fixedFilenameContractIdsForPath(expected.packagePath + "/📜️script.ts", schema)).toContain("root-script");
      expect(semanticDirectoryKindId(basename(dirname(inputRoot)), schema, { parentKindId: "members-of-modules" })).toBe("fixtures");
      expect(readdirSync(packageRoot).sort()).toEqual([...expected.packageFiles].sort());
      for (const member of expected.members) {
        expect(semanticDirectoryKindId(member.name, schema, { parentKindId: member.ownerKindId }), member.name).toBe(member.registry);
        expect(schema.semanticDirectoryMemberKinds[member.registry].memberNames.filter((name: string) => name === member.name)).toEqual([member.name]);
      }
      expect(semanticDirectoryKindId("📦️packages", schema, { parentKindId: "members-of-members-of-modules" })).toBe("packages");
      expect(semanticDirectoryKindId("🦀️rust", schema, { parentKindId: "packages" })).toBe("rust-language");
      for (const [base, operand, target] of [[expected.packagePath, expected.cargo.libPath, expected.cargo.libSourcePath], ...expected.sourceOracles.map((oracle: { importSpecifier: string; scriptPath: string }) => [expected.packagePath, oracle.importSpecifier, oracle.scriptPath]), [expected.packagePath, expected.browserHost.script, expected.browserHost.sourcePath], [expected.packagePath, expected.project.$schema, "node_modules/nx/schemas/project-schema.json"]]) {
        expect(posix.normalize(posix.join(base, operand))).toBe(target);
        expect(win32.normalize(win32.join(base, operand)).replaceAll("\\", "/")).toBe(target);
      }
      const entry = read(expected.cargo.libSourcePath).toString("utf8");
      expect(entry).not.toContain("🌉️abi");
      for (const path of [expected.browserHost.sourcePath, expected.packagePath + "/📜️script.ts"]) read(path);
      for (const oracle of expected.sourceOracles as { ownerPath: string; scriptPath: string; fixturePath: string; schemaPath: string }[]) {
        for (const path of [oracle.scriptPath, oracle.fixturePath, oracle.schemaPath]) read(path);
        for (const path of [oracle.fixturePath, oracle.schemaPath]) expect(parseJsonc(Buffer.from(captured.get(path)!).toString("utf8"))).toEqual(JSON.parse(Buffer.from(captured.get(path)!).toString("utf8")));
        for (const old of ["📜️script.ts", "🧪️fixture/🔣️.json", "🧪️schema/🔣️.json", "🧪️test/🔣️s.json"]) expect(semanticOwnedInputFileSnapshot(root, posix.join(oracle.ownerPath, old))).toBeNull();
      }
      const toolchainText = read("rust-toolchain.toml").toString("utf8"), toolchain = toml.parse(toolchainText);
      expect(Bun.TOML.parse(toolchainText)).toEqual(toolchain);
      expect((toolchain.toolchain as { targets: string[] }).targets).toContain("wasm32-unknown-unknown");
      const workspaceAfter = Buffer.from(semanticOwnedInputFileSnapshot(root, "Cargo.toml")!.bytes), current = readToml<CargoWorkspaceManifest>(workspaceAfter.toString("utf8")).workspace;
      expect(current.members.filter((path) => path === expected.packagePath)).toEqual([expected.packagePath]);
      expect(current.dependencies[expected.cargo.name]).toEqual(workspace.dependencies[expected.cargo.name]);
    } finally {
      for (const [path, bytes] of captured) expect(semanticOwnedInputFileSnapshot(root, path)?.bytes, path + " changed during metadata proof").toEqual(bytes);
    }
  });

  test("UI-host metadata router calls its real source oracle and preserves bounded native delegation", async () => {
    const ts = await import("typescript"), { spyOn } = await import("bun:test"), { pathToFileURL } = await import("node:url");
    const { semanticOwnedInputFileSnapshot } = await import("../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts");
    const root = resolve(contractRoot, "../../../../../../.."), expected = JSON.parse(readFileSync(join(inputRoot, "🖥️ui-host-package/🔣️.json"), "utf8")), packageRoot = join(root, expected.packagePath);
    const sourcePath = expected.packagePath + "/📜️script.ts", snapshot = semanticOwnedInputFileSnapshot(root, sourcePath);
    expect(snapshot?.nodeKind, sourcePath).toBe("file");
    const source = Buffer.from(snapshot!.bytes).toString("utf8"), syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const oracles = expected.sourceOracles as { scope: string; scriptPath: string; importSpecifier: string; functionName: string; fixturePath: string; schemaPath: string; arguments: unknown[] }[];
    const inputs = [sourcePath, expected.packagePath + "/Cargo.toml", expected.browserHost.sourcePath, ...oracles.flatMap((oracle) => [oracle.scriptPath, oracle.fixturePath, oracle.schemaPath])];
    const before = new Map(inputs.map((path: string) => {
      const row = semanticOwnedInputFileSnapshot(root, path); expect(row?.nodeKind, path).toBe("file"); return [path, row!.bytes] as const;
    }));
    expect(source.split(/\r?\n/u)[0]).toBe("#!/usr/bin/env bun");
    const imports = syntax.statements.filter(ts.isImportDeclaration);
    expect(imports.map(row => ({ module: (row.moduleSpecifier as import("typescript").StringLiteral).text, names: row.importClause?.namedBindings && ts.isNamedImports(row.importClause.namedBindings) ? row.importClause.namedBindings.elements.map(node => node.name.text).sort() : [] }))).toEqual(expected.moduleImports);
    expect(expected.moduleImports.flatMap((row: { names: string[] }) => row.names).sort()).toEqual(expected.imports);
    expect(syntax.statements.filter(ts.isClassDeclaration).map((node) => node.name?.text)).toEqual(["TestScript", "CheckScript", "CheckWasmScript"]);
    const originalLevel = process.env.SEMIO_TEST_LEVEL, originalCoverage = process.env.SEMIO_COVERAGE;
    let dynamicImports = 0;
    const transformed = ts.transform(syntax, [(context) => {
      const visit: import("typescript").Visitor = (node) => {
        if (ts.isImportDeclaration(node)) return undefined;
        if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword) {
          expect(node.arguments).toHaveLength(1);
          expect(ts.isStringLiteral(node.arguments[0]!)).toBe(true);
          expect(oracles.map((oracle) => oracle.importSpecifier)).toContain((node.arguments[0] as import("typescript").StringLiteral).text);
          dynamicImports++;
          return ts.factory.createCallExpression(ts.factory.createIdentifier("loadInputFixture"), undefined, [node.arguments[0]!]);
        }
        if (ts.isPropertyAccessExpression(node) && ts.isMetaProperty(node.expression)) {
          if (node.name.text === "dir") return ts.factory.createStringLiteral(packageRoot);
          if (node.name.text === "url") return ts.factory.createStringLiteral("ui-host-source-only");
          if (node.name.text === "main") return ts.factory.createTrue();
          throw new Error("Unexpected UI-host import.meta dependency");
        }
        return ts.visitEachChild(node, visit, context);
      };
      return node => ts.visitNode(node, visit) as import("typescript").SourceFile;
    }]);
    const code = ts.createPrinter().printFile(transformed.transformed[0]!); transformed.dispose();
    expect(dynamicImports).toBe(oracles.length);
    const compiled = [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext } }).outputText];
    const AsyncFunction = Object.getPrototypeOf(async () => {}).constructor;
    try {
      for (const javascript of compiled) {
        let router: ScriptRouter | undefined, refuse = false;
        const imported: string[] = [], called: string[] = [];
        const calls: unknown[][] = [];
        const dependencies = {
          BundleScript, ScriptRouter, resolve, readCargoTestPolicyV1: (environment:unknown) => {expect(environment).toBe(process.env);return {version:1};}, buildBudgetMs: buildBudgetMs, resolveTestLevel: resolveTestLevel,
          runScriptMain: async (value: ScriptRouter, ...rest: unknown[]) => { expect(rest).toEqual([]); router = value; },
          runCargoTestsV1: async (...args: unknown[]) => { calls.push(args); if (refuse) throw new Error("native delegation refused"); },
          runBudgetedTestCommand: async (...args: unknown[]) => { calls.push(args); if (refuse) throw new Error("native delegation refused"); },
          loadInputFixture: async (specifier: string) => {
            const oracle = oracles.find((row) => row.importSpecifier === specifier);
            expect(oracle, specifier).toBeDefined();
            imported.push(oracle!.scope);
            const absolute = resolve(packageRoot, specifier);
            expect(absolute).toBe(join(root, oracle!.scriptPath));
            const actual = await import(pathToFileURL(absolute).href) as Record<string, (...args: unknown[]) => unknown>;
            expect(typeof actual[oracle!.functionName]).toBe("function");
            return { [oracle!.functionName]: (...args: unknown[]) => { called.push(oracle!.scope); expect(args).toEqual(oracle!.arguments); return actual[oracle!.functionName]!(...args); } };
          },
        };
        await new AsyncFunction(...Object.keys(dependencies), javascript.replace(/^#!\/usr\/bin\/env bun\r?\n/u, ""))(...Object.values(dependencies));
        expect(router).toBeInstanceOf(ScriptRouter);
        expect(router!.usage()).toBe("bun ./📜️script.ts <test|check|check-wasm> [args…]");
        expect(imported).toEqual([]);
        for (const row of expected.nativeCases) {
          process.env.SEMIO_TEST_LEVEL = "fundamental"; delete process.env.SEMIO_COVERAGE;
          calls.length = 0;
          await router!.run(row.segments, routerInvocation("workspace-contract"));
          const args = ["--no-fail-fast", ...row.rest];
          expect(calls).toEqual([[process.execPath, [expected.browserHost.script], { cwd: packageRoot, budgetMs: buildBudgetMs() }], [{manifestPath:join(packageRoot,"Cargo.toml"),packages:[expected.cargo.name],cwd:packageRoot,extraArgs:args},{version:1}]]);
          expect(process.env.SEMIO_TEST_LEVEL).toBe(row.level);
          expect(partitionNextestExecutionFilters(args)).toEqual(row.partition);
          expect(imported).toEqual([]);
        }
        for (const row of expected.checks) {
          calls.length = 0;
          await router!.run([row.command], routerInvocation("workspace-contract"));
          expect(calls).toEqual([["cargo", row.args, { cwd: packageRoot, budgetMs: buildBudgetMs() }]]);
          expect(imported).toEqual([]);
        }
        for (const row of expected.sourceCases as { segments: string[]; scopes: string[] }[]) {
          calls.length = 0; imported.length = 0; called.length = 0;
          await router!.run(row.segments, routerInvocation("workspace-contract"));
          expect(imported).toEqual(row.scopes);
          expect(called).toEqual(row.scopes);
          expect(calls).toEqual([]);
        }
        imported.length = 0; called.length = 0;
        for (const args of expected.rejectedSegments) await expect(router!.run(args, routerInvocation("workspace-contract"))).rejects.toThrow();
        expect([imported, called]).toEqual([[], []]);
        expect(calls).toEqual([]);
        await expect(router!.run(["unknown"], routerInvocation("workspace-contract"))).rejects.toThrow('unknown command "unknown"');
        expect(calls).toEqual([]);
        refuse = true;
        await expect(router!.run(["test"], routerInvocation("workspace-contract"))).rejects.toThrow("native delegation refused");
        expect(calls).toHaveLength(1);
        calls.length = 0;
        await expect(router!.run(["check-wasm"], routerInvocation("workspace-contract"))).rejects.toThrow("native delegation refused");
        expect(calls).toHaveLength(1);
      }
    } finally {
      if (originalLevel === undefined) delete process.env.SEMIO_TEST_LEVEL; else process.env.SEMIO_TEST_LEVEL = originalLevel;
      if (originalCoverage === undefined) delete process.env.SEMIO_COVERAGE; else process.env.SEMIO_COVERAGE = originalCoverage;
      for (const [path, bytes] of before) expect(semanticOwnedInputFileSnapshot(root, path)?.bytes, path + " changed during router proof").toEqual(bytes);
    }
  });

});
