import { describe, expect, test } from "bun:test";
import { createHash, randomUUID } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import glob from "fast-glob";
import MarkdownIt from "markdown-it";
import ts from "typescript";
import { parse as parseJsonc } from "jsonc-parser";
import { ownedFilesystemEntries } from "../🔍️filesystem/🟦️.ts";
import { applyTaxonomyPlan, canonicalJson, inventoryTaxonomy, planTaxonomy, typescriptLeadingDocumentationReferenceAuthority } from "../../🧹️normalization/🟦️.ts";
import { validateTaxonomy, type Taxonomy } from "../../🔍️discovery/🟦️.ts";

type Mapping = Readonly<{ id: string; source: string; destination: string; kindId: string; size: number; sha256: string }>;
type Negative = Readonly<{ id: string; omitDescriptor?: boolean; descriptorPatch?: Record<string, unknown>; extraDescriptor?: boolean; payload?: string; payloadPath?: string }>;
type FixtureKind = "support" | "oracle" | "unowned";
type FixtureRetention = Readonly<{ ownerPath: string; prefixes: Readonly<Record<FixtureKind, string>>; rejectedChildren: readonly string[] }>;
type Vector = Readonly<{ schemaVersion: number; owner: string; subset: string; ownerReadiness: { nodes: number; physicalNodes: number; files: number; sourceBytes: number; sourceTreeDigest: string; alreadyCanonicalSources: readonly string[]; contextSources: readonly { path: string; size: number; sha256: string }[] }; execution: { target: string; command: string; launchName: string; launchCommand: string; launchGroup: string; launchOrder: number; generatorEnvironment: { name: string; value: string }; oracleRetention: { rootPrefix: string; retainedInputs: readonly string[]; disposableOutputs: readonly string[]; retainOnFailure: boolean }; fixtureRetention: FixtureRetention }; sourceInputs: Readonly<Record<string, string>>; cases: readonly Mapping[]; payloadAuthority: unknown; payloadDescriptor: string; documentationCases: readonly { id: string; content: string; values: readonly string[] }[]; negativeCases: readonly Negative[] }>;
const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const ticket = join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION");
const schemaPath = `${library}/🔣️taxonomy.json`;
const schemaBytes = readFileSync(join(repoRoot, schemaPath));
const taxonomy = JSON.parse(schemaBytes.toString()) as Taxonomy;
const vector = JSON.parse(readFileSync(join(repoRoot, library, "📦️packages/🟦️typescript/🧫️fixtures/🍃️artifact-support-leaf-authority/🔣️.json"), "utf8")) as Vector;
const fixtureRetention = vector.execution.fixtureRetention;
const subset = `${vector.owner}/${vector.subset}`;
const sourceBytes = new Map(vector.cases.map((row) => [row.id, Buffer.from(vector.sourceInputs[row.id])]));
const descriptorBytes = Buffer.from(vector.sourceInputs["mutation-descriptor"]);
process.env.NX_DAEMON = "false";
const generatorEnvironment = vector.execution.generatorEnvironment;
if (generatorEnvironment.name !== "SEMIO_TAXONOMY_GENERATOR_SENTINEL") throw new Error("Unexpected fixture environment variable");
process.env.SEMIO_TAXONOMY_GENERATOR_SENTINEL = generatorEnvironment.value;

/** 📁️ Resolves this suite's exact semantic owner without traversing linked ancestors. */
function retainedFixtureParent(create: boolean): string {
  if (fixtureRetention.ownerPath !== "📓️energy-support-acceptance/🧾️runs") throw new Error("Unowned Energy fixture parent");
  const parent = join(ticket, fixtureRetention.ownerPath);
  let current = repoRoot;
  for (const part of relative(repoRoot, parent).split(/[\\/]/u)) {
    current = join(current, part);
    let state: ReturnType<typeof lstatSync>;
    try { state = lstatSync(current); }
    catch (error) { if (!create || (error as NodeJS.ErrnoException).code !== "ENOENT") throw error; mkdirSync(current); state = lstatSync(current); }
    if (!state.isDirectory() || state.isSymbolicLink()) throw new Error("Unsafe Energy fixture ancestor");
  }
  return parent;
}

