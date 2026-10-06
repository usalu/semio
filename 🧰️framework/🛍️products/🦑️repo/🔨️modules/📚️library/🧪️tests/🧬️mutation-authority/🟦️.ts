import Ajv from "ajv";
import { describe, expect, test } from "bun:test";
import fastGlob from "fast-glob";
import { spawnSync } from "node:child_process";
import { chmodSync, copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative } from "node:path";
import { newScaffoldMutationTree } from "../../🏗️authoring/🧬️mutation-tree/🟦️.ts";
import { gitSpawnEnv } from "../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { loadTaxonomy, canonicalPrimaryFilenameForKind, inspectRustModuleGraphFacts, inspectRustMutationAggregateSpan, inspectRustStructure } from "../../🔍️discovery/🟦️.ts";
import { inspectMutationRootReachability, policyMutationStructuralBreaches } from "../../🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts";
import { inventoryMutationTaxonomy } from "../../🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts";
import { mutationRoot, prepareMutationFixtureRoot, mutationFixtureRoot } from "./🧫️fixture/🟦️.ts";

describe("direct mutation ownership", () => {
  test("prepares AST-safe direct mutation scaffolds before one guarded publication", () => {
    const golden = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🏗️mutation-scaffolding/🔣️.json"), "utf8")) as { schemaVersion: number; mutationRoot: string; name: string; attributedAggregate: string; malformedAggregate: string; ambiguousAggregate: string; wrongMountAggregate: string; privateMountAggregate: string; wrongVariantAggregate: string; scopedAggregate: string; unrelatedDocAggregate: string; nestedAggregateDecoy: string; nestedAggregateScopes: string; unmatchedAggregate: string; unprovenScopeAggregate: string; unprovenMountAggregate: string };
    
    const root = mutationFixtureRoot("semio-mutation-scaffold-transaction-");
    const snapshot = (): string => fastGlob.sync("**/*", { cwd: root, onlyFiles: true, followSymbolicLinks: false }).sort().map((path) => `${path}\0${readFileSync(join(root, path), "utf8")}`).join("\0");
    const mutationRoot = join(root, golden.mutationRoot);
    const aggregate = join(mutationRoot, "🦀️.rs");
    const scaffold = (source: string, options: Record<string, unknown> = {}, dryRun = false, name = golden.name) => {
      mkdirSync(mutationRoot, { recursive: true });
      writeFileSync(aggregate, source);
      return newScaffoldMutationTree(root, golden.mutationRoot, name, options as never, dryRun);
    };
    try {
      
      expect(inspectRustMutationAggregateSpan(golden.attributedAggregate)).toMatchObject({ enumName: "ProbeMutation" });
      expect(inspectRustMutationAggregateSpan(golden.scopedAggregate)).toMatchObject({ declarationStart: 0, enumName: "ProbeMutation" });
      expect(inspectRustMutationAggregateSpan(golden.unrelatedDocAggregate)).toMatchObject({ declarationStart: golden.unrelatedDocAggregate.indexOf("#[derive") });
      expect(inspectRustMutationAggregateSpan(golden.nestedAggregateScopes)).toBeNull();
      for (const source of [golden.malformedAggregate, golden.ambiguousAggregate, golden.nestedAggregateDecoy, golden.nestedAggregateScopes, golden.unmatchedAggregate]) {
        mkdirSync(mutationRoot, { recursive: true });
        writeFileSync(aggregate, source);
        const before = snapshot();
        expect(() => newScaffoldMutationTree(root, golden.mutationRoot, golden.name)).toThrow(/aggregate/i);
        expect(snapshot()).toBe(before);
      }
      for (const source of [golden.wrongMountAggregate, golden.privateMountAggregate, golden.wrongVariantAggregate]) {
        writeFileSync(aggregate, source);
        const before = snapshot();
        expect(() => newScaffoldMutationTree(root, golden.mutationRoot, golden.name)).toThrow(/existing (mount|variant)/i);
        expect(snapshot()).toBe(before);
      }
      for (const source of [golden.unprovenScopeAggregate, golden.unprovenMountAggregate]) {
        writeFileSync(aggregate, source);
        const before = snapshot();
        expect(() => newScaffoldMutationTree(root, golden.mutationRoot, golden.name)).toThrow(/scope|mount/i);
        expect(snapshot()).toBe(before);
      }
      writeFileSync(aggregate, golden.attributedAggregate);
      const beforeDryRun = snapshot();
      const preview = newScaffoldMutationTree(root, golden.mutationRoot, golden.name, {}, true);
      expect(preview.created).toContain(`${golden.mutationRoot}/${golden.name}/🦀️.rs`);
      expect(snapshot()).toBe(beforeDryRun);
      mkdirSync(join(mutationRoot, golden.name, "🦀️.rs"), { recursive: true });
      const beforeBlockedTarget = snapshot();
      expect(() => newScaffoldMutationTree(root, golden.mutationRoot, golden.name)).toThrow(/target is not a regular file/i);
      expect(snapshot()).toBe(beforeBlockedTarget);
      rmSync(join(mutationRoot, golden.name), { recursive: true, force: true });
      newScaffoldMutationTree(root, golden.mutationRoot, golden.name);
      const mounted = readFileSync(aggregate, "utf8");
      expect(mounted.indexOf("pub mod insert_page;")).toBeLessThan(mounted.indexOf("#[derive(Clone, Debug)]"));
      expect(mounted).toContain("InsertPage(insert_page::Mutation)");
      const parser = spawnSync("rustc", ["-Zunpretty=ast-tree", "--crate-name", "mutation_scaffold_probe", "--edition", "2021", "-"], { encoding: "utf8", input: mounted });
      expect(parser.status).toBe(0);
      writeFileSync(join(mutationRoot, golden.name, "🦀️.rs"), "pub struct Handwritten;\n");
      newScaffoldMutationTree(root, golden.mutationRoot, golden.name, {}, false);
      expect(readFileSync(join(mutationRoot, golden.name, "🦀️.rs"), "utf8")).toBe("pub struct Handwritten;\n");
      const unrelated = join(root, "unrelated.txt");
      expect(() => scaffold(golden.attributedAggregate, { cancelled: () => { writeFileSync(unrelated, "kept\n"); return true; } }, false, "➖️remove-page")).toThrow(/cancel/i);
      expect(readFileSync(unrelated, "utf8")).toBe("kept\n");
      expect(existsSync(join(mutationRoot, "➖️remove-page"))).toBe(false);
      writeFileSync(aggregate, golden.attributedAggregate);
      const beforeFailure = snapshot();
      let checks = 0;
      expect(() => newScaffoldMutationTree(root, golden.mutationRoot, "✏️edit-page", { cancelled: () => {
        checks += 1;
        if (checks === 5) writeFileSync(aggregate, "pub enum ConcurrentMutation {}\n");
        return false;
      } }, false)).toThrow(/changed during publication/i);
      expect(readFileSync(aggregate, "utf8")).toBe("pub enum ConcurrentMutation {}\n");
      expect(existsSync(join(mutationRoot, "✏️edit-page"))).toBe(false);
      expect(snapshot()).not.toBe(beforeFailure);
      expect(snapshot()).toContain("🦀️.rs\u0000pub enum ConcurrentMutation {}\n");
      writeFileSync(aggregate, golden.attributedAggregate);
      rmSync(join(mutationRoot, golden.name), { recursive: true, force: true });
      symlinkSync(join(root, "missing-leaf"), join(mutationRoot, golden.name), process.platform === "win32" ? "junction" : "file");
      const beforeDanglingLink = snapshot();
      expect(() => newScaffoldMutationTree(root, golden.mutationRoot, golden.name)).toThrow(/symlink/i);
      expect(snapshot()).toBe(beforeDanglingLink);
      rmSync(join(mutationRoot, golden.name));
      const linkedRoot = `${root}-linked`;
      symlinkSync(root, linkedRoot, process.platform === "win32" ? "junction" : "dir");
      expect(() => newScaffoldMutationTree(linkedRoot, golden.mutationRoot, golden.name)).toThrow(/repository root.*regular directory/i);
      rmSync(linkedRoot);
      expect(() => newScaffoldMutationTree(root, "../../📦️packages/escape/🧬️mutations", golden.name)).toThrow(/scope/i);
      expect(() => newScaffoldMutationTree(root, `compose/${golden.mutationRoot}`, golden.name)).toThrow(/scope.*excluded/i);
      expect(() => newScaffoldMutationTree(root, "✏️s/🧪️scaffold/not-a-mutation-owner", golden.name)).toThrow(/scope/i);
      const linked = join(root, "✏️s", "🧪️linked");
      mkdirSync(dirname(linked), { recursive: true });
      symlinkSync(mutationRoot, linked, process.platform === "win32" ? "junction" : "dir");
      expect(() => newScaffoldMutationTree(root, "✏️s/🧪️linked/🧬️mutations", golden.name)).toThrow(/scope/i);
    } finally {
      try { chmodSync(mutationRoot, 0o755); } catch {}
      rmSync(root, { recursive: true, force: true });
    }
  }, 30_000);

  test("resolves mutation consumers and schema-validated assignment evidence from a stable source index", () => {
    const root = mutationFixtureRoot("semio-mutation-inventory-consumers-");
    expect(spawnSync("git", ["init", "--quiet"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
    for (const relative of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️.json"]) {
      mkdirSync(join(root, dirname(relative)), { recursive: true });
      copyFileSync(join(getWorkspaceRoot(), relative), join(root, relative));
    }
    const fixturePath = join(import.meta.dir, "../../🧫️fixtures/📋️mutation-inventory/🧪️consumers/🔣️.json");
    const inventorySchemaPath = join(import.meta.dir, "../../🧬️schema/📋️mutation-inventory/🔣️.json");
    const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { schemaVersion: number; assignmentLedger: unknown; files: readonly { path: string; content: string }[] };
    const fixtureSchema = { type: "object", required: ["schemaVersion", "assignmentLedger", "files"], properties: { schemaVersion: { const: 1 }, assignmentLedger: { type: "object" }, files: { type: "array", minItems: 2, items: { type: "object", required: ["path", "content"], properties: { path: { type: "string" }, content: { type: "string" } }, additionalProperties: false } } }, additionalProperties: false };
    const taxonomy = loadTaxonomy();
    const rustFilename = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🦀️rust"]!, taxonomy);
    const typescriptFilename = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🟦️typescript"]!, taxonomy);
    const jsonFilename = canonicalPrimaryFilenameForKind("json", taxonomy);
    const taxonomyPath = (path: string) => path.replaceAll("🦀️.rs", rustFilename).replaceAll("🟦️.ts", typescriptFilename).replaceAll("🔣️.json", jsonFilename);
    try {
      expect(new Ajv({ strict: true }).compile(fixtureSchema)(fixture)).toBe(true);
      for (const { path, content } of fixture.files) {
        const target = join(root, taxonomyPath(path));
        mkdirSync(dirname(target), { recursive: true });
        writeFileSync(target, content.replaceAll("🦀️.rs", rustFilename).replaceAll("🟦️.ts", typescriptFilename).replaceAll("🔣️.json", jsonFilename));
      }
      const inventory = inventoryMutationTaxonomy(root, { assignmentLedger: fixture.assignmentLedger, assignmentLedgerPath: "ticket/📋️mutation-assignments.json" });
      const inventorySchema = JSON.parse(readFileSync(inventorySchemaPath, "utf8"));
      expect(new Ajv({ strict: true }).compile(inventorySchema)(JSON.parse(JSON.stringify(inventory)))).toBe(true);
      const sourceRoster = fastGlob.sync("**/*", { cwd: root, onlyFiles: true, followSymbolicLinks: false, dot: true }).filter((path) => !path.startsWith(".git/") && !path.startsWith("compose/")).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
      expect(inventory.sourceRoster.filter(({ role }) => role === "source").map(({ path }) => path)).toEqual(sourceRoster);
      expect(inventory.sourceRoster.some(({ path }) => path.startsWith("compose/"))).toBe(false);
      const alpha = inventory.records.find(({ targetMutationDirectoryName }) => targetMutationDirectoryName === "➕️insert-page")!;
      const beta = inventory.records.find(({ targetMutationDirectoryName }) => targetMutationDirectoryName === "✏️change-page")!;
      const remove = inventory.records.find(({ targetMutationDirectoryName }) => targetMutationDirectoryName === "➖️remove-page")!;
      expect(alpha.structuralState).toBe("direct");
      expect(alpha.executionState).toBe("assigned");
      expect(beta.executionState).toBe("conflicting");
      expect(remove.executionState).toBe("unassigned");
      expect(alpha.assignmentEvidence).toMatchObject({ status: "resolved", ledgerPath: null, rows: [{ terraExecutor: "TERRA-ALPHA-INSERT" }] });
      expect(beta.assignmentEvidence).toMatchObject({ status: "conflicting" });
      expect(remove.assignmentEvidence).toMatchObject({ status: "missing" });
      expect(alpha.consumerEdges).toEqual(expect.arrayContaining([
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🎮️command/🦀️.rs"), targetPath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➕️insert-page/🦀️.rs"), kind: "command", relation: "import" }),
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/✏️editor/🟦️.ts"), targetPath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➕️insert-page/🟦️.ts"), kind: "editor", relation: "import" }),
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/👁️viewer/🟦️.ts"), targetPath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➕️insert-page/🟦️.ts"), kind: "viewer", relation: "import" }),
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/📚️catalog/🦀️.rs"), kind: "catalog", relation: "reexport" }),
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/📋️registry/🦀️.rs"), kind: "registry", relation: "reexport" }),
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️operations/🦀️.rs"), kind: "sibling-operation" }),
        expect.objectContaining({ sourcePath: "✏️s/🔌️plugins/🅰️alpha/🧪️tests/two-file/command.rs", targetPath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➕️insert-page/🦀️.rs"), kind: "test", relation: "import" }),
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/📦️crate/command.rs"), targetPath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➕️insert-page/🦀️.rs"), kind: "leaf", relation: "import" }),
        expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➕️insert-page/🦀️.rs"), targetPath: taxonomyPath("🧰️framework/🔨️modules/🅱️beta/🧬️mutations/✏️change-page/🦀️.rs"), kind: "cross-owner", relation: "import" }),
      ]));
      expect(alpha.consumerEdges.some(({ sourcePath }) => sourcePath.endsWith("🧪️identity-controls.rs"))).toBe(false);
      expect(alpha.schemaAndLanguageSurfaces).toEqual([taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➕️insert-page/🟦️.ts")]);
      expect(beta.schemaAndLanguageSurfaces).toEqual([taxonomyPath("🧰️framework/🔨️modules/🅱️beta/🧬️mutations/✏️change-page/🔣️.json")]);
      expect(alpha.sharedHelpers).toEqual(["✏️s/🔌️plugins/🅰️alpha/🧰️helpers/🦀️page.rs"]);
      expect(alpha.crossOwnerDependencies).toEqual([taxonomyPath("🧰️framework/🔨️modules/🅱️beta/🧬️mutations/✏️change-page/🦀️.rs")]);
      expect(beta.consumerEdges).toEqual(expect.arrayContaining([expect.objectContaining({ sourcePath: taxonomyPath("✏️s/🔌️plugins/🅰️alpha/📦️crate/🎮️command/🦀️.rs"), kind: "command", relation: "import" })]));
      expect(inventory.unresolved).toEqual(expect.arrayContaining([
        expect.objectContaining({ path: "✏️s/🔌️plugins/🅰️alpha/🧬️mutations/➖️remove-page", reason: expect.stringMatching(/assignment/i) }),
        expect.objectContaining({ path: "🧰️framework/🔨️modules/🅱️beta/🧬️mutations/✏️change-page", reason: expect.stringMatching(/conflicting/i) }),
      ]));
      const digest = inventory.sourceTreeDigest;
      writeFileSync(join(root, taxonomyPath("✏️s/🔌️plugins/🅰️alpha/🎮️command/🦀️.rs")), `#[path = "../../📦️packages/🧬️mutations/➕️insert-page/${rustFilename}"] mod insert_page;\nuse insert_page::Mutation;\n// byte-only external consumer edit\n`);
      expect(inventoryMutationTaxonomy(root, { assignmentLedger: fixture.assignmentLedger, assignmentLedgerPath: "ticket/📋️mutation-assignments.json" }).sourceTreeDigest).not.toBe(digest);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  }, 30_000);

  test("proves direct leaf reachability through exact public canonical mounts and wrapped types", () => {
    const fixturePath = join(import.meta.dir, "../../🧫️fixtures/📡️mutation-reachability/🔣️.json");
    
    const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { cases: readonly { name: string; source: string; leafSource?: string; extraFiles?: readonly { path: string; source: string }[]; accepted: boolean; nativeAccepted?: true }[] };
    
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (artifactRoot) mkdirSync(artifactRoot, { recursive: true });
    const root = mkdtempSync(join(artifactRoot ?? tmpdir(), "semio-mutation-reachability-"));
    const taxonomy = loadTaxonomy();
    const rust = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🦀️rust"]!, taxonomy);
    try {
      for (const vector of fixture.cases) {
        const caseRoot = join(root, vector.name);
        mkdirSync(caseRoot, { recursive: true });
        prepareMutationFixtureRoot(caseRoot);
        const mutations = mutationRoot(caseRoot);
        mkdirSync(join(mutations, "➕️insert-page"), { recursive: true });
        writeFileSync(join(mutations, "➕️insert-page", rust), (vector.leafSource ?? "pub struct Mutation;\n").replaceAll("🦀️.rs", rust));
        for (const file of vector.extraFiles ?? []) { const target = join(mutations, "➕️insert-page", file.path.replaceAll("🦀️.rs", rust)); mkdirSync(dirname(target), { recursive: true }); writeFileSync(target, file.source); }
        writeFileSync(join(mutations, rust), vector.source.replaceAll("🦀️.rs", rust));
        if (vector.nativeAccepted || ["public-canonical", "semantic-type-alias", "child-facet-reexport"].includes(vector.name)) {
          const out = join(caseRoot, "🧪️compiler-artifacts");
          mkdirSync(out, { recursive: true });
          const compiled = spawnSync("rustc", ["--edition=2021", "--crate-name", "reachability_probe", "--crate-type", "lib", "--out-dir", out, join(mutations, rust)], { encoding: "utf8", timeout: 30_000 });
          writeFileSync(join(out, "compiler.log"), `${compiled.stdout}\n${compiled.stderr}`);
          if (compiled.status !== 0) throw new Error(`${vector.name}: ${compiled.stderr || compiled.error}`);
          console.log("[DEBUG] reachability native oracle; id=" + vector.name + "; status=" + compiled.status);
        }
        const relativeRoot = relative(caseRoot, mutations).replaceAll("\\", "/");
        const reaches = policyMutationStructuralBreaches(caseRoot, [relativeRoot]).some(({ kind, scope }) => (kind === "mutation/reachability" && scope === `${relativeRoot}/➕️insert-page/${rust}`) || kind === "mutation/folder-variant-bijection");
        expect(reaches).toBe(!vector.accepted);
      }
    } finally { if (!artifactRoot) rmSync(root, { recursive: true, force: true }); }  }, 60000);

  test("projects the actual wrapped mutation declaration origin through public aliases only", () => {
    const fixturePath = join(import.meta.dir, "../../🧫️fixtures/🧬️mutation-type-origin/🔣️.json");
    
    const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { schemaVersion: 1; mutationRoot: string; leaf: string; rustFilename: "🦀️.rs"; cases: readonly { id: string; mutationRoot?: string; leaf?: string; rustFilename?: string; virtualFilesystem?: true; repoRoot?: string; repositoryRootSymlink?: true; repositoryAncestorSymlink?: true; rootSource: string; leafSource: string; extraFiles?: readonly { path: string; source: string }[]; links?: readonly { path: string; target: string }[]; compileAccepted: boolean; expected: { sourcePath: string; declarationName: string; modulePath: string[] } | null }[] };
    
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (artifactRoot) mkdirSync(artifactRoot, { recursive: true });
    const root = mkdtempSync(join(artifactRoot ?? tmpdir(), "semio-mutation-type-origin-"));
    try {
      for (const vector of fixture.cases) {
        const rust = vector.rustFilename ?? fixture.rustFilename, mutationRoot = vector.mutationRoot ?? fixture.mutationRoot, leafName = vector.leaf ?? fixture.leaf, caseDirectory = join(root, vector.id);
        const sourceRoot = vector.repositoryRootSymlink ? join(caseDirectory, "source") : vector.repositoryAncestorSymlink ? join(caseDirectory, "source", "workspace") : vector.repoRoot ?? caseDirectory;
        const caseRoot = vector.repositoryRootSymlink ? join(caseDirectory, "repo") : vector.repositoryAncestorSymlink ? join(caseDirectory, "alias", "workspace") : sourceRoot;
        const mutations = join(sourceRoot, mutationRoot), leaf = join(mutations, leafName);
        if (!vector.virtualFilesystem) {
          mkdirSync(leaf, { recursive: true });
          prepareMutationFixtureRoot(sourceRoot);
          writeFileSync(join(mutations, rust), vector.rootSource.replaceAll("🦀️.rs", rust));
          writeFileSync(join(leaf, rust), vector.leafSource.replaceAll("🦀️.rs", rust));
          for (const file of vector.extraFiles ?? []) {
            const target = join(leaf, file.path.replaceAll("🦀️.rs", rust));
            mkdirSync(dirname(target), { recursive: true });
            writeFileSync(target, file.source.replaceAll("🦀️.rs", rust));
          }
          for (const link of vector.links ?? []) {
            const path = join(leaf, link.path.replaceAll("🦀️.rs", rust));
            mkdirSync(dirname(path), { recursive: true });
            symlinkSync(process.platform === "win32" ? join(dirname(path), link.target.replaceAll("🦀️.rs", rust)) : link.target.replaceAll("🦀️.rs", rust), path, "file");
          }
          if (vector.repositoryRootSymlink) symlinkSync(process.platform === "win32" ? sourceRoot : "source", caseRoot, process.platform === "win32" ? "junction" : "dir");
          if (vector.repositoryAncestorSymlink) symlinkSync(process.platform === "win32" ? join(caseDirectory, "source") : "source", join(caseDirectory, "alias"), process.platform === "win32" ? "junction" : "dir");
        }
        const graph = inspectRustModuleGraphFacts(vector.rootSource.replaceAll("🦀️.rs", rust));
        expect(graph.modules.find((module) => module.name === "insert_page")?.conditional === true, vector.id).toBe(["conditional-mount", "inner-conditional-mount", "cfg-attr-conditional-mount", "root-inner-cfg", "unproven-root-body", "known-and-unproven-mount"].includes(vector.id));
        const structure = inspectRustStructure(vector.rootSource.replaceAll("🦀️.rs", rust)), aggregate = structure.enums.filter((item) => item.name === "ProbeMutation"), variant = aggregate.flatMap((item) => item.variants).filter((item) => item.name === "InsertPage");
        expect(aggregate.some((item) => item.conditional === true), vector.id).toBe(["inner-conditional-mount", "disabled-aggregate-cfg", "conditional-aggregate-cfg-attr", "root-inner-cfg", "conditional-ancestor-inline-enum", "unproven-root-body"].includes(vector.id));
        expect(variant.some((item) => item.conditional === true), vector.id).toBe(["inner-conditional-mount", "disabled-aggregate-cfg", "conditional-aggregate-cfg-attr", "root-inner-cfg", "disabled-variant-cfg", "conditional-variant-cfg-attr", "conditional-ancestor-inline-enum", "unproven-root-body"].includes(vector.id));
        if (vector.compileAccepted) {
          const out = join(caseRoot, "🧪️rustc");
          mkdirSync(out, { recursive: true });
          const compiled = spawnSync("rustc", ["--edition=2021", "--crate-name", "wrapped_type_origin_probe", "--crate-type", "lib", "--out-dir", out, join(mutations, rust)], { encoding: "utf8", timeout: 30_000 });
          writeFileSync(join(out, "📓️compiler.md"), `# Rustc ${vector.id}\n\n\`\`\`text\n${compiled.stdout}${compiled.stderr}\n\`\`\`\n`);
          expect(compiled.status, vector.id).toBe(0);
          console.log("[DEBUG] type-origin native oracle; id=" + vector.id + "; status=" + compiled.status);
        }
        const relativeMutations = mutationRoot;
        const proof = inspectMutationRootReachability(caseRoot, relativeMutations, vector.rootSource.replaceAll("🦀️.rs", rust), [leafName], rust);
        expect(proof).toHaveLength(1);
        expect(proof[0]!.origin, vector.id).toEqual(vector.expected === null ? null : { ...vector.expected, sourcePath: vector.expected.sourcePath.replaceAll("🦀️.rs", rust) });
        expect(proof[0]!.wrapped, vector.id).toBe(vector.expected !== null);
      }
    } finally { if (!artifactRoot) rmSync(root, { recursive: true, force: true }); }  }, 60000);
});
