/** 🔸️ The requires owned generator expose tests of the loadTaxonomy suite. */
import { requireRecord } from "@semio-tech/schema-validator-ts";
import Ajv from "ajv";
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../../../🗂️workspaces/🟦️.ts";
import { loadTaxonomy, validateTaxonomy, validateGeneratorContractsAgainstWorkspace } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { findWorkspaceRoot } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { generatorBuildPreviewCommand } from "../../../../🔍️discovery/🛠️generator-commands/🟦️.ts";
import { type Taxonomy } from "../../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { generatorInputPaths } from "../../../../🧹️normalization/🏭️generator-inputs/🟦️.ts";
import { parseGeneratorPreviewManifest } from "../../../../🧹️normalization/🧾️plan-parsing/🟦️.ts";
import { loadTaxonomy as loadSourceTaxonomy } from "../../../../🧹️normalization/🔣️taxonomy/🟦️.ts";
import { normalizationEngineDeclarations } from "../../../../🧹️normalization/🧪️support/🏗️source-services/🟦️.ts";
import { contractRoot } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🔹 Registers the requires owned generator expose tests of the loadTaxonomy suite. */
export function registerRequiresOwnedGeneratorExposeTests(): void {
  test("requires every owned generator to expose one exact read-only preview target", async () => {
    const taxonomy = loadTaxonomy();
    const owned = Object.entries(taxonomy.generatorContracts).filter(([, contract]) => contract.ownership === "owned");
    const external = Object.values(taxonomy.generatorContracts).filter((contract) => contract.ownership === "external");
    const inputRoot = join(contractRoot, "../🧫️fixtures/🏭️owned-generator-preview-inventory"), input = readFileSync(join(inputRoot, "🔣️.json"), "utf8");
    const vector = JSON.parse(input) as { schemaVersion: number; contract: string; ownedGeneratorIds: string[] };
    expect(vector.ownedGeneratorIds).toEqual([...vector.ownedGeneratorIds].sort());
    const jsonc = await import("jsonc-parser"), inputErrors: import("jsonc-parser").ParseError[] = [];
    expect(jsonc.parse(input, inputErrors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
    expect(inputErrors).toEqual([]);
    const members = (node: import("jsonc-parser").Node | undefined): Map<string, import("jsonc-parser").Node> => {
      if (node?.type !== "object") throw new Error("Expected an exact JSON object");
      const result = new Map<string, import("jsonc-parser").Node>();
      for (const property of node.children ?? []) {
        const [key, value] = property.children ?? [];
        if (property.type !== "property" || key?.type !== "string" || !value || result.has(key.value)) throw new Error("Duplicate or malformed JSON property");
        result.set(key.value, value);
      }
      return result;
    };
    const oracleIds = (source: string): string[] => {
      const errors: import("jsonc-parser").ParseError[] = [], tree = jsonc.parseTree(source, errors, { disallowComments: true, allowTrailingComma: false });
      if (errors.length) throw new Error("Invalid JSON authority");
      return [...members(members(tree).get("generatorContracts"))].flatMap(([id, contract]) => {
        const ownership = members(contract).get("ownership");
        if (ownership?.type !== "string") throw new Error("Missing ownership authority");
        return ownership.value === "owned" ? [id] : [];
      }).sort();
    };
    const expectedIds = vector.ownedGeneratorIds, actualIds = owned.map(([id]) => id).sort();
    expect(actualIds).toEqual(expectedIds);
    expect(oracleIds(readFileSync(join(contractRoot, "../../🔣️taxonomy.json"), "utf8"))).toEqual(expectedIds);
    for (const altered of [actualIds.slice(1), [...actualIds, "unexpected-generator"], ["unexpected-generator", ...actualIds.slice(1)]]) expect(altered.sort()).not.toEqual(expectedIds);
    for (const malformed of ['{"generatorContracts":{},"generatorContracts":{}}', '{"generatorContracts":{"duplicate":{"ownership":"owned"},"duplicate":{"ownership":"owned"}}}', '{"generatorContracts":{"single":{"ownership":"external","ownership":"owned"}}}']) expect(() => oracleIds(malformed)).toThrow("Duplicate or malformed JSON property");
    for (const [, contract] of owned) {
      const project = contract.target!.slice(0, contract.target!.lastIndexOf(":"));
      if (!contract.previewArguments) expect(contract.previewTarget).toBe(`${project}:preview-generated`);
      else expect(contract.previewTarget!.slice(0, contract.previewTarget!.lastIndexOf(":"))).toBe(project);
      expect(generatorBuildPreviewCommand(contract)).toEqual(["repo", "run", contract.previewTarget!]);
    }
    expect(external.every((contract) => contract.previewTarget === undefined)).toBe(true);
    expect(validateGeneratorContractsAgainstWorkspace(getWorkspaceRoot(), taxonomy)).toEqual([]);
  });
  test("admits only explicit same-project preview invocations", async () => {
    const root = join(contractRoot, "../🧫️fixtures/🏭️owned-generator-preview-inventory");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const taxonomy = loadTaxonomy(), actor = taxonomy.generatorContracts["actor-typegen"]!;
    for (const row of fixture.previewRoutes) {
      const contract = { ...actor, target: row.target, checkTarget: undefined, previewTarget: row.previewTarget, previewArguments: row.previewArguments };
      const changed = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "actor-typegen": contract } };
      expect(validateTaxonomy(changed).length === 0, JSON.stringify(row)).toBe(row.valid);
    }
  }, 30_000);
  test("rejects missing, external, and non-canonical generator preview targets", () => {
    const taxonomy = loadTaxonomy();
    const actor = taxonomy.generatorContracts["actor-typegen"]!;
    expect(actor.ownerPath).toBe("🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust");
    expect(actor.previewLimits?.timeoutMs).toBe(600_000);
    const locks = taxonomy.generatorContracts["external-cargo-locks"]!;
    const missing = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "actor-typegen": { ...actor, previewTarget: undefined } } } as unknown as Taxonomy;
    const external = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "external-cargo-locks": { ...locks, previewTarget: "workspace:preview-generated" } } } as unknown as Taxonomy;
    const nonCanonical = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "actor-typegen": { ...actor, previewTarget: "@semio-tech/framework-actor-rs:preview" } } } as unknown as Taxonomy;
    expect(validateTaxonomy(missing).some((problem) => problem.includes("previewTarget is required"))).toBe(true);
    expect(validateTaxonomy(external).some((problem) => problem.includes("previewTarget is forbidden"))).toBe(true);
    expect(validateTaxonomy(nonCanonical).some((problem) => problem.includes("must be the exact owner preview-generated target"))).toBe(true);
  });
  test("bounds explicit preview resources and invokes only the declared owner command", async () => {
    const fixture = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🏭️owned-generator-preview-inventory/🔣️.json"), "utf8"));
    const { generatorPreviewExecution } = await import("../../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts"), { generatorPreviewResourceLimits } = await import("../../../../🔍️discovery/🛠️generator-commands/🟦️.ts"), ts = await import("typescript");
    const limitSchema = { type: "object", additionalProperties: false, required: ["maxOutputBytes", "timeoutMs"], properties: { maxOutputBytes: { type: "integer", minimum: 1024, maximum: 536870912 }, timeoutMs: { type: "integer", minimum: 1000, maximum: 600000 } } };
    const oracle = new Ajv({ strict: true }).compile(limitSchema);
    const taxonomy = loadTaxonomy(), actor = taxonomy.generatorContracts["actor-typegen"]!;
    for (const row of fixture.previewLimits) {
      expect(oracle(row.limits), JSON.stringify(row)).toBe(row.valid);
      expect(validateTaxonomy({ ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "actor-typegen": { ...actor, previewLimits: row.limits } } }).length === 0).toBe(row.valid);
    }
    const source = normalizationEngineDeclarations(join(contractRoot, "../../🧹️normalization")), syntax = ts.createSourceFile("normalization.ts", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const names = ["assertGeneratorPreviewTarget", "invokeGeneratorPreview"];
    const selected = names.map(name => syntax.statements.find(statement => ts.isFunctionDeclaration(statement) && statement.name?.text === name)?.getText(syntax));
    expect(selected.every(Boolean)).toBe(true);
    const contract = { ...actor, ownerPath: "compiler", target: "@neutral/compiler:generate-browser", previewTarget: "@neutral/compiler:preview-browser", previewArguments: ["browser", "preview"], previewLimits: fixture.previewLimits[0].limits };
    for (const javascript of [new Bun.Transpiler({ loader: "ts" }).transformSync(selected.join("\n") + "\nreturn invokeGeneratorPreview;"), ts.transpileModule(selected.join("\n") + "\nreturn invokeGeneratorPreview;", { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
      let command = "repo ./📜️script.ts browser preview", cwd = "compiler";
      const invocations: unknown[] = [];
      const invoke = new Function("buildTargetRecord", "requireRecord", "generatorPreviewExecution", "generatorPreviewResourceLimits", "receiveGeneratorPreviewProgressV1", "checkCancellation", "absolutePath", "spawnSync", "parseGeneratorPreviewManifest", "generatorPathCompare", "sha256", javascript)(() => ({ executor: "repo:script", options: { cwd, command } }), requireRecord, generatorPreviewExecution, (generatorPreviewResourceLimits), (await import("../../../../🏭️generator/👁️preview/📈️progress/🟦️.ts")).receiveGeneratorPreviewProgressV1, () => {}, join, (executable: string, args: string[], options: unknown) => { invocations.push({ executable, args, options }); return { stdout: "{}\n", stderr: "", status: 0, signal: null }; }, () => ({}), (left: string, right: string) => left.localeCompare(right), () => "digest");
      invoke({ repoRoot: "/neutral" }, "neutral", contract, { exclusions: [] });
      expect(invocations).toMatchObject([{ executable: "bun", args: ["./📜️script.ts", "browser", "preview"], options: { cwd: "/neutral/compiler", maxBuffer: 268435456, timeout: 240000 } }]);
      command = fixture.ownerExecutionRoutes.find((row: { id: string }) => row.id === "native-owner").command.replace("preview-generated", "browser preview");
      cwd = ".";
      invoke({ repoRoot: "/neutral" }, "neutral", contract, { exclusions: [] });
      expect(invocations[1]).toMatchObject({ executable: "bun", args: command.split(" ").slice(1), options: { cwd: "/neutral", maxBuffer: 268435456, timeout: 240000 } });
      command = "repo ./📜️script.ts preview-generated";
      expect(() => invoke({ repoRoot: "/neutral" }, "neutral", contract, { exclusions: [] })).toThrow("exact owner JSON preview");
      expect(invocations).toHaveLength(2);
    }
  });
  test("the actual empty General publisher passes the strict normalization receiver", async () => {
    const workspace = getWorkspaceRoot(), discovery = { ...(await import("../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts")), ...(await import("../../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts")), ...(await import("../../../../🔍️discovery/🛠️generator-commands/🟦️.ts")) }, progress = await import("../../../../🏭️generator/👁️preview/📈️progress/🟦️.ts"), ts = await import("typescript"), esbuild = await import("esbuild");
    const source = normalizationEngineDeclarations(join(contractRoot, "../../🧹️normalization")), syntax = ts.createSourceFile("normalization.ts", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), names = ["assertGeneratorPreviewTarget", "invokeGeneratorPreview"], functions = names.map(name => syntax.statements.find(statement => ts.isFunctionDeclaration(statement) && statement.name?.text === name)?.getText(syntax));
    expect(functions.every(Boolean)).toBe(true);
    const contract = discovery.loadCatalogTaxonomy().generatorContracts["playground-session"]!, target = JSON.parse(readFileSync(join(workspace, contract.ownerPath!, "📋️project.json"), "utf8")).targets[contract.previewTarget!.slice(contract.previewTarget!.lastIndexOf(":") + 1)], output = join(workspace, contract.outputRoots[0].path), before = readFileSync(output), code = functions.join("\n") + "\nreturn invokeGeneratorPreview;";
    expect(contract.previewProgress).toBeUndefined();
    for (const javascript of [ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText, (await esbuild.transform(code, { loader: "ts", target: "es2022" })).code]) {
      const controller = new AbortController();
      const invoke = new Function("buildTargetRecord", "requireRecord", "generatorPreviewExecution", "generatorPreviewResourceLimits", "receiveGeneratorPreviewProgressV1", "checkCancellation", "absolutePath", "spawnSync", "parseGeneratorPreviewManifest", "generatorPathCompare", "sha256", javascript)(() => target, requireRecord, discovery.generatorPreviewExecution, discovery.generatorPreviewResourceLimits, progress.receiveGeneratorPreviewProgressV1, () => { if (controller.signal.aborted) throw new Error("owned test cancellation"); }, join, spawnSync, parseGeneratorPreviewManifest, (left: string, right: string) => Buffer.from(left).compare(Buffer.from(right)), (text: string) => createHash("sha256").update(text).digest("hex"));
      const actual = invoke({ repoRoot: workspace }, "playground-session", contract, { exclusions: [] });
      expect(actual.manifest.contractId).toBe("playground-session");
      expect(actual.manifest.nodes).toHaveLength(1);
      expect(Buffer.from(actual.manifest.nodes[0].bytesBase64, "base64")).toEqual(before);
      expect(readFileSync(output)).toEqual(before);
      controller.abort();expect(() => invoke({ repoRoot: workspace }, "playground-session", contract, { exclusions: [] })).toThrow("owned test cancellation");
      expect(controller.signal.aborted).toBe(true);
    }
  }, 120000);
  test("routes native generator previews through their declared owner", async () => {
    const root = join(contractRoot, "../🧫️fixtures/🏭️owned-generator-preview-inventory"), fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const discovery = await import("../../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts");
    const { default: parseArgv } = await import("yargs-parser");
    for (const row of fixture.ownerExecutionRoutes) {
      const contract = { ownership: "owned" as const, ownerPath: row.ownerPath ?? "compiler", target: "@neutral/compiler:generate", previewTarget: "@neutral/compiler:preview-generated", ...(row.previewArguments ? { previewArguments: row.previewArguments as string[] } : {}), ...(row.previewScript ? { previewScript: row.previewScript as string } : {}) };
      const target = { executor: "repo:script", options: { cwd: row.cwd, command: row.command } };
      if (!row.valid) expect(() => discovery.generatorPreviewExecution(contract, target)).toThrow("exact owner JSON preview");
      else {
        const route = discovery.generatorPreviewExecution(contract, target);
        expect([route.command, ...route.args].join(" "), row.id).toBe(row.command);
        expect(route.cwd, row.id).toBe(row.cwd);
        expect(parseArgv([route.command, ...route.args], { configuration: { "populate--": true, "camel-case-expansion": false, "parse-numbers": false } }), row.id).toEqual(parseArgv(row.command, { configuration: { "populate--": true, "camel-case-expansion": false, "parse-numbers": false } }));
      }
    }
  });
  test("admits only a closed compiler input manifest authority", () => {
    const fixture = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🏭️owned-generator-preview-inventory/🔣️.json"), "utf8"));
    
    expect(fixture["schemaVersion"]).toEqual(1);expect(fixture["contract"]).toEqual("owned-generator-preview-inventory-v1");expect(fixture["compilerManifest"]["version"]).toEqual(1);
    const taxonomy = loadTaxonomy(), actor = taxonomy.generatorContracts["actor-typegen"]!;
    for (const row of fixture.compilerInputAuthorities) {
      const changed = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "actor-typegen": { ...actor, compilerInputManifest: row.authority } } };
      expect(validateTaxonomy(changed).length === 0, JSON.stringify(row)).toBe(row.valid);
    }
  });
  test("discovers only exact byte-matched compiler manifest inputs", () => {
    const root = join(contractRoot, "../🧫️fixtures/🏭️owned-generator-preview-inventory"), fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const sandbox = mkdtempSync(join(tmpdir(), "compiler-input-manifest-")), manifestPath = "📤️output/🧾️manifest.json";
    mkdirSync(join(sandbox, "🌱️source"), { recursive: true });
    mkdirSync(join(sandbox, "📤️output"), { recursive: true });
    writeFileSync(join(sandbox, "🌱️source/🟦️.ts"), "neutral input\n");
    const contract = { ownership: "owned" as const, ownerPath: "owner", target: "owner:generate", previewTarget: "owner:preview-generated", inputPatterns: ["authority.json"], outputRoots: [{ path: "📤️output", inclusion: "ignored" as const }], compilerInputManifest: { kind: "compiler-input-manifest-v1" as const, manifestOutputPath: manifestPath, manifestSchemaPath: "authority.json", staticAuthorityPath: "authority.json", maxFiles: 2 }, reason: "neutral" };
    const taxonomy = loadSourceTaxonomy({ repoRoot: findWorkspaceRoot(contractRoot) });
    writeFileSync(join(sandbox, "authority.json"), "{}\n");
    writeFileSync(join(sandbox, manifestPath), JSON.stringify(fixture.compilerManifest));
    expect(generatorInputPaths({ repoRoot: sandbox }, contract, taxonomy)).toEqual(["authority.json", "🌱️source/🟦️.ts"]);
    for (const manifest of [
      { ...fixture.compilerManifest, inputs: [{ ...fixture.compilerManifest.inputs[0], bytes: 13 }] },
      { ...fixture.compilerManifest, inputs: [{ ...fixture.compilerManifest.inputs[0], path: "../../📦️packages/escape" }] },
      { ...fixture.compilerManifest, inputs: [...fixture.compilerManifest.inputs, fixture.compilerManifest.inputs[0]] },
      { ...fixture.compilerManifest, extra: true },
    ]) {
      writeFileSync(join(sandbox, manifestPath), JSON.stringify(manifest));
      expect(() => generatorInputPaths({ repoRoot: sandbox }, contract, taxonomy)).toThrow();
    }
  });
}