/** 🔒️ Admits only exact run roots of the requested fixture role. */
function validateRetainedFixtureRoot(root: string, kind: FixtureKind): void {
  const prefix = fixtureRetention.prefixes[kind], name = basename(root);
  if (dirname(root) !== join(ticket, fixtureRetention.ownerPath) || !name.startsWith(prefix) || !/^[A-Za-z0-9]{6}$/u.test(name.slice(prefix.length))) throw new Error("Unowned oracle output root");
  retainedFixtureParent(false);
  const state = lstatSync(root);
  if (!state.isDirectory() || state.isSymbolicLink()) throw new Error("Unowned oracle output root");
}

/** 🧾️ Allocates a unique retained run and records its input/recovery ownership. */
function retainedFixtureRoot(kind: FixtureKind): string {
  const prefix = fixtureRetention.prefixes[kind];
  if (!prefix || prefix.includes("/") || prefix.includes("\\")) throw new Error("Invalid Energy fixture prefix");
  const root = mkdtempSync(join(retainedFixtureParent(true), prefix));
  validateRetainedFixtureRoot(root, kind);
  writeFileSync(join(root, "📝️.md"), `# Energy ${kind} Fixture Run\n\nCreated ${new Date().toISOString()}. Authored inputs and active recovery evidence are retained. This is a fresh test run, not recovered historical evidence.\n`, { flag: "wx" });
  return root;
}

/** 🧫️ Builds an isolated, untracked-input inventory without modifying the shared repository's Git state. */
function fixture(negative?: Negative): string {
  const root = retainedFixtureRoot("support");
  const put = (path: string, bytes: string | Buffer): void => {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), bytes);
  };
  put(schemaPath, schemaBytes);
  for (const row of vector.cases) put(`${subset}/${row.id === "mutation-payload-schema" ? negative?.payloadPath ?? row.source : row.source}`, row.id === "mutation-payload-schema" && negative?.payload !== undefined ? negative.payload : sourceBytes.get(row.id)!);
  if (!negative?.omitDescriptor) {
    const descriptor = negative?.descriptorPatch ? `${JSON.stringify({ ...JSON.parse(descriptorBytes.toString()), ...negative.descriptorPatch }, null, 2)}\n` : descriptorBytes;
    put(`${subset}/${vector.payloadDescriptor}`, descriptor);
    if (negative?.extraDescriptor) put(`${subset}/${dirname(vector.payloadDescriptor)}/🪪️descriptor.json`, descriptor);
  }
  const git = Bun.spawnSync(["git", "init", "--quiet", "--object-format=sha1"], { cwd: root, stdout: "pipe", stderr: "pipe" });
  if (git.exitCode !== 0) throw new Error(git.stderr.toString());
  return root;
}

