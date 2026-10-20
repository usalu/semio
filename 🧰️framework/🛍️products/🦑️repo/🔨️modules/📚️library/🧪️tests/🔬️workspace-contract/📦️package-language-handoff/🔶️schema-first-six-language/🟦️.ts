/** 🔶️ The schema first six language tests of the package language semantic handoff suite. */
import toml from "@iarna/toml";
import Ajv from "ajv";
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join, posix, resolve } from "node:path";
import { readSemioMarker } from "../../../../🔍️discovery/🔭️repository-scan/🟦️.ts";
import { validateTaxonomy, canonicalPrimaryFilenameForKind } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { pathEmojiStatuteFindings } from "@semio-tech/framework/identity/path";
import { normalizationEngineDeclarations } from "../../../../🧹️normalization/🧪️support/🏗️source-services/🟦️.ts";
import { contractRoot } from "../../🏗️shared-fixtures/🟦️.ts";
import { inputRoot, schema } from "../🟢️shared/🟦️.ts";

/** 🔷 Registers the schema first six language tests of the package language semantic handoff suite. */
export function registerSchemaFirstSixLanguageTests(): void {
  const vector = JSON.parse(readFileSync(join(inputRoot, "🔣️.json"), "utf8"));
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
    const { createTaxonomyPathMatcher } = await import("../../../../🔍️discovery/🎯️taxonomy-path-matching/🟦️.ts"), { mutationOwnerIdentity } = await import("../../../../🔍️discovery/🪪️mutation-owner-identity/🟦️.ts"), { targetInsidePackageBoundaryFinding } = await import("../../../../🔍️discovery/🗂️file-kind-resolution/🟦️.ts");
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
    const { semanticDirectoryKindId } = await import("../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts"), { semanticOwnedInputFileSnapshot } = await import("../../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts");
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
}
