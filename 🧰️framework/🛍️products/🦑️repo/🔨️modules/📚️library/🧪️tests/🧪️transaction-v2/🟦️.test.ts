//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — Transaction Plan/Journal v2 crash, recovery, and concurrency proof.
//#endregion 🧲️Header

//#region 🔌️Adapters
import { afterAll, describe, expect, test } from "bun:test";
import { execFileSync, spawn, spawnSync, type ChildProcess } from "node:child_process";
import { appendFileSync, chmodSync, constants, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, readlinkSync, renameSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { getWorkspaceRoot } from "../../📦️packages/🟦️typescript/📦️index.ts";
import { applyTaxonomyPlan, canonicalJson, inventoryTaxonomy, noFollowTreeDigest, parseTaxonomyPlan, planTaxonomy, taxonomyPlanDigest, type TaxonomyInventoryOptions, type TaxonomyPlan } from "../../🧹️normalization/🟦️.ts";
import { ownedFilePaths, ownedFilesystemEntries, ownedPathByteSort } from "../🔍️filesystem/🟦️component.ts";
//#endregion 🔌️Adapters

//#region 📜️Contracts
type Fixture = Readonly<{ baselineCommit: string; options: TaxonomyInventoryOptions; planCacheKey?: string; repoRoot: string; root: string; scope: string; ticketDir: string; workspace: string }>;
type Snapshot = Readonly<Record<string, string>>;

const TICKET_REL = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION";
const SCHEMA_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const TRANSACTION_GOLDEN = resolve(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️transaction-dispositions/🔣️.json");
const NORMALIZATION_MODULE = process.env.SEMIO_TRANSACTION_V2_MODULE ?? resolve(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts");
const FIXTURE_SCHEMA = process.env.SEMIO_TRANSACTION_V2_SCHEMA ?? resolve(getWorkspaceRoot(), SCHEMA_REL);
const FIXTURE_RUN_ID = process.env.SEMIO_TRANSACTION_V2_RUN_ID ?? `${process.pid}-${crypto.randomUUID()}`;
process.env.NX_DAEMON = "false";
type BoundaryNode = Readonly<{ path: string; kind: "directory"; mode: string } | { path: string; kind: "symlink"; mode: string; target: string; targetHash: string } | { path: string; kind: "file"; mode: string; size: number; sha256: string; bytesBase64: string; canonicalJson?: boolean; json?: unknown }>;
type BoundaryGolden = Readonly<{ boundaries?: Readonly<Record<string, Readonly<{ transaction: string; workspace: string }>>>; transactionLedgers?: Readonly<Record<string, readonly BoundaryNode[]>>; workspaceLedgers?: Readonly<Record<string, readonly BoundaryNode[]>> }>;
const transactionGolden = JSON.parse(readFileSync(TRANSACTION_GOLDEN, "utf8")) as BoundaryGolden;
const FAILURE_STAGES = ["after-staging", "after-embedded-root-staging", "after-moves", "after-relocations", "after-symlink-retargeting", "after-edits", "after-regenerations", "before-verify"] as const;
const KILL_PHASES = ["transaction-attempt-preparation-mkdir", "transaction-attempt-preparation-children", "transaction-initial-lease-json-write-mkdir", "transaction-initial-lease-json-candidate-written", "transaction-initial-lease-json-canonical-exchanged", "transaction-initial-lease-prepared", "transaction-initial-wal-mkdir", "transaction-initial-journal-write-mkdir", "transaction-initial-journal-candidate-written", "transaction-initial-journal-canonical-exchanged", "transaction-initial-journal-canonical", "transaction-attempt-canonical-published", "transaction-journal-write-mkdir", "transaction-journal-candidate-written", "transaction-journal-previous-exchanged", "transaction-journal-canonical-exchanged", "transaction-wal-prepared", "transaction-backup-write-mkdir", "transaction-backup-write-mid", "transaction-backup-write-prepared", "transaction-backup-inner-exchange", "transaction-backup-exchange", "transaction-backup-retained", "transaction-edit-write-mkdir", "transaction-edit-write-mid", "transaction-edit-write-prepared", "transaction-edit-inner-exchange", "transaction-edit-exchange", "transaction-edit-canonical-exchange"] as const;
const RESTORE_PHASES = ["transaction-restore-mkdir", "transaction-restore-prepared", "transaction-restore-exchange", "transaction-restore-canonical-exchange"] as const;
const LEASE_PHASES = ["transaction-lease-stale-quarantined", "transaction-lease-preparation-mkdir", "transaction-lease-json-write-mkdir", "transaction-lease-json-candidate-written", "transaction-lease-json-canonical-exchanged", "transaction-lease-prepared", "transaction-lease-canonical-published"] as const;
const LATE_KILL_PHASES = new Set<string>(KILL_PHASES.slice(12));
const EXACT_BOUNDARY_KEYS = [
  ...FAILURE_STAGES.map((stage) => `rolledback:${stage}`),
  ...[...KILL_PHASES, ...RESTORE_PHASES, ...LEASE_PHASES].flatMap((phase) => [`killed:${phase}`, `recovered:${phase}`]),
  "killed:transaction-terminal-committed-stage-removed", "recovered:transaction-terminal-committed-stage-removed",
  "killed:transaction-terminal-rolled-back-stage-removed", "recovered:transaction-terminal-rolled-back-stage-removed",
  "killed:process-tree-mixed-generator", "recovered:process-tree-mixed-generator", "committed:process-tree-mixed-generator",
  "rolledback:cancellation", "rolledback:caught-attempt-canonical-published", "rolledback:caught-journal-previous-exchanged",
].sort();
const fixtureTemplates = new Map<string, Readonly<{ baselineCommit: string; processOwned: boolean; root: string }>>();
const fixturePlans = new Map<string, TaxonomyPlan>();
const childEvidence = new WeakMap<ChildProcess, { output: string; error?: Error }>();
let activeReferenceTemplate: Fixture | undefined, activeReferenceTemplatePromise: Promise<Fixture> | undefined;
let activeReferenceTemplateProcessOwned = true;
afterAll(() => { for (const template of fixtureTemplates.values()) if (template.processOwned) retainFixture(template.root); if (activeReferenceTemplate && activeReferenceTemplateProcessOwned) retainFixture(activeReferenceTemplate.root); });

function registerChild(childProcess: ChildProcess): ChildProcess {
  const evidence: { output: string; error?: Error } = { output: "" };
  childEvidence.set(childProcess, evidence);
  for (const stream of [childProcess.stdout, childProcess.stderr]) stream?.on("data", (bytes) => { evidence.output = (evidence.output + String(bytes)).slice(-32_768); });
  childProcess.once("error", (error) => { evidence.error = error; });
  const registry = process.env.SEMIO_TRANSACTION_V2_PID_REGISTRY;
  if (registry && childProcess.pid) appendFileSync(registry, `${childProcess.pid}\n`);
  return childProcess;
}
//#endregion 📜️Contracts

//#region 🧪️Fixture
function retainFixture(root: string): void {
  if (dirname(root) !== resolve(getWorkspaceRoot(), TICKET_REL) || !basename(root).startsWith("🧪️s-transaction-v2-")) throw new Error("Unexpected transaction fixture root");
}

function git(root: string, args: readonly string[]): string {
  const result = Bun.spawnSync(["git", ...args], { cwd: root, env: { ...process.env, GIT_AUTHOR_DATE: "2000-01-01T00:00:00Z", GIT_COMMITTER_DATE: "2000-01-01T00:00:00Z" }, stdout: "pipe", stderr: "pipe" });
  const stdout = result.stdout.toString(), stderr = result.stderr.toString();
  if (result.exitCode !== 0) throw new Error(`fixture git ${args.join(" ")} failed: ${stderr || stdout}`);
  return stdout.trim();
}

function writeFiles(root: string, files: Readonly<Record<string, string>>): void {
  for (const [path, bytes] of Object.entries(files)) {
    const target = join(root, path);
    mkdirSync(dirname(target), { recursive: true });
    writeFileSync(target, bytes);
  }
}

function fixture(name: string, files: Readonly<Record<string, string>>, configure?: (row: Fixture) => void): Fixture {
  const owner = resolve(getWorkspaceRoot(), TICKET_REL);
  mkdirSync(owner, { recursive: true });
  const templateKey = `${canonicalJson(files)}\0${configure?.toString() ?? ""}`;
  let template = fixtureTemplates.get(templateKey);
  if (!template) {
    const shared = process.env.SEMIO_TRANSACTION_V2_RUN_ID !== undefined;
    const digest = new Bun.CryptoHasher("sha256").update(templateKey).digest("hex");
    const root = shared ? join(owner, `🧪️s-transaction-v2-${FIXTURE_RUN_ID}-shared-template-${digest}`) : mkdtempSync(join(owner, `🧪️s-transaction-v2-${FIXTURE_RUN_ID}-template-${process.pid}-`));
    let buildRoot = root, ownsBuild = !shared;
    if (shared && !existsSync(root)) {
      buildRoot = `${root}-preparing`;
      try { mkdirSync(buildRoot); ownsBuild = true; }
      catch (error) { if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error; }
      if (!ownsBuild) {
        const deadline = Date.now() + 10_000;
        while (!existsSync(root)) {
          if (Date.now() >= deadline) throw new Error(`Timed out waiting for shared fixture template ${digest}`);
          Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 2);
        }
      }
    }
    if (ownsBuild) {
      const ticketDir = join(buildRoot, "🧪️tests"), workspace = join(ticketDir, "🧪️fixture"), schemaPath = join(buildRoot, SCHEMA_REL), scope = relative(buildRoot, workspace).replaceAll("\\", "/");
      mkdirSync(workspace, { recursive: true });
      mkdirSync(dirname(schemaPath), { recursive: true });
      const taxonomy = JSON.parse(readFileSync(FIXTURE_SCHEMA, "utf8"));
      delete taxonomy.generatorContracts["plugin-registry"].inputDiscovery;
      writeFileSync(schemaPath, `${JSON.stringify(taxonomy, null, 2)}\n`);
      writeFiles(workspace, files);
      git(buildRoot, ["init", "--quiet", "--object-format=sha1"]);
      git(buildRoot, ["config", "user.name", "Semio Transaction Fixture"]);
      git(buildRoot, ["config", "user.email", "transaction-fixture@invalid.example"]);
      git(buildRoot, ["config", "commit.gpgsign", "false"]);
      configure?.({ baselineCommit: "", options: { repoRoot: buildRoot, scope, ticketDir, workers: 1 }, repoRoot: buildRoot, root: buildRoot, scope, ticketDir, workspace });
      git(buildRoot, ["add", "--all", "--", ".", `:(exclude,literal)${SCHEMA_REL}`]);
      git(buildRoot, ["commit", "--quiet", "-m", "transaction fixture"]);
      if (shared) renameSync(buildRoot, root);
    }
    const head = readFileSync(join(root, ".git", "HEAD"), "utf8").trim();
    template = { baselineCommit: head.startsWith("ref: ") ? readFileSync(join(root, ".git", head.slice(5)), "utf8").trim() : head, processOwned: !shared, root };
    fixtureTemplates.set(templateKey, template);
  }
  const root = mkdtempSync(join(owner, `🧪️s-transaction-v2-${FIXTURE_RUN_ID}-${name}-`));
  cpSync(template.root, root, { recursive: true, mode: constants.COPYFILE_FICLONE });
  const rebaseSymlinks = (path: string): void => {
    for (const name of readdirSync(path)) {
      const child = join(path, name), stat = lstatSync(child);
      if (stat.isDirectory() && !stat.isSymbolicLink()) rebaseSymlinks(child);
      else if (stat.isSymbolicLink()) {
        const target = readlinkSync(child);
        if (target.startsWith(template!.root)) { rmSync(child); symlinkSync(`${root}${target.slice(template!.root.length)}`, child, "file"); }
      }
    }
  };
  rebaseSymlinks(root);
  const ticketDir = join(root, "🧪️tests"), workspace = join(ticketDir, "🧪️fixture"), scope = relative(root, workspace).replaceAll("\\", "/");
  return { baselineCommit: template.baselineCommit, options: { repoRoot: root, scope, ticketDir, workers: 1 }, planCacheKey: configure?.toString().includes("symlinkSync") ? undefined : templateKey, repoRoot: root, root, scope, ticketDir, workspace };
}

function plan(row: Fixture): TaxonomyPlan {
  const cached = row.planCacheKey ? fixturePlans.get(row.planCacheKey) : undefined;
  if (cached) return cached;
  const value = planTaxonomy(inventoryTaxonomy(row.options), { baselineCommit: row.baselineCommit, excludedTreeDigests: [] });
  if (row.planCacheKey) fixturePlans.set(row.planCacheKey, value);
  return value;
}

function snapshot(root: string): Snapshot {
  if (!existsSync(root)) return {};
  const rows: [string, string][] = [];
  const visit = (path: string): void => {
    const absolute = path ? join(root, path) : root, stat = lstatSync(absolute), mode = (stat.mode & 0o7777).toString(8);
    if (stat.isSymbolicLink()) rows.push([path || ".", `symlink|${mode}|${Buffer.from(readlinkSync(absolute)).toString("base64")}`]);
    else if (stat.isFile()) rows.push([path || ".", `file|${mode}|${readFileSync(absolute).toString("base64")}`]);
    else if (stat.isDirectory()) {
      rows.push([path || ".", `directory|${mode}`]);
      for (const name of readdirSync(absolute).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)))) visit(path ? `${path}/${name}` : name);
    }
  };
  visit("");
  return Object.fromEntries(rows);
}