/** 🧵️ Exercises actual reference edits, runtime reads and Nx generation in a ticket-owned fixture. */
function lifecycleFixture(): { root: string; baselineCommit: string; ticketDir: string; consumer: string } {
  const root = fixture();
  const put = (path: string, bytes: string): void => {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), bytes);
  };
  const fixtureSchema = JSON.parse(schemaBytes.toString());
  delete fixtureSchema.generatorContracts["plugin-registry"].inputDiscovery;
  const generator = "🧪️tests/🧪️generator", outputRoot = `${generator}/🤖️generated`;
  fixtureSchema.generatorContracts["fixture-generator"] = { ownership: "owned", ownerPath: generator, target: "@fixture/support:generate", previewTarget: "@fixture/support:preview-generated", checkTarget: "@fixture/support:check-generated", inputPatterns: [`${vector.owner}/**`], outputRoots: [{ path: outputRoot, inclusion: "ignored" }], reason: "Isolated support reference lifecycle oracle" };
  fixtureSchema.generatorContracts = Object.fromEntries(Object.entries(fixtureSchema.generatorContracts).sort(([a], [b]) => a.localeCompare(b)));
  put(schemaPath, `${JSON.stringify(fixtureSchema, null, 2)}\n`);
  put(".git/info/exclude", `${schemaPath}\n.nx/\n${outputRoot}/\n`);
  put("nx.json", '{"defaultBase":"main"}\n');
  put("package.json", '{"name":"support-lifecycle-fixture","private":true}\n');
  const project = { name: "@fixture/support", root: generator, targets: Object.fromEntries(["generate", "preview-generated", "check-generated"].map((command) => [command, { executor: "nx:run-commands", options: { cwd: generator, command: `bun ./📜️script.ts ${command}` } }])) };
  put("project.json", `${JSON.stringify(project, null, 2)}\n`);
  put(`${generator}/📋️project.json`, `${JSON.stringify(project, null, 2)}\n`);
  put(`${generator}/📜️script.ts`, [
    'import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";',
    'import { join } from "node:path";',
    `const outputRoot=${JSON.stringify(outputRoot)}, outputFile="🔣️.json", bytes=Buffer.from('{"supportFixture":true}\\n');`,
    'const root=join(process.cwd(),"🤖️generated"), file=join(root,outputFile);',
    'const nodes=[{bytesBase64:"",mode:0o755,nodeKind:"directory",path:outputRoot},{bytesBase64:bytes.toString("base64"),mode:0o644,nodeKind:"file",path:`${outputRoot}/${outputFile}`}];',
    'const command=process.argv[2];',
    `if(command!=="preview-generated"&&process.env[${JSON.stringify(generatorEnvironment.name)}]!==${JSON.stringify(generatorEnvironment.value)})throw new Error("missing current generator environment");`,
    'if(command==="preview-generated") process.stdout.write(`${JSON.stringify({contractId:"fixture-generator",nodes,schemaVersion:1,staleRemovals:[]})}\\n`);',
    'else if(command==="generate"){mkdirSync(root,{recursive:true,mode:0o755});writeFileSync(file,bytes,{mode:0o644});console.log("[DEBUG] support fixture generated");}',
    'else if(command==="check-generated"){if(!existsSync(file)||!readFileSync(file).equals(bytes))throw new Error("stale generated fixture");console.log("[DEBUG] support fixture checked");}',
    'else throw new Error(`unknown command ${command}`);',
    "",
  ].join("\n"));
  const consumer = "🟦️consumer.ts";
  const paths = vector.cases.map((row) => `../${subset}/${row.source}`);
  put(consumer, [
    `/** 🧬️ Fixture documentation reads \`${paths[0]}\`. */`,
    ...paths.map((path, index) => `const source${index} = await Bun.file(new URL(${JSON.stringify(path)}, import.meta.url)).text();`),
    'console.log(`[DEBUG] ${JSON.stringify({events:JSON.parse(source0).events.length,dsl:source1.split("\\n")[0],payload:JSON.parse(source2).title})}`);',
    "",
  ].join("\n"));
  const commit = Bun.spawnSync(["git", "-c", "user.name=Semio Fixture", "-c", "user.email=fixture@invalid.example", "-c", "commit.gpgsign=false", "commit", "--quiet", "--allow-empty", "-m", "support fixture"], { cwd: root, stdout: "pipe", stderr: "pipe" });
  if (commit.exitCode !== 0) throw new Error(commit.stderr.toString());
  const head = Bun.spawnSync(["git", "rev-parse", "HEAD"], { cwd: root, stdout: "pipe", stderr: "pipe" });
  if (head.exitCode !== 0) throw new Error(head.stderr.toString());
  return { root, baselineCommit: head.stdout.toString().trim(), ticketDir: join(root, "🧪️tests"), consumer };
}

/** 🧺️ Retires only completed Cargo outputs after validating every exact no-follow target. */
function retireRustOracleOutputs(root: string, completed = true): void {
  validateRetainedFixtureRoot(root, "oracle");
  if (!completed) return;
  const outputs = ([["Cargo.lock", false], ["🎯️target", true]] as const).flatMap(([name, directory]) => {
    const path = join(root, name);
    let state: ReturnType<typeof lstatSync>;
    try { state = lstatSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return []; throw error; }
    if (state.isSymbolicLink() || (directory ? !state.isDirectory() : !state.isFile())) throw new Error("Unexpected oracle output kind");
    return [{ path, directory }];
  });
  for (const output of outputs) rmSync(output.path, { recursive: output.directory });
}

