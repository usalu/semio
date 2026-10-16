/** 🧾️ `schema` command family: generate, check, compile, docs, entries, oracle, test, verify and audit.
 * @see 📜️script.ts
 */
import { createIssuedResourceControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { dispatchOwnedScriptRoute, runCmd, runCmdStatus } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { inventorySchemaScopes, renderSchemaCatalog, renderSchemaCatalogDocument, renderSchemaCheckReport, schemaRustEntryDiagnostics, type SchemaRustEntryDump, type SchemaScopeDiagnostic, type SchemaScopeInventory, loadCatalogTaxonomy } from "../../🔍️discovery/🟦️.ts";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, isAbsolute, join } from "node:path";
import { pathToFileURL } from "node:url";

//#region 🔖️SchemaScript
/** 🧪️ The tracked vitest project of `framework.schema`, which declares the module's own third-party oracle specs. */
const SCHEMA_ORACLE_VITEST_CONFIG = "🧰️framework/🔨️modules/🧬️schema/🧪️tests/🎚️config/🟦️.ts";

/** 🦀️ The crate whose registered test binaries produce the Rust-side evidence the schema gate consumes. */
const SCHEMA_RUST_CRATE = "semio-framework-schema";

/** 🧩️ Compiles every schema module against the owned draft-07 validator; `--out` keeps the JSON report. */
const SCHEMA_MODULE_COMPILE_TEST = { test: "schema-module-compile", variable: "SEMIO_SCHEMA_MODULE_COMPILE_OUT" } as const;

/** 🧬️ Scope-owned schema contracts: catalog generation, invariant checking and the generated index. */
export class SchemaScript extends Script {
  async run(segments: string[]): Promise<void> {
    const sub = segments[0];
    const rest = segments.slice(1);
    if (sub === "generate") return this.generate(rest);
    if (sub === "check") return await this.check(rest);
    if (sub === "runtime-graph") {
      if (!await dispatchOwnedScriptRoute(this.invocation,this.root,["schema",...segments])) throw Error("No present owner contributes schema runtime-graph");
      return;
    }
    if (sub === "verify") return this.verify(rest);
    if (sub === "audit") return this.audit(rest);
    if (sub === "docs") return this.docs();
    if (sub === "oracle") return this.oracle();
    if (sub === "compile") return this.compile(rest);
    if (sub === "entries") return this.entries(rest);
    if (sub === "test") return this.test(rest);
    throw new Error(`unknown schema subcommand: ${JSON.stringify(sub)} (expected audit | check | compile | docs | entries | generate | oracle | test | verify).`);
  }

  private inventory(): SchemaScopeInventory {
    return inventorySchemaScopes(this.root, loadCatalogTaxonomy());
  }