function transactionRoot(row: Fixture): string {
  return join(row.ticketDir, "🧾️taxonomy-transaction");
}

function journalPaths(row: Fixture): string[] {
  return ownedFilePaths(row.ticketDir)
    .filter((path) => path.startsWith("🧾️taxonomy-transaction/") && /\/🔂️attempts\/🔢️[^/]+\/🔣️\.json$/u.test(path))
    .map((path) => join(row.ticketDir, path))
    .sort(ownedPathByteSort);
}

function writePlan(row: Fixture, value: TaxonomyPlan): string {
  const path = join(row.ticketDir, "📊️taxonomy-plan", "🔣️.json");
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${canonicalJson(value)}\n`);
  return path;
}

function referenceFixture(name: string): Fixture {
  return fixture(name, {
    "🧪️subject/🟦️component.ts": "export const value = 1;\n",
    "🧪️consumer/🟦️component.ts": "export { value } from \"../🧪️subject/🟦️component.ts\";\n",
  });
}

function cloneFixture(source: Fixture, name: string): Fixture {
  const owner = resolve(getWorkspaceRoot(), TICKET_REL), root = mkdtempSync(join(owner, `🧪️s-transaction-v2-${FIXTURE_RUN_ID}-${name}-`));
  cpSync(source.root, root, { recursive: true, mode: constants.COPYFILE_FICLONE });
  const ticketDir = join(root, "🧪️tests"), workspace = join(ticketDir, "🧪️fixture"), scope = relative(root, workspace).replaceAll("\\", "/");
  return { baselineCommit: source.baselineCommit, options: { ...source.options, repoRoot: root, scope, ticketDir }, planCacheKey: source.planCacheKey, repoRoot: root, root, scope, ticketDir, workspace };
}

async function activeReferenceFixture(name: string): Promise<Fixture> {
  activeReferenceTemplatePromise ??= (async () => {
    const sharedRoot = process.env.SEMIO_TRANSACTION_V2_RUN_ID ? join(resolve(getWorkspaceRoot(), TICKET_REL), `🧪️s-transaction-v2-${FIXTURE_RUN_ID}-shared-active-reference`) : undefined;
    if (sharedRoot && existsSync(sharedRoot)) {
      const head = readFileSync(join(sharedRoot, ".git", "HEAD"), "utf8").trim(), baselineCommit = head.startsWith("ref: ") ? readFileSync(join(sharedRoot, ".git", head.slice(5)), "utf8").trim() : head;
      const ticketDir = join(sharedRoot, "🧪️tests"), workspace = join(ticketDir, "🧪️fixture"), scope = relative(sharedRoot, workspace).replaceAll("\\", "/");
      activeReferenceTemplateProcessOwned = false;
      activeReferenceTemplate = { baselineCommit, options: { repoRoot: sharedRoot, scope, ticketDir, workers: 1 }, planCacheKey: "active-reference-v2", repoRoot: sharedRoot, root: sharedRoot, scope, ticketDir, workspace };
      return activeReferenceTemplate;
    }
    const lock = sharedRoot ? `${sharedRoot}-preparing` : undefined;
    let ownsShared = false;
    if (lock) {
      try { mkdirSync(lock); ownsShared = true; }
      catch (error) { if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error; }
      if (!ownsShared) {
        const deadline = Date.now() + 10_000;
        while (!existsSync(sharedRoot!)) {
          if (Date.now() >= deadline) throw new Error("Timed out waiting for the shared active reference fixture");
          await Bun.sleep(2);
        }
        const head = readFileSync(join(sharedRoot!, ".git", "HEAD"), "utf8").trim(), baselineCommit = head.startsWith("ref: ") ? readFileSync(join(sharedRoot!, ".git", head.slice(5)), "utf8").trim() : head;
        const ticketDir = join(sharedRoot!, "🧪️tests"), workspace = join(ticketDir, "🧪️fixture"), scope = relative(sharedRoot!, workspace).replaceAll("\\", "/");
        activeReferenceTemplateProcessOwned = false;
        activeReferenceTemplate = { baselineCommit, options: { repoRoot: sharedRoot!, scope, ticketDir, workers: 1 }, planCacheKey: "active-reference-v2", repoRoot: sharedRoot!, root: sharedRoot!, scope, ticketDir, workspace };
        return activeReferenceTemplate;
      }
    }
    const row = referenceFixture("active-template"), value = plan(row);
    await killedAt(row, value, "transaction-wal-prepared");
    if (!sharedRoot) { activeReferenceTemplate = row; return row; }
    renameSync(row.root, sharedRoot);
    retainFixture(lock!);
    const ticketDir = join(sharedRoot, "🧪️tests"), workspace = join(ticketDir, "🧪️fixture"), scope = relative(sharedRoot, workspace).replaceAll("\\", "/");
    activeReferenceTemplateProcessOwned = false;
    activeReferenceTemplate = { ...row, options: { ...row.options, repoRoot: sharedRoot, scope, ticketDir }, planCacheKey: "active-reference-v2", repoRoot: sharedRoot, root: sharedRoot, scope, ticketDir, workspace };
    return activeReferenceTemplate;
  })();
  return cloneFixture(await activeReferenceTemplatePromise, name);
}

function symlinkFixture(name: string): Fixture {
  return fixture(name, { "🧪️target/🟦️component.ts": "export const target = true;\n" }, (row) => {
    const link = join(row.workspace, "🧪️link", "🟦️component.ts");
    mkdirSync(dirname(link), { recursive: true });
    symlinkSync("../🧪️target/🟦️component.ts", link);
  });
}

function embeddedFixture(name: string): Fixture {
  const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL";
  const files: Record<string, string> = {};
  for (const owner of ["pkg-a", "pkg-b"]) {
    files[`${owner}/${ticket}/🧪️target-os-errors/CACHEDIR.TAG`] = "Signature: 8a477f597d28d172789f06886806bc55\n";
    files[`${owner}/${ticket}/🧪️unique-${owner.slice(-1)}/CACHEDIR.TAG`] = "Signature: 8a477f597d28d172789f06886806bc55\n";
  }
  return fixture(name, files, (row) => {
    const canonicalManifest = join(row.repoRoot, ticket, "🎫️ticket.json");
    mkdirSync(dirname(canonicalManifest), { recursive: true });
    writeFileSync(canonicalManifest, '{"id":"26/08/17/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL"}\n');
    const schemaPath = join(row.repoRoot, SCHEMA_REL), taxonomy = JSON.parse(readFileSync(schemaPath, "utf8")) as Record<string, unknown>;
    const scope = relative(row.repoRoot, row.workspace).replaceAll("\\", "/");
    taxonomy.fixedFilenameContracts = Object.fromEntries(Object.entries({ ...(taxonomy.fixedFilenameContracts as Record<string, unknown>), "fixture-cache-tag": { pathPattern: `${scope}/**/CACHEDIR.TAG`, authority: "Transaction disposition golden", reason: "Exact cache marker fixture", configurability: "unconfigurable", scope: { kind: "path-pattern" }, verification: "fixture census", expires: null } }).sort(([left], [right]) => left.localeCompare(right)));
    taxonomy.fixedDirectoryContracts = Object.fromEntries(Object.entries({ ...(taxonomy.fixedDirectoryContracts as Record<string, unknown>), "fixture-package-prefix": { pathPattern: `${scope}/pkg-*`, authority: "Transaction disposition golden", reason: "Exact embedded-root owner fixture", configurability: "unconfigurable", scope: { kind: "path-pattern" }, verification: "fixture census", expires: null } }).sort(([left], [right]) => left.localeCompare(right)));
    writeFileSync(schemaPath, `${JSON.stringify(taxonomy, null, 2)}\n`);
  });
}

function generatorFixture(name: string): Fixture {
  return fixture(name, { "🧪️generator/🟦️.ts": "export const input = true;\n", "🧪️generator/🤖️generated/old.txt": "stale\n" }, (row) => {
    const owner = relative(row.repoRoot, join(row.workspace, "🧪️generator")).replaceAll("\\", "/"), outputRoot = `${owner}/🤖️generated`, schemaPath = join(row.repoRoot, SCHEMA_REL);
    const taxonomy = JSON.parse(readFileSync(schemaPath, "utf8")) as Record<string, unknown>;
    taxonomy.generatorContracts = Object.fromEntries(Object.entries({ ...(taxonomy.generatorContracts as Record<string, unknown>), "fixture-generator": { ownership: "owned", ownerPath: owner, target: "@fixture/generator:generate", previewTarget: "@fixture/generator:preview-generated", inputPatterns: [`${owner}/🟦️.ts`], outputRoots: [{ path: outputRoot, inclusion: "ignored" }], reason: "Transaction mixed-output fixture" } }).sort(([left], [right]) => left.localeCompare(right)));
    writeFileSync(schemaPath, `${JSON.stringify(taxonomy, null, 2)}\n`);
    writeFileSync(join(row.repoRoot, ".gitignore"), `${outputRoot}\n`);
    writeFileSync(join(row.repoRoot, "nx.json"), "{\"defaultBase\":\"main\"}\n");
    writeFileSync(join(row.repoRoot, "package.json"), "{\"name\":\"transaction-generator-fixture\",\"private\":true}\n");
    const project = `${JSON.stringify({ name: "@fixture/generator", root: owner, targets: { generate: { executor: "nx:run-commands", options: { cwd: owner, command: "bun ./📜️script.ts generate" } }, "preview-generated": { executor: "nx:run-commands", options: { cwd: owner, command: "bun ./📜️script.ts preview-generated" } }, check: { executor: "nx:run-commands", options: { cwd: owner, command: "bun ./📜️script.ts check" } } } }, null, 2)}\n`;
    writeFileSync(join(row.repoRoot, "project.json"), project);
    writeFiles(join(row.workspace, "🧪️generator"), {
      "📋️project.json": project,
      "📜️script.ts": [
        'import { existsSync, mkdirSync, readdirSync, rmSync, writeFileSync, readFileSync } from "node:fs";',
        'import { join } from "node:path";',
        'const outputRoot=join(process.cwd(), "🤖️generated");',
        `const outputRelative=${JSON.stringify(outputRoot)};`,
        'const outputFile=join(outputRoot, "🔤️.txt"), bytes=Buffer.from("generated\\n");',
        'const nodes=[{bytesBase64:"",mode:0o755,nodeKind:"directory",path:outputRelative},{bytesBase64:bytes.toString("base64"),mode:0o644,nodeKind:"file",path:`${outputRelative}/🔤️.txt`}];',
        'const staleRemovals=(existsSync(outputRoot)?readdirSync(outputRoot):[]).filter((name)=>name!=="🔤️.txt").map((name)=>`${outputRelative}/${name.normalize("NFC")}`).sort((a,b)=>Buffer.from(a).compare(Buffer.from(b)));',
        'const command=process.argv[2];',
        'if(command==="preview-generated") process.stdout.write(`${JSON.stringify({contractId:"fixture-generator",nodes,schemaVersion:1,staleRemovals})}\\n`);',
        'else if(command==="generate"){rmSync(outputRoot,{recursive:true,force:true});mkdirSync(outputRoot,{recursive:true,mode:0o755});writeFileSync(outputFile,bytes,{mode:0o644});if(process.env.MIXED_GENERATOR_MARKER){writeFileSync(join(outputRoot,"unexpected.txt"),"mixed\\n");writeFileSync(process.env.MIXED_GENERATOR_MARKER,`${JSON.stringify({pid:process.pid})}\\n`);Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,30000);}}',
        'else if(command==="check"){if(!existsSync(outputFile)||!readFileSync(outputFile).equals(bytes)||readdirSync(outputRoot).join("\\0")!=="🔤️.txt")throw new Error("generated output is stale");}',
        'else throw new Error(`unknown command ${command}`);',
        "",
      ].join("\n"),
    });
  });
}
//#endregion 🧪️Fixture

//#region 💥️ChildControl
const CHILD_SOURCE = `const [modulePath,planPath,repoRoot,ticketDir,baseline,resumeJournal,phase,marker,inject]=process.argv.slice(1);const {applyTaxonomyPlan}=await import(modulePath);const plan=JSON.parse(await Bun.file(planPath).text());applyTaxonomyPlan(plan,{repoRoot,ticketDir,expectedBaselineCommit:baseline,resumeJournal:resumeJournal||undefined,injectFailureAt:inject||undefined,progress:(row)=>{if(row.phase===phase){require("node:fs").writeFileSync(marker,"ready\\n");Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,30000);}}});`;

function child(row: Fixture, value: TaxonomyPlan, phase: string, marker: string, resumeJournal = "", inject = "", env: NodeJS.ProcessEnv = {}): ChildProcess {
  const planPath = writePlan(row, value);
  return registerChild(spawn(process.execPath, ["-e", CHILD_SOURCE, NORMALIZATION_MODULE, planPath, row.repoRoot, row.ticketDir, row.baselineCommit, resumeJournal, phase, marker, inject], { detached: process.platform !== "win32", env: { ...process.env, ...env }, stdio: ["ignore", "pipe", "pipe"] }));
}

function assertChildWaiting(childProcess: ChildProcess | undefined, path: string): void {
  if (!childProcess) return;
  const evidence = childEvidence.get(childProcess);
  if (evidence?.error || childProcess.exitCode !== null || childProcess.signalCode !== null) throw new Error(`Child ended before marker ${path}: ${evidence?.error?.message ?? childProcess.signalCode ?? childProcess.exitCode}\n${evidence?.output ?? ""}`);
}

async function waitFor(path: string, timeoutMs = 10_000, childProcess?: ChildProcess): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (!existsSync(path)) {
    assertChildWaiting(childProcess, path);
    if (Date.now() >= deadline) throw new Error(`Timed out waiting for ${path}`);
    await Bun.sleep(2);
  }
}

async function waitForGeneratorPid(path: string, childProcess: ChildProcess, timeoutMs = 12_000): Promise<number> {
  const deadline = Date.now() + timeoutMs;
  for (;;) {
    assertChildWaiting(childProcess, path);
    if (existsSync(path)) {
      try {
        const pid = (JSON.parse(readFileSync(path, "utf8")) as { pid?: unknown }).pid;
        if (Number.isSafeInteger(pid) && (pid as number) > 0) return pid as number;
      } catch {}
    }
    if (Date.now() >= deadline) throw new Error(`Timed out waiting for a complete generator marker at ${path}\n${childEvidence.get(childProcess)?.output ?? ""}`);
    await Bun.sleep(2);
  }
}

async function killedAt(row: Fixture, value: TaxonomyPlan, phase: string, resumeJournal = "", inject = ""): Promise<Readonly<{ transaction: Snapshot; workspace: Snapshot }>> {
  const marker = join(row.root, `marker-${phase}-${crypto.randomUUID()}`), childProcess = child(row, value, phase, marker, resumeJournal, inject);
  try {
    await waitFor(marker, 10_000, childProcess);
    const workspace = snapshot(row.workspace), transaction = snapshot(transactionRoot(row));
    expect(Object.values(transaction).some((entry) => entry.startsWith("directory|"))).toBe(true);
    killTree(childProcess);
    const exit = await boundedExit(childProcess);
    expect(exit.signal === "SIGKILL" || exit.code !== 0).toBe(true);
    return { transaction, workspace };
  } finally {
    if (childProcess.exitCode === null && childProcess.signalCode === null) { killTree(childProcess); await boundedExit(childProcess); }
  }
}

function killTree(childProcess: ChildProcess): void {
  if (!childProcess.pid) throw new Error("Child process has no pid");
  if (process.platform === "win32") {
    const killed = spawnSync("taskkill", ["/pid", String(childProcess.pid), "/t", "/f"], { stdio: "ignore" });
    if (killed.status !== 0) throw new Error(`taskkill failed for ${childProcess.pid}`);
  } else {
    try { process.kill(-childProcess.pid, "SIGKILL"); }
    catch (error) {
      if ((error as NodeJS.ErrnoException).code === "ESRCH") return;
      if ((error as NodeJS.ErrnoException).code === "EPERM") { childProcess.kill("SIGKILL"); return; }
      throw error;
    }
  }
}

function boundedExit(childProcess: ChildProcess, timeoutMs = 10_000): Promise<Readonly<{ code: number | null; signal: NodeJS.Signals | null }>> {
  if (childProcess.exitCode !== null || childProcess.signalCode !== null) return Promise.resolve({ code: childProcess.exitCode, signal: childProcess.signalCode });
  return new Promise((resolveExit, rejectExit) => {
    let timedOut = false;
    const timer = setTimeout(() => { timedOut = true; killTree(childProcess); }, timeoutMs);
    childProcess.once("exit", (code, signal) => { clearTimeout(timer); if (timedOut) rejectExit(new Error(`Child ${childProcess.pid ?? "unknown"} exit timed out`)); else resolveExit({ code, signal }); });
    childProcess.once("error", (error) => { clearTimeout(timer); rejectExit(error); });
  });
}

async function waitForExit(pid: number): Promise<void> {
  const deadline = Date.now() + 5_000;
  for (;;) {
    try { process.kill(pid, 0); }
    catch (error) { if ((error as NodeJS.ErrnoException).code === "ESRCH") return; throw error; }
    if (Date.now() >= deadline) throw new Error(`Descendant pid ${pid} remained alive after process-tree kill`);
    await Bun.sleep(2);
  }
}

async function killedMixedGenerator(row: Fixture, value: TaxonomyPlan): Promise<Readonly<{ journal: string; transaction: Snapshot; workspace: Snapshot }>> {
  const markerRoot = mkdtempSync(join(resolve(getWorkspaceRoot(), TICKET_REL), `🧪️s-transaction-v2-${FIXTURE_RUN_ID}-mixed-generator-marker-`)), marker = join(markerRoot, "ready");
  const planPath = writePlan(row, value), childProcess = registerChild(spawn(process.execPath, ["-e", `const [m,p,r,t,b]=process.argv.slice(1);const {applyTaxonomyPlan}=await import(m);applyTaxonomyPlan(JSON.parse(await Bun.file(p).text()),{repoRoot:r,ticketDir:t,expectedBaselineCommit:b});`, NORMALIZATION_MODULE, planPath, row.repoRoot, row.ticketDir, row.baselineCommit], { detached: process.platform !== "win32", env: { ...process.env, MIXED_GENERATOR_MARKER: marker }, stdio: ["ignore", "pipe", "pipe"] }));
  try {
    const generatorPid = await waitForGeneratorPid(marker, childProcess);
    if (process.env.SEMIO_TRANSACTION_V2_MIXED_READY) writeFileSync(process.env.SEMIO_TRANSACTION_V2_MIXED_READY, "ready\n");
    const workspace = snapshot(row.workspace), transaction = snapshot(transactionRoot(row));
    killTree(childProcess);
    const exit = await boundedExit(childProcess);
    expect(exit.signal === "SIGKILL" || exit.code !== 0).toBe(true);
    await waitForExit(generatorPid);
    const journals = journalPaths(row);
    expect(journals).toHaveLength(1);
    return { journal: journals[0], transaction, workspace };
  } finally {
    if (childProcess.exitCode === null && childProcess.signalCode === null) { killTree(childProcess); await boundedExit(childProcess); }
  }
}

function attemptJournal(row: Fixture, value: TaxonomyPlan, ordinal = 1): string {
  return join(transactionRoot(row), `🔖️${value.planDigest}`, "🔂️attempts", `🔢️${String(ordinal).padStart(6, "0")}`, "🔣️.json");
}

function expectCompleteSnapshot(value: Snapshot): void {
  const entries = Object.entries(value);
  expect(entries.length).toBeGreaterThan(0);
  expect(entries.every(([path, record]) => path.normalize("NFC") === path && /^(?:directory\|[0-7]+|file\|[0-7]+\|[A-Za-z0-9+/=]*|symlink\|[0-7]+\|[A-Za-z0-9+/=]*)$/u.test(record))).toBe(true);
  expect(JSON.parse(JSON.stringify(value))).toEqual(value);
}

function normalizedBoundaryPath(path: string, root: string): string {
  return path.replaceAll(root, "<root>")
    .replace(/[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}/gu, "<uuid>")
    .replace(/-([1-9][0-9]*)-<uuid>/gu, "-<pid>-<uuid>");
}

function normalizedBoundaryJson(value: unknown, key = "", root = ""): unknown {
  if (key === "pid" && typeof value === "number") return "<pid>";
  if (key === "token" && typeof value === "string" && /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/u.test(value)) return "<uuid>";
  if (typeof value === "string") {
    if (key === "contentHash" || key === "targetHash") return value;
    return normalizedBoundaryPath(value, root);
  }
  if (Array.isArray(value)) return value.map((entry) => normalizedBoundaryJson(entry, key, root));
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value as Record<string, unknown>).sort(([left], [right]) => Buffer.from(left).compare(Buffer.from(right))).map(([childKey, entry]) => [normalizedBoundaryPath(childKey, root), normalizedBoundaryJson(entry, childKey, root)]));
  return value;
}

function normalizedBoundaryLedger(value: Snapshot, root: string): BoundaryNode[] {
  return Object.entries(value).map(([path, record]) => {
    const [kind, mode, bytesBase64 = ""] = record.split("|");
    const normalizedPath = normalizedBoundaryPath(path, root);
    if (kind === "directory") return { path: normalizedPath, kind: "directory", mode };
    const bytes = Buffer.from(bytesBase64, "base64");
    if (kind === "symlink") {
      const target = normalizedBoundaryPath(bytes.toString("utf8"), root);
      return { path: normalizedPath, kind: "symlink", mode, target, targetHash: new Bun.CryptoHasher("sha256").update(target).digest("hex") };
    }
    if (path.endsWith(".json")) {
      const parsed = JSON.parse(bytes.toString("utf8")), normalized = normalizedBoundaryJson(parsed, "", root) as Record<string, unknown>;
      const canonical = `${canonicalJson(normalized)}\n`;
      return { path: normalizedPath, kind: "file", mode, size: Buffer.byteLength(canonical), sha256: new Bun.CryptoHasher("sha256").update(canonical).digest("hex"), bytesBase64: Buffer.from(canonical).toString("base64"), canonicalJson: bytes.toString("utf8") === `${canonicalJson(parsed)}\n`, json: normalized };
    }
    return { path: normalizedPath, kind: "file", mode, size: bytes.length, sha256: new Bun.CryptoHasher("sha256").update(bytes).digest("hex"), bytesBase64 };
  }).sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
}

function boundaryLedger(value: Snapshot, root: string): Readonly<{ digest: string; nodes: readonly BoundaryNode[] }> {
  const nodes = normalizedBoundaryLedger(value, root);
  return { digest: new Bun.CryptoHasher("sha256").update(canonicalJson(nodes)).digest("hex"), nodes };
}

function expectBoundaryGolden(transaction: Snapshot, workspace: Snapshot, key: string, root: string): void {
  const transactionLedger = boundaryLedger(transaction, root), workspaceLedger = boundaryLedger(workspace, root), capture = process.env.SEMIO_CAPTURE_TRANSACTION_BOUNDARIES;
  if (capture) {
    mkdirSync(capture, { recursive: true });
    const filename = `${new Bun.CryptoHasher("sha256").update(key).digest("hex")}.json`;
    writeFileSync(join(capture, filename), `${canonicalJson({ key, transaction: transactionLedger, workspace: workspaceLedger })}\n`);
    return;
  }
  expect(transactionGolden.boundaries?.[key]).toEqual({ transaction: transactionLedger.digest, workspace: workspaceLedger.digest });
  expect(transactionGolden.transactionLedgers?.[transactionLedger.digest]).toEqual(transactionLedger.nodes);
  expect(transactionGolden.workspaceLedgers?.[workspaceLedger.digest]).toEqual(workspaceLedger.nodes);
  if (process.env.SEMIO_TRANSACTION_V2_BOUNDARY_REGISTRY) appendFileSync(process.env.SEMIO_TRANSACTION_V2_BOUNDARY_REGISTRY, `${key}\n`);
}

function expectBoundaryTuple(transaction: Snapshot, workspace: Snapshot, phase: string, root: string): void {
  expectCompleteSnapshot(transaction);
  expectCompleteSnapshot(workspace);
  expectBoundaryGolden(transaction, workspace, `killed:${phase}`, root);
  const keys = Object.keys(transaction), files = keys.filter((path) => transaction[path].startsWith("file|"));
  for (const path of files.filter((entry) => entry.endsWith(".json"))) {
    const bytes = Buffer.from(transaction[path].split("|")[2], "base64").toString("utf8"), value = JSON.parse(bytes);
    expect(bytes).toBe(`${canonicalJson(value)}\n`);
  }
  const writeRoots = keys.filter((path) => transaction[path].startsWith("directory|") && /(?:^|\/)\S*write-[1-9][0-9]*-[0-9a-f-]+$/u.test(path));
  const descendants = (root: string): string[] => keys.filter((path) => path.startsWith(`${root}/`));
  const attemptPreparations = keys.filter((path) => transaction[path].startsWith("directory|") && /(?:^|\/)[^/]*prepare-000001-[^/]+$/u.test(path));
  if (phase === "transaction-attempt-preparation-mkdir") { expect(attemptPreparations).toHaveLength(1); expect(descendants(attemptPreparations[0])).toEqual([]); }
  if (phase === "transaction-attempt-preparation-children") { expect(attemptPreparations).toHaveLength(1); expect(descendants(attemptPreparations[0]).filter((path) => transaction[path].startsWith("directory|"))).toHaveLength(3); }
  if (phase.endsWith("write-mkdir")) { expect(writeRoots).toHaveLength(1); expect(descendants(writeRoots[0])).toEqual([]); }
  if (phase.endsWith("candidate-written") || phase.endsWith("write-mid") || phase.endsWith("write-prepared")) { expect(writeRoots).toHaveLength(1); expect(descendants(writeRoots[0]).filter((path) => transaction[path].startsWith("file|"))).toHaveLength(1); }
  if (phase === "transaction-journal-previous-exchanged") {
    expect(writeRoots).toHaveLength(1);
    expect(descendants(writeRoots[0]).filter((path) => path.endsWith(".json"))).toHaveLength(2);
    expect(keys.some((path) => /\/🔢️000001\/🔣️\.json$/u.test(path))).toBe(false);
  }
  if (phase === "transaction-journal-canonical-exchanged") {
    expect(writeRoots).toHaveLength(1);
    expect(descendants(writeRoots[0]).filter((path) => path.includes("⏮️"))).toHaveLength(1);
    expect(keys.some((path) => /\/🔢️000001\/🔣️\.json$/u.test(path))).toBe(true);
  }
  if (phase.includes("json-canonical-exchanged")) { expect(writeRoots).toHaveLength(1); expect(descendants(writeRoots[0])).toEqual([]); }
  if (phase === "transaction-initial-lease-prepared") { expect(writeRoots).toEqual([]); expect(keys.some((path) => path.includes("🔒️lease/🔣️.json"))).toBe(true); }
  if (phase === "transaction-initial-wal-mkdir") expect(keys.some((path) => path.endsWith("🚧️journal") && descendants(path).length === 0)).toBe(true);
  if (phase === "transaction-initial-journal-canonical") expect(keys.some((path) => path.includes("prepare-000001-") && path.endsWith("🔣️.json"))).toBe(true);
  if (phase === "transaction-attempt-canonical-published") expect(keys.some((path) => /\/🔢️000001\/🔣️\.json$/u.test(path))).toBe(true);
  if (phase === "transaction-wal-prepared") expect(keys.some((path) => path.endsWith("🚧️journal") && descendants(path).length === 0)).toBe(true);
  if (phase === "transaction-backup-inner-exchange") expect(keys.filter((path) => path.endsWith(".backup"))).toHaveLength(1);
  if (phase === "transaction-backup-exchange") { expect(writeRoots).toEqual([]); expect(keys.filter((path) => path.endsWith(".backup"))).toHaveLength(1); }
  if (phase === "transaction-backup-retained") { expect(writeRoots).toEqual([]); expect(keys.filter((path) => path.endsWith(".backup"))).toHaveLength(2); }
  if (phase === "transaction-edit-inner-exchange") expect(keys.filter((path) => path.endsWith(".edit"))).toHaveLength(1);
  if (phase === "transaction-edit-exchange") { expect(keys.filter((path) => path.endsWith(".edit"))).toHaveLength(1); expect(keys.filter((path) => path.endsWith(".pre"))).toHaveLength(1); }
  if (phase === "transaction-edit-canonical-exchange") { expect(keys.filter((path) => path.endsWith(".edit"))).toEqual([]); expect(keys.filter((path) => path.endsWith(".pre"))).toHaveLength(1); }
  if (phase === "transaction-restore-mkdir") expect(keys.filter((path) => path.includes("restore-")).some((path) => descendants(path).length === 0)).toBe(true);
  if (phase === "transaction-restore-prepared") expect(keys.filter((path) => path.endsWith(".backup"))).toHaveLength(2);
  if (phase === "transaction-restore-exchange") { expect(keys.filter((path) => path.endsWith(".backup"))).toHaveLength(2); expect(keys.filter((path) => path.endsWith(".post"))).toHaveLength(1); }
  if (phase === "transaction-restore-canonical-exchange") { expect(keys.filter((path) => path.endsWith(".backup"))).toHaveLength(1); expect(keys.filter((path) => path.endsWith(".post"))).toHaveLength(1); }
  if (phase === "transaction-lease-stale-quarantined") { expect(keys.some((path) => path.includes("-stale"))).toBe(true); expect(keys.some((path) => path.includes("🔒️lease/🔣️.json"))).toBe(false); }
  if (phase === "transaction-lease-preparation-mkdir") expect(keys.some((path) => path.includes("-preparing") && descendants(path).length === 0)).toBe(true);
  if (phase === "transaction-lease-prepared") expect(keys.some((path) => path.includes("-preparing/🔣️.json"))).toBe(true);
  if (phase === "transaction-lease-canonical-published") expect(keys.some((path) => path.includes("🔒️lease/🔣️.json"))).toBe(true);
  if (phase.includes("terminal-") && phase.endsWith("stage-removed")) {
    expect(keys.some((path) => path.includes("🚧️stage"))).toBe(false);
    expect(keys.some((path) => path.includes("💾️backup"))).toBe(true);
    expect(keys.some((path) => path.includes("🔒️lease/🔣️.json"))).toBe(true);
  }
}

function expectTerminalAttempts(row: Fixture, state: "committed" | "rolled-back"): void {
  const journals = journalPaths(row);
  expect(journals.length).toBeGreaterThan(0);
  expect(JSON.parse(readFileSync(journals.at(-1)!, "utf8")).state).toBe(state);
  for (const journal of journals) expect(readdirSync(dirname(journal))).toEqual(["🔣️.json"]);
}

function expectEmptyPlan(row: Fixture): void {
  const value = planTaxonomy(inventoryTaxonomy(row.options), { baselineCommit: row.baselineCommit, excludedTreeDigests: [] });
  expect({
    moves: value.moves,
    embeddedTicketRoots: value.embeddedTicketRoots,
    embeddedTicketRootRelocations: value.embeddedTicketRootRelocations,
    symlinkTargetEdits: value.symlinkTargetEdits,
    evidenceRemovals: value.evidenceRemovals,
    edits: value.edits,
    regenerations: value.regenerations,
    unresolved: value.unresolved,
  }).toEqual({ moves: [], embeddedTicketRoots: [], embeddedTicketRootRelocations: [], symlinkTargetEdits: [], evidenceRemovals: [], edits: [], regenerations: [], unresolved: [] });
}
//#endregion 💥️ChildControl

//#region 🧾️TransactionV2
describe("transaction plan journal v2 aggregate", () => {
  test("keeps the language-neutral golden aligned with owned no-follow enumeration", () => {
    const golden = JSON.parse(readFileSync(TRANSACTION_GOLDEN, "utf8")) as BoundaryGolden & { failureStages: string[]; journalStates: string[]; virtualPreimageNodes: { path: string; state: string }[] };
    expect(golden.failureStages).toEqual(FAILURE_STAGES);
    expect(golden.journalStates).toHaveLength(11);
    expect(Object.keys(golden.boundaries ?? {}).sort()).toEqual(EXACT_BOUNDARY_KEYS);
    for (const [owner, ledgers] of [["transaction", golden.transactionLedgers], ["workspace", golden.workspaceLedgers]] as const) for (const [digest, nodes] of Object.entries(ledgers ?? {})) {
      expect(new Bun.CryptoHasher("sha256").update(canonicalJson(nodes)).digest("hex")).toBe(digest);
      for (const node of nodes) {
        if (node.kind === "symlink") expect(new Bun.CryptoHasher("sha256").update(node.target).digest("hex")).toBe(node.targetHash);
        if (node.kind === "file") {
          const bytes = Buffer.from(node.bytesBase64, "base64");
          expect(bytes.length).toBe(node.size);
          expect(new Bun.CryptoHasher("sha256").update(bytes).digest("hex")).toBe(node.sha256);
          if (node.json !== undefined) { if (owner === "transaction") expect(node.canonicalJson).toBe(true); expect(bytes.toString("utf8")).toBe(`${canonicalJson(node.json)}\n`); }
        }
      }
    }
    const referencedTransactions = [...new Set(Object.values(golden.boundaries ?? {}).map((entry) => entry.transaction))].sort(), referencedWorkspaces = [...new Set(Object.values(golden.boundaries ?? {}).map((entry) => entry.workspace))].sort();
    expect(Object.keys(golden.transactionLedgers ?? {}).sort()).toEqual(referencedTransactions);
    expect(Object.keys(golden.workspaceLedgers ?? {}).sort()).toEqual(referencedWorkspaces);
    for (const reference of Object.values(golden.boundaries ?? {})) {
      expect(golden.transactionLedgers?.[reference.transaction]).toBeDefined();
      expect(golden.workspaceLedgers?.[reference.workspace]).toBeDefined();
    }
    const root = mkdtempSync(join(resolve(getWorkspaceRoot(), TICKET_REL), "🧪️s-transaction-v2-parity-"));
    try {
      mkdirSync(join(root, "evidence", "directory"), { recursive: true });
      writeFileSync(join(root, "evidence", "file.txt"), "sentinel\n");
      symlinkSync("../file.txt", join(root, "evidence", "link"));
      expect(ownedFilesystemEntries(join(root, "evidence"), true).map(({ path }) => path === "." ? "evidence" : `evidence/${path}`)).toEqual(["evidence", "evidence/directory", "evidence/file.txt", "evidence/link"]);
      expect(noFollowTreeDigest(root, "evidence")).toMatchObject({ directories: 2, files: 1, symlinks: 1 });
      expect(golden.virtualPreimageNodes.some((entry) => entry.state === "symlink")).toBe(true);
    } finally { retainFixture(root); }
  });

  test("rejects incomplete plans and plan-digest drift", async () => {
    expect(() => parseTaxonomyPlan({ schemaVersion: 1 })).toThrow();
    const row = referenceFixture("strict-plan");
    try {
      expect(git(row.root, ["ls-files", "--", SCHEMA_REL])).toBe("");
      const value = plan(row), drift = JSON.parse(JSON.stringify(value)) as TaxonomyPlan;
      (drift as { planDigest: string }).planDigest = "0".repeat(64);
      expect(() => parseTaxonomyPlan(drift)).toThrow();
      expect(taxonomyPlanDigest(value)).toBe(value.planDigest);
      const failed = registerChild(spawn(process.execPath, ["-e", 'process.stderr.write("[DEBUG] deliberate fixture child failure\\n"); process.exit(7);'], { stdio: ["ignore", "pipe", "pipe"] }));
      await expect(waitFor(join(row.root, "missing-marker"), 1_000, failed)).rejects.toThrow("deliberate fixture child failure");
      expect(failed.exitCode).toBe(7);
    } finally { retainFixture(row.root); }
  });

  const failureCases = [
    { stage: "after-staging", fixture: referenceFixture, relevant: (value: TaxonomyPlan) => value.moves.length },
    { stage: "after-embedded-root-staging", fixture: embeddedFixture, relevant: (value: TaxonomyPlan) => value.embeddedTicketRoots.length },
    { stage: "after-moves", fixture: referenceFixture, relevant: (value: TaxonomyPlan) => value.moves.length },
    { stage: "after-relocations", fixture: embeddedFixture, relevant: (value: TaxonomyPlan) => value.embeddedTicketRootRelocations.length },
    { stage: "after-symlink-retargeting", fixture: symlinkFixture, relevant: (value: TaxonomyPlan) => value.symlinkTargetEdits.length },
    { stage: "after-edits", fixture: referenceFixture, relevant: (value: TaxonomyPlan) => value.edits.length },
    { stage: "after-regenerations", fixture: generatorFixture, relevant: (value: TaxonomyPlan) => value.regenerations.length },
    { stage: "before-verify", fixture: referenceFixture, relevant: (value: TaxonomyPlan) => value.moves.length + value.edits.length },
  ] as const;

  for (const failure of failureCases) test.concurrent(`rolls back ${failure.stage} with non-empty phase authority`, () => {
    const row = failure.fixture(`failure-${failure.stage}`);
    try {
      const value = plan(row), before = snapshot(row.workspace);
      expect(failure.relevant(value)).toBeGreaterThan(0);
      expect(applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, expectedPlanDigest: value.planDigest, injectFailureAt: failure.stage }).state).toBe("rolled-back");
      expect(snapshot(row.workspace)).toEqual(before);
      expectTerminalAttempts(row, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(row)), snapshot(row.workspace), `rolledback:${failure.stage}`, row.root);
      if (failure.stage === "before-verify") {
        const retry = applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit });
        expect(retry.state).toBe("committed");
        expect(retry.journalPath).toMatch(/🔢️000002\/🔣️\.json$/u);
        expectEmptyPlan(row);
      }
    } finally { retainFixture(row.root); }
  }, 15_000);

  for (const phase of KILL_PHASES) test.concurrent(`recovers parent-killed ${phase}`, async () => {
    const active = LATE_KILL_PHASES.has(phase), row = active ? await activeReferenceFixture(`kill-${phase}`) : referenceFixture(`kill-${phase}`);
    try {
      const value = plan(row), before = snapshot(row.workspace), killed = await killedAt(row, value, phase, active ? attemptJournal(row, value) : "");
      expectBoundaryTuple(killed.transaction, killed.workspace, phase, row.root);
      const journal = attemptJournal(row, value), canonicalAttempt = existsSync(dirname(journal));
      const injectFailureAt = phase.startsWith("transaction-backup") || phase.startsWith("transaction-edit") ? "after-edits" : "after-staging";
      const result = applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, ...(canonicalAttempt ? { resumeJournal: journal } : {}), injectFailureAt });
      expect(result.state).toBe("rolled-back");
      expect(snapshot(row.workspace)).toEqual(before);
      expectTerminalAttempts(row, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(row)), snapshot(row.workspace), `recovered:${phase}`, row.root);
    } finally { retainFixture(row.root); }
  }, 15_000);

  for (const phase of RESTORE_PHASES) test.concurrent(`recovers parent-killed ${phase}`, async () => {
    const row = await activeReferenceFixture(`kill-${phase}`);
    try {
      const value = plan(row), before = snapshot(row.workspace), killed = await killedAt(row, value, phase, attemptJournal(row, value), "after-edits");
      expectBoundaryTuple(killed.transaction, killed.workspace, phase, row.root);
      expect(applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, resumeJournal: attemptJournal(row, value) }).state).toBe("rolled-back");
      expect(snapshot(row.workspace)).toEqual(before);
      expectTerminalAttempts(row, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(row)), snapshot(row.workspace), `recovered:${phase}`, row.root);
    } finally { retainFixture(row.root); }
  }, 15_000);

  for (const phase of LEASE_PHASES) test.concurrent(`recovers parent-killed ${phase}`, async () => {
    const row = await activeReferenceFixture(`kill-${phase}`);
    try {
      const value = plan(row), before = snapshot(row.workspace);
      const killed = await killedAt(row, value, phase, attemptJournal(row, value));
      expectBoundaryTuple(killed.transaction, killed.workspace, phase, row.root);
      expect(applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, resumeJournal: attemptJournal(row, value), injectFailureAt: "after-staging" }).state).toBe("rolled-back");
      expect(snapshot(row.workspace)).toEqual(before);
      expectTerminalAttempts(row, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(row)), snapshot(row.workspace), `recovered:${phase}`, row.root);
    } finally { retainFixture(row.root); }
  }, 15_000);

  test.concurrent("recovers parent-killed committed and rolled-back backup-only terminal cleanup", async () => {
    const committed = referenceFixture("terminal-committed"), rolledBack = referenceFixture("terminal-rolled-back");
    try {
      const committedPlan = plan(committed), committedKill = await killedAt(committed, committedPlan, "transaction-terminal-committed-stage-removed");
      expectBoundaryTuple(committedKill.transaction, committedKill.workspace, "transaction-terminal-committed-stage-removed", committed.root);
      expect(applyTaxonomyPlan(committedPlan, { repoRoot: committed.repoRoot, ticketDir: committed.ticketDir, expectedBaselineCommit: committed.baselineCommit, resumeJournal: attemptJournal(committed, committedPlan) }).state).toBe("committed");
      expectTerminalAttempts(committed, "committed");
      expectBoundaryGolden(snapshot(transactionRoot(committed)), snapshot(committed.workspace), "recovered:transaction-terminal-committed-stage-removed", committed.root);
      const rolledPlan = plan(rolledBack), before = snapshot(rolledBack.workspace), rolledKill = await killedAt(rolledBack, rolledPlan, "transaction-terminal-rolled-back-stage-removed", "", "after-edits");
      expectBoundaryTuple(rolledKill.transaction, rolledKill.workspace, "transaction-terminal-rolled-back-stage-removed", rolledBack.root);
      expect(applyTaxonomyPlan(rolledPlan, { repoRoot: rolledBack.repoRoot, ticketDir: rolledBack.ticketDir, expectedBaselineCommit: rolledBack.baselineCommit, injectFailureAt: "after-staging" }).state).toBe("rolled-back");
      expect(snapshot(rolledBack.workspace)).toEqual(before);
      expect(journalPaths(rolledBack).map((path) => JSON.parse(readFileSync(path, "utf8")).state)).toEqual(["rolled-back", "rolled-back"]);
      for (const journal of journalPaths(rolledBack)) expect(readdirSync(dirname(journal))).toEqual(["🔣️.json"]);
      expectBoundaryGolden(snapshot(transactionRoot(rolledBack)), snapshot(rolledBack.workspace), "recovered:transaction-terminal-rolled-back-stage-removed", rolledBack.root);
    } finally { retainFixture(committed.root); retainFixture(rolledBack.root); }
  }, 15_000);

  test.concurrent("rolls back a process-tree-killed mixed generator and commits ordinal two", async () => {
    const row = generatorFixture("mixed-generator");
    try {
      const value = plan(row), before = snapshot(row.workspace);
      expect(value.regenerations).toHaveLength(1);
      const killed = await killedMixedGenerator(row, value);
      expectCompleteSnapshot(killed.transaction);
      expectCompleteSnapshot(killed.workspace);
      expectBoundaryGolden(killed.transaction, killed.workspace, "killed:process-tree-mixed-generator", row.root);
      expect(Object.values(killed.workspace).some((entry) => entry.includes(Buffer.from("mixed\n").toString("base64")))).toBe(true);
      expect(applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, resumeJournal: killed.journal }).state).toBe("rolled-back");
      expect(snapshot(row.workspace)).toEqual(before);
      expectBoundaryGolden(snapshot(transactionRoot(row)), snapshot(row.workspace), "recovered:process-tree-mixed-generator", row.root);
      const committed = applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit });
      expect(committed.journalPath).toMatch(/🔢️000002\/🔣️\.json$/u);
      expect(journalPaths(row).map((path) => JSON.parse(readFileSync(path, "utf8")).state)).toEqual(["rolled-back", "committed"]);
      expectTerminalAttempts(row, "committed");
      expectEmptyPlan(row);
      expectBoundaryGolden(snapshot(transactionRoot(row)), snapshot(row.workspace), "committed:process-tree-mixed-generator", row.root);
    } finally { retainFixture(row.root); }
  }, 15_000);

  test.concurrent("elects exactly one synchronized stale-lease contender and permits committed retry", async () => {
    const row = await activeReferenceFixture("contenders");
    let contenders: ChildProcess[] = [];
    try {
      const value = plan(row);
      const journal = attemptJournal(row, value), beforeTransaction = snapshot(transactionRoot(row)), beforeWorkspace = snapshot(row.workspace);
      const barrier = mkdtempSync(join(row.root, "barrier-")), acquireRelease = join(barrier, "acquire"), winnerRelease = join(barrier, "winner"), owner = join(barrier, "owner");
      const contenderSource = `const [m,p,r,t,baseline,j,b,acquire,winner,owner]=process.argv.slice(1);const {applyTaxonomyPlan}=await import(m);const fs=require("node:fs"),wait=(path)=>{while(!fs.existsSync(path))Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,2)},result=b+"/result-"+process.pid;try{applyTaxonomyPlan(JSON.parse(await Bun.file(p).text()),{repoRoot:r,ticketDir:t,expectedBaselineCommit:baseline,resumeJournal:j,progress:(row)=>{if(row.phase==="transaction-lease-scanned"){fs.writeFileSync(b+"/ready-"+process.pid,"ready\\n");wait(acquire)}if(row.phase==="transaction-lease-canonical-published"){fs.writeFileSync(owner,String(process.pid));wait(winner)}}});fs.writeFileSync(result,"success\\n")}catch(error){fs.writeFileSync(result,"failure:"+(error instanceof Error?error.message:String(error))+"\\n");process.exitCode=17}`;
      const planPath = writePlan(row, value);
      contenders = [0, 1].map(() => registerChild(spawn(process.execPath, ["-e", contenderSource, NORMALIZATION_MODULE, planPath, row.repoRoot, row.ticketDir, row.baselineCommit, journal, barrier, acquireRelease, winnerRelease, owner], { detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"] })));
      const deadline = Date.now() + 10_000;
      while (readdirSync(barrier).filter((name) => name.startsWith("ready-")).length < 2) { if (Date.now() >= deadline) throw new Error("Contender barrier timed out"); await Bun.sleep(2); }
      writeFileSync(acquireRelease, "go\n");
      while (!existsSync(owner) || readdirSync(barrier).filter((name) => name.startsWith("result-") && readFileSync(join(barrier, name), "utf8").startsWith("failure:")).length !== 1) { if (Date.now() >= deadline) throw new Error("Contender election timed out"); await Bun.sleep(2); }
      expect(snapshot(row.workspace)).toEqual(beforeWorkspace);
      expect(snapshot(transactionRoot(row))).not.toEqual(beforeTransaction);
      expect(Object.keys(snapshot(transactionRoot(row))).some((path) => path.includes("-preparing"))).toBe(false);
      writeFileSync(winnerRelease, "go\n");
      const exits = await Promise.all(contenders.map((childProcess) => boundedExit(childProcess)));
      expect(exits.filter((entry) => entry.code === 0)).toHaveLength(1);
      expect(exits.filter((entry) => entry.code === 17)).toHaveLength(1);
      const failure = readdirSync(barrier).filter((name) => name.startsWith("result-")).map((name) => readFileSync(join(barrier, name), "utf8")).find((value) => value.startsWith("failure:"));
      expect(failure).toMatch(/lease acquisition failed|leased by active pid|concurrent canonical lease|stale lease destination changed/u);
      expectTerminalAttempts(row, "committed");
      expect(applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, resumeJournal: journal }).state).toBe("committed");
      expectTerminalAttempts(row, "committed");
    } finally {
      for (const contender of contenders) if (contender.exitCode === null && contender.signalCode === null) killTree(contender);
      await Promise.all(contenders.map((contender) => boundedExit(contender, 5_000).catch(() => undefined)));
      for (const contender of contenders) expect(contender.exitCode !== null || contender.signalCode !== null).toBe(true);
      retainFixture(row.root);
    }
  }, 15_000);

  test.concurrent("restores a quarantined stale lease exactly when acquisition callback throws", async () => {
    const row = await activeReferenceFixture("stale-lease-restoration");
    try {
      const value = plan(row);
      const beforeTransaction = snapshot(transactionRoot(row)), beforeWorkspace = snapshot(row.workspace);
      expect(() => applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, resumeJournal: attemptJournal(row, value), progress: (event) => { if (event.phase === "transaction-lease-stale-quarantined") throw new Error("forced callback failure"); } })).toThrow(/forced callback failure/u);
      expect(snapshot(transactionRoot(row))).toEqual(beforeTransaction);
      expect(snapshot(row.workspace)).toEqual(beforeWorkspace);
    } finally { retainFixture(row.root); }
  }, 15_000);

  test.concurrent("rejects stale baseline, source digest, and counterfeit transaction segment with zero mutation", () => {
    const baseline = referenceFixture("stale-baseline"), source = referenceFixture("stale-source"), counterfeit = referenceFixture("counterfeit-segment");
    try {
      const baselinePlan = plan(baseline), baselineTransaction = snapshot(transactionRoot(baseline)), baselineWorkspace = snapshot(baseline.workspace);
      expect(() => applyTaxonomyPlan(baselinePlan, { repoRoot: baseline.repoRoot, ticketDir: baseline.ticketDir, expectedBaselineCommit: "0".repeat(40) })).toThrow(/expectedBaselineCommit/u);
      expect(snapshot(transactionRoot(baseline))).toEqual(baselineTransaction);
      expect(snapshot(baseline.workspace)).toEqual(baselineWorkspace);
      const sourcePlan = plan(source);
      writeFileSync(join(source.workspace, "unrelated.txt"), "drift\n");
      const sourceTransaction = snapshot(transactionRoot(source)), sourceWorkspace = snapshot(source.workspace);
      expect(() => applyTaxonomyPlan(sourcePlan, { repoRoot: source.repoRoot, ticketDir: source.ticketDir, expectedBaselineCommit: source.baselineCommit })).toThrow(/source-tree digest/u);
      expect(snapshot(transactionRoot(source))).toEqual(sourceTransaction);
      expect(snapshot(source.workspace)).toEqual(sourceWorkspace);
      const counterfeitPlan = plan(counterfeit), counterfeitPath = join(counterfeit.workspace, "🧾️taxonomy-transaction", "plain.txt");
      mkdirSync(dirname(counterfeitPath), { recursive: true });
      writeFileSync(counterfeitPath, "counterfeit\n");
      const counterfeitTransaction = snapshot(transactionRoot(counterfeit)), counterfeitWorkspace = snapshot(counterfeit.workspace);
      expect(() => applyTaxonomyPlan(counterfeitPlan, { repoRoot: counterfeit.repoRoot, ticketDir: counterfeit.ticketDir, expectedBaselineCommit: counterfeit.baselineCommit })).toThrow();
      expect(snapshot(transactionRoot(counterfeit))).toEqual(counterfeitTransaction);
      expect(snapshot(counterfeit.workspace)).toEqual(counterfeitWorkspace);
      const inventory = inventoryTaxonomy(counterfeit.options);
      expect(inventory.entries.some((entry) => entry.sourcePath.endsWith("🧾️taxonomy-transaction/plain.txt"))).toBe(true);
    } finally { retainFixture(baseline.root); retainFixture(source.root); retainFixture(counterfeit.root); }
  }, 15_000);

  test.concurrent("rejects stale resume preimage and malformed resume evidence byte-for-byte", async () => {
    const stale = await activeReferenceFixture("stale-resume"), forged = await activeReferenceFixture("forged-resume"), missing = referenceFixture("missing-resume");
    try {
      const stalePlan = plan(stale);
      writeFileSync(join(stale.workspace, "🧪️subject", "🟦️component.ts"), "export const value = 2;\n");
      const staleTransaction = snapshot(transactionRoot(stale)), staleWorkspace = snapshot(stale.workspace);
      expect(() => applyTaxonomyPlan(stalePlan, { repoRoot: stale.repoRoot, ticketDir: stale.ticketDir, expectedBaselineCommit: stale.baselineCommit, resumeJournal: attemptJournal(stale, stalePlan) })).toThrow(/resume-state-drift/u);
      expect(snapshot(transactionRoot(stale))).toEqual(staleTransaction);
      expect(snapshot(stale.workspace)).toEqual(staleWorkspace);
      const forgedPlan = plan(forged);
      writeFileSync(attemptJournal(forged, forgedPlan), "{}\n");
      const forgedTransaction = snapshot(transactionRoot(forged)), forgedWorkspace = snapshot(forged.workspace);
      expect(() => applyTaxonomyPlan(forgedPlan, { repoRoot: forged.repoRoot, ticketDir: forged.ticketDir, expectedBaselineCommit: forged.baselineCommit, resumeJournal: attemptJournal(forged, forgedPlan) })).toThrow();
      expect(snapshot(transactionRoot(forged))).toEqual(forgedTransaction);
      expect(snapshot(forged.workspace)).toEqual(forgedWorkspace);
      const missingPlan = plan(missing), missingTransaction = snapshot(transactionRoot(missing)), missingWorkspace = snapshot(missing.workspace);
      expect(() => applyTaxonomyPlan(missingPlan, { repoRoot: missing.repoRoot, ticketDir: missing.ticketDir, expectedBaselineCommit: missing.baselineCommit, resumeJournal: attemptJournal(missing, missingPlan) })).toThrow(/exact existing canonical attempt/u);
      expect(snapshot(transactionRoot(missing))).toEqual(missingTransaction);
      expect(snapshot(missing.workspace)).toEqual(missingWorkspace);
    } finally { retainFixture(stale.root); retainFixture(forged.root); retainFixture(missing.root); }
  }, 15_000);

  test.concurrent("rejects stale generator and embedded-reference authority before mutation", async () => {
    const generator = generatorFixture("stale-generator"), embedded = embeddedFixture("stale-reference");
    try {
      const generatorPlan = plan(generator);
      writeFileSync(join(generator.workspace, "🧪️generator", "🟦️.ts"), "export const input = false;\n");
      const generatorTransaction = snapshot(transactionRoot(generator)), generatorWorkspace = snapshot(generator.workspace);
      expect(() => applyTaxonomyPlan(generatorPlan, { repoRoot: generator.repoRoot, ticketDir: generator.ticketDir, expectedBaselineCommit: generator.baselineCommit })).toThrow(/Regeneration input preimage changed/u);
      expect(snapshot(transactionRoot(generator))).toEqual(generatorTransaction);
      expect(snapshot(generator.workspace)).toEqual(generatorWorkspace);
      const embeddedPlan = plan(embedded);
      await killedAt(embedded, embeddedPlan, "transaction-wal-prepared");
      const reference = join(embedded.workspace, "incoming.json");
      writeFileSync(reference, `${JSON.stringify({ path: embeddedPlan.embeddedTicketRoots[0].sourceMetadataRoot })}\n`);
      const embeddedTransaction = snapshot(transactionRoot(embedded)), embeddedWorkspace = snapshot(embedded.workspace);
      expect(() => applyTaxonomyPlan(embeddedPlan, { repoRoot: embedded.repoRoot, ticketDir: embedded.ticketDir, expectedBaselineCommit: embedded.baselineCommit, resumeJournal: attemptJournal(embedded, embeddedPlan) })).toThrow(/embedded incoming references/u);
      expect(snapshot(transactionRoot(embedded))).toEqual(embeddedTransaction);
      expect(snapshot(embedded.workspace)).toEqual(embeddedWorkspace);
    } finally { retainFixture(generator.root); retainFixture(embedded.root); }
  }, 15_000);

  test.concurrent("rejects ordinal collisions and malformed attempt siblings without mutation", async () => {
    const canonical = await activeReferenceFixture("canonical-collision"), duplicate = await activeReferenceFixture("duplicate-collision"), future = await activeReferenceFixture("future-collision"), malformed = await activeReferenceFixture("malformed-sibling");
    try {
      const canonicalPlan = plan(canonical), canonicalAttempts = dirname(dirname(attemptJournal(canonical, canonicalPlan)));
      mkdirSync(join(canonicalAttempts, "🚧️prepare-000001-999999-00000000-0000-4000-8000-000000000001"));
      const canonicalTransaction = snapshot(transactionRoot(canonical)), canonicalWorkspace = snapshot(canonical.workspace);
      expect(() => applyTaxonomyPlan(canonicalPlan, { repoRoot: canonical.repoRoot, ticketDir: canonical.ticketDir, expectedBaselineCommit: canonical.baselineCommit, resumeJournal: attemptJournal(canonical, canonicalPlan) })).toThrow(/collides with canonical ordinal/u);
      expect(snapshot(transactionRoot(canonical))).toEqual(canonicalTransaction);
      expect(snapshot(canonical.workspace)).toEqual(canonicalWorkspace);
      const duplicatePlan = plan(duplicate), duplicateAttempts = dirname(dirname(attemptJournal(duplicate, duplicatePlan)));
      mkdirSync(join(duplicateAttempts, "🚧️prepare-000002-999998-00000000-0000-4000-8000-000000000002"));
      mkdirSync(join(duplicateAttempts, "🚧️prepare-000002-999999-00000000-0000-4000-8000-000000000003"));
      const duplicateTransaction = snapshot(transactionRoot(duplicate)), duplicateWorkspace = snapshot(duplicate.workspace);
      expect(() => applyTaxonomyPlan(duplicatePlan, { repoRoot: duplicate.repoRoot, ticketDir: duplicate.ticketDir, expectedBaselineCommit: duplicate.baselineCommit, resumeJournal: attemptJournal(duplicate, duplicatePlan) })).toThrow(/duplicate ordinal/u);
      expect(snapshot(transactionRoot(duplicate))).toEqual(duplicateTransaction);
      expect(snapshot(duplicate.workspace)).toEqual(duplicateWorkspace);
      const futurePlan = plan(future), futureAttempts = dirname(dirname(attemptJournal(future, futurePlan)));
      mkdirSync(join(futureAttempts, "🚧️prepare-000003-999999-00000000-0000-4000-8000-000000000004"));
      const futureTransaction = snapshot(transactionRoot(future)), futureWorkspace = snapshot(future.workspace);
      expect(() => applyTaxonomyPlan(futurePlan, { repoRoot: future.repoRoot, ticketDir: future.ticketDir, expectedBaselineCommit: future.baselineCommit, resumeJournal: attemptJournal(future, futurePlan) })).toThrow(/not exact next ordinal/u);
      expect(snapshot(transactionRoot(future))).toEqual(futureTransaction);
      expect(snapshot(future.workspace)).toEqual(futureWorkspace);
      const malformedPlan = plan(malformed), malformedAttempts = dirname(dirname(attemptJournal(malformed, malformedPlan)));
      mkdirSync(join(malformedAttempts, "malformed"));
      const malformedTransaction = snapshot(transactionRoot(malformed)), malformedWorkspace = snapshot(malformed.workspace);
      expect(() => applyTaxonomyPlan(malformedPlan, { repoRoot: malformed.repoRoot, ticketDir: malformed.ticketDir, expectedBaselineCommit: malformed.baselineCommit, resumeJournal: attemptJournal(malformed, malformedPlan) })).toThrow(/Unexpected transaction attempt entry/u);
      expect(snapshot(transactionRoot(malformed))).toEqual(malformedTransaction);
      expect(snapshot(malformed.workspace)).toEqual(malformedWorkspace);
    } finally { retainFixture(canonical.root); retainFixture(duplicate.root); retainFixture(future.root); retainFixture(malformed.root); }
  }, 15_000);

  test.concurrent("rejects forged backup and restore preparations without mutation", async () => {
    const backup = referenceFixture("forged-backup"), restore = referenceFixture("forged-restore");
    try {
      const backupPlan = plan(backup);
      const backupKill = await killedAt(backup, backupPlan, "transaction-backup-retained"), backupLeaf = Object.keys(backupKill.transaction).find((path) => path.endsWith(".backup"))!;
      writeFileSync(join(transactionRoot(backup), backupLeaf), "forged\n");
      const backupTransaction = snapshot(transactionRoot(backup)), backupWorkspace = snapshot(backup.workspace);
      expect(() => applyTaxonomyPlan(backupPlan, { repoRoot: backup.repoRoot, ticketDir: backup.ticketDir, expectedBaselineCommit: backup.baselineCommit, resumeJournal: attemptJournal(backup, backupPlan) })).toThrow(/backup/u);
      expect(snapshot(transactionRoot(backup))).toEqual(backupTransaction);
      expect(snapshot(backup.workspace)).toEqual(backupWorkspace);
      const restorePlan = plan(restore);
      const restoreKill = await killedAt(restore, restorePlan, "transaction-restore-prepared", "", "after-edits"), restoreLeaf = Object.keys(restoreKill.transaction).find((path) => path.includes("restore-") && path.endsWith(".backup"))!;
      writeFileSync(join(transactionRoot(restore), restoreLeaf), "forged\n");
      const restoreTransaction = snapshot(transactionRoot(restore)), restoreWorkspace = snapshot(restore.workspace);
      expect(() => applyTaxonomyPlan(restorePlan, { repoRoot: restore.repoRoot, ticketDir: restore.ticketDir, expectedBaselineCommit: restore.baselineCommit, resumeJournal: attemptJournal(restore, restorePlan) })).toThrow(/Restore preparation bytes/u);
      expect(snapshot(transactionRoot(restore))).toEqual(restoreTransaction);
      expect(snapshot(restore.workspace)).toEqual(restoreWorkspace);
    } finally { retainFixture(backup.root); retainFixture(restore.root); }
  }, 15_000);

  test.concurrent("rejects unreachable duplicate backup and edit publication tuples exactly", async () => {
    const backup = referenceFixture("duplicate-backup-tuple"), edit = referenceFixture("duplicate-edit-tuple"), exchanged = referenceFixture("writer-edit-tuple");
    try {
      const backupPlan = plan(backup), editPlan = plan(edit), exchangedPlan = plan(exchanged);
      const [backupKilled, editKilled, exchangedKilled] = await Promise.all([
        killedAt(backup, backupPlan, "transaction-backup-inner-exchange"),
        killedAt(edit, editPlan, "transaction-edit-inner-exchange"),
        killedAt(exchanged, exchangedPlan, "transaction-edit-exchange"),
      ]);
      const duplicateNestedLeaf = (row: Fixture, killed: Readonly<{ transaction: Snapshot }>, outerSuffix: ".backup" | ".edit"): void => {
        const writeRoot = Object.keys(killed.transaction).find((path) => killed.transaction[path].startsWith("directory|") && /write-[1-9][0-9]*-[0-9a-f-]+$/u.test(path));
        const outer = Object.keys(killed.transaction).find((path) => path.endsWith(outerSuffix) && !path.startsWith(`${writeRoot}/`));
        if (!writeRoot || !outer) throw new Error(`Missing duplicate tuple authority for ${outerSuffix}`);
        const target = join(transactionRoot(row), writeRoot, outerSuffix === ".backup" ? "🚧️.backup" : "🚧️.edit");
        writeFileSync(target, readFileSync(join(transactionRoot(row), outer)));
        chmodSync(target, Number.parseInt(killed.transaction[outer].split("|")[1], 8));
      };
      duplicateNestedLeaf(backup, backupKilled, ".backup");
      duplicateNestedLeaf(edit, editKilled, ".edit");
      const editPreparation = Object.keys(exchangedKilled.transaction).find((path) => exchangedKilled.transaction[path].startsWith("directory|") && path.includes("edit-"));
      if (!editPreparation) throw new Error("Missing exchanged edit preparation");
      mkdirSync(join(transactionRoot(exchanged), editPreparation, `🚧️write-999999-${crypto.randomUUID()}`));
      for (const [row, value, pattern] of [[backup, backupPlan, /duplicate outer and nested/u], [edit, editPlan, /duplicate outer and nested/u], [exchanged, exchangedPlan, /writer coexists/u]] as const) {
        const beforeTransaction = snapshot(transactionRoot(row)), beforeWorkspace = snapshot(row.workspace);
        expect(() => applyTaxonomyPlan(value, { repoRoot: row.repoRoot, ticketDir: row.ticketDir, expectedBaselineCommit: row.baselineCommit, resumeJournal: attemptJournal(row, value) })).toThrow(pattern);
        expect(snapshot(transactionRoot(row))).toEqual(beforeTransaction);
        expect(snapshot(row.workspace)).toEqual(beforeWorkspace);
      }
    } finally { retainFixture(backup.root); retainFixture(edit.root); retainFixture(exchanged.root); }
  }, 15_000);

  test.concurrent("keeps double-plan bytes stable, cancellation exact, and committed second apply immutable", () => {
    const stable = referenceFixture("double-plan"), cancelled = referenceFixture("cancelled"), committed = referenceFixture("second-apply");
    try {
      const first = plan(stable), second = plan(stable);
      expect(canonicalJson(first)).toBe(canonicalJson(second));
      expect(first.planDigest).toBe(second.planDigest);
      const cancelledPlan = plan(cancelled), cancelledBefore = snapshot(cancelled.workspace), cancelFile = join(cancelled.ticketDir, "cancel");
      let cancellationFired = false;
      expect(cancelledPlan.moves.length).toBeGreaterThan(0);
      expect(applyTaxonomyPlan(cancelledPlan, { repoRoot: cancelled.repoRoot, ticketDir: cancelled.ticketDir, expectedBaselineCommit: cancelled.baselineCommit, cancelFile, progress: (event) => { if (event.phase === "staging") { cancellationFired = true; writeFileSync(cancelFile, "cancel\n"); } } }).state).toBe("rolled-back");
      expect(cancellationFired).toBe(true);
      rmSync(cancelFile);
      expect(snapshot(cancelled.workspace)).toEqual(cancelledBefore);
      expectTerminalAttempts(cancelled, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(cancelled)), snapshot(cancelled.workspace), "rolledback:cancellation", cancelled.root);
      const cancelledTransaction = snapshot(transactionRoot(cancelled));
      expect(() => applyTaxonomyPlan(cancelledPlan, { repoRoot: cancelled.repoRoot, ticketDir: cancelled.ticketDir, expectedBaselineCommit: cancelled.baselineCommit, resumeJournal: attemptJournal(cancelled, cancelledPlan) })).toThrow(/rolled-back/u);
      expect(snapshot(transactionRoot(cancelled))).toEqual(cancelledTransaction);
      expect(snapshot(cancelled.workspace)).toEqual(cancelledBefore);
      const committedPlan = plan(committed);
      expect(applyTaxonomyPlan(committedPlan, { repoRoot: committed.repoRoot, ticketDir: committed.ticketDir, expectedBaselineCommit: committed.baselineCommit }).state).toBe("committed");
      const committedTransaction = snapshot(transactionRoot(committed)), committedWorkspace = snapshot(committed.workspace);
      expect(() => applyTaxonomyPlan(committedPlan, { repoRoot: committed.repoRoot, ticketDir: committed.ticketDir, expectedBaselineCommit: committed.baselineCommit })).toThrow(/already has a committed/u);
      expect(snapshot(transactionRoot(committed))).toEqual(committedTransaction);
      expect(snapshot(committed.workspace)).toEqual(committedWorkspace);
    } finally { retainFixture(stable.root); retainFixture(cancelled.root); retainFixture(committed.root); }
  }, 15_000);

  test.concurrent("recovers caught allocation and journal previous-image callback failures", () => {
    const preparation = referenceFixture("caught-preparation"), published = referenceFixture("caught-published"), previous = referenceFixture("caught-previous");
    try {
      const preparationPlan = plan(preparation), preparationWorkspace = snapshot(preparation.workspace), preparationTransaction = snapshot(transactionRoot(preparation));
      expect(() => applyTaxonomyPlan(preparationPlan, { repoRoot: preparation.repoRoot, ticketDir: preparation.ticketDir, expectedBaselineCommit: preparation.baselineCommit, progress: (event) => { if (event.phase === "transaction-attempt-preparation-mkdir") throw new Error("caught preparation"); } })).toThrow(/caught preparation/u);
      expect(snapshot(preparation.workspace)).toEqual(preparationWorkspace);
      expect(snapshot(transactionRoot(preparation))).toEqual(preparationTransaction);
      const publishedPlan = plan(published), publishedWorkspace = snapshot(published.workspace);
      expect(applyTaxonomyPlan(publishedPlan, { repoRoot: published.repoRoot, ticketDir: published.ticketDir, expectedBaselineCommit: published.baselineCommit, progress: (event) => { if (event.phase === "transaction-attempt-canonical-published") throw new Error("caught published"); } }).state).toBe("rolled-back");
      expect(snapshot(published.workspace)).toEqual(publishedWorkspace);
      expectTerminalAttempts(published, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(published)), snapshot(published.workspace), "rolledback:caught-attempt-canonical-published", published.root);
      const previousPlan = plan(previous), previousWorkspace = snapshot(previous.workspace);
      expect(applyTaxonomyPlan(previousPlan, { repoRoot: previous.repoRoot, ticketDir: previous.ticketDir, expectedBaselineCommit: previous.baselineCommit, progress: (event) => { if (event.phase === "transaction-journal-previous-exchanged") throw new Error("caught previous"); } }).state).toBe("rolled-back");
      expect(snapshot(previous.workspace)).toEqual(previousWorkspace);
      expectTerminalAttempts(previous, "rolled-back");
      expectBoundaryGolden(snapshot(transactionRoot(previous)), snapshot(previous.workspace), "rolledback:caught-journal-previous-exchanged", previous.root);
    } finally { retainFixture(preparation.root); retainFixture(published.root); retainFixture(previous.root); }
  }, 15_000);
});
//#endregion 🧾️TransactionV2