/** 🦀️ Executes the actual Rust pointer assertion against the committed descriptor with serde_json. */
async function verifyRustPointerAssertion(repo: string, aggregate: string, descriptor: string): Promise<void> {
  const assertions = readFileSync(join(repo, aggregate), "utf8").split(/\r?\n/u).map((line) => line.trim()).filter((line) => line.startsWith("assert_eq!(") && line.includes('descriptor["payloadSchema"]'));
  expect(assertions).toHaveLength(1);
  const root = retainedFixtureRoot("oracle");
  writeFileSync(join(root, "Cargo.toml"), '[package]\nname = "energy-pointer-oracle"\nversion = "0.0.0"\nedition = "2021"\n\n[[bin]]\nname = "energy-pointer-oracle"\npath = "🦀️.rs"\n\n[dependencies]\nserde_json = "=1.0.149"\n\n[workspace]\n');
  writeFileSync(join(root, "🦀️.rs"), [
    "fn main() {",
    "    let args: Vec<String> = std::env::args().collect();",
    "    let owner = std::path::Path::new(&args[1]);",
    "    let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&args[2]).unwrap()).unwrap();",
    `    ${assertions[0]}`,
    '    println!("[DEBUG] descriptor pointer checked: {}", owner.display());',
    "}",
    "",
  ].join("\n"));
  let completed = false;
  try {
    const child = Bun.spawn(["cargo", "run", "--offline", "--quiet", "--manifest-path", join(root, "Cargo.toml"), "--target-dir", join(root, "🎯️target"), "--", dirname(join(repo, descriptor)), join(repo, descriptor)], { cwd: root, env: { ...process.env, RUSTC_WRAPPER: "" }, stdout: "pipe", stderr: "pipe" });
    const [code, stdout, stderr] = await Promise.all([child.exited, new Response(child.stdout).text(), new Response(child.stderr).text()]);
    expect(code, stderr).toBe(0);
    expect(stdout).toBe(`[DEBUG] descriptor pointer checked: ${dirname(join(repo, descriptor))}\n`);
    completed = true;
  } finally { retireRustOracleOutputs(root, completed); }
}