  /**
   * 📖️ Reads the tracked catalog once and tolerates exactly one torn read: a catalog being rewritten by a
   * concurrent `schema generate` parses as garbage, and re-reading it after a short pause is the difference
   * between one honest finding and hundreds of spurious ones.
   */
  private readCatalog(): { readonly text: string; readonly parsed: Record<string, unknown> | null } {
    const abs = join(this.root, this.catalogPath());
    if (!existsSync(abs)) return { text: "", parsed: null };
    for (let attempt = 0; ; attempt += 1) {
      const text = readFileSync(abs, "utf8");
      try {
        const parsed = JSON.parse(text) as Record<string, unknown>;
        if (parsed && typeof parsed === "object" && !Array.isArray(parsed)) return { text, parsed };
      } catch {
        // a torn read is retried once below
      }
      if (attempt > 0) return { text, parsed: null };
      Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 2000);
    }
  }

  private catalogPath(): string {
    return loadCatalogTaxonomy().schemaExportResolution.catalogPath;
  }

  private documentPath(): string {
    return loadCatalogTaxonomy().schemaExportResolution.catalogDocumentPath;
  }

  /**
   * 🧭️ Where the test domain lives is taxonomy vocabulary, so the root router never spells it out; the
   * `Taxonomy` interface does not yet surface the key, which is why the read is narrowed here.
   * @see the `🧩️ open/closed` suite under the test domain's own `🧪️tests/🔍️discovery-contract`
   */
  private testDomainPath(): string {
    const declared = (loadCatalogTaxonomy() as unknown as { testDomainPath?: unknown }).testDomainPath;
    if (typeof declared !== "string" || declared.length === 0) throw new Error("[schema test] the taxonomy declares no testDomainPath.");
    return declared;
  }

  /** 📇️ Rewrites the derived scope catalog from the schema modules on disk. */
  private generate(args: string[]): void {
    const inventory = this.inventory();
    const rel = this.catalogPath();
    const rendered = renderSchemaCatalog(inventory.catalog);
    const abs = join(this.root, rel);
    if (args.includes("--check")) {
      const current = existsSync(abs) ? readFileSync(abs, "utf8") : "";
      if (current !== rendered) {
        console.error(`[schema generate] ${rel} is stale; run bun ./📜️script.ts schema generate.`);
        process.exit(1);
      }
      console.log(`[schema generate] ${rel} is current (${Object.keys(inventory.catalog.scopes).length} scopes).`);
      return;
    }
    writeFileSync(abs, rendered);
    console.log(`[schema generate] ${rel}: ${Object.keys(inventory.catalog.scopes).length} scopes, ${inventory.diagnostics.length} diagnostics.`);
  }

  /** 🚦️ Reports scope invariant violations; --fixture-boundary runs the independent example-authority gate. */
  private async check(args: string[]): Promise<void> {
    const inventory = this.inventory();
    const rel = this.catalogPath();
    const abs = join(this.root, rel);
    const fixtureBoundary = args.includes("--fixture-boundary");
    const findings: SchemaScopeDiagnostic[] = [...inventory.diagnostics, ...inventory.placement].filter(finding => !fixtureBoundary || finding.code === "schema-fixture-defines-schema");
    if (!fixtureBoundary) {
      const rendered = renderSchemaCatalog(inventory.catalog);
      const tracked = this.readCatalog();
      if (!existsSync(abs)) findings.push({ code: "schema-catalog-missing", path: rel, detail: "Run bun ./📜️script.ts schema generate." });
      else if (!tracked.parsed) findings.push({ code: "schema-catalog-malformed", path: rel, detail: "The catalog is not a JSON object after a re-read; a concurrent schema generate is still writing it." });
      else if (tracked.text !== rendered) findings.push({ code: "schema-catalog-stale", path: rel, detail: "The catalog does not match the schema modules on disk." });
    }
    const sorted = findings.slice().sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)) || Buffer.from(left.code).compare(Buffer.from(right.code)) || Buffer.from(left.detail).compare(Buffer.from(right.detail)));
    const reportIndex = args.indexOf("--report");
    if (reportIndex >= 0) {
      const target = args[reportIndex + 1];
      if (!target) throw new Error("[schema check] --report requires a path.");
      const path = isAbsolute(target) ? target : join(this.root, target);
      mkdirSync(dirname(path), { recursive: true });
      writeFileSync(path, renderSchemaCheckReport(sorted));
      console.log(`[schema check] wrote ${sorted.length} findings to ${target}`);
    } else for (const finding of sorted) console.log(JSON.stringify(finding));
    const counts = new Map<string, number>();
    for (const finding of sorted) counts.set(finding.code, (counts.get(finding.code) ?? 0) + 1);
    console.log(`[schema check] modules=${inventory.modules.length} scopes=${Object.keys(inventory.catalog.scopes).length} findings=${sorted.length}`);
    for (const [code, count] of [...counts].sort(([left], [right]) => left.localeCompare(right))) console.log(`[schema check] ${code}=${count}`);
    const shared = await this.sharedDiagnosticCodes();
    const unshared = [...counts.keys()].filter((code) => !shared.has(code)).sort();
    console.log(`[schema check] shared-code-table=${shared.size} unshared-codes=${unshared.length}${unshared.length === 0 ? "" : `: ${unshared.join(", ")}`}`);
    if (sorted.length > 0) process.exitCode = 1;
  }

  /**
   * 🔠️ The single diagnostic-code table both implementations of the rule set emit from: the harness owns
   * the `schema-*` vocabulary and this command reads it rather than restating a second one. The module is
   * addressed through the taxonomy's declared test-domain location, so this file still spells out no
   * implementation path of its own.
   * @see `SCHEMA_DIAGNOSTIC_CODE_TABLE` in the test domain's TypeScript package
   */
  private async sharedDiagnosticCodes(): Promise<ReadonlySet<string>> {
    const specifier = pathToFileURL(join(this.root, this.testDomainPath(), "📦️packages/🟦️typescript/🟦️.ts")).href;
    const loaded = (await import(specifier)) as { SCHEMA_DIAGNOSTIC_CODES?: readonly string[] };
    const codes = loaded.SCHEMA_DIAGNOSTIC_CODES;
    if (!Array.isArray(codes) || codes.length === 0) throw new Error("[schema check] the test domain package exports no SCHEMA_DIAGNOSTIC_CODES table.");
    return new Set(codes);
  }

  /** 🔒️ Joins current catalog facets to a fresh production registry without test inputs. */
  private async verify(args: string[], completed?: SchemaRustEntryDump): Promise<void> {
    if (args.includes("--rust-entries")) throw Error("[schema verify] Registry inputs are produced fresh; --out names the produced output.");
    const dump = completed ?? await this.productionEntries(args);
    const inventory = this.inventory();
    const stale: string[] = [];
    for (const [rel, rendered] of [[this.catalogPath(), renderSchemaCatalog(inventory.catalog)], [this.documentPath(), renderSchemaCatalogDocument(inventory.catalog)]] as const) {
      const abs = join(this.root, rel);
      if (!existsSync(abs) || readFileSync(abs, "utf8") !== rendered) stale.push(rel);
    }
    const catalog = this.readCatalog().parsed as { generator?: string; taxonomySchemaVersion?: number } | null;
    if (catalog && (catalog.generator !== inventory.catalog.generator || catalog.taxonomySchemaVersion !== inventory.catalog.taxonomySchemaVersion)) stale.push(`${this.catalogPath()} (provenance)`);
    if (stale.length > 0) console.error(`[schema verify] stale generated output: ${stale.join(", ")}. Run bun ./📜️script.ts schema generate && bun ./📜️script.ts schema docs.`);
    else console.log(`[schema verify] catalog and index are current (${Object.keys(inventory.catalog.scopes).length} scopes, generator ${inventory.catalog.generator}).`);
    const findings = schemaRustEntryDiagnostics(inventory.catalog, dump, loadCatalogTaxonomy(), args.includes("--rust-entries-complete"));
    const registered = new Set(dump.entries.map((entry) => entry.scope));
    for (const finding of findings) console.log(JSON.stringify(finding));
    const counts = new Map<string, number>();
    for (const finding of findings) counts.set(finding.code, (counts.get(finding.code) ?? 0) + 1);
    console.log(`[schema verify] rust entries=${dump.entries.length} scopes=${registered.size} of ${Object.keys(inventory.catalog.scopes).length} catalogued, findings=${findings.length}`);
    for (const [code, count] of [...counts].sort(([left], [right]) => left.localeCompare(right))) console.log(`[schema verify] ${code}=${count}`);
    if (findings.length > 0 || stale.length > 0) process.exit(1);
  }

  /** 📤️ Runs the genuine native producer and removes only its unique intermediate output. */
  private async productionEntries(args:string[]):Promise<SchemaRustEntryDump>{
    const outIndex=args.indexOf("--out"),name=outIndex>=0?args[outIndex+1]:undefined;if(outIndex>=0&&!name)throw Error("[schema] --out requires a report name");
    const {withRepositoryCacheStoreV1}=await import("../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts");
    return withRepositoryCacheStoreV1(this.root,"diagnostics",this.invocation,createIssuedResourceControlV1(this.invocation,"cache:store-entry",0),async store=>{
      const {requireNativeStoragePathV1}=await import("../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts"),base=requireNativeStoragePathV1(store.root,join(store.root,"tmp"));mkdirSync(base,{recursive:true});const directory=mkdtempSync(join(base,"registry-")),output=requireNativeStoragePathV1(store.root,join(directory,"entries.json")),module=join(this.root,"🧰️framework/🔨️modules/🧬️schema");
      const {loadProductionSchemaEntriesV1}=await import("../../🔍️discovery/📤️schema-registry/🟦️.ts"),{runOwnedCommand}=await import("../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts"),{scriptInvocationEnvironment,scriptInvocationBudget}=await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");let text="";
      try{await store.recheck();const dump=await loadProductionSchemaEntriesV1({produce:async()=>{await runOwnedCommand(process.execPath,[join(this.root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts"),"relay","run","--locked","--manifest-path",join(module,"📦️packages/🦀️rust/Cargo.toml"),"-p",SCHEMA_RUST_CRATE,"--bin","schema-export-entries","--","--out",output],this.root,"schema:registry-production",scriptInvocationBudget(this.invocation,buildBudgetMs()),{signal:this.invocation.control.signal,env:scriptInvocationEnvironment(this.invocation,store.environment)});await store.recheck();text=readFileSync(output,"utf8");return text;}},JSON.parse(readFileSync(join(module,"🔣️.json"),"utf8")));
        if(name){const {withRepositoryBuildReportV1}=await import("../../🏗️build/📊️reporting/🟦️.ts");await withRepositoryBuildReportV1(this.root,name,JSON.parse(text),this.invocation,async artifact=>{console.log(artifact.path);});}return dump;
      }finally{rmSync(directory,{recursive:true,force:true});}
    });
  }

  /**
   * ⚙️ Vitest matches spec files against its configured `include`, and the repository names a test case
   * `🧪️tests/<case>/🟦️.ts` rather than `*.test.ts`, so the run needs a config that names them. That config
   * is the module's own tracked vitest project, so adding an oracle spec to `framework.schema` needs no
   * edit here: the module declares its specs once, and this command only points vitest at that declaration.
   * @see `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🎚️config/🟦️.ts` owns this module's explicit test configuration.
   */
  private oracleArguments(): string[] {
    return ["vitest", "run", "--config", join(this.root, SCHEMA_ORACLE_VITEST_CONFIG)];
  }

  /** 🧪️ Runs the ajv draft-07 oracle spec that validates the owned Rust structural validator. */
  private oracle(): void {
    runCmd(process.execPath, ["x", ...this.oracleArguments()], { cwd: this.root });
  }

  /**
   * 🦀️ Runs one registered `semio-framework-schema` test binary with the environment variable that makes it
   * write its JSON report, and answers where that report landed. `--out <path>` redirects it; otherwise it
   * lands on the binary's declared `tracked` home, or — when the report is evidence rather than a contract,
   * the exit status being the gate — on a scratch file.
   */
  private rustReport({ test, variable, tracked }: { readonly test: string; readonly variable: string; readonly tracked?: string }, args: string[]): string {
    const outIndex = args.indexOf("--out");
    const target = outIndex >= 0 ? args[outIndex + 1] : tracked;
    if (outIndex >= 0 && !target) throw new Error(`[schema] --out requires a path.`);
    const path = target ? (isAbsolute(target) ? target : join(this.root, target)) : join(tmpdir(), `semio-${test}-${process.pid}.json`);
    mkdirSync(dirname(path), { recursive: true });
    runCmd("cargo", ["test", "-p", SCHEMA_RUST_CRATE, "--test", test], { cwd: this.root, env: { ...process.env, [variable]: path } });
    return path;
  }

  /** 🧩️ The repo-wide gate: every schema module compiles against the owned draft-07 validator. */
  private compile(args: string[]): void {
    const report = this.rustReport(SCHEMA_MODULE_COMPILE_TEST, args);
    console.log(`[schema compile] cargo test -p ${SCHEMA_RUST_CRATE} --test ${SCHEMA_MODULE_COMPILE_TEST.test} passed; report at ${report}`);
  }

  /** 📤️ Produces and verifies current registry entries once from the clean native binary. */
  private async entries(args: string[]): Promise<void> {
    if (args.includes("--rust-entries")) throw Error("[schema entries] Registry inputs are produced fresh; --out names the produced output.");
    await this.verify(args, await this.productionEntries(args));
  }

  /**
   * 🧪️ The whole schema gate: the harness invariants over the catalog and the third-party draft-07 oracle.
   * Both halves always run — a failing harness must not hide a failing oracle — and the exit code is their union.
   */
  private test(rest: string[]): void {
    const tracked = this.readCatalog();
    if (!tracked.parsed) throw new Error(`[schema test] ${this.catalogPath()} is absent or unreadable after a re-read. Run bun ./📜️script.ts schema generate before the gate; running the harness against a torn catalog reports every scope as malformed.`);
    const harness = runCmdStatus("bun", [`${this.testDomainPath()}/📜️script.ts`, "test", "schema", ...rest], { cwd: this.root });
    const oracle = runCmdStatus(process.execPath, ["x", ...this.oracleArguments()], { cwd: this.root });
    console.log(`[schema test] harness exit=${harness}, draft-07 oracle exit=${oracle}`);
    if (harness !== 0 || oracle !== 0) process.exit(1);
  }

  /** 📊️ Writes the full scope/diagnostic evidence pair into a report directory. */
  private audit(args: string[]): void {
    const outIndex = args.indexOf("--out");
    const target = args[outIndex + 1];
    if (outIndex < 0 || !target) throw new Error("[schema audit] --out <directory> is required.");
    const directory = isAbsolute(target) ? target : join(this.root, target);
    mkdirSync(directory, { recursive: true });
    const inventory = this.inventory();
    const findings = [...inventory.diagnostics, ...inventory.placement];
    const counts = new Map<string, number>();
    for (const finding of findings) counts.set(finding.code, (counts.get(finding.code) ?? 0) + 1);
    const byLevel = new Map<string, number>();
    for (const module of inventory.modules) byLevel.set(module.level ?? "(ineligible)", (byLevel.get(module.level ?? "(ineligible)") ?? 0) + 1);
    writeFileSync(join(directory, "📊️schema-audit.json"), `${JSON.stringify({ catalog: inventory.catalog, modules: inventory.modules.map(({ modulePath, ownerPath, level, facetKindId, scopeId }) => ({ modulePath, ownerPath, level, facetKindId, scopeId })), findings }, null, 2)}\n`);
    const lines = [
      "# 🧬️ Schema scope audit",
      "",
      `- modules: \`${inventory.modules.length}\``,
      `- catalogued scopes: \`${Object.keys(inventory.catalog.scopes).length}\``,
      `- findings: \`${findings.length}\``,
      "",
      "## Modules per declared level",
      "",
      "| Level | Modules |",
      "| --- | --- |",
      ...[...byLevel].sort(([left], [right]) => left.localeCompare(right)).map(([level, count]) => `| ${level} | ${count} |`),
      "",
      "## Findings per code",
      "",
      "| Code | Findings |",
      "| --- | --- |",
      ...[...counts].sort(([left], [right]) => left.localeCompare(right)).map(([code, count]) => `| \`${code}\` | ${count} |`),
      "",
    ];
    writeFileSync(join(directory, "📓️schema-audit.md"), lines.join("\n"));
    console.log(`[schema audit] ${inventory.modules.length} modules, ${findings.length} findings -> ${target}`);
  }

  /** 📓️ Rewrites the Markdown scope/export index beside the catalog. */
  private docs(): void {
    const inventory = this.inventory();
    const rel = this.documentPath();
    writeFileSync(join(this.root, rel), renderSchemaCatalogDocument(inventory.catalog));
    console.log(`[schema docs] ${rel}: ${Object.keys(inventory.catalog.scopes).length} scopes.`);
  }
}
//#endregion 🔖️SchemaScript