describe("artifact support leaf authority", () => {
  test("owns Energy fixture runs through a language-neutral no-follow retention contract", () => {
    const contract = fixtureRetention;
    const oracle = new Ajv({ strict: true }).compile({ type: "object", additionalProperties: false, required: ["ownerPath", "prefixes", "rejectedChildren"], properties: { ownerPath: { const: "📓️energy-support-acceptance/🧾️runs" }, prefixes: { type: "object", additionalProperties: false, required: ["support", "oracle", "unowned"], properties: { support: { const: "🧪️energy-support-current-" }, oracle: { const: "🧪️energy-pointer-oracle-" }, unowned: { const: "🧪️energy-oracle-unowned-" } } }, rejectedChildren: { type: "array", minItems: 4, uniqueItems: true, items: { type: "string" } } } });
    expect(oracle(contract), JSON.stringify(oracle.errors)).toBe(true);
    expect(typeof retainedFixtureRoot).toBe("function");
    const root = retainedFixtureRoot("support");
    expect(dirname(root)).toBe(join(ticket, contract.ownerPath));
    expect(basename(root).startsWith(contract.prefixes.support)).toBe(true);
    expect(glob.sync("*", { cwd: dirname(root), onlyDirectories: true, followSymbolicLinks: false })).toContain(basename(root));
    expect(() => validateRetainedFixtureRoot(root, "support")).not.toThrow();
    for (const child of contract.rejectedChildren) expect(() => validateRetainedFixtureRoot(join(dirname(root), child), "oracle")).toThrow();
    const other = retainedFixtureRoot("unowned"), linked = join(dirname(root), contract.prefixes.support + randomUUID().slice(0, 6));
    symlinkSync(other, linked, "junction");
    expect(() => validateRetainedFixtureRoot(linked, "support")).toThrow();
    expect(lstatSync(linked).isSymbolicLink()).toBe(true);
    expect(readFileSync(join(root, "📝️.md"), "utf8")).toContain("Authored inputs and active recovery evidence are retained");
  });

  test("retains authored oracle inputs and retires only exact completed compiler outputs", () => {
    const contract = vector.execution.oracleRetention;
    expect(contract).toEqual({ rootPrefix: "🧪️energy-pointer-oracle-", retainedInputs: ["Cargo.toml", "🦀️.rs"], disposableOutputs: ["Cargo.lock", "🎯️target"], retainOnFailure: true });
    const root = retainedFixtureRoot("oracle");
    for (const name of contract.retainedInputs) writeFileSync(join(root, name), `authored ${name}\n`);
    writeFileSync(join(root, "Cargo.lock"), "generated lock\n");
    mkdirSync(join(root, "🎯️target"));
    writeFileSync(join(root, "🎯️target/🧪️.bin"), "generated binary\n");
    retireRustOracleOutputs(root);
    expect(glob.sync("**/*", { cwd: root, onlyFiles: true, followSymbolicLinks: false }).sort()).toEqual([...contract.retainedInputs, "📝️.md"].sort());
    for (const name of contract.retainedInputs) expect(readFileSync(join(root, name), "utf8")).toBe(`authored ${name}\n`);
    for (const name of contract.disposableOutputs) expect(existsSync(join(root, name))).toBe(false);
    const outside = retainedFixtureRoot("unowned");
    writeFileSync(join(outside, "Cargo.lock"), "unowned sentinel\n");
    expect(() => retireRustOracleOutputs(outside)).toThrow("Unowned oracle output root");
    expect(readFileSync(join(outside, "Cargo.lock"), "utf8")).toBe("unowned sentinel\n");
    const linked = retainedFixtureRoot("oracle");
    writeFileSync(join(linked, "Cargo.lock"), "retain on failed preflight\n");
    symlinkSync(outside, join(linked, "🎯️target"), "junction");
    expect(() => retireRustOracleOutputs(linked)).toThrow("Unexpected oracle output kind");
    expect(readFileSync(join(linked, "Cargo.lock"), "utf8")).toBe("retain on failed preflight\n");
    expect(lstatSync(join(linked, "🎯️target")).isSymbolicLink()).toBe(true);
    const failed = retainedFixtureRoot("oracle");
    for (const name of [...contract.retainedInputs, "Cargo.lock"]) writeFileSync(join(failed, name), `failed run ${name}\n`);
    mkdirSync(join(failed, "🎯️target"));
    writeFileSync(join(failed, "🎯️target/🧪️.bin"), "failed compiler evidence\n");
    retireRustOracleOutputs(failed, false);
    expect(glob.sync("**/*", { cwd: failed, onlyFiles: true, followSymbolicLinks: false }).sort()).toEqual([...contract.retainedInputs, "Cargo.lock", "🎯️target/🧪️.bin", "📝️.md"].sort());
    expect(readFileSync(join(failed, "Cargo.lock"), "utf8")).toBe("failed run Cargo.lock\n");
  });

  test("registers the canonical test through Nx and the launch catalog", () => {
    const expected = vector.execution;
    const project = JSON.parse(readFileSync(join(repoRoot, library, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
    expect(project.targets[expected.target]?.options.command).toBe(expected.command);
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const raw = readFileSync(join(repoRoot, path), "utf8");
      const launches = parseJsonc(raw).configurations.filter((entry: { name: string }) => entry.name === expected.launchName);
      expect(launches).toHaveLength(1);
      expect(launches[0].command).toBe(expected.launchCommand);
      expect(launches[0].presentation).toEqual({ group: expected.launchGroup, order: expected.launchOrder });
    }
  });

  test("matches the language-neutral owner contracts and exact production inputs", () => {
    expect(validateTaxonomy(taxonomy)).toEqual([]);
    expect(taxonomy.mutationPayloadSchemaAuthority).toEqual(vector.payloadAuthority);
    const validate = new Ajv({ strict: true }).compile({ type: "array", minItems: 3, maxItems: 3, items: { type: "object", additionalProperties: false, required: ["id", "source", "destination", "kindId", "size", "sha256"], properties: { id: { type: "string" }, source: { type: "string" }, destination: { type: "string" }, kindId: { type: "string" }, size: { type: "integer", minimum: 1 }, sha256: { type: "string", pattern: "^[a-f0-9]{64}$" } } } });
    expect(validate(vector.cases), JSON.stringify(validate.errors)).toBe(true);
    for (const row of vector.cases) {
      const bytes = sourceBytes.get(row.id)!;
      expect(bytes.length).toBe(row.size);
      expect(createHash("sha256").update(bytes).digest("hex")).toBe(row.sha256);
    }
    const payloadSchema = JSON.parse(sourceBytes.get("mutation-payload-schema")!.toString());
    const payload = new Ajv({ strict: true }).compile(payloadSchema);
    expect(payload({ newModelJson: "{}" })).toBe(true);
    expect(payload({ newModelJson: 1 })).toBe(false);
  });

  test("projects only the three physically evidenced support leaves", () => {
    const root = fixture();
    const inventory = inventoryTaxonomy({ repoRoot: root, scope: vector.owner, workers: 1 });
    expect(inventory.violations).toEqual([]);
    const files = inventory.entries.filter((entry) => entry.nodeKind === "file");
    const oracle = glob.sync("**/*", { cwd: join(root, vector.owner), onlyFiles: true, dot: true, followSymbolicLinks: false }).map((path) => `${vector.owner}/${path}`).sort();
    expect(files.map((entry) => entry.sourcePath).sort()).toEqual(oracle);
    for (const row of vector.cases) {
      const actual = files.find((entry) => entry.sourcePath === `${subset}/${row.source}`)!;
      expect(actual.normalizedPath).toBe(`${subset}/${row.destination}`);
      expect(actual.fileKind).toBe(row.kindId);
      expect(actual.contentHash).toBe(row.sha256);
    }
  });

  for (const negative of vector.negativeCases) test(`rejects unproven payload ownership: ${negative.id}`, () => {
    const root = fixture(negative);
    const inventory = inventoryTaxonomy({ repoRoot: root, scope: vector.owner, workers: 1 });
    const row = vector.cases.find((entry) => entry.id === "mutation-payload-schema")!;
    const path = `${subset}/${negative.payloadPath ?? row.source}`;
    const actual = inventory.entries.find((entry) => entry.sourcePath === path)!;
    expect(actual).toBeDefined();
    const owner = `${subset}/${dirname(vector.payloadDescriptor)}`;
    expect(inventory.violations.some((problem) => problem.code === "mutation-payload-schema-authority-invalid" && (problem.path === owner || problem.path === `${subset}/${vector.payloadDescriptor}`))).toBe(true);
    expect(readFileSync(join(root, path)).toString()).toBe(negative.payload ?? sourceBytes.get(row.id)!.toString());
  });

  test("matches JSDoc path vectors with independent TypeScript and CommonMark parsers", () => {
    const markdown = new MarkdownIt();
    for (const row of vector.documentationCases) {
      const actual = typescriptLeadingDocumentationReferenceAuthority(row.content);
      expect(actual.map((entry) => entry.value), row.id).toEqual(row.values);
      for (const token of actual) {
        expect(row.content.slice(token.start, token.end)).toBe(token.value);
        expect(token.structuredLocation.startsWith("typescript-leading-jsdoc-path:")).toBe(true);
      }
      if (row.values.length === 0) continue;
      const ranges = ts.getLeadingCommentRanges(row.content, 0) ?? [];
      expect(ranges).toHaveLength(1);
      expect(ranges[0].kind).toBe(ts.SyntaxKind.MultiLineCommentTrivia);
      const prose = row.content.slice(ranges[0].pos + 3, ranges[0].end - 2).replace(/^\s*\* ?/gmu, "");
      const oracle = markdown.parseInline(prose, {}).flatMap((block) => block.children ?? []).filter((token) => token.type === "code_inline").map((token) => token.content);
      expect(oracle).toEqual(actual.map((entry) => entry.value));
    }
  });

  test("preserves JSDoc and runtime readers through rollback, retry, Nx generation and an empty plan", () => {
    const row = lifecycleFixture();
    const options = { repoRoot: row.root, scope: vector.owner, ticketDir: row.ticketDir, workers: 1 };
    const inventory = inventoryTaxonomy(options);
    const plan = planTaxonomy(inventory, { baselineCommit: row.baselineCommit, excludedTreeDigests: [] });
    expect(plan.unresolved).toEqual([]);
    expect(plan.moves).toHaveLength(3);
    expect(plan.edits).toHaveLength(3);
    expect(plan.edits.filter((entry) => entry.structuredLocation.startsWith("typescript-leading-jsdoc-path:"))).toHaveLength(1);
    expect(plan.regenerations.map((entry) => entry.contractId)).toEqual(["fixture-generator"]);
    const planPath = join(row.ticketDir, "🧾️plan/🔣️.json");
    mkdirSync(dirname(planPath), { recursive: true });
    writeFileSync(planPath, `${canonicalJson(plan)}\n`);
    const runReader = (): void => {
      const result = Bun.spawnSync([process.execPath, join(row.root, row.consumer)], { cwd: row.root, stdout: "pipe", stderr: "pipe" });
      expect(result.exitCode, result.stderr.toString()).toBe(0);
      expect(result.stdout.toString().trim()).toBe('[DEBUG] {"events":6,"dsl":"semio energy.model.dsl v1","payload":"ReplaceModel"}');
    };
    runReader();
    const apply = { ...options, expectedBaselineCommit: row.baselineCommit, expectedPlanDigest: plan.planDigest, planArtifactPath: planPath };
    const rollback = applyTaxonomyPlan(plan, { ...apply, injectFailureAt: "after-edits" });
    expect(rollback.state).toBe("rolled-back");
    expect(JSON.parse(readFileSync(rollback.journalPath, "utf8")).error).toContain("after-edits");
    runReader();
    for (const move of plan.moves) {
      expect(createHash("sha256").update(readFileSync(join(row.root, move.sourcePath))).digest("hex")).toBe(move.sourcePreimage.contentHash);
      expect(existsSync(join(row.root, move.destinationPath))).toBe(false);
    }
    expect(applyTaxonomyPlan(plan, apply).state).toBe("committed");
    runReader();
    for (const mapping of vector.cases) {
      expect(existsSync(join(row.root, subset, mapping.source))).toBe(false);
      const path = join(row.root, subset, mapping.destination);
      expect(createHash("sha256").update(readFileSync(path)).digest("hex")).toBe(mapping.sha256);
      expect(lstatSync(path).mode & 0o777).toBe(0o644);
    }
    const empty = planTaxonomy(inventoryTaxonomy(options), { baselineCommit: row.baselineCommit, excludedTreeDigests: [] });
    expect([empty.moves.length, empty.edits.length, empty.regenerations.length, empty.evidenceRemovals.length, empty.unresolved.length]).toEqual([0, 0, 0, 0, 0]);
  }, 120_000);

  test("scoped mutation references preserve foreign audit values and rewrite only exact moving targets", () => {
    const row = lifecycleFixture();
    const source = `${vector.subset}/🧬️schema/🧬️mutations/♻️replace-model/🧪️tests/replaces-a-model/🎯️outcome/🔣️.json`;
    const destination = "🧪️tests/🪆️1-any/♻️replace-model/🧪️replaces-a-model/🎯️outcome/🔣️.json";
    const foreignOwner = "✏️s/🔌️plugins/🧊️cube/🗿️artifacts/🧊️cube";
    const auditPath = "🔣️audit.json", foreignPath = `${foreignOwner}/🧪️tests/🔣️reference.json`;
    const put = (path: string, value: unknown): void => {
      mkdirSync(dirname(join(row.root, path)), { recursive: true });
      writeFileSync(join(row.root, path), JSON.stringify(value) + "\n");
    };
    const bundle = JSON.parse(readFileSync(join(repoRoot, library, "📦️packages/🟦️typescript/🧫️fixtures/🔣️mutation-path-projection.json"), "utf8")).bundle as readonly { source: string }[];
    const scenario = source.slice(0, -"/🎯️outcome/🔣️.json".length);
    for (const owner of [vector.owner, foreignOwner]) for (const leaf of bundle) {
      const path = `${owner}/${scenario}/${leaf.source}`;
      put(path, {});
      if (leaf.source.endsWith(".rs")) writeFileSync(join(row.root, path), "pub fn fixture() {}\n");
    }
    put(`${subset}/🔣️oracle.json`, { schemaVersion: 1, oracles: [], noOracleDecisions: [], mutationCatalogs: [{ id: "energy-model-1-any", capability: "energy-model-mutate", standardDirectoryName: "🔖️1", subsetDirectoryName: "✳️any", kinds: [], vectors: [{ mutationId: "replace-model", sourceMutationDirectoryName: "♻️replace-model", mutationDirectoryName: "♻️replace-model", scenarios: [{ id: "replaces-a-model", directoryName: "🧪️replaces-a-model" }] }] }] });
    put(auditPath, [{ owner: foreignOwner, uri: `asset://${source}`, path: `${foreignOwner}/${source}` }, { path: `${vector.owner}/${source}` }]);
    put(foreignPath, { uri: `asset://${source}` });
    const before = [auditPath, foreignPath].map((path) => readFileSync(join(row.root, path)));
    const plan = planTaxonomy(inventoryTaxonomy({ repoRoot: row.root, scope: vector.owner, ticketDir: row.ticketDir, workers: 1 }), { baselineCommit: row.baselineCommit, excludedTreeDigests: [] });
    put("🧪️tests/🧾️scope-plan/🔣️.json", plan);
    expect(plan.unresolved).toEqual([]);
    expect(plan.moves.find((move) => move.sourcePath === `${vector.owner}/${source}`)?.destinationPath).toBe(`${vector.owner}/${destination}`);
    expect(plan.edits.filter((edit) => edit.path === foreignPath)).toEqual([]);
    const edits = plan.edits.filter((edit) => edit.path === auditPath);
    expect(edits).toHaveLength(1);
    expect(edits[0].oldValue).toBe(`${vector.owner}/${source}`);
    expect(edits[0].newValue).toBe(`${vector.owner}/${destination}`);
    expect(parseJsonc(before[0].toString())).toEqual(JSON.parse(before[0].toString()));
    expect([auditPath, foreignPath].map((path) => readFileSync(join(row.root, path)))).toEqual(before);
    console.log("[DEBUG] scoped mutation reference plan", JSON.stringify({ root: row.root, moves: plan.moves.length, exactAuditEdits: edits.length, foreignEdits: 0, unresolved: plan.unresolved.length }));
  }, 120_000);

  test("plans the complete real Energy owner with exact Cargo mounting context", async () => {
    const row = lifecycleFixture();
    const expected = vector.ownerReadiness;
    const entries = ownedFilesystemEntries(join(repoRoot, vector.owner), true);
    expect(entries).toHaveLength(expected.physicalNodes);
    expect(entries.some((entry) => entry.nodeKind === "symlink")).toBe(false);
    const files = entries.filter((entry) => entry.nodeKind === "file").map((entry) => `${vector.owner}/${entry.path}`);
    expect(files).toHaveLength(expected.files);
    expect(glob.sync("**/*", { cwd: join(repoRoot, vector.owner), dot: true, onlyFiles: true, followSymbolicLinks: false }).map((path) => `${vector.owner}/${path}`).sort()).toEqual([...files].sort());
    let bytes = 0;
    for (const path of files) {
      const content = readFileSync(join(repoRoot, path));
      bytes += content.length;
      mkdirSync(dirname(join(row.root, path)), { recursive: true });
      writeFileSync(join(row.root, path), content);
    }
    expect(bytes).toBe(expected.sourceBytes);
    for (const context of expected.contextSources) {
      const content = readFileSync(join(repoRoot, context.path));
      expect(content.length).toBe(context.size);
      expect(createHash("sha256").update(content).digest("hex")).toBe(context.sha256);
      mkdirSync(dirname(join(row.root, context.path)), { recursive: true });
      writeFileSync(join(row.root, context.path), content);
    }
    const inventory = inventoryTaxonomy({ repoRoot: row.root, scope: vector.owner, ticketDir: row.ticketDir, workers: 1 });
    expect(inventory.entries).toHaveLength(expected.nodes);
    expect(inventory.sourceTreeDigest).toBe(expected.sourceTreeDigest);
    expect(inventory.violations).toEqual([]);
    expect(inventory.entries.filter((entry) => entry.nodeKind === "file" && entry.sourcePath === entry.normalizedPath).map((entry) => entry.sourcePath.slice(vector.owner.length + 1)).sort()).toEqual([...expected.alreadyCanonicalSources].sort());
    const plan = planTaxonomy(inventory, { baselineCommit: row.baselineCommit, excludedTreeDigests: [] });
    const planPath = join(row.ticketDir, "🧾️energy-owner-plan/🔣️.json");
    mkdirSync(dirname(planPath), { recursive: true });
    writeFileSync(planPath, `${canonicalJson(plan)}\n`);
    expect(plan.unresolved).toEqual([]);
    expect(plan.moves).toHaveLength(expected.files - expected.alreadyCanonicalSources.length);
    expect(plan.edits.some((entry) => entry.path === expected.contextSources[1].path)).toBe(true);
    expect(plan.regenerations.map((entry) => entry.contractId)).toEqual(["fixture-generator"]);
    const options = { repoRoot: row.root, scope: vector.owner, ticketDir: row.ticketDir, workers: 1, expectedBaselineCommit: row.baselineCommit, expectedPlanDigest: plan.planDigest, planArtifactPath: planPath };
    const rollback = applyTaxonomyPlan(plan, { ...options, injectFailureAt: "after-edits" });
    expect(rollback.state).toBe("rolled-back");
    expect(JSON.parse(readFileSync(rollback.journalPath, "utf8")).error).toContain("after-edits");
    for (const move of plan.moves) {
      expect(createHash("sha256").update(readFileSync(join(row.root, move.sourcePath))).digest("hex")).toBe(move.sourcePreimage.contentHash);
      expect(existsSync(join(row.root, move.destinationPath))).toBe(false);
    }
    const committed = applyTaxonomyPlan(plan, options);
    expect(committed.state, JSON.parse(readFileSync(committed.journalPath, "utf8")).error).toBe("committed");
    const empty = planTaxonomy(inventoryTaxonomy(options), { baselineCommit: row.baselineCommit, excludedTreeDigests: [] });
    expect([empty.moves.length, empty.edits.length, empty.regenerations.length, empty.unresolved.length]).toEqual([0, 0, 0, 0]);
    await verifyRustPointerAssertion(row.root, `${subset}/🧬️schema/🧬️mutations/🦀️.rs`, `${subset}/🧬️schema/🧬️mutations/♻️replace-model/🔣️.json`);
  }, 120_000);
});
